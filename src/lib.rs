pub mod corr;
pub mod float_sum;

use polars::prelude::*;
use pyo3::prelude::*;

#[pyfunction]
fn correlation_request() ->  PyResult<i64> {
    // Perfect positive correlation (slope = 1)
    let a_pos = Series::new("a".into(), &[1, 2, 3, 4, 5]);
    let b_pos = Series::new("b".into(), &[1, 2, 3, 4, 5]);

    // Perfect negative correlation (slope = -1)
    let a_neg = Series::new("a".into(), &[1, 2, 3, 4, 5]);
    let b_neg = Series::new("b".into(), &[5, 4, 3, 2, 1]);

    // No correlation (random numbers)
    let a_zero = Series::new("a".into(), &[1, 2, 3, 4, 5]);
    let b_zero = Series::new("b".into(), &[3, 3, 3, 3, 4]);

    // Helper to compute and unwrap the correlation
    fn corr(s1: &Series, s2: &Series) -> f64 {
        corr::correlation_fn(s1, s2).unwrap()
    }

    assert!((corr(&a_pos, &b_pos) - 1.0).abs() < 1e-9);
    assert!((corr(&a_neg, &b_neg) + 1.0).abs() < 1e-9);
    assert!(corr(&a_zero, &b_zero).is_nan()); // constant series → NaN
    Ok(1)
}

#[pymodule]
fn polar_subsecond(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(correlation_request, m)?)?;
    Ok(())
}
