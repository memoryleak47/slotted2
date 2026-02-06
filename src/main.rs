mod renaming;
pub use renaming::*;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Id(usize);

// expresses m*a, where
// - a :: X
// - m :: X -> Y
// - forall x :: X, m[x] :: Y
#[derive(PartialEq, Eq, Clone)]
struct RenamedId(/*m*/Renaming, /*a*/Id);

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
