package com.github.jamsa.leetcode;

import java.util.ArrayList;
import java.util.List;
import java.util.Stack;

/**
 * 32. 最长有效括号
 * https://leetcode-cn.com/problems/longest-valid-parentheses/
 *
 * 思路一：（有问题）
 * longestValidParentheses1
 * 累计值左括号+1，右括号减1。累计值减1时，如果累计值大于等于0，计数器加2。
 * 计数器加2时，比较计数器与当前最大计数器，在最大计数器上保存较大值。
 * 如果累计值小于0，且遇到右括号，则累计值和计数器立即置0。
 * 最大计数器即为结果。
 * 存在的问题是连续多个闭括号，如()()形式时，计数器不会连续计数。
 * 应该需要增加额外的记录每段计数器值是否连续的数组，在数组中对连续的计数器值进行合并。
 * 再找到数组中的最大值。
 *
 * 思路二：
 * 建立与字符串等长的整数数组arr
 * 利用Stack找出所有成对的括号的位置，在arr中将这些位置都设置为0，其余值为1。
 * 找出arr中连续为0的切片的最大长度即为结果
 *
 * @time : 2021/7/26 7:49 下午
 * @author: zhujie
 */
public class Solution32 {
    public int longestValidParentheses1(String s) {
        //分段计数结果
        List<Integer> results = new ArrayList<>();
        //分段计数器
        int count = 0;
        //分段计数结果
        int result = 0;
        char[] chars = s.toCharArray();
        for(int i=0;i<chars.length;i++){
            //count为负时，总是设置为0
            if(count<0) count = 0;

            //左括号增加
            if(chars[i]=='('){
                count++;
            }else{
                //右括号减少
                count--;
                if(count>=0){
                    result += 2; //减少时，如果count计数仍不小于0，则表明仍能匹配上左括号，每匹配一次分段计数加2
                    if(count==0) { //如果匹配后count为0了，则表明分段匹配完毕
                        results.add(result);
                        result = 0;
                    }
                }else{
                    //小于0则表明出现了不匹配的情况，需要插入一个0值，标识不同分段间的间隔
                    results.add(0);
                }
            }
        }

        //末尾未结束，需要插入分段间隔和已经匹配的值
        if(result!=0 && count>0){
            results.add(0);
            results.add(result);
        }

        //合并连续不为0的值，找出最大的结果
        int maxValue = 0;
        int value = 0;
        for(Integer row:results){
            //System.out.print(row+",");
            if(row>0){
                value += row;
            }else{
                if(value>maxValue) maxValue = value;
                value = 0;
            }
        }
        if(value>maxValue) maxValue = value;
        //System.out.println("");
        return maxValue;
    }

    public int longestValidParentheses(String s) {
        char[] chars = s.toCharArray();
        int[] arr = new int[chars.length];
        Stack<Integer> st = new Stack<>();
        for(int i=0;i<chars.length;i++){
            arr[i] = 1;
            if(chars[i]=='('){
                st.push(i);
            }else{
                if(!st.empty()){
                    Integer a = st.pop();
                    //System.out.println(i+","+a);
                    arr[a] = 0;
                    arr[i] = 0;
                }
            }
        }

        int maxCount = 0;
        int count = 0;
        for(int i=0;i<arr.length;i++){
            //System.out.print(arr[i]+",");
            if(arr[i]==0){
                count++;
            }else{
                if(count>maxCount) maxCount = count;
                count = 0;
            }
        }
        if(count>maxCount) maxCount = count;
        //System.out.println("");
        return maxCount;
    }

    public static void main(String[] args) {
        Solution32 solution = new Solution32();
        System.out.println(solution.longestValidParentheses("()(()"));
        System.out.println(solution.longestValidParentheses(")(()))"));
        System.out.println(solution.longestValidParentheses("(()"));
        System.out.println(solution.longestValidParentheses(")()())"));
        System.out.println(solution.longestValidParentheses(""));

        System.out.println("=============");

        System.out.println(solution.longestValidParentheses1("()(()"));
        System.out.println(solution.longestValidParentheses1(")(()))"));
        System.out.println(solution.longestValidParentheses1("(()"));
        System.out.println(solution.longestValidParentheses1(")()())"));
        System.out.println(solution.longestValidParentheses1(""));
    }
}
