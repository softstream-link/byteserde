fn is_printable(c: u8) -> bool {
    (c > 0x21 && c < 0x7e) || (c > 0xa0)
}
fn map_2_printable(c: u8) -> char {
    if is_printable(c) {
        char::from(c)
    } else {
        '.'
    }
}
pub fn to_hex_line(bytes: &[u8]) -> String {
    // todo make pretty inline
    to_hex_pretty(bytes).replace('\n', " ").replace("   ", "")
}
pub fn to_hex_pretty(bytes: &[u8]) -> String {
    struct HexAscii {
        hex: String,
        asc: String,
    }
    let hex: Vec<HexAscii> = bytes
        .iter()
        .map(|v| HexAscii {
            hex: format!("{:02x}", v),
            asc: format!("{}", map_2_printable(*v)),
        })
        .collect();
    let mut result = String::new();

    for (idx, chunk16) in hex.chunks(16).enumerate() {
        let mut hex_view = String::new();
        let mut asc_view = String::new();
        for chunk4 in chunk16.chunks(4) {
            for ha in chunk4 {
                hex_view.push_str(&format!("{} ", ha.hex));
                asc_view.push_str(&format!("{} ", ha.asc));
            }
            hex_view.push(' ');
            asc_view.push(' ');
        }
        let asc_view = asc_view.trim_end();
        result.push_str(&format!("{idx:<04}: {hex_view:<52}| {asc_view}\n"));
    }
    result
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::unittest;
    use text_diff::{diff, print_diff};

    #[test]
    fn hex() {
        unittest::setup::log::configure();
        let mut v: Vec<u8> = Vec::new();
        for i in 0..=255 {
            v.push(i);
        }
        for i in 0..10 {
            v.push(i);
        }

        let expected = r#"0000: 00 01 02 03  04 05 06 07  08 09 0a 0b  0c 0d 0e 0f  | . . . .  . . . .  . . . .  . . . .
0001: 10 11 12 13  14 15 16 17  18 19 1a 1b  1c 1d 1e 1f  | . . . .  . . . .  . . . .  . . . .
0002: 20 21 22 23  24 25 26 27  28 29 2a 2b  2c 2d 2e 2f  | . . " #  $ % & '  ( ) * +  , - . /
0003: 30 31 32 33  34 35 36 37  38 39 3a 3b  3c 3d 3e 3f  | 0 1 2 3  4 5 6 7  8 9 : ;  < = > ?
0004: 40 41 42 43  44 45 46 47  48 49 4a 4b  4c 4d 4e 4f  | @ A B C  D E F G  H I J K  L M N O
0005: 50 51 52 53  54 55 56 57  58 59 5a 5b  5c 5d 5e 5f  | P Q R S  T U V W  X Y Z [  \ ] ^ _
0006: 60 61 62 63  64 65 66 67  68 69 6a 6b  6c 6d 6e 6f  | ` a b c  d e f g  h i j k  l m n o
0007: 70 71 72 73  74 75 76 77  78 79 7a 7b  7c 7d 7e 7f  | p q r s  t u v w  x y z {  | } . .
0008: 80 81 82 83  84 85 86 87  88 89 8a 8b  8c 8d 8e 8f  | . . . .  . . . .  . . . .  . . . .
0009: 90 91 92 93  94 95 96 97  98 99 9a 9b  9c 9d 9e 9f  | . . . .  . . . .  . . . .  . . . .
0010: a0 a1 a2 a3  a4 a5 a6 a7  a8 a9 aa ab  ac ad ae af  | . ¡ ¢ £  ¤ ¥ ¦ §  ¨ © ª «  ¬ ­ ® ¯
0011: b0 b1 b2 b3  b4 b5 b6 b7  b8 b9 ba bb  bc bd be bf  | ° ± ² ³  ´ µ ¶ ·  ¸ ¹ º »  ¼ ½ ¾ ¿
0012: c0 c1 c2 c3  c4 c5 c6 c7  c8 c9 ca cb  cc cd ce cf  | À Á Â Ã  Ä Å Æ Ç  È É Ê Ë  Ì Í Î Ï
0013: d0 d1 d2 d3  d4 d5 d6 d7  d8 d9 da db  dc dd de df  | Ð Ñ Ò Ó  Ô Õ Ö ×  Ø Ù Ú Û  Ü Ý Þ ß
0014: e0 e1 e2 e3  e4 e5 e6 e7  e8 e9 ea eb  ec ed ee ef  | à á â ã  ä å æ ç  è é ê ë  ì í î ï
0015: f0 f1 f2 f3  f4 f5 f6 f7  f8 f9 fa fb  fc fd fe ff  | ð ñ ò ó  ô õ ö ÷  ø ù ú û  ü ý þ ÿ
0016: 00 01 02 03  04 05 06 07  08 09                     | . . . .  . . . .  . ."#;
        let actual = to_hex_pretty(&v);
        let diff_res = diff(expected, actual.as_str(), "\n");
        print_diff(expected, actual.as_str(), "\n");
        assert_eq!(diff_res.0, 1);
    }

    #[test]
    fn hex_line() {
        unittest::setup::log::configure();
        let mut v: Vec<u8> = Vec::new();
        for i in 0..=18 {
            v.push(i);
        }
        let expected = "0000: 00 01 02 03  04 05 06 07  08 09 0a 0b  0c 0d 0e 0f  | . . . .  . . . .  . . . .  . . . . 0001: 10 11 12  | . . . ";
        let actual = to_hex_line(&v);
        let diff_res = diff(expected, actual.as_str(), "\n");
        print_diff(expected, actual.as_str(), "\n");
        assert_eq!(diff_res.0, 0);
    }
}
