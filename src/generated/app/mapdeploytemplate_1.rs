
use crate::app::bitfield64::BitField64;
use crate::app::bitfield64::IBitField64;
use crate::app::bitfieldcommon::BitFieldCommon;
use crate::app::bitfieldcommon::IBitFieldCommon;
use crate::app::bitfieldtemplate64_1::BitFieldTemplate64_1;
use crate::app::bitfieldtemplate64_1::IBitFieldTemplate64_1;
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapdeploytemplate_1/MapDeployTemplate_1_FlagField.md")))]
#[::unity2::class(namespace = "App", name = "MapDeployTemplate`1.FlagField")]
pub struct MapDeployTemplate_1_FlagField<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "app-mapdeploytemplate_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> MapDeployTemplate_1_FlagField<T0> {
    #[method(name = "ToLong", args = 1)]
    pub fn to_long(
        self,
        value: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
    ) -> i64;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapdeploytemplate_1")]
impl<T0: ::unity2::ClassIdentity> MapDeployTemplate_1_FlagField<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapDeployTemplate_1_FlagField),
                ::core::stringify!(new),
            )
        });
        <Self as IMapDeployTemplate_1_FlagFieldMethods<T0>>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapdeploytemplate_1/MapDeployTemplate_1_Queue.md")))]
#[::unity2::class(namespace = "App", name = "MapDeployTemplate`1.Queue")]
pub struct MapDeployTemplate_1_Queue<T0: ::unity2::ClassIdentity> {
    #[static_field]
    #[rename(name = "QueueSize")]
    pub queue_size: i32,
    #[rename(name = "m_Datas")]
    pub m_datas:
        ::unity2::Array<crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Queue_Data<T0>>,
    #[rename(name = "m_Index")]
    pub m_index: i32,
}

#[cfg(feature = "app-mapdeploytemplate_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> MapDeployTemplate_1_Queue<T0> {
    #[method(name = "Set", args = 4)]
    pub fn set(self, x: i32, z: i32, dir: crate::app::dir_2::Dir_Type, cost: i32) -> ();

    #[method(name = "Next", args = 0)]
    pub fn next(self) -> ();

    #[method(name = "ResetIndex", args = 0)]
    pub fn reset_index(self) -> ();

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self)
        -> crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Queue_Data<T0>;

    #[method(name = "set_Current", args = 1)]
    pub fn set_current(
        self,
        value: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Queue_Data<T0>,
    ) -> ();

    #[method(name = "get_CurrentX", args = 0)]
    pub fn get_current_x(self) -> i32;

    #[method(name = "set_CurrentX", args = 1)]
    pub fn set_current_x(self, value: i32) -> ();

    #[method(name = "get_CurrentZ", args = 0)]
    pub fn get_current_z(self) -> i32;

    #[method(name = "set_CurrentZ", args = 1)]
    pub fn set_current_z(self, value: i32) -> ();

    #[method(name = "get_CurrentDir", args = 0)]
    pub fn get_current_dir(self) -> crate::app::dir_2::Dir_Type;

    #[method(name = "set_CurrentDir", args = 1)]
    pub fn set_current_dir(self, value: crate::app::dir_2::Dir_Type) -> ();

    #[method(name = "get_CurrentCost", args = 0)]
    pub fn get_current_cost(self) -> i32;

    #[method(name = "set_CurrentCost", args = 1)]
    pub fn set_current_cost(self, value: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapdeploytemplate_1")]
impl<T0: ::unity2::ClassIdentity> MapDeployTemplate_1_Queue<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapDeployTemplate_1_Queue),
                ::core::stringify!(new),
            )
        });
        <Self as IMapDeployTemplate_1_QueueMethods<T0>>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapdeploytemplate_1/MapDeployTemplate_1_Queue_Data.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapDeployTemplate_1_Queue_Data<T0> {
    pub _phantom: ::core::marker::PhantomData<(T0,)>,
}

