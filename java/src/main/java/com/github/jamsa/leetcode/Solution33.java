package com.github.jamsa.leetcode;

/**
 * 33. 搜索旋转排序数组
 * https://leetcode-cn.com/problems/search-in-rotated-sorted-array/
 *
 * 采用双指针二分法查找
 * 因为旋转排序数组总有一边是有序的，在取得左右指针的中间值后，剪除掉不含目标值的有序的一端（？）
 * 
 * @time : 2021/8/3 7:31 下午
 * @author: zhujie
 */
public class Solution33 {
    public int search(int[] nums, int target) {
        //双指针
        int left = 0;
        int right = nums.length - 1;
        while(left <= right){
            //二分
            int mid = (left + right)/2;
            if(nums[mid]==target) return mid;

            if(nums[mid]<nums[right]){
                //中间值比右指针值小
                if(nums[mid]<target && target <= nums[right]){
                    //右半边有序
                    left = mid + 1;
                }else{
                    //左边有序
                    right = mid - 1;
                }
            }else{
                //中间值比右指针值大
                if(nums[left] <= target && target < nums[mid]){
                    //左边有序
                    right = mid - 1;
                }else{
                    //右边有序
                    left = mid + 1;
                }
            }
        }
        return -1;
    }

    public static void main(String[] args) {
        Solution33 solution = new Solution33();
        int[] nums = new int[]{4, 5, 6,7,0,1,2};

        System.out.println(solution.search(nums,0));

        nums = new int[]{4, 5, 6,7,0,1,2};

        System.out.println(solution.search(nums,3));

        nums = new int[]{1};

        System.out.println(solution.search(nums,0));
    }
}
