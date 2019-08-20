//! JavaScript bindings for `mice`!
//! Not the entire API is provided yet,
//! but what is *should* satisfy existing
//! guarantees.
use crate::{
    builder::{BuildError, RollBuilder},
    parse::ParseError,
    ExpressionResult, RollError,
};
use js_sys::Math::random;
use rand::rngs::StdRng;
use rand::SeedableRng;
use wasm_bindgen::prelude::*;

impl From<RollError> for JsValue {
    fn from(e: RollError) -> JsValue {
        JsValue::from_str(&format!("{}", e))
    }
}
impl From<ParseError> for JsValue {
    fn from(e: ParseError) -> JsValue {
        JsValue::from_str(&format!("{}", e))
    }
}
impl From<BuildError> for JsValue {
    fn from(e: BuildError) -> JsValue {
        JsValue::from_str(&format!("{}", e))
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl ExpressionResult {
    pub fn display(&self) -> String {
        format!("{}", self)
    }
}

/// JavaScript binding for `mice::roll`.
// thread_rng isn't supported on WASM
#[wasm_bindgen]
pub fn roll(input: &str) -> Result<ExpressionResult, JsValue> {
    Ok(RollBuilder::new()
        .parse(input)?
        .with_rng(Box::new(StdRng::seed_from_u64(random().to_bits())))
        .into_roll()?
        .roll()?)
}
