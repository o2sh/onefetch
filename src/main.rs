extern crate colored;
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
        let mut s = String::new();
        s.push_str(&("Project: ".blue().bold().to_string() + &format!("{}\n", self.project_name)));
        s.push_str(&("Language: ".blue().bold().to_string() + &format!("{}\n", self.language)));
        s.push_str(&("Author: ".blue().bold().to_string() + &format!("{}\n", self.author)));
        s.push_str(&("Repo: ".blue().bold().to_string() + &format!("{}\n", self.repo)));
        s.push_str(&("Number of lines: ".blue().bold().to_string() + &format!("{}\n", self.number_of_lines)));
        s.push_str(&("License: ".blue().bold().to_string() + &format!("{}\n", self.license)));
        s.push_str(&self.get_ascii().blue().bold().to_string());
        write!(f, "{}", s)
    }
}

enum Language {
    Rust,
    Go,
    Java,
    Cpp,
    C,
    Javascript,
    Python,
    Csharp,
    Scala,
    Shell,
    Lisp,
    Haskell,
    Ruby,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
          Language::Rust => write!(f, "Rust"),
          Language::Go => write!(f, "Rust"),
          Language::Java => write!(f, "Rust"),
          Language::Cpp => write!(f, "Rust"),
          Language::C => write!(f, "Rust"),
          Language::Javascript => write!(f, "Rust"),
          Language::Python => write!(f, "Rust"),
          Language::Csharp => write!(f, "Rust"),
          Language::Scala => write!(f, "Rust"),
          Language::Shell => write!(f, "Rust"),
          Language::Lisp => write!(f, "Rust"),
          Language::Haskell => write!(f, "Rust"),
          Language::Ruby => write!(f, "Rust"),
       }
    }
}

fn main() {
let info = Info { 
    project_name: String::from("onefetch"),
    language: Language::Rust,
    author: String::from("Ossama Hjaji"),
    repo: String::from("https://github.com/02sh/onefetch"),
    number_of_lines: 15656, 
    license: String::from("MIT"),
};

println!("{}", info);

//let left_pad = ascii.lines().map(|l| l.len()).max().unwrap_or(0) + 5; 
//for (a,b) in ascii.lines().zip(info.lines()) {
//    println!("{:width$} {}", a, b, width = left_pad);
//}
}


impl Info {
    pub fn get_ascii(&self) -> &str {

        let ascii="
                   `:+ssssossossss+-`
                .oys///oyhddddhyo///sy+.
              /yo:+hNNNNNNNNNNNNNNNNh+:oy/
            :h/:yNNNNNNNNNNNNNNNNNNNNNNy-+h:
          `ys.yNNNNNNNNNNNNNNNNNNNNNNNNNNy.ys
         `h+-mNNNNNNNNNNNNNNNNNNNNNNNNNNNNm-oh
         h+-NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNN.oy
        /d`mNNNNNNN/::mNNNd::m+:/dNNNo::dNNNd`m:
        h//NNNNNNN: . .NNNh  mNo  od. -dNNNNN:+y
        N.sNNNNNN+ -N/ -NNh  mNNd.   sNNNNNNNo-m
        N.sNNNNNs  +oo  /Nh  mNNs` ` /mNNNNNNo-m
        h//NNNNh  ossss` +h  md- .hm/ `sNNNNN:+y
        :d`mNNN+/yNNNNNd//y//h//oNNNNy//sNNNd`m-
         yo-NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNm.ss
         `h+-mNNNNNNNNNNNNNNNNNNNNNNNNNNNNm-oy
           sy.yNNNNNNNNNNNNNNNNNNNNNNNNNNs.yo
            :h+-yNNNNNNNNNNNNNNNNNNNNNNs-oh-
              :ys:/yNNNNNNNNNNNNNNNmy/:sy:
                .+ys///osyhhhhys+///sy+.
                    -/osssossossso/-";

            match self.language {
                  Language::Rust => ascii,
                  Language::Go => ascii, 
                  Language::Java => ascii, 
                  Language::Cpp => ascii, 
                  Language::C => ascii, 
                  Language::Javascript => ascii, 
                  Language::Python => ascii, 
                  Language::Csharp => ascii,
                  Language::Scala => ascii,
                  Language::Shell => ascii,
                  Language::Lisp => ascii,
                  Language::Haskell => ascii,
                  Language::Ruby =>  ascii,
                  _ => "none"
            }
    }
}
