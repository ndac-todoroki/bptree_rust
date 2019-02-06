#![feature(slice_patterns)]

use bptree::BPlusTree;
use clap::{clap_app, crate_name, crate_version, crate_authors, value_t};
use std::io;
use std::time::{Duration, Instant};
use rand::prelude::*;
use rand::distributions::{Distribution, Uniform};

fn main() {
   // command line options and help messages
   let app = clap_app!((crate_name!()) =>
      (version: crate_version!())
      (author: crate_authors!())
      (about: "B+-Tree experiment application.")
      (@subcommand benchmark =>
         (about: "benchmarks by performing count-up, count-down, and random inputs.")
         (@arg N: +required "Size of benchmark. Defaults to 10000.")
      )
   );
   let mut matches = app.get_matches();

   if let Some(matches) = matches.subcommand_matches("benchmark") {
      let count = value_t!(matches.value_of("N"), usize).unwrap_or(1_0000);
      benchmark(count);
   } else {
      user_input_tree_loop();
   }
}

/// makes the user insert values to the tree..
fn user_input_tree_loop() {
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
               println!("{}", tree);
               println!("{:#?}", tree);
            },
            _ => println!("failed to parse key value"),
         };
      }
   }
}

/// Take benchmark of given size of all up, down, and random.
fn benchmark(n: usize) {
   println!("** 1->{}", n);
   let mut tree = BPlusTree::new(5);
   let start = Instant::now();
   for i in 1..=n {
      tree.insert(i, i);
   }
   let end = Instant::now();
   println!("TIME: {}s + {}us", end.duration_since(start).as_secs(), end.duration_since(start).subsec_micros());

   println!("");

   println!("** {}->1", n);
   let mut tree = BPlusTree::new(5);
   let start = Instant::now();
   for i in (1..=n).rev() {
      tree.insert(i, i);
   }
   let end = Instant::now();
   println!("TIME: {}s + {}us", end.duration_since(start).as_secs(), end.duration_since(start).subsec_micros());

   println!("");

   println!("** random (count {})", n);
   let mut tree = BPlusTree::new(5);
   let between = Uniform::from(1..=100_000_000);
   let mut rng = rand::thread_rng();
   let start = Instant::now();
   for _ in 1..=n {
      let i = between.sample(&mut rng);
      tree.insert(i, i);
   }
   let end = Instant::now();
   println!("TIME: {}s + {}us", end.duration_since(start).as_secs(), end.duration_since(start).subsec_micros());
}
