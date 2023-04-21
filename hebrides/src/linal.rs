//! Implementations for linear algebra.
//!
//! `linal` provides two main structs: [`Vector`] and [`Matrix`]. These together
//! support a wide array of operations in finite-dimensional space and
//! form the basis of the linear algebra system for `hebrides`.

use std::ops::{Add, Sub, Mul, Div, Neg, Index};

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
		if dim == 0 {
			panic!("Must provide at least one component to Vector")
		}
		Self { components, dim }
	}

	/// Returns whether or not `a` and `b` are of the same dimension.
	pub fn same_dim(a: &Self, b: &Self) -> bool {
		a.dim == b.dim
	}

}

impl<T> Vector<T> where T: Copy + Mul<Output=T> + Add<Output=T> + Default {

	/// Implements a dot product.
	///
	/// This method differs from the implementation offered by Mul<Vector<T>> in that
	/// it provides for both arguments being borrowed so that ownership of the
	/// [`Vector`]s involved does not have to be given up to get their product.
	pub fn dot(&self, other: &Self) -> T {
		if self.dim != other.dim {
			panic!("Vectors must be of the same dimension to have their dot product taken")
		}
		let mut sum = self.components[0] * other.components[0];
		for (i, (a, b)) in self.components.iter().zip(other.components.iter()).enumerate() {
			if i == 0 {
				continue;
			}
			sum = sum + *a * *b;
		}
		sum
	}

}

impl<T> Vector<T> where T: Copy + Mul<Output=T> + Sub<Output=T> {

	/// Returns the cross product of `self` with `other`.
	pub fn cross(&self, other: &Self) -> Self {
		if self.dim != other.dim {
			panic!("Vectors must be of the same dimension to be crossed")
		}
		if self.dim != 3 {
			panic!("Vectors must be three-dimensional to have valid cross products")
		}
		let x = self.components[1]*other.components[2] - self.components[2]*other.components[1];
		let y = self.components[2]*other.components[0] - self.components[0]*other.components[2];
		let z = self.components[0]*other.components[1] - self.components[1]*other.components[0];
		Vector::new(vec![x, y, z])
	}

}

impl<T> Vector<T> where T: Copy + Add<Output=T> + std::iter::Sum<T> {

	/// Returns the squared norm of `self`.
	pub fn square_norm(&self) -> T {
		self.components.iter().map(|e| *e).sum::<T>()
	}

}

impl Vector<f32> {

	/// Returns the norm of `self`.
	pub fn norm(&self) -> f32 {
		self.square_norm().powf(0.5)
	}

	/// Normalized version of `self`.
	pub fn normalized(&self) -> Self {
		let norm = self.norm();
		Vector::new(self.components.iter().map(|e| *e / norm).collect())
	}

}

impl Vector<f64> {

	/// Returns the norm of `self`.
	pub fn norm(&self) -> f64 {
		self.square_norm().powf(0.5)
	}

	/// Normalized version of `self`.
	pub fn normalized(&self) -> Self {
		let norm = self.norm();
		Vector::new(self.components.iter().map(|e| *e / norm).collect())
	}

}

