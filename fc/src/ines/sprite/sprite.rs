use crate::binary::Bit;
use image::{ImageBuffer, RgbImage};
use std::iter::Iterator;

pub const UNIT_SPRITE_LENGTH: usize = 16;

#[derive(PartialEq, Eq, Debug)]
pub struct Sprite(pub Vec<Bit>);

impl std::default::Default for Sprite {
    fn default() -> Self {
        Sprite(vec![
            Bit::default(),
            Bit::default(),
            Bit::default(),
            Bit::default(),
            Bit::default(),
            Bit::default(),
            Bit::default(),
            Bit::default(),
        ])
    }
}

impl Sprite {
    pub fn parse_sprites(raw: &Vec<u8>) -> Vec<Sprite> {
        let mut sprites = Vec::<Sprite>::new();
        let mut i = 0;
        while i + UNIT_SPRITE_LENGTH < raw.len() {
            let sprite = Sprite::parse(&raw, i as u16);
            i += UNIT_SPRITE_LENGTH;
            sprites.push(sprite);
        }
        sprites
    }

    pub fn parse(rom: &[u8], point: u16) -> Sprite {
        let point = point as usize;
        let half_length = UNIT_SPRITE_LENGTH / 2;
        let sprites_a = &rom[point..(point + half_length)];
        let sprites_b = &rom[(point + half_length)..(point + UNIT_SPRITE_LENGTH)];
        let sprites_a = sprites_a.iter().map(|v| Bit::from(v.clone()));
        let sprites_b = sprites_b.iter().map(|v| Bit::from(v.clone()));

        let s = sprites_a
            .zip(sprites_b)
            .map(|(a, b)| a + b)
            .collect::<Vec<Bit>>();
        Sprite(s)
    }

    pub fn draw(&self, name: &str) {
        let Sprite(s) = self;
        let mut image: RgbImage = ImageBuffer::new(64, 64);
        for y in 0..8 {
            for x in 0..8 {
                let c = 127 * Vec::<u8>::from(s[y as usize])[x as usize];
                *image.get_pixel_mut(x, y) = image::Rgb([c, c, c]);
            }
        }

        image.save(name).unwrap();
    }

    /// debug用のサイズ指定できるDraw
    pub fn debug_draw(&self, name: &str, n: u32) {
        let Sprite(s) = self;
        let mut image: RgbImage = ImageBuffer::new(8 * n, 8 * n);
        for y in 0..8 {
            for x in 0..8 {
                let c = 127 * Vec::<u8>::from(s[y as usize])[x as usize];
                // *image.get_pixel_mut(x, y) = image::Rgb([c, c, c]);
                // Debug >>
                // 8x8を同じ色で埋める
                for w in 0..n {
                    for h in 0..n {
                        *image.get_pixel_mut(x * n + w, y * n + h) = image::Rgb([c, c, c]);
                    }
                }
                // Debug <<
            }
        }

        image.save(name).unwrap();
    }
}

impl std::convert::From<Vec<Bit>> for Sprite {
    fn from(v: Vec<Bit>) -> Self {
        Sprite(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io;

    #[test]
    fn it_picture() {
        let src: &mut [u8] = &mut [
            0x66, 0x7f, 0xff, 0xff, 0xff, 0x7e, 0x3c, 0x18, //
            0x66, 0x5f, 0xbf, 0xbf, 0xff, 0x7e, 0x3c, 0x18,
        ];
        Sprite::parse(&src, 0).debug_draw("./tmp/it_picture.png", 10);
    }
}
