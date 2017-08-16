pub mod utils;

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
        if x > self.width || y > self.height {
            None
        } else {
            self.pixels.get(y * self.width + x)
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Pixel> {
        if x > self.width || y > self.height {
            None
        } else {
            self.pixels.get_mut(y * self.width + x)
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
            .map(|row| format!("{}{}[0m\n", row, 27 as char))
            .collect::<String>()
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
            foreground: foreground.code() + 30,
            background: background.code() + 40,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}[{};{}m", 27 as char, self.foreground, self.background)
    }
}

#[derive(Clone, Copy)]
pub enum ColorCode {
    Black, Red, Green, Yellow, Blue, Magenta, Cyan, White,
}

impl ColorCode {
    fn code(self) -> u8 {
        match self {
            ColorCode::Black   => 0,
            ColorCode::Red     => 1,
            ColorCode::Green   => 2,
            ColorCode::Yellow  => 3,
            ColorCode::Blue    => 4,
            ColorCode::Magenta => 5,
            ColorCode::Cyan    => 6,
            ColorCode::White   => 7,
        }
    }
}

