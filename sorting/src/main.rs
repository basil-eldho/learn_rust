use rand::Rng;
use std::env::args;

fn main() {
    let mut sort_alg = String::new();
    for arg in args() {
        sort_alg = arg
    }

    let mut rng = rand::rng();
    let mut numbers: Vec<i32> = (0..7).map(|_| rng.random_range(0..100)).collect();
    let len = numbers.len();
    println!("{:?}", numbers);

    match sort_alg.as_str() {
        "bubble-sort" => bubble_sort(&mut numbers),
        "selection-sort" => selection_sort(&mut numbers),
        "insertion-sort" => insertion_sort(&mut numbers),
        "merge-sort" => merge_sort(&mut numbers),
        _ => merge_sort(&mut numbers),
    }
    println!("{:?}", numbers);
}

fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 0..arr.len() - 1 {
        let mut swapped = false;
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

fn selection_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 0..arr.len() - 1 {
        let mut min = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[min] {
                min = j;
            }
        }
        if i != min {
            arr.swap(i, min);
        }
    }
}

fn insertion_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 1..arr.len() {
        for j in 1..=i {
            if arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
            }
        }
    }
}

fn merge_sort<T: PartialOrd + Clone>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();
    merge_sort(&mut left);
    merge_sort(&mut right);
    merge(&left, &right, arr)
}

fn merge<T: PartialOrd + Clone>(left: &[T], right: &[T], arr: &mut [T]) {
    let (mut i, mut j) = (0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[i + j] = left[i].clone();
            i += 1;
        } else {
            arr[i + j] = right[j].clone();
            j += 1;
        }
    }

    while i < left.len() {
        arr[i + j] = left[i].clone();
        i += 1;
    }

    while j < right.len() {
        arr[i + j] = right[j].clone();
        j += 1;
    }
}
