use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn detect(input: Vec<f64>) -> bool {
    // Example: placeholder logic
    input.iter().any(|&x| x > 0.5)
}

#[pymodule]
fn trustml(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(detect, m)?)?;
    Ok(())
}
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
