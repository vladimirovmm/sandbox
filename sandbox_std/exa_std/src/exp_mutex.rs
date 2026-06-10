use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};
use tracing::debug;
use tracing_test::traced_test;

#[test]
#[traced_test]
fn test_mutex() {
    let mutex = Mutex::new(0);
    let mut lock = mutex.lock().unwrap();
    *lock = 1;
    drop(lock);

    let lock = mutex.lock().unwrap();
    assert_eq!(*lock, 1);
    drop(lock);
}

#[test]
#[traced_test]
fn test_mutex_thread() {
    const SIZE: usize = 10;
    let counter = Arc::new(Mutex::new([0_usize; SIZE]));

    let join_handles = (0..SIZE / 2)
        .map(|thread_num| {
            let counter_clone = Arc::clone(&counter);
            std::thread::spawn(move || {
                for index in 0..SIZE {
                    // Ожидаем доступа к мьютексу
                    let mut lock = counter_clone.lock().unwrap();
                    debug!("{thread_num}:{index}:{:?}", &lock[index]);
                    lock[index] += thread_num * index;
                }
            })
        })
        .chain((SIZE / 2..SIZE).map(|thread_num| {
            let counter_clone = Arc::clone(&counter);
            std::thread::spawn(move || {
                for index in 0..SIZE {
                    loop {
                        // Попытка захвата мьютекса
                        let Ok(mut lock) = counter_clone.try_lock() else {
                            // Если захват не удался, ждем и повторяем
                            sleep(Duration::from_millis(10));
                            continue;
                        };
                        debug!("{thread_num}:{index}:{:?}", &lock[index]);
                        lock[index] += thread_num * index;
                        break;
                    }
                }
            })
        }))
        .collect::<Vec<_>>();

    for handle in join_handles {
        handle.join().unwrap();
    }

    let lock = Arc::try_unwrap(counter).unwrap().into_inner().unwrap();
    println!("{lock:#?}");
}
