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

    /// Функция сумирует 2-а значения разных типов. 2-ой аргумент - ссылка на значение любого типа.
    /// Эта функция лишь демонстрация работы с dyn Any. Понятнее и удобнее в использование реализация чере Trait.
    fn multi_add<T: Any>(back_result: isize, value: &T) -> isize {
        let value_any = value as &dyn Any;

        let val = if let Some(val) = value_any.downcast_ref::<i8>() {
            *val as isize
        } else if let Some(val) = value_any.downcast_ref::<u8>() {
            *val as isize
        } else if let Some(val) = value_any.downcast_ref::<&str>() {
            val.parse().unwrap_or_default()
        } else if let Some(val) = value_any.downcast_ref::<char>() {
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
    result = multi_add(result, &"null");
    result = multi_add(result, &["null"]);

    assert_eq!(result, 10);
}
