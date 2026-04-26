use wasm_bindgen::prelude::*;

// Image dimensions
const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

/// World struct containing the pixel buffer and dimensions
#[wasm_bindgen]
pub struct World {
    width: u32,
    height: u32,
    pixel_buffer: Vec<u8>,
}

#[wasm_bindgen]
impl World {
    /// Create a new World instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> World {
        World {
            width: WIDTH,
            height: HEIGHT,
            pixel_buffer: vec![0u8; (WIDTH * HEIGHT * 4) as usize],
        }
    }

    /// Get the width
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// Get the height
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Get pointer to the pixel buffer
    pub fn get_pixel_buffer_ptr(&self) -> *const u8 {
        self.pixel_buffer.as_ptr()
    }

    /// Update the pixel buffer with a new frame
    pub fn update_frame(&mut self, time: f64) {
        let width = self.width as usize;
        let height = self.height as usize;
        
        // Create a dynamic animated pattern based on time
        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) * 4;
                
                // Create a moving gradient pattern
                let phase = time * 0.001;
                let dx = (x as f64) / (width as f64) * 2.0 * std::f64::consts::PI;
                let dy = (y as f64) / (height as f64) * 2.0 * std::f64::consts::PI;
                
                // Animated color values
                let r = ((dx.sin() + 1.0) * 0.5 * 127.0 + (phase * 2.0).sin() * 64.0) as u8;
                let g = ((dy.cos() + 1.0) * 0.5 * 127.0 + (phase * 3.0).cos() * 64.0) as u8;
                let b = (((dx + dy).sin() + 1.0) * 0.5 * 127.0 + (phase * 5.0).sin() * 64.0) as u8;
                
                self.pixel_buffer[idx] = r;     // Red
                self.pixel_buffer[idx + 1] = g; // Green
                self.pixel_buffer[idx + 2] = b; // Blue
                self.pixel_buffer[idx + 3] = 255; // Alpha (fully opaque)
            }
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}