package com.github.jamsa.leetcode;

/**
 * 29. 两数相除
 * https://leetcode-cn.com/problems/divide-two-integers/
 *
 * 直接用加法处理，理论上可以求解，但是遇到大数字时，求解会非常耗时
 *
 * @time : 2021/7/14 4:33 下午
 * @author: zhujie
 */
public class Solution29 {
    public int divide(int dividend, int divisor) {
        if(divisor==1) return dividend;
        if(divisor==-1){
            if(dividend>Integer.MIN_VALUE) return -dividend; //除负一取反
            return Integer.MAX_VALUE;
        }

        //避免int溢出，转为long计算
        long x = dividend;
        long y = divisor;
        //符号
        boolean neg = (x<0||y<0)?true:false;
        neg = (x<0 && y<0)?false:neg;

        x = Math.abs(x);
        y = Math.abs(y);

        long d = div(x,y);
        int result = d>Integer.MAX_VALUE?Integer.MAX_VALUE:new Long(d).intValue();

        if(neg) result = - result;
        return result;
    }

    public long div(long a,long b){
        if(a<b) return 0;

        // 结果，初始为1，因为a > b 至少为1
        long result = 1;
        // 累加值，初始为 b，当作已经加了一次了
        long acc = b;
        while((acc+acc)<=a){
            result += result; //结果也翻倍加
            acc += acc; //累计值翻倍加
            //System.out.println(a+","+acc+","+result);
        }
        return result + div(a-acc,b); //减少已经加过的数值，重新计算
    }

    public static void main(String[] args) {
        Solution29 solution = new Solution29();
        System.out.println(solution.divide(-2147483648,2));

        System.out.println(solution.divide(-2147483648,-1));
        System.out.println(solution.divide(-2147483648,1));

        System.out.println(solution.divide(10,3));

        System.out.println(solution.divide(7,-3));

        System.out.println(solution.divide(1,1));
        System.out.println(solution.divide(1,-1));
        System.out.println(solution.divide(-1,-1));

        System.out.println(solution.divide(232332231,-1));



    }
}
