use super::utils::format_number;
use crate::info::langs::get_total_size;
use crate::info::langs::language::Language;
use crate::{cli::NumberSeparator, info::utils::info_field::InfoField};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlobSizeInfo {
    pub size: usize,
    #[serde(skip_serializing)]
    number_separator: NumberSeparator,
}

impl BlobSizeInfo {
    pub fn new(size_by_language: &[(Language, usize)], number_separator: NumberSeparator) -> Self {
        let size = get_total_size(size_by_language);
        Self {
            size,
            number_separator,
        }
    }
}

#[typetag::serialize]
impl InfoField for BlobSizeInfo {
    fn value(&self) -> String {
        format_number(&self.size, self.number_separator)
    }

    fn title(&self) -> String {
        "Total blob size".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_loc_info() {
        let loc_info = BlobSizeInfo {
            size: 1235,
            number_separator: NumberSeparator::Plain,
        };

        assert_eq!(loc_info.value(), "1235".to_string());
    }
}
