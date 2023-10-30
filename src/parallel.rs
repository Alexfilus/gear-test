use std::sync::Arc;
use std::thread;

pub fn parallel_computation<T, R, F>(data: Vec<T>, func: F, threshold: usize) -> Vec<R>
    where
        T: Send + Sync + Clone + 'static,
        R: Send + 'static,
        F: Fn(T) -> R + Send + Sync + 'static,
{
    let data_len = data.len();

    if data_len < threshold {
        return data.into_iter().map(func).collect();
    }

    let data = Arc::new(data);
    let func = Arc::new(func);

    let num_threads = 12;
    let chunk_size = data_len / num_threads;

    let mut handles = vec![];

    for i in 0..num_threads {
        let data_clone = Arc::clone(&data);
        let func_clone = Arc::clone(&func);

        let handle = thread::spawn(move || {
            let start = i * chunk_size;
            let end = if i == num_threads - 1 {
                data_len
            } else {
                start + chunk_size
            };

            let mut result = Vec::with_capacity(end - start);

            for j in start..end {
                let item = data_clone[j].clone();
                result.push((func_clone)(item));
            }

            result
        });

        handles.push(handle);
    }

    let mut result = Vec::with_capacity(data_len);

    for handle in handles {
        result.extend(handle.join().unwrap());
    }

    result
}

