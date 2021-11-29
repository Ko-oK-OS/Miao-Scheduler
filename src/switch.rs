use crate::context::Context;

global_asm!(include_str!("switch.S"));

extern "C" {
    pub fn __switch(
        new_context: *mut Context,
        old_context: *mut Context
    );
}

