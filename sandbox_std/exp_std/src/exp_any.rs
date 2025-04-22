#![cfg(test)]

use std::any::{Any, TypeId};

#[test]
#[allow(clippy::type_id_on_box)]
fn test_box_dyn() {
    let a = 1_i8;
    let b = 1_i16;

    let a_a: Box<dyn Any> = Box::new(a);
    let a_b: Box<dyn Any> = Box::new(b);

    assert_eq!(TypeId::of::<Box<dyn Any>>(), a_a.type_id());
    assert_eq!(TypeId::of::<i8>(), (*a_a).type_id());
    assert_eq!(TypeId::of::<i16>(), (*a_b).type_id());

    /// Функция суммирует 2-а значения разных типов. 2-ой аргумент - ссылка на значение любого типа.
    /// Эта функция лишь демонстрация работы с dyn Any. Понятнее и удобнее в использование реализация через Trait.
    fn multi_add<T: Any>(back_result: isize, value: &T) -> isize {
        let value_any = value as &dyn Any;
        println!("type name: {}", std::any::type_name::<T>());

        multi_add2(back_result, value_any)
    }
    /// Разбито на отдельную функцию просто для демонстрации возможности и избежания повторения кода.
    fn multi_add2(back_result: isize, value: &dyn Any) -> isize {
        println!("type name by value: {}", std::any::type_name_of_val(value));

        if value.is::<String>() {
            println!("Cтрока: {}", value.downcast_ref::<String>().unwrap());
        }

        let val = if let Some(val) = value.downcast_ref::<i8>() {
            *val as isize
        } else if let Some(val) = value.downcast_ref::<u8>() {
            *val as isize
        } else if let Some(val) = value.downcast_ref::<&str>() {
            val.parse().unwrap_or_default()
        } else if let Some(val) = value.downcast_ref::<String>() {
            val.parse().unwrap_or_default()
        } else if let Some(val) = value.downcast_ref::<char>() {
            val.to_string().parse().unwrap_or_default()
        } else {
            return back_result;
        };
        back_result + val
    }
    let mut result = 0;
    result = multi_add(result, &1_i8);
    result = multi_add(result, &2_u8);
    result = multi_add(result, &"3");
    result = multi_add(result, &'4');
    result = multi_add(result, &"null"); // Некорректное значение.
    result = multi_add(result, &("5".to_string()));
    result = multi_add(result, &"5-..".to_string()); // Некорректное значение.
    result = multi_add(result, &["null"]); // Некорректное значение.

    assert_eq!(result, 15);
}
