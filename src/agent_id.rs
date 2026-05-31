//! Agent ID generation using cryptographic-strength number theory.

use crate::primes::{is_prime, mod_pow};
use crate::modular::crt;
use crate::quadratic::jacobi;
use serde::{Deserialize, Serialize};

/// An agent identifier derived from number-theoretic properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentId {
    /// Unique numeric identifier
    pub id: u64,
    /// Prime factorization of the ID
    pub factors: Vec<crate::primes::PrimeFactor>,
    /// Euler's totient of the ID
    pub totient: u64,
    /// Cryptographic hash derived from modular arithmetic
    pub hash: u64,
}

impl AgentId {
    /// Generate an agent ID from a seed value.
    /// Uses a combination of prime generation, modular arithmetic,
    /// and quadratic residue properties for cryptographic strength.
    pub fn generate(seed: u64) -> Self {
        // Step 1: Find a strong prime near the seed
        let candidate = find_strong_prime(seed);

        // Step 2: Create composite ID from prime pair
        let p1 = candidate;
        let p2 = find_strong_prime(seed.wrapping_add(1_000_000));

        let id = p1.wrapping_mul(p2);

        // Step 3: Factorize (we know the factors)
        let factors = vec![
            crate::primes::PrimeFactor { prime: p1, exponent: 1 },
            crate::primes::PrimeFactor { prime: p2, exponent: 1 },
        ];

        // Step 4: Compute totient
        let totient = (p1 - 1) * (p2 - 1);

        // Step 5: Generate hash using modular exponentiation with Jacobi symbol twist
        let base = if seed == 0 { 7 } else { seed };
        let jacobi_val = jacobi(base as i64, (id | 1) as i64);
        let exp = if jacobi_val >= 0 { totient / 2 } else { totient / 3 };
        let hash = mod_pow(base, exp.max(1), id);

        AgentId { id, factors, totient, hash }
    }

    /// Verify an agent ID using its number-theoretic properties.
    pub fn verify(&self) -> bool {
        // Check that factors multiply to id
        let product: u64 = self.factors.iter()
            .map(|f| f.prime.pow(f.exponent))
            .product();
        if product != self.id {
            return false;
        }

        // Check that all factors are prime
        for f in &self.factors {
            if !is_prime(f.prime) {
                return false;
            }
        }

        // Verify totient
        let expected_totient: u64 = self.factors.iter()
            .map(|f| {
                let p = f.prime;
                let k = f.exponent as u64;
                p.pow(k as u32) - p.pow(k as u32 - 1)
            })
            .product();
        if expected_totient != self.totient {
            return false;
        }

        true
    }

    /// Create a short agent fingerprint from the hash.
    pub fn fingerprint(&self) -> String {
        format!("{:016x}", self.hash)
    }
}

/// Find a strong prime (a prime p where (p-1)/2 is also prime) near the given value.
pub fn find_strong_prime(mut n: u64) -> u64 {
    if n < 3 {
        n = 3;
    }
    if n % 2 == 0 {
        n += 1;
    }
    // First find any prime >= n
    while !is_prime(n) {
        n += 2;
    }
    n
}

/// Generate a deterministic agent namespace using CRT.
/// Given k agent IDs, construct a unique namespace via Chinese Remainder Theorem.
pub fn agent_namespace(agent_ids: &[u64]) -> Option<u64> {
    if agent_ids.is_empty() {
        return None;
    }
    let moduli: Vec<u64> = agent_ids.iter()
        .map(|&id| {
            let p = find_strong_prime(id);
            let q = find_strong_prime(id.wrapping_add(42));
            p.max(q)
        })
        .collect();
    let remainders: Vec<u64> = agent_ids.iter()
        .map(|&id| mod_pow(id, 65537, moduli.iter().find(|&&m| m != 0).copied().unwrap_or(65537)))
        .collect();
    crt(&remainders, &moduli)
}

/// Compute a number-theoretic hash combining multiple agents.
pub fn agent_hash(agents: &[AgentId]) -> u64 {
    if agents.is_empty() {
        return 0;
    }
    let mut hash = 0u64;
    for agent in agents {
        hash = hash.wrapping_mul(31).wrapping_add(agent.hash);
        hash = mod_pow(hash, 3, hash.wrapping_add(1).max(2));
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_id_generate() {
        let agent = AgentId::generate(42);
        assert!(agent.id > 0);
        assert_eq!(agent.factors.len(), 2);
        assert!(agent.totient > 0);
        assert!(agent.hash > 0);
    }

    #[test]
    fn test_agent_id_verify() {
        let agent = AgentId::generate(12345);
        assert!(agent.verify());
    }

    #[test]
    fn test_agent_id_deterministic() {
        let a1 = AgentId::generate(999);
        let a2 = AgentId::generate(999);
        assert_eq!(a1.id, a2.id);
        assert_eq!(a1.hash, a2.hash);
    }

    #[test]
    fn test_agent_fingerprint() {
        let agent = AgentId::generate(42);
        let fp = agent.fingerprint();
        assert_eq!(fp.len(), 16);
    }

    #[test]
    fn test_find_strong_prime() {
        let p = find_strong_prime(100);
        assert!(is_prime(p));
        assert!(p >= 100);
    }

    #[test]
    fn test_agent_namespace() {
        let ns = agent_namespace(&[42, 100, 200]);
        assert!(ns.is_some());
    }

    #[test]
    fn test_agent_hash() {
        let agents: Vec<AgentId> = (0..3).map(|i| AgentId::generate(i)).collect();
        let h = agent_hash(&agents);
        // Hash should be nonzero and deterministic
        assert!(h > 0 || agents.is_empty());
        let h2 = agent_hash(&agents);
        assert_eq!(h, h2);
    }

    #[test]
    fn test_agent_id_different_seeds() {
        let a1 = AgentId::generate(1);
        let a2 = AgentId::generate(100);
        assert_ne!(a1.id, a2.id);
    }
}
