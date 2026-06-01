# lau-number-theory

Algebraic and analytic number theory in pure Rust — prime generation and testing, modular arithmetic, arithmetic functions (Euler's totient, Möbius, divisor sums, Riemann zeta), continued fractions, quadratic residues, Diophantine equations (linear + Pell's), Dirichlet characters and L-functions, and cryptographic agent ID generation.

No external number theory dependencies — everything is implemented from scratch using `nalgebra`, `serde`, and the standard library.

---

## What This Does

This library covers the core of computational number theory:

- **Primes** — Sieve of Eratosthenes, trial division, deterministic Miller-Rabin (12 witnesses, correct for all u64), prime factorization, modular exponentiation (overflow-safe), nth prime, prime counting function π(n).
- **Modular arithmetic** — Extended Euclidean algorithm, modular inverse, Chinese Remainder Theorem (CRT), Tonelli-Shanks modular square root, modular add/sub/mul.
- **Arithmetic functions** — Euler's totient φ(n) (single + sieve), Möbius function μ(n) (single + linear sieve), divisor count d(n), divisor sum σ(n) (overflow-safe u128 variant), Riemann zeta ζ(s) via Euler product, Mertens function M(n), gcd, lcm.
- **Continued fractions** — CF expansion of √n (periodic part extraction), convergent computation (h/k pairs), rational CF expansion, period length.
- **Quadratic residues** — Legendre symbol (a/p), Jacobi symbol (a/n), Kronecker symbol (a/n), quadratic residue testing, enumeration of all QRs mod p.
- **Diophantine equations** — Linear Diophantine solver (particular + general parameterized solution), Pell's equation x²−Dy²=1 (fundamental solution via continued fractions), negative Pell x²−Dy²=−1, nth Pell solution via exponentiation in ℤ[√D].
- **Dirichlet characters & L-functions** — Principal and Legendre characters, L(s,χ) evaluation for real and complex s, custom Complex type with arithmetic ops.
- **Agent IDs** — Cryptographic agent identifiers from strong primes, deterministic generation, verification via factorization and totient checking, CRT-based agent namespaces, multi-agent hashing.

---

## Key Idea

Number theory is the foundation of cryptography, and this library bridges pure mathematics with practical agent identity. Every function — from prime sieves to Pell's equation — is implemented explicitly and testably. The agent ID system uses strong primes, modular exponentiation, Jacobi symbols, and CRT to generate deterministic, verifiable identities grounded in hard number-theoretic problems.

---

## Install

```toml
[dependencies]
lau-number-theory = "0.1"
```

Or clone and build:

```bash
git clone https://github.com/SuperInstance/lau-number-theory.git
cd lau-number-theory
cargo build
```

**Requirements:** Rust 2021 edition (≥ 1.56).

---

## Quick Start

### Prime generation and factorization

```rust
use lau_number_theory::*;

let primes = sieve_primes(100);
println!("primes up to 100: {:?}", primes);

let factors = factorize(360);
for f in &factors {
    println!("{}^{}", f.prime, f.exponent);
}
// 2^3, 3^2, 5^1

assert!(is_prime(999999999999999989));
assert!(!is_prime(999999999999999990));
```

### Chinese Remainder Theorem

```rust
use lau_number_theory::*;

// x ≡ 2 (mod 3), x ≡ 3 (mod 5), x ≡ 2 (mod 7) → x = 23
let x = crt(&[2, 3, 2], &[3, 5, 7]);
assert_eq!(x, Some(23));
```

### Arithmetic functions

```rust
use lau_number_theory::*;

println!("φ(30) = {}", euler_totient(30));  // 8
println!("μ(30) = {}", mobius(30));         // -1
println!("σ(28) = {}", divisor_sum_safe(28)); // 56 (perfect number)
println!("ζ(2) ≈ {:.6}", zeta_approx(2.0, 100)); // ≈ 1.6449... = π²/6
println!("M(100) = {}", mertens(100));       // 1
```

### Continued fractions and Pell's equation

```rust
use lau_number_theory::*;

let cf = ContinuedFraction::from_sqrt(2);
println!("√2 = [{}; {:?}]", cf.a0, cf.periodic); // [1; [2]]

let convs = cf.convergents(5);
for (h, k) in &convs {
    println!("  {}/{} = {:.10}", h, k, *h as f64 / *k as f64);
}

// Solve x² - 13y² = 1 → x=649, y=180
let sol = pell_solve(13).unwrap();
println!("Pell(13): x={}, y={}", sol.x, sol.y);
assert_eq!(sol.x, 649);
assert_eq!(sol.y, 180);
```

### Quadratic residues

```rust
use lau_number_theory::*;

println!("(2/7) = {}", legendre(2, 7));   // 1 (QR)
println!("(3/7) = {}", legendre(3, 7));   // -1 (non-residue)
println!("(2/15) = {}", jacobi(2, 15));   // 1
println!("QRs mod 7: {:?}", quadratic_residues(7)); // [1, 2, 4]

let sqrt = mod_sqrt(2, 7); // √2 mod 7
assert!(sqrt.is_some());
```

