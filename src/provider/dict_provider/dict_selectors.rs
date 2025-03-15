use scraper::Selector;

pub trait DictSelectors {
    // Главный блок с контентом
    fn content() -> Selector;

    // Само слово
    fn word() -> Selector;

    // Часть речи (verb, noun, etc)
    fn pos() -> Selector;

    // Определения
    fn meaning_list() -> Selector;

    // Примеры
    fn example_list() -> Selector;

    // Ссылки к ознакомлению (идиомы, фразальные глаголы)
    fn poi_links() -> Option<Selector>;
}
