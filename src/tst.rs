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
    let aid = suf.alloc(3);
    let bid = suf.alloc(3);

    let a = RenamedId(Renaming::identity(3), aid);
    let b = RenamedId(Renaming::identity(3), bid);
    suf.union(a.clone(), b.clone());
    assert!(suf.is_equal(a.clone(), b.clone()));

    let x = RenamedId(Renaming(Box::new([Slot(30), Slot(31), Slot(32)])), aid);
    let x = suf.find(x);
    let y = RenamedId(Renaming(Box::new([Slot(30), Slot(31), Slot(32)])), bid);
    let y = suf.find(y);
    assert!(x.1 == y.1);
}
