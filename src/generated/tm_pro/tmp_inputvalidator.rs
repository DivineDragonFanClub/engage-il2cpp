
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_inputvalidator/TMP_InputValidator.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMP_InputValidator")]
#[parent(crate::unity_engine::scriptableobject::ScriptableObject)]
pub struct TMP_InputValidator {}

#[cfg(feature = "tm_pro-tmp_inputvalidator")]
#[::unity2::methods]
impl TMP_InputValidator {
    #[method(name = "Validate", args = 3)]
    pub fn validate(self, text: ::unity2::Il2CppString, pos: i32, ch: u16) -> u16;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "tm_pro-tmp_inputvalidator")]
impl TMP_InputValidator {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_InputValidator),
                ::core::stringify!(new),
            )
        });
        <Self as ITMP_InputValidatorMethods>::ctor(this);
        this
    }
}
