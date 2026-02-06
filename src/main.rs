mod applied_id;
pub use applied_id::*;

mod tst;
pub use tst::*;

struct Class {
    group: (),
    arity: usize,
    leader: AppliedId, // AppliedId equivalent to this Id.
}

struct SlottedUF {
    classes: Vec<Class>,
}

impl SlottedUF {
    fn new() -> Self { Self { classes: Vec::new() } }

    fn alloc(&mut self, arity: usize) -> Id {
        let m = (0..arity).map(Slot).collect();
        let i = Id(self.classes.len());
        self.classes.push(Class {
            group: (),
            arity,
            leader: AppliedId(i, m),
        });
        i
    }

    fn find(&self, AppliedId(mut a, mut args_a): AppliedId) -> AppliedId {
        loop {
            let AppliedId(b, args_b) = &self.classes[a.0].leader;
            if a == *b { return AppliedId(a, args_a) }

            let args_ab = args_b.iter().map(|x| args_a[x.0]).collect();
            AppliedId(a, args_a) = AppliedId(*b, args_ab);
        }
    }

    fn union(&mut self, x: AppliedId, y: AppliedId) {
        let AppliedId(mut x, mut args_x) = self.find(x);
        let AppliedId(mut y, mut args_y) = self.find(y);

        loop {
            let mut changed = false;
            for a in args_x.iter() {
                if !args_y.contains(&a) { self.drop_slot(AppliedId(x, args_x.clone()), *a); changed = true; }
            }
            for a in args_y.iter() {
                if !args_x.contains(&a) { self.drop_slot(AppliedId(y, args_y.clone()), *a); changed = true; }
            }
            if !changed { break }
            AppliedId(x, args_x) = self.find(AppliedId(x, args_x));
            AppliedId(y, args_y) = self.find(AppliedId(y, args_y));
        }

        if x == y {
            if args_x == args_y { return }
            else { panic!("symmetries unsupported!") }
        } else {
            let y_arity = self.classes[y.0].arity;

            let mut out: Box<[Slot]> = (0..y_arity).map(Slot).collect();
            for i in 0..y_arity {
                let aa = args_y[i];
                // TODO is there a more efficient way?
                let aa = Slot(args_x.iter().position(|j| *j == aa).unwrap());
                out[i] = aa;
            }
            self.classes[x.0].leader = AppliedId(y, out);
        }
    }

    fn drop_slot(&mut self, AppliedId(x, args_x): AppliedId, s: Slot) {
        assert!(args_x.contains(&s));

        let AppliedId(x, args_x) = self.find(AppliedId(x, args_x));
        let Some(p) = args_x.iter().position(|a| *a == s) else { return /*already dropped in the past*/ };
        self.drop_leader_slot(x, Slot(p));
    }

    fn drop_leader_slot(&mut self, x: Id, s: Slot) {
        let arity = self.classes[x.0].arity;
        let new = self.alloc(arity - 1);
        let args = (0..s.0).chain((s.0 + 1)..arity).map(Slot).collect();
        self.classes[x.0].leader = AppliedId(new, args);
    }

    fn is_equal(&self, x: AppliedId, y: AppliedId) -> bool {
        // NOTE incomplete due to groups!
        let x = self.find(x);
        let y = self.find(y);
        x == y
    }
}

fn main() {}
