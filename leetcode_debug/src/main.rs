/*
 * @lc app=leetcode.cn id=29 lang=rust
 *
 * [29] 两数相除
 */

// @lc code=start
mod list_node;
use crate::list_node::*;

// use leetcode_debug::list_node::*;
struct Solution {}

fn main() {
    print!(
        "{:?}",
        Solution::swap_pairs(build_listnode(vec![1,2,3,4,5]))
    );
}

impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list = ListNode::new(0);
        let mut tail = &mut list.next;

        let ref mut t = 8;
        *t= *t + 1;
        // let cx = &mut list.next;
        let mut temp = None;
        while let Some(mut node) = head.take() {
            head = node.next.take();

            match temp.take(){
                None => temp = Some(node),
                Some(temp) => {
                    node.next = Some(temp);
                    *tail = Some(node);
                    tail = &mut tail
                        .as_mut().unwrap().next
                        .as_mut().unwrap().next;
                }
            }
        }

        *tail = temp;
        list.next.take()
    }
}