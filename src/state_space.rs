use nalgebra::{DMatrix, DVector};
use thiserror::Error;

/// Represents a Linear Time-Invariant (LTI) system in state-space form
///
/// Continuous-time: dx/dt = A*x + B*u, y = C*x + D*u
/// Discrete-time: x[k+1] = A*x[k] + B*u[k], y[k] = C*x[k] + D*u[k]
#[derive(Debug, Clone)]
pub struct StateSpaceSystem {
    /// State matrix (n x n)
    pub a: DMatrix<f64>,
    /// Input matrix (n x m)
    pub b: DMatrix<f64>,
    /// Output matrix (p x n)
    pub c: DMatrix<f64>,
    /// Feedthrough matrix (p x m)
    pub d: DMatrix<f64>,
    /// Sampling time for discrete systems (None for continuous)
    pub dt: Option<f64>,
}

impl StateSpaceSystem {
    /// Create a new state-space system
    ///
    /// # Arguments
    /// * `a` - State matrix (n x n)
    /// * `b` - Input matrix (n x m)
    /// * `c` - Output matrix (p x n)
    /// * `d` - Feedthrough matrix (p x m)
    /// * `dt` - Sampling time (None for continuous-time)
    ///
    /// # Returns
    /// * `Result<StateSpaceSystem, StateSpaceError>`
    pub fn new<A, B, C, D>(a: A, b: B, c: C, d: D, dt: Option<f64>) -> Result<Self, StateSpaceError>
    where
        A: Into<DMatrix<f64>>,
        B: Into<DMatrix<f64>>,
        C: Into<DMatrix<f64>>,
        D: Into<DMatrix<f64>>,
    {
        let a = a.into();
        let b = b.into();
        let c = c.into();
        let d = d.into();

        // Validate dimensions
        let n = a.nrows();
        if a.ncols() != n {
            return Err(StateSpaceError::InvalidStateMatrix {
                expected: (n, n),
                actual: (a.nrows(), a.ncols()),
            });
        }

        if b.nrows() != n {
            return Err(StateSpaceError::InvalidInputMatrix {
                expected_rows: n,
                actual: b.nrows(),
            });
        }

        let m = b.ncols();
        let p = c.nrows();
        if c.ncols() != n {
            return Err(StateSpaceError::InvalidOutputMatrix {
                expected_cols: n,
                actual: c.ncols(),
            });
        }

        if d.nrows() != p || d.ncols() != m {
            return Err(StateSpaceError::InvalidFeedthroughMatrix {
                expected: (p, m),
                actual: (d.nrows(), d.ncols()),
            });
        }

        Ok(StateSpaceSystem { a, b, c, d, dt })
    }

    /// Convert continuous-time system to discrete-time using zero-order hold
    ///
    /// # Arguments
    /// * `dt` - Sampling time
    ///
    /// # Returns
    /// * `Result<StateSpaceSystem, StateSpaceError>`
    pub fn to_discrete(&self, dt: f64) -> Result<StateSpaceSystem, StateSpaceError> {
        if self.dt.is_some() {
            return Err(StateSpaceError::AlreadyDiscrete);
        }

        // Using matrix exponential for exact discretization
        // Ad = exp(A*dt), Bd = integral(exp(A*tau)dtau from 0 to dt) * B
        let a_dt = &self.a * dt;
        let ad = a_dt.exp(); // Matrix exponential

        // Compute Bd = (Ad - I) * A^(-1) * B (if A is invertible)
        // For simplicity, we'll use approximation for now
        let bd = (&ad - DMatrix::identity(self.a.nrows(), self.a.ncols()))
            * self.a.clone().try_inverse().ok_or(StateSpaceError::StateMatrixSingular)?
            * &self.b;

        Ok(StateSpaceSystem {
            a: ad,
            b: bd,
            c: self.c.clone(),
            d: self.d.clone(),
            dt: Some(dt),
        })
    }

