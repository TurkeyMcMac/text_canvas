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
}

pub trait CanvasUtils {
    unsafe fn rect_unchecked(&mut self, x: usize, y: usize, width: usize, height: usize, filler: Pixel);
    
    fn rect(&mut self, x: usize, y: usize, width: usize, height: usize, filler: Pixel);
}
