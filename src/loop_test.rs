mod corr;
mod float_sum;

use polars::prelude::*;
use std::io::{self, Write};

use dioxus_devtools::{connect_subsecond};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use subsecond::{call, HotFn};



#[unsafe(no_mangle)]
#[inline(never)]
pub fn my_hot_function_test(a: &Series, b: &Series) -> Option<f64> {
    let value = corr::correlation_fn(a, b);
    value
}

fn main() {
    connect_subsecond();

    let hotfn = HotFn::current(my_hot_function_test);
    let original_addr: u64 = hotfn.ptr_address().0;
    println!("my_hot_function_test initial address: HotFnPtr({})", original_addr);


    loop {
        // Wait for user to press Enter
        print!("▶ Press Enter to execute...");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        // Perfect positive correlation (slope = 1)
        let a_pos = Series::new("a".into(), &[1, 2, 3, 4, 5]);
        let b_pos = Series::new("b".into(), &[1, 2, 3, 4, 5]);

        // 2️Perfect negative correlation (slope = -1)
        let a_neg = Series::new("a".into(), &[1, 2, 3, 4, 5]);
        let b_neg = Series::new("b".into(), &[5, 4, 3, 2, 1]);

        // 3️No correlation / constant series
        let a_zero = Series::new("a".into(), &[1, 2, 3, 4, 5]);
        let b_zero = Series::new("b".into(), &[3, 3, 3, 3, 3]);

        // Helper to compute and unwrap the correlation
        fn corr(s1: &Series, s2: &Series) -> f64 {
            call(|| my_hot_function_test(s1, s2)).unwrap_or(f64::NAN)
        }

        let pos_corr = corr(&a_pos, &b_pos);
        let neg_corr = corr(&a_neg, &b_neg);
        let zero_corr = corr(&a_zero, &b_zero);

        println!("Positive correlation : {:.10}", pos_corr);
        println!("Negative correlation : {:.10}", neg_corr);
        println!("Constant series      : {:.10}", zero_corr);
        println!();

    }
}