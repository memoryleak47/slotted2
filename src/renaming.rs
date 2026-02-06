use crate::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

    pub fn iter(&self) -> impl Iterator<Item=(Slot, Slot)> {
        self.0.iter().enumerate().map(|(i, x)| (Slot(i), *x))
    }
}
