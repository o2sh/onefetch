use super::Printer;
use crate::cli::CliOptions;
use crate::info::Info;
use crate::ui::printer::ascii::AsciiPrinter;
use crate::ui::printer::image::ImagePrinter;
use crate::ui::printer::json::JsonPrinter;
use crate::ui::printer::yaml::YamlPrinter;
use crate::ui::printer::SerializationFormat;
use crate::{info::langs::language::Language, ui::printer::plain::PlainPrinter};
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

    pub fn create(self) -> Result<Box<dyn Printer>> {
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
            Some(SerializationFormat::Json) => Ok(Box::new(JsonPrinter { info })),
            Some(SerializationFormat::Yaml) => Ok(Box::new(YamlPrinter { info })),
            None => {
                if art_off || (ascii_input.is_none() && info.dominant_language.is_none()) {
                    Ok(Box::new(PlainPrinter { info }))
                } else if let Some(image) = image {
                    Ok(Box::new(ImagePrinter {
                        info,
                        image,
                        image_backend: image_backend.context("No supported image backend")?,
                        color_resolution,
                    }))
                } else {
                    let ascii_art = ascii_input.unwrap_or_else(|| {
                        let language = if let Some(lang) = &ascii_language {
                            lang
                        } else {
                            &info.dominant_language.unwrap()
                        };
                        language.get_ascii_art().to_string()
                    });

                    Ok(Box::new(AsciiPrinter {
                        info,
                        ascii_art: ascii_art.to_string(),
                        no_bold,
                    }))
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
        ui::printer::{
            ascii::AsciiPrinter, factory::PrinterFactory, image::ImagePrinter, json::JsonPrinter,
            plain::PlainPrinter, yaml::YamlPrinter, SerializationFormat,
        },
    };
    use image::DynamicImage;
    use std::any::Any;

    #[test]
    fn test_create_json_printer() {
        let info = Info::default();
        let mut options = CliOptions::default();
        options.developer.output = Some(SerializationFormat::Json);

        let factory = PrinterFactory::new(info, options).unwrap();
        let printer = factory.create().unwrap();
        let printer_ref = printer.as_ref() as &dyn Any;

        assert!(printer_ref.downcast_ref::<JsonPrinter>().is_some());
    }

    #[test]
    fn test_create_yaml_printer() {
        let info = Info::default();
        let mut options = CliOptions::default();
        options.developer.output = Some(SerializationFormat::Yaml);

        let factory = PrinterFactory::new(info, options).unwrap();
        let printer = factory.create().unwrap();
        let printer_ref = printer.as_ref() as &dyn Any;

        assert!(printer_ref.downcast_ref::<YamlPrinter>().is_some());
    }

    #[test]
    fn test_create_plain_printer_when_no_art() {
        let mut info = Info::default();
        info.dominant_language = Some(Language::Rust);
        let mut options = CliOptions::default();
        options.visuals.no_art = true;

        let factory = PrinterFactory::new(info, options).unwrap();
        let printer = factory.create().unwrap();
        let printer_ref = printer.as_ref() as &dyn Any;

        assert!(printer_ref.downcast_ref::<PlainPrinter>().is_some());
    }

    #[test]
    fn test_create_plain_printer_when_no_dominant_language_no_ascii_input() {
        let info = Info::default();
        let options = CliOptions::default();

        let factory = PrinterFactory::new(info, options).unwrap();
        let printer = factory.create().unwrap();
        let printer_ref = printer.as_ref() as &dyn Any;

        assert!(printer_ref.downcast_ref::<PlainPrinter>().is_some());
    }

    #[test]
    fn test_create_ascii_printer_when_dominant_language() {
        let mut info = Info::default();
        info.dominant_language = Some(Language::Rust);
        let options = CliOptions::default();

        let factory = PrinterFactory::new(info, options).unwrap();
        let printer = factory.create().unwrap();
        let printer_ref = printer.as_ref() as &dyn Any;

        assert!(printer_ref.downcast_ref::<AsciiPrinter>().is_some());
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
        let printer_ref = printer.as_ref() as &dyn Any;

        assert!(printer_ref.downcast_ref::<ImagePrinter>().is_some());
    }
}
