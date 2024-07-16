use pyo3::prelude::*;
use pyo3::types::PyDict;

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let d = PyDict::new_bound(py);
        d.set_item("hello", "world").unwrap();
        dbg!(d.get_item("hello").unwrap().unwrap());
        Ok(())
    })
}
