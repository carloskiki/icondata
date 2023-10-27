//! This crate provides a collection of icons in the form of SVG data
//! and an enum to select them.
//!
//! ## Usage
//!
//! Every icon is shipped as its own feature; the enum variant and their corresponding feature name are
//! identical.
//!

/// This enum provides every icon as a variant.
/// It implements [`Into<icondata_core::IconData>`][icondata_core::IconData].
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "strum", derive(strum::EnumIter, strum::EnumVariantNames))]
pub enum ChIcon {
    #[cfg(ChAnchor)]
    ChAnchor,
    #[cfg(ChApps)]
    ChApps,
    #[cfg(ChAppsMinus)]
    ChAppsMinus,
    #[cfg(ChAppsPlus)]
    ChAppsPlus,
    #[cfg(ChArchive)]
    ChArchive,
    #[cfg(ChArrowDown)]
    ChArrowDown,
    #[cfg(ChArrowDownLeft)]
    ChArrowDownLeft,
    #[cfg(ChArrowDownRight)]
    ChArrowDownRight,
    #[cfg(ChArrowLeft)]
    ChArrowLeft,
    #[cfg(ChArrowRight)]
    ChArrowRight,
    #[cfg(ChArrowUp)]
    ChArrowUp,
    #[cfg(ChArrowUpLeft)]
    ChArrowUpLeft,
    #[cfg(ChArrowUpRight)]
    ChArrowUpRight,
    #[cfg(ChAtSign)]
    ChAtSign,
    #[cfg(ChAtom)]
    ChAtom,
    #[cfg(ChBell)]
    ChBell,
    #[cfg(ChBellSlash)]
    ChBellSlash,
    #[cfg(ChBin)]
    ChBin,
    #[cfg(ChBinary)]
    ChBinary,
    #[cfg(ChBlock)]
    ChBlock,
    #[cfg(ChBluetooth)]
    ChBluetooth,
    #[cfg(ChBluetoothConnected)]
    ChBluetoothConnected,
    #[cfg(ChBluetoothSearching)]
    ChBluetoothSearching,
    #[cfg(ChBluetoothSlash)]
    ChBluetoothSlash,
    #[cfg(ChBook)]
    ChBook,
    #[cfg(ChBookOpen)]
    ChBookOpen,
    #[cfg(ChBookmark)]
    ChBookmark,
    #[cfg(ChBriefcase)]
    ChBriefcase,
    #[cfg(ChBug)]
    ChBug,
    #[cfg(ChCalendar)]
    ChCalendar,
    #[cfg(ChCamera)]
    ChCamera,
    #[cfg(ChCameraVideo)]
    ChCameraVideo,
    #[cfg(ChCameraVideoSlash)]
    ChCameraVideoSlash,
    #[cfg(ChCandy)]
    ChCandy,
    #[cfg(ChCards)]
    ChCards,
    #[cfg(ChCast)]
    ChCast,
    #[cfg(ChCertificate)]
    ChCertificate,
    #[cfg(ChChartBar)]
    ChChartBar,
    #[cfg(ChChartLine)]
    ChChartLine,
    #[cfg(ChChevronDown)]
    ChChevronDown,
    #[cfg(ChChevronLeft)]
    ChChevronLeft,
    #[cfg(ChChevronRight)]
    ChChevronRight,
    #[cfg(ChChevronUp)]
    ChChevronUp,
    #[cfg(ChChevronsDown)]
    ChChevronsDown,
    #[cfg(ChChevronsLeft)]
    ChChevronsLeft,
    #[cfg(ChChevronsRight)]
    ChChevronsRight,
    #[cfg(ChChevronsUp)]
    ChChevronsUp,
    #[cfg(ChChevronsUpDown)]
    ChChevronsUpDown,
    #[cfg(ChChip)]
    ChChip,
    #[cfg(ChCircle)]
    ChCircle,
    #[cfg(ChCircleCross)]
    ChCircleCross,
    #[cfg(ChCircleMinus)]
    ChCircleMinus,
    #[cfg(ChCircleTick)]
    ChCircleTick,
    #[cfg(ChCircleWarning)]
    ChCircleWarning,
    #[cfg(ChClipboard)]
    ChClipboard,
    #[cfg(ChClipboardTick)]
    ChClipboardTick,
    #[cfg(ChClock)]
    ChClock,
    #[cfg(ChClockAlarm)]
    ChClockAlarm,
    #[cfg(ChCloud)]
    ChCloud,
    #[cfg(ChClover)]
    ChClover,
    #[cfg(ChCode)]
    ChCode,
    #[cfg(ChCoffee)]
    ChCoffee,
    #[cfg(ChCog)]
    ChCog,
    #[cfg(ChCompass)]
    ChCompass,
    #[cfg(ChConicalFlask)]
    ChConicalFlask,
    #[cfg(ChContainer)]
    ChContainer,
    #[cfg(ChCopy)]
    ChCopy,
    #[cfg(ChCopyleft)]
    ChCopyleft,
    #[cfg(ChCopyright)]
    ChCopyright,
    #[cfg(ChCreditCard)]
    ChCreditCard,
    #[cfg(ChCrop)]
    ChCrop,
    #[cfg(ChCross)]
    ChCross,
    #[cfg(ChCrosshair)]
    ChCrosshair,
    #[cfg(ChCube)]
    ChCube,
    #[cfg(ChCursor)]
    ChCursor,
    #[cfg(ChDatabase)]
    ChDatabase,
    #[cfg(ChDiamond)]
    ChDiamond,
    #[cfg(ChDiff)]
    ChDiff,
    #[cfg(ChDisc)]
    ChDisc,
    #[cfg(ChDownload)]
    ChDownload,
    #[cfg(ChDroplet)]
    ChDroplet,
    #[cfg(ChEraser)]
    ChEraser,
    #[cfg(ChEye)]
    ChEye,
    #[cfg(ChEyeSlash)]
    ChEyeSlash,
    #[cfg(ChFaceFrown)]
    ChFaceFrown,
    #[cfg(ChFaceNeutral)]
    ChFaceNeutral,
    #[cfg(ChFaceSmile)]
    ChFaceSmile,
    #[cfg(ChFile)]
    ChFile,
    #[cfg(ChFileBinary)]
    ChFileBinary,
    #[cfg(ChFileCode)]
    ChFileCode,
    #[cfg(ChFileSymlink)]
    ChFileSymlink,
    #[cfg(ChFiles)]
    ChFiles,
    #[cfg(ChFilter)]
    ChFilter,
    #[cfg(ChFlag)]
    ChFlag,
    #[cfg(ChFlame)]
    ChFlame,
    #[cfg(ChFloppyDisk)]
    ChFloppyDisk,
    #[cfg(ChFolder)]
    ChFolder,
    #[cfg(ChFolderSymlink)]
    ChFolderSymlink,
    #[cfg(ChFolders)]
    ChFolders,
    #[cfg(ChForward)]
    ChForward,
    #[cfg(ChGamepad)]
    ChGamepad,
    #[cfg(ChGem)]
    ChGem,
    #[cfg(ChGift)]
    ChGift,
    #[cfg(ChGitBranch)]
    ChGitBranch,
    #[cfg(ChGitCherryPick)]
    ChGitCherryPick,
    #[cfg(ChGitCommit)]
    ChGitCommit,
    #[cfg(ChGitCompare)]
    ChGitCompare,
    #[cfg(ChGitFork)]
    ChGitFork,
    #[cfg(ChGitMerge)]
    ChGitMerge,
    #[cfg(ChGitRequest)]
    ChGitRequest,
    #[cfg(ChGitRequestCross)]
    ChGitRequestCross,
    #[cfg(ChGitRequestDraft)]
    ChGitRequestDraft,
    #[cfg(ChGithub)]
    ChGithub,
    #[cfg(ChGitlab)]
    ChGitlab,
    #[cfg(ChGlasses)]
    ChGlasses,
    #[cfg(ChGlobe)]
    ChGlobe,
    #[cfg(ChGrabHorizontal)]
    ChGrabHorizontal,
    #[cfg(ChGrabVertical)]
    ChGrabVertical,
    #[cfg(ChGraduateCap)]
    ChGraduateCap,
    #[cfg(ChHash)]
    ChHash,
    #[cfg(ChHeadphones)]
    ChHeadphones,
    #[cfg(ChHeart)]
    ChHeart,
    #[cfg(ChHelp)]
    ChHelp,
    #[cfg(ChHexagon)]
    ChHexagon,
    #[cfg(ChHome)]
    ChHome,
    #[cfg(ChHourglass)]
    ChHourglass,
    #[cfg(ChId)]
    ChId,
    #[cfg(ChImage)]
    ChImage,
    #[cfg(ChInbox)]
    ChInbox,
    #[cfg(ChInfinity)]
    ChInfinity,
    #[cfg(ChInfo)]
    ChInfo,
    #[cfg(ChKey)]
    ChKey,
    #[cfg(ChLaptop)]
    ChLaptop,
    #[cfg(ChLayoutColumns)]
    ChLayoutColumns,
    #[cfg(ChLayoutDashboard)]
    ChLayoutDashboard,
    #[cfg(ChLayoutGrid)]
    ChLayoutGrid,
    #[cfg(ChLayoutList)]
    ChLayoutList,
    #[cfg(ChLayoutRows)]
    ChLayoutRows,
    #[cfg(ChLayoutSidebar)]
    ChLayoutSidebar,
    #[cfg(ChLayoutStackH)]
    ChLayoutStackH,
    #[cfg(ChLayoutStackV)]
    ChLayoutStackV,
    #[cfg(ChLightbulb)]
    ChLightbulb,
    #[cfg(ChLightningBolt)]
    ChLightningBolt,
    #[cfg(ChLink)]
    ChLink,
    #[cfg(ChLinkExternal)]
    ChLinkExternal,
    #[cfg(ChLinkSlash)]
    ChLinkSlash,
    #[cfg(ChMail)]
    ChMail,
    #[cfg(ChMap)]
    ChMap,
    #[cfg(ChMapPin)]
    ChMapPin,
    #[cfg(ChMediaBack)]
    ChMediaBack,
    #[cfg(ChMediaEject)]
    ChMediaEject,
    #[cfg(ChMediaFastForward)]
    ChMediaFastForward,
    #[cfg(ChMediaPause)]
    ChMediaPause,
    #[cfg(ChMediaPlay)]
    ChMediaPlay,
    #[cfg(ChMediaRewind)]
    ChMediaRewind,
    #[cfg(ChMediaSkip)]
    ChMediaSkip,
    #[cfg(ChMenuHamburger)]
    ChMenuHamburger,
    #[cfg(ChMenuKebab)]
    ChMenuKebab,
    #[cfg(ChMenuMeatball)]
    ChMenuMeatball,
    #[cfg(ChMessage)]
    ChMessage,
    #[cfg(ChMessages)]
    ChMessages,
    #[cfg(ChMicrophone)]
    ChMicrophone,
    #[cfg(ChMinus)]
    ChMinus,
    #[cfg(ChMobile)]
    ChMobile,
    #[cfg(ChMonitor)]
    ChMonitor,
    #[cfg(ChMonitorArrow)]
    ChMonitorArrow,
    #[cfg(ChMonitorCross)]
    ChMonitorCross,
    #[cfg(ChMoon)]
    ChMoon,
    #[cfg(ChMove)]
    ChMove,
    #[cfg(ChMusic)]
    ChMusic,
    #[cfg(ChNewspaper)]
    ChNewspaper,
    #[cfg(ChNorthStar)]
    ChNorthStar,
    #[cfg(ChNotes)]
    ChNotes,
    #[cfg(ChNotesCross)]
    ChNotesCross,
    #[cfg(ChNotesTick)]
    ChNotesTick,
    #[cfg(ChNut)]
    ChNut,
    #[cfg(ChOctagon)]
    ChOctagon,
    #[cfg(ChOctagonWarning)]
    ChOctagonWarning,
    #[cfg(ChOrganisation)]
    ChOrganisation,
    #[cfg(ChPackage)]
    ChPackage,
    #[cfg(ChPadlock)]
    ChPadlock,
    #[cfg(ChPaperPlane)]
    ChPaperPlane,
    #[cfg(ChPaperclip)]
    ChPaperclip,
    #[cfg(ChPencil)]
    ChPencil,
    #[cfg(ChPeople)]
    ChPeople,
    #[cfg(ChPerson)]
    ChPerson,
    #[cfg(ChPhone)]
    ChPhone,
    #[cfg(ChPhoneCall)]
    ChPhoneCall,
    #[cfg(ChPhoneCross)]
    ChPhoneCross,
    #[cfg(ChPhoneForward)]
    ChPhoneForward,
    #[cfg(ChPhoneIncoming)]
    ChPhoneIncoming,
    #[cfg(ChPhoneOutgoing)]
    ChPhoneOutgoing,
    #[cfg(ChPin)]
    ChPin,
    #[cfg(ChPlantPot)]
    ChPlantPot,
    #[cfg(ChPlus)]
    ChPlus,
    #[cfg(ChPower)]
    ChPower,
    #[cfg(ChPrinter)]
    ChPrinter,
    #[cfg(ChPulse)]
    ChPulse,
    #[cfg(ChQuote)]
    ChQuote,
    #[cfg(ChRefresh)]
    ChRefresh,
    #[cfg(ChReply)]
    ChReply,
    #[cfg(ChRobot)]
    ChRobot,
    #[cfg(ChRocket)]
    ChRocket,
    #[cfg(ChRotateAntiClockwise)]
    ChRotateAntiClockwise,
    #[cfg(ChRotateClockwise)]
    ChRotateClockwise,
    #[cfg(ChScales)]
    ChScales,
    #[cfg(ChScreenMaximise)]
    ChScreenMaximise,
    #[cfg(ChScreenMinimise)]
    ChScreenMinimise,
    #[cfg(ChSearch)]
    ChSearch,
    #[cfg(ChServer)]
    ChServer,
    #[cfg(ChShare)]
    ChShare,
    #[cfg(ChShield)]
    ChShield,
    #[cfg(ChShieldCross)]
    ChShieldCross,
    #[cfg(ChShieldKeyhole)]
    ChShieldKeyhole,
    #[cfg(ChShieldTick)]
    ChShieldTick,
    #[cfg(ChShieldWarning)]
    ChShieldWarning,
    #[cfg(ChShoppingBag)]
    ChShoppingBag,
    #[cfg(ChSignIn)]
    ChSignIn,
    #[cfg(ChSignOut)]
    ChSignOut,
    #[cfg(ChSignpost)]
    ChSignpost,
    #[cfg(ChSkull)]
    ChSkull,
    #[cfg(ChSnowflake)]
    ChSnowflake,
    #[cfg(ChSoundDown)]
    ChSoundDown,
    #[cfg(ChSoundMute)]
    ChSoundMute,
    #[cfg(ChSoundUp)]
    ChSoundUp,
    #[cfg(ChSpeaker)]
    ChSpeaker,
    #[cfg(ChSquare)]
    ChSquare,
    #[cfg(ChSquareCross)]
    ChSquareCross,
    #[cfg(ChSquareTick)]
    ChSquareTick,
    #[cfg(ChStack)]
    ChStack,
    #[cfg(ChStackPop)]
    ChStackPop,
    #[cfg(ChStackPush)]
    ChStackPush,
    #[cfg(ChStar)]
    ChStar,
    #[cfg(ChStickyNote)]
    ChStickyNote,
    #[cfg(ChSun)]
    ChSun,
    #[cfg(ChSwapHorizontal)]
    ChSwapHorizontal,
    #[cfg(ChSwapVertical)]
    ChSwapVertical,
    #[cfg(ChSword)]
    ChSword,
    #[cfg(ChSwords)]
    ChSwords,
    #[cfg(ChTablet)]
    ChTablet,
    #[cfg(ChTag)]
    ChTag,
    #[cfg(ChTelescope)]
    ChTelescope,
    #[cfg(ChTent)]
    ChTent,
    #[cfg(ChTerminal)]
    ChTerminal,
    #[cfg(ChThumbDown)]
    ChThumbDown,
    #[cfg(ChThumbUp)]
    ChThumbUp,
    #[cfg(ChTick)]
    ChTick,
    #[cfg(ChTickDouble)]
    ChTickDouble,
    #[cfg(ChTicket)]
    ChTicket,
    #[cfg(ChTreeFir)]
    ChTreeFir,
    #[cfg(ChTriangle)]
    ChTriangle,
    #[cfg(ChTrophy)]
    ChTrophy,
    #[cfg(ChUmbrella)]
    ChUmbrella,
    #[cfg(ChUpload)]
    ChUpload,
    #[cfg(ChWifi)]
    ChWifi,
    #[cfg(ChWifiFair)]
    ChWifiFair,
    #[cfg(ChWifiPoor)]
    ChWifiPoor,
    #[cfg(ChWifiSlash)]
    ChWifiSlash,
    #[cfg(ChWifiWarning)]
    ChWifiWarning,
    #[cfg(ChZoomIn)]
    ChZoomIn,
    #[cfg(ChZoomOut)]
    ChZoomOut,
}

