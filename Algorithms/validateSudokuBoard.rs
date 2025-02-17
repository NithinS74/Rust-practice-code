#![allow(warnings)]
use std::collections::HashMap as hm;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    for i in (0..9).step_by(3) {
        for j in (0..9).step_by(3) {
            let mut record: Vec<char> = vec![];
            for k in (0 + i..3 + i) {
                for l in (0 + j..3 + j) {
                    if board[k][l] == '.' {
                        continue;
                    }
                    if record.contains(&board[k][l]) {
                        return false;
                    } else {
                        record.push(board[k][l]);
                    }
                }
                println!();
            }
        }
    }

    for i in 0..9 {
        let mut record: Vec<char> = vec![];
        for j in 0..9 {
            if board[i][j] == '.' {
                continue;
            }
            if record.contains(&board[i][j]) {
                return false;
            } else {
                record.push(board[i][j]);
            }
        }
        record.clear();
        for j in 0..9 {
            if board[j][i] == '.' {
                continue;
            }
            if record.contains(&board[j][i]) {
                return false;
            } else {
                record.push(board[j][i]);
            }
        }
    }
    return true;
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let out: bool = is_valid_sudoku(board);
        assert_eq!(out, true);
    }

    #[test]
    fn test_2() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let out: bool = is_valid_sudoku(board);
        assert_eq!(out, false);
    }

    #[test]
    fn test_3() {
        let board = vec![
            vec!['.', '.', '.', '.', '5', '.', '.', '1', '.'],
            vec!['.', '4', '.', '3', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '3', '.', '.', '1'],
            vec!['8', '.', '.', '.', '.', '.', '.', '2', '.'],
            vec!['.', '.', '2', '.', '7', '.', '.', '.', '.'],
            vec!['.', '1', '5', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '2', '.', '.', '.'],
            vec!['.', '2', '.', '9', '.', '.', '.', '.', '.'],
            vec!['.', '.', '4', '.', '.', '.', '.', '.', '.'],
        ];
        let out: bool = is_valid_sudoku(board);
        assert_eq!(out, false);
    }
}
