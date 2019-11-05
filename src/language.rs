use std::collections::HashMap;

use colored::Color;

use crate::{Error, Result};

#[derive(PartialEq, Eq, Hash, Clone, EnumString, EnumIter)]
#[strum(serialize_all = "lowercase")]
pub enum Language {
    Assembly,
    C,
    Clojure,
    CoffeeScript,
    #[strum(serialize = "c++")]
    Cpp,
    #[strum(serialize = "c#")]
    Csharp,
    CSS,
    D,
    Dart,
    Elixir,
    Elm,
    Erlang,
    Forth,
    #[strum(serialize = "fortran")]
    FortranModern,
    FSharp,
    Go,
    Haskell,
    HTML,
    Idris,
    Java,
    JavaScript,
    Julia,
    Kotlin,
    Lisp,
    Lua,
    Markdown,
    Nim,
    #[strum(serialize = "objective-c")]
    ObjectiveC,
    OCaml,
    Perl,
    Php,
    Prolog,
    PureScript,
    Python,
    R,
    Racket,
    Ruby,
    Rust,
    Scala,
    Shell,
    Swift,
    Tcl,
    Tex,
    TypeScript,
    Vue,
    XML,
    Zig,
    Unknown,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Language::Assembly => write!(f, "Assembly"),
            Language::C => write!(f, "C"),
            Language::Clojure => write!(f, "Clojure"),
            Language::CoffeeScript => write!(f, "CoffeeScript"),
            Language::Cpp => write!(f, "C++"),
            Language::Csharp => write!(f, "C#"),
            Language::CSS => write!(f, "CSS"),
            Language::D => write!(f, "D"),
            Language::Dart => write!(f, "Dart"),
            Language::Elixir => write!(f, "Elixir"),
            Language::Elm => write!(f, "Elm"),
            Language::Erlang => write!(f, "Erlang"),
            Language::Forth => write!(f, "Forth"),
            Language::FortranModern => write!(f, "Fortran"),
            Language::FSharp => write!(f, "FSharp"),
            Language::Go => write!(f, "Go"),
            Language::Haskell => write!(f, "Haskell"),
            Language::HTML => write!(f, "HTML"),
            Language::Idris => write!(f, "Idris"),
            Language::Java => write!(f, "Java"),
            Language::JavaScript => write!(f, "JavaScript"),
            Language::Julia => write!(f, "Julia"),
            Language::Kotlin => write!(f, "Kotlin"),
            Language::Lisp => write!(f, "Lisp"),
            Language::Lua => write!(f, "Lua"),
            Language::Markdown => write!(f, "Markdown"),
            Language::Nim => write!(f, "Nim"),       
            Language::ObjectiveC => write!(f, "Objective-C"),
            Language::OCaml => write!(f, "OCaml"),
            Language::PureScript => write!(f, "PureScript"),
            Language::Python => write!(f, "Python"),
            Language::R => write!(f, "R"),
            Language::Racket => write!(f, "Racket"),
            Language::Ruby => write!(f, "Ruby"),
            Language::Rust => write!(f, "Rust"),
            Language::Scala => write!(f, "Scala"),
            Language::Shell => write!(f, "Shell"),
            Language::Swift => write!(f, "Swift"),
            Language::Prolog => write!(f, "Prolog"),
            Language::Perl => write!(f, "Perl"),
            Language::Php => write!(f, "Php"),
            Language::Tcl => write!(f, "Tcl"),
            Language::Tex => write!(f, "Tex"),
            Language::TypeScript => write!(f, "TypeScript"),
            Language::Vue => write!(f, "Vue"),
            Language::XML => write!(f, "XML"),
            Language::Zig => write!(f, "Zig"),
            Language::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Convert from tokei LanguageType to known Language type .
impl From<tokei::LanguageType> for Language {
    fn from(language: tokei::LanguageType) -> Self {
        match language {
            tokei::LanguageType::Assembly => Language::Assembly,
            tokei::LanguageType::C => Language::C,
            tokei::LanguageType::Clojure => Language::Clojure,
            tokei::LanguageType::CoffeeScript => Language::CoffeeScript,
            tokei::LanguageType::Cpp => Language::Cpp,
            tokei::LanguageType::CSharp => Language::Csharp,
            tokei::LanguageType::Css => Language::CSS,
            tokei::LanguageType::D => Language::D,
            tokei::LanguageType::Dart => Language::Dart,
            tokei::LanguageType::Elixir => Language::Elixir,
            tokei::LanguageType::Elm => Language::Elm,
            tokei::LanguageType::Erlang => Language::Erlang,
            tokei::LanguageType::Forth => Language::Forth,
            tokei::LanguageType::FortranModern => Language::FortranModern,
            tokei::LanguageType::FSharp => Language::FSharp,
            tokei::LanguageType::Go => Language::Go,
            tokei::LanguageType::Haskell => Language::Haskell,
            tokei::LanguageType::Html => Language::HTML,
            tokei::LanguageType::Idris => Language::Idris,
            tokei::LanguageType::Java => Language::Java,
            tokei::LanguageType::JavaScript => Language::JavaScript,
            tokei::LanguageType::Julia => Language::Julia,
            tokei::LanguageType::Kotlin => Language::Kotlin,
            tokei::LanguageType::Lisp => Language::Lisp,
            tokei::LanguageType::Lua => Language::Lua,
            tokei::LanguageType::Markdown => Language::Markdown,
            tokei::LanguageType::Nim => Language::Nim,            
            tokei::LanguageType::ObjectiveC => Language::ObjectiveC,
            tokei::LanguageType::OCaml => Language::OCaml,
            tokei::LanguageType::Prolog => Language::Prolog,
            tokei::LanguageType::Perl => Language::Perl,
            tokei::LanguageType::Php => Language::Php,
            tokei::LanguageType::PureScript => Language::PureScript,
            tokei::LanguageType::Python => Language::Python,
            tokei::LanguageType::R => Language::R,
            tokei::LanguageType::Racket => Language::Racket,
            tokei::LanguageType::Ruby => Language::Ruby,
            tokei::LanguageType::Rust => Language::Rust,
            tokei::LanguageType::Scala => Language::Scala,
            tokei::LanguageType::Sh => Language::Shell,
            tokei::LanguageType::Swift => Language::Swift,
            tokei::LanguageType::Tcl => Language::Tcl,
            tokei::LanguageType::Tex => Language::Tex,
            tokei::LanguageType::TypeScript => Language::TypeScript,
            tokei::LanguageType::Vue => Language::Vue,
            tokei::LanguageType::Xml => Language::XML,
            tokei::LanguageType::Zig => Language::Zig,
            _ => unimplemented!(),
        }
    }
}

impl Language {
    pub fn get_ascii_art(&self) -> &str {
        match *self {
            Language::Assembly => include_str!("../resources/assembly.ascii"),
            Language::C => include_str!("../resources/c.ascii"),
            Language::Clojure => include_str!("../resources/clojure.ascii"),
            Language::CoffeeScript => include_str!("../resources/coffeescript.ascii"),
            Language::Cpp => include_str!("../resources/cpp.ascii"),
            Language::Csharp => include_str!("../resources/csharp.ascii"),
            Language::CSS => include_str!("../resources/css.ascii"),
            Language::D => include_str!("../resources/d.ascii"),
            Language::Dart => include_str!("../resources/dart.ascii"),
            Language::Elixir => include_str!("../resources/elixir.ascii"),
            Language::Elm => include_str!("../resources/elm.ascii"),
            Language::Erlang => include_str!("../resources/erlang.ascii"),
            Language::Forth => include_str!("../resources/forth.ascii"),
            Language::FortranModern => include_str!("../resources/f90.ascii"),
            Language::FSharp => include_str!("../resources/fsharp.ascii"),
            Language::Go => include_str!("../resources/go.ascii"),
            Language::Haskell => include_str!("../resources/haskell.ascii"),
            Language::HTML => include_str!("../resources/html.ascii"),
            Language::Idris => include_str!("../resources/idris.ascii"),
            Language::Java => include_str!("../resources/java.ascii"),
            Language::JavaScript => include_str!("../resources/javascript.ascii"),
            Language::Julia => include_str!("../resources/julia.ascii"),
            Language::Kotlin => include_str!("../resources/kotlin.ascii"),
            Language::Lisp => include_str!("../resources/lisp.ascii"),
            Language::Lua => include_str!("../resources/lua.ascii"),
            Language::Markdown => include_str!("../resources/markdown.ascii"),
            Language::Nim => include_str!("../resources/nim.ascii"),            
            Language::ObjectiveC => include_str!("../resources/objectivec.ascii"),
            Language::OCaml => include_str!("../resources/ocaml.ascii"),
            Language::Perl => include_str!("../resources/perl.ascii"),
            Language::Php => include_str!("../resources/php.ascii"),
            Language::Prolog => include_str!("../resources/prolog.ascii"),
            Language::PureScript => include_str!("../resources/purescript.ascii"),
            Language::Python => include_str!("../resources/python.ascii"),
            Language::R => include_str!("../resources/r.ascii"),
            Language::Racket => include_str!("../resources/racket.ascii"),
            Language::Ruby => include_str!("../resources/ruby.ascii"),
            Language::Rust => include_str!("../resources/rust.ascii"),
            Language::Scala => include_str!("../resources/scala.ascii"),
            Language::Shell => include_str!("../resources/shell.ascii"),
            Language::Swift => include_str!("../resources/swift.ascii"),
            Language::Tcl => include_str!("../resources/tcl.ascii"),
            Language::Tex => include_str!("../resources/tex.ascii"),
            Language::TypeScript => include_str!("../resources/typescript.ascii"),
            Language::Vue => include_str!("../resources/vue.ascii"),
            Language::XML => include_str!("../resources/xml.ascii"),
            Language::Zig => include_str!("../resources/zig.ascii"),
            Language::Unknown => include_str!("../resources/unknown.ascii"),
            // _ => include_str!("../resources/unknown.ascii"),
        }
    }

