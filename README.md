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

- **Split-and-fold one by one (default):** The split-and-fold is exactly as the one shown above. It requires 2 hint elements for 
deriving $\alpha$, 1 hint element for the sibling (per query point), and log(n/2) hint elements (depending on the Merkle tree arity) for Merkle 
tree (per query point). At the same time, it needs to perform one qm31 multiplication (between $\alpha$ and $h(X)$) on the queried point. 

$$f(X) = g(X^2) + X \cdot h(X^2)$$

- **Split-and-fold two by two:** It now performs two split-and-fold using the same $\alpha$. This is by splitting the 
polynomial $f(X)$ into four, say $g_1(X), g_2(X), g_3(X), g_4(X)$:

$$f(X) = g_1(X^4) + X\cdot g_2(X^4) + X^2 \cdot g_3(X^4) + X^4 \cdot g_4(X^4)$$



### Other strategies to control the number of hints and the number of 