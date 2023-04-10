//! Implementations for linear algebra.
//!
//! `linal` provides two main structs: [`Vector`] and [`Matrix`]. These together
//! support a wide array of operations in finite-dimensional space and
//! form the basis of the linear algebra system for `hebrides`.

use std::ops::{Add, Sub, Mul, Div};

/// Implementation for a finite-dimensional vector over T.
#[derive(Debug)]
pub struct Vector<T> {
	components: Vec<T>,
	dim: usize
}

impl<T> Vector<T> {

	/// Constructs a [`Vector`] from a slice.
	pub fn new(components: Vec<T>) -> Vector<T> {
		let dim = components.len();
		Self { components, dim }
	}

	/// Returns whether or not `a` and `b` are of the same dimension.
	pub fn same_dim(a: &Self, b: &Self) -> bool {
		a.dim == b.dim
	}

}

impl<T> PartialEq for Vector<T> where T: PartialEq {
	fn eq(&self, other: &Self) -> bool {
		if !Vector::same_dim(self, other) {
			return false;
		}
		self.components.iter()
					   .zip(other.components.iter())
					   .all(|pair| *pair.0 == *pair.1)
	}
}

impl<T> Add<Self> for Vector<T> where T: Copy + Add<Output=T> {
	type Output = Self;
	fn add(mut self, other: Self) -> Self {
		if !Vector::same_dim(&self, &other) {
			panic!("Sums can only be taken between Vectors of the same dimension")
		}
		for (i, component) in other.components.iter().enumerate() {
			self.components[i] = self.components[i] + *component;
		}
		self
	}
}

impl<T> Sub<Self> for Vector<T> where T: Copy + Sub<Output=T> {
	type Output = Self;
	fn sub(mut self, other: Self) -> Self {
		if !Vector::same_dim(&self, &other) {
			panic!("Differences can only be taken between Vectors of the same dimension")
		}
		for (i, component) in other.components.iter().enumerate() {
			self.components[i] = self.components[i] - *component;
		}
		self
	}
}

/// Implements a dot product.
impl<T> Mul<Self> for Vector<T> where T: Copy + Mul<Output=T> + Add<Output=T> {
	type Output = T;
	fn mul(self, other: Self) -> T {
		if !Vector::same_dim(&self, &other) {
			panic!("Dot product can only be taken between Vectors of the same dimension")
		}
		self.components.iter()
					   .zip(other.components.iter())
					   .map(|pair| *pair.0 * *pair.1)
					   .reduce(|acc, component| acc + component)
					   .unwrap()
	}
}

impl<T> Div<T> for Vector<T> where T: Copy + Div<Output=T> {
	type Output = Self;
	fn div(mut self, other: T) -> Self {
		for i in 0..self.dim {
			self.components[i] = self.components[i] / other;
		}
		self
	}
}

/// TODO!
pub struct Matrix;


#[cfg(test)]
mod test {

	use super::*;

	mod vec {

		use super::*;

		mod addition {

			use super::*;

			#[test]
			fn left_identity() {
				let zero = Vector::new(vec![0, 0, 0]);
				let a = Vector::new(vec![1, 2, 3]);
				assert_eq!(zero + a, Vector::new(vec![1, 2, 3]))
			}

			#[test]
			fn right_identity() {
				let zero = Vector::new(vec![0, 0, 0]);
				let a = Vector::new(vec![1, 2, 3]);
				assert_eq!(a + zero, Vector::new(vec![1, 2, 3]))
			}

			#[test]
			fn standard() {
				let a = Vector::new(vec![1, 2, 3]);
				let b = Vector::new(vec![4, 5, 6]);
				assert_eq!(a + b, Vector::new(vec![5, 7, 9]))
			}

		}

		mod subtraction {

			use super::*;

			#[test]
			fn left_identity() {
				let zero = Vector::new(vec![0; 3]);
				let a = Vector::new(vec![1, 2, 3]);
				assert_eq!(zero - a, Vector::new(vec![-1, -2, -3]))
			}

			#[test]
			fn right_identity() {
				let zero = Vector::new(vec![0; 3]);
				let a = Vector::new(vec![1, 2, 3]);
				assert_eq!(a - zero, Vector::new(vec![1, 2, 3]))
			}

			#[test]
			fn standard() {
				let a = Vector::new(vec![1, 2, 3]);
				let b = Vector::new(vec![4, 5, 6]);
				assert_eq!(a - b, Vector::new(vec![-3, -3, -3]))
			}

		}

		mod dot_product {

			use super::*;

			#[test]
			fn left_identity() {
				let zero = Vector::new(vec![0; 3]);
				let a = Vector::new(vec![1, 2, 3]);
				assert_eq!(zero * a, 0)
			}

			#[test]
			fn right_identity() {
				let zero = Vector::new(vec![0; 3]);
				let a = Vector::new(vec![1, 2, 3]);
				assert_eq!(a * zero, 0)
			}

			#[test]
			fn square() {
				let a = Vector::new(vec![1, 2, 3]);
				let b = Vector::new(vec![1, 2, 3]);
				assert_eq!(a * b, 14)
			}

			#[test]
			fn orthogonal() {
				let k_hat = Vector::new(vec![0, 0, 1]);
				let j_hat = Vector::new(vec![0, 1, 0]);
				assert_eq!(k_hat * j_hat, 0)
			}

			#[test]
			fn standard() {
				let a = Vector::new(vec![1, 2, 3]);
				let b = Vector::new(vec![4, 5, 6]);
				assert_eq!(a * b, 32)
			}

		}

		mod division {

			use super::*;

			#[test]
			#[should_panic]
			fn divide_by_zero_error() {
				let a = Vector::new(vec![1, 2, 3]);
				let _ = a / 0;
			}

			#[test]
			fn identity() {
				let a = Vector::new(vec![1, 2, 3]);
				assert_eq!(a / 1, Vector::new(vec![1, 2, 3]))
			}

			#[test]
			fn standard() {
				let a = Vector::new(vec![2, 4, 6]);
				assert_eq!(a / 2, Vector::new(vec![1, 2, 3]))
			}

		}

	}

	

}
