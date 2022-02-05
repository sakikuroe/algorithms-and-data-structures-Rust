use std::ops;

#[derive(Clone, Debug)]
pub struct Matrix {
    row_size: usize,
    col_size: usize,
    data: Vec<Vec<usize>>,
}

impl Matrix {
    pub fn new(data: Vec<Vec<usize>>) -> Matrix {
        Matrix {
            row_size: data.len(),
            col_size: data[0].len(),
            data,
        }
    }
    pub fn mat_pow(&self, n: usize) -> Result<Matrix, &str> {
        if self.row_size != self.col_size {
            return Err(
                "Because it is not a square matrix, matrix power cannot be defined.",
            );
        }
        if n == 0 {
            Ok(Matrix {
                row_size: self.row_size,
                col_size: self.col_size,
                data: {
                    let mut res = vec![vec![0; self.col_size]; self.row_size];
                    for i in 0..self.row_size {
                        res[i][i] = 1;
                    }
                    res
                },
            })
        } else if n % 2 == 0 {
            Ok((self.clone() * self.clone()).unwrap().mat_pow(n / 2).unwrap())
        } else {
            Ok((self.clone() * self.clone().mat_pow(n - 1).unwrap()).unwrap())
        }
    }
}

impl ops::Mul for Matrix {
    type Output = Result<Matrix, &'static str>;

    fn mul(self, other: Self) -> Result<Matrix, &'static str> {
        if self.col_size != other.row_size {
            return Err("Because the number of columns in the matrix on the left and the number of row_size in the matrix on the right are different, 
                        the product of the matrices cannot be calculated.");
        }

        let mut data = vec![vec![0; other.col_size]; self.row_size];

        for i in 0..self.row_size {
            for j in 0..other.col_size {
                for k in 0..self.row_size {
                    data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        Ok(Self {
            row_size: self.row_size,
            col_size: other.col_size,
            data,
        })
    }
}