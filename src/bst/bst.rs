// // TODO: Iterator. Is balanced. Do AVL.

// use std::cmp;
use std::fmt;
use std::mem;

use self::BST::*;

pub struct NodeData<T> {
  value: T,
  left: Box<BST<T>>,
  right: Box<BST<T>>,
}

pub enum BST<T> {
  Nil,
  Node(NodeData<T>),
}

impl<T: fmt::Debug> fmt::Debug for NodeData<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_struct("NodeData")
      .field("value", &self.value)
      .field("left", &self.left)
      .field("right", &self.right)
      .finish()
  }
}

impl<T: fmt::Debug> fmt::Debug for BST<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Nil => f.write_str("Nil"),
      Node(node_data) => {
        f.debug_tuple("Node").field(node_data).finish()
      }
    }
  }
}

impl<T: PartialOrd> BST<T> {
  pub fn new() -> BST<T> {
    Nil
  }

  fn take(&mut self) -> BST<T> {
    let mut rv = Nil;
    mem::swap(self, &mut rv);
    rv
  }

  pub fn value(&self) -> Option<&T> {
    match self {
      Nil => None,
      Node(NodeData { value, .. }) => Some(value),
    }
  }

  fn take_value(self) -> Option<T> {
    match self {
      Nil => None,
      Node(NodeData { value, .. }) => Some(value),
    }
  }

  pub fn find(&self, t: &T) -> &BST<T> {
    match self {
      Nil => {
        self
      },
      Node(NodeData { value, left, .. }) if *t < *value => {
        left.find(t)
      },
      Node(NodeData { value, .. }) if *t == *value => {
        self
      },
      Node(NodeData { value, right, .. }) if *value < *t => {
        right.find(t)
      },
      Node(_) => unreachable!(),
    }
  }

  // Seriously people? I have to write _mut versions for everything?
  fn find_mut(&mut self, t: &T) -> &mut BST<T> {
    match self {
      Nil => {
        self
      },
      Node(NodeData { value, left, .. }) if *t < *value => {
        left.find_mut(t)
      },
      Node(NodeData { value, .. }) if *t == *value => {
        self
      },
      Node(NodeData { value, right, .. }) if *value < *t => {
        right.find_mut(t)
      },
      Node(_) => unreachable!(),
    }
  }

  pub fn find_max(&self) -> &BST<T> {
    match self {
      Nil => self,
      Node(NodeData { right: box Nil, .. }) => self,
      Node(NodeData { right, .. }) => right.find_max(),
    }
  }

  pub fn find_max_mut(&mut self) -> &mut BST<T> {
    match self {
      Nil => self,
      Node(NodeData { left: box Nil, right: box Nil, .. }) => self,
      Node(NodeData { right: box Nil, .. }) => self,
      Node(NodeData { right, .. }) => right.find_max_mut(),
    }
  }

  pub fn insert(&mut self, t: T) -> &mut BST<T> {
    match self.find_mut(&t) {
      l @ Nil => {
        *l = Node(NodeData { value: t, left: box Nil, right: box Nil });
      },
      Node(_) => {
        // We are already inserted!
      }
    }

    self
  }

  // Returns the deleted value.
  fn delete(&mut self) -> Option<T> {
    match self {
      Nil => {
        None
      },
      Node(NodeData { left: box Nil, right, .. }) => {
        let mut node = right.take();
        mem::swap(self, &mut node);
        node.take_value()
      },
      Node(NodeData { left, right: box Nil, .. }) => {
        let mut node = left.take();
        mem::swap(self, &mut node);
        node.take_value()
      },
      Node(NodeData { value, left, .. }) => {
        let mut prev_node = left.find_max_mut();
        // Safe to unwrap because match ensures that left cannot be Nil.
        let mut rv = prev_node.delete().unwrap();
        mem::swap(value, &mut rv);
        Some(rv)
      },
    }
  }

  pub fn remove(&mut self, t: T) -> &mut BST<T> {
    self.find_mut(&t).delete();
    self
  }
}
