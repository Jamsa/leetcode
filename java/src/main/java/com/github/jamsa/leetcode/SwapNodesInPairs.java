package com.github.jamsa.leetcode;

import java.util.Arrays;
import java.util.PriorityQueue;

/**
 * 24. 两两交换链表中的节点
 * https://leetcode-cn.com/problems/swap-nodes-in-pairs/
 *
 * @time : 2021/7/9 10:29 上午
 * @author: zhujie
 */

public class SwapNodesInPairs {

    public static ListNode swapPairs(ListNode head) {
        if (head == null ) return null;
        ListNode dummy = new ListNode(0);
        ListNode p = dummy;
        ListNode node = head;
        while(node!=null){
            ListNode p1 = node;
            ListNode p2 = p1.next==null?null:p1.next;

            if(p1!=null){
                if(p2!=null) {
                    p.next = p2;
                    p1.next = p2.next;
                    p2.next = p1;
                }else{
                    p.next = p1;
                }
                p = p1;
                node = p.next;
            }else{
                node = null;
            }
        }

        return dummy.next;
    }

    public static void main(String[] args) {
        ListNode result = swapPairs(ListNode.arrayToList(new int[]{1, 4, 5,6}));
        ListNode.printNode(result);
        ListNode result1 = swapPairs(ListNode.arrayToList(new int[]{1, 4, 5}));
        ListNode.printNode(result1);
    }
}
