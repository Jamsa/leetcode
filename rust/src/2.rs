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

struct Solution{}

impl Solution {
    pub fn add_two(l:Option<Box<ListNode>>,r:Option<Box<ListNode>>,carry:i32)->Option<Box<ListNode>>{
        if l==None&&r==None{
            if carry == 0{
                return None;
            }else{
                return Some(Box::new(ListNode::new(carry)));
            }
        }

        let (lnext,lval) = match l {
            Some(node_box) => ((*node_box).next,(*node_box).val),
            _ => (None,0)
        };
        let (rnext,rval) = match r {
            Some(node_box) => ((*node_box).next,(*node_box).val),
            _ => (None,0)
        };
        let total = lval + rval + carry;
        let value = total % 10;
        let carry:i32 = total / 10;
        let child = Self::add_two(lnext,rnext,carry);
        // let child = if carry > 0 || lnext != None || rnext != None {
        //     Self::add_two(lnext,rnext,carry)
        // }else{
        //     None
        // };
        let mut result = ListNode::new(value);
        result.next = child;
        Some(Box::new(result))
    }
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::add_two(l1,l2,0)
    }
}


fn main() {
    println!("{:?}",Solution::add_two_numbers(None,None));
    let l = Some(Box::new(ListNode::new(0)));
    let r = Some(Box::new(ListNode::new(0)));
    println!("{:?}",Solution::add_two_numbers(l,r));
    let l = Some(Box::new(ListNode::new(1)));
    let r = Some(Box::new(ListNode::new(2)));
    println!("{:?}",Solution::add_two_numbers(l,r));

    let mut lv = ListNode::new(1);
    let l1 = Some(Box::new(ListNode::new(2)));
    lv.next = l1;
    let l = Some(Box::new(lv));

    let mut rv = ListNode::new(1);
    let r1 = Some(Box::new(ListNode::new(2)));
    rv.next = r1;
    let r = Some(Box::new(rv));

    println!("{:?}",Solution::add_two_numbers(l,r));
}
