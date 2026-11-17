//! Raw OpenVR C function-table bindings.
//!
//! Generated with bindgen 0.73 from Valve's `headers/openvr_capi.h` (OpenVR SDK master,
//! IVRSystem_026 / IVROverlay_028 / IVRInput_011 / IVRApplications_008 / IVRSettings_003)
//! using the Windows x64 ABI (`--target=x86_64-pc-windows-gnu`), because the header packs
//! several structs differently on Linux. Regenerate rather than edit by hand.

#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, dead_code, clippy::all)]

pub type PropertyContainerHandle_t = u64;
pub type PropertyTypeTag_t = u32;
pub type VRActionHandle_t = u64;
pub type VRActionSetHandle_t = u64;
pub type VRInputValueHandle_t = u64;
pub const k_nDriverNone: ::core::ffi::c_ulong = 4294967295;
pub const k_unMaxDriverDebugResponseSize: ::core::ffi::c_ulong = 32768;
pub const k_unTrackedDeviceIndex_Hmd: ::core::ffi::c_ulong = 0;
pub const k_unMaxTrackedDeviceCount: ::core::ffi::c_ulong = 64;
pub const k_unTrackedDeviceIndexOther: ::core::ffi::c_ulong = 4294967294;
pub const k_unTrackedDeviceIndexInvalid: ::core::ffi::c_ulong = 4294967295;
pub const k_ulInvalidPropertyContainer: ::core::ffi::c_ulonglong = 0;
pub const k_unInvalidPropertyTag: ::core::ffi::c_ulong = 0;
pub const k_ulInvalidDriverHandle: ::core::ffi::c_ulonglong = 0;
pub const k_unFloatPropertyTag: ::core::ffi::c_ulong = 1;
pub const k_unInt32PropertyTag: ::core::ffi::c_ulong = 2;
pub const k_unUint64PropertyTag: ::core::ffi::c_ulong = 3;
pub const k_unBoolPropertyTag: ::core::ffi::c_ulong = 4;
pub const k_unStringPropertyTag: ::core::ffi::c_ulong = 5;
pub const k_unErrorPropertyTag: ::core::ffi::c_ulong = 6;
pub const k_unDoublePropertyTag: ::core::ffi::c_ulong = 7;
pub const k_unHmdMatrix34PropertyTag: ::core::ffi::c_ulong = 20;
pub const k_unHmdMatrix44PropertyTag: ::core::ffi::c_ulong = 21;
pub const k_unHmdVector3PropertyTag: ::core::ffi::c_ulong = 22;
pub const k_unHmdVector4PropertyTag: ::core::ffi::c_ulong = 23;
pub const k_unHmdVector2PropertyTag: ::core::ffi::c_ulong = 24;
pub const k_unHmdQuadPropertyTag: ::core::ffi::c_ulong = 25;
pub const k_unHiddenAreaPropertyTag: ::core::ffi::c_ulong = 30;
pub const k_unPathHandleInfoTag: ::core::ffi::c_ulong = 31;
pub const k_unActionPropertyTag: ::core::ffi::c_ulong = 32;
pub const k_unInputValuePropertyTag: ::core::ffi::c_ulong = 33;
pub const k_unWildcardPropertyTag: ::core::ffi::c_ulong = 34;
pub const k_unHapticVibrationPropertyTag: ::core::ffi::c_ulong = 35;
pub const k_unSkeletonPropertyTag: ::core::ffi::c_ulong = 36;
pub const k_unSpatialAnchorPosePropertyTag: ::core::ffi::c_ulong = 40;
pub const k_unJsonPropertyTag: ::core::ffi::c_ulong = 41;
pub const k_unActiveActionSetPropertyTag: ::core::ffi::c_ulong = 42;
pub const k_unOpenVRInternalReserved_Start: ::core::ffi::c_ulong = 1000;
pub const k_unOpenVRInternalReserved_End: ::core::ffi::c_ulong = 10000;
pub const k_unMaxPropertyStringSize: ::core::ffi::c_ulong = 32768;
pub const k_ulInvalidActionHandle: ::core::ffi::c_ulonglong = 0;
pub const k_ulInvalidActionSetHandle: ::core::ffi::c_ulonglong = 0;
pub const k_ulInvalidInputValueHandle: ::core::ffi::c_ulonglong = 0;
pub const k_ulInvalidInputComponentHandle: ::core::ffi::c_ulonglong = 0;
pub const k_unControllerStateAxisCount: ::core::ffi::c_ulong = 5;
pub const k_ulOverlayHandleInvalid: ::core::ffi::c_ulonglong = 0;
pub const k_unMaxDistortionFunctionParameters: ::core::ffi::c_ulong = 8;
pub const k_unScreenshotHandleInvalid: ::core::ffi::c_ulong = 0;
pub const k_unMaxApplicationKeyLength: ::core::ffi::c_ulong = 128;
pub const k_pch_MimeType_HomeApp: &[u8; 8] = b"vr/home\0";
pub const k_pch_MimeType_GameTheater: &[u8; 16] = b"vr/game_theater\0";
pub const k_unVROverlayMaxKeyLength: ::core::ffi::c_ulong = 128;
pub const k_unVROverlayMaxNameLength: ::core::ffi::c_ulong = 128;
pub const k_unMaxOverlayCount: ::core::ffi::c_ulong = 128;
pub const k_unMaxOverlayIntersectionMaskPrimitivesCount: ::core::ffi::c_ulong = 32;
pub const k_unHeadsetViewMaxWidth: ::core::ffi::c_ulong = 3840;
pub const k_unHeadsetViewMaxHeight: ::core::ffi::c_ulong = 2160;
pub const k_pchHeadsetViewOverlayKey: &[u8; 19] = b"system.HeadsetView\0";
pub const k_pch_Controller_Component_GDC2015: &[u8; 8] = b"gdc2015\0";
pub const k_pch_Controller_Component_Base: &[u8; 5] = b"base\0";
pub const k_pch_Controller_Component_Tip: &[u8; 4] = b"tip\0";
pub const k_pch_Controller_Component_OpenXR_Aim: &[u8; 11] = b"openxr_aim\0";
pub const k_pch_Controller_Component_HandGrip: &[u8; 9] = b"handgrip\0";
pub const k_pch_Controller_Component_OpenXR_Grip: &[u8; 12] = b"openxr_grip\0";
pub const k_pch_Controller_Component_OpenXR_HandModel: &[u8; 17] = b"openxr_handmodel\0";
pub const k_pch_Controller_Component_Status: &[u8; 7] = b"status\0";
pub const k_unNotificationTextMaxSize: ::core::ffi::c_ulong = 256;
pub const k_unMaxSettingsKeyLength: ::core::ffi::c_ulong = 128;
pub const k_pch_SteamVR_Section: &[u8; 8] = b"steamvr\0";
pub const k_pch_SteamVR_Contrast_Float: &[u8; 9] = b"contrast\0";
pub const k_pch_SteamVR_RequireHmd_String: &[u8; 11] = b"requireHmd\0";
pub const k_pch_SteamVR_ForcedDriverKey_String: &[u8; 13] = b"forcedDriver\0";
pub const k_pch_SteamVR_ForcedHmdKey_String: &[u8; 10] = b"forcedHmd\0";
pub const k_pch_SteamVR_DisplayDebug_Bool: &[u8; 13] = b"displayDebug\0";
pub const k_pch_SteamVR_DebugProcessPipe_String: &[u8; 17] = b"debugProcessPipe\0";
pub const k_pch_SteamVR_DisplayDebugX_Int32: &[u8; 14] = b"displayDebugX\0";
pub const k_pch_SteamVR_DisplayDebugY_Int32: &[u8; 14] = b"displayDebugY\0";
pub const k_pch_SteamVR_SendSystemButtonToAllApps_Bool: &[u8; 26] = b"sendSystemButtonToAllApps\0";
pub const k_pch_SteamVR_LogLevel_Int32: &[u8; 9] = b"loglevel\0";
pub const k_pch_SteamVR_IPD_Float: &[u8; 4] = b"ipd\0";
pub const k_pch_SteamVR_Background_String: &[u8; 11] = b"background\0";
pub const k_pch_SteamVR_BackgroundUseDomeProjection_Bool: &[u8; 28] =
    b"backgroundUseDomeProjection\0";
pub const k_pch_SteamVR_BackgroundCameraHeight_Float: &[u8; 23] = b"backgroundCameraHeight\0";
pub const k_pch_SteamVR_BackgroundDomeRadius_Float: &[u8; 21] = b"backgroundDomeRadius\0";
pub const k_pch_SteamVR_GridColor_String: &[u8; 10] = b"gridColor\0";
pub const k_pch_SteamVR_PlayAreaColor_String: &[u8; 14] = b"playAreaColor\0";
pub const k_pch_SteamVR_TrackingLossColor_String: &[u8; 18] = b"trackingLossColor\0";
pub const k_pch_SteamVR_StartColor_String: &[u8; 11] = b"startColor\0";
pub const k_pch_SteamVR_ShowStage_Bool: &[u8; 10] = b"showStage\0";
pub const k_pch_SteamVR_DrawTrackingReferences_Bool: &[u8; 23] = b"drawTrackingReferences\0";
pub const k_pch_SteamVR_ActivateMultipleDrivers_Bool: &[u8; 24] = b"activateMultipleDrivers\0";
pub const k_pch_SteamVR_UsingSpeakers_Bool: &[u8; 14] = b"usingSpeakers\0";
pub const k_pch_SteamVR_SpeakersForwardYawOffsetDegrees_Float: &[u8; 32] =
    b"speakersForwardYawOffsetDegrees\0";
pub const k_pch_SteamVR_BaseStationPowerManagement_Int32: &[u8; 27] =
    b"basestationPowerManagement\0";
pub const k_pch_SteamVR_ShowBaseStationPowerManagementTip_Int32: &[u8; 34] =
    b"ShowBaseStationPowerManagementTip\0";
pub const k_pch_SteamVR_NeverKillProcesses_Bool: &[u8; 19] = b"neverKillProcesses\0";
pub const k_pch_SteamVR_SupersampleScale_Float: &[u8; 17] = b"supersampleScale\0";
pub const k_pch_SteamVR_MaxRecommendedResolution_Int32: &[u8; 25] = b"maxRecommendedResolution\0";
pub const k_pch_SteamVR_MotionSmoothing_Bool: &[u8; 16] = b"motionSmoothing\0";
pub const k_pch_SteamVR_MotionSmoothingOverride_Int32: &[u8; 24] = b"motionSmoothingOverride\0";
pub const k_pch_SteamVR_FoveatedSharpening_Bool: &[u8; 11] = b"sharpening\0";
pub const k_pch_SteamVR_FoveatedSharpeningOverride_Int32: &[u8; 19] = b"sharpeningOverride\0";
pub const k_pch_SteamVR_FramesToThrottle_Int32: &[u8; 17] = b"framesToThrottle\0";
pub const k_pch_SteamVR_AdditionalFramesToPredict_Int32: &[u8; 26] = b"additionalFramesToPredict\0";
pub const k_pch_SteamVR_WorldScale_Float: &[u8; 11] = b"worldScale\0";
pub const k_pch_SteamVR_FovScale_Int32: &[u8; 9] = b"fovScale\0";
pub const k_pch_SteamVR_FovScaleInner_Int32: &[u8; 14] = b"fovScaleInner\0";
pub const k_pch_SteamVR_FovScaleUpper_Int32: &[u8; 14] = b"fovScaleUpper\0";
pub const k_pch_SteamVR_FovScaleLower_Int32: &[u8; 14] = b"fovScaleLower\0";
pub const k_pch_SteamVR_FovScaleFormat_Int32: &[u8; 15] = b"fovScaleFormat\0";
pub const k_pch_SteamVR_FovScaleLetterboxed_Bool: &[u8; 20] = b"fovScaleLetterboxed\0";
pub const k_pch_SteamVR_DisableAsyncReprojection_Bool: &[u8; 13] = b"disableAsync\0";
pub const k_pch_SteamVR_ForceFadeOnBadTracking_Bool: &[u8; 23] = b"forceFadeOnBadTracking\0";
pub const k_pch_SteamVR_DefaultMirrorView_Int32: &[u8; 11] = b"mirrorView\0";
pub const k_pch_SteamVR_ShowLegacyMirrorView_Bool: &[u8; 21] = b"showLegacyMirrorView\0";
pub const k_pch_SteamVR_MirrorViewVisibility_Bool: &[u8; 15] = b"showMirrorView\0";
pub const k_pch_SteamVR_MirrorViewDisplayMode_Int32: &[u8; 22] = b"mirrorViewDisplayMode\0";
pub const k_pch_SteamVR_MirrorViewEye_Int32: &[u8; 14] = b"mirrorViewEye\0";
pub const k_pch_SteamVR_MirrorViewGeometry_String: &[u8; 19] = b"mirrorViewGeometry\0";
pub const k_pch_SteamVR_MirrorViewGeometryMaximized_String: &[u8; 28] =
    b"mirrorViewGeometryMaximized\0";
pub const k_pch_SteamVR_PerfGraphVisibility_Bool: &[u8; 14] = b"showPerfGraph\0";
pub const k_pch_SteamVR_StartMonitorFromAppLaunch: &[u8; 26] = b"startMonitorFromAppLaunch\0";
pub const k_pch_SteamVR_StartCompositorFromAppLaunch_Bool: &[u8; 29] =
    b"startCompositorFromAppLaunch\0";
pub const k_pch_SteamVR_StartDashboardFromAppLaunch_Bool: &[u8; 28] =
    b"startDashboardFromAppLaunch\0";
pub const k_pch_SteamVR_StartOverlayAppsFromDashboard_Bool: &[u8; 30] =
    b"startOverlayAppsFromDashboard\0";
pub const k_pch_SteamVR_EnableHomeApp: &[u8; 14] = b"enableHomeApp\0";
pub const k_pch_SteamVR_CycleBackgroundImageTimeSec_Int32: &[u8; 28] =
    b"CycleBackgroundImageTimeSec\0";
pub const k_pch_SteamVR_RetailDemo_Bool: &[u8; 11] = b"retailDemo\0";
pub const k_pch_SteamVR_IpdOffset_Float: &[u8; 10] = b"ipdOffset\0";
pub const k_pch_SteamVR_AllowSupersampleFiltering_Bool: &[u8; 26] = b"allowSupersampleFiltering\0";
pub const k_pch_SteamVR_SupersampleManualOverride_Bool: &[u8; 26] = b"supersampleManualOverride\0";
pub const k_pch_SteamVR_EnableLinuxVulkanAsync_Bool: &[u8; 23] = b"enableLinuxVulkanAsync\0";
pub const k_pch_SteamVR_AllowDisplayLockedMode_Bool: &[u8; 23] = b"allowDisplayLockedMode\0";
pub const k_pch_SteamVR_HaveStartedTutorialForNativeChaperoneDriver_Bool: &[u8; 44] =
    b"haveStartedTutorialForNativeChaperoneDriver\0";
pub const k_pch_SteamVR_DebugInputBinding: &[u8; 18] = b"debugInputBinding\0";
pub const k_pch_SteamVR_DoNotFadeToGrid: &[u8; 16] = b"doNotFadeToGrid\0";
pub const k_pch_SteamVR_EnableSharedResourceJournaling: &[u8; 31] =
    b"enableSharedResourceJournaling\0";
pub const k_pch_SteamVR_EnableSafeMode: &[u8; 15] = b"enableSafeMode\0";
pub const k_pch_SteamVR_PreferredRefreshRate: &[u8; 21] = b"preferredRefreshRate\0";
pub const k_pch_SteamVR_LastVersionNotice: &[u8; 18] = b"lastVersionNotice\0";
pub const k_pch_SteamVR_LastVersionNoticeDate: &[u8; 22] = b"lastVersionNoticeDate\0";
pub const k_pch_SteamVR_HmdDisplayColorGainR_Float: &[u8; 21] = b"hmdDisplayColorGainR\0";
pub const k_pch_SteamVR_HmdDisplayColorGainG_Float: &[u8; 21] = b"hmdDisplayColorGainG\0";
pub const k_pch_SteamVR_HmdDisplayColorGainB_Float: &[u8; 21] = b"hmdDisplayColorGainB\0";
pub const k_pch_SteamVR_CustomIconStyle_String: &[u8; 16] = b"customIconStyle\0";
pub const k_pch_SteamVR_CustomOffIconStyle_String: &[u8; 19] = b"customOffIconStyle\0";
pub const k_pch_SteamVR_CustomIconForceUpdate_String: &[u8; 22] = b"customIconForceUpdate\0";
pub const k_pch_SteamVR_AllowGlobalActionSetPriority: &[u8; 24] = b"globalActionSetPriority\0";
pub const k_pch_SteamVR_OverlayRenderQuality: &[u8; 23] = b"overlayRenderQuality_2\0";
pub const k_pch_SteamVR_BlockOculusSDKOnOpenVRLaunchOption_Bool: &[u8; 35] =
    b"blockOculusSDKOnOpenVRLaunchOption\0";
pub const k_pch_SteamVR_BlockOculusSDKOnAllLaunches_Bool: &[u8; 28] =
    b"blockOculusSDKOnAllLaunches\0";
pub const k_pch_SteamVR_HDCPLegacyCompatibility_Bool: &[u8; 26] = b"hdcp14legacyCompatibility\0";
pub const k_pch_SteamVR_DisplayPortTrainingMode_Int: &[u8; 24] = b"displayPortTrainingMode\0";
pub const k_pch_SteamVR_UsePrism_Bool: &[u8; 9] = b"usePrism\0";
pub const k_pch_SteamVR_AllowFallbackMirrorWindowLinux_Bool: &[u8; 31] =
    b"allowFallbackMirrorWindowLinux\0";
pub const k_pch_SteamVR_DisableKeyboardPrivacy_Bool: &[u8; 23] = b"disableKeyboardPrivacy\0";
pub const k_pch_OpenXR_Section: &[u8; 7] = b"openxr\0";
pub const k_pch_OpenXR_MetaUnityPluginCompatibility_Int32: &[u8; 29] =
    b"metaUnityPluginCompatibility\0";
pub const k_pch_DirectMode_Section: &[u8; 12] = b"direct_mode\0";
pub const k_pch_DirectMode_Enable_Bool: &[u8; 7] = b"enable\0";
pub const k_pch_DirectMode_Count_Int32: &[u8; 6] = b"count\0";
pub const k_pch_DirectMode_EdidVid_Int32: &[u8; 8] = b"edidVid\0";
pub const k_pch_DirectMode_EdidPid_Int32: &[u8; 8] = b"edidPid\0";
pub const k_pch_Lighthouse_Section: &[u8; 18] = b"driver_lighthouse\0";
pub const k_pch_Lighthouse_DisableIMU_Bool: &[u8; 11] = b"disableimu\0";
pub const k_pch_Lighthouse_DisableIMUExceptHMD_Bool: &[u8; 20] = b"disableimuexcepthmd\0";
pub const k_pch_Lighthouse_UseDisambiguation_String: &[u8; 18] = b"usedisambiguation\0";
pub const k_pch_Lighthouse_DisambiguationDebug_Int32: &[u8; 20] = b"disambiguationdebug\0";
pub const k_pch_Lighthouse_PrimaryBasestation_Int32: &[u8; 19] = b"primarybasestation\0";
pub const k_pch_Lighthouse_DBHistory_Bool: &[u8; 10] = b"dbhistory\0";
pub const k_pch_Lighthouse_EnableBluetooth_Bool: &[u8; 16] = b"enableBluetooth\0";
pub const k_pch_Lighthouse_PowerManagedBaseStations_String: &[u8; 25] =
    b"PowerManagedBaseStations\0";
pub const k_pch_Lighthouse_PowerManagedBaseStations2_String: &[u8; 26] =
    b"PowerManagedBaseStations2\0";
pub const k_pch_Lighthouse_InactivityTimeoutForBaseStations_Int32: &[u8; 33] =
    b"InactivityTimeoutForBaseStations\0";
pub const k_pch_Lighthouse_EnableImuFallback_Bool: &[u8; 18] = b"enableImuFallback\0";
pub const k_pch_Null_Section: &[u8; 12] = b"driver_null\0";
pub const k_pch_Null_SerialNumber_String: &[u8; 13] = b"serialNumber\0";
pub const k_pch_Null_ModelNumber_String: &[u8; 12] = b"modelNumber\0";
pub const k_pch_Null_WindowX_Int32: &[u8; 8] = b"windowX\0";
pub const k_pch_Null_WindowY_Int32: &[u8; 8] = b"windowY\0";
pub const k_pch_Null_WindowWidth_Int32: &[u8; 12] = b"windowWidth\0";
pub const k_pch_Null_WindowHeight_Int32: &[u8; 13] = b"windowHeight\0";
pub const k_pch_Null_RenderWidth_Int32: &[u8; 12] = b"renderWidth\0";
pub const k_pch_Null_RenderHeight_Int32: &[u8; 13] = b"renderHeight\0";
pub const k_pch_Null_SecondsFromVsyncToPhotons_Float: &[u8; 26] = b"secondsFromVsyncToPhotons\0";
pub const k_pch_Null_DisplayFrequency_Float: &[u8; 17] = b"displayFrequency\0";
pub const k_pch_WindowsMR_Section: &[u8; 19] = b"driver_holographic\0";
pub const k_pch_UserInterface_Section: &[u8; 14] = b"userinterface\0";
pub const k_pch_UserInterface_StatusAlwaysOnTop_Bool: &[u8; 18] = b"StatusAlwaysOnTop\0";
pub const k_pch_UserInterface_MinimizeToTray_Bool: &[u8; 15] = b"MinimizeToTray\0";
pub const k_pch_UserInterface_HidePopupsWhenStatusMinimized_Bool: &[u8; 30] =
    b"HidePopupsWhenStatusMinimized\0";
pub const k_pch_UserInterface_Screenshots_Bool: &[u8; 12] = b"screenshots\0";
pub const k_pch_UserInterface_ScreenshotType_Int: &[u8; 15] = b"screenshotType\0";
pub const k_pch_UserInterface_CheckStatusInterval_Int: &[u8; 23] = b"vrmStatusCheckInterval\0";
pub const k_pch_UserInterface_CheckForSteam_Bool: &[u8; 17] = b"vrmCheckForSteam\0";
pub const k_pch_Notifications_Section: &[u8; 14] = b"notifications\0";
pub const k_pch_Notifications_DoNotDisturb_Bool: &[u8; 13] = b"DoNotDisturb\0";
pub const k_pch_Keyboard_Section: &[u8; 9] = b"keyboard\0";
pub const k_pch_Keyboard_TutorialCompletions: &[u8; 20] = b"TutorialCompletions\0";
pub const k_pch_Keyboard_ScaleX: &[u8; 7] = b"ScaleX\0";
pub const k_pch_Keyboard_ScaleY: &[u8; 7] = b"ScaleY\0";
pub const k_pch_Keyboard_OffsetLeftX: &[u8; 12] = b"OffsetLeftX\0";
pub const k_pch_Keyboard_OffsetRightX: &[u8; 13] = b"OffsetRightX\0";
pub const k_pch_Keyboard_OffsetY: &[u8; 8] = b"OffsetY\0";
pub const k_pch_Keyboard_Smoothing: &[u8; 10] = b"Smoothing\0";
pub const k_pch_Perf_Section: &[u8; 10] = b"perfcheck\0";
pub const k_pch_Perf_PerfGraphInHMD_Bool: &[u8; 15] = b"perfGraphInHMD\0";
pub const k_pch_Perf_AllowTimingStore_Bool: &[u8; 17] = b"allowTimingStore\0";
pub const k_pch_Perf_SaveTimingsOnExit_Bool: &[u8; 18] = b"saveTimingsOnExit\0";
pub const k_pch_Perf_TestData_Float: &[u8; 13] = b"perfTestData\0";
pub const k_pch_Perf_GPUProfiling_Bool: &[u8; 13] = b"GPUProfiling\0";
pub const k_pch_Perf_GpuBusMonitoring_Bool: &[u8; 17] = b"gpuBusMonitoring\0";
pub const k_pch_CollisionBounds_Section: &[u8; 16] = b"collisionBounds\0";
pub const k_pch_CollisionBounds_Style_Int32: &[u8; 21] = b"CollisionBoundsStyle\0";
pub const k_pch_CollisionBounds_GroundPerimeterOn_Bool: &[u8; 33] =
    b"CollisionBoundsGroundPerimeterOn\0";
pub const k_pch_CollisionBounds_CenterMarkerOn_Bool: &[u8; 30] = b"CollisionBoundsCenterMarkerOn\0";
pub const k_pch_CollisionBounds_PlaySpaceOn_Bool: &[u8; 27] = b"CollisionBoundsPlaySpaceOn\0";
pub const k_pch_CollisionBounds_FadeDistance_Float: &[u8; 28] = b"CollisionBoundsFadeDistance\0";
pub const k_pch_CollisionBounds_WallHeight_Float: &[u8; 26] = b"CollisionBoundsWallHeight\0";
pub const k_pch_CollisionBounds_ColorGammaR_Int32: &[u8; 27] = b"CollisionBoundsColorGammaR\0";
pub const k_pch_CollisionBounds_ColorGammaG_Int32: &[u8; 27] = b"CollisionBoundsColorGammaG\0";
pub const k_pch_CollisionBounds_ColorGammaB_Int32: &[u8; 27] = b"CollisionBoundsColorGammaB\0";
pub const k_pch_CollisionBounds_ColorGammaA_Int32: &[u8; 27] = b"CollisionBoundsColorGammaA\0";
pub const k_pch_CollisionBounds_EnableDriverImport: &[u8; 25] = b"enableDriverBoundsImport\0";
pub const k_pch_Camera_Section: &[u8; 7] = b"camera\0";
pub const k_pch_Camera_EnableCamera_Bool: &[u8; 13] = b"enableCamera\0";
pub const k_pch_Camera_ShowOnController_Bool: &[u8; 17] = b"showOnController\0";
pub const k_pch_Camera_EnableCameraForCollisionBounds_Bool: &[u8; 31] =
    b"enableCameraForCollisionBounds\0";
pub const k_pch_Camera_RoomView_Int32: &[u8; 9] = b"roomView\0";
pub const k_pch_Camera_BoundsColorGammaR_Int32: &[u8; 24] = b"cameraBoundsColorGammaR\0";
pub const k_pch_Camera_BoundsColorGammaG_Int32: &[u8; 24] = b"cameraBoundsColorGammaG\0";
pub const k_pch_Camera_BoundsColorGammaB_Int32: &[u8; 24] = b"cameraBoundsColorGammaB\0";
pub const k_pch_Camera_BoundsColorGammaA_Int32: &[u8; 24] = b"cameraBoundsColorGammaA\0";
pub const k_pch_Camera_BoundsStrength_Int32: &[u8; 21] = b"cameraBoundsStrength\0";
pub const k_pch_Camera_RoomViewStyle_Int32: &[u8; 14] = b"roomViewStyle\0";
pub const k_pch_audio_Section: &[u8; 6] = b"audio\0";
pub const k_pch_audio_SetOsDefaultPlaybackDevice_Bool: &[u8; 27] = b"setOsDefaultPlaybackDevice\0";
pub const k_pch_audio_EnablePlaybackDeviceOverride_Bool: &[u8; 29] =
    b"enablePlaybackDeviceOverride\0";
pub const k_pch_audio_PlaybackDeviceOverride_String: &[u8; 23] = b"playbackDeviceOverride\0";
pub const k_pch_audio_PlaybackDeviceOverrideName_String: &[u8; 27] =
    b"playbackDeviceOverrideName\0";
pub const k_pch_audio_SetOsDefaultRecordingDevice_Bool: &[u8; 28] =
    b"setOsDefaultRecordingDevice\0";
pub const k_pch_audio_EnableRecordingDeviceOverride_Bool: &[u8; 30] =
    b"enableRecordingDeviceOverride\0";
pub const k_pch_audio_RecordingDeviceOverride_String: &[u8; 24] = b"recordingDeviceOverride\0";
pub const k_pch_audio_RecordingDeviceOverrideName_String: &[u8; 28] =
    b"recordingDeviceOverrideName\0";
pub const k_pch_audio_EnablePlaybackMirror_Bool: &[u8; 21] = b"enablePlaybackMirror\0";
pub const k_pch_audio_PlaybackMirrorDevice_String: &[u8; 21] = b"playbackMirrorDevice\0";
pub const k_pch_audio_PlaybackMirrorDeviceName_String: &[u8; 25] = b"playbackMirrorDeviceName\0";
pub const k_pch_audio_OldPlaybackMirrorDevice_String: &[u8; 23] = b"onPlaybackMirrorDevice\0";
pub const k_pch_audio_ActiveMirrorDevice_String: &[u8; 27] = b"activePlaybackMirrorDevice\0";
pub const k_pch_audio_EnablePlaybackMirrorIndependentVolume_Bool: &[u8; 38] =
    b"enablePlaybackMirrorIndependentVolume\0";
pub const k_pch_audio_LastHmdPlaybackDeviceId_String: &[u8; 24] = b"lastHmdPlaybackDeviceId\0";
pub const k_pch_audio_VIVEHDMIGain: &[u8; 13] = b"viveHDMIGain\0";
pub const k_pch_audio_DualSpeakerAndJackOutput_Bool: &[u8; 25] = b"dualSpeakerAndJackOutput\0";
pub const k_pch_audio_MuteMicMonitor_Bool: &[u8; 15] = b"muteMicMonitor\0";
pub const k_pch_Power_Section: &[u8; 6] = b"power\0";
pub const k_pch_Power_PowerOffOnExit_Bool: &[u8; 15] = b"powerOffOnExit\0";
pub const k_pch_Power_TurnOffScreensTimeout_Float: &[u8; 22] = b"turnOffScreensTimeout\0";
pub const k_pch_Power_TurnOffControllersTimeout_Float: &[u8; 26] = b"turnOffControllersTimeout\0";
pub const k_pch_Power_ReturnToWatchdogTimeout_Float: &[u8; 24] = b"returnToWatchdogTimeout\0";
pub const k_pch_Power_AutoLaunchSteamVROnButtonPress: &[u8; 31] =
    b"autoLaunchSteamVROnButtonPress\0";
pub const k_pch_Power_PauseCompositorOnStandby_Bool: &[u8; 25] = b"pauseCompositorOnStandby\0";
pub const k_pch_Power_OverrideWindowsPowerScheme_Bool: &[u8; 27] = b"overrideWindowsPowerScheme\0";
pub const k_pch_Dashboard_Section: &[u8; 10] = b"dashboard\0";
pub const k_pch_Dashboard_EnableDashboard_Bool: &[u8; 16] = b"enableDashboard\0";
pub const k_pch_Dashboard_ArcadeMode_Bool: &[u8; 11] = b"arcadeMode\0";
pub const k_pch_Dashboard_Position: &[u8; 9] = b"position\0";
pub const k_pch_Dashboard_DashboardScale: &[u8; 15] = b"dashboardScale\0";
pub const k_pch_Dashboard_UseStandaloneSystemLayer: &[u8; 22] = b"standaloneSystemLayer\0";
pub const k_pch_Dashboard_AllowSteamOverlays_Bool: &[u8; 19] = b"allowSteamOverlays\0";
pub const k_pch_Dashboard_AllowVRGamepadUI_Bool: &[u8; 17] = b"allowVRGamepadUI\0";
pub const k_pch_Dashboard_SteamMatchesHMDFramerate: &[u8; 25] = b"steamMatchesHMDFramerate\0";
pub const k_pch_Dashboard_GrabHandleAcceleration: &[u8; 23] = b"grabHandleAcceleration\0";
pub const k_pch_Dashboard_OverlayBacksideColor_String: &[u8; 21] = b"overlayBacksideColor\0";
pub const k_pch_modelskin_Section: &[u8; 11] = b"modelskins\0";
pub const k_pch_Driver_Enable_Bool: &[u8; 7] = b"enable\0";
pub const k_pch_Driver_BlockedBySafemode_Bool: &[u8; 21] = b"blocked_by_safe_mode\0";
pub const k_pch_Driver_LoadPriority_Int32: &[u8; 13] = b"loadPriority\0";
pub const k_pch_Driver_Hmd_AllowsClientToControlTextureIndex_Bool: &[u8; 37] =
    b"hmdAllowsClientToControlTextureIndex\0";
pub const k_pch_Driver_ForceSystemLayerUseAppPoses_Bool: &[u8; 28] =
    b"forceSystemLayerUseAppPoses\0";
