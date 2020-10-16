use {
    crate::onefetch::error::*,
    colored::Color,
    regex::Regex,
    std::collections::HashMap,
    strum::{EnumIter, EnumString, IntoStaticStr},
    std::env,
};

pub struct Colors {
    basic_colors: Vec<Color>,
    true_colors: Option<Vec<Color>>,
}

macro_rules! define_colors {
    ( [ $($color:expr),+ ] ) => { Colors { basic_colors: vec![$($color),+], true_colors: None } };
    ( [ $($bc:expr),+ ] : [ $($tc:expr),+ ] ) => { Colors { basic_colors: vec![$($bc),+], true_colors: Some(vec![$($tc),+]) } };
    (   $color:expr ) => { $color };
}

macro_rules! define_languages {
    ($( { $name:ident, $ascii:literal, $display:literal, $colors:expr $(, $serialize:literal )? } ),* ,) => {

        #[strum(serialize_all = "lowercase")]
        #[derive(PartialEq, Eq, Hash, Clone, EnumString, EnumIter, IntoStaticStr)]
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
                    $( Language::$name => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/", $ascii)), )*
                    Language::Unknown => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/unknown.ascii")),
                }
            }

            pub fn get_colors(&self) -> Vec<Color> {
                let colors = match *self {
                    $( Language::$name => $colors, )*
                    Language::Unknown => define_colors!( [Color::White] ),
                };
                let use_true_color = env::var("COLORTERM");
                if use_true_color.is_ok() && use_true_color.unwrap().to_lowercase() == "truecolor" && colors.true_colors.is_some() {
                    colors.true_colors.unwrap()
                } else {
                    colors.basic_colors
                }
            }
        }

        fn get_all_language_types() -> Vec<tokei::LanguageType> {
            vec![ $( tokei::LanguageType::$name ,)* ]
        }

        #[cfg(test)]
        mod ascii_size {
            use lazy_static::lazy_static;
            use more_asserts::assert_le;
            use paste::paste;
            use regex::Regex;

            const MAX_WIDTH: usize = 40;
            const MAX_HEIGHT: usize = 25;

            lazy_static! {
                static ref COLOR_INTERPOLATION: Regex = Regex::new(r"\{\d+\}").unwrap();
            }

            $(
                paste! {
                    #[test]
                    #[ignore]
                    fn [<$name:lower _width>] () {
                        const ASCII: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/", $ascii));

                        for (line_number, line) in ASCII.lines().enumerate() {
                            let line = COLOR_INTERPOLATION.replace_all(line, "");
                            if (line.trim().len() > MAX_WIDTH) {
                                panic!("{} is too wide at line {}\n{:?}", $ascii, line_number + 1, line)
                            }
                        }
                    }

                    #[test]
                    #[ignore]
                    fn [<$name:lower _height>] () {
                        const ASCII: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/", $ascii));
                        assert_le!(ASCII.lines().count(), MAX_HEIGHT, concat!($ascii, " is too tall."));
                    }
                }
            )*
        }
    };
}

