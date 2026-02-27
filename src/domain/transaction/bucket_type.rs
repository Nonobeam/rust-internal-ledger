pub struct UserType {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl UserType {
    pub fn new(id: String, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
        }
    }
}