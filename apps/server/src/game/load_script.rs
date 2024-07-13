use std::sync::{Arc, Mutex};

use rlua::Lua;

pub(crate) fn load_script(sandbox: &Arc<Mutex<Lua>>, script: String) {
    sandbox
        .lock()
        .unwrap()
        .context(|ctx| ctx.load(&script).exec())
        .unwrap();
}
