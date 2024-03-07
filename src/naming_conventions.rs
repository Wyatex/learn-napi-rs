#[napi]
fn a_function(a_arg: u32) -> u32 {
    a_arg + 1
}

#[napi(js_name = "coolFunction")]
fn c_function(a_arg: u32) -> u32 {
    a_arg + 1
}