#[derive(Clone, Copy, PartialEq, Eq)]
struct Id(usize);

#[derive(Clone, Copy, PartialEq, Eq)]
struct Slot(usize);

#[derive(PartialEq, Eq, Clone)]
struct Renaming(Box<[Slot]>);

impl Renaming {
    // m1 :: X -> Y
    // m2 :: Y -> Z
    // m1*m2 :: X -> Z
    fn compose(&self, m2: &Renaming) -> Renaming {
        let m1 = self;
        let b = m1.0.iter().map(|y| m2.0[y.0]).collect();
        Renaming(b)
    }

    fn rev(&self) -> Renaming {
        let mut out = Renaming::identity(self.0.len());
        for (x, y) in self.iter() {
            out.0[y.0] = x;
        }
        out
    }

    // m1⁻¹ * m2
    // (y, x) in m1 && (y, z) in m2 -> (x, z) in m1⁻¹ * m2
    fn revcompose(&self, m2: &Renaming) -> Renaming {
        let mut out = Renaming::identity(self.0.len());
        for (x, z) in self.0.iter().zip(m2.0.iter()) {
            out.0[x.0] = *z;
        }
        out
    }

    fn iter(&self) -> impl Iterator<Item=(Slot, Slot)> {
        self.0.iter().enumerate().map(|(i, x)| (Slot(i), *x))
    }
}

// expresses m*a, where
// - a :: X
// - m :: X -> Y
// - forall x :: X, m[x] :: Y
#[derive(PartialEq, Eq, Clone)]
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
        RenamedId(m1.compose(m2), *x)
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
    fn new() -> Self { Self { classes: Vec::new() } }
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
        let RenamedId(mx, x) = self.find(x);
        let RenamedId(my, y) = self.find(y);

        if x == y {
            if mx == my { return }
            else { panic!("symmetries unsupported!") }
        } else {
            // mx * x = my * y
            // -> x = mx⁻¹ * my * y
            self.classes[x.0].leader = RenamedId(mx.revcompose(&my), y);
        }
    }

    fn is_equal(&self, x: RenamedId, y: RenamedId) -> bool {
        // NOTE incomplete due to groups!
        let x = self.find(x);
        let y = self.find(y);
        x == y
    }
}

fn main() {
    let mut suf = SlottedUF::new();
    let a = suf.alloc(3);
    let b = suf.alloc(3);

    let a = RenamedId(Renaming::identity(3), a);
    let b = RenamedId(Renaming::identity(3), b);
    suf.union(a.clone(), b.clone());
    println!("{}", suf.is_equal(a.clone(), b.clone()));
}
