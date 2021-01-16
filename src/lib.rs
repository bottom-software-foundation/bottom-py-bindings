use bottomify::bottom::{decode_string, encode_string};

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;


#[pymodule]
fn bottom(_py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "encode")]
    fn encode_string_py(_py: Python, string: String) -> String {
        encode_string(&string) 
    }
    
    #[pyfn(m, "decode")]
    fn decode_string_py(_py: Python, string: String) -> PyResult<String> {
        match decode_string(&string) {
            Err(e) => return Err(PyValueError::new_err(e.why)),
            Ok(o) => return Ok(o),
        }
    }

    Ok(())
}
