#![cfg(test)]

#[test]
fn test_ref() {
    let a = "Hello";
    let mut ref_a: *const u8 = a.as_ptr();

    assert!(!ref_a.is_null());
    println!("ref_a={ref_a:p}");

    unsafe {
        // После 5 выйдет за приделы размера строки. Ошибки не будет но информация будет неверной.
        for _ in 0..10 {
            println!("value={}", *ref_a as char);
            println!("ref: {ref_a:p}");
            ref_a = ref_a.wrapping_add(1);
        }
    }
    println!();

    ref_a = a.as_ptr();
    unsafe {
        // После 5 выйдет за приделы размера строки. Ошибки не будет но информация будет неверной.
        for _ in 0..10 {
            println!("value={}", *ref_a as char);
            println!("ref: {ref_a:p}");
            ref_a = ref_a.add(1);
        }
    }
    println!();

    let b = 11_u8;
    let ptr: *const u8 = &b as *const u8;

    unsafe {
        if let Some(val_back) = ptr.as_ref() {
            assert_eq!(val_back, &11);
        }
    }

    ref_a = a.as_ptr();
    unsafe {
        for offset in 0..13 {
            println!("value={}", *ref_a.offset(offset) as char);
        }
    }

    ref_a = a.as_ptr(); // Выставляем указатель на начало строки.
    ref_a = ref_a.wrapping_add(4); // сдвигаем указатель на 4 байта вперёд.
    unsafe {
        for n in 0..5 {
            // сдвигаем указатель на N байта назад и возвращаем значение. Указатель не перезаписываем. Т.е. он остаётся смотреть на 4 бата от начала
            println!("value={}", *ref_a.sub(n) as char);
        }
    }
}
