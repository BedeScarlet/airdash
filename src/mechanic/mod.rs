use ::{
    smash::{
        app::{ self, lua_bind::*, sv_animcmd::*, * },
        hash40,
        lib::{ lua_const::*, L2CAgent, L2CValue },
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::{ Priority::*, * },
};

unsafe extern "C" fn game_escapeairslide(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor,*FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor,*FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
        notify_event_msc_cmd!(agent,Hash40::new_raw(0x2127e37c07),*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn escapeairslide_opff(fighter: &mut L2CFighterCommon) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_PENALTY);
    } else {
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_PENALTY);
    }
}

unsafe extern "C" fn elight_forceoff_foresight(fighter: &mut L2CFighterCommon) {
    if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("escape_air_slide") {
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_FORESIGHT_SUCCESS);
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_FORESIGHT_FAILURE);
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_FORESIGHT_DISABLE);
    }
}

pub fn install() {
    Agent::new("elight")
        .on_line(Main, elight_forceoff_foresight)
        .install();
    Agent::new("fighter")
        .on_line(Main, escapeairslide_opff)
        .game_acmd("game_escapeairslide", game_escapeairslide, High)
        .install();
}
