use core::ops::{BitAnd, Shl, Shr};
use std::convert::{TryFrom, TryInto};
use std::mem::size_of;

pub fn bubble_sort<T: Ord + Debug>(slice: &mut [T]) {
    for i in 0..slice.len() {
        let is_swapped = (0..(slice.len() - 1 - i))
            .map(|j| {
                if slice[j] > slice[j + 1] {
                    slice.swap(j, j + 1);
                    true
                } else {
                    false
                }
            })
            .fold(false, |state, f| state | f);
        if !is_swapped {
            break;
        }
    }
}

pub fn selection_sort<T: Ord>(slice: &mut [T]) {
    for i in 0..(slice.len() - 1) {
        let min_i = i + slice[i..]
            .iter()
            .enumerate()
            .min_by_key(|(_, x)| *x)
            .unwrap()
            .0;
        slice.swap(min_i, i);
    }
}

pub fn insertion_sort<T: Ord + Clone>(slice: &mut [T]) {
    for i in 0..slice.len() {
        let (mut j, slice_i) = (i, slice[i].clone());
        while (j > 0) && (slice[j - 1] > slice_i) {
            slice[j] = slice[j - 1].clone();
            j -= 1;
        }
        slice[j] = slice_i.clone();
    }
}

pub fn shell_sort<T: Ord + Clone>(slice: &mut [T]) {
    let len = slice.len();
    let mut step = len / 2;
    while step > 0 {
        for i in step..len {
            let (mut j, slice_i) = (i, slice[i].clone());
            while (j >= step) && (slice[j - step] > slice_i) {
                slice[j] = slice[j - step].clone();
                j -= step;
            }
            slice[j] = slice_i.clone();
        }
        step /= 2;
    }
}

pub fn qsort<T: Ord + Clone>(slice: &mut [T]) {
    fn hoare_partition<T: Ord + Clone>(slice: &mut [T]) -> (&mut [T], &mut [T]) {
        use rand::Rng;
        let len = slice.len();
        let pivot_i = rand::thread_rng().gen_range(0, len - 1);
        let pivot = slice[0].clone();
        slice.swap(pivot_i, 0);
        let (mut l, mut r) = (0, len - 1);
        loop {
            while slice[l] < pivot {
                l += 1
            }
            while slice[r] > pivot {
                r -= 1
            }
            if l < r {
                slice.swap(l, r);
                (l += 1, r -= 1);
            } else {
                break;
            }
        }
        slice.split_at_mut(r + 1)
    }

    if slice.len() < 5 {
        insertion_sort(slice);
    } else {
        let (left, right) = hoare_partition(slice);
        (qsort(left), qsort(right));
    }
}

pub fn heap_sort<T: Ord>(slice: &mut [T]) {
    fn heapify<T: Ord>(slice: &mut [T], i: usize) {
        let (l, r) = (2 * i + 1, 2 * i + 2);
        let mut max_i = i;
        if (l < slice.len()) && (slice[l] > slice[max_i]) {
            max_i = l;
        }
        if (r < slice.len()) && (slice[r] > slice[max_i]) {
            max_i = r;
        }
        if max_i != i {
            slice.swap(max_i, i);
            heapify(slice, max_i)
        }
    }

    (0..slice.len() / 2).rev().for_each(|i| heapify(slice, i));
    (0..slice.len()).rev().for_each(|i| {
        slice.swap(i, 0);
        heapify(&mut slice[..i], 0);
    });
}

pub trait RadixSortable:
    Ord
    + Copy
    + Default
    + Sized
    + Shr<usize, Output = Self>
    + BitAnd<Output = Self>
    + TryInto<usize>
    + TryFrom<usize>
{
}

pub trait Unsigned {}

impl RadixSortable for u8 {}
impl RadixSortable for u16 {}
impl RadixSortable for u32 {}
impl RadixSortable for u64 {}
impl RadixSortable for u128 {}
impl RadixSortable for i8 {}
impl RadixSortable for i16 {}
impl RadixSortable for i32 {}
impl RadixSortable for i64 {}
impl RadixSortable for i128 {}
impl Unsigned for u8 {}
impl Unsigned for u16 {}
impl Unsigned for u32 {}
impl Unsigned for u64 {}
impl Unsigned for u128 {}

pub fn radix_sort_unsigned<T: RadixSortable + Unsigned>(slice: &mut [T], radix: usize) {
    let size_of_bits = size_of::<T>() * 8;
    assert!(radix % 2 == 0);
    assert!(radix <= size_of_bits);
    let mask = ((1 << radix) - 1).try_into().unwrap();
    for shr_by in (0..size_of_bits).step_by(radix) {
        let mut count = vec![0_usize; 1 << radix];
        let mut sorted = vec![T::default(); slice.len()];
        for i in 0..slice.len() {
            let shifted = slice[i] >> shr_by;
            let index = (shifted & mask).try_into().unwrap();
            count[index] += 1;
        }
        for i in 1..(1 << radix) {
            count[i] += count[i - 1];
        }
        for i in (0..slice.len()).rev() {
            let shifted = slice[i] >> shr_by;
            let index = (shifted & mask).try_into().unwrap();
            count[index] -= 1;
            sorted[count[index]] = slice[i];
        }
        slice.copy_from_slice(&sorted[..]);
    }
}

pub fn comb_sort<T: Ord>(slice: &mut [T]) {
    let mut step = slice.len() - 1;
    while step > 0 {
        for i in 0..slice.len() - step {
            if slice[i] > slice[i + step] {
                slice.swap(i, i + step);
            }
        }
        step = (step as f64 / 1.247) as usize;
    }
    bubble_sort(slice);
}
