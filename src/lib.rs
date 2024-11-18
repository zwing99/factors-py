use std::sync::Mutex;

use pyo3::prelude::*;
use rayon::prelude::*;

#[pyclass]
struct FactorsResult {
    #[pyo3(get)]
    factors: Vec<u128>,
    #[pyo3(get)]
    total: u128,
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn factors(number: u128) -> PyResult<FactorsResult> {
    let factors = _factors(number);
    let factors = FactorsResult {
        factors: factors.0,
        total: factors.1,
    };
    return Ok(factors);
}

fn _factors(number: u128) -> (Vec<u128>, u128) {
    let factors = Mutex::new(Vec::<u128>::new());
    let to_check = 1..=(number / 2);
    to_check.clone().into_par_iter().for_each(|i| {
        if (number % i) == 0 {
            {
                let mut factors = factors.lock().unwrap();
                factors.push(i);
            }
        }
    });
    factors.lock().unwrap().push(number); // add the input number to the factors list

    let mut factors = factors.lock().unwrap();
    factors.sort();

    let factors = (factors.clone(), to_check.clone().count() as u128);

    return factors;
}

/// A Python module implemented in Rust.
#[pymodule]
fn factors_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(factors, m)?)?;
    m.add_class::<FactorsResult>()?;
    Ok(())
}

#[cfg(test)]
mod tests;
