use std::thread::JoinHandle;

pub enum OpCodeResultType {
    Bool(bool),
    Thread(Option<JoinHandle<()>>),
    Empty
}