### Agent ID generation

```rust
use lau_number_theory::*;

let agent = AgentId::generate(42);
println!("id: {}", agent.id);
println!("factors: {:?}", agent.factors);
println!("φ(id): {}", agent.totient);
println!("hash: {}", agent.hash);
println!("fingerprint: {}", agent.fingerprint());
assert!(agent.verify());
```

---

## API Reference

### Primes (`primes`)

| Function | Description |
|----------|-------------|
| `sieve_primes(limit)` | All primes ≤ limit via Eratosthenes |
| `is_prime_small(n)` | Trial division (n < 10⁶) |
| `miller_rabin(n, k)` | Miller-Rabin with 12 deterministic witnesses |
| `is_prime(n)` | Automatic: trial division or Miller-Rabin |
| `factorize(n)` | Prime factorization → `Vec<PrimeFactor>` |
| `mod_pow(base, exp, m)` | Modular exponentiation (u128 overflow-safe) |
| `nth_prime(n)` | 0-indexed nth prime |
| `prime_count(n)` | Prime counting function π(n) |

### Modular Arithmetic (`modular`)

| Function | Description |
|----------|-------------|
| `extended_gcd(a, b)` | Returns (gcd, x, y) with ax + by = gcd |
| `mod_inverse(a, m)` | Modular inverse via extended GCD |
| `crt(remainders, moduli)` | Chinese Remainder Theorem solver |
| `mod_sqrt(n, p)` | Tonelli-Shanks modular square root |
| `mod_add(a, b, m)`, `mod_sub(a, b, m)`, `mod_mul(a, b, m)` | Modular arithmetic |

### Arithmetic Functions (`arithmetic`)

| Function | Description |
|----------|-------------|
| `euler_totient(n)` | φ(n) — count of coprimes |
| `euler_totient_sieve(n)` | φ(1..=n) via sieve |
| `mobius(n)` | μ(n) — +1/−1/0 for square-free |
| `mobius_sieve(n)` | μ(1..=n) via linear sieve |
| `divisor_count(n)` | d(n) — number of divisors |
| `divisor_sum(n)` | σ(n) — sum of divisors (u64) |
| `divisor_sum_safe(n)` | σ(n) — overflow-safe (u128) |
| `zeta_approx(s, terms)` | ζ(s) via Euler product |
| `mertens(n)` | M(n) = Σ μ(k) for k=1..n |
| `gcd(a, b)`, `lcm(a, b)` | Greatest common divisor, least common multiple |

### Continued Fractions (`continued_fraction`)

| Method | Description |
|--------|-------------|
| `ContinuedFraction::from_sqrt(n)` | CF expansion of √n |
| `rational_cf(p, q)` | CF expansion of p/q |
| `.convergents(count)` | Best rational approximations (h, k) pairs |
| `.period_length()` | Period of the CF expansion |
| `convergent_value(h, k)` | Float value h/k |

### Quadratic Residues (`quadratic`)

| Function | Description |
|----------|-------------|
| `legendre(a, p)` | Legendre symbol (a/p) for odd prime p |
| `jacobi(a, n)` | Jacobi symbol (a/n) for odd n |
| `kronecker(a, n)` | Kronecker symbol (generalization) |
| `is_quadratic_residue(a, p)` | Boolean QR test |
| `quadratic_residues(p)` | All QRs mod p, sorted |

### Diophantine Equations (`diophantine`)

| Function | Description |
|----------|-------------|
| `linear_diophantine(a, b, c)` | Solve ax + by = c → particular solution |
| `linear_diophantine_general(a, b, c)` | Parameterized general solution |
| `pell_solve(d)` | Fundamental solution of x² − Dy² = 1 |
| `pell_solve_negative(d)` | Solve x² − Dy² = −1 |
| `pell_nth_solution(d, &fundamental, n)` | nth solution via √D-ring exponentiation |

### Dirichlet Characters & L-Functions (`dirichlet`)

| Method | Description |
|--------|-------------|
| `DirichletCharacter::principal(q)` | Principal character mod q |
| `DirichletCharacter::legendre_character(p)` | Legendre symbol character mod p |
| `.eval(n)` / `.eval_i64(n)` | Evaluate χ(n) |
| `.is_principal()` | Check if principal character |
| `l_function_at_1(&chi, terms)` | L(1, χ) via partial sums |
| `l_function(&chi, s, terms)` | L(s, χ) for complex s |
| `l_function_real(&chi, s, terms)` | L(s, χ) for real s |

**Complex** — Custom complex number type with +, −, ×, ÷, pow, exp, ln.

### Agent IDs (`agent_id`)

| Method | Description |
|--------|-------------|
| `AgentId::generate(seed)` | Deterministic ID from strong primes |
| `.verify()` | Check factorization and totient consistency |
| `.fingerprint()` | 16-char hex fingerprint |
| `find_strong_prime(n)` | Smallest prime ≥ n |
| `agent_namespace(&[ids])` | CRT-based unique namespace |
| `agent_hash(&[agents])` | Multi-agent number-theoretic hash |

