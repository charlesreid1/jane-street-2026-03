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

Model the celestial sphere as $S^2$. A planet at direction $\mathbf{u}$ is visible from surface point $\mathbf{p}$ iff $\mathbf{u} \cdot \mathbf{p} > 0$ (it lies in the open hemisphere above the horizon). Define:

- **Event A**: all 6 planets lie in some open hemisphere (a parade is visible from *somewhere* on Pyrknot).
- **Event B(ε)**: all 6 planets are visible from the tower at $\mathbf{p}$, where $\varepsilon = r/R$.

We need $P(B(\varepsilon) \mid A) \approx \alpha + \beta \cdot \varepsilon$.

### Finding P(A) via Wendel's formula

The probability that $n$ independent uniform points on $S^{d-1}$ all lie in some open hemisphere is:

$$p(n, d) = 2^{1-n} \sum_{k=0}^{d-1} \binom{n-1}{k}$$

The celestial sphere is $S^2$ ($d = 3$) and $n = 6$:

$$P(A) = 2^{-5} \left[ \binom{5}{0} + \binom{5}{1} + \binom{5}{2} \right] = \frac{1 + 5 + 10}{32} = \frac{1}{2}$$

### Finding α

The friend is at a fixed point $\mathbf{p}$. By symmetry, each planet's "height" $h = \mathbf{u} \cdot \mathbf{p}$ is uniform on $[-1, 1]$, and the 6 heights are independent. All 6 are visible from $\mathbf{p}$ iff all $h_i > 0$:

$$P(B(0)) = \left(\frac{1}{2}\right)^6 = \frac{1}{64}$$

Since $B(0) \subseteq A$ (if the friend sees all 6, some hemisphere contains all 6):

$$\alpha = \frac{P(B(0))}{P(A)} = \frac{1/64}{1/2} = \frac{1}{32}$$

### Tower geometry

The tower at $\mathbf{p}$ sees every direction $\mathbf{u}$ visible from any surface point within geodesic distance $r$ of the base. A direction $\mathbf{u}$ at angle $\varphi$ from $\mathbf{p}$ is visible from a surface point $\mathbf{q}$ at angle $\theta = r/R$ from $\mathbf{p}$ when $\mathbf{q}$ is tilted toward $\mathbf{u}$, giving visibility when:

$$\cos(\varphi - \theta) > 0 \implies \varphi < \frac{\pi}{2} + \theta \implies \mathbf{u} \cdot \mathbf{p} > -\sin\theta \approx -\varepsilon$$

So the tower sees direction $\mathbf{u}$ iff $\mathbf{u} \cdot \mathbf{p} > -\varepsilon$, and:

$$P(B(\varepsilon)) = \left(\frac{1 + \varepsilon}{2}\right)^6$$

### Finding β

#### Theoretical solution

We need $P(B(\varepsilon) \cap A^c) = O(\varepsilon^2)$ to justify the approximation $P(B(\varepsilon) \mid A) \approx P(B(\varepsilon)) / P(A)$. The event $B(\varepsilon) \setminus A$ requires all 6 planets in the expanded cap ($h_i > -\varepsilon$) yet no hemisphere containing all 6. This requires at least two planets in the thin band $h_i \in (-\varepsilon, 0)$ in adversarial positions -- a codimension-2 condition with probability $O(\varepsilon^2)$. So to first order:

$$P(B(\varepsilon) \mid A) \approx \frac{P(B(\varepsilon))}{P(A)} = \frac{\left(\frac{1+\varepsilon}{2}\right)^6}{\frac{1}{2}} = \frac{(1 + \varepsilon)^6}{32} \approx \frac{1 + 6\varepsilon}{32} = \frac{1}{32} + \frac{3}{16} \cdot \varepsilon$$

Therefore $\beta = 3/16$.

#### Computational solution

The Rust program (`src/main.rs`) performs a Monte Carlo verification of the theoretical result. Over 10 million independent trials it:

1. **Samples 6 planets uniformly on $S^2$** by drawing $\cos\theta \sim \text{Uniform}(-1, 1)$ and $\varphi \sim \text{Uniform}(0, 2\pi)$, then converting to Cartesian coordinates.

2. **Tests Event A** (parade exists somewhere) by checking whether all 6 points lie in some open hemisphere. For continuous random points this is decided by checking $O(n^2)$ candidate hemispheres: those centered at each point, and those whose bounding great circle passes through each pair of points.

3. **Tests Event B(ε)** (tower visibility) by fixing the friend at the north pole $\mathbf{p} = (0, 0, 1)$ and checking that the minimum height $h_{\min} = \min_i \mathbf{u}_i \cdot \mathbf{p} > -\varepsilon$, which is the tower visibility condition derived above.

4. **Estimates α and β** from the conditional counts:
   - $\hat{\alpha} = (\text{count of } B(0) \cap A) / (\text{count of } A)$
   - For each $\varepsilon$ in $\{0.001, 0.005, 0.01, 0.02\}$: $\hat{\beta}_\varepsilon = (\hat{P}(B(\varepsilon) \mid A) - \hat{\alpha}) / \varepsilon$

As $\varepsilon \to 0$ the estimates converge to the theoretical values $\alpha = 1/32$ and $\beta = 3/16$.
