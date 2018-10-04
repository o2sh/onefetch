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
        s.push_str(&("Project: ".color(color).bold().to_string() + &format!("{}\n", self.project_name)));
        s.push_str(&("Language: ".color(color).bold().to_string() + &format!("{}\n", self.language)));
        s.push_str(&("Author: ".color(color).bold().to_string() + &format!("{}\n", self.author)));
        s.push_str(&("Repo: ".color(color).bold().to_string() + &format!("{}\n", self.repo)));
        s.push_str(&("Number of lines: ".color(color).bold().to_string() + &format!("{}\n", self.number_of_lines)));
        s.push_str(&("License: ".color(color).bold().to_string() + &format!("{}\n", self.license)));

        let logo= self.get_ascii();
        let mut lines = s.lines();
        let left_pad = logo.lines().map(|l| l.len()).max().unwrap_or(0);
        let mut o = String::new();
        for a in logo.lines() {
            let b = match lines.next() {
                Some(line) => line,
                None => "",
            };
            o.push_str(&format!("{:width$} {}\n", a.color(color).bold(), b, width = left_pad));
        }

        write!(f, "{}", o)
    }
}

enum Language {
    Rust,
    Go,
    Java,
    Cpp,
    C,
    Python,
    Csharp,
    Scala,
    Clojure,
    Shell,
    Lisp,
    Haskell,
    Ruby,
    R,
}

fn get_color(l : &Language) -> &str {

     match l {
         Language::Rust => "cyan",
         Language::Go => "white",
         Language::Java => "green",
         Language::Cpp => "yellow",
         Language::C => "cyan",
         Language::Python => "magenta",
         Language::Csharp => "white",
         Language::Scala => "blue",
         Language::Clojure => "cyan",
         Language::Shell => "green",
         Language::Lisp => "yellow",
         Language::Haskell => "cyan",
         Language::Ruby => "magenta",
         Language::R => "blue",
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
          Language::Rust => write!(f, "Rust"),
          Language::Go => write!(f, "Go"),
          Language::Java => write!(f, "Java"),
          Language::Cpp => write!(f, "C++"),
          Language::C => write!(f, "C"),
          Language::Python => write!(f, "Python"),
          Language::Csharp => write!(f, "C#"),
          Language::Scala => write!(f, "Scala"),
          Language::Clojure => write!(f, "Clojure"),
          Language::Shell => write!(f, "Shell"),
          Language::Lisp => write!(f, "Lisp"),
          Language::Haskell => write!(f, "Haskell"),
          Language::Ruby => write!(f, "Ruby"),
          Language::R => write!(f, "R"),
       }
    }
}

fn main() {

    let language = match get_dominant_language() {
        Some(language) => language,
        None => {
            eprintln!("Could not find any source code in this directory.");
            return;
        }
    };

    let info = Info {
        project_name: String::from("onefetch"),
        language: language,
        //language: Language::Python,
        author: String::from("Ossama Hjaji"),
        repo: String::from("https://github.com/02sh/onefetch"),
        number_of_lines: 15656,
        license: String::from("MIT"),
    };

    println!("{}", info);

}

/// Traverse current directory and search for dominant
/// language using tokei.
fn get_dominant_language() -> Option<Language> {
    let mut languages = tokei::Languages::new();
    let required_languages = get_all_language_types();
    languages.get_statistics( &["."], vec![".git", "target"], Some(required_languages));

    languages.remove_empty().iter()
                            .max_by_key(|(_, v)| v.code)
                            .map(|(k, _)| Language::from(**k))
}

/// Convert from tokei LanguageType to known Language type .
impl From<tokei::LanguageType> for Language {
    fn from(language: tokei::LanguageType) -> Self {
        match language {
            tokei::LanguageType::Rust => Language::Rust,
            tokei::LanguageType::Go => Language::Go,
            tokei::LanguageType::Java => Language::Java,
            tokei::LanguageType::Cpp => Language::Cpp,
            tokei::LanguageType::C => Language::C,
            tokei::LanguageType::Python => Language::Python,
            tokei::LanguageType::CSharp => Language::Csharp,
            tokei::LanguageType::Scala => Language::Scala,
            tokei::LanguageType::Sh => Language::Shell,
            tokei::LanguageType::Lisp => Language::Lisp,
            tokei::LanguageType::Haskell => Language::Haskell,
            tokei::LanguageType::Ruby => Language::Ruby,
            tokei::LanguageType::R => Language::R,
            tokei::LanguageType::Clojure => Language::Clojure,
            _ => unimplemented!(),
        }
    }
}

fn get_all_language_types() -> Vec<tokei::LanguageType> {
    vec![
        tokei::LanguageType::Rust,
        tokei::LanguageType::Go,
        tokei::LanguageType::Java,
        tokei::LanguageType::Cpp,
        tokei::LanguageType::C,
        tokei::LanguageType::Python,
        tokei::LanguageType::CSharp,
        tokei::LanguageType::Scala,
        tokei::LanguageType::Sh,
        tokei::LanguageType::Lisp,
        tokei::LanguageType::Haskell,
        tokei::LanguageType::Ruby,
        tokei::LanguageType::R,
        tokei::LanguageType::Clojure,
    ]
}

impl Info {
    pub fn get_ascii(&self) -> &str {
        match self.language {
            Language::Rust => include_str!("../resources/rust.ascii"),
            Language::Go => include_str!("../resources/go.ascii"),
            Language::Java => include_str!("../resources/java.ascii"),
            Language::Cpp => include_str!("../resources/cpp.ascii"),
            Language::C => include_str!("../resources/c.ascii"),
            Language::Python => include_str!("../resources/python.ascii"),
            Language::Csharp => include_str!("../resources/csharp.ascii"),
            Language::Scala => include_str!("../resources/scala.ascii"),
            Language::Clojure => include_str!("../resources/clojure.ascii"),
            Language::Shell => include_str!("../resources/shell.ascii"),
            Language::Lisp => include_str!("../resources/lisp.ascii"),
            Language::Haskell => include_str!("../resources/haskell.ascii"),
            Language::Ruby => include_str!("../resources/ruby.ascii"),
            Language::R => include_str!("../resources/r.ascii"),
           // _ => include_str!("../resources/unknown.ascii"),
        }
    }
}
