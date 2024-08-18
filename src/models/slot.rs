use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Slot {
    pub id: usize,
    pub subject: String,
    pub day: u8,
    pub name: String,
}

impl Slot {
    pub fn new(id: usize, subject: &str, day: u8, name: &str) -> Slot {
        Slot {
            id,
            subject: subject.to_string(),
            day,
            name: name.to_string(),
        }
    }
}
