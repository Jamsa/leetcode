package com.github.jamsa.leetcode;

/**
 * 34. 在排序数组中查找元素的第一个和最后一个位置
 * https://leetcode-cn.com/problems/find-first-and-last-position-of-element-in-sorted-array/
 *
 * 采用双指针二分法查找
 * 找到匹配的中间值后，向两头查找到结束位置
 * @time : 2021/8/18 1:48 下午
 * @author: zhujie
 */
public class Solution34 {
    public int[] searchRange(int[] nums, int target) {
        if(nums.length==0) return new int[]{-1,-1};
        int start = 0;
        int end = nums.length;
        int mid = start + (end - start)/2;
        while(end>start && /*mid > 0 &&*/ start < mid){
            //System.out.println(start+"==="+mid+"==="+end);
            int midValue = nums[mid];
            if(midValue==target) break;
            if(midValue>target){
                end = mid;
            }else{
                start = mid;
            }
            mid = start + (end - start)/2;

            //System.out.println(start+"===="+mid+"===="+end);
        }

        int[] result = new int[]{-1,-1};
        //向前后查找
        if(nums[mid]==target){
            result[0] = mid;
            result[1] = mid;
            int from = mid;
            int to = mid;
            while(from >= 0 && nums[from]==target){
                if(nums[from]==target) result[0] = from;
                from--;
            }
            while(to<nums.length && nums[to]==target){
                if(nums[to]==target) result[1] = to;
                to++;
            }
        }
        return result;
    }

    public static void main(String[] args) {
        Solution34 solution = new Solution34();
        int[] nums = new int[]{5,7,7,8,8,10};
        int target = 6;
        int[] result = solution.searchRange(nums,target);
        System.out.println(result[0]+","+result[1]);

        nums = new int[]{5,7,7,8,8,10};
        target = 8;
        result = solution.searchRange(nums,target);
        System.out.println(result[0]+","+result[1]);

        nums = new int[]{};
        target = 0;
        result = solution.searchRange(nums,target);
        System.out.println(result[0]+","+result[1]);

        nums = new int[]{1};
        target = 1;
        result = solution.searchRange(nums,target);
        System.out.println(result[0]+","+result[1]);

        nums = new int[]{2,2};
        target = 3;
        result = solution.searchRange(nums,target);
        System.out.println(result[0]+","+result[1]);

        nums = new int[]{2,2};
        target = 2;
        result = solution.searchRange(nums,target);
        System.out.println(result[0]+","+result[1]);

        nums = new int[]{1,2,3};
        target = 3;
        result = solution.searchRange(nums,target);
        System.out.println(result[0]+","+result[1]);
    }
}
