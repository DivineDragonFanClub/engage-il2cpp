
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::procscenesequence_1::IProcSceneSequence_1;
use crate::app::procscenesequence_1::ProcSceneSequence_1;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/jobintrosequence/JobIntroSequence.md")))]
#[::unity2::class(namespace = "App", name = "JobIntroSequence")]
# [parent (crate :: app :: procscenesequence_1 :: ProcSceneSequence_1 < crate :: app :: jobintrosequence :: JobIntroSequence >)]
pub struct JobIntroSequence {
    #[rename(name = "m_JobDataList")]
    pub m_job_data_list:
        crate::app::structlist_1::StructList_1<crate::app::jobintrodata::JobIntroData>,
    #[rename(name = "m_CurrentIndex")]
    pub m_current_index: i32,
    #[rename(name = "m_WaitTime")]
    pub m_wait_time: f32,
}

#[cfg(feature = "app-jobintrosequence")]
#[::unity2::methods]
impl JobIntroSequence {
    #[method(name = "get_CurrentGroupIndex", args = 0)]
    pub fn get_current_group_index() -> i32;

    #[method(name = "set_CurrentGroupIndex", args = 1)]
    pub fn set_current_group_index(value: i32) -> ();

    #[method(name = "ToNextIndex", args = 0)]
    pub fn to_next_index() -> ();

    #[method(name = "IsExist", args = 0)]
    pub fn is_exist() -> bool;

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "get_IsFinished", args = 0)]
    pub fn get_is_finished(self) -> bool;

    #[method(name = "get_ReadyTime1", args = 0)]
    pub fn get_ready_time1() -> f32;

    #[method(name = "get_ReadyTime2", args = 0)]
    pub fn get_ready_time2() -> f32;

    #[method(name = "get_CurrentData", args = 0)]
    pub fn get_current_data(self) -> crate::app::jobintrodata::JobIntroData;

    #[method(name = "Initialize", args = 0)]
    pub fn initialize(self) -> ();

    #[method(name = "BeginMap", args = 0)]
    pub fn begin_map(self) -> ();

    #[method(name = "Unload", args = 0)]
    pub fn unload(self) -> ();

    #[method(name = "Begin", args = 0)]
    pub fn begin(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "Finish", args = 0)]
    pub fn finish(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "CheckKeyPush", args = 0)]
    pub fn check_key_push(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "BranchLoop", args = 0)]
    pub fn branch_loop(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-jobintrosequence")]
impl JobIntroSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(JobIntroSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IJobIntroSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/jobintrosequence/JobIntroSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct JobIntroSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for JobIntroSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "JobIntroSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for JobIntroSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl JobIntroSequence_Label {
    pub fn r#loop() -> Self {
        Self { value: 0 }
    }

    pub fn end() -> Self {
        Self { value: 1 }
    }
}
