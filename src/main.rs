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
        let RenamedId(mut mx, mut x) = self.find(x);
        let RenamedId(mut my, mut y) = self.find(y);

        loop {
            let mut changed = false;
            for a in mx.0.iter() {
                if !my.0.contains(&a) { self.drop_slot(RenamedId(my.clone(), y), *a); changed = true; }
            }
            for a in my.0.iter() {
                if !mx.0.contains(&a) { self.drop_slot(RenamedId(mx.clone(), x), *a); changed = true; }
            }
            if !changed { break }
            RenamedId(mx, x) = self.find(RenamedId(mx, x));
            RenamedId(my, y) = self.find(RenamedId(my, y));
        }

        if x == y {
            if mx == my { return }
            else { panic!("symmetries unsupported!") }
        } else {
            // mx * x = my * y
            // -> x = mx⁻¹ * my * y
            self.classes[x.0].leader = RenamedId(mx.revcompose(&my), y);
        }
    }

    fn drop_slot(&mut self, x: RenamedId, s: Slot) {
        let x = self.find(x);
        let p = x.0.0.iter().position(|a| *a == s).unwrap();
        self.drop_leader_slot(x.1, Slot(p));
    }

    fn drop_leader_slot(&mut self, x: Id, s: Slot) {
        let arity = self.classes[x.0].arity;
        let new = self.alloc(arity - 1);
        let m = Renaming((0..s.0).chain((s.0 + 1)..arity).map(Slot).collect());
        self.classes[x.0].leader = RenamedId(m, new);
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
