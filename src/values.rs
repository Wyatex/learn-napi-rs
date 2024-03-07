use napi::bindgen_prelude::{Buffer, Null, Object, Undefined};
use napi::Env;

#[napi]
fn get_undefined() -> Undefined {
    ()
}

#[napi]
fn get_null() -> Null {
    Null
}

#[napi]
fn get_env(env: String) -> Option<String> {
    match std::env::var(env) {
        Ok(val) => Some(val),
        Err(e) => None,
    }
}

#[napi]
fn sum(a: u32, b: i32) -> i64 {
    (b + a as i32).into()
}

#[napi]
fn is_good() -> bool {
    true
}

#[napi]
fn get_buffer_sum(buf: Buffer) -> i64 {
    let buf: Vec<u8> = buf.into();
    // 计算buf值相加
    buf.iter().sum::<u8>() as i64
}

#[napi]
fn read_buffer(file: String) -> Buffer {
    Buffer::from(std::fs::read(file).unwrap())
}

// JavaScript和Rust之间对象转换的成本比其他基本类型要高。
// Object.get(“key”)的每次调用实际上都被分派到node端，
// 包括两个步骤:获取值，将JS转换为rust值，
// Object.set("key", v)也是如此。

#[napi]
fn keys(obj: Object) -> Vec<String> {
    Object::keys(&obj).unwrap()
}

#[napi]
fn log_string_field(obj: Object, field: String) {
    println!("{}: {:?}", &field, obj.get::<String, String>(field.clone()));
}

#[napi]
fn create_obj(env: Env) -> Object {
    let mut obj = env.create_object().unwrap();
    obj.set("test", 1).unwrap();
    obj
}