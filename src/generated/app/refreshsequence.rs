
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refreshsequence/RefreshSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct RefreshSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for RefreshSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "RefreshSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RefreshSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl RefreshSequence_Label {
    pub fn entry() -> Self {
        Self { value: 0 }
    }

    pub fn load_resources() -> Self {
        Self { value: 1 }
    }

    pub fn create_root() -> Self {
        Self { value: 2 }
    }

    pub fn facility_select() -> Self {
        Self { value: 3 }
    }

    pub fn check_visited() -> Self {
        Self { value: 4 }
    }

    pub fn unit_set() -> Self {
        Self { value: 5 }
    }

    pub fn unit_select() -> Self {
        Self { value: 6 }
    }

    pub fn confirm() -> Self {
        Self { value: 7 }
    }

    pub fn demo() -> Self {
        Self { value: 8 }
    }

    pub fn result() -> Self {
        Self { value: 9 }
    }

    pub fn end() -> Self {
        Self { value: 10 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refreshsequence/RefreshSequence.md")))]
#[::unity2::class(namespace = "App", name = "RefreshSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct RefreshSequence {
    #[static_field]
    #[rename(name = "m_RelianceExpParamName")]
    pub m_reliance_exp_param_name: ::unity2::Il2CppString,
    #[rename(name = "m_RefreshUnitSelectRoot")]
    pub m_refresh_unit_select_root: crate::app::refreshunitselectroot::RefreshUnitSelectRoot,
    #[rename(name = "m_FacilityAid")]
    pub m_facility_aid: ::unity2::Il2CppString,
    #[rename(name = "m_EnteredFromFacility")]
    pub m_entered_from_facility: bool,
    #[rename(name = "m_Unit")]
    pub m_unit: ::unity2::Array<crate::app::unit::Unit>,
    #[rename(name = "m_UnitIndex")]
    pub m_unit_index: i32,
    #[rename(name = "m_AddedRelianceExp")]
    pub m_added_reliance_exp: bool,
    #[rename(name = "m_UnitSetMenu")]
    pub m_unit_set_menu: crate::app::refreshunitsetmenu::RefreshUnitSetMenu,
    #[rename(name = "m_UnitSetMenuResult")]
    pub m_unit_set_menu_result: crate::app::refreshunitsetmenu::RefreshUnitSetMenu_Result2,
    #[rename(name = "m_ConfirmDialogResult")]
    pub m_confirm_dialog_result: crate::app::refreshconfirmdialog::RefreshConfirmDialog_Result2,
    #[rename(name = "m_UnitSelectMenuScrollIndex")]
    pub m_unit_select_menu_scroll_index: i32,
}

#[cfg(feature = "app-refreshsequence")]
#[::unity2::methods]
impl RefreshSequence {
    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst, aid: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, aid: ::unity2::Il2CppString) -> ();

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "HadVisited", args = 0)]
    pub fn had_visited(self) -> bool;

    #[method(name = "ShowAlreadyVisitedDialog", args = 0)]
    pub fn show_already_visited_dialog(self) -> ();

    #[method(name = "LoadResources", args = 0)]
    pub fn load_resources(self) -> ();

    #[method(name = "IsLoadingResources", args = 0)]
    pub fn is_loading_resources(self) -> bool;

    #[method(name = "StartSequence", args = 0)]
    pub fn start_sequence(self) -> ();

    #[method(name = "CreateUnitSelectRoot", args = 0)]
    pub fn create_unit_select_root(self) -> ();

    #[method(name = "CreateFacilitySelectMenu", args = 0)]
    pub fn create_facility_select_menu(self) -> ();

    #[method(name = "CreateUnitSet", args = 0)]
    pub fn create_unit_set(self) -> ();

    #[method(name = "CreateUnitSelect", args = 0)]
    pub fn create_unit_select(self) -> ();

    #[method(name = "ShowConfirmDialog", args = 0)]
    pub fn show_confirm_dialog(self) -> ();

    #[method(name = "CloseUnitSelectRoot", args = 0)]
    pub fn close_unit_select_root(self) -> ();

    #[method(name = "IsClosedUnitSelectRoot", args = 0)]
    pub fn is_closed_unit_select_root(self) -> bool;

    #[method(name = "DestroyUnitSelectRoot", args = 0)]
    pub fn destroy_unit_select_root(self) -> ();

    #[method(name = "CalcReliance", args = 0)]
    pub fn calc_reliance(self) -> ();

    #[method(name = "CreateDemo", args = 0)]
    pub fn create_demo(self) -> ();

    #[method(name = "ShowResultDialog", args = 0)]
    pub fn show_result_dialog(self) -> ();

    #[method(name = "EndSequence", args = 0)]
    pub fn end_sequence(self) -> ();

    #[method(name = "UnloadResources", args = 0)]
    pub fn unload_resources(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-refreshsequence")]
impl RefreshSequence {
    pub fn new(aid: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefreshSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IRefreshSequenceMethods>::ctor(this, aid);
        this
    }
}
