
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mess/Mess_TagID_Localize.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Mess_TagID_Localize {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Mess_TagID_Localize {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Mess.TagID_Localize";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Mess_TagID_Localize {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Mess_TagID_Localize {
    pub fn mf_tag() -> Self {
        Self { value: 0 }
    }

    pub fn uncap() -> Self {
        Self { value: 1 }
    }

    pub fn c_nthird() -> Self {
        Self { value: 2 }
    }

    pub fn t_wthird() -> Self {
        Self { value: 3 }
    }

    pub fn k_rppn01() -> Self {
        Self { value: 4 }
    }

    pub fn k_rppn02() -> Self {
        Self { value: 5 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mess/Mess_TagGroup.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Mess_TagGroup {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Mess_TagGroup {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Mess.TagGroup";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Mess_TagGroup {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Mess_TagGroup {
    pub fn system() -> Self {
        Self { value: 0 }
    }

    pub fn arg() -> Self {
        Self { value: 1 }
    }

    pub fn talk_type() -> Self {
        Self { value: 2 }
    }

    pub fn window() -> Self {
        Self { value: 3 }
    }

    pub fn wait() -> Self {
        Self { value: 4 }
    }

    pub fn expression() -> Self {
        Self { value: 5 }
    }

    pub fn name() -> Self {
        Self { value: 6 }
    }

    pub fn fade() -> Self {
        Self { value: 7 }
    }

    pub fn icon() -> Self {
        Self { value: 8 }
    }

    pub fn text() -> Self {
        Self { value: 9 }
    }

    pub fn localize() -> Self {
        Self { value: 10 }
    }

    pub fn picture() -> Self {
        Self { value: 11 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mess/Mess_TagID_Arg.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Mess_TagID_Arg {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Mess_TagID_Arg {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Mess.TagID_Arg";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Mess_TagID_Arg {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Mess_TagID_Arg {
    pub fn arg0() -> Self {
        Self { value: 0 }
    }

    pub fn arg1() -> Self {
        Self { value: 1 }
    }

    pub fn arg2() -> Self {
        Self { value: 2 }
    }

    pub fn arg3() -> Self {
        Self { value: 3 }
    }

    pub fn arg4() -> Self {
        Self { value: 4 }
    }

    pub fn arg5() -> Self {
        Self { value: 5 }
    }

    pub fn arg6() -> Self {
        Self { value: 6 }
    }

    pub fn arg7() -> Self {
        Self { value: 7 }
    }

    pub fn arg_b() -> Self {
        Self { value: 8 }
    }

    pub fn arg_bs() -> Self {
        Self { value: 9 }
    }

    pub fn arg_b_uncap() -> Self {
        Self { value: 10 }
    }

    pub fn arg_bs_uncap() -> Self {
        Self { value: 11 }
    }

    pub fn arg_max() -> Self {
        Self { value: 8 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mess/Mess_IconCategory.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Mess_IconCategory {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Mess_IconCategory {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Mess.IconCategory";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Mess_IconCategory {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Mess_IconCategory {
    pub fn item() -> Self {
        Self { value: 0 }
    }

    pub fn skill() -> Self {
        Self { value: 1 }
    }

    pub fn system() -> Self {
        Self { value: 2 }
    }

    pub fn god_symbol_engrave() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mess/Mess_TagID_Text.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Mess_TagID_Text {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Mess_TagID_Text {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Mess.TagID_Text";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Mess_TagID_Text {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Mess_TagID_Text {
    pub fn space() -> Self {
        Self { value: 0 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mess/Mess.md")))]
#[::unity2::class(namespace = "App", name = "Mess")]
#[parent(crate::system::object::Object)]
pub struct Mess {
    #[static_field]
    #[rename(name = "CharSize")]
    pub char_size: i32,
    #[static_field]
    #[rename(name = "ShiftIn")]
    pub shift_in: u16,
    #[static_field]
    #[rename(name = "ShiftOut")]
    pub shift_out: u16,
    #[static_field]
    #[rename(name = "NotFoundText")]
    pub not_found_text: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "FileName_HubCommon")]
    pub file_name_hub_common: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "FileNameHeader_Reliance")]
    pub file_name_header_reliance: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "FileNameHeader_GodReliance")]
    pub file_name_header_god_reliance: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "FileNameHeader_MainScenario")]
    pub file_name_header_main_scenario: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "FileNameHeader_SideScenario")]
    pub file_name_header_side_scenario: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "FileNameHeader_GodScenario")]
    pub file_name_header_god_scenario: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "FileNameHeader_EvilScenario")]
    pub file_name_header_evil_scenario: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "LabelHeader_Reliance")]
    pub label_header_reliance: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "LabelHeader_GodReliance")]
    pub label_header_god_reliance: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "LabelHeader_Die")]
    pub label_header_die: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "LabelHeader_Hub")]
    pub label_header_hub: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "ReplaceStr_DefaultHeroName")]
    pub replace_str_default_hero_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "ReplaceStr_DefaultMorphName")]
    pub replace_str_default_morph_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "ReplaceStr_DefaultMascotName")]
    pub replace_str_default_mascot_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "ReplaceStr_CNThirdTagMale")]
    pub replace_str_cn_third_tag_male: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "ReplaceStr_CNThirdTagFemale")]
    pub replace_str_cn_third_tag_female: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "ReplaceStr_TWThirdTagMale")]
    pub replace_str_tw_third_tag_male: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "ReplaceStr_TWThirdTagFemale")]
    pub replace_str_tw_third_tag_female: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "ReplaceStr_Bracelet")]
    pub replace_str_bracelet: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "ReplaceStr_Bracelets")]
    pub replace_str_bracelets: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "ReplaceStr_Bracelet_UNCAP")]
    pub replace_str_bracelet_uncap: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "ReplaceStr_Bracelets_UNCAP")]
    pub replace_str_bracelets_uncap: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "s_messFileDictionary")]
    pub s_mess_file_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::app::msgfile::MsgFile,
    >,
    #[static_field]
    #[rename(name = "s_soundCmdFileDictionary")]
    pub s_sound_cmd_file_dictionary:
        crate::system::collections::generic::dictionary_2::Dictionary_2<
            ::unity2::Il2CppString,
            crate::app::msgfile::MsgFile,
        >,
    #[static_field]
    #[rename(name = "s_eventCmdFileDictionary")]
    pub s_event_cmd_file_dictionary:
        crate::system::collections::generic::dictionary_2::Dictionary_2<
            ::unity2::Il2CppString,
            crate::app::msgfile::MsgFile,
        >,
    #[static_field]
    #[rename(name = "s_messDataDictionary")]
    pub s_mess_data_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        ::unity2::IntPtr,
    >,
    #[static_field]
    #[rename(name = "s_soundCmdDataDictionary")]
    pub s_sound_cmd_data_dictionary:
        crate::system::collections::generic::dictionary_2::Dictionary_2<
            ::unity2::Il2CppString,
            ::unity2::IntPtr,
        >,
    #[static_field]
    #[rename(name = "s_eventCmdDataDictionary")]
    pub s_event_cmd_data_dictionary:
        crate::system::collections::generic::dictionary_2::Dictionary_2<
            ::unity2::Il2CppString,
            ::unity2::IntPtr,
        >,
    #[static_field]
    #[rename(name = "s_pathDictionary")]
    pub s_path_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        ::unity2::Il2CppString,
    >,
    #[static_field]
    #[rename(name = "s_checkStack")]
    pub s_check_stack: crate::system::collections::generic::stack_1::Stack_1<
        crate::system::collections::generic::dictionary_2::Dictionary_2<
            ::unity2::Il2CppString,
            i32,
        >,
    >,
    #[static_field]
    #[rename(name = "s_mess")]
    pub s_mess: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "s_isBody")]
    pub s_is_body: bool,
    #[static_field]
    #[rename(name = "s_isStrToLower")]
    pub s_is_str_to_lower: bool,
    #[static_field]
    #[rename(name = "s_isNesting")]
    pub s_is_nesting: bool,
    #[static_field]
    #[rename(name = "ArgMax")]
    pub arg_max: i32,
    #[static_field]
    #[rename(name = "ArgStack")]
    pub arg_stack: i32,
    #[static_field]
    #[rename(name = "s_argArray")]
    pub s_arg_array: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "s_argStack")]
    pub s_arg_stack: crate::system::collections::generic::stack_1::Stack_1<
        ::unity2::Array<::unity2::Il2CppString>,
    >,
    #[static_field]
    #[rename(name = "s_partnerPid")]
    pub s_partner_pid: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "s_spriteAssetHandleDictionary")]
    pub s_sprite_asset_handle_dictionary:
        crate::system::collections::generic::dictionary_2::Dictionary_2<
            ::unity2::Il2CppString,
            crate::app::resourcehandle_2::ResourceHandle_2,
        >,
}

