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
    #[cfg(any(ChAnchor, icondata_include_all))]
    ChAnchor,
    #[cfg(any(ChApps, icondata_include_all))]
    ChApps,
    #[cfg(any(ChAppsMinus, icondata_include_all))]
    ChAppsMinus,
    #[cfg(any(ChAppsPlus, icondata_include_all))]
    ChAppsPlus,
    #[cfg(any(ChArchive, icondata_include_all))]
    ChArchive,
    #[cfg(any(ChArrowDown, icondata_include_all))]
    ChArrowDown,
    #[cfg(any(ChArrowDownLeft, icondata_include_all))]
    ChArrowDownLeft,
    #[cfg(any(ChArrowDownRight, icondata_include_all))]
    ChArrowDownRight,
    #[cfg(any(ChArrowLeft, icondata_include_all))]
    ChArrowLeft,
    #[cfg(any(ChArrowRight, icondata_include_all))]
    ChArrowRight,
    #[cfg(any(ChArrowUp, icondata_include_all))]
    ChArrowUp,
    #[cfg(any(ChArrowUpLeft, icondata_include_all))]
    ChArrowUpLeft,
    #[cfg(any(ChArrowUpRight, icondata_include_all))]
    ChArrowUpRight,
    #[cfg(any(ChAtSign, icondata_include_all))]
    ChAtSign,
    #[cfg(any(ChAtom, icondata_include_all))]
    ChAtom,
    #[cfg(any(ChBell, icondata_include_all))]
    ChBell,
    #[cfg(any(ChBellSlash, icondata_include_all))]
    ChBellSlash,
    #[cfg(any(ChBin, icondata_include_all))]
    ChBin,
    #[cfg(any(ChBinary, icondata_include_all))]
    ChBinary,
    #[cfg(any(ChBlock, icondata_include_all))]
    ChBlock,
    #[cfg(any(ChBluetooth, icondata_include_all))]
    ChBluetooth,
    #[cfg(any(ChBluetoothConnected, icondata_include_all))]
    ChBluetoothConnected,
    #[cfg(any(ChBluetoothSearching, icondata_include_all))]
    ChBluetoothSearching,
    #[cfg(any(ChBluetoothSlash, icondata_include_all))]
    ChBluetoothSlash,
    #[cfg(any(ChBook, icondata_include_all))]
    ChBook,
    #[cfg(any(ChBookOpen, icondata_include_all))]
    ChBookOpen,
    #[cfg(any(ChBookmark, icondata_include_all))]
    ChBookmark,
    #[cfg(any(ChBriefcase, icondata_include_all))]
    ChBriefcase,
    #[cfg(any(ChBug, icondata_include_all))]
    ChBug,
    #[cfg(any(ChCalendar, icondata_include_all))]
    ChCalendar,
    #[cfg(any(ChCamera, icondata_include_all))]
    ChCamera,
    #[cfg(any(ChCameraVideo, icondata_include_all))]
    ChCameraVideo,
    #[cfg(any(ChCameraVideoSlash, icondata_include_all))]
    ChCameraVideoSlash,
    #[cfg(any(ChCandy, icondata_include_all))]
    ChCandy,
    #[cfg(any(ChCards, icondata_include_all))]
    ChCards,
    #[cfg(any(ChCast, icondata_include_all))]
    ChCast,
    #[cfg(any(ChCertificate, icondata_include_all))]
    ChCertificate,
    #[cfg(any(ChChartBar, icondata_include_all))]
    ChChartBar,
    #[cfg(any(ChChartLine, icondata_include_all))]
    ChChartLine,
    #[cfg(any(ChChevronDown, icondata_include_all))]
    ChChevronDown,
    #[cfg(any(ChChevronLeft, icondata_include_all))]
    ChChevronLeft,
    #[cfg(any(ChChevronRight, icondata_include_all))]
    ChChevronRight,
    #[cfg(any(ChChevronUp, icondata_include_all))]
    ChChevronUp,
    #[cfg(any(ChChevronsDown, icondata_include_all))]
    ChChevronsDown,
    #[cfg(any(ChChevronsLeft, icondata_include_all))]
    ChChevronsLeft,
    #[cfg(any(ChChevronsRight, icondata_include_all))]
    ChChevronsRight,
    #[cfg(any(ChChevronsUp, icondata_include_all))]
    ChChevronsUp,
    #[cfg(any(ChChevronsUpDown, icondata_include_all))]
    ChChevronsUpDown,
    #[cfg(any(ChChip, icondata_include_all))]
    ChChip,
    #[cfg(any(ChCircle, icondata_include_all))]
    ChCircle,
    #[cfg(any(ChCircleCross, icondata_include_all))]
    ChCircleCross,
    #[cfg(any(ChCircleMinus, icondata_include_all))]
    ChCircleMinus,
    #[cfg(any(ChCircleTick, icondata_include_all))]
    ChCircleTick,
    #[cfg(any(ChCircleWarning, icondata_include_all))]
    ChCircleWarning,
    #[cfg(any(ChClipboard, icondata_include_all))]
    ChClipboard,
    #[cfg(any(ChClipboardTick, icondata_include_all))]
    ChClipboardTick,
    #[cfg(any(ChClock, icondata_include_all))]
    ChClock,
    #[cfg(any(ChClockAlarm, icondata_include_all))]
    ChClockAlarm,
    #[cfg(any(ChCloud, icondata_include_all))]
    ChCloud,
    #[cfg(any(ChClover, icondata_include_all))]
    ChClover,
    #[cfg(any(ChCode, icondata_include_all))]
    ChCode,
    #[cfg(any(ChCoffee, icondata_include_all))]
    ChCoffee,
    #[cfg(any(ChCog, icondata_include_all))]
    ChCog,
    #[cfg(any(ChCompass, icondata_include_all))]
    ChCompass,
    #[cfg(any(ChConicalFlask, icondata_include_all))]
    ChConicalFlask,
    #[cfg(any(ChContainer, icondata_include_all))]
    ChContainer,
    #[cfg(any(ChCopy, icondata_include_all))]
    ChCopy,
    #[cfg(any(ChCopyleft, icondata_include_all))]
    ChCopyleft,
    #[cfg(any(ChCopyright, icondata_include_all))]
    ChCopyright,
    #[cfg(any(ChCreditCard, icondata_include_all))]
    ChCreditCard,
    #[cfg(any(ChCrop, icondata_include_all))]
    ChCrop,
    #[cfg(any(ChCross, icondata_include_all))]
    ChCross,
    #[cfg(any(ChCrosshair, icondata_include_all))]
    ChCrosshair,
    #[cfg(any(ChCube, icondata_include_all))]
    ChCube,
    #[cfg(any(ChCursor, icondata_include_all))]
    ChCursor,
    #[cfg(any(ChDatabase, icondata_include_all))]
    ChDatabase,
    #[cfg(any(ChDiamond, icondata_include_all))]
    ChDiamond,
    #[cfg(any(ChDiff, icondata_include_all))]
    ChDiff,
    #[cfg(any(ChDisc, icondata_include_all))]
    ChDisc,
    #[cfg(any(ChDownload, icondata_include_all))]
    ChDownload,
    #[cfg(any(ChDroplet, icondata_include_all))]
    ChDroplet,
    #[cfg(any(ChEraser, icondata_include_all))]
    ChEraser,
    #[cfg(any(ChEye, icondata_include_all))]
    ChEye,
    #[cfg(any(ChEyeSlash, icondata_include_all))]
    ChEyeSlash,
    #[cfg(any(ChFaceFrown, icondata_include_all))]
    ChFaceFrown,
    #[cfg(any(ChFaceNeutral, icondata_include_all))]
    ChFaceNeutral,
    #[cfg(any(ChFaceSmile, icondata_include_all))]
    ChFaceSmile,
    #[cfg(any(ChFile, icondata_include_all))]
    ChFile,
    #[cfg(any(ChFileBinary, icondata_include_all))]
    ChFileBinary,
    #[cfg(any(ChFileCode, icondata_include_all))]
    ChFileCode,
    #[cfg(any(ChFileSymlink, icondata_include_all))]
    ChFileSymlink,
    #[cfg(any(ChFiles, icondata_include_all))]
    ChFiles,
    #[cfg(any(ChFilter, icondata_include_all))]
    ChFilter,
    #[cfg(any(ChFlag, icondata_include_all))]
    ChFlag,
    #[cfg(any(ChFlame, icondata_include_all))]
    ChFlame,
    #[cfg(any(ChFloppyDisk, icondata_include_all))]
    ChFloppyDisk,
    #[cfg(any(ChFolder, icondata_include_all))]
    ChFolder,
    #[cfg(any(ChFolderSymlink, icondata_include_all))]
    ChFolderSymlink,
    #[cfg(any(ChFolders, icondata_include_all))]
    ChFolders,
    #[cfg(any(ChForward, icondata_include_all))]
    ChForward,
    #[cfg(any(ChGamepad, icondata_include_all))]
    ChGamepad,
    #[cfg(any(ChGem, icondata_include_all))]
    ChGem,
    #[cfg(any(ChGift, icondata_include_all))]
    ChGift,
    #[cfg(any(ChGitBranch, icondata_include_all))]
    ChGitBranch,
    #[cfg(any(ChGitCherryPick, icondata_include_all))]
    ChGitCherryPick,
    #[cfg(any(ChGitCommit, icondata_include_all))]
    ChGitCommit,
    #[cfg(any(ChGitCompare, icondata_include_all))]
    ChGitCompare,
    #[cfg(any(ChGitFork, icondata_include_all))]
    ChGitFork,
    #[cfg(any(ChGitMerge, icondata_include_all))]
    ChGitMerge,
    #[cfg(any(ChGitRequest, icondata_include_all))]
    ChGitRequest,
    #[cfg(any(ChGitRequestCross, icondata_include_all))]
    ChGitRequestCross,
    #[cfg(any(ChGitRequestDraft, icondata_include_all))]
    ChGitRequestDraft,
    #[cfg(any(ChGithub, icondata_include_all))]
    ChGithub,
    #[cfg(any(ChGitlab, icondata_include_all))]
    ChGitlab,
    #[cfg(any(ChGlasses, icondata_include_all))]
    ChGlasses,
    #[cfg(any(ChGlobe, icondata_include_all))]
    ChGlobe,
    #[cfg(any(ChGrabHorizontal, icondata_include_all))]
    ChGrabHorizontal,
    #[cfg(any(ChGrabVertical, icondata_include_all))]
    ChGrabVertical,
    #[cfg(any(ChGraduateCap, icondata_include_all))]
    ChGraduateCap,
    #[cfg(any(ChHash, icondata_include_all))]
    ChHash,
    #[cfg(any(ChHeadphones, icondata_include_all))]
    ChHeadphones,
    #[cfg(any(ChHeart, icondata_include_all))]
    ChHeart,
    #[cfg(any(ChHelp, icondata_include_all))]
    ChHelp,
    #[cfg(any(ChHexagon, icondata_include_all))]
    ChHexagon,
    #[cfg(any(ChHome, icondata_include_all))]
    ChHome,
    #[cfg(any(ChHourglass, icondata_include_all))]
    ChHourglass,
    #[cfg(any(ChId, icondata_include_all))]
    ChId,
    #[cfg(any(ChImage, icondata_include_all))]
    ChImage,
    #[cfg(any(ChInbox, icondata_include_all))]
    ChInbox,
    #[cfg(any(ChInfinity, icondata_include_all))]
    ChInfinity,
    #[cfg(any(ChInfo, icondata_include_all))]
    ChInfo,
    #[cfg(any(ChKey, icondata_include_all))]
    ChKey,
    #[cfg(any(ChLaptop, icondata_include_all))]
    ChLaptop,
    #[cfg(any(ChLayoutColumns, icondata_include_all))]
    ChLayoutColumns,
    #[cfg(any(ChLayoutDashboard, icondata_include_all))]
    ChLayoutDashboard,
    #[cfg(any(ChLayoutGrid, icondata_include_all))]
    ChLayoutGrid,
    #[cfg(any(ChLayoutList, icondata_include_all))]
    ChLayoutList,
    #[cfg(any(ChLayoutRows, icondata_include_all))]
    ChLayoutRows,
    #[cfg(any(ChLayoutSidebar, icondata_include_all))]
    ChLayoutSidebar,
    #[cfg(any(ChLayoutStackH, icondata_include_all))]
    ChLayoutStackH,
    #[cfg(any(ChLayoutStackV, icondata_include_all))]
    ChLayoutStackV,
    #[cfg(any(ChLightbulb, icondata_include_all))]
    ChLightbulb,
    #[cfg(any(ChLightningBolt, icondata_include_all))]
    ChLightningBolt,
    #[cfg(any(ChLink, icondata_include_all))]
    ChLink,
    #[cfg(any(ChLinkExternal, icondata_include_all))]
    ChLinkExternal,
    #[cfg(any(ChLinkSlash, icondata_include_all))]
    ChLinkSlash,
    #[cfg(any(ChMail, icondata_include_all))]
    ChMail,
    #[cfg(any(ChMap, icondata_include_all))]
    ChMap,
    #[cfg(any(ChMapPin, icondata_include_all))]
    ChMapPin,
    #[cfg(any(ChMediaBack, icondata_include_all))]
    ChMediaBack,
    #[cfg(any(ChMediaEject, icondata_include_all))]
    ChMediaEject,
    #[cfg(any(ChMediaFastForward, icondata_include_all))]
    ChMediaFastForward,
    #[cfg(any(ChMediaPause, icondata_include_all))]
    ChMediaPause,
    #[cfg(any(ChMediaPlay, icondata_include_all))]
    ChMediaPlay,
    #[cfg(any(ChMediaRewind, icondata_include_all))]
    ChMediaRewind,
    #[cfg(any(ChMediaSkip, icondata_include_all))]
    ChMediaSkip,
    #[cfg(any(ChMenuHamburger, icondata_include_all))]
    ChMenuHamburger,
    #[cfg(any(ChMenuKebab, icondata_include_all))]
    ChMenuKebab,
    #[cfg(any(ChMenuMeatball, icondata_include_all))]
    ChMenuMeatball,
    #[cfg(any(ChMessage, icondata_include_all))]
    ChMessage,
    #[cfg(any(ChMessages, icondata_include_all))]
    ChMessages,
    #[cfg(any(ChMicrophone, icondata_include_all))]
    ChMicrophone,
    #[cfg(any(ChMinus, icondata_include_all))]
    ChMinus,
    #[cfg(any(ChMobile, icondata_include_all))]
    ChMobile,
    #[cfg(any(ChMonitor, icondata_include_all))]
    ChMonitor,
    #[cfg(any(ChMonitorArrow, icondata_include_all))]
    ChMonitorArrow,
    #[cfg(any(ChMonitorCross, icondata_include_all))]
    ChMonitorCross,
    #[cfg(any(ChMoon, icondata_include_all))]
    ChMoon,
    #[cfg(any(ChMove, icondata_include_all))]
    ChMove,
    #[cfg(any(ChMusic, icondata_include_all))]
    ChMusic,
    #[cfg(any(ChNewspaper, icondata_include_all))]
    ChNewspaper,
    #[cfg(any(ChNorthStar, icondata_include_all))]
    ChNorthStar,
    #[cfg(any(ChNotes, icondata_include_all))]
    ChNotes,
    #[cfg(any(ChNotesCross, icondata_include_all))]
    ChNotesCross,
    #[cfg(any(ChNotesTick, icondata_include_all))]
    ChNotesTick,
    #[cfg(any(ChNut, icondata_include_all))]
    ChNut,
    #[cfg(any(ChOctagon, icondata_include_all))]
    ChOctagon,
    #[cfg(any(ChOctagonWarning, icondata_include_all))]
    ChOctagonWarning,
    #[cfg(any(ChOrganisation, icondata_include_all))]
    ChOrganisation,
    #[cfg(any(ChPackage, icondata_include_all))]
    ChPackage,
    #[cfg(any(ChPadlock, icondata_include_all))]
    ChPadlock,
    #[cfg(any(ChPaperPlane, icondata_include_all))]
    ChPaperPlane,
    #[cfg(any(ChPaperclip, icondata_include_all))]
    ChPaperclip,
    #[cfg(any(ChPencil, icondata_include_all))]
    ChPencil,
    #[cfg(any(ChPeople, icondata_include_all))]
    ChPeople,
    #[cfg(any(ChPerson, icondata_include_all))]
    ChPerson,
    #[cfg(any(ChPhone, icondata_include_all))]
    ChPhone,
    #[cfg(any(ChPhoneCall, icondata_include_all))]
    ChPhoneCall,
    #[cfg(any(ChPhoneCross, icondata_include_all))]
    ChPhoneCross,
    #[cfg(any(ChPhoneForward, icondata_include_all))]
    ChPhoneForward,
    #[cfg(any(ChPhoneIncoming, icondata_include_all))]
    ChPhoneIncoming,
    #[cfg(any(ChPhoneOutgoing, icondata_include_all))]
    ChPhoneOutgoing,
    #[cfg(any(ChPin, icondata_include_all))]
    ChPin,
    #[cfg(any(ChPlantPot, icondata_include_all))]
    ChPlantPot,
    #[cfg(any(ChPlus, icondata_include_all))]
    ChPlus,
    #[cfg(any(ChPower, icondata_include_all))]
    ChPower,
    #[cfg(any(ChPrinter, icondata_include_all))]
    ChPrinter,
    #[cfg(any(ChPulse, icondata_include_all))]
    ChPulse,
    #[cfg(any(ChQuote, icondata_include_all))]
    ChQuote,
    #[cfg(any(ChRefresh, icondata_include_all))]
    ChRefresh,
    #[cfg(any(ChReply, icondata_include_all))]
    ChReply,
    #[cfg(any(ChRobot, icondata_include_all))]
    ChRobot,
    #[cfg(any(ChRocket, icondata_include_all))]
    ChRocket,
    #[cfg(any(ChRotateAntiClockwise, icondata_include_all))]
    ChRotateAntiClockwise,
    #[cfg(any(ChRotateClockwise, icondata_include_all))]
    ChRotateClockwise,
    #[cfg(any(ChScales, icondata_include_all))]
    ChScales,
    #[cfg(any(ChScreenMaximise, icondata_include_all))]
    ChScreenMaximise,
    #[cfg(any(ChScreenMinimise, icondata_include_all))]
    ChScreenMinimise,
    #[cfg(any(ChSearch, icondata_include_all))]
    ChSearch,
    #[cfg(any(ChServer, icondata_include_all))]
    ChServer,
    #[cfg(any(ChShare, icondata_include_all))]
    ChShare,
    #[cfg(any(ChShield, icondata_include_all))]
    ChShield,
    #[cfg(any(ChShieldCross, icondata_include_all))]
    ChShieldCross,
    #[cfg(any(ChShieldKeyhole, icondata_include_all))]
    ChShieldKeyhole,
    #[cfg(any(ChShieldTick, icondata_include_all))]
    ChShieldTick,
    #[cfg(any(ChShieldWarning, icondata_include_all))]
    ChShieldWarning,
    #[cfg(any(ChShoppingBag, icondata_include_all))]
    ChShoppingBag,
    #[cfg(any(ChSignIn, icondata_include_all))]
    ChSignIn,
    #[cfg(any(ChSignOut, icondata_include_all))]
    ChSignOut,
    #[cfg(any(ChSignpost, icondata_include_all))]
    ChSignpost,
    #[cfg(any(ChSkull, icondata_include_all))]
    ChSkull,
    #[cfg(any(ChSnowflake, icondata_include_all))]
    ChSnowflake,
    #[cfg(any(ChSoundDown, icondata_include_all))]
    ChSoundDown,
    #[cfg(any(ChSoundMute, icondata_include_all))]
    ChSoundMute,
    #[cfg(any(ChSoundUp, icondata_include_all))]
    ChSoundUp,
    #[cfg(any(ChSpeaker, icondata_include_all))]
    ChSpeaker,
    #[cfg(any(ChSquare, icondata_include_all))]
    ChSquare,
    #[cfg(any(ChSquareCross, icondata_include_all))]
    ChSquareCross,
    #[cfg(any(ChSquareTick, icondata_include_all))]
    ChSquareTick,
    #[cfg(any(ChStack, icondata_include_all))]
    ChStack,
    #[cfg(any(ChStackPop, icondata_include_all))]
    ChStackPop,
    #[cfg(any(ChStackPush, icondata_include_all))]
    ChStackPush,
    #[cfg(any(ChStar, icondata_include_all))]
    ChStar,
    #[cfg(any(ChStickyNote, icondata_include_all))]
    ChStickyNote,
    #[cfg(any(ChSun, icondata_include_all))]
    ChSun,
    #[cfg(any(ChSwapHorizontal, icondata_include_all))]
    ChSwapHorizontal,
    #[cfg(any(ChSwapVertical, icondata_include_all))]
    ChSwapVertical,
    #[cfg(any(ChSword, icondata_include_all))]
    ChSword,
    #[cfg(any(ChSwords, icondata_include_all))]
    ChSwords,
    #[cfg(any(ChTablet, icondata_include_all))]
    ChTablet,
    #[cfg(any(ChTag, icondata_include_all))]
    ChTag,
    #[cfg(any(ChTelescope, icondata_include_all))]
    ChTelescope,
    #[cfg(any(ChTent, icondata_include_all))]
    ChTent,
    #[cfg(any(ChTerminal, icondata_include_all))]
    ChTerminal,
    #[cfg(any(ChThumbDown, icondata_include_all))]
    ChThumbDown,
    #[cfg(any(ChThumbUp, icondata_include_all))]
    ChThumbUp,
    #[cfg(any(ChTick, icondata_include_all))]
    ChTick,
    #[cfg(any(ChTickDouble, icondata_include_all))]
    ChTickDouble,
    #[cfg(any(ChTicket, icondata_include_all))]
    ChTicket,
    #[cfg(any(ChTreeFir, icondata_include_all))]
    ChTreeFir,
    #[cfg(any(ChTriangle, icondata_include_all))]
    ChTriangle,
    #[cfg(any(ChTrophy, icondata_include_all))]
    ChTrophy,
    #[cfg(any(ChUmbrella, icondata_include_all))]
    ChUmbrella,
    #[cfg(any(ChUpload, icondata_include_all))]
    ChUpload,
    #[cfg(any(ChWifi, icondata_include_all))]
    ChWifi,
    #[cfg(any(ChWifiFair, icondata_include_all))]
    ChWifiFair,
    #[cfg(any(ChWifiPoor, icondata_include_all))]
    ChWifiPoor,
    #[cfg(any(ChWifiSlash, icondata_include_all))]
    ChWifiSlash,
    #[cfg(any(ChWifiWarning, icondata_include_all))]
    ChWifiWarning,
    #[cfg(any(ChZoomIn, icondata_include_all))]
    ChZoomIn,
    #[cfg(any(ChZoomOut, icondata_include_all))]
    ChZoomOut,
}

