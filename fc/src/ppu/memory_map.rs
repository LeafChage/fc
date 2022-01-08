use std::vec::*;

const PATTERN0_RANGE: std::ops::Range<usize> = 0x0000..0x1000;
const PATTERN2_RANGE: std::ops::Range<usize> = 0x1000..0x2000;
const NAME0_RANGE: std::ops::Range<usize> = 0x2000..0x23c0;
const ATTRIBUTE0_RANGE: std::ops::Range<usize> = 0x23c0..0x2400;
const NAME1_RANGE: std::ops::Range<usize> = 0x2400..0x27c0;
const ATTRIBUTE1_RANGE: std::ops::Range<usize> = 0x27c0..0x2800;
const NAME2_RANGE: std::ops::Range<usize> = 0x2800..0x2bc0;
const ATTRIBUTE2_RANGE: std::ops::Range<usize> = 0x2bc0..0x2c00;
const NAME3_RANGE: std::ops::Range<usize> = 0x2c00..0x2fc0;
const ATTRIBUTE3_RANGE: std::ops::Range<usize> = 0x2fc0..0x3000;
const NAME_ATTRIBUTE_MIRROR_RANGE: std::ops::Range<usize> = 0x3000..0x3f00;
const BACKGROUND_RANGE: std::ops::Range<usize> = 0x3f00..0x3f10;
const SPRITE_RANGE: std::ops::Range<usize> = 0x3f10..0x3f20;
const BG_SPRITE_MIRROR_RANGE: std::ops::Range<usize> = 0x3f20..0x4000;

pub const NAME_LENGTH: usize = NAME0_RANGE.end - NAME0_RANGE.start;

#[derive(PartialEq, Eq, Debug)]
pub struct MemoryMap {
    /// 0x0000～0x0FFF 	パターンテーブル0
    pub pattern0: [u8; PATTERN0_RANGE.end - PATTERN0_RANGE.start],
    /// 0x1000～0x1FFF 	パターンテーブル1
    pub pattern1: [u8; PATTERN2_RANGE.end - PATTERN2_RANGE.start],
    /// 0x2000～0x23BF 	ネームテーブル0
    pub name0: [u8; NAME0_RANGE.end - NAME0_RANGE.start],
    /// 0x23C0～0x23FF 	属性テーブル0
    pub attribute0: [u8; ATTRIBUTE0_RANGE.end - ATTRIBUTE0_RANGE.start],
    /// 0x2400～0x27BF 	ネームテーブル1
    pub name1: [u8; NAME1_RANGE.end - NAME1_RANGE.start],
    /// 0x27C0～0x27FF 	属性テーブル1
    pub attribute1: [u8; ATTRIBUTE1_RANGE.end - ATTRIBUTE1_RANGE.start],
    /// 0x2800～0x2BBF 	ネームテーブル2
    pub name2: [u8; NAME2_RANGE.end - NAME2_RANGE.start],
    /// 0x2BC0～0x2BFF 	属性テーブル2
    pub attribute2: [u8; ATTRIBUTE2_RANGE.end - ATTRIBUTE2_RANGE.start],
    /// 0x2C00～0x2FBF 	ネームテーブル3
    pub name3: [u8; NAME3_RANGE.end - NAME3_RANGE.start],
    /// 0x2FC0～0x2FFF 	属性テーブル3
    pub attribute3: [u8; ATTRIBUTE3_RANGE.end - ATTRIBUTE3_RANGE.start],
    /// 0x3000～0x3EFF 	0x2000-0x2EFFのミラー
    pub name_attribute_mirrow:
        [u8; NAME_ATTRIBUTE_MIRROR_RANGE.end - NAME_ATTRIBUTE_MIRROR_RANGE.start],
    /// 0x3F00～0x3F0F 	バックグラウンドパレット
    pub background: [u8; BACKGROUND_RANGE.end - BACKGROUND_RANGE.start],
    /// 0x3F10～0x3F1F 	スプライトパレット
    pub sprite: [u8; SPRITE_RANGE.end - SPRITE_RANGE.start],
    /// 0x3F20～0x3FFF 	0x3F00-0x3F1Fのミラー
    pub bg_sprite_mirror: [u8; BG_SPRITE_MIRROR_RANGE.end - BG_SPRITE_MIRROR_RANGE.start],
}

