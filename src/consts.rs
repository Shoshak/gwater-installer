pub mod constants {
    // need to be put in garrysmod/lua/bin folder
    pub const LUA_BIN_MODULES: [&'static str; 2] =
        ["gmcl_GWater_win32.dll", "gmcl_GWater_win64.dll"];
    // need to be put directly in garrysmod folder
    pub const GARRYSMOD_MODULES: [&'static str; 14] = [
        "GFSDK_Aftermath_Lib.x86.dll",
        "GFSDK_Aftermath_Lib.x64.dll",
        "nvToolsExt32_1.dll",
        "nvToolsExt64_1.dll",
        "amd_ags_x86.dll",
        "amd_ags_x64.dll",
        "NvFlexExtReleaseD3D_x86.dll",
        "NvFlexExtReleaseD3D_x64.dll",
        "NvFlexReleaseD3D_x86.dll",
        "NvFlexReleaseD3D_x64.dll",
        "NvFlexReleaseCUDA_x86.dll",
        "NvFlexReleaseCUDA_x64.dll",
        "NvFlexExtReleaseCUDA_x86.dll",
        "NvFlexExtReleaseCUDA_x64.dll",
    ];
}
