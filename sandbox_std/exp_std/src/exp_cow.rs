//! `Cow` - это перечисление, которое расшифровывается как `Clone-on-Write`.
//! Его можно использовать в качестве интеллектуального указателя для работы с заимствованными
//! или владеющими значениями. Этот тип может помочь нам избежать ненужного выделения памяти.

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

    fn lower(s: &str) -> Cow<str> {
        if s.chars().any(char::is_uppercase) {
            Cow::Owned(s.to_lowercase())
        } else {
            Cow::Borrowed(s)
        }
    }
    let a = "hi";
    let b = lower(a);
    assert_eq!(b, "hi");
    assert_eq!(b.as_ptr(), a.as_ptr()); // Указатель на один и теже данные
    println!("a={a}; b={b}");

    let a = "Hi";
    let b = lower(a);
    assert_eq!(b, "hi");
    assert_ne!(b.as_ptr(), a.as_ptr()); // Указатель на разные данные
    println!("a={a}; b={b}");
}
