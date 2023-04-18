use crate::tools::config::SDKConfig as Config;
pub trait SDKProgram {
    fn init(&mut self) {()}
    fn main(&mut self) {()}
}