use super::result::{Result, TelegramError};
use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;
use serde_json::{Value, Map};

#[derive(Debug, Clone, PartialEq)]
pub struct FormDataFile {
    pub bytes: Vec<u8>,
    pub name: String,
    pub file_name: Option<String>,
    pub media_type: Option<String>,
}

impl FormDataFile {
    pub fn new(bytes: &[u8], media_type: &str, file_name: &str) -> Self {
        Self {
            bytes: bytes.to_vec(),
            name: file_name.splitn(1, '.').collect::<Vec<&str>>().first().unwrap_or(&"new_file").to_owned().to_owned(),
            media_type: Some(media_type.to_owned()),
            file_name: Some(file_name.to_owned())
        }
    }

    pub fn new_from_file(file: &mut File, file_name: &str) -> Result<Self> {
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;

        Ok(Self {
            bytes,
            name: file_name.splitn(1, '.').collect::<Vec<&str>>().first().unwrap_or(&"new_file").to_owned().to_owned(),
            file_name: Some(file_name.to_owned()),
            media_type: Some(get_media_type(file_name)?.to_owned()),
        })
    }
}

pub static BOUNDARY: &str = "----------telexide-form-data-boundary";

pub fn encode_multipart_form_data(files: &[FormDataFile]) -> Result<Vec<u8>> {
    let mut data = Vec::new();

    for file in files {
        write!(&mut data, "--{}\r\n", BOUNDARY)?;

        if file.file_name.is_some() {
            write!(&mut data, "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n", file.name, file.file_name.as_ref().unwrap())?;
        } else {
            write!(&mut data, "Content-Disposition: form-data; name=\"{}\"\r\n", file.name)?;
        }

        if file.media_type.is_some() {
            write!(&mut data, "Content-Type: {}\r\n", file.media_type.as_ref().unwrap())?;
        }

        write!(&mut data, "\r\n")?;

        file.bytes.as_slice().read_to_end(&mut data)?;

        write!(&mut data, "\r\n")?;
    }

    write!(&mut data, "--{}--\r\n", BOUNDARY)?;

    Ok(data)
}

pub fn encode_file_as_multipart_form_data(mut file: &mut File, file_name: &str) -> Result<Vec<u8>> {
    encode_multipart_form_data(&[FormDataFile::new_from_file(&mut file, file_name)?])
}

fn get_media_type(file_name: &str) -> Result<&str> {
    let ext: &str = if let Some(ext) = Path::new(file_name).extension() {
        ext.to_str().ok_or_else(|| TelegramError::InvalidArgument("file name contained invalid characters".to_owned()))?
    } else {
        ""
    };

    Ok(match ext {
        "png" => "image/png",
        "gif" => "image/gif",
        "jpg" | "jpeg" => "image/jpeg",
        "csv" => "text/csv",
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "json" => "application/json",
        "pdf" => "application/pdf",
        "zip" => "application/zip",
        "m4a" => "audio/mp4",
        "mp3" => "audio/mpeg",
        ".ogg" => "audio/ogg",
        _ => "text/plain"
    })
}

pub trait AsFormData {
    fn as_form_data(&self) -> Result<Vec<FormDataFile>>;
}

impl AsFormData for Value {
    fn as_form_data(&self) -> Result<Vec<FormDataFile>> {
        let map: Map<String, Value> = self.as_object()
            .ok_or_else(
                || TelegramError::InvalidArgument(
                    "Toplevel part of form-data has to be a struct".into()
                )
            )?.clone();

        let mut res = Vec::new();

        for (key, value) in map.into_iter() {
            if value.is_null() {
                continue
            } else {
                res.push(
                    FormDataFile {
                        name: key,
                        file_name: None,
                        media_type,
                        bytes: serde_json::to_string(&value)?.trim_matches('"').as_bytes().to_vec()
                    }
                )
            }
        }

        println!("{}", String::from_utf8_lossy(res.last().as_ref().unwrap().bytes.as_slice()));

        Ok(res)
    }
}
