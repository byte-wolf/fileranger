use tauri_plugin_fs::FsExt;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            // allowed the given directory
            let scope = app.fs_scope();
            let _ = scope.allow_directory("C:\\", true);
            dbg!(scope.allowed_patterns());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
