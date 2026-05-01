
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/playables/scriptplayablebinding/ScriptPlayableBinding.md")))]
#[::unity2::class(namespace = "UnityEngine.Playables", name = "ScriptPlayableBinding")]
#[parent(crate::system::object::Object)]
pub struct ScriptPlayableBinding {}

#[cfg(feature = "unity_engine-playables-scriptplayablebinding")]
#[::unity2::methods]
impl ScriptPlayableBinding {
    #[method(name = "Create", args = 3)]
    pub fn create(
        name: ::unity2::Il2CppString,
        key: crate::unity_engine::object_2::Object_2,
        r#type: ::unity2::SystemType,
    ) -> crate::unity_engine::playables::playablebinding::PlayableBinding;

    #[method(name = "CreateScriptOutput", args = 2)]
    pub fn create_script_output(
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::playables::playableoutput::PlayableOutput;
}
