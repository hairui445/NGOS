
use ngos_runtime::Runtime;
use pyo3::prelude::*;

#[pyclass]
struct Device {
    name: String,
}

#[pymethods]
impl Device {
    #[getter]
    fn name(&self) -> &str {
        &self.name
    }

    fn execute(&self, values: Vec<f32>) -> Vec<f32> {
        values
    }
}

#[pyclass]
struct Tensor {
    values: Vec<f32>,
}

#[pymethods]
impl Tensor {
    #[new]
    fn new(values: Vec<f32>) -> Self {
        Self { values }
    }

    fn tolist(&self) -> Vec<f32> {
        self.values.clone()
    }

    fn len(&self) -> usize {
        self.values.len()
    }
}

#[pyfunction]
fn get_device() -> Device {
    let runtime = Runtime::new();
    let device = runtime
        .discover_devices()
        .into_iter()
        .next()
        .map(|item| item.name)
        .unwrap_or_else(|| "CPU Runtime".to_string());
    Device { name: device }
}

#[pyfunction]
fn tensor(values: Vec<f32>) -> Tensor {
    Tensor::new(values)
}

#[pymodule]
fn ngos(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<Device>()?;
    module.add_class::<Tensor>()?;
    module.add_function(wrap_pyfunction!(get_device, module)?)?;
    module.add_function(wrap_pyfunction!(tensor, module)?)?;
    Ok(())
}

