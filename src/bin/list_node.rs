/**
<https://leetcode.cn/problems/find-nearest-point-that-has-the-same-x-or-y-coordinate/>
*/
use leetcode::list_node::ListNode;

fn main() {
    let mut values = vec![1, 2, 3, 4, 5];
    values.push(100);
    let list_node = ListNode::new_with_vec(values);
    println!("{}", list_node.stringify());
}