pub const k_pch_WebInterface_Section: &[u8; 13] = b"WebInterface\0";
pub const k_pch_VRWebHelper_Section: &[u8; 12] = b"VRWebHelper\0";
pub const k_pch_VRWebHelper_DebuggerEnabled_Bool: &[u8; 16] = b"DebuggerEnabled\0";
pub const k_pch_VRWebHelper_DebuggerPort_Int32: &[u8; 13] = b"DebuggerPort\0";
pub const k_pch_TrackingOverride_Section: &[u8; 18] = b"TrackingOverrides\0";
pub const k_pch_App_BindingAutosaveURLSuffix_String: &[u8; 12] = b"AutosaveURL\0";
pub const k_pch_App_BindingLegacyAPISuffix_String: &[u8; 8] = b"_legacy\0";
pub const k_pch_App_BindingSteamVRInputAPISuffix_String: &[u8; 14] = b"_steamvrinput\0";
pub const k_pch_App_BindingOpenXRAPISuffix_String: &[u8; 8] = b"_openxr\0";
pub const k_pch_App_BindingCurrentURLSuffix_String: &[u8; 11] = b"CurrentURL\0";
pub const k_pch_App_BindingPreviousURLSuffix_String: &[u8; 12] = b"PreviousURL\0";
pub const k_pch_App_NeedToUpdateAutosaveSuffix_Bool: &[u8; 21] = b"NeedToUpdateAutosave\0";
pub const k_pch_App_DominantHand_Int32: &[u8; 13] = b"DominantHand\0";
pub const k_pch_App_BlockOculusSDK_Bool: &[u8; 15] = b"blockOculusSDK\0";
pub const k_pch_Trackers_Section: &[u8; 9] = b"trackers\0";
pub const k_pch_DesktopUI_Section: &[u8; 10] = b"DesktopUI\0";
pub const k_pch_LastKnown_Section: &[u8; 10] = b"LastKnown\0";
pub const k_pch_LastKnown_HMDManufacturer_String: &[u8; 16] = b"HMDManufacturer\0";
pub const k_pch_LastKnown_HMDModel_String: &[u8; 9] = b"HMDModel\0";
pub const k_pch_LastKnown_ActualHMDDriver_String: &[u8; 16] = b"ActualHMDDriver\0";
pub const k_pch_LastKnown_HMDSerialNumber_String: &[u8; 16] = b"HMDSerialNumber\0";
pub const k_pch_LastKnown_HMDRemoteClientID_String: &[u8; 15] = b"RemoteClientID\0";
pub const k_pch_DismissedWarnings_Section: &[u8; 18] = b"DismissedWarnings\0";
pub const k_pch_Input_Section: &[u8; 6] = b"input\0";
pub const k_pch_Input_LeftThumbstickRotation_Float: &[u8; 23] = b"leftThumbstickRotation\0";
pub const k_pch_Input_RightThumbstickRotation_Float: &[u8; 24] = b"rightThumbstickRotation\0";
pub const k_pch_Input_ThumbstickDeadzone_Float: &[u8; 19] = b"thumbstickDeadzone\0";
pub const k_pch_GpuSpeed_Section: &[u8; 9] = b"GpuSpeed\0";
pub const k_pch_XRRenderModelCache_Section: &[u8; 23] = b"XRRenderModelUuidCache\0";
pub const k_unMaxActionNameLength: ::core::ffi::c_ulong = 64;
pub const k_unMaxActionSetNameLength: ::core::ffi::c_ulong = 64;
pub const k_unMaxActionOriginCount: ::core::ffi::c_ulong = 16;
pub const k_unMaxBoneNameLength: ::core::ffi::c_ulong = 32;
pub const k_nActionSetOverlayGlobalPriorityMin: ::core::ffi::c_int = 16777216;
pub const k_nActionSetOverlayGlobalPriorityMax: ::core::ffi::c_int = 33554431;
pub const k_nActionSetPriorityReservedMin: ::core::ffi::c_int = 33554432;
pub const k_ulInvalidIOBufferHandle: ::core::ffi::c_ulonglong = 0;
pub const k_ulInvalidSpatialAnchorHandle: ::core::ffi::c_ulong = 0;
pub const k_nSteamVRVersionMajor: ::core::ffi::c_ulong = 2;
pub const k_nSteamVRVersionMinor: ::core::ffi::c_ulong = 15;
pub const k_nSteamVRVersionBuild: ::core::ffi::c_ulong = 6;
pub const k_ulDisplayRedirectContainer: ::core::ffi::c_ulonglong = 25769803779;
pub const k_pchPathUserHandRight: &[u8; 17] = b"/user/hand/right\0";
pub const k_pchPathUserHandLeft: &[u8; 16] = b"/user/hand/left\0";
pub const k_pchPathUserHandPrimary: &[u8; 19] = b"/user/hand/primary\0";
pub const k_pchPathUserHandSecondary: &[u8; 21] = b"/user/hand/secondary\0";
pub const k_pchPathUserHead: &[u8; 11] = b"/user/head\0";
pub const k_pchPathUserGamepad: &[u8; 14] = b"/user/gamepad\0";
pub const k_pchPathUserTreadmill: &[u8; 16] = b"/user/treadmill\0";
pub const k_pchPathUserStylus: &[u8; 13] = b"/user/stylus\0";
pub const k_pchPathDevices: &[u8; 9] = b"/devices\0";
pub const k_pchPathDevicePath: &[u8; 13] = b"/device_path\0";
pub const k_pchPathBestAliasPath: &[u8; 17] = b"/best_alias_path\0";
pub const k_pchPathBoundTrackerAliasPath: &[u8; 20] = b"/bound_tracker_path\0";
pub const k_pchPathBoundTrackerRole: &[u8; 20] = b"/bound_tracker_role\0";
pub const k_pchPathPoseRaw: &[u8; 10] = b"/pose/raw\0";
pub const k_pchPathPoseTip: &[u8; 10] = b"/pose/tip\0";
pub const k_pchPathPoseGrip: &[u8; 11] = b"/pose/grip\0";
pub const k_pchPathSystemButtonClick: &[u8; 20] = b"/input/system/click\0";
pub const k_pchPathProximity: &[u8; 11] = b"/proximity\0";
pub const k_pchPathControllerTypePrefix: &[u8; 18] = b"/controller_type/\0";
pub const k_pchPathInputProfileSuffix: &[u8; 15] = b"/input_profile\0";
pub const k_pchPathBindingNameSuffix: &[u8; 14] = b"/binding_name\0";
pub const k_pchPathBindingUrlSuffix: &[u8; 13] = b"/binding_url\0";
pub const k_pchPathBindingErrorSuffix: &[u8; 15] = b"/binding_error\0";
pub const k_pchPathActiveActionSets: &[u8; 20] = b"/active_action_sets\0";
pub const k_pchPathComponentUpdates: &[u8; 25] = b"/total_component_updates\0";
pub const k_pchPathUserFootLeft: &[u8; 16] = b"/user/foot/left\0";
pub const k_pchPathUserFootRight: &[u8; 17] = b"/user/foot/right\0";
pub const k_pchPathUserShoulderLeft: &[u8; 20] = b"/user/shoulder/left\0";
pub const k_pchPathUserShoulderRight: &[u8; 21] = b"/user/shoulder/right\0";
pub const k_pchPathUserElbowLeft: &[u8; 17] = b"/user/elbow/left\0";
pub const k_pchPathUserElbowRight: &[u8; 18] = b"/user/elbow/right\0";
pub const k_pchPathUserKneeLeft: &[u8; 16] = b"/user/knee/left\0";
pub const k_pchPathUserKneeRight: &[u8; 17] = b"/user/knee/right\0";
pub const k_pchPathUserWristLeft: &[u8; 17] = b"/user/wrist/left\0";
pub const k_pchPathUserWristRight: &[u8; 18] = b"/user/wrist/right\0";
pub const k_pchPathUserAnkleLeft: &[u8; 17] = b"/user/ankle/left\0";
pub const k_pchPathUserAnkleRight: &[u8; 18] = b"/user/ankle/right\0";
pub const k_pchPathUserWaist: &[u8; 12] = b"/user/waist\0";
pub const k_pchPathUserChest: &[u8; 12] = b"/user/chest\0";
pub const k_pchPathUserCamera: &[u8; 13] = b"/user/camera\0";
pub const k_pchPathUserKeyboard: &[u8; 15] = b"/user/keyboard\0";
pub const k_pchPathClientAppKey: &[u8; 21] = b"/client_info/app_key\0";
pub const k_ulInvalidPathHandle: ::core::ffi::c_ulonglong = 0;
pub const EVREye_Eye_Left: EVREye = 0;
pub const EVREye_Eye_Right: EVREye = 1;
pub type EVREye = ::core::ffi::c_uint;
pub const ETextureType_TextureType_Invalid: ETextureType = -1;
pub const ETextureType_TextureType_DirectX: ETextureType = 0;
pub const ETextureType_TextureType_OpenGL: ETextureType = 1;
pub const ETextureType_TextureType_Vulkan: ETextureType = 2;
pub const ETextureType_TextureType_IOSurface: ETextureType = 3;
pub const ETextureType_TextureType_DirectX12: ETextureType = 4;
pub const ETextureType_TextureType_DXGISharedHandle: ETextureType = 5;
pub const ETextureType_TextureType_Metal: ETextureType = 6;
pub const ETextureType_TextureType_Reserved: ETextureType = 7;
pub const ETextureType_TextureType_SharedTextureHandle: ETextureType = 8;
pub type ETextureType = ::core::ffi::c_int;
pub const EColorSpace_ColorSpace_Auto: EColorSpace = 0;
pub const EColorSpace_ColorSpace_Gamma: EColorSpace = 1;
pub const EColorSpace_ColorSpace_Linear: EColorSpace = 2;
pub type EColorSpace = ::core::ffi::c_uint;
pub const ETrackingResult_TrackingResult_Uninitialized: ETrackingResult = 1;
pub const ETrackingResult_TrackingResult_Calibrating_InProgress: ETrackingResult = 100;
pub const ETrackingResult_TrackingResult_Calibrating_OutOfRange: ETrackingResult = 101;
pub const ETrackingResult_TrackingResult_Running_OK: ETrackingResult = 200;
pub const ETrackingResult_TrackingResult_Running_OutOfRange: ETrackingResult = 201;
pub const ETrackingResult_TrackingResult_Fallback_RotationOnly: ETrackingResult = 300;
pub type ETrackingResult = ::core::ffi::c_uint;
pub const ETrackedDeviceClass_TrackedDeviceClass_Invalid: ETrackedDeviceClass = 0;
pub const ETrackedDeviceClass_TrackedDeviceClass_HMD: ETrackedDeviceClass = 1;
pub const ETrackedDeviceClass_TrackedDeviceClass_Controller: ETrackedDeviceClass = 2;
pub const ETrackedDeviceClass_TrackedDeviceClass_GenericTracker: ETrackedDeviceClass = 3;
pub const ETrackedDeviceClass_TrackedDeviceClass_TrackingReference: ETrackedDeviceClass = 4;
pub const ETrackedDeviceClass_TrackedDeviceClass_DisplayRedirect: ETrackedDeviceClass = 5;
pub const ETrackedDeviceClass_TrackedDeviceClass_Max: ETrackedDeviceClass = 6;
pub type ETrackedDeviceClass = ::core::ffi::c_uint;
pub const ETrackedControllerRole_TrackedControllerRole_Invalid: ETrackedControllerRole = 0;
pub const ETrackedControllerRole_TrackedControllerRole_LeftHand: ETrackedControllerRole = 1;
pub const ETrackedControllerRole_TrackedControllerRole_RightHand: ETrackedControllerRole = 2;
pub const ETrackedControllerRole_TrackedControllerRole_OptOut: ETrackedControllerRole = 3;
pub const ETrackedControllerRole_TrackedControllerRole_Treadmill: ETrackedControllerRole = 4;
pub const ETrackedControllerRole_TrackedControllerRole_Stylus: ETrackedControllerRole = 5;
pub const ETrackedControllerRole_TrackedControllerRole_Max: ETrackedControllerRole = 5;
pub type ETrackedControllerRole = ::core::ffi::c_uint;
pub const ETrackingUniverseOrigin_TrackingUniverseSeated: ETrackingUniverseOrigin = 0;
pub const ETrackingUniverseOrigin_TrackingUniverseStanding: ETrackingUniverseOrigin = 1;
pub const ETrackingUniverseOrigin_TrackingUniverseRawAndUncalibrated: ETrackingUniverseOrigin = 2;
pub type ETrackingUniverseOrigin = ::core::ffi::c_uint;
pub const ETrackedDeviceProperty_Prop_Invalid: ETrackedDeviceProperty = 0;
pub const ETrackedDeviceProperty_Prop_TrackingSystemName_String: ETrackedDeviceProperty = 1000;
pub const ETrackedDeviceProperty_Prop_ModelNumber_String: ETrackedDeviceProperty = 1001;
pub const ETrackedDeviceProperty_Prop_SerialNumber_String: ETrackedDeviceProperty = 1002;
pub const ETrackedDeviceProperty_Prop_RenderModelName_String: ETrackedDeviceProperty = 1003;
pub const ETrackedDeviceProperty_Prop_WillDriftInYaw_Bool: ETrackedDeviceProperty = 1004;
pub const ETrackedDeviceProperty_Prop_ManufacturerName_String: ETrackedDeviceProperty = 1005;
pub const ETrackedDeviceProperty_Prop_TrackingFirmwareVersion_String: ETrackedDeviceProperty = 1006;
pub const ETrackedDeviceProperty_Prop_HardwareRevision_String: ETrackedDeviceProperty = 1007;
pub const ETrackedDeviceProperty_Prop_AllWirelessDongleDescriptions_String: ETrackedDeviceProperty =
    1008;
pub const ETrackedDeviceProperty_Prop_ConnectedWirelessDongle_String: ETrackedDeviceProperty = 1009;
pub const ETrackedDeviceProperty_Prop_DeviceIsWireless_Bool: ETrackedDeviceProperty = 1010;
pub const ETrackedDeviceProperty_Prop_DeviceIsCharging_Bool: ETrackedDeviceProperty = 1011;
pub const ETrackedDeviceProperty_Prop_DeviceBatteryPercentage_Float: ETrackedDeviceProperty = 1012;
pub const ETrackedDeviceProperty_Prop_StatusDisplayTransform_Matrix34: ETrackedDeviceProperty =
    1013;
pub const ETrackedDeviceProperty_Prop_Firmware_UpdateAvailable_Bool: ETrackedDeviceProperty = 1014;
pub const ETrackedDeviceProperty_Prop_Firmware_ManualUpdate_Bool: ETrackedDeviceProperty = 1015;
pub const ETrackedDeviceProperty_Prop_Firmware_ManualUpdateURL_String: ETrackedDeviceProperty =
    1016;
pub const ETrackedDeviceProperty_Prop_HardwareRevision_Uint64: ETrackedDeviceProperty = 1017;
pub const ETrackedDeviceProperty_Prop_FirmwareVersion_Uint64: ETrackedDeviceProperty = 1018;
pub const ETrackedDeviceProperty_Prop_FPGAVersion_Uint64: ETrackedDeviceProperty = 1019;
pub const ETrackedDeviceProperty_Prop_VRCVersion_Uint64: ETrackedDeviceProperty = 1020;
pub const ETrackedDeviceProperty_Prop_RadioVersion_Uint64: ETrackedDeviceProperty = 1021;
pub const ETrackedDeviceProperty_Prop_DongleVersion_Uint64: ETrackedDeviceProperty = 1022;
pub const ETrackedDeviceProperty_Prop_BlockServerShutdown_Bool: ETrackedDeviceProperty = 1023;
pub const ETrackedDeviceProperty_Prop_CanUnifyCoordinateSystemWithHmd_Bool: ETrackedDeviceProperty =
    1024;
pub const ETrackedDeviceProperty_Prop_ContainsProximitySensor_Bool: ETrackedDeviceProperty = 1025;
pub const ETrackedDeviceProperty_Prop_DeviceProvidesBatteryStatus_Bool: ETrackedDeviceProperty =
    1026;
pub const ETrackedDeviceProperty_Prop_DeviceCanPowerOff_Bool: ETrackedDeviceProperty = 1027;
pub const ETrackedDeviceProperty_Prop_Firmware_ProgrammingTarget_String: ETrackedDeviceProperty =
    1028;
pub const ETrackedDeviceProperty_Prop_DeviceClass_Int32: ETrackedDeviceProperty = 1029;
pub const ETrackedDeviceProperty_Prop_HasCamera_Bool: ETrackedDeviceProperty = 1030;
pub const ETrackedDeviceProperty_Prop_DriverVersion_String: ETrackedDeviceProperty = 1031;
pub const ETrackedDeviceProperty_Prop_Firmware_ForceUpdateRequired_Bool: ETrackedDeviceProperty =
    1032;
pub const ETrackedDeviceProperty_Prop_ViveSystemButtonFixRequired_Bool: ETrackedDeviceProperty =
    1033;
pub const ETrackedDeviceProperty_Prop_ParentDriver_Uint64: ETrackedDeviceProperty = 1034;
pub const ETrackedDeviceProperty_Prop_ResourceRoot_String: ETrackedDeviceProperty = 1035;
pub const ETrackedDeviceProperty_Prop_RegisteredDeviceType_String: ETrackedDeviceProperty = 1036;
pub const ETrackedDeviceProperty_Prop_InputProfilePath_String: ETrackedDeviceProperty = 1037;
pub const ETrackedDeviceProperty_Prop_NeverTracked_Bool: ETrackedDeviceProperty = 1038;
pub const ETrackedDeviceProperty_Prop_NumCameras_Int32: ETrackedDeviceProperty = 1039;
pub const ETrackedDeviceProperty_Prop_CameraFrameLayout_Int32: ETrackedDeviceProperty = 1040;
pub const ETrackedDeviceProperty_Prop_CameraStreamFormat_Int32: ETrackedDeviceProperty = 1041;
pub const ETrackedDeviceProperty_Prop_AdditionalDeviceSettingsPath_String: ETrackedDeviceProperty =
    1042;
pub const ETrackedDeviceProperty_Prop_Identifiable_Bool: ETrackedDeviceProperty = 1043;
pub const ETrackedDeviceProperty_Prop_BootloaderVersion_Uint64: ETrackedDeviceProperty = 1044;
pub const ETrackedDeviceProperty_Prop_AdditionalSystemReportData_String: ETrackedDeviceProperty =
    1045;
pub const ETrackedDeviceProperty_Prop_CompositeFirmwareVersion_String: ETrackedDeviceProperty =
    1046;
pub const ETrackedDeviceProperty_Prop_Firmware_RemindUpdate_Bool: ETrackedDeviceProperty = 1047;
pub const ETrackedDeviceProperty_Prop_PeripheralApplicationVersion_Uint64: ETrackedDeviceProperty =
    1048;
pub const ETrackedDeviceProperty_Prop_ManufacturerSerialNumber_String: ETrackedDeviceProperty =
    1049;
pub const ETrackedDeviceProperty_Prop_ComputedSerialNumber_String: ETrackedDeviceProperty = 1050;
pub const ETrackedDeviceProperty_Prop_EstimatedDeviceFirstUseTime_Int32: ETrackedDeviceProperty =
    1051;
pub const ETrackedDeviceProperty_Prop_DevicePowerUsage_Float: ETrackedDeviceProperty = 1052;
pub const ETrackedDeviceProperty_Prop_IgnoreMotionForStandby_Bool: ETrackedDeviceProperty = 1053;
pub const ETrackedDeviceProperty_Prop_ActualTrackingSystemName_String: ETrackedDeviceProperty =
    1054;
pub const ETrackedDeviceProperty_Prop_AllowCameraToggle_Bool: ETrackedDeviceProperty = 1055;
pub const ETrackedDeviceProperty_Prop_AllowLightSourceFrequency_Bool: ETrackedDeviceProperty = 1056;
pub const ETrackedDeviceProperty_Prop_SteamRemoteClientID_Uint64: ETrackedDeviceProperty = 1057;
pub const ETrackedDeviceProperty_Prop_Reserved_1058: ETrackedDeviceProperty = 1058;
pub const ETrackedDeviceProperty_Prop_Reserved_1059: ETrackedDeviceProperty = 1059;
pub const ETrackedDeviceProperty_Prop_Reserved_1060: ETrackedDeviceProperty = 1060;
pub const ETrackedDeviceProperty_Prop_ReportsTimeSinceVSync_Bool: ETrackedDeviceProperty = 2000;
pub const ETrackedDeviceProperty_Prop_SecondsFromVsyncToPhotons_Float: ETrackedDeviceProperty =
    2001;
pub const ETrackedDeviceProperty_Prop_DisplayFrequency_Float: ETrackedDeviceProperty = 2002;
pub const ETrackedDeviceProperty_Prop_UserIpdMeters_Float: ETrackedDeviceProperty = 2003;
pub const ETrackedDeviceProperty_Prop_CurrentUniverseId_Uint64: ETrackedDeviceProperty = 2004;
pub const ETrackedDeviceProperty_Prop_PreviousUniverseId_Uint64_deprecated: ETrackedDeviceProperty =
    0;
pub const ETrackedDeviceProperty_Prop_DisplayFirmwareVersion_Uint64: ETrackedDeviceProperty = 2006;
pub const ETrackedDeviceProperty_Prop_IsOnDesktop_Bool: ETrackedDeviceProperty = 2007;
pub const ETrackedDeviceProperty_Prop_DisplayMCType_Int32: ETrackedDeviceProperty = 2008;
pub const ETrackedDeviceProperty_Prop_DisplayMCOffset_Float: ETrackedDeviceProperty = 2009;
pub const ETrackedDeviceProperty_Prop_DisplayMCScale_Float: ETrackedDeviceProperty = 2010;
pub const ETrackedDeviceProperty_Prop_EdidVendorID_Int32: ETrackedDeviceProperty = 2011;
pub const ETrackedDeviceProperty_Prop_DisplayMCImageLeft_String: ETrackedDeviceProperty = 2012;
pub const ETrackedDeviceProperty_Prop_DisplayMCImageRight_String: ETrackedDeviceProperty = 2013;
pub const ETrackedDeviceProperty_Prop_DisplayGCBlackClamp_Float: ETrackedDeviceProperty = 2014;
pub const ETrackedDeviceProperty_Prop_EdidProductID_Int32: ETrackedDeviceProperty = 2015;
pub const ETrackedDeviceProperty_Prop_CameraToHeadTransform_Matrix34: ETrackedDeviceProperty = 2016;
pub const ETrackedDeviceProperty_Prop_DisplayGCType_Int32: ETrackedDeviceProperty = 2017;
pub const ETrackedDeviceProperty_Prop_DisplayGCOffset_Float: ETrackedDeviceProperty = 2018;
pub const ETrackedDeviceProperty_Prop_DisplayGCScale_Float: ETrackedDeviceProperty = 2019;
pub const ETrackedDeviceProperty_Prop_DisplayGCPrescale_Float: ETrackedDeviceProperty = 2020;
pub const ETrackedDeviceProperty_Prop_DisplayGCImage_String: ETrackedDeviceProperty = 2021;
pub const ETrackedDeviceProperty_Prop_LensCenterLeftU_Float: ETrackedDeviceProperty = 2022;
pub const ETrackedDeviceProperty_Prop_LensCenterLeftV_Float: ETrackedDeviceProperty = 2023;
pub const ETrackedDeviceProperty_Prop_LensCenterRightU_Float: ETrackedDeviceProperty = 2024;
pub const ETrackedDeviceProperty_Prop_LensCenterRightV_Float: ETrackedDeviceProperty = 2025;
pub const ETrackedDeviceProperty_Prop_UserHeadToEyeDepthMeters_Float: ETrackedDeviceProperty = 2026;
pub const ETrackedDeviceProperty_Prop_CameraFirmwareVersion_Uint64: ETrackedDeviceProperty = 2027;
pub const ETrackedDeviceProperty_Prop_CameraFirmwareDescription_String: ETrackedDeviceProperty =
    2028;
pub const ETrackedDeviceProperty_Prop_DisplayFPGAVersion_Uint64: ETrackedDeviceProperty = 2029;
pub const ETrackedDeviceProperty_Prop_DisplayBootloaderVersion_Uint64: ETrackedDeviceProperty =
    2030;
pub const ETrackedDeviceProperty_Prop_DisplayHardwareVersion_Uint64: ETrackedDeviceProperty = 2031;
pub const ETrackedDeviceProperty_Prop_AudioFirmwareVersion_Uint64: ETrackedDeviceProperty = 2032;
pub const ETrackedDeviceProperty_Prop_CameraCompatibilityMode_Int32: ETrackedDeviceProperty = 2033;
pub const ETrackedDeviceProperty_Prop_ScreenshotHorizontalFieldOfViewDegrees_Float:
    ETrackedDeviceProperty = 2034;
pub const ETrackedDeviceProperty_Prop_ScreenshotVerticalFieldOfViewDegrees_Float:
    ETrackedDeviceProperty = 2035;
pub const ETrackedDeviceProperty_Prop_DisplaySuppressed_Bool: ETrackedDeviceProperty = 2036;
pub const ETrackedDeviceProperty_Prop_DisplayAllowNightMode_Bool: ETrackedDeviceProperty = 2037;
pub const ETrackedDeviceProperty_Prop_DisplayMCImageWidth_Int32: ETrackedDeviceProperty = 2038;
pub const ETrackedDeviceProperty_Prop_DisplayMCImageHeight_Int32: ETrackedDeviceProperty = 2039;
pub const ETrackedDeviceProperty_Prop_DisplayMCImageNumChannels_Int32: ETrackedDeviceProperty =
    2040;
pub const ETrackedDeviceProperty_Prop_DisplayMCImageData_Binary: ETrackedDeviceProperty = 2041;
pub const ETrackedDeviceProperty_Prop_SecondsFromPhotonsToVblank_Float: ETrackedDeviceProperty =
    2042;
pub const ETrackedDeviceProperty_Prop_DriverDirectModeSendsVsyncEvents_Bool:
    ETrackedDeviceProperty = 2043;
pub const ETrackedDeviceProperty_Prop_DisplayDebugMode_Bool: ETrackedDeviceProperty = 2044;
pub const ETrackedDeviceProperty_Prop_GraphicsAdapterLuid_Uint64: ETrackedDeviceProperty = 2045;
pub const ETrackedDeviceProperty_Prop_DriverProvidedChaperonePath_String: ETrackedDeviceProperty =
    2048;
pub const ETrackedDeviceProperty_Prop_ExpectedTrackingReferenceCount_Int32: ETrackedDeviceProperty =
    2049;
pub const ETrackedDeviceProperty_Prop_ExpectedControllerCount_Int32: ETrackedDeviceProperty = 2050;
pub const ETrackedDeviceProperty_Prop_NamedIconPathControllerLeftDeviceOff_String:
    ETrackedDeviceProperty = 2051;
pub const ETrackedDeviceProperty_Prop_NamedIconPathControllerRightDeviceOff_String:
    ETrackedDeviceProperty = 2052;
pub const ETrackedDeviceProperty_Prop_NamedIconPathTrackingReferenceDeviceOff_String:
    ETrackedDeviceProperty = 2053;
pub const ETrackedDeviceProperty_Prop_DoNotApplyPrediction_Bool: ETrackedDeviceProperty = 2054;
pub const ETrackedDeviceProperty_Prop_CameraToHeadTransforms_Matrix34_Array:
    ETrackedDeviceProperty = 2055;
pub const ETrackedDeviceProperty_Prop_DistortionMeshResolution_Int32: ETrackedDeviceProperty = 2056;
pub const ETrackedDeviceProperty_Prop_DriverIsDrawingControllers_Bool: ETrackedDeviceProperty =
    2057;
pub const ETrackedDeviceProperty_Prop_DriverRequestsApplicationPause_Bool: ETrackedDeviceProperty =
    2058;
pub const ETrackedDeviceProperty_Prop_DriverRequestsReducedRendering_Bool: ETrackedDeviceProperty =
    2059;
pub const ETrackedDeviceProperty_Prop_MinimumIpdStepMeters_Float: ETrackedDeviceProperty = 2060;
pub const ETrackedDeviceProperty_Prop_AudioBridgeFirmwareVersion_Uint64: ETrackedDeviceProperty =
    2061;
pub const ETrackedDeviceProperty_Prop_ImageBridgeFirmwareVersion_Uint64: ETrackedDeviceProperty =
    2062;
pub const ETrackedDeviceProperty_Prop_ImuToHeadTransform_Matrix34: ETrackedDeviceProperty = 2063;
pub const ETrackedDeviceProperty_Prop_ImuFactoryGyroBias_Vector3: ETrackedDeviceProperty = 2064;
pub const ETrackedDeviceProperty_Prop_ImuFactoryGyroScale_Vector3: ETrackedDeviceProperty = 2065;
pub const ETrackedDeviceProperty_Prop_ImuFactoryAccelerometerBias_Vector3: ETrackedDeviceProperty =
    2066;
pub const ETrackedDeviceProperty_Prop_ImuFactoryAccelerometerScale_Vector3: ETrackedDeviceProperty =
    2067;
pub const ETrackedDeviceProperty_Prop_ConfigurationIncludesLighthouse20Features_Bool:
    ETrackedDeviceProperty = 2069;
pub const ETrackedDeviceProperty_Prop_AdditionalRadioFeatures_Uint64: ETrackedDeviceProperty = 2070;
pub const ETrackedDeviceProperty_Prop_CameraWhiteBalance_Vector4_Array: ETrackedDeviceProperty =
    2071;
pub const ETrackedDeviceProperty_Prop_CameraDistortionFunction_Int32_Array: ETrackedDeviceProperty =
    2072;
pub const ETrackedDeviceProperty_Prop_CameraDistortionCoefficients_Float_Array:
    ETrackedDeviceProperty = 2073;
pub const ETrackedDeviceProperty_Prop_ExpectedControllerType_String: ETrackedDeviceProperty = 2074;
pub const ETrackedDeviceProperty_Prop_HmdTrackingStyle_Int32: ETrackedDeviceProperty = 2075;
pub const ETrackedDeviceProperty_Prop_DriverProvidedChaperoneVisibility_Bool:
    ETrackedDeviceProperty = 2076;
pub const ETrackedDeviceProperty_Prop_HmdColumnCorrectionSettingPrefix_String:
    ETrackedDeviceProperty = 2077;
pub const ETrackedDeviceProperty_Prop_CameraSupportsCompatibilityModes_Bool:
    ETrackedDeviceProperty = 2078;
pub const ETrackedDeviceProperty_Prop_SupportsRoomViewDepthProjection_Bool: ETrackedDeviceProperty =
    2079;
pub const ETrackedDeviceProperty_Prop_DisplayAvailableFrameRates_Float_Array:
    ETrackedDeviceProperty = 2080;
pub const ETrackedDeviceProperty_Prop_DisplaySupportsMultipleFramerates_Bool:
    ETrackedDeviceProperty = 2081;
pub const ETrackedDeviceProperty_Prop_DisplayColorMultLeft_Vector3: ETrackedDeviceProperty = 2082;
pub const ETrackedDeviceProperty_Prop_DisplayColorMultRight_Vector3: ETrackedDeviceProperty = 2083;
pub const ETrackedDeviceProperty_Prop_DisplaySupportsRuntimeFramerateChange_Bool:
    ETrackedDeviceProperty = 2084;
pub const ETrackedDeviceProperty_Prop_DisplaySupportsAnalogGain_Bool: ETrackedDeviceProperty = 2085;
pub const ETrackedDeviceProperty_Prop_DisplayMinAnalogGain_Float: ETrackedDeviceProperty = 2086;
pub const ETrackedDeviceProperty_Prop_DisplayMaxAnalogGain_Float: ETrackedDeviceProperty = 2087;
pub const ETrackedDeviceProperty_Prop_CameraExposureTime_Float: ETrackedDeviceProperty = 2088;
pub const ETrackedDeviceProperty_Prop_CameraGlobalGain_Float: ETrackedDeviceProperty = 2089;
pub const ETrackedDeviceProperty_Prop_DashboardScale_Float: ETrackedDeviceProperty = 2091;
pub const ETrackedDeviceProperty_Prop_Hmd_SupportsHDR10_Bool: ETrackedDeviceProperty = 2093;
pub const ETrackedDeviceProperty_Prop_Hmd_EnableParallelRenderCameras_Bool: ETrackedDeviceProperty =
    2094;
pub const ETrackedDeviceProperty_Prop_DriverProvidedChaperoneJson_String: ETrackedDeviceProperty =
    2095;
pub const ETrackedDeviceProperty_Prop_ForceSystemLayerUseAppPoses_Bool: ETrackedDeviceProperty =
    2096;
pub const ETrackedDeviceProperty_Prop_DashboardLinkSupport_Int32: ETrackedDeviceProperty = 2097;
pub const ETrackedDeviceProperty_Prop_DisplayMinUIAnalogGain_Float: ETrackedDeviceProperty = 2098;
pub const ETrackedDeviceProperty_Prop_IpdUIRangeMinMeters_Float: ETrackedDeviceProperty = 2100;
pub const ETrackedDeviceProperty_Prop_IpdUIRangeMaxMeters_Float: ETrackedDeviceProperty = 2101;
pub const ETrackedDeviceProperty_Prop_Hmd_SupportsHDCP14LegacyCompat_Bool: ETrackedDeviceProperty =
    2102;
pub const ETrackedDeviceProperty_Prop_Hmd_SupportsMicMonitoring_Bool: ETrackedDeviceProperty = 2103;
pub const ETrackedDeviceProperty_Prop_Hmd_SupportsDisplayPortTrainingMode_Bool:
    ETrackedDeviceProperty = 2104;
pub const ETrackedDeviceProperty_Prop_Hmd_SupportsRoomViewDirect_Bool: ETrackedDeviceProperty =
    2105;
pub const ETrackedDeviceProperty_Prop_Hmd_SupportsAppThrottling_Bool: ETrackedDeviceProperty = 2106;
pub const ETrackedDeviceProperty_Prop_Hmd_SupportsGpuBusMonitoring_Bool: ETrackedDeviceProperty =
    2107;
pub const ETrackedDeviceProperty_Prop_DriverDisplaysIPDChanges_Bool: ETrackedDeviceProperty = 2108;
pub const ETrackedDeviceProperty_Prop_Reserved_2110: ETrackedDeviceProperty = 2110;
pub const ETrackedDeviceProperty_Prop_Reserved_2111: ETrackedDeviceProperty = 2111;
pub const ETrackedDeviceProperty_Prop_Reserved_2112: ETrackedDeviceProperty = 2112;
pub const ETrackedDeviceProperty_Prop_Hmd_MaxDistortedTextureWidth_Int32: ETrackedDeviceProperty =
    2113;
pub const ETrackedDeviceProperty_Prop_Hmd_MaxDistortedTextureHeight_Int32: ETrackedDeviceProperty =
    2114;
pub const ETrackedDeviceProperty_Prop_Hmd_AllowSupersampleFiltering_Bool: ETrackedDeviceProperty =
    2115;
pub const ETrackedDeviceProperty_Prop_Hmd_AllowsClientToControlTextureIndex:
    ETrackedDeviceProperty = 2116;
pub const ETrackedDeviceProperty_Prop_Reserved_2117: ETrackedDeviceProperty = 2117;
pub const ETrackedDeviceProperty_Prop_DriverRequestedMuraCorrectionMode_Int32:
    ETrackedDeviceProperty = 2200;
pub const ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_InnerLeft_Int32:
    ETrackedDeviceProperty = 2201;
pub const ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_InnerRight_Int32:
    ETrackedDeviceProperty = 2202;
pub const ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_InnerTop_Int32:
    ETrackedDeviceProperty = 2203;
pub const ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_InnerBottom_Int32:
    ETrackedDeviceProperty = 2204;
pub const ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_OuterLeft_Int32:
    ETrackedDeviceProperty = 2205;
pub const ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_OuterRight_Int32:
    ETrackedDeviceProperty = 2206;
pub const ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_OuterTop_Int32:
    ETrackedDeviceProperty = 2207;
pub const ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_OuterBottom_Int32:
    ETrackedDeviceProperty = 2208;
pub const ETrackedDeviceProperty_Prop_Audio_DefaultPlaybackDeviceId_String: ETrackedDeviceProperty =
    2300;
pub const ETrackedDeviceProperty_Prop_Audio_DefaultRecordingDeviceId_String:
    ETrackedDeviceProperty = 2301;
pub const ETrackedDeviceProperty_Prop_Audio_DefaultPlaybackDeviceVolume_Float:
    ETrackedDeviceProperty = 2302;
pub const ETrackedDeviceProperty_Prop_Audio_SupportsDualSpeakerAndJackOutput_Bool:
    ETrackedDeviceProperty = 2303;
pub const ETrackedDeviceProperty_Prop_Audio_DriverManagesPlaybackVolumeControl_Bool:
    ETrackedDeviceProperty = 2304;
pub const ETrackedDeviceProperty_Prop_Audio_DriverPlaybackVolume_Float: ETrackedDeviceProperty =
    2305;
pub const ETrackedDeviceProperty_Prop_Audio_DriverPlaybackMute_Bool: ETrackedDeviceProperty = 2306;
pub const ETrackedDeviceProperty_Prop_Audio_DriverManagesRecordingVolumeControl_Bool:
    ETrackedDeviceProperty = 2307;
pub const ETrackedDeviceProperty_Prop_Audio_DriverRecordingVolume_Float: ETrackedDeviceProperty =
    2308;
pub const ETrackedDeviceProperty_Prop_Audio_DriverRecordingMute_Bool: ETrackedDeviceProperty = 2309;
pub const ETrackedDeviceProperty_Prop_Audio_PipewirePlaybackNode_Int32: ETrackedDeviceProperty =
    2400;
pub const ETrackedDeviceProperty_Prop_Audio_PipewireRecordingNode_Int32: ETrackedDeviceProperty =
    2401;
pub const ETrackedDeviceProperty_Prop_AttachedDeviceId_String: ETrackedDeviceProperty = 3000;
pub const ETrackedDeviceProperty_Prop_SupportedButtons_Uint64: ETrackedDeviceProperty = 3001;
pub const ETrackedDeviceProperty_Prop_Axis0Type_Int32: ETrackedDeviceProperty = 3002;
pub const ETrackedDeviceProperty_Prop_Axis1Type_Int32: ETrackedDeviceProperty = 3003;
pub const ETrackedDeviceProperty_Prop_Axis2Type_Int32: ETrackedDeviceProperty = 3004;
pub const ETrackedDeviceProperty_Prop_Axis3Type_Int32: ETrackedDeviceProperty = 3005;
pub const ETrackedDeviceProperty_Prop_Axis4Type_Int32: ETrackedDeviceProperty = 3006;
pub const ETrackedDeviceProperty_Prop_ControllerRoleHint_Int32: ETrackedDeviceProperty = 3007;
pub const ETrackedDeviceProperty_Prop_FieldOfViewLeftDegrees_Float: ETrackedDeviceProperty = 4000;
pub const ETrackedDeviceProperty_Prop_FieldOfViewRightDegrees_Float: ETrackedDeviceProperty = 4001;
pub const ETrackedDeviceProperty_Prop_FieldOfViewTopDegrees_Float: ETrackedDeviceProperty = 4002;
pub const ETrackedDeviceProperty_Prop_FieldOfViewBottomDegrees_Float: ETrackedDeviceProperty = 4003;
pub const ETrackedDeviceProperty_Prop_TrackingRangeMinimumMeters_Float: ETrackedDeviceProperty =
    4004;
pub const ETrackedDeviceProperty_Prop_TrackingRangeMaximumMeters_Float: ETrackedDeviceProperty =
    4005;
pub const ETrackedDeviceProperty_Prop_ModeLabel_String: ETrackedDeviceProperty = 4006;
pub const ETrackedDeviceProperty_Prop_CanWirelessIdentify_Bool: ETrackedDeviceProperty = 4007;
pub const ETrackedDeviceProperty_Prop_Nonce_Int32: ETrackedDeviceProperty = 4008;
pub const ETrackedDeviceProperty_Prop_IconPathName_String: ETrackedDeviceProperty = 5000;
pub const ETrackedDeviceProperty_Prop_NamedIconPathDeviceOff_String: ETrackedDeviceProperty = 5001;
pub const ETrackedDeviceProperty_Prop_NamedIconPathDeviceSearching_String: ETrackedDeviceProperty =
    5002;
pub const ETrackedDeviceProperty_Prop_NamedIconPathDeviceSearchingAlert_String:
    ETrackedDeviceProperty = 5003;
pub const ETrackedDeviceProperty_Prop_NamedIconPathDeviceReady_String: ETrackedDeviceProperty =
    5004;
pub const ETrackedDeviceProperty_Prop_NamedIconPathDeviceReadyAlert_String: ETrackedDeviceProperty =
    5005;
pub const ETrackedDeviceProperty_Prop_NamedIconPathDeviceNotReady_String: ETrackedDeviceProperty =
    5006;
pub const ETrackedDeviceProperty_Prop_NamedIconPathDeviceStandby_String: ETrackedDeviceProperty =
    5007;
pub const ETrackedDeviceProperty_Prop_NamedIconPathDeviceAlertLow_String: ETrackedDeviceProperty =
    5008;
pub const ETrackedDeviceProperty_Prop_NamedIconPathDeviceStandbyAlert_String:
    ETrackedDeviceProperty = 5009;
pub const ETrackedDeviceProperty_Prop_DisplayHiddenArea_Binary_Start: ETrackedDeviceProperty = 5100;
pub const ETrackedDeviceProperty_Prop_DisplayHiddenArea_Binary_End: ETrackedDeviceProperty = 5150;
pub const ETrackedDeviceProperty_Prop_ParentContainer: ETrackedDeviceProperty = 5151;
pub const ETrackedDeviceProperty_Prop_OverrideContainer_Uint64: ETrackedDeviceProperty = 5152;
pub const ETrackedDeviceProperty_Prop_UserConfigPath_String: ETrackedDeviceProperty = 6000;
pub const ETrackedDeviceProperty_Prop_InstallPath_String: ETrackedDeviceProperty = 6001;
pub const ETrackedDeviceProperty_Prop_HasDisplayComponent_Bool: ETrackedDeviceProperty = 6002;
pub const ETrackedDeviceProperty_Prop_HasControllerComponent_Bool: ETrackedDeviceProperty = 6003;
pub const ETrackedDeviceProperty_Prop_HasCameraComponent_Bool: ETrackedDeviceProperty = 6004;
pub const ETrackedDeviceProperty_Prop_HasDriverDirectModeComponent_Bool: ETrackedDeviceProperty =
    6005;
pub const ETrackedDeviceProperty_Prop_HasVirtualDisplayComponent_Bool: ETrackedDeviceProperty =
    6006;
pub const ETrackedDeviceProperty_Prop_HasSpatialAnchorsSupport_Bool: ETrackedDeviceProperty = 6007;
pub const ETrackedDeviceProperty_Prop_SupportsXrTextureSets_Bool: ETrackedDeviceProperty = 6008;
pub const ETrackedDeviceProperty_Prop_SupportsXrEyeGazeInteraction_Bool: ETrackedDeviceProperty =
    6009;
pub const ETrackedDeviceProperty_Prop_DeviceHasNoIMU_Bool: ETrackedDeviceProperty = 6010;
pub const ETrackedDeviceProperty_Prop_UseAdvancedPrediction_Bool: ETrackedDeviceProperty = 6011;
pub const ETrackedDeviceProperty_Prop_ControllerType_String: ETrackedDeviceProperty = 7000;
pub const ETrackedDeviceProperty_Prop_ControllerHandSelectionPriority_Int32:
    ETrackedDeviceProperty = 7002;
