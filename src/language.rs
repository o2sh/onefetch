use {
    crate::{Error, Result},
    colored::Color,
    regex::Regex,
    std::collections::HashMap,
    strum::{EnumIter, EnumString},
};

macro_rules! define_languages {
    ($( { $name:ident, $ascii:literal, $display:literal $(, $serialize:literal )? } ),* ,) => {

        #[strum(serialize_all = "lowercase")]
        #[derive(PartialEq, Eq, Hash, Clone, EnumString, EnumIter)]
        pub enum Language {
            $(
                $( #[strum(serialize = $serialize)] )?
                $name ,
            )*
            Unknown,
        }

        impl std::fmt::Display for Language {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match *self {
                    $( Language::$name => write!(f, $display), )*
                    Language::Unknown => write!(f, "Unknown" ),
                }
            }
        }

        impl From<tokei::LanguageType> for Language {
            fn from(language: tokei::LanguageType) -> Self {
                match language {
                    $( tokei::LanguageType::$name => Language::$name, )*
                        _ => unimplemented!("Language {:?}", language),
                }
            }
        }

        impl Language {
            pub fn get_ascii_art(&self) -> &str {
                match *self {
                    $( Language::$name => include_str!(concat!("../resources/", $ascii)), )*
                    Language::Unknown => include_str!("../resources/unknown.ascii"),
                }
            }
        }
    };
}

define_languages!{
    { Assembly, "assembly.ascii", "Assembly" },
    { C, "c.ascii", "C" },
    { Clojure, "clojure.ascii", "Clojure" },
    { CMake, "cmake.ascii", "CMake" },
    { CoffeeScript, "coffeescript.ascii", "CoffeeScript" },
    { Cpp, "cpp.ascii", "C++", "c++" },
    { CSharp, "csharp.ascii", "C#", "c#" },
    { Css, "css.ascii", "CSS" },
    { D, "d.ascii", "D" },
    { Dart, "dart.ascii", "Dart" },
    { Dockerfile, "dockerfile.ascii", "Dockerfile" },
    { Elisp, "emacslisp.ascii", "EmacsLisp", "emacslisp" },
    { Elixir, "elixir.ascii", "Elixir" },
    { Elm, "elm.ascii", "Elm" },
    { Erlang, "erlang.ascii", "Erlang" },
    { Fish, "fish.ascii", "Fish" },
    { Forth, "forth.ascii", "Forth" },
    { FortranModern, "f90.ascii", "Fortran", "fortran" },
    { FSharp, "fsharp.ascii", "FSharp" },
    { Go, "go.ascii", "Go" },
    { Groovy, "groovy.ascii", "Groovy" },
    { Haskell, "haskell.ascii", "Haskell" },
    { Html, "html.ascii", "HTML" },
    { Idris, "idris.ascii", "Idris" },
    { Java, "java.ascii", "Java" },
    { JavaScript, "javascript.ascii", "JavaScript" },
    { Julia, "julia.ascii", "Julia" },
    { Jupyter, "jupyter.ascii", "Jupyter-Notebooks", "jupyter-notebooks" },
    { Kotlin, "kotlin.ascii", "Kotlin" },
    { Lisp, "lisp.ascii", "Lisp" },
    { Lua, "lua.ascii", "Lua" },
    { Markdown, "markdown.ascii", "Markdown" },
    { Nim, "nim.ascii", "Nim" },
    { Nix, "nix.ascii", "Nix" },
    { ObjectiveC, "objectivec.ascii", "Objective-C", "objective-c" },
    { OCaml, "ocaml.ascii", "OCaml" },
    { Org, "org.ascii", "Org" },
    { Perl, "perl.ascii", "Perl" },
    { Php, "php.ascii", "Php" },
    { Prolog, "prolog.ascii", "Prolog" },
    { PureScript, "purescript.ascii", "PureScript" },
    { Python, "python.ascii", "Python" },
    { R, "r.ascii", "R" },
    { Racket, "racket.ascii", "Racket" },
    { Ruby, "ruby.ascii", "Ruby" },
    { Rust, "rust.ascii", "Rust" },
    { Scala, "scala.ascii", "Scala" },
    { Sh, "shell.ascii", "Shell", "shell" },
    { Swift, "swift.ascii", "Swift" },
    { Tcl, "tcl.ascii", "Tcl" },
    { Tex, "tex.ascii", "Tex" },
    { TypeScript, "typescript.ascii", "TypeScript" },
    { Vue, "vue.ascii", "Vue" },
    { Xml, "xml.ascii", "XML" },
    { Zig, "zig.ascii", "Zig" },
}

