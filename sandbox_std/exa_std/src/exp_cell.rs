use std::cell::Cell;

use tracing_test::traced_test;

/// Методы у Cell
#[test]
#[traced_test]
fn test_primitive_types() {
    let mut cell_var = Cell::new(42);
    assert_eq!(cell_var.get(), 42);

    cell_var.set(43);
    assert_eq!(cell_var.get(), 43);

    let old_value = cell_var.replace(44);
    assert_eq!(old_value, 43);
    assert_eq!(cell_var.get(), 44);

    let taken_value = cell_var.take();
    assert_eq!(taken_value, 44);
    assert_eq!(cell_var.get(), 0);

    cell_var.set(46);
    *cell_var.get_mut() += 1;
    assert_eq!(cell_var.get(), 47);

    let new_cell = Cell::new(48);
    cell_var.swap(&new_cell);
    assert_eq!(cell_var.get(), 48);
    assert_eq!(new_cell.get(), 47);

    cell_var.update(|v| v + 1);
    assert_eq!(cell_var.get(), 49);
}

/// Тест демонстрирует что можно изменять поля структуры, даже если сам объект не является mutable если значение хранится в Cell.
#[test]
#[traced_test]
fn test_with_struct_field() {
    struct MyStruct {
        value: Cell<i32>,
    }
    let my_struct = MyStruct {
        value: Cell::new(42),
    };
    assert_eq!(my_struct.value.get(), 42);

    // Даже если объект структуры не является mutable,
    // мы можем изменять его поля, так как они являются Cell.
    my_struct.value.update(|x| x + 1);
    assert_eq!(my_struct.value.get(), 43);
}

/// Тест демонстрирует, что Cell может хранить не только примитивные типы, но и структуры, если последние реализуют трейт Copy.
#[test]
#[traced_test]
fn test_with_struct_copy() {
    #[derive(Copy, Clone)] // Без copy работать не будет
    struct MyStruct {
        value: i32,
    }
    let my_struct = Cell::new(MyStruct { value: 42 });
    let t = my_struct.get().value;
    assert_eq!(t, 42);
}
