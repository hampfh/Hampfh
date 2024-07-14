use std::sync::{Arc, Mutex};

use mlua::Lua;

pub(crate) fn load_script(sandbox: &Arc<Mutex<Lua>>, script: String) {
    sandbox.lock().unwrap().load(&script).exec().unwrap();
}
