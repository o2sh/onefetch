extern crate askalono;

use std::fs::File;
use std::path::Path;

use askalono::Store;

const EMBEDDED_CACHE: &str = "resources/licenses/cache.bin.gz";

fn main() {
    if Path::new(EMBEDDED_CACHE).exists() {
        println!("cargo:warning=onefetch license cache file already exists; not re-building");
        return;
    }

    let mut store = Store::new();
    store
        .load_spdx(Path::new("resources/licenses/spdx/json/details"), false)
        .expect("Couldn't create a store from SPDX data. Have submodules been initialized?");

    let mut cache = File::create(EMBEDDED_CACHE).unwrap();
    store.to_cache(&mut cache).unwrap();
}
