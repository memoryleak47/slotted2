use crate::*;

#[test]
fn t1() {
    let mut suf = SlottedUF::new();
    let aid = suf.alloc(3);
    let l = AppliedId { id: aid, args: Box::new([Slot(10), Slot(11), Slot(12)]) };
    let r = AppliedId { id: aid, args: Box::new([Slot(110), Slot(11), Slot(112)]) };
    suf.union(l, r);

    let z = AppliedId { id: aid, args: Box::new([Slot(20), Slot(21), Slot(22)]) };
    let z = suf.find(z);
    assert!(&*z.args == [Slot(21)]);
}

#[test]
fn t2() {
    let mut suf = SlottedUF::new();
    let aid = suf.alloc(2);
    let bid = suf.alloc(2);

    let a = AppliedId { id: aid, args: Box::new([Slot(300), Slot(400)]) };
    let b = AppliedId { id: bid, args: Box::new([Slot(400), Slot(300)]) };
    suf.union(a, b);

    let c = AppliedId { id: aid, args: Box::new([Slot(500), Slot(600)]) };
    let d = AppliedId { id: bid, args: Box::new([Slot(600), Slot(500)]) };
    assert!(suf.is_equal(c, d));

    let e = AppliedId { id: aid, args: Box::new([Slot(1000), Slot(2000)]) };
    let f = AppliedId { id: bid, args: Box::new([Slot(1000), Slot(2000)]) };
    assert!(!suf.is_equal(e, f));
}
