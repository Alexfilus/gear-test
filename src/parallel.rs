use std::fmt::Debug;
use std::thread;

pub fn parallel_computation<T, R, F>(data: Vec<T>, func: F, threshold: usize) -> Vec<R>
    where
        T: Send + Debug + Clone + 'static,
        R: Send + 'static,
        F: Fn(T) -> R + Send + Clone + 'static,
{
    let data_len = data.len();

    if data_len < threshold {
        return data.into_iter().map(func).collect();
    }

    let num_threads = 4;
    let chunk_size = data_len / num_threads;

    let mut handles = vec![];
    let mut start = 0;
    let mut remainder = data_len % num_threads;
    for _ in 0..num_threads {
        let mut end = start + chunk_size;
        if remainder > 0 {
            end += 1;
            remainder -= 1;
        }

        let chunk: Vec<T> = data[start..end].to_vec();
        let func_clone = func.clone();

        let handle = thread::spawn(move || {
            chunk.into_iter().map(func_clone).collect::<Vec<R>>()
        });
        handles.push(handle);

        start = end;
    }

    let mut result = Vec::with_capacity(data_len);

    for handle in handles {
        result.extend(handle.join().unwrap());
    }

    result
}

