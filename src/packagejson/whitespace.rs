use std::fmt::Display;

use anyhow::Result;

pub(super) enum WhitespaceType {
    Tabs,
    Spaces(i32),
}

impl Display for WhitespaceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WhitespaceType::Tabs => "\t".to_owned(),
                WhitespaceType::Spaces(count) => " ".repeat(*count as usize),
            }
        )
    }
}

pub(super) fn determine(str: &str) -> Result<WhitespaceType> {
    let lines = str.split('\n').collect::<Vec<&str>>();

    if lines.first().is_some_and(|f| f == &"{") {
        let line_two = lines.get(2);

        if let Some(line_two) = line_two {
            if line_two.starts_with('\t') {
                return Ok(WhitespaceType::Tabs);
            } else if line_two.starts_with("    ") {
                return Ok(WhitespaceType::Spaces(4));
            } else if line_two.starts_with("  ") {
                return Ok(WhitespaceType::Spaces(2));
            }
        }
    }

    Ok(WhitespaceType::Spaces(2))
}
