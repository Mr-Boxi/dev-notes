use std::cmp;

pub fn insertion_sort<T: Copy + cmp::PartialOrd>(arr: &mut Vec<T>) {
    if arr.is_empty() { return;}

    for i in 1..arr.len() {
        let current = arr[i];

        let mut j = ( i - 1) as i32;
        while j >= 0 {
            if arr[j as usize] > current {
                    arr[(j + 1) as usize] = arr[j as usize]
            }else {
                break;
            }
            j -= 1;
        }
        arr[ (j + 1) as usize] = current;
    }
}

pub fn is_sorted<T: cmp::PartialOrd>(arr: &Vec<T>) -> bool {

    for i in 0..arr.len() - 1 {
        if arr[i] > arr[i + 1] {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insertion_test() {
        let mut nums = vec![7.7, 9.9, 12.12, 11.11, 6.6, 3.3];
        insertion_sort(&mut nums);
        assert!(is_sorted(&nums));
    }

    
}