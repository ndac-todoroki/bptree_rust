use super::{InsertResult, Key, Node, NodeType, Value};
use std::cell::RefCell;
use std::fmt;
use std::mem;

#[derive(Debug, Clone)]
/// A struct representing an internal node in a B+-tree.
///
/// Each key's index is for the node **SMALLER** than its index.
/// So for example if `keys[0] == 10`, `pointer[0]` will lead to a node which
/// has smaller keys than `10`. When `keys[MAX] == 100`, the pointer leading to
/// the node holding keys equal to and above `100` is in the `greater`
/// attribute.
///
/// In other words, the keys and pointers align like below:
///
/// ```erlang
/// [pointers[0], keys[0], pointers[1], ...keys[N], greater]
/// ```
pub struct InternalNode {
   pub node_size: usize, // keys' and pointers' vec length must be (node_size - 1)
   pub keys:      Vec<Key>,
   pub pointers:  RefCell<Vec<Box<NodeType>>>,
   pub greater:   RefCell<Box<NodeType>>,
}

impl fmt::Display for InternalNode {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      use std::ops::Deref;
      let pointers = self.pointers.borrow();

      write!(f, "[")?;
      for pointer in pointers.deref() {
         pointer.fmt(f)?;
         write!(f, ", ")?;
      }
      self.greater.borrow().fmt(f)?;
      write!(f, "]")?;
      Ok(())
   }
}

impl InternalNode {
   /// The child division that should include the given key.
   fn get_child_division(&self, key: Key) -> Option<usize> {
      self.keys.iter().position(|&k| k > key)
   }

   /// Creates a new `InternalNode` by passing two child node `Box`es.
   pub fn new_by_nodes(node_size: usize, node1: Box<NodeType>, node2: Box<NodeType>, seperator_key: usize) -> Self {
      InternalNode {
         node_size,
         keys: vec![seperator_key],
         pointers: RefCell::new(vec![node1]),
         greater: RefCell::new(node2),
      }
   }

   /// Internal nodes should not be insert-able if having `(node_size - 1)`
   /// pointers. This is when `keys` have `(node_size - 2)` elements, for one
   /// exists for `greater`.
   ///
   /// - `[k/p, k/p, k/p][p]` @ N=5  is insert-able
   /// - `[k/p, k/p, k/p, k/p][p]` @ N=5  is NOT insert-able
   fn insertable(&self) -> bool { self.keys.len() <= self.node_size - 2 }
}

impl Node for InternalNode {
   /// Lookups for the value for the given key recursively.
   ///
   /// Returns `None` when key is not found.
   fn lookup(&self, key: Key) -> Option<Value> {
      let pointers = self.pointers.borrow();
      match self.get_child_division(key) {
         Some(div) => pointers[div].lookup(key),
         None => self.greater.borrow().lookup(key),
      }
   }

