extern crate canvas;
use canvas::*;
use utils::*;
use ColorCode::*;

use std::thread;
use std::time::Duration;

fn main() {
    let mut can = Canvas::new(15, 15, pix('~', Red, Green));
    
    let mut x = 2; let mut y = 0;
    
    let width = 3; let height = 2;
    
    loop {
        can.rect(x, y, width, height, pix('#', Cyan, Black));
        
        println!("{}", can);

        x += 1; y += 1;

        thread::sleep(Duration::new(1, 0));
    }
}