impl MemoryMap {
    pub fn new() -> Self {
        MemoryMap {
            pattern0: [0u8; PATTERN0_RANGE.end - PATTERN0_RANGE.start],
            pattern1: [0u8; PATTERN2_RANGE.end - PATTERN2_RANGE.start],
            name0: [0u8; NAME0_RANGE.end - NAME0_RANGE.start],
            attribute0: [0u8; ATTRIBUTE0_RANGE.end - ATTRIBUTE0_RANGE.start],
            name1: [0u8; NAME1_RANGE.end - NAME1_RANGE.start],
            attribute1: [0u8; ATTRIBUTE1_RANGE.end - ATTRIBUTE1_RANGE.start],
            name2: [0u8; NAME2_RANGE.end - NAME2_RANGE.start],
            attribute2: [0u8; ATTRIBUTE2_RANGE.end - ATTRIBUTE2_RANGE.start],
            name3: [0u8; NAME3_RANGE.end - NAME3_RANGE.start],
            attribute3: [0u8; ATTRIBUTE3_RANGE.end - ATTRIBUTE3_RANGE.start],
            name_attribute_mirrow: [0u8; NAME_ATTRIBUTE_MIRROR_RANGE.end
                - NAME_ATTRIBUTE_MIRROR_RANGE.start],
            background: [0u8; BACKGROUND_RANGE.end - BACKGROUND_RANGE.start],
            sprite: [0u8; SPRITE_RANGE.end - SPRITE_RANGE.start],
            bg_sprite_mirror: [0u8; BG_SPRITE_MIRROR_RANGE.end - BG_SPRITE_MIRROR_RANGE.start],
        }
    }

    pub fn read(&self, p: u16) -> u8 {
        let p = p as usize;
        if PATTERN0_RANGE.contains(&p) {
            self.pattern0[p - PATTERN0_RANGE.start]
        } else if PATTERN2_RANGE.contains(&p) {
            self.pattern1[p - PATTERN2_RANGE.start]
        } else if NAME0_RANGE.contains(&p) {
            self.name0[p - NAME0_RANGE.start]
        } else if ATTRIBUTE0_RANGE.contains(&p) {
            self.attribute0[p - ATTRIBUTE0_RANGE.start]
        } else if NAME1_RANGE.contains(&p) {
            self.name1[p - NAME1_RANGE.start]
        } else if ATTRIBUTE1_RANGE.contains(&p) {
            self.attribute1[p - ATTRIBUTE1_RANGE.start]
        } else if NAME2_RANGE.contains(&p) {
            self.name2[p - NAME2_RANGE.start]
        } else if ATTRIBUTE2_RANGE.contains(&p) {
            self.attribute2[p - ATTRIBUTE2_RANGE.start]
        } else if NAME3_RANGE.contains(&p) {
            self.name3[p - NAME3_RANGE.start]
        } else if ATTRIBUTE3_RANGE.contains(&p) {
            self.attribute3[p - ATTRIBUTE3_RANGE.start]
        } else if NAME_ATTRIBUTE_MIRROR_RANGE.contains(&p) {
            self.name_attribute_mirrow[p - NAME_ATTRIBUTE_MIRROR_RANGE.start]
        } else if BACKGROUND_RANGE.contains(&p) {
            self.background[p - BACKGROUND_RANGE.start]
        } else if SPRITE_RANGE.contains(&p) {
            self.sprite[p - SPRITE_RANGE.start]
        } else if BG_SPRITE_MIRROR_RANGE.contains(&p) {
            self.bg_sprite_mirror[p - BG_SPRITE_MIRROR_RANGE.start]
        } else {
            panic!("???")
        }
    }

    pub fn write(&mut self, p: u16, data: u8) {
        let p = p as usize;
        if PATTERN0_RANGE.contains(&p) {
            self.pattern0[p - PATTERN0_RANGE.start] = data;
        } else if PATTERN2_RANGE.contains(&p) {
            self.pattern1[p - PATTERN2_RANGE.start] = data;
        } else if NAME0_RANGE.contains(&p) {
            self.name0[p - NAME0_RANGE.start] = data;
        } else if ATTRIBUTE0_RANGE.contains(&p) {
            self.attribute0[p - ATTRIBUTE0_RANGE.start] = data;
        } else if NAME1_RANGE.contains(&p) {
            self.name1[p - NAME1_RANGE.start] = data;
        } else if ATTRIBUTE1_RANGE.contains(&p) {
            self.attribute1[p - ATTRIBUTE1_RANGE.start] = data;
        } else if NAME2_RANGE.contains(&p) {
            self.name2[p - NAME2_RANGE.start] = data;
        } else if ATTRIBUTE2_RANGE.contains(&p) {
            self.attribute2[p - ATTRIBUTE2_RANGE.start] = data;
        } else if NAME3_RANGE.contains(&p) {
            self.name3[p - NAME3_RANGE.start] = data;
        } else if ATTRIBUTE3_RANGE.contains(&p) {
            self.attribute3[p - ATTRIBUTE3_RANGE.start] = data;
        } else if NAME_ATTRIBUTE_MIRROR_RANGE.contains(&p) {
            self.name_attribute_mirrow[p - NAME_ATTRIBUTE_MIRROR_RANGE.start] = data;
        } else if BACKGROUND_RANGE.contains(&p) {
            self.background[p - BACKGROUND_RANGE.start] = data;
        } else if SPRITE_RANGE.contains(&p) {
            self.sprite[p - SPRITE_RANGE.start] = data;
        } else if BG_SPRITE_MIRROR_RANGE.contains(&p) {
            self.bg_sprite_mirror[p - BG_SPRITE_MIRROR_RANGE.start] = data;
        } else {
            panic!("???")
        }
    }
}
