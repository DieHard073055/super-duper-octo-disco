#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution {}
impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result_list = ListNode::new(-1);
        let mut current_node = &mut result_list;

        while list1.is_some() && list2.is_some() {
            let val1 = list1.as_ref().unwrap().val;
            let val2 = list2.as_ref().unwrap().val;

            if val1 < val2 {
                let next = list1.as_mut().unwrap().next.take();
                current_node.next = list1.take();
                list1 = next;
            } else {
                let next = list2.as_mut().unwrap().next.take();
                current_node.next = list2.take();
                list2 = next;
            }
            current_node = current_node.next.as_mut().unwrap();
        }

        if list1.is_some() {
            current_node.next = list1;
        } else {
            current_node.next = list2;
        }
        result_list.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_two_lists() {
        // Test case 1
        let l1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 4, next: None })),
                        })),
                    })),
                })),
            })),
        }));
        assert_eq!(Solution::merge_two_lists(l1, l2), expected);

        // Test case 2
        let l1 = Some(Box::new(ListNode { val: 5, next: None }));
        let l2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 7, next: None })),
                })),
            })),
        }));
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 5,
                        next: Some(Box::new(ListNode { val: 7, next: None })),
                    })),
                })),
            })),
        }));
        assert_eq!(Solution::merge_two_lists(l1, l2), expected);

        // // Test case 3
        let l1 = None;
        let l2 = Some(Box::new(ListNode { val: 0, next: None }));
        let expected = Some(Box::new(ListNode { val: 0, next: None }));
        assert_eq!(Solution::merge_two_lists(l1, l2), expected);
    }
}
