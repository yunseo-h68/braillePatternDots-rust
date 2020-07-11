//! # braillepatterndots
//!
//! `braillepatterndots` is a simple library for braille pattern written rust.
//!

#[derive(Debug)]
pub struct Braille {
    pub code: Vec<bool>,
    pub character: char,
}

impl Braille {
    fn new() -> Braille {
        Braille {
            code: vec![false, false, false, false, false, false],
            character: ' ',
        }
    }
}

pub mod bpd {
    /// When you pass Braille pattern dots number such as 123, 135, 123456 as parameter,
    /// it returns a Braille code in which the status of points is expressed in bool
    ///
    /// # Example
    ///
    /// ```
    /// use braillepatterndots::bpd;
    ///
    /// let braille_code_135 : Vec<bool> = bpd::get_code(135);
    ///
    /// // bpd::get_code(135) => [true, false, true, false, true, false]
    ///
    /// assert_eq!(braille_code_135[0], true);
    /// assert_eq!(braille_code_135[1], false);
    /// assert_eq!(braille_code_135[2], true);
    /// assert_eq!(braille_code_135[3], false);
    /// assert_eq!(braille_code_135[4], true);
    /// assert_eq!(braille_code_135[5], false);
    ///
    /// ```
    pub fn get_code(num: u32) -> Vec<bool> {
        let mut result = Vec::new();

        if num > 123456 {
            return vec![false, false, false, false, false, false];
        }

        let s = num.to_string();

        if s.contains("7") || s.contains("8") || s.contains("9") || s.contains("0") {
            return vec![false, false, false, false, false, false];
        }

        result.push(s.contains("1"));
        result.push(s.contains("2"));
        result.push(s.contains("3"));
        result.push(s.contains("4"));
        result.push(s.contains("5"));
        result.push(s.contains("6"));
        result
    }

    /// When you pass Braille pattern dots number such as 123, 135, 123456 as parameter,
    /// it returns a Braille character
    ///
    /// # Example
    ///
    /// ```
    /// use braillepatterndots::bpd;
    ///
    /// let braille_char_145 : char = bpd::get_char(145);
    ///
    /// assert_eq!(braille_char_145, '⠙');
    ///
    /// ```
    pub fn get_char(num: u32) -> char {
        use std::collections::HashMap;

        let braille_char = vec![
            ' ', '⠁', '⠃', '⠉', '⠙', '⠑', '⠋', '⠛', '⠓', '⠊', '⠚', '⠈', '⠘', '⠄', '⠅', '⠇', '⠍',
            '⠝', '⠕', '⠏', '⠟', '⠗', '⠎', '⠞', '⠌', '⠜', '⠤', '⠥', '⠧', '⠭', '⠽', '⠵', '⠯', '⠿',
            '⠷', '⠮', '⠾', '⠬', '⠼', '⠠', '⠡', '⠣', '⠩', '⠹', '⠱', '⠫', '⠻', '⠳', '⠪', '⠺', '⠨',
            '⠸', '⠂', '⠆', '⠒', '⠲', '⠢', '⠖', '⠶', '⠦', '⠔', '⠴', '⠐', '⠰',
        ];
        let braille_num = vec![
            0, 1, 12, 14, 145, 15, 124, 1245, 125, 24, 245, 4, 45, 3, 13, 123, 134, 1345, 135,
            1234, 12345, 1235, 234, 2345, 34, 345, 36, 136, 1236, 1346, 13456, 1356, 12346, 123456,
            12356, 2346, 23456, 346, 3456, 6, 16, 126, 146, 1456, 156, 1246, 12456, 1256, 246,
            2456, 46, 456, 2, 23, 25, 256, 26, 235, 2356, 236, 35, 356, 5, 56,
        ];
        let braille_chars: HashMap<_, _> = braille_num.iter().zip(braille_char.iter()).collect();
        if braille_chars.contains_key(&num) {
            *braille_chars[&num]
        } else {
            *braille_chars[&0]
        }
    }

    /// When you pass Braille pattern dots number such as 123, 135, 123456 as parameter,
    /// it returns a Braille structure
    ///
    /// # Example
    ///
    /// ```
    /// use braillepatterndots::bpd;
    ///
    /// let braille_124 = bpd::get(124);
    ///
    /// assert_eq!(braille_124.character, '⠋');
    /// assert_eq!(braille_124.code[0], true);
    /// assert_eq!(braille_124.code[1], true);
    /// assert_eq!(braille_124.code[2], false);
    /// assert_eq!(braille_124.code[3], true);
    /// assert_eq!(braille_124.code[4], false);
    /// assert_eq!(braille_124.code[5], false);
    ///
    /// ```
    pub fn get(num: u32) -> super::Braille {
        let mut braille = super::Braille::new();
        braille.character = get_char(num);
        braille.code = get_code(num);
        braille
    }

    /// When you pass vector of Braille pattern dots number such as 123, 135, 123456 as parameter,
    /// it returns vector of Braille structure
    ///
    /// # Example
    ///
    /// ```
    /// use braillepatterndots::bpd;
    /// use braillepatterndots::Braille;
    ///
    /// let brailles : Vec<Braille> = bpd::gets(vec![135, 145, 124]);
    ///
    /// // Braille pattern dots-135
    /// assert_eq!(brailles[0].character, '⠕');
    /// assert_eq!(brailles[0].code[0], true);
    /// assert_eq!(brailles[0].code[1], false);
    /// assert_eq!(brailles[0].code[2], true);
    /// assert_eq!(brailles[0].code[3], false);
    /// assert_eq!(brailles[0].code[4], true);
    /// assert_eq!(brailles[0].code[5], false);
    ///
    /// // Braille pattern dots-145
    /// assert_eq!(brailles[1].character, '⠙');
    /// assert_eq!(brailles[1].code[0], true);
    /// assert_eq!(brailles[1].code[1], false);
    /// assert_eq!(brailles[1].code[2], false);
    /// assert_eq!(brailles[1].code[3], true);
    /// assert_eq!(brailles[1].code[4], true);
    /// assert_eq!(brailles[1].code[5], false);
    ///
    /// // Braille pattern dots-124
    /// assert_eq!(brailles[2].character, '⠋');
    /// assert_eq!(brailles[2].code[0], true);
    /// assert_eq!(brailles[2].code[1], true);
    /// assert_eq!(brailles[2].code[2], false);
    /// assert_eq!(brailles[2].code[3], true);
    /// assert_eq!(brailles[2].code[4], false);
    /// assert_eq!(brailles[2].code[5], false);
    ///
    /// ```
    pub fn gets(nums: Vec<u32>) -> Vec<super::Braille> {
        let mut result = Vec::new();
        for num in &nums {
            result.push(get(*num));
        }
        result
    }
}
