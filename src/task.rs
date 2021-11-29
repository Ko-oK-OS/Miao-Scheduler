use crate::context::Context;

pub enum TaskStatus {
    Ready,
    Running,
    Exited
}

pub trait Task {
    fn run(&self);
    fn get_status(&self) -> TaskStatus;
    fn get_context(&self) -> *mut Context;
}