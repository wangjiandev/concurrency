use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul},
};

use anyhow::{Ok, Result};

#[derive(Debug)]
pub struct Matrix<T: Debug> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Debug> Matrix<T> {
    // data: impl Into<Vec<T>> 代表的意思：任何数据结构，只要实现了Into<Vec<T>> trait，就可以作为参数传递给new函数
    pub fn new(data: impl Into<Vec<T>>, rows: usize, cols: usize) -> Self {
        Self {
            data: data.into(),
            rows,
            cols,
        }
    }
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Mul<Output = T> + Add<Output = T> + Debug + AddAssign + Copy,
{
    if a.cols != b.rows {
        return Err(anyhow::anyhow!("Invalid matrix dimensions"));
    }

    let mut data = Vec::with_capacity(a.rows * b.cols);
    for i in 0..a.rows {
        for j in 0..b.cols {
            let mut sum = a.data[i * a.cols] * b.data[j];
            for k in 1..a.cols {
                sum += a.data[i * a.cols + k] * b.data[k * b.cols + j];
            }
            data.push(sum);
        }
    }

    Ok(Matrix {
        data,
        rows: a.rows,
        cols: b.cols,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() -> Result<()> {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);
        let c = multiply(&a, &b)?;
        assert_eq!(c.data, vec![22, 28, 49, 64]);
        assert_eq!(c.cols, 2);
        assert_eq!(
            format!("{:?}", c),
            "Matrix { data: [22, 28, 49, 64], rows: 2, cols: 2 }"
        );
        Ok(())
    }
}
