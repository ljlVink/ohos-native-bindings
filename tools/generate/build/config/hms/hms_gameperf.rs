use once_cell::sync::Lazy;

use super::super::SysConfig;

pub const GAMEPERF: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "hms-gameperf-sys",
    headers: vec!["GameServiceKit/game_performance.h"],
    white_list: vec![
        "HMS_GamePerformance.*",
        "GamePerformance.*",
        "GAME_PERFORMANCE.*",
    ],
    block_list: vec![],
    dynamic_library: vec!["game_performance.z"],
    extra: "",
});
