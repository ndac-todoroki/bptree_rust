use bptree::BPlusTree;

fn main() {
   // N=5のB+-Treeを作る
   let mut tree = BPlusTree::new(5);

   println!("{}", tree);

   for i in 1..=20 {
      tree.insert(i, i);
      println!("{}", tree);
   }

   // もう一度初期化，今度は逆順で数字を入れる
   let mut tree = BPlusTree::new(5);

   println!("{}", tree);

   for i in (1..=20).rev() {
      tree.insert(i, i);
      println!("{}", tree);
   }
}
