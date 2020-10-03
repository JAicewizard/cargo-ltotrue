use image::Luma;

pub struct CharCache {
    pub cache:
        std::sync::RwLock<std::collections::HashMap<char, image::ImageBuffer<Luma<u8>, Vec<u8>>>>,
    codepoints: std::collections::HashMap<char, (u32, u32)>,
    line_height: f32,
}

impl CharCache {
    pub fn new() -> Self {
        CharCache {
            cache: std::sync::RwLock::new(std::collections::HashMap::new()),
            codepoints: std::collections::HashMap::new(),
            line_height: 10.0,
        }
    }

    pub fn get_line_height(&self) -> f32 {
        self.line_height
    }

    pub fn set_font(&self, chars: &crate::glyphset::Characters, line_height: f32) -> Self {
        let out = CharCache {
            cache: std::sync::RwLock::new(std::collections::HashMap::with_capacity(
                chars.characters.len(),
            )),
            codepoints: std::collections::HashMap::with_capacity(chars.characters.len()),
            line_height: line_height,
        };
        out
    }
    pub fn set_height(&self, line_height: f32) -> Self {
        let out = CharCache {
            cache: std::sync::RwLock::new(std::collections::HashMap::with_capacity(
                self.codepoints.len(),
            )),
            codepoints: std::collections::HashMap::with_capacity(self.codepoints.len()),
            line_height: line_height,
        };

        out
    }

    pub fn render_char<O: FnMut(u32, u32, u8)>(&self, c: char, mut o: O) {
        if let Some(bitmap) = self.cache.read().unwrap().get(&c) {
            for (x, y, p) in bitmap.enumerate_pixels() {
                o(x, y, p.0[0]);
            }
            return;
        }
        if let Some((width, height)) = self.codepoints.get(&c) {
            let bitmap: image::ImageBuffer<Luma<u8>, Vec<u8>> =
                image::ImageBuffer::new(*width, *height);
            for (x, y, p) in bitmap.enumerate_pixels() {
                o(x, y, p.0[0]);
            }
            self.cache.write().unwrap().insert(c, bitmap);
        }
    }
}
