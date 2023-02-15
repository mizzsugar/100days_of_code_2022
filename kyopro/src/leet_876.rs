
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut fast = head.as_ref();
    let mut slow = head.as_ref();

    while let Some(node) = fast {
        match node.next.as_ref() {
            None => break,
            Some(fast_next) => {
                fast = fast_next.next.as_ref();
                slow = slow.unwrap().next.as_ref();
            }
        }
    }
    slow.map(|n| n.clone())
}