    pub fn get_colors(&self) -> Vec<Color> {
        match *self {
            Language::Assembly => vec![Color::Cyan],
            Language::C => vec![Color::BrightBlue, Color::Blue],
            Language::Clojure => vec![Color::BrightBlue, Color::BrightGreen],
            Language::CoffeeScript => vec![Color::Red],
            Language::Cpp => vec![Color::Yellow, Color::Cyan],
            Language::Csharp => vec![Color::White],
            Language::CSS => vec![Color::Blue, Color::White],
            Language::D => vec![Color::Red],
            Language::Dart => vec![Color::BrightBlue, Color::BrightCyan],
            Language::Elixir => vec![Color::Magenta],
            Language::Elm => vec![Color::BrightBlack, Color::Green, Color::Yellow, Color::Cyan],
            Language::Erlang => vec![Color::BrightRed],
            Language::Forth => vec![Color::BrightRed],
            Language::FortranModern => vec![
                Color::BrightWhite,
                Color::BrightGreen,
                Color::BrightBlue,
                Color::BrightYellow,
                Color::BrightRed
            ],
            Language::FSharp => vec![Color::BrightBlue, Color::Cyan],
            Language::Go => vec![Color::White],
            Language::Haskell => vec![Color::BrightBlue, Color::BrightMagenta, Color::Blue],
            Language::HTML => vec![Color::Red, Color::White],
            Language::Idris => vec![Color::Red],
            Language::Java => vec![Color::BrightBlue, Color::Red],
            Language::JavaScript => vec![Color::BrightYellow],
            Language::Julia => vec![
                Color::BrightWhite,
                Color::Blue,
                Color::BrightGreen,
                Color::Red,
                Color::BrightMagenta,
            ],
            Language::Kotlin => vec![Color::Blue, Color::Yellow, Color::Magenta],
            Language::Lisp => vec![Color::Yellow],
            Language::Lua => vec![Color::Blue],
            Language::Markdown => vec![Color::BrightWhite, Color::BrightRed],
            Language::Nim => vec![Color::Yellow, Color::BrightWhite],            
            Language::ObjectiveC => vec![Color::BrightBlue, Color::Blue],
            Language::OCaml => vec![Color::Yellow],
            Language::Perl => vec![Color::BrightBlue],
            Language::Php => vec![Color::BrightWhite],
            Language::Prolog => vec![Color::Blue, Color::Red],
            Language::PureScript => vec![Color::White],
            Language::Python => vec![Color::Blue, Color::Yellow],
            Language::R => vec![Color::White, Color::Blue],
            Language::Racket => vec![Color::Red, Color::White, Color::Blue],
            Language::Ruby => vec![Color::Magenta],
            Language::Rust => vec![Color::White, Color::BrightRed],
            Language::Scala => vec![Color::Blue],
            Language::Shell => vec![Color::Green],
            Language::Swift => vec![Color::BrightRed],
            Language::Tcl => vec![Color::Blue, Color::White, Color::BrightBlue],
            Language::Tex => vec![Color::White, Color::Black],
            Language::TypeScript => vec![Color::Cyan],
            Language::Vue => vec![Color::BrightGreen, Color::Blue],
            Language::XML => vec![Color::Yellow, Color::BrightBlack, Color::BrightGreen],
            Language::Zig => vec![Color::Yellow],
            Language::Unknown => vec![Color::White],
        }
    }

