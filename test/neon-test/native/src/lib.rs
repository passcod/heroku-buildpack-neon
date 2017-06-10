#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult};
use neon::js::JsString;

fn versions(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    Ok(JsString::new(scope, &format!("Node: {}\nRust: {:?}",
        env!("npm_config_user_agent"),
        env!("npm_package_engines_rust"),
    )).unwrap())
}

register_module!(m, {
    m.export("versions", versions)
});