define_languages! {
    { Assembly, "assembly.ascii", "Assembly", define_colors!( [Color::Cyan] ) },
    { C, "c.ascii", "C", define_colors!( [Color::Cyan, Color::Blue] ) },
    { Clojure, "clojure.ascii", "Clojure", define_colors!( [Color::Cyan, Color::Green] ) },
    { CMake, "cmake.ascii", "CMake", define_colors!( [Color::Blue, Color::Green, Color::Red, Color::Black] ) },
    { CoffeeScript, "coffeescript.ascii", "CoffeeScript", define_colors!( [Color::Red] ) },
    { Cpp, "cpp.ascii", "C++", define_colors!( [Color::Cyan, Color::Blue] ), "c++" },
    { Crystal, "crystal.ascii", "Crystal", define_colors!( [Color::White, Color::Black] ) },
    { CSharp, "csharp.ascii", "C#", define_colors!( [Color::Blue, Color::Magenta] ), "c#" },
    { Css, "css.ascii", "CSS", define_colors!( [Color::Blue, Color::White] ) },
    { D, "d.ascii", "D", define_colors!( [Color::Red] ) },
    { Dart, "dart.ascii", "Dart", define_colors!( [Color::Cyan, Color::Blue] ) },
    { Dockerfile, "dockerfile.ascii", "Dockerfile", define_colors!( [Color::Cyan, Color::White, Color::Cyan] ) },
    { Elisp, "emacslisp.ascii", "EmacsLisp", define_colors!( [Color::Magenta, Color::White] ), "emacslisp" },
    { Elixir, "elixir.ascii", "Elixir", define_colors!( [Color::Magenta] ) },
    { Elm, "elm.ascii", "Elm", define_colors!( [Color::Blue, Color::Green, Color::Yellow, Color::Cyan] ) },
    { Erlang, "erlang.ascii", "Erlang", define_colors!( [Color::Red] ) },
    { Fish, "fish.ascii", "Fish", define_colors!( [Color::Red, Color::Yellow] ) },
    { Forth, "forth.ascii", "Forth", define_colors!( [Color::Red] ) },
    { FortranModern, "f90.ascii", "Fortran", define_colors!( [Color::White, Color::Green, Color::Cyan, Color::Yellow, Color::Red] ), "fortran" },
    { FSharp, "fsharp.ascii", "F#", define_colors!( [Color::Cyan, Color::Cyan] ), "f#" },
    { Go, "go.ascii", "Go", define_colors!( [Color::White] ) },
    { Groovy, "groovy.ascii", "Groovy", define_colors!( [Color::Cyan, Color::White] ) },
    { Haskell, "haskell.ascii", "Haskell", define_colors!( [Color::Cyan, Color::Magenta, Color::Blue] ) },
    { Html, "html.ascii", "HTML", define_colors!( [Color::Red, Color::White] ) },
    { Idris, "idris.ascii", "Idris", define_colors!( [Color::Red] ) },
    { Java, "java.ascii", "Java", define_colors!( [Color::Cyan, Color::Red] ) },
    { JavaScript, "javascript.ascii", "JavaScript", define_colors!( [Color::Yellow] ) },
    { Julia, "julia.ascii", "Julia", define_colors!( [Color::White, Color::Blue, Color::Green, Color::Red, Color::Magenta] ) },
    { Jupyter, "jupyter.ascii", "Jupyter-Notebooks", define_colors!( [Color::White, Color::Yellow, Color::White] ), "jupyter-notebooks" },
    { Kotlin, "kotlin.ascii", "Kotlin", define_colors!( [Color::Blue, Color::Yellow, Color::Magenta] ) },
    { Lisp, "lisp.ascii", "Lisp", define_colors!( [Color::White] ) },
    { Lua, "lua.ascii", "Lua", define_colors!( [Color::Blue, Color::White] ) },
    { Markdown, "markdown.ascii", "Markdown", define_colors!( [Color::White, Color::Red] ) },
    { Nim, "nim.ascii", "Nim", define_colors!( [Color::Yellow, Color::White] ) },
    { Nix, "nix.ascii", "Nix", define_colors!( [Color::Cyan, Color::Blue] ) },
    { ObjectiveC, "objectivec.ascii", "Objective-C", define_colors!( [Color::Cyan, Color::Blue] ), "objective-c" },
    { OCaml, "ocaml.ascii", "OCaml", define_colors!( [Color::Yellow] ) },
    { Org, "org.ascii", "Org", define_colors!( [Color::Green, Color::Red, Color::White] ) },
    { Perl, "perl.ascii", "Perl", define_colors!( [Color::Cyan] ) },
    { Php, "php.ascii", "Php", define_colors!( [Color::Magenta, Color::Blue, Color::Cyan, Color::White] ) },
    { Prolog, "prolog.ascii", "Prolog", define_colors!( [Color::White] ) },
    { PureScript, "purescript.ascii", "PureScript", define_colors!( [Color::White] ) },
    { Python, "python.ascii", "Python", define_colors!( [Color::Blue, Color::Yellow] ) },
    { R, "r.ascii", "R", define_colors!( [Color::White, Color::Blue] ) },
    { Racket, "racket.ascii", "Racket", define_colors!( [Color::Red, Color::White, Color::Blue] ) },
    { Ruby, "ruby.ascii", "Ruby", define_colors!( [Color::Magenta] ) },
    { Rust, "rust.ascii", "Rust", define_colors!( [Color::Red, Color::White] ) },
    { Scala, "scala.ascii", "Scala", define_colors!( [Color::Blue] ) },
    { Sh, "shell.ascii", "Shell", define_colors!( [Color::Green] ), "shell" },
    {
        Swift, "swift.ascii", "Swift", define_colors!( [
            Color::Red,
            Color::Red,
            Color::Red,
            Color::Red,
            Color::Red,
            Color::Red,
            Color::Red,
            Color::Red,
            Color::Red,
            Color::Red
        ] : [
            Color::TrueColor{ r:248, g:129, b:052 },
            Color::TrueColor{ r:249, g:119, b:050 },
            Color::TrueColor{ r:249, g:109, b:048 },
            Color::TrueColor{ r:250, g:099, b:046 },
            Color::TrueColor{ r:250, g:089, b:044 },
            Color::TrueColor{ r:251, g:080, b:042 },
            Color::TrueColor{ r:251, g:070, b:040 },
            Color::TrueColor{ r:252, g:060, b:038 },
            Color::TrueColor{ r:252, g:050, b:036 },
            Color::TrueColor{ r:253, g:040, b:034 }
        ] )
    },
    { Tcl, "tcl.ascii", "Tcl", define_colors!( [Color::Blue, Color::White, Color::Cyan] ) },
    { Tex, "tex.ascii", "Tex", define_colors!( [Color::White, Color::Black] ) },
    { TypeScript, "typescript.ascii", "TypeScript", define_colors!( [Color::Cyan] ) },
    { Vue, "vue.ascii", "Vue", define_colors!( [Color::Green, Color::Blue] ) },
    { Xml, "xml.ascii", "XML", define_colors!( [Color::Yellow, Color::White, Color::Green] ) },
    { Zig, "zig.ascii", "Zig", define_colors!( [Color::Yellow] ) },
}

