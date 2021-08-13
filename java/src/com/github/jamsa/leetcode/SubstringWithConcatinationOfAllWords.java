package com.github.jamsa.leetcode;

import java.sql.Array;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Map;
import java.util.function.Function;
import java.util.stream.Collectors;

/**
 * @time : 2021/7/14 7:59 下午
 * @author: zhujie
 */
public class SubstringWithConcatinationOfAllWords {

    private Long countSubstring(String s,String substr){
        long count = 0;
        int index = 0;
        while(index>-1){
            index = s.indexOf(substr, index);
            if(index>-1){
                count ++;
                index+=substr.length();
            }
        }
        return count;
    }

    private Map<String,Long> elementCount(String[] words){
        return Arrays.stream(words).collect(Collectors.groupingBy(Function.identity(),Collectors.counting()));
    }

    private Map<String,Long> elementCount(List<String> words){
        return words.stream().collect(Collectors.groupingBy(Function.identity(),Collectors.counting()));
    }

    /**
     * 根据题目要求 words 中各个元素的长度相同！！！
     * @param s
     * @param words
     * @return
     */
    public List<Integer> findSubstring(String s, String[] words) {
         if(s==null||s.trim().equals("")||words==null||words.length==0) return new ArrayList<Integer>();

         List<Integer> result = new ArrayList<>();
         //每个词出现的次数
         Map<String,Long> wordsCount = elementCount(words);
         int wordLen = words[0].length();

         //所有词的总长度
        //Integer allWordsLen = Arrays.stream(words).map(e->e.length()).reduce(0,Integer::sum);
        Integer allWordsLen = wordLen * words.length;

        //每次取一个长度为所有关键字长度的片段
        for(int i=0;i<s.length()-allWordsLen+1;i++){
            String tmp = s.substring(i,i+allWordsLen);
            //按wordLen截取tmp
            List<String> tmpWords = new ArrayList<>();
            for(int j=0;j<allWordsLen;j+=wordLen){
                tmpWords.add(tmp.substring(j,j+wordLen));
            }
            Map<String,Long> tmpCount = elementCount(tmpWords);
            if(tmpCount.equals(wordsCount)) result.add(i);

            /*
            boolean flag = true;
            for (String word:wordsCount.keySet()){
                //关键字数量与tmp中的关键字出现数量不同，直接这样计数是不行的，因为中间可能被截断。
                //如：s = "ababaab", words = ["ab","ba","ba"]，检查至babaab时，应该是能正常匹配的，实际上ba count = 2，ab count = 2，导致匹配失败
                if (wordsCount.get(word)!=countSubstring(tmp,word)){
                    flag = false;
                    break;
                }
            }
            if(flag) result.add(i);
             */
        }
        return result;
    }

    public void printResult(List<Integer> result){
        System.out.println(String.join(",",result.stream().map(e->e+"").collect(Collectors.toList())));
    }

    public static void main(String[] args) {
        SubstringWithConcatinationOfAllWords solution = new SubstringWithConcatinationOfAllWords();
        //System.out.println(solution.countSubstring(s,"ba"));

        List<Integer> result = solution.findSubstring("ababaab",new String[]{"ab","ba","ba"});
        solution.printResult(result);

        result = solution.findSubstring("barfoothefoobarman",new String[]{"foo","bar"});
        solution.printResult(result);


    }
}