pub const ETrackedDeviceProperty_Prop_VendorSpecific_Reserved_Start: ETrackedDeviceProperty = 10000;
pub const ETrackedDeviceProperty_Prop_VendorSpecific_Reserved_End: ETrackedDeviceProperty = 10999;
pub const ETrackedDeviceProperty_Prop_Reserved_11000: ETrackedDeviceProperty = 11000;
pub const ETrackedDeviceProperty_Prop_Reserved_11001: ETrackedDeviceProperty = 11001;
pub const ETrackedDeviceProperty_Prop_Reserved_11002: ETrackedDeviceProperty = 11002;
pub const ETrackedDeviceProperty_Prop_Reserved_11003: ETrackedDeviceProperty = 11003;
pub const ETrackedDeviceProperty_Prop_Reserved_11004: ETrackedDeviceProperty = 11004;
pub const ETrackedDeviceProperty_Prop_TrackedDeviceProperty_Max: ETrackedDeviceProperty = 1000000;
pub type ETrackedDeviceProperty = ::core::ffi::c_uint;
pub const ETrackedPropertyError_TrackedProp_Success: ETrackedPropertyError = 0;
pub const ETrackedPropertyError_TrackedProp_WrongDataType: ETrackedPropertyError = 1;
pub const ETrackedPropertyError_TrackedProp_WrongDeviceClass: ETrackedPropertyError = 2;
pub const ETrackedPropertyError_TrackedProp_BufferTooSmall: ETrackedPropertyError = 3;
pub const ETrackedPropertyError_TrackedProp_UnknownProperty: ETrackedPropertyError = 4;
pub const ETrackedPropertyError_TrackedProp_InvalidDevice: ETrackedPropertyError = 5;
pub const ETrackedPropertyError_TrackedProp_CouldNotContactServer: ETrackedPropertyError = 6;
pub const ETrackedPropertyError_TrackedProp_ValueNotProvidedByDevice: ETrackedPropertyError = 7;
pub const ETrackedPropertyError_TrackedProp_StringExceedsMaximumLength: ETrackedPropertyError = 8;
pub const ETrackedPropertyError_TrackedProp_NotYetAvailable: ETrackedPropertyError = 9;
pub const ETrackedPropertyError_TrackedProp_PermissionDenied: ETrackedPropertyError = 10;
pub const ETrackedPropertyError_TrackedProp_InvalidOperation: ETrackedPropertyError = 11;
pub const ETrackedPropertyError_TrackedProp_CannotWriteToWildcards: ETrackedPropertyError = 12;
pub const ETrackedPropertyError_TrackedProp_IPCReadFailure: ETrackedPropertyError = 13;
pub const ETrackedPropertyError_TrackedProp_OutOfMemory: ETrackedPropertyError = 14;
pub const ETrackedPropertyError_TrackedProp_InvalidContainer: ETrackedPropertyError = 15;
pub type ETrackedPropertyError = ::core::ffi::c_uint;
pub const EVRSubmitFlags_Submit_Default: EVRSubmitFlags = 0;
pub const EVRSubmitFlags_Submit_LensDistortionAlreadyApplied: EVRSubmitFlags = 1;
pub const EVRSubmitFlags_Submit_GlRenderBuffer: EVRSubmitFlags = 2;
pub const EVRSubmitFlags_Submit_Reserved: EVRSubmitFlags = 4;
pub const EVRSubmitFlags_Submit_TextureWithPose: EVRSubmitFlags = 8;
pub const EVRSubmitFlags_Submit_TextureWithDepth: EVRSubmitFlags = 16;
pub const EVRSubmitFlags_Submit_FrameDiscontinuity: EVRSubmitFlags = 32;
pub const EVRSubmitFlags_Submit_VulkanTextureWithArrayData: EVRSubmitFlags = 64;
pub const EVRSubmitFlags_Submit_GlArrayTexture: EVRSubmitFlags = 128;
pub const EVRSubmitFlags_Submit_IsEgl: EVRSubmitFlags = 256;
pub const EVRSubmitFlags_Submit_TextureWithMotion: EVRSubmitFlags = 536;
pub const EVRSubmitFlags_Submit_Reserved2: EVRSubmitFlags = 32768;
pub const EVRSubmitFlags_Submit_Reserved3: EVRSubmitFlags = 65536;
pub const EVRSubmitFlags_Submit_Reserved4: EVRSubmitFlags = 131072;
pub const EVRSubmitFlags_Submit_Reserved5: EVRSubmitFlags = 262144;
pub const EVRSubmitFlags_Submit_Reserved6: EVRSubmitFlags = 524288;
pub type EVRSubmitFlags = ::core::ffi::c_uint;
pub const EVRState_VRState_Undefined: EVRState = -1;
pub const EVRState_VRState_Off: EVRState = 0;
pub const EVRState_VRState_Searching: EVRState = 1;
pub const EVRState_VRState_Searching_Alert: EVRState = 2;
pub const EVRState_VRState_Ready: EVRState = 3;
pub const EVRState_VRState_Ready_Alert: EVRState = 4;
pub const EVRState_VRState_NotReady: EVRState = 5;
pub const EVRState_VRState_Standby: EVRState = 6;
pub const EVRState_VRState_Ready_Alert_Low: EVRState = 7;
pub type EVRState = ::core::ffi::c_int;
pub const EVREventType_VREvent_None: EVREventType = 0;
pub const EVREventType_VREvent_TrackedDeviceActivated: EVREventType = 100;
pub const EVREventType_VREvent_TrackedDeviceDeactivated: EVREventType = 101;
pub const EVREventType_VREvent_TrackedDeviceUpdated: EVREventType = 102;
pub const EVREventType_VREvent_TrackedDeviceUserInteractionStarted: EVREventType = 103;
pub const EVREventType_VREvent_TrackedDeviceUserInteractionEnded: EVREventType = 104;
pub const EVREventType_VREvent_IpdChanged: EVREventType = 105;
pub const EVREventType_VREvent_EnterStandbyMode: EVREventType = 106;
pub const EVREventType_VREvent_LeaveStandbyMode: EVREventType = 107;
pub const EVREventType_VREvent_TrackedDeviceRoleChanged: EVREventType = 108;
pub const EVREventType_VREvent_WatchdogWakeUpRequested: EVREventType = 109;
pub const EVREventType_VREvent_LensDistortionChanged: EVREventType = 110;
pub const EVREventType_VREvent_PropertyChanged: EVREventType = 111;
pub const EVREventType_VREvent_WirelessDisconnect: EVREventType = 112;
pub const EVREventType_VREvent_WirelessReconnect: EVREventType = 113;
pub const EVREventType_VREvent_Reserved_0114: EVREventType = 114;
pub const EVREventType_VREvent_Reserved_0115: EVREventType = 115;
pub const EVREventType_VREvent_ButtonPress: EVREventType = 200;
pub const EVREventType_VREvent_ButtonUnpress: EVREventType = 201;
pub const EVREventType_VREvent_ButtonTouch: EVREventType = 202;
pub const EVREventType_VREvent_ButtonUntouch: EVREventType = 203;
pub const EVREventType_VREvent_Modal_Cancel: EVREventType = 257;
pub const EVREventType_VREvent_MouseMove: EVREventType = 300;
pub const EVREventType_VREvent_MouseButtonDown: EVREventType = 301;
pub const EVREventType_VREvent_MouseButtonUp: EVREventType = 302;
pub const EVREventType_VREvent_FocusEnter: EVREventType = 303;
pub const EVREventType_VREvent_FocusLeave: EVREventType = 304;
pub const EVREventType_VREvent_ScrollDiscrete: EVREventType = 305;
pub const EVREventType_VREvent_TouchPadMove: EVREventType = 306;
pub const EVREventType_VREvent_OverlayFocusChanged: EVREventType = 307;
pub const EVREventType_VREvent_ReloadOverlays: EVREventType = 308;
pub const EVREventType_VREvent_ScrollSmooth: EVREventType = 309;
pub const EVREventType_VREvent_LockMousePosition: EVREventType = 310;
pub const EVREventType_VREvent_UnlockMousePosition: EVREventType = 311;
pub const EVREventType_VREvent_InputFocusCaptured: EVREventType = 400;
pub const EVREventType_VREvent_InputFocusReleased: EVREventType = 401;
pub const EVREventType_VREvent_SceneApplicationChanged: EVREventType = 404;
pub const EVREventType_VREvent_InputFocusChanged: EVREventType = 406;
pub const EVREventType_VREvent_SceneApplicationUsingWrongGraphicsAdapter: EVREventType = 408;
pub const EVREventType_VREvent_ActionBindingReloaded: EVREventType = 409;
pub const EVREventType_VREvent_HideRenderModels: EVREventType = 410;
pub const EVREventType_VREvent_ShowRenderModels: EVREventType = 411;
pub const EVREventType_VREvent_SceneApplicationStateChanged: EVREventType = 412;
pub const EVREventType_VREvent_SceneAppPipeDisconnected: EVREventType = 413;
pub const EVREventType_VREvent_ConsoleOpened: EVREventType = 420;
pub const EVREventType_VREvent_ConsoleClosed: EVREventType = 421;
pub const EVREventType_VREvent_OverlayShown: EVREventType = 500;
pub const EVREventType_VREvent_OverlayHidden: EVREventType = 501;
pub const EVREventType_VREvent_DashboardActivated: EVREventType = 502;
pub const EVREventType_VREvent_DashboardDeactivated: EVREventType = 503;
pub const EVREventType_VREvent_ResetDashboard: EVREventType = 506;
pub const EVREventType_VREvent_ImageLoaded: EVREventType = 508;
pub const EVREventType_VREvent_ShowKeyboard: EVREventType = 509;
pub const EVREventType_VREvent_HideKeyboard: EVREventType = 510;
pub const EVREventType_VREvent_OverlayGamepadFocusGained: EVREventType = 511;
pub const EVREventType_VREvent_OverlayGamepadFocusLost: EVREventType = 512;
pub const EVREventType_VREvent_OverlaySharedTextureChanged: EVREventType = 513;
pub const EVREventType_VREvent_ScreenshotTriggered: EVREventType = 516;
pub const EVREventType_VREvent_ImageFailed: EVREventType = 517;
pub const EVREventType_VREvent_DashboardOverlayCreated: EVREventType = 518;
pub const EVREventType_VREvent_SwitchGamepadFocus: EVREventType = 519;
pub const EVREventType_VREvent_RequestScreenshot: EVREventType = 520;
pub const EVREventType_VREvent_ScreenshotTaken: EVREventType = 521;
pub const EVREventType_VREvent_ScreenshotFailed: EVREventType = 522;
pub const EVREventType_VREvent_SubmitScreenshotToDashboard: EVREventType = 523;
pub const EVREventType_VREvent_ScreenshotProgressToDashboard: EVREventType = 524;
pub const EVREventType_VREvent_PrimaryDashboardDeviceChanged: EVREventType = 525;
pub const EVREventType_VREvent_RoomViewShown: EVREventType = 526;
pub const EVREventType_VREvent_RoomViewHidden: EVREventType = 527;
pub const EVREventType_VREvent_ShowUI: EVREventType = 528;
pub const EVREventType_VREvent_ShowDevTools: EVREventType = 529;
pub const EVREventType_VREvent_DesktopViewUpdating: EVREventType = 530;
pub const EVREventType_VREvent_DesktopViewReady: EVREventType = 531;
pub const EVREventType_VREvent_StartDashboard: EVREventType = 532;
pub const EVREventType_VREvent_ElevatePrism: EVREventType = 533;
pub const EVREventType_VREvent_OverlayClosed: EVREventType = 534;
pub const EVREventType_VREvent_DashboardThumbChanged: EVREventType = 535;
pub const EVREventType_VREvent_DesktopMightBeVisible: EVREventType = 536;
pub const EVREventType_VREvent_DesktopMightBeHidden: EVREventType = 537;
pub const EVREventType_VREvent_MutualSteamCapabilitiesChanged: EVREventType = 538;
pub const EVREventType_VREvent_OverlayCreated: EVREventType = 539;
pub const EVREventType_VREvent_OverlayDestroyed: EVREventType = 540;
pub const EVREventType_VREvent_OverlayNameChanged: EVREventType = 544;
pub const EVREventType_VREvent_TrackingRecordingStarted: EVREventType = 541;
pub const EVREventType_VREvent_TrackingRecordingStopped: EVREventType = 542;
pub const EVREventType_VREvent_SetTrackingRecordingPath: EVREventType = 543;
pub const EVREventType_VREvent_Reserved_0560: EVREventType = 560;
pub const EVREventType_VREvent_Reserved_0561: EVREventType = 561;
pub const EVREventType_VREvent_Reserved_0562: EVREventType = 562;
pub const EVREventType_VREvent_Reserved_0563: EVREventType = 563;
pub const EVREventType_VREvent_Notification_Shown: EVREventType = 600;
pub const EVREventType_VREvent_Notification_Hidden: EVREventType = 601;
pub const EVREventType_VREvent_Notification_BeginInteraction: EVREventType = 602;
pub const EVREventType_VREvent_Notification_Destroyed: EVREventType = 603;
pub const EVREventType_VREvent_Quit: EVREventType = 700;
pub const EVREventType_VREvent_ProcessQuit: EVREventType = 701;
pub const EVREventType_VREvent_QuitAcknowledged: EVREventType = 703;
pub const EVREventType_VREvent_DriverRequestedQuit: EVREventType = 704;
pub const EVREventType_VREvent_RestartRequested: EVREventType = 705;
pub const EVREventType_VREvent_InvalidateSwapTextureSets: EVREventType = 706;
pub const EVREventType_VREvent_RequestDisconnectWirelessHMD: EVREventType = 707;
pub const EVREventType_VREvent_ChaperoneDataHasChanged: EVREventType = 800;
pub const EVREventType_VREvent_ChaperoneUniverseHasChanged: EVREventType = 801;
pub const EVREventType_VREvent_ChaperoneTempDataHasChanged: EVREventType = 802;
pub const EVREventType_VREvent_ChaperoneSettingsHaveChanged: EVREventType = 803;
pub const EVREventType_VREvent_SeatedZeroPoseReset: EVREventType = 804;
pub const EVREventType_VREvent_ChaperoneFlushCache: EVREventType = 805;
pub const EVREventType_VREvent_ChaperoneRoomSetupStarting: EVREventType = 806;
pub const EVREventType_VREvent_ChaperoneRoomSetupCommitted: EVREventType = 807;
pub const EVREventType_VREvent_StandingZeroPoseReset: EVREventType = 808;
pub const EVREventType_VREvent_Reserved_0809: EVREventType = 809;
pub const EVREventType_VREvent_Reserved_0810: EVREventType = 810;
pub const EVREventType_VREvent_Reserved_0811: EVREventType = 811;
pub const EVREventType_VREvent_Reserved_0812: EVREventType = 812;
pub const EVREventType_VREvent_Reserved_0813: EVREventType = 813;
pub const EVREventType_VREvent_Reserved_0814: EVREventType = 814;
pub const EVREventType_VREvent_AudioSettingsHaveChanged: EVREventType = 820;
pub const EVREventType_VREvent_BackgroundSettingHasChanged: EVREventType = 850;
pub const EVREventType_VREvent_CameraSettingsHaveChanged: EVREventType = 851;
pub const EVREventType_VREvent_ReprojectionSettingHasChanged: EVREventType = 852;
pub const EVREventType_VREvent_ModelSkinSettingsHaveChanged: EVREventType = 853;
pub const EVREventType_VREvent_EnvironmentSettingsHaveChanged: EVREventType = 854;
pub const EVREventType_VREvent_PowerSettingsHaveChanged: EVREventType = 855;
pub const EVREventType_VREvent_EnableHomeAppSettingsHaveChanged: EVREventType = 856;
pub const EVREventType_VREvent_SteamVRSectionSettingChanged: EVREventType = 857;
pub const EVREventType_VREvent_LighthouseSectionSettingChanged: EVREventType = 858;
pub const EVREventType_VREvent_NullSectionSettingChanged: EVREventType = 859;
pub const EVREventType_VREvent_UserInterfaceSectionSettingChanged: EVREventType = 860;
pub const EVREventType_VREvent_NotificationsSectionSettingChanged: EVREventType = 861;
pub const EVREventType_VREvent_KeyboardSectionSettingChanged: EVREventType = 862;
pub const EVREventType_VREvent_PerfSectionSettingChanged: EVREventType = 863;
pub const EVREventType_VREvent_DashboardSectionSettingChanged: EVREventType = 864;
pub const EVREventType_VREvent_WebInterfaceSectionSettingChanged: EVREventType = 865;
pub const EVREventType_VREvent_TrackersSectionSettingChanged: EVREventType = 866;
pub const EVREventType_VREvent_LastKnownSectionSettingChanged: EVREventType = 867;
pub const EVREventType_VREvent_DismissedWarningsSectionSettingChanged: EVREventType = 868;
pub const EVREventType_VREvent_GpuSpeedSectionSettingChanged: EVREventType = 869;
pub const EVREventType_VREvent_WindowsMRSectionSettingChanged: EVREventType = 870;
pub const EVREventType_VREvent_OtherSectionSettingChanged: EVREventType = 871;
pub const EVREventType_VREvent_AnyDriverSettingsChanged: EVREventType = 872;
pub const EVREventType_VREvent_Reserved_0873: EVREventType = 873;
pub const EVREventType_VREvent_StatusUpdate: EVREventType = 900;
pub const EVREventType_VREvent_WebInterface_InstallDriverCompleted: EVREventType = 950;
pub const EVREventType_VREvent_MCImageUpdated: EVREventType = 1000;
pub const EVREventType_VREvent_FirmwareUpdateStarted: EVREventType = 1100;
pub const EVREventType_VREvent_FirmwareUpdateFinished: EVREventType = 1101;
pub const EVREventType_VREvent_KeyboardClosed: EVREventType = 1200;
pub const EVREventType_VREvent_KeyboardCharInput: EVREventType = 1201;
pub const EVREventType_VREvent_KeyboardDone: EVREventType = 1202;
pub const EVREventType_VREvent_KeyboardOpened_Global: EVREventType = 1203;
pub const EVREventType_VREvent_KeyboardClosed_Global: EVREventType = 1204;
pub const EVREventType_VREvent_ApplicationListUpdated: EVREventType = 1303;
pub const EVREventType_VREvent_ApplicationMimeTypeLoad: EVREventType = 1304;
pub const EVREventType_VREvent_ProcessConnected: EVREventType = 1306;
pub const EVREventType_VREvent_ProcessDisconnected: EVREventType = 1307;
pub const EVREventType_VREvent_Compositor_ChaperoneBoundsShown: EVREventType = 1410;
pub const EVREventType_VREvent_Compositor_ChaperoneBoundsHidden: EVREventType = 1411;
pub const EVREventType_VREvent_Compositor_DisplayDisconnected: EVREventType = 1412;
pub const EVREventType_VREvent_Compositor_DisplayReconnected: EVREventType = 1413;
pub const EVREventType_VREvent_Compositor_HDCPError: EVREventType = 1414;
pub const EVREventType_VREvent_Compositor_ApplicationNotResponding: EVREventType = 1415;
pub const EVREventType_VREvent_Compositor_ApplicationResumed: EVREventType = 1416;
pub const EVREventType_VREvent_Compositor_OutOfVideoMemory: EVREventType = 1417;
pub const EVREventType_VREvent_Compositor_DisplayModeNotSupported: EVREventType = 1418;
pub const EVREventType_VREvent_Compositor_StageOverrideReady: EVREventType = 1419;
pub const EVREventType_VREvent_Compositor_RequestDisconnectReconnect: EVREventType = 1420;
pub const EVREventType_VREvent_TrackedCamera_StartVideoStream: EVREventType = 1500;
pub const EVREventType_VREvent_TrackedCamera_StopVideoStream: EVREventType = 1501;
pub const EVREventType_VREvent_TrackedCamera_PauseVideoStream: EVREventType = 1502;
pub const EVREventType_VREvent_TrackedCamera_ResumeVideoStream: EVREventType = 1503;
pub const EVREventType_VREvent_TrackedCamera_EditingSurface: EVREventType = 1550;
pub const EVREventType_VREvent_PerformanceTest_EnableCapture: EVREventType = 1600;
pub const EVREventType_VREvent_PerformanceTest_DisableCapture: EVREventType = 1601;
pub const EVREventType_VREvent_PerformanceTest_FidelityLevel: EVREventType = 1602;
pub const EVREventType_VREvent_MessageOverlay_Closed: EVREventType = 1650;
pub const EVREventType_VREvent_MessageOverlayCloseRequested: EVREventType = 1651;
pub const EVREventType_VREvent_Input_HapticVibration: EVREventType = 1700;
pub const EVREventType_VREvent_Input_BindingLoadFailed: EVREventType = 1701;
pub const EVREventType_VREvent_Input_BindingLoadSuccessful: EVREventType = 1702;
pub const EVREventType_VREvent_Input_ActionManifestReloaded: EVREventType = 1703;
pub const EVREventType_VREvent_Input_ActionManifestLoadFailed: EVREventType = 1704;
pub const EVREventType_VREvent_Input_ProgressUpdate: EVREventType = 1705;
pub const EVREventType_VREvent_Input_TrackerActivated: EVREventType = 1706;
pub const EVREventType_VREvent_Input_BindingsUpdated: EVREventType = 1707;
pub const EVREventType_VREvent_Input_BindingSubscriptionChanged: EVREventType = 1708;
pub const EVREventType_VREvent_SpatialAnchors_PoseUpdated: EVREventType = 1800;
pub const EVREventType_VREvent_SpatialAnchors_DescriptorUpdated: EVREventType = 1801;
pub const EVREventType_VREvent_SpatialAnchors_RequestPoseUpdate: EVREventType = 1802;
pub const EVREventType_VREvent_SpatialAnchors_RequestDescriptorUpdate: EVREventType = 1803;
pub const EVREventType_VREvent_SystemReport_Started: EVREventType = 1900;
pub const EVREventType_VREvent_Monitor_ShowHeadsetView: EVREventType = 2000;
pub const EVREventType_VREvent_Monitor_HideHeadsetView: EVREventType = 2001;
pub const EVREventType_VREvent_Audio_SetSpeakersVolume: EVREventType = 2100;
pub const EVREventType_VREvent_Audio_SetSpeakersMute: EVREventType = 2101;
pub const EVREventType_VREvent_Audio_SetMicrophoneVolume: EVREventType = 2102;
pub const EVREventType_VREvent_Audio_SetMicrophoneMute: EVREventType = 2103;
pub const EVREventType_VREvent_RenderModel_CountChanged: EVREventType = 2200;
pub const EVREventType_VREvent_VendorSpecific_Reserved_Start: EVREventType = 10000;
pub const EVREventType_VREvent_VendorSpecific_Reserved_End: EVREventType = 19999;
pub type EVREventType = ::core::ffi::c_uint;
pub const EDeviceActivityLevel_k_EDeviceActivityLevel_Unknown: EDeviceActivityLevel = -1;
pub const EDeviceActivityLevel_k_EDeviceActivityLevel_Idle: EDeviceActivityLevel = 0;
pub const EDeviceActivityLevel_k_EDeviceActivityLevel_UserInteraction: EDeviceActivityLevel = 1;
pub const EDeviceActivityLevel_k_EDeviceActivityLevel_UserInteraction_Timeout:
    EDeviceActivityLevel = 2;
pub const EDeviceActivityLevel_k_EDeviceActivityLevel_Standby: EDeviceActivityLevel = 3;
pub const EDeviceActivityLevel_k_EDeviceActivityLevel_Idle_Timeout: EDeviceActivityLevel = 4;
pub type EDeviceActivityLevel = ::core::ffi::c_int;
pub const EVRButtonId_k_EButton_System: EVRButtonId = 0;
pub const EVRButtonId_k_EButton_ApplicationMenu: EVRButtonId = 1;
pub const EVRButtonId_k_EButton_Grip: EVRButtonId = 2;
pub const EVRButtonId_k_EButton_DPad_Left: EVRButtonId = 3;
pub const EVRButtonId_k_EButton_DPad_Up: EVRButtonId = 4;
pub const EVRButtonId_k_EButton_DPad_Right: EVRButtonId = 5;
pub const EVRButtonId_k_EButton_DPad_Down: EVRButtonId = 6;
pub const EVRButtonId_k_EButton_A: EVRButtonId = 7;
pub const EVRButtonId_k_EButton_ProximitySensor: EVRButtonId = 31;
pub const EVRButtonId_k_EButton_Axis0: EVRButtonId = 32;
pub const EVRButtonId_k_EButton_Axis1: EVRButtonId = 33;
pub const EVRButtonId_k_EButton_Axis2: EVRButtonId = 34;
pub const EVRButtonId_k_EButton_Axis3: EVRButtonId = 35;
pub const EVRButtonId_k_EButton_Axis4: EVRButtonId = 36;
pub const EVRButtonId_k_EButton_SteamVR_Touchpad: EVRButtonId = 32;
pub const EVRButtonId_k_EButton_SteamVR_Trigger: EVRButtonId = 33;
pub const EVRButtonId_k_EButton_Dashboard_Back: EVRButtonId = 2;
pub const EVRButtonId_k_EButton_IndexController_A: EVRButtonId = 2;
pub const EVRButtonId_k_EButton_IndexController_B: EVRButtonId = 1;
pub const EVRButtonId_k_EButton_IndexController_JoyStick: EVRButtonId = 35;
pub const EVRButtonId_k_EButton_Reserved0: EVRButtonId = 50;
pub const EVRButtonId_k_EButton_Reserved1: EVRButtonId = 51;
pub const EVRButtonId_k_EButton_Max: EVRButtonId = 64;
pub type EVRButtonId = ::core::ffi::c_uint;
pub const EVRMouseButton_VRMouseButton_Left: EVRMouseButton = 1;
pub const EVRMouseButton_VRMouseButton_Right: EVRMouseButton = 2;
pub const EVRMouseButton_VRMouseButton_Middle: EVRMouseButton = 4;
pub type EVRMouseButton = ::core::ffi::c_uint;
pub const EShowUIType_ShowUI_ControllerBinding: EShowUIType = 0;
pub const EShowUIType_ShowUI_ManageTrackers: EShowUIType = 1;
pub const EShowUIType_ShowUI_Pairing: EShowUIType = 3;
pub const EShowUIType_ShowUI_Settings: EShowUIType = 4;
pub const EShowUIType_ShowUI_DebugCommands: EShowUIType = 5;
pub const EShowUIType_ShowUI_FullControllerBinding: EShowUIType = 6;
pub const EShowUIType_ShowUI_ManageDrivers: EShowUIType = 7;
pub type EShowUIType = ::core::ffi::c_uint;
pub const EHDCPError_HDCPError_None: EHDCPError = 0;
pub const EHDCPError_HDCPError_LinkLost: EHDCPError = 1;
pub const EHDCPError_HDCPError_Tampered: EHDCPError = 2;
pub const EHDCPError_HDCPError_DeviceRevoked: EHDCPError = 3;
pub const EHDCPError_HDCPError_Unknown: EHDCPError = 4;
pub type EHDCPError = ::core::ffi::c_uint;
pub const EVRComponentProperty_VRComponentProperty_IsStatic: EVRComponentProperty = 1;
pub const EVRComponentProperty_VRComponentProperty_IsVisible: EVRComponentProperty = 2;
pub const EVRComponentProperty_VRComponentProperty_IsTouched: EVRComponentProperty = 4;
pub const EVRComponentProperty_VRComponentProperty_IsPressed: EVRComponentProperty = 8;
pub const EVRComponentProperty_VRComponentProperty_IsScrolled: EVRComponentProperty = 16;
pub const EVRComponentProperty_VRComponentProperty_IsHighlighted: EVRComponentProperty = 32;
pub type EVRComponentProperty = ::core::ffi::c_uint;
pub const EVRInputError_VRInputError_None: EVRInputError = 0;
pub const EVRInputError_VRInputError_NameNotFound: EVRInputError = 1;
pub const EVRInputError_VRInputError_WrongType: EVRInputError = 2;
pub const EVRInputError_VRInputError_InvalidHandle: EVRInputError = 3;
pub const EVRInputError_VRInputError_InvalidParam: EVRInputError = 4;
pub const EVRInputError_VRInputError_NoSteam: EVRInputError = 5;
pub const EVRInputError_VRInputError_MaxCapacityReached: EVRInputError = 6;
pub const EVRInputError_VRInputError_IPCError: EVRInputError = 7;
pub const EVRInputError_VRInputError_NoActiveActionSet: EVRInputError = 8;
pub const EVRInputError_VRInputError_InvalidDevice: EVRInputError = 9;
pub const EVRInputError_VRInputError_InvalidSkeleton: EVRInputError = 10;
pub const EVRInputError_VRInputError_InvalidBoneCount: EVRInputError = 11;
pub const EVRInputError_VRInputError_InvalidCompressedData: EVRInputError = 12;
pub const EVRInputError_VRInputError_NoData: EVRInputError = 13;
pub const EVRInputError_VRInputError_BufferTooSmall: EVRInputError = 14;
pub const EVRInputError_VRInputError_MismatchedActionManifest: EVRInputError = 15;
pub const EVRInputError_VRInputError_MissingSkeletonData: EVRInputError = 16;
pub const EVRInputError_VRInputError_InvalidBoneIndex: EVRInputError = 17;
pub const EVRInputError_VRInputError_InvalidPriority: EVRInputError = 18;
pub const EVRInputError_VRInputError_PermissionDenied: EVRInputError = 19;
pub const EVRInputError_VRInputError_InvalidRenderModel: EVRInputError = 20;
pub type EVRInputError = ::core::ffi::c_uint;
pub const EVRSpatialAnchorError_VRSpatialAnchorError_Success: EVRSpatialAnchorError = 0;
pub const EVRSpatialAnchorError_VRSpatialAnchorError_Internal: EVRSpatialAnchorError = 1;
pub const EVRSpatialAnchorError_VRSpatialAnchorError_UnknownHandle: EVRSpatialAnchorError = 2;
pub const EVRSpatialAnchorError_VRSpatialAnchorError_ArrayTooSmall: EVRSpatialAnchorError = 3;
pub const EVRSpatialAnchorError_VRSpatialAnchorError_InvalidDescriptorChar: EVRSpatialAnchorError =
    4;
pub const EVRSpatialAnchorError_VRSpatialAnchorError_NotYetAvailable: EVRSpatialAnchorError = 5;
pub const EVRSpatialAnchorError_VRSpatialAnchorError_NotAvailableInThisUniverse:
    EVRSpatialAnchorError = 6;
pub const EVRSpatialAnchorError_VRSpatialAnchorError_PermanentlyUnavailable: EVRSpatialAnchorError =
    7;
pub const EVRSpatialAnchorError_VRSpatialAnchorError_WrongDriver: EVRSpatialAnchorError = 8;
pub const EVRSpatialAnchorError_VRSpatialAnchorError_DescriptorTooLong: EVRSpatialAnchorError = 9;
pub const EVRSpatialAnchorError_VRSpatialAnchorError_Unknown: EVRSpatialAnchorError = 10;
pub const EVRSpatialAnchorError_VRSpatialAnchorError_NoRoomCalibration: EVRSpatialAnchorError = 11;
pub const EVRSpatialAnchorError_VRSpatialAnchorError_InvalidArgument: EVRSpatialAnchorError = 12;
pub const EVRSpatialAnchorError_VRSpatialAnchorError_UnknownDriver: EVRSpatialAnchorError = 13;
pub type EVRSpatialAnchorError = ::core::ffi::c_uint;
pub const EHiddenAreaMeshType_k_eHiddenAreaMesh_Standard: EHiddenAreaMeshType = 0;
pub const EHiddenAreaMeshType_k_eHiddenAreaMesh_Inverse: EHiddenAreaMeshType = 1;
pub const EHiddenAreaMeshType_k_eHiddenAreaMesh_LineLoop: EHiddenAreaMeshType = 2;
pub const EHiddenAreaMeshType_k_eHiddenAreaMesh_Max: EHiddenAreaMeshType = 3;
pub type EHiddenAreaMeshType = ::core::ffi::c_uint;
pub const EVRControllerAxisType_k_eControllerAxis_None: EVRControllerAxisType = 0;
pub const EVRControllerAxisType_k_eControllerAxis_TrackPad: EVRControllerAxisType = 1;
pub const EVRControllerAxisType_k_eControllerAxis_Joystick: EVRControllerAxisType = 2;
pub const EVRControllerAxisType_k_eControllerAxis_Trigger: EVRControllerAxisType = 3;
pub type EVRControllerAxisType = ::core::ffi::c_uint;
pub const EVRControllerEventOutputType_ControllerEventOutput_OSEvents:
    EVRControllerEventOutputType = 0;
pub const EVRControllerEventOutputType_ControllerEventOutput_VREvents:
    EVRControllerEventOutputType = 1;
pub type EVRControllerEventOutputType = ::core::ffi::c_uint;
pub const EVROverlayError_VROverlayError_None: EVROverlayError = 0;
pub const EVROverlayError_VROverlayError_UnknownOverlay: EVROverlayError = 10;
pub const EVROverlayError_VROverlayError_InvalidHandle: EVROverlayError = 11;
pub const EVROverlayError_VROverlayError_PermissionDenied: EVROverlayError = 12;
pub const EVROverlayError_VROverlayError_OverlayLimitExceeded: EVROverlayError = 13;
pub const EVROverlayError_VROverlayError_WrongVisibilityType: EVROverlayError = 14;
pub const EVROverlayError_VROverlayError_KeyTooLong: EVROverlayError = 15;
pub const EVROverlayError_VROverlayError_NameTooLong: EVROverlayError = 16;
pub const EVROverlayError_VROverlayError_KeyInUse: EVROverlayError = 17;
pub const EVROverlayError_VROverlayError_WrongTransformType: EVROverlayError = 18;
pub const EVROverlayError_VROverlayError_InvalidTrackedDevice: EVROverlayError = 19;
pub const EVROverlayError_VROverlayError_InvalidParameter: EVROverlayError = 20;
pub const EVROverlayError_VROverlayError_ThumbnailCantBeDestroyed: EVROverlayError = 21;
pub const EVROverlayError_VROverlayError_ArrayTooSmall: EVROverlayError = 22;
pub const EVROverlayError_VROverlayError_RequestFailed: EVROverlayError = 23;
pub const EVROverlayError_VROverlayError_InvalidTexture: EVROverlayError = 24;
pub const EVROverlayError_VROverlayError_UnableToLoadFile: EVROverlayError = 25;
pub const EVROverlayError_VROverlayError_KeyboardAlreadyInUse: EVROverlayError = 26;
pub const EVROverlayError_VROverlayError_NoNeighbor: EVROverlayError = 27;
pub const EVROverlayError_VROverlayError_TooManyMaskPrimitives: EVROverlayError = 29;
pub const EVROverlayError_VROverlayError_BadMaskPrimitive: EVROverlayError = 30;
pub const EVROverlayError_VROverlayError_TextureAlreadyLocked: EVROverlayError = 31;
pub const EVROverlayError_VROverlayError_TextureLockCapacityReached: EVROverlayError = 32;
pub const EVROverlayError_VROverlayError_TextureNotLocked: EVROverlayError = 33;
pub const EVROverlayError_VROverlayError_TimedOut: EVROverlayError = 34;
pub type EVROverlayError = ::core::ffi::c_uint;
pub const EVRApplicationType_VRApplication_Other: EVRApplicationType = 0;
pub const EVRApplicationType_VRApplication_Scene: EVRApplicationType = 1;
pub const EVRApplicationType_VRApplication_Overlay: EVRApplicationType = 2;
pub const EVRApplicationType_VRApplication_Background: EVRApplicationType = 3;
pub const EVRApplicationType_VRApplication_Utility: EVRApplicationType = 4;
pub const EVRApplicationType_VRApplication_VRMonitor: EVRApplicationType = 5;
pub const EVRApplicationType_VRApplication_SteamWatchdog: EVRApplicationType = 6;
pub const EVRApplicationType_VRApplication_Bootstrapper: EVRApplicationType = 7;
pub const EVRApplicationType_VRApplication_WebHelper: EVRApplicationType = 8;
pub const EVRApplicationType_VRApplication_OpenXRInstance: EVRApplicationType = 9;
pub const EVRApplicationType_VRApplication_OpenXRScene: EVRApplicationType = 10;
pub const EVRApplicationType_VRApplication_OpenXROverlay: EVRApplicationType = 11;
pub const EVRApplicationType_VRApplication_Prism: EVRApplicationType = 12;
pub const EVRApplicationType_VRApplication_RoomView: EVRApplicationType = 13;
pub const EVRApplicationType_VRApplication_Max: EVRApplicationType = 14;
pub type EVRApplicationType = ::core::ffi::c_uint;
pub const EVRFirmwareError_VRFirmwareError_None: EVRFirmwareError = 0;
pub const EVRFirmwareError_VRFirmwareError_Success: EVRFirmwareError = 1;
pub const EVRFirmwareError_VRFirmwareError_Fail: EVRFirmwareError = 2;
pub type EVRFirmwareError = ::core::ffi::c_uint;
pub const EVRNotificationError_VRNotificationError_OK: EVRNotificationError = 0;
pub const EVRNotificationError_VRNotificationError_InvalidNotificationId: EVRNotificationError =
    100;
pub const EVRNotificationError_VRNotificationError_NotificationQueueFull: EVRNotificationError =
    101;
pub const EVRNotificationError_VRNotificationError_InvalidOverlayHandle: EVRNotificationError = 102;
pub const EVRNotificationError_VRNotificationError_SystemWithUserValueAlreadyExists:
    EVRNotificationError = 103;
pub const EVRNotificationError_VRNotificationError_ServiceUnavailable: EVRNotificationError = 104;
pub type EVRNotificationError = ::core::ffi::c_uint;
pub const EVRSkeletalMotionRange_VRSkeletalMotionRange_WithController: EVRSkeletalMotionRange = 0;
pub const EVRSkeletalMotionRange_VRSkeletalMotionRange_WithoutController: EVRSkeletalMotionRange =
    1;
