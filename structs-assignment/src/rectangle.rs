//creates a rectangle struct
pub struct Rectangle {
    height: u32,
    width: u32,
}


//implementation of the triangle
impl Rectangle{
    //sets an instance of the rectangle stuct
    pub fn new(height:u32, width:u32) -> Self {
        return Rectangle{height, width};
    }

    //defines the area function
    pub fn area(&self) -> u32 {
        self.height * self.width
    }
}