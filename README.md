## Dynamic Programming on Optimal FRI Split-and-Fold Strategy

The core of low-degree testing in FRI is to show that a polynomial $f(X)$ has a degree no more than $2^n$. This is done 
by showing that, after split-and-folding the polynomials by $k$ times, in which each time the degree of the polynomial 
goes down by half, the polynomial becomes a polynomial of degree no more than $2^{k-n}$.

**Split-and-fold.** Given a polynomial $f(X)$, the split-and-fold process first splits the polynomial, based on its 
coefficients, into the even terms and odd terms. This gives us two polynomials $g(X)$ and $h(X)$, the degree of which 
is the half of that of $f(X)$. In addition, $f(X), g(X), h(X)$ satisfy the following relationship.

$$f(X) = g(X^2) + X \cdot h(X^2)$$

Now, we want to combine the two polynomials, $g(X)$ and $h(X)$, into one polynomial. This is done by having the FRI verifier 
provides a challenge $\alpha$ and create the new polynomial $f'(X)$ as a linear combination of $g(X)$ and $h(X)$.

$$f'(X) = g(X) + \alpha \cdot h(X)$$

The new polynomial has degree that is $ceil(f(X) / 2)$. By repeating this process $k$ times, the polynomial is guaranteed 
to go down $2^k$ in the degree and one can then check if the polynomial is of a low degree.

**How is FRI being used**. Given a polynomial $a(X)$, if we want to show that $a(\beta) = y$, we just need to show that:

$$\frac{a(X) - y}{X - \beta}$$

is a low degree polynomial. In fact, its degree should be the degree of $a(X)$ minus one. If $a(\beta) \not= y$, this 
polynomial would not be a low-degree polynomial. 

### Why is there a strategy?

There are several ways to do split-and-fold, and these methods have different costs when it comes to the number of witness 
stack it uses and the number of weight units it contributes.

- **Standard split-and-fold:** The split-and-fold is exactly as the one shown above. It requires 2 hint elements for 
deriving $\alpha$, 1 hint element for the sibling (per query point), and log(n/2) hint elements (depending on the Merkle tree arity) for Merkle 
tree (per query point). At the same time, it needs to perform one qm31 multiplication (between $\alpha$ and $h(X)$) on the queried point. 

   $$f(X) = g(X^2) + X \cdot h(X^2)$$

- **Double split-and-fold:** It now performs two split-and-fold using the same $\alpha$. This is by splitting the 
polynomial $f(X)$ into four, say $g_1(X), g_2(X), g_3(X), g_4(X)$:

   $$f(X) = g_1(X^4) + X\cdot g_2(X^4) + X^2 \cdot g_3(X^4) + X^3 \cdot g_4(X^4)$$ 

   One can see that the four polynomials here, $g_1(X), g_2(X), g_3(X), g_4(X)$, are of degree one quarter. This requires 2 
hint elements for deriving $\alpha$, 3 hint elements for the siblings (per query point), and $log(n/4)$ hint elements for 
Merkle tree (per query point). At the same time, it needs to perform 3 qm31 multiplications per query point and one qm31 
multiplication to compute $\alpha^2$ from $\alpha$. Although such a two-time split-and-fold is much more expensive, one 
needs to know that the total number of split-and-fold is also lower, and sometimes this can end up being more beneficial.

- **Triple split-and-fold:** Similarly, one can perform three split-and-fold using the same $\alpha$. Here, the polynomial 
$f(X)$ is being split into 8 polynomials, $g_1(X), g_2(X), g_3(X), g_4(X), g_5(X), g_6(X), g_7(X), g_8(X)$:

  $$f(X) = \sum_{i=1}^8 X^{i-1}\cdot g_{i}(X^8)$$

  Here, it requires 2 hint elements for deriving $\alpha$, 7 hint elements for the siblings (per query point), and $log(n/8)$
hint elements for Merkle tree (per query point). At the same time, it needs to perform 7 qm31 multiplications per query 
point in addition to 2 qm31 multiplications to compute $\alpha^2$ and $\alpha^4$ from $\alpha$. This can reduce the total 
number of split-and-fold further, but at the cost of more computation.

- **Quadruple split-and-fold:** There is some situation where the DP algorithm will suggest a more aggressive version, which 
splits the polynomial $f(X)$ into 16 polynomials $g_1(X), g_2(X), ..., g_{16}(X)$.

   $$f(X) = \sum_{i=1}^{16} X^{i-1}\cdot g_{i}(X^{16})$$

   This still requires 2 hint elements for deriving $\alpha$, 15 hint elements for the siblings (per query point), and 
   $log(n/16)$ hint elements for Merkle tree (per query point). Regarding the number of qm31 multiplications, 
   there are 15 qm31 multiplications per query point in addition to 3 qm31 multiplications to compute $\alpha^2$, $\alpha^4$,
   and $\alpha^8$. This is a more extreme direction of the input stack size vs the qm31 multiplication tradeoff.

