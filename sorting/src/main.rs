fn main() {
    // bubble sort
    let input = [5, 8, 95, 6, 844, 1, 2, 5, 56];
    println!("Hello, world!");
}

// Bubble Sort in Rust (in-place, generic)
fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}