pub type EVRSkeletalMotionRange = ::core::ffi::c_uint;
pub const EVRSkeletalTrackingLevel_VRSkeletalTracking_Estimated: EVRSkeletalTrackingLevel = 0;
pub const EVRSkeletalTrackingLevel_VRSkeletalTracking_Partial: EVRSkeletalTrackingLevel = 1;
pub const EVRSkeletalTrackingLevel_VRSkeletalTracking_Full: EVRSkeletalTrackingLevel = 2;
pub const EVRSkeletalTrackingLevel_VRSkeletalTrackingLevel_Count: EVRSkeletalTrackingLevel = 3;
pub const EVRSkeletalTrackingLevel_VRSkeletalTrackingLevel_Max: EVRSkeletalTrackingLevel = 2;
pub type EVRSkeletalTrackingLevel = ::core::ffi::c_uint;
pub const EVRInitError_VRInitError_None: EVRInitError = 0;
pub const EVRInitError_VRInitError_Unknown: EVRInitError = 1;
pub const EVRInitError_VRInitError_Init_InstallationNotFound: EVRInitError = 100;
pub const EVRInitError_VRInitError_Init_InstallationCorrupt: EVRInitError = 101;
pub const EVRInitError_VRInitError_Init_VRClientDLLNotFound: EVRInitError = 102;
pub const EVRInitError_VRInitError_Init_FileNotFound: EVRInitError = 103;
pub const EVRInitError_VRInitError_Init_FactoryNotFound: EVRInitError = 104;
pub const EVRInitError_VRInitError_Init_InterfaceNotFound: EVRInitError = 105;
pub const EVRInitError_VRInitError_Init_InvalidInterface: EVRInitError = 106;
pub const EVRInitError_VRInitError_Init_UserConfigDirectoryInvalid: EVRInitError = 107;
pub const EVRInitError_VRInitError_Init_HmdNotFound: EVRInitError = 108;
pub const EVRInitError_VRInitError_Init_NotInitialized: EVRInitError = 109;
pub const EVRInitError_VRInitError_Init_PathRegistryNotFound: EVRInitError = 110;
pub const EVRInitError_VRInitError_Init_NoConfigPath: EVRInitError = 111;
pub const EVRInitError_VRInitError_Init_NoLogPath: EVRInitError = 112;
pub const EVRInitError_VRInitError_Init_PathRegistryNotWritable: EVRInitError = 113;
pub const EVRInitError_VRInitError_Init_AppInfoInitFailed: EVRInitError = 114;
pub const EVRInitError_VRInitError_Init_Retry: EVRInitError = 115;
pub const EVRInitError_VRInitError_Init_InitCanceledByUser: EVRInitError = 116;
pub const EVRInitError_VRInitError_Init_AnotherAppLaunching: EVRInitError = 117;
pub const EVRInitError_VRInitError_Init_SettingsInitFailed: EVRInitError = 118;
pub const EVRInitError_VRInitError_Init_ShuttingDown: EVRInitError = 119;
pub const EVRInitError_VRInitError_Init_TooManyObjects: EVRInitError = 120;
pub const EVRInitError_VRInitError_Init_NoServerForBackgroundApp: EVRInitError = 121;
pub const EVRInitError_VRInitError_Init_NotSupportedWithCompositor: EVRInitError = 122;
pub const EVRInitError_VRInitError_Init_NotAvailableToUtilityApps: EVRInitError = 123;
pub const EVRInitError_VRInitError_Init_Internal: EVRInitError = 124;
pub const EVRInitError_VRInitError_Init_HmdDriverIdIsNone: EVRInitError = 125;
pub const EVRInitError_VRInitError_Init_HmdNotFoundPresenceFailed: EVRInitError = 126;
pub const EVRInitError_VRInitError_Init_VRMonitorNotFound: EVRInitError = 127;
pub const EVRInitError_VRInitError_Init_VRMonitorStartupFailed: EVRInitError = 128;
pub const EVRInitError_VRInitError_Init_LowPowerWatchdogNotSupported: EVRInitError = 129;
pub const EVRInitError_VRInitError_Init_InvalidApplicationType: EVRInitError = 130;
pub const EVRInitError_VRInitError_Init_NotAvailableToWatchdogApps: EVRInitError = 131;
pub const EVRInitError_VRInitError_Init_WatchdogDisabledInSettings: EVRInitError = 132;
pub const EVRInitError_VRInitError_Init_VRDashboardNotFound: EVRInitError = 133;
pub const EVRInitError_VRInitError_Init_VRDashboardStartupFailed: EVRInitError = 134;
pub const EVRInitError_VRInitError_Init_VRHomeNotFound: EVRInitError = 135;
pub const EVRInitError_VRInitError_Init_VRHomeStartupFailed: EVRInitError = 136;
pub const EVRInitError_VRInitError_Init_RebootingBusy: EVRInitError = 137;
pub const EVRInitError_VRInitError_Init_FirmwareUpdateBusy: EVRInitError = 138;
pub const EVRInitError_VRInitError_Init_FirmwareRecoveryBusy: EVRInitError = 139;
pub const EVRInitError_VRInitError_Init_USBServiceBusy: EVRInitError = 140;
pub const EVRInitError_VRInitError_Init_VRWebHelperStartupFailed: EVRInitError = 141;
pub const EVRInitError_VRInitError_Init_TrackerManagerInitFailed: EVRInitError = 142;
pub const EVRInitError_VRInitError_Init_AlreadyRunning: EVRInitError = 143;
pub const EVRInitError_VRInitError_Init_FailedForVrMonitor: EVRInitError = 144;
pub const EVRInitError_VRInitError_Init_PropertyManagerInitFailed: EVRInitError = 145;
pub const EVRInitError_VRInitError_Init_WebServerFailed: EVRInitError = 146;
pub const EVRInitError_VRInitError_Init_IllegalTypeTransition: EVRInitError = 147;
pub const EVRInitError_VRInitError_Init_MismatchedRuntimes: EVRInitError = 148;
pub const EVRInitError_VRInitError_Init_InvalidProcessId: EVRInitError = 149;
pub const EVRInitError_VRInitError_Init_VRServiceStartupFailed: EVRInitError = 150;
pub const EVRInitError_VRInitError_Init_PrismNeedsNewDrivers: EVRInitError = 151;
pub const EVRInitError_VRInitError_Init_PrismStartupTimedOut: EVRInitError = 152;
pub const EVRInitError_VRInitError_Init_CouldNotStartPrism: EVRInitError = 153;
pub const EVRInitError_VRInitError_Init_PrismClientInitFailed: EVRInitError = 154;
pub const EVRInitError_VRInitError_Init_PrismClientStartFailed: EVRInitError = 155;
pub const EVRInitError_VRInitError_Init_PrismExitedUnexpectedly: EVRInitError = 156;
pub const EVRInitError_VRInitError_Init_BadLuid: EVRInitError = 157;
pub const EVRInitError_VRInitError_Init_NoServerForAppContainer: EVRInitError = 158;
pub const EVRInitError_VRInitError_Init_DuplicateBootstrapper: EVRInitError = 159;
pub const EVRInitError_VRInitError_Init_VRDashboardServicePending: EVRInitError = 160;
pub const EVRInitError_VRInitError_Init_VRDashboardServiceTimeout: EVRInitError = 161;
pub const EVRInitError_VRInitError_Init_VRDashboardServiceStopped: EVRInitError = 162;
pub const EVRInitError_VRInitError_Init_VRDashboardAlreadyStarted: EVRInitError = 163;
pub const EVRInitError_VRInitError_Init_VRDashboardCopyFailed: EVRInitError = 164;
pub const EVRInitError_VRInitError_Init_VRDashboardTokenFailure: EVRInitError = 165;
pub const EVRInitError_VRInitError_Init_VRDashboardEnvironmentFailure: EVRInitError = 166;
pub const EVRInitError_VRInitError_Init_VRDashboardPathFailure: EVRInitError = 167;
pub const EVRInitError_VRInitError_Init_InstallationTooOld: EVRInitError = 168;
pub const EVRInitError_VRInitError_Init_ClientVersionAlreadyProvided: EVRInitError = 169;
pub const EVRInitError_VRInitError_Driver_Failed: EVRInitError = 200;
pub const EVRInitError_VRInitError_Driver_Unknown: EVRInitError = 201;
pub const EVRInitError_VRInitError_Driver_HmdUnknown: EVRInitError = 202;
pub const EVRInitError_VRInitError_Driver_NotLoaded: EVRInitError = 203;
pub const EVRInitError_VRInitError_Driver_RuntimeOutOfDate: EVRInitError = 204;
pub const EVRInitError_VRInitError_Driver_HmdInUse: EVRInitError = 205;
pub const EVRInitError_VRInitError_Driver_NotCalibrated: EVRInitError = 206;
pub const EVRInitError_VRInitError_Driver_CalibrationInvalid: EVRInitError = 207;
pub const EVRInitError_VRInitError_Driver_HmdDisplayNotFound: EVRInitError = 208;
pub const EVRInitError_VRInitError_Driver_TrackedDeviceInterfaceUnknown: EVRInitError = 209;
pub const EVRInitError_VRInitError_Driver_HmdDriverIdOutOfBounds: EVRInitError = 211;
pub const EVRInitError_VRInitError_Driver_HmdDisplayMirrored: EVRInitError = 212;
pub const EVRInitError_VRInitError_Driver_HmdDisplayNotFoundLaptop: EVRInitError = 213;
pub const EVRInitError_VRInitError_Driver_PeerDriverNotInstalled: EVRInitError = 214;
pub const EVRInitError_VRInitError_Driver_WirelessHmdNotConnected: EVRInitError = 215;
pub const EVRInitError_VRInitError_IPC_ServerInitFailed: EVRInitError = 300;
pub const EVRInitError_VRInitError_IPC_ConnectFailed: EVRInitError = 301;
pub const EVRInitError_VRInitError_IPC_SharedStateInitFailed: EVRInitError = 302;
pub const EVRInitError_VRInitError_IPC_CompositorInitFailed: EVRInitError = 303;
pub const EVRInitError_VRInitError_IPC_MutexInitFailed: EVRInitError = 304;
pub const EVRInitError_VRInitError_IPC_Failed: EVRInitError = 305;
pub const EVRInitError_VRInitError_IPC_CompositorConnectFailed: EVRInitError = 306;
pub const EVRInitError_VRInitError_IPC_CompositorInvalidConnectResponse: EVRInitError = 307;
pub const EVRInitError_VRInitError_IPC_ConnectFailedAfterMultipleAttempts: EVRInitError = 308;
pub const EVRInitError_VRInitError_IPC_ConnectFailedAfterTargetExited: EVRInitError = 309;
pub const EVRInitError_VRInitError_IPC_NamespaceUnavailable: EVRInitError = 310;
pub const EVRInitError_VRInitError_Compositor_Failed: EVRInitError = 400;
pub const EVRInitError_VRInitError_Compositor_D3D11HardwareRequired: EVRInitError = 401;
pub const EVRInitError_VRInitError_Compositor_FirmwareRequiresUpdate: EVRInitError = 402;
pub const EVRInitError_VRInitError_Compositor_OverlayInitFailed: EVRInitError = 403;
pub const EVRInitError_VRInitError_Compositor_ScreenshotsInitFailed: EVRInitError = 404;
pub const EVRInitError_VRInitError_Compositor_UnableToCreateDevice: EVRInitError = 405;
pub const EVRInitError_VRInitError_Compositor_SharedStateIsNull: EVRInitError = 406;
pub const EVRInitError_VRInitError_Compositor_NotificationManagerIsNull: EVRInitError = 407;
pub const EVRInitError_VRInitError_Compositor_ResourceManagerClientIsNull: EVRInitError = 408;
pub const EVRInitError_VRInitError_Compositor_MessageOverlaySharedStateInitFailure: EVRInitError =
    409;
pub const EVRInitError_VRInitError_Compositor_PropertiesInterfaceIsNull: EVRInitError = 410;
pub const EVRInitError_VRInitError_Compositor_CreateFullscreenWindowFailed: EVRInitError = 411;
pub const EVRInitError_VRInitError_Compositor_SettingsInterfaceIsNull: EVRInitError = 412;
pub const EVRInitError_VRInitError_Compositor_FailedToShowWindow: EVRInitError = 413;
pub const EVRInitError_VRInitError_Compositor_DistortInterfaceIsNull: EVRInitError = 414;
pub const EVRInitError_VRInitError_Compositor_DisplayFrequencyFailure: EVRInitError = 415;
pub const EVRInitError_VRInitError_Compositor_RendererInitializationFailed: EVRInitError = 416;
pub const EVRInitError_VRInitError_Compositor_DXGIFactoryInterfaceIsNull: EVRInitError = 417;
pub const EVRInitError_VRInitError_Compositor_DXGIFactoryCreateFailed: EVRInitError = 418;
pub const EVRInitError_VRInitError_Compositor_DXGIFactoryQueryFailed: EVRInitError = 419;
pub const EVRInitError_VRInitError_Compositor_InvalidAdapterDesktop: EVRInitError = 420;
pub const EVRInitError_VRInitError_Compositor_InvalidHmdAttachment: EVRInitError = 421;
pub const EVRInitError_VRInitError_Compositor_InvalidOutputDesktop: EVRInitError = 422;
pub const EVRInitError_VRInitError_Compositor_InvalidDeviceProvided: EVRInitError = 423;
pub const EVRInitError_VRInitError_Compositor_D3D11RendererInitializationFailed: EVRInitError = 424;
pub const EVRInitError_VRInitError_Compositor_FailedToFindDisplayMode: EVRInitError = 425;
pub const EVRInitError_VRInitError_Compositor_FailedToCreateSwapChain: EVRInitError = 426;
pub const EVRInitError_VRInitError_Compositor_FailedToGetBackBuffer: EVRInitError = 427;
pub const EVRInitError_VRInitError_Compositor_FailedToCreateRenderTarget: EVRInitError = 428;
pub const EVRInitError_VRInitError_Compositor_FailedToCreateDXGI2SwapChain: EVRInitError = 429;
pub const EVRInitError_VRInitError_Compositor_FailedtoGetDXGI2BackBuffer: EVRInitError = 430;
pub const EVRInitError_VRInitError_Compositor_FailedToCreateDXGI2RenderTarget: EVRInitError = 431;
pub const EVRInitError_VRInitError_Compositor_FailedToGetDXGIDeviceInterface: EVRInitError = 432;
pub const EVRInitError_VRInitError_Compositor_SelectDisplayMode: EVRInitError = 433;
pub const EVRInitError_VRInitError_Compositor_FailedToCreateNvAPIRenderTargets: EVRInitError = 434;
pub const EVRInitError_VRInitError_Compositor_NvAPISetDisplayMode: EVRInitError = 435;
pub const EVRInitError_VRInitError_Compositor_FailedToCreateDirectModeDisplay: EVRInitError = 436;
pub const EVRInitError_VRInitError_Compositor_InvalidHmdPropertyContainer: EVRInitError = 437;
pub const EVRInitError_VRInitError_Compositor_UpdateDisplayFrequency: EVRInitError = 438;
pub const EVRInitError_VRInitError_Compositor_CreateRasterizerState: EVRInitError = 439;
pub const EVRInitError_VRInitError_Compositor_CreateWireframeRasterizerState: EVRInitError = 440;
pub const EVRInitError_VRInitError_Compositor_CreateSamplerState: EVRInitError = 441;
pub const EVRInitError_VRInitError_Compositor_CreateClampToBorderSamplerState: EVRInitError = 442;
pub const EVRInitError_VRInitError_Compositor_CreateAnisoSamplerState: EVRInitError = 443;
pub const EVRInitError_VRInitError_Compositor_CreateOverlaySamplerState: EVRInitError = 444;
pub const EVRInitError_VRInitError_Compositor_CreatePanoramaSamplerState: EVRInitError = 445;
pub const EVRInitError_VRInitError_Compositor_CreateFontSamplerState: EVRInitError = 446;
pub const EVRInitError_VRInitError_Compositor_CreateNoBlendState: EVRInitError = 447;
pub const EVRInitError_VRInitError_Compositor_CreateBlendState: EVRInitError = 448;
pub const EVRInitError_VRInitError_Compositor_CreateAlphaBlendState: EVRInitError = 449;
pub const EVRInitError_VRInitError_Compositor_CreateBlendStateMaskR: EVRInitError = 450;
pub const EVRInitError_VRInitError_Compositor_CreateBlendStateMaskG: EVRInitError = 451;
pub const EVRInitError_VRInitError_Compositor_CreateBlendStateMaskB: EVRInitError = 452;
pub const EVRInitError_VRInitError_Compositor_CreateDepthStencilState: EVRInitError = 453;
pub const EVRInitError_VRInitError_Compositor_CreateDepthStencilStateNoWrite: EVRInitError = 454;
pub const EVRInitError_VRInitError_Compositor_CreateDepthStencilStateNoDepth: EVRInitError = 455;
pub const EVRInitError_VRInitError_Compositor_CreateFlushTexture: EVRInitError = 456;
pub const EVRInitError_VRInitError_Compositor_CreateDistortionSurfaces: EVRInitError = 457;
pub const EVRInitError_VRInitError_Compositor_CreateConstantBuffer: EVRInitError = 458;
pub const EVRInitError_VRInitError_Compositor_CreateHmdPoseConstantBuffer: EVRInitError = 459;
pub const EVRInitError_VRInitError_Compositor_CreateHmdPoseStagingConstantBuffer: EVRInitError =
    460;
pub const EVRInitError_VRInitError_Compositor_CreateSharedFrameInfoConstantBuffer: EVRInitError =
    461;
pub const EVRInitError_VRInitError_Compositor_CreateOverlayConstantBuffer: EVRInitError = 462;
pub const EVRInitError_VRInitError_Compositor_CreateSceneTextureIndexConstantBuffer: EVRInitError =
    463;
pub const EVRInitError_VRInitError_Compositor_CreateReadableSceneTextureIndexConstantBuffer:
    EVRInitError = 464;
pub const EVRInitError_VRInitError_Compositor_CreateLayerGraphicsTextureIndexConstantBuffer:
    EVRInitError = 465;
pub const EVRInitError_VRInitError_Compositor_CreateLayerComputeTextureIndexConstantBuffer:
    EVRInitError = 466;
pub const EVRInitError_VRInitError_Compositor_CreateLayerComputeSceneTextureIndexConstantBuffer:
    EVRInitError = 467;
pub const EVRInitError_VRInitError_Compositor_CreateComputeHmdPoseConstantBuffer: EVRInitError =
    468;
pub const EVRInitError_VRInitError_Compositor_CreateGeomConstantBuffer: EVRInitError = 469;
pub const EVRInitError_VRInitError_Compositor_CreatePanelMaskConstantBuffer: EVRInitError = 470;
pub const EVRInitError_VRInitError_Compositor_CreatePixelSimUBO: EVRInitError = 471;
pub const EVRInitError_VRInitError_Compositor_CreateMSAARenderTextures: EVRInitError = 472;
pub const EVRInitError_VRInitError_Compositor_CreateResolveRenderTextures: EVRInitError = 473;
pub const EVRInitError_VRInitError_Compositor_CreateComputeResolveRenderTextures: EVRInitError =
    474;
pub const EVRInitError_VRInitError_Compositor_CreateDriverDirectModeResolveTextures: EVRInitError =
    475;
pub const EVRInitError_VRInitError_Compositor_OpenDriverDirectModeResolveTextures: EVRInitError =
    476;
pub const EVRInitError_VRInitError_Compositor_CreateFallbackSyncTexture: EVRInitError = 477;
pub const EVRInitError_VRInitError_Compositor_ShareFallbackSyncTexture: EVRInitError = 478;
pub const EVRInitError_VRInitError_Compositor_CreateOverlayIndexBuffer: EVRInitError = 479;
pub const EVRInitError_VRInitError_Compositor_CreateOverlayVertexBuffer: EVRInitError = 480;
pub const EVRInitError_VRInitError_Compositor_CreateTextVertexBuffer: EVRInitError = 481;
pub const EVRInitError_VRInitError_Compositor_CreateTextIndexBuffer: EVRInitError = 482;
pub const EVRInitError_VRInitError_Compositor_CreateMirrorTextures: EVRInitError = 483;
pub const EVRInitError_VRInitError_Compositor_CreateLastFrameRenderTexture: EVRInitError = 484;
pub const EVRInitError_VRInitError_Compositor_CreateMirrorOverlay: EVRInitError = 485;
pub const EVRInitError_VRInitError_Compositor_FailedToCreateVirtualDisplayBackbuffer: EVRInitError =
    486;
pub const EVRInitError_VRInitError_Compositor_DisplayModeNotSupported: EVRInitError = 487;
pub const EVRInitError_VRInitError_Compositor_CreateOverlayInvalidCall: EVRInitError = 488;
pub const EVRInitError_VRInitError_Compositor_CreateOverlayAlreadyInitialized: EVRInitError = 489;
pub const EVRInitError_VRInitError_Compositor_FailedToCreateMailbox: EVRInitError = 490;
pub const EVRInitError_VRInitError_Compositor_WindowInterfaceIsNull: EVRInitError = 491;
pub const EVRInitError_VRInitError_Compositor_SystemLayerCreateInstance: EVRInitError = 492;
pub const EVRInitError_VRInitError_Compositor_SystemLayerCreateSession: EVRInitError = 493;
pub const EVRInitError_VRInitError_Compositor_CreateInverseDistortUVs: EVRInitError = 494;
pub const EVRInitError_VRInitError_Compositor_CreateBackbufferDepth: EVRInitError = 495;
pub const EVRInitError_VRInitError_Compositor_CannotDRMLeaseDisplay: EVRInitError = 496;
pub const EVRInitError_VRInitError_Compositor_CannotConnectToDisplayServer: EVRInitError = 497;
pub const EVRInitError_VRInitError_Compositor_GnomeNoDRMLeasing: EVRInitError = 498;
pub const EVRInitError_VRInitError_Compositor_FailedToInitializeEncoder: EVRInitError = 499;
pub const EVRInitError_VRInitError_Compositor_CreateBlurTexture: EVRInitError = 500;
pub const EVRInitError_VRInitError_VendorSpecific_UnableToConnectToOculusRuntime: EVRInitError =
    1000;
pub const EVRInitError_VRInitError_VendorSpecific_WindowsNotInDevMode: EVRInitError = 1001;
pub const EVRInitError_VRInitError_VendorSpecific_OculusLinkNotEnabled: EVRInitError = 1002;
pub const EVRInitError_VRInitError_VendorSpecific_HmdFound_CantOpenDevice: EVRInitError = 1101;
pub const EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToRequestConfigStart:
    EVRInitError = 1102;
pub const EVRInitError_VRInitError_VendorSpecific_HmdFound_NoStoredConfig: EVRInitError = 1103;
pub const EVRInitError_VRInitError_VendorSpecific_HmdFound_ConfigTooBig: EVRInitError = 1104;
pub const EVRInitError_VRInitError_VendorSpecific_HmdFound_ConfigTooSmall: EVRInitError = 1105;
pub const EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToInitZLib: EVRInitError = 1106;
pub const EVRInitError_VRInitError_VendorSpecific_HmdFound_CantReadFirmwareVersion: EVRInitError =
    1107;
pub const EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToSendUserDataStart: EVRInitError =
    1108;
pub const EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToGetUserDataStart: EVRInitError =
    1109;
pub const EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToGetUserDataNext: EVRInitError =
    1110;
pub const EVRInitError_VRInitError_VendorSpecific_HmdFound_UserDataAddressRange: EVRInitError =
    1111;
pub const EVRInitError_VRInitError_VendorSpecific_HmdFound_UserDataError: EVRInitError = 1112;
pub const EVRInitError_VRInitError_VendorSpecific_HmdFound_ConfigFailedSanityCheck: EVRInitError =
    1113;
pub const EVRInitError_VRInitError_VendorSpecific_OculusRuntimeBadInstall: EVRInitError = 1114;
pub const EVRInitError_VRInitError_VendorSpecific_HmdFound_UnexpectedConfiguration_1: EVRInitError =
    1115;
pub const EVRInitError_VRInitError_VendorSpecific_Oasis_UnlockRequired: EVRInitError = 1150;
pub const EVRInitError_VRInitError_VendorSpecific_VRLink_OutdatedDriverMESA: EVRInitError = 1200;
pub const EVRInitError_VRInitError_VendorSpecific_VRLink_OutdatedDriverNVIDIA: EVRInitError = 1201;
pub const EVRInitError_VRInitError_VendorSpecific_VRLink_NoVideoSupport: EVRInitError = 1202;
pub const EVRInitError_VRInitError_Steam_SteamInstallationNotFound: EVRInitError = 2000;
pub const EVRInitError_VRInitError_LastError: EVRInitError = 2001;
pub type EVRInitError = ::core::ffi::c_uint;
pub const EVRScreenshotType_VRScreenshotType_None: EVRScreenshotType = 0;
pub const EVRScreenshotType_VRScreenshotType_Mono: EVRScreenshotType = 1;
pub const EVRScreenshotType_VRScreenshotType_Stereo: EVRScreenshotType = 2;
pub const EVRScreenshotType_VRScreenshotType_Cubemap: EVRScreenshotType = 3;
pub const EVRScreenshotType_VRScreenshotType_MonoPanorama: EVRScreenshotType = 4;
pub const EVRScreenshotType_VRScreenshotType_StereoPanorama: EVRScreenshotType = 5;
pub type EVRScreenshotType = ::core::ffi::c_uint;
pub const EVRScreenshotPropertyFilenames_VRScreenshotPropertyFilenames_Preview:
    EVRScreenshotPropertyFilenames = 0;
pub const EVRScreenshotPropertyFilenames_VRScreenshotPropertyFilenames_VR:
    EVRScreenshotPropertyFilenames = 1;
pub type EVRScreenshotPropertyFilenames = ::core::ffi::c_uint;
pub const EVRTrackedCameraError_VRTrackedCameraError_None: EVRTrackedCameraError = 0;
pub const EVRTrackedCameraError_VRTrackedCameraError_OperationFailed: EVRTrackedCameraError = 100;
pub const EVRTrackedCameraError_VRTrackedCameraError_InvalidHandle: EVRTrackedCameraError = 101;
pub const EVRTrackedCameraError_VRTrackedCameraError_InvalidFrameHeaderVersion:
    EVRTrackedCameraError = 102;
pub const EVRTrackedCameraError_VRTrackedCameraError_OutOfHandles: EVRTrackedCameraError = 103;
pub const EVRTrackedCameraError_VRTrackedCameraError_IPCFailure: EVRTrackedCameraError = 104;
pub const EVRTrackedCameraError_VRTrackedCameraError_NotSupportedForThisDevice:
    EVRTrackedCameraError = 105;
pub const EVRTrackedCameraError_VRTrackedCameraError_SharedMemoryFailure: EVRTrackedCameraError =
    106;
pub const EVRTrackedCameraError_VRTrackedCameraError_FrameBufferingFailure: EVRTrackedCameraError =
    107;
pub const EVRTrackedCameraError_VRTrackedCameraError_StreamSetupFailure: EVRTrackedCameraError =
    108;
pub const EVRTrackedCameraError_VRTrackedCameraError_InvalidGLTextureId: EVRTrackedCameraError =
    109;
pub const EVRTrackedCameraError_VRTrackedCameraError_InvalidSharedTextureHandle:
    EVRTrackedCameraError = 110;
pub const EVRTrackedCameraError_VRTrackedCameraError_FailedToGetGLTextureId: EVRTrackedCameraError =
    111;
pub const EVRTrackedCameraError_VRTrackedCameraError_SharedTextureFailure: EVRTrackedCameraError =
    112;
pub const EVRTrackedCameraError_VRTrackedCameraError_NoFrameAvailable: EVRTrackedCameraError = 113;
pub const EVRTrackedCameraError_VRTrackedCameraError_InvalidArgument: EVRTrackedCameraError = 114;
pub const EVRTrackedCameraError_VRTrackedCameraError_InvalidFrameBufferSize: EVRTrackedCameraError =
    115;
pub type EVRTrackedCameraError = ::core::ffi::c_uint;
pub const EVRTrackedCameraFrameLayout_Mono: EVRTrackedCameraFrameLayout = 1;
pub const EVRTrackedCameraFrameLayout_Stereo: EVRTrackedCameraFrameLayout = 2;
pub const EVRTrackedCameraFrameLayout_VerticalLayout: EVRTrackedCameraFrameLayout = 16;
pub const EVRTrackedCameraFrameLayout_HorizontalLayout: EVRTrackedCameraFrameLayout = 32;
pub type EVRTrackedCameraFrameLayout = ::core::ffi::c_uint;
pub const EVRTrackedCameraFrameType_VRTrackedCameraFrameType_Distorted: EVRTrackedCameraFrameType =
    0;
pub const EVRTrackedCameraFrameType_VRTrackedCameraFrameType_Undistorted:
    EVRTrackedCameraFrameType = 1;
pub const EVRTrackedCameraFrameType_VRTrackedCameraFrameType_MaximumUndistorted:
    EVRTrackedCameraFrameType = 2;
pub const EVRTrackedCameraFrameType_MAX_CAMERA_FRAME_TYPES: EVRTrackedCameraFrameType = 3;
pub type EVRTrackedCameraFrameType = ::core::ffi::c_uint;
pub const EVRDistortionFunctionType_VRDistortionFunctionType_None: EVRDistortionFunctionType = 0;
pub const EVRDistortionFunctionType_VRDistortionFunctionType_FTheta: EVRDistortionFunctionType = 1;
pub const EVRDistortionFunctionType_VRDistortionFunctionType_Extended_FTheta:
    EVRDistortionFunctionType = 2;
pub const EVRDistortionFunctionType_MAX_DISTORTION_FUNCTION_TYPES: EVRDistortionFunctionType = 3;
pub type EVRDistortionFunctionType = ::core::ffi::c_uint;
pub const EVRMuraCorrectionMode_Default: EVRMuraCorrectionMode = 0;
pub const EVRMuraCorrectionMode_NoCorrection: EVRMuraCorrectionMode = 1;
pub type EVRMuraCorrectionMode = ::core::ffi::c_uint;
pub const EVRDistortionChannel_Red: EVRDistortionChannel = 0;
pub const EVRDistortionChannel_Green: EVRDistortionChannel = 1;
pub const EVRDistortionChannel_Blue: EVRDistortionChannel = 2;
pub const EVRDistortionChannel_InverseRed: EVRDistortionChannel = 3;
pub const EVRDistortionChannel_InverseGreen: EVRDistortionChannel = 4;
pub const EVRDistortionChannel_InverseBlue: EVRDistortionChannel = 5;
pub const EVRDistortionChannel_Count: EVRDistortionChannel = 6;
pub type EVRDistortionChannel = ::core::ffi::c_uint;
pub const EVRApplicationError_VRApplicationError_None: EVRApplicationError = 0;
pub const EVRApplicationError_VRApplicationError_AppKeyAlreadyExists: EVRApplicationError = 100;
pub const EVRApplicationError_VRApplicationError_NoManifest: EVRApplicationError = 101;
pub const EVRApplicationError_VRApplicationError_NoApplication: EVRApplicationError = 102;
pub const EVRApplicationError_VRApplicationError_InvalidIndex: EVRApplicationError = 103;
pub const EVRApplicationError_VRApplicationError_UnknownApplication: EVRApplicationError = 104;
pub const EVRApplicationError_VRApplicationError_IPCFailed: EVRApplicationError = 105;
pub const EVRApplicationError_VRApplicationError_ApplicationAlreadyRunning: EVRApplicationError =
    106;
pub const EVRApplicationError_VRApplicationError_InvalidManifest: EVRApplicationError = 107;
pub const EVRApplicationError_VRApplicationError_InvalidApplication: EVRApplicationError = 108;
pub const EVRApplicationError_VRApplicationError_LaunchFailed: EVRApplicationError = 109;
pub const EVRApplicationError_VRApplicationError_ApplicationAlreadyStarting: EVRApplicationError =
    110;
pub const EVRApplicationError_VRApplicationError_LaunchInProgress: EVRApplicationError = 111;
pub const EVRApplicationError_VRApplicationError_OldApplicationQuitting: EVRApplicationError = 112;
pub const EVRApplicationError_VRApplicationError_TransitionAborted: EVRApplicationError = 113;
pub const EVRApplicationError_VRApplicationError_IsTemplate: EVRApplicationError = 114;
pub const EVRApplicationError_VRApplicationError_SteamVRIsExiting: EVRApplicationError = 115;
pub const EVRApplicationError_VRApplicationError_WaitingForChaperone: EVRApplicationError = 116;
pub const EVRApplicationError_VRApplicationError_BufferTooSmall: EVRApplicationError = 200;
pub const EVRApplicationError_VRApplicationError_PropertyNotSet: EVRApplicationError = 201;
pub const EVRApplicationError_VRApplicationError_UnknownProperty: EVRApplicationError = 202;
pub const EVRApplicationError_VRApplicationError_InvalidParameter: EVRApplicationError = 203;
pub const EVRApplicationError_VRApplicationError_NotImplemented: EVRApplicationError = 300;
pub type EVRApplicationError = ::core::ffi::c_uint;
pub const EVRApplicationProperty_VRApplicationProperty_Name_String: EVRApplicationProperty = 0;
pub const EVRApplicationProperty_VRApplicationProperty_LaunchType_String: EVRApplicationProperty =
    11;
pub const EVRApplicationProperty_VRApplicationProperty_WorkingDirectory_String:
    EVRApplicationProperty = 12;
pub const EVRApplicationProperty_VRApplicationProperty_BinaryPath_String: EVRApplicationProperty =
    13;
pub const EVRApplicationProperty_VRApplicationProperty_Arguments_String: EVRApplicationProperty =
    14;
pub const EVRApplicationProperty_VRApplicationProperty_URL_String: EVRApplicationProperty = 15;
pub const EVRApplicationProperty_VRApplicationProperty_Description_String: EVRApplicationProperty =
    50;
pub const EVRApplicationProperty_VRApplicationProperty_NewsURL_String: EVRApplicationProperty = 51;
pub const EVRApplicationProperty_VRApplicationProperty_ImagePath_String: EVRApplicationProperty =
    52;
pub const EVRApplicationProperty_VRApplicationProperty_ImagePathCapsule_String:
    EVRApplicationProperty = 55;
pub const EVRApplicationProperty_VRApplicationProperty_Source_String: EVRApplicationProperty = 53;
pub const EVRApplicationProperty_VRApplicationProperty_ActionManifestURL_String:
    EVRApplicationProperty = 54;
pub const EVRApplicationProperty_VRApplicationProperty_IsDashboardOverlay_Bool:
    EVRApplicationProperty = 60;
pub const EVRApplicationProperty_VRApplicationProperty_IsTemplate_Bool: EVRApplicationProperty = 61;
pub const EVRApplicationProperty_VRApplicationProperty_IsInstanced_Bool: EVRApplicationProperty =
    62;
pub const EVRApplicationProperty_VRApplicationProperty_IsInternal_Bool: EVRApplicationProperty = 63;
pub const EVRApplicationProperty_VRApplicationProperty_WantsCompositorPauseInStandby_Bool:
    EVRApplicationProperty = 64;
pub const EVRApplicationProperty_VRApplicationProperty_IsHidden_Bool: EVRApplicationProperty = 65;
pub const EVRApplicationProperty_VRApplicationProperty_LastLaunchTime_Uint64:
    EVRApplicationProperty = 70;
pub type EVRApplicationProperty = ::core::ffi::c_uint;
pub const EVRSceneApplicationState_None: EVRSceneApplicationState = 0;
pub const EVRSceneApplicationState_Starting: EVRSceneApplicationState = 1;
pub const EVRSceneApplicationState_Quitting: EVRSceneApplicationState = 2;
pub const EVRSceneApplicationState_Running: EVRSceneApplicationState = 3;
pub const EVRSceneApplicationState_Waiting: EVRSceneApplicationState = 4;
pub type EVRSceneApplicationState = ::core::ffi::c_uint;
pub const EVRCompositorError_VRCompositorError_None: EVRCompositorError = 0;
pub const EVRCompositorError_VRCompositorError_RequestFailed: EVRCompositorError = 1;
pub const EVRCompositorError_VRCompositorError_IncompatibleVersion: EVRCompositorError = 100;
pub const EVRCompositorError_VRCompositorError_DoNotHaveFocus: EVRCompositorError = 101;
pub const EVRCompositorError_VRCompositorError_InvalidTexture: EVRCompositorError = 102;
pub const EVRCompositorError_VRCompositorError_IsNotSceneApplication: EVRCompositorError = 103;
pub const EVRCompositorError_VRCompositorError_TextureIsOnWrongDevice: EVRCompositorError = 104;
pub const EVRCompositorError_VRCompositorError_TextureUsesUnsupportedFormat: EVRCompositorError =
    105;
pub const EVRCompositorError_VRCompositorError_SharedTexturesNotSupported: EVRCompositorError = 106;
pub const EVRCompositorError_VRCompositorError_IndexOutOfRange: EVRCompositorError = 107;
pub const EVRCompositorError_VRCompositorError_AlreadySubmitted: EVRCompositorError = 108;
pub const EVRCompositorError_VRCompositorError_InvalidBounds: EVRCompositorError = 109;
pub const EVRCompositorError_VRCompositorError_AlreadySet: EVRCompositorError = 110;
pub type EVRCompositorError = ::core::ffi::c_uint;
pub const EVRCompositorTextureUsage_VRCompositorTextureUsage_Left: EVRCompositorTextureUsage = 0;
pub const EVRCompositorTextureUsage_VRCompositorTextureUsage_Right: EVRCompositorTextureUsage = 1;
pub const EVRCompositorTextureUsage_VRCompositorTextureUsage_Both: EVRCompositorTextureUsage = 2;
pub type EVRCompositorTextureUsage = ::core::ffi::c_uint;
pub const EVRCompositorTimingMode_VRCompositorTimingMode_Implicit: EVRCompositorTimingMode = 0;
pub const EVRCompositorTimingMode_VRCompositorTimingMode_Explicit_RuntimePerformsPostPresentHandoff : EVRCompositorTimingMode = 1 ;
pub const EVRCompositorTimingMode_VRCompositorTimingMode_Explicit_ApplicationPerformsPostPresentHandoff : EVRCompositorTimingMode = 2 ;
pub type EVRCompositorTimingMode = ::core::ffi::c_uint;
pub const VROverlayInputMethod_None: VROverlayInputMethod = 0;
pub const VROverlayInputMethod_Mouse: VROverlayInputMethod = 1;
pub type VROverlayInputMethod = ::core::ffi::c_uint;
pub const VROverlayTransformType_VROverlayTransform_Invalid: VROverlayTransformType = -1;
pub const VROverlayTransformType_VROverlayTransform_Absolute: VROverlayTransformType = 0;
pub const VROverlayTransformType_VROverlayTransform_TrackedDeviceRelative: VROverlayTransformType =
    1;
