use vm::Frame;

pub fn eval(val: i64, pc: u16, frame: &mut Frame) -> Option<u16> {
    trace!("lconst_{}: Pushing {}L to stack", val, val);
    frame.stack_push_long(val);

    Some(pc+1)
}