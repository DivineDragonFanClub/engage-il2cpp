
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/guitargetattribute/GUITargetAttribute.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "GUITargetAttribute")]
pub struct GUITargetAttribute {
    #[rename(name = "displayMask")]
    pub display_mask: i32,
}

#[cfg(feature = "unity_engine-guitargetattribute")]
#[::unity2::methods]
impl GUITargetAttribute {
    #[method(name = "GetGUITargetAttrValue", args = 2)]
    pub fn get_gui_target_attr_value(
        klass: ::unity2::SystemType,
        method_name: ::unity2::Il2CppString,
    ) -> i32;
}