    fn get_languages_stat(languages: &tokei::Languages) -> Option<HashMap<Language, f64>> {
        let mut stats = HashMap::new();

        let sum_language_code: usize = languages.iter().map(|(_, v)| v.code).sum();

        if sum_language_code == 0 {
            None
        } else {
            for (k, v) in languages.iter() {
                let code = v.code as f64;
                stats.insert(
                    Language::from(*k),
                    (code / sum_language_code as f64) * 100.00,
                );
            }
            Some(stats)
        }
    }

    pub fn get_language_stats(dir: &str) -> Result<(Vec<(Language, f64)>, usize)> {
        let tokei_langs = project_languages(&dir);
        let languages_stat =
            Language::get_languages_stat(&tokei_langs).ok_or(Error::SourceCodeNotFound)?;
        let mut stat_vec: Vec<(_, _)> = languages_stat.into_iter().collect();
        stat_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());
        let loc = get_total_loc(&tokei_langs);
        Ok((stat_vec, loc))
    }

    pub fn get_dominant_language(languages_stat_vec: Vec<(Language, f64)>) -> Language {
        languages_stat_vec[0].0.clone()
    }
}

fn get_total_loc(languages: &tokei::Languages) -> usize {
    languages
        .values()
        .collect::<Vec<&tokei::Language>>()
        .iter()
        .fold(0, |sum, val| sum + val.code)
}

