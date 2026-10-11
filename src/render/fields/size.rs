use super::FieldView;
use crate::info::fields::size::Size;
use crate::render::RenderOptions;
use crate::render::style::Line;
use byte_unit::{Byte, UnitType};

impl FieldView for Size {
    fn value(&self, options: &RenderOptions) -> Vec<Line> {
        let repo_size = bytes_to_human_readable(self.repo_size);
        let size = match self.file_count {
            0 => repo_size,
            1 => format!("{repo_size} (1 file)"),
            _ => format!("{repo_size} ({} files)", options.number(&self.file_count)),
        };
        vec![Line::from(size)]
    }

    fn key(&self) -> String {
        "Size".into()
    }
}

fn bytes_to_human_readable(bytes: u64) -> String {
    let byte = Byte::from_u64(bytes);
    let adjusted_byte_based = byte.get_appropriate_unit(UnitType::Binary);
    format!("{adjusted_byte_based:#.2}")
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(123, "2.46 MiB (123 files)")]
    #[case(0, "2.46 MiB")]
    #[case(1, "2.46 MiB (1 file)")]
    fn test_display_size(#[case] file_count: u64, #[case] expected: &str) {
        let size = Size {
            repo_size: 2_577_152,
            file_count,
        };

        assert_eq!(
            size.value(&RenderOptions::default()),
            vec![Line::from(expected)]
        );
    }

    #[rstest(
        case(0, "0 B"),
        case(1023, "1023 B"),
        case(1024, "1 KiB"),
        case(2048, "2 KiB"),
        case(1048576, "1 MiB"),
        case(1099511627776, "1 TiB"),
        case(2577152, "2.46 MiB")
    )]
    fn test_bytes_to_human_readable(#[case] input: u64, #[case] expected: &str) {
        assert_eq!(bytes_to_human_readable(input), expected);
    }
}
