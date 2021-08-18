package com.github.jamsa.leetcode;

import java.util.Arrays;

/**
 *  27. 移除元素
 *  https://leetcode-cn.com/problems/remove-element/
 *
 * @time : 2021/7/12 2:36 下午
 * @author: zhujie
 */
public class Solution27 {

    public int removeElement(int[] nums, int val) {
        if(nums==null||nums.length==0) return 0;

        //p+1为填充位置，q为读取位置
        int p = -1, q = 0;
        while(q < nums.length){
            //q位置不为空的元素就向p+1位置填充
            if(nums[q]!=val){
                //如果p+1==q则没必要填充了
                if((p+1)<q) nums[p+1] = nums[q];
                p++;
            }
            q++;
        }
        return p+1;
    }

    public static void main(String[] args) {
        Solution27 solution = new Solution27();
        int[] nums = new int[]{1,1,3};
        int result = solution.removeElement(nums,1);
        Arrays.stream(nums).forEach(i->System.out.print(i+","));
        System.out.println(result);

        nums = new int[]{3,2,2,3};
        result = solution.removeElement(nums,3);
        Arrays.stream(nums).forEach(i->System.out.print(i+","));
        System.out.println(result);

        nums = new int[]{0,1,2,2,3,0,4,2};
        result = solution.removeElement(nums,2);
        Arrays.stream(nums).forEach(i->System.out.print(i+","));
        System.out.println(result);

        nums = new int[]{3,3};
        result = solution.removeElement(nums,5);
        Arrays.stream(nums).forEach(i->System.out.print(i+","));
        System.out.println(result);

        nums = new int[]{4,5};
        result = solution.removeElement(nums,4);
        Arrays.stream(nums).forEach(i->System.out.print(i+","));
        System.out.println(result);

        nums = new int[]{3,3};
        result = solution.removeElement(nums,3);
        Arrays.stream(nums).forEach(i->System.out.print(i+","));
        System.out.println(result);
    }
}
