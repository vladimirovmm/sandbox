#![cfg(test)]

use std::borrow::Cow;

//
#[test]
fn test() {
    let a = ['h', 'i'];
    let b = Cow::from(&a[..]);
    assert_eq!(a.as_ptr(), b.as_ptr());

    let b;
    {
        let a = "hi";
        b = Cow::from(a);
        assert_eq!(a.as_ptr(), b.as_ptr());
    }
    assert_eq!(b, "hi");

    let mut b;
    {
        let a = "hi".to_string() + "!";
        b = Cow::Owned(a);
    }
    assert_eq!(b, "hi!");
    let c = b.to_mut(); // return String
    c.push('!');
    assert_eq!(c, "hi!!");
    assert_eq!(b, "hi!!");
    // assert_eq!(c, "hi!!");// panic
}
