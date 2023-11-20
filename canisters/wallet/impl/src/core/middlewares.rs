use super::CallContext;

pub fn call_context() -> CallContext {
    CallContext::get()
}

pub fn authorize(middleware: (&'static str, &Vec<&'static str>), context: CallContext) {
    middleware.1.iter().for_each(|permission| {
        context.check_access(permission);
    });
}
