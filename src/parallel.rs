use rayon::prelude::*;

pub fn parallel_computation<T, R, F>(input: Vec<T>, function: F, threshold: usize) -> Vec<R>
    where
        T: Send,
        R: Send + Default,
        F: Fn(T) -> R + Sync + Send,
{
    if input.len() < threshold {
        input.into_iter().map(function).collect()
    } else {
        input.into_par_iter().map(function).collect()
    }
}