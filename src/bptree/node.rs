mod external;
mod internal;

// use std::cell::Box;
use std::fmt;

pub type Key = usize;
pub type Value = usize;

/// Trait that all node types in a B+-tree must implement.
pub trait Node {
   /// Returns the first key of the leaf. Used when adding child to parent.
   fn first_key(&self) -> &Key;

   /// Look-ups the value of the given key, mostly by recursively searching for
   /// it.
   fn lookup(&self, key: Key) -> Option<Value>;

   /// Inserts a new key-value pair to the tree. It recursively goes down to the
   /// right leaf.
   fn insert(&mut self, key: Key, value: Value) -> Result<InsertResult, &str>;

   /// A node must _meiosis_ when it becomes full. ※meiosis == 減数分裂
   fn meiosis(&self) -> (Box<NodeType>, Box<NodeType>, usize);

   /// The height of the node.
   fn height(&self) -> usize;
}

#[derive(Debug, Clone)]
pub enum NodeType {
   Int(InternalNode),
   Ext(ExternalNode),
}

#[derive(Debug, Clone, PartialEq)]
pub enum InsertResult {
   Full,
   Open,
}

impl fmt::Display for NodeType {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         NodeType::Int(node) => node.fmt(f)?,
         NodeType::Ext(node) => node.fmt(f)?,
      };
      Ok(())
   }
}

impl Node for NodeType {
   fn first_key(&self) -> &Key {
      match self {
         // TODO: change to `Self::Foo` when #49683 is implemented
         NodeType::Int(node) => node.first_key(),
         NodeType::Ext(node) => node.first_key(),
      }
   }
   fn lookup(&self, key: Key) -> Option<Value> {
      match self {
         // TODO: change to `Self::Foo` when #49683 is implemented
         NodeType::Int(node) => node.lookup(key),
         NodeType::Ext(node) => node.lookup(key),
      }
   }
   fn insert(&mut self, key: Key, value: Value) -> Result<InsertResult, &str> {
      match self {
         NodeType::Int(node) => node.insert(key, value),
         NodeType::Ext(node) => node.insert(key, value),
      }
   }
   fn meiosis(&self) -> (Box<NodeType>, Box<NodeType>, usize) {
      match self {
         NodeType::Int(node) => node.meiosis(),
         NodeType::Ext(node) => node.meiosis(),
      }
   }
   fn height(&self) -> usize {
      match self {
         NodeType::Int(node) => node.height(),
         NodeType::Ext(node) => node.height(),
      }
   }
}

// Exports
pub use self::external::ExternalNode;
pub use self::internal::InternalNode;
