#include <Windows.h>
#include <VersionHelpers.h>

BOOL helper_is_winxp() { return IsWindowsXPOrGreater(); }
BOOL helper_is_winxp_sp1() { return IsWindowsXPSP1OrGreater(); }
BOOL helper_is_winxp_sp2() { return IsWindowsXPSP2OrGreater(); }
BOOL helper_is_winxp_sp3() { return IsWindowsXPSP3OrGreater(); }
BOOL helper_is_winvista() { return IsWindowsVistaOrGreater(); }
BOOL helper_is_winvista_sp1() { return IsWindowsVistaSP1OrGreater(); }
BOOL helper_is_winvista_sp2() { return IsWindowsVistaSP2OrGreater(); }
BOOL helper_is_win7() { return IsWindows7OrGreater(); }
BOOL helper_is_win7_sp1() { return IsWindows7SP1OrGreater(); }
BOOL helper_is_win8() { return IsWindows8OrGreater(); }
BOOL helper_is_win8_p1() { return IsWindows8Point1OrGreater(); }
BOOL helper_is_winthreshold() { return IsWindowsThresholdOrGreater(); }
BOOL helper_is_win10() { return IsWindows10OrGreater(); }
BOOL helper_is_winserver() { return IsWindowsServer(); }
