pub fn sort<T: PartialOrd> (array: &mut Vec<T>, comparator: fn(&T,&T) -> bool) {
    
    loop{
        let mut swap = false;

        for i in 1..array.len() {
            
            if comparator(&array[i], &array[i - 1]) {

                array.swap(i - 1, i);
                swap = true;
            }
        }

        if !swap { break; }
    }
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
