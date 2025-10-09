//! Assumptions: all abundances have the same probability of appearing

// /// Computes the standard deviation of a discrete uniform distribution
// fn standard_deviation_from_uniform_distribution(n: u64) -> f64 {
//     // variance = (N ** 2 - 1) / 12
//     ((n.pow(2) - 1) as f64 / 12.0).sqrt()
// }

use std::collections::HashSet;

// TODO normalize ?
pub fn score(data: &[u64]) -> Option<f64> {
    if data.is_empty() {
        return None;
    }
    let mean = data.iter().sum::<u64>() as f64 / data.len() as f64;
    let sum_square_error: f64 = data.iter().map(|x| (*x as f64 - mean).powi(2)).sum::<f64>();
    Some((sum_square_error / data.len() as f64).sqrt())
}

pub fn pelt<F: Fn(&[u64]) -> Option<f64>>(data: &[u64], cost: F, penalty: f64) -> HashSet<usize> {
    let T = data.len();

    let mut Z: Vec<f64> = vec![0.0; T + 1];
    Z[0] = -penalty;
    let mut L: Vec<HashSet<usize>> = vec![HashSet::new(); T + 1];
    let mut xi: HashSet<usize> = HashSet::from_iter(std::iter::once(0));

    for t in 1..=T {
        let teta = xi
            .iter()
            .copied()
            .map(|s| (s, Z[s] + cost(&data[s..t]).unwrap() + penalty))
            .collect::<Vec<(usize, f64)>>();
        let teta = *teta
            .iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(s, _val)| s) // TODO another min ?
            .unwrap();
        Z[t] = Z[teta] + cost(&data[teta..t]).unwrap() + penalty;
        L[t] = L[teta].clone();
        L[t].insert(teta);
        xi = xi
            .iter()
            .filter(|s| Z[**s] + cost(&data[**s..t]).unwrap() <= Z[t])
            .copied()
            .collect();
        xi.insert(t);
    }
    let l = L[L.len() - 1].clone();
    l
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::pelt;
    use super::score;

    #[test]
    fn test_random_noise_has_lower_score_than_segment() {
        let random = [
            95, 30, 0, 68, 50, 21, 30, 59, 30, 30, 93, 80, 79, 60, 39, 54, 25, 11, 25, 10, 40, 28,
            76, 60, 34,
        ];
        let segment = [
            30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
            30, 30, 30,
        ];
        let semi_segment = [
            30, 30, 30, 36, 30, 30, 30, 30, 30, 45, 30, 30, 30, 30, 12, 30, 30, 30, 30, 30, 30, 30,
            30, 30, 30,
        ];
        let score_random = score(&random).unwrap();
        let score_segment = score(&segment).unwrap();
        let score_semi_segment = score(&semi_segment).unwrap();

        assert!(score_random > score_semi_segment);
        assert!(score_semi_segment > score_segment);
    }

    #[test]
    fn test_pelt_all_same() {
        let segment = [
            30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
            30, 30, 30,
        ];
        let cp = pelt(&segment, score, 1.0);
        assert_eq!(cp, HashSet::from_iter(vec![0]));
    }

    #[test]
    fn test_pelt_segment() {
        let segment = [
            30, 30, 30, 30, 30, 30, 30, 30, //0 - 7
            50, 50, 50, 50, 50, 50, 50, //8 - 14
            30, 30, 30, 30, 30, 30, 30, 30, 30, 30, // 15 - 24
            100, 100, 100, 100, 100, // 25 - 28
        ];
        let cp = pelt(&segment, score, 1.0);
        assert_eq!(cp, HashSet::from_iter(vec![0, 8, 15, 25]));
    }

    #[test]
    fn test_pelt_with_noise_segment() {
        let segment = [
            30, 29, 30, 32, 30, 26, 28, 30, //0 - 7
            50, 50, 50, 50, 50, 50, 50, //8 - 14
            30, 30, 30, 30, 30, 30, 30, 30, 30, 30, // 15 - 24
            100, 100, 100, 102, 100, // 25 - 28
        ];
        let cp = pelt(&segment, score, 1.0);
        assert_eq!(cp, HashSet::from_iter(vec![0, 8, 15, 25]));
    }
}
