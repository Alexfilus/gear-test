
#[cfg(test)]
mod tests {
    use crate::parallel::parallel_computation;

    #[test]
    fn test_small_data() {
        let data: Vec<u32> = vec![1, 2, 3, 4, 5];
        let result = parallel_computation(data, |x| x * 2, 1000);
        assert_eq!(result, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_large_data() {
        let data: Vec<u32> = (1..=5000).collect();
        let result = parallel_computation(data.clone(), |x| x * 2, 1000);
        let expected: Vec<u32> = data.into_iter().map(|x| x * 2).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_data() {
        let data: Vec<u32> = vec![];
        let result = parallel_computation(data, |x| x * 2, 1000);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_string_data() {
        let data: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let result = parallel_computation(data, |x| x.repeat(2), 1000);
        assert_eq!(result, vec!["aa".to_string(), "bb".to_string(), "cc".to_string()]);
    }

    #[test]
    fn test_complex_task() {
        let data: Vec<f64> = (1..=10000).map(|x| x as f64).collect();
        let result = parallel_computation(data.clone(), |x| x.sqrt(), 1000);

        let expected: Vec<f64> = data.into_iter().map(|x| x.sqrt()).collect();
        assert_eq!(result, expected);
    }
}