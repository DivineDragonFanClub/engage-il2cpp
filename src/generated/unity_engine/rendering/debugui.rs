
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugui/DebugUI_UIntField.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugUI.UIntField")]
# [parent (crate :: unity_engine :: rendering :: debugui :: DebugUI_Field_1 < u32 >)]
pub struct DebugUI_UIntField {
    #[rename(name = "min")]
    pub min: crate::system::func_1::Func_1<u32>,
    #[rename(name = "max")]
    pub max: crate::system::func_1::Func_1<u32>,
    #[rename(name = "incStep")]
    pub inc_step: u32,
    #[rename(name = "intStepMult")]
    pub int_step_mult: u32,
}

#[cfg(feature = "unity_engine-rendering-debugui")]
#[::unity2::methods]
impl DebugUI_UIntField {
    #[method(name = "ValidateValue", args = 1)]
    pub fn validate_value(self, value: u32) -> u32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-debugui")]
impl DebugUI_UIntField {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUI_UIntField),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUI_UIntFieldMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugui/DebugUI_BoolField.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugUI.BoolField")]
# [parent (crate :: unity_engine :: rendering :: debugui :: DebugUI_Field_1 < bool >)]
pub struct DebugUI_BoolField {}

#[cfg(feature = "unity_engine-rendering-debugui")]
#[::unity2::methods]
impl DebugUI_BoolField {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-debugui")]
impl DebugUI_BoolField {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUI_BoolField),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUI_BoolFieldMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugui/DebugUI_Table_Row.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugUI.Table.Row")]
#[parent(crate::unity_engine::rendering::debugui::DebugUI_Foldout)]
pub struct DebugUI_Table_Row {}

#[cfg(feature = "unity_engine-rendering-debugui")]
#[::unity2::methods]
impl DebugUI_Table_Row {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-debugui")]
impl DebugUI_Table_Row {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUI_Table_Row),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUI_Table_RowMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugui/DebugUI_Vector4Field.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugUI.Vector4Field")]
# [parent (crate :: unity_engine :: rendering :: debugui :: DebugUI_Field_1 < crate :: unity_engine :: vector4 :: Vector4 >)]
pub struct DebugUI_Vector4Field {
    #[rename(name = "incStep")]
    pub inc_step: f32,
    #[rename(name = "incStepMult")]
    pub inc_step_mult: f32,
    #[rename(name = "decimals")]
    pub decimals: i32,
}

#[cfg(feature = "unity_engine-rendering-debugui")]
#[::unity2::methods]
impl DebugUI_Vector4Field {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-debugui")]
impl DebugUI_Vector4Field {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUI_Vector4Field),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUI_Vector4FieldMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugui/DebugUI_FloatField.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugUI.FloatField")]
# [parent (crate :: unity_engine :: rendering :: debugui :: DebugUI_Field_1 < f32 >)]
pub struct DebugUI_FloatField {
    #[rename(name = "min")]
    pub min: crate::system::func_1::Func_1<f32>,
    #[rename(name = "max")]
    pub max: crate::system::func_1::Func_1<f32>,
    #[rename(name = "incStep")]
    pub inc_step: f32,
    #[rename(name = "incStepMult")]
    pub inc_step_mult: f32,
    #[rename(name = "decimals")]
    pub decimals: i32,
}

#[cfg(feature = "unity_engine-rendering-debugui")]
#[::unity2::methods]
impl DebugUI_FloatField {
    #[method(name = "ValidateValue", args = 1)]
    pub fn validate_value(self, value: f32) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-debugui")]
impl DebugUI_FloatField {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUI_FloatField),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUI_FloatFieldMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugui/DebugUI_Foldout.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugUI.Foldout")]
#[parent(crate::unity_engine::rendering::debugui::DebugUI_Container)]
pub struct DebugUI_Foldout {
    #[rename(name = "opened")]
    pub opened: bool,
}

#[cfg(feature = "unity_engine-rendering-debugui")]
#[::unity2::methods]
impl DebugUI_Foldout {
    #[method(name = "get_isReadOnly", args = 0)]
    pub fn get_is_read_only(self) -> bool;

