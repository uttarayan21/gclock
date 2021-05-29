use pyo3::{types::PyModule, PyObject, PyResult, Python};

pub fn speak<S: AsRef<str>>(text: S) -> PyResult<()> {
    Python::with_gil(|py| {
        let google_speech = PyModule::import(py, "google_speech")?;
        let speech: PyObject = google_speech
            .call1("Speech", (text.as_ref(), "en-us"))?
            .extract()?;
        speech.call_method0(py, "play")?;
        Ok(())
    })
}
