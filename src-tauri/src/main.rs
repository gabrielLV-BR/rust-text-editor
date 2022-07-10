#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{Menu, CustomMenuItem, Event, MenuEntry, Submenu, MenuItem};

fn main() {

  let mut menu = Menu::new();

  menu = menu.add_submenu(Submenu::new("File", Menu::with_items([
    CustomMenuItem::new("new_file", "Novo Arquivo").into(),
    CustomMenuItem::new("new_folder", "Nova Pasta").into(),
    CustomMenuItem::new("open_folder", "Abrir Pasta").into(),
    CustomMenuItem::new("save", "Salvar").into(),
  ])));

  tauri::Builder::default()
    .menu(menu)
    .on_menu_event(|event| {

      match event.menu_item_id() {
        "new_file" => {
          event.window().emit("new_file", "").expect("Error when emitting event");
          println!("Novo arquivo");
        },
        "new_folder" => {
          println!("Nova pasta");
        },
        "open_folder" => {
          println!("Abrir pasta");
        },
        "save" => {
          println!("Salvar");
        },
        _ => {
          println!("Opção inválida");
        }
      }

    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
