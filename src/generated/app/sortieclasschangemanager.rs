
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieclasschangemanager/SortieClassChangeManager.md")))]
#[::unity2::class(namespace = "App", name = "SortieClassChangeManager")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: sortieclasschangemanager :: SortieClassChangeManager >)]
pub struct SortieClassChangeManager {
    #[rename(name = "m_jobData")]
    pub m_job_data: crate::app::classchange::ClassChange_ChangeJobData,
}

#[cfg(feature = "app-sortieclasschangemanager")]
#[::unity2::methods]
impl SortieClassChangeManager {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_JobData", args = 0)]
    pub fn get_job_data(self) -> crate::app::classchange::ClassChange_ChangeJobData;

    #[method(name = "set_JobData", args = 1)]
    pub fn set_job_data(self, value: crate::app::classchange::ClassChange_ChangeJobData) -> ();
}

#[cfg(feature = "app-sortieclasschangemanager")]
impl SortieClassChangeManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieClassChangeManager),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieClassChangeManagerMethods>::ctor(this);
        this
    }
}
