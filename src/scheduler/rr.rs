use alloc::collections::VecDeque;
use crate::task::{ Task, TaskStatus };
use crate::switch::__switch;

pub struct RRScheduler<T: Task> {
    task_queue: VecDeque<T>,
    current_task_id: usize
}

impl<T: Task> RRScheduler<T>{
    pub fn find_next_task(&self) -> Option<usize> {
        for (index, task) in self.task_queue.iter().enumerate() {
            match task.get_status() {
                TaskStatus::Ready => {
                    return Some(index)
                },
                _ => {}
            }
        }
        None
    }

    pub fn schedule(&self) {
        let next_task_id = self.find_next_task();
        if let Some(next_task_id) = next_task_id {
            let next_task = &self.task_queue[next_task_id];
            let current_task = &self.task_queue[self.current_task_id];
            let next_context = next_task.get_context();
            let current_context = current_task.get_context();
            unsafe{
                __switch(
                    current_context,
                    next_context
                );
            }
        }
    }
} 