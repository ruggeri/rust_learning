use std::fmt;

use super::bst::BST;

pub struct NodeData<T> {
  pub value: T,
  pub left: Box<BST<T>>,
  pub right: Box<BST<T>>,
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
