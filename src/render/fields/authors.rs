use super::FieldView;
use crate::info::fields::authors::Authors;
use crate::render::format::DisplayOptions;
use crate::render::text::Line;

impl FieldView for Authors {
    fn value(&self, options: &DisplayOptions) -> Vec<Line> {
        let top_contribution = self.authors.first().map_or(0, |a| a.contribution);
        self.authors
            .iter()
            .map(|author| {
                let pad = digit_difference(top_contribution, author.contribution);
                let contribution = format!("{:pad$}{}%", "", author.contribution);
                let commits = options.number(&author.nbr_of_commits);
                let line = match &author.email {
                    Some(email) => format!("{contribution} {} <{email}> {commits}", author.name),
                    None => format!("{contribution} {} {commits}", author.name),
                };
                Line::from(line)
            })
            .collect()
    }

    fn key(&self) -> String {
        let mut title: String = "Author".into();
        if self.authors.len() > 1 {
            title.push('s');
        }
        title
    }
}

fn digit_difference(num1: usize, num2: usize) -> usize {
    let count_digits = |num: usize| (num.checked_ilog10().unwrap_or(0) + 1) as usize;
    count_digits(num1).abs_diff(count_digits(num2))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::info::fields::authors::Author;
    use rstest::rstest;

    fn john_and_roberto() -> Vec<Author> {
        vec![
            Author::new(
                "John Doe".into(),
                Some("john.doe@email.com".into()),
                1500,
                2000,
            ),
            Author::new("Roberto Berto".into(), None, 240, 300),
        ]
    }

    #[test]
    fn test_authors_title_with_one_author() {
        let mut authors = john_and_roberto();
        authors.truncate(1);
        assert_eq!(Authors { authors }.key(), "Author");
    }

    #[test]
    fn test_authors_title_with_two_authors() {
        let authors = Authors {
            authors: john_and_roberto(),
        };
        assert_eq!(authors.key(), "Authors");
    }

    #[test]
    fn test_display_two_authors() {
        let authors = Authors {
            authors: john_and_roberto(),
        };
        assert_eq!(
            authors.value(&DisplayOptions::default()),
            vec![
                Line::from("75% John Doe <john.doe@email.com> 1500"),
                Line::from("80% Roberto Berto 240"),
            ]
        );
    }

    #[test]
    fn test_display_authors_alignment_with_three_authors() {
        let mut authors = john_and_roberto();
        authors.push(Author::new("Jane Doe".into(), None, 1, 100));
        assert_eq!(
            Authors { authors }.value(&DisplayOptions::default()),
            vec![
                Line::from("75% John Doe <john.doe@email.com> 1500"),
                Line::from("80% Roberto Berto 240"),
                Line::from(" 1% Jane Doe 1"),
            ]
        );
    }

    #[rstest]
    #[case(456, 123, 0)]
    #[case(456789, 123, 3)]
    #[case(1, 12, 1)]
    fn test_digit_difference(#[case] num1: usize, #[case] num2: usize, #[case] expected: usize) {
        assert_eq!(digit_difference(num1, num2), expected);
    }
}