fn project_languages(dir: &str) -> tokei::Languages {
    use tokei::Config;

    let mut languages = tokei::Languages::new();
    let required_languages = get_all_language_types();
    let tokei_config = Config {
        types: Some(required_languages),
        ..Config::default()
    };
    languages.get_statistics(&[&dir], &[".git", "target"], &tokei_config);
    languages
}

fn get_all_language_types() -> Vec<tokei::LanguageType> {
    vec![
        tokei::LanguageType::Assembly,
        tokei::LanguageType::C,
        tokei::LanguageType::Clojure,
        tokei::LanguageType::CoffeeScript,
        tokei::LanguageType::Cpp,
        tokei::LanguageType::CSharp,
        tokei::LanguageType::Css,
        tokei::LanguageType::D,
        tokei::LanguageType::Dart,
        tokei::LanguageType::Elixir,
        tokei::LanguageType::Elm,
        tokei::LanguageType::Erlang,
        tokei::LanguageType::Forth,
        tokei::LanguageType::FortranModern,
        tokei::LanguageType::FSharp,
        tokei::LanguageType::Go,
        tokei::LanguageType::Haskell,
        tokei::LanguageType::Html,
        tokei::LanguageType::Idris,
        tokei::LanguageType::Java,
        tokei::LanguageType::JavaScript,
        tokei::LanguageType::Julia,
        tokei::LanguageType::Kotlin,
        tokei::LanguageType::Lisp,
        tokei::LanguageType::Lua,
        tokei::LanguageType::Markdown,
        tokei::LanguageType::Nim,
        tokei::LanguageType::ObjectiveC,
        tokei::LanguageType::OCaml,        
        tokei::LanguageType::Perl,
        tokei::LanguageType::Php,
        tokei::LanguageType::Prolog,
        tokei::LanguageType::PureScript,
        tokei::LanguageType::Python,
        tokei::LanguageType::R,
        tokei::LanguageType::Ruby,
        tokei::LanguageType::Rust,
        tokei::LanguageType::Scala,
        tokei::LanguageType::Sh,
        tokei::LanguageType::Swift,
        tokei::LanguageType::Tcl,
        tokei::LanguageType::Tex,
        tokei::LanguageType::TypeScript,
        tokei::LanguageType::Vue,
        tokei::LanguageType::Xml,
        tokei::LanguageType::Zig,
    ]
}
