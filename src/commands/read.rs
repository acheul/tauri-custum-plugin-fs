use std::fs;

use crate::error::{Result, Error};


// this is temporary
#[tauri::command]
pub fn read_file(path: &str) -> Result<String> {

  let res = fs::read(path)?;
  let s = String::from_utf8(res).map_err(|_| Error::Etc("Utf8Error".to_string()))?;

  return Ok(s);
}