// Rc<T> (Reference Counted): Умный указатель для подсчёта ссылок,
// позволяющий иметь несколько владельцев одного значения в куче.
// Используется в однопоточных приложениях.

use std::{collections::BTreeMap, rc::Rc};
use tracing_test::traced_test;

#[test]
#[traced_test]
fn test_rc() {
    let mut a = Rc::new(1);
    assert!(Rc::get_mut(&mut a).is_some());
    assert_eq!(1, Rc::strong_count(&a)); // Ссылок на данные: 1
    assert_eq!(*a, 1);

    *Rc::make_mut(&mut a) += 1;
    assert_eq!(2, *a);

    {
        let b = a.clone();
        assert_eq!(2, Rc::strong_count(&a)); // Ссылок на данные: 2
        let _c = b;
        assert_eq!(3, Rc::strong_count(&a)); // Ссылок на данные: 3

        assert!(Rc::get_mut(&mut a).is_none()); // Нельзя получить. Ссылок больше одной.
    }
    assert!(Rc::get_mut(&mut a).is_some()); // Можно получить. Только A ссылается на данные.

    {
        let b = a.clone();
        assert_eq!(2, Rc::strong_count(&a));
        assert_eq!(*b, *a); // Значение одинаковое = 2
        assert!(Rc::ptr_eq(&a, &b)); // Ссылки на данные указывают на одно место памяти.

        *Rc::make_mut(&mut a) += 1; // Ссылок больше одной. Данные копируются.

        // Теперь a и b указывают на разные области памяти.
        assert_ne!(*a, *b);
        assert_eq!(3, *a);
        assert_eq!(2, *b);
        assert_eq!(1, Rc::strong_count(&a));
        assert_eq!(1, Rc::strong_count(&b));
        assert!(!Rc::ptr_eq(&a, &b)); // Ссылки на данные указывают на разные места памяти.

        let _c = a.clone();
        assert_eq!(2, Rc::strong_count(&a));
    }

    let a = "demo".to_string();
    let ptr_a = a.as_ptr();
    let a = Rc::new(a);
    let v = Rc::unwrap_or_clone(a); // После этого a больше не существует. Владение передаётся на данные v.
    assert_eq!(v, "demo");
    let ptr_v = v.as_ptr();
    assert_eq!(ptr_a, ptr_v);

    let a = "demo".to_string();
    let ptr_a = a.as_ptr();
    let a = Rc::new(a);
    let _b = a.clone(); // Создаётся вторая ссылка на данные.
    let v = Rc::unwrap_or_clone(a); // Есть вторая ссылка на данные. Данные будут скопированы.
    assert_eq!(v, "demo");
    let ptr_v = v.as_ptr();
    assert_ne!(ptr_a, ptr_v);
}

#[derive(Debug)]
#[allow(dead_code)]
struct Block {
    number: u64,
    divider: BTreeMap<u64, Rc<Block>>,
}

#[test]
fn test_vert() {
    let mut nums: BTreeMap<u64, Rc<Block>> = Default::default();

    for number in 2..=100 {
        let mut block = Block {
            number,
            divider: Default::default(),
        };

        for (num_f, block_f) in nums.iter().rev() {
            if number % num_f != 0 || block.divider.contains_key(num_f) {
                continue;
            }

            block.divider.insert(*num_f, block_f.clone());
            block.divider.append(&mut block_f.divider.clone());
        }
        nums.insert(number, block.into());
    }

    println!("Блоки: {nums:#?}");
}
