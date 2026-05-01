
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitstatus/UnitStatus.md")))]
#[::unity2::class(namespace = "App", name = "UnitStatus")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: unitstatus :: UnitStatus >)]
pub struct UnitStatus {
    #[static_field]
    #[rename(name = "ResNameC")]
    pub res_name_c: ::unity2::Il2CppString,
    #[rename(name = "m_PrefabHandle")]
    pub m_prefab_handle: crate::app::tresourcehandle_1::TResourceHandle_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[static_field]
    #[rename(name = "m_GameObject")]
    pub m_game_object: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-unitstatus")]
#[::unity2::methods]
impl UnitStatus {
    #[method(name = "CreateAsync", args = 1)]
    pub fn create_async(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateImpl", args = 2)]
    pub fn create_impl(
        super_: crate::app::procinst::ProcInst,
        p: crate::app::unitstatus::UnitStatus,
    ) -> ();

    #[method(name = "CreateDescs", args = 1)]
    pub fn create_descs(
        p: crate::app::unitstatus::UnitStatus,
    ) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "CreateAsync", args = 0)]
    pub fn create_async_2(self) -> ();

    #[method(name = "IsCreating", args = 0)]
    pub fn is_creating(self) -> bool;

    #[method(name = "Prepare", args = 0)]
    pub fn prepare(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "StartJobIntro", args = 1)]
    pub fn start_job_intro(unit: crate::app::unit::Unit) -> ();

    #[method(name = "StartUnitSelect", args = 2)]
    pub fn start_unit_select(unit: crate::app::unit::Unit, is_sortie: bool) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close() -> ();

    #[method(name = "SetUnit", args = 1)]
    pub fn set_unit(unit: crate::app::unit::Unit) -> ();

    #[method(name = "SetFront", args = 0)]
    pub fn set_front() -> ();

    #[method(name = "GetGameObject", args = 0)]
    pub fn get_game_object() -> crate::unity_engine::gameobject::GameObject;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitstatus")]
impl UnitStatus {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitStatus),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitStatusMethods>::ctor(this);
        this
    }
}
