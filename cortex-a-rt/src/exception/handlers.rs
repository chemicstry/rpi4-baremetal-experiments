use super::exception::ExceptionContext;

pub type ExceptionHandler = dyn Fn(&mut ExceptionContext);

#[derive(Default)]
pub struct ExceptionHandlers {
    pub current_el0_synchronous: Option<&'static ExceptionHandler>,
}

pub static mut EXCEPTION_HANDLERS: ExceptionHandlers = Default::default();
