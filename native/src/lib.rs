#[macro_use]
extern crate neon;
extern crate left_pad_io;


use neon::vm::{Call, JsResult};
use neon::js::{JsInteger, JsString};
use neon::mem::Handle;


fn left_pad(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let str_h: Handle<JsString> = try!(try!(call.arguments.require(scope, 0)).check::<JsString>());
    let len_h: Handle<JsInteger> = try!(try!(call.arguments.require(scope, 1)).check::<JsInteger>());
    let ch_h: Handle<JsString> = try!(try!(call.arguments.require(scope, 2)).check::<JsString>());
    
    let string = (*str_h).value();
    let ch = (*ch_h).value();
    let len : u32 = (*len_h).value() as u32;
   
    if let Ok(result) = left_pad_io::left_pad(&string, &ch, len) {
      Ok(JsString::new(scope, &result).unwrap())
    }
    else {
      Err(neon::vm::Throw)
    }
    
}

register_module!(m, {
    m.export("leftPad", left_pad)
});
