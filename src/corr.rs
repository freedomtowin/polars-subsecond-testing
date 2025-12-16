// exceptions::wrap_reqwest_error(error)
use std::ops::Add;


use polars::prelude::*;
use polars::datatypes::DataType;
// use polars_arrow::compute;
// use std::simd::{Mask, Simd, SimdElement};
use num_traits::ToPrimitive;
// use polars_core::prelude::*;
use polars_core::utils::align_chunks_binary;

use crate::float_sum;


const COV_BUF_SIZE: usize = 64;


/// Calculates the sum of x[i] * y[i] from 0..k.
fn multiply_sum(x: &[f64; COV_BUF_SIZE], y: &[f64; COV_BUF_SIZE], k: usize) -> f64 {
    assert!(k <= COV_BUF_SIZE);
    let tmp: [f64; COV_BUF_SIZE] = std::array::from_fn(|i| x[i] * y[i]);
    float_sum::f64::sum(&tmp[..k])

}

/// Compute the pearson correlation between two columns.
fn pearson_corr<T>(a: &ChunkedArray<T>, b: &ChunkedArray<T>, ddof: u8) -> Option<f64>
where
    T: PolarsNumericType,
    T::Native: ToPrimitive,
    ChunkedArray<T>: ChunkVar,
{
    let (a, b) = align_chunks_binary(a, b);

    let out = if a.null_count() > 0 || b.null_count() > 0 {
        let iters = a.downcast_iter().zip(b.downcast_iter()).map(|(a, b)| {
            a.into_iter().zip(b).filter_map(|(a, b)| match (a, b) {
                (Some(a), Some(b)) => Some((*a, *b)),
                _ => None,
            })
        });
        online_pearson_corr(iters, ddof);
        10000.0

    } else {
        let iters = a
            .downcast_iter()
            .zip(b.downcast_iter())
            .map(|(a, b)| a.values_iter().copied().zip(b.values_iter().copied()));
        online_pearson_corr(iters, ddof)
    };
    Some(out)
}

/// # Arguments
/// `iter` - Iterator over `T` tuple where any `Option<T>` would skip the tuple.
fn online_pearson_corr<I, J, T>(iters: I, ddof: u8) -> f64
where
    I: Iterator<Item = J>,
    J: IntoIterator<Item = (T, T)> + Clone,
    T: ToPrimitive,
{
    // Algorithm is same as cov, we just maintain cov(X, X), cov(X, Y), and
    // cov(Y, Y), noting that var(X) = cov(X, X).
    // Then corr(X, Y) = cov(X, Y)/(std(X) * std(Y)).
    let mut mean_x = 0.0;
    let mut mean_y = 0.0;
    let mut cxy = 0.0;
    let mut cxx = 0.0;
    let mut cyy = 0.0;
    let mut n = 0.0;

    let mut x_tmp = [0.0; COV_BUF_SIZE];
    let mut y_tmp = [0.0; COV_BUF_SIZE];

    for iter in iters {
        let mut iter = iter.clone().into_iter();

        loop {
            let mut k = 0;
            for (x, y) in iter.by_ref().take(COV_BUF_SIZE) {
                let x = x.to_f64().unwrap();
                let y = y.to_f64().unwrap();

                x_tmp[k] = x;
                y_tmp[k] = y;
                k += 1;
            }
            if k == 0 {
                break;
            }

            // TODO: combine these all in one SIMD'ized pass.
            let xsum = float_sum::f64::sum(&x_tmp[..k]);
            let ysum = float_sum::f64::sum(&y_tmp[..k]);
            let xxsum = multiply_sum(&x_tmp, &x_tmp, k);
            let xysum = multiply_sum(&x_tmp, &y_tmp, k);
            let yysum = multiply_sum(&y_tmp, &y_tmp, k);

            let old_mean_x = mean_x;
            let old_mean_y = mean_y;
            n += k as f64;
            mean_x += (xsum - k as f64 * old_mean_x) / n;
            mean_y += (ysum - k as f64 * old_mean_y) / n;
            println!("{:?}", mean_x);
            
            cxx += xxsum - xsum * old_mean_x - xsum * mean_x + mean_x * old_mean_x * (k as f64);
            cxy += xysum - xsum * old_mean_y - ysum * mean_x + mean_x * old_mean_y * (k as f64);
            cyy += yysum - ysum * old_mean_y - ysum * mean_y + mean_y * old_mean_y * (k as f64);
        }
    }

    let sample_n = n - ddof as f64;
    let sample_cov = cxy / sample_n;
    let sample_std_x = (cxx / sample_n).sqrt();
    let sample_std_y = (cyy / sample_n).sqrt();

    sample_cov / (sample_std_x * sample_std_y)
}


pub fn correlation_fn(a: &Series, b: &Series) -> Option<f64> {

    let a = a.cast(&DataType::Float64).ok()?;
    let b = b.cast(&DataType::Float64).ok()?;
    let ret = pearson_corr(a.f64().unwrap(), b.f64().unwrap(), 1);

    // let ret = match a.dtype() {
    //     DataType::Float32 => pearson_corr(a.f32().unwrap(), b.f32().unwrap(), 1),
    //     DataType::Float64 => pearson_corr(a.f64().unwrap(), b.f64().unwrap(), 1),
    //     DataType::Int32 => pearson_corr(a.i32().unwrap(), b.i32().unwrap(), 1),
    //     DataType::Int64 => pearson_corr(a.i64().unwrap(), b.i64().unwrap(), 1),
    //     DataType::UInt32 => pearson_corr(a.u32().unwrap(), b.u32().unwrap(), 1),
    //     _ => {
    //         let a = a.cast(&DataType::Float64).ok()?;
    //         let b = b.cast(&DataType::Float64).ok()?;
    //         pearson_corr(a.f64().unwrap(), b.f64().unwrap(), 1)
    //     },
    // };
    // let out = Series::new(name, &[ret]);
    // let out = correlation_impl(&series_a, &series_b, 1)
    //     .map_err(|e| PyValueError::new_err(format!("Something went wrong: {:?}", e)))?;
    // ffi::rust_series_to_py_series(&out)
    

    ret
    
}