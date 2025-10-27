use crate::colors::{hsv_interp, Pixel, PixelHsv};
use crate::sprites::RandomSprites;

pub trait Leds {
    fn data(&self) -> &[u8];
    
    fn all_off(&mut self);
    
    fn set_led(&mut self, rgb: Pixel, index: usize);
    
    fn fill_gradient(&mut self, start: &PixelHsv, end: &PixelHsv);
    
    fn fill_gradient_dual(&mut self, start: &PixelHsv, end: &PixelHsv);
    
    fn fill_triangle(&mut self, start: &PixelHsv, end: &PixelHsv, phase: f32);
    
    fn fill_random(&mut self, start: &PixelHsv, end: &PixelHsv, sprites: &RandomSprites);
}

pub struct Apa102 {
    len: usize,
    buf_end: usize,
    buffer: [u8; 4612]
}

impl Apa102 {
    pub fn new(led_count: usize) -> Self {
        // LED count is limited to 1024
        assert!(led_count <= 1024);
        let buf_end = 4 + (led_count * 4) + ((led_count + 1) / 2);
        let mut buffer = [0_u8; 4612];
        for (i, v) in buffer.chunks_mut(4).skip(1).enumerate() {
            if i >= led_count { break; }
            v[0] = 255;
        }
        Self {
            //~ led_type: Apa102,
            len: led_count,
            buf_end: buf_end,
            buffer: buffer,
        }
    }
}

impl Leds for Apa102 {
    fn data(&self) -> &[u8] {
        &self.buffer[..self.buf_end]
    }
    
    fn all_off(&mut self) {
        for (i, v) in self.buffer.chunks_mut(4).skip(1).enumerate() {
            if i >= self.len { break; }
            v[1] = 0;
            v[2] = 0;
            v[3] = 0;
        }
    }
    
    fn set_led(&mut self, rgb: Pixel, index: usize) {
        match self.buffer.chunks_mut(4).skip(1).nth(index) {
            Some(v) => {
                set_array_apa102(v, rgb);
            },
            None => {},
        }
    }
    
    fn fill_gradient(&mut self, start: &PixelHsv, end: &PixelHsv) {
        for (i, v) in self.buffer.chunks_mut(4).skip(1).enumerate() {
            if i >= self.len { break; }
            let pos = position(i, self.len);
            let rgb = hsv_interp(&start, &end, pos).to_rgb();
            set_array_apa102(v, rgb);
        }
    }
    
    fn fill_gradient_dual(&mut self, start: &PixelHsv, end: &PixelHsv) {
        for (i, v) in self.buffer.chunks_mut(4).skip(1).enumerate() {
            if i >= self.len { break; }
            let pos = position(i, self.len);
            let mut pos_bipolar = pos * 2.0 - 1.0;
            if pos_bipolar < 0.0 { pos_bipolar = pos_bipolar * -1.0 }
            let rgb = hsv_interp(&end, &start, pos_bipolar).to_rgb();
            set_array_apa102(v, rgb);
        }
    }
    
    fn fill_triangle(&mut self, start: &PixelHsv, end: &PixelHsv, phase: f32) {
        for (i, v) in self.buffer.chunks_mut(4).skip(1).enumerate() {
            if i >= self.len { break; }
            let pos = position(i, self.len);
            let mut pos_triangle = ((pos + phase) % 1.0) * 2.0 - 1.0;
            if pos_triangle < 0.0 { pos_triangle = pos_triangle * -1.0 }
            let rgb = hsv_interp(&end, &start, pos_triangle).to_rgb();
            set_array_apa102(v, rgb);
        }
    }
    
    fn fill_random(&mut self, start: &PixelHsv, end: &PixelHsv, sprites: &RandomSprites) {
        self.buffer.chunks_mut(4).skip(1).zip(sprites.get_sprites()).for_each(|(led, sprite)| {
            let rgb = hsv_interp(&end, &start, sprite.get_value()).to_rgb();
            set_array_apa102(led, rgb);
        });
    }
}

pub struct Sk9822 {
    len: usize,
    buf_end: usize,
    buffer: [u8; 4612]
}

impl Sk9822 {
    pub fn new(led_count: usize) -> Self {
        // LED count is limited to 1024
        assert!(led_count <= 1024);
        let buf_end = 4 + (led_count * 4) + 4;
        let mut buffer = [0_u8; 4612];
        for (index, i) in buffer.chunks_mut(4).skip(1).enumerate() {
            if index >= led_count { break; }
            i[0] = 255;
        }
        for i in buffer.chunks_mut(4).skip(1 + led_count).take(1) { 
            for j in i {
                *j = 255;
            }
        }
        Self {
            //~ led_type: Sk9822,
            len: led_count,
            buf_end: buf_end,
            buffer: buffer,
        }
    }
}

impl Leds for Sk9822 {
    fn data(&self) -> &[u8] {
        &self.buffer[..self.buf_end]
    }
    
    fn all_off(&mut self) {
        for (i, v) in self.buffer.chunks_mut(4).skip(1).enumerate() {
            if i >= self.len { break; }
            //~ v[0] = 255;
            v[1] = 0;
            v[2] = 0;
            v[3] = 0;
        }
    }
    
    fn set_led(&mut self, rgb: Pixel, index: usize) {
        match self.buffer.chunks_mut(4).skip(1).nth(index) {
            Some(v) => {
                set_array_apa102(v, rgb);
            },
            None => {},
        }
    }
    
