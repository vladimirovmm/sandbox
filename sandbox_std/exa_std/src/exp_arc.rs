//! Arc<T> (Atomic Reference Counting, «атомарный подсчёт ссылок») — смарт‑указатель из стандартной библиотеки Rust (std::sync::Arc),
//! который позволяет безопасно разделять владение данными между несколькими потоками (threads)

use std::{ptr, sync::Arc, thread, time::Duration};

use rand::random_range;
use tracing_test::traced_test;

#[test]
#[traced_test]
fn test_arc() {
    let v = "demo".to_string();
    let source_ptr = v.as_ptr();
    let arc = Arc::new(v);
    let inner = Arc::unwrap_or_clone(arc);
    assert!(ptr::eq(source_ptr, inner.as_ptr()));

    let arc = Arc::new(inner);
    let _arc_clone = arc.clone();
    let inner = Arc::unwrap_or_clone(arc);
    assert!(!ptr::eq(source_ptr, inner.as_ptr()));
}

#[test]
#[traced_test]
fn test_arc_thread() {
    let value = Arc::new("demo".to_string());

    let th = (0..10)
        .map(|num_th| {
            let value_clone = value.clone();
            thread::spawn(move || {
                for num in 0..10 {
                    println!("{num_th}:{num}:{value_clone}");
                    thread::sleep(Duration::from_millis(random_range(0..50)));
                }
            })
        })
        .collect::<Vec<_>>();

    th.into_iter().for_each(|th| th.join().unwrap());
}
