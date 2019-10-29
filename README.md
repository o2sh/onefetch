<p align="center">
  <img src="https://raw.githubusercontent.com/o2sh/onefetch/master/assets/onefetch.png" height="130px"></h3><br><br>
  <a href="https://crates.io/crates/onefetch"><img src="https://img.shields.io/badge/crates.io-1.7.0-orange.svg" alt="cargo"></a>
  <a href="https://travis-ci.org/o2sh/onefetch"><img src="https://travis-ci.org/o2sh/onefetch.svg?branch=master" alt="Build Status"></a>
  <a href="./LICENSE.md"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
</p>

<p align="center">
  <a href="https://snapcraft.io/onefetch"><img src="https://raw.githubusercontent.com/snapcore/snap-store-badges/master/EN/%5BEN%5D-snap-store-black.png" alt="Get it from the Snap Store"></a>
</p>

Onefetch is a command line tool that displays information about your Git project directly on your terminal. Onefetch supports almost 50 different programming languages. If your language of choice isn't supported: Open up an issue and support will be added.

<p align="center">
<img src="https://raw.githubusercontent.com/o2sh/onefetch/master/assets/cpp.png" align="left" height="217px" width="420px">
<img src="https://raw.githubusercontent.com/o2sh/onefetch/master/assets/kitty.png" height="217px" width="420px">
</p>

## Installation

### Unix

Clone the repository and install to `/usr/local/bin` with `make install`.

Or get the binary from [here](https://github.com/o2sh/onefetch/releases) and add it to your $PATH.

### Windows

Clone the repository and build with `cargo build --release`. Add the resulting binary to your $PATH.

## Usage

```sh
> onefetch -d /path/of/your/repo
 ```
 Or
 
```sh
> cd /path/of/your/repo
> onefetch
```

## Compatibility

Only works with Git repositories.
