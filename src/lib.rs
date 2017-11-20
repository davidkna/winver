extern crate winapi;
use winapi::minwindef::{BOOL, TRUE};

#[link(name = "winverhelper")]
extern "C" {
    fn helper_is_winxp() -> BOOL;
    fn helper_is_winxp_sp1() -> BOOL;
    fn helper_is_winxp_sp2() -> BOOL;
    fn helper_is_winxp_sp3() -> BOOL;
    fn helper_is_winvista() -> BOOL;
    fn helper_is_winvista_sp1() -> BOOL;
    fn helper_is_winvista_sp2() -> BOOL;
    fn helper_is_win7() -> BOOL;
    fn helper_is_win7_sp1() -> BOOL;
    fn helper_is_win8() -> BOOL;
    fn helper_is_win8_p1() -> BOOL;
    fn helper_is_winthreshold() -> BOOL;
    fn helper_is_win10() -> BOOL;
    fn helper_is_winserver() -> BOOL;
}

pub fn is_winxp() -> bool {
    unsafe { helper_is_winxp() == TRUE }
}

pub fn is_winxp_sp1() -> bool {
    unsafe { helper_is_winxp_sp1() == TRUE }
}

pub fn is_winxp_sp2() -> bool {
    unsafe { helper_is_winxp_sp2() == TRUE }
}

pub fn is_winxp_sp3() -> bool {
    unsafe { helper_is_winxp_sp3() == TRUE }
}

pub fn is_winvista() -> bool {
    unsafe { helper_is_winvista() == TRUE }
}

pub fn is_winvista_sp1() -> bool {
    unsafe { helper_is_winvista_sp1() == TRUE }
}

pub fn is_winvista_sp2() -> bool {
    unsafe { helper_is_winvista_sp2() == TRUE }
}

pub fn is_win7() -> bool {
    unsafe { helper_is_win7() == TRUE }
}

pub fn is_win7_sp1() -> bool {
    unsafe { helper_is_win7_sp1() == TRUE }
}

pub fn is_win8() -> bool {
    unsafe { helper_is_win8() == TRUE }
}

pub fn is_win8_p1() -> bool {
    unsafe { helper_is_win8_p1() == TRUE }
}

pub fn is_winthreshold() -> bool {
    unsafe { helper_is_winthreshold() == TRUE }
}

pub fn is_win10() -> bool {
    unsafe { helper_is_win10() == TRUE }
}

pub fn is_winserver() -> bool {
    unsafe { helper_is_winserver() == TRUE }
}
