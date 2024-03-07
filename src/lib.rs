#![deny(clippy::all)]

mod naming_conventions;
mod values;

#[macro_use]
extern crate napi_derive;

#[napi]
pub const DEFAULT_CONST: u32 = 12;

#[napi(constructor)]
struct Animal {
  pub name: String,
  pub kind: u32,
}

#[napi]
impl Animal {
  #[napi]
  pub fn change_name(&mut self, new_name: String) {
    self.name = new_name;
  }
}

#[napi]
pub enum Kind {
  Dog,
  Cat,
  Duck,
}