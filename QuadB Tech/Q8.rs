pub struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

fn max_depth<T>(node: &Option<Box<TreeNode<T>>>) -> i32 {
    match node {
        Some(node) => {
            let left_depth = max_depth(&node.left);
            let right_depth = max_depth(&node.right);
            i32::max(left_depth, right_depth) + 1
        },
        None => 0,
    }
}

fn main() {
    // Create a binary tree for testing
    let tree = Some(Box::new(TreeNode {
        value: 1,
        left: Some(Box::new(TreeNode {
            value: 2,
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            value: 3,
            left: None,
            right: None,
        })),
    }));

    let depth = max_depth(&tree);
    println!("The maximum depth of the tree is: {}", depth);
}
