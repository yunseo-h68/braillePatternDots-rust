extern crate braillepatterndots;
use braillepatterndots::bpd;

#[test]
fn get_char_test() {
    assert_eq!(bpd::get_char(0), ' ');
    assert_eq!(bpd::get_char(1), '⠁');
    assert_eq!(bpd::get_char(12), '⠃');
    assert_eq!(bpd::get_char(14), '⠉');
    assert_eq!(bpd::get_char(145), '⠙');
    assert_eq!(bpd::get_char(15), '⠑');
    assert_eq!(bpd::get_char(124), '⠋');
}

#[test]
fn get_code_test() {
    let braille_code = bpd::get_code(0);
    assert_eq!(braille_code[0], false);
    assert_eq!(braille_code[1], false);
    assert_eq!(braille_code[2], false);
    assert_eq!(braille_code[3], false);
    assert_eq!(braille_code[4], false);
    assert_eq!(braille_code[5], false);
    let braille_code = bpd::get_code(1);
    assert_eq!(braille_code[0], true);
    assert_eq!(braille_code[1], false);
    assert_eq!(braille_code[2], false);
    assert_eq!(braille_code[3], false);
    assert_eq!(braille_code[4], false);
    assert_eq!(braille_code[5], false);
    let braille_code = bpd::get_code(12);
    assert_eq!(braille_code[0], true);
    assert_eq!(braille_code[1], true);
    assert_eq!(braille_code[2], false);
    assert_eq!(braille_code[3], false);
    assert_eq!(braille_code[4], false);
    assert_eq!(braille_code[5], false);
    let braille_code = bpd::get_code(14);
    assert_eq!(braille_code[0], true);
    assert_eq!(braille_code[1], false);
    assert_eq!(braille_code[2], false);
    assert_eq!(braille_code[3], true);
    assert_eq!(braille_code[4], false);
    assert_eq!(braille_code[5], false);
    let braille_code = bpd::get_code(145);
    assert_eq!(braille_code[0], true);
    assert_eq!(braille_code[1], false);
    assert_eq!(braille_code[2], false);
    assert_eq!(braille_code[3], true);
    assert_eq!(braille_code[4], true);
    assert_eq!(braille_code[5], false);
    let braille_code = bpd::get_code(15);
    assert_eq!(braille_code[0], true);
    assert_eq!(braille_code[1], false);
    assert_eq!(braille_code[2], false);
    assert_eq!(braille_code[3], false);
    assert_eq!(braille_code[4], true);
    assert_eq!(braille_code[5], false);
    let braille_code = bpd::get_code(124);
    assert_eq!(braille_code[0], true);
    assert_eq!(braille_code[1], true);
    assert_eq!(braille_code[2], false);
    assert_eq!(braille_code[3], true);
    assert_eq!(braille_code[4], false);
    assert_eq!(braille_code[5], false);
}
