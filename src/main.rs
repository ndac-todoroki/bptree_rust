#![feature(slice_patterns)]

use bptree::BPlusTree;
use std::io;

fn main() {
   // N=5のB+-Treeを作る
   let mut tree = BPlusTree::new(5);

   println!("Enter key and value! eg. `1 100`");

   loop {
      let mut input_text = String::new();
      io::stdin()
         .read_line(&mut input_text)
         .expect("failed to read from stdin");

      let words: Vec<&str> = input_text.trim().split(' ').collect();
      let numbers: Option<Vec<Result<usize, std::num::ParseIntError>>> = words
         .get(..)
         .map(|args| args.iter().map(|arg| arg.parse()).collect());
      if let Some(num_vec) = &numbers {
         match &num_vec[..] {
            &[Ok(key)] => {
               tree.insert(key, key);
               println!("{}", tree);
            },
            &[Ok(key), Ok(value), ..] => {
               tree.insert(key, value);
               println!("{:#?}", tree);
            },
            _ => println!("failed to parse key value"),
         };
      }
   }
}