---

## How It Works

### Miller-Rabin Primality

Write n − 1 = 2ʳ · d with d odd. For each witness a, compute x = aᵈ mod n. If x ≡ ±1, the witness passes. Otherwise square r − 1 times; if none equal n − 1, n is composite. The 12 witnesses {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37} are deterministic for all n < 3.3 × 10²⁴, which covers the entire u64 range.

### Chinese Remainder Theorem

Given coprime moduli m₁, …, mₖ and remainders r₁, …, rₖ, iteratively merge congruences: at each step, find x such that x ≡ current (mod M) and x ≡ rᵢ (mod mᵢ) using the extended GCD to compute modular inverses.

### Tonelli-Shanks Modular Square Root

Given n and odd prime p with (n/p) = 1:
1. Write p − 1 = Q · 2ˢ with Q odd.
2. Find a quadratic non-residue z.
3. Initialize c = z^Q, t = n^Q, r = n^((Q+1)/2).
4. Iteratively reduce: find least i with t^(2ⁱ) ≡ 1, then update r, c, t using b = c^(2^(s-i-1)).

### Pell's Equation

For x² − Dy² = 1, the continued fraction expansion of √D converges to the fundamental solution. The convergents pₖ/qₖ of [a₀; a₁, a₂, …] are tested: the first (h, k) satisfying h² − Dk² = 1 is the fundamental solution. Further solutions are generated by exponentiation in ℤ[√D]: (xₙ + yₙ√D) = (x₁ + y₁√D)ⁿ.

### Riemann Zeta via Euler Product

$$\zeta(s) = \prod_{p \text{ prime}} \frac{p^s}{p^s - 1}$$

Uses the first N primes from the sieve. For s = 2, converges to π²/6 ≈ 1.6449.

### Agent ID Generation

1. Find two strong primes p₁, p₂ near seed-derived offsets.
2. Compute id = p₁ · p₂ and φ(id) = (p₁−1)(p₂−1).
3. Generate hash via modular exponentiation with a Jacobi symbol twist.
4. Verification checks factorization product, primality of factors, and totient consistency.

---

## The Math

### Euler's Totient (Multiplicative)

For n = p₁^{k₁} · p₂^{k₂} · … :

$$\varphi(n) = n \prod_{p | n} \left(1 - \frac{1}{p}\right)$$

### Möbius Function

$$\mu(n) = \begin{cases} 0 & \text{if } n \text{ has a squared factor} \\ (-1)^k & \text{if } n = p_1 \cdots p_k \text{ (square-free)} \end{cases}$$

Key identity: $\sum_{d|n} \mu(d) = [n = 1]$.

### Möbius Inversion

$$f(n) = \sum_{d|n} g(d) \iff g(n) = \sum_{d|n} \mu\left(\frac{n}{d}\right) f(d)$$

### Legendre Symbol

$$\left(\frac{a}{p}\right) = a^{(p-1)/2} \bmod p \in \{0, 1, -1\}$$

### Jacobi Symbol (Recursion)

$$\left(\frac{a}{n}\right) = \left(\frac{a}{p_1}\right)^{e_1} \cdots \left(\frac{a}{p_k}\right)^{e_k}$$

Computed via quadratic reciprocity: if a, n ≡ 3 (mod 4), flip sign on swap.

### Dirichlet L-Functions

$$L(s, \chi) = \sum_{n=1}^{\infty} \frac{\chi(n)}{n^s}$$

For non-principal characters, L(1, χ) converges and encodes class number information (Dirichlet's class number formula).

### Pell's Equation

$$x^2 - Dy^2 = 1$$

The fundamental solution comes from the continued fraction of √D. All solutions form a group under multiplication in ℤ[√D]:

$$(x_n, y_n) = (x_1 + y_1\sqrt{D})^n$$

---

## Tests

69 inline unit tests across all modules:

```bash
cargo test
```

Tests cover: sieve correctness, primality (small, large, Carmichael numbers like 561), factorization, modular exponentiation, nth prime, prime counting, modular inverse, extended GCD (Bézout's identity), CRT (single, pair, multi-modulus), Tonelli-Shanks (QR/NQR), Euler totient (single + sieve), Möbius (single + sieve), divisor count and sum, Riemann zeta approximation (ζ(2) ≈ π²/6), gcd/lcm, continued fractions (√2, √3, √7, perfect squares), convergent accuracy, rational CF, period lengths, Legendre/Jacobi/Kronecker symbols, quadratic residue enumeration, linear Diophantine (with and without solutions), Pell's equation (D=2,3,5,7,13), negative Pell, Pell nth solution, Dirichlet characters (principal, Legendre), L-function convergence, complex arithmetic, agent ID generation/verification/determinism/fingerprints, and namespace construction.

---

## License

MIT
