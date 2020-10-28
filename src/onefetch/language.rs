use {
    crate::onefetch::error::*,
    colored::Color,
    regex::Regex,
    std::collections::HashMap,
    std::env,
    strum::{EnumIter, EnumString, IntoStaticStr},
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

            pub fn get_colors(&self, true_color: bool) -> Vec<Color> {
                let colors = match *self {
                    $( Language::$name => $colors, )*
                    Language::Unknown => define_colors!( [Color::White] ),
                };
                match colors.true_colors {
                  Some( true_colors ) if true_color => true_colors,
                  _ => colors.basic_colors,
                }
            }
        }

        fn get_all_language_types() -> Vec<tokei::LanguageType> {
            vec![ $( tokei::LanguageType::$name ,)* ]
        }

        #[cfg(test)]
        mod true_colors {

            use std::fmt;
            use super::*;
            use paste::paste;

            impl fmt::Display for Colors {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    let mut output = String::new();
                    output += "Colors {\n";
                    output += "    basic_colors: vec![\n";
                    for (color_index, bc) in self.basic_colors.iter().enumerate() {
                        output += &format!( "        Color::{:?}, // {}\n", bc, color_index );
                    }
                    output += "    ], \n";
                    if let Some(tcs) = &self.true_colors {
                        output += "    true_colors: vec![\n";
                        for (color_index,tc) in tcs.iter().enumerate() {
                            output += &format!( "        Color::{:?}, // {}\n", tc, color_index );
                        }
                    } else {
                        output += "    true_colors: None\n";
                    };
                    output += "    ], \n";
                    output += "}\n";
                    write!( f, "{}", output )
                }
            }

            $(
                paste! {
                    #[test]
                    fn [<$name:lower _basic_color_test>] () {
                        let colors = $colors;
                        for (color_index, bc) in colors.basic_colors.iter().enumerate() {
                            let color_str = &format!( "Color::{:?}", bc );
                            if let Color::TrueColor { .. } = bc {
                                panic!( "TrueColor found in basic_colors for {} at index {} found {}", stringify!( $name ), color_index, color_str );
                            }
                        }
                    }

                    #[test]
                    fn [<$name:lower _color_vector_length_test>] () {
                        let colors = $colors;
                        let bc_count = colors.basic_colors.len();
                        if let Some(tcs) = &colors.true_colors {
                            assert_eq!( bc_count, tcs.len(), " left (basic) color length do not match right (true) color length.\n{}", colors );
                        }
                    }
                }
            )*
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
    { Bash, "bash.ascii", "Bash", define_colors!( [Color::White] ), "bash" },
    { C, "c.ascii", "C", define_colors!( [Color::Cyan, Color::Blue] : [Color::TrueColor{r:93, g:108, b:191}, Color::TrueColor{r:41, g:54, b:147}] ) },
    { Clojure, "clojure.ascii", "Clojure", define_colors!( [Color::Cyan, Color::Green] ) },
    { CMake, "cmake.ascii", "CMake", define_colors!( [Color::Blue, Color::Green, Color::Red, Color::Black] ) },
    { CoffeeScript, "coffeescript.ascii", "CoffeeScript", define_colors!( [Color::Red] ) },
    { Cpp, "cpp.ascii", "C++", define_colors!( [Color::Cyan, Color::Blue] : [Color::TrueColor{r:100, g:154, b:210}, Color::TrueColor{r:0, g:68, b:130}]), "c++" },
    { Crystal, "crystal.ascii", "Crystal", define_colors!( [Color::White, Color::Black] ) },
    { CSharp, "csharp.ascii", "C#", define_colors!( [Color::Blue, Color::Magenta] : [Color::TrueColor{r:154, g:73, b:147}, Color::TrueColor{r:106, g:21, b:119}] ), "c#" },
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
    { Go, "go.ascii", "Go", define_colors!( [Color::Cyan, Color::White, Color::Yellow] : [Color::TrueColor{ r:116, g:205, b:221 }, Color::TrueColor{ r:255, g:255, b:255 }, Color::TrueColor{ r:246, g:210, b:162 }] ) },
    { Groovy, "groovy.ascii", "Groovy", define_colors!( [Color::Cyan, Color::White] ) },
    { Haskell, "haskell.ascii", "Haskell", define_colors!( [Color::Cyan, Color::Magenta, Color::Blue] ) },
    { Html, "html.ascii", "HTML", define_colors!( [Color::Red, Color::White] ) },
    { Idris, "idris.ascii", "Idris", define_colors!( [Color::Red] ) },
    { Java, "java.ascii", "Java", define_colors!( [Color::Cyan, Color::Red] ) },
    { JavaScript, "javascript.ascii", "JavaScript", define_colors!( [Color::Yellow] : [Color::TrueColor{ r:236, g:230 ,b:83} ]) },
    { Jsx, "jsx.ascii", "JSX", define_colors!( [Color::Yellow] ) },
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
    { Rust, "rust.ascii", "Rust", define_colors!( [Color::Red, Color::White] : [Color::TrueColor{ r:228, g:55 ,b:23}, Color::TrueColor{ r:255, g:255 ,b:255} ] ) },
    { Scala, "scala.ascii", "Scala", define_colors!( [Color::Red, Color::Red] : [Color::TrueColor{ r:223, g:63 ,b:61}, Color::TrueColor{ r:127, g:12 ,b:29} ] ) },
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
            Color::TrueColor{ r:248, g:129, b:52 },
            Color::TrueColor{ r:249, g:119, b:50 },
            Color::TrueColor{ r:249, g:109, b:48 },
            Color::TrueColor{ r:250, g:99, b:46 },
            Color::TrueColor{ r:250, g:89, b:44 },
            Color::TrueColor{ r:251, g:80, b:42 },
            Color::TrueColor{ r:251, g:70, b:40 },
            Color::TrueColor{ r:252, g:60, b:38 },
            Color::TrueColor{ r:252, g:50, b:36 },
            Color::TrueColor{ r:253, g:40, b:34 }
        ] )
    },
    { Tcl, "tcl.ascii", "Tcl", define_colors!( [Color::Blue, Color::White, Color::Cyan] ) },
    { Tex, "tex.ascii", "Tex", define_colors!( [Color::White, Color::Black] ) },
    { Tsx, "tsx.ascii", "TSX", define_colors!( [Color::Blue] ) },
    { TypeScript, "typescript.ascii", "TypeScript", define_colors!( [Color::Cyan] : [Color::TrueColor{ r:0, g:122, b:204} ]) },
    { Vue, "vue.ascii", "Vue", define_colors!( [Color::Green, Color::Blue] ) },
    { Xml, "xml.ascii", "XML", define_colors!( [Color::Yellow, Color::White, Color::Green] ) },
    { Zig, "zig.ascii", "Zig", define_colors!( [Color::Yellow] ) },
    { Zsh, "zsh.ascii", "Zsh", define_colors!( [Color::White] ) },
}

