
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/moviedefinedata/MovieDefineData.md")))]
#[::unity2::class(namespace = "App", name = "MovieDefineData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: moviedefinedata :: MovieDefineData >)]
pub struct MovieDefineData {}

#[cfg(feature = "app-moviedefinedata")]
#[::unity2::methods]
impl MovieDefineData {
    #[method(name = "get_MovieFileName", args = 0)]
    pub fn get_movie_file_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_MovieFileName", args = 1)]
    pub fn set_movie_file_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Help", args = 0)]
    pub fn get_help(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Help", args = 1)]
    pub fn set_help(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Condition", args = 0)]
    pub fn get_condition(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Condition", args = 1)]
    pub fn set_condition(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_No", args = 0)]
    pub fn get_no(self) -> i32;

    #[method(name = "set_No", args = 1)]
    pub fn set_no(self, value: i32) -> ();

    #[method(name = "get_BeforeSoundEventName1", args = 0)]
    pub fn get_before_sound_event_name1(self) -> ::unity2::Il2CppString;

    #[method(name = "set_BeforeSoundEventName1", args = 1)]
    pub fn set_before_sound_event_name1(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_BeforeSoundEventName2", args = 0)]
    pub fn get_before_sound_event_name2(self) -> ::unity2::Il2CppString;

    #[method(name = "set_BeforeSoundEventName2", args = 1)]
    pub fn set_before_sound_event_name2(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_BeforeSoundEventName3", args = 0)]
    pub fn get_before_sound_event_name3(self) -> ::unity2::Il2CppString;

    #[method(name = "set_BeforeSoundEventName3", args = 1)]
    pub fn set_before_sound_event_name3(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AfterSoundEventName1", args = 0)]
    pub fn get_after_sound_event_name1(self) -> ::unity2::Il2CppString;

    #[method(name = "set_AfterSoundEventName1", args = 1)]
    pub fn set_after_sound_event_name1(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AfterSoundEventName2", args = 0)]
    pub fn get_after_sound_event_name2(self) -> ::unity2::Il2CppString;

    #[method(name = "set_AfterSoundEventName2", args = 1)]
    pub fn set_after_sound_event_name2(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AfterSoundEventName3", args = 0)]
    pub fn get_after_sound_event_name3(self) -> ::unity2::Il2CppString;

    #[method(name = "set_AfterSoundEventName3", args = 1)]
    pub fn set_after_sound_event_name3(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_MessFileName", args = 0)]
    pub fn get_mess_file_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_MessFileName", args = 1)]
    pub fn set_mess_file_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_DLCDirectoryName", args = 0)]
    pub fn get_dlc_directory_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_DLCDirectoryName", args = 1)]
    pub fn set_dlc_directory_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();
}

#[cfg(feature = "app-moviedefinedata")]
impl MovieDefineData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MovieDefineData),
                ::core::stringify!(new),
            )
        });
        <Self as IMovieDefineDataMethods>::ctor(this);
        this
    }
}
