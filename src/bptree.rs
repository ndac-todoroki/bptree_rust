//! A module representing the whole B+-Tree.
//!
//! This module includes the B+-Tree struct, and function implementations to
//! add/lookup items of it.
//!
//! # Usage Examples
//! The usages below are for this module; You should be looking for the crate
//! documentation when working with the crate from your project.
//!
//! ```
//! use bptree::BPlusTree;
//!
//! // a tree with a node size of 5
//! // this must be mutable; unless you don't want to change anything
//! let mut tree = BPlusTree::new(5);
//!
//! tree.insert(2, 200); // key and value
//!
//! let result1 = tree.lookup(2);
//! let result2 = tree.lookup(4);
//!
//! assert_eq!(Some(200), result1);
//! assert_eq!(None, result2);
//! ```
//!
//! # Printing
//! You can also print/format trees. Debug formatting are derived from `std`
//! and `core` crates. Take the example below:
//!
//! ```
//! # use bptree::BPlusTree;
//! let mut tree = BPlusTree::new(4);
//! tree.insert(1, 1);
//! tree.insert(3, 2);
//! tree.insert(5, 3);
//! tree.insert(2, 4);
//! tree.insert(4, 5);
//! tree.insert(6, 6);
//! tree.insert(8, 7);
//!
//! print!("{}", tree);
//! ```
//!
//! and it will print like:
//!
//! ```ignore
//! [[1, 2], [3, 4], [5, 6, 8]]
//! ```
//!
//! You can always pretty debug with `print!("{:#?}", tree)` too.

mod node;

use std::fmt;

use self::node::InsertResult;
pub use self::node::{ExternalNode, InternalNode, Key, Node, NodeType, Value};

#[derive(Debug, Clone)]
pub struct BPlusTree {
   node_size: usize,
   root:      NodeType,
}

impl BPlusTree {
   pub fn new(node_size: usize) -> Self {
      BPlusTree {
         node_size,
         root: NodeType::Ext(ExternalNode::new(node_size)),
      }
   }

   pub fn insert(&mut self, key: Key, value: Value) -> Result<(), ()> {
      use self::InsertResult::*;

      match self.root.insert(key, value) {
         Ok(Open) => Ok(()),
         Ok(Full) => {
            let (node1, node2, key) = self.root.meiosis();
            let new_root = InternalNode::new_by_nodes(self.node_size, node1, node2, key);
            self.root = NodeType::Int(new_root);
            Ok(())
         },
         _ => Err(()),
      }
   }

   /// lookups for a key by the given
   pub fn lookup(&self, key: Key) -> Option<Value> { self.root.lookup(key) }

   pub fn height(&self) -> usize { self.root.height() }
}

// print! などの際につかうフォーマッタ定義
impl fmt::Display for BPlusTree {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      self.root.fmt(f)?;
      Ok(())
   }
}
