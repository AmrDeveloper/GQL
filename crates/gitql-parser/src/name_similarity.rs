const MIN_DISTANCE: usize = 2;

pub(crate) fn find_closeest_string(target: &str, candidates: &[&&str]) -> Option<String> {
    if candidates.is_empty() {
        return None;
    }

    let mut closest_match: Option<String> = None;
    let mut min_distance = usize::MAX;

    for candidate in candidates {
        let distance = levenshtein_distance(target, candidate);
        if distance < min_distance {
            min_distance = distance;
            closest_match = Some(candidate.to_string());
        }
    }

    if min_distance <= MIN_DISTANCE {
        return closest_match;
    }
    None
}

fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    let s1_len = s1_chars.len();
    let s2_len = s2_chars.len();

    let vec1_len = s1_len + 1;
    let vec12_len = s2_len + 1;

    let mut matrix = vec![vec![0; vec12_len]; vec1_len];
    for (i, vector) in matrix.iter_mut().enumerate().take(vec1_len) {
        vector[0] = i;
    }

    #[allow(clippy::needless_range_loop)]
    for j in 0..vec12_len {
        matrix[0][j] = j;
    }

    for i in 1..vec1_len {
        for j in 1..vec12_len {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] {
                0
            } else {
                1
            };

            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }

    matrix[s1_len][s2_len]
}
