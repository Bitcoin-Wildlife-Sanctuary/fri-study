## Dynamic Programming on Optimal FRI Split-and-Fold Strategy

The core of low-degree testing in FRI is to show that a polynomial $f(X)$ has a degree no more than $2^n$. This is done 
by showing that, after split-and-folding the polynomials by $k$ times, in which each time the degree of the polynomial 
goes down by half, the polynomial becomes a polynomial of degree no more than $2^{k-n}$.

**Split-and-fold.** Given a polynomial $f(X)$, the split-and-fold process first splits the polynomial, based on its 
coefficients, into the even terms and odd terms. This gives us two polynomials $g(X)$ and $h(X)$, the degree of which 
is the half of that of $f(X)$. In addition, $f(X), g(X), h(X)$ satisfy the following relationship.

$$f(X) = g(X^2) + X \cdot h(X^2)$$



### Other strategies to control the number of hints and the number of 