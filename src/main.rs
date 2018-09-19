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
                
        let logo= self.get_ascii();
        let mut lines = s.lines();
        let left_pad = logo.lines().map(|l| l.len()).max().unwrap_or(0);
        let mut o = String::new();
        for a in logo.lines() {
            let b = match lines.next() {
                Some(line) => line,
                None => "",
            };
            o.push_str(&format!("{:width$} {}\n", a, b, width = left_pad));
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
       }
    }
}

fn main() {
let info = Info { 
    project_name: String::from("onefetch"),
    language: Language::Java,
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

        let rust="
             `  :y.`yy`.y:  `            
         -``MNsNMMNNNNMMNsNM``-         
      ` -MMNMMMMNNm``NNNMMMMNMM- `      
     `NNNMMMdo:` `+md/  `:odMMMNNN`     
   -ssNMMNo.                .oNMMNss-   
   `mMMMMNmmmmmmmmmmmmmmmdy+` `sMMMm`   
 `mMMMMMMMMMMMMMMMMMMMMMMMMMN/  hMMMMm` 
 -oMN-:Ny:mMMMMMm/////oNMMMMMm oN::MMo- 
.yMMMhhh+ dMMMMMd:::::+mMMMMN/ odyhMMMy.
-sNMMy    dMMMMMMMMMMMMMMMMs`    `yMMNs-
-sNMMy    dMMMMMNyyyydMMMMMMy   .odMMNs-
.yMMMm````dMMMMMh     +MMMMMM+``sMMMMMy.
 -oMMMMMMMMMMMMMMMMM+  mMMMMMMMMMMMMMo- 
 `mMMMMMMMMMMMMMMMMM+  :NMMMMMMMMMMMMm` 
   `mMMMm//+o-------`   `-:o+:/mMMMm`   
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
            _ => unknown
        }
    }
}
