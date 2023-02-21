// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//include!(concat!(env!("OUT_DIR"), "/tauri-build-context.rs"));

fn main() {
  tauri::Builder::default()
    .run(tauri::tauri_build_context!())
    //.run(__tauri_build_context::context())
    .expect("error while running tauri application");
}
