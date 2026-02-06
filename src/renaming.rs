use crate::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Slot(pub usize);

#[derive(PartialEq, Eq, Clone)]
pub struct Renaming(pub Box<[Slot]>);

impl Renaming {
    pub fn identity(arity: usize) -> Self {
        Self(
            (0..arity).map(Slot).collect()
        )
    }

    pub fn rev(&self) -> Renaming {
        let mut out = Renaming::identity(self.0.len());
        for (x, y) in self.iter() {
            out.0[y.0] = x;
        }
        out
    }

    // m1⁻¹ * m2
    // (y, x) in m1 && (y, z) in m2 -> (x, z) in m1⁻¹ * m2
    pub fn revcompose(&self, m2: &Renaming) -> Renaming {
        let mut out = Renaming::identity(self.0.len());
        for (x, z) in self.0.iter().zip(m2.0.iter()) {
            out.0[x.0] = *z;
        }
        out
    }

    pub fn iter(&self) -> impl Iterator<Item=(Slot, Slot)> {
        self.0.iter().enumerate().map(|(i, x)| (Slot(i), *x))
    }
}
