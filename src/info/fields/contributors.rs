use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Contributors {
    pub total_number_of_authors: usize,
}

impl Contributors {
    pub fn new(total_number_of_authors: usize) -> Self {
        Self {
            total_number_of_authors,
        }
    }
}
