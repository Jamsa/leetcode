package com.github.jamsa.leetcode;

import java.util.Arrays;

/**
 *  28. 实现 strStr()
 *  https://leetcode-cn.com/problems/implement-strstr/
 *
 * @time : 2021/7/12 2:36 下午
 * @author: zhujie
 */
public class ImplementStrStr {

    public boolean cmp(String haystack,int beginIndex, int endIndex,String needle){
        for(int i=beginIndex;i<endIndex;i++){
            if(haystack.charAt(i)!=needle.charAt(i-beginIndex)) return false;
        }
        return true;
    }


    //暴力匹配
    public int strStr(String haystack, String needle) {
        if(needle==null||needle.length()==0) return 0;
        int len = needle.length();
        for(int i=0;i<haystack.length()-len+1;i++){
            /*if(haystack.substring(i,i+len).equals(needle)){
                return i;
            }*/
            if(this.cmp(haystack,i,i+len,needle)) return i;
        }
        return -1;
    }


    //TODO:  KMP 未完成
    public int strStr1(String haystack, String needle) {
        if(needle==null || needle.isEmpty()) return 0;

        int n = haystack.length();
        int m = needle.length();

        char[] s = (" " + haystack).toCharArray();
        char[] p = (" " + needle).toCharArray();

        int[] next = new int[m+1];
        for(int i = 2, j = 0; i<=m; i++){
            while(j>0 && p[i] != p[j+1]) j = next[j];
            if(p[i] == p[j+1]) j++;
            next[i] = j;
        }
        for(int i = 0;i<next.length;i++){
            System.out.print(i+",");
        }
        System.out.println("");
        return 0;
    }

    public static void main(String[] args) {
        ImplementStrStr solution = new ImplementStrStr();
        int result = solution.strStr("hello","ll");
        System.out.println(result);
        result = solution.strStr1("a","a");
        System.out.println(result);

        result = solution.strStr("hello","ll");
        System.out.println(result);

        result = solution.strStr("a","a");
        System.out.println(result);

    }
}
