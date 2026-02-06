use crate::*;

#[test]
fn t1() {
    let mut suf = SlottedUF::new();
    let aid = suf.alloc(3);
    let l = RenamedId(Renaming(Box::new([Slot(30), Slot(31), Slot(32)])), aid);
    let r = RenamedId(Renaming(Box::new([Slot(12), Slot(31), Slot(42)])), aid);
    suf.union(l, r);

    let z = RenamedId(Renaming(Box::new([Slot(30), Slot(31), Slot(32)])), aid);
    let RenamedId(m, _) = suf.find(z);
    assert!(m.0.len() == 1);
}

#[test]
fn t2() {
    let mut suf = SlottedUF::new();
    let aid = suf.alloc(3);
    let bid = suf.alloc(3);

    let a = RenamedId(Renaming::identity(3), aid);
    let b = RenamedId(Renaming::identity(3), bid);
    suf.union(a.clone(), b.clone());
    println!("{}", suf.is_equal(a.clone(), b.clone()));
    let a = RenamedId(Renaming(Box::new([Slot(30), Slot(31), Slot(32)])), aid);
    let a = suf.find(a);
}

