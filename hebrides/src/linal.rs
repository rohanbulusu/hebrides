//! Implementations for linear algebra.
//!
//! `linal` provides two main structs: Vector and Matrix. These together
//! support a wide array of operations in finite-dimensional space and
//! form the basis of the linear algebra system for `hebrides`.

use std::ops::{Add, Sub, Mul, Div};

/// Representation of finite-dimensional vectors
#[derive(Debug)]
pub struct Vector<T> {
	components: Vec<T>,
	dim: usize
}

impl<T> Vector<T> where T: Clone {

	/// Constructs a `Vector` from a slice
	pub fn new(components: &[T]) -> Vector<T> {
		Vector {
			components: components.to_vec(),
			dim: components.len()
		}
	}

}

impl<T> PartialEq for Vector<T> where T: PartialEq {
	fn eq(&self, other: &Vector<T>) -> bool {
		let pairs = self.components.iter().zip(other.components.iter());
		for pair in pairs {
			if pair.0 != pair.1 {
				return false;
			}
		}
		return true;
	}
}

impl<T> Add<Self> for Vector<T> where T: Copy + Add<Output=T> {
	type Output = Self;
	fn add(self, other: Self) -> Self {
		if self.dim != other.dim {
			panic!("Vectors must be of the same dimension to be summed")
		}
		let mut sum = Vec::with_capacity(self.dim);
		let pairs = self.components.iter().zip(other.components.iter());
		for pair in pairs {
			sum.push(*pair.0 + *pair.1);
		}
		Vector::new(sum.as_slice())
	}
}

impl<T> Sub<Self> for Vector<T> where T: Copy + Sub<Output=T> {
	type Output = Self;
	fn sub(self, other: Self) -> Self {
		if self.dim != other.dim {
			panic!("Vectors must be of the same dimension to be subtracted")
		}
		let mut diff = Vec::with_capacity(self.dim);
		let pairs = self.components.iter().zip(other.components.iter());
		for pair in pairs {
			diff.push(*pair.0 - *pair.1);
		}
		Vector::new(diff.as_slice())
	}
}

/// Implements a dot product between Vectors
impl<T> Mul<Self> for Vector<T> where T: Copy + Mul<Output=T> + std::iter::Sum {
	type Output = T;
	fn mul(self, other: Self) -> T {
		if self.dim != other.dim {
			panic!("Dot products can only be taken between vectors of the same dimension")
		}
		let pairs = self.components.iter().zip(other.components.iter());
		pairs.map(|pair| *pair.0 * *pair.1).sum()
	}
}

impl<T> Div<T> for Vector<T> where T: Copy + Div<Output=T> {
	type Output = Self;
	fn div(self, other: T) -> Self {
		let mut dividend = Vec::with_capacity(self.dim);
		for component in self.components {
			dividend.push(component / other);
		}
		Vector::new(dividend.as_slice())
	}
}

fn all_rows_have_equal_length<'a, T>(array: &'a[&'a[T]]) -> bool {
	let first_row_length = array[0].len();
	for row in array {
		if row.len() != first_row_length {
			return false;
		}
	}
	true
}


/// Container for the dimensions of a `Matrix`
struct MatrixDimensions {
	rows: usize,
	cols: usize
}

impl MatrixDimensions {

	fn new(num_rows: usize, num_cols: usize) -> MatrixDimensions {
		Self {
			rows: num_rows,
			cols: num_cols
		}
	}

}

/// Representation of finite-dimensional matrices
pub struct Matrix<'a, T> {
	rows: &'a[&'a[T]],
	dims: MatrixDimensions
}

impl<'a, T> Matrix<'a, T> where T: Clone {

	/// Constructs a `Matrix` from nested slices
	pub fn new(components: &'a [&'a [T]]) -> Matrix<'a, T> {
		if !all_rows_have_equal_length(components) {
			panic!("Expected all rows of Matrix to have equal length")
		}
		Self {
			rows: components,
			dims: MatrixDimensions::new(components.len(), components[0].len())		}
	}

}




