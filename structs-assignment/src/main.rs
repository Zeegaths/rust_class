//import the rectangle module
mod rectangle;
use rectangle::Rectangle;

//populate the reactangle and call the area function
fn main() {
    let my_rectangle = Rectangle::new(5, 6);
    let my_area = my_rectangle.area();
    print!("The area is {}", my_area);    
}
