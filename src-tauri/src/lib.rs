use tauri::Manager;

#[tauri::command]
fn show_content_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("content") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn hide_content_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("content") {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn navigate_content_window(app: tauri::AppHandle, url: String) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("content") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        window.eval(&format!("window.location.href = '{}'", url)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn move_floating_ball(app: tauri::AppHandle, x: f64, y: f64) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("floating-ball") {
        window.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x: x as i32, y: y as i32 })).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            show_content_window,
            hide_content_window,
            navigate_content_window,
            move_floating_ball
        ])
        .setup(|app| {
            if let Some(floating_ball) = app.get_webview_window("floating-ball") {
                let _ = floating_ball.set_always_on_top(true);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
