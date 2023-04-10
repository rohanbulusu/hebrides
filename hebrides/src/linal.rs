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

/// Helper struct carrying the dimensions of a [`Matrix`].
struct MatrixDimensions {
	num_rows: usize,
	num_cols: usize
}

impl MatrixDimensions {

	/// Constructs a new [`MatrixDimensions`] from a pair of usizes.
	pub fn new(num_rows: usize, num_cols: usize) -> MatrixDimensions {
		Self { num_rows, num_cols }
	}

}

/// Implementation for a finite-dimensional matrix over T.
pub struct Matrix<T> {
	rows: Vec<Vec<T>>,
	dims: MatrixDimensions
}

impl<T> Matrix<T> {

	/// Returns whether or not all containers within the provided [`Vec`] have
	/// equal length.
	fn have_equal_length(rows: &Vec<Vec<T>>) -> bool {
		rows.iter().all(|row| row.len() == rows[0].len())
	}

	/// Constructs a new [`Matrix`] from a nested [`Vec`].
	///
	/// # Panics
	/// This constructor panics in two scenarios. If the provided `Vec` has no 
	/// elements, as depicted below,
	///
	/// ```should_panic
	/// # use hebrides::linal::Matrix;
	/// let _: Matrix<i32> = Matrix::new(vec![]); // panics!
	/// ```
	///
	/// then a Matrix cannot be constructed and a panic issues. If the provided
	/// `Vec` has rows of differing length, then it is similarly impossible to 
	/// determine the appropriate dimension of the Matrix desired and there is a
	/// panic.
	/// 
	/// ```should_panic
	/// # use hebrides::linal::Matrix;
	/// let _ = Matrix::new(vec![vec![1, 0], vec![0]]); // panics!
	/// ```
	pub fn new(rows: Vec<Vec<T>>) -> Matrix<T> {
		if rows.len() == 0 {
			panic!("Matrix must be non-empty")
		}
		if !Matrix::have_equal_length(&rows) {
			panic!("All rows of a Matrix must have equal length")
		}
		let dims = MatrixDimensions::new(rows.len(), rows[0].len());
		Self { rows, dims }
	}

}

impl<T> Add<Self> for Matrix<T> {
	type Output = Self;
	fn add(mut self, other: Self) -> Self {
		todo!()
	}
}

impl<T> Sub<Self> for Matrix<T> {
	type Output = Self;
	fn sub(mut self, other: Self) -> Self {
		todo!()
	}
}

impl<T> Mul<Self> for Matrix<T> {
	type Output = Self;
	fn mul(mut self, other: Self) -> Self {
		todo!()
	}
}

impl<T> Div<T> for Matrix<T> {
	type Output = Self;
	fn div(mut self, other: T) -> Self {
		todo!()
	}
}


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
