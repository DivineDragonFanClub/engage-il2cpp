
use crate::app::mapinfobase::IMapInfoBase;
use crate::app::mapinfobase::MapInfoBase;
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
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapinfounitlocatorroot/MapInfoUnitLocatorRoot_CanvasGroupKind.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapInfoUnitLocatorRoot_CanvasGroupKind {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapInfoUnitLocatorRoot_CanvasGroupKind {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapInfoUnitLocatorRoot.CanvasGroupKind";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapInfoUnitLocatorRoot_CanvasGroupKind {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapInfoUnitLocatorRoot_CanvasGroupKind {
    pub fn frm1() -> Self {
        Self { value: 0 }
    }

    pub fn frm2() -> Self {
        Self { value: 1 }
    }

    pub fn info0() -> Self {
        Self { value: 2 }
    }

    pub fn info1() -> Self {
        Self { value: 3 }
    }

    pub fn info2() -> Self {
        Self { value: 4 }
    }

    pub fn info3() -> Self {
        Self { value: 5 }
    }

    pub fn info4() -> Self {
        Self { value: 6 }
    }

    pub fn info5() -> Self {
        Self { value: 7 }
    }

    pub fn info6() -> Self {
        Self { value: 8 }
    }

    pub fn num() -> Self {
        Self { value: 9 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapinfounitlocatorroot/MapInfoUnitLocatorRoot_TextMeshProKind.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapInfoUnitLocatorRoot_TextMeshProKind {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapInfoUnitLocatorRoot_TextMeshProKind {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapInfoUnitLocatorRoot.TextMeshProKind";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapInfoUnitLocatorRoot_TextMeshProKind {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapInfoUnitLocatorRoot_TextMeshProKind {
    pub fn battle_type() -> Self {
        Self { value: 0 }
    }

    pub fn phys_atk_title() -> Self {
        Self { value: 1 }
    }

    pub fn phys_atk_value() -> Self {
        Self { value: 2 }
    }

    pub fn def_title() -> Self {
        Self { value: 3 }
    }

    pub fn def_value() -> Self {
        Self { value: 4 }
    }

    pub fn mag_atk_title() -> Self {
        Self { value: 5 }
    }

    pub fn mag_atk_value() -> Self {
        Self { value: 6 }
    }

    pub fn res_title() -> Self {
        Self { value: 7 }
    }

    pub fn res_value() -> Self {
        Self { value: 8 }
    }

    pub fn hit_title() -> Self {
        Self { value: 9 }
    }

    pub fn hit_value() -> Self {
        Self { value: 10 }
    }

    pub fn avo_title() -> Self {
        Self { value: 11 }
    }

    pub fn avo_value() -> Self {
        Self { value: 12 }
    }

    pub fn crit_title() -> Self {
        Self { value: 13 }
    }

    pub fn crit_value() -> Self {
        Self { value: 14 }
    }

    pub fn crit_avo_title() -> Self {
        Self { value: 15 }
    }

    pub fn crit_avo_value() -> Self {
        Self { value: 16 }
    }

    pub fn skill_title() -> Self {
        Self { value: 17 }
    }

    pub fn skill_value() -> Self {
        Self { value: 18 }
    }

    pub fn spd_title() -> Self {
        Self { value: 19 }
    }

    pub fn spd_value() -> Self {
        Self { value: 20 }
    }

    pub fn move_title() -> Self {
        Self { value: 21 }
    }

    pub fn move_value() -> Self {
        Self { value: 22 }
    }

    pub fn phys_title() -> Self {
        Self { value: 23 }
    }

    pub fn phys_value() -> Self {
        Self { value: 24 }
    }

    pub fn num() -> Self {
        Self { value: 25 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapinfounitlocatorroot/MapInfoUnitLocatorRoot_ImageKind.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapInfoUnitLocatorRoot_ImageKind {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapInfoUnitLocatorRoot_ImageKind {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapInfoUnitLocatorRoot.ImageKind";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapInfoUnitLocatorRoot_ImageKind {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapInfoUnitLocatorRoot_ImageKind {
    pub fn battle_type() -> Self {
        Self { value: 0 }
    }

    pub fn phys_atk_title() -> Self {
        Self { value: 1 }
    }

    pub fn phys_atk_value() -> Self {
        Self { value: 2 }
    }

    pub fn phys_atk_value1() -> Self {
        Self { value: 2 }
    }

    pub fn phys_atk_value2() -> Self {
        Self { value: 3 }
    }

    pub fn phys_atk_value3() -> Self {
        Self { value: 4 }
    }

    pub fn phys_atk_value_empty() -> Self {
        Self { value: 5 }
    }

    pub fn def_title() -> Self {
        Self { value: 6 }
    }

    pub fn def_value() -> Self {
        Self { value: 7 }
    }

    pub fn def_value1() -> Self {
        Self { value: 7 }
    }

    pub fn def_value2() -> Self {
        Self { value: 8 }
    }

    pub fn def_value3() -> Self {
        Self { value: 9 }
    }

    pub fn def_value_empty() -> Self {
        Self { value: 10 }
    }

    pub fn mag_atk_title() -> Self {
        Self { value: 11 }
    }

    pub fn mag_atk_value() -> Self {
        Self { value: 12 }
    }

    pub fn mag_atk_value1() -> Self {
        Self { value: 12 }
    }

    pub fn mag_atk_value2() -> Self {
        Self { value: 13 }
    }

    pub fn mag_atk_value3() -> Self {
        Self { value: 14 }
    }

    pub fn mag_atk_value_empty() -> Self {
        Self { value: 15 }
    }

    pub fn res_title() -> Self {
        Self { value: 16 }
    }

    pub fn res_value() -> Self {
        Self { value: 17 }
    }

    pub fn res_value1() -> Self {
        Self { value: 17 }
    }

    pub fn res_value2() -> Self {
        Self { value: 18 }
    }

    pub fn res_value3() -> Self {
        Self { value: 19 }
    }

    pub fn res_value_empty() -> Self {
        Self { value: 20 }
    }

    pub fn hit_title() -> Self {
        Self { value: 21 }
    }

    pub fn hit_value() -> Self {
        Self { value: 22 }
    }

    pub fn hit_value1() -> Self {
        Self { value: 22 }
    }

    pub fn hit_value2() -> Self {
        Self { value: 23 }
    }

    pub fn hit_value3() -> Self {
        Self { value: 24 }
    }

    pub fn hit_value_empty() -> Self {
        Self { value: 25 }
    }

    pub fn hit_value_infinity() -> Self {
        Self { value: 26 }
    }

    pub fn avo_title() -> Self {
        Self { value: 27 }
    }

    pub fn avo_value() -> Self {
        Self { value: 28 }
    }

    pub fn avo_value1() -> Self {
        Self { value: 28 }
    }

    pub fn avo_value2() -> Self {
        Self { value: 29 }
    }

    pub fn avo_value3() -> Self {
        Self { value: 30 }
    }

    pub fn avo_value_empty() -> Self {
        Self { value: 31 }
    }

    pub fn crit_title() -> Self {
        Self { value: 32 }
    }

    pub fn crit_value() -> Self {
        Self { value: 33 }
    }

    pub fn crit_value1() -> Self {
        Self { value: 33 }
    }

    pub fn crit_value2() -> Self {
        Self { value: 34 }
    }

    pub fn crit_value3() -> Self {
        Self { value: 35 }
    }

    pub fn crit_value_empty() -> Self {
        Self { value: 36 }
    }

    pub fn crit_avo_title() -> Self {
        Self { value: 37 }
    }

    pub fn crit_avo_value() -> Self {
        Self { value: 38 }
    }

    pub fn crit_avo_value1() -> Self {
        Self { value: 38 }
    }

    pub fn crit_avo_value2() -> Self {
        Self { value: 39 }
    }

    pub fn crit_avo_value3() -> Self {
        Self { value: 40 }
    }

    pub fn crit_avo_value_empty() -> Self {
        Self { value: 41 }
    }

    pub fn skill_title() -> Self {
        Self { value: 42 }
    }

    pub fn skill_value() -> Self {
        Self { value: 43 }
    }

    pub fn skill_value1() -> Self {
        Self { value: 43 }
    }

    pub fn skill_value2() -> Self {
        Self { value: 44 }
    }

    pub fn skill_value3() -> Self {
        Self { value: 45 }
    }

    pub fn skill_value_empty() -> Self {
        Self { value: 46 }
    }

    pub fn spd_title() -> Self {
        Self { value: 47 }
    }

    pub fn spd_value() -> Self {
        Self { value: 48 }
    }

    pub fn spd_value1() -> Self {
        Self { value: 48 }
    }

    pub fn spd_value2() -> Self {
        Self { value: 49 }
    }

    pub fn spd_value3() -> Self {
        Self { value: 50 }
    }

    pub fn spd_value_empty() -> Self {
        Self { value: 51 }
    }

    pub fn move_title() -> Self {
        Self { value: 52 }
    }

    pub fn move_value() -> Self {
        Self { value: 53 }
    }

    pub fn move_value1() -> Self {
        Self { value: 53 }
    }

    pub fn move_value2() -> Self {
        Self { value: 54 }
    }

    pub fn move_value3() -> Self {
        Self { value: 55 }
    }

    pub fn move_value_empty() -> Self {
        Self { value: 56 }
    }

    pub fn phys_title() -> Self {
        Self { value: 57 }
    }

    pub fn phys_value() -> Self {
        Self { value: 58 }
    }

    pub fn phys_value1() -> Self {
        Self { value: 58 }
    }

    pub fn phys_value2() -> Self {
        Self { value: 59 }
    }

    pub fn phys_value3() -> Self {
        Self { value: 60 }
    }

    pub fn phys_value_empty() -> Self {
        Self { value: 61 }
    }

    pub fn num() -> Self {
        Self { value: 62 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapinfounitlocatorroot/MapInfoUnitLocatorRoot.md")))]
#[::unity2::class(namespace = "App", name = "MapInfoUnitLocatorRoot")]
#[parent(crate::app::mapinfobase::MapInfoBase)]
pub struct MapInfoUnitLocatorRoot {
    #[rename(name = "m_BattleType")]
    pub m_battle_type: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_PhysAtkTitle")]
    pub m_phys_atk_title: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_PhysAtkValue1")]
    pub m_phys_atk_value1: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_PhysAtkValue2")]
    pub m_phys_atk_value2: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_PhysAtkValue3")]
    pub m_phys_atk_value3: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_PhysAtkValueEmpty")]
    pub m_phys_atk_value_empty: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_DefTitle")]
    pub m_def_title: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_DefValue1")]
    pub m_def_value1: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_DefValue2")]
    pub m_def_value2: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_DefValue3")]
    pub m_def_value3: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_DefValueEmpty")]
    pub m_def_value_empty: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_MagAtkTitle")]
    pub m_mag_atk_title: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_MagAtkValue1")]
    pub m_mag_atk_value1: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_MagAtkValue2")]
    pub m_mag_atk_value2: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_MagAtkValue3")]
    pub m_mag_atk_value3: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_MagAtkValueEmpty")]
    pub m_mag_atk_value_empty: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_ResTitle")]
    pub m_res_title: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_ResValue1")]
    pub m_res_value1: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_ResValue2")]
    pub m_res_value2: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_ResValue3")]
    pub m_res_value3: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_ResValueEmpty")]
    pub m_res_value_empty: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_HitTitle")]
    pub m_hit_title: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_HitValue1")]
    pub m_hit_value1: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_HitValue2")]
    pub m_hit_value2: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_HitValue3")]
    pub m_hit_value3: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_HitValueEmpty")]
    pub m_hit_value_empty: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_AvoTitle")]
    pub m_avo_title: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_AvoValue1")]
    pub m_avo_value1: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_AvoValue2")]
    pub m_avo_value2: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_AvoValue3")]
    pub m_avo_value3: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_AvoValueEmpty")]
    pub m_avo_value_empty: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_CritTitle")]
    pub m_crit_title: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_CritValue1")]
    pub m_crit_value1: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_CritValue2")]
    pub m_crit_value2: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_CritValue3")]
    pub m_crit_value3: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_CritValueEmpty")]
    pub m_crit_value_empty: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_CritAvoTitle")]
    pub m_crit_avo_title: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_CritAvoValue1")]
    pub m_crit_avo_value1: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_CritAvoValue2")]
    pub m_crit_avo_value2: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_CritAvoValue3")]
    pub m_crit_avo_value3: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_CritAvoValueEmpty")]
    pub m_crit_avo_value_empty: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_SkillTitle")]
    pub m_skill_title: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_SkillValue1")]
    pub m_skill_value1: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_SkillValue2")]
    pub m_skill_value2: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_SkillValue3")]
    pub m_skill_value3: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_SkillValueEmpty")]
    pub m_skill_value_empty: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_SpdTitle")]
    pub m_spd_title: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_SpdValue1")]
    pub m_spd_value1: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_SpdValue2")]
    pub m_spd_value2: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_SpdValue3")]
    pub m_spd_value3: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_SpdValueEmpty")]
    pub m_spd_value_empty: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_MoveTitle")]
    pub m_move_title: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_MoveValue1")]
    pub m_move_value1: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_MoveValue2")]
    pub m_move_value2: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_MoveValue3")]
    pub m_move_value3: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_MoveValueEmpty")]
    pub m_move_value_empty: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_PhysTitle")]
    pub m_phys_title: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_PhysValue1")]
    pub m_phys_value1: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_PhysValue2")]
    pub m_phys_value2: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_PhysValue3")]
    pub m_phys_value3: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_PhysValueEmpty")]
    pub m_phys_value_empty: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_LocatorRoot")]
    pub m_locator_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_TotalInfo")]
    pub m_total_info: crate::unity_engine::canvasgroup::CanvasGroup,
    #[rename(name = "m_PartInfo")]
    pub m_part_info: ::unity2::Array<crate::unity_engine::canvasgroup::CanvasGroup>,
    #[rename(name = "m_Images")]
    pub m_images: ::unity2::Array<crate::unity_engine::ui::image::Image>,
    #[rename(name = "m_PageAlpha")]
    pub m_page_alpha: ::unity2::Array<f32>,
    #[rename(name = "m_IsUpdate")]
    pub m_is_update: bool,
    #[rename(name = "m_IsDirtyAlpha")]
    pub m_is_dirty_alpha: bool,
}

#[cfg(feature = "app-mapinfounitlocatorroot")]
#[::unity2::methods]
impl MapInfoUnitLocatorRoot {
    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "GetPartInfo", args = 0)]
    pub fn get_part_info(self) -> ();

    #[method(name = "InitPartInfoAlpha", args = 0)]
    pub fn init_part_info_alpha(self) -> ();

    #[method(name = "GetImage", args = 0)]
    pub fn get_image(self) -> ();

    #[method(name = "InitPageAlpha", args = 0)]
    pub fn init_page_alpha(self) -> ();

    #[method(name = "UpdatePosition", args = 1)]
    pub fn update_position(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UpdateParam", args = 1)]
    pub fn update_param(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UpdateActive", args = 0)]
    pub fn update_active(self) -> ();

    #[method(name = "IsVisible", args = 1)]
    pub fn is_visible(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "SetAlphaTotalInfo", args = 1)]
    pub fn set_alpha_total_info(self, enable: bool) -> ();

    #[method(name = "SetAlphaPartInfo", args = 0)]
    pub fn set_alpha_part_info(self) -> ();

    #[method(name = "SetAlphaInfo", args = 3)]
    pub fn set_alpha_info(
        self,
        page: crate::app::gameconfig::GameConfig_UnitInfoType,
        index: crate::app::mapinfounitlocatorroot::MapInfoUnitLocatorRoot_CanvasGroupKind,
        enabled: bool,
    ) -> ();

    #[method(name = "IsUpdateAlpha", args = 2)]
    pub fn is_update_alpha(alpha: f32, enable: bool) -> bool;

    #[method(name = "SetUnitInfo", args = 1)]
    pub fn set_unit_info(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "SetUnitInfoValue", args = 4)]
    pub fn set_unit_info_value(
        self,
        value: i32,
        kind: crate::app::mapinfounitlocatorroot::MapInfoUnitLocatorRoot_ImageKind,
        is_withhold: bool,
        is_sure_hit: bool,
    ) -> ();

    #[method(name = "SetUnitInfo0", args = 1)]
    pub fn set_unit_info0(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "SetUnitInfo1", args = 1)]
    pub fn set_unit_info1(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "SetUnitInfo2", args = 1)]
    pub fn set_unit_info2(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "SetUnitInfo3", args = 1)]
    pub fn set_unit_info3(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "SetUnitInfo4", args = 1)]
    pub fn set_unit_info4(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "SetUnitInfo5", args = 1)]
    pub fn set_unit_info5(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "SetUnitInfo6", args = 1)]
    pub fn set_unit_info6(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "GetLanguageLabel", args = 1)]
    pub fn get_language_label(self, label: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleName", args = 2)]
    pub fn get_title_name(
        self,
        unit: crate::app::unit::Unit,
        kind: crate::app::mapinfounitlocatorroot::MapInfoUnitLocatorRoot_TextMeshProKind,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetTitleSprite", args = 2)]
    pub fn get_title_sprite(
        self,
        unit: crate::app::unit::Unit,
        kind: crate::app::mapinfounit::MapInfoUnit_SpriteKind,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "SetAttackValue", args = 2)]
    pub fn set_attack_value(
        self,
        unit: crate::app::unit::Unit,
        kind: crate::app::mapinfounitlocatorroot::MapInfoUnitLocatorRoot_ImageKind,
    ) -> ();

    #[method(name = "set_IsUpdate", args = 1)]
    pub fn set_is_update(self, value: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapinfounitlocatorroot")]
impl MapInfoUnitLocatorRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapInfoUnitLocatorRoot),
                ::core::stringify!(new),
            )
        });
        <Self as IMapInfoUnitLocatorRootMethods>::ctor(this);
        this
    }
}
