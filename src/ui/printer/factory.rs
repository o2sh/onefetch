use super::Printer;
use crate::cli::CliOptions;
use crate::info::langs::language::Language;
use crate::info::Info;
use crate::ui::printer::{PrinterType, SerializationFormat};
use anyhow::{Context, Result};
use image::DynamicImage;
use onefetch_image::ImageBackend;

pub struct PrinterFactory {
    pub output: Option<SerializationFormat>,
    pub info: Info,
    image: Option<DynamicImage>,
    pub no_bold: bool,
    pub art_off: bool,
    image_backend: Option<Box<dyn ImageBackend>>,
    color_resolution: usize,
    ascii_input: Option<String>,
    ascii_language: Option<Language>,
}

impl PrinterFactory {
    pub fn new(info: Info, cli_options: CliOptions) -> Result<Self> {
        let image =
            match cli_options.image.image {
                Some(p) => Some(image::open(&p).with_context(|| {
                    format!("Could not load the image file at '{}'", p.display())
                })?),
                None => None,
            };

        let image_backend = if image.is_some() {
            cli_options
                .image
                .image_protocol
                .map_or_else(onefetch_image::get_best_backend, |s| {
                    onefetch_image::get_image_backend(s)
                })
        } else {
            None
        };

        Ok(Self {
            output: cli_options.developer.output,
            info,
            image,
            no_bold: cli_options.text_formatting.no_bold,
            art_off: cli_options.visuals.no_art,
            image_backend,
            color_resolution: cli_options.image.color_resolution,
            ascii_input: cli_options.ascii.ascii_input,
            ascii_language: cli_options.ascii.ascii_language,
        })
    }

    pub fn create(self) -> Result<Printer> {
        let PrinterFactory {
            output,
            info,
            image,
            no_bold,
            art_off,
            image_backend,
            color_resolution,
            ascii_input,
            ascii_language,
        } = self;

        match output {
            Some(SerializationFormat::Json) => Ok(Printer {
                r#type: PrinterType::Json,
                info,
            }),
            Some(SerializationFormat::Yaml) => Ok(Printer {
                r#type: PrinterType::Yaml,
                info,
            }),
            None => {
                if art_off || (ascii_input.is_none() && info.dominant_language.is_none()) {
                    Ok(Printer {
                        r#type: PrinterType::Plain,
                        info,
                    })
                } else if let Some(image) = image {
                    Ok(Printer {
                        r#type: PrinterType::Image {
                            image,
                            backend: image_backend.context("No supported image backend")?,
                            resolution: color_resolution,
                        },
                        info,
                    })
                } else {
                    let ascii_art = ascii_input.unwrap_or_else(|| {
                        let language = if let Some(lang) = &ascii_language {
                            lang
                        } else {
                            &info.dominant_language.unwrap()
                        };
                        language.get_ascii_art().to_string()
                    });

                    Ok(Printer {
                        r#type: PrinterType::Ascii {
                            art: ascii_art.to_string(),
                            no_bold,
                        },
                        info,
                    })
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cli::CliOptions,
        info::{langs::language::Language, Info},
        ui::printer::{factory::PrinterFactory, PrinterType, SerializationFormat},
    };
    use image::DynamicImage;

    #[test]
    fn test_create_json_printer() {
        let info = Info::default();
        let mut options = CliOptions::default();
        options.developer.output = Some(SerializationFormat::Json);

        let factory = PrinterFactory::new(info, options).unwrap();
        let printer = factory.create().unwrap();

        assert_eq!(printer.r#type, PrinterType::Json);
    }

    #[test]
    fn test_create_yaml_printer() {
        let info = Info::default();
        let mut options = CliOptions::default();
        options.developer.output = Some(SerializationFormat::Yaml);

        let factory = PrinterFactory::new(info, options).unwrap();
        let printer = factory.create().unwrap();

        assert_eq!(printer.r#type, PrinterType::Yaml);
    }

    #[test]
    fn test_create_plain_printer_when_no_art() {
        let mut info = Info::default();
        info.dominant_language = Some(Language::Rust);
        let mut options = CliOptions::default();
        options.visuals.no_art = true;

        let factory = PrinterFactory::new(info, options).unwrap();
        let printer = factory.create().unwrap();

        assert_eq!(printer.r#type, PrinterType::Plain);
    }

    #[test]
    fn test_create_plain_printer_when_no_dominant_language_no_ascii_input() {
        let info = Info::default();
        let options = CliOptions::default();

        let factory = PrinterFactory::new(info, options).unwrap();
        let printer = factory.create().unwrap();

        assert_eq!(printer.r#type, PrinterType::Json);
    }

    #[test]
    fn test_create_ascii_printer_when_dominant_language() {
        let mut info = Info::default();
        info.dominant_language = Some(Language::Rust);
        let options = CliOptions::default();

        let factory = PrinterFactory::new(info, options).unwrap();
        let printer = factory.create().unwrap();

        assert!(matches!(printer.r#type, PrinterType::Ascii { .. }));
    }

    pub struct DummyBackend {}
    impl DummyBackend {
        pub fn new() -> Self {
            Self {}
        }
    }
    impl super::ImageBackend for DummyBackend {
        fn add_image(
            &self,
            _lines: Vec<String>,
            _image: &DynamicImage,
            _colors: usize,
        ) -> anyhow::Result<String> {
            Ok("foo".to_string())
        }
    }

    #[test]
    fn test_create_image_printer() {
        let mut factory = PrinterFactory {
            output: None,
            info: Info::default(),
            image: Some(DynamicImage::default()),
            no_bold: false,
            art_off: false,
            image_backend: Some(Box::new(DummyBackend::new())),
            color_resolution: 8,
            ascii_input: None,
            ascii_language: None,
        };

        factory.info.dominant_language = Some(Language::ABNF);
        let printer = factory.create().unwrap();

        assert!(matches!(printer.r#type, PrinterType::Image { .. }));
    }
}
