package com.github.jamsa.leetcode;


import java.util.Arrays;
import java.util.stream.Collectors;

/**
 * 31. 下一个排列
 * https://leetcode-cn.com/problems/next-permutation/
 * 实现获取 下一个排列 的函数，算法需要将给定数字序列重新排列成字典序中下一个更大的排列。
 *
 * 如果不存在下一个更大的排列，则将数字重新排列成最小的排列（即升序排列）。
 *
 * 必须 原地 修改，只允许使用额外常数空间。
 *
 *
 * 解题思路：
 * 从后向前遍历元素，正常应该是从小至大排列的队列，如:[1,2,3,7,6,5,4]
 * 如果遇到前一元素比后一元素大的位置，如遇到 3 时发现 7 比 3大
 * 则从之前遍历过的元素中，找到最小的比 3 大的数，即 4。
 * 交换 3 和 4，之后从 7 至 数组末尾，重新排序（从小至大）
 * 得到 [1,2,4,3,5,6,7]
 *
 * 如果整个遍历过程中不存在下一个更大的排列，即，没有发生过前面的操作就遍历至了数组的第一个元素，
 * 则表明整个数组从前至后是降序排列的，只需要按题意重新升序排列即可。
 * 由于数组本身是降序的，改成升序，只需要对整个数组进行逆序排列，而不使用排序算法。
 *
 *
 * @time : 2021/7/22 7:00 下午
 * @author: zhujie
 */
public class Solution31 {

    /**
     * 数组逆序
     * 两个指针，从两头向中间逐个交换
     * @param nums 数组
     * @param from 开始下标
     * @param to 结束下标（包含）
     */
    public void reverse(int[] nums,int from,int to){
        int length = to-from+1;
        int half = length/2;
        int m = length % 2;
        half = m==0?(half-1):half;
        for(int i=from,j=to; i<=(from+half); i++,j--){
            int tmp = nums[j];
            nums[j] = nums[i];
            nums[i] = tmp;
        }
    }

    public void reverse(int[] nums){
        reverse(nums,0,nums.length-1);
    }

    /**
     * 冒泡排序
     * @param nums      数组
     * @param fromIndex 开始位置
     * @param toIndex   结束位置（包含）
     */
    public void bubbleSort(int[] nums,int fromIndex,int toIndex){
        int count = 0; // 记录最右边已经冒泡了几个元素
        for (int i = fromIndex; i < toIndex; i++) {
            //第一轮，两两比较
            for (int j = fromIndex; j < toIndex-count; j++) {
                if (nums[j]>nums[j+1]) {
                    int temp=nums[j];
                    nums[j]=nums[j+1];
                    nums[j+1]=temp;
                }
            }
            count ++;
        }
    }


    public void nextPermutation(int[] nums) {
        for(int i=nums.length-1;i>=0;i--){
            if(i<nums.length-1 && nums[i]<nums[i+1]){

                //从后向前找出比nums[i]大的数，交换位置
                for(int j=nums.length-1;j>i;j--) {
                    if(nums[j]>nums[i]) {
                        int tmp = nums[i];
                        nums[i] = nums[j];
                        nums[j] = tmp;
                        break;
                    }
                }
                //对nums[i+1]之后的数重新排序
                bubbleSort(nums,i+1,nums.length-1);
                return;
            }
        }
        reverse(nums);
    }

    public void printResult(int[] result){
        System.out.println(String.join(",", Arrays.stream(result).mapToObj(e->e+"").collect(Collectors.toList())));
    }

    public static void main(String[] args) {
        Solution31 solution = new Solution31();
        int[] nums = new int[]{1,2,3,7,6,5,4};
        solution.nextPermutation(nums);
        solution.printResult(nums);

        nums = new int[]{1,2,5};
        solution.nextPermutation(nums);
        solution.printResult(nums);

        nums = new int[]{5,2,1};
        solution.nextPermutation(nums);
        solution.printResult(nums);

        nums = new int[]{5,3,2,1};
        solution.nextPermutation(nums);
        solution.printResult(nums);
    }
}
