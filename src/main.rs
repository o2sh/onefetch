fn main() {

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

let info= " 
o2sh@arch 
--------- 
OS: Arch Linux x86_64 
Host: 20HN005NFR ThinkPad X270 
Kernel: 4.18.5-arch1-1-ARCH 
Uptime: 32 mins 
Packages: 447 (pacman) 
Shell: bash 4.4.23 
Resolution: 1920x1080
Theme: Adwaita [GTK3] 
Icons: Adwaita [GTK3] 
Terminal: termite 
Terminal Font: inconsolata 12 
CPU: Intel i7-7500U (4) @ 3.500GHz
GPU: Intel HD Graphics 620 
Memory: 295MiB / 7743MiB 
";

//println!("{} {}", ascii, info);

let left_pad = ascii.lines().map(|l| l.len()).max().unwrap_or(0) + 5; 
for (a,b) in ascii.lines().zip(info.lines()) {
    println!("{:width$} {}", a, b, width = left_pad);
}
}
