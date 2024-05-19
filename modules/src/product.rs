pub struct Book {
    name: String,
    category: String,
    status: bool,
}


impl Book {
    pub fn new(name: String, category: String, status: bool) -> Self {
        return Book{name, category, status};
    }
    pub fn if_available (&self) -> bool {
        return self.status;
    }
}