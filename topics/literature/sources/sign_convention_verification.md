# Sign Convention Verification

## Setup
- Coordinates: x = (q₁, q₂, p₁, p₂) ∈ ℝ⁴
- J(q₁,q₂,p₁,p₂) = (p₁,p₂,-q₁,-q₂)
- Block form: J = [[0, I₂], [-I₂, 0]]
- Symplectic form: ω(u,v) = ⟨-Ju, v⟩

---

## Claim 1: Coordinate formula for ω

**Claim:** ω(u,v) = u_{q₁}v_{p₁} + u_{q₂}v_{p₂} - u_{p₁}v_{q₁} - u_{p₂}v_{q₂}

**Computation:**
Let u = (u_{q₁}, u_{q₂}, u_{p₁}, u_{p₂}) and v = (v_{q₁}, v_{q₂}, v_{p₁}, v_{p₂}).

Ju = (u_{p₁}, u_{p₂}, -u_{q₁}, -u_{q₂})

-Ju = (-u_{p₁}, -u_{p₂}, u_{q₁}, u_{q₂})

ω(u,v) = ⟨-Ju, v⟩
       = (-u_{p₁})(v_{q₁}) + (-u_{p₂})(v_{q₂}) + (u_{q₁})(v_{p₁}) + (u_{q₂})(v_{p₂})
       = u_{q₁}v_{p₁} + u_{q₂}v_{p₂} - u_{p₁}v_{q₁} - u_{p₂}v_{q₂}

**Result:** **CORRECT**

---

## Claim 2: J² = -I₄ and J^T = -J

**Computation of J²:**
J(q₁,q₂,p₁,p₂) = (p₁,p₂,-q₁,-q₂)

J(J(q₁,q₂,p₁,p₂)) = J(p₁,p₂,-q₁,-q₂)
                    = (-q₁,-q₂,-p₁,-p₂)
                    = -(q₁,q₂,p₁,p₂)

So J² = -I₄. ✓

**Computation of J^T:**
J = [[0, I₂], [-I₂, 0]]

J^T = [[0, -I₂], [I₂, 0]]

-J = [[0, -I₂], [I₂, 0]]

So J^T = -J. ✓

**Result:** **CORRECT**

---

## Claim 3: ω is J-invariant

**Claim:** ω(Ju, Jv) = ω(u,v)

**Computation:**
ω(Ju, Jv) = ⟨-J(Ju), Jv⟩
          = ⟨-J²u, Jv⟩
          = ⟨u, Jv⟩        [since -J² = I₄]

Now, using J^T = -J:
⟨u, Jv⟩ = u^T(Jv) = (u^T J)v = (J^T u)^T v = (-Ju)^T v = ⟨-Ju, v⟩ = ω(u,v)

**Result:** **CORRECT**

---

## Claim 4: Action formula in coordinates

**Claim:** For γ(t) = (q(t), p(t)), A(γ) = ½∫₀ᵀ ⟨Jγ, γ̇⟩dt = ½∫(p·q̇ - q·ṗ)dt

**Computation:**
γ = (q₁, q₂, p₁, p₂)
Jγ = (p₁, p₂, -q₁, -q₂)
γ̇ = (q̇₁, q̇₂, ṗ₁, ṗ₂)

⟨Jγ, γ̇⟩ = p₁q̇₁ + p₂q̇₂ + (-q₁)ṗ₁ + (-q₂)ṗ₂
        = p₁q̇₁ + p₂q̇₂ - q₁ṗ₁ - q₂ṗ₂
        = p·q̇ - q·ṗ

Therefore:
A(γ) = ½∫₀ᵀ ⟨Jγ, γ̇⟩dt = ½∫₀ᵀ (p·q̇ - q·ṗ)dt

**Result:** **CORRECT**

---

## Claim 5: Integration by parts identity

**Claim:** For closed curves, ∫₀ᵀ ⟨Jγ, γ̇⟩dt = -∫₀ᵀ ⟨Jγ̇, γ⟩dt

**Computation:**
Consider the product rule:
d/dt[⟨Jγ, γ⟩] = ⟨Jγ̇, γ⟩ + ⟨Jγ, γ̇⟩

For closed curves γ(0) = γ(T), so:
∫₀ᵀ d/dt[⟨Jγ, γ⟩]dt = ⟨Jγ(T), γ(T)⟩ - ⟨Jγ(0), γ(0)⟩ = 0

