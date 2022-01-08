use crate::ines::sprite::Sprite;
use crate::ppu::memory_map::NAME_LENGTH;
use image::{ImageBuffer, RgbImage};

const DISPLAY_WIDTH: u32 = 256;
const DISPLAY_HEIGHT: u32 = 240;

#[derive(PartialEq, Eq, Debug)]
pub struct Display();

impl Display {
    pub fn draw<'a>(sprites: [&Sprite; NAME_LENGTH], name: &'a str) {
        let mut image: RgbImage = ImageBuffer::new(DISPLAY_WIDTH, DISPLAY_HEIGHT);
        let mut i = 0;
        for sprite in sprites.iter() {
            let offset_x = i % (DISPLAY_WIDTH / 8);
            let offset_y = i / (DISPLAY_WIDTH / 8);
            let Sprite(s) = sprite;
            for y in 0..8 {
                let s = Vec::<u8>::from(s[y as usize]);
                for x in 0..8 {
                    let c = 127 * s[x as usize];
                    *image.get_pixel_mut(offset_x * 8 + x, offset_y * 8 + y) =
                        image::Rgb([c, c, c]);
                }
            }
            i += 1;
        }
        image.save(name).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn it_draw() {
        let sprite = Sprite::default();
        let mut sprites = vec![];
        for _ in 0..(NAME_LENGTH - 1) {
            sprites.push(&sprite);
        }
        let sprite = Sprite(vec![
            crate::binary::Bit(1, 1, 1, 1, 1, 1, 1, 1),
            crate::binary::Bit(1, 1, 1, 1, 1, 1, 1, 1),
            crate::binary::Bit(1, 1, 1, 1, 1, 1, 1, 1),
            crate::binary::Bit(1, 1, 1, 1, 1, 1, 1, 1),
            crate::binary::Bit(1, 1, 1, 1, 1, 1, 1, 1),
            crate::binary::Bit(1, 1, 1, 1, 1, 1, 1, 1),
            crate::binary::Bit(1, 1, 1, 1, 1, 1, 1, 1),
            crate::binary::Bit(1, 1, 1, 1, 1, 1, 1, 1),
        ]);
        sprites.push(&sprite);
        Display::draw(
            sprites[..].try_into().unwrap(),
            "./tmp/display_test_draw.png",
        );
    }
}
