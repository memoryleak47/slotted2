use crate::*;

struct Class {
    group: (),
    arity: usize,
    leader: AppliedId, // AppliedId equivalent to this Id.
}

pub struct SlottedUF {
    classes: Vec<Class>,
}

impl SlottedUF {
    pub fn new() -> Self { Self { classes: Vec::new() } }

    pub fn alloc(&mut self, arity: usize) -> Id {
        let args = (0..arity).map(Slot).collect();
        let id = Id(self.classes.len());
        self.classes.push(Class {
            group: (),
            arity,
            leader: AppliedId { id, args },
        });
        id
    }

    pub fn find(&self, mut a: AppliedId) -> AppliedId {
        loop {
            let b = &self.classes[a.id.0].leader;
            if a.id == b.id { return a }

            let args = b.args.iter().map(|x| a.args[x.0]).collect();
            a = AppliedId { id: b.id, args };
        }
    }

    pub fn union(&mut self, x: AppliedId, y: AppliedId) {
        let mut x = self.find(x);
        let mut y = self.find(y);

        loop {
            let mut changed = false;
            for a in x.args.iter() {
                if !y.args.contains(&a) { self.drop_slot(x.clone(), *a); changed = true; }
            }
            for a in y.args.iter() {
                if !x.args.contains(&a) { self.drop_slot(y.clone(), *a); changed = true; }
            }
            if !changed { break }
            x = self.find(x);
            y = self.find(y);
        }

        if x.id == y.id {
            if x.args == y.args { return }
            else { panic!("symmetries unsupported!") }
        } else {
            let y_arity = self.classes[y.id.0].arity;

            let mut out: Box<[Slot]> = (0..y_arity).map(Slot).collect();
            for i in 0..y_arity {
                let aa = y.args[i];
                // TODO is there a more efficient way?
                let aa = Slot(x.args.iter().position(|j| *j == aa).unwrap());
                out[i] = aa;
            }
            self.classes[x.id.0].leader = AppliedId { id: y.id, args: out.into() };
        }
    }

    pub fn drop_slot(&mut self, x: AppliedId, s: Slot) {
        assert!(x.args.contains(&s));

        let x = self.find(x);
        let Some(p) = x.args.iter().position(|a| *a == s) else { return /*already dropped in the past*/ };
        self.drop_leader_slot(x.id, Slot(p));
    }

    pub fn drop_leader_slot(&mut self, x: Id, s: Slot) {
        let arity = self.classes[x.0].arity;
        let new = self.alloc(arity - 1);
        let args = (0..s.0).chain((s.0 + 1)..arity).map(Slot).collect();
        self.classes[x.0].leader = AppliedId { id: new, args };
    }

    pub fn is_equal(&self, x: AppliedId, y: AppliedId) -> bool {
        // NOTE incomplete due to groups!
        let x = self.find(x);
        let y = self.find(y);
        x == y
    }
}
