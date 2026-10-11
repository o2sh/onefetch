use super::FieldView;
use crate::info::fields::created::Created;
use crate::render::format::DisplayOptions;
use crate::render::text::Line;

impl FieldView for Created {
    fn value(&self, options: &DisplayOptions) -> Vec<Line> {
        vec![Line::from(options.time(self.creation_date))]
    }

    fn key(&self) -> String {
        "Created".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use gix::date::Time;

    #[test]
    fn test_display_created() {
        let created = Created {
            creation_date: Time::new(946_771_200, 0),
        };
        let iso_time = DisplayOptions {
            iso_time: true,
            ..DisplayOptions::default()
        };

        assert_eq!(
            created.value(&iso_time),
            vec![Line::from("2000-01-02T00:00:00Z")]
        );
    }
}
