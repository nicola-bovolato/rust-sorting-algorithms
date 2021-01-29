mod selection_sort;
mod insertion_sort;
mod bubble_sort;
mod merge_sort;

fn main() {
    let v = vec![1, 5, 2, 6, 3, 6, 0];
    let mut v1 = v.clone();
    let mut v2 = v.clone();
    let mut v3 = v.clone();
    let mut v4 = v.clone();

    println!("Before: \t\t {:?}",v);

    selection_sort::sort(&mut v1, |x,y| x < y);
    insertion_sort::sort(&mut v2, |x,y| x < y);
    bubble_sort::sort(&mut v3, |x,y| x < y);
    merge_sort::sort(&mut v4, |x,y| x < y);

    println!("After Selection Sort: \t {:?}",v1);
    println!("After Insertion Sort: \t {:?}",v2);
    println!("After Bubble Sort: \t {:?}",v3);
    println!("After Merge Sort: \t {:?}",v4);
}
