#![deny(clippy::all)]

mod snow;
use snow::*;

#[macro_use]
extern crate napi_derive;

use napi::{CallContext, JsNumber, JsObject, JsString, JsUndefined};

#[module_exports]
fn init(mut exports: JsObject) -> napi::Result<()> {
    register_js(&mut exports)?;
    Ok(())
}

#[js_function(2)]
fn snow_helper(ctx: CallContext) -> napi::Result<JsUndefined> {
    let worker_id: u32 = ctx.get::<JsNumber>(0)?.try_into()?;
    let data_center_id: u32 = ctx.get::<JsNumber>(1)?.try_into()?;

    let mut this = ctx.this_unchecked::<JsObject>();
    ctx.env.wrap(&mut this, SnowflakeIdWorker::new(worker_id as u128, data_center_id as u128).unwrap())?;
    ctx.env.get_undefined()
}

#[js_function(2)]
fn snow_next(ctx: CallContext) -> napi::Result<JsString> {
    let this = ctx.this_unchecked::<JsObject>();
    let snow = ctx.env.unwrap::<SnowflakeIdWorker>(&this)?;

    println!("{}", snow.next_id().unwrap());

    ctx.env.create_string(snow.next_id().unwrap().to_string().as_str())
}

fn register_js(exports: &mut JsObject) -> napi::Result<()> {
    exports.create_named_method("snow_helper", snow_helper)?;
    exports.create_named_method("snow_next", snow_next)?;
    Ok(())
}