Therefore:
0 = ∫₀ᵀ [⟨Jγ̇, γ⟩ + ⟨Jγ, γ̇⟩]dt

This gives:
∫₀ᵀ ⟨Jγ, γ̇⟩dt = -∫₀ᵀ ⟨Jγ̇, γ⟩dt

**Result:** **CORRECT**

---

## Claim 6: Hamiltonian vector field

**Claim:** If X_H = J∇H, then q̇ = ∂H/∂p and ṗ = -∂H/∂q

**Computation:**
Let ∇H = (∂H/∂q₁, ∂H/∂q₂, ∂H/∂p₁, ∂H/∂p₂)

X_H = J∇H = J(∂H/∂q₁, ∂H/∂q₂, ∂H/∂p₁, ∂H/∂p₂)
           = (∂H/∂p₁, ∂H/∂p₂, -∂H/∂q₁, -∂H/∂q₂)

Writing X_H = (q̇₁, q̇₂, ṗ₁, ṗ₂), we get:
- q̇₁ = ∂H/∂p₁, q̇₂ = ∂H/∂p₂  ⟹  q̇ = ∂H/∂p
- ṗ₁ = -∂H/∂q₁, ṗ₂ = -∂H/∂q₂  ⟹  ṗ = -∂H/∂q

**Result:** **CORRECT**

---

## Claim 7: Reeb orbit on unit square

**Claim:** For unit square with J(q,p)=(p,-q) and p_i = 2Jn_i, the orbit is clockwise.

**Computation:**
In 2D, J(q,p) = (p,-q), so J = [[0,1],[-1,0]].

Facet normals and Reeb vectors:
1. **Right facet** (q=1): n = (1,0), so p = 2J(1,0) = 2(0,-1) = (0,-2)
   - Velocity points DOWN (p < 0)

2. **Top facet** (p=1): n = (0,1), so p = 2J(0,1) = 2(1,0) = (2,0)
   - Velocity points RIGHT (q > 0)

3. **Left facet** (q=-1): n = (-1,0), so p = 2J(-1,0) = 2(0,1) = (0,2)
   - Velocity points UP (p > 0)

4. **Bottom facet** (p=-1): n = (0,-1), so p = 2J(0,-1) = 2(-1,0) = (-2,0)
   - Velocity points LEFT (q < 0)

Orbit traced: (1,1) → (1,-1) → (-1,-1) → (-1,1) → (1,1)

This is **CLOCKWISE** when viewing the (q,p)-plane in standard orientation.

**Result:** **CORRECT**

---

## Claim 8: Action for 2-segment piecewise linear curve

**Claim:** For z with velocities w₁ on [0,T₁] and w₂ on [T₁,T], with z(0)=a and closure T₁w₁+T₂w₂=0,
then ∫₀ᵀ ⟨-Jż, z⟩dt = T₁T₂·ω(w₂,w₁).

**Computation:**
Since z is piecewise linear:
- z(t) = a + tw₁ for t ∈ [0,T₁]
- z(t) = a + T₁w₁ + (t-T₁)w₂ for t ∈ [T₁,T]

And ż = w₁ on [0,T₁], ż = w₂ on [T₁,T].

**First segment [0,T₁]:**
∫₀^{T₁} ⟨-Jw₁, a+tw₁⟩dt = ∫₀^{T₁} [⟨-Jw₁,a⟩ + t⟨-Jw₁,w₁⟩]dt

Since ω is skew-symmetric: ω(w₁,w₁) = ⟨-Jw₁,w₁⟩ = 0

= T₁⟨-Jw₁,a⟩

**Second segment [T₁,T]:**
z(t) = a + T₁w₁ + (t-T₁)w₂ = a - T₂w₂ + (t-T₁)w₂  [using closure: T₁w₁ = -T₂w₂]

∫_{T₁}^T ⟨-Jw₂, a-T₂w₂+(t-T₁)w₂⟩dt
= ∫_{T₁}^T [⟨-Jw₂,a⟩ - T₂⟨-Jw₂,w₂⟩ + (t-T₁)⟨-Jw₂,w₂⟩]dt
= ∫_{T₁}^T ⟨-Jw₂,a⟩dt  [since ⟨-Jw₂,w₂⟩ = 0]
= T₂⟨-Jw₂,a⟩

