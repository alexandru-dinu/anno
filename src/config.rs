// const ANNO_DIR: &str = concat!(env!("HOME"), "/.anno");
pub const ANNO_DIR: &str = "/tmp/anno";
pub const READ_BUFFER_SIZE: usize = 4096;

pub fn get_editor() -> String {
    std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string())
}
