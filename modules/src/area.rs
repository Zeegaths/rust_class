pub struct Rectangle {
    height: u32,
    width: u32,
}


impl Rectangle {
    pub fn new(height: u32, width: u32) -> Self {
        return Rectangle{height, width};
    }
    pub fn area (&self) -> u32 {
        return self.height * self.width;
    }
}
