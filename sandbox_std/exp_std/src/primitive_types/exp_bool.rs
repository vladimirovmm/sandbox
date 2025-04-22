#![cfg(test)]

#[test]
#[allow(clippy::assertions_on_constants, clippy::nonminimal_bool)]
fn test_operators() {
    assert!(true & true);
    assert!(true && true);
    assert!(!(true & false));

    assert!(true | false);
    assert!(true || false);
    assert!(!(false | false));

    assert!(!(true ^ true));
    assert!(false ^ true);
    assert!(true ^ false);
    assert!(!(false ^ false));
}

#[test]
fn test_convert() {
    assert_eq!(true as u8, 1);
    assert_eq!(false as u8, 0);
}

#[test]
fn test_then() {
    let mut a = 0;

    assert_eq!(
        true.then(|| {
            a += 1;
            a
        }),
        Some(1)
    );
    assert_eq!(
        false.then(|| {
            a *= 2;
            a
        }),
        None
    );

    assert_eq!(true.then_some(1), Some(1));
}
