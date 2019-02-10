#![feature(slice_patterns)]

use bptree::BPlusTree;
use clap::{clap_app, crate_name, crate_version, crate_authors, value_t};
use std::io;
use std::time::Instant;
use rand;
use rand::distributions::{Distribution, Uniform};

fn main() {
   // command line options and help messages
   let app = clap_app!((crate_name!()) =>
      (version: crate_version!())
      (author: crate_authors!())
      (about: "B+-Tree experiment application.")
      (@arg node_size: --node_size -s +takes_value +global "Size of node. must be over 3. Defaults to 5")
      (@subcommand benchmark =>
         (about: "benchmarks by performing count-up, count-down, and random inputs.")
         (@arg N: "Number of data to insert. Defaults to 10000.")
      )
      (@subcommand lookup =>
         (about: "Inserts randomly, then finds a value by key number input.")
         (@arg N: "Number of data to insert. Defaults to 10000.")
      )
   );
   let matches = app.get_matches();
   let node_size = value_t!(matches.value_of("node_size"), usize).unwrap_or(5);

   match matches.subcommand() {
      ("benchmark", Some(submatch)) => {
         let count = value_t!(submatch.value_of("N"), usize).unwrap_or(1_0000);
         benchmark(count, node_size);
      },
      ("lookup", Some(submatch)) => {
         let count = value_t!(submatch.value_of("N"), usize).unwrap_or(1_0000);
         lookup_loop(count, node_size);
      },
      _ => user_input_tree_loop(node_size),
   }
}

/// makes the user insert values to the tree..
fn user_input_tree_loop(node_size: usize) {
   let mut tree = BPlusTree::new(node_size);
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
fn benchmark(n: usize, ns: usize) {
   println!("** 1->{}", n);
   let mut tree = BPlusTree::new(ns);
   let start = Instant::now();
   for i in 1..=n {
      tree.insert(i, i);
   }
   let end = Instant::now();
   println!("height:{}", tree.height());
   println!("TIME: {}s + {}us", end.duration_since(start).as_secs(), end.duration_since(start).subsec_micros());

   println!("");

   println!("** {}->1", n);
   let mut tree = BPlusTree::new(ns);
   let start = Instant::now();
   for i in (1..=n).rev() {
      tree.insert(i, i);
   }
   let end = Instant::now();
   println!("height:{}", tree.height());
   println!("TIME: {}s + {}us", end.duration_since(start).as_secs(), end.duration_since(start).subsec_micros());

   println!("");

   println!("** random (count {})", n);
   let mut tree = BPlusTree::new(ns);
   let between = Uniform::from(1..=100_000_000);
   let mut rng = rand::thread_rng();
   let start = Instant::now();
   for _ in 1..=n {
      let i = between.sample(&mut rng);
      tree.insert(i, i);
   }
   let end = Instant::now();
   println!("height:{}", tree.height());
   println!("TIME: {}s + {}us", end.duration_since(start).as_secs(), end.duration_since(start).subsec_micros());
}

fn lookup_loop(n: usize, ns: usize) {
   let mut tree = BPlusTree::new(ns);
   let between = Uniform::from(1..=100_000_000);
   let key_range = Uniform::from(1..=n);
   let mut rng = rand::thread_rng();
   for _ in 1..=n {
      let k = key_range.sample(&mut rng);
      let v = between.sample(&mut rng);
      tree.insert(k, v);
   }

   println!("Lookup for: (1 - {})", n);

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
               match tree.lookup(key) {
                  Some(value) => println!("-- value for key {} is: {}", key, value),
                  None => println!("-- key not found."),
               };
            },
            _ => println!("failed to parse key value"),
         };
      }
   }
}

