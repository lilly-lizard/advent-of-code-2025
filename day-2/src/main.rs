// step 1: print all ids to be scanned

use core::slice;

const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

fn main() {
    let ranges = INPUT.split(",");
    let mut invalid_ids = Vec::<u64>::new();

    for range in ranges {
        let start_end_str: Vec<_> = range.split("-").collect();
        let start = start_end_str[0].parse::<u64>().unwrap();
        let end = start_end_str[1].parse::<u64>().unwrap();
        for id in start..=end {
            if !is_valid(id) {
                invalid_ids.push(id);
            }
        }
    }

    let mut invalid_id_sum = 0;
    for invalid_id in invalid_ids {
        invalid_id_sum += invalid_id;
    }
    println!("{}", invalid_id_sum);
}

fn is_valid_old(id: u64) -> bool {
    let digits = get_digits(id);
    let len = digits.len();
    if len % 2 == 1 {
        // can't be invalid if odd number of digits
        return true;
    }
    let half_len = len / 2;
    let first_half = &digits[0..half_len];
    let second_half = &digits[half_len..len];

    return first_half != second_half;
}

fn is_valid(id: u64) -> bool {
    let digits = get_digits(id);
    let len = digits.len();
    if len <= 1 {
        return true;
    }
    let factors = factors(len);

    for factor in factors {
        if !is_valid_for_slice_size(&digits, factor) {
            return false;
        }
    }

    true
}

fn is_valid_for_slice_size(digits: &Vec<u64>, slice_size: usize) -> bool {
    let first_slice = &digits[0..slice_size];
    let slice_count = digits.len() / slice_size;
    for i in 1..slice_count {
        // assume slice_size is factor of len so this doesn't go out of bounds
        let slice = &digits[(slice_size * i)..(slice_size * (i + 1))];
        if slice != first_slice {
            return true;
        }
    }
    false
}

fn factors(x: usize) -> Vec<usize> {
    let mut factors = Vec::<usize>::new();
    for f in 1..=(x / 2) {
        if x % f == 0 {
            factors.push(f);
        }
    }
    factors
}

// note: puts smallest digit at front and largest at back of vec
fn get_digits(mut id: u64) -> Vec<u64> {
    if id == 0 {
        return vec![0];
    }
    let mut digits = Vec::<u64>::new();
    while id != 0 {
        digits.push(id % 10);
        id /= 10;
    }
    digits
}

#[cfg(test)]
mod tests {
    use crate::{factors, get_digits, is_valid, is_valid_for_slice_size};

    #[test]
    fn test_get_digits() {
        let expected = vec![8, 3, 1];
        let actual = get_digits(138);
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_is_valid_0() {
        assert!(is_valid(0));
    }
    #[test]
    fn test_is_valid_2() {
        assert!(is_valid(1));
    }
    #[test]
    fn test_is_valid_3() {
        assert!(is_valid(12));
    }
    #[test]
    fn test_is_valid_4() {
        assert!(is_valid(121));
    }
    #[test]
    fn test_is_valid_5() {
        assert!(!is_valid(1212));
    }
    #[test]
    fn test_is_valid_6() {
        assert!(!is_valid(38593859));
    }
    #[test]
    fn test_is_valid_7() {
        assert!(!is_valid(824824824));
    }
    #[test]
    fn test_is_valid_8() {
        assert!(!is_valid(1212121212));
    }

    #[test]
    fn test_factors_1() {
        let actual = factors(9);
        let expected = vec![1, 3];
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_factors_2() {
        let actual = factors(8);
        let expected = vec![1, 2, 4];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_is_valid_for_slice_size_1() {
        assert!(!is_valid_for_slice_size(&get_digits(2121212121), 2));
    }
    #[test]
    fn test_is_valid_for_slice_size_2() {
        assert!(is_valid_for_slice_size(&get_digits(2121212121), 1));
    }
    #[test]
    fn test_is_valid_for_slice_size_3() {
        assert!(!is_valid_for_slice_size(&get_digits(824824824), 3));
    }
    #[test]
    fn test_is_valid_for_slice_size_4() {
        assert!(!is_valid_for_slice_size(&get_digits(22), 1));
    }
}

const INPUT: &str = "7777742220-7777814718,3201990-3447830,49-86,653243-683065,91-129,24-41,1-15,2678-4638,1407-2511,221-504,867867-942148,1167452509-1167622686,9957459726-9957683116,379068-535983,757-1242,955118-1088945,297342-362801,548256-566461,4926-10075,736811-799457,1093342-1130060,620410-651225,65610339-65732429,992946118-993033511,5848473-5907215,17190619-17315301,203488-286290,15631-36109,5858509282-5858695889,87824047-87984031,1313113913-1313147594,795745221-795825571,46303-100636,4743038-4844422";
