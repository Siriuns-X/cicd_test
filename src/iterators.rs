/// 给定整数列表，返回其中偶数的平方，按升序排列
pub fn even_squares(nums: &[i32]) -> Vec<i32> {
    // TODO: 用迭代器链完成（filter / map / collect）
    let mut result: Vec<i32> = nums
        .iter()
        .filter(|x| **x % 2 == 0)
        .map(|x| x * x)
        .collect();
    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        assert_eq!(even_squares(&[1, 2, 3, 4, 5, 6]), vec![4, 16, 36]);
    }
    #[test]
    fn test_empty() {
        assert_eq!(even_squares(&[]), vec![]);
    }
    #[test]
    fn test_odds() {
        assert_eq!(even_squares(&[1, 3, 5]), vec![]);
    }
    #[test]
    fn test_sort() {
        assert_eq!(even_squares(&[6, 2, 4, 1, 3, 5]), vec![4, 16, 36]);
    }
}
