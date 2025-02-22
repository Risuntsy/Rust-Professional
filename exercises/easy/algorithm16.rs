/*
    Rotate Matrix 90 Degrees
    Given a 2D matrix, rotate it 90 degrees in place. 
    You need to perform the rotation without using any additional matrix storage.

    You need to implement the function `rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>)`.
    The function should rotate the input matrix in place.

    Hint: Consider rotating the matrix layer by layer, starting from the outermost layer and working your way inward.
*/

use std::{fmt::{self, Display, Formatter}, vec};


pub fn print_matrix(matrix: &Vec<Vec<i32>>) {
    println!("Rotated matrix:");
    for row in matrix {
        for num in row {
            print!("{} ", num);
        }
        println!();
    }

}

pub fn rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>) {
    if matrix.len() == 0 || matrix[0].len() == 0 {
        return
    }

    let (n, m) = (matrix.len(), matrix[0].len());

    let min = n.min(m);
    let max = n.max(m);
    let diff = max - min;

    if n > m {
        for i in 0..diff {
            for j in 0..min {
                let elem = (&mut matrix[i]).pop().unwrap();
                matrix[max - j - 1].push(elem);
            }
        }

        for i in 0..min {
            matrix.swap(i, i + diff);
        }

        for  _ in 0..diff {
            matrix.pop();
        }
    } else if m > n {
        for _ in 0..diff {
            matrix.push(vec![]);

            for j in 0..min {
                let elem = matrix[min - j - 1].pop().unwrap();
                matrix.last_mut().unwrap().push(elem);
            }
        }

        for i in 0..diff / 2 {
            matrix.swap(min + i + 1, max - i);
        }
    }


    let (mut start, mut end ) = (0 , min -1);

    // print_matrix(matrix);

    while start < end {
        for i in start..end {
            let temp = matrix[start][i];
            let offset = i - start;

            matrix[start][i] = matrix[end -offset][start];
            matrix[end - offset][start] = matrix[end][end - offset];
            matrix[end][end - offset] = matrix[i][end];
            matrix[i][end] = temp;
        }

        start +=1;
        end -=1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix_1() {
        let mut matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        print_matrix(&matrix);
        assert_eq!(matrix, vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3],
        ]);
    }

    #[test]
    fn test_rotate_matrix_2() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        print_matrix(&matrix);
        assert_eq!(matrix, vec![
            vec![3, 1],
            vec![4, 2],
        ]);
    }

    #[test]
    fn test_rotate_matrix_3() {
        let mut matrix = vec![
            vec![1],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        print_matrix(&matrix);
        assert_eq!(matrix, vec![
            vec![1],
        ]);
    }

    #[test]
    fn test_rotate_matrix_4() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        print_matrix(&matrix);
        assert_eq!(matrix, vec![
            vec![5, 3, 1],
            vec![6, 4, 2],
        ]);
    }

    #[test]
    fn test_rotate_matrix_5() {
        let mut matrix = vec![
            vec![5, 3, 1],
            vec![6, 4, 2],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        print_matrix(&matrix);
        assert_eq!(matrix, vec![
            vec![6,5],
            vec![4,3],
            vec![2,1],
        ]);
    }
}
