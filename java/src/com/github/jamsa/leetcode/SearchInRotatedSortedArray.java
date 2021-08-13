package com.github.jamsa.leetcode;

/**
 * 33. 搜索旋转排序数组
 * https://leetcode-cn.com/problems/search-in-rotated-sorted-array/
 *
 *
 * @time : 2021/8/3 7:31 下午
 * @author: zhujie
 */
public class SearchInRotatedSortedArray {
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
        SearchInRotatedSortedArray solution = new SearchInRotatedSortedArray();
        int[] nums = new int[]{4, 5, 6,7,0,1,2};

        System.out.println(solution.search(nums,0));
    }
}
