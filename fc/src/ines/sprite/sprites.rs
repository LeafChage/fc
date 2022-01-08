use super::sprite::{Sprite, UNIT_SPRITE_LENGTH};
use image::{ImageBuffer, RgbImage};

pub trait Sprites {
    fn draw<'a>(&'a self, name: &'a str);
}

impl Sprites for Vec<Sprite> {
    fn draw<'a>(&'a self, name: &'a str) {
        let mut image: RgbImage = ImageBuffer::new(256, 240);
        let mut i = 0;
        for sprite in self.iter() {
            let mut unit: RgbImage = ImageBuffer::new(256, 240);
            let offset_x = i % 32;
            let offset_y = i / 32;
            let Sprite(s) = sprite;
            for y in 0..8 {
                for x in 0..8 {
                    let c = 127 * Vec::<u8>::from(s[y as usize])[x as usize];
                    *image.get_pixel_mut(offset_x * 8 + x, offset_y * 8 + y) =
                        image::Rgb([c, c, c]);

                    *unit.get_pixel_mut(x, y) = image::Rgb([c, c, c]);
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
    use crate::io;

    #[test]
    fn it_picture_from_nes() {
        let mut contents =
            io::read_to_binary("../docs/demo/sample1.nes").expect("Don't warry, it is debug");

        let chara_rom = crate::ines::parser(&mut contents)
            .unwrap()
            .character_rom_data;

        let sprites = Sprite::parse_sprites(&chara_rom);
        sprites.draw("./tmp/full_image.png");
    }
}