    #[method(name = "get_columnLabels", args = 0)]
    pub fn get_column_labels(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_columnLabels", args = 1)]
    pub fn set_column_labels(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_2(
        self,
        display_name: ::unity2::Il2CppString,
        children: crate::unity_engine::rendering::observablelist_1::ObservableList_1<
            crate::unity_engine::rendering::debugui::DebugUI_Widget,
        >,
        column_labels: ::unity2::Array<::unity2::Il2CppString>,
    ) -> ();

    #[method(name = "GetValue", args = 0)]
    pub fn get_value(self) -> bool;

    #[method(name = "UnityEngine.Rendering.DebugUI.IValueField.GetValue", args = 0)]
    pub fn unity_engine_rendering_debug_ui_i_value_field_get_value(
        self,
    ) -> crate::system::object::Object;

    #[method(name = "SetValue", args = 1)]
    pub fn set_value(self, value: crate::system::object::Object) -> ();

    #[method(name = "ValidateValue", args = 1)]
    pub fn validate_value(
        self,
        value: crate::system::object::Object,
    ) -> crate::system::object::Object;

    #[method(name = "SetValue", args = 1)]
    pub fn set_value_2(self, value: bool) -> ();
}

#[cfg(feature = "unity_engine-rendering-debugui")]
impl DebugUI_Foldout {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUI_Foldout),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUI_FoldoutMethods>::ctor(this);
        this
    }

