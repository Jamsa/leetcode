package com.github.jamsa.leetcode;

/**
 * @time : 2021/7/9 2:35 下午
 * @author: zhujie
 */
public class ListNode {
    int val;
    ListNode next;

    ListNode() {
    }

    ListNode(int val) {
        this.val = val;
    }

    ListNode(int val, ListNode next) {
        this.val = val;
        this.next = next;
    }

    public static ListNode arrayToList(int[] data) {
        if (data == null || data.length == 0) return null;
        ListNode root = new ListNode(0);
        ListNode node = root;
        for (int i = 0; i < data.length; i++) {
            node.next = new ListNode(data[i]);
            node = node.next;
        }

        return root.next;
    }

    public static void printNode(ListNode node) {
        if (node == null) {
            System.out.println("");
            return;
        }
        ListNode p = node;
        System.out.print(p.val + "->");
        while (p.next != null) {
            p = p.next;
            System.out.print(p.val + "->");
        }
        System.out.println("");
    }
}
