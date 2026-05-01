
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubaccessmanager/HubAccessManager.md")))]
#[::unity2::class(namespace = "App", name = "HubAccessManager")]
#[parent(crate::system::object::Object)]
pub struct HubAccessManager {}

#[cfg(feature = "app-hubaccessmanager")]
#[::unity2::methods]
impl HubAccessManager {
    #[method(name = "get_SceneName", args = 0)]
    pub fn get_scene_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_SceneName", args = 1)]
    pub fn set_scene_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AccessList", args = 0)]
    pub fn get_access_list(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::hubaccessdata::HubAccessData>;

    #[method(name = "set_AccessList", args = 1)]
    pub fn set_access_list(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::app::hubaccessdata::HubAccessData,
        >,
    ) -> ();

    #[method(name = "get_DisposList", args = 0)]
    pub fn get_dispos_list(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::hubdisposdata::HubDisposData>;

    #[method(name = "set_DisposList", args = 1)]
    pub fn set_dispos_list(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::app::hubdisposdata::HubDisposData,
        >,
    ) -> ();

    #[method(name = "get_DisposItemList", args = 0)]
    pub fn get_dispos_item_list(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::hubdisposdata::HubDisposData>;

    #[method(name = "set_DisposItemList", args = 1)]
    pub fn set_dispos_item_list(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::app::hubdisposdata::HubDisposData,
        >,
    ) -> ();

    #[method(name = "get_AnimalDataList", args = 0)]
    pub fn get_animal_data_list(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::animaldata::AnimalData>;

    #[method(name = "set_AnimalDataList", args = 1)]
    pub fn set_animal_data_list(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::app::animaldata::AnimalData,
        >,
    ) -> ();

    #[method(name = "IsItemType", args = 1)]
    pub fn is_item_type(dispos: crate::app::hubdisposdata::HubDisposData) -> bool;

    #[method(name = "Setup", args = 2)]
    pub fn setup(
        self,
        scene_name: ::unity2::Il2CppString,
        timezone_type: crate::app::hubutil::HubUtil_TimezoneType,
    ) -> ();

    #[method(name = "ConfirmContent", args = 0)]
    pub fn confirm_content(self) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "Refresh", args = 0)]
    pub fn refresh(self) -> ();

    #[method(name = "IsUsedLocator", args = 1)]
    pub fn is_used_locator(self, locator_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "TrySetAccessObject", args = 2)]
    pub fn try_set_access_object(
        self,
        data: crate::app::hubdisposdata::HubDisposData,
        random: crate::app::random_2::Random_2,
    ) -> bool;

    #[method(name = "TryRemoveAccessObject", args = 1)]
    pub fn try_remove_access_object(self, data: crate::app::hubdisposdata::HubDisposData) -> bool;

    #[method(name = "AddNewLocator", args = 1)]
    pub fn add_new_locator(
        self,
        locator: ::unity2::Il2CppString,
    ) -> crate::app::hubaccessdata::HubAccessData;

    #[method(name = "ClearLocator", args = 1)]
    pub fn clear_locator(self, locator: ::unity2::Il2CppString) -> ();

    #[method(name = "FindLocator", args = 1)]
    pub fn find_locator(
        self,
        locator: ::unity2::Il2CppString,
    ) -> crate::app::hubaccessdata::HubAccessData;

    #[method(name = "FindPID", args = 1)]
    pub fn find_pid(self, pid: ::unity2::Il2CppString) -> crate::app::hubaccessdata::HubAccessData;

    #[method(name = "IsAlreadyLocated", args = 1)]
    pub fn is_already_located(self, pid: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsAvailablePID", args = 2)]
    pub fn is_available_pid(self, pid: ::unity2::Il2CppString, disabled_talk: bool) -> bool;

    #[method(name = "GetSelectedGodWithSpecial", args = 1)]
    pub fn get_selected_god_with_special(
        self,
        god_unit: crate::app::godunit::GodUnit,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetPlayerGod", args = 0)]
    pub fn get_player_god(self) -> ::unity2::Il2CppString;

    #[method(name = "GetChooseID", args = 3)]
    pub fn get_choose_id(
        self,
        data: crate::app::hubdisposdata::HubDisposData,
        count: i32,
        random: crate::app::random_2::Random_2,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetChooseAnimalID", args = 2)]
    pub fn get_choose_animal_id(
        self,
        data: crate::app::hubdisposdata::HubDisposData,
        random: crate::app::random_2::Random_2,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetChooseAnimalItem", args = 1)]
    pub fn get_choose_animal_item(
        self,
        data: crate::app::hubdisposdata::HubDisposData,
    ) -> ::unity2::Il2CppString;

    #[method(name = "EntryTalkLimit", args = 1)]
    pub fn entry_talk_limit(self, talk_type: ::unity2::Il2CppString) -> bool;

    #[method(name = "GetNotTakedPieceOfBond", args = 0)]
    pub fn get_not_taked_piece_of_bond(self) -> i32;

    #[method(name = "Dump", args = 0)]
    pub fn dump(self) -> ();

    #[method(name = "CopyDisposList", args = 1)]
    pub fn copy_dispos_list(
        self,
        scene_name: ::unity2::Il2CppString,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::hubdisposdata::HubDisposData>;

    #[method(name = "CopyAnimalList", args = 0)]
    pub fn copy_animal_list(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::animaldata::AnimalData>;

    #[method(name = "ConfirmMaterial", args = 0)]
    pub fn confirm_material(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubaccessmanager")]
impl HubAccessManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubAccessManager),
                ::core::stringify!(new),
            )
        });
        <Self as IHubAccessManagerMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubaccessmanager/HubAccessManager_MaterialCalculator.md")))]
#[::unity2::class(namespace = "App", name = "HubAccessManager.MaterialCalculator")]
#[parent(crate::system::object::Object)]
pub struct HubAccessManager_MaterialCalculator {}

#[cfg(feature = "app-hubaccessmanager")]
#[::unity2::methods]
impl HubAccessManager_MaterialCalculator {
    #[method(name = "GenerateHigherMaterials", args = 1)]
    pub fn generate_higher_materials(
        num: i32,
    ) -> crate::system::collections::generic::dictionary_2::Dictionary_2<
        crate::app::hubaccessmanager::HubAccessManager_MaterialCalculator_Type,
        i32,
    >;

    #[method(name = "GetValidHigherMateriaslType", args = 1)]
    pub fn get_valid_higher_materiasl_type(
        materials: crate::system::collections::generic::dictionary_2::Dictionary_2<
            crate::app::hubaccessmanager::HubAccessManager_MaterialCalculator_Type,
            i32,
        >,
    ) -> crate::app::hubaccessmanager::HubAccessManager_MaterialCalculator_Type;

    #[method(name = "ConvertToLowerMaterialType", args = 1)]
    pub fn convert_to_lower_material_type(
        r#type: crate::app::hubaccessmanager::HubAccessManager_MaterialCalculator_Type,
    ) -> crate::app::hubaccessmanager::HubAccessManager_MaterialCalculator_Type;

    #[method(name = "FilterLowerMaterials", args = 1)]
    pub fn filter_lower_materials(
        materials: crate::system::collections::generic::dictionary_2::Dictionary_2<
            crate::app::hubaccessmanager::HubAccessManager_MaterialCalculator_Type,
            i32,
        >,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubaccessmanager")]
impl HubAccessManager_MaterialCalculator {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubAccessManager_MaterialCalculator),
                ::core::stringify!(new),
            )
        });
        <Self as IHubAccessManager_MaterialCalculatorMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubaccessmanager/HubAccessManager_MaterialCalculator_Type.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct HubAccessManager_MaterialCalculator_Type {
    pub value: i32,
}

impl ::unity2::ClassIdentity for HubAccessManager_MaterialCalculator_Type {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "HubAccessManager.MaterialCalculator.Type";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for HubAccessManager_MaterialCalculator_Type {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl HubAccessManager_MaterialCalculator_Type {
    pub fn iron() -> Self {
        Self { value: 0 }
    }

    pub fn steel() -> Self {
        Self { value: 1 }
    }

    pub fn silver() -> Self {
        Self { value: 2 }
    }
}
