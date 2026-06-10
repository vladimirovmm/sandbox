//!RwLock (Read‑Write Lock) нужен для эффективной синхронизации доступа к общим данным в многопоточной среде:
//! он позволяет множественным потокам одновременно читать данные без блокировок,
//! а при необходимости записи предоставляет эксклюзивный доступ одному потоку,
//! временно блокируя и чтение, и другие записи. Это оптимально для сценариев, где чтение происходит гораздо чаще,
//! чем изменение данных (например, кэши, конфигурации, справочные таблицы).

use std::{
    sync::{Arc, Mutex, RwLock},
    thread::sleep,
    time::Duration,
};
use tracing::info;

use tracing_test::traced_test;

#[test]
#[traced_test]
fn test_rwlock() {
    use std::sync::RwLock;
    let lock = RwLock::new(0);
    {
        let mut guard = lock.write().unwrap();
        *guard = 1;
    }
    assert_eq!(*lock.read().unwrap(), 1);
}

/// Поиск простых чисел в мультипоточной среде
#[test]
#[traced_test]
fn test_rwlock_find_primes() {
    const LIMIT_THREADS: usize = 4;
    const MAX_NUM: usize = 1_000_000;
    let last_num = Arc::new(Mutex::new(2_usize));
    let primes = Arc::new(RwLock::new(vec![2_usize]));

    let threads = (0..LIMIT_THREADS).map(|n| {
        let last_num = Arc::clone(&last_num);
        let primes = Arc::clone(&primes);
        std::thread::spawn(move || {
            loop {
                let num = {
                    let mut last = last_num.lock().unwrap();
                    *last += 1;
                    *last
                };
                if num >= MAX_NUM {
                    break;
                }
                if num % 2 == 0 {
                    continue;
                }

                loop {
                    let primes_vec = primes.read().unwrap().clone();
                    if primes_vec
                        .iter()
                        .take_while(|p| **p < num / 2)
                        .any(|p| num % p == 0)
                    {
                        break;
                    }
                    if let Some(last) = primes_vec.last() {
                        if *last > num / 2 {
                            info!("{n}: found prime: {num}");
                            primes.write().unwrap().push(num);
                            break;
                        } else {
                            sleep(Duration::from_millis(10));
                        }
                    }
                }
            }
        })
    });

    threads.for_each(|t| t.join().unwrap());
    let primes = Arc::try_unwrap(primes).unwrap().into_inner().unwrap();
    println!("{:#?}", primes);
}
