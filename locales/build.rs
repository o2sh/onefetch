use std::{
    collections::BTreeMap,
    fs::{create_dir_all, read_to_string, File},
    io::{BufWriter, Write as _},
    path::PathBuf,
};

use quote::{format_ident, quote};

/// Concatenate all FTL files in the locales directory into a single file for each locale.
pub fn concat_locales() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=locales/");
    let locales_dir = PathBuf::from("locales").read_dir()?;
    let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("__locales_compiled");

    for dir in locales_dir {
        let dir = dir?.path();
        if !dir.is_dir() {
            continue;
        }
        let lang = dir.components().last().unwrap();

        let out_dir = out_dir.join(lang);
        create_dir_all(&out_dir).unwrap();

        let mut out_file = BufWriter::new(File::create(out_dir.join("onefetch.ftl"))?);

        for ftl in dir.read_dir()? {
            let ftl = ftl?.path();
            let contents = read_to_string(&ftl)?;
            writeln!(out_file, "{contents}")?;
        }
    }
    Ok(())
}

/// Generate Rust constants for all localization keys.
// WARNING: AI slop
pub fn generate_consts(out: &String) -> Result<(), Box<dyn std::error::Error>> {
    let ftl_contents = read_to_string(
        PathBuf::from("__locales_compiled")
            .join("en-US")
            .join("onefetch.ftl"),
    )?;
    let out_path = PathBuf::from(out).join("locales_consts.rs");
    let mut out_file = BufWriter::new(File::create(&out_path)?);

    let mut keys = Vec::new();
    for line in ftl_contents.lines() {
        if !line.trim().contains('=') {
            continue;
        }
        let (key, ..) = line.split_once('=').unwrap();
        keys.push(key.trim().to_string());
    }

    // Используем struct вместо type alias
    #[derive(Default)]
    struct Module {
        constants: Vec<(String, String)>,
        submodules: BTreeMap<String, Module>,
    }

    let mut root = Module::default();

    for key in &keys {
        let parts: Vec<&str> = key.split('-').collect();
        let const_name = parts.last().unwrap().to_uppercase();

        let mut current = &mut root;

        // Проходим по всем частям пути (кроме последней)
        for part in parts.iter().take(parts.len() - 1) {
            current = current
                .submodules
                .entry(part.to_string())
                .or_insert_with(Module::default);
        }

        // Добавляем константу
        current.constants.push((const_name, key.clone()));
    }

    // Рекурсивная генерация кода
    fn generate_tree(module: &Module) -> proc_macro2::TokenStream {
        let mut items = Vec::new();

        // Добавляем константы текущего модуля
        for (const_name, const_value) in &module.constants {
            let const_ident = format_ident!("{}", const_name);
            items.push(quote! {
                pub const #const_ident: &str = #const_value;
            });
        }

        // Добавляем подмодули
        for (mod_name, submodule) in &module.submodules {
            let mod_ident = format_ident!("{}", mod_name);
            let submodule_code = generate_tree(submodule);

            items.push(quote! {
                pub mod #mod_ident {
                    #submodule_code
                }
            });
        }

        quote! { #(#items)* }
    }

    let module_code = generate_tree(&root);

    let output = quote! {
        pub mod locale_keys {
            #module_code
        }
    };

    writeln!(out_file, "#[rustfmt::skip]")?;
    write!(out_file, "{}", output)?;

    Ok(())
}
