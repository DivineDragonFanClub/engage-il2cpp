
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/playables/iplayableoutput_interface/IPlayableOutput_Interface.md")))]
#[::unity2::class(namespace = "UnityEngine.Playables", name = "IPlayableOutput")]
pub struct IPlayableOutput_Interface {}

#[cfg(feature = "unity_engine-playables-iplayableoutput_interface")]
#[::unity2::methods]
impl IPlayableOutput_Interface {
    #[method(name = "GetHandle", args = 0)]
    pub fn get_handle(
        self,
    ) -> crate::unity_engine::playables::playableoutputhandle::PlayableOutputHandle;
}
