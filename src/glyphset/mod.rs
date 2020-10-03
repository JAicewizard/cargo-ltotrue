// Copyright 2020 Antipy V.O.F. All rights reserved.
pub mod cache;

#[derive(Clone)]
pub struct Characters {
    pub characters: Vec<(u32, u16)>, //WHOO WE SUPPORT 8 BIT GREYSCALE!!
}

impl Characters {
    pub fn empty() -> Self {
        Characters {
            characters: Vec::new(),
        }
    }

    pub fn find_closest_match(&self, target: u16, add_noise: usize) -> Option<u32> {
        let length: usize = (1 + add_noise * 2).min(self.characters.len());

        let mut equal_codepoint: Vec<u32> = vec![0; length];
        let mut i = 0;
        for &(code_point, darkness) in &self.characters {
            equal_codepoint[i % length] = code_point;
            if darkness >= target {
                if add_noise == 0 {
                    return Some(code_point);
                }
                break;
            }
            i += 1;
        }
        for l in i..length {
            if self.characters.len() > l {
                equal_codepoint[l] = self.characters[l].0
            }
        }

        Some(equal_codepoint[0])
    }
}

pub fn load_font() -> Characters {
    load_font_from_bytes()
}

pub fn load_font_from_bytes() -> Characters {
    let mut characters: Vec<(u32, u16)> = Vec::new();
    characters.sort_unstable_by(|(_, a), (_, b)| a.cmp(b));
    Characters { characters }
}

pub fn is_bad(codepoint: u32) -> bool {
    std::char::from_u32(codepoint).unwrap().is_control() || codepoint == 0x00AD
}