impl Language {
    pub fn get_colors(&self) -> Vec<Color> {
        match *self {
            Language::Assembly => vec![Color::Cyan],
            Language::C => vec![Color::Cyan, Color::Blue],
            Language::Clojure => vec![Color::Cyan, Color::Green],
            Language::CMake => vec![Color::Blue, Color::Green, Color::Red, Color::Black],
            Language::CoffeeScript => vec![Color::Red],
            Language::Cpp => vec![Color::Cyan, Color::Blue],
            Language::CSharp => vec![Color::Blue, Color::Magenta],
            Language::Css => vec![Color::Blue, Color::White],
            Language::D => vec![Color::Red],
            Language::Dart => vec![Color::Cyan, Color::Blue],
            Language::Dockerfile => vec![Color::Cyan, Color::White, Color::Cyan],
            Language::Elisp => vec![Color::Magenta, Color::White],
            Language::Elixir => vec![Color::Magenta],
            Language::Elm => vec![Color::Black, Color::Green, Color::Yellow, Color::Cyan],
            Language::Erlang => vec![Color::Red],
            Language::Fish => vec![Color::Red, Color::Yellow],
            Language::Forth => vec![Color::Red],
            Language::FortranModern => vec![
                Color::White,
                Color::Green,
                Color::Cyan,
                Color::Yellow,
                Color::Red,
            ],
            Language::FSharp => vec![Color::Cyan, Color::Cyan],
            Language::Go => vec![Color::White],
            Language::Groovy => vec![Color::Cyan, Color::White],
            Language::Haskell => vec![Color::Cyan, Color::Magenta, Color::Blue],
            Language::Html => vec![Color::Red, Color::White],
            Language::Idris => vec![Color::Red],
            Language::Java => vec![Color::Cyan, Color::Red],
            Language::JavaScript => vec![Color::Yellow],
            Language::Julia => vec![
                Color::White,
                Color::Blue,
                Color::Green,
                Color::Red,
                Color::Magenta,
            ],
            Language::Jupyter => vec![Color::White, Color::Yellow, Color::White],
            Language::Kotlin => vec![Color::Blue, Color::Yellow, Color::Magenta],
            Language::Lisp => vec![Color::Yellow],
            Language::Lua => vec![Color::Blue],
            Language::Markdown => vec![Color::White, Color::Red],
            Language::Nim => vec![Color::Yellow, Color::White],
            Language::Nix => vec![Color::Cyan, Color::Blue],
            Language::ObjectiveC => vec![Color::Cyan, Color::Blue],
            Language::OCaml => vec![Color::Yellow],
            Language::Org => vec![Color::Green, Color::Red, Color::White],
            Language::Perl => vec![Color::Cyan],
            Language::Php => vec![Color::Magenta, Color::Black],
            Language::Prolog => vec![Color::Blue, Color::Red],
            Language::PureScript => vec![Color::White],
            Language::Python => vec![Color::Blue, Color::Yellow],
            Language::R => vec![Color::White, Color::Blue],
            Language::Racket => vec![Color::Red, Color::White, Color::Blue],
            Language::Ruby => vec![Color::Magenta],
            Language::Rust => vec![Color::White, Color::Red],
            Language::Scala => vec![Color::Blue],
            Language::Sh => vec![Color::Green],
            Language::Swift => vec![Color::Red],
            Language::Tcl => vec![Color::Blue, Color::White, Color::Cyan],
            Language::Tex => vec![Color::White, Color::Black],
            Language::TypeScript => vec![Color::Cyan],
            Language::Vue => vec![Color::Green, Color::Blue],
            Language::Xml => vec![Color::Yellow, Color::White, Color::Green],
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

    pub fn get_language_stats(
        dir: &str,
        ignored_directories: Vec<&str>,
    ) -> Result<(Vec<(Language, f64)>, usize)> {
        let tokei_langs = project_languages(&dir, ignored_directories);
        let languages_stat =
            Language::get_languages_stat(&tokei_langs).ok_or(Error::SourceCodeNotFound)?;
        let mut stat_vec: Vec<(_, _)> = languages_stat.into_iter().collect();
        stat_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());
        let loc = get_total_loc(&tokei_langs);
        Ok((stat_vec, loc))
    }

    pub async fn get_dominant_language(languages_stat_vec: Vec<(Language, f64)>) -> Language {
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

fn project_languages(dir: &str, ignored_directories: Vec<&str>) -> tokei::Languages {
    use tokei::Config;

    let mut languages = tokei::Languages::new();
    let required_languages = get_all_language_types();
    let tokei_config = Config {
        types: Some(required_languages),
        ..Config::default()
    };

    if !ignored_directories.is_empty() {
        let re = Regex::new(r"((.*)+/)+(.*)").unwrap();
        let mut v = Vec::with_capacity(ignored_directories.len());
        for ignored in ignored_directories {
            if re.is_match(ignored) {
                let p = if ignored.starts_with("/") {
                    "**"
                } else {
                    "**/"
                };
                v.push(format!("{}{}", p, ignored));
            } else {
                v.push(String::from(ignored));
            }
        }
        let ignored_directories_for_ab: Vec<&str> = v.iter().map(|x| &**x).collect();
        languages.get_statistics(&[&dir], &ignored_directories_for_ab, &tokei_config);
    } else {
        languages.get_statistics(&[&dir], &ignored_directories, &tokei_config);
    }

    languages
}

fn get_all_language_types() -> Vec<tokei::LanguageType> {
    vec![
        tokei::LanguageType::Assembly,
        tokei::LanguageType::C,
        tokei::LanguageType::Clojure,
        tokei::LanguageType::CMake,
        tokei::LanguageType::CoffeeScript,
        tokei::LanguageType::Cpp,
        tokei::LanguageType::CSharp,
        tokei::LanguageType::Css,
        tokei::LanguageType::D,
        tokei::LanguageType::Dart,
        tokei::LanguageType::Dockerfile,
        tokei::LanguageType::Elixir,
        tokei::LanguageType::Elisp,
        tokei::LanguageType::Elm,
        tokei::LanguageType::Erlang,
        tokei::LanguageType::Fish,
        tokei::LanguageType::Forth,
        tokei::LanguageType::FortranModern,
        tokei::LanguageType::FSharp,
        tokei::LanguageType::Go,
        tokei::LanguageType::Groovy,
        tokei::LanguageType::Haskell,
        tokei::LanguageType::Html,
        tokei::LanguageType::Idris,
        tokei::LanguageType::Java,
        tokei::LanguageType::JavaScript,
        tokei::LanguageType::Julia,
        tokei::LanguageType::Jupyter,
        tokei::LanguageType::Kotlin,
        tokei::LanguageType::Lisp,
        tokei::LanguageType::Lua,
        tokei::LanguageType::Markdown,
        tokei::LanguageType::Nim,
        tokei::LanguageType::Nix,
        tokei::LanguageType::ObjectiveC,
        tokei::LanguageType::OCaml,
        tokei::LanguageType::Org,
        tokei::LanguageType::Perl,
        tokei::LanguageType::Php,
        tokei::LanguageType::Prolog,
        tokei::LanguageType::PureScript,
        tokei::LanguageType::Python,
        tokei::LanguageType::R,
        tokei::LanguageType::Racket,
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
