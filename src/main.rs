use pyo3::prelude::*;
use pyo3::types::PyDict;

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let module = py.import_bound("hash_errors")?;
        let class = module.getattr("HashErrors")?;
        let instance = class.call0()?;
        let d = PyDict::new_bound(py);
        d.set_item("hello", "world").unwrap();
        dbg!(d.get_item("hello").unwrap().unwrap());
        dbg!(d.get_item(instance)?);
        Ok(())
    })
}
