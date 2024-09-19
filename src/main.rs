use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

use dashmap::DashMap;

fn main() {
    let mut handles = vec![];
    let map1 = Arc::new(DashMap::new()); // создаем пустую хэш-таблицу обеспечивающую безопасный доступ к данным в многопоточной среде
    let map2 = Arc::new(Mutex::new(HashMap::new())); // создаем пустую хэш-таблицу, помещаем ее в mutex для предотвращения одновременного доступа к данным, и получаем умный указатель Arc позволяющий нескольким потокам совместно использовать значения

    // создаем 10 потоков для записи данных в DashMap
    for i in 1..=10 {
        let map1 = map1.clone(); // клонируем данные для каждого потока

        let handle = thread::spawn(move || {
            map1.insert(i, i * 2);
        });
        handles.push(handle);
    }

    // создаем 10 потоков для записи данных в HashMap
    for i in 1..=10 {
        let map2 = Arc::clone(&map2); // клонируем данные для каждого потока
        let handle = thread::spawn(move || {
            let mut map2 = map2.lock().unwrap();
            map2.insert(i, i * 2);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("{:?}", map1);
    println!("{:?}", map2);
}
