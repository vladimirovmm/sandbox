//! Проверка является ли число палиндромом

#![cfg(test)]

fn is_palindrome(x: i32) -> bool {
    let mut x = x;
    if x < 0 {
        return false;
    }
    if x < 10 {
        return true;
    }

    let mut num = Vec::new();

    while x != 0 {
        num.push(x % 10);
        x /= 10;
    }

    let r = num.len() / 2;
    let l = if num.len() % 2 == 0 { r - 1 } else { r };

    for offset in 0..=l {
        if num[l - offset] != num[r + offset] {
            return false;
        }
    }
    true
}

#[test]
fn test_is_palindrome() {
    assert!(is_palindrome(121));
    assert!(!is_palindrome(-121));
    assert!(!is_palindrome(10));
    assert!(is_palindrome(0));
    assert!(is_palindrome(1));
    assert!(!is_palindrome(12));
    assert!(!is_palindrome(123));
    assert!(is_palindrome(12321));
    assert!(is_palindrome(123321));
    assert!(is_palindrome(1234321));
    assert!(is_palindrome(123454321));
    assert!(is_palindrome(808));
    assert!(!is_palindrome(-808));
}
