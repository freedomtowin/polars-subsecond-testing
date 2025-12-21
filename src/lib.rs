pub mod corr;
pub mod float_sum;

use polars::prelude::*;
use pyo3::prelude::*;
use pyo3_polars::{PySeries, PyDataFrame};
use pyo3::exceptions::{PyTypeError, PyRuntimeError};

#[pyfunction]
fn correlation_request(a: PySeries, b: PySeries) ->  PyResult<f64> {
    corr::correlation_fn(a, b)
}

#[pymodule]
fn polar_subsecond(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(correlation_request, m)?)?;
    Ok(())
}
