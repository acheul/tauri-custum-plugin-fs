mod entry;
mod basic;
mod read;

use entry::*;
use basic::*;
use read::*;

use tauri::{
  plugin::{Builder, TauriPlugin},
  Runtime,
  // AppHandle,
};


pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("fs")
    .invoke_handler(tauri::generate_handler![
      read_a_dir, // read_first_dir,
      read_dir,
      read_dir_lvs, read_dir_recursive,
      // basic
      open, rename, remove_dir, remove_file, create_dir, create_file,
      // read
      read_file,
    ])
    .build()
}
