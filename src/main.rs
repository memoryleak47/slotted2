#[derive(Clone, Copy)]
struct Id(usize);

#[derive(Clone, Copy)]
struct Slot(usize);

struct Renaming(Box<[Slot]>);
struct RenamedId(Renaming, Id); // m*a

impl Renaming {
    pub fn identity(arity: usize) -> Self {
        Self(
            (0..arity).map(Slot).collect()
        )
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

    fn union(&mut self, x: RenamedId, y: RenamedId) {
        todo!()
    }

    fn is_equal(&self, x: RenamedId, y: RenamedId) -> bool {
        todo!()
    }
}

fn main() {}
