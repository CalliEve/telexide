mod form_data;
pub mod macros;
pub mod result;

pub(crate) use form_data::{encode_multipart_form_data, AsFormData, FormDataFile, BOUNDARY};
