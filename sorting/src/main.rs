use rand::Rng;
use std::time::SystemTime;

fn main() {
    let mut rng = rand::rng();
    let mut numbers: Vec<i32> = (0..=50).map(|_| rng.random_range(0..1000)).collect(); // bubble sort
    let now = SystemTime::now();
    bubble_sort(&mut numbers);
    println!("Elapsed time: {:?}", now.elapsed());
    println!("{:?}", numbers);
}

fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 0..arr.len().saturating_sub(1) {
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
    for i in 0..arr.len() {
        let mut min = i;
        for j in (i + 1)..arr.len() {
            if arr[j] < arr[min] {
                min = j;
            }
        }
        if i != min {
            arr.swap(i, min);
        }
    }
}

fn quick_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let pivot_index = partition(arr);
    quick_sort(&mut arr[..pivot_index]); // left partition
    quick_sort(&mut arr[pivot_index + 1..]); // right partition
}

fn partition(arr: &mut [i32]) -> usize {
    let len = arr.len();
    let pivot = arr[len - 1]; // last element as pivot
    let mut i = 0;

    for j in 0..len - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, len - 1); // put pivot in correct position
    i
}
