use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Contributors {
    pub total_number_of_authors: usize,
    /// The `--number-of-authors` cap. The count is only worth displaying when
    /// there are more contributors than the Authors field lists.
    #[serde(skip)]
    pub number_of_authors: usize,
}

impl Contributors {
    pub fn new(total_number_of_authors: usize, number_of_authors: usize) -> Self {
        Self {
            total_number_of_authors,
            number_of_authors,
        }
    }
}
