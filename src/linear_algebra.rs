//! This module contains implementation of all primitives related to linear algebra:
//! vectors, matrices, and R1CS.
//! NOTE: This is a dummy implementation and is not meant to be used in production.
//! Typically, you would use a library like arkworks-rs/algebra and such to work with linear algebra.

use crate::finite_field::Fp;

// First, we implement a vector of fixed length with an inner product operation.

/// Structure to represent a vector of fixed length with an inner product operation
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector<const N: usize>([Fp; N]);

impl<const N: usize> Vector<N> {
    /// Creates a new vector from a fixed-size slice
    pub fn new(value: [Fp; N]) -> Self {
        assert_eq!(value.len(), N, "Vector length mismatch");
        Vector(value)
    }

    /// Returns the ith element of the vector
    pub fn get(&self, i: usize) -> Fp {
        self.0[i]
    }

    /// Implements the dot product of two vectors
    pub fn dot(&self, other: &Self) -> Fp {
        let mut res = Fp::from(0);
        for i in 0..N {
            res += self.get(i) * other.get(i);
        }
        res
    }

    // Maybe it's incorrect
    pub fn hadamard_product(&self, other: &Self) -> Self {
        let mut res = Vector::new([Fp::from(0); N]);
        for i in 0..N {
            res.0[i] = self.get(i) * other.get(i);
        }
        res

        // TODO: Implement hadamard product here!
        // unimplemented!("Implement hadamard product!")
    }
}

// Next, we implement some matrix operations.

/// Structure, representing a matrix.
/// Here, we matrix consists of M row vectors of length N. This way,
/// essentially, Matrix.0[i][j] is the j-th element of the i-th row.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix<const N: usize, const M: usize>([Vector<N>; M]);

impl<const N: usize, const M: usize> Matrix<N, M> {
    /// Creates a new matrix with all elements set to zero
    pub fn zero() -> Self {
        Matrix::<N, M>::new([Vector::<N>::new([Fp::from(0); N]); M])
    }

    /// Creates a new matrix from a fixed-size slice of row vectors
    pub fn new(value: [Vector<N>; M]) -> Self {
        // Asserting that sizes are correct
        assert_eq!(value.len(), M, "Matrix length mismatch");
        assert_eq!(value[0].0.len(), N, "Matrix row length mismatch");

        Matrix(value)
    }

    /// Returns the ith row of the matrix
    pub fn row(&self, i: usize) -> &Vector<N> {
        &self.0[i]
    }

    /// Returns the jth column of the matrix
    pub fn column(&self, j: usize) -> Vector<M> {
        let mut resultant_column = Vector::<M>::new([Fp::from(0); M]);

        for i in 0..M {
            resultant_column.0[i] = self.0[i].0[j];
        }

        resultant_column
    }

    /// Returns the (i,j) element of the matrix
    pub fn get(&self, i: usize, j: usize) -> Fp {
        self.0[i].0[j]
    }

    /// Implements the hadamard product of two matrices. Namely, given two matrices `A` and `B`,
    /// consisting of elements `a_ij` and `b_ij` respectively, the hadamard product is a matrix `C`
    /// consisting of elements `c_ij = a_ij * b_ij`.
    pub fn hadamard_product(&self, other: &Self) -> Self {
        let mut res = Matrix::zero();
        for i in 0..M {
            res.0[i] = self.row(i).hadamard_product(other.row(i));
        }
        res
        // TODO: Implement hadamard product here!
        //unimplemented!("Implement hadamard product!")
    }

    /// Implements the matrix-vector product. Namely, given a matrix `A` and a vector `b`, the
    /// matrix-vector product gives `Ab`.
    ///
    /// **Hint:** this is a vector `c` such that `c_i = \sum_j a_ij * b_j`.
    pub fn vector_product(&self, other: &Vector<N>) -> Vector<M> {
        let mut res = Vector::new([Fp::from(0); M]);
        for i in 0..M {
            res.0[i] = self.row(i).dot(other);
        }
        res
    }
}

/// Below code simply tests the correctness of the your implementation. Do not touch
/// it unless you know what you are doing.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_dot() {
        // a = [1, 5, 5]
        let a = Vector::new([Fp::from(1), Fp::from(5), Fp::from(5)]);
        // b = [2, 3, 2]
        let b = Vector::new([Fp::from(2), Fp::from(3), Fp::from(2)]);

        // We expect to get 1*2+5*3+5*2 = 27
        assert_eq!(a.dot(&b), Fp::from(27));
    }

    #[test]
    fn test_matrix_vector_product() {
        // A = {{1, 5, 5}, {2, 3, 2}}
        let a = Matrix::new([
            Vector::new([Fp::from(1), Fp::from(5), Fp::from(5)]),
            Vector::new([Fp::from(2), Fp::from(3), Fp::from(2)]),
        ]);

        // b = [2, 3, 2]
        let b = Vector::new([Fp::from(2), Fp::from(3), Fp::from(2)]);

        // c = A*b. We expect to get
        // {1*2+5*3+5*2, 2*2+3*3+2*2} = {27, 17}
        let c = a.vector_product(&b);

        assert_eq!(c, Vector::new([Fp::from(27), Fp::from(17)]));
    }

    #[test]
    fn test_matrix_hadamard_product() {
        // A = {{1, 5, 5}, {2, 3, 2}}
        let a = Matrix::new([
            Vector::new([Fp::from(1), Fp::from(5), Fp::from(5)]),
            Vector::new([Fp::from(2), Fp::from(3), Fp::from(2)]),
        ]);

        // B = {{2, 3, 2}, {1, 5, 5}}
        let b = Matrix::new([
            Vector::new([Fp::from(2), Fp::from(3), Fp::from(2)]),
            Vector::new([Fp::from(1), Fp::from(5), Fp::from(5)]),
        ]);

        // C = A hadamard B. We expect to get
        // {{1*2, 5*3, 5*2}, {2*1, 3*5, 2*5}} = {{2, 15, 10}, {2, 15, 10}}
        let c = a.hadamard_product(&b);

        assert_eq!(
            c,
            Matrix::new([
                Vector::new([Fp::from(2), Fp::from(15), Fp::from(10)]),
                Vector::new([Fp::from(2), Fp::from(15), Fp::from(10)]),
            ])
        );
    }
}
