use owo_colors::OwoColorize;

use crate::info::info_field::{InfoField, InfoFieldType, InfoFieldValue};
use std::fmt::Write;

include!(concat!(env!("OUT_DIR"), "/language.rs"));

pub struct LanguagesInfoField {
    pub languages: Vec<(Language, f64)>,
    pub true_color: bool,
    pub info_color: DynColors,
}

impl std::fmt::Display for LanguagesInfoField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut language_field = String::from("");
        let language_bar_length = 26;
        let pad = "Languages".len() + 2;
        let color_palette = vec![
            DynColors::Ansi(AnsiColors::Red),
            DynColors::Ansi(AnsiColors::Green),
            DynColors::Ansi(AnsiColors::Yellow),
            DynColors::Ansi(AnsiColors::Blue),
            DynColors::Ansi(AnsiColors::Magenta),
            DynColors::Ansi(AnsiColors::Cyan),
        ];

        let languages: Vec<(String, f64, DynColors)> = {
            let mut iter = self
                .languages
                .iter()
                .enumerate()
                .map(|(i, (language, perc))| {
                    let circle_color = if self.true_color {
                        language.get_circle_color()
                    } else {
                        color_palette[i % color_palette.len()]
                    };
                    (language.to_string(), *perc, circle_color)
                });
            if self.languages.len() > 6 {
                let mut languages = iter.by_ref().take(6).collect::<Vec<_>>();
                let other_perc = iter.fold(0.0, |acc, x| acc + x.1);
                languages.push((
                    "Other".to_string(),
                    other_perc,
                    DynColors::Ansi(AnsiColors::White),
                ));
                languages
            } else {
                iter.collect()
            }
        };

        let language_bar: String = languages
            .iter()
            .map(|(_, perc, circle_color)| {
                let bar_width = std::cmp::max(
                    (perc / 100. * language_bar_length as f64).round() as usize,
                    1,
                );
                format!("{:<width$}", "".on_color(*circle_color), width = bar_width)
            })
            .collect();

        language_field.push_str(&language_bar);

        for (i, (language, perc, circle_color)) in languages.iter().enumerate() {
            let formatted_number = format!("{:.*}", 1, perc);
            let circle = "\u{25CF}".color(*circle_color);
            let language_str = format!(
                "{} {} ",
                circle,
                format!("{} ({} %)", language, formatted_number).color(self.info_color)
            );
            if i % 2 == 0 {
                let _ = write!(
                    language_field,
                    "\n{:<width$}{}",
                    "",
                    language_str,
                    width = pad
                );
            } else {
                language_field.push_str(&language_str.to_string());
            }
        }
        write!(f, "{}", language_field)
    }
}

impl InfoField for LanguagesInfoField {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: InfoFieldType::Languages,
            value: format!("{}", &self),
        }
    }
}