#[cfg(ChAnchor)]
const CH_ANCHOR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m8 5.75v8.25m-4.75-6.25h-1.5c0 4 2.5 6.5 6.25 6.5s6.25-2.5 6.25-6.5h-1.5" />
<circle cx="8" cy="3.5" r="1.75" />"###
};
#[cfg(ChApps)]
const CH_APPS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="4.5" height="4.5" x="1.75" y="1.75" />
<rect width="4.5" height="4.5" x="1.75" y="9.75" />
<rect width="4.5" height="4.5" x="9.75" y="9.75" />
<rect width="4.5" height="4.5" x="9.75" y="1.75" />"###
};
#[cfg(ChAppsMinus)]
const CH_APPS_MINUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="1.75" y="1.75" width="4.5" height="4.5" />
<rect x="1.75" y="9.75" width="4.5" height="4.5" />
<rect x="9.75" y="9.75" width="4.5" height="4.5" />
<path d="m14.8 3.75h-5" />"###
};
#[cfg(ChAppsPlus)]
const CH_APPS_PLUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="1.75" y="1.75" width="4.5" height="4.5" />
<rect x="1.75" y="9.75" width="4.5" height="4.5" />
<rect x="9.75" y="9.75" width="4.5" height="4.5" />
<path d="m14.8 3.75h-5m2.5-2.5v5" />"###
};
#[cfg(ChArchive)]
const CH_ARCHIVE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="3.5" width="12.5" y="2.75" x="1.75" />
<path d="m6.75 9.25h2.5m-6.5-2.5v7.5h10.5v-7.5" />"###
};
#[cfg(ChArrowDown)]
const CH_ARROW_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3.25 8.75 4.5 4.5 4.5-4.5m-4.5-6v10.5" />"###
};
#[cfg(ChArrowDownLeft)]
const CH_ARROW_DOWN_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m10.75 11.75h-6.5v-6.5m7.5-1-7.5 7.5" />"###
};
#[cfg(ChArrowDownRight)]
const CH_ARROW_DOWN_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m5.25 11.75h6.5v-6.5m-7.5-1 7.5 7.5" />"###
};
#[cfg(ChArrowLeft)]
const CH_ARROW_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m7.25 3.75-4.5 4.5 4.5 4.5m6-4.5h-10.5" />"###
};
#[cfg(ChArrowRight)]
const CH_ARROW_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m8.75 3.25 4.5 4.5-4.5 4.5m-6-4.5h10.5" />"###
};
#[cfg(ChArrowUp)]
const CH_ARROW_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3.75 7.25 4.5-4.5 4.5 4.5m-4.5 6v-10.5" />"###
};
#[cfg(ChArrowUpLeft)]
const CH_ARROW_UP_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m10.75 4.25h-6.5v6.5m7.5 1-7.5-7.5" />"###
};
#[cfg(ChArrowUpRight)]
const CH_ARROW_UP_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m5.25 4.25h6.5v6.5m-7.5 1 7.5-7.5" />"###
};
#[cfg(ChAtSign)]
const CH_AT_SIGN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m10.25 8c0 3.25 4 3.25 4 0 0-3.45178-2.7982-6.25-6.25-6.25-3.45178 0-6.25 2.79822-6.25 6.25s2.79822 6.25 6.25 6.25c2.25 0 3.25-1 3.25-1" />
<circle cx="8" cy="8" r="2.25" />"###
};
#[cfg(ChAtom)]
const CH_ATOM: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<ellipse transform="rotate(45)" cx="11.3" rx="8.28" ry="3.17" />
<ellipse transform="rotate(315)" cy="11.3" rx="8.28" ry="3.17" />
<path d="m8 8v0" />"###
};
#[cfg(ChBell)]
const CH_BELL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m8 1.75c-2.46803 0-4.25 1.5-4.25 3.5v3l-2 3.5h12.5l-2-3.5v-3c0-2-1.16605-3.5-4.25-3.5z" />
<path d="m5.75 12.25c0 3 4.5 3 4.5 0" />"###
};
#[cfg(ChBellSlash)]
const CH_BELL_SLASH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m5.75 12.25c0 3 4.5 3 4.5 0" />
<path d="m12.25 8.25v-3c0-2-1.16605-3.5-4.25-3.5m-3.75 2c-.530590.584957-.5.674089-.5 1.5v3l-2 3.5h8.5" />
<path d="m5.75 12.25c0 3 4.5 3 4.5 0" />
<path d="m2.75 1.75 10.5 12.5" />"###
};
#[cfg(ChBin)]
const CH_BIN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m5.75 4.25v-2.5h4.5v2.5m-6.5 1v9h8.5v-9m-9.5-.5h10.5" />"###
};
#[cfg(ChBinary)]
const CH_BINARY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="4.5" width="3" y="1.75" x="3.25" />
<path d="m9.75 6.25h3m-3-4.5h1.5v4" />
<rect height="4.5" width="3" y="9.75" x="9.75" />
<path d="m3.25 14.25h3m-3-4.5h1.5v4" />"###
};
#[cfg(ChBlock)]
const CH_BLOCK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cy="8" cx="8" r="6.25" />
<line x1="4.25" x2="12.25" y1="11.75" y2="3.75" />"###
};
#[cfg(ChBluetooth)]
const CH_BLUETOOTH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3.75 11.25 8.5-6.25-4.5-3.25v12.5l4.5-3.25-8.5-6.25" />"###
};
#[cfg(ChBluetoothConnected)]
const CH_BLUETOOTH_CONNECTED: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3.75 11.25 8.5-6.25-4.5-3.25v12.5l4.5-3.25-8.5-6.25" />
<path d="m1.75 8h1.5m9.5 0h1.5" />"###
};
#[cfg(ChBluetoothSearching)]
const CH_BLUETOOTH_SEARCHING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m1.75 11.25 8.5-6.25-4.5-3.25v12.5l4.5-3.25-8.5-6.25" />
<path d="m13.25 6.25s1 .5 1 1.75-1 1.75-1 1.75m-2-1.75v0" />"###
};
#[cfg(ChBluetoothSlash)]
const CH_BLUETOOTH_SLASH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m10.75 6.25 1.5-1.25-4.5-3.25v2.5m4.5 6.75-4.5 3.25v-6l-4 3" />
<path d="m1.75 3.25 12.5 9" />"###
};
#[cfg(ChBook)]
const CH_BOOK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m11.75 11.75v2m1.5.5h-9c-.75 0-1.5-.5-1.5-1.5s.75-1.5 1.5-1.5h9v-9.5h-9c-.75 0-1.5.75-1.5 1.5v9.5" />"###
};
#[cfg(ChBookOpen)]
const CH_BOOK_OPEN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m8 3.75c-1.75-1-2.25-1-6.25-1v9.5c4 0 4.5 0 6.25 1 1.75-1 3.25-1 6.25-1v-9.5c-4 0-4.5 0-6.25 1z" />
<path d="m8 4.25v8.5" />"###
};
#[cfg(ChBookmark)]
const CH_BOOKMARK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="3.75 1.75,12.25 1.75,12.25 14.25,8 9.75,3.75 14.25" />"###
};
#[cfg(ChBriefcase)]
const CH_BRIEFCASE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="9.5" width="12.5" y="4.75" x="1.75" />
<path d="m1.75 6.25s-.5 3.5 3 3.5h6.5c3.5 0 3-3.5 3-3.5m-8.5-2v-2.5h4.5v2.5" />"###
};
#[cfg(ChBug)]
const CH_BUG: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="8" cy="10" r="4.25" />
<path d="m14.25 10.25h-1.5m-1 2.5 1.5 1.5m0-8.5-1.5 1.5m-10 3h1.5m1 2.5-1.5 1.5m0-8.5 1.5 1.5m1.5-1.5s-.75-3 2.25-3 2.25 3 2.25 3" />"###
};
#[cfg(ChCalendar)]
const CH_CALENDAR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="10.5" width="12.5" y="3.75" x="1.75" />
<path d="m11.25 1.75v1.5m-6.5-1.5v1.5m-2.5 4h11.5" />"###
};
#[cfg(ChCamera)]
const CH_CAMERA: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m1.75 4.75v8.5h12.5v-8.5h-3l-1.5-2h-3.5l-1.5 2z" />
<circle cx="8" cy="8.5" r="2.25" />"###
};
#[cfg(ChCameraVideo)]
const CH_CAMERA_VIDEO: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="7.5" width="7.5" y="4.75" x="1.75" />
<path d="m9.75 7.25 4.5-2.5v7.5l-4.5-2.5" />"###
};
#[cfg(ChCameraVideoSlash)]
const CH_CAMERA_VIDEO_SLASH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m11.25 10.75 3 1.5v-7.5l-5 2.5v-2.5h-2.5m1.5 7.5h-6.5v-7.5h1.5" />
<line x1="1.75" y1="2.25" x2="10.25" y2="14.25" />"###
};
#[cfg(ChCandy)]
const CH_CANDY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="8" cy="8" r="3.25" />
<path d="m7.25 11.25c0 1-.5 2.5-1.5 3-.75 0-1.5-1-2-2-1-.5-2-1.5-2-2 .5-1 2-1.5 3-1.5m4-4c0-1 .5-2.5 1.5-3 .75 0 1.5 1 2 2 1 .5 2 1.5 2 2-.5 1-2 1.5-3 1.5" />"###
};
#[cfg(ChCards)]
const CH_CARDS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="11.5" width="8.25" y="2.75" x="1.75" />
<path d="m10 3.75 4.25 2-4.25 7.5" />"###
};
#[cfg(ChCast)]
const CH_CAST: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m1.75 5.25v-2.5h12.5v10.5h-4.5" />
<path d="m1.75 8.25c2.76142 0 5 2.23858 5 5m-5-2.5c1.38071 0 2.5 1.11929 2.5 2.5m-2.5 0v0" />"###
};
#[cfg(ChCertificate)]
const CH_CERTIFICATE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="11.25 1.75,2.75 1.75,2.75 13.25,5.25 13.25" />
<polyline points="8.75 9.75,8.25 14.25,10.50 13.25,12.75 14.25,12.25 9.75" />
<circle cx="10.5" cy="7.5" r="2.75" />"###
};
#[cfg(ChChartBar)]
const CH_CHART_BAR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m1.75 1.75v12.5h12.5m-9-3v-2.5m4 2.5v-5.5m4 5.5v-8.5" />"###
};
#[cfg(ChChartLine)]
const CH_CHART_LINE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m4.75 11.25 2.5-4.5 2.5 2.5 3.5-6m-11.5-1.5v12.5h12.5" />"###
};
#[cfg(ChChevronDown)]
const CH_CHEVRON_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3.75 5.75 4.25 4.5 4.25-4.5" />"###
};
#[cfg(ChChevronLeft)]
const CH_CHEVRON_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m10.25 3.75-4.5 4.25l4.5 4.25" />"###
};
#[cfg(ChChevronRight)]
const CH_CHEVRON_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m5.75 12.25 4.5-4.25-4.5-4.25" />"###
};
#[cfg(ChChevronUp)]
const CH_CHEVRON_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m12.25 10.25-4.25-4.5-4.25 4.5" />"###
};
#[cfg(ChChevronsDown)]
const CH_CHEVRONS_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3.75 3.75 4.25 4.5 4.25-4.5m-8.5 5 4.25 4.5 4.25-4.5" />"###
};
#[cfg(ChChevronsLeft)]
const CH_CHEVRONS_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m12.25 3.75-4.5 4.25l4.5 4.25m-5-8.5-4.5 4.25 4.5 4.25" />"###
};
#[cfg(ChChevronsRight)]
const CH_CHEVRONS_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3.75 12.25 4.5-4.25-4.5-4.25m5 8.5l4.5-4.25-4.5-4.25" />"###
};
#[cfg(ChChevronsUp)]
const CH_CHEVRONS_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m12.25 12.25-4.25-4.5-4.25 4.5m8.5-5l-4.25-4.5-4.25 4.5" />"###
};
#[cfg(ChChevronsUpDown)]
const CH_CHEVRONS_UP_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m11.25 10.75-3.25 3.5-3.25-3.5" />
<path d="m11.25 5.25-3.25-3.5-3.25 3.5" />"###
};
#[cfg(ChChip)]
const CH_CHIP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="10.5" width="10.5" y="2.75" x="2.75" />
<rect height="3.5" width="3.5" y="6.25" x="6.25" />
<path d="m2.25 10.25h-1m1-4.5h-1m13.5 4.5h-1m1-4.5h-1m-3.5 8v1m-4.5-1v1m4.5-13.5v1m-4.5-1v1" />"###
};
#[cfg(ChCircle)]
const CH_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="8" cy="8" r="6.25" />"###
};
#[cfg(ChCircleCross)]
const CH_CIRCLE_CROSS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m10.25 5.75-4.5 4.5m0-4.5 4.5 4.5" />
<circle cx="8" cy="8" r="6.25" />"###
};
#[cfg(ChCircleMinus)]
const CH_CIRCLE_MINUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="8" cy="8" r="6.25" />
<line x1="4.75" y1="8" x2="11.25" y2="8" />"###
};
#[cfg(ChCircleTick)]
const CH_CIRCLE_TICK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m14.25 8.75c-.5 2.5-2.3849 4.85363-5.03069 5.37991-2.64578.5263-5.33066-.7044-6.65903-3.0523-1.32837-2.34784-1.00043-5.28307.81336-7.27989 1.81379-1.99683 4.87636-2.54771 7.37636-1.54771" />
<polyline points="5.75 7.75,8.25 10.25,14.25 3.75" />"###
};
#[cfg(ChCircleWarning)]
const CH_CIRCLE_WARNING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="8" cy="8" r="6.25" />
<path d="m8 10.75v0m0-6v3.5" />"###
};
#[cfg(ChClipboard)]
const CH_CLIPBOARD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="3.5" width="4.5" y="1.75" x="5.75" />
<path d="m5.25 2.75h-2.5v11.5h10.5v-11.5h-2.5" />"###
};
#[cfg(ChClipboardTick)]
const CH_CLIPBOARD_TICK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="5.75" y="1.75" width="4.5" height="3.5" />
<path d="m9.75 12.8 1.5 1.5 3-2.5m-9-9h-2.5v11.5h4.5m6-5v-6.5h-2.5" />"###
};
#[cfg(ChClock)]
const CH_CLOCK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cy="8" cx="8" r="6.25" />
<path d="m8.25 4.75v3.5l-2.5 2" />"###
};
#[cfg(ChClockAlarm)]
const CH_CLOCK_ALARM: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m11.75 1.75 2.5 2m-10-2-2.5 2m10.5 9.5 1 1m-9.5-1-1 1m5.5-7.5v2.5l-1.5 1" />
<circle cx="8" cy="9" r="5.25" />"###
};
#[cfg(ChCloud)]
const CH_CLOUD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m7 3.75c-1.79493 0-3.25 1.45507-3.25 3.25.00152.254757.032983.508452.09375.755859h-.00195c-1.17822.08305-2.09165 1.063-2.0918 2.24414 0 1.24264 1.00736 2.25 2.25 2.25h7.5c1.51878 0 2.75-1.23122 2.75-2.75s-1.2312-2.75-2.75-2.75c-.4352-.00022-.8643.10286-1.252.30078.0008-.01692.0015-.03385.0020-.05078 0-1.79493-1.45507-3.25-3.25-3.25z" />"###
};
#[cfg(ChClover)]
const CH_CLOVER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m4.75 2.75c-.50 1.5 1.25 3.25 3.25 5.25 2-2 3.75-3.75 3.25-5.25s-2.5-1-3.25.50c-.75-1.5-2.75-2-3.25-.50zm3.25 5.25c2 2 3.75 3.75 5.25 3.25s1-2.5-.5-3.25c1.5-.75 2-2.75.5-3.25s-3.25 1.25-5.25 3.25zm0 0c-2 2-3.75 3.75-3.25 5.25s2.5 1 3.25-.5c.75 1.5 2.75 2 3.25.5s-1.25-3.25-3.25-5.25zm0 0c-2-2-3.75-3.75-5.25-3.25s-1 2.5.5 3.25c-1.5.75-2 2.75-.5 3.25s3.25-1.25 5.25-3.25z" />"###
};
#[cfg(ChCode)]
const CH_CODE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m5.25 11.25-3.5-3.25 3.5-3.25m5.5 6.5 3.5-3.25-3.5-3.25" />"###
};
#[cfg(ChCoffee)]
const CH_COFFEE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m10.75 11.25c4.5 0 4.5-5.5 0-5.5h-9v5c0 5 8.5 5 8.5 0v-5" />
<path d="m8.75 1.75v1.5m-3-1.5v1.5m-3-1.5v1.5" />"###
};
#[cfg(ChCog)]
const CH_COG: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="8" cy="8" r="1.75" />
<path d="m6.75 1.75-.5 1.5-1.5 1-2-.5-1 2 1.5 1.5v1.5l-1.5 1.5 1 2 2-.5 1.5 1 .5 1.5h2.5l.5-1.5 1.5-1 2 .5 1-2-1.5-1.5v-1.5l1.5-1.5-1-2-2 .5-1.5-1-.5-1.5z" />"###
};
#[cfg(ChCompass)]
const CH_COMPASS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cy="8" cx="8" r="6.25" />
<polygon points="6.75 6.75,5.75 10.75,9.25 9.25,10.25 5.25" />"###
};
#[cfg(ChConicalFlask)]
const CH_CONICAL_FLASK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m4.75 1.75h6.5m-6.5 8h6.5m-5.5-7.5v4.5l-4 7.5h12.5l-4-7.5v-4.5" />"###
};
#[cfg(ChContainer)]
const CH_CONTAINER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m1.75 12.2 5.5 2 7-4.5v-6l-5.5-2-7 4.5z" />
<path d="m10.8 6.25v5.5m-3.5-3.5v6m-5.5-8 5.5 2 7-4.5" />"###
};
#[cfg(ChCopy)]
const CH_COPY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m11.25 4.25v-2.5h-9.5v9.5h2.5m.5-6.5v9.5h9.5v-9.5z" />"###
};
#[cfg(ChCopyleft)]
const CH_COPYLEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cy="8" cx="8" r="6.25" />
<path d="m6 6.75s.75-1 2-1 2.25 1 2.25 2.25-1 2.25-2.25 2.25-2-1-2-1" />"###
};
#[cfg(ChCopyright)]
const CH_COPYRIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cy="8" cx="8" r="6.25" />
<path d="m10 6.75s-.75-1-2-1-2.25 1-2.25 2.25 1 2.25 2.25 2.25 2-1 2-1" />"###
};
#[cfg(ChCreditCard)]
const CH_CREDIT_CARD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="9.5" width="12.5" y="3.75" x="1.75" />
<path d="m9.75 10.25h1.5m-9-3h11.5" />"###
};
#[cfg(ChCrop)]
const CH_CROP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m4.25 1.75v10h10" />
<path d="m11.8 14.2v-2.5m0-2.5v-5h-5m-2.5 0h-2.5" />"###
};
#[cfg(ChCross)]
const CH_CROSS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m11.25 4.75-6.5 6.5m0-6.5 6.5 6.5" />"###
};
#[cfg(ChCrosshair)]
const CH_CROSSHAIR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m8 5.25v-3m0 11.5v-3m2.75-2.75h3m-11.5 0h3" />
<circle cx="8" cy="8" r="6.25" />"###
};
#[cfg(ChCube)]
const CH_CUBE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="1.75 4.75 8 1.25 14.25 4.75 14.25 11.25 8 14.75 1.75 11.25" />
<path d="m8 14v-6m5.75-3-5.75 3m-6-3 6 3" />"###
};
#[cfg(ChCursor)]
const CH_CURSOR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="1.75 1.75,6.25 14.25,8.75 8.75,14.25 6.25" />
<line x1="9.25" y1="9.25" x2="13.25" y2="13.25" />"###
};
#[cfg(ChDatabase)]
const CH_DATABASE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m8 1.75c-3.75 0-5.25 2-5.25 2v4.5 4s1.5 2 5.25 2 5.25-2 5.25-2v-4-4.5s-1.5-2-5.25-2z" />
<path d="m2.75 8.25s1.5 2 5.25 2 5.25-2 5.25-2m-10.5-4s1.5 2 5.25 2 5.25-2 5.25-2" />"###
};
#[cfg(ChDiamond)]
const CH_DIAMOND: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="1.25 8,8 14.75,14.75 8,8 1.25" />"###
};
#[cfg(ChDiff)]
const CH_DIFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3.75 13.75h8m0-7.5h-8m4-4v8" />"###
};
#[cfg(ChDisc)]
const CH_DISC: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="8" cy="8" r="6.25" />
<circle cx="8" cy="8" r="1.75" />"###
};
#[cfg(ChDownload)]
const CH_DOWNLOAD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3.25 13.25h9m-8.5-6.5 4 3.5 4-3.5m-4-5v8.5" />"###
};
#[cfg(ChDroplet)]
const CH_DROPLET: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m2.75 9c0 2.9 2.35 5.25 5.25 5.25s5.25-2.35 5.25-5.25c0-3.25-5.25-7.25-5.25-7.25s-5.25 4-5.25 7.25z" />"###
};
#[cfg(ChEraser)]
const CH_ERASER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="14.25 13.25,4.75 13.25,1.75 10.25,9.25 2.75,14.25 7.75,8.75 13.25" />
<line x1="5.25" y1="6.75" x2="10.25" y2="11.75" />"###
};
#[cfg(ChEye)]
const CH_EYE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m1.75 8s2-4.25 6.25-4.25 6.25 4.25 6.25 4.25-2 4.25-6.25 4.25-6.25-4.25-6.25-4.25z" />
<circle cx="8" cy="8" r="1.25" fill="currentColor" />"###
};
#[cfg(ChEyeSlash)]
const CH_EYE_SLASH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m8.75 3.75c3.5.5 5.5 4.25 5.5 4.25s-.5 1.25-1.5 2.25m-2.5 1.5c-6 2-8.5-3.75-8.5-3.75s.5-1.75 3-3.25" />
<path d="m8.625 9.08253a1.25 1.25 0 0 1 -1.64894 -.36556 1.25 1.25 0 0 1 .22046 -1.67453l.80348.95756z" fill="currentColor" />
<path d="m3.75 1.75 8.5 12.5" />"###
};
#[cfg(ChFaceFrown)]
const CH_FACE_FROWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="8" cy="8" r="6.25" />
<path d="m9.75 6.25v-.5m-3.5.5v-.5m-.5 5s.5-1 2.25-1 2.25 1 2.25 1" />"###
};
#[cfg(ChFaceNeutral)]
const CH_FACE_NEUTRAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="8" cy="8" r="6.25" />
<path d="m9.75 6.25v-.5m-3.5.5v-.5m-.5 4.5h4.5" />"###
};
#[cfg(ChFaceSmile)]
const CH_FACE_SMILE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="8" cy="8" r="6.25" />
<path d="m9.75 6.25v-.5m-3.5.5v-.5m-.5 4s.5 1.5 2.25 1.5 2.25-1.5 2.25-1.5" />"###
};
#[cfg(ChFile)]
const CH_FILE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="2.75 1.75,8.25 1.75,13.25 6.75,13.25 14.25,2.75 14.25" />
<polyline points="7.75 2.25,7.75 7.25,12.75 7.25" />"###
};
#[cfg(ChFileBinary)]
const CH_FILE_BINARY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="2.75 7.75,2.75 1.75,8.25 1.75,13.25 6.75,13.25 14.25" />
<rect x="1.75" y="10.8" width="3" height="3.5" />
<path d="m7.25 14.2h3m-3-3.5h1.5v3" />
<polyline points="7.75 2.25 7.75 7.25 12.8 7.25" />"###
};
#[cfg(ChFileCode)]
const CH_FILE_CODE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="2.75 7.75,2.75 1.75,8.25 1.75,13.25 6.75,13.25 14.25,11.25 14.25" />
<polyline points="7.75 2.25 7.75 7.25 12.8 7.25" />
<path d="m6.75 10.8 2 1.75-2 1.75m-3-3.5-2 1.75 2 1.75" />"###
};
#[cfg(ChFileSymlink)]
const CH_FILE_SYMLINK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="2.75 7.75,2.75 1.75,8.25 1.75,13.25 6.75,13.25 14.25,9.25 14.25" />
<polyline points="7.75 2.25 7.75 7.25 12.75 7.25" />
<path d="m2.75 14.25 3.5-3.5m0 3v-3h-3" />"###
};
#[cfg(ChFiles)]
const CH_FILES: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="9.25 1.75,13.25 5.75,13.25 11.25,5.75 11.25,5.75 1.75" />
<polyline points="9.25 2.25,9.25 5.75,12.75 5.75" />
<polyline points="10.25 11.75,10.25 14.25,2.75 14.25,2.75 4.75,5.25 4.75" />"###
};
#[cfg(ChFilter)]
const CH_FILTER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="1.75 1.75,14.25 1.75,14.25 3.25,9.25 8.75,9.25 12.75,6.75 14.25,6.75 8.75,1.75 3.25" />"###
};
#[cfg(ChFlag)]
const CH_FLAG: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m1.75 14.25v-11s2-1.5 4-1.5 2.5 1.5 4.5 1.5 4-1.5 4-1.5v7s-2 1.5-4 1.5-2.5-1.5-4.5-1.5-4 1.5-4 1.5" />"###
};
#[cfg(ChFlame)]
const CH_FLAME: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m8.25 7.75c2 2 2.5-2.5 3.5-2s1.5 2 1.5 3.25c0 3.25-2.35 5.25-5.25 5.25s-5.25-2.5-5.25-6 3.5-7 5.5-7c0 0-2 4.5 0 6.5z" />"###
};
#[cfg(ChFloppyDisk)]
const CH_FLOPPY_DISK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="2.75 2.75,2.75 13.25,13.25 13.25,13.25 5.75,10.25 2.75" />
<polyline points="5.75 13.25,5.75 9.75,10.25 9.75,10.25 13.25" />"###
};
#[cfg(ChFolder)]
const CH_FOLDER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="1.75 2.75,1.75 13.25,14.25 13.25,14.25 4.75,8.25 4.75,6.75 2.75" />"###
};
#[cfg(ChFolderSymlink)]
const CH_FOLDER_SYMLINK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m1.75 13.25 3.5-3.5m0 3v-3h-3" />
<polyline points="8.25 13.25,14.25 13.25,14.25 4.75,8.25 4.75,6.75 2.75,1.75 2.75,1.75 6.75" />"###
};
#[cfg(ChFolders)]
const CH_FOLDERS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="4.75 2.25,4.75 10.25,14.25 10.25,14.25 3.75,9.25 3.75,7.75 2.25" />
<polyline points="4.75 5.25,1.75 5.25,1.75 13.25,11.25 13.25,11.25 10.25" />"###
};
#[cfg(ChForward)]
const CH_FORWARD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m1.75 13.25c.5-6 5.5-7.5 8-7v-3.5l4.5 5.25-4.5 5.25v-3.5c-2.5-0.5-6.5 0.5-8 3.5z" />"###
};
#[cfg(ChGamepad)]
const CH_GAMEPAD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3.25 3.75c-2 5-2 9 0 9.5s2.5-2 2.5-2h4.5s.5 2.5 2.5 2 2-4.5 0-9.5h-2l-1 1h-3.5l-1-1z" />"###
};
#[cfg(ChGem)]
const CH_GEM: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="4.75 2.75,11.25 2.75,14.25 6.25,8 13.25,1.75 6.25" />"###
};
#[cfg(ChGift)]
const CH_GIFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="3.5" width="12.5" y="4.75" x="1.75" />
<path d="m10.25 4.75h-2.25c0-2 .5-3 2.25-3 2 0 2 3 0 3zm-4.5 0h2.25c0-2-.5-3-2.25-3-2 0-2 3 0 3zm2.25 9v-8.75m-5.25 3.75v5.5h10.5v-5.5" />"###
};
#[cfg(ChGitBranch)]
const CH_GIT_BRANCH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="4.5" cy="3.5" r="1.75" />
<circle cx="11.5" cy="3.5" r="1.75" />
<circle cx="4.5" cy="12.5" r="1.75" />
<path d="m5.25 8.25c3 0 6 .5 6-2.5m-6.5 4.5v-4.5" />"###
};
#[cfg(ChGitCherryPick)]
const CH_GIT_CHERRY_PICK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cy="8" cx="5" r="2.25" />
<path d="m5 10.75v3.5m0-12.5v3.5" />
<path d="m11.75 8h1.5m-4.5-3.25h1.5l1 3.25-1 3.25h-1.5" />"###
};
#[cfg(ChGitCommit)]
const CH_GIT_COMMIT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="8" cy="8" r="2.25" />
<path d="m8 10.75v3.5m0-12.5v3.5" />"###
};
#[cfg(ChGitCompare)]
const CH_GIT_COMPARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12.5" cy="12.5" r="1.75" />
<circle cx="3.5" cy="3.5" r="1.75" />
<path d="m3.75 5.75v5c0 1 .5 1.5 1.5 1.5h2m-.5 2 1.5-2-1.5-2m5.5 0v-5c0-1-.5-1.5-1.5-1.5h-2m.5-2-1.5 2 1.5 2" />"###
};
#[cfg(ChGitFork)]
const CH_GIT_FORK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="8" cy="12.5" r="1.75" />
<circle cx="4.5" cy="3.5" r="1.75" />
<circle cx="11.5" cy="3.5" r="1.75" />
<path d="m8 8.75v1.5m-3.25-4.5c0 3.5 6.5 3.5 6.5 0" />"###
};
#[cfg(ChGitMerge)]
const CH_GIT_MERGE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="4.5" cy="3.5" r="1.75" />
<circle cx="4.5" cy="12.5" r="1.75" />
<circle cx="12.5" cy="8.5" r="1.75" />
<path d="m4.75 10.25v-4.5c1 2 2 3 5.5 3" />"###
};
#[cfg(ChGitRequest)]
const CH_GIT_REQUEST: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12.5" cy="12.5" r="1.75" />
<circle cx="3.5" cy="12.5" r="1.75" />
<circle cx="3.5" cy="3.5" r="1.75" />
<path d="m9.25 1.75-1.5 2 1.5 2m3 4.5v-5c0-1-.5-1.5-1.5-1.5h-2m-5 2v4.5" />"###
};
#[cfg(ChGitRequestCross)]
const CH_GIT_REQUEST_CROSS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cy="12.5" cx="12.5" r="1.75" />
<circle cy="12.5" cx="3.5" r="1.75" />
<circle cy="3.5" cx="3.5" r="1.75" />
<path d="m12.25 7.25v3m-8.5-4.5v4.5" />
<path d="m14.25 1.75-3.5 3.5m0-3.5 3.5 3.5" />"###
};
#[cfg(ChGitRequestDraft)]
const CH_GIT_REQUEST_DRAFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cy="12.5" cx="12.5" r="1.75" />
<circle cy="12.5" cx="3.5" r="1.75" />
<circle cy="3.5" cx="3.5" r="1.75" />
<path d="m7.75 2.75h.5m2.5 0h.5m1.5 2.5v-.5m0 3v.5m-9-2.5v4.5" />"###
};
#[cfg(ChGithub)]
const CH_GITHUB: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m5.75 14.25s-.5-2 .5-3c0 0-2 0-3.5-1.5s-1-4.5 0-5.5c-.5-1.5.5-2.5.5-2.5s1.5 0 2.5 1c1-.5 3.5-.5 4.5 0 1-1 2.5-1 2.5-1s1 1 .5 2.5c1 1 1.5 4 0 5.5s-3.5 1.5-3.5 1.5c1 1 .5 3 .5 3" />
<path d="m5.25 13.75c-1.5.5-3-.5-3.5-1" />"###
};
#[cfg(ChGitlab)]
const CH_GITLAB: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m8 14.25-6.25-4.5 2-8 2 5.5h4.5l2-5.5 2 8z" />"###
};
#[cfg(ChGlasses)]
const CH_GLASSES: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="4" cy="11" r="2.25" />
<circle cx="12" cy="11" r="2.25" />
<path d="m14.25 10.75c-1.5-6-2-6.5-3.5-7m-9 7c1.5-6 2-6.5 3.5-7m1 7c1-1 2.5-1 3.5 0" />"###
};
#[cfg(ChGlobe)]
const CH_GLOBE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cy="8" cx="8" r="6.25" />
<path d="m2 8.25h12" />
<path d="m8.25 14.2c2.75-3.2 2.75-9.2 0-12.4" />
<path d="m7.75 14.2c-2.75-3.2-2.75-9.2 0-12.4" />"###
};
#[cfg(ChGrabHorizontal)]
const CH_GRAB_HORIZONTAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cy="5.5" cx="2.5" r=".75" />
<circle cy="5.5" cx="8" r=".75" />
<circle cy="5.5" cx="13.5" r=".75" />
<circle cy="10.5" cx="2.5" r=".75" />
<circle cy="10.5" cx="8" r=".75" />
<circle cy="10.5" cx="13.5" r=".75" />"###
};
#[cfg(ChGrabVertical)]
const CH_GRAB_VERTICAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cy="2.5" cx="5.5" r=".75" />
<circle cy="8" cx="5.5" r=".75" />
<circle cy="13.5" cx="5.5" r=".75" />
<circle cy="2.5" cx="10.4957" r=".75" />
<circle cy="8" cx="10.4957" r=".75" />
<circle cy="13.5" cx="10.4957" r=".75" />"###
};
#[cfg(ChGraduateCap)]
const CH_GRADUATE_CAP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m14.25 9.25v-3.25l-6.25-3.25-6.25 3.25 6.25 3.25 3.25-1.5v3.5c0 1-1.5 2-3.25 2s-3.25-1-3.25-2v-3.5" />"###
};
#[cfg(ChHash)]
const CH_HASH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m2.75 10.25h9.5m-8.5-4.5h9.5m-2.5-4-1.5 12.5m-2.5-12.5-1.5 12.5" />"###
};
#[cfg(ChHeadphones)]
const CH_HEADPHONES: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m1.75 11.75c0-2.5 3.5-2 3.5-2v4.5s-3.5.5-3.5-2.5v-3.5c0-3 .5-6.5 6.25-6.5s6.25 3.5 6.25 6.5v3.5c0 3-3.5 2.5-3.5 2.5v-4.5s3.5-.5 3.5 2" />"###
};
#[cfg(ChHeart)]
const CH_HEART: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3.25 9.75c3 3.5 4.75 4.5 4.75 4.5s1.75-1 4.75-4.5 1-7-1.5-7-3.25 3-3.25 3-.75-3-3.25-3-4.5 3.5-1.5 7z" />"###
};
#[cfg(ChHelp)]
const CH_HELP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cy="8" cx="8" r="6.25" />
<path d="m5.75 6.75c0-1 1-2 2.25-2s2.25 1.0335 2.25 2c0 1.5-1.5 1.5-2.25 2m0 2.5v0" />"###
};
#[cfg(ChHexagon)]
const CH_HEXAGON: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="1.75 4.75,8 1.25,14.25 4.75,14.25 11.25,8 14.75,1.75 11.25" />"###
};
#[cfg(ChHome)]
const CH_HOME: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3.75 5.75v7.5h8.5v-7.5m-10.5 1.5 6.25-5.5l6.25 5.5" />"###
};
#[cfg(ChHourglass)]
const CH_HOURGLASS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m11.75 13.75c0-5-2-4-2-5.75s2-0.75 2-5.75m-7.5 11.5c0-5 2-4 2-5.75s-2-.75-2-5.75m-1.5-.5h10.5m-10.5 12.5h10.5" />"###
};
#[cfg(ChId)]
const CH_ID: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="10.5" width="12.5" y="2.75" x="1.75" />
<circle cy="7.5" cx="8" r="2.25" />
<path d="m4.75 12.75c0-1 .75-3 3.25-3s3.25 2 3.25 3" />"###
};
#[cfg(ChImage)]
const CH_IMAGE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="1.75" y="2.75" width="12.5" height="10.5" />
<path d="m3.75 13.2 6.5-5.5 4 3" />
<circle fill="currentColor" cx="5.25" cy="6.25" r=".5" />"###
};
#[cfg(ChInbox)]
const CH_INBOX: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="1.75 13.25,14.25 13.25,14.25 8.25,11.75 2.75,4.25 2.75,1.75 8.25" />
<path d="m2.25 8.75h3l1 1.5h3.5l1-1.5h3" />"###
};
#[cfg(ChInfinity)]
const CH_INFINITY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m5 5c2.5 1 3.5 5 6 6s3.25-1.25 3.25-3-.75-4-3.25-3-3.5 5-6 6-3.25-1.25-3.25-3 .75-4 3.25-3z" />"###
};
#[cfg(ChInfo)]
const CH_INFO: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cy="8" cx="8" r="6.25" />
<path d="m8 5.25v0m0 6v-3.5" />"###
};
#[cfg(ChKey)]
const CH_KEY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m10 1.75c-2.34721 0-4.25 1.90279-4.25 4.25.00023.37267.04949.74369.14648 1.10352l-4.14648 4.14648v3h3v-1.5h1.5v-1.5h1.5l1.15039-1.15039c.35839.0980.72808.1486 1.09961.1504 2.3472 0 4.25-1.90279 4.25-4.25s-1.9028-4.25-4.25-4.25z" />
<circle cx="10.75" cy="5.25" r="0.5" fill="currentColor" />"###
};
#[cfg(ChLaptop)]
const CH_LAPTOP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="7.5" width="10.5" y="2.75" x="2.75" />
<path d="m2.75 10.25-1 3h12.5l-1-3" />"###
};
#[cfg(ChLayoutColumns)]
const CH_LAYOUT_COLUMNS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="10.5" width="12.5" y="2.75" x="1.75" />
<line x1="8" y1="3.25" x2="8" y2="12.75" />"###
};
#[cfg(ChLayoutDashboard)]
const CH_LAYOUT_DASHBOARD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="10.5" width="12.5" y="2.75" x="1.75" />
<path d="m8.25 6.75h5.5m-11.5 2.5h5.5m.25-6v9.5" />"###
};
#[cfg(ChLayoutGrid)]
const CH_LAYOUT_GRID: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="10.5" width="12.5" y="2.75" x="1.75" />
<path d="m2 8h12m-3.75-4.75v9.5m-4.5-9.5v9.5" />"###
};
#[cfg(ChLayoutList)]
const CH_LAYOUT_LIST: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="10.5" width="12.5" y="2.75" x="1.75" />
<path d="m5.25 3.25v9.5m-3-6.5h11.5m-11.5 3.5h11.5" />"###
};
#[cfg(ChLayoutRows)]
const CH_LAYOUT_ROWS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="10.5" width="12.5" y="2.75" x="1.75" />
<line x1="2" y1="8" x2="14" y2="8" />"###
};
#[cfg(ChLayoutSidebar)]
const CH_LAYOUT_SIDEBAR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="10.5" width="12.5" y="2.75" x="1.75" />
<path d="m6.25 3v9.5" />"###
};
#[cfg(ChLayoutStackH)]
const CH_LAYOUT_STACK_H: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="10.5" width="12.5" y="2.75" x="1.75" />
<line x1="2" y1="8" x2="14" y2="8" />
<line x1="8" y1="8" x2="8" y2="12.75" />"###
};
#[cfg(ChLayoutStackV)]
const CH_LAYOUT_STACK_V: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="10.5" width="12.5" y="2.75" x="1.75" />
<line x1="8" y1="3.25" x2="8" y2="12.75" />
<line x1="8" y1="8" x2="14" y2="8" />"###
};
#[cfg(ChLightbulb)]
const CH_LIGHTBULB: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m6.75 14.25h2.5m-1.25-12.5c-2.75 0-4.25 2-4.25 4s2 2.5 2 4.5v1h4.5v-1c0-2 2-2.5 2-4.5s-1.5-4-4.25-4z" />"###
};
#[cfg(ChLightningBolt)]
const CH_LIGHTNING_BOLT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="9.25 1.75,2.75 9.25,7.25 9.75,6.75 14.25,13.25 6.75,8.75 6.25" />"###
};
#[cfg(ChLink)]
const CH_LINK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m9.75 4.75c3 0 4.5 1.5 4.5 3.25s-1.5 3.25-4.5 3.25m-4-3.25h4.5m-4-3.25c-3 0-4.5 1.5-4.5 3.25s1.5 3.25 4.5 3.25" />"###
};
#[cfg(ChLinkExternal)]
const CH_LINK_EXTERNAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="8.25 2.75,2.75 2.75,2.75 13.25,13.25 13.25,13.25 7.75" />
<path d="m13.25 2.75-5.5 5.5m3-6.5h3.5v3.5" />"###
};
#[cfg(ChLinkSlash)]
const CH_LINK_SLASH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m10.75 1.75-5.5 12.5m4.5-9.5c3 0 4.5 1.5 4.5 3.25s-1.5 3.25-4.5 3.25m-3.5-6.5c-3 0-4.5 1.5-4.5 3.25s1.5 3.25 4.5 3.25" />"###
};
#[cfg(ChMail)]
const CH_MAIL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="9.5" width="12.5" y="3.75" x="1.75" />
<path d="m2.25 4.25 5.75 5 5.75-5" />"###
};
#[cfg(ChMap)]
const CH_MAP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m10.25 5.25v8.5m-4.5-10.5v8.5m-4 2.5v-9.5l4-2 4.5 2 4-2v9.5l-4 2-4.5-2z" />"###
};
#[cfg(ChMapPin)]
const CH_MAP_PIN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m13.25 7c0 3.75-5.25 7.25-5.25 7.25s-5.25-3.5-5.25-7.25c0-2.89949 2.35051-5.25 5.25-5.25 2.8995 0 5.25 2.35051 5.25 5.25z" />
<circle cx="8" cy="7" r="1.25" fill="currentColor" />"###
};
#[cfg(ChMediaBack)]
const CH_MEDIA_BACK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="13.25 13.25,4.75 8,13.25 2.75" />
<line x1="1.75" y1="3.75" x2="1.75" y2="12.25" />"###
};
#[cfg(ChMediaEject)]
const CH_MEDIA_EJECT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="2.75 11.25,13.25 11.25,8 2.75" />
<line x1="13.25" y1="14.25" x2="2.75" y2="14.25" />"###
};
#[cfg(ChMediaFastForward)]
const CH_MEDIA_FAST_FORWARD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="8.25 3.75,8.25 12.25,14.25 8" />
<polygon points="1.75 3.75,1.75 12.25,7.75 8" />"###
};
#[cfg(ChMediaPause)]
const CH_MEDIA_PAUSE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="10.5" width="3.5" y="2.75" x="2.75" />
<rect height="10.5" width="3.5" y="2.75" x="9.75" />"###
};
#[cfg(ChMediaPlay)]
const CH_MEDIA_PLAY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="2.75 2.75,2.75 13.25,12.25 8" />"###
};
#[cfg(ChMediaRewind)]
const CH_MEDIA_REWIND: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="7.75 3.75,7.75 12.25,1.75 8" />
<polygon points="14.25 3.75,14.25 12.25,8.25 8" />"###
};
#[cfg(ChMediaSkip)]
const CH_MEDIA_SKIP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="2.75 13.25,11.25 8,2.75 2.75" />
<line x1="14.25" y1="3.75" x2="14.25" y2="12.25" />"###
};
#[cfg(ChMenuHamburger)]
const CH_MENU_HAMBURGER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m2.75 12.25h10.5m-10.5-4h10.5m-10.5-4h10.5" />"###
};
#[cfg(ChMenuKebab)]
const CH_MENU_KEBAB: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="8" cy="2.5" r=".75" />
<circle cx="8" cy="8" r=".75" />
<circle cx="8" cy="13.5" r=".75" />"###
};
#[cfg(ChMenuMeatball)]
const CH_MENU_MEATBALL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="2.5" cy="8" r=".75" />
<circle cx="8" cy="8" r=".75" />
<circle cx="13.5" cy="8" r=".75" />"###
};
#[cfg(ChMessage)]
const CH_MESSAGE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="1.75 14.25,1.75 2.75,14.25 2.75,14.25 11.25,5.75 11.25" />"###
};
#[cfg(ChMessages)]
const CH_MESSAGES: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="14.25 14.25,14.25 5.25,4.75 5.25,4.75 11.25,10.75 11.25" />
<path d="m4.75 7.25-3 3v-8.5h10v3" />"###
};
#[cfg(ChMicrophone)]
const CH_MICROPHONE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m8 1.75c-2.25 0-2.25 2-2.25 3v1.5c0 1 0 3 2.25 3s2.25-2 2.25-3v-1.5c0-1 0-3-2.25-3z" />
<path d="m8 13v1.25m-5.25-6.5s0 4.5 5.25 4.50785c5.25.0079 5.25-4.5078 5.25-4.5078" />"###
};
#[cfg(ChMinus)]
const CH_MINUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m13.25 7.75h-10.5" />"###
};
#[cfg(ChMobile)]
const CH_MOBILE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="12.5" width="8.5" y="1.75" x="3.75" />
<path d="m8.25 11.75h-.5" />"###
};
#[cfg(ChMonitor)]
const CH_MONITOR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="9.5" width="12.5" y="1.75" x="1.75" />
<path d="m4.75 14.25h6.5m-3.25-2.5v2.5" />"###
};
#[cfg(ChMonitorArrow)]
const CH_MONITOR_ARROW: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m14.2 7.75v3.5h-12.5v-9.5h6.5" />
<path d="m4.75 14.2h6.5m-3.25-2.5v2.5" />
<path d="m9.75 6.25 4.5-4.5m-3.5-0.5h4v4" />"###
};
#[cfg(ChMonitorCross)]
const CH_MONITOR_CROSS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m14.2 7.75v3.5h-12.5v-9.5h6.5" />
<path d="m4.75 14.2h6.5m-3.25-2.5v2.5" />
<path d="m14.2 1.75-3.5 3.5m0-3.5 3.5 3.5" />"###
};
#[cfg(ChMoon)]
const CH_MOON: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m1.75 8c0 3.45 2.8 6.25 6.25 6.25 3.41-.0027 6.25-3 6.25-6-1 .5-4 1.5-6-.5s-1-5-.5-6c-3 0-6 2.84-6 6.25z" />"###
};
#[cfg(ChMove)]
const CH_MOVE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m12.25 10.25 2-2.25-2-2.25m-2-2-2.25-2-2.25 2m-2 2-2 2.25 2 2.25m2 2 2.25 2 2.25-2m-2.25-10.5v12m5.75-5.75h-12" />"###
};
#[cfg(ChMusic)]
const CH_MUSIC: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="4" cy="12" r="2.25" />
<circle cx="12" cy="11" r="2.25" />
<polyline points="6.25 12,6.25 2.75,14.25 1.75,14.25 11" />"###
};
#[cfg(ChNewspaper)]
const CH_NEWSPAPER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m11.2 14.2h0.5c1.5 0 2.5-1 2.5-2.5v-6h-3m-9.5-4h9.5v12.5h-7c-1.5 0-2.5-1-2.5-2.5v-9.44z" />
<path d="m4.75 11.2h3.5" />
<rect x="4.75" y="4.75" width="3.5" height="3.5" />"###
};
#[cfg(ChNorthStar)]
const CH_NORTH_STAR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m13.75 7.75h-12" />
<path d="m7.75 1.75v12" />
<path d="m4.25 11.25 7-7" />
<path d="m11.25 11.25-7-7" />"###
};
#[cfg(ChNotes)]
const CH_NOTES: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="12.5" width="10.5" y="1.75" x="2.75" />
<path d="m5.75 7.75h4.5m-4.5 3h2.5m-2.5-6h4.5" />"###
};
#[cfg(ChNotesCross)]
const CH_NOTES_CROSS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="8.25 14.25,2.75 14.25,2.75 1.75,13.25 1.75,13.25 8.25" />
<path d="m14.25 10.75-3.5 3.5m-5-6.5h4.5m-4.5 3h1.5m-1.5-6h4.5m.5 6 3.5 3.5" />"###
};
#[cfg(ChNotesTick)]
const CH_NOTES_TICK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="7.25 14.25,2.75 14.25,2.75 1.75,13.25 1.75,13.25 9.25" />
<path d="m9.75 12.75 1.5 1.5 3-2.5m-8.5-4h4.5m-4.5 3h1.5m-1.5-6h4.5" />"###
};
#[cfg(ChNut)]
const CH_NUT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="8 1.25 14.25 4.75 14.25 11.25 8 14.75 1.75 11.25 1.75 4.75" />
<circle cx="8" cy="8" r="2.25" />"###
};
#[cfg(ChOctagon)]
const CH_OCTAGON: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="5.25 1.75,10.75 1.75,14.25 5.25,14.25 10.75,10.75 14.25,5.25 14.25,1.75 10.75,1.75 5.25" />"###
};
#[cfg(ChOctagonWarning)]
const CH_OCTAGON_WARNING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="5.25 1.75,10.75 1.75,14.25 5.25,14.25 10.75,10.75 14.25,5.25 14.25,1.75 10.75,1.75 5.25" />
<path d="m8 11.25v0m0-6.5v3.5" />"###
};
#[cfg(ChOrganisation)]
const CH_ORGANISATION: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="6.75" y="1.75" width="3.5" height="3.5" />
<rect x="10.75" y="10.75" width="3.5" height="3.5" />
<rect x="2.75" y="10.75" width="3.5" height="3.5" />
<path d="m8.5 5.75v2m-3.75 2.5v-2h7.5v2" />"###
};
#[cfg(ChPackage)]
const CH_PACKAGE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="1.75 5.75,1.75 14.25,1.75 14.25,14.25 14.25,14.25 5.75,10.75 1.75,5.25 1.75" />
<path d="m8 1.75v3.5m-5.75.5h11.5" />"###
};
#[cfg(ChPadlock)]
const CH_PADLOCK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="7.5" width="10.5" y="6.75" x="2.75" />
<path d="m4.75 6.25s-1-4.5 3.25-4.5 3.25 4.5 3.25 4.5" />"###
};
#[cfg(ChPaperPlane)]
const CH_PAPER_PLANE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="1.75 1.75,14.25 7.75,1.75 14.25,3.25 7.75" />
<line x1="3.75" y1="7.75" x2="7.25" y2="7.75" />"###
};
#[cfg(ChPaperclip)]
const CH_PAPERCLIP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m8.25 10.25v-7s0-1.5-1.75-1.5-1.75 1.5-1.75 1.5v8s0 3 3.25 3 3.25-3 3.25-3v-4.5" />"###
};
#[cfg(ChPencil)]
const CH_PENCIL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="1.75 11.25,1.75 14.25,4.75 14.25,14.25 4.75,11.25 1.75" />
<line x1="8.75" y1="4.75" x2="11.25" y2="7.25" />"###
};
#[cfg(ChPeople)]
const CH_PEOPLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="5" cy="9" r="2.25" />
<circle cx="11" cy="4" r="2.25" />
<path d="m7.75 9.25c0-1 .75-3 3.25-3s3.25 2 3.25 3m-12.5 5c0-1 .75-3 3.25-3s3.25 2 3.25 3" />"###
};
#[cfg(ChPerson)]
const CH_PERSON: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="8" cy="6" r="3.25" />
<path d="m2.75 14.25c0-2.5 2-5 5.25-5s5.25 2.5 5.25 5" />"###
};
#[cfg(ChPhone)]
const CH_PHONE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m1.75 1.75c0 8.5 4 12.5 12.5 12.5v-4l-3.5-1-1 1.5c-2 0-4.5-2.5-4.5-4.5l1.5-1-1-3.5z" />"###
};
#[cfg(ChPhoneCall)]
const CH_PHONE_CALL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m1.75 1.75c0 8.5 4 12.5 12.5 12.5v-4l-3.5-1-1 1.5c-2 0-4.5-2.5-4.5-4.5l1.5-1-1-3.5z" />
<path d="m9.75 1.75c2.5 0 4.5 2 4.5 4.5m-4.5-2c1 0 2 1 2 2" />"###
};
#[cfg(ChPhoneCross)]
const CH_PHONE_CROSS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m1.75 1.75c0 8.5 4 12.5 12.5 12.5v-4l-3.5-1-1 1.5c-2 0-4.5-2.5-4.5-4.5l1.5-1-1-3.5z" />
<path d="m13.25 2.75-3.5 3.5m0-3.5 3.5 3.5" />"###
};
#[cfg(ChPhoneForward)]
const CH_PHONE_FORWARD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m1.75 1.75c0 8.5 4 12.5 12.5 12.5v-4l-3.5-1-1 1.5c-2 0-4.5-2.5-4.5-4.5l1.5-1-1-3.5z" />
<path d="m9.75 4.75h4.5m-2 2 2-2-2-2" />"###
};
#[cfg(ChPhoneIncoming)]
const CH_PHONE_INCOMING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m1.75 1.75c0 8.5 4 12.5 12.5 12.5v-4l-3.5-1-1 1.5c-2 0-4.5-2.5-4.5-4.5l1.5-1-1-3.5z" />
<path d="m13.25 2.75-3.5 3.5m0-3v3h3" />"###
};
#[cfg(ChPhoneOutgoing)]
const CH_PHONE_OUTGOING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m1.75 1.75c0 8.5 4 12.5 12.5 12.5v-4l-3.5-1-1 1.5c-2 0-4.5-2.5-4.5-4.5l1.5-1-1-3.5z" />
<path d="m9.75 6.25 3.5-3.5m0 3v-3h-3" />"###
};
#[cfg(ChPin)]
const CH_PIN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m10.25 10.25 4 4m-12.5-7.5 5-5s1 2 2 3 4.5 2 4.5 2l-6.5 6.5s-1-3.5-2-4.5-3-2-3-2z" />"###
};
#[cfg(ChPlantPot)]
const CH_PLANT_POT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m8.75 6.75c0 1.25-.75 3-.75 3m.25-2.5s.75-2-1-3.5-4.5-1-4.5-1 0 2 1.5 3.5 4 1 4 1zm.5-1s-.75-2 1-3.5 4.5-1 4.5-1 0 2-1.5 3.5-4 1-4 1z" />
<path d="m4.75 9.75h6.5s.5 4.5-3.25 4.5-3.25-4.5-3.25-4.5z" />"###
};
#[cfg(ChPlus)]
const CH_PLUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m12.75 7.75h-10m5-5v10" />"###
};
#[cfg(ChPower)]
const CH_POWER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m8 1.75v6.5m4.25-5s2 1.29822 2 4.75-2.79822 6.25-6.25 6.25-6.25-2.79822-6.25-6.25 2-4.75 2-4.75" />"###
};
#[cfg(ChPrinter)]
const CH_PRINTER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="4.5" width="6.5" y="9.75" x="4.75" />
<path d="m4.75 4.25v-2.5h6.5v2.5m-7 8h-2.5v-7.5h12.5v7.5h-2.5" />"###
};
#[cfg(ChPulse)]
const CH_PULSE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="1.75 8.25, 4.25 8.25, 6.25 3.75, 9.75 12.25, 11.75 8.25, 14.25 8.25" />"###
};
#[cfg(ChQuote)]
const CH_QUOTE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m6.25 3.75h-4.5v5.5c0 3.5 2.5 4.5 4.5 4-1.5-1.5-1.5-2.5-1.5-4h1.5z" />
<path d="m13.25 3.75h-4.5v5.5c0 3.5 2.5 4.5 4.5 4-1.5-1.5-1.5-2.5-1.5-4h1.5z" />"###
};
#[cfg(ChRefresh)]
const CH_REFRESH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m4.75 10.75h-3m12.5-2c0 3-2.79822 5.5-6.25 5.5-3.75 0-6.25-3.5-6.25-3.5v3.5m9.5-9h3m-12.5 2c0-3 2.79822-5.5 6.25-5.5 3.75 0 6.25 3.5 6.25 3.5v-3.5" />"###
};
#[cfg(ChReply)]
const CH_REPLY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m14.25 13.25c-.5-6-5.5-7.5-8-7v-3.5l-4.5 5.25 4.5 5.25v-3.5c2.50001-0.5 6.5 0.5 8 3.5z" />"###
};
#[cfg(ChRobot)]
const CH_ROBOT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="7.5" width="12.5" y="5.75" x="1.75" />
<path d="m10.75 8.75v1.5m-5.5-1.5v1.5m-.5-7.5 3.25 3 3.25-3" />"###
};
#[cfg(ChRocket)]
const CH_ROCKET: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m4.25 9.75-2-.5s0-1.5.5-3 4-1.5 4-1.5m-.50 7l.5 2s1.5 0 3-.5 1.5-4 1.5-4m-7 .5 2 2s5-2 6.5-4.5 1.5-5.5 1.5-5.5-3 0-5.5 1.5-4.5 6.5-4.5 6.5z" />
<path d="m1.75 14.25 2-1-1-1z" fill="currentColor" />
<circle cx="10.25" cy="5.75" r=".5" fill="currentColor" />"###
};
#[cfg(ChRotateAntiClockwise)]
const CH_ROTATE_ANTI_CLOCKWISE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m4.75 5.25h-3m0 3.5c0 2.5 2.79822 5.5 6.25 5.5s6.25-2.79822 6.25-6.25-2.79822-6.25-6.25-6.25c-3.75 0-6.25 3.5-6.25 3.5v-3.5" />"###
};
#[cfg(ChRotateClockwise)]
const CH_ROTATE_CLOCKWISE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m11.25 5.25h3m0 3.5c0 2.5-2.79822 5.5-6.25 5.5s-6.25-2.7982-6.25-6.25c0-3.45178 2.79822-6.25 6.25-6.25 3.75 0 6.25 3.5 6.25 3.5v-3.5" />"###
};
#[cfg(ChScales)]
const CH_SCALES: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m1.75 3.75c1 1 2.5 1.5 4 0h4.5c1.5 1.5 3 1 4 0m-6.25-2v12m-3.25.5h6.5" />
<path d="m12.75 4.75-2 5c.5 1 3.5 1 4 0zm-9.5 0-2 5c.5 1 3.5 1 4 0z" />"###
};
#[cfg(ChScreenMaximise)]
const CH_SCREEN_MAXIMISE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m5.25 14.25h-3.5v-3.5m12.5 0v3.5h-3.5m0-12.5h3.5v3.5m-12.5 0v-3.5h3.5" />"###
};
#[cfg(ChScreenMinimise)]
const CH_SCREEN_MINIMISE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m1.75 10.75h3.5v3.5m5.5 0v-3.5h3.5m0-5.5h-3.5v-3.5m-5.5 0v3.5h-3.5" />"###
};
#[cfg(ChSearch)]
const CH_SEARCH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m11.25 11.25 3 3" />
<circle cx="7.5" cy="7.5" r="4.75" />"###
};
#[cfg(ChServer)]
const CH_SERVER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="1.75" y="3.25" width="12.5" height="10" />
<line x1="2.25" y1="8.25" x2="13.75" y2="8.25" />
<path d="m4.75 10.75v0m0-5v0m6.5 0h-3m3 5h-3" />"###
};
#[cfg(ChShare)]
const CH_SHARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="4" cy="8" r="2.25" />
<circle cx="12" cy="12" r="2.25" />
<circle cx="12" cy="4" r="2.25" />
<path d="m6 9 4 2m-4-4 4-2" />"###
};
#[cfg(ChShield)]
const CH_SHIELD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m8 1.75 5.25 2v5c0 2.25-2 4.5-5.25 5.5-3.25-1-5.25-3-5.25-5.5v-5z" />"###
};
#[cfg(ChShieldCross)]
const CH_SHIELD_CROSS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m8 1.75 5.25 2v5c0 2.25-2 4.5-5.25 5.5-3.25-1-5.25-3-5.25-5.5v-5z" />
<path d="m9.75 5.75-3.5 3.5m0-3.5 3.5 3.5" />"###
};
#[cfg(ChShieldKeyhole)]
const CH_SHIELD_KEYHOLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m8 1.75 5.25 2v5c0 2.25-2 4.5-5.25 5.5-3.25-1-5.25-3-5.25-5.5v-5z" />
<path d="m8 7.25v3" />
<circle cx="8" cy="6.5" r=".75" fill="currentColor" />"###
};
#[cfg(ChShieldTick)]
const CH_SHIELD_TICK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m8 1.75 5.25 2v5c0 2.25-2 4.5-5.25 5.5-3.25-1-5.25-3-5.25-5.5v-5z" />
<polyline points="5.75 7.75,7.25 9.25,10.25 5.75" />"###
};
#[cfg(ChShieldWarning)]
const CH_SHIELD_WARNING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m8 1.75 5.25 2v5c0 2.25-2 4.5-5.25 5.5-3.25-1-5.25-3-5.25-5.5v-5z" />
<path d="m8 10.75v0m0-5.5v3" />"###
};
#[cfg(ChShoppingBag)]
const CH_SHOPPING_BAG: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="9.5" width="10.5" y="4.75" x="2.75" />
<path d="m5.75 7.75c0 1.5 1 2.5 2.25 2.5s2.25-1 2.25-2.5m-7.5-3 1.5-3h7.5l1.5 3" />"###
};
#[cfg(ChSignIn)]
const CH_SIGN_IN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m5.25 2.25h-3.5v12h3.5m4-9.5-3.5 3.5 3.5 3.5m5-3.5h-8.5" />"###
};
#[cfg(ChSignOut)]
const CH_SIGN_OUT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m5.25 2.25h-3.5v12h3.5m5.5-9.5 3.5 3.5-3.5 3.5m-5-3.5h8.5" />"###
};
#[cfg(ChSignpost)]
const CH_SIGNPOST: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="1.75 9.25,12.25 9.25,14.25 7.00,12.25 4.75,1.75 4.75" />
<path d="m7.25 9.75v4.5m0-12.5v2.5" />"###
};
#[cfg(ChSkull)]
const CH_SKULL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m1.75 11.25h3v3h6.5v-3h3s1-9.5-6.25-9.5-6.25 9.5-6.25 9.5z" />
<circle cx="5.25" cy="7.75" r=".5" fill="currentColor" />
<circle cx="10.75" cy="7.75" r=".5" fill="currentColor" />"###
};
#[cfg(ChSnowflake)]
const CH_SNOWFLAKE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m13.75 7.75h-12" />
<path d="m7.75 1.75v12" />
<path d="m5.25 12.75 2.5-2.5 2.5 2.5" />
<path d="m2.75 5.25 2.5 2.5-2.5 2.5" />
<path d="m10.25 2.75-2.5 2.5-2.5-2.5" />
<path d="m12.75 10.25-2.5-2.5 2.5-2.5" />"###
};
#[cfg(ChSoundDown)]
const CH_SOUND_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="1.75 5.75 1.75 10.25 4.25 10.25 8.25 13.25 8.25 2.75 4.25 5.75" />
<path d="m10.75 6.25s1 .5 1 1.75-1 1.75-1 1.75" />"###
};
#[cfg(ChSoundMute)]
const CH_SOUND_MUTE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="1.75 5.75 1.75 10.25 4.25 10.25 8.25 13.25 8.25 2.75 4.25 5.75" />
<path d="m14.25 5.75-3.5 4.5m0-4.5 3.5 4.5" />"###
};
#[cfg(ChSoundUp)]
const CH_SOUND_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="1.75 5.75,1.75 10.25,4.25 10.25,8.25 13.25,8.25 2.75,4.25 5.75" />
<path d="m10.75 6.25s1 .5 1 1.75-1 1.75-1 1.75m1-6.5c2 1 3 2.5 3 4.75s-1 3.75-3 4.75" />"###
};
#[cfg(ChSpeaker)]
const CH_SPEAKER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="12.5" width="9.5" y="1.75" x="3.25" />
<path d="m8.25 4.25h-.5" />
<circle cx="8" cy="9.5" r="2.25" />"###
};
#[cfg(ChSquare)]
const CH_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="10.5" width="10.5" y="2.75" x="2.75" />"###
};
#[cfg(ChSquareCross)]
const CH_SQUARE_CROSS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m10.25 5.75-4.5 4.5m0-4.5 4.5 4.5" />
<rect height="10.5" width="10.5" y="2.75" x="2.75" />"###
};
#[cfg(ChSquareTick)]
const CH_SQUARE_TICK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="10.25 2.75,2.75 2.75,2.75 13.25,13.25 13.25,13.25 9.75" />
<polyline points="5.75 7.75,8.25 10.25,14.25 3.75" />"###
};
#[cfg(ChStack)]
const CH_STACK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m1.75 11 6.25 3.25 6.25-3.25m-12.5-3 6.25 3.25 6.25-3.25m-6.25-6.25-6.25 3.25 6.25 3.25 6.25-3.25z" />"###
};
#[cfg(ChStackPop)]
const CH_STACK_POP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m4.25 6.75-2.5 1.25 6.25 3.25 6.25-3.25-2.5-1.25m-10 4.25 6.25 3.25 6.25-3.25" />
<path d="m8 8.25v-6.5m-2.25 2 2.25-2 2.25 2" />"###
};
#[cfg(ChStackPush)]
const CH_STACK_PUSH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3.25 7.25-1.5.75 6.25 3.25 6.25-3.25-1.5-.75m-11 3.75 6.25 3.25 6.25-3.25" />
<path d="m8 8.25v-6.5m-2.25 4.5 2.25 2 2.25-2" />"###
};
#[cfg(ChStar)]
const CH_STAR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="8 1.75,5.75 5.75,1.75 6.25,4.75 9.75,3.75 14.25,8.00 12.25,12.25 14.25,11.25 9.75,14.25 6.25,10.25 5.75" />"###
};
#[cfg(ChStickyNote)]
const CH_STICKY_NOTE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="9.25 13.25,2.75 13.25,2.75 2.75,13.25 2.75,13.25 9.25" />
<polyline points="8.75 13.25,8.75 8.75,13.25 8.75" />"###
};
#[cfg(ChSun)]
const CH_SUN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cy="8" cx="8" r="3.25" />
<path d="m2.75 13.25.5-.5m9.5 0 .5.5m-.5-10 .5-.5m-10 .5-.5-.5m-.50 5.25h-1m13.5 0h-1m-5.75 5.75v1m0-13.5v1" />"###
};
#[cfg(ChSwapHorizontal)]
const CH_SWAP_HORIZONTAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m5.75 8.25-3 3 3 3m7.5-3h-10.5m7.5-9.5l3 3-3 3m-7.5-3h10.5" />"###
};
#[cfg(ChSwapVertical)]
const CH_SWAP_VERTICAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m7.75 5.75-3-3-3 3m3 7.5v-10.5m9.5 7.5-3 3-3-3m3-7.5v10.5" />"###
};
#[cfg(ChSword)]
const CH_SWORD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m2.75 9.25 1.5 2.5 2 1.5m-4.5 0 1 1m1.5-2.5-1.5 1.5m3-1 8.5-8.5v-2h-2l-8.5 8.5" />"###
};
#[cfg(ChSwords)]
const CH_SWORDS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m2.75 9.25 1.5 2.5 2 1.5m-4.5 0 1 1m1.5-2.5-1.5 1.5m3-1 8.5-8.5v-2h-2l-8.5 8.5" />
<path d="m10.25 12.25-2.25-2.25m2-2 2.25 2.25m1-1-1.5 2.5-2 1.5m4.5 0-1 1m-1.5-2.5 1.5 1.5m-7.25-5.25-4.25-4.25v-2h2l4.25 4.25" />"###
};
#[cfg(ChTablet)]
const CH_TABLET: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="12.5" width="10.5" y="1.75" x="2.75" />
<path d="m8.25 11.75h-.5" />"###
};
#[cfg(ChTag)]
const CH_TAG: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="7.25 14.25,1.75 8.75,8.75 1.75,14.25 1.75,14.25 7.25" />
<circle cx="11" cy="5" r=".5" fill="currentColor" />"###
};
#[cfg(ChTelescope)]
const CH_TELESCOPE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m4.75 5.75 1 2.5m3.5-4.5 1.5 3.5m-9 0 1 2.5 11.5-3.5-2-4.5z" />
<path d="m7.75 11.2v3m-3-0.5 2.25-2.5 1.75-0.5 2.5 3" />"###
};
#[cfg(ChTent)]
const CH_TENT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M 5.25,14.25 8,10 l 2.75,4.25" />
<path d="m9.75 1.75-8 12.5h12.5l-8-12.5" />"###
};
#[cfg(ChTerminal)]
const CH_TERMINAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="10.5" width="12.5" y="2.75" x="1.75" />
<path d="m8.75 10.25h2.5m-6.5-4.5 2.5 2.25-2.5 2.25" />"###
};
#[cfg(ChThumbDown)]
const CH_THUMB_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m5.25 10.25c1.5 0 3 4 4.5 4v-4h4.5s-.5-7.5-3.5-7.5h-5.5z" />
<path d="m5.25 10.25h-3.5v-7.5h3.5" />"###
};
#[cfg(ChThumbUp)]
const CH_THUMB_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m5.25 5.75c1.5 0 3-4 4.5-4v4h4.5s-.5 7.5-3.5 7.5h-5.5z" />
<path d="m5.25 5.75h-3.5v7.5h3.5" />"###
};
#[cfg(ChTick)]
const CH_TICK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="2.75 8.75,6.25 12.25,13.25 4.75" />"###
};
#[cfg(ChTickDouble)]
const CH_TICK_DOUBLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m1.75 9.75 2.5 2.5m3.5-4 2.5-2.5m-4.5 4 2.5 2.5 6-6.5" />"###
};
#[cfg(ChTicket)]
const CH_TICKET: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m1.75 3.75h12.5v3s-2 0-2 1.75 2 1.75 2 1.75v3h-12.5v-3s2 0 2-1.75-2-1.75-2-1.75z" />
<path d="m8.75 11.75v1.5m0-9.5v1.5m0 2.5v1.5" />"###
};
#[cfg(ChTreeFir)]
const CH_TREE_FIR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m8 1.75-4.25 5.5h2.5l-3.5 4h4v3h2.5v-3h4l-3.5-4h2.5z" />"###
};
#[cfg(ChTriangle)]
const CH_TRIANGLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="8 2.75,1.75 14.25,14.25 14.25" />"###
};
#[cfg(ChTrophy)]
const CH_TROPHY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect height="3.5" width="6.5" y="10.75" x="4.75" />
<path d="m8 8.75v2m-3.25-9c-1.5 0-3 .5-3 2.25s1.5 2.25 3 2.25m6.5-4.5c1.5 0 3 .5 3 2.25s-1.5 2.25-3 2.25m-6.5-4.5h6.5v3.5c0 1.5-1 3-3.25 3s-3.25-1.5-3.25-3z" />"###
};
#[cfg(ChUmbrella)]
const CH_UMBRELLA: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m1.75 8.25s.5-6.5 6.25-6.5 6.25 6.5 6.25 6.5z" />
<path d="m7.75 8.75v4s0 1.5 1.5 1.5 1.5-1.5 1.5-1.5" />"###
};
#[cfg(ChUpload)]
const CH_UPLOAD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3.75 2.75h9m-8.5 6.5 4-3.5 4 3.5m-4 5v-8.5" />"###
};
#[cfg(ChWifi)]
const CH_WIFI: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m1.75 4.75 6.25 8.5 6.25-8.5c-3.25-2.75-9.25-2.75-12.5 0z" />"###
};
#[cfg(ChWifiFair)]
const CH_WIFI_FAIR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m1.75 4.75 6.25 8.5 6.25-8.5c-3.25-2.75-9.25-2.75-12.5 0z" />
<path d="m4.25 8c2-1.75 5.5-1.75 7.5 0" />"###
};
#[cfg(ChWifiPoor)]
const CH_WIFI_POOR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m1.75 4.75 6.25 8.5 6.25-8.5c-3.25-2.75-9.25-2.75-12.5 0z" />
<path d="m5 9c.75-1.75 5.25-1.75 6 0" />"###
};
#[cfg(ChWifiSlash)]
const CH_WIFI_SLASH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m5.25 3.25c-1.5 0-3.5 1.5-3.5 1.5l6.25 8.5 2.25-3m-1.5-7.5s2.97688-.134944 5.5 2l-2 2.5" />
<line x1="4.25" y1="1.75" x2="12.25" y2="12.25" />"###
};
#[cfg(ChWifiWarning)]
const CH_WIFI_WARNING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m14.25 4.75c-3.25-2.75-9.25-2.75-12.5 0l6.25 8.5 1-1.5" />
<path d="m12.25 13.75v0m0-6v3.5" />"###
};
#[cfg(ChZoomIn)]
const CH_ZOOM_IN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="7.5" cy="7.5" r="4.75" />
<path d="m9.25 7.49992h-3.5m1.74992-1.74992v3.5m3.75008 2 3 3" />"###
};
#[cfg(ChZoomOut)]
const CH_ZOOM_OUT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("16"),
    height: Some("16"),
    view_box: Some("0 0 16 16"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("1.5"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="7.5" cy="7.5" r="4.75" />
<path d="m9.25 7.49992h-3.5m5.5 3.75008 3 3" />"###
};

impl From<ChIcon> for icondata_core::IconData {
    fn from(icon: ChIcon) -> icondata_core::IconData {
        match icon {
            #[cfg(ChAnchor)]
            ChIcon::ChAnchor => CH_ANCHOR,
            #[cfg(ChApps)]
            ChIcon::ChApps => CH_APPS,
            #[cfg(ChAppsMinus)]
            ChIcon::ChAppsMinus => CH_APPS_MINUS,
            #[cfg(ChAppsPlus)]
            ChIcon::ChAppsPlus => CH_APPS_PLUS,
            #[cfg(ChArchive)]
            ChIcon::ChArchive => CH_ARCHIVE,
            #[cfg(ChArrowDown)]
            ChIcon::ChArrowDown => CH_ARROW_DOWN,
            #[cfg(ChArrowDownLeft)]
            ChIcon::ChArrowDownLeft => CH_ARROW_DOWN_LEFT,
            #[cfg(ChArrowDownRight)]
            ChIcon::ChArrowDownRight => CH_ARROW_DOWN_RIGHT,
            #[cfg(ChArrowLeft)]
            ChIcon::ChArrowLeft => CH_ARROW_LEFT,
            #[cfg(ChArrowRight)]
            ChIcon::ChArrowRight => CH_ARROW_RIGHT,
            #[cfg(ChArrowUp)]
            ChIcon::ChArrowUp => CH_ARROW_UP,
            #[cfg(ChArrowUpLeft)]
            ChIcon::ChArrowUpLeft => CH_ARROW_UP_LEFT,
            #[cfg(ChArrowUpRight)]
            ChIcon::ChArrowUpRight => CH_ARROW_UP_RIGHT,
            #[cfg(ChAtSign)]
            ChIcon::ChAtSign => CH_AT_SIGN,
            #[cfg(ChAtom)]
            ChIcon::ChAtom => CH_ATOM,
            #[cfg(ChBell)]
            ChIcon::ChBell => CH_BELL,
            #[cfg(ChBellSlash)]
            ChIcon::ChBellSlash => CH_BELL_SLASH,
            #[cfg(ChBin)]
            ChIcon::ChBin => CH_BIN,
            #[cfg(ChBinary)]
            ChIcon::ChBinary => CH_BINARY,
            #[cfg(ChBlock)]
            ChIcon::ChBlock => CH_BLOCK,
            #[cfg(ChBluetooth)]
            ChIcon::ChBluetooth => CH_BLUETOOTH,
            #[cfg(ChBluetoothConnected)]
            ChIcon::ChBluetoothConnected => CH_BLUETOOTH_CONNECTED,
            #[cfg(ChBluetoothSearching)]
            ChIcon::ChBluetoothSearching => CH_BLUETOOTH_SEARCHING,
            #[cfg(ChBluetoothSlash)]
            ChIcon::ChBluetoothSlash => CH_BLUETOOTH_SLASH,
            #[cfg(ChBook)]
            ChIcon::ChBook => CH_BOOK,
            #[cfg(ChBookOpen)]
            ChIcon::ChBookOpen => CH_BOOK_OPEN,
            #[cfg(ChBookmark)]
            ChIcon::ChBookmark => CH_BOOKMARK,
            #[cfg(ChBriefcase)]
            ChIcon::ChBriefcase => CH_BRIEFCASE,
            #[cfg(ChBug)]
            ChIcon::ChBug => CH_BUG,
            #[cfg(ChCalendar)]
            ChIcon::ChCalendar => CH_CALENDAR,
            #[cfg(ChCamera)]
            ChIcon::ChCamera => CH_CAMERA,
            #[cfg(ChCameraVideo)]
            ChIcon::ChCameraVideo => CH_CAMERA_VIDEO,
            #[cfg(ChCameraVideoSlash)]
            ChIcon::ChCameraVideoSlash => CH_CAMERA_VIDEO_SLASH,
            #[cfg(ChCandy)]
            ChIcon::ChCandy => CH_CANDY,
            #[cfg(ChCards)]
            ChIcon::ChCards => CH_CARDS,
            #[cfg(ChCast)]
            ChIcon::ChCast => CH_CAST,
            #[cfg(ChCertificate)]
            ChIcon::ChCertificate => CH_CERTIFICATE,
            #[cfg(ChChartBar)]
            ChIcon::ChChartBar => CH_CHART_BAR,
            #[cfg(ChChartLine)]
            ChIcon::ChChartLine => CH_CHART_LINE,
            #[cfg(ChChevronDown)]
            ChIcon::ChChevronDown => CH_CHEVRON_DOWN,
            #[cfg(ChChevronLeft)]
            ChIcon::ChChevronLeft => CH_CHEVRON_LEFT,
            #[cfg(ChChevronRight)]
            ChIcon::ChChevronRight => CH_CHEVRON_RIGHT,
            #[cfg(ChChevronUp)]
            ChIcon::ChChevronUp => CH_CHEVRON_UP,
            #[cfg(ChChevronsDown)]
            ChIcon::ChChevronsDown => CH_CHEVRONS_DOWN,
            #[cfg(ChChevronsLeft)]
            ChIcon::ChChevronsLeft => CH_CHEVRONS_LEFT,
            #[cfg(ChChevronsRight)]
            ChIcon::ChChevronsRight => CH_CHEVRONS_RIGHT,
            #[cfg(ChChevronsUp)]
            ChIcon::ChChevronsUp => CH_CHEVRONS_UP,
            #[cfg(ChChevronsUpDown)]
            ChIcon::ChChevronsUpDown => CH_CHEVRONS_UP_DOWN,
            #[cfg(ChChip)]
            ChIcon::ChChip => CH_CHIP,
            #[cfg(ChCircle)]
            ChIcon::ChCircle => CH_CIRCLE,
            #[cfg(ChCircleCross)]
            ChIcon::ChCircleCross => CH_CIRCLE_CROSS,
            #[cfg(ChCircleMinus)]
            ChIcon::ChCircleMinus => CH_CIRCLE_MINUS,
            #[cfg(ChCircleTick)]
            ChIcon::ChCircleTick => CH_CIRCLE_TICK,
            #[cfg(ChCircleWarning)]
            ChIcon::ChCircleWarning => CH_CIRCLE_WARNING,
            #[cfg(ChClipboard)]
            ChIcon::ChClipboard => CH_CLIPBOARD,
            #[cfg(ChClipboardTick)]
            ChIcon::ChClipboardTick => CH_CLIPBOARD_TICK,
            #[cfg(ChClock)]
            ChIcon::ChClock => CH_CLOCK,
            #[cfg(ChClockAlarm)]
            ChIcon::ChClockAlarm => CH_CLOCK_ALARM,
            #[cfg(ChCloud)]
            ChIcon::ChCloud => CH_CLOUD,
            #[cfg(ChClover)]
            ChIcon::ChClover => CH_CLOVER,
            #[cfg(ChCode)]
            ChIcon::ChCode => CH_CODE,
            #[cfg(ChCoffee)]
            ChIcon::ChCoffee => CH_COFFEE,
            #[cfg(ChCog)]
            ChIcon::ChCog => CH_COG,
            #[cfg(ChCompass)]
            ChIcon::ChCompass => CH_COMPASS,
            #[cfg(ChConicalFlask)]
            ChIcon::ChConicalFlask => CH_CONICAL_FLASK,
            #[cfg(ChContainer)]
            ChIcon::ChContainer => CH_CONTAINER,
            #[cfg(ChCopy)]
            ChIcon::ChCopy => CH_COPY,
            #[cfg(ChCopyleft)]
            ChIcon::ChCopyleft => CH_COPYLEFT,
            #[cfg(ChCopyright)]
            ChIcon::ChCopyright => CH_COPYRIGHT,
            #[cfg(ChCreditCard)]
            ChIcon::ChCreditCard => CH_CREDIT_CARD,
            #[cfg(ChCrop)]
            ChIcon::ChCrop => CH_CROP,
            #[cfg(ChCross)]
            ChIcon::ChCross => CH_CROSS,
            #[cfg(ChCrosshair)]
            ChIcon::ChCrosshair => CH_CROSSHAIR,
            #[cfg(ChCube)]
            ChIcon::ChCube => CH_CUBE,
            #[cfg(ChCursor)]
            ChIcon::ChCursor => CH_CURSOR,
            #[cfg(ChDatabase)]
            ChIcon::ChDatabase => CH_DATABASE,
            #[cfg(ChDiamond)]
            ChIcon::ChDiamond => CH_DIAMOND,
            #[cfg(ChDiff)]
            ChIcon::ChDiff => CH_DIFF,
            #[cfg(ChDisc)]
            ChIcon::ChDisc => CH_DISC,
            #[cfg(ChDownload)]
            ChIcon::ChDownload => CH_DOWNLOAD,
            #[cfg(ChDroplet)]
            ChIcon::ChDroplet => CH_DROPLET,
            #[cfg(ChEraser)]
            ChIcon::ChEraser => CH_ERASER,
            #[cfg(ChEye)]
            ChIcon::ChEye => CH_EYE,
            #[cfg(ChEyeSlash)]
            ChIcon::ChEyeSlash => CH_EYE_SLASH,
            #[cfg(ChFaceFrown)]
            ChIcon::ChFaceFrown => CH_FACE_FROWN,
            #[cfg(ChFaceNeutral)]
            ChIcon::ChFaceNeutral => CH_FACE_NEUTRAL,
            #[cfg(ChFaceSmile)]
            ChIcon::ChFaceSmile => CH_FACE_SMILE,
            #[cfg(ChFile)]
            ChIcon::ChFile => CH_FILE,
            #[cfg(ChFileBinary)]
            ChIcon::ChFileBinary => CH_FILE_BINARY,
            #[cfg(ChFileCode)]
            ChIcon::ChFileCode => CH_FILE_CODE,
            #[cfg(ChFileSymlink)]
            ChIcon::ChFileSymlink => CH_FILE_SYMLINK,
            #[cfg(ChFiles)]
            ChIcon::ChFiles => CH_FILES,
            #[cfg(ChFilter)]
            ChIcon::ChFilter => CH_FILTER,
            #[cfg(ChFlag)]
            ChIcon::ChFlag => CH_FLAG,
            #[cfg(ChFlame)]
            ChIcon::ChFlame => CH_FLAME,
            #[cfg(ChFloppyDisk)]
            ChIcon::ChFloppyDisk => CH_FLOPPY_DISK,
            #[cfg(ChFolder)]
            ChIcon::ChFolder => CH_FOLDER,
            #[cfg(ChFolderSymlink)]
            ChIcon::ChFolderSymlink => CH_FOLDER_SYMLINK,
            #[cfg(ChFolders)]
            ChIcon::ChFolders => CH_FOLDERS,
            #[cfg(ChForward)]
            ChIcon::ChForward => CH_FORWARD,
            #[cfg(ChGamepad)]
            ChIcon::ChGamepad => CH_GAMEPAD,
            #[cfg(ChGem)]
            ChIcon::ChGem => CH_GEM,
            #[cfg(ChGift)]
            ChIcon::ChGift => CH_GIFT,
            #[cfg(ChGitBranch)]
            ChIcon::ChGitBranch => CH_GIT_BRANCH,
            #[cfg(ChGitCherryPick)]
            ChIcon::ChGitCherryPick => CH_GIT_CHERRY_PICK,
            #[cfg(ChGitCommit)]
            ChIcon::ChGitCommit => CH_GIT_COMMIT,
            #[cfg(ChGitCompare)]
            ChIcon::ChGitCompare => CH_GIT_COMPARE,
            #[cfg(ChGitFork)]
            ChIcon::ChGitFork => CH_GIT_FORK,
            #[cfg(ChGitMerge)]
            ChIcon::ChGitMerge => CH_GIT_MERGE,
            #[cfg(ChGitRequest)]
            ChIcon::ChGitRequest => CH_GIT_REQUEST,
            #[cfg(ChGitRequestCross)]
            ChIcon::ChGitRequestCross => CH_GIT_REQUEST_CROSS,
            #[cfg(ChGitRequestDraft)]
            ChIcon::ChGitRequestDraft => CH_GIT_REQUEST_DRAFT,
            #[cfg(ChGithub)]
            ChIcon::ChGithub => CH_GITHUB,
            #[cfg(ChGitlab)]
            ChIcon::ChGitlab => CH_GITLAB,
            #[cfg(ChGlasses)]
            ChIcon::ChGlasses => CH_GLASSES,
            #[cfg(ChGlobe)]
            ChIcon::ChGlobe => CH_GLOBE,
            #[cfg(ChGrabHorizontal)]
            ChIcon::ChGrabHorizontal => CH_GRAB_HORIZONTAL,
            #[cfg(ChGrabVertical)]
            ChIcon::ChGrabVertical => CH_GRAB_VERTICAL,
            #[cfg(ChGraduateCap)]
            ChIcon::ChGraduateCap => CH_GRADUATE_CAP,
            #[cfg(ChHash)]
            ChIcon::ChHash => CH_HASH,
            #[cfg(ChHeadphones)]
            ChIcon::ChHeadphones => CH_HEADPHONES,
            #[cfg(ChHeart)]
            ChIcon::ChHeart => CH_HEART,
            #[cfg(ChHelp)]
            ChIcon::ChHelp => CH_HELP,
            #[cfg(ChHexagon)]
            ChIcon::ChHexagon => CH_HEXAGON,
            #[cfg(ChHome)]
            ChIcon::ChHome => CH_HOME,
            #[cfg(ChHourglass)]
            ChIcon::ChHourglass => CH_HOURGLASS,
            #[cfg(ChId)]
            ChIcon::ChId => CH_ID,
            #[cfg(ChImage)]
            ChIcon::ChImage => CH_IMAGE,
            #[cfg(ChInbox)]
            ChIcon::ChInbox => CH_INBOX,
            #[cfg(ChInfinity)]
            ChIcon::ChInfinity => CH_INFINITY,
            #[cfg(ChInfo)]
            ChIcon::ChInfo => CH_INFO,
            #[cfg(ChKey)]
            ChIcon::ChKey => CH_KEY,
            #[cfg(ChLaptop)]
            ChIcon::ChLaptop => CH_LAPTOP,
            #[cfg(ChLayoutColumns)]
            ChIcon::ChLayoutColumns => CH_LAYOUT_COLUMNS,
            #[cfg(ChLayoutDashboard)]
            ChIcon::ChLayoutDashboard => CH_LAYOUT_DASHBOARD,
            #[cfg(ChLayoutGrid)]
            ChIcon::ChLayoutGrid => CH_LAYOUT_GRID,
            #[cfg(ChLayoutList)]
            ChIcon::ChLayoutList => CH_LAYOUT_LIST,
            #[cfg(ChLayoutRows)]
            ChIcon::ChLayoutRows => CH_LAYOUT_ROWS,
            #[cfg(ChLayoutSidebar)]
            ChIcon::ChLayoutSidebar => CH_LAYOUT_SIDEBAR,
            #[cfg(ChLayoutStackH)]
            ChIcon::ChLayoutStackH => CH_LAYOUT_STACK_H,
            #[cfg(ChLayoutStackV)]
            ChIcon::ChLayoutStackV => CH_LAYOUT_STACK_V,
            #[cfg(ChLightbulb)]
            ChIcon::ChLightbulb => CH_LIGHTBULB,
            #[cfg(ChLightningBolt)]
            ChIcon::ChLightningBolt => CH_LIGHTNING_BOLT,
            #[cfg(ChLink)]
            ChIcon::ChLink => CH_LINK,
            #[cfg(ChLinkExternal)]
            ChIcon::ChLinkExternal => CH_LINK_EXTERNAL,
            #[cfg(ChLinkSlash)]
            ChIcon::ChLinkSlash => CH_LINK_SLASH,
            #[cfg(ChMail)]
            ChIcon::ChMail => CH_MAIL,
            #[cfg(ChMap)]
            ChIcon::ChMap => CH_MAP,
            #[cfg(ChMapPin)]
            ChIcon::ChMapPin => CH_MAP_PIN,
            #[cfg(ChMediaBack)]
            ChIcon::ChMediaBack => CH_MEDIA_BACK,
            #[cfg(ChMediaEject)]
            ChIcon::ChMediaEject => CH_MEDIA_EJECT,
            #[cfg(ChMediaFastForward)]
            ChIcon::ChMediaFastForward => CH_MEDIA_FAST_FORWARD,
            #[cfg(ChMediaPause)]
            ChIcon::ChMediaPause => CH_MEDIA_PAUSE,
            #[cfg(ChMediaPlay)]
            ChIcon::ChMediaPlay => CH_MEDIA_PLAY,
            #[cfg(ChMediaRewind)]
            ChIcon::ChMediaRewind => CH_MEDIA_REWIND,
            #[cfg(ChMediaSkip)]
            ChIcon::ChMediaSkip => CH_MEDIA_SKIP,
            #[cfg(ChMenuHamburger)]
            ChIcon::ChMenuHamburger => CH_MENU_HAMBURGER,
            #[cfg(ChMenuKebab)]
            ChIcon::ChMenuKebab => CH_MENU_KEBAB,
            #[cfg(ChMenuMeatball)]
            ChIcon::ChMenuMeatball => CH_MENU_MEATBALL,
            #[cfg(ChMessage)]
            ChIcon::ChMessage => CH_MESSAGE,
            #[cfg(ChMessages)]
            ChIcon::ChMessages => CH_MESSAGES,
            #[cfg(ChMicrophone)]
            ChIcon::ChMicrophone => CH_MICROPHONE,
            #[cfg(ChMinus)]
            ChIcon::ChMinus => CH_MINUS,
            #[cfg(ChMobile)]
            ChIcon::ChMobile => CH_MOBILE,
            #[cfg(ChMonitor)]
            ChIcon::ChMonitor => CH_MONITOR,
            #[cfg(ChMonitorArrow)]
            ChIcon::ChMonitorArrow => CH_MONITOR_ARROW,
            #[cfg(ChMonitorCross)]
            ChIcon::ChMonitorCross => CH_MONITOR_CROSS,
            #[cfg(ChMoon)]
            ChIcon::ChMoon => CH_MOON,
            #[cfg(ChMove)]
            ChIcon::ChMove => CH_MOVE,
            #[cfg(ChMusic)]
            ChIcon::ChMusic => CH_MUSIC,
            #[cfg(ChNewspaper)]
            ChIcon::ChNewspaper => CH_NEWSPAPER,
            #[cfg(ChNorthStar)]
            ChIcon::ChNorthStar => CH_NORTH_STAR,
            #[cfg(ChNotes)]
            ChIcon::ChNotes => CH_NOTES,
            #[cfg(ChNotesCross)]
            ChIcon::ChNotesCross => CH_NOTES_CROSS,
            #[cfg(ChNotesTick)]
            ChIcon::ChNotesTick => CH_NOTES_TICK,
            #[cfg(ChNut)]
            ChIcon::ChNut => CH_NUT,
            #[cfg(ChOctagon)]
            ChIcon::ChOctagon => CH_OCTAGON,
            #[cfg(ChOctagonWarning)]
            ChIcon::ChOctagonWarning => CH_OCTAGON_WARNING,
            #[cfg(ChOrganisation)]
            ChIcon::ChOrganisation => CH_ORGANISATION,
            #[cfg(ChPackage)]
            ChIcon::ChPackage => CH_PACKAGE,
            #[cfg(ChPadlock)]
            ChIcon::ChPadlock => CH_PADLOCK,
            #[cfg(ChPaperPlane)]
            ChIcon::ChPaperPlane => CH_PAPER_PLANE,
            #[cfg(ChPaperclip)]
            ChIcon::ChPaperclip => CH_PAPERCLIP,
            #[cfg(ChPencil)]
            ChIcon::ChPencil => CH_PENCIL,
            #[cfg(ChPeople)]
            ChIcon::ChPeople => CH_PEOPLE,
            #[cfg(ChPerson)]
            ChIcon::ChPerson => CH_PERSON,
            #[cfg(ChPhone)]
            ChIcon::ChPhone => CH_PHONE,
            #[cfg(ChPhoneCall)]
            ChIcon::ChPhoneCall => CH_PHONE_CALL,
            #[cfg(ChPhoneCross)]
            ChIcon::ChPhoneCross => CH_PHONE_CROSS,
            #[cfg(ChPhoneForward)]
            ChIcon::ChPhoneForward => CH_PHONE_FORWARD,
            #[cfg(ChPhoneIncoming)]
            ChIcon::ChPhoneIncoming => CH_PHONE_INCOMING,
            #[cfg(ChPhoneOutgoing)]
            ChIcon::ChPhoneOutgoing => CH_PHONE_OUTGOING,
            #[cfg(ChPin)]
            ChIcon::ChPin => CH_PIN,
            #[cfg(ChPlantPot)]
            ChIcon::ChPlantPot => CH_PLANT_POT,
            #[cfg(ChPlus)]
            ChIcon::ChPlus => CH_PLUS,
            #[cfg(ChPower)]
            ChIcon::ChPower => CH_POWER,
            #[cfg(ChPrinter)]
            ChIcon::ChPrinter => CH_PRINTER,
            #[cfg(ChPulse)]
            ChIcon::ChPulse => CH_PULSE,
            #[cfg(ChQuote)]
            ChIcon::ChQuote => CH_QUOTE,
            #[cfg(ChRefresh)]
            ChIcon::ChRefresh => CH_REFRESH,
            #[cfg(ChReply)]
            ChIcon::ChReply => CH_REPLY,
            #[cfg(ChRobot)]
            ChIcon::ChRobot => CH_ROBOT,
            #[cfg(ChRocket)]
            ChIcon::ChRocket => CH_ROCKET,
            #[cfg(ChRotateAntiClockwise)]
            ChIcon::ChRotateAntiClockwise => CH_ROTATE_ANTI_CLOCKWISE,
            #[cfg(ChRotateClockwise)]
            ChIcon::ChRotateClockwise => CH_ROTATE_CLOCKWISE,
            #[cfg(ChScales)]
            ChIcon::ChScales => CH_SCALES,
            #[cfg(ChScreenMaximise)]
            ChIcon::ChScreenMaximise => CH_SCREEN_MAXIMISE,
            #[cfg(ChScreenMinimise)]
            ChIcon::ChScreenMinimise => CH_SCREEN_MINIMISE,
            #[cfg(ChSearch)]
            ChIcon::ChSearch => CH_SEARCH,
            #[cfg(ChServer)]
            ChIcon::ChServer => CH_SERVER,
            #[cfg(ChShare)]
            ChIcon::ChShare => CH_SHARE,
            #[cfg(ChShield)]
            ChIcon::ChShield => CH_SHIELD,
            #[cfg(ChShieldCross)]
            ChIcon::ChShieldCross => CH_SHIELD_CROSS,
            #[cfg(ChShieldKeyhole)]
            ChIcon::ChShieldKeyhole => CH_SHIELD_KEYHOLE,
            #[cfg(ChShieldTick)]
            ChIcon::ChShieldTick => CH_SHIELD_TICK,
            #[cfg(ChShieldWarning)]
            ChIcon::ChShieldWarning => CH_SHIELD_WARNING,
            #[cfg(ChShoppingBag)]
            ChIcon::ChShoppingBag => CH_SHOPPING_BAG,
            #[cfg(ChSignIn)]
            ChIcon::ChSignIn => CH_SIGN_IN,
            #[cfg(ChSignOut)]
            ChIcon::ChSignOut => CH_SIGN_OUT,
            #[cfg(ChSignpost)]
            ChIcon::ChSignpost => CH_SIGNPOST,
            #[cfg(ChSkull)]
            ChIcon::ChSkull => CH_SKULL,
            #[cfg(ChSnowflake)]
            ChIcon::ChSnowflake => CH_SNOWFLAKE,
            #[cfg(ChSoundDown)]
            ChIcon::ChSoundDown => CH_SOUND_DOWN,
            #[cfg(ChSoundMute)]
            ChIcon::ChSoundMute => CH_SOUND_MUTE,
            #[cfg(ChSoundUp)]
            ChIcon::ChSoundUp => CH_SOUND_UP,
            #[cfg(ChSpeaker)]
            ChIcon::ChSpeaker => CH_SPEAKER,
            #[cfg(ChSquare)]
            ChIcon::ChSquare => CH_SQUARE,
            #[cfg(ChSquareCross)]
            ChIcon::ChSquareCross => CH_SQUARE_CROSS,
            #[cfg(ChSquareTick)]
            ChIcon::ChSquareTick => CH_SQUARE_TICK,
            #[cfg(ChStack)]
            ChIcon::ChStack => CH_STACK,
            #[cfg(ChStackPop)]
            ChIcon::ChStackPop => CH_STACK_POP,
            #[cfg(ChStackPush)]
            ChIcon::ChStackPush => CH_STACK_PUSH,
            #[cfg(ChStar)]
            ChIcon::ChStar => CH_STAR,
            #[cfg(ChStickyNote)]
            ChIcon::ChStickyNote => CH_STICKY_NOTE,
            #[cfg(ChSun)]
            ChIcon::ChSun => CH_SUN,
            #[cfg(ChSwapHorizontal)]
            ChIcon::ChSwapHorizontal => CH_SWAP_HORIZONTAL,
            #[cfg(ChSwapVertical)]
            ChIcon::ChSwapVertical => CH_SWAP_VERTICAL,
            #[cfg(ChSword)]
            ChIcon::ChSword => CH_SWORD,
            #[cfg(ChSwords)]
            ChIcon::ChSwords => CH_SWORDS,
            #[cfg(ChTablet)]
            ChIcon::ChTablet => CH_TABLET,
            #[cfg(ChTag)]
            ChIcon::ChTag => CH_TAG,
            #[cfg(ChTelescope)]
            ChIcon::ChTelescope => CH_TELESCOPE,
            #[cfg(ChTent)]
            ChIcon::ChTent => CH_TENT,
            #[cfg(ChTerminal)]
            ChIcon::ChTerminal => CH_TERMINAL,
            #[cfg(ChThumbDown)]
            ChIcon::ChThumbDown => CH_THUMB_DOWN,
            #[cfg(ChThumbUp)]
            ChIcon::ChThumbUp => CH_THUMB_UP,
            #[cfg(ChTick)]
            ChIcon::ChTick => CH_TICK,
            #[cfg(ChTickDouble)]
            ChIcon::ChTickDouble => CH_TICK_DOUBLE,
            #[cfg(ChTicket)]
            ChIcon::ChTicket => CH_TICKET,
            #[cfg(ChTreeFir)]
            ChIcon::ChTreeFir => CH_TREE_FIR,
            #[cfg(ChTriangle)]
            ChIcon::ChTriangle => CH_TRIANGLE,
            #[cfg(ChTrophy)]
            ChIcon::ChTrophy => CH_TROPHY,
            #[cfg(ChUmbrella)]
            ChIcon::ChUmbrella => CH_UMBRELLA,
            #[cfg(ChUpload)]
            ChIcon::ChUpload => CH_UPLOAD,
            #[cfg(ChWifi)]
            ChIcon::ChWifi => CH_WIFI,
            #[cfg(ChWifiFair)]
            ChIcon::ChWifiFair => CH_WIFI_FAIR,
            #[cfg(ChWifiPoor)]
            ChIcon::ChWifiPoor => CH_WIFI_POOR,
            #[cfg(ChWifiSlash)]
            ChIcon::ChWifiSlash => CH_WIFI_SLASH,
            #[cfg(ChWifiWarning)]
            ChIcon::ChWifiWarning => CH_WIFI_WARNING,
            #[cfg(ChZoomIn)]
            ChIcon::ChZoomIn => CH_ZOOM_IN,
            #[cfg(ChZoomOut)]
            ChIcon::ChZoomOut => CH_ZOOM_OUT,
        }
    }
}