    /// Predict next state given current state and input
    ///
    /// # Arguments
    /// * `x` - Current state vector
    /// * `u` - Input vector
    ///
    /// # Returns
    /// * `Result<VectorN<f64, Dyn>, StateSpaceError>`
    pub fn predict(&self, x: &DVector<f64>, u: &DVector<f64>) -> Result<DVector<f64>, StateSpaceError> {
        if self.dt.is_some() {
            // Discrete-time: x[k+1] = A*x[k] + B*u[k]
            if x.len() != self.a.nrows() {
                return Err(StateSpaceError::InvalidStateVector {
                    expected: self.a.nrows(),
                    actual: x.len(),
                });
            }
            if u.len() != self.b.ncols() {
                return Err(StateSpaceError::InvalidInputVector {
                    expected: self.b.ncols(),
                    actual: u.len(),
                });
            }
            Ok(&self.a * x + &self.b * u)
        } else {
            // Continuous-time: dx/dt = A*x + B*u
            // Return derivative (caller needs to integrate)
            if x.len() != self.a.nrows() {
                return Err(StateSpaceError::InvalidStateVector {
                    expected: self.a.nrows(),
                    actual: x.len(),
                });
            }
            if u.len() != self.b.ncols() {
                return Err(StateSpaceError::InvalidInputVector {
                    expected: self.b.ncols(),
                    actual: u.len(),
                });
            }
            Ok(&self.a * x + &self.b * u)
        }
    }

    /// Compute output given state and input
    ///
    /// # Arguments
    /// * `x` - State vector
    /// * `u` - Input vector
    ///
    /// # Returns
    /// * `Result<VectorN<f64, Dyn>, StateSpaceError>`
    pub fn output(&self, x: &DVector<f64>, u: &DVector<f64>) -> Result<DVector<f64>, StateSpaceError> {
        if x.len() != self.a.nrows() {
            return Err(StateSpaceError::InvalidStateVector {
                expected: self.a.nrows(),
                actual: x.len(),
            });
        }
        if u.len() != self.b.ncols() {
            return Err(StateSpaceError::InvalidInputVector {
                expected: self.b.ncols(),
                actual: u.len(),
            });
        }
        Ok(&self.c * x + &self.d * u)
    }

    /// Check if the system is controllable
    ///
    /// # Returns
    /// * `bool` - True if controllable
    pub fn is_controllable(&self) -> bool {
        let n = self.a.nrows();
        let m = self.b.ncols();
        let mut controllability_matrix = DMatrix::zeros(n, n * m);
        
        let mut current_ab = self.b.clone();
        for i in 0..n {
            let mut slice = controllability_matrix
                .view_mut((0, i * m), (n, m));
            slice.copy_from(&current_ab);
            current_ab = &self.a * &current_ab;
        }
        
        controllability_matrix.rank(1e-6) == n
    }

    /// Check if the system is observable
    ///
    /// # Returns
    /// * `bool` - True if observable
    pub fn is_observable(&self) -> bool {
        let n = self.a.nrows();
        let p = self.c.nrows();
        let mut observability_matrix = DMatrix::zeros(p * n, n);
         
        let mut current_ca = self.c.clone();
        for i in 0..n {
            let mut slice = observability_matrix
                .view_mut((i * p, 0), (p, n));
            slice.copy_from(&current_ca);
            current_ca = &current_ca * &self.a;
        }
        
        observability_matrix.rank(1e-6) == n
    }
}

/// Errors that can occur when working with state-space systems
#[derive(Debug, Error)]
pub enum StateSpaceError {
    #[error("Invalid state matrix dimensions: expected {expected:?}, got {actual:?}")]
    InvalidStateMatrix { expected: (usize, usize), actual: (usize, usize) },
    
    #[error("Invalid input matrix: expected {expected_rows} rows, got {actual}")]
    InvalidInputMatrix { expected_rows: usize, actual: usize },
    
    #[error("Invalid output matrix: expected {expected_cols} columns, got {actual}")]
    InvalidOutputMatrix { expected_cols: usize, actual: usize },
    