impl Language {
    pub fn get_dominant_language(languages_stat_vec: &[(Language, f64)]) -> Language {
        languages_stat_vec[0].0.clone()
    }

    pub fn get_language_statistics(
        dir: &str,
        ignored_directories: &[String],
    ) -> Result<(Vec<(Language, f64)>, usize)> {
        let stats = Language::get_statistics(&dir, ignored_directories);
        let language_distribution = Language::get_language_distribution(&stats)
            .ok_or("Could not find any source code in this directory")?;
        let mut language_distribution_vec: Vec<(_, _)> =
            language_distribution.into_iter().collect();
        language_distribution_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());
        let loc = Language::get_total_loc(&stats);
        Ok((language_distribution_vec, loc))
    }

    fn get_language_distribution(languages: &tokei::Languages) -> Option<HashMap<Language, f64>> {
        let mut language_distribution = HashMap::new();

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

            language_distribution.insert(Language::from(*language_type), code as f64);
        }

        let total: f64 = language_distribution.iter().map(|(_, v)| v).sum();

        if total.abs() < f64::EPSILON {
            None
        } else {
            for (_, val) in language_distribution.iter_mut() {
                *val /= total;
                *val *= 100_f64;
            }

            Some(language_distribution)
        }
    }

    fn get_total_loc(languages: &tokei::Languages) -> usize {
        languages
            .values()
            .collect::<Vec<&tokei::Language>>()
            .iter()
            .fold(0, |sum, val| sum + val.code)
    }

    fn get_statistics(dir: &str, ignored_directories: &[String]) -> tokei::Languages {
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
            let ignored_directories_ref: Vec<&str> =
                ignored_directories.iter().map(|s| &**s).collect();
            languages.get_statistics(&[&dir], &ignored_directories_ref, &tokei_config);
        }

        languages
    }
}
