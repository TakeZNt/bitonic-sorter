use num_cpus;

use bitonic_sorter::SortOrder;
use bitonic_sorter::third::sort as seq_sort;
use bitonic_sorter::fourth::sort as par_sort;
use bitonic_sorter::utils::{is_sorted_ascending, new_u32_vec};

use std::{env, f64};
use std::str::FromStr;
use std::time::Instant;

fn main() {
    // 第一引数を整数として取得する。取得した整数をnとし、2^n個の整数を生成し、ソートする
    if let Some(n) = env::args().nth(1) {
        let bits = u32::from_str(&n).expect("error parsing argument");
        run_sorts(bits);
    }
    else {
        eprintln!("Usage {} <number of elements in bits>", env::args().nth(0).unwrap());
        std::process::exit(1);
    }
}

fn run_sorts(bits: u32) {
    let len = 1 << bits;

    println!(
        "sorting {} integers ({:.1} MB)", 
        len, 
        (len * std::mem::size_of::<u32>()) as f64 / 1024.0 / 1024.0
    );

    println!(
        "cpu info: {} physical cores, {} logical cores",
        num_cpus::get_physical(),
        num_cpus::get()
    );

    let seq_duration = time_sort(&seq_sort, len, "seq_sort");

    let par_duration = time_sort(&par_sort, len, "par_sort");

    println!("Speed up: {:.2}x", seq_duration / par_duration);
}

fn time_sort<F>(sorter: &F, len: usize, name: &str) -> f64 
    where F: Fn(&mut [u32], &SortOrder) -> Result<(), String>
{
    let mut array = new_u32_vec(len);

    let start = Instant::now();

    sorter(&mut array, &SortOrder::Ascending).expect("Failed to sort: ");

    let dur = start.elapsed();

    let nano_secs = dur.subsec_nanos() as f64 + dur.as_secs() as f64 * 1e9_f64;
    println!("{}: sorted {} integers in {} seconds", name, len, nano_secs / 1e9);

    assert!(is_sorted_ascending(&array));

    nano_secs
}