**Total:**
∫₀^T ⟨-Jż,z⟩dt = T₁⟨-Jw₁,a⟩ + T₂⟨-Jw₂,a⟩
                = T₁ω(w₁,a) + T₂ω(w₂,a)

Now, using closure T₁w₁ + T₂w₂ = 0:
ω(T₁w₁,a) + ω(T₂w₂,a) = ω(T₁w₁+T₂w₂,a) = ω(0,a) = 0

So: T₁ω(w₁,a) = -T₂ω(w₂,a)

Therefore:
∫₀^T ⟨-Jż,z⟩dt = T₁ω(w₁,a) + T₂ω(w₂,a)
                = T₁ω(w₁,a) - T₁ω(w₁,a)·(T₂/T₁)

Wait, let me recalculate more carefully.

Actually, using bilinearity and the closure constraint:
T₁ω(w₁,a) + T₂ω(w₂,a)
= T₁⟨-Jw₁,a⟩ + T₂⟨-Jw₂,a⟩
= ⟨-J(T₁w₁),a⟩ + ⟨-J(T₂w₂),a⟩
= ⟨-J(T₁w₁+T₂w₂),a⟩
= 0  [by closure]

Hmm, this gives 0, not T₁T₂·ω(w₂,w₁). Let me reconsider the calculation.

**Recomputing with correct expansion:**

For segment 1: z(t) = a + tw₁
∫₀^{T₁} ⟨-Jw₁, a+tw₁⟩dt = T₁⟨-Jw₁,a⟩ + ½T₁²⟨-Jw₁,w₁⟩ = T₁⟨-Jw₁,a⟩

For segment 2: z(t) = a + T₁w₁ + (t-T₁)w₂
∫_{T₁}^T ⟨-Jw₂, a+T₁w₁+(t-T₁)w₂⟩dt
= ∫₀^{T₂} ⟨-Jw₂, a+T₁w₁+sw₂⟩ds  [substituting s=t-T₁]
= T₂⟨-Jw₂,a⟩ + T₁T₂⟨-Jw₂,w₁⟩ + ½T₂²⟨-Jw₂,w₂⟩
= T₂⟨-Jw₂,a⟩ + T₁T₂ω(w₂,w₁)

**Total:**
∫₀^T ⟨-Jż,z⟩dt = T₁⟨-Jw₁,a⟩ + T₂⟨-Jw₂,a⟩ + T₁T₂ω(w₂,w₁)

The first two terms sum to zero by closure, leaving:
∫₀^T ⟨-Jż,z⟩dt = T₁T₂ω(w₂,w₁)

**Result:** **CORRECT**

---

## Claim 9: Relating 2A to action

**Claim:** With A = ½∫⟨Jγ,γ̇⟩, for 2 segments: 2A = T₂T₁ω(w₂,w₁).

**Computation:**
From Claim 5: ∫⟨Jγ,γ̇⟩ = -∫⟨Jγ̇,γ⟩ = ∫⟨-Jγ̇,γ⟩

Therefore: 2A = ∫⟨Jγ,γ̇⟩

From Claim 8: ∫⟨-Jż,z⟩ = T₁T₂ω(w₂,w₁)

So: 2A = T₁T₂ω(w₂,w₁) = T₂T₁ω(w₂,w₁)

**Result:** **CORRECT**

---

## Claim 10: Integrated Fenchel equality

**Claim:** For orbit γ on ∂K with g_K(γ)=1, the integrated Fenchel equality gives T + I_K(γ) = 2A(γ).
In normalization A=T, this gives I_K = T.

**Analysis:**
This claim involves the integrated Fenchel equality, which relates:
- T = period
- I_K(γ) = integral of second fundamental form
- A(γ) = action

The integrated Fenchel equality states: T + I_K(γ) = 2A(γ)

If we use the "talk normalization" where A=T, then:
T + I_K = 2T
⟹ I_K = T

The mathematical statement is internally consistent. Whether this normalization (A=T) is the one used in the codebase/thesis requires checking the actual definition of gauge function or Hamiltonian used.

**Result:** **CORRECT** (mathematically consistent; normalization choice needs verification against codebase)

---

## Summary

All 10 claims are **CORRECT** under the stated conventions.

**Note on Claim 10:** The formula is mathematically correct given the normalization A=T. This normalization corresponds to using gauge function g_K (with max value 1 on ∂K) as the Hamiltonian. If a different normalization is used elsewhere in the code (e.g., H = ½g_K²), the numerical factors would differ.
