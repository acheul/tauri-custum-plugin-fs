use serde::Serialize;
use std::fs::{self, Metadata};
use std::path::{Path, PathBuf};
use filetime::FileTime;
use chrono::{DateTime, Duration, Local};

#[cfg(windows)]
use std::os::windows::fs::MetadataExt;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(linux)]
use std::os::linux::fs::MetadataExt;

use crate::error::{Result, Error};

#[derive(Debug, Serialize)]
pub struct DiskEntry {
  pub path: String,
  pub name: String,
  pub is_dir: bool,
  pub ctime: String, // 2023-05-19 12:10:37 +09:00
  pub mtime: String,
  pub fid1: u32,
  pub fid2: u32,
  pub sub: Option<Vec<DiskEntry>>, // recursive option
}

/*
* Notice: In case of lvs=1, gets a 2 level recursive hierarchy.
*/
fn read_dir_option<P: AsRef<Path>>(path: P, recursive: bool, lvs: u8) 
-> Result<Vec<DiskEntry>>
{
  if recursive && lvs>0 {
    return Err(Error::Etc("read_dir_option--recursive && lvs".to_string()));
  }

  let mut entries: Vec<DiskEntry> = Vec::new();

  for entry in fs::read_dir(&path)? {
    let entry: fs::DirEntry = entry?;
    let path: PathBuf = entry.path();
    let name: String = path.file_name().map(|name| name.to_string_lossy().to_string()).or(Some("".to_string())).unwrap();
    if let Ok(meta) = fs::metadata(path.clone()) {
      let (is_dir, ctime, mtime, fid1, fid2) = get_meta(meta);

      entries.push(
        DiskEntry{
          path: path.display().to_string(),
          name: name,
          is_dir: is_dir,
          ctime: ctime,
          mtime: mtime,
          fid1: fid1,
          fid2: fid2,
          sub: if !is_dir { None } else {
            Some(
              if recursive {
                read_dir_option(&path, recursive, lvs)?
              } else if lvs==0 {
                Vec::new()
              } else {
                read_dir_option(&path, recursive, lvs-1)?
              }
            )
          },
        }
      )
    }
  }

  Ok(entries)
}

// Get a directory's information (not entries)
pub fn get_dir(path: &str) -> Result<DiskEntry> {

  let path = PathBuf::from(path);
  let name: String = path.file_name().map(|name| name.to_string_lossy().to_string()).or(Some("".to_string())).unwrap();

  if let Ok(meta) = fs::metadata(path.clone()) {
    let (is_dir, ctime, mtime, fid1, fid2) = get_meta(meta);

    Ok(DiskEntry {
      path: path.display().to_string(),
      name: name,
      is_dir: is_dir,
      ctime: ctime,
      mtime: mtime,
      fid1: fid1,
      fid2: fid2,
      sub: if !is_dir { None } else { Some(Vec::new()) },
    })
  } else {
    Err(Error::Etc("get_dir".to_string()))
  }
}

fn get_meta(meta: Metadata) -> (bool, String, String, u32, u32) {
  let is_dir: bool = meta.is_dir();
  let ctime_sec = match FileTime::from_creation_time(&meta) {
    Some(ctime) => ctime.seconds(),
    None => 0,
  };
  let mtime_sec = FileTime::from_last_modification_time(&meta).seconds();
  // let atime_sec = FileTime::from_last_access_time(&meta).seconds();

  let ctime = sec_to_datetime(ctime_sec); 
  let mtime = sec_to_datetime(mtime_sec); 

  #[cfg(windows)]
  let fid: u64 = match meta.file_index() {
    Some(k) => k,
    None => 0,
  };
  #[cfg(linux)]
  let fid: u64 = meta.st_ino();
  #[cfg(unix)]
  let fid: u64 = meta.ino();
  #[cfg(not(any(windows, linux, unix)))]
  let fid: u64 = 0;

  let fid2 = fid>>32;
  let fid1 = (fid-(fid2<<32)) as u32;
  let fid2 = fid2 as u32;

  return (is_dir, ctime, mtime, fid1, fid2);
}

//const UNIX_EPOCH: &str = "1970-01-01T00:00:00Z";
//const WINDOW_EPOCH: &str = "1601-01-01T00:00:00Z";

fn sec_to_datetime(sec: i64) -> String {

  #[cfg(windows)]
  let t = DateTime::parse_from_rfc3339("1601-01-01T00:00:00Z").unwrap() + Duration::seconds(sec);
  #[cfg(not(windows))]
  let t = DateTime::parse_from_rfc3339("1970-01-01T00:00:00Z").unwrap() + Duration::seconds(sec);

  let local_t: DateTime<Local> = DateTime::from(t);
  local_t.to_string() // "2023-05-19 12:10:37 +09:00"
}


// Commands //

#[tauri::command]
pub fn read_a_dir(path: String) -> Result<DiskEntry> {
  get_dir(&path)
}

/* Fill the DiskEntry.sub field.  */
/* #[tauri::command]
fn read_first_dir(path: String) -> Result<DiskEntry> {
  let mut entry = get_dir(&path)?;
  entry.sub = Some(read_dir_option(&path, false, 0)?);
  return Ok(entry);
} */

// Not recursive.
#[tauri::command]
pub fn read_dir(path: String) -> Result<Vec<DiskEntry>> {
  Ok(read_dir_option(&path, false, 0)?)
} 

// Not recursive - but conducts recursion within some levels.
#[tauri::command]
pub fn read_dir_lvs(path: String, lvs: Option<u8>)
-> Result<Vec<DiskEntry>> {
  let recursive = false;
  let lvs: u8 = match lvs {
    Some(u) => u,
    None => 0,
  };
  Ok(read_dir_option(&path, recursive, lvs)?)
} 

// recursive.
#[tauri::command]
pub fn read_dir_recursive(path: String)
-> Result<Vec<DiskEntry>> {
  let recursive: bool = true;
  let lvs: u8 = 0;
  Ok(read_dir_option(&path, recursive, lvs)?)
} 