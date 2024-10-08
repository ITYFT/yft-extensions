use std::collections::HashMap;

pub trait AppLogger {
    fn write_info(&self, process: String, message: String, ctx: Option<HashMap<String, String>>);
    fn write_warning(&self, process: String, message: String, ctx: Option<HashMap<String, String>>);
    fn write_error(&self, process: String, message: String, ctx: Option<HashMap<String, String>>);
    fn write_fatal_error(
        &self,
        process: String,
        message: String,
        ctx: Option<HashMap<String, String>>,
    );

    fn write_debug_info(
        &self,
        process: String,
        message: String,
        ctx: Option<HashMap<String, String>>,
    );
}
