package com.github.jamsa.leetcode;

import java.util.Arrays;
import java.util.PriorityQueue;

/**
 * 23. 合并K个升序链表
 * https://leetcode-cn.com/problems/merge-k-sorted-lists/
 *
 * @time : 2021/7/9 10:29 上午
 * @author: zhujie
 */

public class MergeKSortedLists {

    public static ListNode mergeKLists(ListNode[] lists) {
        if (lists == null || lists.length == 0) return null;

        //利用优先级队列对数据进行排序
        PriorityQueue<ListNode> queue = new PriorityQueue<>(lists.length, (o1, o2) -> {
            if(o1.val < o2.val) return -1;
            else if (o1.val == o2.val) return 0;
            else return 1;
        });

        //用于保存结果的首节点
        ListNode root = new ListNode(0);
        ListNode node = root;

        Arrays.stream(lists).filter(n -> n!=null).forEach(n->queue.add(n));
        System.out.println(queue.size());
        while(!queue.isEmpty()){
            node.next = queue.poll();
            System.out.println(node.next.val);
            node = node.next;
            if(node.next!=null)
                queue.add(node.next);
        }

        return root.next;
    }

    public static void main(String[] args) {

        ListNode[] lists = new ListNode[]{
                ListNode.arrayToList(new int[]{1, 4, 5}),
                ListNode.arrayToList(new int[]{1,3,4}),
                ListNode.arrayToList(new int[]{2,6})
        };
        ListNode.printNode(ListNode.arrayToList(new int[]{1, 4, 5}));

        ListNode.printNode(mergeKLists(lists));

    }
}
