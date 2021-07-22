/*
 * 合并K个升序链表
 * https://leetcode-cn.com/problems/merge-k-sorted-lists/
 */
use std::cmp::Ordering;
use std::collections::BinaryHeap;


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

//利用标准库的BinaryHeap提供的PriorityQueue https://doc.rust-lang.org/std/collections/binary_heap/
impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val).reverse() //从小到大排列
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn vec_to_list(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut node = None;
    for i in v.iter().rev(){
        if node.is_none(){
            node = Some(Box::new(ListNode::new(*i)))
        }else{
            let mut nnode = Some(Box::new(ListNode::new(*i)));
            nnode.as_mut().unwrap().next = node;
            node = nnode;
        }
    }
    //println!("{:?}",node);
    return node;
}

struct Solution {}

impl Solution {

    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }

        let mut heap = BinaryHeap::with_capacity(lists.len());
        lists.iter().for_each(|x|{
            if x.is_some(){
                heap.push(x.as_ref().unwrap());
            }
        });
        let mut root_node = ListNode::new(0);
        let root = &mut root_node;
        let mut p = root;

        while !heap.is_empty() {
            let val = heap.pop().unwrap().val;
            p.next = Some(Box::new(ListNode::new(val)));
            p = &mut p.next;
        }

        return None;
    }
}

fn main() {
    //测试BinaryHeap
    let mut a = ListNode::new(1);
    let mut d = ListNode::new(4);
    let mut heap = BinaryHeap::new();
    heap.push(&a);
    heap.push(&d);


    let mut a = ListNode::new(1);
    let mut b = ListNode::new(2);
    let mut c = ListNode::new(3);
    let mut d = ListNode::new(4);
    let e = ListNode::new(5);
    d.next=Some(Box::new(e));
    c.next=Some(Box::new(d));
    b.next=Some(Box::new(c));
    a.next = Some(Box::new(b));

    // println!(
    //     "result:{:?}\n",
    //     Solution::merge_k_lists(vec![Some(Box::new(a))]));


    println!("{:?}",heap.pop().unwrap().val);


}
