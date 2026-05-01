
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/jobintrotitlesetter/JobIntroTitleSetter.md")))]
#[::unity2::class(namespace = "App", name = "JobIntroTitleSetter")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct JobIntroTitleSetter {
    #[rename(name = "JobName")]
    pub job_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "ReadyTime1")]
    pub ready_time1: f32,
    #[rename(name = "ReadyTime2")]
    pub ready_time2: f32,
}

#[cfg(feature = "app-jobintrotitlesetter")]
#[::unity2::methods]
impl JobIntroTitleSetter {
    #[method(name = "SetData", args = 1)]
    pub fn set_data(self, job: crate::app::jobdata::JobData) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-jobintrotitlesetter")]
impl JobIntroTitleSetter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(JobIntroTitleSetter),
                ::core::stringify!(new),
            )
        });
        <Self as IJobIntroTitleSetterMethods>::ctor(this);
        this
    }
}
