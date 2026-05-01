
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequencealternateconfirm/MapSequenceAlternateConfirm.md")))]
#[::unity2::class(namespace = "App", name = "MapSequenceAlternateConfirm")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: mapsequencealternateconfirm :: MapSequenceAlternateConfirm >)]
pub struct MapSequenceAlternateConfirm {
    #[rename(name = "m_GameObject")]
    pub m_game_object: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-mapsequencealternateconfirm")]
#[::unity2::methods]
impl MapSequenceAlternateConfirm {
    #[method(name = "get_GlobalAssetPath", args = 0)]
    pub fn get_global_asset_path(self) -> ::unity2::Il2CppString;

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "LoadRes", args = 0)]
    pub fn load_res(self) -> ();

    #[method(name = "IsLoadingRes", args = 0)]
    pub fn is_loading_res(self) -> bool;

    #[method(name = "Open", args = 0)]
    pub fn open(self) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapsequencealternateconfirm")]
impl MapSequenceAlternateConfirm {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSequenceAlternateConfirm),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSequenceAlternateConfirmMethods>::ctor(this);
        this
    }
}
