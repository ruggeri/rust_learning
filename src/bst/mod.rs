mod bst;
mod node_data;

pub use ::bst::bst::BST;
// I think this could remain an opaque type. But then your user wouldn't
// have a lot of power to use your BST creatively. Do you trust them?
pub use ::bst::node_data::NodeData;
