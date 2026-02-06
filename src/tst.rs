use crate::*;

#[test]
fn t1() {
    let mut suf = SlottedUF::new();
    let aid = suf.alloc(3);
    let l = RenamedId(Renaming(Box::new([Slot(10), Slot(11), Slot(12)])), aid);
    let r = RenamedId(Renaming(Box::new([Slot(110), Slot(11), Slot(112)])), aid);
    suf.union(l, r);

    let z = RenamedId(Renaming(Box::new([Slot(20), Slot(21), Slot(22)])), aid);
    let RenamedId(m, _) = suf.find(z);
    assert!(&*m.0 == [Slot(21)]);
}

#[test]
fn t2() {
    let mut suf = SlottedUF::new();
    let aid = suf.alloc(2);
    let bid = suf.alloc(2);

    let a = RenamedId(Renaming(Box::new([Slot(300), Slot(400)])), aid);
    let b = RenamedId(Renaming(Box::new([Slot(400), Slot(300)])), bid);
    suf.union(a, b);

    let c = RenamedId(Renaming(Box::new([Slot(500), Slot(600)])), aid);
    let d = RenamedId(Renaming(Box::new([Slot(600), Slot(500)])), bid);
    assert!(suf.is_equal(c, d));
}
