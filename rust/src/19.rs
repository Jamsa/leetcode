/*
 * 删除链表的倒数第 N 个结点
 * https://leetcode-cn.com/problems/remove-nth-node-from-end-of-list/
 */

struct Solution {}

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

impl Solution {

    /* 无法使用快慢指针的形式，因为无法同时使用可变引用的不可变引用指向head
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut fast = head.as_ref();
        let mut slow = head.as_ref();
        let mut distance = 0;

        while fast.is_some(){
            fast = fast.unwrap().next.as_ref();
            distance += 1;
            if distance>n{
                slow = slow.unwrap().next.as_ref()
            }
        }

        if distance>n && slow.is_some(){
            let next = slow.unwrap().next.as_ref();
            if next.is_some(){
                //let aa = next.unwrap().as_mut();
            }
        }

        return head;
    }
    */

    /*
    * 不使用 unsafe 代码，两次扫描
    * 第一次遍历得到元素数量，第二次找到要删除的元素进行删除。找到元素的结果分两种情况，一种是第一个元素，一种是中间元素
    */
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut v = head.clone();

        let mut cur = v.as_mut();
        let mut count = 0;
        while cur.is_some(){
            //计数记录的是每个next操作，最后cur会落在结束的None上，因此count数量与元素数量相同（不含结束的None）
            cur = cur.unwrap().next.as_mut();
            count += 1;
        }

        count = count - n ;
        //println!("index:{:?},count:{:?},n:{:?}",count,count + n,n);

        cur = v.as_mut(); // 0 节点

        if count == 0 {
            //移除首节点
            if let Some(node) = cur {
                let next = node.next.take();
                v = next;
            }else{
                v = None;
            }
        } else if count > 0 {
            //移除中间节点，先移动到被移除节点的前面count - 1
            while cur.is_some() && count - 1 > 0{
                cur = cur.unwrap().next.as_mut();
                count -= 1;
            }

            if let Some(node) = cur {
                let next = node.next.as_mut().take();
                if next.is_some(){
                    let nnext = next.unwrap().next.take();
                    node.next = nnext;
                }else{
                    node.next = None;
                }
                //node.next = node.next.as_mut().unwrap().next.take();
            }else{
                // panic! count > 0 时，不应该出现
                return None;
            }
        }else{
            return None;
        }

        return v;
    }

}

fn main() {
    let mut a = ListNode::new(1);
    let mut b = ListNode::new(2);
    let mut c = ListNode::new(3);
    let mut d = ListNode::new(4);
    let e = ListNode::new(5);
    d.next=Some(Box::new(e));
    c.next=Some(Box::new(d));
    b.next=Some(Box::new(c));
    a.next = Some(Box::new(b));


    println!(
        "result:{:?}\n",
        Solution::remove_nth_from_end(Some(Box::new(a)),2)
    );


    println!(
        "result:{:?}\n",
        Solution::remove_nth_from_end(Some(Box::new(ListNode::new(1))),1)
    );


    let mut a = ListNode::new(1);
    let b = ListNode::new(2);

    a.next = Some(Box::new(b));
    println!(
        "result:{:?}\n",
        Solution::remove_nth_from_end(Some(Box::new(a)),2)
    );

}
