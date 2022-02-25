#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub phone: u32
}

#[derive(Debug, Clone)]
pub enum Category {
    Image,
    Video
}

#[derive(Debug, Clone)]
pub struct Art {
    pub name: String,
    pub price: u64,
    pub owner: Person,
    pub creator: Person,
    pub category: Category
}
