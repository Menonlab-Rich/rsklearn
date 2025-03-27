use numpy::{PyArray1, PyReadonlyArray2};
use petal_clustering::{Dbscan, Fit};
use petal_neighbors::distance::*;
use pyo3::prelude::*;

#[pyclass]
pub struct DBScan {
    eps: f32,
    min_samples: usize,
    metric: String,
}

#[pymethods]
impl DBScan {
    #[new]
    fn new(eps: f32, min_samples: usize, metric: &str) -> Self {
        DBScan {
            eps,
            min_samples,
            metric: metric.to_string(),
        }
    }

    fn fit<'py>(
        &self,
        py: Python<'py>,
        data: PyReadonlyArray2<f32>,
    ) -> PyResult<Py<PyArray1<i32>>> {
        // TODO implement other metrics
        let arr_data = data.as_array();
        let labels = Dbscan::new(self.eps, self.min_samples, Euclidean::default()).fit(&arr_data);

        Ok(PyArray1::from_vec(py, labels).unbind())
    }
}
