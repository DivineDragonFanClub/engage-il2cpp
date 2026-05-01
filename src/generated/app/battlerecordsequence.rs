
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battlerecordsequence/BattleRecordSequence.md")))]
#[::unity2::class(namespace = "App", name = "BattleRecordSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: battlerecordsequence :: BattleRecordSequence >)]
pub struct BattleRecordSequence {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_BgmHeader")]
    pub m_bgm_header: ::unity2::Il2CppString,
    #[rename(name = "m_PrefabHandle")]
    pub m_prefab_handle: crate::app::tresourcehandle_1::TResourceHandle_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_Setter")]
    pub m_setter: crate::app::battlerecordsetter::BattleRecordSetter,
    #[rename(name = "m_IsSkipped")]
    pub m_is_skipped: bool,
}

#[cfg(feature = "app-battlerecordsequence")]
#[::unity2::methods]
impl BattleRecordSequence {
    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        bgm_header: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "LoadScene", args = 0)]
    pub fn load_scene(self) -> ();

    #[method(name = "WaitLoad", args = 0)]
    pub fn wait_load(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "Setup", args = 0)]
    pub fn setup(self) -> ();

    #[method(name = "Unload", args = 0)]
    pub fn unload(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "FadeOut", args = 0)]
    pub fn fade_out(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-battlerecordsequence")]
impl BattleRecordSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BattleRecordSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IBattleRecordSequenceMethods>::ctor(this);
        this
    }
}
