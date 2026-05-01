
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubaccess/HubAccess.md")))]
#[::unity2::class(namespace = "App", name = "HubAccess")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct HubAccess {
    #[rename(name = "AID")]
    pub aid: ::unity2::Il2CppString,
    #[rename(name = "m_accessData")]
    pub m_access_data: crate::app::hubaccessdata::HubAccessData,
}

#[cfg(feature = "app-hubaccess")]
#[::unity2::methods]
impl HubAccess {
    #[method(name = "get_PlayerTransform", args = 0)]
    pub fn get_player_transform(self) -> crate::unity_engine::transform::Transform;

    #[method(name = "set_PlayerTransform", args = 1)]
    pub fn set_player_transform(self, value: crate::unity_engine::transform::Transform) -> ();

    #[method(name = "get_TargetTransform", args = 0)]
    pub fn get_target_transform(self) -> crate::unity_engine::transform::Transform;

    #[method(name = "set_TargetTransform", args = 1)]
    pub fn set_target_transform(self, value: crate::unity_engine::transform::Transform) -> ();

    #[method(name = "get_HelpOffset", args = 0)]
    pub fn get_help_offset(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_HelpOffset", args = 1)]
    pub fn set_help_offset(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_MID", args = 0)]
    pub fn get_mid(self) -> ::unity2::Il2CppString;

    #[method(name = "get_DisposData", args = 0)]
    pub fn get_dispos_data(self) -> crate::app::hubdisposdata::HubDisposData;

    #[method(name = "get_IsAvailable", args = 0)]
    pub fn get_is_available(self) -> bool;

    #[method(name = "get_IsStory", args = 0)]
    pub fn get_is_story(self) -> bool;

    #[method(name = "get_IsReliance", args = 0)]
    pub fn get_is_reliance(self) -> bool;

    #[method(name = "get_IsGod", args = 0)]
    pub fn get_is_god(self) -> bool;

    #[method(name = "get_IsUnit", args = 0)]
    pub fn get_is_unit(self) -> bool;

    #[method(name = "get_IsPerson", args = 0)]
    pub fn get_is_person(self) -> bool;

    #[method(name = "get_IsAnimal", args = 0)]
    pub fn get_is_animal(self) -> bool;

    #[method(name = "get_IsAccessEnabled", args = 0)]
    pub fn get_is_access_enabled(self) -> bool;

    #[method(name = "get_IsAccessed", args = 0)]
    pub fn get_is_accessed(self) -> bool;

    #[method(name = "get_IsDone", args = 0)]
    pub fn get_is_done(self) -> bool;

    #[method(name = "get_IsWall", args = 0)]
    pub fn get_is_wall(self) -> bool;

    #[method(name = "set_IsWall", args = 1)]
    pub fn set_is_wall(self, value: bool) -> ();

    #[method(name = "get_ResultTalkIndex", args = 0)]
    pub fn get_result_talk_index(self) -> i32;

    #[method(name = "get_IsHeroBirthday", args = 0)]
    pub fn get_is_hero_birthday(self) -> bool;

    #[method(name = "get_AccessCount", args = 0)]
    pub fn get_access_count(self) -> i32;

    #[method(name = "get_TalkItem", args = 0)]
    pub fn get_talk_item(self) -> ::unity2::Il2CppString;

    #[method(name = "get_ItemCount", args = 0)]
    pub fn get_item_count(self) -> i32;

    #[method(name = "get_OrigPosition", args = 0)]
    pub fn get_orig_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_OrigPosition", args = 1)]
    pub fn set_orig_position(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_OrigRotation", args = 0)]
    pub fn get_orig_rotation(self) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "set_OrigRotation", args = 1)]
    pub fn set_orig_rotation(self, value: crate::unity_engine::quaternion::Quaternion) -> ();

    #[method(name = "get_ItemEffect", args = 0)]
    pub fn get_item_effect(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_ItemEffect", args = 1)]
    pub fn set_item_effect(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "get_AccessCursorObject", args = 0)]
    pub fn get_access_cursor_object(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_AccessCursorObject", args = 1)]
    pub fn set_access_cursor_object(self, value: crate::unity_engine::gameobject::GameObject)
        -> ();

    #[method(name = "get_AccessCursor", args = 0)]
    pub fn get_access_cursor(self) -> crate::app::hubaccesscursor::HubAccessCursor;

    #[method(name = "set_AccessCursor", args = 1)]
    pub fn set_access_cursor(self, value: crate::app::hubaccesscursor::HubAccessCursor) -> ();

    #[method(name = "GetMessageCount", args = 0)]
    pub fn get_message_count(self) -> i32;

    #[method(name = "GetMessage", args = 1)]
    pub fn get_message(self, index: i32) -> ::unity2::Il2CppString;

    #[method(name = "UpdateAccessCount", args = 0)]
    pub fn update_access_count(self) -> ();

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "OnDrawGizmos", args = 0)]
    pub fn on_draw_gizmos(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "GetHelpPosition", args = 0)]
    pub fn get_help_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Locate", args = 1)]
    pub fn locate(self, locator: ::unity2::Il2CppString) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "DoneAccess", args = 0)]
    pub fn done_access(self) -> ();

    #[method(name = "Execute", args = 1)]
    pub fn execute(self, hub_sequence: crate::app::hubsequence::HubSequence) -> bool;

    #[method(name = "Refresh", args = 0)]
    pub fn refresh(self) -> ();

    #[method(name = "SetupHelp", args = 1)]
    pub fn setup_help(self, help_object: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "GetHelpTitle", args = 0)]
    pub fn get_help_title(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMainText", args = 0)]
    pub fn get_main_text(self) -> ::unity2::Il2CppString;

    #[method(name = "GetButtonName", args = 0)]
    pub fn get_button_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetAnimalButtonName", args = 0)]
    pub fn get_animal_button_name(self) -> ::unity2::Il2CppString;

    #[method(name = "IsCollideFront", args = 0)]
    pub fn is_collide_front(self) -> bool;

    #[method(name = "IsAccessAngle", args = 1)]
    pub fn is_access_angle(self, pos: crate::unity_engine::vector3::Vector3) -> bool;

    #[method(name = "TryGetPID", args = 0)]
    pub fn try_get_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "CreateItemEffect", args = 0)]
    pub fn create_item_effect(self) -> ();

    #[method(name = "CreateAccessCursor", args = 0)]
    pub fn create_access_cursor(self) -> ();

    #[method(name = "CanAccessDoor", args = 0)]
    pub fn can_access_door(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubaccess")]
impl HubAccess {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubAccess),
                ::core::stringify!(new),
            )
        });
        <Self as IHubAccessMethods>::ctor(this);
        this
    }
}
