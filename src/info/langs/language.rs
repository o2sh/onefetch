use owo_colors::{AnsiColors, DynColors};
use serde::Serialize;
use std::env;
use strum::{EnumIter, EnumString, IntoStaticStr};

pub struct Colors {
    basic_colors: Vec<DynColors>,
    true_colors: Option<Vec<DynColors>>,
}

/// Maps colors to preferred versions. Used to allow contributors to include
/// colors with minimal confusion.
macro_rules! clean_color {
    ( White ) => {
        clean_color!(Default)
    };
    ( $color:ident ) => {
        DynColors::Ansi(AnsiColors::$color)
    };
}

macro_rules! define_colors {
    ( [ $($color:ident),+ ] ) => { Colors { basic_colors: vec![$( clean_color!($color) ),+], true_colors: None } };
    ( [ $($bc:ident),+ ] : [ $($c:ident($r:expr, $g:expr, $b:expr)),+ ] ) => { Colors { basic_colors: vec![$(clean_color!($bc)),+], true_colors: Some(vec![$(DynColors::$c($r, $g, $b)),+]) } };
}

#[derive(Clone, PartialEq, EnumString, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
pub enum LanguageType {
    Programming,
    Markup,
    Prose,
    Data,
}

macro_rules! define_languages {
    ($( { $name:ident, $type:ident, $ascii:literal, $colors:expr, $circle_color:ident($r:expr, $g:expr, $b:expr) $(, $serialize:literal )? } ),* ,) => {

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

            pub fn get_colors(&self, true_color: bool) -> Vec<DynColors> {
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

            pub fn get_circle_color(&self) -> DynColors{
                match *self {
                    $( Language::$name => DynColors::$circle_color($r, $g, $b), )*
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
                        output += &format!( "        AnsiColors::{:?}, // {}\n", bc, color_index );
                    }
                    output += "    ], \n";
                    if let Some(tcs) = &self.true_colors {
                        output += "    true_colors: vec![\n";
                        for (color_index,tc) in tcs.iter().enumerate() {
                            output += &format!( "        AnsiColors::{:?}, // {}\n", tc, color_index );
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
                            let color_str = &format!( "AnsiColors::{:?}", bc );
                            if let DynColors::Rgb( .. ) = bc {
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
    { Ada, Programming, "ada.ascii", define_colors!([White, Cyan, Blue] : [Rgb(255, 255, 255), Rgb(0, 24, 201), Rgb(12, 10, 124)]), Rgb(2, 248, 140) },
    { Assembly, Programming, "assembly.ascii", define_colors!([Cyan]), Rgb(110, 76, 19) },
    { AutoHotKey, Programming, "autohotkey.ascii", define_colors!([White, Green] : [Rgb(255, 255, 255), Rgb(0x11, 0x98, 0x10)]), Rgb(101, 148, 185) },
    { Bash, Programming, "bash.ascii", define_colors!([White]), Rgb(137, 224, 81) },
    { C, Programming, "c.ascii", define_colors!([Cyan, Blue, Blue, White] : [Rgb(93, 108, 191), Rgb(41, 54, 147), Rgb(57, 73, 170), Rgb(255, 255, 255)]), Rgb(85, 85, 85) },
    { Ceylon, Programming, "ceylon.ascii", define_colors!([Yellow, Yellow, Yellow, Yellow, Yellow] : [Rgb(194, 126, 16), Rgb(221, 161, 46), Rgb(209, 145, 31), Rgb(204, 139, 24), Rgb(171, 112, 8)]), Rgb(223, 165, 53) },
    { Clojure, Programming, "clojure.ascii", define_colors!([Cyan, Green]), Rgb(219, 88, 85) },
    { CMake, Programming, "cmake.ascii", define_colors!([Blue, Green, Red, Black]), Rgb(218, 52, 52) },
    { CoffeeScript, Programming, "coffeescript.ascii", define_colors!([Red]), Rgb(36, 71, 118) },
    { Coq, Programming, "coq.ascii", define_colors!([Yellow, White] : [Rgb(191, 140, 94), Rgb(213, 190, 153)]), Rgb(208, 182, 140) },
    { Cpp, Programming, "cpp.ascii", define_colors!([Cyan, Blue, Blue, White] : [Rgb(100, 154, 210), Rgb(0, 66, 131), Rgb(0, 89, 157), Rgb(255, 255, 255)]), Rgb(243, 75, 125), "c++" },
    { Crystal, Programming, "crystal.ascii", define_colors!([White, Black]), Rgb(0, 1, 0) },
    { CSharp, Programming, "csharp.ascii", define_colors!([Blue, Magenta, Magenta, White] : [Rgb(155, 79, 151), Rgb(103, 33, 122), Rgb(128, 55, 136), Rgb(255, 255, 255)]), Rgb(23, 134, 0), "c#" },
    { Css, Markup, "css.ascii", define_colors!([Blue, White]), Rgb(86, 61, 124) },
    { D, Programming, "d.ascii", define_colors!([Red]), Rgb(186, 89, 94) },
    { Dart, Programming, "dart.ascii", define_colors!([Blue, Cyan, Blue ] : [Rgb(0, 163, 231), Rgb(66, 223, 205), Rgb(1, 89, 125)]), Rgb(0, 180, 171) },
    { Dockerfile, Programming, "dockerfile.ascii", define_colors!([Cyan, White, Cyan]), Rgb(56, 77, 84) },
    { Elisp, Programming, "emacslisp.ascii", define_colors!([Magenta, White]), Rgb(192, 101, 219), "emacs-lisp" },
    { Elixir, Programming, "elixir.ascii", define_colors!([Magenta]), Rgb(110, 74, 126) },
    { Elm, Programming, "elm.ascii", define_colors!([Blue, Green, Yellow, Cyan]), Rgb(96, 181, 204) },
    { Emojicode, Programming, "emojicode.ascii", define_colors!([Green, Magenta,  Magenta, Magenta] : [Rgb(119, 178, 85), Rgb(146, 102, 204), Rgb(170, 141, 216), Rgb(116, 78, 170)]), Rgb(96, 181, 204) },
    { Erlang, Programming, "erlang.ascii", define_colors!([Red]), Rgb(184, 57, 152) },
    { Fish, Programming, "fish.ascii", define_colors!([Red, Yellow]), Rgb(74, 174, 71) },
    { Forth, Programming, "forth.ascii", define_colors!([Red]), Rgb(52, 23, 8) },
    { FortranModern, Programming, "f90.ascii", define_colors!([White, Green, Cyan, Yellow, Red]), Rgb(77, 65, 177), "fortran" },
    { FortranLegacy, Programming, "f77.ascii", define_colors!([White, Green, Cyan, Yellow, Red]), Rgb(77, 65, 177), "fortran-legacy" },
    { FSharp, Programming, "fsharp.ascii", define_colors!([Cyan, Cyan]), Rgb(184, 69, 252), "f#" },
    { GdScript, Programming, "gdscript.ascii", define_colors!([Cyan, White] : [Rgb(69, 141, 192), Rgb(255, 255, 255)]), Rgb(53, 85, 112) },
    { Go, Programming, "go.ascii", define_colors!([Cyan, White, Yellow] : [Rgb(116, 205, 221), Rgb(255, 255, 255), Rgb(246, 210, 162)]), Rgb(0, 173, 216) },
    { Graphql, Data, "graphql.ascii", define_colors!([Magenta]), Rgb(225, 0, 152) },
    { Groovy, Programming, "groovy.ascii", define_colors!([Cyan, White]), Rgb(66, 152, 184) },
    { Haskell, Programming, "haskell.ascii", define_colors!([Cyan, Magenta, Blue] : [Rgb(69, 58, 98), Rgb(94, 80, 134), Rgb(143, 78, 139)]), Rgb(94, 80, 134) },
    { Haxe, Programming, "haxe.ascii", define_colors!([Yellow, Yellow, Yellow] : [Rgb(250, 178, 11), Rgb(246, 153, 18), Rgb(244, 114, 22)]), Rgb(223, 121, 0) },
    { Hcl, Programming, "hcl.ascii", define_colors!([Magenta, Magenta] : [Rgb(95, 67, 233), Rgb(64, 64, 178)]), Rgb(170, 206, 96) },
    { HolyC, Programming, "holyc.ascii", define_colors!([Yellow, Cyan, White] : [Rgb(251, 254 ,103), Rgb(11, 68 ,157), Rgb(255, 255 ,255)]), Rgb(255, 239, 175) },
    { Html, Markup, "html.ascii", define_colors!([Red, White]), Rgb(227, 76, 38) },
    { Idris, Programming, "idris.ascii", define_colors!([Red]), Rgb(179, 0, 0) },
    { Java, Programming, "java.ascii", define_colors!([Red, Blue] : [Rgb(244, 67 ,54), Rgb(22, 101 ,192)]), Rgb(176, 114, 25) },
    { JavaScript, Programming, "javascript.ascii", define_colors!([Yellow] : [Rgb(236, 230 ,83)]), Rgb(241, 224, 90) },
    { Json, Data, "json.ascii", define_colors!([White, Black]), Rgb(41, 41, 41) },
    { Jsonnet, Programming, "jsonnet.ascii", define_colors!([White, Black]), Rgb(0, 100, 189) },
    { Jsx, Programming, "jsx.ascii", define_colors!([Yellow]), Rgb(241, 224, 90) },
    { Julia, Programming, "julia.ascii", define_colors!([White, Blue, Green, Red, Magenta]), Rgb(162, 112, 186) },
    { Jupyter, Programming, "jupyter.ascii", define_colors!([White, Yellow, White] : [Rgb(255, 255 ,255), Rgb(255, 112 ,15), Rgb(158, 158 ,158)]), Rgb(218, 91, 11), "jupyter-notebooks" },
    { Kotlin, Programming, "kotlin.ascii", define_colors!([Blue, Yellow, Magenta]), Rgb(169, 123, 255) },
    { Lisp, Programming, "lisp.ascii", define_colors!([White]), Rgb(63, 182, 139) },
    { Lua, Programming, "lua.ascii", define_colors!([Blue, White, White] : [Rgb(46, 0 ,127), Rgb(128, 128 ,128), Rgb(255, 255 ,255)]), Rgb(0, 0, 128) },
    { LLVM, Programming, "llvm.ascii", define_colors!([Red] : [Rgb(152, 1 ,46)]), Rgb(24, 86, 25) },
    { Markdown, Prose, "markdown.ascii", define_colors!([White, Red]), Rgb(8, 63, 161) },
    { Nim, Programming, "nim.ascii", define_colors!([Yellow, White]), Rgb(255, 194, 0) },
    { Nix, Programming, "nix.ascii", define_colors!([Cyan, Blue]), Rgb(126, 126, 255) },
    { ObjectiveC, Programming, "objectivec.ascii", define_colors!([Cyan, Blue]), Rgb(67, 142, 255), "objective-c" },
    { OCaml, Programming, "ocaml.ascii", define_colors!([Yellow]), Rgb(59, 225, 51) },
    { Org, Prose, "org.ascii", define_colors!([Green, Red, White]), Rgb(119, 170, 153) },
    { Perl, Programming, "perl.ascii", define_colors!([Cyan]), Rgb(2, 152, 195) },
    { Php, Programming, "php.ascii", define_colors!([Magenta, White] : [Rgb(119, 123, 179), Rgb(255, 255, 255)]), Rgb(79, 93, 149) },
    { PowerShell, Programming, "powershell.ascii", define_colors!([Blue, White] : [Rgb(49, 108, 185), Rgb(255, 255, 255)]), Rgb(1, 36, 86) },
    { Processing, Programming, "processing.ascii", define_colors!([Blue, White] : [Rgb(80, 80 ,80), Rgb(255, 255 ,255)]), Rgb(0, 150, 216) },
    { Prolog, Programming, "prolog.ascii", define_colors!([White]), Rgb(116, 40, 60) },
    { Protobuf, Programming, "protobuf.ascii", define_colors!([Red, Blue, Green, Yellow]), Rgb(116, 40, 60), "protocol-buffers" },
    { PureScript, Programming, "purescript.ascii", define_colors!([White]), Rgb(29, 34, 45) },
    { Python, Programming, "python.ascii", define_colors!([Blue, Yellow] : [Rgb(47, 105 ,162), Rgb(255, 217 ,64)]), Rgb(53, 114, 165) },
    { Qml, Programming, "qml.ascii", define_colors!([Green, White, Green] : [Rgb(128, 195 ,66), Rgb(255, 255 ,255), Rgb(77, 117 ,40)]), Rgb(68, 165, 28) },
    { R, Programming, "r.ascii", define_colors!([White, Blue]), Rgb(25, 140, 231) },
    { Racket, Programming, "racket.ascii", define_colors!([Red, White, Blue]), Rgb(60, 92, 170) },
    { Perl6, Programming, "raku.ascii", define_colors!([Blue, Red, Yellow, White, Green] : [Rgb(91, 0, 253), Rgb(255, 0, 94), Rgb(243, 255, 39), Rgb(255, 255, 255), Rgb(0, 255, 57)]), Rgb(0, 0, 251), "raku"  },
    { Renpy, Programming, "renpy.ascii", define_colors!([White, Red, White, Blue, Yellow, White, Magenta] : [Rgb(234, 219, 204), Rgb(255, 127, 127), Rgb(251, 238, 226), Rgb(73, 95, 142), Rgb(250, 228, 90), Rgb(255, 255, 255), Rgb(181, 163, 150)]), Rgb(255, 127, 127) },
    { Ruby, Programming, "ruby.ascii", define_colors!([Red] : [Rgb(204, 52, 45)]), Rgb(112, 21, 22) },
    { Rust, Programming, "rust.ascii", define_colors!([Red, White] : [Rgb(228, 55 ,23), Rgb(255, 255 ,255)]), Rgb(222, 165, 132) },
    { Sass, Markup, "sass.ascii", define_colors!([Magenta] : [Rgb(205, 103 ,153)]), Rgb(165, 59, 112) },
    { Scala, Programming, "scala.ascii", define_colors!([Red, Red] : [Rgb(223, 63 ,61), Rgb(127, 12 ,29)]), Rgb(194, 45, 64) },
    { Scheme, Programming, "scheme.ascii", define_colors!([White] : [Rgb(85, 85, 85)]), Rgb(30, 74, 236) },
    { Sh, Programming, "shell.ascii", define_colors!([Green]), Rgb(137, 224, 81), "shell" },
    { Solidity, Programming, "solidity.ascii", define_colors!([White, Black, Black, Black, Black] : [Rgb(255, 255, 255), Rgb(46, 46, 46), Rgb(26, 26, 26), Rgb(51, 51, 51), Rgb(81, 81, 81)]), Rgb(170, 103, 70) },
    { Sql, Data, "sql.ascii", define_colors!([Cyan, Yellow]), Rgb(227, 140, 0) },
    { Svelte, Markup, "svelte.ascii", define_colors!([Red, White] : [Rgb(255, 60, 0), Rgb(255, 255, 255)]), Rgb(255, 62, 0) },
    { Swift, Programming, "swift.ascii", define_colors!([Red, Red, Red, Red, Red, Red, Red, Red, Red, Red] : [Rgb(248, 129, 52), Rgb(249, 119, 50), Rgb(249, 109, 48), Rgb(250, 99, 46), Rgb(250, 89, 44), Rgb(251, 80, 42), Rgb(251, 70, 40), Rgb(252, 60, 38), Rgb(252, 50, 36), Rgb(253, 40, 34)]), Rgb(240, 81, 56) },
    { Tcl, Programming, "tcl.ascii", define_colors!([Blue, White, Cyan]), Rgb(228, 204, 152) },
    { Tex, Markup, "tex.ascii", define_colors!([White, Black]), Rgb(61, 97, 23) },
    { Toml, Data, "toml.ascii", define_colors!([Red, White] : [Rgb(156, 66, 33), Rgb(255, 255, 255)]), Rgb(156, 66, 33) },
    { Tsx, Programming, "tsx.ascii", define_colors!([Blue]), Rgb(43, 116, 137) },
    { TypeScript, Programming, "typescript.ascii", define_colors!([Cyan, White] : [Rgb(0, 122, 204), Rgb(255, 255, 255)]), Rgb(43, 116, 137) },
    { Vala, Programming, "vala.ascii", define_colors!([Magenta, White]), Rgb(165, 109, 226) },
    { Vhdl, Programming, "vhdl.ascii", define_colors!([Yellow, Green, White]), Rgb(173, 178, 203) },
    { VimScript, Programming, "vimscript.ascii", define_colors!([Green, Black, White]), Rgb(25, 159, 75) },
    { Vue, Programming, "vue.ascii", define_colors!([Green, Blue]), Rgb(65, 184, 131) },
    { WebAssembly, Programming, "webassembly.ascii", define_colors!([Magenta, White] : [Rgb(101, 79, 240), Rgb(255, 255, 255)]), Rgb(4, 19, 59) },
    { Wolfram, Programming, "wolfram.ascii", define_colors!([Red, White, Black] : [Rgb(255, 10, 1), Rgb(255, 255, 255), Rgb(0, 0, 0)]), Rgb(221, 17, 0) },
    { Xaml, Data, "xaml.ascii", define_colors!([Blue, White] : [Rgb(51, 120, 206), Rgb(255, 255, 255)]), Rgb(0, 96, 172) },
    { Xml, Data, "xml.ascii", define_colors!([Yellow, White, Green]), Rgb(0, 96, 172) },
    { Yaml, Data, "yaml.ascii", define_colors!([White]), Rgb(203, 23, 30) },
    { Zig, Programming, "zig.ascii", define_colors!([Yellow]), Rgb(236, 145, 92) },
    { Zsh, Programming, "zsh.ascii", define_colors!([White]), Rgb(137, 224, 81) },
}
