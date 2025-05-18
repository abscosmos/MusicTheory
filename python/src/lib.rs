use pyo3::prelude::*;

mod implementation;

use implementation as py;

#[pymodule]
fn music_theory(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<py::Pitch>()?;
    m.add_class::<py::Letter>()?;
    m.add_class::<py::AccidentalSign>()?;
    m.add_class::<py::Interval>()?;
    
    Ok(())
}