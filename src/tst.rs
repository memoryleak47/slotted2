use crate::*;

#[test]
fn t1() {
    let mut suf = SlottedUF::new();
    let aid = suf.alloc(3);
    let l = AppliedId::mk(aid, [Slot(10), Slot(11), Slot(12)]);
    let r = AppliedId::mk(aid, [Slot(110), Slot(11), Slot(112)]);
    suf.union(l, r);

    let z = AppliedId::mk(aid, [Slot(20), Slot(21), Slot(22)]);
    let z = suf.find(z);
    assert!(&*z.args == [Slot(21)]);
}

#[test]
fn t2() {
    let mut suf = SlottedUF::new();
    let aid = suf.alloc(2);
    let bid = suf.alloc(2);

    let a = AppliedId::mk(aid, [Slot(300), Slot(400)]);
    let b = AppliedId::mk(bid, [Slot(400), Slot(300)]);
    suf.union(a, b);

    let c = AppliedId::mk(aid, [Slot(500), Slot(600)]);
    let d = AppliedId::mk(bid, [Slot(600), Slot(500)]);
    assert!(suf.is_equal(c, d));

    let e = AppliedId::mk(aid, [Slot(1000), Slot(2000)]);
    let f = AppliedId::mk(bid, [Slot(1000), Slot(2000)]);
    assert!(!suf.is_equal(e, f));
}
