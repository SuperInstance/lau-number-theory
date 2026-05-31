# lau-number-theory

Algebraic and analytic number theory in Rust — primes, modular arithmetic, Diophantine equations, continued fractions, and cryptographic agent ID generation.

## Features

- **Prime generation & testing**: Sieve of Eratosthenes, Miller-Rabin (deterministic for all u64), trial division
- **Modular arithmetic**: Modular exponentiation, inverse (extended GCD), CRT, modular square root (Tonelli-Shanks)
- **Arithmetic functions**: Euler's totient, Möbius function, divisor count/sum, Riemann zeta approximation, Mertens function
- **Continued fractions**: CF expansion of √n, convergents, quadratic irrationals, rational CF
- **Quadratic residues**: Legendre symbol, Jacobi symbol, Kronecker symbol, QR enumeration
- **Diophantine equations**: Linear Diophantine (general solution), Pell's equation (fundamental + nth solutions)
- **Dirichlet characters & L-functions**: Principal/Legendre characters, L(s,χ) computation
- **Agent ID generation**: Cryptographic-strength number-theoretic agent identification

## Usage

```toml
[dependencies]
lau-number-theory = "0.1"
```

## License

MIT
