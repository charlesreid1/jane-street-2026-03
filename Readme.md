# Planetary Parade

Solution to the [Jane Street puzzle, March 2026](https://www.janestreet.com/puzzles/): finding the probability of observing a planetary parade from the surface (and from a tower) on the planet Pyrknot.

**Answers: α = 1/32, β = 3/16**

## Quick Start

```bash
cargo run --release
```

This builds and runs a 10M-trial Monte Carlo simulation that prints:
- The estimated P(A), α, and β alongside their theoretical values.
- A table of β estimates at several small ε values, converging to 3/16.

## Problem Statement

We are enjoying a planetary parade during the beginning of this month here on Earth, but my alien friend from the
distant planet Pyrknot (well modeled as a perfect sphere) isn't too impressed. Their single-star system has six
planets other than Pyrknot with such chaotic orbits that each night these planets independently appear in uniformly
random locations in the Pyrknothian sky (the planets aren't visible in daylight). If at a given moment there exists
somewhere on Pyrknot that all six planets are visible, then from my friend's position on the surface they have an α
probability of also seeing all of the planets.

My friend is considering building a tower to improve their chances of seeing these planetary parades. Assume the
tower allows the viewing of every celestial body that could be seen from at least one location on the surface of
Pyrknot less than a distance r from the base of the tower. In the limit of r being small compared to the radius R
of Pyrknot, the new probability of seeing the planetary parade from the top of the tower is linearly approximated
by α + β·(r/R). Find α and β in exact terms.

## Solution

### Setup

Model the celestial sphere as S². A planet at direction **u** is visible from surface point **p** iff **u**·**p** > 0 (it lies in the open hemisphere above the horizon). Define:

- **Event A**: all 6 planets lie in some open hemisphere (a parade is visible from *somewhere* on Pyrknot).
- **Event B(ε)**: all 6 planets are visible from the tower at **p**, where ε = r/R.

We need P(B(ε) | A) ≈ α + β·ε.

### Finding P(A) via Wendel's formula

The probability that n independent uniform points on S^(d−1) all lie in some open hemisphere is:

```
p(n, d) = 2^(1-n) * sum_{k=0}^{d-1} C(n-1, k)
```

The celestial sphere is S² (d = 3) and n = 6:

```
P(A) = 2^(-5) * [C(5,0) + C(5,1) + C(5,2)] = (1 + 5 + 10) / 32 = 1/2
```

### Finding α

The friend is at a fixed point **p**. By symmetry, each planet's "height" h = **u**·**p** is uniform on [-1, 1], and the 6 heights are independent. All 6 are visible from **p** iff all h_i > 0:

```
P(B(0)) = (1/2)^6 = 1/64
```

Since B(0) is a subset of A (if the friend sees all 6, some hemisphere contains all 6):

```
α = P(B(0)) / P(A) = (1/64) / (1/2) = 1/32
```

### Tower geometry

The tower at **p** sees every direction **u** visible from any surface point within geodesic distance r of the base. A direction **u** at angle φ from **p** is visible from a surface point **q** at angle θ = r/R from **p** when **q** is tilted toward **u**, giving visibility when:

```
cos(φ - θ) > 0   =>   φ < π/2 + θ   =>   u·p > -sin(θ) ≈ -ε
```

So the tower sees direction **u** iff **u**·**p** > -ε, and:

```
P(B(ε)) = ((1 + ε) / 2)^6
```

### Finding β

We need P(B(ε) ∩ A^c) = O(ε²) to justify the approximation P(B(ε) | A) ≈ P(B(ε)) / P(A). The event B(ε) \ A requires all 6 planets in the expanded cap (h_i > -ε) yet no hemisphere containing all 6. This requires at least two planets in the thin band h_i ∈ (-ε, 0) in adversarial positions -- a codimension-2 condition with probability O(ε²). So to first order:

```
P(B(ε) | A) ≈ P(B(ε)) / P(A)
             = ((1 + ε) / 2)^6 / (1/2)
             = (1 + ε)^6 / 32
             ≈ (1 + 6ε) / 32
             = 1/32 + (3/16)·ε
```

Therefore **β = 3/16**.
