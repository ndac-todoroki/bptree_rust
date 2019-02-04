use bptree::BPlusTree;
// use bptree::bptree; // ::{NodeType, ExternalNode, InternalNode}

fn main() {
   println!("Hello, world!");

   let mut tree = BPlusTree::new(5);

   println!("{}", tree);

   for i in 1..=20 {
      tree.insert(i, i);
      println!("{}", tree);
   }

   let mut tree = BPlusTree::new(5);

   println!("{}", tree);

   for i in (1..=20).rev() {
      tree.insert(i, i);
      println!("{}", tree);
   }
}
