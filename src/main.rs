extern crate canvas;
use canvas::*;
use utils::*;
use ColorCode::*;

fn main() {
    let mut can = Canvas::new(15, 15, pix('~', Red, Green));
    
    can.text("foojfhddddhjfdhjfdhjfhjfdhjdfhjfdhjdf\nbar\nxyzzy\rbaz", 1, 2, Color::new(Cyan, Black));
    
    println!("{}", can);
}
