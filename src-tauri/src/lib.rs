mod scanner;
use scanner::{Manifest,Diff};
#[tauri::command]
fn snapshot(path: String) -> Result<Manifest,String> { scanner::snapshot(&path) }
#[tauri::command]
fn verify(manifest: Manifest) -> Result<Diff,String> { scanner::verify(&manifest) }
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
 tauri::Builder::default()
  .plugin(tauri_plugin_dialog::init())
  .invoke_handler(tauri::generate_handler![snapshot,verify])
  .run(tauri::generate_context!())
  .expect("Error running desktop application");
}