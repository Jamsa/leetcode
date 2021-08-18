package com.github.jamsa.leetcode;

import java.util.Arrays;

/**
 * 26. 删除有序数组中的重复项
 *  https://leetcode-cn.com/problems/remove-duplicates-from-sorted-array/
 *
 * @time : 2021/7/12 2:36 下午
 * @author: zhujie
 */
public class Solution26 {
    public int removeDuplicates(int[] nums) {
        if(nums==null||nums.length==1) return 1;
        int p = 0,q = 1;
        while(q<nums.length){
            if(nums[p]!=nums[q]){
                nums[p+1] = nums[q];
                p++;
            }
            q++;
        }
        return p + 1;
    }

    public static void main(String[] args) {
        Solution26 solution = new Solution26();
        int[] nums = new int[]{1,1,3};
        int result = solution.removeDuplicates(nums);
        Arrays.stream(nums).forEach(i->System.out.print(i+","));
        System.out.println(result);
    }
}
