use rand::Rng;
use std::env::args;

fn main() {
    let mut sort_alg = String::new();
    for arg in args() {
        sort_alg = arg
    }

    let mut rng = rand::rng();
    let mut numbers: Vec<i32> = (0..10).map(|_| rng.random_range(0..100)).collect();
    println!("{:?}", numbers);

    match sort_alg.as_str() {
        "bubble-sort" => bubble_sort(&mut numbers),
        "selection-sort" => selection_sort(&mut numbers),
        "insertion-sort" => insertion_sort(&mut numbers),
        _ => selection_sort(&mut numbers),
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
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }

        // for j in (1..=i) {
        //     if arr[j] < arr[j-1] {
        //         arr.swap(j, j-1);
        //     }
        // }
    }
}
