use crate::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Id(pub usize);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Slot(pub usize);

#[derive(PartialEq, Eq, Clone)]
// a[x1, ..., xn]
pub struct AppliedId(pub Id, pub Box<[Slot]>);

mod fmt {
    use crate::*;
    use std::fmt::*;

    impl Display for Id {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "id{}", self.0) }
    }

    impl Display for Slot {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "${}", self.0) }
    }

    impl Display for AppliedId {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let AppliedId(x, args) = self;

            write!(f, "{x}")?;
            if args.is_empty() { return Ok(()) }

            write!(f, "[")?;
            for (i, a) in args.iter().enumerate() {
                write!(f, "{}", a)?;
                if i != args.len()-1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, "]")?;

            Ok(())
        }
    }

    // Debug
    impl Debug for Id { fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "{}", self) } }
    impl Debug for Slot { fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "{}", self) } }
    impl Debug for AppliedId { fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "{}", self) } }
}
