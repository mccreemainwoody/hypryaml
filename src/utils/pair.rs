use std::fmt;

#[derive(Debug)]
pub struct Pair<A, B = A>(A, B);

impl<A, B> Pair<A, B> {
    pub fn new(first: A, second: B) -> Self {
        return Self(first, second);
    }

    pub fn first(&self) -> &A {
        &self.0
    }

    pub fn second(&self) -> &B {
        &self.1
    }
}

impl<A: fmt::Display, B: fmt::Display> fmt::Display for Pair<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})", self.first(), self.second())
    }
}