#[cfg(test)]
mod test {

	use super::*;

	mod vector {

		use super::*;

		mod equality {

			use super::*;

			#[test]
			fn equality() {
				let a = Vector::new(&[1, 2, 3]);
				let b = Vector::new(&[1, 2, 3]);
				assert_eq!(a, b)
			}

			#[test]
			fn normal_inequality() {
				let a = Vector::new(&[1, 2, 3]);
				let b = Vector::new(&[4, 5, 6]);
				assert_ne!(a, b)
			}

			#[test]
			fn mixed_inequality() {
				let a = Vector::new(&[1, 2, 3]);
				let b = Vector::new(&[3, 2, 1]);
				assert_ne!(a, b)
			}

		}

		mod dimension {

			use super::*;

			#[test]
			fn equal_dimension() {
				let a = Vector::new(&[1, 2, 3]);
				let b = Vector::new(&[4, 5, 6]);
				assert_eq!(a.dim, b.dim)
			}

			#[test]
			fn differing_dimension() {
				let a = Vector::new(&[1, 2, 3]);
				let b = Vector::new(&[1, 2]);
				assert_ne!(a.dim, b.dim)
			}

		}

		mod addition {

			use super::*;

			#[test]
			fn normal() {
				let a = Vector::new(&[1, 2, 3]);
				let b = Vector::new(&[4, 5, 6]);
				assert_eq!(a + b, Vector::new(&[5, 7, 9]))
			}

			#[test]
			#[should_panic]
			fn differing_dims() {
				let a = Vector::new(&[1]);
				let b = Vector::new(&[1, 2]);
				let _ = a + b;
			}

		}

		mod subtraction {

			use super::*;

			#[test]
			fn normal() {
				let a = Vector::new(&[1, 2, 3]);
				let b = Vector::new(&[3, 4, 5]);
				assert_eq!(a - b, Vector::new(&[-2, -2, -2]))
			}

			#[test]
			#[should_panic]
			fn differing_dims() {
				let a = Vector::new(&[1]);
				let b = Vector::new(&[1, 2]);
				let _ = a - b;
			}

		}

		mod dot_product {

			use super::*;

			#[test]
			fn normal() {
				let a = Vector::new(&[1, 2, 3]);
				let b = Vector::new(&[1, 2, 3]);
				assert_eq!(a * b, 14)
			}

			#[test]
			fn orthogonal() {
				let a = Vector::new(&[1, 0]);
				let b = Vector::new(&[0, 1]);
				assert_eq!(a * b, 0);
			}

			#[test]
			#[should_panic]
			fn differing_dims() {
				let a = Vector::new(&[1]);
				let b = Vector::new(&[1, 2]);
				let _ = a * b;
			}

		}

		mod division {

			use super::*;

			#[test]
			fn normal() {
				let a = Vector::new(&[2, 4, 6]);
				assert_eq!(a / 2, Vector::new(&[1, 2, 3]))
			}

			#[test]
			#[should_panic]
			fn zero_divisor() {
				let b = Vector::new(&[2, 3, 4]);
				let _ = b / 0;
			}

		}

	}

	mod matrix {

		use super::*;

		mod dimensionality {

			use super::*;

			#[test]
			#[should_panic]
			fn incompatible_array() {
				let _ = Matrix::new(&[
					&[1, 2, 3],
					&[1, 2]
				]);
			}

			#[test]
			fn num_rows() {
				let m = Matrix::new(&[
					&[1, 2, 3],
					&[1, 2, 3]
				]);
				assert_eq!(m.dims.rows, 2)
			}

			#[test]
			fn num_cols() {
				let m = Matrix::new(&[
					&[1, 2, 3],
					&[1, 2, 3]
				]);
				assert_eq!(m.dims.cols, 3)
			}

		}

	}

}



