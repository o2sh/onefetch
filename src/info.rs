use colored::Color;
use std::{cmp, fmt, fmt::Write};

use commitinfo::CommitInfo;
use language::Language;

pub struct Info {
    pub project_name: String,
    pub current_commit: CommitInfo,
    pub version: String,
    pub creation_date: String,
    pub dominant_language: Language,
    pub languages: Vec<(Language, f64)>,
    pub authors: Vec<(String, usize, usize)>,
    pub last_change: String,
    pub repo: String,
    pub commits: String,
    pub repo_size: String,
    pub number_of_lines: usize,
    pub license: String,
    pub custom_logo: Language,
}

impl Info {
    pub fn get_ascii(&self) -> &str {
        let language = if let Language::Unknown = self.custom_logo {
            &self.dominant_language
        } else {
            &self.custom_logo
        };

        match language {
            Language::Assembly => include_str!("../resources/assembly.ascii"),
            Language::C => include_str!("../resources/c.ascii"),
            Language::Clojure => include_str!("../resources/clojure.ascii"),
            Language::CoffeeScript => include_str!("../resources/coffeescript.ascii"),
            Language::Cpp => include_str!("../resources/cpp.ascii"),
            Language::Csharp => include_str!("../resources/csharp.ascii"),
            Language::CSS => include_str!("../resources/css.ascii"),
            Language::Dart => include_str!("../resources/dart.ascii"),
            Language::Elm => include_str!("../resources/elm.ascii"),
            Language::Erlang => include_str!("../resources/erlang.ascii"),
            Language::Forth => include_str!("../resources/forth.ascii"),
            Language::Go => include_str!("../resources/go.ascii"),
            Language::Haskell => include_str!("../resources/haskell.ascii"),
            Language::HTML => include_str!("../resources/html.ascii"),
            Language::Idris => include_str!("../resources/idris.ascii"),
            Language::Java => include_str!("../resources/java.ascii"),
            Language::Kotlin => include_str!("../resources/kotlin.ascii"),
            Language::Lisp => include_str!("../resources/lisp.ascii"),
            Language::Lua => include_str!("../resources/lua.ascii"),
            Language::Nim => include_str!("../resources/nim.ascii"),
            Language::ObjectiveC => include_str!("../resources/objectivec.ascii"),
            Language::PureScript => include_str!("../resources/purescript.ascii"),
            Language::Python => include_str!("../resources/python.ascii"),
            Language::R => include_str!("../resources/r.ascii"),
            Language::Ruby => include_str!("../resources/ruby.ascii"),
            Language::Rust => include_str!("../resources/rust.ascii"),
            Language::Scala => include_str!("../resources/scala.ascii"),
            Language::Shell => include_str!("../resources/shell.ascii"),
            Language::Swift => include_str!("../resources/swift.ascii"),
            Language::Tcl => include_str!("../resources/tcl.ascii"),
            Language::TypeScript => include_str!("../resources/typescript.ascii"),
            Language::JavaScript => include_str!("../resources/javascript.ascii"),
            Language::Vue => include_str!("../resources/vue.ascii"),
            Language::Perl => include_str!("../resources/perl.ascii"),
            Language::Php => include_str!("../resources/php.ascii"),
            Language::Zig => include_str!("../resources/zig.ascii"),
            Language::Unknown => include_str!("../resources/unknown.ascii"),
            // _ => include_str!("../resources/unknown.ascii"),
        }
    }

    fn colors(&self) -> Vec<Color> {
        let language = if let Language::Unknown = self.custom_logo {
            &self.dominant_language
        } else {
            &self.custom_logo
        };

        match language {
            Language::Assembly => vec![Color::Cyan],
            Language::C => vec![Color::BrightBlue, Color::Blue],
            Language::Clojure => vec![Color::BrightBlue, Color::BrightGreen],
            Language::CoffeeScript => vec![Color::Red],
            Language::Cpp => vec![Color::Yellow, Color::Cyan],
            Language::Csharp => vec![Color::White],
            Language::CSS => vec![Color::Blue, Color::White],
            Language::Dart => vec![Color::Blue, Color::Cyan],
            Language::Elm => vec![Color::BrightBlack, Color::Green, Color::Yellow, Color::Cyan],
            Language::Erlang => vec![Color::BrightRed],
            Language::Forth => vec![Color::BrightRed],
            Language::Go => vec![Color::White],
            Language::Haskell => vec![Color::BrightBlue, Color::BrightMagenta, Color::Blue],
            Language::HTML => vec![Color::Red, Color::White],
            Language::Idris => vec![Color::Red],
            Language::Java => vec![Color::BrightBlue, Color::Red],
            Language::Kotlin => vec![Color::Blue, Color::Yellow, Color::Magenta],
            Language::Lisp => vec![Color::Yellow],
            Language::Lua => vec![Color::Blue],
            Language::Nim => vec![Color::Yellow, Color::BrightWhite],
            Language::ObjectiveC => vec![Color::BrightBlue, Color::Blue],
            Language::PureScript => vec![Color::White],
            Language::Python => vec![Color::Blue, Color::Yellow],
            Language::R => vec![Color::White, Color::Blue],
            Language::Ruby => vec![Color::Magenta],
            Language::Rust => vec![Color::White, Color::BrightRed],
            Language::Scala => vec![Color::Blue],
            Language::Shell => vec![Color::Green],
            Language::Swift => vec![Color::BrightRed],
            Language::Tcl => vec![Color::Blue, Color::White, Color::BrightBlue],
            Language::TypeScript => vec![Color::Cyan],
            Language::JavaScript => vec![Color::BrightYellow],
            Language::Vue => vec![Color::BrightGreen, Color::Blue],
            Language::Perl => vec![Color::BrightBlue],
            Language::Php => vec![Color::BrightWhite],
            Language::Zig => vec![Color::Yellow],
            Language::Unknown => vec![Color::White],
        }
    }
}

impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();
        let color = match self.colors().get(0) {
            Some(&c) => c,
            None => Color::White,
        };

        writeln!(
            buffer,
            "{}{}",
            "Project: ".color(color).bold(),
            self.project_name
        )?;

        writeln!(
            buffer,
            "{}{}",
            "HEAD: ".color(color).bold(),
            self.current_commit
        )?;

        writeln!(
            buffer,
            "{}{}",
            "Version: ".color(color).bold(),
            self.version
        )?;

        writeln!(
            buffer,
            "{}{}",
            "Created: ".color(color).bold(),
            self.creation_date
        )?;

        if !self.languages.is_empty() {
            if self.languages.len() > 1 {
                let title = "Languages: ";
                let pad = " ".repeat(title.len());
                let mut s = String::from("");
                for (cnt, language) in self.languages.iter().enumerate() {
                    let formatted_number = format!("{:.*}", 2, language.1);
                    if cnt != 0 && cnt % 3 == 0 {
                        s = s + &format!("\n{}{} ({} %) ", pad, language.0, formatted_number);
                    } else {
                        s = s + &format!("{} ({} %) ", language.0, formatted_number);
                    }
                }
                writeln!(buffer, "{}{}", title.color(color).bold(), s)?;
            } else {
                let title = "Language: ";
                writeln!(
                    buffer,
                    "{}{}",
                    title.color(color).bold(),
                    self.dominant_language
                )?;
            };
        }

        if !self.authors.is_empty() {
            let title = if self.authors.len() > 1 {
                "Authors: "
            } else {
                "Author: "
            };

            writeln!(
                buffer,
                "{}{}% {} {}",
                title.color(color).bold(),
                self.authors[0].2,
                self.authors[0].0,
                self.authors[0].1
            )?;

            let title = " ".repeat(title.len());

            for author in self.authors.iter().skip(1) {
                writeln!(
                    buffer,
                    "{}{}% {} {}",
                    title.color(color).bold(),
                    author.2,
                    author.0,
                    author.1
                )?;
            }
        }

        writeln!(
            buffer,
            "{}{}",
            "Last change: ".color(color).bold(),
            self.last_change
        )?;

        writeln!(buffer, "{}{}", "Repo: ".color(color).bold(), self.repo)?;
        writeln!(
            buffer,
            "{}{}",
            "Commits: ".color(color).bold(),
            self.commits
        )?;
        writeln!(
            buffer,
            "{}{}",
            "Lines of code: ".color(color).bold(),
            self.number_of_lines
        )?;
        writeln!(buffer, "{}{}", "Size: ".color(color).bold(), self.repo_size)?;
        writeln!(
            buffer,
            "{}{}",
            "License: ".color(color).bold(),
            self.license
        )?;

        let logo = self.get_ascii();
        let mut logo_lines = logo.lines();
        let mut info_lines = buffer.lines();
        let left_pad = logo.lines().map(|l| true_len(l)).max().unwrap_or(0);

        for _ in 0..cmp::max(count_newlines(logo), count_newlines(&buffer)) {
            let logo_line = match logo_lines.next() {
                Some(line) => line,
                None => "",
            };

            let info_line = match info_lines.next() {
                Some(line) => line,
                None => "",
            };

            let (logo_line, extra_pad) = colorize_str(logo_line, self.colors());
            // If the string is empty the extra padding should not be added
            let pad = if logo_line.is_empty() {
                left_pad
            } else {
                left_pad + extra_pad
            };
            writeln!(f, "{:<width$} {:^}", logo_line, info_line, width = pad,)?;
        }

        Ok(())
    }
}
