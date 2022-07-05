use owo_colors::{AnsiColors, DynColors::{self, Rgb, Ansi}};
use std::fmt;
use strum::EnumIter;

pub struct Colors {
    basic_colors: Vec<DynColors>,
    true_colors: Option<Vec<DynColors>>,
}

#[derive(Clone, PartialEq, clap::ValueEnum)]
pub enum LanguageType {
    Programming,
    Markup,
    Prose,
    Data,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, EnumIter, clap::ValueEnum)]
#[allow(clippy::upper_case_acronyms)]
#[clap(rename_all = "lowercase")]
pub enum Language {
    {% for language, attrs in languages -%}
        {% if attrs.serialization %}#[clap(name="{{ attrs.serialization }}")]{% endif %}
        {{ language }},
    {% endfor %}
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            {% for language, _ in languages -%}
                Self::{{ language }} => write!(f, "{}", tokei::LanguageType::{{ language }}.name()),
            {% endfor %}
        }
    }
}

impl From<tokei::LanguageType> for Language {
    fn from(language: tokei::LanguageType) -> Self {
        match language {
            {% for language, _ in languages -%}
                tokei::LanguageType::{{ language }} => Self::{{ language }},
            {% endfor %}
            _ => unimplemented!("Language {:?}", language),
        }
    }
}

impl From<Language> for tokei::LanguageType {
    fn from(language: Language) -> Self {
        match language {
            {% for language, _ in languages -%}
                Language::{{ language }} => tokei::LanguageType::{{ language }},
            {% endfor %}
        }
    }
}

impl Language {
    pub fn get_ascii_art(&self) -> &'static str {
        match self {
            {% for language, attrs in languages -%}
                Language::{{ language }} => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/", "{{ attrs.ascii }}")),
            {% endfor %}
        }
    }

    pub fn get_colors(&self, true_color: bool) -> Vec<DynColors> {
        let colors = match self {
            {% for language, attrs in languages -%}
                Language::{{ language }} => Colors {
                    basic_colors: vec![{%- for color in attrs.colors.ansi -%}Ansi(AnsiColors::{{ color | title }}),{% endfor %}],
                    true_colors: {% if attrs.colors.rgb %}Some(vec![{%- for rgb in attrs.colors.rgb -%}Rgb({{ rgb | nth(n=0) }}, {{ rgb | nth(n=1) }}, {{ rgb | nth(n=2) }}),{% endfor %}]){% else %}None{% endif %},
                },
            {% endfor %}
        };
        match colors.true_colors {
            Some(true_colors) if true_color => true_colors,
            _ => colors.basic_colors,
        }
    }

    pub fn get_type(&self) -> LanguageType {
        match self {
            {% for language, attrs in languages -%}
                Language::{{ language }} => LanguageType::{{ attrs.type | title }},
            {% endfor %}
        }
    }

    pub fn get_circle_color(&self) -> DynColors {
        // TODO
        DynColors::Ansi(AnsiColors::White)
    }
}

{% for language, attrs in languages -%}
    {% if attrs.colors.rgb %}
        {% set ansi_length = attrs.colors.ansi | length %}
        {% set rgb_length = attrs.colors.rgb | length %}
        {% if ansi_length != rgb_length %}
            compile_error!("{{ language }}: ansi and rgb colors must be the same length");
        {% endif %}
    {% endif %}
{% endfor %}

#[cfg(test)]
mod ascii_size {
    use crate::ui::ascii_art::get_min_start_max_end;
    use more_asserts::assert_le;

    const MAX_WIDTH: usize = 40;
    const MAX_HEIGHT: usize = 25;

    // TODO Make compiler errors
    {% for language, attrs in languages -%}
        #[test]
        fn {{ language | lower }}_width() {
            const ASCII: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/", "{{ attrs.ascii }}"));
            let lines: Vec<_> = ASCII.lines().skip_while(|line| line.is_empty()).collect();
            let (start, end) = get_min_start_max_end(&lines);
            assert!(start <= end);
            assert_le!(end - start, MAX_WIDTH, "{{ attrs.ascii }} is too wide.");
        }

        #[test]
        fn {{ language | lower }}_height() {
            const ASCII: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/", "{{ attrs.ascii }}"));
            assert_le!(ASCII.lines().count(), MAX_HEIGHT, "{{ attrs.ascii }} is too tall.");
        }
    {% endfor %}
}
