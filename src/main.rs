#[derive(Clone, Copy, PartialEq, Eq)]
struct Id(usize);

#[derive(Clone, Copy, PartialEq, Eq)]
struct Slot(usize);

#[derive(PartialEq, Eq)]
struct Renaming(Box<[Slot]>);

impl Renaming {
    // self * m2
    // (self*m2)[x] = self[m2[x]]
    fn compose(&self, m2: &Renaming) -> Renaming {
        todo!()
    }
}

// expresses m*a, where
// - a :: X
// - m :: X -> Y
// - forall x :: X, m[x] :: Y
#[derive(PartialEq, Eq)]
struct RenamedId(/*m*/Renaming, /*a*/Id);

impl Renaming {
    pub fn identity(arity: usize) -> Self {
        Self(
            (0..arity).map(Slot).collect()
        )
    }

    // m1 * (m2 * x)
    // is (m1 * m2) * x
    pub fn mul(&self, RenamedId(m2, x): &RenamedId) -> RenamedId {
        let m1 = self;
        todo!()
    }
}

struct Class {
    group: (),
    arity: usize,
    leader: RenamedId, // equivalent to this Id.
}

struct SlottedUF {
    classes: Vec<Class>,
}

impl SlottedUF {
    fn alloc(&mut self, arity: usize) -> Id {
        let m = Renaming::identity(arity);
        let i = Id(self.classes.len());
        self.classes.push(Class {
            group: (),
            arity,
            leader: RenamedId(m, i),
        });
        i
    }

    fn find(&self, RenamedId(mut m, mut a): RenamedId) -> RenamedId {
        loop {
            let RenamedId(m2, a2) = m.mul(&self.classes[a.0].leader);
            if a == a2 { return RenamedId(m, a) }
            RenamedId(m, a) = RenamedId(m2, a2);
        }
    }

    fn union(&mut self, x: RenamedId, y: RenamedId) {
        let x = self.find(x);
        let y = self.find(y);
        todo!()
    }

    fn is_equal(&self, x: RenamedId, y: RenamedId) -> bool {
        // NOTE incomplete due to groups!
        let x = self.find(x);
        let y = self.find(y);
        x == y
    }
}

fn main() {}
