use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Group {
    pub id: usize,
    pub has_en: bool,
}

impl Group {
    pub fn new(id: usize, has_en: bool) -> Group {
        Group { id, has_en }
    }
}
