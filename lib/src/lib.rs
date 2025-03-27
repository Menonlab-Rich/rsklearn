pub mod clustering;

use pyo3::prelude::*;

#[pymodule]
fn rsklearn(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add the clustering submodule
    let clustering_module = PyModule::new(_py, "clustering")?;
    clustering::init_module(&clustering_module)?;
    m.add_submodule(&clustering_module)?;
    Ok(())
}
