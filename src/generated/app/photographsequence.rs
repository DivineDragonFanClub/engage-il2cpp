
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/photographsequence/PhotographSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct PhotographSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for PhotographSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "PhotographSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for PhotographSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl PhotographSequence_Label {
    pub fn entry() -> Self {
        Self { value: 0 }
    }

    pub fn select_dispos() -> Self {
        Self { value: 1 }
    }

    pub fn edit_dispos() -> Self {
        Self { value: 2 }
    }

    pub fn select_character() -> Self {
        Self { value: 3 }
    }

    pub fn select_body_acc() -> Self {
        Self { value: 4 }
    }

    pub fn select_face_acc() -> Self {
        Self { value: 5 }
    }

    pub fn select_pause() -> Self {
        Self { value: 6 }
    }

    pub fn select_scarf_color() -> Self {
        Self { value: 7 }
    }

    pub fn select_weapon() -> Self {
        Self { value: 8 }
    }

    pub fn photograph_mode() -> Self {
        Self { value: 9 }
    }

    pub fn exit() -> Self {
        Self { value: 10 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/photographsequence/PhotographSequence.md")))]
#[::unity2::class(namespace = "App", name = "PhotographSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct PhotographSequence {
    #[rename(name = "m_ReservedLabel")]
    pub m_reserved_label: crate::app::photographsequence::PhotographSequence_Label,
    #[rename(name = "m_AllMenuContent")]
    pub m_all_menu_content: crate::app::photographallmenucontent::PhotographAllMenuContent,
    #[rename(name = "m_SpotObj")]
    pub m_spot_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_DisposManager")]
    pub m_dispos_manager: crate::app::photographdisposmanager::PhotographDisposManager,
    #[rename(name = "m_CameraController")]
    pub m_camera_controller: crate::app::photographcameracontroller::PhotographCameraController,
    #[rename(name = "m_AnimalLocatorNameList")]
    pub m_animal_locator_name_list:
        crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
}

#[cfg(feature = "app-photographsequence")]
#[::unity2::methods]
impl PhotographSequence {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "OpenKeyHelp", args = 1)]
    pub fn open_key_help(self, key_help_id: ::unity2::Il2CppString) -> ();

    #[method(name = "CloseKeyHelp", args = 0)]
    pub fn close_key_help(self) -> ();

    #[method(name = "StartSequence", args = 0)]
    pub fn start_sequence(self) -> ();

    #[method(name = "OpenSelectDisposMenu", args = 0)]
    pub fn open_select_dispos_menu(self) -> ();

    #[method(name = "CloseSelectDisposMenu", args = 0)]
    pub fn close_select_dispos_menu(self) -> ();

    #[method(name = "OpenEditDisposMenu", args = 0)]
    pub fn open_edit_dispos_menu(self) -> ();

    #[method(name = "CloseEditDisposMenu", args = 0)]
    pub fn close_edit_dispos_menu(self) -> ();

    #[method(name = "OpenSelectCharacterMenu", args = 0)]
    pub fn open_select_character_menu(self) -> ();

    #[method(name = "CloseSelectCharacterMenu", args = 0)]
    pub fn close_select_character_menu(self) -> ();

    #[method(name = "OpenSelectBodyAccMenu", args = 0)]
    pub fn open_select_body_acc_menu(self) -> ();

    #[method(name = "CloseSelectBodyAccMenu", args = 0)]
    pub fn close_select_body_acc_menu(self) -> ();

    #[method(name = "OpenSelectFaceAccMenu", args = 0)]
    pub fn open_select_face_acc_menu(self) -> ();

    #[method(name = "CloseSelectFaceAccMenu", args = 0)]
    pub fn close_select_face_acc_menu(self) -> ();

    #[method(name = "OpenSelectPauseMenu", args = 0)]
    pub fn open_select_pause_menu(self) -> ();

    #[method(name = "CloseSelectPauseMenu", args = 0)]
    pub fn close_select_pause_menu(self) -> ();

    #[method(name = "OpenSelectScarfColorMenu", args = 0)]
    pub fn open_select_scarf_color_menu(self) -> ();

    #[method(name = "CloseSelectScarfColorMenu", args = 0)]
    pub fn close_select_scarf_color_menu(self) -> ();

    #[method(name = "OpenSelectWeaponMenu", args = 0)]
    pub fn open_select_weapon_menu(self) -> ();

    #[method(name = "CloseSelectWeaponMenu", args = 0)]
    pub fn close_select_weapon_menu(self) -> ();

    #[method(name = "StartPhotographModeSequence", args = 0)]
    pub fn start_photograph_mode_sequence(self) -> ();

    #[method(name = "EndPhotographModeSequence", args = 0)]
    pub fn end_photograph_mode_sequence(self) -> ();

    #[method(name = "EndSequence", args = 0)]
    pub fn end_sequence(self) -> ();

    #[method(name = "LoadResource", args = 0)]
    pub fn load_resource(self) -> ();

    #[method(name = "IsLoadingResource", args = 0)]
    pub fn is_loading_resource(self) -> bool;

    #[method(name = "UnloadResource", args = 0)]
    pub fn unload_resource(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-photographsequence")]
impl PhotographSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PhotographSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IPhotographSequenceMethods>::ctor(this);
        this
    }
}
