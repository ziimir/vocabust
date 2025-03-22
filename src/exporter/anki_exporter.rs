use std::fs::File;
use std::io::Write;

use crate::data::{Meaning, WordData};

const ANKI_FILE_HEADER: &str = "#separator:Tab\n#html:true";

pub fn create_anki_file(
    file: &mut File,
    data: &WordData,
) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(file, "{}", ANKI_FILE_HEADER)?;

    data.definitions.iter().for_each(|def| {
        let question = format!("{} ({})", data.word, def.pos);

        let answer = def
            .meaning
            .iter()
            .map(|mean| format_meaning(mean))
            .collect::<Vec<String>>()
            .join("<br>---<br><br>");

        writeln!(file, "{}\t{}", question, answer);
    });

    Ok(())
}

fn format_meaning(meaning: &Meaning) -> String {
    let examples = if meaning.examples.len() > 0 {
        format!(
            "<br><br>```<br>{}<br>```<br>",
            meaning.examples.join("<br><br>")
        )
    } else {
        String::from("<br>")
    };

    format!("**{}**{}", meaning.description, examples)
}
