
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubaccessdata/HubAccessData.md")))]
#[::unity2::class(namespace = "App", name = "HubAccessData")]
#[parent(crate::system::object::Object)]
pub struct HubAccessData {
    #[rename(name = "AID")]
    pub aid: ::unity2::Il2CppString,
    #[rename(name = "m_flagName")]
    pub m_flag_name: ::unity2::Il2CppString,
    #[rename(name = "m_messages")]
    pub m_messages: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "ignoreUsedPattern")]
    pub ignore_used_pattern:
        crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
}

#[cfg(feature = "app-hubaccessdata")]
#[::unity2::methods]
impl HubAccessData {
    #[method(name = "get_MID", args = 0)]
    pub fn get_mid(self) -> ::unity2::Il2CppString;

    #[method(name = "get_DisposData", args = 0)]
    pub fn get_dispos_data(self) -> crate::app::hubdisposdata::HubDisposData;

    #[method(name = "set_DisposData", args = 1)]
    pub fn set_dispos_data(self, value: crate::app::hubdisposdata::HubDisposData) -> ();

    #[method(name = "get_IsStory", args = 0)]
    pub fn get_is_story(self) -> bool;

    #[method(name = "set_IsStory", args = 1)]
    pub fn set_is_story(self, value: bool) -> ();

    #[method(name = "get_IsReliance", args = 0)]
    pub fn get_is_reliance(self) -> bool;

    #[method(name = "set_IsReliance", args = 1)]
    pub fn set_is_reliance(self, value: bool) -> ();

    #[method(name = "get_IsGod", args = 0)]
    pub fn get_is_god(self) -> bool;

    #[method(name = "set_IsGod", args = 1)]
    pub fn set_is_god(self, value: bool) -> ();

    #[method(name = "get_IsUnit", args = 0)]
    pub fn get_is_unit(self) -> bool;

    #[method(name = "set_IsUnit", args = 1)]
    pub fn set_is_unit(self, value: bool) -> ();

    #[method(name = "get_IsAnimal", args = 0)]
    pub fn get_is_animal(self) -> bool;

    #[method(name = "set_IsAnimal", args = 1)]
    pub fn set_is_animal(self, value: bool) -> ();

    #[method(name = "get_IsPerson", args = 0)]
    pub fn get_is_person(self) -> bool;

    #[method(name = "set_IsPerson", args = 1)]
    pub fn set_is_person(self, value: bool) -> ();

    #[method(name = "get_IsDone", args = 0)]
    pub fn get_is_done(self) -> bool;

    #[method(name = "get_IsAccessed", args = 0)]
    pub fn get_is_accessed(self) -> bool;

    #[method(name = "get_AccessCount", args = 0)]
    pub fn get_access_count(self) -> i32;

    #[method(name = "get_AccessCountFlagName", args = 0)]
    pub fn get_access_count_flag_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_IsAccessEnabled", args = 0)]
    pub fn get_is_access_enabled(self) -> bool;

    #[method(name = "get_ResultTalkIndex", args = 0)]
    pub fn get_result_talk_index(self) -> i32;

    #[method(name = "set_ResultTalkIndex", args = 1)]
    pub fn set_result_talk_index(self, value: i32) -> ();

    #[method(name = "get_IsHeroBirthday", args = 0)]
    pub fn get_is_hero_birthday(self) -> bool;

    #[method(name = "set_IsHeroBirthday", args = 1)]
    pub fn set_is_hero_birthday(self, value: bool) -> ();

    #[method(name = "get_TalkItem", args = 0)]
    pub fn get_talk_item(self) -> ::unity2::Il2CppString;

    #[method(name = "set_TalkItem", args = 1)]
    pub fn set_talk_item(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ItemCount", args = 0)]
    pub fn get_item_count(self) -> i32;

    #[method(name = "set_ItemCount", args = 1)]
    pub fn set_item_count(self, value: i32) -> ();

    #[method(name = "GetMessageCount", args = 0)]
    pub fn get_message_count(self) -> i32;

    #[method(name = "GetMessage", args = 1)]
    pub fn get_message(self, index: i32) -> ::unity2::Il2CppString;

    #[method(name = "UpdateAccessCount", args = 0)]
    pub fn update_access_count(self) -> ();

    #[method(name = "PreLocate", args = 0)]
    pub fn pre_locate(self) -> ();

    #[method(name = "DoneAccess", args = 0)]
    pub fn done_access(self) -> bool;

    #[method(name = "ConfirmMessage", args = 2)]
    pub fn confirm_message(self, label: ::unity2::Il2CppString, new_add: bool) -> bool;

    #[method(name = "ConfirmContent", args = 1)]
    pub fn confirm_content(self, manager: crate::app::hubaccessmanager::HubAccessManager) -> ();

    #[method(name = "ConfirmContentHub", args = 1)]
    pub fn confirm_content_hub(self, manager: crate::app::hubaccessmanager::HubAccessManager)
        -> ();

    #[method(name = "ConfirmResult", args = 4)]
    pub fn confirm_result(
        self,
        manager: crate::app::hubaccessmanager::HubAccessManager,
        base_name: ::unity2::Il2CppString,
        talk_type: ::unity2::Il2CppString,
        func: crate::system::func_3::Func_3<i32, i32, bool>,
    ) -> bool;

    #[method(name = "ConfirmStoryResult", args = 1)]
    pub fn confirm_story_result(self, story_label: ::unity2::Il2CppString) -> bool;

    #[method(name = "ConfirmContentKizuna", args = 1)]
    pub fn confirm_content_kizuna(
        self,
        manager: crate::app::hubaccessmanager::HubAccessManager,
    ) -> ();

    #[method(name = "TryGetPID", args = 0)]
    pub fn try_get_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "ChangeTalk", args = 1)]
    pub fn change_talk(self, mid: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-hubaccessdata")]
impl HubAccessData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubAccessData),
                ::core::stringify!(new),
            )
        });
        <Self as IHubAccessDataMethods>::ctor(this);
        this
    }
}
