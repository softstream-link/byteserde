// #![feature(test)]
// extern crate test;
// use test::Bencher;

// use std::{self, hint::black_box};
// #[derive(Debug)]
// struct Error {
//     message: String,
// }
// type Result<T> = std::result::Result<T, Error>;

// struct Deserializer<'a> {
//     bytes: &'a [u8],
//     idx: usize,
// }

// impl<'a> Deserializer<'a> {
//     pub fn deserialize_bytes<const N: usize>(&mut self) -> Result<[u8; N]> {
//         match self.bytes.get(self.idx..self.idx + N) {
//             Some(v) => {
//                 self.idx += N;
//                 Ok(v.try_into().unwrap()) // this shall not panic since slice method succeeded
//             }
//             None => {
//                 // ****** WE KNOW THIS ARM IS NEVER MATCHED BUT CHANGING IRRELEVANT HERE AFFECTS BENCHMARKS ******
//                 let e = Error {
//                     message: "".to_string(),                            // (A)    test test_playground ... bench:         702 ns/iter (+/- 76)
//                     // message: format!("{}", "a".repeat(usize::MAX)),  // (B)    test test_playground ... bench:         977 ns/iter (+/- 112)
//                 };

//                 // uncommenting print                                   // (A=on) test test_playground ... bench:       1,347 ns/iter (+/- 58)
//                 // uncommenting print                                   // (B=on) test test_playground ... bench:         972 ns/iter (+/- 104)
//                 // println!("Does not appear in terminal");                  
//                 Err(e)
//             }
//         }
//     }
//     pub fn new(bytes: &'a [u8]) -> Self {
//         Self { bytes, idx: 0 }
//     }
// }

// fn workload(buf: &[u8]) -> Result<()> {
//     let mut des = black_box(Deserializer::new(buf));
//     for _ in 0..400 {
//         let r = black_box(des.deserialize_bytes::<2>());
//         match r {
//             Err(e) => {
//                 panic!("failed {}", e.message);
//             }
//             _ => {}
//         }
//     }
//     Ok(())
// }

// #[bench]
// fn test_playground(b: &mut Bencher) {
//     let buf: [u8; 1024] = [0; 1024];
//     b.iter(|| workload(&buf));
// }
