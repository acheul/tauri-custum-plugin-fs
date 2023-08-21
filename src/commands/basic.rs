use std::process::Command;
use std::fs;

use crate::error::Result;



#[tauri::command]
pub fn open(path: &str) -> Result<()> { // path would be either directory's or file's.

  #[cfg(windows)]
  let _ = Command::new("explorer").arg(path).spawn()?;

  #[cfg(linux)]
  let _ = Command::new("xdg-open").arg(path).spawn()?;

  #[cfg(unix)]
  let _ = Command::new("open").arg(path).spawn()?;

  Ok(())
}


#[tauri::command]
pub fn rename(from: &str, to: &str) -> Result<()> {

  let _ = fs::rename(from, to)?;
  Ok(())
}


#[tauri::command]
pub fn remove_dir(path: &str) -> Result<()> {

  let _ = fs::remove_dir(path)?;

  Ok(())
}

#[tauri::command]
pub fn remove_file(path: &str) -> Result<()> {

  let _ = fs::remove_file(path)?;

  Ok(())
}


#[tauri::command]
pub fn create_dir(path: &str) -> Result<()> {
  let _ = fs::create_dir(path)?;
  Ok(())
}

#[tauri::command]
pub fn create_file(path: &str) -> Result<()> {
  let _ = fs::write(path, b"")?;
  Ok(())
}