mod node;

use std::fmt;

pub use self::node::{ExternalNode, InternalNode, Key, Node, NodeType, Value};
use self::node::{InsertResult};

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
            let (node1, node2) = self.root.meiosis();
            let new_root = InternalNode::new_by_nodes(self.node_size, node1, node2);
            self.root = NodeType::Int(new_root);
            Ok(())
         },
         _ => Err(()),
      }
   }

   /// lookups for a key by the given
   pub fn lookup(&self, key: Key) -> Option<Value> { self.root.lookup(key) }
}

// print! などの際につかうフォーマッタ定義
impl fmt::Display for BPlusTree {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      self.root.fmt(f)?;
      Ok(())
   }
}
