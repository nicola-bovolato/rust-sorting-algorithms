pub fn sort<T: PartialOrd + Copy> (array: &mut Vec<T>, comparator: fn(&T,&T) -> bool) {
    
    merge_sort_fun(array, 0, array.len() - 1, comparator);
}

fn merge_sort_fun<T: PartialOrd + Copy> (array: &mut Vec<T>, start: usize, end: usize, comparator: fn(&T,&T) -> bool) {

    if start >= end { return; }

    let m = (start + end) / 2;

    merge_sort_fun(array, start, m, comparator);
    merge_sort_fun(array, m + 1, end, comparator);

    merge_fun(array, start, m, end, comparator);
}

fn merge_fun<T: PartialOrd + Copy> (array: &mut Vec<T>, start: usize, middle: usize, end: usize, comparator: fn(&T,&T) -> bool) {

    let mut x = vec![];

    let mut i = start;
    let mut j = middle + 1;

    while i <= middle && j <= end {

        if comparator(&array[i], &array[j]) {
            x.push(array[i]);
            i += 1;
        }
        else {
            x.push(array[j]);
            j += 1;
        }
    }

    x.extend_from_slice(&array[i..middle + 1]);
    x.extend_from_slice(&array[j..end + 1]);

    array.splice(start..end + 1, x.iter().cloned());
}

#[cfg(test)]
mod tests {

    #[test]
    fn already_sorted(){
        let mut v = vec![1, 2, 3, 4, 5];

        super::sort(&mut v, |x,y| x < y);

        assert_eq!(v, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn one_element(){
        let mut v = vec![1];

        super::sort(&mut v, |x,y| x < y);

        assert_eq!(v, [1]);
    }

    #[test]
    fn two_elements(){
        let mut v = vec![2, 1];

        super::sort(&mut v, |x,y| x < y);

        assert_eq!(v, [1, 2]);
    }

    #[test]
    fn unsorted(){
        let mut v = vec![3, 5, 1, 2, 4];

        super::sort(&mut v, |x,y| x < y);

        assert_eq!(v, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn last_should_be_first(){
        let mut v = vec![2, 3, 4, 5, 1];

        super::sort(&mut v, |x,y| x < y);

        assert_eq!(v, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn first_should_be_last(){
        let mut v = vec![5, 1, 2, 3, 4];

        super::sort(&mut v, |x,y| x < y);

        assert_eq!(v, [1, 2, 3, 4, 5]);
    }
}
