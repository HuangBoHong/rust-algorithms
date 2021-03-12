fn kmp(input: &String, pattern: &String) -> Vec<usize> {
    if pattern.is_empty() {
        return vec![];
    }

    let input_chars: Vec<char> = input.chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();

    let pmt: Vec<usize> = pattern_chars.iter().enumerate().scan(0, |acc, (i, c) | {
        if i > 0 {
            if pattern_chars.get(*acc).unwrap() == c {
                *acc += 1;
                Some(*acc)
            } else {
                *acc = 0;
                Some(0)
            }
        } else {
            Some(0)
        }
    }).collect();

    let mut i = 0;
    let mut j = 0;
    let mut matches: Vec<usize> = vec![];
    while i < input_chars.len() {
        if input_chars.get(i).unwrap() == pattern_chars.get(j).unwrap() {
            j += 1;
            i += 1;
            if j == pattern_chars.len() {
                matches.push(i - pattern_chars.len());
                j = 0;
            }
        } else if j > 0 {
            j = *pmt.get(j - 1).unwrap();
        } else {
            i += 1;
            j = 0;
        }
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!( kmp(&"The cat catches a fat rat and scatters the others under the mat.".to_string(), &"at".to_string()), vec![5, 9, 19, 23, 32, 61]);
    }
}