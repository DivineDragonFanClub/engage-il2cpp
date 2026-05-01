
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::event_systems::uibehaviour::IUIBehaviour;
use crate::unity_engine::event_systems::uibehaviour::UIBehaviour;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::ui::layoutgroup::ILayoutGroup;
use crate::unity_engine::ui::layoutgroup::LayoutGroup;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/gridlayoutgroup/GridLayoutGroup_Axis.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GridLayoutGroup_Axis {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GridLayoutGroup_Axis {
    const NAMESPACE: &'static str = "UnityEngine.UI";

    const NAME: &'static str = "GridLayoutGroup.Axis";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GridLayoutGroup_Axis {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GridLayoutGroup_Axis {
    pub fn horizontal() -> Self {
        Self { value: 0 }
    }

    pub fn vertical() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/gridlayoutgroup/GridLayoutGroup_Corner.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GridLayoutGroup_Corner {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GridLayoutGroup_Corner {
    const NAMESPACE: &'static str = "UnityEngine.UI";

    const NAME: &'static str = "GridLayoutGroup.Corner";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GridLayoutGroup_Corner {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GridLayoutGroup_Corner {
    pub fn upper_left() -> Self {
        Self { value: 0 }
    }

    pub fn upper_right() -> Self {
        Self { value: 1 }
    }

    pub fn lower_left() -> Self {
        Self { value: 2 }
    }

    pub fn lower_right() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/gridlayoutgroup/GridLayoutGroup.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "GridLayoutGroup")]
#[parent(crate::unity_engine::ui::layoutgroup::LayoutGroup)]
pub struct GridLayoutGroup {
    #[rename(name = "m_StartCorner")]
    pub m_start_corner: crate::unity_engine::ui::gridlayoutgroup::GridLayoutGroup_Corner,
    #[rename(name = "m_StartAxis")]
    pub m_start_axis: crate::unity_engine::ui::gridlayoutgroup::GridLayoutGroup_Axis,
    #[rename(name = "m_CellSize")]
    pub m_cell_size: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_Spacing")]
    pub m_spacing: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_Constraint")]
    pub m_constraint: crate::unity_engine::ui::gridlayoutgroup::GridLayoutGroup_Constraint,
    #[rename(name = "m_ConstraintCount")]
    pub m_constraint_count: i32,
}

#[cfg(feature = "unity_engine-ui-gridlayoutgroup")]
#[::unity2::methods]
impl GridLayoutGroup {
    #[method(name = "get_startCorner", args = 0)]
    pub fn get_start_corner(
        self,
    ) -> crate::unity_engine::ui::gridlayoutgroup::GridLayoutGroup_Corner;

    #[method(name = "set_startCorner", args = 1)]
    pub fn set_start_corner(
        self,
        value: crate::unity_engine::ui::gridlayoutgroup::GridLayoutGroup_Corner,
    ) -> ();

    #[method(name = "get_startAxis", args = 0)]
    pub fn get_start_axis(self) -> crate::unity_engine::ui::gridlayoutgroup::GridLayoutGroup_Axis;

    #[method(name = "set_startAxis", args = 1)]
    pub fn set_start_axis(
        self,
        value: crate::unity_engine::ui::gridlayoutgroup::GridLayoutGroup_Axis,
    ) -> ();

    #[method(name = "get_cellSize", args = 0)]
    pub fn get_cell_size(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "set_cellSize", args = 1)]
    pub fn set_cell_size(self, value: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "get_spacing", args = 0)]
    pub fn get_spacing(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "set_spacing", args = 1)]
    pub fn set_spacing(self, value: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "get_constraint", args = 0)]
    pub fn get_constraint(
        self,
    ) -> crate::unity_engine::ui::gridlayoutgroup::GridLayoutGroup_Constraint;

    #[method(name = "set_constraint", args = 1)]
    pub fn set_constraint(
        self,
        value: crate::unity_engine::ui::gridlayoutgroup::GridLayoutGroup_Constraint,
    ) -> ();

    #[method(name = "get_constraintCount", args = 0)]
    pub fn get_constraint_count(self) -> i32;

    #[method(name = "set_constraintCount", args = 1)]
    pub fn set_constraint_count(self, value: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "CalculateLayoutInputHorizontal", args = 0)]
    pub fn calculate_layout_input_horizontal(self) -> ();

    #[method(name = "CalculateLayoutInputVertical", args = 0)]
    pub fn calculate_layout_input_vertical(self) -> ();

    #[method(name = "SetLayoutHorizontal", args = 0)]
    pub fn set_layout_horizontal(self) -> ();

    #[method(name = "SetLayoutVertical", args = 0)]
    pub fn set_layout_vertical(self) -> ();

    #[method(name = "SetCellsAlongAxis", args = 1)]
    pub fn set_cells_along_axis(self, axis: i32) -> ();
}

#[cfg(feature = "unity_engine-ui-gridlayoutgroup")]
impl GridLayoutGroup {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GridLayoutGroup),
                ::core::stringify!(new),
            )
        });
        <Self as IGridLayoutGroupMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/gridlayoutgroup/GridLayoutGroup_Constraint.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GridLayoutGroup_Constraint {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GridLayoutGroup_Constraint {
    const NAMESPACE: &'static str = "UnityEngine.UI";

    const NAME: &'static str = "GridLayoutGroup.Constraint";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GridLayoutGroup_Constraint {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GridLayoutGroup_Constraint {
    pub fn flexible() -> Self {
        Self { value: 0 }
    }

    pub fn fixed_column_count() -> Self {
        Self { value: 1 }
    }

    pub fn fixed_row_count() -> Self {
        Self { value: 2 }
    }
}
