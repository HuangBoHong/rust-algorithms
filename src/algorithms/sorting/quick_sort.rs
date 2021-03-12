use std::slice::Iter;
use std::iter::Copied;

fn quick_sort_functional<T: Ord + Copy>(arr: Vec<T>) -> Vec<T> {
    match arr.first() {
        Some(&head) => {
            quick_sort_functional(arr.iter().filter_map(|&x| if x < head { Some(x) } else { None }).collect()).iter()
                .chain(arr.iter().filter(|&&x| x == head))
                .chain(quick_sort_functional(arr.iter().filter_map(|&x| if x > head { Some(x) } else { None }).collect()).iter())
            .map(|&x| x).collect()
        }
        None => arr
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use algorithms::sorting::rand_arr;

    #[test]
    fn functional() {
        let mut vec1 = Vec::from(rand_arr());
        let vec2 = quick_sort_functional(vec1.to_vec());
        vec1.sort();
        assert_eq!(vec1, vec2);
    }
}