#[cfg(feature = "app-mess")]
#[::unity2::methods]
impl Mess {
    #[method(name = "Initialize", args = 0)]
    pub fn initialize() -> ();

    #[method(name = "UpdateReplaceStr", args = 0)]
    pub fn update_replace_str() -> ();

    #[method(name = "LoadSpriteAsset", args = 0)]
    pub fn load_sprite_asset() -> ();

    #[method(name = "LoadSpriteAsset", args = 1)]
    pub fn load_sprite_asset_2(sprite_asset_name: ::unity2::Il2CppString) -> ();

    #[method(name = "IsSpriteExistInSpriteAsset", args = 2)]
    pub fn is_sprite_exist_in_sprite_asset(
        sprite_asset_name: ::unity2::Il2CppString,
        sprite_name: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "GetItemSpriteAssetsName", args = 1)]
    pub fn get_item_sprite_assets_name(
        item: crate::app::itemdata::ItemData,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetSkillSpriteAssetsName", args = 1)]
    pub fn get_skill_sprite_assets_name(
        skill: crate::app::skilldata::SkillData,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetSystemSpriteAssetsName", args = 1)]
    pub fn get_system_sprite_assets_name(
        icon_name: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetGodSymbolEngraveSpriteAssetsName", args = 1)]
    pub fn get_god_symbol_engrave_sprite_assets_name(
        icon_name: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "OnSpriteAssetRequestCallback", args = 2)]
    pub fn on_sprite_asset_request_callback(
        arg: i32,
        sprite_asset_name: ::unity2::Il2CppString,
    ) -> crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset;

    #[method(name = "PushCheck", args = 0)]
    pub fn push_check() -> ();

    #[method(name = "PopCheck", args = 0)]
    pub fn pop_check() -> ();

    #[method(name = "Load", args = 1)]
    pub fn load(file_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "TryLoad", args = 1)]
    pub fn try_load(file_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "LoadImpl", args = 2)]
    pub fn load_impl(file_name: ::unity2::Il2CppString, is_warning: bool) -> bool;

    #[method(name = "LoadImpl", args = 4)]
    pub fn load_impl_2(
        file_path: ::unity2::Il2CppString,
        file_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<
            ::unity2::Il2CppString,
            crate::app::msgfile::MsgFile,
        >,
        data_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<
            ::unity2::Il2CppString,
            ::unity2::IntPtr,
        >,
        is_warning: bool,
    ) -> bool;

    #[method(name = "Free", args = 1)]
    pub fn free(file_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "TryFree", args = 1)]
    pub fn try_free(file_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "FreeImpl", args = 2)]
    pub fn free_impl(file_name: ::unity2::Il2CppString, is_warning: bool) -> bool;

    #[method(name = "GetReference", args = 1)]
    pub fn get_reference(file_name: ::unity2::Il2CppString) -> i32;

    #[method(name = "FreeImpl", args = 4)]
    pub fn free_impl_2(
        file_name: ::unity2::Il2CppString,
        file_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<
            ::unity2::Il2CppString,
            crate::app::msgfile::MsgFile,
        >,
        data_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<
            ::unity2::Il2CppString,
            ::unity2::IntPtr,
        >,
        is_warning: bool,
    ) -> bool;

    #[method(name = "Reload", args = 0)]
    pub fn reload() -> ();

    #[method(name = "IsLoadDone", args = 1)]
    pub fn is_load_done(file_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsFileExist", args = 1)]
    pub fn is_file_exist(file_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsHeroFemale", args = 0)]
    pub fn is_hero_female() -> bool;

    #[method(name = "GetLanguageDirectoryName", args = 0)]
    pub fn get_language_directory_name() -> ::unity2::Il2CppString;

    #[method(name = "CreateMessFilePath", args = 1)]
    pub fn create_mess_file_path(file_name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "CreateSoundCmdFilePath", args = 1)]
    pub fn create_sound_cmd_file_path(file_name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "CreateEventCmdFilePath", args = 1)]
    pub fn create_event_cmd_file_path(file_name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "GetFilePath", args = 1)]
    pub fn get_file_path(label: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "GetMsgFile", args = 1)]
    pub fn get_msg_file(file_name: ::unity2::Il2CppString) -> crate::app::msgfile::MsgFile;

    #[method(name = "IsMainReleaseScenarioMessFile", args = 1)]
    pub fn is_main_release_scenario_mess_file(file_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsDLCScenarioMessFile", args = 1)]
    pub fn is_dlc_scenario_mess_file(file_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsRelianceMessFile", args = 1)]
    pub fn is_reliance_mess_file(file_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsGodRelianceMessFile", args = 1)]
    pub fn is_god_reliance_mess_file(file_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "DumpFile", args = 0)]
    pub fn dump_file() -> ();

    #[method(name = "DumpLabel", args = 0)]
    pub fn dump_label() -> ();

    #[method(name = "GetAllLabel", args = 0)]
    pub fn get_all_label(
    ) -> crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>;

    #[method(name = "Get", args = 1)]
    pub fn get(label: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 2)]
    pub fn get_2(
        label: ::unity2::Il2CppString,
        arg0: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 3)]
    pub fn get_3(
        label: ::unity2::Il2CppString,
        arg0: ::unity2::Il2CppString,
        arg1: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 4)]
    pub fn get_4(
        label: ::unity2::Il2CppString,
        arg0: ::unity2::Il2CppString,
        arg1: ::unity2::Il2CppString,
        arg2: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 2)]
    pub fn get_5(
        label: ::unity2::Il2CppString,
        args: ::unity2::Array<::unity2::Il2CppString>,
    ) -> ::unity2::Il2CppString;

    #[method(name = "TryGetReplace", args = 1)]
    pub fn try_get_replace(label: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "GetIntPtr", args = 1)]
    pub fn get_int_ptr(label: ::unity2::Il2CppString) -> ::unity2::IntPtr;

    #[method(name = "GetImpl", args = 2)]
    pub fn get_impl(label: ::unity2::Il2CppString, is_replace: bool) -> ::unity2::Il2CppString;

    #[method(name = "GetSoundCmdText", args = 1)]
    pub fn get_sound_cmd_text(label: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "GetSoundCmdText", args = 3)]
    pub fn get_sound_cmd_text_2(
        label: ::unity2::Il2CppString,
        sound_cmd_exec_before: ::unity2::Il2CppString,
        sound_cmd_exec_after: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "GetEventCmdText", args = 1)]
    pub fn get_event_cmd_text(label: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "GetEventCmdText", args = 3)]
    pub fn get_event_cmd_text_2(
        label: ::unity2::Il2CppString,
        event_cmd_exec_before: ::unity2::Il2CppString,
        event_cmd_exec_after: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "GetCmdImpl", args = 2)]
    pub fn get_cmd_impl(
        label: ::unity2::Il2CppString,
        data_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<
            ::unity2::Il2CppString,
            ::unity2::IntPtr,
        >,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetCmdImpl", args = 4)]
    pub fn get_cmd_impl_2(
        label: ::unity2::Il2CppString,
        data_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<
            ::unity2::Il2CppString,
            ::unity2::IntPtr,
        >,
        cmd_exec_before: ::unity2::Il2CppString,
        cmd_exec_after: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "IsExist", args = 1)]
    pub fn is_exist(label: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsNesting", args = 0)]
    pub fn is_nesting() -> bool;

    #[method(name = "GetString", args = 3)]
    pub fn get_string(
        label: ::unity2::Il2CppString,
        p_mess_data: ::unity2::IntPtr,
        is_replace: bool,
    ) -> ::unity2::Il2CppString;

    #[method(name = "ReadChar", args = 2)]
    pub fn read_char(p: ::unity2::IntPtr, offset: i32) -> u16;

    #[method(name = "CalcTalkStringTotalWidth", args = 1)]
    pub fn calc_talk_string_total_width(mess_str: ::unity2::Il2CppString) -> f32;

    #[method(name = "CalcWaitMsecForTalkAutoPlay", args = 1)]
    pub fn calc_wait_msec_for_talk_auto_play(mess_str: ::unity2::Il2CppString) -> f32;

    #[method(name = "SplitCmd", args = 3)]
    pub fn split_cmd(
        cmd: ::unity2::Il2CppString,
        cmd_exec_before: ::unity2::Il2CppString,
        cmd_exec_after: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "AddTagString", args = 3)]
    pub fn add_tag_string(tag_group_id: u16, tag_id: u16, param: ::unity2::Array<u8>) -> ();

    #[method(name = "ReplaceArgString", args = 1)]
    pub fn replace_arg_string(tag_id: crate::app::mess::Mess_TagID_Arg) -> ();

    #[method(name = "AddTagString_Localize", args = 2)]
    pub fn add_tag_string_localize(tag_id: u16, param: ::unity2::Array<u8>) -> ();

    #[method(name = "IsPatchim1", args = 1)]
    pub fn is_patchim1(c: u16) -> bool;

    #[method(name = "IsPatchim2", args = 1)]
    pub fn is_patchim2(c: u16) -> bool;

    #[method(name = "IsPatchimStrCode", args = 2)]
    pub fn is_patchim_str_code(chr_code: i32, is_without2: bool) -> bool;

    #[method(name = "SetArgument", args = 2)]
    pub fn set_argument(index: i32, value: i32) -> ();

    #[method(name = "SetArgument", args = 2)]
    pub fn set_argument_2(index: i32, value: ::unity2::Il2CppString) -> ();

    #[method(name = "ClearArgument", args = 0)]
    pub fn clear_argument() -> ();

    #[method(name = "GetArgument", args = 1)]
    pub fn get_argument(index: i32) -> ::unity2::Il2CppString;

    #[method(name = "get_NewArgScope", args = 0)]
    pub fn get_new_arg_scope() -> crate::app::mess::Mess_ArgScope;

    #[method(name = "CreateSpaceTag", args = 1)]
    pub fn create_space_tag(pixel: u32) -> ::unity2::Il2CppString;

    #[method(name = "CreateSpriteTag", args = 2)]
    pub fn create_sprite_tag(
        icon_category: crate::app::mess::Mess_IconCategory,
        kind_name: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "CreateSpriteTag", args = 2)]
    pub fn create_sprite_tag_2(
        sprite_asset_name: ::unity2::Il2CppString,
        icon_name: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "CreateItemSpriteTag", args = 2)]
    pub fn create_item_sprite_tag(
        item: crate::app::itemdata::ItemData,
        for_system: bool,
    ) -> ::unity2::Il2CppString;

    #[method(name = "CreateSkillSpriteTag", args = 1)]
    pub fn create_skill_sprite_tag(
        skill: crate::app::skilldata::SkillData,
    ) -> ::unity2::Il2CppString;

    #[method(name = "CreateSystemSpriteTag", args = 1)]
    pub fn create_system_sprite_tag(icon_name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "CreateGodSymbleEngraveSpriteTag", args = 1)]
    pub fn create_god_symble_engrave_sprite_tag(
        icon_name: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "SetPartnerPID", args = 1)]
    pub fn set_partner_pid(pid: ::unity2::Il2CppString) -> ();

    #[method(name = "GetPartnerPID", args = 0)]
    pub fn get_partner_pid() -> ::unity2::Il2CppString;

    #[method(name = "GetPartnerName", args = 0)]
    pub fn get_partner_name() -> ::unity2::Il2CppString;

    #[method(name = "GetGameDataName", args = 1)]
    pub fn get_game_data_name(value: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-mess")]
impl Mess {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Mess),
                ::core::stringify!(new),
            )
        });
        <Self as IMessMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mess/Mess_LanguageScope.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Mess_LanguageScope {
    pub m_count: i32,
    pub m_name: ::unity2::Il2CppString,
    pub m_lang: crate::app::language::Language_Langs,
}

