
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonrideresultui/DragonRideResultUI_DragonRideResultUITarget.md")))]
#[::unity2::class(
    namespace = "App",
    name = "DragonRideResultUI.DragonRideResultUITarget"
)]
#[parent(crate::system::object::Object)]
pub struct DragonRideResultUI_DragonRideResultUITarget {
    #[rename(name = "m_Node")]
    pub m_node: crate::unity_engine::transform::Transform,
    #[rename(name = "m_Text")]
    pub m_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-dragonrideresultui")]
#[::unity2::methods]
impl DragonRideResultUI_DragonRideResultUITarget {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Init", args = 1)]
    pub fn init(self, base_obj: crate::unity_engine::transform::Transform) -> ();

    #[method(name = "SetText", args = 1)]
    pub fn set_text(self, str: ::unity2::Il2CppString) -> ();

    #[method(name = "Show", args = 0)]
    pub fn show(self) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();
}

#[cfg(feature = "app-dragonrideresultui")]
impl DragonRideResultUI_DragonRideResultUITarget {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DragonRideResultUI_DragonRideResultUITarget),
                ::core::stringify!(new),
            )
        });
        <Self as IDragonRideResultUI_DragonRideResultUITargetMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonrideresultui/DragonRideResultUI_ResultPhase.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct DragonRideResultUI_ResultPhase {
    pub value: i32,
}

impl ::unity2::ClassIdentity for DragonRideResultUI_ResultPhase {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "DragonRideResultUI.ResultPhase";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DragonRideResultUI_ResultPhase {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl DragonRideResultUI_ResultPhase {
    pub fn phase_before_open() -> Self {
        Self { value: 0 }
    }

    pub fn phase_title() -> Self {
        Self { value: 1 }
    }

    pub fn phase_target_count() -> Self {
        Self { value: 2 }
    }

    pub fn phase_assist_count() -> Self {
        Self { value: 3 }
    }

    pub fn phase_score() -> Self {
        Self { value: 4 }
    }

    pub fn phase_best_score() -> Self {
        Self { value: 5 }
    }

    pub fn phase_rank() -> Self {
        Self { value: 6 }
    }

    pub fn phase_other() -> Self {
        Self { value: 7 }
    }

    pub fn phase_finish() -> Self {
        Self { value: 8 }
    }

    pub fn phase_count() -> Self {
        Self { value: 9 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonrideresultui/DragonRideResultUI.md")))]
#[::unity2::class(namespace = "App", name = "DragonRideResultUI")]
#[parent(crate::system::object::Object)]
pub struct DragonRideResultUI {
    #[static_field]
    #[rename(name = "cFanfareExcellentIndex")]
    pub c_fanfare_excellent_index: i32,
    #[static_field]
    #[rename(name = "cFanfareGoodIndex")]
    pub c_fanfare_good_index: i32,
    #[rename(name = "cRankNodeList")]
    pub c_rank_node_list: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "cTargetNodeList")]
    pub c_target_node_list: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "cTargetMessageList")]
    pub c_target_message_list: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "m_Root")]
    pub m_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_data")]
    pub m_data: crate::app::dragonrideresultui::DragonRideResultUI_DataSet,
    #[rename(name = "m_Timer")]
    pub m_timer: f64,
    #[rename(name = "m_Phase")]
    pub m_phase: crate::app::dragonrideresultui::DragonRideResultUI_ResultPhase,
    #[rename(name = "m_LevelText")]
    pub m_level_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_BrokenTargetNode")]
    pub m_broken_target_node: crate::unity_engine::transform::Transform,
    #[rename(name = "m_AssistNode")]
    pub m_assist_node: crate::unity_engine::transform::Transform,
    #[rename(name = "m_ScoreNode")]
    pub m_score_node: crate::unity_engine::transform::Transform,
    #[rename(name = "m_ScoreText")]
    pub m_score_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_RankRoot")]
    pub m_rank_root: crate::unity_engine::transform::Transform,
    #[rename(name = "m_RankNode")]
    pub m_rank_node: crate::unity_engine::transform::Transform,
    #[rename(name = "m_BestScoreNode")]
    pub m_best_score_node: crate::unity_engine::transform::Transform,
    #[rename(name = "m_BestScoreText")]
    pub m_best_score_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_NewRecordNode")]
    pub m_new_record_node: crate::unity_engine::transform::Transform,
    #[rename(name = "m_Config")]
    pub m_config: crate::app::dragonrideconfig::DragonRideConfig,
}

