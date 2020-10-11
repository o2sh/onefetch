use {
    crate::onefetch::{image_backends::ImageBackend, info::InfoFieldOn, language::Language},
    image::DynamicImage,
};

/// Configuration options for *onefetch*.
pub struct Options {
    ///
    pub path: String,
    ///
    pub ascii_language: Language,
    ///
    pub ascii_colors: Vec<String>,
    ///
    pub disabled_fields: InfoFieldOn,
    ///
    pub no_bold: bool,
    ///
    pub image: Option<DynamicImage>,
    ///
    pub image_backend: Option<Box<dyn ImageBackend>>,
    ///
    pub no_merges: bool,
    ///
    pub no_color_blocks: bool,
    ///
    pub number_of_authors: usize,
    ///
    pub excluded: Vec<String>,
}