impl ::unity2::ClassIdentity for Mess_LanguageScope {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Mess.LanguageScope";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Mess_LanguageScope {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-mess")]
#[::unity2::methods(value)]
impl Mess_LanguageScope {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        name: ::unity2::Il2CppString,
        lang: crate::app::language::Language_Langs,
    ) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mess/Mess_TagID_Picture.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Mess_TagID_Picture {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Mess_TagID_Picture {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Mess.TagID_Picture";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Mess_TagID_Picture {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Mess_TagID_Picture {
    pub fn show() -> Self {
        Self { value: 0 }
    }

    pub fn hide() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mess/Mess_ReloadFileInfo.md")))]
#[::unity2::class(namespace = "App", name = "Mess.ReloadFileInfo")]
#[parent(crate::system::object::Object)]
pub struct Mess_ReloadFileInfo {
    #[rename(name = "m_fileName")]
    pub m_file_name: ::unity2::Il2CppString,
    #[rename(name = "m_refCount")]
    pub m_ref_count: i32,
}

#[cfg(feature = "app-mess")]
#[::unity2::methods]
impl Mess_ReloadFileInfo {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, file_name: ::unity2::Il2CppString, ref_count: i32) -> ();
}

#[cfg(feature = "app-mess")]
impl Mess_ReloadFileInfo {
    pub fn new(file_name: ::unity2::Il2CppString, ref_count: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Mess_ReloadFileInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IMess_ReloadFileInfoMethods>::ctor(this, file_name, ref_count);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mess/Mess_TagID_Name.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Mess_TagID_Name {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Mess_TagID_Name {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Mess.TagID_Name";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Mess_TagID_Name {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Mess_TagID_Name {
    pub fn replace() -> Self {
        Self { value: 0 }
    }

    pub fn publish() -> Self {
        Self { value: 1 }
    }

    pub fn private() -> Self {
        Self { value: 2 }
    }

    pub fn user() -> Self {
        Self { value: 3 }
    }

    pub fn partner() -> Self {
        Self { value: 4 }
    }

    pub fn mascot() -> Self {
        Self { value: 5 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mess/Mess_ArgScope.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Mess_ArgScope {
    pub current: ::unity2::Array<::unity2::Il2CppString>,
}

impl ::unity2::ClassIdentity for Mess_ArgScope {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Mess.ArgScope";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Mess_ArgScope {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-mess")]
#[::unity2::methods(value)]
impl Mess_ArgScope {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, dummy: i32) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}
