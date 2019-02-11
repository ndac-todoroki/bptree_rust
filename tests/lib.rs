use bptree;

#[test]
fn displays_all_keys() {
   let node_size = 5;
   let mut tree = bptree::BPlusTree::new(node_size);
   for i in 1..=10 {
      tree.insert(i, i);
   }
   assert_eq!("[[1, 2, 3]4[4, 5, 6]7[7, 8, 9, 10]]", format!("{}", tree));
}
