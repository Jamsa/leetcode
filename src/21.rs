/*
 * 合并两个有序链表
 * https://leetcode-cn.com/problems/merge-two-sorted-lists/
 */


// Definition for singly-linked list.
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

struct Solution {}

impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>{
        let mut rv = Vec::<i32>::new();
        let mut v1 = l1.as_ref();
        let mut v2 = l2.as_ref();
        loop{
            match (v1.as_ref(),v2.as_ref()) {
                (Some(x),Some(y))=>{
                    if x.val > y.val{
                        rv.push(y.val);
                        //rv.push(x.val);
                        v2 = y.next.as_ref();
                    }else{
                        rv.push(x.val);
                        //rv.push(y.val);
                        v1 = x.next.as_ref();
                    }
                },
                (None, None) => {break;},
                (None, Some(y)) => {
                    rv.push(y.val);
                    v2 = y.next.as_ref();
                },
                (Some(x), None) => {
                    rv.push(x.val);
                    v1 = x.next.as_ref();
                },
            }
        }

        return Self::vec_to_list(rv);
    }

    // pub fn vec_to_list1(v: Vec<i32>) -> Option<Box<ListNode>> {
    //     let mut node: Option<&mut Box<ListNode>> = None;

    //     for i in v.iter(){
    //         if node.is_none(){
    //             node = Some(Box::new(ListNode::new(*i))).as_mut();
    //         }else{
    //             let mut nnode = Some(Box::new(ListNode::new(*i)));
    //             let aa = nnode.as_mut();
    //             node.unwrap().next = nnode;
    //             node = aa;
    //         }
    //     }
    //     println!("{:?}",node);
    //     //let b = node.unwrap().to_owned();
    //     //return Some(b);
    //     return None;
    // }

    pub fn vec_to_list(v: Vec<i32>) -> Option<Box<ListNode>> {
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
        println!("{:?}",node);
        return node;
    }
}

fn main() {
    let l1 = vec![1,2,4];
    let l2 = vec![1,3,4];

    println!(
        "result:{:?}\n",
        Solution::merge_two_lists(Solution::vec_to_list(l1),Solution::vec_to_list(l2)));

}
