
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/itextpreprocessor/ITextPreprocessor.md")))]
#[::unity2::class(namespace = "TMPro", name = "ITextPreprocessor")]
pub struct ITextPreprocessor {}

#[cfg(feature = "tm_pro-itextpreprocessor")]
#[::unity2::methods]
impl ITextPreprocessor {
    #[method(name = "PreprocessText", args = 1)]
    pub fn preprocess_text(self, text: ::unity2::Il2CppString) -> ::unity2::Il2CppString;
}
