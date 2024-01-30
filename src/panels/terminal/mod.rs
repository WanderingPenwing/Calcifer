#[allow(unused_imports)]
#[cfg(target_os = "linux")]
mod linux_terminal;
#[cfg(target_os = "linux")]
pub use linux_terminal::*;

#[cfg(target_os = "windows")]
mod windows_terminal;
#[cfg(target_os = "windows")]
pub use windows_terminal::*;
