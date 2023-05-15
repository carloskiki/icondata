//! This crate provides a collection of icons in the form of SVG data
//! and an enum to select them.
//!
//! ## Usage
//!
//! Every icon is shipped as its own feature; the enum variant and their corresponding feature name are
//! identical.
//!
//! The enum implements [`Into<icondata_core::IconData>`][icondata_core::IconData].
//!
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "strum", derive(strum::EnumIter, strum::EnumVariantNames))]
pub enum ChIcon {
    #[cfg(feature = "ChAnchor")]
    ChAnchor,
    #[cfg(feature = "ChApps")]
    ChApps,
    #[cfg(feature = "ChAppsMinus")]
    ChAppsMinus,
    #[cfg(feature = "ChAppsPlus")]
    ChAppsPlus,
    #[cfg(feature = "ChArchive")]
    ChArchive,
    #[cfg(feature = "ChArrowDown")]
    ChArrowDown,
    #[cfg(feature = "ChArrowDownLeft")]
    ChArrowDownLeft,
    #[cfg(feature = "ChArrowDownRight")]
    ChArrowDownRight,
    #[cfg(feature = "ChArrowLeft")]
    ChArrowLeft,
    #[cfg(feature = "ChArrowRight")]
    ChArrowRight,
    #[cfg(feature = "ChArrowUp")]
    ChArrowUp,
    #[cfg(feature = "ChArrowUpLeft")]
    ChArrowUpLeft,
    #[cfg(feature = "ChArrowUpRight")]
    ChArrowUpRight,
    #[cfg(feature = "ChAtSign")]
    ChAtSign,
    #[cfg(feature = "ChAtom")]
    ChAtom,
    #[cfg(feature = "ChBell")]
    ChBell,
    #[cfg(feature = "ChBellSlash")]
    ChBellSlash,
    #[cfg(feature = "ChBin")]
    ChBin,
    #[cfg(feature = "ChBinary")]
    ChBinary,
    #[cfg(feature = "ChBlock")]
    ChBlock,
    #[cfg(feature = "ChBluetooth")]
    ChBluetooth,
    #[cfg(feature = "ChBluetoothConnected")]
    ChBluetoothConnected,
    #[cfg(feature = "ChBluetoothSearching")]
    ChBluetoothSearching,
    #[cfg(feature = "ChBluetoothSlash")]
    ChBluetoothSlash,
    #[cfg(feature = "ChBook")]
    ChBook,
    #[cfg(feature = "ChBookOpen")]
    ChBookOpen,
    #[cfg(feature = "ChBookmark")]
    ChBookmark,
    #[cfg(feature = "ChBriefcase")]
    ChBriefcase,
    #[cfg(feature = "ChBug")]
    ChBug,
    #[cfg(feature = "ChCalendar")]
    ChCalendar,
    #[cfg(feature = "ChCamera")]
    ChCamera,
    #[cfg(feature = "ChCameraVideo")]
    ChCameraVideo,
    #[cfg(feature = "ChCameraVideoSlash")]
    ChCameraVideoSlash,
    #[cfg(feature = "ChCandy")]
    ChCandy,
    #[cfg(feature = "ChCards")]
    ChCards,
    #[cfg(feature = "ChCast")]
    ChCast,
    #[cfg(feature = "ChCertificate")]
    ChCertificate,
    #[cfg(feature = "ChChartBar")]
    ChChartBar,
    #[cfg(feature = "ChChartLine")]
    ChChartLine,
    #[cfg(feature = "ChChevronDown")]
    ChChevronDown,
    #[cfg(feature = "ChChevronLeft")]
    ChChevronLeft,
    #[cfg(feature = "ChChevronRight")]
    ChChevronRight,
    #[cfg(feature = "ChChevronUp")]
    ChChevronUp,
    #[cfg(feature = "ChChevronsDown")]
    ChChevronsDown,
    #[cfg(feature = "ChChevronsLeft")]
    ChChevronsLeft,
    #[cfg(feature = "ChChevronsRight")]
    ChChevronsRight,
    #[cfg(feature = "ChChevronsUp")]
    ChChevronsUp,
    #[cfg(feature = "ChChevronsUpDown")]
    ChChevronsUpDown,
    #[cfg(feature = "ChChip")]
    ChChip,
    #[cfg(feature = "ChCircle")]
    ChCircle,
    #[cfg(feature = "ChCircleCross")]
    ChCircleCross,
    #[cfg(feature = "ChCircleMinus")]
    ChCircleMinus,
    #[cfg(feature = "ChCircleTick")]
    ChCircleTick,
    #[cfg(feature = "ChCircleWarning")]
    ChCircleWarning,
    #[cfg(feature = "ChClipboard")]
    ChClipboard,
    #[cfg(feature = "ChClipboardTick")]
    ChClipboardTick,
    #[cfg(feature = "ChClock")]
    ChClock,
    #[cfg(feature = "ChClockAlarm")]
    ChClockAlarm,
    #[cfg(feature = "ChCloud")]
    ChCloud,
    #[cfg(feature = "ChClover")]
    ChClover,
    #[cfg(feature = "ChCode")]
    ChCode,
    #[cfg(feature = "ChCoffee")]
    ChCoffee,
    #[cfg(feature = "ChCog")]
    ChCog,
    #[cfg(feature = "ChCompass")]
    ChCompass,
    #[cfg(feature = "ChConicalFlask")]
    ChConicalFlask,
    #[cfg(feature = "ChContainer")]
    ChContainer,
    #[cfg(feature = "ChCopy")]
    ChCopy,
    #[cfg(feature = "ChCopyleft")]
    ChCopyleft,
    #[cfg(feature = "ChCopyright")]
    ChCopyright,
    #[cfg(feature = "ChCreditCard")]
    ChCreditCard,
    #[cfg(feature = "ChCrop")]
    ChCrop,
    #[cfg(feature = "ChCross")]
    ChCross,
    #[cfg(feature = "ChCrosshair")]
    ChCrosshair,
    #[cfg(feature = "ChCube")]
    ChCube,
    #[cfg(feature = "ChCursor")]
    ChCursor,
    #[cfg(feature = "ChDatabase")]
    ChDatabase,
    #[cfg(feature = "ChDiamond")]
    ChDiamond,
    #[cfg(feature = "ChDiff")]
    ChDiff,
    #[cfg(feature = "ChDisc")]
    ChDisc,
    #[cfg(feature = "ChDownload")]
    ChDownload,
    #[cfg(feature = "ChDroplet")]
    ChDroplet,
    #[cfg(feature = "ChEraser")]
    ChEraser,
    #[cfg(feature = "ChEye")]
    ChEye,
    #[cfg(feature = "ChEyeSlash")]
    ChEyeSlash,
    #[cfg(feature = "ChFaceFrown")]
    ChFaceFrown,
    #[cfg(feature = "ChFaceNeutral")]
    ChFaceNeutral,
    #[cfg(feature = "ChFaceSmile")]
    ChFaceSmile,
    #[cfg(feature = "ChFile")]
    ChFile,
    #[cfg(feature = "ChFileBinary")]
    ChFileBinary,
    #[cfg(feature = "ChFileCode")]
    ChFileCode,
    #[cfg(feature = "ChFileSymlink")]
    ChFileSymlink,
    #[cfg(feature = "ChFiles")]
    ChFiles,
    #[cfg(feature = "ChFilter")]
    ChFilter,
    #[cfg(feature = "ChFlag")]
    ChFlag,
    #[cfg(feature = "ChFlame")]
    ChFlame,
    #[cfg(feature = "ChFloppyDisk")]
    ChFloppyDisk,
    #[cfg(feature = "ChFolder")]
    ChFolder,
    #[cfg(feature = "ChFolderSymlink")]
    ChFolderSymlink,
    #[cfg(feature = "ChFolders")]
    ChFolders,
    #[cfg(feature = "ChForward")]
    ChForward,
    #[cfg(feature = "ChGamepad")]
    ChGamepad,
    #[cfg(feature = "ChGem")]
    ChGem,
    #[cfg(feature = "ChGift")]
    ChGift,
    #[cfg(feature = "ChGitBranch")]
    ChGitBranch,
    #[cfg(feature = "ChGitCherryPick")]
    ChGitCherryPick,
    #[cfg(feature = "ChGitCommit")]
    ChGitCommit,
    #[cfg(feature = "ChGitCompare")]
    ChGitCompare,
    #[cfg(feature = "ChGitFork")]
    ChGitFork,
    #[cfg(feature = "ChGitMerge")]
    ChGitMerge,
    #[cfg(feature = "ChGitRequest")]
    ChGitRequest,
    #[cfg(feature = "ChGitRequestCross")]
    ChGitRequestCross,
    #[cfg(feature = "ChGitRequestDraft")]
    ChGitRequestDraft,
    #[cfg(feature = "ChGithub")]
    ChGithub,
    #[cfg(feature = "ChGitlab")]
    ChGitlab,
    #[cfg(feature = "ChGlasses")]
    ChGlasses,
    #[cfg(feature = "ChGlobe")]
    ChGlobe,
    #[cfg(feature = "ChGrabHorizontal")]
    ChGrabHorizontal,
    #[cfg(feature = "ChGrabVertical")]
    ChGrabVertical,
    #[cfg(feature = "ChGraduateCap")]
    ChGraduateCap,
    #[cfg(feature = "ChHash")]
    ChHash,
    #[cfg(feature = "ChHeadphones")]
    ChHeadphones,
    #[cfg(feature = "ChHeart")]
    ChHeart,
    #[cfg(feature = "ChHelp")]
    ChHelp,
    #[cfg(feature = "ChHexagon")]
    ChHexagon,
    #[cfg(feature = "ChHome")]
    ChHome,
    #[cfg(feature = "ChHourglass")]
    ChHourglass,
    #[cfg(feature = "ChId")]
    ChId,
    #[cfg(feature = "ChImage")]
    ChImage,
    #[cfg(feature = "ChInbox")]
    ChInbox,
    #[cfg(feature = "ChInfinity")]
    ChInfinity,
    #[cfg(feature = "ChInfo")]
    ChInfo,
    #[cfg(feature = "ChKey")]
    ChKey,
    #[cfg(feature = "ChLaptop")]
    ChLaptop,
    #[cfg(feature = "ChLayoutColumns")]
    ChLayoutColumns,
    #[cfg(feature = "ChLayoutDashboard")]
    ChLayoutDashboard,
    #[cfg(feature = "ChLayoutGrid")]
    ChLayoutGrid,
    #[cfg(feature = "ChLayoutList")]
    ChLayoutList,
    #[cfg(feature = "ChLayoutRows")]
    ChLayoutRows,
    #[cfg(feature = "ChLayoutSidebar")]
    ChLayoutSidebar,
    #[cfg(feature = "ChLayoutStackH")]
    ChLayoutStackH,
    #[cfg(feature = "ChLayoutStackV")]
    ChLayoutStackV,
    #[cfg(feature = "ChLightbulb")]
    ChLightbulb,
    #[cfg(feature = "ChLightningBolt")]
    ChLightningBolt,
    #[cfg(feature = "ChLink")]
    ChLink,
    #[cfg(feature = "ChLinkExternal")]
    ChLinkExternal,
    #[cfg(feature = "ChLinkSlash")]
    ChLinkSlash,
    #[cfg(feature = "ChMail")]
    ChMail,
    #[cfg(feature = "ChMap")]
    ChMap,
    #[cfg(feature = "ChMapPin")]
    ChMapPin,
    #[cfg(feature = "ChMediaBack")]
    ChMediaBack,
    #[cfg(feature = "ChMediaEject")]
    ChMediaEject,
    #[cfg(feature = "ChMediaFastForward")]
    ChMediaFastForward,
    #[cfg(feature = "ChMediaPause")]
    ChMediaPause,
    #[cfg(feature = "ChMediaPlay")]
    ChMediaPlay,
    #[cfg(feature = "ChMediaRewind")]
    ChMediaRewind,
    #[cfg(feature = "ChMediaSkip")]
    ChMediaSkip,
    #[cfg(feature = "ChMenuHamburger")]
    ChMenuHamburger,
    #[cfg(feature = "ChMenuKebab")]
    ChMenuKebab,
    #[cfg(feature = "ChMenuMeatball")]
    ChMenuMeatball,
    #[cfg(feature = "ChMessage")]
    ChMessage,
    #[cfg(feature = "ChMessages")]
    ChMessages,
    #[cfg(feature = "ChMicrophone")]
    ChMicrophone,
    #[cfg(feature = "ChMinus")]
    ChMinus,
    #[cfg(feature = "ChMobile")]
    ChMobile,
    #[cfg(feature = "ChMonitor")]
    ChMonitor,
    #[cfg(feature = "ChMonitorArrow")]
    ChMonitorArrow,
    #[cfg(feature = "ChMonitorCross")]
    ChMonitorCross,
    #[cfg(feature = "ChMoon")]
    ChMoon,
    #[cfg(feature = "ChMove")]
    ChMove,
    #[cfg(feature = "ChMusic")]
    ChMusic,
    #[cfg(feature = "ChNewspaper")]
    ChNewspaper,
    #[cfg(feature = "ChNorthStar")]
    ChNorthStar,
    #[cfg(feature = "ChNotes")]
    ChNotes,
    #[cfg(feature = "ChNotesCross")]
    ChNotesCross,
    #[cfg(feature = "ChNotesTick")]
    ChNotesTick,
    #[cfg(feature = "ChNut")]
    ChNut,
    #[cfg(feature = "ChOctagon")]
    ChOctagon,
    #[cfg(feature = "ChOctagonWarning")]
    ChOctagonWarning,
    #[cfg(feature = "ChOrganisation")]
    ChOrganisation,
    #[cfg(feature = "ChPackage")]
    ChPackage,
    #[cfg(feature = "ChPadlock")]
    ChPadlock,
    #[cfg(feature = "ChPaperPlane")]
    ChPaperPlane,
    #[cfg(feature = "ChPaperclip")]
    ChPaperclip,
    #[cfg(feature = "ChPencil")]
    ChPencil,
    #[cfg(feature = "ChPeople")]
    ChPeople,
    #[cfg(feature = "ChPerson")]
    ChPerson,
    #[cfg(feature = "ChPhone")]
    ChPhone,
    #[cfg(feature = "ChPhoneCall")]
    ChPhoneCall,
    #[cfg(feature = "ChPhoneCross")]
    ChPhoneCross,
    #[cfg(feature = "ChPhoneForward")]
    ChPhoneForward,
    #[cfg(feature = "ChPhoneIncoming")]
    ChPhoneIncoming,
    #[cfg(feature = "ChPhoneOutgoing")]
    ChPhoneOutgoing,
    #[cfg(feature = "ChPin")]
    ChPin,
    #[cfg(feature = "ChPlantPot")]
    ChPlantPot,
    #[cfg(feature = "ChPlus")]
    ChPlus,
    #[cfg(feature = "ChPower")]
    ChPower,
    #[cfg(feature = "ChPrinter")]
    ChPrinter,
    #[cfg(feature = "ChPulse")]
    ChPulse,
    #[cfg(feature = "ChQuote")]
    ChQuote,
    #[cfg(feature = "ChRefresh")]
    ChRefresh,
    #[cfg(feature = "ChReply")]
    ChReply,
    #[cfg(feature = "ChRobot")]
    ChRobot,
    #[cfg(feature = "ChRocket")]
    ChRocket,
    #[cfg(feature = "ChRotateAntiClockwise")]
    ChRotateAntiClockwise,
    #[cfg(feature = "ChRotateClockwise")]
    ChRotateClockwise,
    #[cfg(feature = "ChScales")]
    ChScales,
    #[cfg(feature = "ChScreenMaximise")]
    ChScreenMaximise,
    #[cfg(feature = "ChScreenMinimise")]
    ChScreenMinimise,
    #[cfg(feature = "ChSearch")]
    ChSearch,
    #[cfg(feature = "ChServer")]
    ChServer,
    #[cfg(feature = "ChShare")]
    ChShare,
    #[cfg(feature = "ChShield")]
    ChShield,
    #[cfg(feature = "ChShieldCross")]
    ChShieldCross,
    #[cfg(feature = "ChShieldKeyhole")]
    ChShieldKeyhole,
    #[cfg(feature = "ChShieldTick")]
    ChShieldTick,
    #[cfg(feature = "ChShieldWarning")]
    ChShieldWarning,
    #[cfg(feature = "ChShoppingBag")]
    ChShoppingBag,
    #[cfg(feature = "ChSignIn")]
    ChSignIn,
    #[cfg(feature = "ChSignOut")]
    ChSignOut,
    #[cfg(feature = "ChSignpost")]
    ChSignpost,
    #[cfg(feature = "ChSkull")]
    ChSkull,
    #[cfg(feature = "ChSnowflake")]
    ChSnowflake,
    #[cfg(feature = "ChSoundDown")]
    ChSoundDown,
    #[cfg(feature = "ChSoundMute")]
    ChSoundMute,
    #[cfg(feature = "ChSoundUp")]
    ChSoundUp,
    #[cfg(feature = "ChSpeaker")]
    ChSpeaker,
    #[cfg(feature = "ChSquare")]
    ChSquare,
    #[cfg(feature = "ChSquareCross")]
    ChSquareCross,
    #[cfg(feature = "ChSquareTick")]
    ChSquareTick,
    #[cfg(feature = "ChStack")]
    ChStack,
    #[cfg(feature = "ChStackPop")]
    ChStackPop,
    #[cfg(feature = "ChStackPush")]
    ChStackPush,
    #[cfg(feature = "ChStar")]
    ChStar,
    #[cfg(feature = "ChStickyNote")]
    ChStickyNote,
    #[cfg(feature = "ChSun")]
    ChSun,
    #[cfg(feature = "ChSwapHorizontal")]
    ChSwapHorizontal,
    #[cfg(feature = "ChSwapVertical")]
    ChSwapVertical,
    #[cfg(feature = "ChSword")]
    ChSword,
    #[cfg(feature = "ChSwords")]
    ChSwords,
    #[cfg(feature = "ChTablet")]
    ChTablet,
    #[cfg(feature = "ChTag")]
    ChTag,
    #[cfg(feature = "ChTelescope")]
    ChTelescope,
    #[cfg(feature = "ChTent")]
    ChTent,
    #[cfg(feature = "ChTerminal")]
    ChTerminal,
    #[cfg(feature = "ChThumbDown")]
    ChThumbDown,
    #[cfg(feature = "ChThumbUp")]
    ChThumbUp,
    #[cfg(feature = "ChTick")]
    ChTick,
    #[cfg(feature = "ChTickDouble")]
    ChTickDouble,
    #[cfg(feature = "ChTicket")]
    ChTicket,
    #[cfg(feature = "ChTreeFir")]
    ChTreeFir,
    #[cfg(feature = "ChTriangle")]
    ChTriangle,
    #[cfg(feature = "ChTrophy")]
    ChTrophy,
    #[cfg(feature = "ChUmbrella")]
    ChUmbrella,
    #[cfg(feature = "ChUpload")]
    ChUpload,
    #[cfg(feature = "ChWifi")]
    ChWifi,
    #[cfg(feature = "ChWifiFair")]
    ChWifiFair,
    #[cfg(feature = "ChWifiPoor")]
    ChWifiPoor,
    #[cfg(feature = "ChWifiSlash")]
    ChWifiSlash,
    #[cfg(feature = "ChWifiWarning")]
    ChWifiWarning,
    #[cfg(feature = "ChZoomIn")]
    ChZoomIn,
    #[cfg(feature = "ChZoomOut")]
    ChZoomOut,
}
#[cfg(feature = "ChAnchor")]
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
    data: "<path d=\"m8 5.75v8.25m-4.75-6.25h-1.5c0 4 2.5 6.5 6.25 6.5s6.25-2.5 6.25-6.5h-1.5\" />\n<circle cx=\"8\" cy=\"3.5\" r=\"1.75\" />",
};
#[cfg(feature = "ChApps")]
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
    data: "<rect width=\"4.5\" height=\"4.5\" x=\"1.75\" y=\"1.75\" />\n<rect width=\"4.5\" height=\"4.5\" x=\"1.75\" y=\"9.75\" />\n<rect width=\"4.5\" height=\"4.5\" x=\"9.75\" y=\"9.75\" />\n<rect width=\"4.5\" height=\"4.5\" x=\"9.75\" y=\"1.75\" />",
};
#[cfg(feature = "ChAppsMinus")]
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
    data: "<rect x=\"1.75\" y=\"1.75\" width=\"4.5\" height=\"4.5\" />\n<rect x=\"1.75\" y=\"9.75\" width=\"4.5\" height=\"4.5\" />\n<rect x=\"9.75\" y=\"9.75\" width=\"4.5\" height=\"4.5\" />\n<path d=\"m14.8 3.75h-5\" />",
};
#[cfg(feature = "ChAppsPlus")]
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
    data: "<rect x=\"1.75\" y=\"1.75\" width=\"4.5\" height=\"4.5\" />\n<rect x=\"1.75\" y=\"9.75\" width=\"4.5\" height=\"4.5\" />\n<rect x=\"9.75\" y=\"9.75\" width=\"4.5\" height=\"4.5\" />\n<path d=\"m14.8 3.75h-5m2.5-2.5v5\" />",
};
#[cfg(feature = "ChArchive")]
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
    data: "<rect height=\"3.5\" width=\"12.5\" y=\"2.75\" x=\"1.75\" />\n<path d=\"m6.75 9.25h2.5m-6.5-2.5v7.5h10.5v-7.5\" />",
};
#[cfg(feature = "ChArrowDown")]
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
    data: "<path d=\"m3.25 8.75 4.5 4.5 4.5-4.5m-4.5-6v10.5\" />",
};
#[cfg(feature = "ChArrowDownLeft")]
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
    data: "<path d=\"m10.75 11.75h-6.5v-6.5m7.5-1-7.5 7.5\" />",
};
#[cfg(feature = "ChArrowDownRight")]
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
    data: "<path d=\"m5.25 11.75h6.5v-6.5m-7.5-1 7.5 7.5\" />",
};
#[cfg(feature = "ChArrowLeft")]
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
    data: "<path d=\"m7.25 3.75-4.5 4.5 4.5 4.5m6-4.5h-10.5\" />",
};
#[cfg(feature = "ChArrowRight")]
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
    data: "<path d=\"m8.75 3.25 4.5 4.5-4.5 4.5m-6-4.5h10.5\" />",
};
#[cfg(feature = "ChArrowUp")]
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
    data: "<path d=\"m3.75 7.25 4.5-4.5 4.5 4.5m-4.5 6v-10.5\" />",
};
#[cfg(feature = "ChArrowUpLeft")]
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
    data: "<path d=\"m10.75 4.25h-6.5v6.5m7.5 1-7.5-7.5\" />",
};
#[cfg(feature = "ChArrowUpRight")]
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
    data: "<path d=\"m5.25 4.25h6.5v6.5m-7.5 1 7.5-7.5\" />",
};
#[cfg(feature = "ChAtSign")]
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
    data: "<path d=\"m10.25 8c0 3.25 4 3.25 4 0 0-3.45178-2.7982-6.25-6.25-6.25-3.45178 0-6.25 2.79822-6.25 6.25s2.79822 6.25 6.25 6.25c2.25 0 3.25-1 3.25-1\" />\n<circle cx=\"8\" cy=\"8\" r=\"2.25\" />",
};
#[cfg(feature = "ChAtom")]
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
    data: "<ellipse transform=\"rotate(45)\" cx=\"11.3\" rx=\"8.28\" ry=\"3.17\" />\n<ellipse transform=\"rotate(315)\" cy=\"11.3\" rx=\"8.28\" ry=\"3.17\" />\n<path d=\"m8 8v0\" />",
};
#[cfg(feature = "ChBell")]
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
    data: "<path d=\"m8 1.75c-2.46803 0-4.25 1.5-4.25 3.5v3l-2 3.5h12.5l-2-3.5v-3c0-2-1.16605-3.5-4.25-3.5z\" />\n<path d=\"m5.75 12.25c0 3 4.5 3 4.5 0\" />",
};
#[cfg(feature = "ChBellSlash")]
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
    data: "<path d=\"m5.75 12.25c0 3 4.5 3 4.5 0\" />\n<path d=\"m12.25 8.25v-3c0-2-1.16605-3.5-4.25-3.5m-3.75 2c-.530590.584957-.5.674089-.5 1.5v3l-2 3.5h8.5\" />\n<path d=\"m5.75 12.25c0 3 4.5 3 4.5 0\" />\n<path d=\"m2.75 1.75 10.5 12.5\" />",
};
#[cfg(feature = "ChBin")]
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
    data: "<path d=\"m5.75 4.25v-2.5h4.5v2.5m-6.5 1v9h8.5v-9m-9.5-.5h10.5\" />",
};
#[cfg(feature = "ChBinary")]
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
    data: "<rect height=\"4.5\" width=\"3\" y=\"1.75\" x=\"3.25\" />\n<path d=\"m9.75 6.25h3m-3-4.5h1.5v4\" />\n<rect height=\"4.5\" width=\"3\" y=\"9.75\" x=\"9.75\" />\n<path d=\"m3.25 14.25h3m-3-4.5h1.5v4\" />",
};
#[cfg(feature = "ChBlock")]
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
    data: "<circle cy=\"8\" cx=\"8\" r=\"6.25\" />\n<line x1=\"4.25\" x2=\"12.25\" y1=\"11.75\" y2=\"3.75\" />",
};
#[cfg(feature = "ChBluetooth")]
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
    data: "<path d=\"m3.75 11.25 8.5-6.25-4.5-3.25v12.5l4.5-3.25-8.5-6.25\" />",
};
#[cfg(feature = "ChBluetoothConnected")]
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
    data: "<path d=\"m3.75 11.25 8.5-6.25-4.5-3.25v12.5l4.5-3.25-8.5-6.25\" />\n<path d=\"m1.75 8h1.5m9.5 0h1.5\" />",
};
#[cfg(feature = "ChBluetoothSearching")]
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
    data: "<path d=\"m1.75 11.25 8.5-6.25-4.5-3.25v12.5l4.5-3.25-8.5-6.25\" />\n<path d=\"m13.25 6.25s1 .5 1 1.75-1 1.75-1 1.75m-2-1.75v0\" />",
};
#[cfg(feature = "ChBluetoothSlash")]
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
    data: "<path d=\"m10.75 6.25 1.5-1.25-4.5-3.25v2.5m4.5 6.75-4.5 3.25v-6l-4 3\" />\n<path d=\"m1.75 3.25 12.5 9\" />",
};
#[cfg(feature = "ChBook")]
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
    data: "<path d=\"m11.75 11.75v2m1.5.5h-9c-.75 0-1.5-.5-1.5-1.5s.75-1.5 1.5-1.5h9v-9.5h-9c-.75 0-1.5.75-1.5 1.5v9.5\" />",
};
#[cfg(feature = "ChBookOpen")]
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
    data: "<path d=\"m8 3.75c-1.75-1-2.25-1-6.25-1v9.5c4 0 4.5 0 6.25 1 1.75-1 3.25-1 6.25-1v-9.5c-4 0-4.5 0-6.25 1z\" />\n<path d=\"m8 4.25v8.5\" />",
};
#[cfg(feature = "ChBookmark")]
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
    data: "<polygon points=\"3.75 1.75,12.25 1.75,12.25 14.25,8 9.75,3.75 14.25\" />",
};
#[cfg(feature = "ChBriefcase")]
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
    data: "<rect height=\"9.5\" width=\"12.5\" y=\"4.75\" x=\"1.75\" />\n<path d=\"m1.75 6.25s-.5 3.5 3 3.5h6.5c3.5 0 3-3.5 3-3.5m-8.5-2v-2.5h4.5v2.5\" />",
};
#[cfg(feature = "ChBug")]
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
    data: "<circle cx=\"8\" cy=\"10\" r=\"4.25\" />\n<path d=\"m14.25 10.25h-1.5m-1 2.5 1.5 1.5m0-8.5-1.5 1.5m-10 3h1.5m1 2.5-1.5 1.5m0-8.5 1.5 1.5m1.5-1.5s-.75-3 2.25-3 2.25 3 2.25 3\" />",
};
#[cfg(feature = "ChCalendar")]
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
    data: "<rect height=\"10.5\" width=\"12.5\" y=\"3.75\" x=\"1.75\" />\n<path d=\"m11.25 1.75v1.5m-6.5-1.5v1.5m-2.5 4h11.5\" />",
};
#[cfg(feature = "ChCamera")]
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
    data: "<path d=\"m1.75 4.75v8.5h12.5v-8.5h-3l-1.5-2h-3.5l-1.5 2z\" />\n<circle cx=\"8\" cy=\"8.5\" r=\"2.25\" />",
};
#[cfg(feature = "ChCameraVideo")]
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
    data: "<rect height=\"7.5\" width=\"7.5\" y=\"4.75\" x=\"1.75\" />\n<path d=\"m9.75 7.25 4.5-2.5v7.5l-4.5-2.5\" />",
};
#[cfg(feature = "ChCameraVideoSlash")]
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
    data: "<path d=\"m11.25 10.75 3 1.5v-7.5l-5 2.5v-2.5h-2.5m1.5 7.5h-6.5v-7.5h1.5\" />\n<line x1=\"1.75\" y1=\"2.25\" x2=\"10.25\" y2=\"14.25\" />",
};
#[cfg(feature = "ChCandy")]
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
    data: "<circle cx=\"8\" cy=\"8\" r=\"3.25\" />\n<path d=\"m7.25 11.25c0 1-.5 2.5-1.5 3-.75 0-1.5-1-2-2-1-.5-2-1.5-2-2 .5-1 2-1.5 3-1.5m4-4c0-1 .5-2.5 1.5-3 .75 0 1.5 1 2 2 1 .5 2 1.5 2 2-.5 1-2 1.5-3 1.5\" />",
};
#[cfg(feature = "ChCards")]
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
    data: "<rect height=\"11.5\" width=\"8.25\" y=\"2.75\" x=\"1.75\" />\n<path d=\"m10 3.75 4.25 2-4.25 7.5\" />",
};
#[cfg(feature = "ChCast")]
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
    data: "<path d=\"m1.75 5.25v-2.5h12.5v10.5h-4.5\" />\n<path d=\"m1.75 8.25c2.76142 0 5 2.23858 5 5m-5-2.5c1.38071 0 2.5 1.11929 2.5 2.5m-2.5 0v0\" />",
};
#[cfg(feature = "ChCertificate")]
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
    data: "<polyline points=\"11.25 1.75,2.75 1.75,2.75 13.25,5.25 13.25\" />\n<polyline points=\"8.75 9.75,8.25 14.25,10.50 13.25,12.75 14.25,12.25 9.75\" />\n<circle cx=\"10.5\" cy=\"7.5\" r=\"2.75\" />",
};
#[cfg(feature = "ChChartBar")]
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
    data: "<path d=\"m1.75 1.75v12.5h12.5m-9-3v-2.5m4 2.5v-5.5m4 5.5v-8.5\" />",
};
#[cfg(feature = "ChChartLine")]
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
    data: "<path d=\"m4.75 11.25 2.5-4.5 2.5 2.5 3.5-6m-11.5-1.5v12.5h12.5\" />",
};
#[cfg(feature = "ChChevronDown")]
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
    data: "<path d=\"m3.75 5.75 4.25 4.5 4.25-4.5\" />",
};
#[cfg(feature = "ChChevronLeft")]
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
    data: "<path d=\"m10.25 3.75-4.5 4.25l4.5 4.25\" />",
};
#[cfg(feature = "ChChevronRight")]
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
    data: "<path d=\"m5.75 12.25 4.5-4.25-4.5-4.25\" />",
};
#[cfg(feature = "ChChevronUp")]
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
    data: "<path d=\"m12.25 10.25-4.25-4.5-4.25 4.5\" />",
};
#[cfg(feature = "ChChevronsDown")]
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
    data: "<path d=\"m3.75 3.75 4.25 4.5 4.25-4.5m-8.5 5 4.25 4.5 4.25-4.5\" />",
};
#[cfg(feature = "ChChevronsLeft")]
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
    data: "<path d=\"m12.25 3.75-4.5 4.25l4.5 4.25m-5-8.5-4.5 4.25 4.5 4.25\" />",
};
#[cfg(feature = "ChChevronsRight")]
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
    data: "<path d=\"m3.75 12.25 4.5-4.25-4.5-4.25m5 8.5l4.5-4.25-4.5-4.25\" />",
};
#[cfg(feature = "ChChevronsUp")]
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
    data: "<path d=\"m12.25 12.25-4.25-4.5-4.25 4.5m8.5-5l-4.25-4.5-4.25 4.5\" />",
};
#[cfg(feature = "ChChevronsUpDown")]
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
    data: "<path d=\"m11.25 10.75-3.25 3.5-3.25-3.5\" />\n<path d=\"m11.25 5.25-3.25-3.5-3.25 3.5\" />",
};
#[cfg(feature = "ChChip")]
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
    data: "<rect height=\"10.5\" width=\"10.5\" y=\"2.75\" x=\"2.75\" />\n<rect height=\"3.5\" width=\"3.5\" y=\"6.25\" x=\"6.25\" />\n<path d=\"m2.25 10.25h-1m1-4.5h-1m13.5 4.5h-1m1-4.5h-1m-3.5 8v1m-4.5-1v1m4.5-13.5v1m-4.5-1v1\" />",
};
#[cfg(feature = "ChCircle")]
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
    data: "<circle cx=\"8\" cy=\"8\" r=\"6.25\" />",
};
#[cfg(feature = "ChCircleCross")]
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
    data: "<path d=\"m10.25 5.75-4.5 4.5m0-4.5 4.5 4.5\" />\n<circle cx=\"8\" cy=\"8\" r=\"6.25\" />",
};
#[cfg(feature = "ChCircleMinus")]
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
    data: "<circle cx=\"8\" cy=\"8\" r=\"6.25\" />\n<line x1=\"4.75\" y1=\"8\" x2=\"11.25\" y2=\"8\" />",
};
#[cfg(feature = "ChCircleTick")]
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
    data: "<path d=\"m14.25 8.75c-.5 2.5-2.3849 4.85363-5.03069 5.37991-2.64578.5263-5.33066-.7044-6.65903-3.0523-1.32837-2.34784-1.00043-5.28307.81336-7.27989 1.81379-1.99683 4.87636-2.54771 7.37636-1.54771\" />\n<polyline points=\"5.75 7.75,8.25 10.25,14.25 3.75\" />",
};
#[cfg(feature = "ChCircleWarning")]
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
    data: "<circle cx=\"8\" cy=\"8\" r=\"6.25\" />\n<path d=\"m8 10.75v0m0-6v3.5\" />",
};
#[cfg(feature = "ChClipboard")]
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
    data: "<rect height=\"3.5\" width=\"4.5\" y=\"1.75\" x=\"5.75\" />\n<path d=\"m5.25 2.75h-2.5v11.5h10.5v-11.5h-2.5\" />",
};
#[cfg(feature = "ChClipboardTick")]
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
    data: "<rect x=\"5.75\" y=\"1.75\" width=\"4.5\" height=\"3.5\" />\n<path d=\"m9.75 12.8 1.5 1.5 3-2.5m-9-9h-2.5v11.5h4.5m6-5v-6.5h-2.5\" />",
};
#[cfg(feature = "ChClock")]
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
    data: "<circle cy=\"8\" cx=\"8\" r=\"6.25\" />\n<path d=\"m8.25 4.75v3.5l-2.5 2\" />",
};
#[cfg(feature = "ChClockAlarm")]
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
    data: "<path d=\"m11.75 1.75 2.5 2m-10-2-2.5 2m10.5 9.5 1 1m-9.5-1-1 1m5.5-7.5v2.5l-1.5 1\" />\n<circle cx=\"8\" cy=\"9\" r=\"5.25\" />",
};
#[cfg(feature = "ChCloud")]
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
    data: "<path d=\"m7 3.75c-1.79493 0-3.25 1.45507-3.25 3.25.00152.254757.032983.508452.09375.755859h-.00195c-1.17822.08305-2.09165 1.063-2.0918 2.24414 0 1.24264 1.00736 2.25 2.25 2.25h7.5c1.51878 0 2.75-1.23122 2.75-2.75s-1.2312-2.75-2.75-2.75c-.4352-.00022-.8643.10286-1.252.30078.0008-.01692.0015-.03385.0020-.05078 0-1.79493-1.45507-3.25-3.25-3.25z\" />",
};
#[cfg(feature = "ChClover")]
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
    data: "<path d=\"m4.75 2.75c-.50 1.5 1.25 3.25 3.25 5.25 2-2 3.75-3.75 3.25-5.25s-2.5-1-3.25.50c-.75-1.5-2.75-2-3.25-.50zm3.25 5.25c2 2 3.75 3.75 5.25 3.25s1-2.5-.5-3.25c1.5-.75 2-2.75.5-3.25s-3.25 1.25-5.25 3.25zm0 0c-2 2-3.75 3.75-3.25 5.25s2.5 1 3.25-.5c.75 1.5 2.75 2 3.25.5s-1.25-3.25-3.25-5.25zm0 0c-2-2-3.75-3.75-5.25-3.25s-1 2.5.5 3.25c-1.5.75-2 2.75-.5 3.25s3.25-1.25 5.25-3.25z\" />",
};
#[cfg(feature = "ChCode")]
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
    data: "<path d=\"m5.25 11.25-3.5-3.25 3.5-3.25m5.5 6.5 3.5-3.25-3.5-3.25\" />",
};
#[cfg(feature = "ChCoffee")]
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
    data: "<path d=\"m10.75 11.25c4.5 0 4.5-5.5 0-5.5h-9v5c0 5 8.5 5 8.5 0v-5\" />\n<path d=\"m8.75 1.75v1.5m-3-1.5v1.5m-3-1.5v1.5\" />",
};
#[cfg(feature = "ChCog")]
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
    data: "<circle cx=\"8\" cy=\"8\" r=\"1.75\" />\n<path d=\"m6.75 1.75-.5 1.5-1.5 1-2-.5-1 2 1.5 1.5v1.5l-1.5 1.5 1 2 2-.5 1.5 1 .5 1.5h2.5l.5-1.5 1.5-1 2 .5 1-2-1.5-1.5v-1.5l1.5-1.5-1-2-2 .5-1.5-1-.5-1.5z\" />",
};
#[cfg(feature = "ChCompass")]
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
    data: "<circle cy=\"8\" cx=\"8\" r=\"6.25\" />\n<polygon points=\"6.75 6.75,5.75 10.75,9.25 9.25,10.25 5.25\" />",
};
#[cfg(feature = "ChConicalFlask")]
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
    data: "<path d=\"m4.75 1.75h6.5m-6.5 8h6.5m-5.5-7.5v4.5l-4 7.5h12.5l-4-7.5v-4.5\" />",
};
#[cfg(feature = "ChContainer")]
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
    data: "<path d=\"m1.75 12.2 5.5 2 7-4.5v-6l-5.5-2-7 4.5z\" />\n<path d=\"m10.8 6.25v5.5m-3.5-3.5v6m-5.5-8 5.5 2 7-4.5\" />",
};
#[cfg(feature = "ChCopy")]
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
    data: "<path d=\"m11.25 4.25v-2.5h-9.5v9.5h2.5m.5-6.5v9.5h9.5v-9.5z\" />",
};
#[cfg(feature = "ChCopyleft")]
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
    data: "<circle cy=\"8\" cx=\"8\" r=\"6.25\" />\n<path d=\"m6 6.75s.75-1 2-1 2.25 1 2.25 2.25-1 2.25-2.25 2.25-2-1-2-1\" />",
};
#[cfg(feature = "ChCopyright")]
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
    data: "<circle cy=\"8\" cx=\"8\" r=\"6.25\" />\n<path d=\"m10 6.75s-.75-1-2-1-2.25 1-2.25 2.25 1 2.25 2.25 2.25 2-1 2-1\" />",
};
#[cfg(feature = "ChCreditCard")]
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
    data: "<rect height=\"9.5\" width=\"12.5\" y=\"3.75\" x=\"1.75\" />\n<path d=\"m9.75 10.25h1.5m-9-3h11.5\" />",
};
#[cfg(feature = "ChCrop")]
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
    data: "<path d=\"m4.25 1.75v10h10\" />\n<path d=\"m11.8 14.2v-2.5m0-2.5v-5h-5m-2.5 0h-2.5\" />",
};
#[cfg(feature = "ChCross")]
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
    data: "<path d=\"m11.25 4.75-6.5 6.5m0-6.5 6.5 6.5\" />",
};
#[cfg(feature = "ChCrosshair")]
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
    data: "<path d=\"m8 5.25v-3m0 11.5v-3m2.75-2.75h3m-11.5 0h3\" />\n<circle cx=\"8\" cy=\"8\" r=\"6.25\" />",
};
#[cfg(feature = "ChCube")]
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
    data: "<polygon points=\"1.75 4.75 8 1.25 14.25 4.75 14.25 11.25 8 14.75 1.75 11.25\" />\n<path d=\"m8 14v-6m5.75-3-5.75 3m-6-3 6 3\" />",
};
#[cfg(feature = "ChCursor")]
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
    data: "<polygon points=\"1.75 1.75,6.25 14.25,8.75 8.75,14.25 6.25\" />\n<line x1=\"9.25\" y1=\"9.25\" x2=\"13.25\" y2=\"13.25\" />",
};
#[cfg(feature = "ChDatabase")]
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
    data: "<path d=\"m8 1.75c-3.75 0-5.25 2-5.25 2v4.5 4s1.5 2 5.25 2 5.25-2 5.25-2v-4-4.5s-1.5-2-5.25-2z\" />\n<path d=\"m2.75 8.25s1.5 2 5.25 2 5.25-2 5.25-2m-10.5-4s1.5 2 5.25 2 5.25-2 5.25-2\" />",
};
#[cfg(feature = "ChDiamond")]
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
    data: "<polygon points=\"1.25 8,8 14.75,14.75 8,8 1.25\" />",
};
#[cfg(feature = "ChDiff")]
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
    data: "<path d=\"m3.75 13.75h8m0-7.5h-8m4-4v8\" />",
};
#[cfg(feature = "ChDisc")]
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
    data: "<circle cx=\"8\" cy=\"8\" r=\"6.25\" />\n<circle cx=\"8\" cy=\"8\" r=\"1.75\" />",
};
#[cfg(feature = "ChDownload")]
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
    data: "<path d=\"m3.25 13.25h9m-8.5-6.5 4 3.5 4-3.5m-4-5v8.5\" />",
};
#[cfg(feature = "ChDroplet")]
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
    data: "<path d=\"m2.75 9c0 2.9 2.35 5.25 5.25 5.25s5.25-2.35 5.25-5.25c0-3.25-5.25-7.25-5.25-7.25s-5.25 4-5.25 7.25z\" />",
};
#[cfg(feature = "ChEraser")]
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
    data: "<polyline points=\"14.25 13.25,4.75 13.25,1.75 10.25,9.25 2.75,14.25 7.75,8.75 13.25\" />\n<line x1=\"5.25\" y1=\"6.75\" x2=\"10.25\" y2=\"11.75\" />",
};
#[cfg(feature = "ChEye")]
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
    data: "<path d=\"m1.75 8s2-4.25 6.25-4.25 6.25 4.25 6.25 4.25-2 4.25-6.25 4.25-6.25-4.25-6.25-4.25z\" />\n<circle cx=\"8\" cy=\"8\" r=\"1.25\" fill=\"currentColor\" />",
};
#[cfg(feature = "ChEyeSlash")]
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
    data: "<path d=\"m8.75 3.75c3.5.5 5.5 4.25 5.5 4.25s-.5 1.25-1.5 2.25m-2.5 1.5c-6 2-8.5-3.75-8.5-3.75s.5-1.75 3-3.25\" />\n<path d=\"m8.625 9.08253a1.25 1.25 0 0 1 -1.64894 -.36556 1.25 1.25 0 0 1 .22046 -1.67453l.80348.95756z\" fill=\"currentColor\" />\n<path d=\"m3.75 1.75 8.5 12.5\" />",
};
#[cfg(feature = "ChFaceFrown")]
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
    data: "<circle cx=\"8\" cy=\"8\" r=\"6.25\" />\n<path d=\"m9.75 6.25v-.5m-3.5.5v-.5m-.5 5s.5-1 2.25-1 2.25 1 2.25 1\" />",
};
#[cfg(feature = "ChFaceNeutral")]
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
    data: "<circle cx=\"8\" cy=\"8\" r=\"6.25\" />\n<path d=\"m9.75 6.25v-.5m-3.5.5v-.5m-.5 4.5h4.5\" />",
};
#[cfg(feature = "ChFaceSmile")]
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
    data: "<circle cx=\"8\" cy=\"8\" r=\"6.25\" />\n<path d=\"m9.75 6.25v-.5m-3.5.5v-.5m-.5 4s.5 1.5 2.25 1.5 2.25-1.5 2.25-1.5\" />",
};
#[cfg(feature = "ChFile")]
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
    data: "<polygon points=\"2.75 1.75,8.25 1.75,13.25 6.75,13.25 14.25,2.75 14.25\" />\n<polyline points=\"7.75 2.25,7.75 7.25,12.75 7.25\" />",
};
#[cfg(feature = "ChFileBinary")]
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
    data: "<polyline points=\"2.75 7.75,2.75 1.75,8.25 1.75,13.25 6.75,13.25 14.25\" />\n<rect x=\"1.75\" y=\"10.8\" width=\"3\" height=\"3.5\" />\n<path d=\"m7.25 14.2h3m-3-3.5h1.5v3\" />\n<polyline points=\"7.75 2.25 7.75 7.25 12.8 7.25\" />",
};
#[cfg(feature = "ChFileCode")]
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
    data: "<polyline points=\"2.75 7.75,2.75 1.75,8.25 1.75,13.25 6.75,13.25 14.25,11.25 14.25\" />\n<polyline points=\"7.75 2.25 7.75 7.25 12.8 7.25\" />\n<path d=\"m6.75 10.8 2 1.75-2 1.75m-3-3.5-2 1.75 2 1.75\" />",
};
#[cfg(feature = "ChFileSymlink")]
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
    data: "<polyline points=\"2.75 7.75,2.75 1.75,8.25 1.75,13.25 6.75,13.25 14.25,9.25 14.25\" />\n<polyline points=\"7.75 2.25 7.75 7.25 12.75 7.25\" />\n<path d=\"m2.75 14.25 3.5-3.5m0 3v-3h-3\" />",
};
#[cfg(feature = "ChFiles")]
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
    data: "<polygon points=\"9.25 1.75,13.25 5.75,13.25 11.25,5.75 11.25,5.75 1.75\" />\n<polyline points=\"9.25 2.25,9.25 5.75,12.75 5.75\" />\n<polyline points=\"10.25 11.75,10.25 14.25,2.75 14.25,2.75 4.75,5.25 4.75\" />",
};
#[cfg(feature = "ChFilter")]
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
    data: "<polygon points=\"1.75 1.75,14.25 1.75,14.25 3.25,9.25 8.75,9.25 12.75,6.75 14.25,6.75 8.75,1.75 3.25\" />",
};
#[cfg(feature = "ChFlag")]
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
    data: "<path d=\"m1.75 14.25v-11s2-1.5 4-1.5 2.5 1.5 4.5 1.5 4-1.5 4-1.5v7s-2 1.5-4 1.5-2.5-1.5-4.5-1.5-4 1.5-4 1.5\" />",
};
#[cfg(feature = "ChFlame")]
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
    data: "<path d=\"m8.25 7.75c2 2 2.5-2.5 3.5-2s1.5 2 1.5 3.25c0 3.25-2.35 5.25-5.25 5.25s-5.25-2.5-5.25-6 3.5-7 5.5-7c0 0-2 4.5 0 6.5z\" />",
};
#[cfg(feature = "ChFloppyDisk")]
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
    data: "<polygon points=\"2.75 2.75,2.75 13.25,13.25 13.25,13.25 5.75,10.25 2.75\" />\n<polyline points=\"5.75 13.25,5.75 9.75,10.25 9.75,10.25 13.25\" />",
};
#[cfg(feature = "ChFolder")]
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
    data: "<polygon points=\"1.75 2.75,1.75 13.25,14.25 13.25,14.25 4.75,8.25 4.75,6.75 2.75\" />",
};
#[cfg(feature = "ChFolderSymlink")]
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
    data: "<path d=\"m1.75 13.25 3.5-3.5m0 3v-3h-3\" />\n<polyline points=\"8.25 13.25,14.25 13.25,14.25 4.75,8.25 4.75,6.75 2.75,1.75 2.75,1.75 6.75\" />",
};
#[cfg(feature = "ChFolders")]
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
    data: "<polygon points=\"4.75 2.25,4.75 10.25,14.25 10.25,14.25 3.75,9.25 3.75,7.75 2.25\" />\n<polyline points=\"4.75 5.25,1.75 5.25,1.75 13.25,11.25 13.25,11.25 10.25\" />",
};
#[cfg(feature = "ChForward")]
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
    data: "<path d=\"m1.75 13.25c.5-6 5.5-7.5 8-7v-3.5l4.5 5.25-4.5 5.25v-3.5c-2.5-0.5-6.5 0.5-8 3.5z\" />",
};
#[cfg(feature = "ChGamepad")]
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
    data: "<path d=\"m3.25 3.75c-2 5-2 9 0 9.5s2.5-2 2.5-2h4.5s.5 2.5 2.5 2 2-4.5 0-9.5h-2l-1 1h-3.5l-1-1z\" />",
};
#[cfg(feature = "ChGem")]
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
    data: "<polygon points=\"4.75 2.75,11.25 2.75,14.25 6.25,8 13.25,1.75 6.25\" />",
};
#[cfg(feature = "ChGift")]
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
    data: "<rect height=\"3.5\" width=\"12.5\" y=\"4.75\" x=\"1.75\" />\n<path d=\"m10.25 4.75h-2.25c0-2 .5-3 2.25-3 2 0 2 3 0 3zm-4.5 0h2.25c0-2-.5-3-2.25-3-2 0-2 3 0 3zm2.25 9v-8.75m-5.25 3.75v5.5h10.5v-5.5\" />",
};
#[cfg(feature = "ChGitBranch")]
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
    data: "<circle cx=\"4.5\" cy=\"3.5\" r=\"1.75\" />\n<circle cx=\"11.5\" cy=\"3.5\" r=\"1.75\" />\n<circle cx=\"4.5\" cy=\"12.5\" r=\"1.75\" />\n<path d=\"m5.25 8.25c3 0 6 .5 6-2.5m-6.5 4.5v-4.5\" />",
};
#[cfg(feature = "ChGitCherryPick")]
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
    data: "<circle cy=\"8\" cx=\"5\" r=\"2.25\" />\n<path d=\"m5 10.75v3.5m0-12.5v3.5\" />\n<path d=\"m11.75 8h1.5m-4.5-3.25h1.5l1 3.25-1 3.25h-1.5\" />",
};
#[cfg(feature = "ChGitCommit")]
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
    data: "<circle cx=\"8\" cy=\"8\" r=\"2.25\" />\n<path d=\"m8 10.75v3.5m0-12.5v3.5\" />",
};
#[cfg(feature = "ChGitCompare")]
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
    data: "<circle cx=\"12.5\" cy=\"12.5\" r=\"1.75\" />\n<circle cx=\"3.5\" cy=\"3.5\" r=\"1.75\" />\n<path d=\"m3.75 5.75v5c0 1 .5 1.5 1.5 1.5h2m-.5 2 1.5-2-1.5-2m5.5 0v-5c0-1-.5-1.5-1.5-1.5h-2m.5-2-1.5 2 1.5 2\" />",
};
#[cfg(feature = "ChGitFork")]
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
    data: "<circle cx=\"8\" cy=\"12.5\" r=\"1.75\" />\n<circle cx=\"4.5\" cy=\"3.5\" r=\"1.75\" />\n<circle cx=\"11.5\" cy=\"3.5\" r=\"1.75\" />\n<path d=\"m8 8.75v1.5m-3.25-4.5c0 3.5 6.5 3.5 6.5 0\" />",
};
#[cfg(feature = "ChGitMerge")]
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
    data: "<circle cx=\"4.5\" cy=\"3.5\" r=\"1.75\" />\n<circle cx=\"4.5\" cy=\"12.5\" r=\"1.75\" />\n<circle cx=\"12.5\" cy=\"8.5\" r=\"1.75\" />\n<path d=\"m4.75 10.25v-4.5c1 2 2 3 5.5 3\" />",
};
#[cfg(feature = "ChGitRequest")]
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
    data: "<circle cx=\"12.5\" cy=\"12.5\" r=\"1.75\" />\n<circle cx=\"3.5\" cy=\"12.5\" r=\"1.75\" />\n<circle cx=\"3.5\" cy=\"3.5\" r=\"1.75\" />\n<path d=\"m9.25 1.75-1.5 2 1.5 2m3 4.5v-5c0-1-.5-1.5-1.5-1.5h-2m-5 2v4.5\" />",
};
#[cfg(feature = "ChGitRequestCross")]
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
    data: "<circle cy=\"12.5\" cx=\"12.5\" r=\"1.75\" />\n<circle cy=\"12.5\" cx=\"3.5\" r=\"1.75\" />\n<circle cy=\"3.5\" cx=\"3.5\" r=\"1.75\" />\n<path d=\"m12.25 7.25v3m-8.5-4.5v4.5\" />\n<path d=\"m14.25 1.75-3.5 3.5m0-3.5 3.5 3.5\" />",
};
#[cfg(feature = "ChGitRequestDraft")]
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
    data: "<circle cy=\"12.5\" cx=\"12.5\" r=\"1.75\" />\n<circle cy=\"12.5\" cx=\"3.5\" r=\"1.75\" />\n<circle cy=\"3.5\" cx=\"3.5\" r=\"1.75\" />\n<path d=\"m7.75 2.75h.5m2.5 0h.5m1.5 2.5v-.5m0 3v.5m-9-2.5v4.5\" />",
};
#[cfg(feature = "ChGithub")]
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
    data: "<path d=\"m5.75 14.25s-.5-2 .5-3c0 0-2 0-3.5-1.5s-1-4.5 0-5.5c-.5-1.5.5-2.5.5-2.5s1.5 0 2.5 1c1-.5 3.5-.5 4.5 0 1-1 2.5-1 2.5-1s1 1 .5 2.5c1 1 1.5 4 0 5.5s-3.5 1.5-3.5 1.5c1 1 .5 3 .5 3\" />\n<path d=\"m5.25 13.75c-1.5.5-3-.5-3.5-1\" />",
};
#[cfg(feature = "ChGitlab")]
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
    data: "<path d=\"m8 14.25-6.25-4.5 2-8 2 5.5h4.5l2-5.5 2 8z\" />",
};
#[cfg(feature = "ChGlasses")]
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
    data: "<circle cx=\"4\" cy=\"11\" r=\"2.25\" />\n<circle cx=\"12\" cy=\"11\" r=\"2.25\" />\n<path d=\"m14.25 10.75c-1.5-6-2-6.5-3.5-7m-9 7c1.5-6 2-6.5 3.5-7m1 7c1-1 2.5-1 3.5 0\" />",
};
#[cfg(feature = "ChGlobe")]
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
    data: "<circle cy=\"8\" cx=\"8\" r=\"6.25\" />\n<path d=\"m2 8.25h12\" />\n<path d=\"m8.25 14.2c2.75-3.2 2.75-9.2 0-12.4\" />\n<path d=\"m7.75 14.2c-2.75-3.2-2.75-9.2 0-12.4\" />",
};
#[cfg(feature = "ChGrabHorizontal")]
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
    data: "<circle cy=\"5.5\" cx=\"2.5\" r=\".75\" />\n<circle cy=\"5.5\" cx=\"8\" r=\".75\" />\n<circle cy=\"5.5\" cx=\"13.5\" r=\".75\" />\n<circle cy=\"10.5\" cx=\"2.5\" r=\".75\" />\n<circle cy=\"10.5\" cx=\"8\" r=\".75\" />\n<circle cy=\"10.5\" cx=\"13.5\" r=\".75\" />",
};
#[cfg(feature = "ChGrabVertical")]
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
    data: "<circle cy=\"2.5\" cx=\"5.5\" r=\".75\" />\n<circle cy=\"8\" cx=\"5.5\" r=\".75\" />\n<circle cy=\"13.5\" cx=\"5.5\" r=\".75\" />\n<circle cy=\"2.5\" cx=\"10.4957\" r=\".75\" />\n<circle cy=\"8\" cx=\"10.4957\" r=\".75\" />\n<circle cy=\"13.5\" cx=\"10.4957\" r=\".75\" />",
};
#[cfg(feature = "ChGraduateCap")]
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
    data: "<path d=\"m14.25 9.25v-3.25l-6.25-3.25-6.25 3.25 6.25 3.25 3.25-1.5v3.5c0 1-1.5 2-3.25 2s-3.25-1-3.25-2v-3.5\" />",
};
#[cfg(feature = "ChHash")]
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
    data: "<path d=\"m2.75 10.25h9.5m-8.5-4.5h9.5m-2.5-4-1.5 12.5m-2.5-12.5-1.5 12.5\" />",
};
#[cfg(feature = "ChHeadphones")]
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
    data: "<path d=\"m1.75 11.75c0-2.5 3.5-2 3.5-2v4.5s-3.5.5-3.5-2.5v-3.5c0-3 .5-6.5 6.25-6.5s6.25 3.5 6.25 6.5v3.5c0 3-3.5 2.5-3.5 2.5v-4.5s3.5-.5 3.5 2\" />",
};
#[cfg(feature = "ChHeart")]
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
    data: "<path d=\"m3.25 9.75c3 3.5 4.75 4.5 4.75 4.5s1.75-1 4.75-4.5 1-7-1.5-7-3.25 3-3.25 3-.75-3-3.25-3-4.5 3.5-1.5 7z\" />",
};
#[cfg(feature = "ChHelp")]
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
    data: "<circle cy=\"8\" cx=\"8\" r=\"6.25\" />\n<path d=\"m5.75 6.75c0-1 1-2 2.25-2s2.25 1.0335 2.25 2c0 1.5-1.5 1.5-2.25 2m0 2.5v0\" />",
};
#[cfg(feature = "ChHexagon")]
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
    data: "<polygon points=\"1.75 4.75,8 1.25,14.25 4.75,14.25 11.25,8 14.75,1.75 11.25\" />",
};
#[cfg(feature = "ChHome")]
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
    data: "<path d=\"m3.75 5.75v7.5h8.5v-7.5m-10.5 1.5 6.25-5.5l6.25 5.5\" />",
};
#[cfg(feature = "ChHourglass")]
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
    data: "<path d=\"m11.75 13.75c0-5-2-4-2-5.75s2-0.75 2-5.75m-7.5 11.5c0-5 2-4 2-5.75s-2-.75-2-5.75m-1.5-.5h10.5m-10.5 12.5h10.5\" />",
};
#[cfg(feature = "ChId")]
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
    data: "<rect height=\"10.5\" width=\"12.5\" y=\"2.75\" x=\"1.75\" />\n<circle cy=\"7.5\" cx=\"8\" r=\"2.25\" />\n<path d=\"m4.75 12.75c0-1 .75-3 3.25-3s3.25 2 3.25 3\" />",
};
#[cfg(feature = "ChImage")]
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
    data: "<rect x=\"1.75\" y=\"2.75\" width=\"12.5\" height=\"10.5\" />\n<path d=\"m3.75 13.2 6.5-5.5 4 3\" />\n<circle fill=\"currentColor\" cx=\"5.25\" cy=\"6.25\" r=\".5\" />",
};
#[cfg(feature = "ChInbox")]
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
    data: "<polygon points=\"1.75 13.25,14.25 13.25,14.25 8.25,11.75 2.75,4.25 2.75,1.75 8.25\" />\n<path d=\"m2.25 8.75h3l1 1.5h3.5l1-1.5h3\" />",
};
#[cfg(feature = "ChInfinity")]
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
    data: "<path d=\"m5 5c2.5 1 3.5 5 6 6s3.25-1.25 3.25-3-.75-4-3.25-3-3.5 5-6 6-3.25-1.25-3.25-3 .75-4 3.25-3z\" />",
};
#[cfg(feature = "ChInfo")]
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
    data: "<circle cy=\"8\" cx=\"8\" r=\"6.25\" />\n<path d=\"m8 5.25v0m0 6v-3.5\" />",
};
#[cfg(feature = "ChKey")]
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
    data: "<path d=\"m10 1.75c-2.34721 0-4.25 1.90279-4.25 4.25.00023.37267.04949.74369.14648 1.10352l-4.14648 4.14648v3h3v-1.5h1.5v-1.5h1.5l1.15039-1.15039c.35839.0980.72808.1486 1.09961.1504 2.3472 0 4.25-1.90279 4.25-4.25s-1.9028-4.25-4.25-4.25z\" />\n<circle cx=\"10.75\" cy=\"5.25\" r=\"0.5\" fill=\"currentColor\" />",
};
#[cfg(feature = "ChLaptop")]
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
    data: "<rect height=\"7.5\" width=\"10.5\" y=\"2.75\" x=\"2.75\" />\n<path d=\"m2.75 10.25-1 3h12.5l-1-3\" />",
};
#[cfg(feature = "ChLayoutColumns")]
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
    data: "<rect height=\"10.5\" width=\"12.5\" y=\"2.75\" x=\"1.75\" />\n<line x1=\"8\" y1=\"3.25\" x2=\"8\" y2=\"12.75\" />",
};
#[cfg(feature = "ChLayoutDashboard")]
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
    data: "<rect height=\"10.5\" width=\"12.5\" y=\"2.75\" x=\"1.75\" />\n<path d=\"m8.25 6.75h5.5m-11.5 2.5h5.5m.25-6v9.5\" />",
};
#[cfg(feature = "ChLayoutGrid")]
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
    data: "<rect height=\"10.5\" width=\"12.5\" y=\"2.75\" x=\"1.75\" />\n<path d=\"m2 8h12m-3.75-4.75v9.5m-4.5-9.5v9.5\" />",
};
#[cfg(feature = "ChLayoutList")]
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
    data: "<rect height=\"10.5\" width=\"12.5\" y=\"2.75\" x=\"1.75\" />\n<path d=\"m5.25 3.25v9.5m-3-6.5h11.5m-11.5 3.5h11.5\" />",
};
#[cfg(feature = "ChLayoutRows")]
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
    data: "<rect height=\"10.5\" width=\"12.5\" y=\"2.75\" x=\"1.75\" />\n<line x1=\"2\" y1=\"8\" x2=\"14\" y2=\"8\" />",
};
#[cfg(feature = "ChLayoutSidebar")]
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
    data: "<rect height=\"10.5\" width=\"12.5\" y=\"2.75\" x=\"1.75\" />\n<path d=\"m6.25 3v9.5\" />",
};
#[cfg(feature = "ChLayoutStackH")]
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
    data: "<rect height=\"10.5\" width=\"12.5\" y=\"2.75\" x=\"1.75\" />\n<line x1=\"2\" y1=\"8\" x2=\"14\" y2=\"8\" />\n<line x1=\"8\" y1=\"8\" x2=\"8\" y2=\"12.75\" />",
};
#[cfg(feature = "ChLayoutStackV")]
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
    data: "<rect height=\"10.5\" width=\"12.5\" y=\"2.75\" x=\"1.75\" />\n<line x1=\"8\" y1=\"3.25\" x2=\"8\" y2=\"12.75\" />\n<line x1=\"8\" y1=\"8\" x2=\"14\" y2=\"8\" />",
};
#[cfg(feature = "ChLightbulb")]
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
    data: "<path d=\"m6.75 14.25h2.5m-1.25-12.5c-2.75 0-4.25 2-4.25 4s2 2.5 2 4.5v1h4.5v-1c0-2 2-2.5 2-4.5s-1.5-4-4.25-4z\" />",
};
#[cfg(feature = "ChLightningBolt")]
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
    data: "<polygon points=\"9.25 1.75,2.75 9.25,7.25 9.75,6.75 14.25,13.25 6.75,8.75 6.25\" />",
};
#[cfg(feature = "ChLink")]
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
    data: "<path d=\"m9.75 4.75c3 0 4.5 1.5 4.5 3.25s-1.5 3.25-4.5 3.25m-4-3.25h4.5m-4-3.25c-3 0-4.5 1.5-4.5 3.25s1.5 3.25 4.5 3.25\" />",
};
#[cfg(feature = "ChLinkExternal")]
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
    data: "<polyline points=\"8.25 2.75,2.75 2.75,2.75 13.25,13.25 13.25,13.25 7.75\" />\n<path d=\"m13.25 2.75-5.5 5.5m3-6.5h3.5v3.5\" />",
};
#[cfg(feature = "ChLinkSlash")]
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
    data: "<path d=\"m10.75 1.75-5.5 12.5m4.5-9.5c3 0 4.5 1.5 4.5 3.25s-1.5 3.25-4.5 3.25m-3.5-6.5c-3 0-4.5 1.5-4.5 3.25s1.5 3.25 4.5 3.25\" />",
};
#[cfg(feature = "ChMail")]
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
    data: "<rect height=\"9.5\" width=\"12.5\" y=\"3.75\" x=\"1.75\" />\n<path d=\"m2.25 4.25 5.75 5 5.75-5\" />",
};
#[cfg(feature = "ChMap")]
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
    data: "<path d=\"m10.25 5.25v8.5m-4.5-10.5v8.5m-4 2.5v-9.5l4-2 4.5 2 4-2v9.5l-4 2-4.5-2z\" />",
};
#[cfg(feature = "ChMapPin")]
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
    data: "<path d=\"m13.25 7c0 3.75-5.25 7.25-5.25 7.25s-5.25-3.5-5.25-7.25c0-2.89949 2.35051-5.25 5.25-5.25 2.8995 0 5.25 2.35051 5.25 5.25z\" />\n<circle cx=\"8\" cy=\"7\" r=\"1.25\" fill=\"currentColor\" />",
};
#[cfg(feature = "ChMediaBack")]
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
    data: "<polygon points=\"13.25 13.25,4.75 8,13.25 2.75\" />\n<line x1=\"1.75\" y1=\"3.75\" x2=\"1.75\" y2=\"12.25\" />",
};
#[cfg(feature = "ChMediaEject")]
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
    data: "<polygon points=\"2.75 11.25,13.25 11.25,8 2.75\" />\n<line x1=\"13.25\" y1=\"14.25\" x2=\"2.75\" y2=\"14.25\" />",
};
#[cfg(feature = "ChMediaFastForward")]
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
    data: "<polygon points=\"8.25 3.75,8.25 12.25,14.25 8\" />\n<polygon points=\"1.75 3.75,1.75 12.25,7.75 8\" />",
};
#[cfg(feature = "ChMediaPause")]
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
    data: "<rect height=\"10.5\" width=\"3.5\" y=\"2.75\" x=\"2.75\" />\n<rect height=\"10.5\" width=\"3.5\" y=\"2.75\" x=\"9.75\" />",
};
#[cfg(feature = "ChMediaPlay")]
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
    data: "<polygon points=\"2.75 2.75,2.75 13.25,12.25 8\" />",
};
#[cfg(feature = "ChMediaRewind")]
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
    data: "<polygon points=\"7.75 3.75,7.75 12.25,1.75 8\" />\n<polygon points=\"14.25 3.75,14.25 12.25,8.25 8\" />",
};
#[cfg(feature = "ChMediaSkip")]
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
    data: "<polygon points=\"2.75 13.25,11.25 8,2.75 2.75\" />\n<line x1=\"14.25\" y1=\"3.75\" x2=\"14.25\" y2=\"12.25\" />",
};
#[cfg(feature = "ChMenuHamburger")]
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
    data: "<path d=\"m2.75 12.25h10.5m-10.5-4h10.5m-10.5-4h10.5\" />",
};
#[cfg(feature = "ChMenuKebab")]
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
    data: "<circle cx=\"8\" cy=\"2.5\" r=\".75\" />\n<circle cx=\"8\" cy=\"8\" r=\".75\" />\n<circle cx=\"8\" cy=\"13.5\" r=\".75\" />",
};
#[cfg(feature = "ChMenuMeatball")]
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
    data: "<circle cx=\"2.5\" cy=\"8\" r=\".75\" />\n<circle cx=\"8\" cy=\"8\" r=\".75\" />\n<circle cx=\"13.5\" cy=\"8\" r=\".75\" />",
};
#[cfg(feature = "ChMessage")]
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
    data: "<polygon points=\"1.75 14.25,1.75 2.75,14.25 2.75,14.25 11.25,5.75 11.25\" />",
};
#[cfg(feature = "ChMessages")]
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
    data: "<polygon points=\"14.25 14.25,14.25 5.25,4.75 5.25,4.75 11.25,10.75 11.25\" />\n<path d=\"m4.75 7.25-3 3v-8.5h10v3\" />",
};
#[cfg(feature = "ChMicrophone")]
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
    data: "<path d=\"m8 1.75c-2.25 0-2.25 2-2.25 3v1.5c0 1 0 3 2.25 3s2.25-2 2.25-3v-1.5c0-1 0-3-2.25-3z\" />\n<path d=\"m8 13v1.25m-5.25-6.5s0 4.5 5.25 4.50785c5.25.0079 5.25-4.5078 5.25-4.5078\" />",
};
#[cfg(feature = "ChMinus")]
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
    data: "<path d=\"m13.25 7.75h-10.5\" />",
};
#[cfg(feature = "ChMobile")]
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
    data: "<rect height=\"12.5\" width=\"8.5\" y=\"1.75\" x=\"3.75\" />\n<path d=\"m8.25 11.75h-.5\" />",
};
#[cfg(feature = "ChMonitor")]
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
    data: "<rect height=\"9.5\" width=\"12.5\" y=\"1.75\" x=\"1.75\" />\n<path d=\"m4.75 14.25h6.5m-3.25-2.5v2.5\" />",
};
#[cfg(feature = "ChMonitorArrow")]
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
    data: "<path d=\"m14.2 7.75v3.5h-12.5v-9.5h6.5\" />\n<path d=\"m4.75 14.2h6.5m-3.25-2.5v2.5\" />\n<path d=\"m9.75 6.25 4.5-4.5m-3.5-0.5h4v4\" />",
};
#[cfg(feature = "ChMonitorCross")]
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
    data: "<path d=\"m14.2 7.75v3.5h-12.5v-9.5h6.5\" />\n<path d=\"m4.75 14.2h6.5m-3.25-2.5v2.5\" />\n<path d=\"m14.2 1.75-3.5 3.5m0-3.5 3.5 3.5\" />",
};
#[cfg(feature = "ChMoon")]
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
    data: "<path d=\"m1.75 8c0 3.45 2.8 6.25 6.25 6.25 3.41-.0027 6.25-3 6.25-6-1 .5-4 1.5-6-.5s-1-5-.5-6c-3 0-6 2.84-6 6.25z\" />",
};
#[cfg(feature = "ChMove")]
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
    data: "<path d=\"m12.25 10.25 2-2.25-2-2.25m-2-2-2.25-2-2.25 2m-2 2-2 2.25 2 2.25m2 2 2.25 2 2.25-2m-2.25-10.5v12m5.75-5.75h-12\" />",
};
#[cfg(feature = "ChMusic")]
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
    data: "<circle cx=\"4\" cy=\"12\" r=\"2.25\" />\n<circle cx=\"12\" cy=\"11\" r=\"2.25\" />\n<polyline points=\"6.25 12,6.25 2.75,14.25 1.75,14.25 11\" />",
};
#[cfg(feature = "ChNewspaper")]
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
    data: "<path d=\"m11.2 14.2h0.5c1.5 0 2.5-1 2.5-2.5v-6h-3m-9.5-4h9.5v12.5h-7c-1.5 0-2.5-1-2.5-2.5v-9.44z\" />\n<path d=\"m4.75 11.2h3.5\" />\n<rect x=\"4.75\" y=\"4.75\" width=\"3.5\" height=\"3.5\" />",
};
#[cfg(feature = "ChNorthStar")]
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
    data: "<path d=\"m13.75 7.75h-12\" />\n<path d=\"m7.75 1.75v12\" />\n<path d=\"m4.25 11.25 7-7\" />\n<path d=\"m11.25 11.25-7-7\" />",
};
#[cfg(feature = "ChNotes")]
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
    data: "<rect height=\"12.5\" width=\"10.5\" y=\"1.75\" x=\"2.75\" />\n<path d=\"m5.75 7.75h4.5m-4.5 3h2.5m-2.5-6h4.5\" />",
};
#[cfg(feature = "ChNotesCross")]
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
    data: "<polyline points=\"8.25 14.25,2.75 14.25,2.75 1.75,13.25 1.75,13.25 8.25\" />\n<path d=\"m14.25 10.75-3.5 3.5m-5-6.5h4.5m-4.5 3h1.5m-1.5-6h4.5m.5 6 3.5 3.5\" />",
};
#[cfg(feature = "ChNotesTick")]
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
    data: "<polyline points=\"7.25 14.25,2.75 14.25,2.75 1.75,13.25 1.75,13.25 9.25\" />\n<path d=\"m9.75 12.75 1.5 1.5 3-2.5m-8.5-4h4.5m-4.5 3h1.5m-1.5-6h4.5\" />",
};
#[cfg(feature = "ChNut")]
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
    data: "<polygon points=\"8 1.25 14.25 4.75 14.25 11.25 8 14.75 1.75 11.25 1.75 4.75\" />\n<circle cx=\"8\" cy=\"8\" r=\"2.25\" />",
};
#[cfg(feature = "ChOctagon")]
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
    data: "<polygon points=\"5.25 1.75,10.75 1.75,14.25 5.25,14.25 10.75,10.75 14.25,5.25 14.25,1.75 10.75,1.75 5.25\" />",
};
#[cfg(feature = "ChOctagonWarning")]
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
    data: "<polygon points=\"5.25 1.75,10.75 1.75,14.25 5.25,14.25 10.75,10.75 14.25,5.25 14.25,1.75 10.75,1.75 5.25\" />\n<path d=\"m8 11.25v0m0-6.5v3.5\" />",
};
#[cfg(feature = "ChOrganisation")]
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
    data: "<rect x=\"6.75\" y=\"1.75\" width=\"3.5\" height=\"3.5\" />\n<rect x=\"10.75\" y=\"10.75\" width=\"3.5\" height=\"3.5\" />\n<rect x=\"2.75\" y=\"10.75\" width=\"3.5\" height=\"3.5\" />\n<path d=\"m8.5 5.75v2m-3.75 2.5v-2h7.5v2\" />",
};
#[cfg(feature = "ChPackage")]
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
    data: "<polygon points=\"1.75 5.75,1.75 14.25,1.75 14.25,14.25 14.25,14.25 5.75,10.75 1.75,5.25 1.75\" />\n<path d=\"m8 1.75v3.5m-5.75.5h11.5\" />",
};
#[cfg(feature = "ChPadlock")]
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
    data: "<rect height=\"7.5\" width=\"10.5\" y=\"6.75\" x=\"2.75\" />\n<path d=\"m4.75 6.25s-1-4.5 3.25-4.5 3.25 4.5 3.25 4.5\" />",
};
#[cfg(feature = "ChPaperPlane")]
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
    data: "<polygon points=\"1.75 1.75,14.25 7.75,1.75 14.25,3.25 7.75\" />\n<line x1=\"3.75\" y1=\"7.75\" x2=\"7.25\" y2=\"7.75\" />",
};
#[cfg(feature = "ChPaperclip")]
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
    data: "<path d=\"m8.25 10.25v-7s0-1.5-1.75-1.5-1.75 1.5-1.75 1.5v8s0 3 3.25 3 3.25-3 3.25-3v-4.5\" />",
};
#[cfg(feature = "ChPencil")]
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
    data: "<polygon points=\"1.75 11.25,1.75 14.25,4.75 14.25,14.25 4.75,11.25 1.75\" />\n<line x1=\"8.75\" y1=\"4.75\" x2=\"11.25\" y2=\"7.25\" />",
};
#[cfg(feature = "ChPeople")]
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
    data: "<circle cx=\"5\" cy=\"9\" r=\"2.25\" />\n<circle cx=\"11\" cy=\"4\" r=\"2.25\" />\n<path d=\"m7.75 9.25c0-1 .75-3 3.25-3s3.25 2 3.25 3m-12.5 5c0-1 .75-3 3.25-3s3.25 2 3.25 3\" />",
};
#[cfg(feature = "ChPerson")]
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
    data: "<circle cx=\"8\" cy=\"6\" r=\"3.25\" />\n<path d=\"m2.75 14.25c0-2.5 2-5 5.25-5s5.25 2.5 5.25 5\" />",
};
#[cfg(feature = "ChPhone")]
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
    data: "<path d=\"m1.75 1.75c0 8.5 4 12.5 12.5 12.5v-4l-3.5-1-1 1.5c-2 0-4.5-2.5-4.5-4.5l1.5-1-1-3.5z\" />",
};
#[cfg(feature = "ChPhoneCall")]
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
    data: "<path d=\"m1.75 1.75c0 8.5 4 12.5 12.5 12.5v-4l-3.5-1-1 1.5c-2 0-4.5-2.5-4.5-4.5l1.5-1-1-3.5z\" />\n<path d=\"m9.75 1.75c2.5 0 4.5 2 4.5 4.5m-4.5-2c1 0 2 1 2 2\" />",
};
#[cfg(feature = "ChPhoneCross")]
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
    data: "<path d=\"m1.75 1.75c0 8.5 4 12.5 12.5 12.5v-4l-3.5-1-1 1.5c-2 0-4.5-2.5-4.5-4.5l1.5-1-1-3.5z\" />\n<path d=\"m13.25 2.75-3.5 3.5m0-3.5 3.5 3.5\" />",
};
#[cfg(feature = "ChPhoneForward")]
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
    data: "<path d=\"m1.75 1.75c0 8.5 4 12.5 12.5 12.5v-4l-3.5-1-1 1.5c-2 0-4.5-2.5-4.5-4.5l1.5-1-1-3.5z\" />\n<path d=\"m9.75 4.75h4.5m-2 2 2-2-2-2\" />",
};
#[cfg(feature = "ChPhoneIncoming")]
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
    data: "<path d=\"m1.75 1.75c0 8.5 4 12.5 12.5 12.5v-4l-3.5-1-1 1.5c-2 0-4.5-2.5-4.5-4.5l1.5-1-1-3.5z\" />\n<path d=\"m13.25 2.75-3.5 3.5m0-3v3h3\" />",
};
#[cfg(feature = "ChPhoneOutgoing")]
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
    data: "<path d=\"m1.75 1.75c0 8.5 4 12.5 12.5 12.5v-4l-3.5-1-1 1.5c-2 0-4.5-2.5-4.5-4.5l1.5-1-1-3.5z\" />\n<path d=\"m9.75 6.25 3.5-3.5m0 3v-3h-3\" />",
};
#[cfg(feature = "ChPin")]
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
    data: "<path d=\"m10.25 10.25 4 4m-12.5-7.5 5-5s1 2 2 3 4.5 2 4.5 2l-6.5 6.5s-1-3.5-2-4.5-3-2-3-2z\" />",
};
#[cfg(feature = "ChPlantPot")]
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
    data: "<path d=\"m8.75 6.75c0 1.25-.75 3-.75 3m.25-2.5s.75-2-1-3.5-4.5-1-4.5-1 0 2 1.5 3.5 4 1 4 1zm.5-1s-.75-2 1-3.5 4.5-1 4.5-1 0 2-1.5 3.5-4 1-4 1z\" />\n<path d=\"m4.75 9.75h6.5s.5 4.5-3.25 4.5-3.25-4.5-3.25-4.5z\" />",
};
#[cfg(feature = "ChPlus")]
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
    data: "<path d=\"m12.75 7.75h-10m5-5v10\" />",
};
#[cfg(feature = "ChPower")]
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
    data: "<path d=\"m8 1.75v6.5m4.25-5s2 1.29822 2 4.75-2.79822 6.25-6.25 6.25-6.25-2.79822-6.25-6.25 2-4.75 2-4.75\" />",
};
#[cfg(feature = "ChPrinter")]
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
    data: "<rect height=\"4.5\" width=\"6.5\" y=\"9.75\" x=\"4.75\" />\n<path d=\"m4.75 4.25v-2.5h6.5v2.5m-7 8h-2.5v-7.5h12.5v7.5h-2.5\" />",
};
#[cfg(feature = "ChPulse")]
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
    data: "<polyline points=\"1.75 8.25, 4.25 8.25, 6.25 3.75, 9.75 12.25, 11.75 8.25, 14.25 8.25\" />",
};
#[cfg(feature = "ChQuote")]
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
    data: "<path d=\"m6.25 3.75h-4.5v5.5c0 3.5 2.5 4.5 4.5 4-1.5-1.5-1.5-2.5-1.5-4h1.5z\" />\n<path d=\"m13.25 3.75h-4.5v5.5c0 3.5 2.5 4.5 4.5 4-1.5-1.5-1.5-2.5-1.5-4h1.5z\" />",
};
#[cfg(feature = "ChRefresh")]
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
    data: "<path d=\"m4.75 10.75h-3m12.5-2c0 3-2.79822 5.5-6.25 5.5-3.75 0-6.25-3.5-6.25-3.5v3.5m9.5-9h3m-12.5 2c0-3 2.79822-5.5 6.25-5.5 3.75 0 6.25 3.5 6.25 3.5v-3.5\" />",
};
#[cfg(feature = "ChReply")]
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
    data: "<path d=\"m14.25 13.25c-.5-6-5.5-7.5-8-7v-3.5l-4.5 5.25 4.5 5.25v-3.5c2.50001-0.5 6.5 0.5 8 3.5z\" />",
};
#[cfg(feature = "ChRobot")]
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
    data: "<rect height=\"7.5\" width=\"12.5\" y=\"5.75\" x=\"1.75\" />\n<path d=\"m10.75 8.75v1.5m-5.5-1.5v1.5m-.5-7.5 3.25 3 3.25-3\" />",
};
#[cfg(feature = "ChRocket")]
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
    data: "<path d=\"m4.25 9.75-2-.5s0-1.5.5-3 4-1.5 4-1.5m-.50 7l.5 2s1.5 0 3-.5 1.5-4 1.5-4m-7 .5 2 2s5-2 6.5-4.5 1.5-5.5 1.5-5.5-3 0-5.5 1.5-4.5 6.5-4.5 6.5z\" />\n<path d=\"m1.75 14.25 2-1-1-1z\" fill=\"currentColor\" />\n<circle cx=\"10.25\" cy=\"5.75\" r=\".5\" fill=\"currentColor\" />",
};
#[cfg(feature = "ChRotateAntiClockwise")]
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
    data: "<path d=\"m4.75 5.25h-3m0 3.5c0 2.5 2.79822 5.5 6.25 5.5s6.25-2.79822 6.25-6.25-2.79822-6.25-6.25-6.25c-3.75 0-6.25 3.5-6.25 3.5v-3.5\" />",
};
#[cfg(feature = "ChRotateClockwise")]
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
    data: "<path d=\"m11.25 5.25h3m0 3.5c0 2.5-2.79822 5.5-6.25 5.5s-6.25-2.7982-6.25-6.25c0-3.45178 2.79822-6.25 6.25-6.25 3.75 0 6.25 3.5 6.25 3.5v-3.5\" />",
};
#[cfg(feature = "ChScales")]
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
    data: "<path d=\"m1.75 3.75c1 1 2.5 1.5 4 0h4.5c1.5 1.5 3 1 4 0m-6.25-2v12m-3.25.5h6.5\" />\n<path d=\"m12.75 4.75-2 5c.5 1 3.5 1 4 0zm-9.5 0-2 5c.5 1 3.5 1 4 0z\" />",
};
#[cfg(feature = "ChScreenMaximise")]
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
    data: "<path d=\"m5.25 14.25h-3.5v-3.5m12.5 0v3.5h-3.5m0-12.5h3.5v3.5m-12.5 0v-3.5h3.5\" />",
};
#[cfg(feature = "ChScreenMinimise")]
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
    data: "<path d=\"m1.75 10.75h3.5v3.5m5.5 0v-3.5h3.5m0-5.5h-3.5v-3.5m-5.5 0v3.5h-3.5\" />",
};
#[cfg(feature = "ChSearch")]
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
    data: "<path d=\"m11.25 11.25 3 3\" />\n<circle cx=\"7.5\" cy=\"7.5\" r=\"4.75\" />",
};
#[cfg(feature = "ChServer")]
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
    data: "<rect x=\"1.75\" y=\"3.25\" width=\"12.5\" height=\"10\" />\n<line x1=\"2.25\" y1=\"8.25\" x2=\"13.75\" y2=\"8.25\" />\n<path d=\"m4.75 10.75v0m0-5v0m6.5 0h-3m3 5h-3\" />",
};
#[cfg(feature = "ChShare")]
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
    data: "<circle cx=\"4\" cy=\"8\" r=\"2.25\" />\n<circle cx=\"12\" cy=\"12\" r=\"2.25\" />\n<circle cx=\"12\" cy=\"4\" r=\"2.25\" />\n<path d=\"m6 9 4 2m-4-4 4-2\" />",
};
#[cfg(feature = "ChShield")]
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
    data: "<path d=\"m8 1.75 5.25 2v5c0 2.25-2 4.5-5.25 5.5-3.25-1-5.25-3-5.25-5.5v-5z\" />",
};
#[cfg(feature = "ChShieldCross")]
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
    data: "<path d=\"m8 1.75 5.25 2v5c0 2.25-2 4.5-5.25 5.5-3.25-1-5.25-3-5.25-5.5v-5z\" />\n<path d=\"m9.75 5.75-3.5 3.5m0-3.5 3.5 3.5\" />",
};
#[cfg(feature = "ChShieldKeyhole")]
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
    data: "<path d=\"m8 1.75 5.25 2v5c0 2.25-2 4.5-5.25 5.5-3.25-1-5.25-3-5.25-5.5v-5z\" />\n<path d=\"m8 7.25v3\" />\n<circle cx=\"8\" cy=\"6.5\" r=\".75\" fill=\"currentColor\" />",
};
#[cfg(feature = "ChShieldTick")]
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
    data: "<path d=\"m8 1.75 5.25 2v5c0 2.25-2 4.5-5.25 5.5-3.25-1-5.25-3-5.25-5.5v-5z\" />\n<polyline points=\"5.75 7.75,7.25 9.25,10.25 5.75\" />",
};
#[cfg(feature = "ChShieldWarning")]
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
    data: "<path d=\"m8 1.75 5.25 2v5c0 2.25-2 4.5-5.25 5.5-3.25-1-5.25-3-5.25-5.5v-5z\" />\n<path d=\"m8 10.75v0m0-5.5v3\" />",
};
#[cfg(feature = "ChShoppingBag")]
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
    data: "<rect height=\"9.5\" width=\"10.5\" y=\"4.75\" x=\"2.75\" />\n<path d=\"m5.75 7.75c0 1.5 1 2.5 2.25 2.5s2.25-1 2.25-2.5m-7.5-3 1.5-3h7.5l1.5 3\" />",
};
#[cfg(feature = "ChSignIn")]
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
    data: "<path d=\"m5.25 2.25h-3.5v12h3.5m4-9.5-3.5 3.5 3.5 3.5m5-3.5h-8.5\" />",
};
#[cfg(feature = "ChSignOut")]
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
    data: "<path d=\"m5.25 2.25h-3.5v12h3.5m5.5-9.5 3.5 3.5-3.5 3.5m-5-3.5h8.5\" />",
};
#[cfg(feature = "ChSignpost")]
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
    data: "<polygon points=\"1.75 9.25,12.25 9.25,14.25 7.00,12.25 4.75,1.75 4.75\" />\n<path d=\"m7.25 9.75v4.5m0-12.5v2.5\" />",
};
#[cfg(feature = "ChSkull")]
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
    data: "<path d=\"m1.75 11.25h3v3h6.5v-3h3s1-9.5-6.25-9.5-6.25 9.5-6.25 9.5z\" />\n<circle cx=\"5.25\" cy=\"7.75\" r=\".5\" fill=\"currentColor\" />\n<circle cx=\"10.75\" cy=\"7.75\" r=\".5\" fill=\"currentColor\" />",
};
#[cfg(feature = "ChSnowflake")]
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
    data: "<path d=\"m13.75 7.75h-12\" />\n<path d=\"m7.75 1.75v12\" />\n<path d=\"m5.25 12.75 2.5-2.5 2.5 2.5\" />\n<path d=\"m2.75 5.25 2.5 2.5-2.5 2.5\" />\n<path d=\"m10.25 2.75-2.5 2.5-2.5-2.5\" />\n<path d=\"m12.75 10.25-2.5-2.5 2.5-2.5\" />",
};
#[cfg(feature = "ChSoundDown")]
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
    data: "<polygon points=\"1.75 5.75 1.75 10.25 4.25 10.25 8.25 13.25 8.25 2.75 4.25 5.75\" />\n<path d=\"m10.75 6.25s1 .5 1 1.75-1 1.75-1 1.75\" />",
};
#[cfg(feature = "ChSoundMute")]
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
    data: "<polygon points=\"1.75 5.75 1.75 10.25 4.25 10.25 8.25 13.25 8.25 2.75 4.25 5.75\" />\n<path d=\"m14.25 5.75-3.5 4.5m0-4.5 3.5 4.5\" />",
};
#[cfg(feature = "ChSoundUp")]
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
    data: "<polygon points=\"1.75 5.75,1.75 10.25,4.25 10.25,8.25 13.25,8.25 2.75,4.25 5.75\" />\n<path d=\"m10.75 6.25s1 .5 1 1.75-1 1.75-1 1.75m1-6.5c2 1 3 2.5 3 4.75s-1 3.75-3 4.75\" />",
};
#[cfg(feature = "ChSpeaker")]
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
    data: "<rect height=\"12.5\" width=\"9.5\" y=\"1.75\" x=\"3.25\" />\n<path d=\"m8.25 4.25h-.5\" />\n<circle cx=\"8\" cy=\"9.5\" r=\"2.25\" />",
};
#[cfg(feature = "ChSquare")]
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
    data: "<rect height=\"10.5\" width=\"10.5\" y=\"2.75\" x=\"2.75\" />",
};
#[cfg(feature = "ChSquareCross")]
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
    data: "<path d=\"m10.25 5.75-4.5 4.5m0-4.5 4.5 4.5\" />\n<rect height=\"10.5\" width=\"10.5\" y=\"2.75\" x=\"2.75\" />",
};
#[cfg(feature = "ChSquareTick")]
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
    data: "<polyline points=\"10.25 2.75,2.75 2.75,2.75 13.25,13.25 13.25,13.25 9.75\" />\n<polyline points=\"5.75 7.75,8.25 10.25,14.25 3.75\" />",
};
#[cfg(feature = "ChStack")]
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
    data: "<path d=\"m1.75 11 6.25 3.25 6.25-3.25m-12.5-3 6.25 3.25 6.25-3.25m-6.25-6.25-6.25 3.25 6.25 3.25 6.25-3.25z\" />",
};
#[cfg(feature = "ChStackPop")]
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
    data: "<path d=\"m4.25 6.75-2.5 1.25 6.25 3.25 6.25-3.25-2.5-1.25m-10 4.25 6.25 3.25 6.25-3.25\" />\n<path d=\"m8 8.25v-6.5m-2.25 2 2.25-2 2.25 2\" />",
};
#[cfg(feature = "ChStackPush")]
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
    data: "<path d=\"m3.25 7.25-1.5.75 6.25 3.25 6.25-3.25-1.5-.75m-11 3.75 6.25 3.25 6.25-3.25\" />\n<path d=\"m8 8.25v-6.5m-2.25 4.5 2.25 2 2.25-2\" />",
};
#[cfg(feature = "ChStar")]
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
    data: "<polygon points=\"8 1.75,5.75 5.75,1.75 6.25,4.75 9.75,3.75 14.25,8.00 12.25,12.25 14.25,11.25 9.75,14.25 6.25,10.25 5.75\" />",
};
#[cfg(feature = "ChStickyNote")]
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
    data: "<polygon points=\"9.25 13.25,2.75 13.25,2.75 2.75,13.25 2.75,13.25 9.25\" />\n<polyline points=\"8.75 13.25,8.75 8.75,13.25 8.75\" />",
};
#[cfg(feature = "ChSun")]
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
    data: "<circle cy=\"8\" cx=\"8\" r=\"3.25\" />\n<path d=\"m2.75 13.25.5-.5m9.5 0 .5.5m-.5-10 .5-.5m-10 .5-.5-.5m-.50 5.25h-1m13.5 0h-1m-5.75 5.75v1m0-13.5v1\" />",
};
#[cfg(feature = "ChSwapHorizontal")]
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
    data: "<path d=\"m5.75 8.25-3 3 3 3m7.5-3h-10.5m7.5-9.5l3 3-3 3m-7.5-3h10.5\" />",
};
#[cfg(feature = "ChSwapVertical")]
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
    data: "<path d=\"m7.75 5.75-3-3-3 3m3 7.5v-10.5m9.5 7.5-3 3-3-3m3-7.5v10.5\" />",
};
#[cfg(feature = "ChSword")]
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
    data: "<path d=\"m2.75 9.25 1.5 2.5 2 1.5m-4.5 0 1 1m1.5-2.5-1.5 1.5m3-1 8.5-8.5v-2h-2l-8.5 8.5\" />",
};
#[cfg(feature = "ChSwords")]
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
    data: "<path d=\"m2.75 9.25 1.5 2.5 2 1.5m-4.5 0 1 1m1.5-2.5-1.5 1.5m3-1 8.5-8.5v-2h-2l-8.5 8.5\" />\n<path d=\"m10.25 12.25-2.25-2.25m2-2 2.25 2.25m1-1-1.5 2.5-2 1.5m4.5 0-1 1m-1.5-2.5 1.5 1.5m-7.25-5.25-4.25-4.25v-2h2l4.25 4.25\" />",
};
#[cfg(feature = "ChTablet")]
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
    data: "<rect height=\"12.5\" width=\"10.5\" y=\"1.75\" x=\"2.75\" />\n<path d=\"m8.25 11.75h-.5\" />",
};
#[cfg(feature = "ChTag")]
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
    data: "<polygon points=\"7.25 14.25,1.75 8.75,8.75 1.75,14.25 1.75,14.25 7.25\" />\n<circle cx=\"11\" cy=\"5\" r=\".5\" fill=\"currentColor\" />",
};
#[cfg(feature = "ChTelescope")]
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
    data: "<path d=\"m4.75 5.75 1 2.5m3.5-4.5 1.5 3.5m-9 0 1 2.5 11.5-3.5-2-4.5z\" />\n<path d=\"m7.75 11.2v3m-3-0.5 2.25-2.5 1.75-0.5 2.5 3\" />",
};
#[cfg(feature = "ChTent")]
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
    data: "<path d=\"M 5.25,14.25 8,10 l 2.75,4.25\" />\n<path d=\"m9.75 1.75-8 12.5h12.5l-8-12.5\" />",
};
#[cfg(feature = "ChTerminal")]
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
    data: "<rect height=\"10.5\" width=\"12.5\" y=\"2.75\" x=\"1.75\" />\n<path d=\"m8.75 10.25h2.5m-6.5-4.5 2.5 2.25-2.5 2.25\" />",
};
#[cfg(feature = "ChThumbDown")]
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
    data: "<path d=\"m5.25 10.25c1.5 0 3 4 4.5 4v-4h4.5s-.5-7.5-3.5-7.5h-5.5z\" />\n<path d=\"m5.25 10.25h-3.5v-7.5h3.5\" />",
};
#[cfg(feature = "ChThumbUp")]
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
    data: "<path d=\"m5.25 5.75c1.5 0 3-4 4.5-4v4h4.5s-.5 7.5-3.5 7.5h-5.5z\" />\n<path d=\"m5.25 5.75h-3.5v7.5h3.5\" />",
};
#[cfg(feature = "ChTick")]
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
    data: "<polyline points=\"2.75 8.75,6.25 12.25,13.25 4.75\" />",
};
#[cfg(feature = "ChTickDouble")]
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
    data: "<path d=\"m1.75 9.75 2.5 2.5m3.5-4 2.5-2.5m-4.5 4 2.5 2.5 6-6.5\" />",
};
#[cfg(feature = "ChTicket")]
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
    data: "<path d=\"m1.75 3.75h12.5v3s-2 0-2 1.75 2 1.75 2 1.75v3h-12.5v-3s2 0 2-1.75-2-1.75-2-1.75z\" />\n<path d=\"m8.75 11.75v1.5m0-9.5v1.5m0 2.5v1.5\" />",
};
#[cfg(feature = "ChTreeFir")]
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
    data: "<path d=\"m8 1.75-4.25 5.5h2.5l-3.5 4h4v3h2.5v-3h4l-3.5-4h2.5z\" />",
};
#[cfg(feature = "ChTriangle")]
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
    data: "<polygon points=\"8 2.75,1.75 14.25,14.25 14.25\" />",
};
#[cfg(feature = "ChTrophy")]
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
    data: "<rect height=\"3.5\" width=\"6.5\" y=\"10.75\" x=\"4.75\" />\n<path d=\"m8 8.75v2m-3.25-9c-1.5 0-3 .5-3 2.25s1.5 2.25 3 2.25m6.5-4.5c1.5 0 3 .5 3 2.25s-1.5 2.25-3 2.25m-6.5-4.5h6.5v3.5c0 1.5-1 3-3.25 3s-3.25-1.5-3.25-3z\" />",
};
#[cfg(feature = "ChUmbrella")]
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
    data: "<path d=\"m1.75 8.25s.5-6.5 6.25-6.5 6.25 6.5 6.25 6.5z\" />\n<path d=\"m7.75 8.75v4s0 1.5 1.5 1.5 1.5-1.5 1.5-1.5\" />",
};
#[cfg(feature = "ChUpload")]
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
    data: "<path d=\"m3.75 2.75h9m-8.5 6.5 4-3.5 4 3.5m-4 5v-8.5\" />",
};
#[cfg(feature = "ChWifi")]
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
    data: "<path d=\"m1.75 4.75 6.25 8.5 6.25-8.5c-3.25-2.75-9.25-2.75-12.5 0z\" />",
};
#[cfg(feature = "ChWifiFair")]
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
    data: "<path d=\"m1.75 4.75 6.25 8.5 6.25-8.5c-3.25-2.75-9.25-2.75-12.5 0z\" />\n<path d=\"m4.25 8c2-1.75 5.5-1.75 7.5 0\" />",
};
#[cfg(feature = "ChWifiPoor")]
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
    data: "<path d=\"m1.75 4.75 6.25 8.5 6.25-8.5c-3.25-2.75-9.25-2.75-12.5 0z\" />\n<path d=\"m5 9c.75-1.75 5.25-1.75 6 0\" />",
};
#[cfg(feature = "ChWifiSlash")]
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
    data: "<path d=\"m5.25 3.25c-1.5 0-3.5 1.5-3.5 1.5l6.25 8.5 2.25-3m-1.5-7.5s2.97688-.134944 5.5 2l-2 2.5\" />\n<line x1=\"4.25\" y1=\"1.75\" x2=\"12.25\" y2=\"12.25\" />",
};
#[cfg(feature = "ChWifiWarning")]
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
    data: "<path d=\"m14.25 4.75c-3.25-2.75-9.25-2.75-12.5 0l6.25 8.5 1-1.5\" />\n<path d=\"m12.25 13.75v0m0-6v3.5\" />",
};
#[cfg(feature = "ChZoomIn")]
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
    data: "<circle cx=\"7.5\" cy=\"7.5\" r=\"4.75\" />\n<path d=\"m9.25 7.49992h-3.5m1.74992-1.74992v3.5m3.75008 2 3 3\" />",
};
#[cfg(feature = "ChZoomOut")]
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
    data: "<circle cx=\"7.5\" cy=\"7.5\" r=\"4.75\" />\n<path d=\"m9.25 7.49992h-3.5m5.5 3.75008 3 3\" />",
};
impl From<ChIcon> for icondata_core::IconData {
    fn from(icon: ChIcon) -> icondata_core::IconData {
        match icon {
            #[cfg(feature = "ChAnchor")]
            ChIcon::ChAnchor => CH_ANCHOR,
            #[cfg(feature = "ChApps")]
            ChIcon::ChApps => CH_APPS,
            #[cfg(feature = "ChAppsMinus")]
            ChIcon::ChAppsMinus => CH_APPS_MINUS,
            #[cfg(feature = "ChAppsPlus")]
            ChIcon::ChAppsPlus => CH_APPS_PLUS,
            #[cfg(feature = "ChArchive")]
            ChIcon::ChArchive => CH_ARCHIVE,
            #[cfg(feature = "ChArrowDown")]
            ChIcon::ChArrowDown => CH_ARROW_DOWN,
            #[cfg(feature = "ChArrowDownLeft")]
            ChIcon::ChArrowDownLeft => CH_ARROW_DOWN_LEFT,
            #[cfg(feature = "ChArrowDownRight")]
            ChIcon::ChArrowDownRight => CH_ARROW_DOWN_RIGHT,
            #[cfg(feature = "ChArrowLeft")]
            ChIcon::ChArrowLeft => CH_ARROW_LEFT,
            #[cfg(feature = "ChArrowRight")]
            ChIcon::ChArrowRight => CH_ARROW_RIGHT,
            #[cfg(feature = "ChArrowUp")]
            ChIcon::ChArrowUp => CH_ARROW_UP,
            #[cfg(feature = "ChArrowUpLeft")]
            ChIcon::ChArrowUpLeft => CH_ARROW_UP_LEFT,
            #[cfg(feature = "ChArrowUpRight")]
            ChIcon::ChArrowUpRight => CH_ARROW_UP_RIGHT,
            #[cfg(feature = "ChAtSign")]
            ChIcon::ChAtSign => CH_AT_SIGN,
            #[cfg(feature = "ChAtom")]
            ChIcon::ChAtom => CH_ATOM,
            #[cfg(feature = "ChBell")]
            ChIcon::ChBell => CH_BELL,
            #[cfg(feature = "ChBellSlash")]
            ChIcon::ChBellSlash => CH_BELL_SLASH,
            #[cfg(feature = "ChBin")]
            ChIcon::ChBin => CH_BIN,
            #[cfg(feature = "ChBinary")]
            ChIcon::ChBinary => CH_BINARY,
            #[cfg(feature = "ChBlock")]
            ChIcon::ChBlock => CH_BLOCK,
            #[cfg(feature = "ChBluetooth")]
            ChIcon::ChBluetooth => CH_BLUETOOTH,
            #[cfg(feature = "ChBluetoothConnected")]
            ChIcon::ChBluetoothConnected => CH_BLUETOOTH_CONNECTED,
            #[cfg(feature = "ChBluetoothSearching")]
            ChIcon::ChBluetoothSearching => CH_BLUETOOTH_SEARCHING,
            #[cfg(feature = "ChBluetoothSlash")]
            ChIcon::ChBluetoothSlash => CH_BLUETOOTH_SLASH,
            #[cfg(feature = "ChBook")]
            ChIcon::ChBook => CH_BOOK,
            #[cfg(feature = "ChBookOpen")]
            ChIcon::ChBookOpen => CH_BOOK_OPEN,
            #[cfg(feature = "ChBookmark")]
            ChIcon::ChBookmark => CH_BOOKMARK,
            #[cfg(feature = "ChBriefcase")]
            ChIcon::ChBriefcase => CH_BRIEFCASE,
            #[cfg(feature = "ChBug")]
            ChIcon::ChBug => CH_BUG,
            #[cfg(feature = "ChCalendar")]
            ChIcon::ChCalendar => CH_CALENDAR,
            #[cfg(feature = "ChCamera")]
            ChIcon::ChCamera => CH_CAMERA,
            #[cfg(feature = "ChCameraVideo")]
            ChIcon::ChCameraVideo => CH_CAMERA_VIDEO,
            #[cfg(feature = "ChCameraVideoSlash")]
            ChIcon::ChCameraVideoSlash => CH_CAMERA_VIDEO_SLASH,
            #[cfg(feature = "ChCandy")]
            ChIcon::ChCandy => CH_CANDY,
            #[cfg(feature = "ChCards")]
            ChIcon::ChCards => CH_CARDS,
            #[cfg(feature = "ChCast")]
            ChIcon::ChCast => CH_CAST,
            #[cfg(feature = "ChCertificate")]
            ChIcon::ChCertificate => CH_CERTIFICATE,
            #[cfg(feature = "ChChartBar")]
            ChIcon::ChChartBar => CH_CHART_BAR,
            #[cfg(feature = "ChChartLine")]
            ChIcon::ChChartLine => CH_CHART_LINE,
            #[cfg(feature = "ChChevronDown")]
            ChIcon::ChChevronDown => CH_CHEVRON_DOWN,
            #[cfg(feature = "ChChevronLeft")]
            ChIcon::ChChevronLeft => CH_CHEVRON_LEFT,
            #[cfg(feature = "ChChevronRight")]
            ChIcon::ChChevronRight => CH_CHEVRON_RIGHT,
            #[cfg(feature = "ChChevronUp")]
            ChIcon::ChChevronUp => CH_CHEVRON_UP,
            #[cfg(feature = "ChChevronsDown")]
            ChIcon::ChChevronsDown => CH_CHEVRONS_DOWN,
            #[cfg(feature = "ChChevronsLeft")]
            ChIcon::ChChevronsLeft => CH_CHEVRONS_LEFT,
            #[cfg(feature = "ChChevronsRight")]
            ChIcon::ChChevronsRight => CH_CHEVRONS_RIGHT,
            #[cfg(feature = "ChChevronsUp")]
            ChIcon::ChChevronsUp => CH_CHEVRONS_UP,
            #[cfg(feature = "ChChevronsUpDown")]
            ChIcon::ChChevronsUpDown => CH_CHEVRONS_UP_DOWN,
            #[cfg(feature = "ChChip")]
            ChIcon::ChChip => CH_CHIP,
            #[cfg(feature = "ChCircle")]
            ChIcon::ChCircle => CH_CIRCLE,
            #[cfg(feature = "ChCircleCross")]
            ChIcon::ChCircleCross => CH_CIRCLE_CROSS,
            #[cfg(feature = "ChCircleMinus")]
            ChIcon::ChCircleMinus => CH_CIRCLE_MINUS,
            #[cfg(feature = "ChCircleTick")]
            ChIcon::ChCircleTick => CH_CIRCLE_TICK,
            #[cfg(feature = "ChCircleWarning")]
            ChIcon::ChCircleWarning => CH_CIRCLE_WARNING,
            #[cfg(feature = "ChClipboard")]
            ChIcon::ChClipboard => CH_CLIPBOARD,
            #[cfg(feature = "ChClipboardTick")]
            ChIcon::ChClipboardTick => CH_CLIPBOARD_TICK,
            #[cfg(feature = "ChClock")]
            ChIcon::ChClock => CH_CLOCK,
            #[cfg(feature = "ChClockAlarm")]
            ChIcon::ChClockAlarm => CH_CLOCK_ALARM,
            #[cfg(feature = "ChCloud")]
            ChIcon::ChCloud => CH_CLOUD,
            #[cfg(feature = "ChClover")]
            ChIcon::ChClover => CH_CLOVER,
            #[cfg(feature = "ChCode")]
            ChIcon::ChCode => CH_CODE,
            #[cfg(feature = "ChCoffee")]
            ChIcon::ChCoffee => CH_COFFEE,
            #[cfg(feature = "ChCog")]
            ChIcon::ChCog => CH_COG,
            #[cfg(feature = "ChCompass")]
            ChIcon::ChCompass => CH_COMPASS,
            #[cfg(feature = "ChConicalFlask")]
            ChIcon::ChConicalFlask => CH_CONICAL_FLASK,
            #[cfg(feature = "ChContainer")]
            ChIcon::ChContainer => CH_CONTAINER,
            #[cfg(feature = "ChCopy")]
            ChIcon::ChCopy => CH_COPY,
            #[cfg(feature = "ChCopyleft")]
            ChIcon::ChCopyleft => CH_COPYLEFT,
            #[cfg(feature = "ChCopyright")]
            ChIcon::ChCopyright => CH_COPYRIGHT,
            #[cfg(feature = "ChCreditCard")]
            ChIcon::ChCreditCard => CH_CREDIT_CARD,
            #[cfg(feature = "ChCrop")]
            ChIcon::ChCrop => CH_CROP,
            #[cfg(feature = "ChCross")]
            ChIcon::ChCross => CH_CROSS,
            #[cfg(feature = "ChCrosshair")]
            ChIcon::ChCrosshair => CH_CROSSHAIR,
            #[cfg(feature = "ChCube")]
            ChIcon::ChCube => CH_CUBE,
            #[cfg(feature = "ChCursor")]
            ChIcon::ChCursor => CH_CURSOR,
            #[cfg(feature = "ChDatabase")]
            ChIcon::ChDatabase => CH_DATABASE,
            #[cfg(feature = "ChDiamond")]
            ChIcon::ChDiamond => CH_DIAMOND,
            #[cfg(feature = "ChDiff")]
            ChIcon::ChDiff => CH_DIFF,
            #[cfg(feature = "ChDisc")]
            ChIcon::ChDisc => CH_DISC,
            #[cfg(feature = "ChDownload")]
            ChIcon::ChDownload => CH_DOWNLOAD,
            #[cfg(feature = "ChDroplet")]
            ChIcon::ChDroplet => CH_DROPLET,
            #[cfg(feature = "ChEraser")]
            ChIcon::ChEraser => CH_ERASER,
            #[cfg(feature = "ChEye")]
            ChIcon::ChEye => CH_EYE,
            #[cfg(feature = "ChEyeSlash")]
            ChIcon::ChEyeSlash => CH_EYE_SLASH,
            #[cfg(feature = "ChFaceFrown")]
            ChIcon::ChFaceFrown => CH_FACE_FROWN,
            #[cfg(feature = "ChFaceNeutral")]
            ChIcon::ChFaceNeutral => CH_FACE_NEUTRAL,
            #[cfg(feature = "ChFaceSmile")]
            ChIcon::ChFaceSmile => CH_FACE_SMILE,
            #[cfg(feature = "ChFile")]
            ChIcon::ChFile => CH_FILE,
            #[cfg(feature = "ChFileBinary")]
            ChIcon::ChFileBinary => CH_FILE_BINARY,
            #[cfg(feature = "ChFileCode")]
            ChIcon::ChFileCode => CH_FILE_CODE,
            #[cfg(feature = "ChFileSymlink")]
            ChIcon::ChFileSymlink => CH_FILE_SYMLINK,
            #[cfg(feature = "ChFiles")]
            ChIcon::ChFiles => CH_FILES,
            #[cfg(feature = "ChFilter")]
            ChIcon::ChFilter => CH_FILTER,
            #[cfg(feature = "ChFlag")]
            ChIcon::ChFlag => CH_FLAG,
            #[cfg(feature = "ChFlame")]
            ChIcon::ChFlame => CH_FLAME,
            #[cfg(feature = "ChFloppyDisk")]
            ChIcon::ChFloppyDisk => CH_FLOPPY_DISK,
            #[cfg(feature = "ChFolder")]
            ChIcon::ChFolder => CH_FOLDER,
            #[cfg(feature = "ChFolderSymlink")]
            ChIcon::ChFolderSymlink => CH_FOLDER_SYMLINK,
            #[cfg(feature = "ChFolders")]
            ChIcon::ChFolders => CH_FOLDERS,
            #[cfg(feature = "ChForward")]
            ChIcon::ChForward => CH_FORWARD,
            #[cfg(feature = "ChGamepad")]
            ChIcon::ChGamepad => CH_GAMEPAD,
            #[cfg(feature = "ChGem")]
            ChIcon::ChGem => CH_GEM,
            #[cfg(feature = "ChGift")]
            ChIcon::ChGift => CH_GIFT,
            #[cfg(feature = "ChGitBranch")]
            ChIcon::ChGitBranch => CH_GIT_BRANCH,
            #[cfg(feature = "ChGitCherryPick")]
            ChIcon::ChGitCherryPick => CH_GIT_CHERRY_PICK,
            #[cfg(feature = "ChGitCommit")]
            ChIcon::ChGitCommit => CH_GIT_COMMIT,
            #[cfg(feature = "ChGitCompare")]
            ChIcon::ChGitCompare => CH_GIT_COMPARE,
            #[cfg(feature = "ChGitFork")]
            ChIcon::ChGitFork => CH_GIT_FORK,
            #[cfg(feature = "ChGitMerge")]
            ChIcon::ChGitMerge => CH_GIT_MERGE,
            #[cfg(feature = "ChGitRequest")]
            ChIcon::ChGitRequest => CH_GIT_REQUEST,
            #[cfg(feature = "ChGitRequestCross")]
            ChIcon::ChGitRequestCross => CH_GIT_REQUEST_CROSS,
            #[cfg(feature = "ChGitRequestDraft")]
            ChIcon::ChGitRequestDraft => CH_GIT_REQUEST_DRAFT,
            #[cfg(feature = "ChGithub")]
            ChIcon::ChGithub => CH_GITHUB,
            #[cfg(feature = "ChGitlab")]
            ChIcon::ChGitlab => CH_GITLAB,
            #[cfg(feature = "ChGlasses")]
            ChIcon::ChGlasses => CH_GLASSES,
            #[cfg(feature = "ChGlobe")]
            ChIcon::ChGlobe => CH_GLOBE,
            #[cfg(feature = "ChGrabHorizontal")]
            ChIcon::ChGrabHorizontal => CH_GRAB_HORIZONTAL,
            #[cfg(feature = "ChGrabVertical")]
            ChIcon::ChGrabVertical => CH_GRAB_VERTICAL,
            #[cfg(feature = "ChGraduateCap")]
            ChIcon::ChGraduateCap => CH_GRADUATE_CAP,
            #[cfg(feature = "ChHash")]
            ChIcon::ChHash => CH_HASH,
            #[cfg(feature = "ChHeadphones")]
            ChIcon::ChHeadphones => CH_HEADPHONES,
            #[cfg(feature = "ChHeart")]
            ChIcon::ChHeart => CH_HEART,
            #[cfg(feature = "ChHelp")]
            ChIcon::ChHelp => CH_HELP,
            #[cfg(feature = "ChHexagon")]
            ChIcon::ChHexagon => CH_HEXAGON,
            #[cfg(feature = "ChHome")]
            ChIcon::ChHome => CH_HOME,
            #[cfg(feature = "ChHourglass")]
            ChIcon::ChHourglass => CH_HOURGLASS,
            #[cfg(feature = "ChId")]
            ChIcon::ChId => CH_ID,
            #[cfg(feature = "ChImage")]
            ChIcon::ChImage => CH_IMAGE,
            #[cfg(feature = "ChInbox")]
            ChIcon::ChInbox => CH_INBOX,
            #[cfg(feature = "ChInfinity")]
            ChIcon::ChInfinity => CH_INFINITY,
            #[cfg(feature = "ChInfo")]
            ChIcon::ChInfo => CH_INFO,
            #[cfg(feature = "ChKey")]
            ChIcon::ChKey => CH_KEY,
            #[cfg(feature = "ChLaptop")]
            ChIcon::ChLaptop => CH_LAPTOP,
            #[cfg(feature = "ChLayoutColumns")]
            ChIcon::ChLayoutColumns => CH_LAYOUT_COLUMNS,
            #[cfg(feature = "ChLayoutDashboard")]
            ChIcon::ChLayoutDashboard => CH_LAYOUT_DASHBOARD,
            #[cfg(feature = "ChLayoutGrid")]
            ChIcon::ChLayoutGrid => CH_LAYOUT_GRID,
            #[cfg(feature = "ChLayoutList")]
            ChIcon::ChLayoutList => CH_LAYOUT_LIST,
            #[cfg(feature = "ChLayoutRows")]
            ChIcon::ChLayoutRows => CH_LAYOUT_ROWS,
            #[cfg(feature = "ChLayoutSidebar")]
            ChIcon::ChLayoutSidebar => CH_LAYOUT_SIDEBAR,
            #[cfg(feature = "ChLayoutStackH")]
            ChIcon::ChLayoutStackH => CH_LAYOUT_STACK_H,
            #[cfg(feature = "ChLayoutStackV")]
            ChIcon::ChLayoutStackV => CH_LAYOUT_STACK_V,
            #[cfg(feature = "ChLightbulb")]
            ChIcon::ChLightbulb => CH_LIGHTBULB,
            #[cfg(feature = "ChLightningBolt")]
            ChIcon::ChLightningBolt => CH_LIGHTNING_BOLT,
            #[cfg(feature = "ChLink")]
            ChIcon::ChLink => CH_LINK,
            #[cfg(feature = "ChLinkExternal")]
            ChIcon::ChLinkExternal => CH_LINK_EXTERNAL,
            #[cfg(feature = "ChLinkSlash")]
            ChIcon::ChLinkSlash => CH_LINK_SLASH,
            #[cfg(feature = "ChMail")]
            ChIcon::ChMail => CH_MAIL,
            #[cfg(feature = "ChMap")]
            ChIcon::ChMap => CH_MAP,
            #[cfg(feature = "ChMapPin")]
            ChIcon::ChMapPin => CH_MAP_PIN,
            #[cfg(feature = "ChMediaBack")]
            ChIcon::ChMediaBack => CH_MEDIA_BACK,
            #[cfg(feature = "ChMediaEject")]
            ChIcon::ChMediaEject => CH_MEDIA_EJECT,
            #[cfg(feature = "ChMediaFastForward")]
            ChIcon::ChMediaFastForward => CH_MEDIA_FAST_FORWARD,
            #[cfg(feature = "ChMediaPause")]
            ChIcon::ChMediaPause => CH_MEDIA_PAUSE,
            #[cfg(feature = "ChMediaPlay")]
            ChIcon::ChMediaPlay => CH_MEDIA_PLAY,
            #[cfg(feature = "ChMediaRewind")]
            ChIcon::ChMediaRewind => CH_MEDIA_REWIND,
            #[cfg(feature = "ChMediaSkip")]
            ChIcon::ChMediaSkip => CH_MEDIA_SKIP,
            #[cfg(feature = "ChMenuHamburger")]
            ChIcon::ChMenuHamburger => CH_MENU_HAMBURGER,
            #[cfg(feature = "ChMenuKebab")]
            ChIcon::ChMenuKebab => CH_MENU_KEBAB,
            #[cfg(feature = "ChMenuMeatball")]
            ChIcon::ChMenuMeatball => CH_MENU_MEATBALL,
            #[cfg(feature = "ChMessage")]
            ChIcon::ChMessage => CH_MESSAGE,
            #[cfg(feature = "ChMessages")]
            ChIcon::ChMessages => CH_MESSAGES,
            #[cfg(feature = "ChMicrophone")]
            ChIcon::ChMicrophone => CH_MICROPHONE,
            #[cfg(feature = "ChMinus")]
            ChIcon::ChMinus => CH_MINUS,
            #[cfg(feature = "ChMobile")]
            ChIcon::ChMobile => CH_MOBILE,
            #[cfg(feature = "ChMonitor")]
            ChIcon::ChMonitor => CH_MONITOR,
            #[cfg(feature = "ChMonitorArrow")]
            ChIcon::ChMonitorArrow => CH_MONITOR_ARROW,
            #[cfg(feature = "ChMonitorCross")]
            ChIcon::ChMonitorCross => CH_MONITOR_CROSS,
            #[cfg(feature = "ChMoon")]
            ChIcon::ChMoon => CH_MOON,
            #[cfg(feature = "ChMove")]
            ChIcon::ChMove => CH_MOVE,
            #[cfg(feature = "ChMusic")]
            ChIcon::ChMusic => CH_MUSIC,
            #[cfg(feature = "ChNewspaper")]
            ChIcon::ChNewspaper => CH_NEWSPAPER,
            #[cfg(feature = "ChNorthStar")]
            ChIcon::ChNorthStar => CH_NORTH_STAR,
            #[cfg(feature = "ChNotes")]
            ChIcon::ChNotes => CH_NOTES,
            #[cfg(feature = "ChNotesCross")]
            ChIcon::ChNotesCross => CH_NOTES_CROSS,
            #[cfg(feature = "ChNotesTick")]
            ChIcon::ChNotesTick => CH_NOTES_TICK,
            #[cfg(feature = "ChNut")]
            ChIcon::ChNut => CH_NUT,
            #[cfg(feature = "ChOctagon")]
            ChIcon::ChOctagon => CH_OCTAGON,
            #[cfg(feature = "ChOctagonWarning")]
            ChIcon::ChOctagonWarning => CH_OCTAGON_WARNING,
            #[cfg(feature = "ChOrganisation")]
            ChIcon::ChOrganisation => CH_ORGANISATION,
            #[cfg(feature = "ChPackage")]
            ChIcon::ChPackage => CH_PACKAGE,
            #[cfg(feature = "ChPadlock")]
            ChIcon::ChPadlock => CH_PADLOCK,
            #[cfg(feature = "ChPaperPlane")]
            ChIcon::ChPaperPlane => CH_PAPER_PLANE,
            #[cfg(feature = "ChPaperclip")]
            ChIcon::ChPaperclip => CH_PAPERCLIP,
            #[cfg(feature = "ChPencil")]
            ChIcon::ChPencil => CH_PENCIL,
            #[cfg(feature = "ChPeople")]
            ChIcon::ChPeople => CH_PEOPLE,
            #[cfg(feature = "ChPerson")]
            ChIcon::ChPerson => CH_PERSON,
            #[cfg(feature = "ChPhone")]
            ChIcon::ChPhone => CH_PHONE,
            #[cfg(feature = "ChPhoneCall")]
            ChIcon::ChPhoneCall => CH_PHONE_CALL,
            #[cfg(feature = "ChPhoneCross")]
            ChIcon::ChPhoneCross => CH_PHONE_CROSS,
            #[cfg(feature = "ChPhoneForward")]
            ChIcon::ChPhoneForward => CH_PHONE_FORWARD,
            #[cfg(feature = "ChPhoneIncoming")]
            ChIcon::ChPhoneIncoming => CH_PHONE_INCOMING,
            #[cfg(feature = "ChPhoneOutgoing")]
            ChIcon::ChPhoneOutgoing => CH_PHONE_OUTGOING,
            #[cfg(feature = "ChPin")]
            ChIcon::ChPin => CH_PIN,
            #[cfg(feature = "ChPlantPot")]
            ChIcon::ChPlantPot => CH_PLANT_POT,
            #[cfg(feature = "ChPlus")]
            ChIcon::ChPlus => CH_PLUS,
            #[cfg(feature = "ChPower")]
            ChIcon::ChPower => CH_POWER,
            #[cfg(feature = "ChPrinter")]
            ChIcon::ChPrinter => CH_PRINTER,
            #[cfg(feature = "ChPulse")]
            ChIcon::ChPulse => CH_PULSE,
            #[cfg(feature = "ChQuote")]
            ChIcon::ChQuote => CH_QUOTE,
            #[cfg(feature = "ChRefresh")]
            ChIcon::ChRefresh => CH_REFRESH,
            #[cfg(feature = "ChReply")]
            ChIcon::ChReply => CH_REPLY,
            #[cfg(feature = "ChRobot")]
            ChIcon::ChRobot => CH_ROBOT,
            #[cfg(feature = "ChRocket")]
            ChIcon::ChRocket => CH_ROCKET,
            #[cfg(feature = "ChRotateAntiClockwise")]
            ChIcon::ChRotateAntiClockwise => CH_ROTATE_ANTI_CLOCKWISE,
            #[cfg(feature = "ChRotateClockwise")]
            ChIcon::ChRotateClockwise => CH_ROTATE_CLOCKWISE,
            #[cfg(feature = "ChScales")]
            ChIcon::ChScales => CH_SCALES,
            #[cfg(feature = "ChScreenMaximise")]
            ChIcon::ChScreenMaximise => CH_SCREEN_MAXIMISE,
            #[cfg(feature = "ChScreenMinimise")]
            ChIcon::ChScreenMinimise => CH_SCREEN_MINIMISE,
            #[cfg(feature = "ChSearch")]
            ChIcon::ChSearch => CH_SEARCH,
            #[cfg(feature = "ChServer")]
            ChIcon::ChServer => CH_SERVER,
            #[cfg(feature = "ChShare")]
            ChIcon::ChShare => CH_SHARE,
            #[cfg(feature = "ChShield")]
            ChIcon::ChShield => CH_SHIELD,
            #[cfg(feature = "ChShieldCross")]
            ChIcon::ChShieldCross => CH_SHIELD_CROSS,
            #[cfg(feature = "ChShieldKeyhole")]
            ChIcon::ChShieldKeyhole => CH_SHIELD_KEYHOLE,
            #[cfg(feature = "ChShieldTick")]
            ChIcon::ChShieldTick => CH_SHIELD_TICK,
            #[cfg(feature = "ChShieldWarning")]
            ChIcon::ChShieldWarning => CH_SHIELD_WARNING,
            #[cfg(feature = "ChShoppingBag")]
            ChIcon::ChShoppingBag => CH_SHOPPING_BAG,
            #[cfg(feature = "ChSignIn")]
            ChIcon::ChSignIn => CH_SIGN_IN,
            #[cfg(feature = "ChSignOut")]
            ChIcon::ChSignOut => CH_SIGN_OUT,
            #[cfg(feature = "ChSignpost")]
            ChIcon::ChSignpost => CH_SIGNPOST,
            #[cfg(feature = "ChSkull")]
            ChIcon::ChSkull => CH_SKULL,
            #[cfg(feature = "ChSnowflake")]
            ChIcon::ChSnowflake => CH_SNOWFLAKE,
            #[cfg(feature = "ChSoundDown")]
            ChIcon::ChSoundDown => CH_SOUND_DOWN,
            #[cfg(feature = "ChSoundMute")]
            ChIcon::ChSoundMute => CH_SOUND_MUTE,
            #[cfg(feature = "ChSoundUp")]
            ChIcon::ChSoundUp => CH_SOUND_UP,
            #[cfg(feature = "ChSpeaker")]
            ChIcon::ChSpeaker => CH_SPEAKER,
            #[cfg(feature = "ChSquare")]
            ChIcon::ChSquare => CH_SQUARE,
            #[cfg(feature = "ChSquareCross")]
            ChIcon::ChSquareCross => CH_SQUARE_CROSS,
            #[cfg(feature = "ChSquareTick")]
            ChIcon::ChSquareTick => CH_SQUARE_TICK,
            #[cfg(feature = "ChStack")]
            ChIcon::ChStack => CH_STACK,
            #[cfg(feature = "ChStackPop")]
            ChIcon::ChStackPop => CH_STACK_POP,
            #[cfg(feature = "ChStackPush")]
            ChIcon::ChStackPush => CH_STACK_PUSH,
            #[cfg(feature = "ChStar")]
            ChIcon::ChStar => CH_STAR,
            #[cfg(feature = "ChStickyNote")]
            ChIcon::ChStickyNote => CH_STICKY_NOTE,
            #[cfg(feature = "ChSun")]
            ChIcon::ChSun => CH_SUN,
            #[cfg(feature = "ChSwapHorizontal")]
            ChIcon::ChSwapHorizontal => CH_SWAP_HORIZONTAL,
            #[cfg(feature = "ChSwapVertical")]
            ChIcon::ChSwapVertical => CH_SWAP_VERTICAL,
            #[cfg(feature = "ChSword")]
            ChIcon::ChSword => CH_SWORD,
            #[cfg(feature = "ChSwords")]
            ChIcon::ChSwords => CH_SWORDS,
            #[cfg(feature = "ChTablet")]
            ChIcon::ChTablet => CH_TABLET,
            #[cfg(feature = "ChTag")]
            ChIcon::ChTag => CH_TAG,
            #[cfg(feature = "ChTelescope")]
            ChIcon::ChTelescope => CH_TELESCOPE,
            #[cfg(feature = "ChTent")]
            ChIcon::ChTent => CH_TENT,
            #[cfg(feature = "ChTerminal")]
            ChIcon::ChTerminal => CH_TERMINAL,
            #[cfg(feature = "ChThumbDown")]
            ChIcon::ChThumbDown => CH_THUMB_DOWN,
            #[cfg(feature = "ChThumbUp")]
            ChIcon::ChThumbUp => CH_THUMB_UP,
            #[cfg(feature = "ChTick")]
            ChIcon::ChTick => CH_TICK,
            #[cfg(feature = "ChTickDouble")]
            ChIcon::ChTickDouble => CH_TICK_DOUBLE,
            #[cfg(feature = "ChTicket")]
            ChIcon::ChTicket => CH_TICKET,
            #[cfg(feature = "ChTreeFir")]
            ChIcon::ChTreeFir => CH_TREE_FIR,
            #[cfg(feature = "ChTriangle")]
            ChIcon::ChTriangle => CH_TRIANGLE,
            #[cfg(feature = "ChTrophy")]
            ChIcon::ChTrophy => CH_TROPHY,
            #[cfg(feature = "ChUmbrella")]
            ChIcon::ChUmbrella => CH_UMBRELLA,
            #[cfg(feature = "ChUpload")]
            ChIcon::ChUpload => CH_UPLOAD,
            #[cfg(feature = "ChWifi")]
            ChIcon::ChWifi => CH_WIFI,
            #[cfg(feature = "ChWifiFair")]
            ChIcon::ChWifiFair => CH_WIFI_FAIR,
            #[cfg(feature = "ChWifiPoor")]
            ChIcon::ChWifiPoor => CH_WIFI_POOR,
            #[cfg(feature = "ChWifiSlash")]
            ChIcon::ChWifiSlash => CH_WIFI_SLASH,
            #[cfg(feature = "ChWifiWarning")]
            ChIcon::ChWifiWarning => CH_WIFI_WARNING,
            #[cfg(feature = "ChZoomIn")]
            ChIcon::ChZoomIn => CH_ZOOM_IN,
            #[cfg(feature = "ChZoomOut")]
            ChIcon::ChZoomOut => CH_ZOOM_OUT,
        }
    }
}
