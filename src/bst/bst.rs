// TODO: is_balanced, range.

use std::cmp;
use std::fmt;

use super::node_data::NodeData;

pub enum BST<T> {
  Nil,
  Node(NodeData<T>)
}

impl<T: fmt::Debug> fmt::Debug for BST<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      BST::Nil => f.write_str("Nil"),
      BST::Node(ref node_data) => {
        f.debug_tuple("Node").field(&node_data).finish()
      }
    }
  }
}

impl<T: PartialOrd> BST<T> {
  pub fn new() -> BST<T> {
    BST::Nil
  }

  pub fn insert(&mut self, t: T) -> &mut BST<T> {
    match self {
      BST::Nil => {
        *self = BST::Node(NodeData {
          value: t,
          left: Box::new(BST::Nil),
          right: Box::new(BST::Nil),
        });
      },

      BST::Node(nd) if t < nd.value => { nd.left.insert(t); },
      BST::Node(nd) if nd.value == t => {},
      BST::Node(nd) if nd.value < t => { nd.right.insert(t); },
      _ => { unreachable!() },
    }

    self
  }

  pub fn find(&self, t: T) -> Option<&BST<T>> {
    match self {
      BST::Nil => None,
      BST::Node(nd) if t < nd.value => nd.left.find(t),
      BST::Node(nd) if nd.value == t => Some(self),
      BST::Node(nd) if nd.value < t => nd.right.find(t),
      _ => { unreachable!() },
    }
  }

  pub fn maximum(&self) -> Option<&T> {
    match self {
      BST::Nil => None,
      BST::Node(NodeData { value, right: box BST::Nil, .. }) => Some(value),
      BST::Node(NodeData { right, .. }) => right.maximum(),
    }
  }

  pub fn remove_maximum(self) -> (BST<T>, Option<T>) {
    match self {
      BST::Nil => (self, None),
      BST::Node(node_data) => {
        let (new_bst, removed_value) = node_data.remove_maximum();
        (new_bst, Some(removed_value))
      },
    }
  }

  pub fn remove(self, t: T) -> BST<T> {
    match self {
      BST::Nil => BST::Nil,
      BST::Node(node_data) => node_data.remove(t),
    }
  }

  pub fn depth(&self) -> i32 {
    match self {
      BST::Nil => 0,
      BST::Node(NodeData {box left, box right, ..}) => {
        1 + cmp::max(left.depth(), right.depth())
      }
    }
  }
}
