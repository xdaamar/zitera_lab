// Prevents additional console/CMD popup window on Windows in both debug and release
#![windows_subsystem = "windows"]

fn main() {
  app_lib::run();
}
