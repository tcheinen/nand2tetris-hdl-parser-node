use neon::prelude::*;
use nand2tetris_hdl_parser::{Part,Chip,Pin};
use serde::{Serialize, Deserialize};


fn parse_hdl(mut cx: FunctionContext) -> JsResult<JsString> {
    let arg0 = cx.argument::<JsString>(0)?.value();
    Ok(cx.string(&serde_json::to_string( &nand2tetris_hdl_parser::parse_hdl(&arg0).unwrap()).unwrap()))
}

register_module!(mut cx, {
    cx.export_function("parse_hdl", parse_hdl)
});