impl Language {
    fn get_languages_stat(languages: &tokei::Languages) -> Option<HashMap<Language, f64>> {
        let mut stats = HashMap::new();

        for (language_type, language) in languages.iter() {
            let mut code = language.code;

            let has_children = !language.children.is_empty();

            if has_children {
                for reports in language.children.values() {
                    for stats in reports.iter().map(|r| r.stats.summarise()) {
                        code += stats.code;
                    }
                }
            }

            if code == 0 {
                continue;
            }

            stats.insert(Language::from(*language_type), code as f64);
        }

        let total: f64 = stats.iter().map(|(_, v)| v).sum();

        if total.abs() < f64::EPSILON {
            None
        } else {
            for (_, val) in stats.iter_mut() {
                *val /= total;
                *val *= 100_f64;
            }

            Some(stats)
        }
    }

    pub fn get_language_stats(
        dir: &str,
        ignored_directories: &[String],
    ) -> Result<(Vec<(Language, f64)>, usize)> {
        let tokei_langs = project_languages(&dir, ignored_directories);
        let languages_stat = Language::get_languages_stat(&tokei_langs)
            .ok_or("Could not find any source code in this directory")?;
        let mut stat_vec: Vec<(_, _)> = languages_stat.into_iter().collect();
        stat_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());
        let loc = get_total_loc(&tokei_langs);
        Ok((stat_vec, loc))
    }

    pub async fn get_dominant_language(languages_stat_vec: &[(Language, f64)]) -> Language {
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

fn project_languages(dir: &str, ignored_directories: &[String]) -> tokei::Languages {
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
            if re.is_match(&ignored) {
                let p = if ignored.starts_with('/') {
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
        let ignored_directories_ref: Vec<&str> = ignored_directories.iter().map(|s| &**s).collect();
        languages.get_statistics(&[&dir], &ignored_directories_ref, &tokei_config);
    }

    languages
}
