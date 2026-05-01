
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/jobintrostatus/JobIntroStatus.md")))]
#[::unity2::class(namespace = "App", name = "JobIntroStatus")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: jobintrostatus :: JobIntroStatus >)]
pub struct JobIntroStatus {
    #[static_field]
    #[rename(name = "TitlePrefabPath")]
    pub title_prefab_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "InfoPrefabPath")]
    pub info_prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_TitlePrefabHandle")]
    pub m_title_prefab_handle: crate::app::tresourcehandle_1::TResourceHandle_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_TitleGameObject")]
    pub m_title_game_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_TitleSetter")]
    pub m_title_setter: crate::app::jobintrotitlesetter::JobIntroTitleSetter,
    #[rename(name = "m_InfoPrefabHandle")]
    pub m_info_prefab_handle: crate::app::tresourcehandle_1::TResourceHandle_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_InfoGameObject")]
    pub m_info_game_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_InfoSetter")]
    pub m_info_setter: crate::app::jobintroinfosetter::JobIntroInfoSetter,
}

#[cfg(feature = "app-jobintrostatus")]
#[::unity2::methods]
impl JobIntroStatus {
    #[method(name = "get_ReadyTime1", args = 0)]
    pub fn get_ready_time1() -> f32;

    #[method(name = "set_ReadyTime1", args = 1)]
    pub fn set_ready_time1(value: f32) -> ();

    #[method(name = "get_ReadyTime2", args = 0)]
    pub fn get_ready_time2() -> f32;

    #[method(name = "set_ReadyTime2", args = 1)]
    pub fn set_ready_time2(value: f32) -> ();

    #[method(name = "CreateAsync", args = 1)]
    pub fn create_async(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateImpl", args = 2)]
    pub fn create_impl(
        super_: crate::app::procinst::ProcInst,
        p: crate::app::jobintrostatus::JobIntroStatus,
    ) -> ();

    #[method(name = "CreateDescs", args = 1)]
    pub fn create_descs(
        p: crate::app::jobintrostatus::JobIntroStatus,
    ) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "CreateAsync", args = 0)]
    pub fn create_async_2(self) -> ();

    #[method(name = "IsCreating", args = 0)]
    pub fn is_creating(self) -> bool;

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "StartJobIntro", args = 1)]
    pub fn start_job_intro(unit: crate::app::unit::Unit) -> ();

    #[method(name = "StartJobIntroImpl", args = 1)]
    pub fn start_job_intro_impl(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Destroy", args = 0)]
    pub fn destroy() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-jobintrostatus")]
impl JobIntroStatus {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(JobIntroStatus),
                ::core::stringify!(new),
            )
        });
        <Self as IJobIntroStatusMethods>::ctor(this);
        this
    }
}
