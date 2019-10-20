use std::time::{Instant};
use std::thread;

fn find_largest(slice: &[i32]) -> i32{
    let mut tmp = 0;
    for i in 0..slice.len() {
        if slice[i] > tmp {
            tmp = slice[i];
        }
    }
    tmp
}

fn find_largest_parallel(slice: &'static[i32]) -> i32{
    let mut tmp: [i32; 2] = [0, 0];
    let first_half_handle = thread::spawn(move||
        find_largest(&slice[0..slice.len()/2]));
    let second_half_handle = thread::spawn(move||
        find_largest(&slice[slice.len()/2+1..slice.len()-1]));

    tmp[0] = first_half_handle.join().unwrap();
    tmp[1] = second_half_handle.join().unwrap();

    find_largest(&tmp)
}

fn main() {
    static test:[i32; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

    let mut time_start = Instant::now();
    let mut largest_first = find_largest(&test);
    let mut time_delta = Instant::now() - time_start;
    println!("Single Threaded: Largest member of test is {}, it took {} nano seconds."
             , largest_first, time_delta.as_nanos());

    time_start = Instant::now();
    largest_first = thread::spawn(move||find_largest(&test)).join().unwrap();
    time_delta = Instant::now() - time_start;
    println!("Concurrent: Largest member of test is {}, it took {} nano seconds."
             , largest_first, time_delta.as_nanos());

    time_start = Instant::now();
    largest_first = find_largest_parallel(&test);
    time_delta = Instant::now() - time_start;
    println!("Parallel: Largest member of test is {}, it took {} nano seconds."
             , largest_first, time_delta.as_nanos());
}
