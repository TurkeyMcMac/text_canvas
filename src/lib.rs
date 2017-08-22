pub mod utils;

#[cfg(test)]
mod tests {
    use super::*;
    use super::utils::*;
    use ColorCode::*;

    #[test]
    fn demonstration() {
        let mut can = Canvas::new(10, 10, pix('-', Red, Blue));

        println!("\n{}", can);
       
        can.fill(pix('"', Yellow, Green));

        can.dot(3, 4, pix('*', Red, Green));

        can.text("foohfjhfdjhf54rtgttrtgtr\nbar\nxyzzy\rbaz", 1, 1, Color::new(Green, Black), true);

        can.text("a\nb\nc", 0, 5, Color::new(Yellow, Green), false);

        println!("{}", can);
    }

    #[test]
    fn getting_works() {
        let can = Canvas::new(10, 10, pix('-', Red, Blue));
        
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

pub fn pix(icon: char, foreground: ColorCode, background: ColorCode) -> Pixel {
    Pixel { icon, color: Color::new(foreground, background) }
}

#[derive(Clone, Copy)]
pub struct Color {
    foreground: u8,
    background: u8,
}

impl Color {
    pub fn new(foreground: ColorCode, background: ColorCode) -> Color {
        Color {
            foreground: foreground.code(),
            background: background.code(),
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

impl ColorCode {
    fn code(self) -> u8 {
        match self {
            ColorCode::Black   => b'0',
            ColorCode::Red     => b'1',
            ColorCode::Green   => b'2',
            ColorCode::Yellow  => b'3',
            ColorCode::Blue    => b'4',
            ColorCode::Magenta => b'5',
            ColorCode::Cyan    => b'6',
            ColorCode::White   => b'7',
        }
    }
}

