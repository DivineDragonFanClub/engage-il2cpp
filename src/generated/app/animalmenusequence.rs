
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemno::BasicDialogItemNo;
use crate::app::basicdialogitemno::IBasicDialogItemNo;
use crate::app::basicdialogitemyes::BasicDialogItemYes;
use crate::app::basicdialogitemyes::IBasicDialogItemYes;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/animalmenusequence/AnimalMenuSequence.md")))]
#[::unity2::class(namespace = "App", name = "AnimalMenuSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: animalmenusequence :: AnimalMenuSequence >)]
pub struct AnimalMenuSequence {
    #[rename(name = "AnimalPID")]
    pub animal_pid: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_Handle")]
    pub m_handle: crate::app::resourcehandle_2::ResourceHandle_2,
    #[rename(name = "m_RootAnim")]
    pub m_root_anim: crate::unity_engine::animator::Animator,
}

#[cfg(feature = "app-animalmenusequence")]
#[::unity2::methods]
impl AnimalMenuSequence {
    #[method(name = "get_Inside", args = 0)]
    pub fn get_inside(self) -> crate::app::animalinsidemenu::AnimalInsideMenu;

    #[method(name = "set_Inside", args = 1)]
    pub fn set_inside(self, value: crate::app::animalinsidemenu::AnimalInsideMenu) -> ();

    #[method(name = "get_Outside", args = 0)]
    pub fn get_outside(self) -> crate::app::animaloutsidemenu::AnimalOutsideMenu;

    #[method(name = "set_Outside", args = 1)]
    pub fn set_outside(self, value: crate::app::animaloutsidemenu::AnimalOutsideMenu) -> ();

    #[method(name = "get_InsideContent", args = 0)]
    pub fn get_inside_content(self)
        -> crate::app::animalinsidemenucontent::AnimalInsideMenuContent;

    #[method(name = "set_InsideContent", args = 1)]
    pub fn set_inside_content(
        self,
        value: crate::app::animalinsidemenucontent::AnimalInsideMenuContent,
    ) -> ();

    #[method(name = "get_OutsideContent", args = 0)]
    pub fn get_outside_content(
        self,
    ) -> crate::app::animaloutsidemenucontent::AnimalOutsideMenuContent;

    #[method(name = "set_OutsideContent", args = 1)]
    pub fn set_outside_content(
        self,
        value: crate::app::animaloutsidemenucontent::AnimalOutsideMenuContent,
    ) -> ();

    #[method(name = "get_IsOutsideSwap", args = 0)]
    pub fn get_is_outside_swap(self) -> bool;

    #[method(name = "set_IsOutsideSwap", args = 1)]
    pub fn set_is_outside_swap(self, value: bool) -> ();

    #[method(name = "get_IsInsideSwap", args = 0)]
    pub fn get_is_inside_swap(self) -> bool;

    #[method(name = "set_IsInsideSwap", args = 1)]
    pub fn set_is_inside_swap(self, value: bool) -> ();

    #[method(name = "get_IsBlankSelect", args = 0)]
    pub fn get_is_blank_select(self) -> bool;

    #[method(name = "set_IsBlankSelect", args = 1)]
    pub fn set_is_blank_select(self, value: bool) -> ();

    #[method(name = "get_FromMenu", args = 0)]
    pub fn get_from_menu(self) -> crate::app::animalmenusequence::AnimalMenuSequence_From;

    #[method(name = "set_FromMenu", args = 1)]
    pub fn set_from_menu(
        self,
        value: crate::app::animalmenusequence::AnimalMenuSequence_From,
    ) -> ();

