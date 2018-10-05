extern crate colored;
extern crate tokei;

use colored::*;
use std::fmt;

struct Info {
    project_name: String,
    language: Language,
    author: String,
    repo: String,
    number_of_lines: usize,
    license: String,
}

impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::from("\n");
        let color = get_color(&self.language);
        s.push_str(
            &("Project: ".color(color).bold().to_string() + &format!("{}\n", self.project_name)),
        );
        s.push_str(
            &("Language: ".color(color).bold().to_string() + &format!("{}\n", self.language)),
        );
        s.push_str(&("Author: ".color(color).bold().to_string() + &format!("{}\n", self.author)));
        s.push_str(&("Repo: ".color(color).bold().to_string() + &format!("{}\n", self.repo)));
        s.push_str(
            &("Number of lines: ".color(color).bold().to_string()
                + &format!("{}\n", self.number_of_lines)),
        );
        s.push_str(&("License: ".color(color).bold().to_string() + &format!("{}\n", self.license)));

        let logo = self.get_ascii();
        let mut lines = s.lines();
        let left_pad = logo.lines().map(|l| l.len()).max().unwrap_or(0);
        let mut o = String::new();
        for a in logo.lines() {
            let b = match lines.next() {
                Some(line) => line,
                None => "",
            };
            o.push_str(&format!(
                "{:width$} {}\n",
                a.color(color).bold(),
                b,
                width = left_pad
            ));
        }

        write!(f, "{}", o)
    }
}

enum Language {
    C,
    Clojure,
    Cpp,
    Csharp,
    Go,
    Haskell,
    Java,
    Lisp,
    Python,
    R,
    Ruby,
    Rust,
    Scala,
    Shell,
    TypeScript,
}

fn get_color(l: &Language) -> &str {
    match l {
        Language::C => "cyan",
        Language::Clojure => "cyan",
        Language::Cpp => "yellow",
        Language::Csharp => "white",
        Language::Go => "white",
        Language::Haskell => "cyan",
        Language::Java => "green",
        Language::Lisp => "yellow",
        Language::Python => "magenta",
        Language::R => "blue",
        Language::Ruby => "magenta",
        Language::Rust => "cyan",
        Language::Scala => "blue",
        Language::Shell => "green",
        Language::TypeScript => "cyan",
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Language::C => write!(f, "C"),
            Language::Clojure => write!(f, "Clojure"),
            Language::Cpp => write!(f, "C++"),
            Language::Csharp => write!(f, "C#"),
            Language::Go => write!(f, "Go"),
            Language::Haskell => write!(f, "Haskell"),
            Language::Java => write!(f, "Java"),
            Language::Lisp => write!(f, "Lisp"),
            Language::Python => write!(f, "Python"),
            Language::R => write!(f, "R"),
            Language::Ruby => write!(f, "Ruby"),
            Language::Rust => write!(f, "Rust"),
            Language::Scala => write!(f, "Scala"),
            Language::Shell => write!(f, "Shell"),
            Language::TypeScript => write!(f, "TypeScript"),
        }
    }
}

fn main() {
    let tokei_langs = project_languages();
    let language = match get_dominant_language(&tokei_langs) {
        Some(language) => language,
        None => {
            eprintln!("Could not find any source code in this directory.");
            return;
        }
    };

    let info = Info {
        project_name: String::from("onefetch"),
        language: language,
        author: String::from("Ossama Hjaji"),
        repo: String::from("https://github.com/02sh/onefetch"),
        number_of_lines: get_total_loc(&tokei_langs),
        license: String::from("MIT"),
    };

    println!("{}", info);
}

fn project_languages() -> tokei::Languages {
    let mut languages = tokei::Languages::new();
    let required_languages = get_all_language_types();
    languages.get_statistics(&["."], vec![".git", "target"], Some(required_languages));
    languages
}

/// Traverse current directory and search for dominant
/// language using tokei.
fn get_dominant_language(languages: &tokei::Languages) -> Option<Language> {
    languages
        .remove_empty()
        .iter()
        .max_by_key(|(_, v)| v.code)
        .map(|(k, _)| Language::from(**k))
}

fn get_total_loc(languages: &tokei::Languages) -> usize {
    languages
        .values()
        .collect::<Vec<&tokei::Language>>()
        .iter()
        .fold(0, |sum, val| sum + val.code)
}

/// Convert from tokei LanguageType to known Language type .
impl From<tokei::LanguageType> for Language {
    fn from(language: tokei::LanguageType) -> Self {
        match language {
            tokei::LanguageType::C => Language::C,
            tokei::LanguageType::Clojure => Language::Clojure,
            tokei::LanguageType::Cpp => Language::Cpp,
            tokei::LanguageType::CSharp => Language::Csharp,
            tokei::LanguageType::Go => Language::Go,
            tokei::LanguageType::Haskell => Language::Haskell,
            tokei::LanguageType::Java => Language::Java,
            tokei::LanguageType::Lisp => Language::Lisp,
            tokei::LanguageType::Python => Language::Python,
            tokei::LanguageType::R => Language::R,
            tokei::LanguageType::Ruby => Language::Ruby,
            tokei::LanguageType::Rust => Language::Rust,
            tokei::LanguageType::Scala => Language::Scala,
            tokei::LanguageType::Sh => Language::Shell,
            tokei::LanguageType::TypeScript => Language::TypeScript,
            _ => unimplemented!(),
        }
    }
}

fn get_all_language_types() -> Vec<tokei::LanguageType> {
    vec![
        tokei::LanguageType::C,
        tokei::LanguageType::Clojure,
        tokei::LanguageType::Cpp,
        tokei::LanguageType::CSharp,
        tokei::LanguageType::Go,
        tokei::LanguageType::Haskell,
        tokei::LanguageType::Java,
        tokei::LanguageType::Lisp,
        tokei::LanguageType::Python,
        tokei::LanguageType::R,
        tokei::LanguageType::Ruby,
        tokei::LanguageType::Rust,
        tokei::LanguageType::Scala,
        tokei::LanguageType::Sh,
        tokei::LanguageType::TypeScript,
    ]
}

impl Info {
    pub fn get_ascii(&self) -> &str {
        match self.language {
            Language::C => include_str!("../resources/c.ascii"),
            Language::Clojure => include_str!("../resources/clojure.ascii"),
            Language::Cpp => include_str!("../resources/cpp.ascii"),
            Language::Csharp => include_str!("../resources/csharp.ascii"),
            Language::Go => include_str!("../resources/go.ascii"),
            Language::Haskell => include_str!("../resources/haskell.ascii"),
            Language::Java => include_str!("../resources/java.ascii"),
            Language::Lisp => include_str!("../resources/lisp.ascii"),
            Language::Python => include_str!("../resources/python.ascii"),
            Language::R => include_str!("../resources/r.ascii"),
            Language::Ruby => include_str!("../resources/ruby.ascii"),
            Language::Rust => include_str!("../resources/rust.ascii"),
            Language::Scala => include_str!("../resources/scala.ascii"),
            Language::Shell => include_str!("../resources/shell.ascii"),
            Language::TypeScript => include_str!("../resources/typescript.ascii"),
            // _ => include_str!("../resources/unknown.ascii"),
        }
    }
}
