# braillePatternDots-rust

A simple library for braille pattern written rust.

## Structs

- [Braille](#braillepatterndotsbraille)

### braillepatterndots::Braille

```rust
#[derive(Debug)]
pub struct Braille {
    pub code: Vec<bool>,
    pub character: char,
}
```

## Functions

- [braillepatterndots::bpd::get_code](#braillepatterndotsbpdget_code)
- [braillepatterndots::bpd::get_char](#braillepatterndotsbpdget_char)
- [braillepatterndots::bpd::get](#braillepatterndotsbpdget)
- [braillepatterndots::bpd::gets](#braillepatterndotsbpdgets)

### braillepatterndots::bpd::get_code

```rust
pub fn get_code(num: u32) -> Vec<bool>
```

When you pass Braille pattern dots number such as 123, 135, 123456 as parameter, it returns a Braille code in which the status of points is expressed in bool

#### Example

```rust
use braillepatterndots::bpd;

let braille_code_135 : Vec<bool> = bpd::get_code(135);

// bpd::get_code(135) => [true, false, true, false, true, false]

assert_eq!(braille_code_135[0], true);
assert_eq!(braille_code_135[1], false);
assert_eq!(braille_code_135[2], true);
assert_eq!(braille_code_135[3], false);
assert_eq!(braille_code_135[4], true);
assert_eq!(braille_code_135[5], false);
```

### braillepatterndots::bpd::get_char

```rust
pub fn get_char(num: u32) -> char
```

When you pass Braille pattern dots number such as 123, 135, 123456 as parameter, it returns a Braille character

#### Example

```rust
use braillepatterndots::bpd;

let braille_char_145 : char = bpd::get_char(145);

assert_eq!(braille_char_145, '⠙');
```

### braillepatterndots::bpd::get

```rust
pub fn get(num: u32) -> Braille
```

When you pass Braille pattern dots number such as 123, 135, 123456 as parameter, it returns a Braille structure.

#### Example

```rust
use braillepatterndots::bpd;

let braille_124 = bpd::get(124);

assert_eq!(braille_124.character, '⠋');
assert_eq!(braille_124.code[0], true);
assert_eq!(braille_124.code[1], true);
assert_eq!(braille_124.code[2], false);
assert_eq!(braille_124.code[3], true);
assert_eq!(braille_124.code[4], false);
assert_eq!(braille_124.code[5], false);
```

### braillepatterndots::bpd::gets

```rust
pub fn gets(nums: Vec<u32>) -> Vec<Braille>
```

When you pass vector of Braille pattern dots number such as 123, 135, 123456 as parameter, it returns vector of Braille structure

#### Example

```rust
use braillepatterndots::bpd;
use braillepatterndots::Braille;

let brailles : Vec<Braille> = bpd::gets(vec![135, 145, 124]);

// Braille pattern dots-135
assert_eq!(brailles[0].character, '⠕');
assert_eq!(brailles[0].code[0], true);
assert_eq!(brailles[0].code[1], false);
assert_eq!(brailles[0].code[2], true);
assert_eq!(brailles[0].code[3], false);
assert_eq!(brailles[0].code[4], true);
assert_eq!(brailles[0].code[5], false);

// Braille pattern dots-145
assert_eq!(brailles[1].character, '⠙');
assert_eq!(brailles[1].code[0], true);
assert_eq!(brailles[1].code[1], false);
assert_eq!(brailles[1].code[2], false);
assert_eq!(brailles[1].code[3], true);
assert_eq!(brailles[1].code[4], true);
assert_eq!(brailles[1].code[5], false);

// Braille pattern dots-124
assert_eq!(brailles[2].character, '⠋');
assert_eq!(brailles[2].code[0], true);
assert_eq!(brailles[2].code[1], true);
assert_eq!(brailles[2].code[2], false);
assert_eq!(brailles[2].code[3], true);
assert_eq!(brailles[2].code[4], false);
assert_eq!(brailles[2].code[5], false);
```

## Test

```bash
cargo test
```