    fn fill_gradient(&mut self, start: &PixelHsv, end: &PixelHsv) {
        for (i, v) in self.buffer.chunks_mut(4).skip(1).enumerate() {
            if i >= self.len { break; }
            let pos = position(i, self.len);
            let rgb = hsv_interp(&start, &end, pos).to_rgb();
            set_array_apa102(v, rgb);
        }
    }
    
    fn fill_gradient_dual(&mut self, start: &PixelHsv, end: &PixelHsv) {
        for (i, v) in self.buffer.chunks_mut(4).skip(1).enumerate() {
            if i >= self.len { break; }
            let pos = position(i, self.len);
            let mut pos_bipolar = pos * 2.0 - 1.0;
            if pos_bipolar < 0.0 { pos_bipolar = pos_bipolar * -1.0 }
            let rgb = hsv_interp(&end, &start, pos_bipolar).to_rgb();
            set_array_apa102(v, rgb);
        }
    }
    
    fn fill_triangle(&mut self, start: &PixelHsv, end: &PixelHsv, phase: f32) {
        for (i, v) in self.buffer.chunks_mut(4).skip(1).enumerate() {
            if i >= self.len { break; }
            let pos = position(i, self.len);
            let mut pos_triangle = ((pos + phase) % 1.0) * 2.0 - 1.0;
            if pos_triangle < 0.0 { pos_triangle = pos_triangle * -1.0 }
            let rgb = hsv_interp(&end, &start, pos_triangle).to_rgb();
            set_array_apa102(v, rgb);
        }
    }
    
    fn fill_random(&mut self, start: &PixelHsv, end: &PixelHsv, sprites: &RandomSprites) {
        self.buffer.chunks_mut(4).skip(1).zip(sprites.get_sprites()).for_each(|(led, sprite)| {
            let rgb = hsv_interp(&end, &start, sprite.get_value()).to_rgb();
            set_array_apa102(led, rgb);
        });
    }
}

pub struct Ws2801 {
    len: usize,
    buf_end: usize,
    buffer: [u8; 4612]
}

impl Ws2801 {
    pub fn new(led_count: usize) -> Self {
        // LED count is limited to 1024
        assert!(led_count <= 1024);
        let buf_end = led_count * 3;
        Self {
            //~ led_type: Ws2801,
            len: led_count,
            buf_end: buf_end,
            buffer: [0; 4612],
        }
    }
}

impl Leds for Ws2801 {
    fn data(&self) -> &[u8] {
        &self.buffer[..self.buf_end]
    }
    
    fn all_off(&mut self) {
        for v in self.buffer.iter_mut() {
            *v = 0;
        }
    }
    
    fn set_led(&mut self, rgb: Pixel, index: usize) {
        match self.buffer.chunks_mut(3).nth(index) {
            Some(v) => {
                set_array_ws2801(v, rgb);
            },
            None => {},
        }
    }
    
    fn fill_gradient(&mut self, start: &PixelHsv, end: &PixelHsv) {
        for (i, v) in self.buffer.chunks_mut(3).enumerate() {
            if i >= self.len { break; }
            let pos = position(i, self.len);
            let rgb = hsv_interp(&start, &end, pos).to_rgb();
            set_array_ws2801(v, rgb);
        }
    }
    
    fn fill_gradient_dual(&mut self, start: &PixelHsv, end: &PixelHsv) {
        for (i, v) in self.buffer.chunks_mut(3).enumerate() {
            if i >= self.len { break; }
            let pos = position(i, self.len);
            let mut pos_bipolar = pos * 2.0 - 1.0;
            if pos_bipolar < 0.0 { pos_bipolar = pos_bipolar * -1.0 }
            let rgb = hsv_interp(&end, &start, pos_bipolar).to_rgb();
            set_array_ws2801(v, rgb);
        }
    }
    
    fn fill_triangle(&mut self, start: &PixelHsv, end: &PixelHsv, phase: f32) {
        for (i, v) in self.buffer.chunks_mut(3).enumerate() {
            if i >= self.len { break; }
            let pos = position(i, self.len);
            let mut pos_triangle = ((pos + phase) % 1.0) * 2.0 - 1.0;
            if pos_triangle < 0.0 { pos_triangle = pos_triangle * -1.0 }
            let rgb = hsv_interp(&end, &start, pos_triangle).to_rgb();
            set_array_ws2801(v, rgb);
        }
    }
    
    fn fill_random(&mut self, start: &PixelHsv, end: &PixelHsv, sprites: &RandomSprites) {
        self.buffer.chunks_mut(3).zip(sprites.get_sprites()).for_each(|(led, sprite)| {
            let rgb = hsv_interp(&end, &start, sprite.get_value()).to_rgb();
            set_array_ws2801(led, rgb);
        });
    }
}

fn position(index: usize, len: usize) -> f32 {
    (index as f32) / ((len - 1) as f32)
}

fn set_array_apa102(led: &mut [u8], rgb: Pixel) {
    //~ led[0] = 255;
    led[1] = rgb.get_b();
    led[2] = rgb.get_g();
    led[3] = rgb.get_r();
}

fn set_array_ws2801(led: &mut [u8], rgb: Pixel) {
    led[0] = rgb.get_r();
    led[1] = rgb.get_g();
    led[2] = rgb.get_b();
}

/*
fn set_array_analog8(led: &mut [u16], rgb: Pixel) {
    led[0] = (rgb.get_r() as u16) << 8;
    led[1] = (rgb.get_g() as u16) << 8;
    led[2] = (rgb.get_b() as u16) << 8;
}

fn set_array_analog16(led: &mut [u16], rgb: Pixel16) {
    led[0] = rgb.get_r();
    led[1] = rgb.get_g();
    led[2] = rgb.get_b();
}
*/