impl<T> Clone for Vector<T> where T: Clone {
	fn clone(&self) -> Vector<T> {
		Vector::new(self.components.to_vec())
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

impl<T> Index<usize> for Vector<T> {
	type Output = T;
	fn index(&self, index: usize) -> &T {
		&self.components[index]
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
		if self.dim != other.dim {
			panic!("Vectors must be of the same dimension to have their dot product taken")
		}
		let mut sum = self.components[0] * other.components[0];
		for (i, (a, b)) in self.components.iter().zip(other.components.iter()).enumerate() {
			if i == 0 {
				continue;
			}
			sum = sum + *a * *b;
		}
		sum
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

impl<T> Neg for Vector<T> where T: Copy + Neg<Output=T> {
	type Output = Self;
	fn neg(self) -> Self {
		let mut new_components = Vec::with_capacity(self.dim);
		for e in self.components {
			new_components.push(-e);
		}
		Vector::new(new_components)
	}
}

/// Helper struct carrying the dimensions of a [`Matrix`].
#[derive(Copy, Clone)]
struct MatrixDimensions {
	pub num_rows: usize,
	pub num_cols: usize
}

impl MatrixDimensions {

	/// Constructs a new [`MatrixDimensions`] from a pair of usizes.
	pub fn new(num_rows: usize, num_cols: usize) -> MatrixDimensions {
		Self { num_rows, num_cols }
	}

	/// Returns whether or not `a` and `b` have a product defined by the standard
	/// methods of matrix multiplication (ie. whether or not a * b exists).
	pub fn are_compatible(a: MatrixDimensions, b: MatrixDimensions) -> bool {
		a.num_cols == b.num_rows
	}

}

/// Implementation for a finite-dimensional matrix over T.
pub struct Matrix<T> {
	pub rows: Vec<Vec<T>>,
	pub cols: Vec<Vec<T>>,
	pub dims: MatrixDimensions
}

impl<T> Matrix<T> {

	/// Returns whether or not all containers within the provided [`Vec`] have
	/// equal length.
	fn have_equal_length(rows: &Vec<Vec<T>>) -> bool {
		rows.iter().all(|row| row.len() == rows[0].len())
	}

}

impl<T> Matrix<T> where T: Copy {

	/// Takes a [`Vec`] describing a matrix in rows and returns a [`Vec`]
	/// describing the same matrix in terms of columns.
	fn to_columns(rows: &Vec<Vec<T>>) -> Vec<Vec<T>> {
		let mut cols: Vec<Vec<T>> = vec![];
		for i in 0..rows[0].len() {
			let mut col: Vec<T> = vec![];
			for j in 0..rows.len() {
				col.push(rows[j][i])
			}
			cols.push(col);
		}
		cols
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
		let cols = Matrix::to_columns(&rows);
		let dims = MatrixDimensions::new(rows.len(), rows[0].len());
		Self { rows, cols, dims }
	}

}

/// Matrix type specifying a Matrix of [`f32`]s.
pub type Matrix32 = Matrix<f32>;

impl Matrix32{

	/// Creates a homogenous transformation matrix that will cause a vector to point at 
	/// `dir`, using `up` for orientation.
	pub fn look_at_rh(eye: &Vector<f32>, dir: &Vector<f32>, up: &Vector<f32>) -> Matrix32 {
		let f = dir.normalized();
		let s = f.cross(&up).normalized();
		let u = s.cross(&f);

		Matrix::new(vec![
			vec![s[0], u[0], -f[0], 0.0],
			vec![s[1], u[1], -f[1], 0.0],
			vec![s[2], u[2], -f[2], 0.0],
			vec![-eye.dot(&s), -eye.dot(&u), eye.dot(&f), 0.0]
		])
	}

	/// Creates a perspective matrix from the given parameters.
	pub fn perspective(fovy: &f32, aspect: &f32, znear: &f32, zfar: &f32) -> Matrix32 {
		let f = 1.0 / (fovy / 2.0).tan();
		Matrix::new(vec![
			vec![f / aspect, 0.0, 0.0, 0.0],
			vec![0.0, f, 0.0, 0.0],
			vec![0.0, 0.0, (zfar + znear) / (znear - zfar), 2.0*zfar*znear / (znear - zfar)],
			vec![0.0, 0.0, -1.0, 0.0]
		])
	}

}

impl<T> std::fmt::Debug for Matrix<T> where T: std::fmt::Display {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		let mut repr = "[".to_owned();
		for (i, row) in self.rows.iter().enumerate() {
			let mut row_repr = "[".to_owned();
			for (j, component) in row.iter().enumerate() {
				if j == row.len() - 1 {
					row_repr.push_str(&format!("{}", component));
					continue;
				}
				row_repr.push_str(&format!("{}, ", component));
			}
			if i == self.rows.len() - 1 {
				row_repr.push_str("]");
				repr.push_str(&row_repr);
				continue;
			}
			row_repr.push_str("], ");
			repr.push_str(&row_repr);
		}
		write!(f, "{}]", repr)
	}
}

impl<T> PartialEq for Matrix<T> where T: PartialEq {
	fn eq(&self, other: &Self) -> bool {
		self.rows.iter()
				 .zip(other.rows.iter())
				 .map(|pair| *pair.0 == *pair.1)
				 .all(|e| e)
	}
}

impl<T> Index<usize> for Matrix<T> where T: Clone {
	type Output = Vec<T>;
	fn index(&self, index: usize) -> &Vec<T> {
		&self.rows[index]
	}
} 


impl<T> Add<Self> for Matrix<T> where T: Copy + Add<Output=T> {
	type Output = Self;
	fn add(mut self, other: Self) -> Self {
		for (i, row) in other.rows.iter().enumerate() {
			for (j, component) in row.iter().enumerate() {
				self.rows[i][j] = self.rows[i][j] + *component;
			}
		}
		self
	}
}

impl<T> Sub<Self> for Matrix<T> where T: Copy + Sub<Output=T> {
	type Output = Self;
	fn sub(mut self, other: Self) -> Self {
		for (i, row) in other.rows.iter().enumerate() {
			for (j, component) in row.iter().enumerate() {
				self.rows[i][j] = self.rows[i][j] - *component;
			}
		}
		self
	}
}

fn into_chunks<T>(v: Vec<T>, n: usize) -> Vec<Vec<T>> where T: Copy {
	if v.len() % n != 0 {
		panic!("Provided Vec cannot be evenly distributed into chunks of size {}", n)
	}
	let mut counter: usize = 0;
	let mut chunks: Vec<Vec<T>> = Vec::with_capacity(v.len());
	let mut cur_chunk: Vec<T> = Vec::with_capacity(n);
	for item in v {
		cur_chunk.push(item);
		counter += 1;
		if counter == n {
			counter = 0;
			chunks.push(cur_chunk.to_vec());
			cur_chunk.clear();
			continue;
		}
		
	}
	chunks
}

impl<T> Mul<Self> for Matrix<T> where T: Copy + Default + Mul<Output=T> + Add<Output=T> {
	type Output = Self;
	fn mul(self, other: Self) -> Self {
		if !MatrixDimensions::are_compatible(self.dims, other.dims) {
			panic!("Matrices must have compatible dimensions to be multiplied")
		}
		let mut product: Vec<T> = Vec::with_capacity(self.dims.num_rows*other.dims.num_cols);
		for _ in 0..self.dims.num_rows*other.dims.num_cols {
			product.push(T::default());
		}
		for i in 0..self.dims.num_rows {
			for j in 0..other.dims.num_cols {
				for k in 0..other.dims.num_rows {
					let index = j + other.dims.num_cols*i;
					product[index] = product[index] + self.rows[i][k] * other.rows[k][j];
				}
			}
		}
		Matrix::new(into_chunks(product, other.dims.num_cols))
	}
}

impl<T> Mul<Vector<T>> for Matrix<T> where T: Copy + Default + Mul<Output=T> + Add<Output=T> {
	type Output = Vector<T>;
	fn mul(self, other: Vector<T>) -> Vector<T> {
		if self.dims.num_cols != other.dim {
			panic!("Vector must be compatible with matrix to be transformed by it")
		}
		let mut result_components = Vec::with_capacity(self.dims.num_rows);
		for row in self.rows {
			let mut sum = row[0] * other[0];
			for (i, (a, b)) in row.iter().zip(other.components.iter()).enumerate() {
				if i == 0 {
					continue;
				}
				sum = sum + *a * *b
			}
			result_components.push(sum);
		}
		Vector::new(result_components)
	}
}

impl<T> Mul<T> for Matrix<T> where T: Copy + Mul<Output=T> {
	type Output = Self;
	fn mul(self, other: T) -> Self {
		let new_rows: Vec<Vec<T>> = self.rows.iter()
											 .map(|row| row.iter().map(|e| other**e).collect())
											 .collect();
		Matrix::new(new_rows)
	}
}

impl<T> Div<T> for Matrix<T> where T: Copy + Div<Output=T> {
	type Output = Self;
	fn div(self, other: T) -> Self {
		let new_rows: Vec<Vec<T>> = self.rows.iter()
											 .map(|row| row.iter().map(|e| *e / other).collect())
											 .collect();
		Matrix::new(new_rows)
	}
}

#[cfg(test)]
mod test {

	use super::*;

	mod vector {

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

	mod matrix {

		use super::*;

		#[test]
		fn rows_to_columns_works() {
			let m = Matrix::new(vec![
				vec![1, 2],
				vec![3, 4]
			]);
			let cols = vec![
				vec![1, 3],
				vec![2, 4]
			];
			assert_eq!(cols, m.cols)
		}

		#[test]
		fn equality_holds() {
			let p = Matrix::new(vec![
				vec![1, 2],
				vec![3, 4]
			]);
			let q = Matrix::new(vec![
				vec![1, 2],
				vec![3, 4]
			]);
			assert!(p == q)
		}

		#[test]
		fn inequality_holds() {
			let p = Matrix::new(vec![
				vec![-1, 2],
				vec![3, 4]
			]);
			let q = Matrix::new(vec![
				vec![1, 2],
				vec![3, 4]
			]);
			assert!(p != q)
		}

		mod addition {

			use super::*;

			#[test]
			fn left_identity() {
				let zero = Matrix::new(vec![
					vec![0, 0],
					vec![0, 0]
				]);
				let m = Matrix::new(vec![
					vec![1, 2],
					vec![3, 4]
				]);
				let expected_sum = Matrix::new(vec![
					vec![1, 2],
					vec![3, 4]
				]);
				assert_eq!(zero + m, expected_sum)
			}

			#[test]
			fn right_identity() {
				let zero = Matrix::new(vec![
					vec![0, 0],
					vec![0, 0]
				]);
				let m = Matrix::new(vec![
					vec![1, 2],
					vec![3, 4]
				]);
				let expected_sum = Matrix::new(vec![
					vec![1, 2],
					vec![3, 4]
				]);
				assert_eq!(m + zero, expected_sum)
			}

			#[test]
			fn standard() {
				let p = Matrix::new(vec![
					vec![1, 2],
					vec![3, 4]
				]);
				let q = Matrix::new(vec![
					vec![1, 2],
					vec![3, 4]
				]);
				let expected_sum = Matrix::new(vec![
					vec![2, 4],
					vec![6, 8]
				]);
				assert_eq!(p + q, expected_sum)
			}

		}

		mod subtraction {

			use super::*;

			#[test]
			fn left_identity() {
				let zero = Matrix::new(vec![
					vec![0, 0],
					vec![0, 0]
				]);
				let m = Matrix::new(vec![
					vec![1, 2],
					vec![3, 4]
				]);
				let expected_difference = Matrix::new(vec![
					vec![-1, -2],
					vec![-3, -4]
				]);
				assert_eq!(zero - m, expected_difference)
			}

			#[test]
			fn right_identity() {
				let zero = Matrix::new(vec![
					vec![0, 0],
					vec![0, 0]
				]);
				let m = Matrix::new(vec![
					vec![1, 2],
					vec![3, 4]
				]);
				let expected_difference = Matrix::new(vec![
					vec![1, 2],
					vec![3, 4]
				]);
				assert_eq!(m - zero, expected_difference)
			}

			#[test]
			fn standard() {
				let p = Matrix::new(vec![
					vec![1, 2],
					vec![3, 4]
				]);
				let q = Matrix::new(vec![
					vec![1, 2],
					vec![3, 4]
				]);
				let expected_difference = Matrix::new(vec![
					vec![0, 0],
					vec![0, 0]
				]);
				assert_eq!(p - q, expected_difference)
			}

		}

		mod matrix_multiplication {

			use super::*;

			#[test]
			fn left_identity() {
				let one = Matrix::new(vec![
					vec![1, 0],
					vec![0, 1]
				]);
				let a = Matrix::new(vec![
					vec![1, 2],
					vec![3, 4]
				]);
				let expected_product = Matrix::new(vec![
					vec![1, 2],
					vec![3, 4]
				]);
				assert_eq!(one * a, expected_product)
			}

			#[test]
			fn right_identity() {
				let one = Matrix::new(vec![
					vec![1, 0],
					vec![0, 1]
				]);
				let a = Matrix::new(vec![
					vec![1, 2],
					vec![3, 4]
				]);
				let expected_product = Matrix::new(vec![
					vec![1, 2],
					vec![3, 4]
				]);
				assert_eq!(a * one, expected_product)
			}

			#[test]
			fn standard() {
				let a = Matrix::new(vec![
					vec![1, 2, 3],
					vec![4, 5, 6]
				]);
				let b = Matrix::new(vec![
					vec![10, 11],
					vec![20, 21],
					vec![30, 31]
				]);
				let expected_product = Matrix::new(vec![
					vec![140, 146],
					vec![320, 335]
				]);
				assert_eq!(a * b, expected_product)
			}

		}

		mod vector_multiplication {

			use super::*;

			#[test]
			fn identity() {
				let m = Matrix::new(vec![
					vec![1, 0],
					vec![0, 1]
				]);
				let v = Vector::new(vec![1, 2]);
				let expected_product = Vector::new(vec![1, 2]);
				assert_eq!(m * v, expected_product)
			}

			#[test]
			fn standard() {
				let m = Matrix::new(vec![
					vec![1, 2],
					vec![3, 4]
				]);
				let v = Vector::new(vec![1, 2]);
				let expected_product = Vector::new(vec![5, 11]);
				assert_eq!(m * v, expected_product)
			}

		}

		mod scalar_multiplication {

			use super::*;

			#[test]
			fn right_identity() {
				let a = Matrix::new(vec![
					vec![1, 2, 3],
					vec![4, 5, 6]
				]);
				let expected_product = Matrix::new(vec![
					vec![1, 2, 3],
					vec![4, 5, 6]
				]);
				assert_eq!(a * 1, expected_product)
			}

			#[test]
			fn right_standard() {
				let a = Matrix::new(vec![
					vec![1, 2, 3],
					vec![4, 5, 6]
				]);
				let expected_product = Matrix::new(vec![
					vec![4, 8, 12],
					vec![16, 20, 24]
				]);
				assert_eq!(a * 4, expected_product)
			}

		}

		mod scalar_division {

			use super::*;

			#[test]
			fn right_identity() {
				let a = Matrix::new(vec![
					vec![1, 2, 3],
					vec![4, 5, 6]
				]);
				let expected_product = Matrix::new(vec![
					vec![1, 2, 3],
					vec![4, 5, 6]
				]);
				assert_eq!(a / 1, expected_product)
			}

			#[test]
			fn right_standard() {
				let a = Matrix::new(vec![
					vec![4, 8, 12],
					vec![16, 20, 24]
				]);
				let expected_dividend = Matrix::new(vec![
					vec![1, 2, 3],
					vec![4, 5, 6]
				]);
				assert_eq!(a / 4, expected_dividend)
			}

		}

	}
	

}
