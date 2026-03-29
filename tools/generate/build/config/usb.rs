use once_cell::sync::Lazy;

use super::SysConfig;

pub const OH_USB: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-usb-sys",
    headers: vec!["usb/usb_ddk_api.h", "usb/usb_ddk_types.h"],
    white_list: vec!["OH_Usb.*"],
    block_list: vec![],
    dynamic_library: vec!["usb_ndk.z"],
    extra: "",
});