    #[method(name = "get_CursorPosition", args = 0)]
    pub fn get_cursor_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_CursorPosition", args = 1)]
    pub fn set_cursor_position(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_CurrentAnimal", args = 0)]
    pub fn get_current_animal(self) -> crate::app::animaldata::AnimalData;

    #[method(name = "set_CurrentAnimal", args = 1)]
    pub fn set_current_animal(self, value: crate::app::animaldata::AnimalData) -> ();

    #[method(name = "get_Root", args = 0)]
    pub fn get_root(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_Root", args = 1)]
    pub fn set_root(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "get_SelectRoot", args = 0)]
    pub fn get_select_root(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_SelectRoot", args = 1)]
    pub fn set_select_root(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "get_HelpRoot", args = 0)]
    pub fn get_help_root(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_HelpRoot", args = 1)]
    pub fn set_help_root(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "get_HelpMessage", args = 0)]
    pub fn get_help_message(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_HelpMessage", args = 1)]
    pub fn set_help_message(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "SetHelpText", args = 1)]
    pub fn set_help_text(help_text: ::unity2::Il2CppString) -> ();

    #[method(name = "JumpInside", args = 1)]
    pub fn jump_inside(is_outside_swap: bool) -> ();

    #[method(name = "JumpOutside", args = 1)]
    pub fn jump_outside(is_inside_swap: bool) -> ();

    #[method(name = "JumpLeft", args = 0)]
    pub fn jump_left() -> ();

    #[method(name = "JumpRight", args = 0)]
    pub fn jump_right() -> ();

    #[method(name = "JumpConfirm", args = 1)]
    pub fn jump_confirm(from: crate::app::animalmenusequence::AnimalMenuSequence_From) -> ();

    #[method(name = "JumpApply", args = 0)]
    pub fn jump_apply() -> ();

    #[method(name = "GetAnimalCount", args = 1)]
    pub fn get_animal_count(animal: crate::app::animaldata::AnimalData) -> i32;

    #[method(name = "SetAnimal", args = 2)]
    pub fn set_animal(animal: crate::app::animaldata::AnimalData, select_index: i32) -> ();

    #[method(name = "SwapAnimal", args = 2)]
    pub fn swap_animal(select_index1: i32, select_index2: i32) -> ();

    #[method(name = "IsPlaceEmpty", args = 0)]
    pub fn is_place_empty() -> bool;

    #[method(name = "PlaceAnimal", args = 1)]
    pub fn place_animal(animal: crate::app::animaldata::AnimalData) -> ();

    #[method(name = "GetEmptyIndex", args = 0)]
    pub fn get_empty_index() -> i32;

    #[method(name = "IsExistsCurrentAnimal", args = 0)]
    pub fn is_exists_current_animal() -> bool;

    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async(self) -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab(self) -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab(self) -> ();

    #[method(name = "Tutorial", args = 0)]
    pub fn tutorial(self) -> ();

    #[method(name = "Create", args = 0)]
    pub fn create(self) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = "IsClosed", args = 0)]
    pub fn is_closed(self) -> bool;

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();

    #[method(name = "LoadData", args = 0)]
    pub fn load_data(self) -> ();

    #[method(name = "IsDirtyData", args = 0)]
    pub fn is_dirty_data(self) -> bool;

    #[method(name = "ApplyData", args = 0)]
    pub fn apply_data(self) -> ();

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "Return", args = 0)]
    pub fn r#return(self) -> ();

    #[method(name = "Confirm", args = 0)]
    pub fn confirm(self) -> ();

    #[method(name = "RemoveIndex", args = 1)]
    pub fn remove_index(index: i32) -> ();

    #[method(name = "RemoveCurrentAnimal", args = 1)]
    pub fn remove_current_animal(animal: crate::app::animaldata::AnimalData) -> ();

    #[method(name = "InitInside", args = 0)]
    pub fn init_inside(self) -> ();

    #[method(name = "InitOutside", args = 0)]
    pub fn init_outside(self) -> ();

    #[method(name = "InitDecide", args = 0)]
    pub fn init_decide(self) -> ();

    #[method(name = "TickInside", args = 0)]
    pub fn tick_inside(self) -> ();

    #[method(name = "TickOutside", args = 0)]
    pub fn tick_outside(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> crate::app::procinst::ProcInst;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-animalmenusequence")]
impl AnimalMenuSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AnimalMenuSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IAnimalMenuSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/animalmenusequence/AnimalMenuSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct AnimalMenuSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for AnimalMenuSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "AnimalMenuSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AnimalMenuSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl AnimalMenuSequence_Label {
    pub fn init() -> Self {
        Self { value: 0 }
    }

    pub fn outside_select() -> Self {
        Self { value: 1 }
    }

    pub fn inside_select() -> Self {
        Self { value: 2 }
    }

    pub fn confirm() -> Self {
        Self { value: 3 }
    }

    pub fn apply() -> Self {
        Self { value: 4 }
    }

    pub fn end() -> Self {
        Self { value: 5 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/animalmenusequence/AnimalMenuSequence_From.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct AnimalMenuSequence_From {
    pub value: i32,
}

impl ::unity2::ClassIdentity for AnimalMenuSequence_From {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "AnimalMenuSequence.From";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AnimalMenuSequence_From {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl AnimalMenuSequence_From {
    pub fn outside() -> Self {
        Self { value: 0 }
    }

    pub fn inside() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/animalmenusequence/AnimalMenuSequence_ConfirmYes.md")))]
#[::unity2::class(namespace = "App", name = "AnimalMenuSequence.ConfirmYes")]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct AnimalMenuSequence_ConfirmYes {}

#[cfg(feature = "app-animalmenusequence")]
#[::unity2::methods]
impl AnimalMenuSequence_ConfirmYes {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-animalmenusequence")]
impl AnimalMenuSequence_ConfirmYes {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AnimalMenuSequence_ConfirmYes),
                ::core::stringify!(new),
            )
        });
        <Self as IAnimalMenuSequence_ConfirmYesMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/animalmenusequence/AnimalMenuSequence_ConfirmNo.md")))]
#[::unity2::class(namespace = "App", name = "AnimalMenuSequence.ConfirmNo")]
#[parent(crate::app::basicdialogitemno::BasicDialogItemNo)]
pub struct AnimalMenuSequence_ConfirmNo {}

#[cfg(feature = "app-animalmenusequence")]
#[::unity2::methods]
impl AnimalMenuSequence_ConfirmNo {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-animalmenusequence")]
impl AnimalMenuSequence_ConfirmNo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AnimalMenuSequence_ConfirmNo),
                ::core::stringify!(new),
            )
        });
        <Self as IAnimalMenuSequence_ConfirmNoMethods>::ctor(this);
        this
    }
}
