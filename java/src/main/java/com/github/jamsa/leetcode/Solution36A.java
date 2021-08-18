package com.github.jamsa.leetcode;

import java.util.HashSet;
import java.util.Set;

/**
 * 36. 有效的数独
 * https://leetcode-cn.com/problems/valid-sudoku/
 *
 * 优化版
 * 只遍历一次，
 * 遍历时检查它所属的行、列、3x3区域中是否已经存在，需要有9行、9列、9个3x3存储，共270个存储位，记录是数据是否出现过
 *
 * @time : 2021/8/18 3:14 下午
 * @author: zhujie
 */
public class Solution36A {
    public boolean isValidSudoku(char[][] board) {
        //初始值为0，出现过的数字位置设置为1
        int rows[][] = new int[9][10]; //9行中已经出现的数字
        int cols[][] = new int[9][10]; //9列中已经出现的数字
        int boxes[][] = new int[9][10]; //9个3x3已经出现的数字

        for(int i=0;i<9;i++){ //row
            for(int j=0;j<9;j++){ //col
                char chr = board[i][j];
                if(chr =='.') continue;
                int value = chr - '0'; //直接将char映射为以0开始的整数
                if(rows[i][value]==1) return false;
                rows[i][value] = 1;
                if(cols[j][value]==1) return false;
                cols[j][value] = 1;

                //计算boxes索引位置
                int boxRow = i/3;
                int boxCol = j/3;
                int boxIndex = boxRow*3+boxCol;
                //System.out.println("(i,j),(box)=("+i+","+j+"),("+boxIndex+")");
                if(boxes[boxIndex][value]==1) return false;
                boxes[boxIndex][value] = 1;
            }
        }

        return true;
    }

    public static void main(String[] args) {
        Solution36A solution36 = new Solution36A();

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
