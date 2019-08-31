#[no_mangle]
pub extern "C" fn first()
{ }

#[no_mangle]
pub extern fn second()
{ }

#[no_mangle]
pub fn third()
{ }

#[no_mangle]
pub extern "stdcall" fn fourth()
{ }