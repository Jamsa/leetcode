package com.github.jamsa.leetcode;

/**
 * 34. 在排序数组中查找元素的第一个和最后一个位置
 * https://leetcode-cn.com/problems/find-first-and-last-position-of-element-in-sorted-array/
 *
 * nums 为无重复元素的升序排列数组
 *
 *
 * 采用双指针二分法查找，查找出中间值匹配则直接返回中间值位置。
 * 如果查找完毕后仍无法匹配，则target值应该在两个指针之间
 *
 * @time : 2021/8/18 1:48 下午
 * @author: zhujie
 */
public class Solution35 {
    public int searchInsert(int[] nums, int target) {
        if(nums.length==0 || nums[0]>=target) return 0;

        if(nums[nums.length-1]<target) return nums.length;

        int start = 0;
        int end = nums.length;
        int mid = start + (end - start)/2;
        while(end>start && start < mid){
            //System.out.println(start+"==="+mid+"==="+end);
            int midValue = nums[mid];
            if(midValue==target) return mid;
            if(midValue>target){
                end = mid;
            }else{
                start = mid;
            }
            mid = start + (end - start)/2;

            //System.out.println(start+"===="+mid+"===="+end);
        }


        if(nums[mid]>target){
            return mid;
        }else{
            return mid+1;
        }
    }

    public static void main(String[] args) {
        Solution35 solution = new Solution35();
        int[] nums = new int[]{1,3,5,6};
        int target = 5;
        int result = solution.searchInsert(nums,target);
        System.out.println(result);

        nums = new int[]{1,3,5,6};
        target = 2;
        result = solution.searchInsert(nums,target);
        System.out.println(result);

        nums = new int[]{1,3,5,6};
        target = 7;
        result = solution.searchInsert(nums,target);
        System.out.println(result);

        nums = new int[]{1,3,5,6};
        target = 0;
        result = solution.searchInsert(nums,target);
        System.out.println(result);

        nums = new int[]{1};
        target = 0;
        result = solution.searchInsert(nums,target);
        System.out.println(result);
    }
}