pub const VROverlayTransformType_VROverlayTransform_TrackedComponent: VROverlayTransformType = 3;
pub const VROverlayTransformType_VROverlayTransform_Cursor: VROverlayTransformType = 4;
pub const VROverlayTransformType_VROverlayTransform_DashboardTab: VROverlayTransformType = 5;
pub const VROverlayTransformType_VROverlayTransform_DashboardThumb: VROverlayTransformType = 6;
pub const VROverlayTransformType_VROverlayTransform_Mountable: VROverlayTransformType = 7;
pub const VROverlayTransformType_VROverlayTransform_Projection: VROverlayTransformType = 8;
pub const VROverlayTransformType_VROverlayTransform_Subview: VROverlayTransformType = 9;
pub type VROverlayTransformType = ::core::ffi::c_int;
pub const VROverlayFlags_NoDashboardTab: VROverlayFlags = 8;
pub const VROverlayFlags_SendVRDiscreteScrollEvents: VROverlayFlags = 64;
pub const VROverlayFlags_SendVRTouchpadEvents: VROverlayFlags = 128;
pub const VROverlayFlags_ShowTouchPadScrollWheel: VROverlayFlags = 256;
pub const VROverlayFlags_TransferOwnershipToInternalProcess: VROverlayFlags = 512;
pub const VROverlayFlags_SideBySide_Parallel: VROverlayFlags = 1024;
pub const VROverlayFlags_SideBySide_Crossed: VROverlayFlags = 2048;
pub const VROverlayFlags_Panorama: VROverlayFlags = 4096;
pub const VROverlayFlags_StereoPanorama: VROverlayFlags = 8192;
pub const VROverlayFlags_SortWithNonSceneOverlays: VROverlayFlags = 16384;
pub const VROverlayFlags_VisibleInDashboard: VROverlayFlags = 32768;
pub const VROverlayFlags_MakeOverlaysInteractiveIfVisible: VROverlayFlags = 65536;
pub const VROverlayFlags_SendVRSmoothScrollEvents: VROverlayFlags = 131072;
pub const VROverlayFlags_ProtectedContent: VROverlayFlags = 262144;
pub const VROverlayFlags_HideLaserIntersection: VROverlayFlags = 524288;
pub const VROverlayFlags_WantsModalBehavior: VROverlayFlags = 1048576;
pub const VROverlayFlags_IsPremultiplied: VROverlayFlags = 2097152;
pub const VROverlayFlags_IgnoreTextureAlpha: VROverlayFlags = 4194304;
pub const VROverlayFlags_EnableControlBar: VROverlayFlags = 8388608;
pub const VROverlayFlags_EnableControlBarKeyboard: VROverlayFlags = 16777216;
pub const VROverlayFlags_EnableControlBarClose: VROverlayFlags = 33554432;
pub const VROverlayFlags_MinimalControlBar: VROverlayFlags = 67108864;
pub const VROverlayFlags_EnableClickStabilization: VROverlayFlags = 134217728;
pub const VROverlayFlags_MultiCursor: VROverlayFlags = 268435456;
pub const VROverlayFlags_NoBackside: VROverlayFlags = 536870912;
pub type VROverlayFlags = ::core::ffi::c_uint;
pub const VRMessageOverlayResponse_ButtonPress_0: VRMessageOverlayResponse = 0;
pub const VRMessageOverlayResponse_ButtonPress_1: VRMessageOverlayResponse = 1;
pub const VRMessageOverlayResponse_ButtonPress_2: VRMessageOverlayResponse = 2;
pub const VRMessageOverlayResponse_ButtonPress_3: VRMessageOverlayResponse = 3;
pub const VRMessageOverlayResponse_CouldntFindSystemOverlay: VRMessageOverlayResponse = 4;
pub const VRMessageOverlayResponse_CouldntFindOrCreateClientOverlay: VRMessageOverlayResponse = 5;
pub const VRMessageOverlayResponse_ApplicationQuit: VRMessageOverlayResponse = 6;
pub type VRMessageOverlayResponse = ::core::ffi::c_uint;
pub const EGamepadTextInputMode_k_EGamepadTextInputModeNormal: EGamepadTextInputMode = 0;
pub const EGamepadTextInputMode_k_EGamepadTextInputModePassword: EGamepadTextInputMode = 1;
pub const EGamepadTextInputMode_k_EGamepadTextInputModeSubmit: EGamepadTextInputMode = 2;
pub type EGamepadTextInputMode = ::core::ffi::c_uint;
pub const EGamepadTextInputLineMode_k_EGamepadTextInputLineModeSingleLine:
    EGamepadTextInputLineMode = 0;
pub const EGamepadTextInputLineMode_k_EGamepadTextInputLineModeMultipleLines:
    EGamepadTextInputLineMode = 1;
pub type EGamepadTextInputLineMode = ::core::ffi::c_uint;
pub const EVROverlayIntersectionMaskPrimitiveType_OverlayIntersectionPrimitiveType_Rectangle:
    EVROverlayIntersectionMaskPrimitiveType = 0;
pub const EVROverlayIntersectionMaskPrimitiveType_OverlayIntersectionPrimitiveType_Circle:
    EVROverlayIntersectionMaskPrimitiveType = 1;
pub type EVROverlayIntersectionMaskPrimitiveType = ::core::ffi::c_uint;
pub const EKeyboardFlags_KeyboardFlag_Minimal: EKeyboardFlags = 1;
pub const EKeyboardFlags_KeyboardFlag_Modal: EKeyboardFlags = 2;
pub const EKeyboardFlags_KeyboardFlag_ShowArrowKeys: EKeyboardFlags = 4;
pub const EKeyboardFlags_KeyboardFlag_HideDoneKey: EKeyboardFlags = 8;
pub type EKeyboardFlags = ::core::ffi::c_uint;
pub const EVRRenderModelError_VRRenderModelError_None: EVRRenderModelError = 0;
pub const EVRRenderModelError_VRRenderModelError_Loading: EVRRenderModelError = 100;
pub const EVRRenderModelError_VRRenderModelError_NotSupported: EVRRenderModelError = 200;
pub const EVRRenderModelError_VRRenderModelError_InvalidArg: EVRRenderModelError = 300;
pub const EVRRenderModelError_VRRenderModelError_InvalidModel: EVRRenderModelError = 301;
pub const EVRRenderModelError_VRRenderModelError_NoShapes: EVRRenderModelError = 302;
pub const EVRRenderModelError_VRRenderModelError_MultipleShapes: EVRRenderModelError = 303;
pub const EVRRenderModelError_VRRenderModelError_TooManyVertices: EVRRenderModelError = 304;
pub const EVRRenderModelError_VRRenderModelError_MultipleTextures: EVRRenderModelError = 305;
pub const EVRRenderModelError_VRRenderModelError_BufferTooSmall: EVRRenderModelError = 306;
pub const EVRRenderModelError_VRRenderModelError_NotEnoughNormals: EVRRenderModelError = 307;
pub const EVRRenderModelError_VRRenderModelError_NotEnoughTexCoords: EVRRenderModelError = 308;
pub const EVRRenderModelError_VRRenderModelError_InvalidTexture: EVRRenderModelError = 400;
pub type EVRRenderModelError = ::core::ffi::c_uint;
pub const EVRRenderModelTextureFormat_VRRenderModelTextureFormat_RGBA8_SRGB:
    EVRRenderModelTextureFormat = 0;
pub const EVRRenderModelTextureFormat_VRRenderModelTextureFormat_BC2: EVRRenderModelTextureFormat =
    1;
pub const EVRRenderModelTextureFormat_VRRenderModelTextureFormat_BC4: EVRRenderModelTextureFormat =
    2;
pub const EVRRenderModelTextureFormat_VRRenderModelTextureFormat_BC7: EVRRenderModelTextureFormat =
    3;
pub const EVRRenderModelTextureFormat_VRRenderModelTextureFormat_BC7_SRGB:
    EVRRenderModelTextureFormat = 4;
pub const EVRRenderModelTextureFormat_VRRenderModelTextureFormat_RGBA16_FLOAT:
    EVRRenderModelTextureFormat = 5;
pub type EVRRenderModelTextureFormat = ::core::ffi::c_uint;
pub const EVRNotificationType_Transient: EVRNotificationType = 0;
pub const EVRNotificationType_Persistent: EVRNotificationType = 1;
pub const EVRNotificationType_Transient_SystemWithUserValue: EVRNotificationType = 2;
pub type EVRNotificationType = ::core::ffi::c_uint;
pub const EVRNotificationStyle_None: EVRNotificationStyle = 0;
pub const EVRNotificationStyle_Application: EVRNotificationStyle = 100;
pub const EVRNotificationStyle_Contact_Disabled: EVRNotificationStyle = 200;
pub const EVRNotificationStyle_Contact_Enabled: EVRNotificationStyle = 201;
pub const EVRNotificationStyle_Contact_Active: EVRNotificationStyle = 202;
pub type EVRNotificationStyle = ::core::ffi::c_uint;
pub const EVRSettingsError_VRSettingsError_None: EVRSettingsError = 0;
pub const EVRSettingsError_VRSettingsError_IPCFailed: EVRSettingsError = 1;
pub const EVRSettingsError_VRSettingsError_WriteFailed: EVRSettingsError = 2;
pub const EVRSettingsError_VRSettingsError_ReadFailed: EVRSettingsError = 3;
pub const EVRSettingsError_VRSettingsError_JsonParseFailed: EVRSettingsError = 4;
pub const EVRSettingsError_VRSettingsError_UnsetSettingHasNoDefault: EVRSettingsError = 5;
pub const EVRSettingsError_VRSettingsError_AccessDenied: EVRSettingsError = 6;
pub type EVRSettingsError = ::core::ffi::c_uint;
pub const EVRScreenshotError_VRScreenshotError_None: EVRScreenshotError = 0;
pub const EVRScreenshotError_VRScreenshotError_RequestFailed: EVRScreenshotError = 1;
pub const EVRScreenshotError_VRScreenshotError_IncompatibleVersion: EVRScreenshotError = 100;
pub const EVRScreenshotError_VRScreenshotError_NotFound: EVRScreenshotError = 101;
pub const EVRScreenshotError_VRScreenshotError_BufferTooSmall: EVRScreenshotError = 102;
pub const EVRScreenshotError_VRScreenshotError_ScreenshotAlreadyInProgress: EVRScreenshotError =
    108;
