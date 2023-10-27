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
    ChAnchor,
    ChApps,
    ChAppsMinus,
    ChAppsPlus,
    ChArchive,
    ChArrowDown,
    ChArrowDownLeft,
    ChArrowDownRight,
    ChArrowLeft,
    ChArrowRight,
    ChArrowUp,
    ChArrowUpLeft,
    ChArrowUpRight,
    ChAtSign,
    ChAtom,
    ChBell,
    ChBellSlash,
    ChBin,
    ChBinary,
    ChBlock,
    ChBluetooth,
    ChBluetoothConnected,
    ChBluetoothSearching,
    ChBluetoothSlash,
    ChBook,
    ChBookOpen,
    ChBookmark,
    ChBriefcase,
    ChBug,
    ChCalendar,
    ChCamera,
    ChCameraVideo,
    ChCameraVideoSlash,
    ChCandy,
    ChCards,
    ChCast,
    ChCertificate,
    ChChartBar,
    ChChartLine,
    ChChevronDown,
    ChChevronLeft,
    ChChevronRight,
    ChChevronUp,
    ChChevronsDown,
    ChChevronsLeft,
    ChChevronsRight,
    ChChevronsUp,
    ChChevronsUpDown,
    ChChip,
    ChCircle,
    ChCircleCross,
    ChCircleMinus,
    ChCircleTick,
    ChCircleWarning,
    ChClipboard,
    ChClipboardTick,
    ChClock,
    ChClockAlarm,
    ChCloud,
    ChClover,
    ChCode,
    ChCoffee,
    ChCog,
    ChCompass,
    ChConicalFlask,
    ChContainer,
    ChCopy,
    ChCopyleft,
    ChCopyright,
    ChCreditCard,
    ChCrop,
    ChCross,
    ChCrosshair,
    ChCube,
    ChCursor,
    ChDatabase,
    ChDiamond,
    ChDiff,
    ChDisc,
    ChDownload,
    ChDroplet,
    ChEraser,
    ChEye,
    ChEyeSlash,
    ChFaceFrown,
    ChFaceNeutral,
    ChFaceSmile,
    ChFile,
    ChFileBinary,
    ChFileCode,
    ChFileSymlink,
    ChFiles,
    ChFilter,
    ChFlag,
    ChFlame,
    ChFloppyDisk,
    ChFolder,
    ChFolderSymlink,
    ChFolders,
    ChForward,
    ChGamepad,
    ChGem,
    ChGift,
    ChGitBranch,
    ChGitCherryPick,
    ChGitCommit,
    ChGitCompare,
    ChGitFork,
    ChGitMerge,
    ChGitRequest,
    ChGitRequestCross,
    ChGitRequestDraft,
    ChGithub,
    ChGitlab,
    ChGlasses,
    ChGlobe,
    ChGrabHorizontal,
    ChGrabVertical,
    ChGraduateCap,
    ChHash,
    ChHeadphones,
    ChHeart,
    ChHelp,
    ChHexagon,
    ChHome,
    ChHourglass,
    ChId,
    ChImage,
    ChInbox,
    ChInfinity,
    ChInfo,
    ChKey,
    ChLaptop,
    ChLayoutColumns,
    ChLayoutDashboard,
    ChLayoutGrid,
    ChLayoutList,
    ChLayoutRows,
    ChLayoutSidebar,
    ChLayoutStackH,
    ChLayoutStackV,
    ChLightbulb,
    ChLightningBolt,
    ChLink,
    ChLinkExternal,
    ChLinkSlash,
    ChMail,
    ChMap,
    ChMapPin,
    ChMediaBack,
    ChMediaEject,
    ChMediaFastForward,
    ChMediaPause,
    ChMediaPlay,
    ChMediaRewind,
    ChMediaSkip,
    ChMenuHamburger,
    ChMenuKebab,
    ChMenuMeatball,
    ChMessage,
    ChMessages,
    ChMicrophone,
    ChMinus,
    ChMobile,
    ChMonitor,
    ChMonitorArrow,
    ChMonitorCross,
    ChMoon,
    ChMove,
    ChMusic,
    ChNewspaper,
    ChNorthStar,
    ChNotes,
    ChNotesCross,
    ChNotesTick,
    ChNut,
    ChOctagon,
    ChOctagonWarning,
    ChOrganisation,
    ChPackage,
    ChPadlock,
    ChPaperPlane,
    ChPaperclip,
    ChPencil,
    ChPeople,
    ChPerson,
    ChPhone,
    ChPhoneCall,
    ChPhoneCross,
    ChPhoneForward,
    ChPhoneIncoming,
    ChPhoneOutgoing,
    ChPin,
    ChPlantPot,
    ChPlus,
    ChPower,
    ChPrinter,
    ChPulse,
    ChQuote,
    ChRefresh,
    ChReply,
    ChRobot,
    ChRocket,
    ChRotateAntiClockwise,
    ChRotateClockwise,
    ChScales,
    ChScreenMaximise,
    ChScreenMinimise,
    ChSearch,
    ChServer,
    ChShare,
    ChShield,
    ChShieldCross,
    ChShieldKeyhole,
    ChShieldTick,
    ChShieldWarning,
    ChShoppingBag,
    ChSignIn,
    ChSignOut,
    ChSignpost,
    ChSkull,
    ChSnowflake,
    ChSoundDown,
    ChSoundMute,
    ChSoundUp,
    ChSpeaker,
    ChSquare,
    ChSquareCross,
    ChSquareTick,
    ChStack,
    ChStackPop,
    ChStackPush,
    ChStar,
    ChStickyNote,
    ChSun,
    ChSwapHorizontal,
    ChSwapVertical,
    ChSword,
    ChSwords,
    ChTablet,
    ChTag,
    ChTelescope,
    ChTent,
    ChTerminal,
    ChThumbDown,
    ChThumbUp,
    ChTick,
    ChTickDouble,
    ChTicket,
    ChTreeFir,
    ChTriangle,
    ChTrophy,
    ChUmbrella,
    ChUpload,
    ChWifi,
    ChWifiFair,
    ChWifiPoor,
    ChWifiSlash,
    ChWifiWarning,
    ChZoomIn,
    ChZoomOut,
}

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
            ChIcon::ChAnchor => CH_ANCHOR,
            ChIcon::ChApps => CH_APPS,
            ChIcon::ChAppsMinus => CH_APPS_MINUS,
            ChIcon::ChAppsPlus => CH_APPS_PLUS,
            ChIcon::ChArchive => CH_ARCHIVE,
            ChIcon::ChArrowDown => CH_ARROW_DOWN,
            ChIcon::ChArrowDownLeft => CH_ARROW_DOWN_LEFT,
            ChIcon::ChArrowDownRight => CH_ARROW_DOWN_RIGHT,
            ChIcon::ChArrowLeft => CH_ARROW_LEFT,
            ChIcon::ChArrowRight => CH_ARROW_RIGHT,
            ChIcon::ChArrowUp => CH_ARROW_UP,
            ChIcon::ChArrowUpLeft => CH_ARROW_UP_LEFT,
            ChIcon::ChArrowUpRight => CH_ARROW_UP_RIGHT,
            ChIcon::ChAtSign => CH_AT_SIGN,
            ChIcon::ChAtom => CH_ATOM,
            ChIcon::ChBell => CH_BELL,
            ChIcon::ChBellSlash => CH_BELL_SLASH,
            ChIcon::ChBin => CH_BIN,
            ChIcon::ChBinary => CH_BINARY,
            ChIcon::ChBlock => CH_BLOCK,
            ChIcon::ChBluetooth => CH_BLUETOOTH,
            ChIcon::ChBluetoothConnected => CH_BLUETOOTH_CONNECTED,
            ChIcon::ChBluetoothSearching => CH_BLUETOOTH_SEARCHING,
            ChIcon::ChBluetoothSlash => CH_BLUETOOTH_SLASH,
            ChIcon::ChBook => CH_BOOK,
            ChIcon::ChBookOpen => CH_BOOK_OPEN,
            ChIcon::ChBookmark => CH_BOOKMARK,
            ChIcon::ChBriefcase => CH_BRIEFCASE,
            ChIcon::ChBug => CH_BUG,
            ChIcon::ChCalendar => CH_CALENDAR,
            ChIcon::ChCamera => CH_CAMERA,
            ChIcon::ChCameraVideo => CH_CAMERA_VIDEO,
            ChIcon::ChCameraVideoSlash => CH_CAMERA_VIDEO_SLASH,
            ChIcon::ChCandy => CH_CANDY,
            ChIcon::ChCards => CH_CARDS,
            ChIcon::ChCast => CH_CAST,
            ChIcon::ChCertificate => CH_CERTIFICATE,
            ChIcon::ChChartBar => CH_CHART_BAR,
            ChIcon::ChChartLine => CH_CHART_LINE,
            ChIcon::ChChevronDown => CH_CHEVRON_DOWN,
            ChIcon::ChChevronLeft => CH_CHEVRON_LEFT,
            ChIcon::ChChevronRight => CH_CHEVRON_RIGHT,
            ChIcon::ChChevronUp => CH_CHEVRON_UP,
            ChIcon::ChChevronsDown => CH_CHEVRONS_DOWN,
            ChIcon::ChChevronsLeft => CH_CHEVRONS_LEFT,
            ChIcon::ChChevronsRight => CH_CHEVRONS_RIGHT,
            ChIcon::ChChevronsUp => CH_CHEVRONS_UP,
            ChIcon::ChChevronsUpDown => CH_CHEVRONS_UP_DOWN,
            ChIcon::ChChip => CH_CHIP,
            ChIcon::ChCircle => CH_CIRCLE,
            ChIcon::ChCircleCross => CH_CIRCLE_CROSS,
            ChIcon::ChCircleMinus => CH_CIRCLE_MINUS,
            ChIcon::ChCircleTick => CH_CIRCLE_TICK,
            ChIcon::ChCircleWarning => CH_CIRCLE_WARNING,
            ChIcon::ChClipboard => CH_CLIPBOARD,
            ChIcon::ChClipboardTick => CH_CLIPBOARD_TICK,
            ChIcon::ChClock => CH_CLOCK,
            ChIcon::ChClockAlarm => CH_CLOCK_ALARM,
            ChIcon::ChCloud => CH_CLOUD,
            ChIcon::ChClover => CH_CLOVER,
            ChIcon::ChCode => CH_CODE,
            ChIcon::ChCoffee => CH_COFFEE,
            ChIcon::ChCog => CH_COG,
            ChIcon::ChCompass => CH_COMPASS,
            ChIcon::ChConicalFlask => CH_CONICAL_FLASK,
            ChIcon::ChContainer => CH_CONTAINER,
            ChIcon::ChCopy => CH_COPY,
            ChIcon::ChCopyleft => CH_COPYLEFT,
            ChIcon::ChCopyright => CH_COPYRIGHT,
            ChIcon::ChCreditCard => CH_CREDIT_CARD,
            ChIcon::ChCrop => CH_CROP,
            ChIcon::ChCross => CH_CROSS,
            ChIcon::ChCrosshair => CH_CROSSHAIR,
            ChIcon::ChCube => CH_CUBE,
            ChIcon::ChCursor => CH_CURSOR,
            ChIcon::ChDatabase => CH_DATABASE,
            ChIcon::ChDiamond => CH_DIAMOND,
            ChIcon::ChDiff => CH_DIFF,
            ChIcon::ChDisc => CH_DISC,
            ChIcon::ChDownload => CH_DOWNLOAD,
            ChIcon::ChDroplet => CH_DROPLET,
            ChIcon::ChEraser => CH_ERASER,
            ChIcon::ChEye => CH_EYE,
            ChIcon::ChEyeSlash => CH_EYE_SLASH,
            ChIcon::ChFaceFrown => CH_FACE_FROWN,
            ChIcon::ChFaceNeutral => CH_FACE_NEUTRAL,
            ChIcon::ChFaceSmile => CH_FACE_SMILE,
            ChIcon::ChFile => CH_FILE,
            ChIcon::ChFileBinary => CH_FILE_BINARY,
            ChIcon::ChFileCode => CH_FILE_CODE,
            ChIcon::ChFileSymlink => CH_FILE_SYMLINK,
            ChIcon::ChFiles => CH_FILES,
            ChIcon::ChFilter => CH_FILTER,
            ChIcon::ChFlag => CH_FLAG,
            ChIcon::ChFlame => CH_FLAME,
            ChIcon::ChFloppyDisk => CH_FLOPPY_DISK,
            ChIcon::ChFolder => CH_FOLDER,
            ChIcon::ChFolderSymlink => CH_FOLDER_SYMLINK,
            ChIcon::ChFolders => CH_FOLDERS,
            ChIcon::ChForward => CH_FORWARD,
            ChIcon::ChGamepad => CH_GAMEPAD,
            ChIcon::ChGem => CH_GEM,
            ChIcon::ChGift => CH_GIFT,
            ChIcon::ChGitBranch => CH_GIT_BRANCH,
            ChIcon::ChGitCherryPick => CH_GIT_CHERRY_PICK,
            ChIcon::ChGitCommit => CH_GIT_COMMIT,
            ChIcon::ChGitCompare => CH_GIT_COMPARE,
            ChIcon::ChGitFork => CH_GIT_FORK,
            ChIcon::ChGitMerge => CH_GIT_MERGE,
            ChIcon::ChGitRequest => CH_GIT_REQUEST,
            ChIcon::ChGitRequestCross => CH_GIT_REQUEST_CROSS,
            ChIcon::ChGitRequestDraft => CH_GIT_REQUEST_DRAFT,
            ChIcon::ChGithub => CH_GITHUB,
            ChIcon::ChGitlab => CH_GITLAB,
            ChIcon::ChGlasses => CH_GLASSES,
            ChIcon::ChGlobe => CH_GLOBE,
            ChIcon::ChGrabHorizontal => CH_GRAB_HORIZONTAL,
            ChIcon::ChGrabVertical => CH_GRAB_VERTICAL,
            ChIcon::ChGraduateCap => CH_GRADUATE_CAP,
            ChIcon::ChHash => CH_HASH,
            ChIcon::ChHeadphones => CH_HEADPHONES,
            ChIcon::ChHeart => CH_HEART,
            ChIcon::ChHelp => CH_HELP,
            ChIcon::ChHexagon => CH_HEXAGON,
            ChIcon::ChHome => CH_HOME,
            ChIcon::ChHourglass => CH_HOURGLASS,
            ChIcon::ChId => CH_ID,
            ChIcon::ChImage => CH_IMAGE,
            ChIcon::ChInbox => CH_INBOX,
            ChIcon::ChInfinity => CH_INFINITY,
            ChIcon::ChInfo => CH_INFO,
            ChIcon::ChKey => CH_KEY,
            ChIcon::ChLaptop => CH_LAPTOP,
            ChIcon::ChLayoutColumns => CH_LAYOUT_COLUMNS,
            ChIcon::ChLayoutDashboard => CH_LAYOUT_DASHBOARD,
            ChIcon::ChLayoutGrid => CH_LAYOUT_GRID,
            ChIcon::ChLayoutList => CH_LAYOUT_LIST,
            ChIcon::ChLayoutRows => CH_LAYOUT_ROWS,
            ChIcon::ChLayoutSidebar => CH_LAYOUT_SIDEBAR,
            ChIcon::ChLayoutStackH => CH_LAYOUT_STACK_H,
            ChIcon::ChLayoutStackV => CH_LAYOUT_STACK_V,
            ChIcon::ChLightbulb => CH_LIGHTBULB,
            ChIcon::ChLightningBolt => CH_LIGHTNING_BOLT,
            ChIcon::ChLink => CH_LINK,
            ChIcon::ChLinkExternal => CH_LINK_EXTERNAL,
            ChIcon::ChLinkSlash => CH_LINK_SLASH,
            ChIcon::ChMail => CH_MAIL,
            ChIcon::ChMap => CH_MAP,
            ChIcon::ChMapPin => CH_MAP_PIN,
            ChIcon::ChMediaBack => CH_MEDIA_BACK,
            ChIcon::ChMediaEject => CH_MEDIA_EJECT,
            ChIcon::ChMediaFastForward => CH_MEDIA_FAST_FORWARD,
            ChIcon::ChMediaPause => CH_MEDIA_PAUSE,
            ChIcon::ChMediaPlay => CH_MEDIA_PLAY,
            ChIcon::ChMediaRewind => CH_MEDIA_REWIND,
            ChIcon::ChMediaSkip => CH_MEDIA_SKIP,
            ChIcon::ChMenuHamburger => CH_MENU_HAMBURGER,
            ChIcon::ChMenuKebab => CH_MENU_KEBAB,
            ChIcon::ChMenuMeatball => CH_MENU_MEATBALL,
            ChIcon::ChMessage => CH_MESSAGE,
            ChIcon::ChMessages => CH_MESSAGES,
            ChIcon::ChMicrophone => CH_MICROPHONE,
            ChIcon::ChMinus => CH_MINUS,
            ChIcon::ChMobile => CH_MOBILE,
            ChIcon::ChMonitor => CH_MONITOR,
            ChIcon::ChMonitorArrow => CH_MONITOR_ARROW,
            ChIcon::ChMonitorCross => CH_MONITOR_CROSS,
            ChIcon::ChMoon => CH_MOON,
            ChIcon::ChMove => CH_MOVE,
            ChIcon::ChMusic => CH_MUSIC,
            ChIcon::ChNewspaper => CH_NEWSPAPER,
            ChIcon::ChNorthStar => CH_NORTH_STAR,
            ChIcon::ChNotes => CH_NOTES,
            ChIcon::ChNotesCross => CH_NOTES_CROSS,
            ChIcon::ChNotesTick => CH_NOTES_TICK,
            ChIcon::ChNut => CH_NUT,
            ChIcon::ChOctagon => CH_OCTAGON,
            ChIcon::ChOctagonWarning => CH_OCTAGON_WARNING,
            ChIcon::ChOrganisation => CH_ORGANISATION,
            ChIcon::ChPackage => CH_PACKAGE,
            ChIcon::ChPadlock => CH_PADLOCK,
            ChIcon::ChPaperPlane => CH_PAPER_PLANE,
            ChIcon::ChPaperclip => CH_PAPERCLIP,
            ChIcon::ChPencil => CH_PENCIL,
            ChIcon::ChPeople => CH_PEOPLE,
            ChIcon::ChPerson => CH_PERSON,
            ChIcon::ChPhone => CH_PHONE,
            ChIcon::ChPhoneCall => CH_PHONE_CALL,
            ChIcon::ChPhoneCross => CH_PHONE_CROSS,
            ChIcon::ChPhoneForward => CH_PHONE_FORWARD,
            ChIcon::ChPhoneIncoming => CH_PHONE_INCOMING,
            ChIcon::ChPhoneOutgoing => CH_PHONE_OUTGOING,
            ChIcon::ChPin => CH_PIN,
            ChIcon::ChPlantPot => CH_PLANT_POT,
            ChIcon::ChPlus => CH_PLUS,
            ChIcon::ChPower => CH_POWER,
            ChIcon::ChPrinter => CH_PRINTER,
            ChIcon::ChPulse => CH_PULSE,
            ChIcon::ChQuote => CH_QUOTE,
            ChIcon::ChRefresh => CH_REFRESH,
            ChIcon::ChReply => CH_REPLY,
            ChIcon::ChRobot => CH_ROBOT,
            ChIcon::ChRocket => CH_ROCKET,
            ChIcon::ChRotateAntiClockwise => CH_ROTATE_ANTI_CLOCKWISE,
            ChIcon::ChRotateClockwise => CH_ROTATE_CLOCKWISE,
            ChIcon::ChScales => CH_SCALES,
            ChIcon::ChScreenMaximise => CH_SCREEN_MAXIMISE,
            ChIcon::ChScreenMinimise => CH_SCREEN_MINIMISE,
            ChIcon::ChSearch => CH_SEARCH,
            ChIcon::ChServer => CH_SERVER,
            ChIcon::ChShare => CH_SHARE,
            ChIcon::ChShield => CH_SHIELD,
            ChIcon::ChShieldCross => CH_SHIELD_CROSS,
            ChIcon::ChShieldKeyhole => CH_SHIELD_KEYHOLE,
            ChIcon::ChShieldTick => CH_SHIELD_TICK,
            ChIcon::ChShieldWarning => CH_SHIELD_WARNING,
            ChIcon::ChShoppingBag => CH_SHOPPING_BAG,
            ChIcon::ChSignIn => CH_SIGN_IN,
            ChIcon::ChSignOut => CH_SIGN_OUT,
            ChIcon::ChSignpost => CH_SIGNPOST,
            ChIcon::ChSkull => CH_SKULL,
            ChIcon::ChSnowflake => CH_SNOWFLAKE,
            ChIcon::ChSoundDown => CH_SOUND_DOWN,
            ChIcon::ChSoundMute => CH_SOUND_MUTE,
            ChIcon::ChSoundUp => CH_SOUND_UP,
            ChIcon::ChSpeaker => CH_SPEAKER,
            ChIcon::ChSquare => CH_SQUARE,
            ChIcon::ChSquareCross => CH_SQUARE_CROSS,
            ChIcon::ChSquareTick => CH_SQUARE_TICK,
            ChIcon::ChStack => CH_STACK,
            ChIcon::ChStackPop => CH_STACK_POP,
            ChIcon::ChStackPush => CH_STACK_PUSH,
            ChIcon::ChStar => CH_STAR,
            ChIcon::ChStickyNote => CH_STICKY_NOTE,
            ChIcon::ChSun => CH_SUN,
            ChIcon::ChSwapHorizontal => CH_SWAP_HORIZONTAL,
            ChIcon::ChSwapVertical => CH_SWAP_VERTICAL,
            ChIcon::ChSword => CH_SWORD,
            ChIcon::ChSwords => CH_SWORDS,
            ChIcon::ChTablet => CH_TABLET,
            ChIcon::ChTag => CH_TAG,
            ChIcon::ChTelescope => CH_TELESCOPE,
            ChIcon::ChTent => CH_TENT,
            ChIcon::ChTerminal => CH_TERMINAL,
            ChIcon::ChThumbDown => CH_THUMB_DOWN,
            ChIcon::ChThumbUp => CH_THUMB_UP,
            ChIcon::ChTick => CH_TICK,
            ChIcon::ChTickDouble => CH_TICK_DOUBLE,
            ChIcon::ChTicket => CH_TICKET,
            ChIcon::ChTreeFir => CH_TREE_FIR,
            ChIcon::ChTriangle => CH_TRIANGLE,
            ChIcon::ChTrophy => CH_TROPHY,
            ChIcon::ChUmbrella => CH_UMBRELLA,
            ChIcon::ChUpload => CH_UPLOAD,
            ChIcon::ChWifi => CH_WIFI,
            ChIcon::ChWifiFair => CH_WIFI_FAIR,
            ChIcon::ChWifiPoor => CH_WIFI_POOR,
            ChIcon::ChWifiSlash => CH_WIFI_SLASH,
            ChIcon::ChWifiWarning => CH_WIFI_WARNING,
            ChIcon::ChZoomIn => CH_ZOOM_IN,
            ChIcon::ChZoomOut => CH_ZOOM_OUT,
        }
    }
}