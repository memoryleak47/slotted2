mod renaming;
pub use renaming::*;

mod tst;
pub use tst::*;

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
            let RenamedId(mb, b) = &self.classes[a.0].leader;
            if a == *b { return RenamedId(m, a) }

            // a :: A
            // m :: A -> P
            // b :: B
            // mb :: B -> A

            // mab :: B -> P
            let mab = Renaming(mb.0.iter().map(|x| m.0[x.0]).collect());
            RenamedId(m, a) = RenamedId(mab, *b);
        }
    }

    fn union(&mut self, x: RenamedId, y: RenamedId) {
        let RenamedId(mut mx, mut x) = self.find(x);
        let RenamedId(mut my, mut y) = self.find(y);

        loop {
            let mut changed = false;
            for a in mx.0.iter() {
                if !my.0.contains(&a) { self.drop_slot(RenamedId(mx.clone(), x), *a); changed = true; }
            }
            for a in my.0.iter() {
                if !mx.0.contains(&a) { self.drop_slot(RenamedId(my.clone(), y), *a); changed = true; }
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
        assert!(x.0.0.contains(&s));

        let x = self.find(x);
        let Some(p) = x.0.0.iter().position(|a| *a == s) else { return /*already dropped in the past*/ };
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

fn main() {}