pub type EVRScreenshotError = ::core::ffi::c_uint;
pub const EVRSkeletalTransformSpace_VRSkeletalTransformSpace_Model: EVRSkeletalTransformSpace = 0;
pub const EVRSkeletalTransformSpace_VRSkeletalTransformSpace_Parent: EVRSkeletalTransformSpace = 1;
pub type EVRSkeletalTransformSpace = ::core::ffi::c_uint;
pub const EVRSkeletalReferencePose_VRSkeletalReferencePose_BindPose: EVRSkeletalReferencePose = 0;
pub const EVRSkeletalReferencePose_VRSkeletalReferencePose_OpenHand: EVRSkeletalReferencePose = 1;
pub const EVRSkeletalReferencePose_VRSkeletalReferencePose_Fist: EVRSkeletalReferencePose = 2;
pub const EVRSkeletalReferencePose_VRSkeletalReferencePose_GripLimit: EVRSkeletalReferencePose = 3;
pub type EVRSkeletalReferencePose = ::core::ffi::c_uint;
pub const EVRFinger_VRFinger_Thumb: EVRFinger = 0;
pub const EVRFinger_VRFinger_Index: EVRFinger = 1;
pub const EVRFinger_VRFinger_Middle: EVRFinger = 2;
pub const EVRFinger_VRFinger_Ring: EVRFinger = 3;
pub const EVRFinger_VRFinger_Pinky: EVRFinger = 4;
pub const EVRFinger_VRFinger_Count: EVRFinger = 5;
pub type EVRFinger = ::core::ffi::c_uint;
pub const EVRFingerSplay_VRFingerSplay_Thumb_Index: EVRFingerSplay = 0;
pub const EVRFingerSplay_VRFingerSplay_Index_Middle: EVRFingerSplay = 1;
pub const EVRFingerSplay_VRFingerSplay_Middle_Ring: EVRFingerSplay = 2;
pub const EVRFingerSplay_VRFingerSplay_Ring_Pinky: EVRFingerSplay = 3;
pub const EVRFingerSplay_VRFingerSplay_Count: EVRFingerSplay = 4;
pub type EVRFingerSplay = ::core::ffi::c_uint;
pub const EVRSummaryType_VRSummaryType_FromAnimation: EVRSummaryType = 0;
pub const EVRSummaryType_VRSummaryType_FromDevice: EVRSummaryType = 1;
pub type EVRSummaryType = ::core::ffi::c_uint;
pub const EVRInputFilterCancelType_VRInputFilterCancel_Timers: EVRInputFilterCancelType = 0;
pub const EVRInputFilterCancelType_VRInputFilterCancel_Momentum: EVRInputFilterCancelType = 1;
pub type EVRInputFilterCancelType = ::core::ffi::c_uint;
pub const EVRInputStringBits_VRInputString_Hand: EVRInputStringBits = 1;
pub const EVRInputStringBits_VRInputString_ControllerType: EVRInputStringBits = 2;
pub const EVRInputStringBits_VRInputString_InputSource: EVRInputStringBits = 4;
pub const EVRInputStringBits_VRInputString_All: EVRInputStringBits = -1;
pub type EVRInputStringBits = ::core::ffi::c_int;
pub const EVRDebugError_VRDebugError_Success: EVRDebugError = 0;
pub const EVRDebugError_VRDebugError_BadParameter: EVRDebugError = 1;
pub type EVRDebugError = ::core::ffi::c_uint;
pub type TrackedDeviceIndex_t = u32;
pub type VROverlayHandle_t = u64;
pub type SpatialAnchorHandle_t = u32;
pub type WebConsoleHandle_t = u64;
pub type VRComponentProperties = u32;
pub type BoneIndex_t = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HmdMatrix34_t {
    pub m: [[f32; 4usize]; 3usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of HmdMatrix34_t"][::core::mem::size_of::<HmdMatrix34_t>() - 48usize];
    ["Alignment of HmdMatrix34_t"][::core::mem::align_of::<HmdMatrix34_t>() - 4usize];
    ["Offset of field: HmdMatrix34_t::m"][::core::mem::offset_of!(HmdMatrix34_t, m) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HmdMatrix44_t {
    pub m: [[f32; 4usize]; 4usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of HmdMatrix44_t"][::core::mem::size_of::<HmdMatrix44_t>() - 64usize];
    ["Alignment of HmdMatrix44_t"][::core::mem::align_of::<HmdMatrix44_t>() - 4usize];
    ["Offset of field: HmdMatrix44_t::m"][::core::mem::offset_of!(HmdMatrix44_t, m) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HmdVector3_t {
    pub v: [f32; 3usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of HmdVector3_t"][::core::mem::size_of::<HmdVector3_t>() - 12usize];
    ["Alignment of HmdVector3_t"][::core::mem::align_of::<HmdVector3_t>() - 4usize];
    ["Offset of field: HmdVector3_t::v"][::core::mem::offset_of!(HmdVector3_t, v) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HmdVector4_t {
    pub v: [f32; 4usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of HmdVector4_t"][::core::mem::size_of::<HmdVector4_t>() - 16usize];
    ["Alignment of HmdVector4_t"][::core::mem::align_of::<HmdVector4_t>() - 4usize];
    ["Offset of field: HmdVector4_t::v"][::core::mem::offset_of!(HmdVector4_t, v) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HmdVector2_t {
    pub v: [f32; 2usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of HmdVector2_t"][::core::mem::size_of::<HmdVector2_t>() - 8usize];
    ["Alignment of HmdVector2_t"][::core::mem::align_of::<HmdVector2_t>() - 4usize];
    ["Offset of field: HmdVector2_t::v"][::core::mem::offset_of!(HmdVector2_t, v) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HmdQuaternionf_t {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of HmdQuaternionf_t"][::core::mem::size_of::<HmdQuaternionf_t>() - 16usize];
    ["Alignment of HmdQuaternionf_t"][::core::mem::align_of::<HmdQuaternionf_t>() - 4usize];
    ["Offset of field: HmdQuaternionf_t::w"][::core::mem::offset_of!(HmdQuaternionf_t, w) - 0usize];
    ["Offset of field: HmdQuaternionf_t::x"][::core::mem::offset_of!(HmdQuaternionf_t, x) - 4usize];
    ["Offset of field: HmdQuaternionf_t::y"][::core::mem::offset_of!(HmdQuaternionf_t, y) - 8usize];
    ["Offset of field: HmdQuaternionf_t::z"]
        [::core::mem::offset_of!(HmdQuaternionf_t, z) - 12usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HmdRect2_t {
    pub vTopLeft: HmdVector2_t,
    pub vBottomRight: HmdVector2_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of HmdRect2_t"][::core::mem::size_of::<HmdRect2_t>() - 16usize];
    ["Alignment of HmdRect2_t"][::core::mem::align_of::<HmdRect2_t>() - 4usize];
    ["Offset of field: HmdRect2_t::vTopLeft"]
        [::core::mem::offset_of!(HmdRect2_t, vTopLeft) - 0usize];
    ["Offset of field: HmdRect2_t::vBottomRight"]
        [::core::mem::offset_of!(HmdRect2_t, vBottomRight) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VRBoneTransform_t {
    pub position: HmdVector4_t,
    pub orientation: HmdQuaternionf_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VRBoneTransform_t"][::core::mem::size_of::<VRBoneTransform_t>() - 32usize];
    ["Alignment of VRBoneTransform_t"][::core::mem::align_of::<VRBoneTransform_t>() - 4usize];
    ["Offset of field: VRBoneTransform_t::position"]
        [::core::mem::offset_of!(VRBoneTransform_t, position) - 0usize];
    ["Offset of field: VRBoneTransform_t::orientation"]
        [::core::mem::offset_of!(VRBoneTransform_t, orientation) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREyeTrackingData_t {
    pub bActive: bool,
    pub bValid: bool,
    pub bTracked: bool,
    pub vGazeOrigin: HmdVector3_t,
    pub vGazeTarget: HmdVector3_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREyeTrackingData_t"][::core::mem::size_of::<VREyeTrackingData_t>() - 28usize];
    ["Alignment of VREyeTrackingData_t"][::core::mem::align_of::<VREyeTrackingData_t>() - 4usize];
    ["Offset of field: VREyeTrackingData_t::bActive"]
        [::core::mem::offset_of!(VREyeTrackingData_t, bActive) - 0usize];
    ["Offset of field: VREyeTrackingData_t::bValid"]
        [::core::mem::offset_of!(VREyeTrackingData_t, bValid) - 1usize];
    ["Offset of field: VREyeTrackingData_t::bTracked"]
        [::core::mem::offset_of!(VREyeTrackingData_t, bTracked) - 2usize];
    ["Offset of field: VREyeTrackingData_t::vGazeOrigin"]
        [::core::mem::offset_of!(VREyeTrackingData_t, vGazeOrigin) - 4usize];
    ["Offset of field: VREyeTrackingData_t::vGazeTarget"]
        [::core::mem::offset_of!(VREyeTrackingData_t, vGazeTarget) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DistortionCoordinates_t {
    pub rfRed: [f32; 2usize],
    pub rfGreen: [f32; 2usize],
    pub rfBlue: [f32; 2usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of DistortionCoordinates_t"]
        [::core::mem::size_of::<DistortionCoordinates_t>() - 24usize];
    ["Alignment of DistortionCoordinates_t"]
        [::core::mem::align_of::<DistortionCoordinates_t>() - 4usize];
    ["Offset of field: DistortionCoordinates_t::rfRed"]
        [::core::mem::offset_of!(DistortionCoordinates_t, rfRed) - 0usize];
    ["Offset of field: DistortionCoordinates_t::rfGreen"]
        [::core::mem::offset_of!(DistortionCoordinates_t, rfGreen) - 8usize];
    ["Offset of field: DistortionCoordinates_t::rfBlue"]
        [::core::mem::offset_of!(DistortionCoordinates_t, rfBlue) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Texture_t {
    pub handle: *mut ::core::ffi::c_void,
    pub eType: ETextureType,
    pub eColorSpace: EColorSpace,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Texture_t"][::core::mem::size_of::<Texture_t>() - 16usize];
    ["Alignment of Texture_t"][::core::mem::align_of::<Texture_t>() - 8usize];
    ["Offset of field: Texture_t::handle"][::core::mem::offset_of!(Texture_t, handle) - 0usize];
    ["Offset of field: Texture_t::eType"][::core::mem::offset_of!(Texture_t, eType) - 8usize];
    ["Offset of field: Texture_t::eColorSpace"]
        [::core::mem::offset_of!(Texture_t, eColorSpace) - 12usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VRTextureBounds_t {
    pub uMin: f32,
    pub vMin: f32,
    pub uMax: f32,
    pub vMax: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VRTextureBounds_t"][::core::mem::size_of::<VRTextureBounds_t>() - 16usize];
    ["Alignment of VRTextureBounds_t"][::core::mem::align_of::<VRTextureBounds_t>() - 4usize];
    ["Offset of field: VRTextureBounds_t::uMin"]
        [::core::mem::offset_of!(VRTextureBounds_t, uMin) - 0usize];
    ["Offset of field: VRTextureBounds_t::vMin"]
        [::core::mem::offset_of!(VRTextureBounds_t, vMin) - 4usize];
    ["Offset of field: VRTextureBounds_t::uMax"]
        [::core::mem::offset_of!(VRTextureBounds_t, uMax) - 8usize];
    ["Offset of field: VRTextureBounds_t::vMax"]
        [::core::mem::offset_of!(VRTextureBounds_t, vMax) - 12usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TrackedDevicePose_t {
    pub mDeviceToAbsoluteTracking: HmdMatrix34_t,
    pub vVelocity: HmdVector3_t,
    pub vAngularVelocity: HmdVector3_t,
    pub eTrackingResult: ETrackingResult,
    pub bPoseIsValid: bool,
    pub bDeviceIsConnected: bool,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of TrackedDevicePose_t"][::core::mem::size_of::<TrackedDevicePose_t>() - 80usize];
    ["Alignment of TrackedDevicePose_t"][::core::mem::align_of::<TrackedDevicePose_t>() - 4usize];
    ["Offset of field: TrackedDevicePose_t::mDeviceToAbsoluteTracking"]
        [::core::mem::offset_of!(TrackedDevicePose_t, mDeviceToAbsoluteTracking) - 0usize];
    ["Offset of field: TrackedDevicePose_t::vVelocity"]
        [::core::mem::offset_of!(TrackedDevicePose_t, vVelocity) - 48usize];
    ["Offset of field: TrackedDevicePose_t::vAngularVelocity"]
        [::core::mem::offset_of!(TrackedDevicePose_t, vAngularVelocity) - 60usize];
    ["Offset of field: TrackedDevicePose_t::eTrackingResult"]
        [::core::mem::offset_of!(TrackedDevicePose_t, eTrackingResult) - 72usize];
    ["Offset of field: TrackedDevicePose_t::bPoseIsValid"]
        [::core::mem::offset_of!(TrackedDevicePose_t, bPoseIsValid) - 76usize];
    ["Offset of field: TrackedDevicePose_t::bDeviceIsConnected"]
        [::core::mem::offset_of!(TrackedDevicePose_t, bDeviceIsConnected) - 77usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_Controller_t {
    pub button: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_Controller_t"][::core::mem::size_of::<VREvent_Controller_t>() - 4usize];
    ["Alignment of VREvent_Controller_t"][::core::mem::align_of::<VREvent_Controller_t>() - 4usize];
    ["Offset of field: VREvent_Controller_t::button"]
        [::core::mem::offset_of!(VREvent_Controller_t, button) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_Mouse_t {
    pub x: f32,
    pub y: f32,
    pub button: u32,
    pub cursorIndex: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_Mouse_t"][::core::mem::size_of::<VREvent_Mouse_t>() - 16usize];
    ["Alignment of VREvent_Mouse_t"][::core::mem::align_of::<VREvent_Mouse_t>() - 4usize];
    ["Offset of field: VREvent_Mouse_t::x"][::core::mem::offset_of!(VREvent_Mouse_t, x) - 0usize];
    ["Offset of field: VREvent_Mouse_t::y"][::core::mem::offset_of!(VREvent_Mouse_t, y) - 4usize];
    ["Offset of field: VREvent_Mouse_t::button"]
        [::core::mem::offset_of!(VREvent_Mouse_t, button) - 8usize];
    ["Offset of field: VREvent_Mouse_t::cursorIndex"]
        [::core::mem::offset_of!(VREvent_Mouse_t, cursorIndex) - 12usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_Scroll_t {
    pub xdelta: f32,
    pub ydelta: f32,
    pub unused: u32,
    pub viewportscale: f32,
    pub cursorIndex: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_Scroll_t"][::core::mem::size_of::<VREvent_Scroll_t>() - 20usize];
    ["Alignment of VREvent_Scroll_t"][::core::mem::align_of::<VREvent_Scroll_t>() - 4usize];
    ["Offset of field: VREvent_Scroll_t::xdelta"]
        [::core::mem::offset_of!(VREvent_Scroll_t, xdelta) - 0usize];
    ["Offset of field: VREvent_Scroll_t::ydelta"]
        [::core::mem::offset_of!(VREvent_Scroll_t, ydelta) - 4usize];
    ["Offset of field: VREvent_Scroll_t::unused"]
        [::core::mem::offset_of!(VREvent_Scroll_t, unused) - 8usize];
    ["Offset of field: VREvent_Scroll_t::viewportscale"]
        [::core::mem::offset_of!(VREvent_Scroll_t, viewportscale) - 12usize];
    ["Offset of field: VREvent_Scroll_t::cursorIndex"]
        [::core::mem::offset_of!(VREvent_Scroll_t, cursorIndex) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_TouchPadMove_t {
    pub bFingerDown: bool,
    pub flSecondsFingerDown: f32,
    pub fValueXFirst: f32,
    pub fValueYFirst: f32,
    pub fValueXRaw: f32,
    pub fValueYRaw: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_TouchPadMove_t"][::core::mem::size_of::<VREvent_TouchPadMove_t>() - 24usize];
    ["Alignment of VREvent_TouchPadMove_t"]
        [::core::mem::align_of::<VREvent_TouchPadMove_t>() - 4usize];
    ["Offset of field: VREvent_TouchPadMove_t::bFingerDown"]
        [::core::mem::offset_of!(VREvent_TouchPadMove_t, bFingerDown) - 0usize];
    ["Offset of field: VREvent_TouchPadMove_t::flSecondsFingerDown"]
        [::core::mem::offset_of!(VREvent_TouchPadMove_t, flSecondsFingerDown) - 4usize];
    ["Offset of field: VREvent_TouchPadMove_t::fValueXFirst"]
        [::core::mem::offset_of!(VREvent_TouchPadMove_t, fValueXFirst) - 8usize];
    ["Offset of field: VREvent_TouchPadMove_t::fValueYFirst"]
        [::core::mem::offset_of!(VREvent_TouchPadMove_t, fValueYFirst) - 12usize];
    ["Offset of field: VREvent_TouchPadMove_t::fValueXRaw"]
        [::core::mem::offset_of!(VREvent_TouchPadMove_t, fValueXRaw) - 16usize];
    ["Offset of field: VREvent_TouchPadMove_t::fValueYRaw"]
        [::core::mem::offset_of!(VREvent_TouchPadMove_t, fValueYRaw) - 20usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_Notification_t {
    pub ulUserValue: u64,
    pub notificationId: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_Notification_t"][::core::mem::size_of::<VREvent_Notification_t>() - 16usize];
    ["Alignment of VREvent_Notification_t"]
        [::core::mem::align_of::<VREvent_Notification_t>() - 8usize];
    ["Offset of field: VREvent_Notification_t::ulUserValue"]
        [::core::mem::offset_of!(VREvent_Notification_t, ulUserValue) - 0usize];
    ["Offset of field: VREvent_Notification_t::notificationId"]
        [::core::mem::offset_of!(VREvent_Notification_t, notificationId) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_Process_t {
    pub pid: u32,
    pub oldPid: u32,
    pub bForced: bool,
    pub bConnectionLost: bool,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_Process_t"][::core::mem::size_of::<VREvent_Process_t>() - 12usize];
    ["Alignment of VREvent_Process_t"][::core::mem::align_of::<VREvent_Process_t>() - 4usize];
    ["Offset of field: VREvent_Process_t::pid"]
        [::core::mem::offset_of!(VREvent_Process_t, pid) - 0usize];
    ["Offset of field: VREvent_Process_t::oldPid"]
        [::core::mem::offset_of!(VREvent_Process_t, oldPid) - 4usize];
    ["Offset of field: VREvent_Process_t::bForced"]
        [::core::mem::offset_of!(VREvent_Process_t, bForced) - 8usize];
    ["Offset of field: VREvent_Process_t::bConnectionLost"]
        [::core::mem::offset_of!(VREvent_Process_t, bConnectionLost) - 9usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_Overlay_t {
    pub overlayHandle: u64,
    pub devicePath: u64,
    pub memoryBlockId: u64,
    pub cursorIndex: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_Overlay_t"][::core::mem::size_of::<VREvent_Overlay_t>() - 32usize];
    ["Alignment of VREvent_Overlay_t"][::core::mem::align_of::<VREvent_Overlay_t>() - 8usize];
    ["Offset of field: VREvent_Overlay_t::overlayHandle"]
        [::core::mem::offset_of!(VREvent_Overlay_t, overlayHandle) - 0usize];
    ["Offset of field: VREvent_Overlay_t::devicePath"]
        [::core::mem::offset_of!(VREvent_Overlay_t, devicePath) - 8usize];
    ["Offset of field: VREvent_Overlay_t::memoryBlockId"]
        [::core::mem::offset_of!(VREvent_Overlay_t, memoryBlockId) - 16usize];
    ["Offset of field: VREvent_Overlay_t::cursorIndex"]
        [::core::mem::offset_of!(VREvent_Overlay_t, cursorIndex) - 24usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_Status_t {
    pub statusState: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_Status_t"][::core::mem::size_of::<VREvent_Status_t>() - 4usize];
    ["Alignment of VREvent_Status_t"][::core::mem::align_of::<VREvent_Status_t>() - 4usize];
    ["Offset of field: VREvent_Status_t::statusState"]
        [::core::mem::offset_of!(VREvent_Status_t, statusState) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_Keyboard_t {
    pub cNewInput: [::core::ffi::c_char; 8usize],
    pub uUserValue: u64,
    pub overlayHandle: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_Keyboard_t"][::core::mem::size_of::<VREvent_Keyboard_t>() - 24usize];
    ["Alignment of VREvent_Keyboard_t"][::core::mem::align_of::<VREvent_Keyboard_t>() - 8usize];
    ["Offset of field: VREvent_Keyboard_t::cNewInput"]
        [::core::mem::offset_of!(VREvent_Keyboard_t, cNewInput) - 0usize];
    ["Offset of field: VREvent_Keyboard_t::uUserValue"]
        [::core::mem::offset_of!(VREvent_Keyboard_t, uUserValue) - 8usize];
    ["Offset of field: VREvent_Keyboard_t::overlayHandle"]
        [::core::mem::offset_of!(VREvent_Keyboard_t, overlayHandle) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_Ipd_t {
    pub ipdMeters: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_Ipd_t"][::core::mem::size_of::<VREvent_Ipd_t>() - 4usize];
    ["Alignment of VREvent_Ipd_t"][::core::mem::align_of::<VREvent_Ipd_t>() - 4usize];
    ["Offset of field: VREvent_Ipd_t::ipdMeters"]
        [::core::mem::offset_of!(VREvent_Ipd_t, ipdMeters) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_Chaperone_t {
    pub m_nPreviousUniverse_deprecated: u64,
    pub m_nCurrentUniverse: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_Chaperone_t"][::core::mem::size_of::<VREvent_Chaperone_t>() - 16usize];
    ["Alignment of VREvent_Chaperone_t"][::core::mem::align_of::<VREvent_Chaperone_t>() - 8usize];
    ["Offset of field: VREvent_Chaperone_t::m_nPreviousUniverse_deprecated"]
        [::core::mem::offset_of!(VREvent_Chaperone_t, m_nPreviousUniverse_deprecated) - 0usize];
    ["Offset of field: VREvent_Chaperone_t::m_nCurrentUniverse"]
        [::core::mem::offset_of!(VREvent_Chaperone_t, m_nCurrentUniverse) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_Reserved_t {
    pub reserved0: u64,
    pub reserved1: u64,
    pub reserved2: u64,
    pub reserved3: u64,
    pub reserved4: u64,
    pub reserved5: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_Reserved_t"][::core::mem::size_of::<VREvent_Reserved_t>() - 48usize];
    ["Alignment of VREvent_Reserved_t"][::core::mem::align_of::<VREvent_Reserved_t>() - 8usize];
    ["Offset of field: VREvent_Reserved_t::reserved0"]
        [::core::mem::offset_of!(VREvent_Reserved_t, reserved0) - 0usize];
    ["Offset of field: VREvent_Reserved_t::reserved1"]
        [::core::mem::offset_of!(VREvent_Reserved_t, reserved1) - 8usize];
    ["Offset of field: VREvent_Reserved_t::reserved2"]
        [::core::mem::offset_of!(VREvent_Reserved_t, reserved2) - 16usize];
    ["Offset of field: VREvent_Reserved_t::reserved3"]
        [::core::mem::offset_of!(VREvent_Reserved_t, reserved3) - 24usize];
    ["Offset of field: VREvent_Reserved_t::reserved4"]
        [::core::mem::offset_of!(VREvent_Reserved_t, reserved4) - 32usize];
    ["Offset of field: VREvent_Reserved_t::reserved5"]
        [::core::mem::offset_of!(VREvent_Reserved_t, reserved5) - 40usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_PerformanceTest_t {
    pub m_nFidelityLevel: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_PerformanceTest_t"]
        [::core::mem::size_of::<VREvent_PerformanceTest_t>() - 4usize];
    ["Alignment of VREvent_PerformanceTest_t"]
        [::core::mem::align_of::<VREvent_PerformanceTest_t>() - 4usize];
    ["Offset of field: VREvent_PerformanceTest_t::m_nFidelityLevel"]
        [::core::mem::offset_of!(VREvent_PerformanceTest_t, m_nFidelityLevel) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_SeatedZeroPoseReset_t {
    pub bResetBySystemMenu: bool,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_SeatedZeroPoseReset_t"]
        [::core::mem::size_of::<VREvent_SeatedZeroPoseReset_t>() - 1usize];
    ["Alignment of VREvent_SeatedZeroPoseReset_t"]
        [::core::mem::align_of::<VREvent_SeatedZeroPoseReset_t>() - 1usize];
    ["Offset of field: VREvent_SeatedZeroPoseReset_t::bResetBySystemMenu"]
        [::core::mem::offset_of!(VREvent_SeatedZeroPoseReset_t, bResetBySystemMenu) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_Screenshot_t {
    pub handle: u32,
    pub type_: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_Screenshot_t"][::core::mem::size_of::<VREvent_Screenshot_t>() - 8usize];
    ["Alignment of VREvent_Screenshot_t"][::core::mem::align_of::<VREvent_Screenshot_t>() - 4usize];
    ["Offset of field: VREvent_Screenshot_t::handle"]
        [::core::mem::offset_of!(VREvent_Screenshot_t, handle) - 0usize];
    ["Offset of field: VREvent_Screenshot_t::type_"]
        [::core::mem::offset_of!(VREvent_Screenshot_t, type_) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_ScreenshotProgress_t {
    pub progress: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_ScreenshotProgress_t"]
        [::core::mem::size_of::<VREvent_ScreenshotProgress_t>() - 4usize];
    ["Alignment of VREvent_ScreenshotProgress_t"]
        [::core::mem::align_of::<VREvent_ScreenshotProgress_t>() - 4usize];
    ["Offset of field: VREvent_ScreenshotProgress_t::progress"]
        [::core::mem::offset_of!(VREvent_ScreenshotProgress_t, progress) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_ApplicationLaunch_t {
    pub pid: u32,
    pub unArgsHandle: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_ApplicationLaunch_t"]
        [::core::mem::size_of::<VREvent_ApplicationLaunch_t>() - 8usize];
    ["Alignment of VREvent_ApplicationLaunch_t"]
        [::core::mem::align_of::<VREvent_ApplicationLaunch_t>() - 4usize];
    ["Offset of field: VREvent_ApplicationLaunch_t::pid"]
        [::core::mem::offset_of!(VREvent_ApplicationLaunch_t, pid) - 0usize];
    ["Offset of field: VREvent_ApplicationLaunch_t::unArgsHandle"]
        [::core::mem::offset_of!(VREvent_ApplicationLaunch_t, unArgsHandle) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_EditingCameraSurface_t {
    pub overlayHandle: u64,
    pub nVisualMode: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_EditingCameraSurface_t"]
        [::core::mem::size_of::<VREvent_EditingCameraSurface_t>() - 16usize];
    ["Alignment of VREvent_EditingCameraSurface_t"]
        [::core::mem::align_of::<VREvent_EditingCameraSurface_t>() - 8usize];
    ["Offset of field: VREvent_EditingCameraSurface_t::overlayHandle"]
        [::core::mem::offset_of!(VREvent_EditingCameraSurface_t, overlayHandle) - 0usize];
    ["Offset of field: VREvent_EditingCameraSurface_t::nVisualMode"]
        [::core::mem::offset_of!(VREvent_EditingCameraSurface_t, nVisualMode) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_MessageOverlay_t {
    pub unVRMessageOverlayResponse: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_MessageOverlay_t"]
        [::core::mem::size_of::<VREvent_MessageOverlay_t>() - 4usize];
    ["Alignment of VREvent_MessageOverlay_t"]
        [::core::mem::align_of::<VREvent_MessageOverlay_t>() - 4usize];
    ["Offset of field: VREvent_MessageOverlay_t::unVRMessageOverlayResponse"]
        [::core::mem::offset_of!(VREvent_MessageOverlay_t, unVRMessageOverlayResponse) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_Property_t {
    pub container: PropertyContainerHandle_t,
    pub prop: ETrackedDeviceProperty,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_Property_t"][::core::mem::size_of::<VREvent_Property_t>() - 16usize];
    ["Alignment of VREvent_Property_t"][::core::mem::align_of::<VREvent_Property_t>() - 8usize];
    ["Offset of field: VREvent_Property_t::container"]
        [::core::mem::offset_of!(VREvent_Property_t, container) - 0usize];
    ["Offset of field: VREvent_Property_t::prop"]
        [::core::mem::offset_of!(VREvent_Property_t, prop) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_HapticVibration_t {
    pub containerHandle: u64,
    pub componentHandle: u64,
    pub fDurationSeconds: f32,
    pub fFrequency: f32,
    pub fAmplitude: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_HapticVibration_t"]
        [::core::mem::size_of::<VREvent_HapticVibration_t>() - 32usize];
    ["Alignment of VREvent_HapticVibration_t"]
        [::core::mem::align_of::<VREvent_HapticVibration_t>() - 8usize];
    ["Offset of field: VREvent_HapticVibration_t::containerHandle"]
        [::core::mem::offset_of!(VREvent_HapticVibration_t, containerHandle) - 0usize];
    ["Offset of field: VREvent_HapticVibration_t::componentHandle"]
        [::core::mem::offset_of!(VREvent_HapticVibration_t, componentHandle) - 8usize];
    ["Offset of field: VREvent_HapticVibration_t::fDurationSeconds"]
        [::core::mem::offset_of!(VREvent_HapticVibration_t, fDurationSeconds) - 16usize];
    ["Offset of field: VREvent_HapticVibration_t::fFrequency"]
        [::core::mem::offset_of!(VREvent_HapticVibration_t, fFrequency) - 20usize];
    ["Offset of field: VREvent_HapticVibration_t::fAmplitude"]
        [::core::mem::offset_of!(VREvent_HapticVibration_t, fAmplitude) - 24usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_WebConsole_t {
    pub webConsoleHandle: WebConsoleHandle_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_WebConsole_t"][::core::mem::size_of::<VREvent_WebConsole_t>() - 8usize];
    ["Alignment of VREvent_WebConsole_t"][::core::mem::align_of::<VREvent_WebConsole_t>() - 8usize];
    ["Offset of field: VREvent_WebConsole_t::webConsoleHandle"]
        [::core::mem::offset_of!(VREvent_WebConsole_t, webConsoleHandle) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_InputBindingLoad_t {
    pub ulAppContainer: PropertyContainerHandle_t,
    pub pathMessage: u64,
    pub pathUrl: u64,
    pub pathControllerType: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_InputBindingLoad_t"]
        [::core::mem::size_of::<VREvent_InputBindingLoad_t>() - 32usize];
    ["Alignment of VREvent_InputBindingLoad_t"]
        [::core::mem::align_of::<VREvent_InputBindingLoad_t>() - 8usize];
    ["Offset of field: VREvent_InputBindingLoad_t::ulAppContainer"]
        [::core::mem::offset_of!(VREvent_InputBindingLoad_t, ulAppContainer) - 0usize];
    ["Offset of field: VREvent_InputBindingLoad_t::pathMessage"]
        [::core::mem::offset_of!(VREvent_InputBindingLoad_t, pathMessage) - 8usize];
    ["Offset of field: VREvent_InputBindingLoad_t::pathUrl"]
        [::core::mem::offset_of!(VREvent_InputBindingLoad_t, pathUrl) - 16usize];
    ["Offset of field: VREvent_InputBindingLoad_t::pathControllerType"]
        [::core::mem::offset_of!(VREvent_InputBindingLoad_t, pathControllerType) - 24usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_InputActionManifestLoad_t {
    pub pathAppKey: u64,
    pub pathMessage: u64,
    pub pathMessageParam: u64,
    pub pathManifestPath: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_InputActionManifestLoad_t"]
        [::core::mem::size_of::<VREvent_InputActionManifestLoad_t>() - 32usize];
    ["Alignment of VREvent_InputActionManifestLoad_t"]
        [::core::mem::align_of::<VREvent_InputActionManifestLoad_t>() - 8usize];
    ["Offset of field: VREvent_InputActionManifestLoad_t::pathAppKey"]
        [::core::mem::offset_of!(VREvent_InputActionManifestLoad_t, pathAppKey) - 0usize];
    ["Offset of field: VREvent_InputActionManifestLoad_t::pathMessage"]
        [::core::mem::offset_of!(VREvent_InputActionManifestLoad_t, pathMessage) - 8usize];
    ["Offset of field: VREvent_InputActionManifestLoad_t::pathMessageParam"]
        [::core::mem::offset_of!(VREvent_InputActionManifestLoad_t, pathMessageParam) - 16usize];
    ["Offset of field: VREvent_InputActionManifestLoad_t::pathManifestPath"]
        [::core::mem::offset_of!(VREvent_InputActionManifestLoad_t, pathManifestPath) - 24usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_SpatialAnchor_t {
    pub unHandle: SpatialAnchorHandle_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_SpatialAnchor_t"][::core::mem::size_of::<VREvent_SpatialAnchor_t>() - 4usize];
    ["Alignment of VREvent_SpatialAnchor_t"]
        [::core::mem::align_of::<VREvent_SpatialAnchor_t>() - 4usize];
    ["Offset of field: VREvent_SpatialAnchor_t::unHandle"]
        [::core::mem::offset_of!(VREvent_SpatialAnchor_t, unHandle) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_ProgressUpdate_t {
    pub ulApplicationPropertyContainer: u64,
    pub pathDevice: u64,
    pub pathInputSource: u64,
    pub pathProgressAction: u64,
    pub pathIcon: u64,
    pub fProgress: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_ProgressUpdate_t"]
        [::core::mem::size_of::<VREvent_ProgressUpdate_t>() - 48usize];
    ["Alignment of VREvent_ProgressUpdate_t"]
        [::core::mem::align_of::<VREvent_ProgressUpdate_t>() - 8usize];
    ["Offset of field: VREvent_ProgressUpdate_t::ulApplicationPropertyContainer"][::core::mem::offset_of!(
        VREvent_ProgressUpdate_t,
        ulApplicationPropertyContainer
    ) - 0usize];
    ["Offset of field: VREvent_ProgressUpdate_t::pathDevice"]
        [::core::mem::offset_of!(VREvent_ProgressUpdate_t, pathDevice) - 8usize];
    ["Offset of field: VREvent_ProgressUpdate_t::pathInputSource"]
        [::core::mem::offset_of!(VREvent_ProgressUpdate_t, pathInputSource) - 16usize];
    ["Offset of field: VREvent_ProgressUpdate_t::pathProgressAction"]
        [::core::mem::offset_of!(VREvent_ProgressUpdate_t, pathProgressAction) - 24usize];
    ["Offset of field: VREvent_ProgressUpdate_t::pathIcon"]
        [::core::mem::offset_of!(VREvent_ProgressUpdate_t, pathIcon) - 32usize];
    ["Offset of field: VREvent_ProgressUpdate_t::fProgress"]
        [::core::mem::offset_of!(VREvent_ProgressUpdate_t, fProgress) - 40usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_ShowUI_t {
    pub eType: EShowUIType,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_ShowUI_t"][::core::mem::size_of::<VREvent_ShowUI_t>() - 4usize];
    ["Alignment of VREvent_ShowUI_t"][::core::mem::align_of::<VREvent_ShowUI_t>() - 4usize];
    ["Offset of field: VREvent_ShowUI_t::eType"]
        [::core::mem::offset_of!(VREvent_ShowUI_t, eType) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_ShowDevTools_t {
    pub nBrowserIdentifier: i32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_ShowDevTools_t"][::core::mem::size_of::<VREvent_ShowDevTools_t>() - 4usize];
    ["Alignment of VREvent_ShowDevTools_t"]
        [::core::mem::align_of::<VREvent_ShowDevTools_t>() - 4usize];
    ["Offset of field: VREvent_ShowDevTools_t::nBrowserIdentifier"]
        [::core::mem::offset_of!(VREvent_ShowDevTools_t, nBrowserIdentifier) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_HDCPError_t {
    pub eCode: EHDCPError,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_HDCPError_t"][::core::mem::size_of::<VREvent_HDCPError_t>() - 4usize];
    ["Alignment of VREvent_HDCPError_t"][::core::mem::align_of::<VREvent_HDCPError_t>() - 4usize];
    ["Offset of field: VREvent_HDCPError_t::eCode"]
        [::core::mem::offset_of!(VREvent_HDCPError_t, eCode) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_AudioVolumeControl_t {
    pub fVolumeLevel: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_AudioVolumeControl_t"]
        [::core::mem::size_of::<VREvent_AudioVolumeControl_t>() - 4usize];
    ["Alignment of VREvent_AudioVolumeControl_t"]
        [::core::mem::align_of::<VREvent_AudioVolumeControl_t>() - 4usize];
    ["Offset of field: VREvent_AudioVolumeControl_t::fVolumeLevel"]
        [::core::mem::offset_of!(VREvent_AudioVolumeControl_t, fVolumeLevel) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_AudioMuteControl_t {
    pub bMute: bool,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_AudioMuteControl_t"]
        [::core::mem::size_of::<VREvent_AudioMuteControl_t>() - 1usize];
    ["Alignment of VREvent_AudioMuteControl_t"]
        [::core::mem::align_of::<VREvent_AudioMuteControl_t>() - 1usize];
    ["Offset of field: VREvent_AudioMuteControl_t::bMute"]
        [::core::mem::offset_of!(VREvent_AudioMuteControl_t, bMute) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RenderModel_ComponentState_t {
    pub mTrackingToComponentRenderModel: HmdMatrix34_t,
    pub mTrackingToComponentLocal: HmdMatrix34_t,
    pub uProperties: VRComponentProperties,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of RenderModel_ComponentState_t"]
        [::core::mem::size_of::<RenderModel_ComponentState_t>() - 100usize];
    ["Alignment of RenderModel_ComponentState_t"]
        [::core::mem::align_of::<RenderModel_ComponentState_t>() - 4usize];
    ["Offset of field: RenderModel_ComponentState_t::mTrackingToComponentRenderModel"][::core::mem::offset_of!(
        RenderModel_ComponentState_t,
        mTrackingToComponentRenderModel
    ) - 0usize];
    ["Offset of field: RenderModel_ComponentState_t::mTrackingToComponentLocal"][::core::mem::offset_of!(
        RenderModel_ComponentState_t,
        mTrackingToComponentLocal
    ) - 48usize];
    ["Offset of field: RenderModel_ComponentState_t::uProperties"]
        [::core::mem::offset_of!(RenderModel_ComponentState_t, uProperties) - 96usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HiddenAreaMesh_t {
    pub pVertexData: *mut HmdVector2_t,
    pub unTriangleCount: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of HiddenAreaMesh_t"][::core::mem::size_of::<HiddenAreaMesh_t>() - 16usize];
    ["Alignment of HiddenAreaMesh_t"][::core::mem::align_of::<HiddenAreaMesh_t>() - 8usize];
    ["Offset of field: HiddenAreaMesh_t::pVertexData"]
        [::core::mem::offset_of!(HiddenAreaMesh_t, pVertexData) - 0usize];
    ["Offset of field: HiddenAreaMesh_t::unTriangleCount"]
        [::core::mem::offset_of!(HiddenAreaMesh_t, unTriangleCount) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VRControllerAxis_t {
    pub x: f32,
    pub y: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VRControllerAxis_t"][::core::mem::size_of::<VRControllerAxis_t>() - 8usize];
    ["Alignment of VRControllerAxis_t"][::core::mem::align_of::<VRControllerAxis_t>() - 4usize];
    ["Offset of field: VRControllerAxis_t::x"]
        [::core::mem::offset_of!(VRControllerAxis_t, x) - 0usize];
    ["Offset of field: VRControllerAxis_t::y"]
        [::core::mem::offset_of!(VRControllerAxis_t, y) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VRControllerState_t {
    pub unPacketNum: u32,
    pub ulButtonPressed: u64,
    pub ulButtonTouched: u64,
    pub rAxis: [VRControllerAxis_t; 5usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VRControllerState_t"][::core::mem::size_of::<VRControllerState_t>() - 64usize];
    ["Alignment of VRControllerState_t"][::core::mem::align_of::<VRControllerState_t>() - 8usize];
    ["Offset of field: VRControllerState_t::unPacketNum"]
        [::core::mem::offset_of!(VRControllerState_t, unPacketNum) - 0usize];
    ["Offset of field: VRControllerState_t::ulButtonPressed"]
        [::core::mem::offset_of!(VRControllerState_t, ulButtonPressed) - 8usize];
    ["Offset of field: VRControllerState_t::ulButtonTouched"]
        [::core::mem::offset_of!(VRControllerState_t, ulButtonTouched) - 16usize];
    ["Offset of field: VRControllerState_t::rAxis"]
        [::core::mem::offset_of!(VRControllerState_t, rAxis) - 24usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DistortionCoordinate_t {
    pub u: f32,
    pub v: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of DistortionCoordinate_t"][::core::mem::size_of::<DistortionCoordinate_t>() - 8usize];
    ["Alignment of DistortionCoordinate_t"]
        [::core::mem::align_of::<DistortionCoordinate_t>() - 4usize];
    ["Offset of field: DistortionCoordinate_t::u"]
        [::core::mem::offset_of!(DistortionCoordinate_t, u) - 0usize];
    ["Offset of field: DistortionCoordinate_t::v"]
        [::core::mem::offset_of!(DistortionCoordinate_t, v) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AppOverrideKeys_t {
    pub pchKey: *mut ::core::ffi::c_char,
    pub pchValue: *mut ::core::ffi::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of AppOverrideKeys_t"][::core::mem::size_of::<AppOverrideKeys_t>() - 16usize];
    ["Alignment of AppOverrideKeys_t"][::core::mem::align_of::<AppOverrideKeys_t>() - 8usize];
    ["Offset of field: AppOverrideKeys_t::pchKey"]
        [::core::mem::offset_of!(AppOverrideKeys_t, pchKey) - 0usize];
    ["Offset of field: AppOverrideKeys_t::pchValue"]
        [::core::mem::offset_of!(AppOverrideKeys_t, pchValue) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VROverlayIntersectionParams_t {
    pub vSource: HmdVector3_t,
    pub vDirection: HmdVector3_t,
    pub eOrigin: ETrackingUniverseOrigin,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VROverlayIntersectionParams_t"]
        [::core::mem::size_of::<VROverlayIntersectionParams_t>() - 28usize];
    ["Alignment of VROverlayIntersectionParams_t"]
        [::core::mem::align_of::<VROverlayIntersectionParams_t>() - 4usize];
    ["Offset of field: VROverlayIntersectionParams_t::vSource"]
        [::core::mem::offset_of!(VROverlayIntersectionParams_t, vSource) - 0usize];
    ["Offset of field: VROverlayIntersectionParams_t::vDirection"]
        [::core::mem::offset_of!(VROverlayIntersectionParams_t, vDirection) - 12usize];
    ["Offset of field: VROverlayIntersectionParams_t::eOrigin"]
        [::core::mem::offset_of!(VROverlayIntersectionParams_t, eOrigin) - 24usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VROverlayIntersectionResults_t {
    pub vPoint: HmdVector3_t,
    pub vNormal: HmdVector3_t,
    pub vUVs: HmdVector2_t,
    pub fDistance: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VROverlayIntersectionResults_t"]
        [::core::mem::size_of::<VROverlayIntersectionResults_t>() - 36usize];
    ["Alignment of VROverlayIntersectionResults_t"]
        [::core::mem::align_of::<VROverlayIntersectionResults_t>() - 4usize];
    ["Offset of field: VROverlayIntersectionResults_t::vPoint"]
        [::core::mem::offset_of!(VROverlayIntersectionResults_t, vPoint) - 0usize];
    ["Offset of field: VROverlayIntersectionResults_t::vNormal"]
        [::core::mem::offset_of!(VROverlayIntersectionResults_t, vNormal) - 12usize];
    ["Offset of field: VROverlayIntersectionResults_t::vUVs"]
        [::core::mem::offset_of!(VROverlayIntersectionResults_t, vUVs) - 24usize];
    ["Offset of field: VROverlayIntersectionResults_t::fDistance"]
        [::core::mem::offset_of!(VROverlayIntersectionResults_t, fDistance) - 32usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IntersectionMaskRectangle_t {
    pub m_flTopLeftX: f32,
    pub m_flTopLeftY: f32,
    pub m_flWidth: f32,
    pub m_flHeight: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IntersectionMaskRectangle_t"]
        [::core::mem::size_of::<IntersectionMaskRectangle_t>() - 16usize];
    ["Alignment of IntersectionMaskRectangle_t"]
        [::core::mem::align_of::<IntersectionMaskRectangle_t>() - 4usize];
    ["Offset of field: IntersectionMaskRectangle_t::m_flTopLeftX"]
        [::core::mem::offset_of!(IntersectionMaskRectangle_t, m_flTopLeftX) - 0usize];
    ["Offset of field: IntersectionMaskRectangle_t::m_flTopLeftY"]
        [::core::mem::offset_of!(IntersectionMaskRectangle_t, m_flTopLeftY) - 4usize];
    ["Offset of field: IntersectionMaskRectangle_t::m_flWidth"]
        [::core::mem::offset_of!(IntersectionMaskRectangle_t, m_flWidth) - 8usize];
    ["Offset of field: IntersectionMaskRectangle_t::m_flHeight"]
        [::core::mem::offset_of!(IntersectionMaskRectangle_t, m_flHeight) - 12usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IntersectionMaskCircle_t {
    pub m_flCenterX: f32,
    pub m_flCenterY: f32,
    pub m_flRadius: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IntersectionMaskCircle_t"]
        [::core::mem::size_of::<IntersectionMaskCircle_t>() - 12usize];
    ["Alignment of IntersectionMaskCircle_t"]
        [::core::mem::align_of::<IntersectionMaskCircle_t>() - 4usize];
    ["Offset of field: IntersectionMaskCircle_t::m_flCenterX"]
        [::core::mem::offset_of!(IntersectionMaskCircle_t, m_flCenterX) - 0usize];
    ["Offset of field: IntersectionMaskCircle_t::m_flCenterY"]
        [::core::mem::offset_of!(IntersectionMaskCircle_t, m_flCenterY) - 4usize];
    ["Offset of field: IntersectionMaskCircle_t::m_flRadius"]
        [::core::mem::offset_of!(IntersectionMaskCircle_t, m_flRadius) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VROverlayProjection_t {
    pub fLeft: f32,
    pub fRight: f32,
    pub fTop: f32,
    pub fBottom: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VROverlayProjection_t"][::core::mem::size_of::<VROverlayProjection_t>() - 16usize];
    ["Alignment of VROverlayProjection_t"]
        [::core::mem::align_of::<VROverlayProjection_t>() - 4usize];
    ["Offset of field: VROverlayProjection_t::fLeft"]
        [::core::mem::offset_of!(VROverlayProjection_t, fLeft) - 0usize];
    ["Offset of field: VROverlayProjection_t::fRight"]
        [::core::mem::offset_of!(VROverlayProjection_t, fRight) - 4usize];
    ["Offset of field: VROverlayProjection_t::fTop"]
        [::core::mem::offset_of!(VROverlayProjection_t, fTop) - 8usize];
    ["Offset of field: VROverlayProjection_t::fBottom"]
        [::core::mem::offset_of!(VROverlayProjection_t, fBottom) - 12usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InputAnalogActionData_t {
    pub bActive: bool,
    pub activeOrigin: VRInputValueHandle_t,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub deltaX: f32,
    pub deltaY: f32,
    pub deltaZ: f32,
    pub fUpdateTime: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of InputAnalogActionData_t"]
        [::core::mem::size_of::<InputAnalogActionData_t>() - 48usize];
    ["Alignment of InputAnalogActionData_t"]
        [::core::mem::align_of::<InputAnalogActionData_t>() - 8usize];
    ["Offset of field: InputAnalogActionData_t::bActive"]
        [::core::mem::offset_of!(InputAnalogActionData_t, bActive) - 0usize];
    ["Offset of field: InputAnalogActionData_t::activeOrigin"]
        [::core::mem::offset_of!(InputAnalogActionData_t, activeOrigin) - 8usize];
    ["Offset of field: InputAnalogActionData_t::x"]
        [::core::mem::offset_of!(InputAnalogActionData_t, x) - 16usize];
    ["Offset of field: InputAnalogActionData_t::y"]
        [::core::mem::offset_of!(InputAnalogActionData_t, y) - 20usize];
    ["Offset of field: InputAnalogActionData_t::z"]
        [::core::mem::offset_of!(InputAnalogActionData_t, z) - 24usize];
    ["Offset of field: InputAnalogActionData_t::deltaX"]
        [::core::mem::offset_of!(InputAnalogActionData_t, deltaX) - 28usize];
    ["Offset of field: InputAnalogActionData_t::deltaY"]
        [::core::mem::offset_of!(InputAnalogActionData_t, deltaY) - 32usize];
    ["Offset of field: InputAnalogActionData_t::deltaZ"]
        [::core::mem::offset_of!(InputAnalogActionData_t, deltaZ) - 36usize];
    ["Offset of field: InputAnalogActionData_t::fUpdateTime"]
        [::core::mem::offset_of!(InputAnalogActionData_t, fUpdateTime) - 40usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InputDigitalActionData_t {
    pub bActive: bool,
    pub activeOrigin: VRInputValueHandle_t,
    pub bState: bool,
    pub bChanged: bool,
    pub fUpdateTime: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of InputDigitalActionData_t"]
        [::core::mem::size_of::<InputDigitalActionData_t>() - 24usize];
    ["Alignment of InputDigitalActionData_t"]
        [::core::mem::align_of::<InputDigitalActionData_t>() - 8usize];
    ["Offset of field: InputDigitalActionData_t::bActive"]
        [::core::mem::offset_of!(InputDigitalActionData_t, bActive) - 0usize];
    ["Offset of field: InputDigitalActionData_t::activeOrigin"]
        [::core::mem::offset_of!(InputDigitalActionData_t, activeOrigin) - 8usize];
    ["Offset of field: InputDigitalActionData_t::bState"]
        [::core::mem::offset_of!(InputDigitalActionData_t, bState) - 16usize];
    ["Offset of field: InputDigitalActionData_t::bChanged"]
        [::core::mem::offset_of!(InputDigitalActionData_t, bChanged) - 17usize];
    ["Offset of field: InputDigitalActionData_t::fUpdateTime"]
        [::core::mem::offset_of!(InputDigitalActionData_t, fUpdateTime) - 20usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InputPoseActionData_t {
    pub bActive: bool,
    pub activeOrigin: VRInputValueHandle_t,
    pub pose: TrackedDevicePose_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of InputPoseActionData_t"][::core::mem::size_of::<InputPoseActionData_t>() - 96usize];
    ["Alignment of InputPoseActionData_t"]
        [::core::mem::align_of::<InputPoseActionData_t>() - 8usize];
    ["Offset of field: InputPoseActionData_t::bActive"]
        [::core::mem::offset_of!(InputPoseActionData_t, bActive) - 0usize];
    ["Offset of field: InputPoseActionData_t::activeOrigin"]
        [::core::mem::offset_of!(InputPoseActionData_t, activeOrigin) - 8usize];
    ["Offset of field: InputPoseActionData_t::pose"]
        [::core::mem::offset_of!(InputPoseActionData_t, pose) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InputSkeletalActionData_t {
    pub bActive: bool,
    pub activeOrigin: VRInputValueHandle_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of InputSkeletalActionData_t"]
        [::core::mem::size_of::<InputSkeletalActionData_t>() - 16usize];
    ["Alignment of InputSkeletalActionData_t"]
        [::core::mem::align_of::<InputSkeletalActionData_t>() - 8usize];
    ["Offset of field: InputSkeletalActionData_t::bActive"]
        [::core::mem::offset_of!(InputSkeletalActionData_t, bActive) - 0usize];
    ["Offset of field: InputSkeletalActionData_t::activeOrigin"]
        [::core::mem::offset_of!(InputSkeletalActionData_t, activeOrigin) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InputOriginInfo_t {
    pub devicePath: VRInputValueHandle_t,
    pub trackedDeviceIndex: TrackedDeviceIndex_t,
    pub rchRenderModelComponentName: [::core::ffi::c_char; 128usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of InputOriginInfo_t"][::core::mem::size_of::<InputOriginInfo_t>() - 144usize];
    ["Alignment of InputOriginInfo_t"][::core::mem::align_of::<InputOriginInfo_t>() - 8usize];
    ["Offset of field: InputOriginInfo_t::devicePath"]
        [::core::mem::offset_of!(InputOriginInfo_t, devicePath) - 0usize];
    ["Offset of field: InputOriginInfo_t::trackedDeviceIndex"]
        [::core::mem::offset_of!(InputOriginInfo_t, trackedDeviceIndex) - 8usize];
    ["Offset of field: InputOriginInfo_t::rchRenderModelComponentName"]
        [::core::mem::offset_of!(InputOriginInfo_t, rchRenderModelComponentName) - 12usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InputBindingInfo_t {
    pub rchDevicePathName: [::core::ffi::c_char; 128usize],
    pub rchInputPathName: [::core::ffi::c_char; 128usize],
    pub rchModeName: [::core::ffi::c_char; 128usize],
    pub rchSlotName: [::core::ffi::c_char; 128usize],
    pub rchInputSourceType: [::core::ffi::c_char; 32usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of InputBindingInfo_t"][::core::mem::size_of::<InputBindingInfo_t>() - 544usize];
    ["Alignment of InputBindingInfo_t"][::core::mem::align_of::<InputBindingInfo_t>() - 1usize];
    ["Offset of field: InputBindingInfo_t::rchDevicePathName"]
        [::core::mem::offset_of!(InputBindingInfo_t, rchDevicePathName) - 0usize];
    ["Offset of field: InputBindingInfo_t::rchInputPathName"]
        [::core::mem::offset_of!(InputBindingInfo_t, rchInputPathName) - 128usize];
    ["Offset of field: InputBindingInfo_t::rchModeName"]
        [::core::mem::offset_of!(InputBindingInfo_t, rchModeName) - 256usize];
    ["Offset of field: InputBindingInfo_t::rchSlotName"]
        [::core::mem::offset_of!(InputBindingInfo_t, rchSlotName) - 384usize];
    ["Offset of field: InputBindingInfo_t::rchInputSourceType"]
        [::core::mem::offset_of!(InputBindingInfo_t, rchInputSourceType) - 512usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VRActiveActionSet_t {
    pub ulActionSet: VRActionSetHandle_t,
    pub ulRestrictedToDevice: VRInputValueHandle_t,
    pub ulSecondaryActionSet: VRActionSetHandle_t,
    pub unPadding: u32,
    pub nPriority: i32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VRActiveActionSet_t"][::core::mem::size_of::<VRActiveActionSet_t>() - 32usize];
    ["Alignment of VRActiveActionSet_t"][::core::mem::align_of::<VRActiveActionSet_t>() - 8usize];
    ["Offset of field: VRActiveActionSet_t::ulActionSet"]
        [::core::mem::offset_of!(VRActiveActionSet_t, ulActionSet) - 0usize];
    ["Offset of field: VRActiveActionSet_t::ulRestrictedToDevice"]
        [::core::mem::offset_of!(VRActiveActionSet_t, ulRestrictedToDevice) - 8usize];
    ["Offset of field: VRActiveActionSet_t::ulSecondaryActionSet"]
        [::core::mem::offset_of!(VRActiveActionSet_t, ulSecondaryActionSet) - 16usize];
    ["Offset of field: VRActiveActionSet_t::unPadding"]
        [::core::mem::offset_of!(VRActiveActionSet_t, unPadding) - 24usize];
    ["Offset of field: VRActiveActionSet_t::nPriority"]
        [::core::mem::offset_of!(VRActiveActionSet_t, nPriority) - 28usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VRSkeletalSummaryData_t {
    pub flFingerCurl: [f32; 5usize],
    pub flFingerSplay: [f32; 4usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VRSkeletalSummaryData_t"]
        [::core::mem::size_of::<VRSkeletalSummaryData_t>() - 36usize];
    ["Alignment of VRSkeletalSummaryData_t"]
        [::core::mem::align_of::<VRSkeletalSummaryData_t>() - 4usize];
    ["Offset of field: VRSkeletalSummaryData_t::flFingerCurl"]
        [::core::mem::offset_of!(VRSkeletalSummaryData_t, flFingerCurl) - 0usize];
    ["Offset of field: VRSkeletalSummaryData_t::flFingerSplay"]
        [::core::mem::offset_of!(VRSkeletalSummaryData_t, flFingerSplay) - 20usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub union VREvent_Data_t {
    pub reserved: VREvent_Reserved_t,
    pub controller: VREvent_Controller_t,
    pub mouse: VREvent_Mouse_t,
    pub scroll: VREvent_Scroll_t,
    pub process: VREvent_Process_t,
    pub notification: VREvent_Notification_t,
    pub overlay: VREvent_Overlay_t,
    pub status: VREvent_Status_t,
    pub keyboard: VREvent_Keyboard_t,
    pub ipd: VREvent_Ipd_t,
    pub chaperone: VREvent_Chaperone_t,
    pub performanceTest: VREvent_PerformanceTest_t,
    pub touchPadMove: VREvent_TouchPadMove_t,
    pub seatedZeroPoseReset: VREvent_SeatedZeroPoseReset_t,
    pub screenshot: VREvent_Screenshot_t,
    pub screenshotProgress: VREvent_ScreenshotProgress_t,
    pub applicationLaunch: VREvent_ApplicationLaunch_t,
    pub cameraSurface: VREvent_EditingCameraSurface_t,
    pub messageOverlay: VREvent_MessageOverlay_t,
    pub property: VREvent_Property_t,
    pub hapticVibration: VREvent_HapticVibration_t,
    pub webConsole: VREvent_WebConsole_t,
    pub inputBinding: VREvent_InputBindingLoad_t,
    pub actionManifest: VREvent_InputActionManifestLoad_t,
    pub spatialAnchor: VREvent_SpatialAnchor_t,
    pub progressUpdate: VREvent_ProgressUpdate_t,
    pub showUi: VREvent_ShowUI_t,
    pub showDevTools: VREvent_ShowDevTools_t,
    pub hdcpError: VREvent_HDCPError_t,
    pub audioVolumeControl: VREvent_AudioVolumeControl_t,
    pub audioMuteControl: VREvent_AudioMuteControl_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_Data_t"][::core::mem::size_of::<VREvent_Data_t>() - 48usize];
    ["Alignment of VREvent_Data_t"][::core::mem::align_of::<VREvent_Data_t>() - 8usize];
    ["Offset of field: VREvent_Data_t::reserved"]
        [::core::mem::offset_of!(VREvent_Data_t, reserved) - 0usize];
    ["Offset of field: VREvent_Data_t::controller"]
        [::core::mem::offset_of!(VREvent_Data_t, controller) - 0usize];
    ["Offset of field: VREvent_Data_t::mouse"]
        [::core::mem::offset_of!(VREvent_Data_t, mouse) - 0usize];
    ["Offset of field: VREvent_Data_t::scroll"]
        [::core::mem::offset_of!(VREvent_Data_t, scroll) - 0usize];
    ["Offset of field: VREvent_Data_t::process"]
        [::core::mem::offset_of!(VREvent_Data_t, process) - 0usize];
    ["Offset of field: VREvent_Data_t::notification"]
        [::core::mem::offset_of!(VREvent_Data_t, notification) - 0usize];
    ["Offset of field: VREvent_Data_t::overlay"]
        [::core::mem::offset_of!(VREvent_Data_t, overlay) - 0usize];
    ["Offset of field: VREvent_Data_t::status"]
        [::core::mem::offset_of!(VREvent_Data_t, status) - 0usize];
    ["Offset of field: VREvent_Data_t::keyboard"]
        [::core::mem::offset_of!(VREvent_Data_t, keyboard) - 0usize];
    ["Offset of field: VREvent_Data_t::ipd"][::core::mem::offset_of!(VREvent_Data_t, ipd) - 0usize];
    ["Offset of field: VREvent_Data_t::chaperone"]
        [::core::mem::offset_of!(VREvent_Data_t, chaperone) - 0usize];
    ["Offset of field: VREvent_Data_t::performanceTest"]
        [::core::mem::offset_of!(VREvent_Data_t, performanceTest) - 0usize];
    ["Offset of field: VREvent_Data_t::touchPadMove"]
        [::core::mem::offset_of!(VREvent_Data_t, touchPadMove) - 0usize];
    ["Offset of field: VREvent_Data_t::seatedZeroPoseReset"]
        [::core::mem::offset_of!(VREvent_Data_t, seatedZeroPoseReset) - 0usize];
    ["Offset of field: VREvent_Data_t::screenshot"]
        [::core::mem::offset_of!(VREvent_Data_t, screenshot) - 0usize];
    ["Offset of field: VREvent_Data_t::screenshotProgress"]
        [::core::mem::offset_of!(VREvent_Data_t, screenshotProgress) - 0usize];
    ["Offset of field: VREvent_Data_t::applicationLaunch"]
        [::core::mem::offset_of!(VREvent_Data_t, applicationLaunch) - 0usize];
    ["Offset of field: VREvent_Data_t::cameraSurface"]
        [::core::mem::offset_of!(VREvent_Data_t, cameraSurface) - 0usize];
    ["Offset of field: VREvent_Data_t::messageOverlay"]
        [::core::mem::offset_of!(VREvent_Data_t, messageOverlay) - 0usize];
    ["Offset of field: VREvent_Data_t::property"]
        [::core::mem::offset_of!(VREvent_Data_t, property) - 0usize];
    ["Offset of field: VREvent_Data_t::hapticVibration"]
        [::core::mem::offset_of!(VREvent_Data_t, hapticVibration) - 0usize];
    ["Offset of field: VREvent_Data_t::webConsole"]
        [::core::mem::offset_of!(VREvent_Data_t, webConsole) - 0usize];
    ["Offset of field: VREvent_Data_t::inputBinding"]
        [::core::mem::offset_of!(VREvent_Data_t, inputBinding) - 0usize];
    ["Offset of field: VREvent_Data_t::actionManifest"]
        [::core::mem::offset_of!(VREvent_Data_t, actionManifest) - 0usize];
    ["Offset of field: VREvent_Data_t::spatialAnchor"]
        [::core::mem::offset_of!(VREvent_Data_t, spatialAnchor) - 0usize];
    ["Offset of field: VREvent_Data_t::progressUpdate"]
        [::core::mem::offset_of!(VREvent_Data_t, progressUpdate) - 0usize];
    ["Offset of field: VREvent_Data_t::showUi"]
        [::core::mem::offset_of!(VREvent_Data_t, showUi) - 0usize];
    ["Offset of field: VREvent_Data_t::showDevTools"]
        [::core::mem::offset_of!(VREvent_Data_t, showDevTools) - 0usize];
    ["Offset of field: VREvent_Data_t::hdcpError"]
        [::core::mem::offset_of!(VREvent_Data_t, hdcpError) - 0usize];
    ["Offset of field: VREvent_Data_t::audioVolumeControl"]
        [::core::mem::offset_of!(VREvent_Data_t, audioVolumeControl) - 0usize];
    ["Offset of field: VREvent_Data_t::audioMuteControl"]
        [::core::mem::offset_of!(VREvent_Data_t, audioMuteControl) - 0usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VREvent_t {
    pub eventType: u32,
    pub trackedDeviceIndex: TrackedDeviceIndex_t,
    pub eventAgeSeconds: f32,
    pub data: VREvent_Data_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VREvent_t"][::core::mem::size_of::<VREvent_t>() - 64usize];
    ["Alignment of VREvent_t"][::core::mem::align_of::<VREvent_t>() - 8usize];
    ["Offset of field: VREvent_t::eventType"]
        [::core::mem::offset_of!(VREvent_t, eventType) - 0usize];
    ["Offset of field: VREvent_t::trackedDeviceIndex"]
        [::core::mem::offset_of!(VREvent_t, trackedDeviceIndex) - 4usize];
    ["Offset of field: VREvent_t::eventAgeSeconds"]
        [::core::mem::offset_of!(VREvent_t, eventAgeSeconds) - 8usize];
    ["Offset of field: VREvent_t::data"][::core::mem::offset_of!(VREvent_t, data) - 16usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub union VROverlayIntersectionMaskPrimitive_Data_t {
    pub m_Rectangle: IntersectionMaskRectangle_t,
    pub m_Circle: IntersectionMaskCircle_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VROverlayIntersectionMaskPrimitive_Data_t"]
        [::core::mem::size_of::<VROverlayIntersectionMaskPrimitive_Data_t>() - 16usize];
    ["Alignment of VROverlayIntersectionMaskPrimitive_Data_t"]
        [::core::mem::align_of::<VROverlayIntersectionMaskPrimitive_Data_t>() - 4usize];
    ["Offset of field: VROverlayIntersectionMaskPrimitive_Data_t::m_Rectangle"]
        [::core::mem::offset_of!(VROverlayIntersectionMaskPrimitive_Data_t, m_Rectangle) - 0usize];
    ["Offset of field: VROverlayIntersectionMaskPrimitive_Data_t::m_Circle"]
        [::core::mem::offset_of!(VROverlayIntersectionMaskPrimitive_Data_t, m_Circle) - 0usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VROverlayIntersectionMaskPrimitive_t {
    pub m_nPrimitiveType: EVROverlayIntersectionMaskPrimitiveType,
    pub m_Primitive: VROverlayIntersectionMaskPrimitive_Data_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VROverlayIntersectionMaskPrimitive_t"]
        [::core::mem::size_of::<VROverlayIntersectionMaskPrimitive_t>() - 20usize];
    ["Alignment of VROverlayIntersectionMaskPrimitive_t"]
        [::core::mem::align_of::<VROverlayIntersectionMaskPrimitive_t>() - 4usize];
    ["Offset of field: VROverlayIntersectionMaskPrimitive_t::m_nPrimitiveType"]
        [::core::mem::offset_of!(VROverlayIntersectionMaskPrimitive_t, m_nPrimitiveType) - 0usize];
    ["Offset of field: VROverlayIntersectionMaskPrimitive_t::m_Primitive"]
        [::core::mem::offset_of!(VROverlayIntersectionMaskPrimitive_t, m_Primitive) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VR_IVRSystem_FnTable {
    pub GetRecommendedRenderTargetSize:
        ::core::option::Option<unsafe extern "C" fn(pnWidth: *mut u32, pnHeight: *mut u32)>,
    pub GetProjectionMatrix: ::core::option::Option<
        unsafe extern "C" fn(eEye: EVREye, fNearZ: f32, fFarZ: f32) -> HmdMatrix44_t,
    >,
    pub GetProjectionRaw: ::core::option::Option<
        unsafe extern "C" fn(
            eEye: EVREye,
            pfLeft: *mut f32,
            pfRight: *mut f32,
            pfTop: *mut f32,
            pfBottom: *mut f32,
        ),
    >,
    pub ComputeDistortion: ::core::option::Option<
        unsafe extern "C" fn(
            eEye: EVREye,
            fU: f32,
            fV: f32,
            pDistortionCoordinates: *mut DistortionCoordinates_t,
        ) -> bool,
    >,
    pub ComputeDistortionSet: ::core::option::Option<
        unsafe extern "C" fn(
            eEye: EVREye,
            eChannel: EVRDistortionChannel,
            bAsNormalizedDeviceCoordinates: bool,
            nNumCoordinates: u32,
            pInput: *mut DistortionCoordinate_t,
            pOutput: *mut DistortionCoordinate_t,
        ) -> bool,
    >,
    pub GetEyeToHeadTransform:
        ::core::option::Option<unsafe extern "C" fn(eEye: EVREye) -> HmdMatrix34_t>,
    pub GetTimeSinceLastVsync: ::core::option::Option<
        unsafe extern "C" fn(pfSecondsSinceLastVsync: *mut f32, pulFrameCounter: *mut u64) -> bool,
    >,
    pub GetD3D9AdapterIndex: ::core::option::Option<unsafe extern "C" fn() -> i32>,
    pub GetDXGIOutputInfo: ::core::option::Option<unsafe extern "C" fn(pnAdapterIndex: *mut i32)>,
    pub GetOutputDevice: ::core::option::Option<
        unsafe extern "C" fn(
            pnDevice: *mut u64,
            textureType: ETextureType,
            pInstance: *mut VkInstance_T,
        ),
    >,
    pub IsDisplayOnDesktop: ::core::option::Option<unsafe extern "C" fn() -> bool>,
    pub SetDisplayVisibility:
        ::core::option::Option<unsafe extern "C" fn(bIsVisibleOnDesktop: bool) -> bool>,
    pub GetDeviceToAbsoluteTrackingPose: ::core::option::Option<
        unsafe extern "C" fn(
            eOrigin: ETrackingUniverseOrigin,
            fPredictedSecondsToPhotonsFromNow: f32,
            pTrackedDevicePoseArray: *mut TrackedDevicePose_t,
            unTrackedDevicePoseArrayCount: u32,
        ),
    >,
    pub GetSeatedZeroPoseToStandingAbsoluteTrackingPose:
        ::core::option::Option<unsafe extern "C" fn() -> HmdMatrix34_t>,
    pub GetRawZeroPoseToStandingAbsoluteTrackingPose:
        ::core::option::Option<unsafe extern "C" fn() -> HmdMatrix34_t>,
    pub GetSortedTrackedDeviceIndicesOfClass: ::core::option::Option<
        unsafe extern "C" fn(
            eTrackedDeviceClass: ETrackedDeviceClass,
            punTrackedDeviceIndexArray: *mut TrackedDeviceIndex_t,
            unTrackedDeviceIndexArrayCount: u32,
            unRelativeToTrackedDeviceIndex: TrackedDeviceIndex_t,
        ) -> u32,
    >,
    pub GetTrackedDeviceActivityLevel: ::core::option::Option<
        unsafe extern "C" fn(unDeviceId: TrackedDeviceIndex_t) -> EDeviceActivityLevel,
    >,
    pub ApplyTransform: ::core::option::Option<
        unsafe extern "C" fn(
            pOutputPose: *mut TrackedDevicePose_t,
            pTrackedDevicePose: *mut TrackedDevicePose_t,
            pTransform: *mut HmdMatrix34_t,
        ),
    >,
    pub GetTrackedDeviceIndexForControllerRole: ::core::option::Option<
        unsafe extern "C" fn(unDeviceType: ETrackedControllerRole) -> TrackedDeviceIndex_t,
    >,
    pub GetControllerRoleForTrackedDeviceIndex: ::core::option::Option<
        unsafe extern "C" fn(unDeviceIndex: TrackedDeviceIndex_t) -> ETrackedControllerRole,
    >,
    pub GetTrackedDeviceClass: ::core::option::Option<
        unsafe extern "C" fn(unDeviceIndex: TrackedDeviceIndex_t) -> ETrackedDeviceClass,
    >,
    pub IsTrackedDeviceConnected:
        ::core::option::Option<unsafe extern "C" fn(unDeviceIndex: TrackedDeviceIndex_t) -> bool>,
    pub GetBoolTrackedDeviceProperty: ::core::option::Option<
        unsafe extern "C" fn(
            unDeviceIndex: TrackedDeviceIndex_t,
            prop: ETrackedDeviceProperty,
            pError: *mut ETrackedPropertyError,
        ) -> bool,
    >,
    pub GetFloatTrackedDeviceProperty: ::core::option::Option<
        unsafe extern "C" fn(
            unDeviceIndex: TrackedDeviceIndex_t,
            prop: ETrackedDeviceProperty,
            pError: *mut ETrackedPropertyError,
        ) -> f32,
    >,
    pub GetInt32TrackedDeviceProperty: ::core::option::Option<
        unsafe extern "C" fn(
            unDeviceIndex: TrackedDeviceIndex_t,
            prop: ETrackedDeviceProperty,
            pError: *mut ETrackedPropertyError,
        ) -> i32,
    >,
    pub GetUint64TrackedDeviceProperty: ::core::option::Option<
        unsafe extern "C" fn(
            unDeviceIndex: TrackedDeviceIndex_t,
            prop: ETrackedDeviceProperty,
            pError: *mut ETrackedPropertyError,
        ) -> u64,
    >,
    pub GetMatrix34TrackedDeviceProperty: ::core::option::Option<
        unsafe extern "C" fn(
            unDeviceIndex: TrackedDeviceIndex_t,
            prop: ETrackedDeviceProperty,
            pError: *mut ETrackedPropertyError,
        ) -> HmdMatrix34_t,
    >,
    pub GetArrayTrackedDeviceProperty: ::core::option::Option<
        unsafe extern "C" fn(
            unDeviceIndex: TrackedDeviceIndex_t,
            prop: ETrackedDeviceProperty,
            propType: PropertyTypeTag_t,
            pBuffer: *mut ::core::ffi::c_void,
            unBufferSize: u32,
            pError: *mut ETrackedPropertyError,
        ) -> u32,
    >,
    pub GetStringTrackedDeviceProperty: ::core::option::Option<
        unsafe extern "C" fn(
            unDeviceIndex: TrackedDeviceIndex_t,
            prop: ETrackedDeviceProperty,
            pchValue: *mut ::core::ffi::c_char,
            unBufferSize: u32,
            pError: *mut ETrackedPropertyError,
        ) -> u32,
    >,
    pub GetPropErrorNameFromEnum: ::core::option::Option<
        unsafe extern "C" fn(error: ETrackedPropertyError) -> *mut ::core::ffi::c_char,
    >,
    pub PollNextEvent: ::core::option::Option<
        unsafe extern "C" fn(pEvent: *mut VREvent_t, uncbVREvent: u32) -> bool,
    >,
    pub PollNextEventWithPose: ::core::option::Option<
        unsafe extern "C" fn(
            eOrigin: ETrackingUniverseOrigin,
            pEvent: *mut VREvent_t,
            uncbVREvent: u32,
            pTrackedDevicePose: *mut TrackedDevicePose_t,
        ) -> bool,
    >,
    pub PollNextEventWithPoseAndOverlays: ::core::option::Option<
        unsafe extern "C" fn(
            eOrigin: ETrackingUniverseOrigin,
            pEvent: *mut VREvent_t,
            uncbVREvent: u32,
            pTrackedDevicePose: *mut TrackedDevicePose_t,
            pulOverlayHandle: *mut VROverlayHandle_t,
        ) -> bool,
    >,
    pub GetEventTypeNameFromEnum: ::core::option::Option<
        unsafe extern "C" fn(eType: EVREventType) -> *mut ::core::ffi::c_char,
    >,
    pub GetHiddenAreaMesh: ::core::option::Option<
        unsafe extern "C" fn(eEye: EVREye, type_: EHiddenAreaMeshType) -> HiddenAreaMesh_t,
    >,
    pub GetEyeTrackedFoveationCenter: ::core::option::Option<
        unsafe extern "C" fn(pNdcLeft: *mut HmdVector2_t, pNdcRight: *mut HmdVector2_t) -> bool,
    >,
    pub GetEyeTrackedFoveationCenterForProjection: ::core::option::Option<
        unsafe extern "C" fn(pProjMat: *mut HmdMatrix44_t, pNdc: *mut HmdVector2_t) -> bool,
    >,
    pub GetControllerState: ::core::option::Option<
        unsafe extern "C" fn(
            unControllerDeviceIndex: TrackedDeviceIndex_t,
            pControllerState: *mut VRControllerState_t,
            unControllerStateSize: u32,
        ) -> bool,
    >,
    pub GetControllerStateWithPose: ::core::option::Option<
        unsafe extern "C" fn(
            eOrigin: ETrackingUniverseOrigin,
            unControllerDeviceIndex: TrackedDeviceIndex_t,
            pControllerState: *mut VRControllerState_t,
            unControllerStateSize: u32,
            pTrackedDevicePose: *mut TrackedDevicePose_t,
        ) -> bool,
    >,
    pub TriggerHapticPulse: ::core::option::Option<
        unsafe extern "C" fn(
            unControllerDeviceIndex: TrackedDeviceIndex_t,
            unAxisId: u32,
            usDurationMicroSec: ::core::ffi::c_ushort,
        ),
    >,
    pub GetButtonIdNameFromEnum: ::core::option::Option<
        unsafe extern "C" fn(eButtonId: EVRButtonId) -> *mut ::core::ffi::c_char,
    >,
    pub GetControllerAxisTypeNameFromEnum: ::core::option::Option<
        unsafe extern "C" fn(eAxisType: EVRControllerAxisType) -> *mut ::core::ffi::c_char,
    >,
    pub IsInputAvailable: ::core::option::Option<unsafe extern "C" fn() -> bool>,
    pub IsSteamVRDrawingControllers: ::core::option::Option<unsafe extern "C" fn() -> bool>,
    pub ShouldApplicationPause: ::core::option::Option<unsafe extern "C" fn() -> bool>,
    pub ShouldApplicationReduceRenderingWork:
        ::core::option::Option<unsafe extern "C" fn() -> bool>,
    pub PerformFirmwareUpdate: ::core::option::Option<
        unsafe extern "C" fn(unDeviceIndex: TrackedDeviceIndex_t) -> EVRFirmwareError,
    >,
    pub AcknowledgeQuit_Exiting: ::core::option::Option<unsafe extern "C" fn()>,
    pub GetAppContainerFilePaths: ::core::option::Option<
        unsafe extern "C" fn(pchBuffer: *mut ::core::ffi::c_char, unBufferSize: u32) -> u32,
    >,
    pub GetRuntimeVersion:
        ::core::option::Option<unsafe extern "C" fn() -> *mut ::core::ffi::c_char>,
    pub SetSDKVersion: ::core::option::Option<
        unsafe extern "C" fn(
            nVersionMajor: u32,
            nVersionMinor: u32,
            nVersionBuild: u32,
        ) -> EVRInitError,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VR_IVRSystem_FnTable"][::core::mem::size_of::<VR_IVRSystem_FnTable>() - 408usize];
    ["Alignment of VR_IVRSystem_FnTable"][::core::mem::align_of::<VR_IVRSystem_FnTable>() - 8usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetRecommendedRenderTargetSize"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, GetRecommendedRenderTargetSize) - 0usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetProjectionMatrix"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, GetProjectionMatrix) - 8usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetProjectionRaw"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, GetProjectionRaw) - 16usize];
    ["Offset of field: VR_IVRSystem_FnTable::ComputeDistortion"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, ComputeDistortion) - 24usize];
    ["Offset of field: VR_IVRSystem_FnTable::ComputeDistortionSet"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, ComputeDistortionSet) - 32usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetEyeToHeadTransform"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, GetEyeToHeadTransform) - 40usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetTimeSinceLastVsync"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, GetTimeSinceLastVsync) - 48usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetD3D9AdapterIndex"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, GetD3D9AdapterIndex) - 56usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetDXGIOutputInfo"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, GetDXGIOutputInfo) - 64usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetOutputDevice"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, GetOutputDevice) - 72usize];
    ["Offset of field: VR_IVRSystem_FnTable::IsDisplayOnDesktop"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, IsDisplayOnDesktop) - 80usize];
    ["Offset of field: VR_IVRSystem_FnTable::SetDisplayVisibility"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, SetDisplayVisibility) - 88usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetDeviceToAbsoluteTrackingPose"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, GetDeviceToAbsoluteTrackingPose) - 96usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetSeatedZeroPoseToStandingAbsoluteTrackingPose"][::core::mem::offset_of!(
        VR_IVRSystem_FnTable,
        GetSeatedZeroPoseToStandingAbsoluteTrackingPose
    )
        - 104usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetRawZeroPoseToStandingAbsoluteTrackingPose"][::core::mem::offset_of!(
        VR_IVRSystem_FnTable,
        GetRawZeroPoseToStandingAbsoluteTrackingPose
    )
        - 112usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetSortedTrackedDeviceIndicesOfClass"][::core::mem::offset_of!(
        VR_IVRSystem_FnTable,
        GetSortedTrackedDeviceIndicesOfClass
    ) - 120usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetTrackedDeviceActivityLevel"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, GetTrackedDeviceActivityLevel) - 128usize];
    ["Offset of field: VR_IVRSystem_FnTable::ApplyTransform"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, ApplyTransform) - 136usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetTrackedDeviceIndexForControllerRole"][::core::mem::offset_of!(
        VR_IVRSystem_FnTable,
        GetTrackedDeviceIndexForControllerRole
    ) - 144usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetControllerRoleForTrackedDeviceIndex"][::core::mem::offset_of!(
        VR_IVRSystem_FnTable,
        GetControllerRoleForTrackedDeviceIndex
    ) - 152usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetTrackedDeviceClass"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, GetTrackedDeviceClass) - 160usize];
    ["Offset of field: VR_IVRSystem_FnTable::IsTrackedDeviceConnected"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, IsTrackedDeviceConnected) - 168usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetBoolTrackedDeviceProperty"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, GetBoolTrackedDeviceProperty) - 176usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetFloatTrackedDeviceProperty"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, GetFloatTrackedDeviceProperty) - 184usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetInt32TrackedDeviceProperty"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, GetInt32TrackedDeviceProperty) - 192usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetUint64TrackedDeviceProperty"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, GetUint64TrackedDeviceProperty) - 200usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetMatrix34TrackedDeviceProperty"][::core::mem::offset_of!(
        VR_IVRSystem_FnTable,
        GetMatrix34TrackedDeviceProperty
    ) - 208usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetArrayTrackedDeviceProperty"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, GetArrayTrackedDeviceProperty) - 216usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetStringTrackedDeviceProperty"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, GetStringTrackedDeviceProperty) - 224usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetPropErrorNameFromEnum"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, GetPropErrorNameFromEnum) - 232usize];
    ["Offset of field: VR_IVRSystem_FnTable::PollNextEvent"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, PollNextEvent) - 240usize];
    ["Offset of field: VR_IVRSystem_FnTable::PollNextEventWithPose"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, PollNextEventWithPose) - 248usize];
    ["Offset of field: VR_IVRSystem_FnTable::PollNextEventWithPoseAndOverlays"][::core::mem::offset_of!(
        VR_IVRSystem_FnTable,
        PollNextEventWithPoseAndOverlays
    ) - 256usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetEventTypeNameFromEnum"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, GetEventTypeNameFromEnum) - 264usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetHiddenAreaMesh"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, GetHiddenAreaMesh) - 272usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetEyeTrackedFoveationCenter"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, GetEyeTrackedFoveationCenter) - 280usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetEyeTrackedFoveationCenterForProjection"][::core::mem::offset_of!(
        VR_IVRSystem_FnTable,
        GetEyeTrackedFoveationCenterForProjection
    )
        - 288usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetControllerState"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, GetControllerState) - 296usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetControllerStateWithPose"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, GetControllerStateWithPose) - 304usize];
    ["Offset of field: VR_IVRSystem_FnTable::TriggerHapticPulse"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, TriggerHapticPulse) - 312usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetButtonIdNameFromEnum"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, GetButtonIdNameFromEnum) - 320usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetControllerAxisTypeNameFromEnum"][::core::mem::offset_of!(
        VR_IVRSystem_FnTable,
        GetControllerAxisTypeNameFromEnum
    ) - 328usize];
    ["Offset of field: VR_IVRSystem_FnTable::IsInputAvailable"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, IsInputAvailable) - 336usize];
    ["Offset of field: VR_IVRSystem_FnTable::IsSteamVRDrawingControllers"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, IsSteamVRDrawingControllers) - 344usize];
    ["Offset of field: VR_IVRSystem_FnTable::ShouldApplicationPause"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, ShouldApplicationPause) - 352usize];
    ["Offset of field: VR_IVRSystem_FnTable::ShouldApplicationReduceRenderingWork"][::core::mem::offset_of!(
        VR_IVRSystem_FnTable,
        ShouldApplicationReduceRenderingWork
    ) - 360usize];
    ["Offset of field: VR_IVRSystem_FnTable::PerformFirmwareUpdate"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, PerformFirmwareUpdate) - 368usize];
    ["Offset of field: VR_IVRSystem_FnTable::AcknowledgeQuit_Exiting"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, AcknowledgeQuit_Exiting) - 376usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetAppContainerFilePaths"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, GetAppContainerFilePaths) - 384usize];
    ["Offset of field: VR_IVRSystem_FnTable::GetRuntimeVersion"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, GetRuntimeVersion) - 392usize];
    ["Offset of field: VR_IVRSystem_FnTable::SetSDKVersion"]
        [::core::mem::offset_of!(VR_IVRSystem_FnTable, SetSDKVersion) - 400usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VR_IVRApplications_FnTable {
    pub AddApplicationManifest: ::core::option::Option<
        unsafe extern "C" fn(
            pchApplicationManifestFullPath: *mut ::core::ffi::c_char,
            bTemporary: bool,
        ) -> EVRApplicationError,
    >,
    pub RemoveApplicationManifest: ::core::option::Option<
        unsafe extern "C" fn(
            pchApplicationManifestFullPath: *mut ::core::ffi::c_char,
        ) -> EVRApplicationError,
    >,
    pub IsApplicationInstalled:
        ::core::option::Option<unsafe extern "C" fn(pchAppKey: *mut ::core::ffi::c_char) -> bool>,
    pub GetApplicationCount: ::core::option::Option<unsafe extern "C" fn() -> u32>,
    pub GetApplicationKeyByIndex: ::core::option::Option<
        unsafe extern "C" fn(
            unApplicationIndex: u32,
            pchAppKeyBuffer: *mut ::core::ffi::c_char,
            unAppKeyBufferLen: u32,
        ) -> EVRApplicationError,
    >,
    pub GetApplicationKeyByProcessId: ::core::option::Option<
        unsafe extern "C" fn(
            unProcessId: u32,
            pchAppKeyBuffer: *mut ::core::ffi::c_char,
            unAppKeyBufferLen: u32,
        ) -> EVRApplicationError,
    >,
    pub LaunchApplication: ::core::option::Option<
        unsafe extern "C" fn(pchAppKey: *mut ::core::ffi::c_char) -> EVRApplicationError,
    >,
    pub LaunchTemplateApplication: ::core::option::Option<
        unsafe extern "C" fn(
            pchTemplateAppKey: *mut ::core::ffi::c_char,
            pchNewAppKey: *mut ::core::ffi::c_char,
            pKeys: *mut AppOverrideKeys_t,
            unKeys: u32,
        ) -> EVRApplicationError,
    >,
    pub LaunchApplicationFromMimeType: ::core::option::Option<
        unsafe extern "C" fn(
            pchMimeType: *mut ::core::ffi::c_char,
            pchArgs: *mut ::core::ffi::c_char,
        ) -> EVRApplicationError,
    >,
    pub LaunchDashboardOverlay: ::core::option::Option<
        unsafe extern "C" fn(pchAppKey: *mut ::core::ffi::c_char) -> EVRApplicationError,
    >,
    pub CancelApplicationLaunch:
        ::core::option::Option<unsafe extern "C" fn(pchAppKey: *mut ::core::ffi::c_char) -> bool>,
    pub IdentifyApplication: ::core::option::Option<
        unsafe extern "C" fn(
            unProcessId: u32,
            pchAppKey: *mut ::core::ffi::c_char,
        ) -> EVRApplicationError,
    >,
    pub GetApplicationProcessId:
        ::core::option::Option<unsafe extern "C" fn(pchAppKey: *mut ::core::ffi::c_char) -> u32>,
    pub GetApplicationsErrorNameFromEnum: ::core::option::Option<
        unsafe extern "C" fn(error: EVRApplicationError) -> *mut ::core::ffi::c_char,
    >,
    pub GetApplicationPropertyString: ::core::option::Option<
        unsafe extern "C" fn(
            pchAppKey: *mut ::core::ffi::c_char,
            eProperty: EVRApplicationProperty,
            pchPropertyValueBuffer: *mut ::core::ffi::c_char,
            unPropertyValueBufferLen: u32,
            peError: *mut EVRApplicationError,
        ) -> u32,
    >,
    pub GetApplicationPropertyBool: ::core::option::Option<
        unsafe extern "C" fn(
            pchAppKey: *mut ::core::ffi::c_char,
            eProperty: EVRApplicationProperty,
            peError: *mut EVRApplicationError,
        ) -> bool,
    >,
    pub GetApplicationPropertyUint64: ::core::option::Option<
        unsafe extern "C" fn(
            pchAppKey: *mut ::core::ffi::c_char,
            eProperty: EVRApplicationProperty,
            peError: *mut EVRApplicationError,
        ) -> u64,
    >,
    pub SetApplicationAutoLaunch: ::core::option::Option<
        unsafe extern "C" fn(
            pchAppKey: *mut ::core::ffi::c_char,
            bAutoLaunch: bool,
        ) -> EVRApplicationError,
    >,
    pub GetApplicationAutoLaunch:
        ::core::option::Option<unsafe extern "C" fn(pchAppKey: *mut ::core::ffi::c_char) -> bool>,
    pub SetDefaultApplicationForMimeType: ::core::option::Option<
        unsafe extern "C" fn(
            pchAppKey: *mut ::core::ffi::c_char,
            pchMimeType: *mut ::core::ffi::c_char,
        ) -> EVRApplicationError,
    >,
    pub GetDefaultApplicationForMimeType: ::core::option::Option<
        unsafe extern "C" fn(
            pchMimeType: *mut ::core::ffi::c_char,
            pchAppKeyBuffer: *mut ::core::ffi::c_char,
            unAppKeyBufferLen: u32,
        ) -> bool,
    >,
    pub GetApplicationSupportedMimeTypes: ::core::option::Option<
        unsafe extern "C" fn(
            pchAppKey: *mut ::core::ffi::c_char,
            pchMimeTypesBuffer: *mut ::core::ffi::c_char,
            unMimeTypesBuffer: u32,
        ) -> bool,
    >,
    pub GetApplicationsThatSupportMimeType: ::core::option::Option<
        unsafe extern "C" fn(
            pchMimeType: *mut ::core::ffi::c_char,
            pchAppKeysThatSupportBuffer: *mut ::core::ffi::c_char,
            unAppKeysThatSupportBuffer: u32,
        ) -> u32,
    >,
    pub GetApplicationLaunchArguments: ::core::option::Option<
        unsafe extern "C" fn(unHandle: u32, pchArgs: *mut ::core::ffi::c_char, unArgs: u32) -> u32,
    >,
    pub GetStartingApplication: ::core::option::Option<
        unsafe extern "C" fn(
            pchAppKeyBuffer: *mut ::core::ffi::c_char,
            unAppKeyBufferLen: u32,
        ) -> EVRApplicationError,
    >,
    pub GetSceneApplicationState:
        ::core::option::Option<unsafe extern "C" fn() -> EVRSceneApplicationState>,
    pub PerformApplicationPrelaunchCheck: ::core::option::Option<
        unsafe extern "C" fn(pchAppKey: *mut ::core::ffi::c_char) -> EVRApplicationError,
    >,
    pub GetSceneApplicationStateNameFromEnum: ::core::option::Option<
        unsafe extern "C" fn(state: EVRSceneApplicationState) -> *mut ::core::ffi::c_char,
    >,
    pub LaunchInternalProcess: ::core::option::Option<
        unsafe extern "C" fn(
            pchBinaryPath: *mut ::core::ffi::c_char,
            pchArguments: *mut ::core::ffi::c_char,
            pchWorkingDirectory: *mut ::core::ffi::c_char,
        ) -> EVRApplicationError,
    >,
    pub RegisterSubprocess:
        ::core::option::Option<unsafe extern "C" fn(nPid: u32) -> EVRApplicationError>,
    pub GetCurrentSceneProcessId: ::core::option::Option<unsafe extern "C" fn() -> u32>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VR_IVRApplications_FnTable"]
        [::core::mem::size_of::<VR_IVRApplications_FnTable>() - 248usize];
    ["Alignment of VR_IVRApplications_FnTable"]
        [::core::mem::align_of::<VR_IVRApplications_FnTable>() - 8usize];
    ["Offset of field: VR_IVRApplications_FnTable::AddApplicationManifest"]
        [::core::mem::offset_of!(VR_IVRApplications_FnTable, AddApplicationManifest) - 0usize];
    ["Offset of field: VR_IVRApplications_FnTable::RemoveApplicationManifest"]
        [::core::mem::offset_of!(VR_IVRApplications_FnTable, RemoveApplicationManifest) - 8usize];
    ["Offset of field: VR_IVRApplications_FnTable::IsApplicationInstalled"]
        [::core::mem::offset_of!(VR_IVRApplications_FnTable, IsApplicationInstalled) - 16usize];
    ["Offset of field: VR_IVRApplications_FnTable::GetApplicationCount"]
        [::core::mem::offset_of!(VR_IVRApplications_FnTable, GetApplicationCount) - 24usize];
    ["Offset of field: VR_IVRApplications_FnTable::GetApplicationKeyByIndex"]
        [::core::mem::offset_of!(VR_IVRApplications_FnTable, GetApplicationKeyByIndex) - 32usize];
    ["Offset of field: VR_IVRApplications_FnTable::GetApplicationKeyByProcessId"][::core::mem::offset_of!(
        VR_IVRApplications_FnTable,
        GetApplicationKeyByProcessId
    ) - 40usize];
    ["Offset of field: VR_IVRApplications_FnTable::LaunchApplication"]
        [::core::mem::offset_of!(VR_IVRApplications_FnTable, LaunchApplication) - 48usize];
    ["Offset of field: VR_IVRApplications_FnTable::LaunchTemplateApplication"]
        [::core::mem::offset_of!(VR_IVRApplications_FnTable, LaunchTemplateApplication) - 56usize];
    ["Offset of field: VR_IVRApplications_FnTable::LaunchApplicationFromMimeType"][::core::mem::offset_of!(
        VR_IVRApplications_FnTable,
        LaunchApplicationFromMimeType
    ) - 64usize];
    ["Offset of field: VR_IVRApplications_FnTable::LaunchDashboardOverlay"]
        [::core::mem::offset_of!(VR_IVRApplications_FnTable, LaunchDashboardOverlay) - 72usize];
    ["Offset of field: VR_IVRApplications_FnTable::CancelApplicationLaunch"]
        [::core::mem::offset_of!(VR_IVRApplications_FnTable, CancelApplicationLaunch) - 80usize];
    ["Offset of field: VR_IVRApplications_FnTable::IdentifyApplication"]
        [::core::mem::offset_of!(VR_IVRApplications_FnTable, IdentifyApplication) - 88usize];
    ["Offset of field: VR_IVRApplications_FnTable::GetApplicationProcessId"]
        [::core::mem::offset_of!(VR_IVRApplications_FnTable, GetApplicationProcessId) - 96usize];
    ["Offset of field: VR_IVRApplications_FnTable::GetApplicationsErrorNameFromEnum"][::core::mem::offset_of!(
        VR_IVRApplications_FnTable,
        GetApplicationsErrorNameFromEnum
    ) - 104usize];
    ["Offset of field: VR_IVRApplications_FnTable::GetApplicationPropertyString"][::core::mem::offset_of!(
        VR_IVRApplications_FnTable,
        GetApplicationPropertyString
    ) - 112usize];
    ["Offset of field: VR_IVRApplications_FnTable::GetApplicationPropertyBool"][::core::mem::offset_of!(
        VR_IVRApplications_FnTable,
        GetApplicationPropertyBool
    ) - 120usize];
    ["Offset of field: VR_IVRApplications_FnTable::GetApplicationPropertyUint64"][::core::mem::offset_of!(
        VR_IVRApplications_FnTable,
        GetApplicationPropertyUint64
    ) - 128usize];
    ["Offset of field: VR_IVRApplications_FnTable::SetApplicationAutoLaunch"]
        [::core::mem::offset_of!(VR_IVRApplications_FnTable, SetApplicationAutoLaunch) - 136usize];
    ["Offset of field: VR_IVRApplications_FnTable::GetApplicationAutoLaunch"]
        [::core::mem::offset_of!(VR_IVRApplications_FnTable, GetApplicationAutoLaunch) - 144usize];
    ["Offset of field: VR_IVRApplications_FnTable::SetDefaultApplicationForMimeType"][::core::mem::offset_of!(
        VR_IVRApplications_FnTable,
        SetDefaultApplicationForMimeType
    ) - 152usize];
    ["Offset of field: VR_IVRApplications_FnTable::GetDefaultApplicationForMimeType"][::core::mem::offset_of!(
        VR_IVRApplications_FnTable,
        GetDefaultApplicationForMimeType
    ) - 160usize];
    ["Offset of field: VR_IVRApplications_FnTable::GetApplicationSupportedMimeTypes"][::core::mem::offset_of!(
        VR_IVRApplications_FnTable,
        GetApplicationSupportedMimeTypes
    ) - 168usize];
    ["Offset of field: VR_IVRApplications_FnTable::GetApplicationsThatSupportMimeType"][::core::mem::offset_of!(
        VR_IVRApplications_FnTable,
        GetApplicationsThatSupportMimeType
    )
        - 176usize];
    ["Offset of field: VR_IVRApplications_FnTable::GetApplicationLaunchArguments"][::core::mem::offset_of!(
        VR_IVRApplications_FnTable,
        GetApplicationLaunchArguments
    ) - 184usize];
    ["Offset of field: VR_IVRApplications_FnTable::GetStartingApplication"]
        [::core::mem::offset_of!(VR_IVRApplications_FnTable, GetStartingApplication) - 192usize];
    ["Offset of field: VR_IVRApplications_FnTable::GetSceneApplicationState"]
        [::core::mem::offset_of!(VR_IVRApplications_FnTable, GetSceneApplicationState) - 200usize];
    ["Offset of field: VR_IVRApplications_FnTable::PerformApplicationPrelaunchCheck"][::core::mem::offset_of!(
        VR_IVRApplications_FnTable,
        PerformApplicationPrelaunchCheck
    ) - 208usize];
    ["Offset of field: VR_IVRApplications_FnTable::GetSceneApplicationStateNameFromEnum"][::core::mem::offset_of!(
        VR_IVRApplications_FnTable,
        GetSceneApplicationStateNameFromEnum
    )
        - 216usize];
    ["Offset of field: VR_IVRApplications_FnTable::LaunchInternalProcess"]
        [::core::mem::offset_of!(VR_IVRApplications_FnTable, LaunchInternalProcess) - 224usize];
    ["Offset of field: VR_IVRApplications_FnTable::RegisterSubprocess"]
        [::core::mem::offset_of!(VR_IVRApplications_FnTable, RegisterSubprocess) - 232usize];
    ["Offset of field: VR_IVRApplications_FnTable::GetCurrentSceneProcessId"]
        [::core::mem::offset_of!(VR_IVRApplications_FnTable, GetCurrentSceneProcessId) - 240usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VR_IVROverlay_FnTable {
    pub FindOverlay: ::core::option::Option<
        unsafe extern "C" fn(
            pchOverlayKey: *mut ::core::ffi::c_char,
            pOverlayHandle: *mut VROverlayHandle_t,
        ) -> EVROverlayError,
    >,
    pub CreateOverlay: ::core::option::Option<
        unsafe extern "C" fn(
            pchOverlayKey: *mut ::core::ffi::c_char,
            pchOverlayName: *mut ::core::ffi::c_char,
            pOverlayHandle: *mut VROverlayHandle_t,
        ) -> EVROverlayError,
    >,
    pub CreateSubviewOverlay: ::core::option::Option<
        unsafe extern "C" fn(
            parentOverlayHandle: VROverlayHandle_t,
            pchSubviewOverlayKey: *mut ::core::ffi::c_char,
            pchSubviewOverlayName: *mut ::core::ffi::c_char,
            pSubviewOverlayHandle: *mut VROverlayHandle_t,
        ) -> EVROverlayError,
    >,
    pub DestroyOverlay: ::core::option::Option<
        unsafe extern "C" fn(ulOverlayHandle: VROverlayHandle_t) -> EVROverlayError,
    >,
    pub GetOverlayKey: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            pchValue: *mut ::core::ffi::c_char,
            unBufferSize: u32,
            pError: *mut EVROverlayError,
        ) -> u32,
    >,
    pub GetOverlayName: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            pchValue: *mut ::core::ffi::c_char,
            unBufferSize: u32,
            pError: *mut EVROverlayError,
        ) -> u32,
    >,
    pub SetOverlayName: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            pchName: *mut ::core::ffi::c_char,
        ) -> EVROverlayError,
    >,
    pub GetOverlayImageData: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            pvBuffer: *mut ::core::ffi::c_void,
            unBufferSize: u32,
            punWidth: *mut u32,
            punHeight: *mut u32,
        ) -> EVROverlayError,
    >,
    pub GetOverlayErrorNameFromEnum: ::core::option::Option<
        unsafe extern "C" fn(error: EVROverlayError) -> *mut ::core::ffi::c_char,
    >,
    pub SetOverlayRenderingPid: ::core::option::Option<
        unsafe extern "C" fn(ulOverlayHandle: VROverlayHandle_t, unPID: u32) -> EVROverlayError,
    >,
    pub GetOverlayRenderingPid:
        ::core::option::Option<unsafe extern "C" fn(ulOverlayHandle: VROverlayHandle_t) -> u32>,
    pub SetOverlayFlag: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            eOverlayFlag: VROverlayFlags,
            bEnabled: bool,
        ) -> EVROverlayError,
    >,
    pub GetOverlayFlag: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            eOverlayFlag: VROverlayFlags,
            pbEnabled: *mut bool,
        ) -> EVROverlayError,
    >,
    pub GetOverlayFlags: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            pFlags: *mut u32,
        ) -> EVROverlayError,
    >,
    pub SetOverlayColor: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            fRed: f32,
            fGreen: f32,
            fBlue: f32,
        ) -> EVROverlayError,
    >,
    pub GetOverlayColor: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            pfRed: *mut f32,
            pfGreen: *mut f32,
            pfBlue: *mut f32,
        ) -> EVROverlayError,
    >,
    pub SetOverlayAlpha: ::core::option::Option<
        unsafe extern "C" fn(ulOverlayHandle: VROverlayHandle_t, fAlpha: f32) -> EVROverlayError,
    >,
    pub GetOverlayAlpha: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            pfAlpha: *mut f32,
        ) -> EVROverlayError,
    >,
    pub SetOverlayTexelAspect: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            fTexelAspect: f32,
        ) -> EVROverlayError,
    >,
    pub GetOverlayTexelAspect: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            pfTexelAspect: *mut f32,
        ) -> EVROverlayError,
    >,
    pub SetOverlaySortOrder: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            unSortOrder: u32,
        ) -> EVROverlayError,
    >,
    pub GetOverlaySortOrder: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            punSortOrder: *mut u32,
        ) -> EVROverlayError,
    >,
    pub SetOverlayWidthInMeters: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            fWidthInMeters: f32,
        ) -> EVROverlayError,
    >,
    pub GetOverlayWidthInMeters: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            pfWidthInMeters: *mut f32,
        ) -> EVROverlayError,
    >,
    pub SetOverlayCurvature: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            fCurvature: f32,
        ) -> EVROverlayError,
    >,
    pub GetOverlayCurvature: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            pfCurvature: *mut f32,
        ) -> EVROverlayError,
    >,
    pub SetOverlayPreCurvePitch: ::core::option::Option<
        unsafe extern "C" fn(ulOverlayHandle: VROverlayHandle_t, fRadians: f32) -> EVROverlayError,
    >,
    pub GetOverlayPreCurvePitch: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            pfRadians: *mut f32,
        ) -> EVROverlayError,
    >,
    pub SetOverlayTextureColorSpace: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            eTextureColorSpace: EColorSpace,
        ) -> EVROverlayError,
    >,
    pub GetOverlayTextureColorSpace: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            peTextureColorSpace: *mut EColorSpace,
        ) -> EVROverlayError,
    >,
    pub SetOverlayTextureBounds: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            pOverlayTextureBounds: *mut VRTextureBounds_t,
        ) -> EVROverlayError,
    >,
    pub GetOverlayTextureBounds: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            pOverlayTextureBounds: *mut VRTextureBounds_t,
        ) -> EVROverlayError,
    >,
    pub GetOverlayTransformType: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            peTransformType: *mut VROverlayTransformType,
        ) -> EVROverlayError,
    >,
    pub SetOverlayTransformAbsolute: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            eTrackingOrigin: ETrackingUniverseOrigin,
            pmatTrackingOriginToOverlayTransform: *mut HmdMatrix34_t,
        ) -> EVROverlayError,
    >,
    pub GetOverlayTransformAbsolute: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            peTrackingOrigin: *mut ETrackingUniverseOrigin,
            pmatTrackingOriginToOverlayTransform: *mut HmdMatrix34_t,
        ) -> EVROverlayError,
    >,
    pub SetOverlayTransformTrackedDeviceRelative: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            unTrackedDevice: TrackedDeviceIndex_t,
            pmatTrackedDeviceToOverlayTransform: *mut HmdMatrix34_t,
        ) -> EVROverlayError,
    >,
    pub GetOverlayTransformTrackedDeviceRelative: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            punTrackedDevice: *mut TrackedDeviceIndex_t,
            pmatTrackedDeviceToOverlayTransform: *mut HmdMatrix34_t,
        ) -> EVROverlayError,
    >,
    pub SetOverlayTransformTrackedDeviceComponent: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            unDeviceIndex: TrackedDeviceIndex_t,
            pchComponentName: *mut ::core::ffi::c_char,
        ) -> EVROverlayError,
    >,
    pub GetOverlayTransformTrackedDeviceComponent: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            punDeviceIndex: *mut TrackedDeviceIndex_t,
            pchComponentName: *mut ::core::ffi::c_char,
            unComponentNameSize: u32,
        ) -> EVROverlayError,
    >,
    pub SetOverlayTransformCursor: ::core::option::Option<
        unsafe extern "C" fn(
            ulCursorOverlayHandle: VROverlayHandle_t,
            pvHotspot: *mut HmdVector2_t,
        ) -> EVROverlayError,
    >,
    pub GetOverlayTransformCursor: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            pvHotspot: *mut HmdVector2_t,
        ) -> EVROverlayError,
    >,
    pub SetOverlayTransformProjection: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            eTrackingOrigin: ETrackingUniverseOrigin,
            pmatTrackingOriginToOverlayTransform: *mut HmdMatrix34_t,
            pProjection: *mut VROverlayProjection_t,
            eEye: EVREye,
        ) -> EVROverlayError,
    >,
    pub SetSubviewPosition: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            fX: f32,
            fY: f32,
        ) -> EVROverlayError,
    >,
    pub ShowOverlay: ::core::option::Option<
        unsafe extern "C" fn(ulOverlayHandle: VROverlayHandle_t) -> EVROverlayError,
    >,
    pub HideOverlay: ::core::option::Option<
        unsafe extern "C" fn(ulOverlayHandle: VROverlayHandle_t) -> EVROverlayError,
    >,
    pub IsOverlayVisible:
        ::core::option::Option<unsafe extern "C" fn(ulOverlayHandle: VROverlayHandle_t) -> bool>,
    pub GetTransformForOverlayCoordinates: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            eTrackingOrigin: ETrackingUniverseOrigin,
            coordinatesInOverlay: HmdVector2_t,
            pmatTransform: *mut HmdMatrix34_t,
        ) -> EVROverlayError,
    >,
    pub WaitFrameSync:
        ::core::option::Option<unsafe extern "C" fn(nTimeoutMs: u32) -> EVROverlayError>,
    pub PollNextOverlayEvent: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            pEvent: *mut VREvent_t,
            uncbVREvent: u32,
        ) -> bool,
    >,
    pub GetOverlayInputMethod: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            peInputMethod: *mut VROverlayInputMethod,
        ) -> EVROverlayError,
    >,
    pub SetOverlayInputMethod: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            eInputMethod: VROverlayInputMethod,
        ) -> EVROverlayError,
    >,
    pub GetOverlayMouseScale: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            pvecMouseScale: *mut HmdVector2_t,
        ) -> EVROverlayError,
    >,
    pub SetOverlayMouseScale: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            pvecMouseScale: *mut HmdVector2_t,
        ) -> EVROverlayError,
    >,
    pub ComputeOverlayIntersection: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            pParams: *mut VROverlayIntersectionParams_t,
            pResults: *mut VROverlayIntersectionResults_t,
        ) -> bool,
    >,
    pub IsHoverTargetOverlay:
        ::core::option::Option<unsafe extern "C" fn(ulOverlayHandle: VROverlayHandle_t) -> bool>,
    pub SetOverlayIntersectionMask: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            pMaskPrimitives: *mut VROverlayIntersectionMaskPrimitive_t,
            unNumMaskPrimitives: u32,
            unPrimitiveSize: u32,
        ) -> EVROverlayError,
    >,
    pub TriggerLaserMouseHapticVibration: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            fDurationSeconds: f32,
            fFrequency: f32,
            fAmplitude: f32,
        ) -> EVROverlayError,
    >,
    pub SetOverlayCursor: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            ulCursorHandle: VROverlayHandle_t,
        ) -> EVROverlayError,
    >,
    pub SetOverlayCursorPositionOverride: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            pvCursor: *mut HmdVector2_t,
        ) -> EVROverlayError,
    >,
    pub ClearOverlayCursorPositionOverride: ::core::option::Option<
        unsafe extern "C" fn(ulOverlayHandle: VROverlayHandle_t) -> EVROverlayError,
    >,
    pub SetOverlayTexture: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            pTexture: *mut Texture_t,
        ) -> EVROverlayError,
    >,
    pub ClearOverlayTexture: ::core::option::Option<
        unsafe extern "C" fn(ulOverlayHandle: VROverlayHandle_t) -> EVROverlayError,
    >,
    pub SetOverlayRaw: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            pvBuffer: *mut ::core::ffi::c_void,
            unWidth: u32,
            unHeight: u32,
            unBytesPerPixel: u32,
        ) -> EVROverlayError,
    >,
    pub SetOverlayFromFile: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            pchFilePath: *mut ::core::ffi::c_char,
        ) -> EVROverlayError,
    >,
    pub GetOverlayTexture: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            pNativeTextureHandle: *mut *mut ::core::ffi::c_void,
            pNativeTextureRef: *mut ::core::ffi::c_void,
            pWidth: *mut u32,
            pHeight: *mut u32,
            pNativeFormat: *mut u32,
            pAPIType: *mut ETextureType,
            pColorSpace: *mut EColorSpace,
            pTextureBounds: *mut VRTextureBounds_t,
        ) -> EVROverlayError,
    >,
    pub ReleaseNativeOverlayHandle: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            pNativeTextureHandle: *mut ::core::ffi::c_void,
        ) -> EVROverlayError,
    >,
    pub GetOverlayTextureSize: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            pWidth: *mut u32,
            pHeight: *mut u32,
        ) -> EVROverlayError,
    >,
    pub CreateDashboardOverlay: ::core::option::Option<
        unsafe extern "C" fn(
            pchOverlayKey: *mut ::core::ffi::c_char,
            pchOverlayFriendlyName: *mut ::core::ffi::c_char,
            pMainHandle: *mut VROverlayHandle_t,
            pThumbnailHandle: *mut VROverlayHandle_t,
        ) -> EVROverlayError,
    >,
    pub IsDashboardVisible: ::core::option::Option<unsafe extern "C" fn() -> bool>,
    pub IsActiveDashboardOverlay:
        ::core::option::Option<unsafe extern "C" fn(ulOverlayHandle: VROverlayHandle_t) -> bool>,
    pub SetDashboardOverlaySceneProcess: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            unProcessId: u32,
        ) -> EVROverlayError,
    >,
    pub GetDashboardOverlaySceneProcess: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            punProcessId: *mut u32,
        ) -> EVROverlayError,
    >,
    pub ShowDashboard:
        ::core::option::Option<unsafe extern "C" fn(pchOverlayToShow: *mut ::core::ffi::c_char)>,
    pub GetPrimaryDashboardDevice:
        ::core::option::Option<unsafe extern "C" fn() -> TrackedDeviceIndex_t>,
    pub ShowKeyboard: ::core::option::Option<
        unsafe extern "C" fn(
            eInputMode: EGamepadTextInputMode,
            eLineInputMode: EGamepadTextInputLineMode,
            unFlags: u32,
            pchDescription: *mut ::core::ffi::c_char,
            unCharMax: u32,
            pchExistingText: *mut ::core::ffi::c_char,
            uUserValue: u64,
        ) -> EVROverlayError,
    >,
    pub ShowKeyboardForOverlay: ::core::option::Option<
        unsafe extern "C" fn(
            ulOverlayHandle: VROverlayHandle_t,
            eInputMode: EGamepadTextInputMode,
            eLineInputMode: EGamepadTextInputLineMode,
            unFlags: u32,
            pchDescription: *mut ::core::ffi::c_char,
            unCharMax: u32,
            pchExistingText: *mut ::core::ffi::c_char,
            uUserValue: u64,
        ) -> EVROverlayError,
    >,
    pub GetKeyboardText: ::core::option::Option<
        unsafe extern "C" fn(pchText: *mut ::core::ffi::c_char, cchText: u32) -> u32,
    >,
    pub HideKeyboard: ::core::option::Option<unsafe extern "C" fn()>,
    pub SetKeyboardTransformAbsolute: ::core::option::Option<
        unsafe extern "C" fn(
            eTrackingOrigin: ETrackingUniverseOrigin,
            pmatTrackingOriginToKeyboardTransform: *mut HmdMatrix34_t,
        ),
    >,
    pub SetKeyboardPositionForOverlay: ::core::option::Option<
        unsafe extern "C" fn(ulOverlayHandle: VROverlayHandle_t, avoidRect: HmdRect2_t),
    >,
    pub ShowMessageOverlay: ::core::option::Option<
        unsafe extern "C" fn(
            pchText: *mut ::core::ffi::c_char,
            pchCaption: *mut ::core::ffi::c_char,
            pchButton0Text: *mut ::core::ffi::c_char,
            pchButton1Text: *mut ::core::ffi::c_char,
            pchButton2Text: *mut ::core::ffi::c_char,
            pchButton3Text: *mut ::core::ffi::c_char,
        ) -> VRMessageOverlayResponse,
    >,
    pub CloseMessageOverlay: ::core::option::Option<unsafe extern "C" fn()>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VR_IVROverlay_FnTable"][::core::mem::size_of::<VR_IVROverlay_FnTable>() - 656usize];
    ["Alignment of VR_IVROverlay_FnTable"]
        [::core::mem::align_of::<VR_IVROverlay_FnTable>() - 8usize];
    ["Offset of field: VR_IVROverlay_FnTable::FindOverlay"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, FindOverlay) - 0usize];
    ["Offset of field: VR_IVROverlay_FnTable::CreateOverlay"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, CreateOverlay) - 8usize];
    ["Offset of field: VR_IVROverlay_FnTable::CreateSubviewOverlay"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, CreateSubviewOverlay) - 16usize];
    ["Offset of field: VR_IVROverlay_FnTable::DestroyOverlay"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, DestroyOverlay) - 24usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetOverlayKey"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayKey) - 32usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetOverlayName"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayName) - 40usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetOverlayName"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayName) - 48usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetOverlayImageData"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayImageData) - 56usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetOverlayErrorNameFromEnum"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayErrorNameFromEnum) - 64usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetOverlayRenderingPid"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayRenderingPid) - 72usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetOverlayRenderingPid"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayRenderingPid) - 80usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetOverlayFlag"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayFlag) - 88usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetOverlayFlag"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayFlag) - 96usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetOverlayFlags"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayFlags) - 104usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetOverlayColor"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayColor) - 112usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetOverlayColor"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayColor) - 120usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetOverlayAlpha"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayAlpha) - 128usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetOverlayAlpha"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayAlpha) - 136usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetOverlayTexelAspect"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayTexelAspect) - 144usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetOverlayTexelAspect"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayTexelAspect) - 152usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetOverlaySortOrder"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlaySortOrder) - 160usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetOverlaySortOrder"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlaySortOrder) - 168usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetOverlayWidthInMeters"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayWidthInMeters) - 176usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetOverlayWidthInMeters"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayWidthInMeters) - 184usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetOverlayCurvature"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayCurvature) - 192usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetOverlayCurvature"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayCurvature) - 200usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetOverlayPreCurvePitch"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayPreCurvePitch) - 208usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetOverlayPreCurvePitch"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayPreCurvePitch) - 216usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetOverlayTextureColorSpace"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayTextureColorSpace) - 224usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetOverlayTextureColorSpace"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayTextureColorSpace) - 232usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetOverlayTextureBounds"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayTextureBounds) - 240usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetOverlayTextureBounds"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayTextureBounds) - 248usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetOverlayTransformType"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayTransformType) - 256usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetOverlayTransformAbsolute"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayTransformAbsolute) - 264usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetOverlayTransformAbsolute"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayTransformAbsolute) - 272usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetOverlayTransformTrackedDeviceRelative"][::core::mem::offset_of!(
        VR_IVROverlay_FnTable,
        SetOverlayTransformTrackedDeviceRelative
    )
        - 280usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetOverlayTransformTrackedDeviceRelative"][::core::mem::offset_of!(
        VR_IVROverlay_FnTable,
        GetOverlayTransformTrackedDeviceRelative
    )
        - 288usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetOverlayTransformTrackedDeviceComponent"][::core::mem::offset_of!(
        VR_IVROverlay_FnTable,
        SetOverlayTransformTrackedDeviceComponent
    )
        - 296usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetOverlayTransformTrackedDeviceComponent"][::core::mem::offset_of!(
        VR_IVROverlay_FnTable,
        GetOverlayTransformTrackedDeviceComponent
    )
        - 304usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetOverlayTransformCursor"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayTransformCursor) - 312usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetOverlayTransformCursor"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayTransformCursor) - 320usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetOverlayTransformProjection"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayTransformProjection) - 328usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetSubviewPosition"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, SetSubviewPosition) - 336usize];
    ["Offset of field: VR_IVROverlay_FnTable::ShowOverlay"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, ShowOverlay) - 344usize];
    ["Offset of field: VR_IVROverlay_FnTable::HideOverlay"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, HideOverlay) - 352usize];
    ["Offset of field: VR_IVROverlay_FnTable::IsOverlayVisible"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, IsOverlayVisible) - 360usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetTransformForOverlayCoordinates"][::core::mem::offset_of!(
        VR_IVROverlay_FnTable,
        GetTransformForOverlayCoordinates
    ) - 368usize];
    ["Offset of field: VR_IVROverlay_FnTable::WaitFrameSync"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, WaitFrameSync) - 376usize];
    ["Offset of field: VR_IVROverlay_FnTable::PollNextOverlayEvent"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, PollNextOverlayEvent) - 384usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetOverlayInputMethod"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayInputMethod) - 392usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetOverlayInputMethod"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayInputMethod) - 400usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetOverlayMouseScale"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayMouseScale) - 408usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetOverlayMouseScale"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayMouseScale) - 416usize];
    ["Offset of field: VR_IVROverlay_FnTable::ComputeOverlayIntersection"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, ComputeOverlayIntersection) - 424usize];
    ["Offset of field: VR_IVROverlay_FnTable::IsHoverTargetOverlay"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, IsHoverTargetOverlay) - 432usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetOverlayIntersectionMask"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayIntersectionMask) - 440usize];
    ["Offset of field: VR_IVROverlay_FnTable::TriggerLaserMouseHapticVibration"][::core::mem::offset_of!(
        VR_IVROverlay_FnTable,
        TriggerLaserMouseHapticVibration
    ) - 448usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetOverlayCursor"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayCursor) - 456usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetOverlayCursorPositionOverride"][::core::mem::offset_of!(
        VR_IVROverlay_FnTable,
        SetOverlayCursorPositionOverride
    ) - 464usize];
    ["Offset of field: VR_IVROverlay_FnTable::ClearOverlayCursorPositionOverride"][::core::mem::offset_of!(
        VR_IVROverlay_FnTable,
        ClearOverlayCursorPositionOverride
    ) - 472usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetOverlayTexture"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayTexture) - 480usize];
    ["Offset of field: VR_IVROverlay_FnTable::ClearOverlayTexture"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, ClearOverlayTexture) - 488usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetOverlayRaw"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayRaw) - 496usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetOverlayFromFile"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayFromFile) - 504usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetOverlayTexture"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayTexture) - 512usize];
    ["Offset of field: VR_IVROverlay_FnTable::ReleaseNativeOverlayHandle"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, ReleaseNativeOverlayHandle) - 520usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetOverlayTextureSize"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayTextureSize) - 528usize];
    ["Offset of field: VR_IVROverlay_FnTable::CreateDashboardOverlay"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, CreateDashboardOverlay) - 536usize];
    ["Offset of field: VR_IVROverlay_FnTable::IsDashboardVisible"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, IsDashboardVisible) - 544usize];
    ["Offset of field: VR_IVROverlay_FnTable::IsActiveDashboardOverlay"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, IsActiveDashboardOverlay) - 552usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetDashboardOverlaySceneProcess"][::core::mem::offset_of!(
        VR_IVROverlay_FnTable,
        SetDashboardOverlaySceneProcess
    ) - 560usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetDashboardOverlaySceneProcess"][::core::mem::offset_of!(
        VR_IVROverlay_FnTable,
        GetDashboardOverlaySceneProcess
    ) - 568usize];
    ["Offset of field: VR_IVROverlay_FnTable::ShowDashboard"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, ShowDashboard) - 576usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetPrimaryDashboardDevice"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, GetPrimaryDashboardDevice) - 584usize];
    ["Offset of field: VR_IVROverlay_FnTable::ShowKeyboard"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, ShowKeyboard) - 592usize];
    ["Offset of field: VR_IVROverlay_FnTable::ShowKeyboardForOverlay"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, ShowKeyboardForOverlay) - 600usize];
    ["Offset of field: VR_IVROverlay_FnTable::GetKeyboardText"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, GetKeyboardText) - 608usize];
    ["Offset of field: VR_IVROverlay_FnTable::HideKeyboard"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, HideKeyboard) - 616usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetKeyboardTransformAbsolute"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, SetKeyboardTransformAbsolute) - 624usize];
    ["Offset of field: VR_IVROverlay_FnTable::SetKeyboardPositionForOverlay"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, SetKeyboardPositionForOverlay) - 632usize];
    ["Offset of field: VR_IVROverlay_FnTable::ShowMessageOverlay"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, ShowMessageOverlay) - 640usize];
    ["Offset of field: VR_IVROverlay_FnTable::CloseMessageOverlay"]
        [::core::mem::offset_of!(VR_IVROverlay_FnTable, CloseMessageOverlay) - 648usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VR_IVRSettings_FnTable {
    pub GetSettingsErrorNameFromEnum: ::core::option::Option<
        unsafe extern "C" fn(eError: EVRSettingsError) -> *mut ::core::ffi::c_char,
    >,
    pub SetBool: ::core::option::Option<
        unsafe extern "C" fn(
            pchSection: *mut ::core::ffi::c_char,
            pchSettingsKey: *mut ::core::ffi::c_char,
            bValue: bool,
            peError: *mut EVRSettingsError,
        ),
    >,
    pub SetInt32: ::core::option::Option<
        unsafe extern "C" fn(
            pchSection: *mut ::core::ffi::c_char,
            pchSettingsKey: *mut ::core::ffi::c_char,
            nValue: i32,
            peError: *mut EVRSettingsError,
        ),
    >,
    pub SetFloat: ::core::option::Option<
        unsafe extern "C" fn(
            pchSection: *mut ::core::ffi::c_char,
            pchSettingsKey: *mut ::core::ffi::c_char,
            flValue: f32,
            peError: *mut EVRSettingsError,
        ),
    >,
    pub SetString: ::core::option::Option<
        unsafe extern "C" fn(
            pchSection: *mut ::core::ffi::c_char,
            pchSettingsKey: *mut ::core::ffi::c_char,
            pchValue: *mut ::core::ffi::c_char,
            peError: *mut EVRSettingsError,
        ),
    >,
    pub GetBool: ::core::option::Option<
        unsafe extern "C" fn(
            pchSection: *mut ::core::ffi::c_char,
            pchSettingsKey: *mut ::core::ffi::c_char,
            peError: *mut EVRSettingsError,
        ) -> bool,
    >,
    pub GetInt32: ::core::option::Option<
        unsafe extern "C" fn(
            pchSection: *mut ::core::ffi::c_char,
            pchSettingsKey: *mut ::core::ffi::c_char,
            peError: *mut EVRSettingsError,
        ) -> i32,
    >,
    pub GetFloat: ::core::option::Option<
        unsafe extern "C" fn(
            pchSection: *mut ::core::ffi::c_char,
            pchSettingsKey: *mut ::core::ffi::c_char,
            peError: *mut EVRSettingsError,
        ) -> f32,
    >,
    pub GetString: ::core::option::Option<
        unsafe extern "C" fn(
            pchSection: *mut ::core::ffi::c_char,
            pchSettingsKey: *mut ::core::ffi::c_char,
            pchValue: *mut ::core::ffi::c_char,
            unValueLen: u32,
            peError: *mut EVRSettingsError,
        ),
    >,
    pub RemoveSection: ::core::option::Option<
        unsafe extern "C" fn(pchSection: *mut ::core::ffi::c_char, peError: *mut EVRSettingsError),
    >,
    pub RemoveKeyInSection: ::core::option::Option<
        unsafe extern "C" fn(
            pchSection: *mut ::core::ffi::c_char,
            pchSettingsKey: *mut ::core::ffi::c_char,
            peError: *mut EVRSettingsError,
        ),
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VR_IVRSettings_FnTable"][::core::mem::size_of::<VR_IVRSettings_FnTable>() - 88usize];
    ["Alignment of VR_IVRSettings_FnTable"]
        [::core::mem::align_of::<VR_IVRSettings_FnTable>() - 8usize];
    ["Offset of field: VR_IVRSettings_FnTable::GetSettingsErrorNameFromEnum"]
        [::core::mem::offset_of!(VR_IVRSettings_FnTable, GetSettingsErrorNameFromEnum) - 0usize];
    ["Offset of field: VR_IVRSettings_FnTable::SetBool"]
        [::core::mem::offset_of!(VR_IVRSettings_FnTable, SetBool) - 8usize];
    ["Offset of field: VR_IVRSettings_FnTable::SetInt32"]
        [::core::mem::offset_of!(VR_IVRSettings_FnTable, SetInt32) - 16usize];
    ["Offset of field: VR_IVRSettings_FnTable::SetFloat"]
        [::core::mem::offset_of!(VR_IVRSettings_FnTable, SetFloat) - 24usize];
    ["Offset of field: VR_IVRSettings_FnTable::SetString"]
        [::core::mem::offset_of!(VR_IVRSettings_FnTable, SetString) - 32usize];
    ["Offset of field: VR_IVRSettings_FnTable::GetBool"]
        [::core::mem::offset_of!(VR_IVRSettings_FnTable, GetBool) - 40usize];
    ["Offset of field: VR_IVRSettings_FnTable::GetInt32"]
        [::core::mem::offset_of!(VR_IVRSettings_FnTable, GetInt32) - 48usize];
    ["Offset of field: VR_IVRSettings_FnTable::GetFloat"]
        [::core::mem::offset_of!(VR_IVRSettings_FnTable, GetFloat) - 56usize];
    ["Offset of field: VR_IVRSettings_FnTable::GetString"]
        [::core::mem::offset_of!(VR_IVRSettings_FnTable, GetString) - 64usize];
    ["Offset of field: VR_IVRSettings_FnTable::RemoveSection"]
        [::core::mem::offset_of!(VR_IVRSettings_FnTable, RemoveSection) - 72usize];
    ["Offset of field: VR_IVRSettings_FnTable::RemoveKeyInSection"]
        [::core::mem::offset_of!(VR_IVRSettings_FnTable, RemoveKeyInSection) - 80usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VR_IVRInput_FnTable {
    pub SetActionManifestPath: ::core::option::Option<
        unsafe extern "C" fn(pchActionManifestPath: *mut ::core::ffi::c_char) -> EVRInputError,
    >,
    pub GetActionSetHandle: ::core::option::Option<
        unsafe extern "C" fn(
            pchActionSetName: *mut ::core::ffi::c_char,
            pHandle: *mut VRActionSetHandle_t,
        ) -> EVRInputError,
    >,
    pub GetActionHandle: ::core::option::Option<
        unsafe extern "C" fn(
            pchActionName: *mut ::core::ffi::c_char,
            pHandle: *mut VRActionHandle_t,
        ) -> EVRInputError,
    >,
    pub GetInputSourceHandle: ::core::option::Option<
        unsafe extern "C" fn(
            pchInputSourcePath: *mut ::core::ffi::c_char,
            pHandle: *mut VRInputValueHandle_t,
        ) -> EVRInputError,
    >,
    pub UpdateActionState: ::core::option::Option<
        unsafe extern "C" fn(
            pSets: *mut VRActiveActionSet_t,
            unSizeOfVRSelectedActionSet_t: u32,
            unSetCount: u32,
        ) -> EVRInputError,
    >,
    pub GetDigitalActionData: ::core::option::Option<
        unsafe extern "C" fn(
            action: VRActionHandle_t,
            pActionData: *mut InputDigitalActionData_t,
            unActionDataSize: u32,
            ulRestrictToDevice: VRInputValueHandle_t,
        ) -> EVRInputError,
    >,
    pub GetAnalogActionData: ::core::option::Option<
        unsafe extern "C" fn(
            action: VRActionHandle_t,
            pActionData: *mut InputAnalogActionData_t,
            unActionDataSize: u32,
            ulRestrictToDevice: VRInputValueHandle_t,
        ) -> EVRInputError,
    >,
    pub GetPoseActionDataRelativeToNow: ::core::option::Option<
        unsafe extern "C" fn(
            action: VRActionHandle_t,
            eOrigin: ETrackingUniverseOrigin,
            fPredictedSecondsFromNow: f32,
            pActionData: *mut InputPoseActionData_t,
            unActionDataSize: u32,
            ulRestrictToDevice: VRInputValueHandle_t,
        ) -> EVRInputError,
    >,
    pub GetPoseActionDataForNextFrame: ::core::option::Option<
        unsafe extern "C" fn(
            action: VRActionHandle_t,
            eOrigin: ETrackingUniverseOrigin,
            pActionData: *mut InputPoseActionData_t,
            unActionDataSize: u32,
            ulRestrictToDevice: VRInputValueHandle_t,
        ) -> EVRInputError,
    >,
    pub GetSkeletalActionData: ::core::option::Option<
        unsafe extern "C" fn(
            action: VRActionHandle_t,
            pActionData: *mut InputSkeletalActionData_t,
            unActionDataSize: u32,
        ) -> EVRInputError,
    >,
    pub GetDominantHand: ::core::option::Option<
        unsafe extern "C" fn(peDominantHand: *mut ETrackedControllerRole) -> EVRInputError,
    >,
    pub SetDominantHand: ::core::option::Option<
        unsafe extern "C" fn(eDominantHand: ETrackedControllerRole) -> EVRInputError,
    >,
    pub GetEyeTrackingDataRelativeToNow: ::core::option::Option<
        unsafe extern "C" fn(
            action: VRActionHandle_t,
            eOrigin: ETrackingUniverseOrigin,
            fPredictedSecondsFromNow: f32,
            pEyeTrackingData: *mut VREyeTrackingData_t,
            ulEyeTrackingDataSize: u32,
        ) -> EVRInputError,
    >,
    pub GetEyeTrackingDataForNextFrame: ::core::option::Option<
        unsafe extern "C" fn(
            action: VRActionHandle_t,
            eOrigin: ETrackingUniverseOrigin,
            pEyeTrackingData: *mut VREyeTrackingData_t,
            ulEyeTrackingDataSize: u32,
        ) -> EVRInputError,
    >,
    pub GetBoneCount: ::core::option::Option<
        unsafe extern "C" fn(action: VRActionHandle_t, pBoneCount: *mut u32) -> EVRInputError,
    >,
    pub GetBoneHierarchy: ::core::option::Option<
        unsafe extern "C" fn(
            action: VRActionHandle_t,
            pParentIndices: *mut BoneIndex_t,
            unIndexArayCount: u32,
        ) -> EVRInputError,
    >,
    pub GetBoneName: ::core::option::Option<
        unsafe extern "C" fn(
            action: VRActionHandle_t,
            nBoneIndex: BoneIndex_t,
            pchBoneName: *mut ::core::ffi::c_char,
            unNameBufferSize: u32,
        ) -> EVRInputError,
    >,
    pub GetSkeletalReferenceTransforms: ::core::option::Option<
        unsafe extern "C" fn(
            action: VRActionHandle_t,
            eTransformSpace: EVRSkeletalTransformSpace,
            eReferencePose: EVRSkeletalReferencePose,
            pTransformArray: *mut VRBoneTransform_t,
            unTransformArrayCount: u32,
        ) -> EVRInputError,
    >,
    pub GetSkeletalTrackingLevel: ::core::option::Option<
        unsafe extern "C" fn(
            action: VRActionHandle_t,
            pSkeletalTrackingLevel: *mut EVRSkeletalTrackingLevel,
        ) -> EVRInputError,
    >,
    pub GetSkeletalBoneData: ::core::option::Option<
        unsafe extern "C" fn(
            action: VRActionHandle_t,
            eTransformSpace: EVRSkeletalTransformSpace,
            eMotionRange: EVRSkeletalMotionRange,
            pTransformArray: *mut VRBoneTransform_t,
            unTransformArrayCount: u32,
        ) -> EVRInputError,
    >,
    pub GetSkeletalSummaryData: ::core::option::Option<
        unsafe extern "C" fn(
            action: VRActionHandle_t,
            eSummaryType: EVRSummaryType,
            pSkeletalSummaryData: *mut VRSkeletalSummaryData_t,
        ) -> EVRInputError,
    >,
    pub GetSkeletalBoneDataCompressed: ::core::option::Option<
        unsafe extern "C" fn(
            action: VRActionHandle_t,
            eMotionRange: EVRSkeletalMotionRange,
            pvCompressedData: *mut ::core::ffi::c_void,
            unCompressedSize: u32,
            punRequiredCompressedSize: *mut u32,
        ) -> EVRInputError,
    >,
    pub DecompressSkeletalBoneData: ::core::option::Option<
        unsafe extern "C" fn(
            pvCompressedBuffer: *mut ::core::ffi::c_void,
            unCompressedBufferSize: u32,
            eTransformSpace: EVRSkeletalTransformSpace,
            pTransformArray: *mut VRBoneTransform_t,
            unTransformArrayCount: u32,
        ) -> EVRInputError,
    >,
    pub TriggerHapticVibrationAction: ::core::option::Option<
        unsafe extern "C" fn(
            action: VRActionHandle_t,
            fStartSecondsFromNow: f32,
            fDurationSeconds: f32,
            fFrequency: f32,
            fAmplitude: f32,
            ulRestrictToDevice: VRInputValueHandle_t,
        ) -> EVRInputError,
    >,
    pub GetActionOrigins: ::core::option::Option<
        unsafe extern "C" fn(
            actionSetHandle: VRActionSetHandle_t,
            digitalActionHandle: VRActionHandle_t,
            originsOut: *mut VRInputValueHandle_t,
            originOutCount: u32,
        ) -> EVRInputError,
    >,
    pub GetOriginLocalizedName: ::core::option::Option<
        unsafe extern "C" fn(
            origin: VRInputValueHandle_t,
            pchNameArray: *mut ::core::ffi::c_char,
            unNameArraySize: u32,
            unStringSectionsToInclude: i32,
        ) -> EVRInputError,
    >,
    pub GetOriginTrackedDeviceInfo: ::core::option::Option<
        unsafe extern "C" fn(
            origin: VRInputValueHandle_t,
            pOriginInfo: *mut InputOriginInfo_t,
            unOriginInfoSize: u32,
        ) -> EVRInputError,
    >,
    pub GetActionBindingInfo: ::core::option::Option<
        unsafe extern "C" fn(
            action: VRActionHandle_t,
            pOriginInfo: *mut InputBindingInfo_t,
            unBindingInfoSize: u32,
            unBindingInfoCount: u32,
            punReturnedBindingInfoCount: *mut u32,
        ) -> EVRInputError,
    >,
    pub ShowActionOrigins: ::core::option::Option<
        unsafe extern "C" fn(
            actionSetHandle: VRActionSetHandle_t,
            ulActionHandle: VRActionHandle_t,
        ) -> EVRInputError,
    >,
    pub ShowBindingsForActionSet: ::core::option::Option<
        unsafe extern "C" fn(
            pSets: *mut VRActiveActionSet_t,
            unSizeOfVRSelectedActionSet_t: u32,
            unSetCount: u32,
            originToHighlight: VRInputValueHandle_t,
        ) -> EVRInputError,
    >,
    pub GetComponentStateForBinding: ::core::option::Option<
        unsafe extern "C" fn(
            pchRenderModelName: *mut ::core::ffi::c_char,
            pchComponentName: *mut ::core::ffi::c_char,
            pOriginInfo: *mut InputBindingInfo_t,
            unBindingInfoSize: u32,
            unBindingInfoCount: u32,
            pComponentState: *mut RenderModel_ComponentState_t,
        ) -> EVRInputError,
    >,
    pub IsUsingLegacyInput: ::core::option::Option<unsafe extern "C" fn() -> bool>,
    pub OpenBindingUI: ::core::option::Option<
        unsafe extern "C" fn(
            pchAppKey: *mut ::core::ffi::c_char,
            ulActionSetHandle: VRActionSetHandle_t,
            ulDeviceHandle: VRInputValueHandle_t,
            bShowOnDesktop: bool,
        ) -> EVRInputError,
    >,
    pub GetBindingVariant: ::core::option::Option<
        unsafe extern "C" fn(
            ulDevicePath: VRInputValueHandle_t,
            pchVariantArray: *mut ::core::ffi::c_char,
            unVariantArraySize: u32,
        ) -> EVRInputError,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VR_IVRInput_FnTable"][::core::mem::size_of::<VR_IVRInput_FnTable>() - 272usize];
    ["Alignment of VR_IVRInput_FnTable"][::core::mem::align_of::<VR_IVRInput_FnTable>() - 8usize];
    ["Offset of field: VR_IVRInput_FnTable::SetActionManifestPath"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, SetActionManifestPath) - 0usize];
    ["Offset of field: VR_IVRInput_FnTable::GetActionSetHandle"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, GetActionSetHandle) - 8usize];
    ["Offset of field: VR_IVRInput_FnTable::GetActionHandle"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, GetActionHandle) - 16usize];
    ["Offset of field: VR_IVRInput_FnTable::GetInputSourceHandle"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, GetInputSourceHandle) - 24usize];
    ["Offset of field: VR_IVRInput_FnTable::UpdateActionState"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, UpdateActionState) - 32usize];
    ["Offset of field: VR_IVRInput_FnTable::GetDigitalActionData"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, GetDigitalActionData) - 40usize];
    ["Offset of field: VR_IVRInput_FnTable::GetAnalogActionData"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, GetAnalogActionData) - 48usize];
    ["Offset of field: VR_IVRInput_FnTable::GetPoseActionDataRelativeToNow"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, GetPoseActionDataRelativeToNow) - 56usize];
    ["Offset of field: VR_IVRInput_FnTable::GetPoseActionDataForNextFrame"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, GetPoseActionDataForNextFrame) - 64usize];
    ["Offset of field: VR_IVRInput_FnTable::GetSkeletalActionData"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, GetSkeletalActionData) - 72usize];
    ["Offset of field: VR_IVRInput_FnTable::GetDominantHand"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, GetDominantHand) - 80usize];
    ["Offset of field: VR_IVRInput_FnTable::SetDominantHand"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, SetDominantHand) - 88usize];
    ["Offset of field: VR_IVRInput_FnTable::GetEyeTrackingDataRelativeToNow"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, GetEyeTrackingDataRelativeToNow) - 96usize];
    ["Offset of field: VR_IVRInput_FnTable::GetEyeTrackingDataForNextFrame"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, GetEyeTrackingDataForNextFrame) - 104usize];
    ["Offset of field: VR_IVRInput_FnTable::GetBoneCount"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, GetBoneCount) - 112usize];
    ["Offset of field: VR_IVRInput_FnTable::GetBoneHierarchy"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, GetBoneHierarchy) - 120usize];
    ["Offset of field: VR_IVRInput_FnTable::GetBoneName"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, GetBoneName) - 128usize];
    ["Offset of field: VR_IVRInput_FnTable::GetSkeletalReferenceTransforms"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, GetSkeletalReferenceTransforms) - 136usize];
    ["Offset of field: VR_IVRInput_FnTable::GetSkeletalTrackingLevel"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, GetSkeletalTrackingLevel) - 144usize];
    ["Offset of field: VR_IVRInput_FnTable::GetSkeletalBoneData"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, GetSkeletalBoneData) - 152usize];
    ["Offset of field: VR_IVRInput_FnTable::GetSkeletalSummaryData"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, GetSkeletalSummaryData) - 160usize];
    ["Offset of field: VR_IVRInput_FnTable::GetSkeletalBoneDataCompressed"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, GetSkeletalBoneDataCompressed) - 168usize];
    ["Offset of field: VR_IVRInput_FnTable::DecompressSkeletalBoneData"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, DecompressSkeletalBoneData) - 176usize];
    ["Offset of field: VR_IVRInput_FnTable::TriggerHapticVibrationAction"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, TriggerHapticVibrationAction) - 184usize];
    ["Offset of field: VR_IVRInput_FnTable::GetActionOrigins"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, GetActionOrigins) - 192usize];
    ["Offset of field: VR_IVRInput_FnTable::GetOriginLocalizedName"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, GetOriginLocalizedName) - 200usize];
    ["Offset of field: VR_IVRInput_FnTable::GetOriginTrackedDeviceInfo"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, GetOriginTrackedDeviceInfo) - 208usize];
    ["Offset of field: VR_IVRInput_FnTable::GetActionBindingInfo"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, GetActionBindingInfo) - 216usize];
    ["Offset of field: VR_IVRInput_FnTable::ShowActionOrigins"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, ShowActionOrigins) - 224usize];
    ["Offset of field: VR_IVRInput_FnTable::ShowBindingsForActionSet"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, ShowBindingsForActionSet) - 232usize];
    ["Offset of field: VR_IVRInput_FnTable::GetComponentStateForBinding"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, GetComponentStateForBinding) - 240usize];
    ["Offset of field: VR_IVRInput_FnTable::IsUsingLegacyInput"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, IsUsingLegacyInput) - 248usize];
    ["Offset of field: VR_IVRInput_FnTable::OpenBindingUI"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, OpenBindingUI) - 256usize];
    ["Offset of field: VR_IVRInput_FnTable::GetBindingVariant"]
        [::core::mem::offset_of!(VR_IVRInput_FnTable, GetBindingVariant) - 264usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkInstance_T {
    pub _address: u8,
}