#[cfg(feature = "app-dragonrideresultui")]
#[::unity2::methods]
impl DragonRideResultUI {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Init", args = 2)]
    pub fn init(
        self,
        root_obj: crate::unity_engine::gameobject::GameObject,
        data: crate::app::dragonrideresultui::DragonRideResultUI_DataSet,
    ) -> ();

    #[method(name = "NextPhase", args = 0)]
    pub fn next_phase(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "PlayResult", args = 0)]
    pub fn play_result(self) -> ();

    #[method(name = "PlayCloseAnime", args = 0)]
    pub fn play_close_anime(self) -> ();

    #[method(name = "VoiceResultPerfect", args = 0)]
    pub fn voice_result_perfect(self) -> ();

    #[method(name = "VoiceResultGood", args = 0)]
    pub fn voice_result_good(self) -> ();

    #[method(name = "VoiceResultBad", args = 0)]
    pub fn voice_result_bad(self) -> ();

    #[method(name = "DestroyResult", args = 0)]
    pub fn destroy_result(self) -> ();

    #[method(name = "get_IsFinished", args = 0)]
    pub fn get_is_finished(self) -> bool;
}

#[cfg(feature = "app-dragonrideresultui")]
impl DragonRideResultUI {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DragonRideResultUI),
                ::core::stringify!(new),
            )
        });
        <Self as IDragonRideResultUIMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonrideresultui/DragonRideResultUI_DataSet.md")))]
#[::unity2::class(namespace = "App", name = "DragonRideResultUI.DataSet")]
#[parent(crate::system::object::Object)]
pub struct DragonRideResultUI_DataSet {
    #[rename(name = "Level")]
    pub level: i32,
}

#[cfg(feature = "app-dragonrideresultui")]
#[::unity2::methods]
impl DragonRideResultUI_DataSet {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "SetList", args = 0)]
    pub fn set_list(self) -> ();

    #[method(name = "getTotalCount", args = 0)]
    pub fn get_total_count(self) -> i32;

    #[method(name = "get_Score", args = 0)]
    pub fn get_score(self) -> i32;

    #[method(name = "set_Score", args = 1)]
    pub fn set_score(self, value: i32) -> ();

    #[method(name = "get_BestScore", args = 0)]
    pub fn get_best_score(self) -> i32;

    #[method(name = "set_BestScore", args = 1)]
    pub fn set_best_score(self, value: i32) -> ();

    #[method(name = "get_RankNum", args = 0)]
    pub fn get_rank_num(self) -> i32;

    #[method(name = "set_RankNum", args = 1)]
    pub fn set_rank_num(self, value: i32) -> ();

    #[method(name = "get_IsAssisted", args = 0)]
    pub fn get_is_assisted(self) -> bool;

    #[method(name = "set_IsAssisted", args = 1)]
    pub fn set_is_assisted(self, value: bool) -> ();

    #[method(name = "get_AssistCount", args = 0)]
    pub fn get_assist_count(self) -> i32;

    #[method(name = "set_AssistCount", args = 1)]
    pub fn set_assist_count(self, value: i32) -> ();

    #[method(name = "get_NormalCount", args = 0)]
    pub fn get_normal_count(self) -> i32;

    #[method(name = "set_NormalCount", args = 1)]
    pub fn set_normal_count(self, value: i32) -> ();

    #[method(name = "get_BigCount", args = 0)]
    pub fn get_big_count(self) -> i32;

    #[method(name = "set_BigCount", args = 1)]
    pub fn set_big_count(self, value: i32) -> ();

    #[method(name = "get_LinkCount", args = 0)]
    pub fn get_link_count(self) -> i32;

    #[method(name = "set_LinkCount", args = 1)]
    pub fn set_link_count(self, value: i32) -> ();

    #[method(name = "get_SpecialCount", args = 0)]
    pub fn get_special_count(self) -> i32;

    #[method(name = "set_SpecialCount", args = 1)]
    pub fn set_special_count(self, value: i32) -> ();

    #[method(name = "get_RouletteCount", args = 0)]
    pub fn get_roulette_count(self) -> i32;

    #[method(name = "set_RouletteCount", args = 1)]
    pub fn set_roulette_count(self, value: i32) -> ();

    #[method(name = "get_TargetCountList", args = 0)]
    pub fn get_target_count_list(self) -> ::unity2::Array<i32>;

    #[method(name = "set_TargetCountList", args = 1)]
    pub fn set_target_count_list(self, value: ::unity2::Array<i32>) -> ();
}

#[cfg(feature = "app-dragonrideresultui")]
impl DragonRideResultUI_DataSet {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DragonRideResultUI_DataSet),
                ::core::stringify!(new),
            )
        });
        <Self as IDragonRideResultUI_DataSetMethods>::ctor(this);
        this
    }
}