    #[error("Invalid feedthrough matrix: expected {expected:?}, got {actual:?}")]
    InvalidFeedthroughMatrix { expected: (usize, usize), actual: (usize, usize) },
    
    #[error("Invalid state vector: expected {expected} elements, got {actual}")]
    InvalidStateVector { expected: usize, actual: usize },
    
    #[error("Invalid input vector: expected {expected} elements, got {actual}")]
    InvalidInputVector { expected: usize, actual: usize },
    
    #[error("Cannot convert continuous-time system to discrete-time: already discrete")]
    AlreadyDiscrete,
    
    #[error("State matrix is singular and cannot be inverted")]
    StateMatrixSingular,
    
    #[error("Other error: {0}")]
    Other(#[from] Box<dyn std::error::Error>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_state_space_creation() {
        // Simple mass-spring-damper system
        let a = DMatrix::from_row_slice(2, 2, &[0.0, 1.0, -2.0, -0.5]);
        let b = DMatrix::from_row_slice(2, 1, &[0.0, 1.0]);
        let c = DMatrix::from_row_slice(1, 2, &[1.0, 0.0]);
        let d = DMatrix::from_row_slice(1, 1, &[0.0]);
        
        let system = StateSpaceSystem::new(a, b, c, d, None).unwrap();
        assert_eq!(system.a.nrows(), 2);
        assert_eq!(system.a.ncols(), 2);
        assert_eq!(system.b.nrows(), 2);
        assert_eq!(system.b.ncols(), 1);
        assert_eq!(system.c.nrows(), 1);
        assert_eq!(system.c.ncols(), 2);
        assert_eq!(system.d.nrows(), 1);
        assert_eq!(system.d.ncols(), 1);
        assert!(system.dt.is_none());
    }

    #[test]
    fn test_predict_output() {
        // Identity system: x[k+1] = x[k] + u[k], y[k] = x[k]
        let a = DMatrix::identity(2, 2);
        let b = DMatrix::from_row_slice(2, 1, &[1.0, 0.0]);
        let c = DMatrix::from_row_slice(1, 2, &[1.0, 0.0]);
        let d = DMatrix::zeros(1, 1);
        
        let system = StateSpaceSystem::new(a, b, c, d, Some(0.01)).unwrap();
        
        let x = DVector::from_vec(vec![1.0, 2.0]);
        let u = DVector::from_vec(vec![0.5]);
        
        let x_next = system.predict(&x, &u).unwrap();
        assert_abs_diff_eq!(x_next[0], 1.5, epsilon = 1e-10); // 1.0 + 0.5*0.01
        assert_abs_diff_eq!(x_next[1], 2.0, epsilon = 1e-10); // unchanged
        
        let y = system.output(&x, &u).unwrap();
        assert_abs_diff_eq!(y[0], 1.0, epsilon = 1e-10); // first state variable
    }

    #[test]
    fn test_controllability_observability() {
        // Controllable and observable system
        let a = DMatrix::from_row_slice(2, 2, &[0.0, 1.0, -1.0, -0.5]);
        let b = DMatrix::from_row_slice(2, 1, &[0.0, 1.0]);
        let c = DMatrix::from_row_slice(1, 2, &[1.0, 0.0]);
        let d = DMatrix::zeros(1, 1);
        
        let system = StateSpaceSystem::new(a.clone(), b.clone(), c.clone(), d.clone(), Some(0.01)).unwrap();
        assert!(system.is_controllable());
        assert!(system.is_observable());
        
        // Uncontrollable system (B = 0)
        let b_zero = DMatrix::zeros(2, 1);
        let system_uncontrollable = StateSpaceSystem::new(a.clone(), b_zero, c.clone(), d.clone(), Some(0.01)).unwrap();
        assert!(!system_uncontrollable.is_controllable());
        
        // Unobservable system (C = 0)
        let c_zero = DMatrix::zeros(1, 2);
        let system_unobservable = StateSpaceSystem::new(a.clone(), b.clone(), c_zero, d.clone(), Some(0.01)).unwrap();
        assert!(!system_unobservable.is_observable());
    }
}