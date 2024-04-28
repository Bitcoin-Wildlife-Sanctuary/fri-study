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

  $$f(X) = g_1(X^8) + X\cdot g_2(X^8) + X^2\cdot g_3(X^8) + X^3 \cdot g_4(X^8) + X^4 \cdot g_5(X^8) + X^5 \cdot g_6(X^8) + X^6\cdot g_7(X^8) + X^7\cdot g_8(X^8)$$

  Here, it requires 2 hint elements for deriving $\alpha$, 7 hint elements for the siblings (per query point), and $log(n/8)$
hint elements for Merkle tree (per query point). At the same time, it needs to perform 7 qm31 multiplications per query 
point in addition to 2 qm31 multiplications to compute $\alpha^2$ and $\alpha^4$ from $\alpha$. This can reduce the total 
number of split-and-fold further, but at the cost of more computation.

- **Quadruple split-and-fold:** There is some situation where the DP algorithm will suggest a more aggressive version, which 
splits the polynomial $f(X)$ into 16 polynomials $g_1(X), g_2(X), ..., g_{16}(X)$.

   $$f(X) = \sum_{i=1}^{16} X^{i-1} g_{i}(X)$$

   This still requires 2 hint elements for deriving $\alpha$, 15 hint elements for the siblings (per query point), and 
   $log(n/16)$ hint elements for Merkle tree (per query point). Regarding the number of qm31 multiplications, 
   there are 15 qm31 multiplications per query point in addition to 3 qm31 multiplications to compute $\alpha^2$, $\alpha^4$,
   and $\alpha^8$. This is a more extreme direction of the input stack size vs the qm31 multiplication tradeoff.

### Other strategies to control the number of hints and the number of 