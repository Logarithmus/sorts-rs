use crate::sort;
use core::convert::{TryFrom, TryInto};
use core::fmt::{Debug, Display};
use rand::Rng;

#[allow(unused)]
pub fn test_once_unsigned<T: Display + Debug + sort::RadixSortable + sort::Unsigned>(slice: &[T])
where
    <T as TryFrom<usize>>::Error: Debug,
    <T as TryInto<usize>>::Error: Debug,
{
    let mut vec_bubble = slice.to_vec();
    let mut vec_selection = slice.to_vec();
    let mut vec_insertion = slice.to_vec();
    let mut vec_shell = slice.to_vec();
    let mut vec_qsort = slice.to_vec();
    let mut vec_heap = slice.to_vec();
    let mut vec_radix = slice.to_vec();
    let mut vec_comb = slice.to_vec();

    sort::bubble_sort(&mut vec_bubble);
    sort::selection_sort(&mut vec_selection);
    sort::insertion_sort(&mut vec_insertion);
    sort::shell_sort(&mut vec_shell);
    sort::qsort(&mut vec_qsort);
    sort::heap_sort(&mut vec_heap);
    sort::radix_sort_unsigned(&mut vec_radix, 8);
    sort::comb_sort(&mut vec_comb);

    println!("Bubble:    {:?}", &vec_bubble);
    println!("Selection: {:?}", &vec_selection);
    println!("Insertion: {:?}", &vec_insertion);
    println!("Shell:     {:?}", &vec_shell);
    println!("Qsort:     {:?}", &vec_qsort);
    println!("Heap:      {:?}", &vec_heap);
    println!("Radix:     {:?}", &vec_radix);
    println!("Comb:      {:?}", &vec_comb);

    assert!(vec_bubble == vec_selection);
    assert!(vec_selection == vec_insertion);
    assert!(vec_insertion == vec_shell);
    assert!(vec_shell == vec_qsort);
    assert!(vec_qsort == vec_heap);
    assert!(vec_heap == vec_radix);
    assert!(vec_radix == vec_comb);
}

#[test]
fn all_many_times() {
    let mut rng = rand::thread_rng();
    (0..1000).for_each(|_| test_once_unsigned(&(0..100).map(|_| rng.gen()).collect::<Vec<u8>>()));
    (0..1000).for_each(|_| test_once_unsigned(&(0..100).map(|_| rng.gen()).collect::<Vec<u16>>()));
    (0..1000).for_each(|_| test_once_unsigned(&(0..100).map(|_| rng.gen()).collect::<Vec<u32>>()));
}

#[test]
fn all_once() {
    let mut rng = rand::thread_rng();
    let vec = (0..10).map(|_| rng.gen()).collect::<Vec<u8>>();
    test_once_unsigned(&vec);
}

#[test]
fn qsort_sorted() {
    let mut arr = [1, 2, 3, 4, 5];
    sort::qsort(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5]);
}