impl<T0: ::unity2::ClassIdentity> ::unity2::ClassIdentity for MapDeployTemplate_1_Queue_Data<T0> {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapDeployTemplate`1.Queue.Data";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| {
            ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME)
                .make_generic(&[<T0 as ::unity2::ClassIdentity>::class()])
                .expect("generic instantiation")
        })
    }
}

impl<T0: ::unity2::ClassIdentity> ::unity2::IlType for MapDeployTemplate_1_Queue_Data<T0> {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapdeploytemplate_1/MapDeployTemplate_1_DisplayType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapDeployTemplate_1_DisplayType<T0: ::unity2::ClassIdentity> {
    pub value: i32,
    pub _phantom: ::core::marker::PhantomData<(T0)>,
}

impl<T0: ::unity2::ClassIdentity> ::unity2::ClassIdentity for MapDeployTemplate_1_DisplayType<T0> {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapDeployTemplate`1.DisplayType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl<T0: ::unity2::ClassIdentity> ::unity2::IlType for MapDeployTemplate_1_DisplayType<T0> {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl<T0: ::unity2::ClassIdentity> MapDeployTemplate_1_DisplayType<T0> {
    pub fn none() -> Self {
        Self {
            value: 0,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn always() -> Self {
        Self {
            value: 1,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn order() -> Self {
        Self {
            value: 2,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn num() -> Self {
        Self {
            value: 3,
            _phantom: ::core::marker::PhantomData,
        }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapdeploytemplate_1/MapDeployTemplate_1_Flag.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapDeployTemplate_1_Flag<T0: ::unity2::ClassIdentity> {
    pub value: i32,
    pub _phantom: ::core::marker::PhantomData<(T0)>,
}

impl<T0: ::unity2::ClassIdentity> ::unity2::ClassIdentity for MapDeployTemplate_1_Flag<T0> {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapDeployTemplate`1.Flag";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl<T0: ::unity2::ClassIdentity> ::unity2::IlType for MapDeployTemplate_1_Flag<T0> {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl<T0: ::unity2::ClassIdentity> MapDeployTemplate_1_Flag<T0> {
    pub fn full_area() -> Self {
        Self {
            value: 1,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn enemy() -> Self {
        Self {
            value: 2,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn appear() -> Self {
        Self {
            value: 4,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn appear_ai_no_move() -> Self {
        Self {
            value: 16,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn route() -> Self {
        Self {
            value: 32,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn fixed() -> Self {
        Self {
            value: 64,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn idle() -> Self {
        Self {
            value: 128,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn cost_free() -> Self {
        Self {
            value: 256,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn door_free() -> Self {
        Self {
            value: 512,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn sight_free() -> Self {
        Self {
            value: 1024,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn block_free() -> Self {
        Self {
            value: 4096,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn range_side() -> Self {
        Self {
            value: 8192,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn show() -> Self {
        Self {
            value: 16384,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn event() -> Self {
        Self {
            value: 32768,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn dispos() -> Self {
        Self {
            value: 65536,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn cost_free_skill() -> Self {
        Self {
            value: 131072,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn enemy_pass_skill() -> Self {
        Self {
            value: 262144,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn danger() -> Self {
        Self {
            value: 1048576,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn attack() -> Self {
        Self {
            value: 16777216,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn rod() -> Self {
        Self {
            value: 33554432,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn interference_rod() -> Self {
        Self {
            value: 67108864,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn heal_rod() -> Self {
        Self {
            value: 134217728,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn support_rod() -> Self {
        Self {
            value: 268435456,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn rewarp_rod() -> Self {
        Self {
            value: 536870912,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn engage() -> Self {
        Self {
            value: 1073741824,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn engage_here() -> Self {
        Self {
            value: -2147483648,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn cannon() -> Self {
        Self {
            value: 0,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn equip() -> Self {
        Self {
            value: 0,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn select() -> Self {
        Self {
            value: 0,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn basic_rod() -> Self {
        Self {
            value: 0,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn ignore_silent() -> Self {
        Self {
            value: 0,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn keep_attack_image() -> Self {
        Self {
            value: 0,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn keep_rod_image() -> Self {
        Self {
            value: 0,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn keep_action_image() -> Self {
        Self {
            value: 0,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn before_move() -> Self {
        Self {
            value: 0,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn clamp_area() -> Self {
        Self {
            value: 0,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn ignore_cannon() -> Self {
        Self {
            value: 0,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn full_bullet() -> Self {
        Self {
            value: 0,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn mask_free() -> Self {
        Self {
            value: 131328,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn mask_weapon() -> Self {
        Self {
            value: 1593835520,
            _phantom: ::core::marker::PhantomData,
        }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapdeploytemplate_1/MapDeployTemplate_1_ImageType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapDeployTemplate_1_ImageType<T0: ::unity2::ClassIdentity> {
    pub value: i32,
    pub _phantom: ::core::marker::PhantomData<(T0)>,
}

impl<T0: ::unity2::ClassIdentity> ::unity2::ClassIdentity for MapDeployTemplate_1_ImageType<T0> {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapDeployTemplate`1.ImageType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl<T0: ::unity2::ClassIdentity> ::unity2::IlType for MapDeployTemplate_1_ImageType<T0> {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl<T0: ::unity2::ClassIdentity> MapDeployTemplate_1_ImageType<T0> {
    pub fn r#move() -> Self {
        Self {
            value: 0,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn attack() -> Self {
        Self {
            value: 1,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn rod() -> Self {
        Self {
            value: 2,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn heal() -> Self {
        Self {
            value: 3,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn support() -> Self {
        Self {
            value: 4,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn interference() -> Self {
        Self {
            value: 5,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn engage() -> Self {
        Self {
            value: 6,
            _phantom: ::core::marker::PhantomData,
        }
    }

    pub fn num() -> Self {
        Self {
            value: 7,
            _phantom: ::core::marker::PhantomData,
        }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapdeploytemplate_1/MapDeployTemplate_1_SwapBufferScope.md")))]
#[::unity2::class(namespace = "App", name = "MapDeployTemplate`1.SwapBufferScope")]
pub struct MapDeployTemplate_1_SwapBufferScope<T0: ::unity2::ClassIdentity> {
    #[rename(name = "m_Deploy")]
    pub m_deploy: crate::app::imapdeploy_interface::IMapDeploy_Interface,
}

#[cfg(feature = "app-mapdeploytemplate_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> MapDeployTemplate_1_SwapBufferScope<T0> {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, deploy: crate::app::imapdeploy_interface::IMapDeploy_Interface) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}

#[cfg(feature = "app-mapdeploytemplate_1")]
impl<T0: ::unity2::ClassIdentity> MapDeployTemplate_1_SwapBufferScope<T0> {
    pub fn new(deploy: crate::app::imapdeploy_interface::IMapDeploy_Interface) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapDeployTemplate_1_SwapBufferScope),
                ::core::stringify!(new),
            )
        });
        <Self as IMapDeployTemplate_1_SwapBufferScopeMethods<T0>>::ctor(this, deploy);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapdeploytemplate_1/MapDeployTemplate_1.md")))]
#[::unity2::class(namespace = "App", name = "MapDeployTemplate`1")]
pub struct MapDeployTemplate_1<T0: ::unity2::ClassIdentity> {
    #[static_field]
    #[rename(name = "MoveMax")]
    pub move_max: i32,
    #[rename(name = "m_QueueA")]
    pub m_queue_a: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Queue<T0>,
    #[rename(name = "m_QueueB")]
    pub m_queue_b: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Queue<T0>,
    #[rename(name = "m_ReadQueue")]
    pub m_read_queue: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Queue<T0>,
    #[rename(name = "m_WriteQueue")]
    pub m_write_queue: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Queue<T0>,
    #[rename(name = "m_QueueCount")]
    pub m_queue_count: i32,
    #[rename(name = "m_MoveImage")]
    pub m_move_image: crate::app::mapdeploymoveimage::MapDeployMoveImage,
    #[rename(name = "m_AttackImage")]
    pub m_attack_image: crate::app::mapdeployattackimage::MapDeployAttackImage,
    #[rename(name = "m_RodImage")]
    pub m_rod_image: crate::app::mapdeployrodimage::MapDeployRodImage,
    #[rename(name = "m_HealImage")]
    pub m_heal_image: crate::app::mapdeployhealimage::MapDeployHealImage,
    #[rename(name = "m_SupportImage")]
    pub m_support_image: crate::app::mapdeploysupportimage::MapDeploySupportImage,
    #[rename(name = "m_InterferenceImage")]
    pub m_interference_image: crate::app::mapdeployinterferenceimage::MapDeployInterferenceImage,
    #[rename(name = "m_EngageImage")]
    pub m_engage_image: crate::app::mapdeployengageimage::MapDeployEngageImage,
    #[rename(name = "m_DrawMoveImage")]
    pub m_draw_move_image: crate::app::mapimagecorebit::MapImageCoreBit,
    #[rename(name = "m_ActionImage")]
    pub m_action_image: crate::app::mapdeployactionimage::MapDeployActionImage,
    #[rename(name = "m_MaskImage")]
    pub m_mask_image: crate::app::mapdeploymoveimage::MapDeployMoveImage,
    #[rename(name = "m_MoveBufferA")]
    pub m_move_buffer_a: crate::app::mapdeploymoveimage::MapDeployMoveImage,
    #[rename(name = "m_MoveBufferB")]
    pub m_move_buffer_b: crate::app::mapdeploymoveimage::MapDeployMoveImage,
    #[rename(name = "m_DrawMoveBufferA")]
    pub m_draw_move_buffer_a: crate::app::mapimagecorebit::MapImageCoreBit,
    #[rename(name = "m_DrawMoveBufferB")]
    pub m_draw_move_buffer_b: crate::app::mapimagecorebit::MapImageCoreBit,
    #[rename(name = "m_AttackBufferA")]
    pub m_attack_buffer_a: crate::app::mapdeployattackimage::MapDeployAttackImage,
    #[rename(name = "m_AttackBufferB")]
    pub m_attack_buffer_b: crate::app::mapdeployattackimage::MapDeployAttackImage,
    #[rename(name = "m_RodBufferA")]
    pub m_rod_buffer_a: crate::app::mapdeployrodimage::MapDeployRodImage,
    #[rename(name = "m_RodBufferB")]
    pub m_rod_buffer_b: crate::app::mapdeployrodimage::MapDeployRodImage,
    #[rename(name = "m_HealBufferA")]
    pub m_heal_buffer_a: crate::app::mapdeployhealimage::MapDeployHealImage,
    #[rename(name = "m_HealBufferB")]
    pub m_heal_buffer_b: crate::app::mapdeployhealimage::MapDeployHealImage,
    #[rename(name = "m_SupportBufferA")]
    pub m_support_buffer_a: crate::app::mapdeploysupportimage::MapDeploySupportImage,
    #[rename(name = "m_SupportBufferB")]
    pub m_support_buffer_b: crate::app::mapdeploysupportimage::MapDeploySupportImage,
    #[rename(name = "m_InterferenceBufferA")]
    pub m_interference_buffer_a: crate::app::mapdeployinterferenceimage::MapDeployInterferenceImage,
    #[rename(name = "m_InterferenceBufferB")]
    pub m_interference_buffer_b: crate::app::mapdeployinterferenceimage::MapDeployInterferenceImage,
    #[rename(name = "m_ActionBufferA")]
    pub m_action_buffer_a: crate::app::mapdeployactionimage::MapDeployActionImage,
    #[rename(name = "m_ActionBufferB")]
    pub m_action_buffer_b: crate::app::mapdeployactionimage::MapDeployActionImage,
    #[rename(name = "m_EngageBufferA")]
    pub m_engage_buffer_a: crate::app::mapdeployengageimage::MapDeployEngageImage,
    #[rename(name = "m_EngageBufferB")]
    pub m_engage_buffer_b: crate::app::mapdeployengageimage::MapDeployEngageImage,
    #[rename(name = "m_MoveBufferWork")]
    pub m_move_buffer_work: crate::app::mapdeploymoveimage::MapDeployMoveImage,
    #[rename(name = "m_TrickBuffer")]
    pub m_trick_buffer: crate::app::mapdeploytrickimage::MapDeployTrickImage,
    #[rename(name = "m_TrickBuffer2")]
    pub m_trick_buffer2: crate::app::mapdeploytrickimage::MapDeployTrickImage,
    #[rename(name = "m_CostMaxImage")]
    pub m_cost_max_image: crate::app::mapdeployzocimage::MapDeployZocImage,
    #[rename(name = "m_CostMinImage")]
    pub m_cost_min_image: crate::app::mapdeployzocimage::MapDeployZocImage,
    #[rename(name = "m_NotMoveImage")]
    pub m_not_move_image: crate::app::mapdeployzocimage::MapDeployZocImage,
    #[rename(name = "m_TargetImage")]
    pub m_target_image: crate::app::mapdeployattackimage::MapDeployAttackImage,
    #[rename(name = "m_RangeImage")]
    pub m_range_image: crate::app::mapdeployrangeimage::MapDeployRangeImage,
    #[rename(name = "m_OverlapImage")]
    pub m_overlap_image: crate::app::mapdeployoverlapimage::MapDeployOverlapImage,
    #[rename(name = "m_LandImage")]
    pub m_land_image: crate::app::mapdeploylandimage::MapDeployLandImage,
    #[rename(name = "m_SupportForUnitImage")]
    pub m_support_for_unit_image:
        crate::app::mapdeploysupportforunitimage::MapDeploySupportForUnitImage,
    #[rename(name = "m_DanceImage")]
    pub m_dance_image: crate::app::mapdeploydanceimage::MapDeployDanceImage,
    #[rename(name = "m_CannonImage")]
    pub m_cannon_image: crate::app::mapdeploycannonimage::MapDeployCannonImage,
    #[rename(name = "m_Flag")]
    pub m_flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_FlagField<T0>,
    #[rename(name = "m_ForceType")]
    pub m_force_type: crate::app::force::Force_Type,
    #[rename(name = "m_AreaX1")]
    pub m_area_x1: u8,
    #[rename(name = "m_AreaZ1")]
    pub m_area_z1: u8,
    #[rename(name = "m_AreaX2")]
    pub m_area_x2: u8,
    #[rename(name = "m_AreaZ2")]
    pub m_area_z2: u8,
    #[rename(name = "m_MovePower")]
    pub m_move_power: u8,
    #[rename(name = "m_MoveType")]
    pub m_move_type: crate::app::jobdata::JobData_MoveTypes,
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_NoCannonBit")]
    pub m_no_cannon_bit: ::unity2::Array<u64>,
    #[rename(name = "m_NoCannonMax")]
    pub m_no_cannon_max: ::unity2::Array<i32>,
    #[rename(name = "m_BitArray")]
    pub m_bit_array: ::unity2::Array<u64>,
    #[rename(name = "m_MaxArray")]
    pub m_max_array: ::unity2::Array<i32>,
}

#[cfg(feature = "app-mapdeploytemplate_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> MapDeployTemplate_1<T0> {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "TurnReset", args = 1)]
    pub fn turn_reset(self, force_type: crate::app::force::Force_Type) -> ();

    #[method(name = "ResetDisplayType", args = 0)]
    pub fn reset_display_type(self) -> ();

    #[method(name = "Move", args = 5)]
    pub fn r#move(
        self,
        x: i32,
        z: i32,
        move_type: crate::app::jobdata::JobData_MoveTypes,
        move_power: i32,
        flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
    ) -> ();

    #[method(name = "Move", args = 6)]
    pub fn r#move_2(
        self,
        unit: crate::app::unit::Unit,
        x: i32,
        z: i32,
        move_type: crate::app::jobdata::JobData_MoveTypes,
        move_power: i32,
        flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
    ) -> ();

    #[method(name = "UpdateSkillImage", args = 1)]
    pub fn update_skill_image(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "ClearSkillImage", args = 0)]
    pub fn clear_skill_image(self) -> ();

    #[method(name = "EventMove", args = 5)]
    pub fn event_move(
        self,
        x: i32,
        z: i32,
        move_type: crate::app::jobdata::JobData_MoveTypes,
        move_power: i32,
        flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
    ) -> ();

    #[method(name = "DisposMove", args = 5)]
    pub fn dispos_move(
        self,
        x: i32,
        z: i32,
        move_type: crate::app::jobdata::JobData_MoveTypes,
        move_power: i32,
        flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
    ) -> ();

    #[method(name = "UnitMove", args = 4)]
    pub fn unit_move(
        self,
        unit: crate::app::unit::Unit,
        move_power: i32,
        flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
        weapon_flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
    ) -> ();

    #[method(name = "UnitMoveXZ", args = 6)]
    pub fn unit_move_xz(
        self,
        unit: crate::app::unit::Unit,
        x: i32,
        z: i32,
        move_power: i32,
        flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
        weapon_flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
    ) -> ();

    #[method(name = "UnitMoveItem", args = 5)]
    pub fn unit_move_item(
        self,
        unit: crate::app::unit::Unit,
        unit_item: crate::app::unititem::UnitItem,
        move_power: i32,
        flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
        weapon_flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
    ) -> ();

    #[method(name = "UnitDance", args = 1)]
    pub fn unit_dance(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitMoveAction", args = 1)]
    pub fn unit_move_action(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitMoveEngage", args = 5)]
    pub fn unit_move_engage(
        self,
        unit: crate::app::unit::Unit,
        move_power: i32,
        mind: crate::app::mapmind::MapMind_Type,
        flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
        weapon_flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
    ) -> ();

    #[method(name = "UnitAIMove", args = 4)]
    pub fn unit_ai_move(
        self,
        unit: crate::app::unit::Unit,
        move_power: i32,
        flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
        weapon_flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
    ) -> ();

    #[method(name = "UnitAIMoveXY", args = 6)]
    pub fn unit_ai_move_xy(
        self,
        unit: crate::app::unit::Unit,
        x: i32,
        z: i32,
        move_power: i32,
        flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
        weapon_flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
    ) -> ();

    #[method(name = "UnitWarp", args = 4)]
    pub fn unit_warp(
        self,
        rod_unit: crate::app::unit::Unit,
        target_unit: crate::app::unit::Unit,
        unit_item: crate::app::unititem::UnitItem,
        flag: i32,
    ) -> ();

    #[method(name = "UnitRewarp", args = 3)]
    pub fn unit_rewarp(
        self,
        unit: crate::app::unit::Unit,
        skill: crate::app::skilldata::SkillData,
        flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
    ) -> ();

    #[method(name = "UnitRewarp", args = 3)]
    pub fn unit_rewarp_2(
        self,
        unit: crate::app::unit::Unit,
        unit_item: crate::app::unititem::UnitItem,
        flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
    ) -> ();

    #[method(name = "UnitRewarp", args = 3)]
    pub fn unit_rewarp_3(
        self,
        unit: crate::app::unit::Unit,
        range: i32,
        flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
    ) -> ();

    #[method(name = "UnitRewarp", args = 4)]
    pub fn unit_rewarp_4(
        self,
        unit: crate::app::unit::Unit,
        range: i32,
        flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
        mask: crate::app::mapdeployattackimage::MapDeployAttackImage,
    ) -> ();

    #[method(name = "RegistRewarpMoveImage", args = 6)]
    pub fn regist_rewarp_move_image(
        unit: crate::app::unit::Unit,
        args_x: i32,
        args_z: i32,
        rewarp_range: i32,
        r#move: crate::app::mapdeploymoveimage::MapDeployMoveImage,
        mask: crate::app::mapdeployattackimage::MapDeployAttackImage,
    ) -> ();

    #[method(name = "UnitCreation", args = 2)]
    pub fn unit_creation(self, rod_unit: crate::app::unit::Unit, rod_unit_item_index: i32) -> ();

    #[method(name = "UnitFireCannon", args = 1)]
    pub fn unit_fire_cannon(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitAIMoveLimit", args = 2)]
    pub fn unit_ai_move_limit(
        self,
        unit: crate::app::unit::Unit,
        flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
    ) -> ();

    #[method(name = "UnitFill", args = 2)]
    pub fn unit_fill(
        self,
        unit: crate::app::unit::Unit,
        flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
    ) -> ();

    #[method(name = "UnitFillItem", args = 3)]
    pub fn unit_fill_item(
        self,
        unit: crate::app::unit::Unit,
        unit_item: crate::app::unititem::UnitItem,
        flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
    ) -> ();

    #[method(name = "FillOneself", args = 1)]
    pub fn fill_oneself(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitFillAction", args = 1)]
    pub fn unit_fill_action(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "FillDragonVein", args = 0)]
    pub fn fill_dragon_vein(self) -> ();

    #[method(name = "UnitFillEngage", args = 3)]
    pub fn unit_fill_engage(
        self,
        unit: crate::app::unit::Unit,
        mind: crate::app::mapmind::MapMind_Type,
        flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
    ) -> ();

    #[method(name = "UnitAIFillSkill", args = 3)]
    pub fn unit_ai_fill_skill(
        self,
        unit: crate::app::unit::Unit,
        skill: crate::app::skilldata::SkillData,
        flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
    ) -> ();

    #[method(name = "UnitAIFillItem", args = 4)]
    pub fn unit_ai_fill_item(
        self,
        unit: crate::app::unit::Unit,
        unit_item: crate::app::unititem::UnitItem,
        skill: crate::app::skilldata::SkillData,
        flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
    ) -> ();

    #[method(name = "UnitAIFillRewarp", args = 1)]
    pub fn unit_ai_fill_rewarp(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "RangeAround", args = 1)]
    pub fn range_around(self, value: i32) -> ();

    #[method(name = "RangeImmobile", args = 4)]
    pub fn range_immobile(self, xx: i32, zz: i32, range_i: i32, range_o: i32) -> ();

    #[method(name = "UnitRangeImmobile", args = 4)]
    pub fn unit_range_immobile(
        self,
        unit: crate::app::unit::Unit,
        xx: i32,
        zz: i32,
        flag: i32,
    ) -> ();

    #[method(name = "TargetOnly", args = 2)]
    pub fn target_only(
        self,
        attacker: crate::app::unit::Unit,
        unit_item: crate::app::unititem::UnitItem,
    ) -> ();

    #[method(name = "UnitTriangle", args = 1)]
    pub fn unit_triangle(self, attacker: crate::app::unit::Unit) -> ();

    #[method(name = "SetMoveA", args = 0)]
    pub fn set_move_a(self) -> ();

    #[method(name = "SetMoveB", args = 0)]
    pub fn set_move_b(self) -> ();

    #[method(name = "SetAttackA", args = 0)]
    pub fn set_attack_a(self) -> ();

    #[method(name = "SetAttackB", args = 0)]
    pub fn set_attack_b(self) -> ();

    #[method(name = "SetRodA", args = 0)]
    pub fn set_rod_a(self) -> ();

    #[method(name = "SetRodB", args = 0)]
    pub fn set_rod_b(self) -> ();

    #[method(name = "SetHealA", args = 0)]
    pub fn set_heal_a(self) -> ();

    #[method(name = "SetHealB", args = 0)]
    pub fn set_heal_b(self) -> ();

    #[method(name = "SetSupportA", args = 0)]
    pub fn set_support_a(self) -> ();

    #[method(name = "SetSupportB", args = 0)]
    pub fn set_support_b(self) -> ();

    #[method(name = "SetInterferenceA", args = 0)]
    pub fn set_interference_a(self) -> ();

    #[method(name = "SetInterferenceB", args = 0)]
    pub fn set_interference_b(self) -> ();

    #[method(name = "SetEngageA", args = 0)]
    pub fn set_engage_a(self) -> ();

    #[method(name = "SetEngageB", args = 0)]
    pub fn set_engage_b(self) -> ();

    #[method(name = "SetBufferA", args = 0)]
    pub fn set_buffer_a(self) -> ();

    #[method(name = "SetBufferB", args = 0)]
    pub fn set_buffer_b(self) -> ();

    #[method(name = "SetMaskImage", args = 1)]
    pub fn set_mask_image(self, image: crate::app::mapdeploymoveimage::MapDeployMoveImage) -> ();

    #[method(name = "get_MoveImage", args = 0)]
    pub fn get_move_image(self) -> crate::app::mapdeploymoveimage::MapDeployMoveImage;

    #[method(name = "set_MoveImage", args = 1)]
    pub fn set_move_image(self, value: crate::app::mapdeploymoveimage::MapDeployMoveImage) -> ();

    #[method(name = "get_DrawMoveImage", args = 0)]
    pub fn get_draw_move_image(self) -> crate::app::mapimagecorebit::MapImageCoreBit;

    #[method(name = "get_MoveImageA", args = 0)]
    pub fn get_move_image_a(self) -> crate::app::mapdeploymoveimage::MapDeployMoveImage;

    #[method(name = "get_MoveImageB", args = 0)]
    pub fn get_move_image_b(self) -> crate::app::mapdeploymoveimage::MapDeployMoveImage;

    #[method(name = "get_AttackImage", args = 0)]
    pub fn get_attack_image(self) -> crate::app::mapdeployattackimage::MapDeployAttackImage;

    #[method(name = "set_AttackImage", args = 1)]
    pub fn set_attack_image(
        self,
        value: crate::app::mapdeployattackimage::MapDeployAttackImage,
    ) -> ();

    #[method(name = "get_AttackImageA", args = 0)]
    pub fn get_attack_image_a(self) -> crate::app::mapdeployattackimage::MapDeployAttackImage;

    #[method(name = "get_AttackImageB", args = 0)]
    pub fn get_attack_image_b(self) -> crate::app::mapdeployattackimage::MapDeployAttackImage;

    #[method(name = "get_RodImage", args = 0)]
    pub fn get_rod_image(self) -> crate::app::mapdeployrodimage::MapDeployRodImage;

    #[method(name = "set_RodImage", args = 1)]
    pub fn set_rod_image(self, value: crate::app::mapdeployrodimage::MapDeployRodImage) -> ();

    #[method(name = "get_HealImage", args = 0)]
    pub fn get_heal_image(self) -> crate::app::mapdeployhealimage::MapDeployHealImage;

    #[method(name = "set_HealImage", args = 1)]
    pub fn set_heal_image(self, value: crate::app::mapdeployhealimage::MapDeployHealImage) -> ();

    #[method(name = "get_SupportImage", args = 0)]
    pub fn get_support_image(self) -> crate::app::mapdeploysupportimage::MapDeploySupportImage;

    #[method(name = "set_SupportImage", args = 1)]
    pub fn set_support_image(
        self,
        value: crate::app::mapdeploysupportimage::MapDeploySupportImage,
    ) -> ();

    #[method(name = "get_SupportForUnitImage", args = 0)]
    pub fn get_support_for_unit_image(
        self,
    ) -> crate::app::mapdeploysupportforunitimage::MapDeploySupportForUnitImage;

    #[method(name = "set_SupportForUnitImage", args = 1)]
    pub fn set_support_for_unit_image(
        self,
        value: crate::app::mapdeploysupportforunitimage::MapDeploySupportForUnitImage,
    ) -> ();

    #[method(name = "get_InterferenceImage", args = 0)]
    pub fn get_interference_image(
        self,
    ) -> crate::app::mapdeployinterferenceimage::MapDeployInterferenceImage;

    #[method(name = "set_InterferenceImage", args = 1)]
    pub fn set_interference_image(
        self,
        value: crate::app::mapdeployinterferenceimage::MapDeployInterferenceImage,
    ) -> ();

    #[method(name = "get_ActionImage", args = 0)]
    pub fn get_action_image(self) -> crate::app::mapdeployactionimage::MapDeployActionImage;

    #[method(name = "set_ActionImage", args = 1)]
    pub fn set_action_image(
        self,
        value: crate::app::mapdeployactionimage::MapDeployActionImage,
    ) -> ();

    #[method(name = "get_EngageImage", args = 0)]
    pub fn get_engage_image(self) -> crate::app::mapdeployengageimage::MapDeployEngageImage;

    #[method(name = "set_EngageImage", args = 1)]
    pub fn set_engage_image(
        self,
        value: crate::app::mapdeployengageimage::MapDeployEngageImage,
    ) -> ();

    #[method(name = "get_RangeImage", args = 0)]
    pub fn get_range_image(self) -> crate::app::mapdeployrangeimage::MapDeployRangeImage;

    #[method(name = "set_RangeImage", args = 1)]
    pub fn set_range_image(self, value: crate::app::mapdeployrangeimage::MapDeployRangeImage)
        -> ();

    #[method(name = "get_OverlapImage", args = 0)]
    pub fn get_overlap_image(self) -> crate::app::mapdeployoverlapimage::MapDeployOverlapImage;

    #[method(name = "set_OverlapImage", args = 1)]
    pub fn set_overlap_image(
        self,
        value: crate::app::mapdeployoverlapimage::MapDeployOverlapImage,
    ) -> ();

    #[method(name = "get_LandImage", args = 0)]
    pub fn get_land_image(self) -> crate::app::mapdeploylandimage::MapDeployLandImage;

    #[method(name = "set_LandImage", args = 1)]
    pub fn set_land_image(self, value: crate::app::mapdeploylandimage::MapDeployLandImage) -> ();

    #[method(name = "get_DanceImage", args = 0)]
    pub fn get_dance_image(self) -> crate::app::mapdeploydanceimage::MapDeployDanceImage;

    #[method(name = "set_DanceImage", args = 1)]
    pub fn set_dance_image(self, value: crate::app::mapdeploydanceimage::MapDeployDanceImage)
        -> ();

    #[method(name = "get_CannonImage", args = 0)]
    pub fn get_cannon_image(self) -> crate::app::mapdeploycannonimage::MapDeployCannonImage;

    #[method(name = "set_CannonImage", args = 1)]
    pub fn set_cannon_image(
        self,
        value: crate::app::mapdeploycannonimage::MapDeployCannonImage,
    ) -> ();

    #[method(name = "get_TrickImage", args = 0)]
    pub fn get_trick_image(self) -> crate::app::mapdeploytrickimage::MapDeployTrickImage;

    #[method(name = "get_TrickImage2", args = 0)]
    pub fn get_trick_image2(self) -> crate::app::mapdeploytrickimage::MapDeployTrickImage;

    #[method(name = "get_ForceType", args = 0)]
    pub fn get_force_type(self) -> crate::app::force::Force_Type;

    #[method(name = "set_ForceType", args = 1)]
    pub fn set_force_type(self, value: crate::app::force::Force_Type) -> ();

    #[method(name = "get_BmapSize", args = 0)]
    pub fn get_bmap_size(self) -> i32;

    #[method(name = "set_BmapSize", args = 1)]
    pub fn set_bmap_size(self, value: i32) -> ();

    #[method(name = "SearchDir", args = 1)]
    pub fn search_dir(self, dir: crate::app::dir_2::Dir_Type) -> ();

    #[method(name = "IsOutOfArea", args = 2)]
    pub fn is_out_of_area(self, x: i32, z: i32) -> bool;

    #[method(name = "GetCost", args = 2)]
    pub fn get_cost(self, x: i32, z: i32) -> i32;

    #[method(name = "IsBeforeMove", args = 1)]
    pub fn is_before_move(self, skill: crate::app::skilldata::SkillData) -> bool;

    #[method(name = "GetRangeBit", args = 3)]
    pub fn get_range_bit(
        self,
        unit: crate::app::unit::Unit,
        bit: ::unity2::Array<u64>,
        max: ::unity2::Array<i32>,
    ) -> ();

    #[method(name = "GetRangeBit", args = 4)]
    pub fn get_range_bit_2(
        self,
        unit: crate::app::unit::Unit,
        skill: crate::app::skilldata::SkillData,
        bit_array: ::unity2::Array<u64>,
        max_array: ::unity2::Array<i32>,
    ) -> ();

    #[method(name = "IsFill", args = 2)]
    pub fn is_fill(self, x: i32, z: i32) -> bool;

    #[method(name = "Fill", args = 3)]
    pub fn fill(
        self,
        unit: crate::app::unit::Unit,
        skill: crate::app::skilldata::SkillData,
        is_consider_cannon: bool,
    ) -> ();

    #[method(name = "FillAttack", args = 3)]
    pub fn fill_attack(self, unit: crate::app::unit::Unit, bit: u64, max: i32) -> ();

    #[method(name = "FillRod", args = 3)]
    pub fn fill_rod(self, unit: crate::app::unit::Unit, bit: u64, max: i32) -> ();

    #[method(name = "FillRangeImage", args = 0)]
    pub fn fill_range_image(self) -> ();

    #[method(name = "FillSupportForUnitImage", args = 1)]
    pub fn fill_support_for_unit_image(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "IsSupportRodForUnit", args = 1)]
    pub fn is_support_rod_for_unit(self, unit_item: crate::app::unititem::UnitItem) -> bool;

    #[method(name = "FillDanceImage", args = 1)]
    pub fn fill_dance_image(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "FillRangeImageForCannon", args = 2)]
    pub fn fill_range_image_for_cannon(
        self,
        unit: crate::app::unit::Unit,
        flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<T0>,
    ) -> ();

    #[method(name = "FillImage", args = 5)]
    pub fn fill_image(
        self,
        unit: crate::app::unit::Unit,
        bit: u64,
        min: i32,
        max: i32,
        image: crate::app::mapimagecorebit::MapImageCoreBit,
    ) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "GetDistance", args = 2)]
    pub fn get_distance(self, x: i32, z: i32) -> i32;

    #[method(name = "GetDistance", args = 3)]
    pub fn get_distance_2(self, route: crate::app::maproute::MapRoute, x: i32, z: i32) -> i32;
}

#[cfg(feature = "app-mapdeploytemplate_1")]
impl<T0: ::unity2::ClassIdentity> MapDeployTemplate_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapDeployTemplate_1),
                ::core::stringify!(new),
            )
        });
        <Self as IMapDeployTemplate_1Methods<T0>>::ctor(this);
        this
    }
}
