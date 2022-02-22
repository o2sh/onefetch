use colored::Color;
use serde::Serialize;
use std::env;
use strum::{EnumIter, EnumString, IntoStaticStr};

pub struct Colors {
    basic_colors: Vec<Color>,
    true_colors: Option<Vec<Color>>,
}

macro_rules! define_colors {
    ( [ $($color:expr),+ ] ) => { Colors { basic_colors: vec![$($color),+], true_colors: None } };
    ( [ $($bc:expr),+ ] : [ $($tc:expr),+ ] ) => { Colors { basic_colors: vec![$($bc),+], true_colors: Some(vec![$($tc),+]) } };
    (   $color:expr ) => { $color };
}

#[derive(PartialEq, EnumString, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
pub enum LanguageType {
    Programming,
    Markup,
    Prose,
    Data,
}

macro_rules! define_languages {
    ($( { $name:ident, $type:ident, $ascii:literal, $colors:expr $(, $serialize:literal )? } ),* ,) => {

        #[derive(PartialEq, Eq, Hash, Clone, EnumString, EnumIter, IntoStaticStr, Serialize)]
        #[strum(serialize_all = "lowercase")]
        #[allow(clippy::upper_case_acronyms)]
        pub enum Language {
            $(
                $( #[strum(serialize = $serialize)] )?
                $name,
            )*
        }

        impl std::fmt::Display for Language {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match *self {
                    $( Language::$name => {
                        write!(f, "{}", tokei::LanguageType::$name.name())
                    }, )*
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

        impl From<Language> for tokei::LanguageType {
            fn from(language: Language) -> Self {
                match language {
                    $( Language::$name => tokei::LanguageType::$name, )*
                }
            }
        }

        impl Language {
            pub fn get_ascii_art(&self) -> &str {
                match *self {
                    $( Language::$name => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/", $ascii)), )*
                }
            }

            pub fn get_colors(&self, true_color: bool) -> Vec<Color> {
                let colors = match *self {
                    $( Language::$name => $colors, )*
                };
                match colors.true_colors {
                  Some( true_colors ) if true_color => true_colors,
                  _ => colors.basic_colors,
                }
            }

            pub fn get_type(&self) -> LanguageType {
                match *self {
                    $( Language::$name => LanguageType::$type, )*
                }
            }
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
            use crate::ui::ascii_art::get_min_start_max_end;
            use more_asserts::assert_le;
            use paste::paste;

            const MAX_WIDTH: usize = 40;
            const MAX_HEIGHT: usize = 25;

            $(
                paste! {
                    #[test]
                    fn [<$name:lower _width>] () {
                        const ASCII: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/", $ascii));
                        let lines: Vec<_> = ASCII.lines().skip_while(|line| line.is_empty()).collect();
                        let (start, end) = get_min_start_max_end(&lines);
                        assert!(start <= end);
                        assert_le!(end - start, MAX_WIDTH, concat!($ascii, " is too wide."));
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
    { Ada, Programming, "ada.ascii", define_colors!( [Color::White, Color::Cyan, Color::Blue] : [Color::TrueColor{r:255, g:255, b:255}, Color::TrueColor{r:0, g:24, b:201}, Color::TrueColor{r:12, g:10, b:124}] ) },
    { Assembly, Programming, "assembly.ascii", define_colors!( [Color::Cyan] ) },
    { AutoHotKey, Programming, "autohotkey.ascii", define_colors!( [Color::White, Color::Green] : [Color::TrueColor{r:255, g:255, b:255}, Color::TrueColor{r: 0x11, g: 0x98, b: 0x10}]) },
    { Bash, Programming, "bash.ascii", define_colors!( [Color::White] ) },
    { C, Programming, "c.ascii", define_colors!( [Color::Cyan, Color::Blue, Color::Blue, Color::White] : [Color::TrueColor{r:93, g:108, b:191}, Color::TrueColor{r:41, g:54, b:147}, Color::TrueColor{r:57, g:73, b:170}, Color::TrueColor{r:255, g:255, b:255}] ) },
    { Clojure, Programming, "clojure.ascii", define_colors!( [Color::Cyan, Color::Green] ) },
    { CMake, Programming, "cmake.ascii", define_colors!( [Color::Blue, Color::Green, Color::Red, Color::Black] ) },
    { CoffeeScript, Programming, "coffeescript.ascii", define_colors!( [Color::Red] ) },
    { Coq, Programming, "coq.ascii", define_colors!( [Color::Yellow, Color::White] : [Color::TrueColor {r:191, g:140, b:94}, Color::TrueColor {r:213, g:190, b:153}] ) },
    { Cpp, Programming, "cpp.ascii", define_colors!( [Color::Cyan, Color::Blue, Color::Blue, Color::White] : [Color::TrueColor{r:100, g:154, b:210}, Color::TrueColor{r:0, g:66, b:131}, Color::TrueColor{r:0, g:89, b:157}, Color::TrueColor{r:255, g:255, b:255}] ), "c++" },
    { Crystal, Programming, "crystal.ascii", define_colors!( [Color::White, Color::Black] ) },
    { CSharp, Programming, "csharp.ascii", define_colors!( [Color::Blue, Color::Magenta, Color::Magenta, Color::White] : [Color::TrueColor{r:155, g:79, b:151}, Color::TrueColor{r:103, g:33, b:122}, Color::TrueColor{r:128, g:55, b:136}, Color::TrueColor{r:255, g:255, b:255}] ), "c#" },
    { Css, Markup, "css.ascii", define_colors!( [Color::Blue, Color::White] ) },
    { D, Programming, "d.ascii", define_colors!( [Color::Red] ) },
    { Dart, Programming, "dart.ascii", define_colors!( [Color::Blue, Color::Cyan, Color::Blue ] : [Color::TrueColor{ r:0, g:163, b:231 }, Color::TrueColor{ r:66, g:223, b:205 }, Color::TrueColor{ r:1, g:89, b:125 }] ) },
    { Dockerfile, Programming, "dockerfile.ascii", define_colors!( [Color::Cyan, Color::White, Color::Cyan] ) },
    { Elisp, Programming, "emacslisp.ascii", define_colors!( [Color::Magenta, Color::White] ), "emacs-lisp" },
    { Elixir, Programming, "elixir.ascii", define_colors!( [Color::Magenta] ) },
    { Elm, Programming, "elm.ascii", define_colors!( [Color::Blue, Color::Green, Color::Yellow, Color::Cyan] ) },
    { Emojicode, Programming, "emojicode.ascii", define_colors!( [Color::Green, Color::Magenta,  Color::Magenta, Color::Magenta] : [Color::TrueColor{r:119, g:178, b:85}, Color::TrueColor{r:146, g:102, b:204}, Color::TrueColor{r:170, g:141, b:216}, Color::TrueColor{r:116, g:78, b:170}] ) },
    { Erlang, Programming, "erlang.ascii", define_colors!( [Color::Red] ) },
    { Fish, Programming, "fish.ascii", define_colors!( [Color::Red, Color::Yellow] ) },
    { Forth, Programming, "forth.ascii", define_colors!( [Color::Red] ) },
    { FortranModern, Programming, "f90.ascii", define_colors!( [Color::White, Color::Green, Color::Cyan, Color::Yellow, Color::Red] ), "fortran" },
    { FortranLegacy, Programming, "f77.ascii", define_colors!( [Color::White, Color::Green, Color::Cyan, Color::Yellow, Color::Red] ), "fortran-legacy" },
    { FSharp, Programming, "fsharp.ascii", define_colors!( [Color::Cyan, Color::Cyan] ), "f#" },
    { GdScript, Programming, "gdscript.ascii", define_colors!( [Color::Cyan, Color::White] : [Color::TrueColor{ r:69, g:141, b:192 }, Color::TrueColor{ r:255, g:255, b:255}] ) },
    { Go, Programming, "go.ascii", define_colors!( [Color::Cyan, Color::White, Color::Yellow] : [Color::TrueColor{ r:116, g:205, b:221 }, Color::TrueColor{ r:255, g:255, b:255 }, Color::TrueColor{ r:246, g:210, b:162 }] ) },
    { Graphql, Data, "graphql.ascii", define_colors!( [Color::Magenta] ) },
    { Groovy, Programming, "groovy.ascii", define_colors!( [Color::Cyan, Color::White] ) },
    { Haskell, Programming, "haskell.ascii", define_colors!( [Color::Cyan, Color::Magenta, Color::Blue] : [Color::TrueColor{ r:69, g:58, b:98 }, Color::TrueColor{ r:94, g:80, b:134 }, Color::TrueColor{ r:143, g:78, b:139 }] ) },
    { Haxe, Programming, "haxe.ascii", define_colors!( [Color::Yellow, Color::Yellow, Color::Yellow] : [Color::TrueColor{ r: 250, g: 178, b: 11 }, Color::TrueColor{ r:246, g:153, b:18 }, Color::TrueColor{ r: 244, g: 114, b: 22 }] ) },
    { Hcl, Programming, "hcl.ascii", define_colors!( [Color::Magenta, Color::Magenta] : [Color::TrueColor{ r: 95, g: 67, b: 233 }, Color::TrueColor{ r: 64, g: 64, b: 178 }] ) },
    { HolyC, Programming, "holyc.ascii", define_colors!( [Color::Yellow, Color::Cyan, Color::White] : [Color::TrueColor{ r:251, g:254 ,b:103}, Color::TrueColor{ r:11, g:68 ,b:157}, Color::TrueColor{ r:255, g:255 ,b:255} ]) },
    { Html, Markup, "html.ascii", define_colors!( [Color::Red, Color::White] ) },
    { Idris, Programming, "idris.ascii", define_colors!( [Color::Red] ) },
    { Java, Programming, "java.ascii", define_colors!( [Color::Red, Color::Blue] : [Color::TrueColor{ r:244, g:67 ,b:54}, Color::TrueColor{ r:22, g:101 ,b:192} ] ) },
    { JavaScript, Programming, "javascript.ascii", define_colors!( [Color::Yellow] : [Color::TrueColor{ r:236, g:230 ,b:83} ]) },
    { Json, Data, "json.ascii", define_colors!( [Color::White, Color::Black] ) },
    { Jsonnet, Programming, "jsonnet.ascii", define_colors!( [Color::White, Color::Black] ) },
    { Jsx, Programming, "jsx.ascii", define_colors!( [Color::Yellow] ) },
    { Julia, Programming, "julia.ascii", define_colors!( [Color::White, Color::Blue, Color::Green, Color::Red, Color::Magenta] ) },
    { Jupyter, Programming, "jupyter.ascii", define_colors!( [Color::White, Color::Yellow, Color::White] : [Color::TrueColor{ r:255, g:255 ,b:255}, Color::TrueColor{ r:255, g:112 ,b:15}, Color::TrueColor{ r:158, g:158 ,b:158} ] ), "jupyter-notebooks" },
    { Kotlin, Programming, "kotlin.ascii", define_colors!( [Color::Blue, Color::Yellow, Color::Magenta] ) },
    { Lisp, Programming, "lisp.ascii", define_colors!( [Color::White] ) },
    { Lua, Programming, "lua.ascii", define_colors!( [Color::Blue, Color::White, Color::White] : [Color::TrueColor{ r:46, g:0 ,b:127}, Color::TrueColor{ r:128, g:128 ,b:128}, Color::TrueColor{ r:255, g:255 ,b:255} ] ) },
    { LLVM, Programming, "llvm.ascii", define_colors!( [Color::Red] : [Color::TrueColor{ r:152, g:1 ,b:46}] ) },
    { Markdown, Prose, "markdown.ascii", define_colors!( [Color::White, Color::Red] ) },
    { Nim, Programming, "nim.ascii", define_colors!( [Color::Yellow, Color::White] ) },
    { Nix, Programming, "nix.ascii", define_colors!( [Color::Cyan, Color::Blue] ) },
    { ObjectiveC, Programming, "objectivec.ascii", define_colors!( [Color::Cyan, Color::Blue] ), "objective-c" },
    { OCaml, Programming, "ocaml.ascii", define_colors!( [Color::Yellow] ) },
    { Org, Prose, "org.ascii", define_colors!( [Color::Green, Color::Red, Color::White] ) },
    { Perl, Programming, "perl.ascii", define_colors!( [Color::Cyan] ) },
    { Php, Programming, "php.ascii", define_colors!( [Color::Magenta, Color::Blue, Color::Cyan, Color::White] ) },
    { PowerShell, Programming, "powershell.ascii", define_colors!( [Color::Blue, Color::White] : [Color::TrueColor{ r:49, g:108, b:185}, Color::TrueColor{ r:255, g:255, b:255} ] ) },
    { Processing, Programming, "processing.ascii", define_colors!( [Color::Blue, Color::White] : [Color::TrueColor{ r:80, g:80 ,b:80}, Color::TrueColor{ r:255, g:255 ,b:255} ] ) },
    { Prolog, Programming, "prolog.ascii", define_colors!( [Color::White] ) },
    { Protobuf, Programming, "protobuf.ascii", define_colors!( [Color::Red, Color::Blue, Color::Green, Color::Yellow] ), "protocol-buffers" },
    { PureScript, Programming, "purescript.ascii", define_colors!( [Color::White] ) },
    { Python, Programming, "python.ascii", define_colors!( [Color::Blue, Color::Yellow] : [Color::TrueColor{ r:47, g:105 ,b:162}, Color::TrueColor{ r:255, g:217 ,b:64} ] ) },
    { Qml, Programming, "qml.ascii", define_colors!( [Color::Green, Color::White, Color::Green] : [Color::TrueColor{ r:128, g:195 ,b:66}, Color::TrueColor{ r:255, g:255 ,b:255}, Color::TrueColor{ r:77, g:117 ,b:40} ] ) },
    { R, Programming, "r.ascii", define_colors!( [Color::White, Color::Blue] ) },
    { Racket, Programming, "racket.ascii", define_colors!( [Color::Red, Color::White, Color::Blue] ) },
    {
        Perl6, Programming, "raku.ascii", define_colors!( [
            Color::Blue,
            Color::Red,
            Color::Yellow,
            Color::White,
            Color::Green
        ] : [
            Color::TrueColor{ r:91, g:0, b:253 },
            Color::TrueColor{ r:255, g:0, b:94 },
            Color::TrueColor{ r:243, g:255, b:39 },
            Color::TrueColor{ r:255, g:255, b:255 },
            Color::TrueColor{ r:0, g:255, b:57 }
        ] ),
        "raku"
    },
    {
        Renpy, Programming, "renpy.ascii", define_colors!( [
            Color::White,
            Color::Red,
            Color::White,
            Color::Blue,
            Color::Yellow,
            Color::White,
            Color::Magenta
        ] : [
            Color::TrueColor{ r:234, g:219, b:204 },
            Color::TrueColor{ r:255, g:127, b:127 },
            Color::TrueColor{ r:251, g:238, b:226 },
            Color::TrueColor{ r:73, g:95, b:142 },
            Color::TrueColor{ r:250, g:228, b:90 },
            Color::White,
            Color::TrueColor{ r:181, g:163, b:150 }
        ] )
    },
    { Ruby, Programming, "ruby.ascii", define_colors!( [Color::Red] : [Color::TrueColor{ r: 204, g: 52, b: 45 }] ) },
    { Rust, Programming, "rust.ascii", define_colors!( [Color::Red, Color::White] : [Color::TrueColor{ r:228, g:55 ,b:23}, Color::TrueColor{ r:255, g:255 ,b:255} ] ) },
    { Sass, Markup, "sass.ascii", define_colors!( [Color::Magenta] : [Color::TrueColor{ r:205, g:103 ,b:153} ] ) },
    { Scala, Programming, "scala.ascii", define_colors!( [Color::Red, Color::Red] : [Color::TrueColor{ r:223, g:63 ,b:61}, Color::TrueColor{ r:127, g:12 ,b:29} ] ) },
    { Scheme, Programming, "scheme.ascii", define_colors!( [Color::White] : [Color::TrueColor{r: 85, g:85, b:85}] ) },
    { Sh, Programming, "shell.ascii", define_colors!( [Color::Green] ), "shell" },
    { Solidity, Programming, "solidity.ascii", define_colors!( [ Color::White, Color::Black, Color::Black, Color::Black, Color::Black] : [ Color::White, Color::TrueColor{ r: 46, g: 46, b: 46 }, Color::TrueColor{ r: 26, g: 26, b: 26 }, Color::TrueColor{ r: 51, g: 51, b: 51 }, Color::TrueColor{ r: 81, g: 81, b: 81 } ] ) },
    { Sql, Data, "sql.ascii", define_colors!( [Color::Cyan, Color::Yellow] ) },
    { Svelte, Markup, "svelte.ascii", define_colors!( [Color::Red, Color::White] : [Color::TrueColor{ r: 255, g: 60, b: 0 }, Color::TrueColor{ r: 255, g: 255, b: 255 }] ) },
    {
        Swift, Programming, "swift.ascii", define_colors!( [
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
    { Tcl, Programming, "tcl.ascii", define_colors!( [Color::Blue, Color::White, Color::Cyan] ) },
    { Tex, Markup, "tex.ascii", define_colors!( [Color::White, Color::Black] ) },
    { Toml, Data, "toml.ascii", define_colors!( [Color::Red, Color::White] : [Color::TrueColor{ r:156, g:66, b:33}, Color::TrueColor{ r:255, g:255, b:255} ]) },
    { Tsx, Programming, "tsx.ascii", define_colors!( [Color::Blue] ) },
    { TypeScript, Programming, "typescript.ascii", define_colors!( [Color::Cyan, Color::White] : [Color::TrueColor{ r:0, g:122, b:204}, Color::TrueColor{ r:255, g:255, b:255} ]) },
    { Vala, Programming, "vala.ascii", define_colors!( [Color::Magenta, Color::White] ) },
    { VimScript, Programming, "vimscript.ascii", define_colors!( [Color::Green, Color::Black, Color::White] ) },
    { Vue, Programming, "vue.ascii", define_colors!( [Color::Green, Color::Blue] ) },
    { WebAssembly, Programming, "webassembly.ascii", define_colors!( [Color::Magenta, Color::White] : [Color::TrueColor{ r:101, g:79, b:240}, Color::TrueColor{ r:255, g:255, b:255} ]) },
    { Xaml, Data, "xaml.ascii", define_colors!( [Color::Blue, Color::White] : [Color::TrueColor{ r:51, g:120, b:206}, Color::TrueColor{ r:255, g:255, b:255} ]) },
    { Xml, Data, "xml.ascii", define_colors!( [Color::Yellow, Color::White, Color::Green] ) },
    { Yaml, Data, "yaml.ascii", define_colors!( [Color::White] ) },
    { Zig, Programming, "zig.ascii", define_colors!( [Color::Yellow] ) },
    { Zsh, Programming, "zsh.ascii", define_colors!( [Color::White] ) },
}
