use std::collections::BinaryHeap;

pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
    let mut children = BinaryHeap::from(g);
    let mut cookies = BinaryHeap::from(s);
    let total = cookies.len();
    while let (Some(greedy), Some(size)) = (children.pop(), cookies.peek()) {
        if greedy <= *size {
            cookies.pop();
        }
    }
    return (total - cookies.len()) as i32;
}

mod tests {
    use super::find_content_children;
    #[test]
    fn test_find_content_children() {
        let g = vec![1, 2, 3];
        let s = vec![1, 1];
        let expected = 1;
        let actual = find_content_children(g, s);
        assert_eq!(actual, expected);
    }
}
