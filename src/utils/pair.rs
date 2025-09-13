use std::fmt;

/// A very ordinary pair structure.
///
/// This structure is meant to be used exactly like a 2-values tuple. We
/// decide to create a structure for it, although at the cost of very
/// little performance, to have better control over the data we manipulate
/// and, for example, implement dedicated methods, formatting methods... for
/// them.
#[derive(Debug)]
pub struct Pair<A, B = A>(A, B);

impl<A, B> Pair<A, B> {
    /// Return a new Pair with the two given values.
    pub fn new(first: A, second: B) -> Self {
        return Self(first, second);
    }

    /// Return the first value of the pair.
    ///
    /// Return **exactly** a reference to `self.0`.
    pub fn first(&self) -> &A {
        &self.0
    }

    /// Return the second value of the pair.
    ///
    /// Return **exactly** a reference to `pair.1`.
    pub fn second(&self) -> &B {
        &self.1
    }
}

/// Custom printing method for a Pair of values.
impl<A: fmt::Display, B: fmt::Display> fmt::Display for Pair<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})", self.first(), self.second())
    }
}
