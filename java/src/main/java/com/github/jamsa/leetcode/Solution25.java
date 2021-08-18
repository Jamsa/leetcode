package com.github.jamsa.leetcode;

import java.util.ArrayList;
import java.util.List;

/**
 * 25. K 个一组翻转链表
 * https://leetcode-cn.com/problems/reverse-nodes-in-k-group/
 *
 * @time : 2021/7/9 10:29 上午
 * @author: zhujie
 */

public class Solution25 {

    //每次取K个元素保存起来，翻转，如果取不足K个元素，则不进行翻转
    public static ListNode reverseKGroup(ListNode head, int k) {
        if (head == null ) return null;
        List<ListNode> buff = new ArrayList<>(k);

        ListNode dummy = new ListNode(0);
        ListNode p = dummy; //当前位置指针，node的前一个位置
        ListNode node = head; //侍拼接元素指针
        while(node!=null){
            ListNode buffHead = node;
            int buffCount = 0;
            buff.clear();
            while(buffHead!=null && buffCount<k){
                buff.add(buffHead);
                buffHead = buffHead.next;
                buffCount ++;
            }

            if(buffCount==k){
                //k个元素的逆序排列连接
                //将buff中第一个元素的next设置为buff中最后一个元素的next
                buff.get(0).next = buff.get(k-1).next;
                for (int i = buff.size()-1; i>=0; i--) {
                    p.next = buff.get(i);
                    p = p.next;
                }
            }else{
                //小于k个元素的直接连接头元素
                int size = buff.size();
                if(size>0){
                    p.next = buff.get(0);
                    p = buff.get(size-1);
                }
            }
            node = p.next;
        }

        return dummy.next;
    }



    private static ListNode reverse(ListNode start) {
        ListNode previous = null; //前一节点
        ListNode current = start; //当前节点

        // 三个节点的位置 previous current next
        while(current!=null){
            ListNode next = current.next;
            current.next = previous; //p c p
            previous = current;//c c p
            current = next;//n c p
        }
        return previous;
    }

    //每次取K个元素，不保存，只记录它的开始位置，结束位置与原来的链表断开。
    //翻转断开点之前的内容，如果取不足K个元素，则不进行翻转
    //https://leetcode-cn.com/problems/reverse-nodes-in-k-group/solution/tu-jie-kge-yi-zu-fan-zhuan-lian-biao-by-user7208t/
    public static ListNode reverseKGroup1(ListNode head, int k) {
        if (head == null ) return null;

        ListNode dummy = new ListNode(0);
        dummy.next = head;

        ListNode pre = dummy;
        ListNode end = dummy;

        while(end.next!=null){
            for(int i=0;i<k && end != null;i++) end = end.next;
            if (end == null) break; //不足k个时，end会为空，也不需要继续进行翻转或拼接

            ListNode start = pre.next; //翻转开始节点
            ListNode next = end.next; //翻转结束节点的下一节点
            end.next = null; //结束节点断开与后续节点连接

            pre.next = reverse(start);//返回新的头
            start.next = next; //翻转前的头节点变成尾节点，需要拼接上翻转片段之后的节点

            //新的翻转起点
            pre = start;
            end = pre;
        }

        return dummy.next;
    }

    public static void main(String[] args) {
        ListNode result = reverseKGroup1(ListNode.arrayToList(new int[]{1, 4, 5,6}),2);
        ListNode.printNode(result);
        ListNode result1 = reverseKGroup(ListNode.arrayToList(new int[]{1, 4, 5}),3);
        ListNode.printNode(result1);

        ListNode result2 = reverseKGroup(ListNode.arrayToList(new int[]{1, 4, 5}),2);
        ListNode.printNode(result2);
    }
}
