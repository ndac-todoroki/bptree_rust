use super::{InsertResult, Key, Node, NodeType, Value};
// use std::cell::Box;
use std::fmt;

#[derive(Debug, Clone)]
/// Struct representing an external node, or a leaf node.
///
/// `node_size` is used to dynamically assert node key sizes,
/// where `keys` and `values` will have the length of `node_size - 1`
pub struct ExternalNode {
   pub node_size: usize,
   pub keys:      Vec<Key>,
   pub values:    Vec<Value>,
   pub next:      Option<Box<ExternalNode>>,
}

impl fmt::Display for ExternalNode {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "[")?;

      let mut keys = self.keys.iter();

      if let Some(key) = keys.next() {
         key.fmt(f)?;

         for key in keys {
            write!(f, ", ")?;
            key.fmt(f)?;
         }
      }

      write!(f, "]")?;
      Ok(())
   }
}

impl ExternalNode {
   pub fn new(node_size: usize) -> Self {
      Self {
         node_size,
         keys: Vec::with_capacity(node_size),
         values: Vec::with_capacity(node_size),
         next: None,
      }
   }

   /// Leaf nodes should not error in inserting new key-val pairs,
   /// because the node which to insert the pair will be decided in parent
   /// nodes.
   ///
   /// # Examples
   ///
   /// ```ignore
   /// // ex_node.keys = [2, 4]
   /// // ex_node.values = [100, 200]
   /// let pos = ex_node.get_insert_position(3);
   /// assert_eq!(pos, 1);
   /// ```
   fn get_insert_position(&self, key: Key) -> Option<usize> {
      self.keys.iter().position(|&k| k > key)
   }
}

impl Node for ExternalNode {
   /// Lookup a value for the given key.
   ///
   /// Returns `None` if the key was not found.
   fn lookup(&self, key: Key) -> Option<Value> {
      self
         .keys
         .iter()
         .zip(self.values.iter())
         .find(|(&k, &_)| k == key)
         .map(|(&_, &v)| v)
   }

   /// Inserts a key-value pair into the leaf node.
   ///
   /// If full after insert, this returns `Ok(InsertResult::Full)`.
   /// If not, `Ok(InsertResult::Open)`
   fn insert(&mut self, key: Key, value: Value) -> Result<InsertResult, &str> {
      use self::InsertResult::{Full, Open};

      // fail fast
      if self.keys.len() >= self.node_size {
         return Err(
            "Could not insert key-val. Maybe the node was full? That should not happen, check \
             source.",
         );
      }

      // insert
      match self.get_insert_position(key) {
         Some(position) => {
            self.keys.insert(position, key);
            self.values.insert(position, value);
         },
         None => {
            // insert to last if `None`
            self.keys.push(key);
            self.values.push(value);
         },
      };
      if self.keys.len() == self.node_size {
         Ok(Full)
      } else {
         Ok(Open)
      }
   }

   fn first_key(&self) -> &Key { self.keys.first().unwrap() }

   fn height(&self) -> usize { 1 }

   fn meiosis(&self) -> (Box<NodeType>, Box<NodeType>, usize) {
      // on the basis that self is full...
      let cut_at = (self.node_size + 1) >> 1;

      let mut fk = self.keys.clone();
      let mut fv = self.values.clone();

      let mut lk = fk.split_off(cut_at);
      let mut lv = fv.split_off(cut_at);

      lk.reserve(self.node_size);
      lv.reserve(self.node_size);

      let lat_key = *lk.first().unwrap();

      let latter = Self {
         node_size: self.node_size,
         keys:      lk,
         values:    lv, //.to_vec(),
         next:      self.next.clone(),
      };

      let lat_box = Box::new(latter);

      let former = Self {
         node_size: self.node_size,
         keys:      fk.to_vec(),
         values:    fv.to_vec(),
         next:      Some(lat_box.clone()),
      };

      (
         Box::new(NodeType::Ext(former)),
         Box::new(NodeType::Ext(*lat_box)),
         lat_key,
      )
   }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
   use super::super::Node;
   use super::*;

   #[test]
   fn test_get_insert_position() {
      let mut node = ExternalNode::new(3);
      node.keys.push(2);
      node.values.push(100);
      node.keys.push(4);
      node.values.push(200);

      let pos = node.get_insert_position(3);
      assert_eq!(pos, Some(1));

      let pos = node.get_insert_position(5);
      assert_eq!(pos, None);
   }

   #[test]
   fn insert_returns_ok_ok_when_not_full() {
      let mut node = ExternalNode::new(3);
      node.keys.push(2);
      node.values.push(200);

      assert_eq!(Ok(InsertResult::Open), node.insert(3, 300));
   }

   #[test]
   fn insert_returns_ok_full_when_becomes_full() {
      let mut node = ExternalNode::new(2);
      node.keys.push(2);
      node.values.push(200);

      assert_eq!(Ok(InsertResult::Full), node.insert(3, 300));
   }

   #[test]
   fn insert_should_fail_when_full() {
      let mut node = ExternalNode::new(1);
      node.keys.push(2);
      node.values.push(200);

      match node.insert(3, 300) {
         Err(_) => (),
         _ => panic!(),
      };
   }

   #[test]
   fn insert_adds_one_elem_to_both_keys_and_values() {
      let node = ExternalNode::new(3);
      let mut ex_node = node.clone();
      ex_node.insert(2, 2);

      assert_eq!(ex_node.keys.len(), node.keys.len() + 1);
      assert_eq!(ex_node.values.len(), node.values.len() + 1);
   }

   #[test]
   fn test_lookup() {
      let mut node = ExternalNode::new(2);
      node.keys.push(2);
      node.values.push(200);

      assert_eq!(Some(200), node.lookup(2));
   }
}
