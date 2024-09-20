pub struct Counter {
    n: usize,
}

impl Counter {
    pub fn new() -> Self {
        Self { n: 1 }
    }

    pub fn next(&mut self) -> usize {
        // `usize` implements Copy so we can store the "old" count in a variable
        // such that it is not affected by the AddAssign operation below.
        let n = self.n;

        self.n += 1;

        n
    }
}
