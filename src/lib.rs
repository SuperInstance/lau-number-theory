//! # lau-number-theory
//!
//! Algebraic and analytic number theory: primes, modular arithmetic,
//! Diophantine equations, continued fractions, and cryptographic agent ID generation.

pub mod primes;
pub mod modular;
pub mod arithmetic;
pub mod continued_fraction;
pub mod quadratic;
pub mod diophantine;
pub mod dirichlet;
pub mod agent_id;

pub use primes::*;
pub use modular::*;
pub use arithmetic::*;
pub use continued_fraction::*;
pub use quadratic::*;
pub use diophantine::*;
pub use dirichlet::*;
pub use agent_id::*;
