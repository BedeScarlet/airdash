use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{
            self, sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::{*, Priority::*},
};

// does effects
unsafe extern "C" fn effect_handler(fighter: &mut L2CFighterCommon) {
    unsafe {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) { 
            if MotionModule::frame(fighter.module_accessor) == 1.0 {
            let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let speed_x_adjust = if speed_x == 0.0 { 0.01 } else { 0.0 };
            let angle = (speed_y/(speed_x + speed_x_adjust)).atan();

            let pos = Vector3f { x: 0., y: 6.5, z: 0.};
            let mut rot = Vector3f { x:0., y:0., z: (90. + 180. * angle/3.14159)};

                if speed_x >= 1.4 || speed_y >= 1.4 {
                    EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("sys_whirlwind_r"), Hash40::new("top"),
                    &pos, &rot, 0.75, &Vector3f{x:0.0, y:0.0, z:0.0}, &Vector3f{x:0.0, y:0.0, z:0.0}, false, 0, 0, 0);
                }else if speed_x < -1.4 || speed_y < -1.4{
                    rot = Vector3f { x:0., y:0., z: (-90. + 180. * angle/3.14159)};
                    EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("sys_whirlwind_l"), Hash40::new("top"),
                    &pos, &rot, 0.75, &Vector3f{x:0.0, y:0.0, z:0.0}, &Vector3f{x:0.0, y:0.0, z:0.0}, false, 0, 0, 0);
                }
            }

            if MotionModule::frame(fighter.module_accessor) == 10.0 {
                if !WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == FIGHTER_KIND_BAYONETTA{
                    let remaining_frames = MotionModule::end_frame(fighter.module_accessor) - WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME) as f32;
                    let original_time_remaining = remaining_frames / 60.0;
                    let target_time_remaining = 30.0 / 60.0;

                    let play_mult = original_time_remaining / target_time_remaining;

                    MotionModule::set_rate(fighter.module_accessor, play_mult);
                }
            }
        }
    }
}


pub fn install() {
    Agent::new("fighter")
        .on_line(Main, effect_handler) // Global opff
        .install();
}