   /// Inserts a key-value pair into the leaf node.
   ///
   /// If full after insert, this returns `Ok(InsertResult::Full)`.
   /// If not, `Ok(InsertResult::Open)`
   fn insert(&mut self, key: Key, value: Value) -> Result<InsertResult, &str> {
      use self::InsertResult::*;

      // fail fast
      if !self.insertable() {
         return Err(
            "Could not insert key-val. Maybe the node was full? That should not happen, check \
             source.",
         );
      }

      let child_position = self.get_child_division(key);

      // insert
      match child_position {
         Some(position) => {
            let mut pointers = self.pointers.borrow_mut();
            let greater = self.greater.borrow();
            match pointers[position].insert(key, value) {
               Ok(Open) => Ok(Open),
               Ok(Full) => {
                  let (former, latter, key) = pointers[position].meiosis();

                  // 分裂した子の大きい方のキーを追加
                  self.keys.insert(position, key);

                  // ポインタの置き換え，追加
                  // 多分 Vector::remove -> Vector::insert するより mem::replace のほうが速い
                  mem::replace(&mut pointers[position], latter);
                  pointers.insert(position, former);

                  // 新しい子の追加の結果自身がいっぱいになったら `Full` を返して親に自分を分裂させる
                  if self.keys.len() == self.node_size - 1 {
                     Ok(Full)
                  } else {
                     Ok(Open)
                  }
               },
               Err(_) => {
                  Err(
                     "Could not insert key-val. Maybe the node was full? That should not happen, \
                      check source.",
                  )
               },
            }
         },
         None => {
            let mut greater = self.greater.borrow_mut();
            match greater.insert(key, value) {
               Ok(Open) => Ok(Open),
               Ok(Full) => {
                  let (former, latter, key) = greater.meiosis();

                  drop(greater);

                  self.keys.push(key);

                  self.pointers.borrow_mut().push(former);
                  //self.greater = 
                  self.greater.replace(latter);

                  if self.keys.len() == self.node_size - 1 {
                     Ok(Full)
                  } else {
                     Ok(Open)
                  }
               },
               Err(_) => {
                  Err(
                     "Could not insert key-val. Maybe the node was full? That should not happen, \
                      check source.",
                  )
               },
            }
         },
      }
   }

   fn first_key(&self) -> &Key { self.keys.first().unwrap() }

   fn meiosis(&self) -> (Box<NodeType>, Box<NodeType>, usize) {
      let pointers = self.pointers.borrow();
      if pointers.len() < 3 || self.keys.len() < 3 {
         panic!()
      } else {
         // 5 >> 1 == 2, 6 >> 1 == 3
         let div_at = (self.node_size >> 1) - 1;

         let (fk, lk) = self.keys.split_at(div_at);
         let (lkf, lks) = lk.split_first().unwrap();
         let (fp, lp) = pointers.split_at(div_at);
         let (lpf, lps) = lp.split_first().unwrap();

         let former = Self {
            node_size: self.node_size,
            keys:      fk.to_vec(),
            pointers:  RefCell::new(fp.to_vec()),
            greater:   RefCell::new(lpf.to_owned()),
         };

         let latter = Self {
            node_size: self.node_size,
            keys:      lks.to_vec(),
            pointers:  RefCell::new(lps.to_vec()),
            greater:   self.greater.to_owned(),
         };

         (
            Box::new(NodeType::Int(former)),
            Box::new(NodeType::Int(latter)),
            *lkf
         )
      }
   }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
   use super::super::{ExternalNode, Node, NodeType};
   use super::*;

   fn new_internal_node_size_5() -> InternalNode {
      let n = 5;
      let mut ex_node1 = ExternalNode::new(n);
      let mut ex_node2 = ExternalNode::new(n);

      ex_node1.insert(1, 100);
      ex_node1.insert(5, 500);

      ex_node2.insert(10, 1000);
      ex_node2.insert(50, 5000);

      let box2 = Box::new(ex_node2);
      ex_node1.next = Some(box2.clone());

      InternalNode::new_by_nodes(
         n,
         Box::new(NodeType::Ext(ex_node1)),
         Box::new(NodeType::Ext(*box2)),
      )
      // [ <-ex_node1 | 10 | <-ex_node2 ]
   }

   #[test]
   fn test_internal_node_correctly_initializes() {
      let node = new_internal_node_size_5();

      assert_eq!(1, node.keys.len());
      assert_eq!(Some(&10), node.keys.first());
   }

   #[test]
   fn test_get_child_division() {
      let node = new_internal_node_size_5();

      // should go to the first child node
      let pos = node.get_child_division(2);
      assert_eq!(pos, Some(0));

      // should go to `greater`
      let pos = node.get_child_division(10);
      assert_eq!(pos, None);
   }

   #[test]
   fn test_lookup() {
      let node = new_internal_node_size_5();

      assert_eq!(Some(500), node.lookup(5));
      assert_eq!(Some(1000), node.lookup(10));
      assert_eq!(Some(5000), node.lookup(50));
      assert_eq!(None, node.lookup(99));
   }
}
