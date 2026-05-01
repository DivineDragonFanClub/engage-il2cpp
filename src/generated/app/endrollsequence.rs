
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/endrollsequence/EndRollSequence.md")))]
#[::unity2::class(namespace = "App", name = "EndRollSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: endrollsequence :: EndRollSequence >)]
pub struct EndRollSequence {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_BgmHeader")]
    pub m_bgm_header: ::unity2::Il2CppString,
    #[rename(name = "m_PrefabHandle")]
    pub m_prefab_handle: crate::app::tresourcehandle_1::TResourceHandle_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_Canvas")]
    pub m_canvas: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Setter")]
    pub m_setter: crate::app::endrollsetter::EndRollSetter,
}

#[cfg(feature = "app-endrollsequence")]
#[::unity2::methods]
impl EndRollSequence {
    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        bgm_header: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "get_IsSkipped", args = 0)]
    pub fn get_is_skipped(self) -> bool;

    #[method(name = "set_IsSkipped", args = 1)]
    pub fn set_is_skipped(self, value: bool) -> ();

    #[method(name = "LoadScene", args = 0)]
    pub fn load_scene(self) -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "Setup1", args = 0)]
    pub fn setup1(self) -> ();

    #[method(name = "Setup2", args = 0)]
    pub fn setup2(self) -> ();

    #[method(name = "Unload", args = 0)]
    pub fn unload(self) -> ();

    #[method(name = "Tick1", args = 0)]
    pub fn tick1(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "Tick2", args = 0)]
    pub fn tick2(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "EnableControllerSupport", args = 0)]
    pub fn enable_controller_support(self) -> ();

    #[method(name = "DisableControllerSupport", args = 0)]
    pub fn disable_controller_support(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-endrollsequence")]
impl EndRollSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EndRollSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IEndRollSequenceMethods>::ctor(this);
        this
    }
}
