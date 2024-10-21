use crate::vector::{dot_product, Vector};
use anyhow::{anyhow, Result};
use std::{
    fmt::{self, Debug, Display, Formatter},
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
    thread,
};

const NUM_THREADS: usize = 4;

pub struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, rows: usize, cols: usize) -> Self {
        Self {
            data: data.into(),
            rows,
            cols,
        }
    }
}

impl<T> Display for Matrix<T>
where
    T: Display,
{
    // display a 2*3 matrix as {1 2 3,4 5 6}, 3*2 as {1 2,3 4,5 6}
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{}", self.data[i * self.cols + j])?;
                if j != self.cols - 1 {
                    write!(f, " ")?;
                }
            }
            if i != self.rows - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")
    }
}

impl<T> Debug for Matrix<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Matrix(rows: {}, cols: {}, {})",
            self.rows, self.cols, self
        )
    }
}

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}

pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}

impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Copy + Default + Mul<Output = T> + Add<Output = T> + AddAssign + Send + 'static,
{
    if a.cols != b.rows {
        return Err(anyhow!("Matrix dimensions do not match"));
    }

    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.input.idx,
                        value,
                    }) {
                        eprintln!("Error sending message: {}", e);
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();

    // [1,2,3,4,5,6] [10,11,20,21,30,31]
    let mut data = vec![T::default(); a.rows * b.cols];
    let mut receivers = Vec::with_capacity(a.rows * b.cols);
    for i in 0..a.rows {
        for j in 0..b.cols {
            // let a_row = Vector::new(a.data[i * a.cols..(i + 1) * a.cols].to_vec());
            // let b_col = Vector::new((0..b.rows).map(|k| b.data[k * b.cols + j]).collect());
            // data[i * b.cols + j] = dot_product(a_row, b_col)?;

            let a_row = Vector::new(a.data[i * a.cols..(i + 1) * a.cols].to_vec());
            let b_col = Vector::new((0..b.rows).map(|k| b.data[k * b.cols + j]).collect());
            let idx = i * b.cols + j;
            let input = MsgInput::new(idx, a_row, b_col);
            let (tx, rx) = oneshot::channel();
            let msg = Msg::new(input, tx);
            if let Err(e) = senders[idx % NUM_THREADS].send(msg) {
                eprintln!("Error sending message: {}", e);
            }
            receivers.push(rx);
        }
    }

    for rx in receivers {
        let ret = rx.recv()?;
        data[ret.idx] = ret.value;
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
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);
        let c = multiply(&a, &b)?;
        assert_eq!(c.rows, 2);
        assert_eq!(c.cols, 2);
        assert_eq!(c.data, vec![22, 28, 49, 64]);
        assert_eq!(
            format!("{:?}", c),
            "Matrix(rows: 2, cols: 2, {22 28, 49 64})"
        );
        assert_eq!(format!("{}", c), "{22 28, 49 64}");
        Ok(())
    }

    #[test]
    fn test_display() -> Result<()> {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);
        assert_eq!(format!("{}", a), "{1 2, 3 4, 5 6}");
        Ok(())
    }

    #[test]
    fn test_debug() -> Result<()> {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);
        assert_eq!(
            format!("{:?}", a),
            "Matrix(rows: 3, cols: 2, {1 2, 3 4, 5 6})"
        );
        Ok(())
    }
}
