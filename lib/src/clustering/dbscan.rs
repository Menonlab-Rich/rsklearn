use numpy::{PyArray1, PyReadonlyArray2};
use petal_clustering::{Dbscan, Fit};
use petal_neighbors::distance::*;
use pyo3::prelude::*;

#[pyclass]
pub struct DBScan {
    eps: f32,
    min_samples: usize,
    metric: String,
    _chunk_size: usize,
}

#[pymethods]
impl DBScan {
    #[new]
    fn new(eps: f32, min_samples: usize, metric: &str) -> Self {
        DBScan {
            eps,
            min_samples,
            metric: metric.to_string(),
            _chunk_size: 10000,
        }
    }

    fn fit<'py>(
        &self,
        py: Python<'py>,
        data: PyReadonlyArray2<f32>,
    ) -> PyResult<Py<PyArray1<i32>>> {
        // TODO implement other metrics
        let arr_data = data.as_array();
        let mut dbscan = Dbscan::new(self.eps, self.min_samples, Euclidean::default());
        dbscan.set_chunk_size(self._chunk_size);
        let labels = dbscan.fit(&arr_data);
        Ok(PyArray1::from_vec(py, labels).unbind())
    }

    fn set_chunk_size<'py>(&mut self, _py: Python<'py>, size: usize) {
        self._chunk_size = size;
    }
}
