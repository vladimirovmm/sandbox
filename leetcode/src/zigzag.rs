//! Вывести символы из строки в зигзагообразном порядке.
//!

#![cfg(test)]

pub fn convert(s: String, num_rows: i32) -> String {
    let num_rows = num_rows as usize;
    if num_rows == 1 || s.chars().count() <= num_rows {
        return s;
    }

    let mut result: Vec<Vec<char>> = (0..num_rows).map(|_| Default::default()).collect();
    let step = num_rows * 2 - 2;
    for (pos, ch) in s.char_indices() {
        let mut index = pos % step;
        if index >= num_rows {
            index = step - index;
        }
        result[index].push(ch);
    }
    result.into_iter().flatten().collect()
}

#[test]
fn test_zigzag() {
    for ((input_str, num_rows), output) in [
        (("PAYPALISHIRING", 3), "PAHNAPLSIIGYIR"),
        (("PAYPALISHIRING", 4), "PINALSIGYAHRPI"),
        (("A", 1), "A"),
        (("AB", 1), "AB"),
    ] {
        assert_eq!(convert(input_str.into(), num_rows), output.to_string());
    }
}
