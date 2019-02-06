#![feature(slice_patterns)]

use bptree::BPlusTree;
use std::io;
use rand::prelude::*;
use time::PreciseTime;

fn main() {
   println!("1->1000M");
   // N=5のB+-Treeを作る
   let mut tree = BPlusTree::new(5);
   let start = PreciseTime::now();
   for i in 1..=10000000 {
      tree.insert(i, i);
   }
   let end = PreciseTime::now();
   println!("{}", tree);
   println!("{} seconds.", start.to(end));

   println!("1000M->1");
   let mut tree = BPlusTree::new(5);
   let start = PreciseTime::now();
   for i in (1..=10000000).dec() {
      tree.insert(i, i);
   }
   let end = PreciseTime::now();
   println!("{}", tree);
   println!("{} seconds.", start.to(end));

   println!("1000M->1");
   let mut tree = BPlusTree::new(5);
   let between = Uniform::from(1..=10000000);
   let mut rng = rand::thread_rng();
   let start = PreciseTime::now();
   for _ in 1..=10000000 {
      let i = between.sample(&mut rng);
      tree.insert(i, i);
   }
   let end = PreciseTime::now();
   println!("{}", tree);
   println!("{} seconds.", start.to(end));


   // user input based tree
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
