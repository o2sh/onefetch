use std::sync::LazyLock;

use anyhow::Context as _;

use i18n_embed::fluent::FluentLanguageLoader;
use i18n_embed::DefaultLocalizer;
use i18n_embed::DesktopLanguageRequester;
use i18n_embed::LanguageLoader;
use i18n_embed::Localizer as _;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "__locales_compiled"]
struct Localizations;

pub static LOADER: LazyLock<FluentLanguageLoader> = LazyLock::new(|| {
    let loader = i18n_embed::fluent::fluent_language_loader!();
    loader.load_fallback_language(&Localizations).unwrap();
    loader
});

fn localizer() -> DefaultLocalizer<'static> {
    DefaultLocalizer::new(&*LOADER, &Localizations)
}

pub fn init() -> anyhow::Result<()> {
    let localizer = localizer();
    let requested = DesktopLanguageRequester::requested_languages();
    localizer
        .select(&requested)
        .context("Failed to set languages")?;
    Ok(())
}

// TODO: suppress warning "`macro` is experimental"
#[macro_vis::macro_vis(pub)]
macro_rules! tr {
    ($id:literal) => {{
        i18n_embed_fl::fl!($crate::i18n::LOADER, $id)
    }};

    ($id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::i18n::LOADER, $id, $($args),*)
    }}
}
