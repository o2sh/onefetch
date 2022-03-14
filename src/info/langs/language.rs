use colored::Color;
use serde::Serialize;
use std::env;
use strum::{EnumIter, EnumString, IntoStaticStr};
use OptionalColor::*;

pub struct Colors {
    basic_colors: Vec<OptionalColor>,
    true_colors: Option<Vec<OptionalColor>>,
}

macro_rules! define_optional_color {
    ($( $color:ident ),*) => {
        #[derive(Debug)]
        pub enum OptionalColor {
            $( $color, )*
            TrueColor{ r: u8, g: u8, b: u8 },
            Fg,
        }

        impl From<OptionalColor> for Option<::colored::Color> {
            fn from(color: OptionalColor) -> Option<::colored::Color> {
                match color {
                    $( OptionalColor::$color => Some(::colored::Color::$color), )*
                    OptionalColor::TrueColor{ r, g, b } => Some(::colored::Color::TrueColor{r, g, b}),
                    OptionalColor::Fg => Option::None,
                }
            }
        }
    }
}

define_optional_color!(Black, Red, Green, Yellow, Blue, Magenta, Cyan, White);

macro_rules! define_colors {
    ( [ $($color:expr),+ ] ) => { Colors { basic_colors: vec![$($color),+], true_colors: Option::None } };
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

            pub fn get_colors(&self, true_color: bool) -> Vec<Option<Color>> {
                let colors = match *self {
                    $( Language::$name => $colors, )*
                };
                match colors.true_colors {
                  Some( true_colors ) if true_color => true_colors.into_iter().map(|c| c.into()).collect(),
                  _ => colors.basic_colors.into_iter().map(|c| c.into()).collect(),
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
                            if let OptionalColor::TrueColor { .. } = bc {
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
    { Ada, Programming, "ada.ascii", define_colors!( [Fg, Cyan, Blue] : [TrueColor{r:255, g:255, b:255}, TrueColor{r:0, g:24, b:201}, TrueColor{r:12, g:10, b:124}] ) },
    { Assembly, Programming, "assembly.ascii", define_colors!( [Cyan] ) },
    { AutoHotKey, Programming, "autohotkey.ascii", define_colors!( [Fg, Green] : [TrueColor{r:255, g:255, b:255}, TrueColor{r: 0x11, g: 0x98, b: 0x10}]) },
    { Bash, Programming, "bash.ascii", define_colors!( [Fg] ) },
    { C, Programming, "c.ascii", define_colors!( [Cyan, Blue, Blue, Fg] : [TrueColor{r:93, g:108, b:191}, TrueColor{r:41, g:54, b:147}, TrueColor{r:57, g:73, b:170}, TrueColor{r:255, g:255, b:255}] ) },
    {
        Ceylon, Programming, "ceylon.ascii", define_colors!( [
            Yellow,
            Yellow,
            Yellow,
            Yellow,
            Yellow
        ] : [
            TrueColor{ r:194, g:126, b:16 },
            TrueColor{ r:221, g:161, b:46 },
            TrueColor{ r:209, g:145, b:31 },
            TrueColor{ r:204, g:139, b:24 },
            TrueColor{ r:171, g:112, b:8 }
        ] )
    },
    { Clojure, Programming, "clojure.ascii", define_colors!( [Cyan, Green] ) },
    { CMake, Programming, "cmake.ascii", define_colors!( [Blue, Green, Red, Black] ) },
    { CoffeeScript, Programming, "coffeescript.ascii", define_colors!( [Red] ) },
    { Coq, Programming, "coq.ascii", define_colors!( [Yellow, Fg] : [TrueColor {r:191, g:140, b:94}, TrueColor {r:213, g:190, b:153}] ) },
    { Cpp, Programming, "cpp.ascii", define_colors!( [Cyan, Blue, Blue, Fg] : [TrueColor{r:100, g:154, b:210}, TrueColor{r:0, g:66, b:131}, TrueColor{r:0, g:89, b:157}, TrueColor{r:255, g:255, b:255}] ), "c++" },
    { Crystal, Programming, "crystal.ascii", define_colors!( [Fg, Black] ) },
    { CSharp, Programming, "csharp.ascii", define_colors!( [Blue, Magenta, Magenta, Fg] : [TrueColor{r:155, g:79, b:151}, TrueColor{r:103, g:33, b:122}, TrueColor{r:128, g:55, b:136}, TrueColor{r:255, g:255, b:255}] ), "c#" },
    { Css, Markup, "css.ascii", define_colors!( [Blue, Fg] ) },
    { D, Programming, "d.ascii", define_colors!( [Red] ) },
    { Dart, Programming, "dart.ascii", define_colors!( [Blue, Cyan, Blue ] : [TrueColor{ r:0, g:163, b:231 }, TrueColor{ r:66, g:223, b:205 }, TrueColor{ r:1, g:89, b:125 }] ) },
    { Dockerfile, Programming, "dockerfile.ascii", define_colors!( [Cyan, Fg, Cyan] ) },
    { Elisp, Programming, "emacslisp.ascii", define_colors!( [Magenta, Fg] ), "emacs-lisp" },
    { Elixir, Programming, "elixir.ascii", define_colors!( [Magenta] ) },
    { Elm, Programming, "elm.ascii", define_colors!( [Blue, Green, Yellow, Cyan] ) },
    { Emojicode, Programming, "emojicode.ascii", define_colors!( [Green, Magenta,  Magenta, Magenta] : [TrueColor{r:119, g:178, b:85}, TrueColor{r:146, g:102, b:204}, TrueColor{r:170, g:141, b:216}, TrueColor{r:116, g:78, b:170}] ) },
    { Erlang, Programming, "erlang.ascii", define_colors!( [Red] ) },
    { Fish, Programming, "fish.ascii", define_colors!( [Red, Yellow] ) },
    { Forth, Programming, "forth.ascii", define_colors!( [Red] ) },
    { FortranModern, Programming, "f90.ascii", define_colors!( [Fg, Green, Cyan, Yellow, Red] ), "fortran" },
    { FortranLegacy, Programming, "f77.ascii", define_colors!( [Fg, Green, Cyan, Yellow, Red] ), "fortran-legacy" },
    { FSharp, Programming, "fsharp.ascii", define_colors!( [Cyan, Cyan] ), "f#" },
    { GdScript, Programming, "gdscript.ascii", define_colors!( [Cyan, Fg] : [TrueColor{ r:69, g:141, b:192 }, TrueColor{ r:255, g:255, b:255}] ) },
    { Go, Programming, "go.ascii", define_colors!( [Cyan, Fg, Yellow] : [TrueColor{ r:116, g:205, b:221 }, TrueColor{ r:255, g:255, b:255 }, TrueColor{ r:246, g:210, b:162 }] ) },
    { Graphql, Data, "graphql.ascii", define_colors!( [Magenta] ) },
    { Groovy, Programming, "groovy.ascii", define_colors!( [Cyan, Fg] ) },
    { Haskell, Programming, "haskell.ascii", define_colors!( [Cyan, Magenta, Blue] : [TrueColor{ r:69, g:58, b:98 }, TrueColor{ r:94, g:80, b:134 }, TrueColor{ r:143, g:78, b:139 }] ) },
    { Haxe, Programming, "haxe.ascii", define_colors!( [Yellow, Yellow, Yellow] : [TrueColor{ r: 250, g: 178, b: 11 }, TrueColor{ r:246, g:153, b:18 }, TrueColor{ r: 244, g: 114, b: 22 }] ) },
    { Hcl, Programming, "hcl.ascii", define_colors!( [Magenta, Magenta] : [TrueColor{ r: 95, g: 67, b: 233 }, TrueColor{ r: 64, g: 64, b: 178 }] ) },
    { HolyC, Programming, "holyc.ascii", define_colors!( [Yellow, Cyan, Fg] : [TrueColor{ r:251, g:254 ,b:103}, TrueColor{ r:11, g:68 ,b:157}, TrueColor{ r:255, g:255 ,b:255} ]) },
    { Html, Markup, "html.ascii", define_colors!( [Red, Fg] ) },
    { Idris, Programming, "idris.ascii", define_colors!( [Red] ) },
    { Java, Programming, "java.ascii", define_colors!( [Red, Blue] : [TrueColor{ r:244, g:67 ,b:54}, TrueColor{ r:22, g:101 ,b:192} ] ) },
    { JavaScript, Programming, "javascript.ascii", define_colors!( [Yellow] : [TrueColor{ r:236, g:230 ,b:83} ]) },
    { Json, Data, "json.ascii", define_colors!( [Fg, Black] ) },
    { Jsonnet, Programming, "jsonnet.ascii", define_colors!( [Fg, Black] ) },
    { Jsx, Programming, "jsx.ascii", define_colors!( [Yellow] ) },
    { Julia, Programming, "julia.ascii", define_colors!( [Fg, Blue, Green, Red, Magenta] ) },
    { Jupyter, Programming, "jupyter.ascii", define_colors!( [Fg, Yellow, Fg] : [TrueColor{ r:255, g:255 ,b:255}, TrueColor{ r:255, g:112 ,b:15}, TrueColor{ r:158, g:158 ,b:158} ] ), "jupyter-notebooks" },
    { Kotlin, Programming, "kotlin.ascii", define_colors!( [Blue, Yellow, Magenta] ) },
    { Lisp, Programming, "lisp.ascii", define_colors!( [Fg] ) },
    { Lua, Programming, "lua.ascii", define_colors!( [Blue, Fg, Fg] : [TrueColor{ r:46, g:0 ,b:127}, TrueColor{ r:128, g:128 ,b:128}, TrueColor{ r:255, g:255 ,b:255} ] ) },
    { LLVM, Programming, "llvm.ascii", define_colors!( [Red] : [TrueColor{ r:152, g:1 ,b:46}] ) },
    { Markdown, Prose, "markdown.ascii", define_colors!( [Fg, Red] ) },
    { Nim, Programming, "nim.ascii", define_colors!( [Yellow, Fg] ) },
    { Nix, Programming, "nix.ascii", define_colors!( [Cyan, Blue] ) },
    { ObjectiveC, Programming, "objectivec.ascii", define_colors!( [Cyan, Blue] ), "objective-c" },
    { OCaml, Programming, "ocaml.ascii", define_colors!( [Yellow] ) },
    { Org, Prose, "org.ascii", define_colors!( [Green, Red, Fg] ) },
    { Perl, Programming, "perl.ascii", define_colors!( [Cyan] ) },
    { Php, Programming, "php.ascii", define_colors!( [Magenta, Blue, Cyan, Fg] ) },
    { PowerShell, Programming, "powershell.ascii", define_colors!( [Blue, Fg] : [TrueColor{ r:49, g:108, b:185}, TrueColor{ r:255, g:255, b:255} ] ) },
    { Processing, Programming, "processing.ascii", define_colors!( [Blue, Fg] : [TrueColor{ r:80, g:80 ,b:80}, TrueColor{ r:255, g:255 ,b:255} ] ) },
    { Prolog, Programming, "prolog.ascii", define_colors!( [Fg] ) },
    { Protobuf, Programming, "protobuf.ascii", define_colors!( [Red, Blue, Green, Yellow] ), "protocol-buffers" },
    { PureScript, Programming, "purescript.ascii", define_colors!( [Fg] ) },
    { Python, Programming, "python.ascii", define_colors!( [Blue, Yellow] : [TrueColor{ r:47, g:105 ,b:162}, TrueColor{ r:255, g:217 ,b:64} ] ) },
    { Qml, Programming, "qml.ascii", define_colors!( [Green, Fg, Green] : [TrueColor{ r:128, g:195 ,b:66}, TrueColor{ r:255, g:255 ,b:255}, TrueColor{ r:77, g:117 ,b:40} ] ) },
    { R, Programming, "r.ascii", define_colors!( [Fg, Blue] ) },
    { Racket, Programming, "racket.ascii", define_colors!( [Red, Fg, Blue] ) },
    {
        Perl6, Programming, "raku.ascii", define_colors!( [
            Blue,
            Red,
            Yellow,
            Fg,
            Green
        ] : [
            TrueColor{ r:91, g:0, b:253 },
            TrueColor{ r:255, g:0, b:94 },
            TrueColor{ r:243, g:255, b:39 },
            TrueColor{ r:255, g:255, b:255 },
            TrueColor{ r:0, g:255, b:57 }
        ] ),
        "raku"
    },
    {
        Renpy, Programming, "renpy.ascii", define_colors!( [
            Fg,
            Red,
            Fg,
            Blue,
            Yellow,
            Fg,
            Magenta
        ] : [
            TrueColor{ r:234, g:219, b:204 },
            TrueColor{ r:255, g:127, b:127 },
            TrueColor{ r:251, g:238, b:226 },
            TrueColor{ r:73, g:95, b:142 },
            TrueColor{ r:250, g:228, b:90 },
            Fg,
            TrueColor{ r:181, g:163, b:150 }
        ] )
    },
    { Ruby, Programming, "ruby.ascii", define_colors!( [Red] : [TrueColor{ r: 204, g: 52, b: 45 }] ) },
    { Rust, Programming, "rust.ascii", define_colors!( [Red, Fg] : [TrueColor{ r:228, g:55 ,b:23}, TrueColor{ r:255, g:255 ,b:255} ] ) },
    { Sass, Markup, "sass.ascii", define_colors!( [Magenta] : [TrueColor{ r:205, g:103 ,b:153} ] ) },
    { Scala, Programming, "scala.ascii", define_colors!( [Red, Red] : [TrueColor{ r:223, g:63 ,b:61}, TrueColor{ r:127, g:12 ,b:29} ] ) },
    { Scheme, Programming, "scheme.ascii", define_colors!( [Fg] : [TrueColor{r: 85, g:85, b:85}] ) },
    { Sh, Programming, "shell.ascii", define_colors!( [Green] ), "shell" },
    { Solidity, Programming, "solidity.ascii", define_colors!( [ Fg, Black, Black, Black, Black] : [ Fg, TrueColor{ r: 46, g: 46, b: 46 }, TrueColor{ r: 26, g: 26, b: 26 }, TrueColor{ r: 51, g: 51, b: 51 }, TrueColor{ r: 81, g: 81, b: 81 } ] ) },
    { Sql, Data, "sql.ascii", define_colors!( [Cyan, Yellow] ) },
    { Svelte, Markup, "svelte.ascii", define_colors!( [Red, Fg] : [TrueColor{ r: 255, g: 60, b: 0 }, TrueColor{ r: 255, g: 255, b: 255 }] ) },
    {
        Swift, Programming, "swift.ascii", define_colors!( [
            Red,
            Red,
            Red,
            Red,
            Red,
            Red,
            Red,
            Red,
            Red,
            Red
        ] : [
            TrueColor{ r:248, g:129, b:52 },
            TrueColor{ r:249, g:119, b:50 },
            TrueColor{ r:249, g:109, b:48 },
            TrueColor{ r:250, g:99, b:46 },
            TrueColor{ r:250, g:89, b:44 },
            TrueColor{ r:251, g:80, b:42 },
            TrueColor{ r:251, g:70, b:40 },
            TrueColor{ r:252, g:60, b:38 },
            TrueColor{ r:252, g:50, b:36 },
            TrueColor{ r:253, g:40, b:34 }
        ] )
    },
    { Tcl, Programming, "tcl.ascii", define_colors!( [Blue, Fg, Cyan] ) },
    { Tex, Markup, "tex.ascii", define_colors!( [Fg, Black] ) },
    { Toml, Data, "toml.ascii", define_colors!( [Red, Fg] : [TrueColor{ r:156, g:66, b:33}, TrueColor{ r:255, g:255, b:255} ]) },
    { Tsx, Programming, "tsx.ascii", define_colors!( [Blue] ) },
    { TypeScript, Programming, "typescript.ascii", define_colors!( [Cyan, Fg] : [TrueColor{ r:0, g:122, b:204}, TrueColor{ r:255, g:255, b:255} ]) },
    { Vala, Programming, "vala.ascii", define_colors!( [Magenta, Fg] ) },
    { VimScript, Programming, "vimscript.ascii", define_colors!( [Green, Black, Fg] ) },
    { Vue, Programming, "vue.ascii", define_colors!( [Green, Blue] ) },
    { WebAssembly, Programming, "webassembly.ascii", define_colors!( [Magenta, Fg] : [TrueColor{ r:101, g:79, b:240}, TrueColor{ r:255, g:255, b:255} ]) },
    { Xaml, Data, "xaml.ascii", define_colors!( [Blue, Fg] : [TrueColor{ r:51, g:120, b:206}, TrueColor{ r:255, g:255, b:255} ]) },
    { Xml, Data, "xml.ascii", define_colors!( [Yellow, Fg, Green] ) },
    { Yaml, Data, "yaml.ascii", define_colors!( [Fg] ) },
    { Zig, Programming, "zig.ascii", define_colors!( [Yellow] ) },
    { Zsh, Programming, "zsh.ascii", define_colors!( [Fg] ) },
}
