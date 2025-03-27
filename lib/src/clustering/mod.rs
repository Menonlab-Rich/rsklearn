pub mod dbscan;

use pyo3::prelude::*;

pub fn init_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<dbscan::DBScan>()?;
    Ok(())
}