### What about the cost of Merkle tree?

There is a lot of flexibility in Merkle tree. Here we present a few talking points.

**Arity.** If we assume the standard relay policy that limits the hint element to be no more than 80 bytes, 
then we can use a binary tree or a 3-ary tree. There is, however, no benefit in terms of the number of hint elements 
to use a 3-ary tree. The binary tree uses 1 hint element for sibling, and the 3-ary tree uses 2 hint elements for siblings. 
This is not better than doing the binary tree twice.

However, if we forgo the standard relay policy, the current upper bound for OP_CAT output is 520 bytes, which allows us 
to squeeze in 16 hashes. This allows us to do a 16-ary tree, and for each level, we use 2 hint elements for siblings. This 
can roughly reduce the number of hint elements used in Merkle tree by half. This is an example where the transaction being 
nonstandard (in respect to the relay policy) can bring actual benefits.

**Tree-top optimization.** Sometimes, it is useful to remove the top parts of the Merkle tree, so that the new root is no 
longer a single element, but $2^l$ elements. When we verify the Merkle tree path, we check against one of the $2^l$ elements. 
This approach is useful if a single Merkle tree is queried for many times and for a binary Merkle tree. For a binary Merkle 
tree of size 20, setting $l = 4$ meaning that the new root is of 16 elements (15 more elements than the old root). This 
allows the 5 queries to the Merkle tree to reduce 4 levels, resulting in the save of 4 hint elements each and 20 hint elements 
in total. However, one can see that if we only have 5 queries, doing so may be unnecessary.

The benefit seems to completely disappear when we are using nonstandard transaction, since with a 16-ary tree, any tree-top 
optimization seems not better than using just one more layer in the 16-ary tree. For the reasons discussed here, this DP
algorithm does not consider tree-top optimization.

### Dynamic Programming (DP)

The code in the repository wants to do the following:

- Given a limit of the hint elements $N_{h}$ and a limit of qm31 multiplications $N_{m}$,
- Given a **linear** goal functon, such as $g(h, m) = h$ (if we solely want to optimize for fewer hint elements), or $g(h, m) = m$
  (if we solely want to optimize for fewer qm31 multiplications),
- Find the reduction strategy (which is a reduction path from $n$ to $1$, with the freedom of choosing one of the split-and-fold 
  methods for each step of the reduction) that stays within the limits $N_{h}$ and $N_{m}$ and **minimizes** the goal function.

This is done as follows:

- Start with the main problem $(n, N_{h}, N_{m})$.
- Try all the reduction strategies and reduce the current problem to a subproblem. For example, if applying a reduction 
  strategy that lowers $t$ layers at the cost of $a$ hint elements and $b$ qm31 multiplications incurring a cost of $g(a, b)$, 
  we figure out the cost of the subproblem $(n - t, N_{h} - a, N_{m} - b)$ in a similar way, add this reduction's cost 
  $g(a, b)$, and pick the one with the **lowest** cost (i.e., minimizing the goal function).

This can be expressed as follows.

$$G(n, N_{h}, N_{m}) = min(g_1, g_2, g_3, g_4)$$

- $g_1 = G(n - 1, N_{h} - 2 - (1 + log(n/2))\cdot q, N_{m} - q) + g(2 + (1 + log(n/2))\cdot q, q)$
- $g_2 = G(n - 2, N_{h} - 2 - (3 + log(n/4))\cdot q, N_{m} - 3\cdot q - 1) + g(2 + (3 + log(n/4))\cdot q, 3\cdot q + 1)$
- $g_3 = G(n - 3, N_{h} - 2 - (7 + log(n/8))\cdot q, N_{m} - 7\cdot q - 2) + g(2 + (7 + log(n/8))\cdot q, 7\cdot q + 2)$
- $g_4 = G(n - 4, N_{h} - 2 - (15 + log(n/16))\cdot q, N_{m} - 15\cdot q - 3) + g(2 + (15 + log(n/16))\cdot q, 15\cdot q + 3)$

with the following to make sure that the limits are observed and the algorithm terminates, respectively.
- $G(\cdot, x, y) = +\infty$ if $x < 0$ or $y < 0$
- $G(0, x, y) = 0$ for $x \geq 0$ and $y \geq 0$ 

**How to compute.** We use the standard dynamic programming (DP) method to figure out the minimal cost and the corresponding
strategy. This is in essence a depth-first search (DFS) with memoization (aka the top-down approach). Since every step 
it is guaranteed to at least go down one level, the depth is at most $log_{2} (n)$, and it does not seem special programming 
trick to avoid stack overflow.

## License 

This repository is intended to be public good. It is under the MIT license.

The flexibility and usage of different split-and-fold strategies come from a discussion with Shahar Papini from Starkware.