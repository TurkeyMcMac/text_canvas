use super::*;

impl CanvasUtils for Canvas {
    unsafe fn rect_unchecked(&mut self, x: usize, y: usize, width: usize, height: usize, filler: Pixel) {
        for x in x..x + width {
            for y in y..y + height {
                *self.get_unchecked_mut(x, y) = filler;
            }
        }
    }

    fn rect(&mut self, x: usize, y: usize, width: usize, height: usize, filler: Pixel) {
        use std::cmp::min;

        if x < self.width && y < self.height {
            let width = min(width, self.width - x);
            
            let height = min(height, self.height - y);
            
            unsafe {
                self.rect_unchecked(x, y, width, height, filler);
            }
        }
    }
    
    unsafe fn text_unchecked(&mut self, x: usize, y: usize, color: Color, cr_lf: bool, text: &str) {
        let mut current_x = x; let mut current_y = y;
        
        for letter in text.chars() {
            match letter {
                '\n' => {
                    if cr_lf {
                        current_x = x;
                    }

                    current_y += 1;
                },
                '\r' => {
                    current_x = x;
                },
                letter => {
                    *self.get_unchecked_mut(current_x, current_y) = Pixel::new(letter, color);
                    
                    current_x += 1;
                }
            }
        }
    }

    fn text(&mut self, x: usize, y: usize, color: Color, cr_lf: bool, text: &str) {
        let mut current_x = x; let mut current_y = y;
        
        let mut in_bounds = x < self.width && y < self.height;

        for letter in text.chars() {
            match letter {
                '\n' => {
                    if cr_lf {
                        current_x = x;
                    }

                    current_y += 1;
                    in_bounds = current_x < self.width && current_y < self.height;
                },
                '\r' => {
                    current_x = x;
                    in_bounds = current_x < self.width && current_y < self.height;
                },
                letter => {
                    if in_bounds {
                        unsafe {
                            *self.get_unchecked_mut(current_x, current_y) = Pixel::new(letter, color);
                        }

                        current_x += 1;
                        
                        in_bounds = self.width > current_x;
                    }
                }
            }
        }
    }
}

pub trait CanvasUtils {
    unsafe fn rect_unchecked(&mut self, x: usize, y: usize, width: usize, height: usize, filler: Pixel);
    
    fn rect(&mut self, x: usize, y: usize, width: usize, height: usize, filler: Pixel);

    unsafe fn text_unchecked(&mut self, x: usize, y: usize, color: Color, cr_lf: bool, text: &str);
    
    fn text(&mut self, x: usize, y: usize, color: Color, cr_lf: bool, text: &str);
}
