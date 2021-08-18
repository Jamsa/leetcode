package com.github.jamsa.leetcode;

import java.util.HashSet;
import java.util.Set;

/**
 * 36. 有效的数独
 * https://leetcode-cn.com/problems/valid-sudoku/
 *
 * 按题目要求硬解
 * 校验各行、各列是否有重复元素
 * 校验每个3X3区域是否有重复元素
 * 
 * @time : 2021/8/18 3:14 下午
 * @author: zhujie
 */
public class Solution36 {
    public boolean isValidSudoku(char[][] board) {
        Set<Integer> values = new HashSet(9);

        //3x3区域左上角元素坐标
        int[][] bases = new int[][]{
                {0,0},{3,0},{6,0},
                {0,3},{3,3},{6,3},
                {0,6},{3,6},{6,6}};

        for(int i=0;i<9;i++){
            //9行
            values.clear();
            for(int j=0;j<9;j++){
                char value = board[i][j];
                if(value == '.') continue;
                Integer iv = Character.getNumericValue(value);
                if(values.contains(iv)) return false;
                values.add(iv);
            }

            //9列
            values.clear();
            for(int j=0;j<9;j++){
                char value = board[j][i];
                if(value == '.') continue;
                Integer iv = Character.getNumericValue(value);
                if(values.contains(iv)) return false;
                values.add(iv);
            }

            //9个3X3
            values.clear();
            int[] base = bases[i];
            for(int j=0;j<=2;j++){
                for(int k=0;k<=2;k++){
                    int x = base[0] + j;
                    int y = base[1] + k;
                    char value = board[x][y];
                    //System.out.println("x,y="+x+","+y);
                    if(value == '.') continue;
                    Integer iv = Character.getNumericValue(value);
                    if(values.contains(iv)) return false;
                    values.add(iv);
                }
            }
        }

        return true;
    }

    public static void main(String[] args) {
        Solution36 solution36 = new Solution36();

        char[][] board = new char[][]{
                {'8', '3', '.', '.', '7', '.', '.', '.', '.'}
                , {'6', '.', '.', '1', '9', '5', '.', '.', '.'}
                , {'.', '9', '8', '.', '.', '.', '.', '6', '.'}
                , {'8', '.', '.', '.', '6', '.', '.', '.', '3'}
                , {'4', '.', '.', '8', '.', '3', '.', '.', '1'}
                , {'7', '.', '.', '.', '2', '.', '.', '.', '6'}
                , {'.', '6', '.', '.', '.', '.', '2', '8', '.'}
                , {'.', '.', '.', '4', '1', '9', '.', '.', '5'}
                , {'.', '.', '.', '.', '8', '.', '.', '7', '9'}};

        System.out.println(solution36.isValidSudoku(board));
        
        board = new char [][]{
                {'5','3','.','.','7','.','.','.','.'}
                ,{'6','.','.','1','9','5','.','.','.'}
                ,{'.','9','8','.','.','.','.','6','.'}
                ,{'8','.','.','.','6','.','.','.','3'}
                ,{'4','.','.','8','.','3','.','.','1'}
                ,{'7','.','.','.','2','.','.','.','6'}
                ,{'.','6','.','.','.','.','2','8','.'}
                ,{'.','.','.','4','1','9','.','.','5'}
                ,{'.','.','.','.','8','.','.','7','9'}};
        System.out.println(solution36.isValidSudoku(board));
    }
}
