#![cfg(test)]

#[test]
fn test_array() {
    let arr = [1, 2, 3, 4, 5];
    let arr1 = arr.each_ref().map(|t| t * 2);
    assert_eq!(arr1, [2, 4, 6, 8, 10]);

    let arr2 = arr.each_ref().map(|t| t % 2 == 0);
    assert_eq!(arr2, [false, true, false, true, false]);

    let mut arr = [1, 2, 3, 4, 5];
    arr.each_mut().map(|t| {
        if *t % 2 == 0 {
            *t *= *t
        }
    });
    assert_eq!(arr, [1, 4, 3, 16, 5]);

    let [_, a, ..] = arr;
    assert_eq!(a, 4);
}
