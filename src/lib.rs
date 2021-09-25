use neon::prelude::{FunctionContext, JsResult, JsUndefined, JsFunction, Object, Handle, JsValue, Context, ModuleContext, NeonResult};
use std::thread;

fn start(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let callback = cx.argument::<JsFunction>(0)?.root(&mut cx);
    let channel = cx.channel();
    // mark 如何让该线程阻塞
    thread::spawn(move || {
        channel.send(move |mut cx| {
            let callback = callback.into_inner(&mut cx);
            let this = cx.undefined();
            let args = Vec::<Handle<JsValue>>::new();
            callback.call(&mut cx, this, args)?;
            Ok(())
        })
    });
    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("start", start)?;
    Ok(())
}
