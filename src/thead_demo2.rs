use std::sync::RwLock;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use lazy_static::lazy_static;

lazy_static! {
    static ref MUTEX1: Mutex<i64> = Mutex::new(0);
    static ref MUTEX2: Mutex<i64> = Mutex::new(0);
}

fn main() {
    // thread_mutex();

    // thread_arc();

    // thread_dead();

    thread_rwlock();
}

fn thread_mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();

        *num = 6;
    }

    println!("m = {:?}", m)
}

fn thread_arc() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn thread_dead() {
    let mut children = vec![];

    for i_thread in 0..2 {
        children.push(thread::spawn(move || {
            for _ in 0..1 {
                if i_thread % 2 == 0 {
                    let guard: MutexGuard<i64> = MUTEX1.lock().unwrap();
                    println!("线程 {} 锁住了MUTEX1，接着准备去锁MUTEX2 !", i_thread);

                    sleep(Duration::from_millis(10));

                    let guard = MUTEX2.lock().unwrap();
                } else {
                    // 锁住MUTEX2
                    let _guard = MUTEX2.lock().unwrap();

                    println!("线程 {} 锁住了MUTEX2, 准备去锁MUTEX1", i_thread);

                    let _guard = MUTEX1.lock().unwrap();

                    // sleep(Duration::from_millis(10));
                    // let guard = MUTEX1.try_lock();
                    // println!("线程2获取MUTEX1锁的结果: {:?}", guard);
                }
            }
        }))
    }

    // 等子线程完成
    for child in children {
        let _ = child.join();
    }

    println!("死锁没有发生");
}

fn thread_rwlock() {
    let lock = RwLock::new(5);

    {
        let r1 = lock.read().unwrap();
        let r2 = lock.try_read().unwrap();

        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    }

    {
        let mut w = lock.write().unwrap();

        *w += 1;
        assert_eq!(*w, 6);

        // 以下代码会panic，因为读和写不允许同时存在
        // let r1 = lock.read();
        // println!("{:?}",r1);
    }
}
