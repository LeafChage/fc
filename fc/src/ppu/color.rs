// use image::{ImageBuffer, RgbImage};
use once_cell::sync::Lazy;

/// reference: https://emulation.gametechwiki.com/index.php/Famicom_Color_Palettehgg
pub static Colors: Lazy<[image::Rgb<u8>; 0x40]> = Lazy::new(|| {
    [
        image::Rgb([0x66, 0x66, 0x66]),
        image::Rgb([0x00, 0x2A, 0x88]),
        image::Rgb([0x14, 0x12, 0xA7]),
        image::Rgb([0x3B, 0x00, 0xA4]),
        image::Rgb([0x5C, 0x00, 0x7E]),
        image::Rgb([0x6E, 0x00, 0x40]),
        image::Rgb([0x6C, 0x07, 0x00]),
        image::Rgb([0x56, 0x1D, 0x00]),
        image::Rgb([0x33, 0x35, 0x00]),
        image::Rgb([0x0C, 0x48, 0x00]),
        image::Rgb([0x00, 0x52, 0x00]),
        image::Rgb([0x00, 0x4F, 0x08]),
        image::Rgb([0x00, 0x40, 0x4D]),
        image::Rgb([0x00, 0x00, 0x00]),
        image::Rgb([0x00, 0x00, 0x00]),
        image::Rgb([0x00, 0x00, 0x00]),
        image::Rgb([0xAD, 0xAD, 0xAD]),
        image::Rgb([0x15, 0x5F, 0xD9]),
        image::Rgb([0x42, 0x40, 0xFF]),
        image::Rgb([0x75, 0x27, 0xFE]),
        image::Rgb([0xA0, 0x1A, 0xCC]),
        image::Rgb([0xB7, 0x1E, 0x7B]),
        image::Rgb([0xB5, 0x31, 0x20]),
        image::Rgb([0x99, 0x4E, 0x00]),
        image::Rgb([0x6B, 0x6D, 0x00]),
        image::Rgb([0x38, 0x87, 0x00]),
        image::Rgb([0x0D, 0x93, 0x00]),
        image::Rgb([0x00, 0x8F, 0x32]),
        image::Rgb([0x00, 0x7C, 0x8D]),
        image::Rgb([0x00, 0x00, 0x00]),
        image::Rgb([0x00, 0x00, 0x00]),
        image::Rgb([0x00, 0x00, 0x00]),
        image::Rgb([0xFF, 0xFF, 0xFF]),
        image::Rgb([0x64, 0xB0, 0xFF]),
        image::Rgb([0x92, 0x90, 0xFF]),
        image::Rgb([0xC6, 0x76, 0xFF]),
        image::Rgb([0xF2, 0x6A, 0xFF]),
        image::Rgb([0xFF, 0x6E, 0xCC]),
        image::Rgb([0xFF, 0x81, 0x70]),
        image::Rgb([0xEA, 0x9E, 0x22]),
        image::Rgb([0xBC, 0xBE, 0x00]),
        image::Rgb([0x88, 0xD8, 0x00]),
        image::Rgb([0x5C, 0xE4, 0x30]),
        image::Rgb([0x45, 0xE0, 0x82]),
        image::Rgb([0x48, 0xCD, 0xDE]),
        image::Rgb([0x4F, 0x4F, 0x4F]),
        image::Rgb([0x00, 0x00, 0x00]),
        image::Rgb([0x00, 0x00, 0x00]),
        image::Rgb([0xFF, 0xFF, 0xFF]),
        image::Rgb([0xC0, 0xDF, 0xFF]),
        image::Rgb([0xD3, 0xD2, 0xFF]),
        image::Rgb([0xE8, 0xC8, 0xFF]),
        image::Rgb([0xFA, 0xC2, 0xFF]),
        image::Rgb([0xFF, 0xC4, 0xEA]),
        image::Rgb([0xFF, 0xCC, 0xC5]),
        image::Rgb([0xF7, 0xD8, 0xA5]),
        image::Rgb([0xE4, 0xE5, 0x94]),
        image::Rgb([0xCF, 0xEF, 0x96]),
        image::Rgb([0xBD, 0xF4, 0xAB]),
        image::Rgb([0xB3, 0xF3, 0xCC]),
        image::Rgb([0xB5, 0xEB, 0xF2]),
        image::Rgb([0xB8, 0xB8, 0xB8]),
        image::Rgb([0x00, 0x00, 0x00]),
        image::Rgb([0x00, 0x00, 0x00]),
    ]
});