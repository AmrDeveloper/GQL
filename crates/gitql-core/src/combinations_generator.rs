/// Return a list of all non empty and unique combinations
pub fn generate_list_of_all_combinations(n: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::with_capacity((2 << n) - 1);
    let mut current = Vec::with_capacity(n);
    generate_indices_combination(n, 0, &mut current, &mut result);
    result
}

fn generate_indices_combination(
    n: usize,
    start: usize,
    current: &mut Vec<usize>,
    result: &mut Vec<Vec<usize>>,
) {
    if !current.is_empty() {
        result.push(current.clone());
    }

    for i in start..n {
        current.push(i);
        generate_indices_combination(n, i + 1, current, result);
        current.pop();
    }
}
