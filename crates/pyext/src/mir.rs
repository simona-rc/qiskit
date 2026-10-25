// This code is part of Qiskit.
//
// (C) Copyright IBM 2026
//
// This code is licensed under the Apache License, Version 2.0. You may
// obtain a copy of this license in the LICENSE.txt file in the root directory
// of this source tree or at https://www.apache.org/licenses/LICENSE-2.0.
//
// Any modifications or derivative works of this code must retain this
// copyright notice, and modified files need to carry a notice indicating
// that they have been altered from the originals.

use pyo3::prelude::*;
use qiskit_circuit::dag_circuit::PyDAGCircuit;
use qiskit_mir::lowering::pass_manager::lower as rust_lower;
use qiskit_mir::mir::Mir as RustMir;

#[pyclass(subclass, name = "Mir", module = "qiskit._accelerate.mir")]
#[derive(Debug)]
pub struct PyMir(pub RustMir);

#[pymethods]
impl PyMir {
    #[new]
    pub fn new() -> Self {
        PyMir(RustMir::new())
    }

    fn __repr__(&self) -> String {
        "Mir()".to_string()
    }
}

#[pyfunction(name = "lower")]
pub fn py_lower(dag: &PyDAGCircuit) -> PyResult<PyMir> {
    let dag_ref = dag.try_read()?;
    Ok(PyMir(rust_lower(dag_ref)))
}

#[pymodule(name = "mir")]
pub fn mir_mod(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyMir>()?;
    m.add_function(wrap_pyfunction!(py_lower, m)?)?;
    Ok(())
}
