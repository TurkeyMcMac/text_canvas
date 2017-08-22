pub mod utils;

#[cfg(test)]
mod tests {
    use super::*;
    use super::utils::*;
    use colors::*;

    #[test]
    fn demonstration() {
        let mut can = Canvas::new(10, 10, pix('-', RED, BLUE));

        println!("\n{}", can);
       
        can.fill(pix('"', YELLOW, GREEN));

        can.dot(3, 4, pix('*', RED, GREEN));

        can.text("foohfjhfdjhf54rtgttrtgtr\nbar\nxyzzy\rbaz", 1, 1, Color::new(GREEN, BLACK), true);

        can.text("a\nb\nc", 0, 5, Color::new(YELLOW, GREEN), false);

        println!("{}", can);
    }

    #[test]
    fn getting_works() {
        let can = Canvas::new(10, 10, pix('-', RED, BLUE));
        
        assert!(can.get(9, 9).is_some());

        assert!(can.get(10, 10).is_none());

        assert!(can.get(10, 9).is_none());

        assert!(can.get(9, 10).is_none());
    }
}

use std::fmt;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>,
}

impl Canvas {
    pub fn new(width: usize, height: usize, filler: Pixel) -> Canvas {
        Canvas {
            width, height,
            pixels: vec![filler; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Pixel> {
        if x < self.width && y < self.height {
            Some(unsafe {
                self.pixels.get_unchecked(y * self.width + x)
            })
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Pixel> {
        if x < self.width && y < self.height {
            Some(unsafe {
                self.pixels.get_unchecked_mut(y * self.width + x)
            })
        } else {
            None
        }
    }
    
    pub unsafe fn get_unchecked(&self, x: usize, y: usize) -> &Pixel {
        self.pixels.get_unchecked(y * self.width + x)
    }
    
    pub unsafe fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut Pixel {
        self.pixels.get_unchecked_mut(y * self.width + x)
    }

    pub fn view(&self, x: usize, y: usize, width: usize, height: usize) -> String {
        self.pixels.chunks(self.width).skip(y).take(height)
            .map(|chunk| chunk.iter().skip(x).take(width).map(|pixel| pixel.to_string()).collect::<String>())
            .map(|row| format!("{}\u{001B}[0m\n", row))
            .collect::<String>()
    }

    pub fn fill(&mut self, filler: Pixel) {
        for pixel in self.pixels.iter_mut() {
            *pixel = filler;
        }
    }
}

impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.view(0, 0, self.width, self.height))
    }
}

#[derive(Clone, Copy)]
pub struct Pixel {
    icon: char,
    color: Color,
}

impl Pixel {
    pub fn new(icon: char, color: Color) -> Pixel {
        Pixel { icon, color }
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.color, self.icon)
    }
}

pub fn pix(icon: char, foreground: u8, background: u8) -> Pixel {
    Pixel { icon, color: Color::new(foreground, background) }
}

#[derive(Clone, Copy)]
pub struct Color {
    foreground: u8,
    background: u8,
}

impl Color {
    pub fn new(foreground: u8, background: u8) -> Color {
        Color {
            foreground: foreground,
            background: background,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{001B}[3{};4{}m", self.foreground as char, self.background as char)
    }
}

#[derive(Clone, Copy)]
pub enum ColorCode {
    Black, Red, Green, Yellow, Blue, Magenta, Cyan, White,
}

pub mod colors {
    pub const BLACK: u8   = b'0';
    pub const RED: u8     = b'1';
    pub const GREEN: u8   = b'2';
    pub const YELLOW: u8  = b'3';
    pub const BLUE: u8    = b'4';
    pub const MAGENTA: u8 = b'5';
    pub const CYAN: u8    = b'6';
    pub const WHITE: u8   = b'7';
}
