

pub struct Hashma {
    name: String,
    age: String,
    school: String,
}


impl Hashma {
    pub fn new(name: String, age: String, school: String)-> Self {
        return Self{name, age, school};
    }

    pub fn update(&self) -> &str {
        &self.name
    }

}
