
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::procscenesequence_1::IProcSceneSequence_1;
use crate::app::procscenesequence_1::ProcSceneSequence_1;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonridegamesequence/DragonRideGameSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct DragonRideGameSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for DragonRideGameSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "DragonRideGameSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DragonRideGameSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl DragonRideGameSequence_Label {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn init() -> Self {
        Self { value: 1 }
    }

    pub fn fake_load() -> Self {
        Self { value: 2 }
    }

    pub fn main() -> Self {
        Self { value: 3 }
    }

    pub fn retire() -> Self {
        Self { value: 4 }
    }

    pub fn result() -> Self {
        Self { value: 5 }
    }

    pub fn mascot_bond() -> Self {
        Self { value: 6 }
    }

    pub fn reload() -> Self {
        Self { value: 7 }
    }

    pub fn exit() -> Self {
        Self { value: 8 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonridegamesequence/DragonRideGameSequence.md")))]
#[::unity2::class(namespace = "App", name = "DragonRideGameSequence")]
# [parent (crate :: app :: procscenesequence_1 :: ProcSceneSequence_1 < crate :: app :: hubsequence :: HubSequence >)]
pub struct DragonRideGameSequence {
    #[static_field]
    #[rename(name = "cIntervalSec")]
    pub c_interval_sec: f32,
    #[static_field]
    #[rename(name = "cShotPrefabPath")]
    pub c_shot_prefab_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "SolaRenderPath")]
    pub sola_render_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cGroupPrefabPath")]
    pub c_group_prefab_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cEffectRootPath")]
    pub c_effect_root_path: ::unity2::Il2CppString,
    #[rename(name = "cLoadEffectList")]
    pub c_load_effect_list: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "cUIRootPath")]
    pub c_ui_root_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cUITotalScorePath")]
    pub c_ui_total_score_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cUITargetScorePath")]
    pub c_ui_target_score_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cUIResultPath")]
    pub c_ui_result_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cUIInstructionPath")]
    pub c_ui_instruction_path: ::unity2::Il2CppString,
    #[rename(name = "cLoadUIList")]
    pub c_load_ui_list: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "cTelopRootPath")]
    pub c_telop_root_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cStartTelopPath")]
    pub c_start_telop_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cFinishTelopPath")]
    pub c_finish_telop_path: ::unity2::Il2CppString,
    #[rename(name = "m_CameraComponent")]
    pub m_camera_component: crate::app::dragonridecamera::DragonRideCamera,
    #[rename(name = "m_TargetGroupList")]
    pub m_target_group_list: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_Level")]
    pub m_level: i32,
    #[rename(name = "m_ResultCloseTimer")]
    pub m_result_close_timer: f32,
    #[rename(name = "m_ResultUI")]
    pub m_result_ui: crate::app::dragonrideresultui::DragonRideResultUI,
    #[rename(name = "m_ScoreUI")]
    pub m_score_ui: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ScoreText")]
    pub m_score_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_PrevScore")]
    pub m_prev_score: i32,
    #[rename(name = "m_Config")]
    pub m_config: crate::app::dragonrideconfig::DragonRideConfig,
    #[rename(name = "m_TargetGroupResource")]
    pub m_target_group_resource: crate::app::resourcegameobject::ResourceGameObject,
    #[rename(name = "m_AssistTargetPosList")]
    pub m_assist_target_pos_list:
        crate::system::collections::generic::list_1::List_1<crate::unity_engine::vector3::Vector3>,
    #[rename(name = "m_HubSolaLct")]
    pub m_hub_sola_lct: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_IsSetSolaInvisible")]
    pub m_is_set_sola_invisible: bool,
    #[rename(name = "m_PlayerCtrl")]
    pub m_player_ctrl: crate::app::hubplayercontroller::HubPlayerController,
    #[rename(name = "m_WalkThroughCount")]
    pub m_walk_through_count: i32,
}

#[cfg(feature = "app-dragonridegamesequence")]
#[::unity2::methods]
impl DragonRideGameSequence {
    #[method(name = "get_CourseID", args = 0)]
    pub fn get_course_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_CourseID", args = 1)]
    pub fn set_course_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_IsEventMode", args = 0)]
    pub fn get_is_event_mode(self) -> bool;

    #[method(name = "set_IsEventMode", args = 1)]
    pub fn set_is_event_mode(self, value: bool) -> ();

