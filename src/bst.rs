// TODO: is_balanced, range.

use std::cmp;
use std::fmt;

pub struct NodeData<T> {
  value: T,
  left: Box<BST<T>>,
  right: Box<BST<T>>,
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

impl<T: PartialOrd> NodeData<T> {
  pub fn remove(mut self, t: T) -> BST<T> {
    match self {
      _ if t < self.value => {
        self.left = Box::new(self.left.remove(t));
        BST::Node(self)
      },
      _ if self.value < t => {
        self.right = Box::new(self.right.remove(t));
        BST::Node(self)
      },
      NodeData { box left, right: box BST::Nil, .. } => {
        // Nothing on right, so just move up the left.
        left
      },
      NodeData { left: box BST::Nil, box right, .. } => {
        // Nothing on left, so just move up the right.
        right
      }
      NodeData { left: box BST::Node(left_node_data), .. } => {
        // Remove the max on the left, and move it into our value.
        let (new_left, removed_value) = left_node_data.remove_maximum();
        self.value = removed_value;
        self.left = Box::new(new_left);
        BST::Node(self)
      }
    }
  }

  pub fn remove_maximum(mut self) -> (BST<T>, T) {
    match self.right {
      box BST::Nil => (*self.left, self.value),
      box BST::Node(child_data) => {
        let (new_right, removed_value) = child_data.remove_maximum();
        self.right = Box::new(new_right);
        (BST::Node(self), removed_value)
      }
    }
  }
}
