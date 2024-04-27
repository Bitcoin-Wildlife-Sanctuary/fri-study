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

is a low degree polynomial. In fact, its degree should be the degree of $a(X)$ minus one. If $a(\beta) \not= y$

### Other strategies to control the number of hints and the number of 