    #[method(name = "get_IsEnableWalkThrough", args = 0)]
    pub fn get_is_enable_walk_through(self) -> bool;

    #[method(name = "set_IsEnableWalkThrough", args = 1)]
    pub fn set_is_enable_walk_through(self, value: bool) -> ();

    #[method(name = "get_IsEnableUpdateTargetList", args = 0)]
    pub fn get_is_enable_update_target_list(self) -> bool;

    #[method(name = "set_IsEnableUpdateTargetList", args = 1)]
    pub fn set_is_enable_update_target_list(self, value: bool) -> ();

    #[method(name = "_CommonDebugMenu", args = 0)]
    pub fn common_debug_menu(self) -> ();

    #[method(name = "InitFirst", args = 0)]
    pub fn init_first(self) -> ();

    #[method(name = "InitCamera", args = 0)]
    pub fn init_camera(self) -> ();

    #[method(name = "CreatePlayerModel", args = 0)]
    pub fn create_player_model(self) -> ();

    #[method(name = "IsReadyDragonModel", args = 0)]
    pub fn is_ready_dragon_model(self) -> bool;

    #[method(name = "CreateAssitModel", args = 0)]
    pub fn create_assit_model(self) -> ();

    #[method(name = "IsReadyAssistModel", args = 0)]
    pub fn is_ready_assist_model(self) -> bool;

    #[method(name = "InstantTargetGroup", args = 5)]
    pub fn instant_target_group(
        self,
        target_trans: crate::unity_engine::transform::Transform,
        parent_id: ::unity2::Il2CppString,
        pattern_id: ::unity2::Il2CppString,
        billboard_type: crate::app::dragon_ride::billboardtypes::BillboardTypes,
        active_soon: bool,
    ) -> ();

    #[method(name = "InitTarget", args = 0)]
    pub fn init_target(self) -> ();

    #[method(name = "InitReload", args = 0)]
    pub fn init_reload(self) -> ();

    #[method(name = "LoadResource", args = 0)]
    pub fn load_resource(self) -> ();

    #[method(name = "IsLoadFinish", args = 0)]
    pub fn is_load_finish(self) -> bool;

    #[method(name = "UnloadResource", args = 0)]
    pub fn unload_resource(self) -> ();

    #[method(name = "SetGroupVisible", args = 2)]
    pub fn set_group_visible(
        self,
        lct_list: ::unity2::Array<::unity2::Il2CppString>,
        visible: bool,
    ) -> ();

    #[method(name = "JumpMain", args = 0)]
    pub fn jump_main(self) -> ();

    #[method(name = "JumpRetire", args = 0)]
    pub fn jump_retire(self) -> ();

    #[method(name = "TickRetire", args = 0)]
    pub fn tick_retire(self) -> ();

    #[method(name = "InitFake", args = 0)]
    pub fn init_fake(self) -> ();

    #[method(name = "TickFakeLoad", args = 0)]
    pub fn tick_fake_load(self) -> ();

    #[method(name = "StartMain", args = 0)]
    pub fn start_main(self) -> ();

    #[method(name = "Main", args = 0)]
    pub fn main(self) -> ();

    #[method(name = "FinishMain", args = 0)]
    pub fn finish_main(self) -> ();

    #[method(name = "CameraFadeIn", args = 0)]
    pub fn camera_fade_in(self) -> ();

    #[method(name = "SetResultSeq", args = 0)]
    pub fn set_result_seq(self) -> ();

    #[method(name = "CalcRankNum", args = 2)]
    pub fn calc_rank_num(self, level: i32, score: i32) -> i32;

    #[method(name = "CreateResultUI", args = 0)]
    pub fn create_result_ui(self) -> ();

    #[method(name = "TickResult", args = 0)]
    pub fn tick_result(self) -> ();

    #[method(name = "TickResultCloseWait", args = 0)]
    pub fn tick_result_close_wait(self) -> ();

    #[method(name = "SetMascotBond", args = 0)]
    pub fn set_mascot_bond(self) -> ();

    #[method(name = "IncreasePlayCounter", args = 0)]
    pub fn increase_play_counter(self) -> ();

    #[method(name = "Exit", args = 0)]
    pub fn exit(self) -> ();

    #[method(name = "OnShutdown", args = 0)]
    pub fn on_shutdown(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-dragonridegamesequence")]
impl DragonRideGameSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DragonRideGameSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IDragonRideGameSequenceMethods>::ctor(this);
        this
    }
}
