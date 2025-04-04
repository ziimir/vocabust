use futures::future::try_join_all;
use reqwest::{Client, StatusCode};
use scraper::{Html, Selector, ElementRef};

use super::super::{WordData, Meaning};
use super::dict_provider::{DictProvider, DictProviderError};

const POS_EN_COUNT: u8 = 7; // количество частей речи в английском


pub struct OxfordDictProvider {
    client: Client
}

impl OxfordDictProvider {
    const URL_TEMPLATE: &str = "https://www.oxfordlearnersdictionaries.com/search/english/?q={}";

    pub fn new(client: Client) -> Self {
        OxfordDictProvider { client }
    }

    pub async fn search_word(&self, query: &str) -> Result<WordData, DictProviderError> {
        let query_url = Self::URL_TEMPLATE.replace("{}", query);
        let response = self.client.head(&query_url).send().await?;
        let url = response.url();
        let url_str = url.as_str();

        let (url_template, _) = url_str
            .rsplit_once("/")
            .ok_or(DictProviderError::UrlParseError(url_str.to_string()))?;

        if url.path() == "/" {
            return Err(DictProviderError::BadRequest(format!("Bad query string: `{}`", query)));
        }

        if url.path().starts_with("/spellcheck") {
            return Err(DictProviderError::NotFound(format!("Definition for query `{}` is not found", query)));
        }

        let path = url.path();
        let word_variants: Vec<String> = self.word_variants(path)
            .ok_or(DictProviderError::UrlParseError(String::from("Fail to parse word variants")))?
            .iter()
            .map(|variant| format!("{}/{}", url_template, variant))
            .collect();

        let responses: Result<Vec<Option<(String, String)>>, DictProviderError> = try_join_all(
            word_variants
                .into_iter()
                .map(|url| {
                    let client = self.client.clone();

                    async move {
                        let response = client.get(&url).send().await?;

                        match response.status() {
                            status if status.is_success() => {
                                let body = response.text().await?;
                                Ok(Some((url, body)))
                            },
                            StatusCode::NOT_FOUND => {
                                Ok(None)
                            },
                            _ => {
                                let error = response.error_for_status().unwrap_err();
                                Err(DictProviderError::RequestError(error))
                            }
                        }
                    }
                })
        )
            .await;

        let definitions = responses?
            .into_iter()
            .map(|res| {
                match res {
                    Some((source, data)) => {
                        let def = self.definition(&Html::parse_document(&data), source)?;
                        Ok(Some(def))
                    },
                    None => Ok(None),
                }
            })
            .collect::<Result<Vec<Option<WordData>>, DictProviderError>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<WordData>>();

        let word_data = definitions.into_iter().fold(
            WordData::empty(String::from(query)),
            |mut acc, def| {
                acc.add_definition(def.word, def.definitions, def.pronunciation_url);
                acc
            }
        );

        Ok(word_data)
    }

    fn word_variants(&self, path: &str) -> Option<Vec<String>> {
        let segments = path.split("/").last()?;
        let (word, variant) = segments
            .rsplit_once("_")
            .or(Some((segments, "")))?;

        if variant.is_empty() {
            return Some(vec![word.to_string()]);
        }

        let variants_range: Vec<String> = (1 .. POS_EN_COUNT + 1)
            .into_iter()
            .map(|i| {
                format!("{}_{}", word, i)
            })
            .collect();

        Some(variants_range)
    }
}

impl DictProvider for OxfordDictProvider {
    fn definition(&self, data: &Html, source: String) -> Result<WordData, DictProviderError> {
        let content = data
            .select(&Selector::parse("#entryContent").unwrap())
            .next()
            .ok_or(DictProviderError::SelectError(
                String::from(format!("Element with selector #entryContent not found"))
            ))?;

        let word = self
            .word(&content)
            .ok_or(DictProviderError::SelectError(String::from("word is not found")))?;

        let pos = self
            .pos(&content)
            .ok_or(DictProviderError::SelectError(String::from("pos is not found")))?;

        let meaning_list = self
            .meaning_list(&content)
            .ok_or(DictProviderError::SelectError(String::from("meanings is not found")))?;

        let pronunciation = self
            .pronunciation(&content);

        Ok(WordData::new(word, pos, meaning_list, pronunciation, source))
    }

    fn word<'a>(&self, content: &ElementRef<'a>) -> Option<String> {
        let word = content
            .select(&Selector::parse(".headword").unwrap())
            .next()?
            .text()
            .collect::<String>()
            .trim()
            .to_string();

        if word.is_empty() {
            return None;
        }

        Some(word)
    }

    fn pos<'a>(&self, content: &ElementRef<'a>) -> Option<String> {
        let pos = content
            .select(&Selector::parse(".headword + .pos").unwrap())
            .next()?
            .text()
            .collect::<String>()
            .trim()
            .to_string();

        if pos.is_empty() {
            return None;
        }

        Some(pos)
    }

    fn meaning_list<'a>(&self, content: &ElementRef<'a>) -> Option<Vec<Meaning>> {
        let selector = Selector::parse(
            ".entry > .sense_single .sense, .entry > .senses_multiple .sense",
        ).unwrap();

        let meanings: Vec<Meaning> = content
            .select(&selector)
            .map(|sense_item_element| {
                let def = sense_item_element
                    .select(&Selector::parse(".def").unwrap())
                    .next()
                    .or(sense_item_element
                        .select(&Selector::parse(".sensetop").unwrap())
                        .next()
                    )?
                    .text()
                    .collect::<String>()
                    .trim()
                    .to_string();

                let examples = sense_item_element
                    .select(&Selector::parse(".examples li").unwrap())
                    .map(|example_item_element| {
                        example_item_element
                            .text()
                            .collect::<String>()
                            .trim()
                            .to_string()
                        })
                    .collect();

                if def.is_empty() {
                    return None;
                }

                Some(Meaning { description: def, examples })
            })
            .flatten()
            .collect();

        if meanings.len() == 0 {
            return None;
        }

        Some(meanings)
    }

    fn pronunciation<'a>(&self, content: &ElementRef<'a>) -> Option<String> {
        let pronunciation = content
            .select(&Selector::parse(".phonetics .phons_n_am .sound").unwrap())
            .next()?
            .attr("data-src-mp3")
            .map(|x| String::from(x));

        pronunciation
    }
}
