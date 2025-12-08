pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub(crate) fn new(n: usize) -> Self {
        let mut parent: Vec<usize> = Vec::with_capacity(n);

        for i in 0..n {
            parent.push(i);
        }
        Self {
            parent,
            size: vec![1; n],
        }
    }

    pub(crate) fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub(crate) fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);

        if ra == rb {
            return false;
        }

        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }

        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];

        true
    }

    pub(crate) fn all_sizes(&mut self) -> Vec<usize> {
        let mut sizes: Vec<usize> = Vec::new();
        let mut seen: Vec<bool> = vec![false; self.parent.len()];

        for i in 0..self.parent.len() {
            let root: usize = self.find(i);
            if !seen[root] {
                sizes.push(self.size[root]);
                seen[root] = true;
            }
        }

        sizes
    }
}
