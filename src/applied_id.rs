use crate::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Id(pub usize);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Slot(pub usize);

#[derive(PartialEq, Eq, Clone)]
// a[x1, ..., xn]
pub struct AppliedId(pub Id, pub Box<[Slot]>);