#[cfg(any(ChAnchor, icondata_include_all))]
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
#[cfg(any(ChApps, icondata_include_all))]
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
#[cfg(any(ChAppsMinus, icondata_include_all))]
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
#[cfg(any(ChAppsPlus, icondata_include_all))]
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
#[cfg(any(ChArchive, icondata_include_all))]
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
#[cfg(any(ChArrowDown, icondata_include_all))]
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
#[cfg(any(ChArrowDownLeft, icondata_include_all))]
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
#[cfg(any(ChArrowDownRight, icondata_include_all))]
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
#[cfg(any(ChArrowLeft, icondata_include_all))]
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
#[cfg(any(ChArrowRight, icondata_include_all))]
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
#[cfg(any(ChArrowUp, icondata_include_all))]
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
#[cfg(any(ChArrowUpLeft, icondata_include_all))]
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
#[cfg(any(ChArrowUpRight, icondata_include_all))]
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
#[cfg(any(ChAtSign, icondata_include_all))]
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
#[cfg(any(ChAtom, icondata_include_all))]
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
#[cfg(any(ChBell, icondata_include_all))]
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
#[cfg(any(ChBellSlash, icondata_include_all))]
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
#[cfg(any(ChBin, icondata_include_all))]
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
#[cfg(any(ChBinary, icondata_include_all))]
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
#[cfg(any(ChBlock, icondata_include_all))]
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
#[cfg(any(ChBluetooth, icondata_include_all))]
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
#[cfg(any(ChBluetoothConnected, icondata_include_all))]
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
#[cfg(any(ChBluetoothSearching, icondata_include_all))]
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
#[cfg(any(ChBluetoothSlash, icondata_include_all))]
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
#[cfg(any(ChBook, icondata_include_all))]
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
#[cfg(any(ChBookOpen, icondata_include_all))]
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
#[cfg(any(ChBookmark, icondata_include_all))]
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
#[cfg(any(ChBriefcase, icondata_include_all))]
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
#[cfg(any(ChBug, icondata_include_all))]
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
#[cfg(any(ChCalendar, icondata_include_all))]
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
#[cfg(any(ChCamera, icondata_include_all))]
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
#[cfg(any(ChCameraVideo, icondata_include_all))]
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
#[cfg(any(ChCameraVideoSlash, icondata_include_all))]
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
#[cfg(any(ChCandy, icondata_include_all))]
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
#[cfg(any(ChCards, icondata_include_all))]
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
#[cfg(any(ChCast, icondata_include_all))]
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
#[cfg(any(ChCertificate, icondata_include_all))]
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
#[cfg(any(ChChartBar, icondata_include_all))]
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
#[cfg(any(ChChartLine, icondata_include_all))]
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
#[cfg(any(ChChevronDown, icondata_include_all))]
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
#[cfg(any(ChChevronLeft, icondata_include_all))]
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
#[cfg(any(ChChevronRight, icondata_include_all))]
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
#[cfg(any(ChChevronUp, icondata_include_all))]
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
#[cfg(any(ChChevronsDown, icondata_include_all))]
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
#[cfg(any(ChChevronsLeft, icondata_include_all))]
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
#[cfg(any(ChChevronsRight, icondata_include_all))]
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
#[cfg(any(ChChevronsUp, icondata_include_all))]
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
#[cfg(any(ChChevronsUpDown, icondata_include_all))]
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
#[cfg(any(ChChip, icondata_include_all))]
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
#[cfg(any(ChCircle, icondata_include_all))]
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
#[cfg(any(ChCircleCross, icondata_include_all))]
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
#[cfg(any(ChCircleMinus, icondata_include_all))]
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
#[cfg(any(ChCircleTick, icondata_include_all))]
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
#[cfg(any(ChCircleWarning, icondata_include_all))]
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
#[cfg(any(ChClipboard, icondata_include_all))]
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
#[cfg(any(ChClipboardTick, icondata_include_all))]
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
#[cfg(any(ChClock, icondata_include_all))]
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
#[cfg(any(ChClockAlarm, icondata_include_all))]
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
#[cfg(any(ChCloud, icondata_include_all))]
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
#[cfg(any(ChClover, icondata_include_all))]
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
#[cfg(any(ChCode, icondata_include_all))]
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
#[cfg(any(ChCoffee, icondata_include_all))]
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
#[cfg(any(ChCog, icondata_include_all))]
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
#[cfg(any(ChCompass, icondata_include_all))]
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
#[cfg(any(ChConicalFlask, icondata_include_all))]
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
#[cfg(any(ChContainer, icondata_include_all))]
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
#[cfg(any(ChCopy, icondata_include_all))]
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
#[cfg(any(ChCopyleft, icondata_include_all))]
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
#[cfg(any(ChCopyright, icondata_include_all))]
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
#[cfg(any(ChCreditCard, icondata_include_all))]
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
#[cfg(any(ChCrop, icondata_include_all))]
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
#[cfg(any(ChCross, icondata_include_all))]
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
#[cfg(any(ChCrosshair, icondata_include_all))]
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
#[cfg(any(ChCube, icondata_include_all))]
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
#[cfg(any(ChCursor, icondata_include_all))]
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
#[cfg(any(ChDatabase, icondata_include_all))]
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
#[cfg(any(ChDiamond, icondata_include_all))]
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
#[cfg(any(ChDiff, icondata_include_all))]
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
#[cfg(any(ChDisc, icondata_include_all))]
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
#[cfg(any(ChDownload, icondata_include_all))]
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
#[cfg(any(ChDroplet, icondata_include_all))]
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
#[cfg(any(ChEraser, icondata_include_all))]
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
#[cfg(any(ChEye, icondata_include_all))]
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
#[cfg(any(ChEyeSlash, icondata_include_all))]
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
#[cfg(any(ChFaceFrown, icondata_include_all))]
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
#[cfg(any(ChFaceNeutral, icondata_include_all))]
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
#[cfg(any(ChFaceSmile, icondata_include_all))]
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
#[cfg(any(ChFile, icondata_include_all))]
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
#[cfg(any(ChFileBinary, icondata_include_all))]
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
#[cfg(any(ChFileCode, icondata_include_all))]
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
#[cfg(any(ChFileSymlink, icondata_include_all))]
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
#[cfg(any(ChFiles, icondata_include_all))]
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
#[cfg(any(ChFilter, icondata_include_all))]
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
#[cfg(any(ChFlag, icondata_include_all))]
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
#[cfg(any(ChFlame, icondata_include_all))]
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
#[cfg(any(ChFloppyDisk, icondata_include_all))]
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
#[cfg(any(ChFolder, icondata_include_all))]
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
#[cfg(any(ChFolderSymlink, icondata_include_all))]
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
#[cfg(any(ChFolders, icondata_include_all))]
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
#[cfg(any(ChForward, icondata_include_all))]
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
#[cfg(any(ChGamepad, icondata_include_all))]
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
#[cfg(any(ChGem, icondata_include_all))]
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
#[cfg(any(ChGift, icondata_include_all))]
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
#[cfg(any(ChGitBranch, icondata_include_all))]
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
#[cfg(any(ChGitCherryPick, icondata_include_all))]
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
#[cfg(any(ChGitCommit, icondata_include_all))]
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
#[cfg(any(ChGitCompare, icondata_include_all))]
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
#[cfg(any(ChGitFork, icondata_include_all))]
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
#[cfg(any(ChGitMerge, icondata_include_all))]
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
#[cfg(any(ChGitRequest, icondata_include_all))]
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
#[cfg(any(ChGitRequestCross, icondata_include_all))]
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
#[cfg(any(ChGitRequestDraft, icondata_include_all))]
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
#[cfg(any(ChGithub, icondata_include_all))]
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
#[cfg(any(ChGitlab, icondata_include_all))]
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
#[cfg(any(ChGlasses, icondata_include_all))]
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
#[cfg(any(ChGlobe, icondata_include_all))]
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
#[cfg(any(ChGrabHorizontal, icondata_include_all))]
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
#[cfg(any(ChGrabVertical, icondata_include_all))]
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
#[cfg(any(ChGraduateCap, icondata_include_all))]
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
#[cfg(any(ChHash, icondata_include_all))]
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
#[cfg(any(ChHeadphones, icondata_include_all))]
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
#[cfg(any(ChHeart, icondata_include_all))]
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
#[cfg(any(ChHelp, icondata_include_all))]
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
#[cfg(any(ChHexagon, icondata_include_all))]
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
#[cfg(any(ChHome, icondata_include_all))]
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
#[cfg(any(ChHourglass, icondata_include_all))]
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
#[cfg(any(ChId, icondata_include_all))]
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
#[cfg(any(ChImage, icondata_include_all))]
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
#[cfg(any(ChInbox, icondata_include_all))]
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
#[cfg(any(ChInfinity, icondata_include_all))]
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
#[cfg(any(ChInfo, icondata_include_all))]
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
#[cfg(any(ChKey, icondata_include_all))]
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
#[cfg(any(ChLaptop, icondata_include_all))]
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
#[cfg(any(ChLayoutColumns, icondata_include_all))]
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
#[cfg(any(ChLayoutDashboard, icondata_include_all))]
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
#[cfg(any(ChLayoutGrid, icondata_include_all))]
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
#[cfg(any(ChLayoutList, icondata_include_all))]
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
#[cfg(any(ChLayoutRows, icondata_include_all))]
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
#[cfg(any(ChLayoutSidebar, icondata_include_all))]
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
#[cfg(any(ChLayoutStackH, icondata_include_all))]
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
#[cfg(any(ChLayoutStackV, icondata_include_all))]
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
#[cfg(any(ChLightbulb, icondata_include_all))]
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
#[cfg(any(ChLightningBolt, icondata_include_all))]
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
#[cfg(any(ChLink, icondata_include_all))]
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
#[cfg(any(ChLinkExternal, icondata_include_all))]
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
#[cfg(any(ChLinkSlash, icondata_include_all))]
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
#[cfg(any(ChMail, icondata_include_all))]
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
#[cfg(any(ChMap, icondata_include_all))]
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
#[cfg(any(ChMapPin, icondata_include_all))]
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
#[cfg(any(ChMediaBack, icondata_include_all))]
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
#[cfg(any(ChMediaEject, icondata_include_all))]
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
#[cfg(any(ChMediaFastForward, icondata_include_all))]
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
#[cfg(any(ChMediaPause, icondata_include_all))]
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
#[cfg(any(ChMediaPlay, icondata_include_all))]
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
#[cfg(any(ChMediaRewind, icondata_include_all))]
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
#[cfg(any(ChMediaSkip, icondata_include_all))]
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
#[cfg(any(ChMenuHamburger, icondata_include_all))]
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
#[cfg(any(ChMenuKebab, icondata_include_all))]
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
#[cfg(any(ChMenuMeatball, icondata_include_all))]
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
#[cfg(any(ChMessage, icondata_include_all))]
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
#[cfg(any(ChMessages, icondata_include_all))]
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
#[cfg(any(ChMicrophone, icondata_include_all))]
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
#[cfg(any(ChMinus, icondata_include_all))]
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
#[cfg(any(ChMobile, icondata_include_all))]
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
#[cfg(any(ChMonitor, icondata_include_all))]
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
#[cfg(any(ChMonitorArrow, icondata_include_all))]
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
#[cfg(any(ChMonitorCross, icondata_include_all))]
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
#[cfg(any(ChMoon, icondata_include_all))]
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
#[cfg(any(ChMove, icondata_include_all))]
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
#[cfg(any(ChMusic, icondata_include_all))]
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
#[cfg(any(ChNewspaper, icondata_include_all))]
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
#[cfg(any(ChNorthStar, icondata_include_all))]
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
#[cfg(any(ChNotes, icondata_include_all))]
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
#[cfg(any(ChNotesCross, icondata_include_all))]
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
#[cfg(any(ChNotesTick, icondata_include_all))]
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
#[cfg(any(ChNut, icondata_include_all))]
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
#[cfg(any(ChOctagon, icondata_include_all))]
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
#[cfg(any(ChOctagonWarning, icondata_include_all))]
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
#[cfg(any(ChOrganisation, icondata_include_all))]
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
#[cfg(any(ChPackage, icondata_include_all))]
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
#[cfg(any(ChPadlock, icondata_include_all))]
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
#[cfg(any(ChPaperPlane, icondata_include_all))]
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
#[cfg(any(ChPaperclip, icondata_include_all))]
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
#[cfg(any(ChPencil, icondata_include_all))]
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
#[cfg(any(ChPeople, icondata_include_all))]
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
#[cfg(any(ChPerson, icondata_include_all))]
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
#[cfg(any(ChPhone, icondata_include_all))]
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
#[cfg(any(ChPhoneCall, icondata_include_all))]
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
#[cfg(any(ChPhoneCross, icondata_include_all))]
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
#[cfg(any(ChPhoneForward, icondata_include_all))]
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
#[cfg(any(ChPhoneIncoming, icondata_include_all))]
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
#[cfg(any(ChPhoneOutgoing, icondata_include_all))]
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
#[cfg(any(ChPin, icondata_include_all))]
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
#[cfg(any(ChPlantPot, icondata_include_all))]
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
#[cfg(any(ChPlus, icondata_include_all))]
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
#[cfg(any(ChPower, icondata_include_all))]
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
#[cfg(any(ChPrinter, icondata_include_all))]
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
#[cfg(any(ChPulse, icondata_include_all))]
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
#[cfg(any(ChQuote, icondata_include_all))]
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
#[cfg(any(ChRefresh, icondata_include_all))]
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
#[cfg(any(ChReply, icondata_include_all))]
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
#[cfg(any(ChRobot, icondata_include_all))]
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
#[cfg(any(ChRocket, icondata_include_all))]
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
#[cfg(any(ChRotateAntiClockwise, icondata_include_all))]
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
#[cfg(any(ChRotateClockwise, icondata_include_all))]
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
#[cfg(any(ChScales, icondata_include_all))]
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
#[cfg(any(ChScreenMaximise, icondata_include_all))]
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
#[cfg(any(ChScreenMinimise, icondata_include_all))]
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
#[cfg(any(ChSearch, icondata_include_all))]
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
#[cfg(any(ChServer, icondata_include_all))]
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
#[cfg(any(ChShare, icondata_include_all))]
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
#[cfg(any(ChShield, icondata_include_all))]
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
#[cfg(any(ChShieldCross, icondata_include_all))]
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
#[cfg(any(ChShieldKeyhole, icondata_include_all))]
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
#[cfg(any(ChShieldTick, icondata_include_all))]
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
#[cfg(any(ChShieldWarning, icondata_include_all))]
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
#[cfg(any(ChShoppingBag, icondata_include_all))]
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
#[cfg(any(ChSignIn, icondata_include_all))]
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
#[cfg(any(ChSignOut, icondata_include_all))]
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
#[cfg(any(ChSignpost, icondata_include_all))]
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
#[cfg(any(ChSkull, icondata_include_all))]
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
#[cfg(any(ChSnowflake, icondata_include_all))]
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
#[cfg(any(ChSoundDown, icondata_include_all))]
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
#[cfg(any(ChSoundMute, icondata_include_all))]
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
#[cfg(any(ChSoundUp, icondata_include_all))]
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
#[cfg(any(ChSpeaker, icondata_include_all))]
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
#[cfg(any(ChSquare, icondata_include_all))]
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
#[cfg(any(ChSquareCross, icondata_include_all))]
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
#[cfg(any(ChSquareTick, icondata_include_all))]
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
#[cfg(any(ChStack, icondata_include_all))]
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
#[cfg(any(ChStackPop, icondata_include_all))]
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
#[cfg(any(ChStackPush, icondata_include_all))]
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
#[cfg(any(ChStar, icondata_include_all))]
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
#[cfg(any(ChStickyNote, icondata_include_all))]
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
#[cfg(any(ChSun, icondata_include_all))]
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
#[cfg(any(ChSwapHorizontal, icondata_include_all))]
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
#[cfg(any(ChSwapVertical, icondata_include_all))]
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
#[cfg(any(ChSword, icondata_include_all))]
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
#[cfg(any(ChSwords, icondata_include_all))]
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
#[cfg(any(ChTablet, icondata_include_all))]
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
#[cfg(any(ChTag, icondata_include_all))]
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
#[cfg(any(ChTelescope, icondata_include_all))]
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
#[cfg(any(ChTent, icondata_include_all))]
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
#[cfg(any(ChTerminal, icondata_include_all))]
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
#[cfg(any(ChThumbDown, icondata_include_all))]
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
#[cfg(any(ChThumbUp, icondata_include_all))]
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
#[cfg(any(ChTick, icondata_include_all))]
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
#[cfg(any(ChTickDouble, icondata_include_all))]
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
#[cfg(any(ChTicket, icondata_include_all))]
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
#[cfg(any(ChTreeFir, icondata_include_all))]
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
#[cfg(any(ChTriangle, icondata_include_all))]
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
#[cfg(any(ChTrophy, icondata_include_all))]
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
#[cfg(any(ChUmbrella, icondata_include_all))]
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
#[cfg(any(ChUpload, icondata_include_all))]
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
#[cfg(any(ChWifi, icondata_include_all))]
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
#[cfg(any(ChWifiFair, icondata_include_all))]
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
#[cfg(any(ChWifiPoor, icondata_include_all))]
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
#[cfg(any(ChWifiSlash, icondata_include_all))]
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
#[cfg(any(ChWifiWarning, icondata_include_all))]
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
#[cfg(any(ChZoomIn, icondata_include_all))]
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
#[cfg(any(ChZoomOut, icondata_include_all))]
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
            #[cfg(any(ChAnchor, icondata_include_all))]
            ChIcon::ChAnchor => CH_ANCHOR,
            #[cfg(any(ChApps, icondata_include_all))]
            ChIcon::ChApps => CH_APPS,
            #[cfg(any(ChAppsMinus, icondata_include_all))]
            ChIcon::ChAppsMinus => CH_APPS_MINUS,
            #[cfg(any(ChAppsPlus, icondata_include_all))]
            ChIcon::ChAppsPlus => CH_APPS_PLUS,
            #[cfg(any(ChArchive, icondata_include_all))]
            ChIcon::ChArchive => CH_ARCHIVE,
            #[cfg(any(ChArrowDown, icondata_include_all))]
            ChIcon::ChArrowDown => CH_ARROW_DOWN,
            #[cfg(any(ChArrowDownLeft, icondata_include_all))]
            ChIcon::ChArrowDownLeft => CH_ARROW_DOWN_LEFT,
            #[cfg(any(ChArrowDownRight, icondata_include_all))]
            ChIcon::ChArrowDownRight => CH_ARROW_DOWN_RIGHT,
            #[cfg(any(ChArrowLeft, icondata_include_all))]
            ChIcon::ChArrowLeft => CH_ARROW_LEFT,
            #[cfg(any(ChArrowRight, icondata_include_all))]
            ChIcon::ChArrowRight => CH_ARROW_RIGHT,
            #[cfg(any(ChArrowUp, icondata_include_all))]
            ChIcon::ChArrowUp => CH_ARROW_UP,
            #[cfg(any(ChArrowUpLeft, icondata_include_all))]
            ChIcon::ChArrowUpLeft => CH_ARROW_UP_LEFT,
            #[cfg(any(ChArrowUpRight, icondata_include_all))]
            ChIcon::ChArrowUpRight => CH_ARROW_UP_RIGHT,
            #[cfg(any(ChAtSign, icondata_include_all))]
            ChIcon::ChAtSign => CH_AT_SIGN,
            #[cfg(any(ChAtom, icondata_include_all))]
            ChIcon::ChAtom => CH_ATOM,
            #[cfg(any(ChBell, icondata_include_all))]
            ChIcon::ChBell => CH_BELL,
            #[cfg(any(ChBellSlash, icondata_include_all))]
            ChIcon::ChBellSlash => CH_BELL_SLASH,
            #[cfg(any(ChBin, icondata_include_all))]
            ChIcon::ChBin => CH_BIN,
            #[cfg(any(ChBinary, icondata_include_all))]
            ChIcon::ChBinary => CH_BINARY,
            #[cfg(any(ChBlock, icondata_include_all))]
            ChIcon::ChBlock => CH_BLOCK,
            #[cfg(any(ChBluetooth, icondata_include_all))]
            ChIcon::ChBluetooth => CH_BLUETOOTH,
            #[cfg(any(ChBluetoothConnected, icondata_include_all))]
            ChIcon::ChBluetoothConnected => CH_BLUETOOTH_CONNECTED,
            #[cfg(any(ChBluetoothSearching, icondata_include_all))]
            ChIcon::ChBluetoothSearching => CH_BLUETOOTH_SEARCHING,
            #[cfg(any(ChBluetoothSlash, icondata_include_all))]
            ChIcon::ChBluetoothSlash => CH_BLUETOOTH_SLASH,
            #[cfg(any(ChBook, icondata_include_all))]
            ChIcon::ChBook => CH_BOOK,
            #[cfg(any(ChBookOpen, icondata_include_all))]
            ChIcon::ChBookOpen => CH_BOOK_OPEN,
            #[cfg(any(ChBookmark, icondata_include_all))]
            ChIcon::ChBookmark => CH_BOOKMARK,
            #[cfg(any(ChBriefcase, icondata_include_all))]
            ChIcon::ChBriefcase => CH_BRIEFCASE,
            #[cfg(any(ChBug, icondata_include_all))]
            ChIcon::ChBug => CH_BUG,
            #[cfg(any(ChCalendar, icondata_include_all))]
            ChIcon::ChCalendar => CH_CALENDAR,
            #[cfg(any(ChCamera, icondata_include_all))]
            ChIcon::ChCamera => CH_CAMERA,
            #[cfg(any(ChCameraVideo, icondata_include_all))]
            ChIcon::ChCameraVideo => CH_CAMERA_VIDEO,
            #[cfg(any(ChCameraVideoSlash, icondata_include_all))]
            ChIcon::ChCameraVideoSlash => CH_CAMERA_VIDEO_SLASH,
            #[cfg(any(ChCandy, icondata_include_all))]
            ChIcon::ChCandy => CH_CANDY,
            #[cfg(any(ChCards, icondata_include_all))]
            ChIcon::ChCards => CH_CARDS,
            #[cfg(any(ChCast, icondata_include_all))]
            ChIcon::ChCast => CH_CAST,
            #[cfg(any(ChCertificate, icondata_include_all))]
            ChIcon::ChCertificate => CH_CERTIFICATE,
            #[cfg(any(ChChartBar, icondata_include_all))]
            ChIcon::ChChartBar => CH_CHART_BAR,
            #[cfg(any(ChChartLine, icondata_include_all))]
            ChIcon::ChChartLine => CH_CHART_LINE,
            #[cfg(any(ChChevronDown, icondata_include_all))]
            ChIcon::ChChevronDown => CH_CHEVRON_DOWN,
            #[cfg(any(ChChevronLeft, icondata_include_all))]
            ChIcon::ChChevronLeft => CH_CHEVRON_LEFT,
            #[cfg(any(ChChevronRight, icondata_include_all))]
            ChIcon::ChChevronRight => CH_CHEVRON_RIGHT,
            #[cfg(any(ChChevronUp, icondata_include_all))]
            ChIcon::ChChevronUp => CH_CHEVRON_UP,
            #[cfg(any(ChChevronsDown, icondata_include_all))]
            ChIcon::ChChevronsDown => CH_CHEVRONS_DOWN,
            #[cfg(any(ChChevronsLeft, icondata_include_all))]
            ChIcon::ChChevronsLeft => CH_CHEVRONS_LEFT,
            #[cfg(any(ChChevronsRight, icondata_include_all))]
            ChIcon::ChChevronsRight => CH_CHEVRONS_RIGHT,
            #[cfg(any(ChChevronsUp, icondata_include_all))]
            ChIcon::ChChevronsUp => CH_CHEVRONS_UP,
            #[cfg(any(ChChevronsUpDown, icondata_include_all))]
            ChIcon::ChChevronsUpDown => CH_CHEVRONS_UP_DOWN,
            #[cfg(any(ChChip, icondata_include_all))]
            ChIcon::ChChip => CH_CHIP,
            #[cfg(any(ChCircle, icondata_include_all))]
            ChIcon::ChCircle => CH_CIRCLE,
            #[cfg(any(ChCircleCross, icondata_include_all))]
            ChIcon::ChCircleCross => CH_CIRCLE_CROSS,
            #[cfg(any(ChCircleMinus, icondata_include_all))]
            ChIcon::ChCircleMinus => CH_CIRCLE_MINUS,
            #[cfg(any(ChCircleTick, icondata_include_all))]
            ChIcon::ChCircleTick => CH_CIRCLE_TICK,
            #[cfg(any(ChCircleWarning, icondata_include_all))]
            ChIcon::ChCircleWarning => CH_CIRCLE_WARNING,
            #[cfg(any(ChClipboard, icondata_include_all))]
            ChIcon::ChClipboard => CH_CLIPBOARD,
            #[cfg(any(ChClipboardTick, icondata_include_all))]
            ChIcon::ChClipboardTick => CH_CLIPBOARD_TICK,
            #[cfg(any(ChClock, icondata_include_all))]
            ChIcon::ChClock => CH_CLOCK,
            #[cfg(any(ChClockAlarm, icondata_include_all))]
            ChIcon::ChClockAlarm => CH_CLOCK_ALARM,
            #[cfg(any(ChCloud, icondata_include_all))]
            ChIcon::ChCloud => CH_CLOUD,
            #[cfg(any(ChClover, icondata_include_all))]
            ChIcon::ChClover => CH_CLOVER,
            #[cfg(any(ChCode, icondata_include_all))]
            ChIcon::ChCode => CH_CODE,
            #[cfg(any(ChCoffee, icondata_include_all))]
            ChIcon::ChCoffee => CH_COFFEE,
            #[cfg(any(ChCog, icondata_include_all))]
            ChIcon::ChCog => CH_COG,
            #[cfg(any(ChCompass, icondata_include_all))]
            ChIcon::ChCompass => CH_COMPASS,
            #[cfg(any(ChConicalFlask, icondata_include_all))]
            ChIcon::ChConicalFlask => CH_CONICAL_FLASK,
            #[cfg(any(ChContainer, icondata_include_all))]
            ChIcon::ChContainer => CH_CONTAINER,
            #[cfg(any(ChCopy, icondata_include_all))]
            ChIcon::ChCopy => CH_COPY,
            #[cfg(any(ChCopyleft, icondata_include_all))]
            ChIcon::ChCopyleft => CH_COPYLEFT,
            #[cfg(any(ChCopyright, icondata_include_all))]
            ChIcon::ChCopyright => CH_COPYRIGHT,
            #[cfg(any(ChCreditCard, icondata_include_all))]
            ChIcon::ChCreditCard => CH_CREDIT_CARD,
            #[cfg(any(ChCrop, icondata_include_all))]
            ChIcon::ChCrop => CH_CROP,
            #[cfg(any(ChCross, icondata_include_all))]
            ChIcon::ChCross => CH_CROSS,
            #[cfg(any(ChCrosshair, icondata_include_all))]
            ChIcon::ChCrosshair => CH_CROSSHAIR,
            #[cfg(any(ChCube, icondata_include_all))]
            ChIcon::ChCube => CH_CUBE,
            #[cfg(any(ChCursor, icondata_include_all))]
            ChIcon::ChCursor => CH_CURSOR,
            #[cfg(any(ChDatabase, icondata_include_all))]
            ChIcon::ChDatabase => CH_DATABASE,
            #[cfg(any(ChDiamond, icondata_include_all))]
            ChIcon::ChDiamond => CH_DIAMOND,
            #[cfg(any(ChDiff, icondata_include_all))]
            ChIcon::ChDiff => CH_DIFF,
            #[cfg(any(ChDisc, icondata_include_all))]
            ChIcon::ChDisc => CH_DISC,
            #[cfg(any(ChDownload, icondata_include_all))]
            ChIcon::ChDownload => CH_DOWNLOAD,
            #[cfg(any(ChDroplet, icondata_include_all))]
            ChIcon::ChDroplet => CH_DROPLET,
            #[cfg(any(ChEraser, icondata_include_all))]
            ChIcon::ChEraser => CH_ERASER,
            #[cfg(any(ChEye, icondata_include_all))]
            ChIcon::ChEye => CH_EYE,
            #[cfg(any(ChEyeSlash, icondata_include_all))]
            ChIcon::ChEyeSlash => CH_EYE_SLASH,
            #[cfg(any(ChFaceFrown, icondata_include_all))]
            ChIcon::ChFaceFrown => CH_FACE_FROWN,
            #[cfg(any(ChFaceNeutral, icondata_include_all))]
            ChIcon::ChFaceNeutral => CH_FACE_NEUTRAL,
            #[cfg(any(ChFaceSmile, icondata_include_all))]
            ChIcon::ChFaceSmile => CH_FACE_SMILE,
            #[cfg(any(ChFile, icondata_include_all))]
            ChIcon::ChFile => CH_FILE,
            #[cfg(any(ChFileBinary, icondata_include_all))]
            ChIcon::ChFileBinary => CH_FILE_BINARY,
            #[cfg(any(ChFileCode, icondata_include_all))]
            ChIcon::ChFileCode => CH_FILE_CODE,
            #[cfg(any(ChFileSymlink, icondata_include_all))]
            ChIcon::ChFileSymlink => CH_FILE_SYMLINK,
            #[cfg(any(ChFiles, icondata_include_all))]
            ChIcon::ChFiles => CH_FILES,
            #[cfg(any(ChFilter, icondata_include_all))]
            ChIcon::ChFilter => CH_FILTER,
            #[cfg(any(ChFlag, icondata_include_all))]
            ChIcon::ChFlag => CH_FLAG,
            #[cfg(any(ChFlame, icondata_include_all))]
            ChIcon::ChFlame => CH_FLAME,
            #[cfg(any(ChFloppyDisk, icondata_include_all))]
            ChIcon::ChFloppyDisk => CH_FLOPPY_DISK,
            #[cfg(any(ChFolder, icondata_include_all))]
            ChIcon::ChFolder => CH_FOLDER,
            #[cfg(any(ChFolderSymlink, icondata_include_all))]
            ChIcon::ChFolderSymlink => CH_FOLDER_SYMLINK,
            #[cfg(any(ChFolders, icondata_include_all))]
            ChIcon::ChFolders => CH_FOLDERS,
            #[cfg(any(ChForward, icondata_include_all))]
            ChIcon::ChForward => CH_FORWARD,
            #[cfg(any(ChGamepad, icondata_include_all))]
            ChIcon::ChGamepad => CH_GAMEPAD,
            #[cfg(any(ChGem, icondata_include_all))]
            ChIcon::ChGem => CH_GEM,
            #[cfg(any(ChGift, icondata_include_all))]
            ChIcon::ChGift => CH_GIFT,
            #[cfg(any(ChGitBranch, icondata_include_all))]
            ChIcon::ChGitBranch => CH_GIT_BRANCH,
            #[cfg(any(ChGitCherryPick, icondata_include_all))]
            ChIcon::ChGitCherryPick => CH_GIT_CHERRY_PICK,
            #[cfg(any(ChGitCommit, icondata_include_all))]
            ChIcon::ChGitCommit => CH_GIT_COMMIT,
            #[cfg(any(ChGitCompare, icondata_include_all))]
            ChIcon::ChGitCompare => CH_GIT_COMPARE,
            #[cfg(any(ChGitFork, icondata_include_all))]
            ChIcon::ChGitFork => CH_GIT_FORK,
            #[cfg(any(ChGitMerge, icondata_include_all))]
            ChIcon::ChGitMerge => CH_GIT_MERGE,
            #[cfg(any(ChGitRequest, icondata_include_all))]
            ChIcon::ChGitRequest => CH_GIT_REQUEST,
            #[cfg(any(ChGitRequestCross, icondata_include_all))]
            ChIcon::ChGitRequestCross => CH_GIT_REQUEST_CROSS,
            #[cfg(any(ChGitRequestDraft, icondata_include_all))]
            ChIcon::ChGitRequestDraft => CH_GIT_REQUEST_DRAFT,
            #[cfg(any(ChGithub, icondata_include_all))]
            ChIcon::ChGithub => CH_GITHUB,
            #[cfg(any(ChGitlab, icondata_include_all))]
            ChIcon::ChGitlab => CH_GITLAB,
            #[cfg(any(ChGlasses, icondata_include_all))]
            ChIcon::ChGlasses => CH_GLASSES,
            #[cfg(any(ChGlobe, icondata_include_all))]
            ChIcon::ChGlobe => CH_GLOBE,
            #[cfg(any(ChGrabHorizontal, icondata_include_all))]
            ChIcon::ChGrabHorizontal => CH_GRAB_HORIZONTAL,
            #[cfg(any(ChGrabVertical, icondata_include_all))]
            ChIcon::ChGrabVertical => CH_GRAB_VERTICAL,
            #[cfg(any(ChGraduateCap, icondata_include_all))]
            ChIcon::ChGraduateCap => CH_GRADUATE_CAP,
            #[cfg(any(ChHash, icondata_include_all))]
            ChIcon::ChHash => CH_HASH,
            #[cfg(any(ChHeadphones, icondata_include_all))]
            ChIcon::ChHeadphones => CH_HEADPHONES,
            #[cfg(any(ChHeart, icondata_include_all))]
            ChIcon::ChHeart => CH_HEART,
            #[cfg(any(ChHelp, icondata_include_all))]
            ChIcon::ChHelp => CH_HELP,
            #[cfg(any(ChHexagon, icondata_include_all))]
            ChIcon::ChHexagon => CH_HEXAGON,
            #[cfg(any(ChHome, icondata_include_all))]
            ChIcon::ChHome => CH_HOME,
            #[cfg(any(ChHourglass, icondata_include_all))]
            ChIcon::ChHourglass => CH_HOURGLASS,
            #[cfg(any(ChId, icondata_include_all))]
            ChIcon::ChId => CH_ID,
            #[cfg(any(ChImage, icondata_include_all))]
            ChIcon::ChImage => CH_IMAGE,
            #[cfg(any(ChInbox, icondata_include_all))]
            ChIcon::ChInbox => CH_INBOX,
            #[cfg(any(ChInfinity, icondata_include_all))]
            ChIcon::ChInfinity => CH_INFINITY,
            #[cfg(any(ChInfo, icondata_include_all))]
            ChIcon::ChInfo => CH_INFO,
            #[cfg(any(ChKey, icondata_include_all))]
            ChIcon::ChKey => CH_KEY,
            #[cfg(any(ChLaptop, icondata_include_all))]
            ChIcon::ChLaptop => CH_LAPTOP,
            #[cfg(any(ChLayoutColumns, icondata_include_all))]
            ChIcon::ChLayoutColumns => CH_LAYOUT_COLUMNS,
            #[cfg(any(ChLayoutDashboard, icondata_include_all))]
            ChIcon::ChLayoutDashboard => CH_LAYOUT_DASHBOARD,
            #[cfg(any(ChLayoutGrid, icondata_include_all))]
            ChIcon::ChLayoutGrid => CH_LAYOUT_GRID,
            #[cfg(any(ChLayoutList, icondata_include_all))]
            ChIcon::ChLayoutList => CH_LAYOUT_LIST,
            #[cfg(any(ChLayoutRows, icondata_include_all))]
            ChIcon::ChLayoutRows => CH_LAYOUT_ROWS,
            #[cfg(any(ChLayoutSidebar, icondata_include_all))]
            ChIcon::ChLayoutSidebar => CH_LAYOUT_SIDEBAR,
            #[cfg(any(ChLayoutStackH, icondata_include_all))]
            ChIcon::ChLayoutStackH => CH_LAYOUT_STACK_H,
            #[cfg(any(ChLayoutStackV, icondata_include_all))]
            ChIcon::ChLayoutStackV => CH_LAYOUT_STACK_V,
            #[cfg(any(ChLightbulb, icondata_include_all))]
            ChIcon::ChLightbulb => CH_LIGHTBULB,
            #[cfg(any(ChLightningBolt, icondata_include_all))]
            ChIcon::ChLightningBolt => CH_LIGHTNING_BOLT,
            #[cfg(any(ChLink, icondata_include_all))]
            ChIcon::ChLink => CH_LINK,
            #[cfg(any(ChLinkExternal, icondata_include_all))]
            ChIcon::ChLinkExternal => CH_LINK_EXTERNAL,
            #[cfg(any(ChLinkSlash, icondata_include_all))]
            ChIcon::ChLinkSlash => CH_LINK_SLASH,
            #[cfg(any(ChMail, icondata_include_all))]
            ChIcon::ChMail => CH_MAIL,
            #[cfg(any(ChMap, icondata_include_all))]
            ChIcon::ChMap => CH_MAP,
            #[cfg(any(ChMapPin, icondata_include_all))]
            ChIcon::ChMapPin => CH_MAP_PIN,
            #[cfg(any(ChMediaBack, icondata_include_all))]
            ChIcon::ChMediaBack => CH_MEDIA_BACK,
            #[cfg(any(ChMediaEject, icondata_include_all))]
            ChIcon::ChMediaEject => CH_MEDIA_EJECT,
            #[cfg(any(ChMediaFastForward, icondata_include_all))]
            ChIcon::ChMediaFastForward => CH_MEDIA_FAST_FORWARD,
            #[cfg(any(ChMediaPause, icondata_include_all))]
            ChIcon::ChMediaPause => CH_MEDIA_PAUSE,
            #[cfg(any(ChMediaPlay, icondata_include_all))]
            ChIcon::ChMediaPlay => CH_MEDIA_PLAY,
            #[cfg(any(ChMediaRewind, icondata_include_all))]
            ChIcon::ChMediaRewind => CH_MEDIA_REWIND,
            #[cfg(any(ChMediaSkip, icondata_include_all))]
            ChIcon::ChMediaSkip => CH_MEDIA_SKIP,
            #[cfg(any(ChMenuHamburger, icondata_include_all))]
            ChIcon::ChMenuHamburger => CH_MENU_HAMBURGER,
            #[cfg(any(ChMenuKebab, icondata_include_all))]
            ChIcon::ChMenuKebab => CH_MENU_KEBAB,
            #[cfg(any(ChMenuMeatball, icondata_include_all))]
            ChIcon::ChMenuMeatball => CH_MENU_MEATBALL,
            #[cfg(any(ChMessage, icondata_include_all))]
            ChIcon::ChMessage => CH_MESSAGE,
            #[cfg(any(ChMessages, icondata_include_all))]
            ChIcon::ChMessages => CH_MESSAGES,
            #[cfg(any(ChMicrophone, icondata_include_all))]
            ChIcon::ChMicrophone => CH_MICROPHONE,
            #[cfg(any(ChMinus, icondata_include_all))]
            ChIcon::ChMinus => CH_MINUS,
            #[cfg(any(ChMobile, icondata_include_all))]
            ChIcon::ChMobile => CH_MOBILE,
            #[cfg(any(ChMonitor, icondata_include_all))]
            ChIcon::ChMonitor => CH_MONITOR,
            #[cfg(any(ChMonitorArrow, icondata_include_all))]
            ChIcon::ChMonitorArrow => CH_MONITOR_ARROW,
            #[cfg(any(ChMonitorCross, icondata_include_all))]
            ChIcon::ChMonitorCross => CH_MONITOR_CROSS,
            #[cfg(any(ChMoon, icondata_include_all))]
            ChIcon::ChMoon => CH_MOON,
            #[cfg(any(ChMove, icondata_include_all))]
            ChIcon::ChMove => CH_MOVE,
            #[cfg(any(ChMusic, icondata_include_all))]
            ChIcon::ChMusic => CH_MUSIC,
            #[cfg(any(ChNewspaper, icondata_include_all))]
            ChIcon::ChNewspaper => CH_NEWSPAPER,
            #[cfg(any(ChNorthStar, icondata_include_all))]
            ChIcon::ChNorthStar => CH_NORTH_STAR,
            #[cfg(any(ChNotes, icondata_include_all))]
            ChIcon::ChNotes => CH_NOTES,
            #[cfg(any(ChNotesCross, icondata_include_all))]
            ChIcon::ChNotesCross => CH_NOTES_CROSS,
            #[cfg(any(ChNotesTick, icondata_include_all))]
            ChIcon::ChNotesTick => CH_NOTES_TICK,
            #[cfg(any(ChNut, icondata_include_all))]
            ChIcon::ChNut => CH_NUT,
            #[cfg(any(ChOctagon, icondata_include_all))]
            ChIcon::ChOctagon => CH_OCTAGON,
            #[cfg(any(ChOctagonWarning, icondata_include_all))]
            ChIcon::ChOctagonWarning => CH_OCTAGON_WARNING,
            #[cfg(any(ChOrganisation, icondata_include_all))]
            ChIcon::ChOrganisation => CH_ORGANISATION,
            #[cfg(any(ChPackage, icondata_include_all))]
            ChIcon::ChPackage => CH_PACKAGE,
            #[cfg(any(ChPadlock, icondata_include_all))]
            ChIcon::ChPadlock => CH_PADLOCK,
            #[cfg(any(ChPaperPlane, icondata_include_all))]
            ChIcon::ChPaperPlane => CH_PAPER_PLANE,
            #[cfg(any(ChPaperclip, icondata_include_all))]
            ChIcon::ChPaperclip => CH_PAPERCLIP,
            #[cfg(any(ChPencil, icondata_include_all))]
            ChIcon::ChPencil => CH_PENCIL,
            #[cfg(any(ChPeople, icondata_include_all))]
            ChIcon::ChPeople => CH_PEOPLE,
            #[cfg(any(ChPerson, icondata_include_all))]
            ChIcon::ChPerson => CH_PERSON,
            #[cfg(any(ChPhone, icondata_include_all))]
            ChIcon::ChPhone => CH_PHONE,
            #[cfg(any(ChPhoneCall, icondata_include_all))]
            ChIcon::ChPhoneCall => CH_PHONE_CALL,
            #[cfg(any(ChPhoneCross, icondata_include_all))]
            ChIcon::ChPhoneCross => CH_PHONE_CROSS,
            #[cfg(any(ChPhoneForward, icondata_include_all))]
            ChIcon::ChPhoneForward => CH_PHONE_FORWARD,
            #[cfg(any(ChPhoneIncoming, icondata_include_all))]
            ChIcon::ChPhoneIncoming => CH_PHONE_INCOMING,
            #[cfg(any(ChPhoneOutgoing, icondata_include_all))]
            ChIcon::ChPhoneOutgoing => CH_PHONE_OUTGOING,
            #[cfg(any(ChPin, icondata_include_all))]
            ChIcon::ChPin => CH_PIN,
            #[cfg(any(ChPlantPot, icondata_include_all))]
            ChIcon::ChPlantPot => CH_PLANT_POT,
            #[cfg(any(ChPlus, icondata_include_all))]
            ChIcon::ChPlus => CH_PLUS,
            #[cfg(any(ChPower, icondata_include_all))]
            ChIcon::ChPower => CH_POWER,
            #[cfg(any(ChPrinter, icondata_include_all))]
            ChIcon::ChPrinter => CH_PRINTER,
            #[cfg(any(ChPulse, icondata_include_all))]
            ChIcon::ChPulse => CH_PULSE,
            #[cfg(any(ChQuote, icondata_include_all))]
            ChIcon::ChQuote => CH_QUOTE,
            #[cfg(any(ChRefresh, icondata_include_all))]
            ChIcon::ChRefresh => CH_REFRESH,
            #[cfg(any(ChReply, icondata_include_all))]
            ChIcon::ChReply => CH_REPLY,
            #[cfg(any(ChRobot, icondata_include_all))]
            ChIcon::ChRobot => CH_ROBOT,
            #[cfg(any(ChRocket, icondata_include_all))]
            ChIcon::ChRocket => CH_ROCKET,
            #[cfg(any(ChRotateAntiClockwise, icondata_include_all))]
            ChIcon::ChRotateAntiClockwise => CH_ROTATE_ANTI_CLOCKWISE,
            #[cfg(any(ChRotateClockwise, icondata_include_all))]
            ChIcon::ChRotateClockwise => CH_ROTATE_CLOCKWISE,
            #[cfg(any(ChScales, icondata_include_all))]
            ChIcon::ChScales => CH_SCALES,
            #[cfg(any(ChScreenMaximise, icondata_include_all))]
            ChIcon::ChScreenMaximise => CH_SCREEN_MAXIMISE,
            #[cfg(any(ChScreenMinimise, icondata_include_all))]
            ChIcon::ChScreenMinimise => CH_SCREEN_MINIMISE,
            #[cfg(any(ChSearch, icondata_include_all))]
            ChIcon::ChSearch => CH_SEARCH,
            #[cfg(any(ChServer, icondata_include_all))]
            ChIcon::ChServer => CH_SERVER,
            #[cfg(any(ChShare, icondata_include_all))]
            ChIcon::ChShare => CH_SHARE,
            #[cfg(any(ChShield, icondata_include_all))]
            ChIcon::ChShield => CH_SHIELD,
            #[cfg(any(ChShieldCross, icondata_include_all))]
            ChIcon::ChShieldCross => CH_SHIELD_CROSS,
            #[cfg(any(ChShieldKeyhole, icondata_include_all))]
            ChIcon::ChShieldKeyhole => CH_SHIELD_KEYHOLE,
            #[cfg(any(ChShieldTick, icondata_include_all))]
            ChIcon::ChShieldTick => CH_SHIELD_TICK,
            #[cfg(any(ChShieldWarning, icondata_include_all))]
            ChIcon::ChShieldWarning => CH_SHIELD_WARNING,
            #[cfg(any(ChShoppingBag, icondata_include_all))]
            ChIcon::ChShoppingBag => CH_SHOPPING_BAG,
            #[cfg(any(ChSignIn, icondata_include_all))]
            ChIcon::ChSignIn => CH_SIGN_IN,
            #[cfg(any(ChSignOut, icondata_include_all))]
            ChIcon::ChSignOut => CH_SIGN_OUT,
            #[cfg(any(ChSignpost, icondata_include_all))]
            ChIcon::ChSignpost => CH_SIGNPOST,
            #[cfg(any(ChSkull, icondata_include_all))]
            ChIcon::ChSkull => CH_SKULL,
            #[cfg(any(ChSnowflake, icondata_include_all))]
            ChIcon::ChSnowflake => CH_SNOWFLAKE,
            #[cfg(any(ChSoundDown, icondata_include_all))]
            ChIcon::ChSoundDown => CH_SOUND_DOWN,
            #[cfg(any(ChSoundMute, icondata_include_all))]
            ChIcon::ChSoundMute => CH_SOUND_MUTE,
            #[cfg(any(ChSoundUp, icondata_include_all))]
            ChIcon::ChSoundUp => CH_SOUND_UP,
            #[cfg(any(ChSpeaker, icondata_include_all))]
            ChIcon::ChSpeaker => CH_SPEAKER,
            #[cfg(any(ChSquare, icondata_include_all))]
            ChIcon::ChSquare => CH_SQUARE,
            #[cfg(any(ChSquareCross, icondata_include_all))]
            ChIcon::ChSquareCross => CH_SQUARE_CROSS,
            #[cfg(any(ChSquareTick, icondata_include_all))]
            ChIcon::ChSquareTick => CH_SQUARE_TICK,
            #[cfg(any(ChStack, icondata_include_all))]
            ChIcon::ChStack => CH_STACK,
            #[cfg(any(ChStackPop, icondata_include_all))]
            ChIcon::ChStackPop => CH_STACK_POP,
            #[cfg(any(ChStackPush, icondata_include_all))]
            ChIcon::ChStackPush => CH_STACK_PUSH,
            #[cfg(any(ChStar, icondata_include_all))]
            ChIcon::ChStar => CH_STAR,
            #[cfg(any(ChStickyNote, icondata_include_all))]
            ChIcon::ChStickyNote => CH_STICKY_NOTE,
            #[cfg(any(ChSun, icondata_include_all))]
            ChIcon::ChSun => CH_SUN,
            #[cfg(any(ChSwapHorizontal, icondata_include_all))]
            ChIcon::ChSwapHorizontal => CH_SWAP_HORIZONTAL,
            #[cfg(any(ChSwapVertical, icondata_include_all))]
            ChIcon::ChSwapVertical => CH_SWAP_VERTICAL,
            #[cfg(any(ChSword, icondata_include_all))]
            ChIcon::ChSword => CH_SWORD,
            #[cfg(any(ChSwords, icondata_include_all))]
            ChIcon::ChSwords => CH_SWORDS,
            #[cfg(any(ChTablet, icondata_include_all))]
            ChIcon::ChTablet => CH_TABLET,
            #[cfg(any(ChTag, icondata_include_all))]
            ChIcon::ChTag => CH_TAG,
            #[cfg(any(ChTelescope, icondata_include_all))]
            ChIcon::ChTelescope => CH_TELESCOPE,
            #[cfg(any(ChTent, icondata_include_all))]
            ChIcon::ChTent => CH_TENT,
            #[cfg(any(ChTerminal, icondata_include_all))]
            ChIcon::ChTerminal => CH_TERMINAL,
            #[cfg(any(ChThumbDown, icondata_include_all))]
            ChIcon::ChThumbDown => CH_THUMB_DOWN,
            #[cfg(any(ChThumbUp, icondata_include_all))]
            ChIcon::ChThumbUp => CH_THUMB_UP,
            #[cfg(any(ChTick, icondata_include_all))]
            ChIcon::ChTick => CH_TICK,
            #[cfg(any(ChTickDouble, icondata_include_all))]
            ChIcon::ChTickDouble => CH_TICK_DOUBLE,
            #[cfg(any(ChTicket, icondata_include_all))]
            ChIcon::ChTicket => CH_TICKET,
            #[cfg(any(ChTreeFir, icondata_include_all))]
            ChIcon::ChTreeFir => CH_TREE_FIR,
            #[cfg(any(ChTriangle, icondata_include_all))]
            ChIcon::ChTriangle => CH_TRIANGLE,
            #[cfg(any(ChTrophy, icondata_include_all))]
            ChIcon::ChTrophy => CH_TROPHY,
            #[cfg(any(ChUmbrella, icondata_include_all))]
            ChIcon::ChUmbrella => CH_UMBRELLA,
            #[cfg(any(ChUpload, icondata_include_all))]
            ChIcon::ChUpload => CH_UPLOAD,
            #[cfg(any(ChWifi, icondata_include_all))]
            ChIcon::ChWifi => CH_WIFI,
            #[cfg(any(ChWifiFair, icondata_include_all))]
            ChIcon::ChWifiFair => CH_WIFI_FAIR,
            #[cfg(any(ChWifiPoor, icondata_include_all))]
            ChIcon::ChWifiPoor => CH_WIFI_POOR,
            #[cfg(any(ChWifiSlash, icondata_include_all))]
            ChIcon::ChWifiSlash => CH_WIFI_SLASH,
            #[cfg(any(ChWifiWarning, icondata_include_all))]
            ChIcon::ChWifiWarning => CH_WIFI_WARNING,
            #[cfg(any(ChZoomIn, icondata_include_all))]
            ChIcon::ChZoomIn => CH_ZOOM_IN,
            #[cfg(any(ChZoomOut, icondata_include_all))]
            ChIcon::ChZoomOut => CH_ZOOM_OUT,
        }
    }
}