// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use tauri::image::Image;
use tauri::menu::{CheckMenuItemBuilder, MenuBuilder, SubmenuBuilder};
use tauri::{Emitter, Manager};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_window_count(app: tauri::AppHandle) -> usize {
    app.webview_windows().len()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let light_theme = CheckMenuItemBuilder::new("Light")
                    .id("light")
                    .checked(false)
                    .build(app)?;
            let dark_theme = CheckMenuItemBuilder::new("Dark")
                    .id("dark")
                    .checked(true)
                    .build(app)?;
            let whitespace_theme = CheckMenuItemBuilder::new("Whitespace")
                    .id("whitespace")
                    .checked(false)
                    .build(app)?;
            let void_theme = CheckMenuItemBuilder::new("Void")
                    .id("void")
                    .checked(false)
                    .build(app)?;
            let star_theme = CheckMenuItemBuilder::new("STAR")
                    .id("star")
                    .checked(false)
                    .build(app)?;
            let custom_theme = CheckMenuItemBuilder::new("Custom Colors")
                    .checked(false)
                    .build(app)?;

            let app_icon = Image::from_bytes(include_bytes!("../icons/icon.png")).unwrap();
                
            let icon_menu = SubmenuBuilder::new(app, "WIDEPad")
                .text("new_window", "New Window")
                .text("close_window", "Close Window")
                .separator()
                .text("restart", "Restart")
                .text("quit", "Quit")
                .build()?;
            let file_menu = SubmenuBuilder::new(app, "File")
                .text("new_file", "New File")
                .text("open_file", "Open File")
                .text("nearby_files", "Nearby Files...")
                .separator()
                .text("save", "Save")
                .text("save_as", "Save As")
                .build()?;
            let edit_menu = SubmenuBuilder::new(app, "Edit")
                .undo()
                .redo()
                .separator()
                .select_all()
                .cut()
                .copy()
                .paste()
                .separator()
                .text("find_and_replace", "Find and Replace")
                .build()?;
            let view_menu = SubmenuBuilder::new(app, "View")
                .item(&(CheckMenuItemBuilder::new("Side Panel")
                    .id("side-panel")
                    .checked(true)
                    .build(app)?))
                .separator()
                .item(&(CheckMenuItemBuilder::new("Line Numbers")
                    .checked(false)
                    .enabled(false) /* !TO BE IMPLEMENTED */
                    .build(app)?))
                .item(&(CheckMenuItemBuilder::new("File Details")
                    .id("file-details")
                    .checked(false)
                    .build(app)?))
                .item(&(CheckMenuItemBuilder::new("Font Selection")
                    .id("font-selection")
                    .checked(false)
                    .build(app)?))
                .item(&(CheckMenuItemBuilder::new("Font Color")
                    .id("font-color")
                    .checked(false)
                    .build(app)?))
                .item(&(CheckMenuItemBuilder::new("Zoom")
                    .id("zoom")
                    .checked(false)
                    .build(app)?))
                .build()?;
            let tools_menu = SubmenuBuilder::new(app, "Tools")
                .separator()
                .text("directory_explorer", "Directory Explorer")
                .text("sticky_notes", "Sticky Notes")
                .text("terminal", "Terminal")
                .separator()
                .text("word_counter","Word Counter")
                .text("char_counter", "Character Counter")
                .text("unicode_keyboard", "UNICODE Keyboard")
                .text("emoji_keyboard", "Emoji Keyboard")
                .text("ascii_art_keyboard", "ASCII Art Keyboard")
                .text("cryptographer", "Cryptographer")
                .separator()
                .text("calculator", "Calculator")
                .text("unit_converter", "Unit Converter")
                .text("rng", "RNG")
                .separator()
                .text("calendar", "Calendar")
                .text("stopwatch", "Stopwatch")
                .text("timezone_converter", "TimeZone Converter")
                .separator()
                .text("picture_display", "Picture Display")
                .text("audio_player", "Audio Player")
                .text("color_picker", "Color Picker")
                .text("pdf_viewer", "PDF Viewer")
                .build()?;
            let themes_menu = SubmenuBuilder::new(app, "Themes")
                .id("dark")
                .item(&light_theme)
                .item(&dark_theme)
                // .item(&(CheckMenuItemBuilder::new("Transparent").id("transparent").checked(false).build(app)?))
                .item(&whitespace_theme)
                .item(&void_theme)
                .item(&star_theme)
                .item(&custom_theme)
                .item(&(CheckMenuItemBuilder::new("Custom Background").id("custom_bg").build(app)?))
                .build()?;
            let help_menu = SubmenuBuilder::new(app, "Help")
                .text("github_feedback", "Github Feedback")
                .text("documentation", "Docs")
                .build()?;
            let main_menu = MenuBuilder::new(app)
                .item(&icon_menu)
                .item(&file_menu)
                .item(&edit_menu)
                .item(&view_menu)
                .item(&tools_menu)
                .item(&themes_menu)
                .item(&help_menu)
                .build()?;

            app.set_menu(main_menu)?;

            icon_menu.set_icon(Some(app_icon))?;

            app.on_menu_event(move |app_handle, event| {
                match event.id().0.as_str() {
                    "light" | "dark" | "whitespace" | "void" | "star" => {
                        app_handle.emit("themes", event.id().0.as_str().to_string()).expect("Theme failed to load");
                        for item in themes_menu.items().unwrap().iter() {
                            let mut checked: bool = false;
                            if item.id() == event.id() || item.id().0.as_str() == "transparent" || item.id().0.as_str() == "custom_bg" { checked = true; }
                            if item.id().0.as_str() != "custom_bg" { 
                                item.as_check_menuitem().unwrap().set_checked(checked).expect("Cannot enable theme");
                            }
                        };
                    }
                    "side-panel" | "file-details" | "font-selection" | "font-color" | "zoom" => app_handle.emit("view", event.id().0.as_str().to_string()).expect("Viewmode failed"),
                    "github_feedback" => app_handle.emit("help", event.id().0.as_str().to_string()).expect("Viewmode failed"),
                    "new_window" | "close_window" | "restart" => app_handle.emit("icon", event.id().0.as_str().to_string()).expect("Failed to close window"),
                    "quit" => app_handle.exit(0),
                    // "transparent" => app_handle.emit("themes:transparent", {}).expect("Theme failed to load"),
                    _ => {}
                }
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![greet, get_window_count])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
