struct Id(usize);
struct Slot(usize);

struct Renaming(Box<[Slot]>);
struct RenamedId(Renaming, Id); // m*a

struct SlottedUF {
}

impl SlottedUF {
    fn alloc(&mut self, arity: usize) -> Id {
        todo!()
    }

    fn union(&mut self, x: RenamedId, y: RenamedId) {
        todo!()
    }

    fn is_equal(&self, x: RenamedId, y: RenamedId) -> bool {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
