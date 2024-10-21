use anyhow::{anyhow, Result};
use std::{
    ops::{Add, AddAssign, Deref, Index, Mul},
    slice::Iter,
};

pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Copy + Default + Mul<Output = T> + Add<Output = T> + AddAssign,
{
    if a.len() != b.len() {
        return Err(anyhow!("Vector lengths do not match"));
    }
    let mut sum = T::default();
    for (a, b) in a.iter().zip(b.iter()) {
        sum += *a * *b;
    }
    Ok(sum)
}
