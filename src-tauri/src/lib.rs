use tauri::Emitter;
use tauri_plugin_updater::UpdaterExt;

#[tauri::command]
async fn install_update(app: tauri::AppHandle) -> Result<(), String> {
    let Some(update) = app
        .updater()
        .map_err(|e| e.to_string())?
        .check()
        .await
        .map_err(|e| e.to_string())?
    else {
        return Ok(());
    };
    update
        .download_and_install(|_, _| {}, || {})
        .await
        .map_err(|e| e.to_string())?;
    app.restart();
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![install_update])
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                match check_for_update(&handle).await {
                    Ok(Some(version)) => {
                        let _ = handle.emit("update-available", version);
                    }
                    Ok(None) => {}
                    Err(e) => eprintln!("[updater] check failed: {e}"),
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running CareerOS");
}

async fn check_for_update(
    app: &tauri::AppHandle,
) -> tauri_plugin_updater::Result<Option<String>> {
    Ok(app.updater()?.check().await?.map(|u| u.version))
}
