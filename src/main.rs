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
    Shell,
    Lisp,
    Haskell,
    Ruby,
    Coffeescript,
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
         Language::Shell => "green",
         Language::Lisp => "yellow",
         Language::Haskell => "cyan",
         Language::Ruby => "magenta",
         Language::Coffeescript => "cyan",
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
          Language::Shell => write!(f, "Shell"),
          Language::Lisp => write!(f, "Lisp"),
          Language::Haskell => write!(f, "Haskell"),
          Language::Ruby => write!(f, "Ruby"),
          Language::Coffeescript => write!(f, "Coffeescript")
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

}


impl Info {
    pub fn get_ascii(&self) -> &str {

        let rust="
             `  :y.`yy`.y:  `
         -``MNsNMMNNNNMMNsNM``-
      ` -MMNMMMMNNm``NNNMMMMNMM- `
     `NNNMMMdo:` `+md/  `:odMMMNNN`
   -ssNMMNo.                .oNMMNss-
   `mMMMMNmmmmmmmmmmmmmmmdy+` `sMMMm`
 `mMMMMMMMMMMMMMMMMMMMMMMMMMN/  hMMMMm`
 -oMN-:Ny:mMMMMMm    oNMMMMMm oN::MMo-
.yMMMhhh+ dMMMMMd:::::+mMMMMN/ odyhMMMy.
-sNMMy    dMMMMMMMMMMMMMMMMs`    `yMMNs-
-sNMMy    dMMMMMNyyyydMMMMMMy   .odMMNs-
.yMMMm   dMMMMMh     +MMMMMM+  sMMMMMy.
 -oMMMMMMMMMMMMMMMMM+  mMMMMMMMMMMMMMo-
 `mMMMMMMMMMMMMMMMMM+  :NMMMMMMMMMMMMm`
   `mMMMm               `-:o+:/mMMMm`
   -ssNMMMyomo            smohMMMNss-
     `NNNMs+mN/-`      `-/Nd/yMNNN`
      ` -MMNMMMMMNmmmmNMMMMMNMM- `
         -``MNsNMMNMMNMMNsNM``-
            `  :y.`yy`.y:  `";

        let go="
               `..-/::::.::--
           `..` --   os+.s+.`
         -+-... :    ://  `:`
       .-/   sho ---...`    -.
     -./ :   .:/yNs.:        /
    -:/o .....-/`.:-/         :`
     :./        .-.:-          +``
       ..                       +-/
        .`                      +.
         -.             :-.:    ./
          ./           `.`.`     /
           `:                    /
            +`                   /
            `:                  .-
             :                  /
            `:                 -:.-
             /               .-...`
             --`         ...-
               ./.:-..---`
               `/--";

        let java="
                       -`
                      ho
                    /mM:
                 `omMN/  .:/.
               -yMMNs-odNh/
             `yMMMs`oMMy.
             +MMN- .MMm
             `mMh   dMMs
              `sMo   sMM+
                .s+   NN-
       :shmd+-`   `` /+.`.-``:sNd:
       /ydNMMMNNNNNNNmmdyo.    +Mm
          -sh-````    .-.     -mM+
          /hmMMNmmmNNMNmh+  .sdo.
           `+y+----::/++.  `.
     :oyy+-.ymMMMMMMMNmhs.     /`
    sMMM/`     `````        `:yM:
     :sdNMNdhyssooooossyyhmmds/` .+.
         `-::://+++///::-. `.:oyhs.
           `-://++ooosssyyyyo+:`";

        let cpp="


      `/+ooooo:
     .oss:.`.-`    +s-      .ss
     /ss+       /++os+++-.+++ss+++
     :sso`      .--os/--``--:ss---
     `/sso///+-    /+.      `++
       .:////:`


                                        ";

        let c="
                 `-/++/-`
            `.:++++++++++:.`
         .-/++++++++++++++++/-.
     `-:++++++++++++++++++++++++:-`
  .:/++++++++++++++++++++++++++++/:-.`
-++++++++++++++/:--...-:/++++++/:::::::.
++++++++++++/.            ./+/::::::::::
+++++++++++.                `:::::::::::
++++++++++`      -/+++:`     `::::::::::
+++++++++.      /++++++:``````-:::::::::
+++++++++      `++++/:::::::::::::::::::
+++++++++      `++/:::::::::::::::::::::
+++++++++`      ::::::::......::::::::::
+++++++++/       -:::::.     `::::::::::
++++++++++/`       ``        -::::::::::
++++++++++/:.`             .::::::::::::
-+++++++/::::::-.``````..-:::::::::::::.
 `-/++/:::::::::::::::::::::::::::::-.
     `.-::::::::::::::::::::::::-.`
         .-::::::::::::::::::-.
            `.-::::::::::-.`
                `-::::-`";


        let haskell="
 -ssssss+``:+++++/`
 `ossssso. -++++++-
   /ssssss: ./+++++:`
    -ssssss+``:+++++/.
     `ossssso. -++++++- `ossssssssssssss
       /ssssss: ./+++++:``+sssssssssssss
        -ssssss+``:+++++/. -::::::::::::
        -ssssss+``:+++++++-  .----------
       /ssssss: `/+++++++++:``+sssssssss
     `ossssso. -++++++/+++++/. :ssssssss
    -ssssss+``:+++++/. -/+++++-
   /ssssss: ./+++++:`   `/+++++/`
 `ossssso. -++++++-       :+++++/.
-ssssss+``:+++++/`         ./+++++-";


        let python="
              `.-::::::-.`
            :ososssssssssso-
           .ss` .ssssssssyyy.
           .sso+ssssssssyyyy-
           `::::::::syyyyyyy-
    `:ossssssssssssyyyyyyyyy-.----.`
   .ssssssssssssssyyyyyyyyyy-.------`
   +sssssssssssyyyyyyyyyyyys`.-------
   ssssssssssssssssssssso+:`.--------
   sssssssso-```...........----------
   osssssys`..-----------------------
   :sssyyy+`------------------------.
    :syyyy+`-----------------------`
      .-::-`--------`````````````
           `----------------`
           `-----------.  --`
            .-----------..--
              `..........``";


        let lisp="
              .------------`
         -oss+:-`         .---`
      `+dMMMMMMMMdo.          -:-
     /mMMMMMMMMMMMMNs` ````     .:.
   `yMMMMMMMMMMMMMMMMh :mNh.      -:
  `dh++dMMMMMMdooyMMMM: .dMm-      `/
  yMm` -NMMMMy` -mMMMM+  `NMN:      ./
 -MMMs  oMMMs` -NMMMMN.   NMMN:      +`
 oMMMM/  hMy  -NMMMMN/   :MMMMN-     .:
 sMMMMN- .y` `mMMMMy-   `mMM/mMm.    `+
 oMMMMMm.    sMMMN/    `hMM+ -NMh`   .:
 .MMMMMMd`  `NMMM:    `yMMo   /MMs   +`
  oMMMMMMd`  dMMm    `hMN+     yMM- .:
   sMMMMMMd. .dMm    :oo:      .oo/./
    +NMMMMMN/`.yM+                -:
     -yMMMMMMNNNMMo`            ./.
       -sNMMMMMMMMMm+.       `-:.
         `:sdNMMMMMMMMmhsso/:-
              .:+oossoo/:.";


        let scala="
                              `/
                      `.-:+oyhh
        `..-:://+ossyhhhhhhhhhh
      +hhhhhhhhhhhhhhhhhhhhhhhh
      +hhhhhhhhhhhhhhhhhhhhhhhy
      +hhhhhhhhhhhhhhhhhhhhyo/.
      +hhhhhhhhhyyso++/:-`    -
      :+/::-..`        `.-:+oyh
          `..--:/++osyyhhhhhhhh
      /syhhhhhhhhhhhhhhhhhhhhhh
      +hhhhhhhhhhhhhhhhhhhhhhhh
      +hhhhhhhhhhhhhhhhhhhhhs+-
      +hhhhhhhhhhhyyso+/:-.   .
      /o++/::-..`        .-:+sh
            ``.--:/++osyhhhhhhh
      :osyyhhhhhhhhhhhhhhhhhhhh
      +hhhhhhhhhhhhhhhhhhhhhhhh
      +hhhhhhhhhhhhhhhhhhhhhyo:
      +hhhhhhhhhhhhhysoo/:-.
      +yso++//:--.`";


        let csharp="

              `.-:::--.`
          ./oyhhhhhhhhhhyo/.
       `+yhhhhhhhhhhhhhhhhhhy/`
     `+hhhhhhhhhhhhhhhhhhhhhhhh+`
    -yhhhhhhhhhhhhhyhhhhhhhhhhhhy.
   :hhhhhhhhhy+-`    `-+yhhhhhs/.
  .hhhhhhhhh/           `+yo:` .`  .`
  ohhhhhhhh-                 -:ho-:ho:
  yhhhhhhhs                  :+hs//hs/
  hhhhhhhho                  -:ho-:ho-
  ohhhhhhhh.                 /+hs/+hs/
  .hhhhhhhhy-            :s/-  -.  -.
   /hhhhhhhhhs:.      ./shhhhyo:`
    :hhhhhhhhhhhhyssyhhhhhhhhhhhy.
     .shhhhhhhhhhhhhhhhhhhhhhhho`
       -ohhhhhhhhhhhhhhhhhhhho.
         `:+yhhhhhhhhhhhhy+:`
             `.-:////:-.`
                                        ";


        let shell="
      -yyyyy     +yyyy+          +yyyy+
      /MMMMM     yMMMMy          yMMMMy
      /MMMMM     yMMMMy          yMMMMy
 yMMMMMMMMMMMMMMMMMMMMMMMMMM     yMMMMy
 yMMMMMMMMMMMMMMMMMMMMMMMMMM     yMMMMy
 +yyyydMMMMMyyyyymMMMMmyyyyy     yMMMMy
      /MMMMM     yMMMMy          yMMMMy
      /MMMMM     yMMMMy          yMMMMy
 +yyyydMMMMMyyyyymMMMMmyyyyy     -////-
 yMMMMMMMMMMMMMMMMMMMMMMMMMM
 yMMMMMMMMMMMMMMMMMMMMMMMMMM
      /MMMMM     yMMMMy          yMMMMy
      /MMMMM     yMMMMy          yMMMMy
      -yyyyy     +yyyy+          +yyyy+
                                        ";

        let ruby="
                   `-::::--/osyyyhhyo/.
              `:oyhhyyyyy+./yhhhddddddo
           `/shhhhhhhhyyyy. .ohhhddddddo
         .ohhhhhhhhhhhhhyy-/++oyhhdddddd
       .shhhhhhhhhhhhhhhh/syyyyyyhhddddd
     `ohhhhhhhhhhhhhhhhh:ohhhhhhhhhhdddy
    -hhhhhhhhhhhhhhhhhy-:hhhhhhhdddddddo
   /ddddhhhhhhhhhhhhho` :dddddddddddddd+
  /dddddddhhhhhhhhhs:-:::hddddddddddddd:
 .hhddddddddhhhhhs++oooooyddddddddddddd.
 :syhhddddddddh++syyyyyyyyddddddddddddd
`-/osyhhdddho:/yhhhhhhhhhhhdddddddddddh
/o.-/+oo+/. -hhhhhhhhhhhhhhddddddddddds
oyo....---::ydddddddddddddddddddddddhh+
shhy++++oooshdddddddddddddddddddddddhh:
hhhhysssssshddddddddddddddddhhdddddddy.
hhhhhhyyyyhddddddddddddddhhhhhhhhddddy`
/hhdddhhhhhdddddddddddhhhhhhyyyyyyhddy
 +hhhhhddhddddddddddddhhhhhhhhyyyyysys
  `/osyhhhyyysssooo++///:::---..```     ";


        let Coffeescript="
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMWWWWWMMMMMMMMMMMMM
MMMMMMMMMMX0OO0XWN0xdolloodddONMMMMMMMMM
MMMNKNMMMWOldxxdoc:cloolx00x:,kWMMMMMNNM
MMKl;xXWMMWKkxxdxOKNWWKxdxxxdkXMMMWN0llK
MMNOolloodxkO0KKKXXXXXXXKKK00OkxxdooldON
MMN00XXKOkxddddddddddddddddddddxkOKXXK0N
MMNd,;loxkO0KKXXXXXXXXXXXXXXKK0Okxdl:,oX
MMM0c:lcc:;,',,;;;::::::;;;,,''......,OW
MW0oodl:;;;'........................'dNM
Wk;cOd'.............................oXMM
0:'dWNx,..........................'oXMMM
O;.lXMWO:........................;kNMMMM
Nd''l0NWXo'....................'lKWMMMMM
MWOo::oOXNx'...................oXMMMMMMM
MMMWNXXXNMNo'.................cKMMMMMMMM
MMMMMMMMMMMNOoc;,'.......';:okXMMMMMMMMM
MMMMMMMMMMMMMMWNK0OOOOOO0KXNWMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
                                        ";

        let unknown="

                -/oooo+:`
              :dmmmmmmmmms`
             :mmmm/` -dmmmh
             ymmmh    ommmm.
             -----   `dmmmm`
                    /dmmmd-
                  .hmmmms`
                 `dmmmh.
                 -mmmm.
                 `----
                 :ssss.
                 +mmmm-
                 :oooo.
                                        ";

        match self.language {
            Language::Rust => rust,
            Language::Go => go,
            Language::Java => java,
            Language::Cpp => cpp,
            Language::C => c,
            Language::Python => python,
            Language::Csharp => csharp,
            Language::Scala => scala,
            Language::Shell => shell,
            Language::Lisp => lisp,
            Language::Haskell => haskell,
            Language::Ruby =>  ruby,
            Language::Coffeescript => coffeescript,
            _ => unknown
        }
    }
}
