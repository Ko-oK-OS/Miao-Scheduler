use crate::context::Context;

global_asm!(include_str!("switch.S"));

extern "C" {
    pub fn __switch(
        old_context: *mut Context,
        new_context: *mut Context
    );
}

