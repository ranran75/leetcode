pub enum ListNode {
    Cons(i32, Box<ListNode>),
    Nil,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode::Cons(val, Box::new(ListNode::Nil))
    }
    pub fn new_with_vec(values: Vec<i32>) -> Self {
        values
            .iter()
            .rev()
            .fold(ListNode::Nil, |acc, &val| acc.prepend(val))
    }
    fn prepend(self, elem: i32) -> Self {
        ListNode::Cons(elem, Box::new(self))
    }

    fn head_replace(self, elem: i32) -> Self {
        match self {
            ListNode::Cons(_, tail) => ListNode::Cons(elem, tail),
            ListNode::Nil => ListNode::Nil,
        }
    }
    pub fn stringify(&self) -> String {
        match self {
            ListNode::Cons(head, tail) => format!("{head} -> {}", tail.stringify()),
            ListNode::Nil => format!("Nil"),
        }
    }
    fn len(&self) -> i32 {
        match self {
            ListNode::Cons(_, tail) => 1 + tail.len(),
            ListNode::Nil => 0,
        }
    }

    fn sum(&self) -> i32 {
        match self {
            ListNode::Cons(head, tail) => head + tail.sum(),
            ListNode::Nil => 0,
        }
    }
}