    pub fn new_2(
        display_name: ::unity2::Il2CppString,
        children: crate::unity_engine::rendering::observablelist_1::ObservableList_1<
            crate::unity_engine::rendering::debugui::DebugUI_Widget,
        >,
        column_labels: ::unity2::Array<::unity2::Il2CppString>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUI_Foldout),
                ::core::stringify!(new_2),
            )
        });
        <Self as IDebugUI_FoldoutMethods>::ctor_2(this, display_name, children, column_labels);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugui/DebugUI_Field_1.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugUI.Field`1")]
pub struct DebugUI_Field_1<T0: ::unity2::ClassIdentity> {
    #[rename(name = "onValueChanged")]
    pub on_value_changed: crate::system::action_2::Action_2<
        crate::unity_engine::rendering::debugui::DebugUI_Field_1<T0>,
        T0,
    >,
}

#[cfg(feature = "unity_engine-rendering-debugui")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> DebugUI_Field_1<T0> {
    #[method(name = "get_getter", args = 0)]
    pub fn get_getter(self) -> crate::system::func_1::Func_1<T0>;

    #[method(name = "set_getter", args = 1)]
    pub fn set_getter(self, value: crate::system::func_1::Func_1<T0>) -> ();

    #[method(name = "get_setter", args = 0)]
    pub fn get_setter(self) -> crate::system::action_1::Action_1<T0>;

    #[method(name = "set_setter", args = 1)]
    pub fn set_setter(self, value: crate::system::action_1::Action_1<T0>) -> ();

    #[method(
        name = "UnityEngine.Rendering.DebugUI.IValueField.ValidateValue",
        args = 1
    )]
    pub fn unity_engine_rendering_debug_ui_i_value_field_validate_value(
        self,
        value: crate::system::object::Object,
    ) -> crate::system::object::Object;

    #[method(name = "ValidateValue", args = 1)]
    pub fn validate_value(self, value: T0) -> T0;

    #[method(name = "UnityEngine.Rendering.DebugUI.IValueField.GetValue", args = 0)]
    pub fn unity_engine_rendering_debug_ui_i_value_field_get_value(
        self,
    ) -> crate::system::object::Object;

    #[method(name = "GetValue", args = 0)]
    pub fn get_value(self) -> T0;

    #[method(name = "SetValue", args = 1)]
    pub fn set_value(self, value: crate::system::object::Object) -> ();

    #[method(name = "SetValue", args = 1)]
    pub fn set_value_2(self, value: T0) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-debugui")]
impl<T0: ::unity2::ClassIdentity> DebugUI_Field_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUI_Field_1),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUI_Field_1Methods<T0>>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugui/DebugUI_Vector3Field.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugUI.Vector3Field")]
# [parent (crate :: unity_engine :: rendering :: debugui :: DebugUI_Field_1 < crate :: unity_engine :: vector3 :: Vector3 >)]
pub struct DebugUI_Vector3Field {
    #[rename(name = "incStep")]
    pub inc_step: f32,
    #[rename(name = "incStepMult")]
    pub inc_step_mult: f32,
    #[rename(name = "decimals")]
    pub decimals: i32,
}

#[cfg(feature = "unity_engine-rendering-debugui")]
#[::unity2::methods]
impl DebugUI_Vector3Field {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-debugui")]
impl DebugUI_Vector3Field {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUI_Vector3Field),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUI_Vector3FieldMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugui/DebugUI_IValueField.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugUI.IValueField")]
pub struct DebugUI_IValueField {}

#[cfg(feature = "unity_engine-rendering-debugui")]
#[::unity2::methods]
impl DebugUI_IValueField {
    #[method(name = "GetValue", args = 0)]
    pub fn get_value(self) -> crate::system::object::Object;

    #[method(name = "SetValue", args = 1)]
    pub fn set_value(self, value: crate::system::object::Object) -> ();

    #[method(name = "ValidateValue", args = 1)]
    pub fn validate_value(
        self,
        value: crate::system::object::Object,
    ) -> crate::system::object::Object;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugui/DebugUI_Widget.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugUI.Widget")]
#[parent(crate::system::object::Object)]
pub struct DebugUI_Widget {
    #[rename(name = "m_Panel")]
    pub m_panel: crate::unity_engine::rendering::debugui::DebugUI_Panel,
    #[rename(name = "m_Parent")]
    pub m_parent: crate::unity_engine::rendering::debugui::DebugUI_IContainer,
}

#[cfg(feature = "unity_engine-rendering-debugui")]
#[::unity2::methods]
impl DebugUI_Widget {
    #[method(name = "get_panel", args = 0)]
    pub fn get_panel(self) -> crate::unity_engine::rendering::debugui::DebugUI_Panel;

    #[method(name = "set_panel", args = 1)]
    pub fn set_panel(self, value: crate::unity_engine::rendering::debugui::DebugUI_Panel) -> ();

    #[method(name = "get_parent", args = 0)]
    pub fn get_parent(self) -> crate::unity_engine::rendering::debugui::DebugUI_IContainer;

    #[method(name = "set_parent", args = 1)]
    pub fn set_parent(
        self,
        value: crate::unity_engine::rendering::debugui::DebugUI_IContainer,
    ) -> ();

    #[method(name = "get_flags", args = 0)]
    pub fn get_flags(self) -> crate::unity_engine::rendering::debugui::DebugUI_Flags;

    #[method(name = "set_flags", args = 1)]
    pub fn set_flags(self, value: crate::unity_engine::rendering::debugui::DebugUI_Flags) -> ();

    #[method(name = "get_displayName", args = 0)]
    pub fn get_display_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_displayName", args = 1)]
    pub fn set_display_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_queryPath", args = 0)]
    pub fn get_query_path(self) -> ::unity2::Il2CppString;

    #[method(name = "set_queryPath", args = 1)]
    pub fn set_query_path(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_isEditorOnly", args = 0)]
    pub fn get_is_editor_only(self) -> bool;

    #[method(name = "get_isRuntimeOnly", args = 0)]
    pub fn get_is_runtime_only(self) -> bool;

    #[method(name = "get_isInactiveInEditor", args = 0)]
    pub fn get_is_inactive_in_editor(self) -> bool;

    #[method(name = "GenerateQueryPath", args = 0)]
    pub fn generate_query_path(self) -> ();

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-debugui")]
impl DebugUI_Widget {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUI_Widget),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUI_WidgetMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugui/DebugUI_HistoryBoolField.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugUI.HistoryBoolField")]
#[parent(crate::unity_engine::rendering::debugui::DebugUI_BoolField)]
pub struct DebugUI_HistoryBoolField {}

#[cfg(feature = "unity_engine-rendering-debugui")]
#[::unity2::methods]
impl DebugUI_HistoryBoolField {
    #[method(name = "get_historyGetter", args = 0)]
    pub fn get_history_getter(self) -> ::unity2::Array<crate::system::func_1::Func_1<bool>>;

    #[method(name = "set_historyGetter", args = 1)]
    pub fn set_history_getter(
        self,
        value: ::unity2::Array<crate::system::func_1::Func_1<bool>>,
    ) -> ();

    #[method(name = "get_historyDepth", args = 0)]
    pub fn get_history_depth(self) -> i32;

    #[method(name = "GetHistoryValue", args = 1)]
    pub fn get_history_value(self, history_index: i32) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-debugui")]
impl DebugUI_HistoryBoolField {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUI_HistoryBoolField),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUI_HistoryBoolFieldMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugui/DebugUI_Panel.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugUI.Panel")]
#[parent(crate::system::object::Object)]
pub struct DebugUI_Panel {
    #[rename(name = "onSetDirty")]
    pub on_set_dirty:
        crate::system::action_1::Action_1<crate::unity_engine::rendering::debugui::DebugUI_Panel>,
}

#[cfg(feature = "unity_engine-rendering-debugui")]
#[::unity2::methods]
impl DebugUI_Panel {
    #[method(name = "get_flags", args = 0)]
    pub fn get_flags(self) -> crate::unity_engine::rendering::debugui::DebugUI_Flags;

    #[method(name = "set_flags", args = 1)]
    pub fn set_flags(self, value: crate::unity_engine::rendering::debugui::DebugUI_Flags) -> ();

    #[method(name = "get_displayName", args = 0)]
    pub fn get_display_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_displayName", args = 1)]
    pub fn set_display_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_groupIndex", args = 0)]
    pub fn get_group_index(self) -> i32;

    #[method(name = "set_groupIndex", args = 1)]
    pub fn set_group_index(self, value: i32) -> ();

    #[method(name = "get_queryPath", args = 0)]
    pub fn get_query_path(self) -> ::unity2::Il2CppString;

    #[method(name = "get_isEditorOnly", args = 0)]
    pub fn get_is_editor_only(self) -> bool;

    #[method(name = "get_isRuntimeOnly", args = 0)]
    pub fn get_is_runtime_only(self) -> bool;

    #[method(name = "get_isInactiveInEditor", args = 0)]
    pub fn get_is_inactive_in_editor(self) -> bool;

    #[method(name = "get_editorForceUpdate", args = 0)]
    pub fn get_editor_force_update(self) -> bool;

    #[method(name = "get_children", args = 0)]
    pub fn get_children(
        self,
    ) -> crate::unity_engine::rendering::observablelist_1::ObservableList_1<
        crate::unity_engine::rendering::debugui::DebugUI_Widget,
    >;

    #[method(name = "set_children", args = 1)]
    pub fn set_children(
        self,
        value: crate::unity_engine::rendering::observablelist_1::ObservableList_1<
            crate::unity_engine::rendering::debugui::DebugUI_Widget,
        >,
    ) -> ();

    #[method(name = "add_onSetDirty", args = 1)]
    pub fn add_on_set_dirty(
        self,
        value: crate::system::action_1::Action_1<
            crate::unity_engine::rendering::debugui::DebugUI_Panel,
        >,
    ) -> ();

    #[method(name = "remove_onSetDirty", args = 1)]
    pub fn remove_on_set_dirty(
        self,
        value: crate::system::action_1::Action_1<
            crate::unity_engine::rendering::debugui::DebugUI_Panel,
        >,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "OnItemAdded", args = 2)]
    pub fn on_item_added(
        self,
        sender: crate::unity_engine::rendering::observablelist_1::ObservableList_1<
            crate::unity_engine::rendering::debugui::DebugUI_Widget,
        >,
        e: crate::unity_engine::rendering::listchangedeventargs_1::ListChangedEventArgs_1<
            crate::unity_engine::rendering::debugui::DebugUI_Widget,
        >,
    ) -> ();

    #[method(name = "OnItemRemoved", args = 2)]
    pub fn on_item_removed(
        self,
        sender: crate::unity_engine::rendering::observablelist_1::ObservableList_1<
            crate::unity_engine::rendering::debugui::DebugUI_Widget,
        >,
        e: crate::unity_engine::rendering::listchangedeventargs_1::ListChangedEventArgs_1<
            crate::unity_engine::rendering::debugui::DebugUI_Widget,
        >,
    ) -> ();

    #[method(name = "SetDirty", args = 0)]
    pub fn set_dirty(self) -> ();

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;
}

#[cfg(feature = "unity_engine-rendering-debugui")]
impl DebugUI_Panel {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUI_Panel),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUI_PanelMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugui/DebugUI_Container.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugUI.Container")]
#[parent(crate::unity_engine::rendering::debugui::DebugUI_Widget)]
pub struct DebugUI_Container {}

#[cfg(feature = "unity_engine-rendering-debugui")]
#[::unity2::methods]
impl DebugUI_Container {
    #[method(name = "get_children", args = 0)]
    pub fn get_children(
        self,
    ) -> crate::unity_engine::rendering::observablelist_1::ObservableList_1<
        crate::unity_engine::rendering::debugui::DebugUI_Widget,
    >;

    #[method(name = "set_children", args = 1)]
    pub fn set_children(
        self,
        value: crate::unity_engine::rendering::observablelist_1::ObservableList_1<
            crate::unity_engine::rendering::debugui::DebugUI_Widget,
        >,
    ) -> ();

    #[method(name = "get_panel", args = 0)]
    pub fn get_panel(self) -> crate::unity_engine::rendering::debugui::DebugUI_Panel;

    #[method(name = "set_panel", args = 1)]
    pub fn set_panel(self, value: crate::unity_engine::rendering::debugui::DebugUI_Panel) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        display_name: ::unity2::Il2CppString,
        children: crate::unity_engine::rendering::observablelist_1::ObservableList_1<
            crate::unity_engine::rendering::debugui::DebugUI_Widget,
        >,
    ) -> ();

    #[method(name = "GenerateQueryPath", args = 0)]
    pub fn generate_query_path(self) -> ();

    #[method(name = "OnItemAdded", args = 2)]
    pub fn on_item_added(
        self,
        sender: crate::unity_engine::rendering::observablelist_1::ObservableList_1<
            crate::unity_engine::rendering::debugui::DebugUI_Widget,
        >,
        e: crate::unity_engine::rendering::listchangedeventargs_1::ListChangedEventArgs_1<
            crate::unity_engine::rendering::debugui::DebugUI_Widget,
        >,
    ) -> ();

    #[method(name = "OnItemRemoved", args = 2)]
    pub fn on_item_removed(
        self,
        sender: crate::unity_engine::rendering::observablelist_1::ObservableList_1<
            crate::unity_engine::rendering::debugui::DebugUI_Widget,
        >,
        e: crate::unity_engine::rendering::listchangedeventargs_1::ListChangedEventArgs_1<
            crate::unity_engine::rendering::debugui::DebugUI_Widget,
        >,
    ) -> ();

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;
}

#[cfg(feature = "unity_engine-rendering-debugui")]
impl DebugUI_Container {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUI_Container),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUI_ContainerMethods>::ctor(this);
        this
    }

    pub fn new_2(
        display_name: ::unity2::Il2CppString,
        children: crate::unity_engine::rendering::observablelist_1::ObservableList_1<
            crate::unity_engine::rendering::debugui::DebugUI_Widget,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUI_Container),
                ::core::stringify!(new_2),
            )
        });
        <Self as IDebugUI_ContainerMethods>::ctor_2(this, display_name, children);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugui/DebugUI_Flags.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct DebugUI_Flags {
    pub value: i32,
}

impl ::unity2::ClassIdentity for DebugUI_Flags {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "DebugUI.Flags";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DebugUI_Flags {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl DebugUI_Flags {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn editor_only() -> Self {
        Self { value: 2 }
    }

    pub fn runtime_only() -> Self {
        Self { value: 4 }
    }

    pub fn editor_force_update() -> Self {
        Self { value: 8 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugui/DebugUI_Table.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugUI.Table")]
#[parent(crate::unity_engine::rendering::debugui::DebugUI_Container)]
pub struct DebugUI_Table {
    #[rename(name = "isReadOnly")]
    pub is_read_only: bool,
    #[rename(name = "m_Header")]
    pub m_header: ::unity2::Array<bool>,
}

#[cfg(feature = "unity_engine-rendering-debugui")]
#[::unity2::methods]
impl DebugUI_Table {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "SetColumnVisibility", args = 2)]
    pub fn set_column_visibility(self, index: i32, visible: bool) -> ();

    #[method(name = "GetColumnVisibility", args = 1)]
    pub fn get_column_visibility(self, index: i32) -> bool;

    #[method(name = "get_VisibleColumns", args = 0)]
    pub fn get_visible_columns(self) -> ::unity2::Array<bool>;

    #[method(name = "OnItemAdded", args = 2)]
    pub fn on_item_added(
        self,
        sender: crate::unity_engine::rendering::observablelist_1::ObservableList_1<
            crate::unity_engine::rendering::debugui::DebugUI_Widget,
        >,
        e: crate::unity_engine::rendering::listchangedeventargs_1::ListChangedEventArgs_1<
            crate::unity_engine::rendering::debugui::DebugUI_Widget,
        >,
    ) -> ();

    #[method(name = "OnItemRemoved", args = 2)]
    pub fn on_item_removed(
        self,
        sender: crate::unity_engine::rendering::observablelist_1::ObservableList_1<
            crate::unity_engine::rendering::debugui::DebugUI_Widget,
        >,
        e: crate::unity_engine::rendering::listchangedeventargs_1::ListChangedEventArgs_1<
            crate::unity_engine::rendering::debugui::DebugUI_Widget,
        >,
    ) -> ();
}

#[cfg(feature = "unity_engine-rendering-debugui")]
impl DebugUI_Table {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUI_Table),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUI_TableMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugui/DebugUI_HBox.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugUI.HBox")]
#[parent(crate::unity_engine::rendering::debugui::DebugUI_Container)]
pub struct DebugUI_HBox {}

#[cfg(feature = "unity_engine-rendering-debugui")]
#[::unity2::methods]
impl DebugUI_HBox {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-debugui")]
impl DebugUI_HBox {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUI_HBox),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUI_HBoxMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugui/DebugUI_Value.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugUI.Value")]
#[parent(crate::unity_engine::rendering::debugui::DebugUI_Widget)]
pub struct DebugUI_Value {
    #[rename(name = "refreshRate")]
    pub refresh_rate: f32,
}

#[cfg(feature = "unity_engine-rendering-debugui")]
#[::unity2::methods]
impl DebugUI_Value {
    #[method(name = "get_getter", args = 0)]
    pub fn get_getter(self) -> crate::system::func_1::Func_1<crate::system::object::Object>;

    #[method(name = "set_getter", args = 1)]
    pub fn set_getter(
        self,
        value: crate::system::func_1::Func_1<crate::system::object::Object>,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetValue", args = 0)]
    pub fn get_value(self) -> crate::system::object::Object;
}

#[cfg(feature = "unity_engine-rendering-debugui")]
impl DebugUI_Value {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUI_Value),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUI_ValueMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugui/DebugUI_Vector2Field.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugUI.Vector2Field")]
# [parent (crate :: unity_engine :: rendering :: debugui :: DebugUI_Field_1 < crate :: unity_engine :: vector2 :: Vector2 >)]
pub struct DebugUI_Vector2Field {
    #[rename(name = "incStep")]
    pub inc_step: f32,
    #[rename(name = "incStepMult")]
    pub inc_step_mult: f32,
    #[rename(name = "decimals")]
    pub decimals: i32,
}

#[cfg(feature = "unity_engine-rendering-debugui")]
#[::unity2::methods]
impl DebugUI_Vector2Field {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-debugui")]
impl DebugUI_Vector2Field {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUI_Vector2Field),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUI_Vector2FieldMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugui/DebugUI_BitField.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugUI.BitField")]
# [parent (crate :: unity_engine :: rendering :: debugui :: DebugUI_Field_1 < crate :: system :: r#enum :: Enum >)]
pub struct DebugUI_BitField {
    #[rename(name = "m_EnumType")]
    pub m_enum_type: ::unity2::SystemType,
}

#[cfg(feature = "unity_engine-rendering-debugui")]
#[::unity2::methods]
impl DebugUI_BitField {
    #[method(name = "get_enumNames", args = 0)]
    pub fn get_enum_names(self) -> ::unity2::Array<crate::unity_engine::guicontent::GUIContent>;

    #[method(name = "set_enumNames", args = 1)]
    pub fn set_enum_names(
        self,
        value: ::unity2::Array<crate::unity_engine::guicontent::GUIContent>,
    ) -> ();

    #[method(name = "get_enumValues", args = 0)]
    pub fn get_enum_values(self) -> ::unity2::Array<i32>;

    #[method(name = "set_enumValues", args = 1)]
    pub fn set_enum_values(self, value: ::unity2::Array<i32>) -> ();

    #[method(name = "set_enumType", args = 1)]
    pub fn set_enum_type(self, value: ::unity2::SystemType) -> ();

    #[method(name = "get_enumType", args = 0)]
    pub fn get_enum_type(self) -> ::unity2::SystemType;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-debugui")]
impl DebugUI_BitField {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUI_BitField),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUI_BitFieldMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugui/DebugUI_HistoryEnumField.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugUI.HistoryEnumField")]
#[parent(crate::unity_engine::rendering::debugui::DebugUI_EnumField)]
pub struct DebugUI_HistoryEnumField {}

#[cfg(feature = "unity_engine-rendering-debugui")]
#[::unity2::methods]
impl DebugUI_HistoryEnumField {
    #[method(name = "get_historyIndexGetter", args = 0)]
    pub fn get_history_index_getter(self) -> ::unity2::Array<crate::system::func_1::Func_1<i32>>;

    #[method(name = "set_historyIndexGetter", args = 1)]
    pub fn set_history_index_getter(
        self,
        value: ::unity2::Array<crate::system::func_1::Func_1<i32>>,
    ) -> ();

    #[method(name = "get_historyDepth", args = 0)]
    pub fn get_history_depth(self) -> i32;

    #[method(name = "GetHistoryValue", args = 1)]
    pub fn get_history_value(self, history_index: i32) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-debugui")]
impl DebugUI_HistoryEnumField {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUI_HistoryEnumField),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUI_HistoryEnumFieldMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugui/DebugUI_Button.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugUI.Button")]
#[parent(crate::unity_engine::rendering::debugui::DebugUI_Widget)]
pub struct DebugUI_Button {}

#[cfg(feature = "unity_engine-rendering-debugui")]
#[::unity2::methods]
impl DebugUI_Button {
    #[method(name = "get_action", args = 0)]
    pub fn get_action(self) -> crate::system::action::Action;

    #[method(name = "set_action", args = 1)]
    pub fn set_action(self, value: crate::system::action::Action) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-debugui")]
impl DebugUI_Button {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUI_Button),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUI_ButtonMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugui/DebugUI.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugUI")]
#[parent(crate::system::object::Object)]
pub struct DebugUI {}

#[cfg(feature = "unity_engine-rendering-debugui")]
#[::unity2::methods]
impl DebugUI {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-debugui")]
impl DebugUI {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUI),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUIMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugui/DebugUI_IntField.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugUI.IntField")]
# [parent (crate :: unity_engine :: rendering :: debugui :: DebugUI_Field_1 < i32 >)]
pub struct DebugUI_IntField {
    #[rename(name = "min")]
    pub min: crate::system::func_1::Func_1<i32>,
    #[rename(name = "max")]
    pub max: crate::system::func_1::Func_1<i32>,
    #[rename(name = "incStep")]
    pub inc_step: i32,
    #[rename(name = "intStepMult")]
    pub int_step_mult: i32,
}

#[cfg(feature = "unity_engine-rendering-debugui")]
#[::unity2::methods]
impl DebugUI_IntField {
    #[method(name = "ValidateValue", args = 1)]
    pub fn validate_value(self, value: i32) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-debugui")]
impl DebugUI_IntField {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUI_IntField),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUI_IntFieldMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugui/DebugUI_IContainer.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugUI.IContainer")]
pub struct DebugUI_IContainer {}

#[cfg(feature = "unity_engine-rendering-debugui")]
#[::unity2::methods]
impl DebugUI_IContainer {
    #[method(name = "get_children", args = 0)]
    pub fn get_children(
        self,
    ) -> crate::unity_engine::rendering::observablelist_1::ObservableList_1<
        crate::unity_engine::rendering::debugui::DebugUI_Widget,
    >;

    #[method(name = "get_displayName", args = 0)]
    pub fn get_display_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_displayName", args = 1)]
    pub fn set_display_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_queryPath", args = 0)]
    pub fn get_query_path(self) -> ::unity2::Il2CppString;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugui/DebugUI_ColorField.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugUI.ColorField")]
# [parent (crate :: unity_engine :: rendering :: debugui :: DebugUI_Field_1 < crate :: unity_engine :: color :: Color >)]
pub struct DebugUI_ColorField {
    #[rename(name = "hdr")]
    pub hdr: bool,
    #[rename(name = "showAlpha")]
    pub show_alpha: bool,
    #[rename(name = "showPicker")]
    pub show_picker: bool,
    #[rename(name = "incStep")]
    pub inc_step: f32,
    #[rename(name = "incStepMult")]
    pub inc_step_mult: f32,
    #[rename(name = "decimals")]
    pub decimals: i32,
}

#[cfg(feature = "unity_engine-rendering-debugui")]
#[::unity2::methods]
impl DebugUI_ColorField {
    #[method(name = "ValidateValue", args = 1)]
    pub fn validate_value(
        self,
        value: crate::unity_engine::color::Color,
    ) -> crate::unity_engine::color::Color;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-debugui")]
impl DebugUI_ColorField {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUI_ColorField),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUI_ColorFieldMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugui/DebugUI_VBox.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugUI.VBox")]
#[parent(crate::unity_engine::rendering::debugui::DebugUI_Container)]
pub struct DebugUI_VBox {}

#[cfg(feature = "unity_engine-rendering-debugui")]
#[::unity2::methods]
impl DebugUI_VBox {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-debugui")]
impl DebugUI_VBox {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUI_VBox),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUI_VBoxMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugui/DebugUI_EnumField.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugUI.EnumField")]
# [parent (crate :: unity_engine :: rendering :: debugui :: DebugUI_Field_1 < i32 >)]
pub struct DebugUI_EnumField {
    #[rename(name = "enumNames")]
    pub enum_names: ::unity2::Array<crate::unity_engine::guicontent::GUIContent>,
    #[rename(name = "enumValues")]
    pub enum_values: ::unity2::Array<i32>,
    #[rename(name = "quickSeparators")]
    pub quick_separators: ::unity2::Array<i32>,
    #[rename(name = "indexes")]
    pub indexes: ::unity2::Array<i32>,
}

#[cfg(feature = "unity_engine-rendering-debugui")]
#[::unity2::methods]
impl DebugUI_EnumField {
    #[method(name = "get_getIndex", args = 0)]
    pub fn get_get_index(self) -> crate::system::func_1::Func_1<i32>;

    #[method(name = "set_getIndex", args = 1)]
    pub fn set_get_index(self, value: crate::system::func_1::Func_1<i32>) -> ();

    #[method(name = "get_setIndex", args = 0)]
    pub fn get_set_index(self) -> crate::system::action_1::Action_1<i32>;

    #[method(name = "set_setIndex", args = 1)]
    pub fn set_set_index(self, value: crate::system::action_1::Action_1<i32>) -> ();

    #[method(name = "get_currentIndex", args = 0)]
    pub fn get_current_index(self) -> i32;

    #[method(name = "set_currentIndex", args = 1)]
    pub fn set_current_index(self, value: i32) -> ();

    #[method(name = "set_autoEnum", args = 1)]
    pub fn set_auto_enum(self, value: ::unity2::SystemType) -> ();

    #[method(name = "InitQuickSeparators", args = 0)]
    pub fn init_quick_separators(self) -> ();

    #[method(name = "InitIndexes", args = 0)]
    pub fn init_indexes(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-debugui")]
impl DebugUI_EnumField {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUI_EnumField),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUI_EnumFieldMethods>::ctor(this);
        this
    }
}
