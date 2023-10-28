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
pub enum FiIcon {
    #[cfg(any(FiActivity, icondata_include_all))]
    FiActivity,
    #[cfg(any(FiAirplay, icondata_include_all))]
    FiAirplay,
    #[cfg(any(FiAlertCircle, icondata_include_all))]
    FiAlertCircle,
    #[cfg(any(FiAlertOctagon, icondata_include_all))]
    FiAlertOctagon,
    #[cfg(any(FiAlertTriangle, icondata_include_all))]
    FiAlertTriangle,
    #[cfg(any(FiAlignCenter, icondata_include_all))]
    FiAlignCenter,
    #[cfg(any(FiAlignJustify, icondata_include_all))]
    FiAlignJustify,
    #[cfg(any(FiAlignLeft, icondata_include_all))]
    FiAlignLeft,
    #[cfg(any(FiAlignRight, icondata_include_all))]
    FiAlignRight,
    #[cfg(any(FiAnchor, icondata_include_all))]
    FiAnchor,
    #[cfg(any(FiAperture, icondata_include_all))]
    FiAperture,
    #[cfg(any(FiArchive, icondata_include_all))]
    FiArchive,
    #[cfg(any(FiArrowDown, icondata_include_all))]
    FiArrowDown,
    #[cfg(any(FiArrowDownCircle, icondata_include_all))]
    FiArrowDownCircle,
    #[cfg(any(FiArrowDownLeft, icondata_include_all))]
    FiArrowDownLeft,
    #[cfg(any(FiArrowDownRight, icondata_include_all))]
    FiArrowDownRight,
    #[cfg(any(FiArrowLeft, icondata_include_all))]
    FiArrowLeft,
    #[cfg(any(FiArrowLeftCircle, icondata_include_all))]
    FiArrowLeftCircle,
    #[cfg(any(FiArrowRight, icondata_include_all))]
    FiArrowRight,
    #[cfg(any(FiArrowRightCircle, icondata_include_all))]
    FiArrowRightCircle,
    #[cfg(any(FiArrowUp, icondata_include_all))]
    FiArrowUp,
    #[cfg(any(FiArrowUpCircle, icondata_include_all))]
    FiArrowUpCircle,
    #[cfg(any(FiArrowUpLeft, icondata_include_all))]
    FiArrowUpLeft,
    #[cfg(any(FiArrowUpRight, icondata_include_all))]
    FiArrowUpRight,
    #[cfg(any(FiAtSign, icondata_include_all))]
    FiAtSign,
    #[cfg(any(FiAward, icondata_include_all))]
    FiAward,
    #[cfg(any(FiBarChart, icondata_include_all))]
    FiBarChart,
    #[cfg(any(FiBarChart2, icondata_include_all))]
    FiBarChart2,
    #[cfg(any(FiBattery, icondata_include_all))]
    FiBattery,
    #[cfg(any(FiBatteryCharging, icondata_include_all))]
    FiBatteryCharging,
    #[cfg(any(FiBell, icondata_include_all))]
    FiBell,
    #[cfg(any(FiBellOff, icondata_include_all))]
    FiBellOff,
    #[cfg(any(FiBluetooth, icondata_include_all))]
    FiBluetooth,
    #[cfg(any(FiBold, icondata_include_all))]
    FiBold,
    #[cfg(any(FiBook, icondata_include_all))]
    FiBook,
    #[cfg(any(FiBookOpen, icondata_include_all))]
    FiBookOpen,
    #[cfg(any(FiBookmark, icondata_include_all))]
    FiBookmark,
    #[cfg(any(FiBox, icondata_include_all))]
    FiBox,
    #[cfg(any(FiBriefcase, icondata_include_all))]
    FiBriefcase,
    #[cfg(any(FiCalendar, icondata_include_all))]
    FiCalendar,
    #[cfg(any(FiCamera, icondata_include_all))]
    FiCamera,
    #[cfg(any(FiCameraOff, icondata_include_all))]
    FiCameraOff,
    #[cfg(any(FiCast, icondata_include_all))]
    FiCast,
    #[cfg(any(FiCheck, icondata_include_all))]
    FiCheck,
    #[cfg(any(FiCheckCircle, icondata_include_all))]
    FiCheckCircle,
    #[cfg(any(FiCheckSquare, icondata_include_all))]
    FiCheckSquare,
    #[cfg(any(FiChevronDown, icondata_include_all))]
    FiChevronDown,
    #[cfg(any(FiChevronLeft, icondata_include_all))]
    FiChevronLeft,
    #[cfg(any(FiChevronRight, icondata_include_all))]
    FiChevronRight,
    #[cfg(any(FiChevronUp, icondata_include_all))]
    FiChevronUp,
    #[cfg(any(FiChevronsDown, icondata_include_all))]
    FiChevronsDown,
    #[cfg(any(FiChevronsLeft, icondata_include_all))]
    FiChevronsLeft,
    #[cfg(any(FiChevronsRight, icondata_include_all))]
    FiChevronsRight,
    #[cfg(any(FiChevronsUp, icondata_include_all))]
    FiChevronsUp,
    #[cfg(any(FiChrome, icondata_include_all))]
    FiChrome,
    #[cfg(any(FiCircle, icondata_include_all))]
    FiCircle,
    #[cfg(any(FiClipboard, icondata_include_all))]
    FiClipboard,
    #[cfg(any(FiClock, icondata_include_all))]
    FiClock,
    #[cfg(any(FiCloud, icondata_include_all))]
    FiCloud,
    #[cfg(any(FiCloudDrizzle, icondata_include_all))]
    FiCloudDrizzle,
    #[cfg(any(FiCloudLightning, icondata_include_all))]
    FiCloudLightning,
    #[cfg(any(FiCloudOff, icondata_include_all))]
    FiCloudOff,
    #[cfg(any(FiCloudRain, icondata_include_all))]
    FiCloudRain,
    #[cfg(any(FiCloudSnow, icondata_include_all))]
    FiCloudSnow,
    #[cfg(any(FiCode, icondata_include_all))]
    FiCode,
    #[cfg(any(FiCodepen, icondata_include_all))]
    FiCodepen,
    #[cfg(any(FiCodesandbox, icondata_include_all))]
    FiCodesandbox,
    #[cfg(any(FiCoffee, icondata_include_all))]
    FiCoffee,
    #[cfg(any(FiColumns, icondata_include_all))]
    FiColumns,
    #[cfg(any(FiCommand, icondata_include_all))]
    FiCommand,
    #[cfg(any(FiCompass, icondata_include_all))]
    FiCompass,
    #[cfg(any(FiCopy, icondata_include_all))]
    FiCopy,
    #[cfg(any(FiCornerDownLeft, icondata_include_all))]
    FiCornerDownLeft,
    #[cfg(any(FiCornerDownRight, icondata_include_all))]
    FiCornerDownRight,
    #[cfg(any(FiCornerLeftDown, icondata_include_all))]
    FiCornerLeftDown,
    #[cfg(any(FiCornerLeftUp, icondata_include_all))]
    FiCornerLeftUp,
    #[cfg(any(FiCornerRightDown, icondata_include_all))]
    FiCornerRightDown,
    #[cfg(any(FiCornerRightUp, icondata_include_all))]
    FiCornerRightUp,
    #[cfg(any(FiCornerUpLeft, icondata_include_all))]
    FiCornerUpLeft,
    #[cfg(any(FiCornerUpRight, icondata_include_all))]
    FiCornerUpRight,
    #[cfg(any(FiCpu, icondata_include_all))]
    FiCpu,
    #[cfg(any(FiCreditCard, icondata_include_all))]
    FiCreditCard,
    #[cfg(any(FiCrop, icondata_include_all))]
    FiCrop,
    #[cfg(any(FiCrosshair, icondata_include_all))]
    FiCrosshair,
    #[cfg(any(FiDatabase, icondata_include_all))]
    FiDatabase,
    #[cfg(any(FiDelete, icondata_include_all))]
    FiDelete,
    #[cfg(any(FiDisc, icondata_include_all))]
    FiDisc,
    #[cfg(any(FiDivide, icondata_include_all))]
    FiDivide,
    #[cfg(any(FiDivideCircle, icondata_include_all))]
    FiDivideCircle,
    #[cfg(any(FiDivideSquare, icondata_include_all))]
    FiDivideSquare,
    #[cfg(any(FiDollarSign, icondata_include_all))]
    FiDollarSign,
    #[cfg(any(FiDownload, icondata_include_all))]
    FiDownload,
    #[cfg(any(FiDownloadCloud, icondata_include_all))]
    FiDownloadCloud,
    #[cfg(any(FiDribbble, icondata_include_all))]
    FiDribbble,
    #[cfg(any(FiDroplet, icondata_include_all))]
    FiDroplet,
    #[cfg(any(FiEdit, icondata_include_all))]
    FiEdit,
    #[cfg(any(FiEdit2, icondata_include_all))]
    FiEdit2,
    #[cfg(any(FiEdit3, icondata_include_all))]
    FiEdit3,
    #[cfg(any(FiExternalLink, icondata_include_all))]
    FiExternalLink,
    #[cfg(any(FiEye, icondata_include_all))]
    FiEye,
    #[cfg(any(FiEyeOff, icondata_include_all))]
    FiEyeOff,
    #[cfg(any(FiFacebook, icondata_include_all))]
    FiFacebook,
    #[cfg(any(FiFastForward, icondata_include_all))]
    FiFastForward,
    #[cfg(any(FiFeather, icondata_include_all))]
    FiFeather,
    #[cfg(any(FiFigma, icondata_include_all))]
    FiFigma,
    #[cfg(any(FiFile, icondata_include_all))]
    FiFile,
    #[cfg(any(FiFileMinus, icondata_include_all))]
    FiFileMinus,
    #[cfg(any(FiFilePlus, icondata_include_all))]
    FiFilePlus,
    #[cfg(any(FiFileText, icondata_include_all))]
    FiFileText,
    #[cfg(any(FiFilm, icondata_include_all))]
    FiFilm,
    #[cfg(any(FiFilter, icondata_include_all))]
    FiFilter,
    #[cfg(any(FiFlag, icondata_include_all))]
    FiFlag,
    #[cfg(any(FiFolder, icondata_include_all))]
    FiFolder,
    #[cfg(any(FiFolderMinus, icondata_include_all))]
    FiFolderMinus,
    #[cfg(any(FiFolderPlus, icondata_include_all))]
    FiFolderPlus,
    #[cfg(any(FiFramer, icondata_include_all))]
    FiFramer,
    #[cfg(any(FiFrown, icondata_include_all))]
    FiFrown,
    #[cfg(any(FiGift, icondata_include_all))]
    FiGift,
    #[cfg(any(FiGitBranch, icondata_include_all))]
    FiGitBranch,
    #[cfg(any(FiGitCommit, icondata_include_all))]
    FiGitCommit,
    #[cfg(any(FiGitMerge, icondata_include_all))]
    FiGitMerge,
    #[cfg(any(FiGitPullRequest, icondata_include_all))]
    FiGitPullRequest,
    #[cfg(any(FiGithub, icondata_include_all))]
    FiGithub,
    #[cfg(any(FiGitlab, icondata_include_all))]
    FiGitlab,
    #[cfg(any(FiGlobe, icondata_include_all))]
    FiGlobe,
    #[cfg(any(FiGrid, icondata_include_all))]
    FiGrid,
    #[cfg(any(FiHardDrive, icondata_include_all))]
    FiHardDrive,
    #[cfg(any(FiHash, icondata_include_all))]
    FiHash,
    #[cfg(any(FiHeadphones, icondata_include_all))]
    FiHeadphones,
    #[cfg(any(FiHeart, icondata_include_all))]
    FiHeart,
    #[cfg(any(FiHelpCircle, icondata_include_all))]
    FiHelpCircle,
    #[cfg(any(FiHexagon, icondata_include_all))]
    FiHexagon,
    #[cfg(any(FiHome, icondata_include_all))]
    FiHome,
    #[cfg(any(FiImage, icondata_include_all))]
    FiImage,
    #[cfg(any(FiInbox, icondata_include_all))]
    FiInbox,
    #[cfg(any(FiInfo, icondata_include_all))]
    FiInfo,
    #[cfg(any(FiInstagram, icondata_include_all))]
    FiInstagram,
    #[cfg(any(FiItalic, icondata_include_all))]
    FiItalic,
    #[cfg(any(FiKey, icondata_include_all))]
    FiKey,
    #[cfg(any(FiLayers, icondata_include_all))]
    FiLayers,
    #[cfg(any(FiLayout, icondata_include_all))]
    FiLayout,
    #[cfg(any(FiLifeBuoy, icondata_include_all))]
    FiLifeBuoy,
    #[cfg(any(FiLink, icondata_include_all))]
    FiLink,
    #[cfg(any(FiLink2, icondata_include_all))]
    FiLink2,
    #[cfg(any(FiLinkedin, icondata_include_all))]
    FiLinkedin,
    #[cfg(any(FiList, icondata_include_all))]
    FiList,
    #[cfg(any(FiLoader, icondata_include_all))]
    FiLoader,
    #[cfg(any(FiLock, icondata_include_all))]
    FiLock,
    #[cfg(any(FiLogIn, icondata_include_all))]
    FiLogIn,
    #[cfg(any(FiLogOut, icondata_include_all))]
    FiLogOut,
    #[cfg(any(FiMail, icondata_include_all))]
    FiMail,
    #[cfg(any(FiMap, icondata_include_all))]
    FiMap,
    #[cfg(any(FiMapPin, icondata_include_all))]
    FiMapPin,
    #[cfg(any(FiMaximize, icondata_include_all))]
    FiMaximize,
    #[cfg(any(FiMaximize2, icondata_include_all))]
    FiMaximize2,
    #[cfg(any(FiMeh, icondata_include_all))]
    FiMeh,
    #[cfg(any(FiMenu, icondata_include_all))]
    FiMenu,
    #[cfg(any(FiMessageCircle, icondata_include_all))]
    FiMessageCircle,
    #[cfg(any(FiMessageSquare, icondata_include_all))]
    FiMessageSquare,
    #[cfg(any(FiMic, icondata_include_all))]
    FiMic,
    #[cfg(any(FiMicOff, icondata_include_all))]
    FiMicOff,
    #[cfg(any(FiMinimize, icondata_include_all))]
    FiMinimize,
    #[cfg(any(FiMinimize2, icondata_include_all))]
    FiMinimize2,
    #[cfg(any(FiMinus, icondata_include_all))]
    FiMinus,
    #[cfg(any(FiMinusCircle, icondata_include_all))]
    FiMinusCircle,
    #[cfg(any(FiMinusSquare, icondata_include_all))]
    FiMinusSquare,
    #[cfg(any(FiMonitor, icondata_include_all))]
    FiMonitor,
    #[cfg(any(FiMoon, icondata_include_all))]
    FiMoon,
    #[cfg(any(FiMoreHorizontal, icondata_include_all))]
    FiMoreHorizontal,
    #[cfg(any(FiMoreVertical, icondata_include_all))]
    FiMoreVertical,
    #[cfg(any(FiMousePointer, icondata_include_all))]
    FiMousePointer,
    #[cfg(any(FiMove, icondata_include_all))]
    FiMove,
    #[cfg(any(FiMusic, icondata_include_all))]
    FiMusic,
    #[cfg(any(FiNavigation, icondata_include_all))]
    FiNavigation,
    #[cfg(any(FiNavigation2, icondata_include_all))]
    FiNavigation2,
    #[cfg(any(FiOctagon, icondata_include_all))]
    FiOctagon,
    #[cfg(any(FiPackage, icondata_include_all))]
    FiPackage,
    #[cfg(any(FiPaperclip, icondata_include_all))]
    FiPaperclip,
    #[cfg(any(FiPause, icondata_include_all))]
    FiPause,
    #[cfg(any(FiPauseCircle, icondata_include_all))]
    FiPauseCircle,
    #[cfg(any(FiPenTool, icondata_include_all))]
    FiPenTool,
    #[cfg(any(FiPercent, icondata_include_all))]
    FiPercent,
    #[cfg(any(FiPhone, icondata_include_all))]
    FiPhone,
    #[cfg(any(FiPhoneCall, icondata_include_all))]
    FiPhoneCall,
    #[cfg(any(FiPhoneForwarded, icondata_include_all))]
    FiPhoneForwarded,
    #[cfg(any(FiPhoneIncoming, icondata_include_all))]
    FiPhoneIncoming,
    #[cfg(any(FiPhoneMissed, icondata_include_all))]
    FiPhoneMissed,
    #[cfg(any(FiPhoneOff, icondata_include_all))]
    FiPhoneOff,
    #[cfg(any(FiPhoneOutgoing, icondata_include_all))]
    FiPhoneOutgoing,
    #[cfg(any(FiPieChart, icondata_include_all))]
    FiPieChart,
    #[cfg(any(FiPlay, icondata_include_all))]
    FiPlay,
    #[cfg(any(FiPlayCircle, icondata_include_all))]
    FiPlayCircle,
    #[cfg(any(FiPlus, icondata_include_all))]
    FiPlus,
    #[cfg(any(FiPlusCircle, icondata_include_all))]
    FiPlusCircle,
    #[cfg(any(FiPlusSquare, icondata_include_all))]
    FiPlusSquare,
    #[cfg(any(FiPocket, icondata_include_all))]
    FiPocket,
    #[cfg(any(FiPower, icondata_include_all))]
    FiPower,
    #[cfg(any(FiPrinter, icondata_include_all))]
    FiPrinter,
    #[cfg(any(FiRadio, icondata_include_all))]
    FiRadio,
    #[cfg(any(FiRefreshCcw, icondata_include_all))]
    FiRefreshCcw,
    #[cfg(any(FiRefreshCw, icondata_include_all))]
    FiRefreshCw,
    #[cfg(any(FiRepeat, icondata_include_all))]
    FiRepeat,
    #[cfg(any(FiRewind, icondata_include_all))]
    FiRewind,
    #[cfg(any(FiRotateCcw, icondata_include_all))]
    FiRotateCcw,
    #[cfg(any(FiRotateCw, icondata_include_all))]
    FiRotateCw,
    #[cfg(any(FiRss, icondata_include_all))]
    FiRss,
    #[cfg(any(FiSave, icondata_include_all))]
    FiSave,
    #[cfg(any(FiScissors, icondata_include_all))]
    FiScissors,
    #[cfg(any(FiSearch, icondata_include_all))]
    FiSearch,
    #[cfg(any(FiSend, icondata_include_all))]
    FiSend,
    #[cfg(any(FiServer, icondata_include_all))]
    FiServer,
    #[cfg(any(FiSettings, icondata_include_all))]
    FiSettings,
    #[cfg(any(FiShare, icondata_include_all))]
    FiShare,
    #[cfg(any(FiShare2, icondata_include_all))]
    FiShare2,
    #[cfg(any(FiShield, icondata_include_all))]
    FiShield,
    #[cfg(any(FiShieldOff, icondata_include_all))]
    FiShieldOff,
    #[cfg(any(FiShoppingBag, icondata_include_all))]
    FiShoppingBag,
    #[cfg(any(FiShoppingCart, icondata_include_all))]
    FiShoppingCart,
    #[cfg(any(FiShuffle, icondata_include_all))]
    FiShuffle,
    #[cfg(any(FiSidebar, icondata_include_all))]
    FiSidebar,
    #[cfg(any(FiSkipBack, icondata_include_all))]
    FiSkipBack,
    #[cfg(any(FiSkipForward, icondata_include_all))]
    FiSkipForward,
    #[cfg(any(FiSlack, icondata_include_all))]
    FiSlack,
    #[cfg(any(FiSlash, icondata_include_all))]
    FiSlash,
    #[cfg(any(FiSliders, icondata_include_all))]
    FiSliders,
    #[cfg(any(FiSmartphone, icondata_include_all))]
    FiSmartphone,
    #[cfg(any(FiSmile, icondata_include_all))]
    FiSmile,
    #[cfg(any(FiSpeaker, icondata_include_all))]
    FiSpeaker,
    #[cfg(any(FiSquare, icondata_include_all))]
    FiSquare,
    #[cfg(any(FiStar, icondata_include_all))]
    FiStar,
    #[cfg(any(FiStopCircle, icondata_include_all))]
    FiStopCircle,
    #[cfg(any(FiSun, icondata_include_all))]
    FiSun,
    #[cfg(any(FiSunrise, icondata_include_all))]
    FiSunrise,
    #[cfg(any(FiSunset, icondata_include_all))]
    FiSunset,
    #[cfg(any(FiTable, icondata_include_all))]
    FiTable,
    #[cfg(any(FiTablet, icondata_include_all))]
    FiTablet,
    #[cfg(any(FiTag, icondata_include_all))]
    FiTag,
    #[cfg(any(FiTarget, icondata_include_all))]
    FiTarget,
    #[cfg(any(FiTerminal, icondata_include_all))]
    FiTerminal,
    #[cfg(any(FiThermometer, icondata_include_all))]
    FiThermometer,
    #[cfg(any(FiThumbsDown, icondata_include_all))]
    FiThumbsDown,
    #[cfg(any(FiThumbsUp, icondata_include_all))]
    FiThumbsUp,
    #[cfg(any(FiToggleLeft, icondata_include_all))]
    FiToggleLeft,
    #[cfg(any(FiToggleRight, icondata_include_all))]
    FiToggleRight,
    #[cfg(any(FiTool, icondata_include_all))]
    FiTool,
    #[cfg(any(FiTrash, icondata_include_all))]
    FiTrash,
    #[cfg(any(FiTrash2, icondata_include_all))]
    FiTrash2,
    #[cfg(any(FiTrello, icondata_include_all))]
    FiTrello,
    #[cfg(any(FiTrendingDown, icondata_include_all))]
    FiTrendingDown,
    #[cfg(any(FiTrendingUp, icondata_include_all))]
    FiTrendingUp,
    #[cfg(any(FiTriangle, icondata_include_all))]
    FiTriangle,
    #[cfg(any(FiTruck, icondata_include_all))]
    FiTruck,
    #[cfg(any(FiTv, icondata_include_all))]
    FiTv,
    #[cfg(any(FiTwitch, icondata_include_all))]
    FiTwitch,
    #[cfg(any(FiTwitter, icondata_include_all))]
    FiTwitter,
    #[cfg(any(FiType, icondata_include_all))]
    FiType,
    #[cfg(any(FiUmbrella, icondata_include_all))]
    FiUmbrella,
    #[cfg(any(FiUnderline, icondata_include_all))]
    FiUnderline,
    #[cfg(any(FiUnlock, icondata_include_all))]
    FiUnlock,
    #[cfg(any(FiUpload, icondata_include_all))]
    FiUpload,
    #[cfg(any(FiUploadCloud, icondata_include_all))]
    FiUploadCloud,
    #[cfg(any(FiUser, icondata_include_all))]
    FiUser,
    #[cfg(any(FiUserCheck, icondata_include_all))]
    FiUserCheck,
    #[cfg(any(FiUserMinus, icondata_include_all))]
    FiUserMinus,
    #[cfg(any(FiUserPlus, icondata_include_all))]
    FiUserPlus,
    #[cfg(any(FiUserX, icondata_include_all))]
    FiUserX,
    #[cfg(any(FiUsers, icondata_include_all))]
    FiUsers,
    #[cfg(any(FiVideo, icondata_include_all))]
    FiVideo,
    #[cfg(any(FiVideoOff, icondata_include_all))]
    FiVideoOff,
    #[cfg(any(FiVoicemail, icondata_include_all))]
    FiVoicemail,
    #[cfg(any(FiVolume, icondata_include_all))]
    FiVolume,
    #[cfg(any(FiVolume1, icondata_include_all))]
    FiVolume1,
    #[cfg(any(FiVolume2, icondata_include_all))]
    FiVolume2,
    #[cfg(any(FiVolumeX, icondata_include_all))]
    FiVolumeX,
    #[cfg(any(FiWatch, icondata_include_all))]
    FiWatch,
    #[cfg(any(FiWifi, icondata_include_all))]
    FiWifi,
    #[cfg(any(FiWifiOff, icondata_include_all))]
    FiWifiOff,
    #[cfg(any(FiWind, icondata_include_all))]
    FiWind,
    #[cfg(any(FiX, icondata_include_all))]
    FiX,
    #[cfg(any(FiXCircle, icondata_include_all))]
    FiXCircle,
    #[cfg(any(FiXOctagon, icondata_include_all))]
    FiXOctagon,
    #[cfg(any(FiXSquare, icondata_include_all))]
    FiXSquare,
    #[cfg(any(FiYoutube, icondata_include_all))]
    FiYoutube,
    #[cfg(any(FiZap, icondata_include_all))]
    FiZap,
    #[cfg(any(FiZapOff, icondata_include_all))]
    FiZapOff,
    #[cfg(any(FiZoomIn, icondata_include_all))]
    FiZoomIn,
    #[cfg(any(FiZoomOut, icondata_include_all))]
    FiZoomOut,
}

#[cfg(any(FiActivity, icondata_include_all))]
const FI_ACTIVITY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="22 12 18 12 15 21 9 3 6 12 2 12" />"###
};
#[cfg(any(FiAirplay, icondata_include_all))]
const FI_AIRPLAY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 17H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-1" />
<polygon points="12 15 17 21 7 21 12 15" />"###
};
#[cfg(any(FiAlertCircle, icondata_include_all))]
const FI_ALERT_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />
<line x1="12" y1="8" x2="12" y2="12" />
<line x1="12" y1="16" x2="12.01" y2="16" />"###
};
#[cfg(any(FiAlertOctagon, icondata_include_all))]
const FI_ALERT_OCTAGON: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2" />
<line x1="12" y1="8" x2="12" y2="12" />
<line x1="12" y1="16" x2="12.01" y2="16" />"###
};
#[cfg(any(FiAlertTriangle, icondata_include_all))]
const FI_ALERT_TRIANGLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" />
<line x1="12" y1="9" x2="12" y2="13" />
<line x1="12" y1="17" x2="12.01" y2="17" />"###
};
#[cfg(any(FiAlignCenter, icondata_include_all))]
const FI_ALIGN_CENTER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="18" y1="10" x2="6" y2="10" />
<line x1="21" y1="6" x2="3" y2="6" />
<line x1="21" y1="14" x2="3" y2="14" />
<line x1="18" y1="18" x2="6" y2="18" />"###
};
#[cfg(any(FiAlignJustify, icondata_include_all))]
const FI_ALIGN_JUSTIFY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="21" y1="10" x2="3" y2="10" />
<line x1="21" y1="6" x2="3" y2="6" />
<line x1="21" y1="14" x2="3" y2="14" />
<line x1="21" y1="18" x2="3" y2="18" />"###
};
#[cfg(any(FiAlignLeft, icondata_include_all))]
const FI_ALIGN_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="17" y1="10" x2="3" y2="10" />
<line x1="21" y1="6" x2="3" y2="6" />
<line x1="21" y1="14" x2="3" y2="14" />
<line x1="17" y1="18" x2="3" y2="18" />"###
};
#[cfg(any(FiAlignRight, icondata_include_all))]
const FI_ALIGN_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="21" y1="10" x2="7" y2="10" />
<line x1="21" y1="6" x2="3" y2="6" />
<line x1="21" y1="14" x2="3" y2="14" />
<line x1="21" y1="18" x2="7" y2="18" />"###
};
#[cfg(any(FiAnchor, icondata_include_all))]
const FI_ANCHOR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="5" r="3" />
<line x1="12" y1="22" x2="12" y2="8" />
<path d="M5 12H2a10 10 0 0 0 20 0h-3" />"###
};
#[cfg(any(FiAperture, icondata_include_all))]
const FI_APERTURE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />
<line x1="14.31" y1="8" x2="20.05" y2="17.94" />
<line x1="9.69" y1="8" x2="21.17" y2="8" />
<line x1="7.38" y1="12" x2="13.12" y2="2.06" />
<line x1="9.69" y1="16" x2="3.95" y2="6.06" />
<line x1="14.31" y1="16" x2="2.83" y2="16" />
<line x1="16.62" y1="12" x2="10.88" y2="21.94" />"###
};
#[cfg(any(FiArchive, icondata_include_all))]
const FI_ARCHIVE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="21 8 21 21 3 21 3 8" />
<rect x="1" y="3" width="22" height="5" />
<line x1="10" y1="12" x2="14" y2="12" />"###
};
#[cfg(any(FiArrowDown, icondata_include_all))]
const FI_ARROW_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="12" y1="5" x2="12" y2="19" />
<polyline points="19 12 12 19 5 12" />"###
};
#[cfg(any(FiArrowDownCircle, icondata_include_all))]
const FI_ARROW_DOWN_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />
<polyline points="8 12 12 16 16 12" />
<line x1="12" y1="8" x2="12" y2="16" />"###
};
#[cfg(any(FiArrowDownLeft, icondata_include_all))]
const FI_ARROW_DOWN_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="17" y1="7" x2="7" y2="17" />
<polyline points="17 17 7 17 7 7" />"###
};
#[cfg(any(FiArrowDownRight, icondata_include_all))]
const FI_ARROW_DOWN_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="7" y1="7" x2="17" y2="17" />
<polyline points="17 7 17 17 7 17" />"###
};
#[cfg(any(FiArrowLeft, icondata_include_all))]
const FI_ARROW_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="19" y1="12" x2="5" y2="12" />
<polyline points="12 19 5 12 12 5" />"###
};
#[cfg(any(FiArrowLeftCircle, icondata_include_all))]
const FI_ARROW_LEFT_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />
<polyline points="12 8 8 12 12 16" />
<line x1="16" y1="12" x2="8" y2="12" />"###
};
#[cfg(any(FiArrowRight, icondata_include_all))]
const FI_ARROW_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="5" y1="12" x2="19" y2="12" />
<polyline points="12 5 19 12 12 19" />"###
};
#[cfg(any(FiArrowRightCircle, icondata_include_all))]
const FI_ARROW_RIGHT_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />
<polyline points="12 16 16 12 12 8" />
<line x1="8" y1="12" x2="16" y2="12" />"###
};
#[cfg(any(FiArrowUp, icondata_include_all))]
const FI_ARROW_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="12" y1="19" x2="12" y2="5" />
<polyline points="5 12 12 5 19 12" />"###
};
#[cfg(any(FiArrowUpCircle, icondata_include_all))]
const FI_ARROW_UP_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />
<polyline points="16 12 12 8 8 12" />
<line x1="12" y1="16" x2="12" y2="8" />"###
};
#[cfg(any(FiArrowUpLeft, icondata_include_all))]
const FI_ARROW_UP_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="17" y1="17" x2="7" y2="7" />
<polyline points="7 17 7 7 17 7" />"###
};
#[cfg(any(FiArrowUpRight, icondata_include_all))]
const FI_ARROW_UP_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="7" y1="17" x2="17" y2="7" />
<polyline points="7 7 17 7 17 17" />"###
};
#[cfg(any(FiAtSign, icondata_include_all))]
const FI_AT_SIGN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="4" />
<path d="M16 8v5a3 3 0 0 0 6 0v-1a10 10 0 1 0-3.92 7.94" />"###
};
#[cfg(any(FiAward, icondata_include_all))]
const FI_AWARD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="8" r="7" />
<polyline points="8.21 13.89 7 23 12 20 17 23 15.79 13.88" />"###
};
#[cfg(any(FiBarChart, icondata_include_all))]
const FI_BAR_CHART: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="12" y1="20" x2="12" y2="10" />
<line x1="18" y1="20" x2="18" y2="4" />
<line x1="6" y1="20" x2="6" y2="16" />"###
};
#[cfg(any(FiBarChart2, icondata_include_all))]
const FI_BAR_CHART2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="18" y1="20" x2="18" y2="10" />
<line x1="12" y1="20" x2="12" y2="4" />
<line x1="6" y1="20" x2="6" y2="14" />"###
};
#[cfg(any(FiBattery, icondata_include_all))]
const FI_BATTERY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="1" y="6" width="18" height="12" rx="2" ry="2" />
<line x1="23" y1="13" x2="23" y2="11" />"###
};
#[cfg(any(FiBatteryCharging, icondata_include_all))]
const FI_BATTERY_CHARGING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 18H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h3.19M15 6h2a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-3.19" />
<line x1="23" y1="13" x2="23" y2="11" />
<polyline points="11 6 7 12 13 12 9 18" />"###
};
#[cfg(any(FiBell, icondata_include_all))]
const FI_BELL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9" />
<path d="M13.73 21a2 2 0 0 1-3.46 0" />"###
};
#[cfg(any(FiBellOff, icondata_include_all))]
const FI_BELL_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M13.73 21a2 2 0 0 1-3.46 0" />
<path d="M18.63 13A17.89 17.89 0 0 1 18 8" />
<path d="M6.26 6.26A5.86 5.86 0 0 0 6 8c0 7-3 9-3 9h14" />
<path d="M18 8a6 6 0 0 0-9.33-5" />
<line x1="1" y1="1" x2="23" y2="23" />"###
};
#[cfg(any(FiBluetooth, icondata_include_all))]
const FI_BLUETOOTH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="6.5 6.5 17.5 17.5 12 23 12 1 17.5 6.5 6.5 17.5" />"###
};
#[cfg(any(FiBold, icondata_include_all))]
const FI_BOLD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 4h8a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z" />
<path d="M6 12h9a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z" />"###
};
#[cfg(any(FiBook, icondata_include_all))]
const FI_BOOK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20" />
<path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z" />"###
};
#[cfg(any(FiBookOpen, icondata_include_all))]
const FI_BOOK_OPEN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z" />
<path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z" />"###
};
#[cfg(any(FiBookmark, icondata_include_all))]
const FI_BOOKMARK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z" />"###
};
#[cfg(any(FiBox, icondata_include_all))]
const FI_BOX: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" />
<polyline points="3.27 6.96 12 12.01 20.73 6.96" />
<line x1="12" y1="22.08" x2="12" y2="12" />"###
};
#[cfg(any(FiBriefcase, icondata_include_all))]
const FI_BRIEFCASE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="2" y="7" width="20" height="14" rx="2" ry="2" />
<path d="M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16" />"###
};
#[cfg(any(FiCalendar, icondata_include_all))]
const FI_CALENDAR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="3" y="4" width="18" height="18" rx="2" ry="2" />
<line x1="16" y1="2" x2="16" y2="6" />
<line x1="8" y1="2" x2="8" y2="6" />
<line x1="3" y1="10" x2="21" y2="10" />"###
};
#[cfg(any(FiCamera, icondata_include_all))]
const FI_CAMERA: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z" />
<circle cx="12" cy="13" r="4" />"###
};
#[cfg(any(FiCameraOff, icondata_include_all))]
const FI_CAMERA_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="1" y1="1" x2="23" y2="23" />
<path d="M21 21H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h3m3-3h6l2 3h4a2 2 0 0 1 2 2v9.34m-7.72-2.06a4 4 0 1 1-5.56-5.56" />"###
};
#[cfg(any(FiCast, icondata_include_all))]
const FI_CAST: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 16.1A5 5 0 0 1 5.9 20M2 12.05A9 9 0 0 1 9.95 20M2 8V6a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2h-6" />
<line x1="2" y1="20" x2="2.01" y2="20" />"###
};
#[cfg(any(FiCheck, icondata_include_all))]
const FI_CHECK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="20 6 9 17 4 12" />"###
};
#[cfg(any(FiCheckCircle, icondata_include_all))]
const FI_CHECK_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
<polyline points="22 4 12 14.01 9 11.01" />"###
};
#[cfg(any(FiCheckSquare, icondata_include_all))]
const FI_CHECK_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="9 11 12 14 22 4" />
<path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11" />"###
};
#[cfg(any(FiChevronDown, icondata_include_all))]
const FI_CHEVRON_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="6 9 12 15 18 9" />"###
};
#[cfg(any(FiChevronLeft, icondata_include_all))]
const FI_CHEVRON_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="15 18 9 12 15 6" />"###
};
#[cfg(any(FiChevronRight, icondata_include_all))]
const FI_CHEVRON_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="9 18 15 12 9 6" />"###
};
#[cfg(any(FiChevronUp, icondata_include_all))]
const FI_CHEVRON_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="18 15 12 9 6 15" />"###
};
#[cfg(any(FiChevronsDown, icondata_include_all))]
const FI_CHEVRONS_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="7 13 12 18 17 13" />
<polyline points="7 6 12 11 17 6" />"###
};
#[cfg(any(FiChevronsLeft, icondata_include_all))]
const FI_CHEVRONS_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="11 17 6 12 11 7" />
<polyline points="18 17 13 12 18 7" />"###
};
#[cfg(any(FiChevronsRight, icondata_include_all))]
const FI_CHEVRONS_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="13 17 18 12 13 7" />
<polyline points="6 17 11 12 6 7" />"###
};
#[cfg(any(FiChevronsUp, icondata_include_all))]
const FI_CHEVRONS_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="17 11 12 6 7 11" />
<polyline points="17 18 12 13 7 18" />"###
};
#[cfg(any(FiChrome, icondata_include_all))]
const FI_CHROME: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />
<circle cx="12" cy="12" r="4" />
<line x1="21.17" y1="8" x2="12" y2="8" />
<line x1="3.95" y1="6.06" x2="8.54" y2="14" />
<line x1="10.88" y1="21.94" x2="15.46" y2="14" />"###
};
#[cfg(any(FiCircle, icondata_include_all))]
const FI_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />"###
};
#[cfg(any(FiClipboard, icondata_include_all))]
const FI_CLIPBOARD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2" />
<rect x="8" y="2" width="8" height="4" rx="1" ry="1" />"###
};
#[cfg(any(FiClock, icondata_include_all))]
const FI_CLOCK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />
<polyline points="12 6 12 12 16 14" />"###
};
#[cfg(any(FiCloud, icondata_include_all))]
const FI_CLOUD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18 10h-1.26A8 8 0 1 0 9 20h9a5 5 0 0 0 0-10z" />"###
};
#[cfg(any(FiCloudDrizzle, icondata_include_all))]
const FI_CLOUD_DRIZZLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="8" y1="19" x2="8" y2="21" />
<line x1="8" y1="13" x2="8" y2="15" />
<line x1="16" y1="19" x2="16" y2="21" />
<line x1="16" y1="13" x2="16" y2="15" />
<line x1="12" y1="21" x2="12" y2="23" />
<line x1="12" y1="15" x2="12" y2="17" />
<path d="M20 16.58A5 5 0 0 0 18 7h-1.26A8 8 0 1 0 4 15.25" />"###
};
#[cfg(any(FiCloudLightning, icondata_include_all))]
const FI_CLOUD_LIGHTNING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M19 16.9A5 5 0 0 0 18 7h-1.26a8 8 0 1 0-11.62 9" />
<polyline points="13 11 9 17 15 17 11 23" />"###
};
#[cfg(any(FiCloudOff, icondata_include_all))]
const FI_CLOUD_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22.61 16.95A5 5 0 0 0 18 10h-1.26a8 8 0 0 0-7.05-6M5 5a8 8 0 0 0 4 15h9a5 5 0 0 0 1.7-.3" />
<line x1="1" y1="1" x2="23" y2="23" />"###
};
#[cfg(any(FiCloudRain, icondata_include_all))]
const FI_CLOUD_RAIN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="16" y1="13" x2="16" y2="21" />
<line x1="8" y1="13" x2="8" y2="21" />
<line x1="12" y1="15" x2="12" y2="23" />
<path d="M20 16.58A5 5 0 0 0 18 7h-1.26A8 8 0 1 0 4 15.25" />"###
};
#[cfg(any(FiCloudSnow, icondata_include_all))]
const FI_CLOUD_SNOW: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M20 17.58A5 5 0 0 0 18 8h-1.26A8 8 0 1 0 4 16.25" />
<line x1="8" y1="16" x2="8.01" y2="16" />
<line x1="8" y1="20" x2="8.01" y2="20" />
<line x1="12" y1="18" x2="12.01" y2="18" />
<line x1="12" y1="22" x2="12.01" y2="22" />
<line x1="16" y1="16" x2="16.01" y2="16" />
<line x1="16" y1="20" x2="16.01" y2="20" />"###
};
#[cfg(any(FiCode, icondata_include_all))]
const FI_CODE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="16 18 22 12 16 6" />
<polyline points="8 6 2 12 8 18" />"###
};
#[cfg(any(FiCodepen, icondata_include_all))]
const FI_CODEPEN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="12 2 22 8.5 22 15.5 12 22 2 15.5 2 8.5 12 2" />
<line x1="12" y1="22" x2="12" y2="15.5" />
<polyline points="22 8.5 12 15.5 2 8.5" />
<polyline points="2 15.5 12 8.5 22 15.5" />
<line x1="12" y1="2" x2="12" y2="8.5" />"###
};
#[cfg(any(FiCodesandbox, icondata_include_all))]
const FI_CODESANDBOX: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" />
<polyline points="7.5 4.21 12 6.81 16.5 4.21" />
<polyline points="7.5 19.79 7.5 14.6 3 12" />
<polyline points="21 12 16.5 14.6 16.5 19.79" />
<polyline points="3.27 6.96 12 12.01 20.73 6.96" />
<line x1="12" y1="22.08" x2="12" y2="12" />"###
};
#[cfg(any(FiCoffee, icondata_include_all))]
const FI_COFFEE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18 8h1a4 4 0 0 1 0 8h-1" />
<path d="M2 8h16v9a4 4 0 0 1-4 4H6a4 4 0 0 1-4-4V8z" />
<line x1="6" y1="1" x2="6" y2="4" />
<line x1="10" y1="1" x2="10" y2="4" />
<line x1="14" y1="1" x2="14" y2="4" />"###
};
#[cfg(any(FiColumns, icondata_include_all))]
const FI_COLUMNS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 3h7a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-7m0-18H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h7m0-18v18" />"###
};
#[cfg(any(FiCommand, icondata_include_all))]
const FI_COMMAND: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18 3a3 3 0 0 0-3 3v12a3 3 0 0 0 3 3 3 3 0 0 0 3-3 3 3 0 0 0-3-3H6a3 3 0 0 0-3 3 3 3 0 0 0 3 3 3 3 0 0 0 3-3V6a3 3 0 0 0-3-3 3 3 0 0 0-3 3 3 3 0 0 0 3 3h12a3 3 0 0 0 3-3 3 3 0 0 0-3-3z" />"###
};
#[cfg(any(FiCompass, icondata_include_all))]
const FI_COMPASS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />
<polygon points="16.24 7.76 14.12 14.12 7.76 16.24 9.88 9.88 16.24 7.76" />"###
};
#[cfg(any(FiCopy, icondata_include_all))]
const FI_COPY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
<path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />"###
};
#[cfg(any(FiCornerDownLeft, icondata_include_all))]
const FI_CORNER_DOWN_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="9 10 4 15 9 20" />
<path d="M20 4v7a4 4 0 0 1-4 4H4" />"###
};
#[cfg(any(FiCornerDownRight, icondata_include_all))]
const FI_CORNER_DOWN_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="15 10 20 15 15 20" />
<path d="M4 4v7a4 4 0 0 0 4 4h12" />"###
};
#[cfg(any(FiCornerLeftDown, icondata_include_all))]
const FI_CORNER_LEFT_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="14 15 9 20 4 15" />
<path d="M20 4h-7a4 4 0 0 0-4 4v12" />"###
};
#[cfg(any(FiCornerLeftUp, icondata_include_all))]
const FI_CORNER_LEFT_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="14 9 9 4 4 9" />
<path d="M20 20h-7a4 4 0 0 1-4-4V4" />"###
};
#[cfg(any(FiCornerRightDown, icondata_include_all))]
const FI_CORNER_RIGHT_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="10 15 15 20 20 15" />
<path d="M4 4h7a4 4 0 0 1 4 4v12" />"###
};
#[cfg(any(FiCornerRightUp, icondata_include_all))]
const FI_CORNER_RIGHT_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="10 9 15 4 20 9" />
<path d="M4 20h7a4 4 0 0 0 4-4V4" />"###
};
#[cfg(any(FiCornerUpLeft, icondata_include_all))]
const FI_CORNER_UP_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="9 14 4 9 9 4" />
<path d="M20 20v-7a4 4 0 0 0-4-4H4" />"###
};
#[cfg(any(FiCornerUpRight, icondata_include_all))]
const FI_CORNER_UP_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="15 14 20 9 15 4" />
<path d="M4 20v-7a4 4 0 0 1 4-4h12" />"###
};
#[cfg(any(FiCpu, icondata_include_all))]
const FI_CPU: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="4" y="4" width="16" height="16" rx="2" ry="2" />
<rect x="9" y="9" width="6" height="6" />
<line x1="9" y1="1" x2="9" y2="4" />
<line x1="15" y1="1" x2="15" y2="4" />
<line x1="9" y1="20" x2="9" y2="23" />
<line x1="15" y1="20" x2="15" y2="23" />
<line x1="20" y1="9" x2="23" y2="9" />
<line x1="20" y1="14" x2="23" y2="14" />
<line x1="1" y1="9" x2="4" y2="9" />
<line x1="1" y1="14" x2="4" y2="14" />"###
};
#[cfg(any(FiCreditCard, icondata_include_all))]
const FI_CREDIT_CARD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="1" y="4" width="22" height="16" rx="2" ry="2" />
<line x1="1" y1="10" x2="23" y2="10" />"###
};
#[cfg(any(FiCrop, icondata_include_all))]
const FI_CROP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6.13 1L6 16a2 2 0 0 0 2 2h15" />
<path d="M1 6.13L16 6a2 2 0 0 1 2 2v15" />"###
};
#[cfg(any(FiCrosshair, icondata_include_all))]
const FI_CROSSHAIR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />
<line x1="22" y1="12" x2="18" y2="12" />
<line x1="6" y1="12" x2="2" y2="12" />
<line x1="12" y1="6" x2="12" y2="2" />
<line x1="12" y1="22" x2="12" y2="18" />"###
};
#[cfg(any(FiDatabase, icondata_include_all))]
const FI_DATABASE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<ellipse cx="12" cy="5" rx="9" ry="3" />
<path d="M21 12c0 1.66-4 3-9 3s-9-1.34-9-3" />
<path d="M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5" />"###
};
#[cfg(any(FiDelete, icondata_include_all))]
const FI_DELETE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 4H8l-7 8 7 8h13a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2z" />
<line x1="18" y1="9" x2="12" y2="15" />
<line x1="12" y1="9" x2="18" y2="15" />"###
};
#[cfg(any(FiDisc, icondata_include_all))]
const FI_DISC: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />
<circle cx="12" cy="12" r="3" />"###
};
#[cfg(any(FiDivide, icondata_include_all))]
const FI_DIVIDE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="6" r="2" />
<line x1="5" y1="12" x2="19" y2="12" />
<circle cx="12" cy="18" r="2" />"###
};
#[cfg(any(FiDivideCircle, icondata_include_all))]
const FI_DIVIDE_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="8" y1="12" x2="16" y2="12" />
<line x1="12" y1="16" x2="12" y2="16" />
<line x1="12" y1="8" x2="12" y2="8" />
<circle cx="12" cy="12" r="10" />"###
};
#[cfg(any(FiDivideSquare, icondata_include_all))]
const FI_DIVIDE_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
<line x1="8" y1="12" x2="16" y2="12" />
<line x1="12" y1="16" x2="12" y2="16" />
<line x1="12" y1="8" x2="12" y2="8" />"###
};
#[cfg(any(FiDollarSign, icondata_include_all))]
const FI_DOLLAR_SIGN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="12" y1="1" x2="12" y2="23" />
<path d="M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6" />"###
};
#[cfg(any(FiDownload, icondata_include_all))]
const FI_DOWNLOAD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
<polyline points="7 10 12 15 17 10" />
<line x1="12" y1="15" x2="12" y2="3" />"###
};
#[cfg(any(FiDownloadCloud, icondata_include_all))]
const FI_DOWNLOAD_CLOUD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="8 17 12 21 16 17" />
<line x1="12" y1="12" x2="12" y2="21" />
<path d="M20.88 18.09A5 5 0 0 0 18 9h-1.26A8 8 0 1 0 3 16.29" />"###
};
#[cfg(any(FiDribbble, icondata_include_all))]
const FI_DRIBBBLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />
<path d="M8.56 2.75c4.37 6.03 6.02 9.42 8.03 17.72m2.54-15.38c-3.72 4.35-8.94 5.66-16.88 5.85m19.5 1.9c-3.5-.93-6.63-.82-8.94 0-2.58.92-5.01 2.86-7.44 6.32" />"###
};
#[cfg(any(FiDroplet, icondata_include_all))]
const FI_DROPLET: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 2.69l5.66 5.66a8 8 0 1 1-11.31 0z" />"###
};
#[cfg(any(FiEdit, icondata_include_all))]
const FI_EDIT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" />
<path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z" />"###
};
#[cfg(any(FiEdit2, icondata_include_all))]
const FI_EDIT2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17 3a2.828 2.828 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z" />"###
};
#[cfg(any(FiEdit3, icondata_include_all))]
const FI_EDIT3: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 20h9" />
<path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z" />"###
};
#[cfg(any(FiExternalLink, icondata_include_all))]
const FI_EXTERNAL_LINK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
<polyline points="15 3 21 3 21 9" />
<line x1="10" y1="14" x2="21" y2="3" />"###
};
#[cfg(any(FiEye, icondata_include_all))]
const FI_EYE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" />
<circle cx="12" cy="12" r="3" />"###
};
#[cfg(any(FiEyeOff, icondata_include_all))]
const FI_EYE_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24" />
<line x1="1" y1="1" x2="23" y2="23" />"###
};
#[cfg(any(FiFacebook, icondata_include_all))]
const FI_FACEBOOK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18 2h-3a5 5 0 0 0-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 0 1 1-1h3z" />"###
};
#[cfg(any(FiFastForward, icondata_include_all))]
const FI_FAST_FORWARD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="13 19 22 12 13 5 13 19" />
<polygon points="2 19 11 12 2 5 2 19" />"###
};
#[cfg(any(FiFeather, icondata_include_all))]
const FI_FEATHER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M20.24 12.24a6 6 0 0 0-8.49-8.49L5 10.5V19h8.5z" />
<line x1="16" y1="8" x2="2" y2="22" />
<line x1="17.5" y1="15" x2="9" y2="15" />"###
};
#[cfg(any(FiFigma, icondata_include_all))]
const FI_FIGMA: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 5.5A3.5 3.5 0 0 1 8.5 2H12v7H8.5A3.5 3.5 0 0 1 5 5.5z" />
<path d="M12 2h3.5a3.5 3.5 0 1 1 0 7H12V2z" />
<path d="M12 12.5a3.5 3.5 0 1 1 7 0 3.5 3.5 0 1 1-7 0z" />
<path d="M5 19.5A3.5 3.5 0 0 1 8.5 16H12v3.5a3.5 3.5 0 1 1-7 0z" />
<path d="M5 12.5A3.5 3.5 0 0 1 8.5 9H12v7H8.5A3.5 3.5 0 0 1 5 12.5z" />"###
};
#[cfg(any(FiFile, icondata_include_all))]
const FI_FILE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z" />
<polyline points="13 2 13 9 20 9" />"###
};
#[cfg(any(FiFileMinus, icondata_include_all))]
const FI_FILE_MINUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
<polyline points="14 2 14 8 20 8" />
<line x1="9" y1="15" x2="15" y2="15" />"###
};
#[cfg(any(FiFilePlus, icondata_include_all))]
const FI_FILE_PLUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
<polyline points="14 2 14 8 20 8" />
<line x1="12" y1="18" x2="12" y2="12" />
<line x1="9" y1="15" x2="15" y2="15" />"###
};
#[cfg(any(FiFileText, icondata_include_all))]
const FI_FILE_TEXT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
<polyline points="14 2 14 8 20 8" />
<line x1="16" y1="13" x2="8" y2="13" />
<line x1="16" y1="17" x2="8" y2="17" />
<polyline points="10 9 9 9 8 9" />"###
};
#[cfg(any(FiFilm, icondata_include_all))]
const FI_FILM: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="2" y="2" width="20" height="20" rx="2.18" ry="2.18" />
<line x1="7" y1="2" x2="7" y2="22" />
<line x1="17" y1="2" x2="17" y2="22" />
<line x1="2" y1="12" x2="22" y2="12" />
<line x1="2" y1="7" x2="7" y2="7" />
<line x1="2" y1="17" x2="7" y2="17" />
<line x1="17" y1="17" x2="22" y2="17" />
<line x1="17" y1="7" x2="22" y2="7" />"###
};
#[cfg(any(FiFilter, icondata_include_all))]
const FI_FILTER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3" />"###
};
#[cfg(any(FiFlag, icondata_include_all))]
const FI_FLAG: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 15s1-1 4-1 5 2 8 2 4-1 4-1V3s-1 1-4 1-5-2-8-2-4 1-4 1z" />
<line x1="4" y1="22" x2="4" y2="15" />"###
};
#[cfg(any(FiFolder, icondata_include_all))]
const FI_FOLDER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" />"###
};
#[cfg(any(FiFolderMinus, icondata_include_all))]
const FI_FOLDER_MINUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" />
<line x1="9" y1="14" x2="15" y2="14" />"###
};
#[cfg(any(FiFolderPlus, icondata_include_all))]
const FI_FOLDER_PLUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" />
<line x1="12" y1="11" x2="12" y2="17" />
<line x1="9" y1="14" x2="15" y2="14" />"###
};
#[cfg(any(FiFramer, icondata_include_all))]
const FI_FRAMER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 16V9h14V2H5l14 14h-7m-7 0l7 7v-7m-7 0h7" />"###
};
#[cfg(any(FiFrown, icondata_include_all))]
const FI_FROWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />
<path d="M16 16s-1.5-2-4-2-4 2-4 2" />
<line x1="9" y1="9" x2="9.01" y2="9" />
<line x1="15" y1="9" x2="15.01" y2="9" />"###
};
#[cfg(any(FiGift, icondata_include_all))]
const FI_GIFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="20 12 20 22 4 22 4 12" />
<rect x="2" y="7" width="20" height="5" />
<line x1="12" y1="22" x2="12" y2="7" />
<path d="M12 7H7.5a2.5 2.5 0 0 1 0-5C11 2 12 7 12 7z" />
<path d="M12 7h4.5a2.5 2.5 0 0 0 0-5C13 2 12 7 12 7z" />"###
};
#[cfg(any(FiGitBranch, icondata_include_all))]
const FI_GIT_BRANCH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="6" y1="3" x2="6" y2="15" />
<circle cx="18" cy="6" r="3" />
<circle cx="6" cy="18" r="3" />
<path d="M18 9a9 9 0 0 1-9 9" />"###
};
#[cfg(any(FiGitCommit, icondata_include_all))]
const FI_GIT_COMMIT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="4" />
<line x1="1.05" y1="12" x2="7" y2="12" />
<line x1="17.01" y1="12" x2="22.96" y2="12" />"###
};
#[cfg(any(FiGitMerge, icondata_include_all))]
const FI_GIT_MERGE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="18" cy="18" r="3" />
<circle cx="6" cy="6" r="3" />
<path d="M6 21V9a9 9 0 0 0 9 9" />"###
};
#[cfg(any(FiGitPullRequest, icondata_include_all))]
const FI_GIT_PULL_REQUEST: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="18" cy="18" r="3" />
<circle cx="6" cy="6" r="3" />
<path d="M13 6h3a2 2 0 0 1 2 2v7" />
<line x1="6" y1="9" x2="6" y2="21" />"###
};
#[cfg(any(FiGithub, icondata_include_all))]
const FI_GITHUB: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22" />"###
};
#[cfg(any(FiGitlab, icondata_include_all))]
const FI_GITLAB: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22.65 14.39L12 22.13 1.35 14.39a.84.84 0 0 1-.3-.94l1.22-3.78 2.44-7.51A.42.42 0 0 1 4.82 2a.43.43 0 0 1 .58 0 .42.42 0 0 1 .11.18l2.44 7.49h8.1l2.44-7.51A.42.42 0 0 1 18.6 2a.43.43 0 0 1 .58 0 .42.42 0 0 1 .11.18l2.44 7.51L23 13.45a.84.84 0 0 1-.35.94z" />"###
};
#[cfg(any(FiGlobe, icondata_include_all))]
const FI_GLOBE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />
<line x1="2" y1="12" x2="22" y2="12" />
<path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z" />"###
};
#[cfg(any(FiGrid, icondata_include_all))]
const FI_GRID: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="3" y="3" width="7" height="7" />
<rect x="14" y="3" width="7" height="7" />
<rect x="14" y="14" width="7" height="7" />
<rect x="3" y="14" width="7" height="7" />"###
};
#[cfg(any(FiHardDrive, icondata_include_all))]
const FI_HARD_DRIVE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="22" y1="12" x2="2" y2="12" />
<path d="M5.45 5.11L2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z" />
<line x1="6" y1="16" x2="6.01" y2="16" />
<line x1="10" y1="16" x2="10.01" y2="16" />"###
};
#[cfg(any(FiHash, icondata_include_all))]
const FI_HASH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="4" y1="9" x2="20" y2="9" />
<line x1="4" y1="15" x2="20" y2="15" />
<line x1="10" y1="3" x2="8" y2="21" />
<line x1="16" y1="3" x2="14" y2="21" />"###
};
#[cfg(any(FiHeadphones, icondata_include_all))]
const FI_HEADPHONES: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 18v-6a9 9 0 0 1 18 0v6" />
<path d="M21 19a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3zM3 19a2 2 0 0 0 2 2h1a2 2 0 0 0 2-2v-3a2 2 0 0 0-2-2H3z" />"###
};
#[cfg(any(FiHeart, icondata_include_all))]
const FI_HEART: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" />"###
};
#[cfg(any(FiHelpCircle, icondata_include_all))]
const FI_HELP_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />
<path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" />
<line x1="12" y1="17" x2="12.01" y2="17" />"###
};
#[cfg(any(FiHexagon, icondata_include_all))]
const FI_HEXAGON: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" />"###
};
#[cfg(any(FiHome, icondata_include_all))]
const FI_HOME: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" />
<polyline points="9 22 9 12 15 12 15 22" />"###
};
#[cfg(any(FiImage, icondata_include_all))]
const FI_IMAGE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
<circle cx="8.5" cy="8.5" r="1.5" />
<polyline points="21 15 16 10 5 21" />"###
};
#[cfg(any(FiInbox, icondata_include_all))]
const FI_INBOX: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="22 12 16 12 14 15 10 15 8 12 2 12" />
<path d="M5.45 5.11L2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z" />"###
};
#[cfg(any(FiInfo, icondata_include_all))]
const FI_INFO: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />
<line x1="12" y1="16" x2="12" y2="12" />
<line x1="12" y1="8" x2="12.01" y2="8" />"###
};
#[cfg(any(FiInstagram, icondata_include_all))]
const FI_INSTAGRAM: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="2" y="2" width="20" height="20" rx="5" ry="5" />
<path d="M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z" />
<line x1="17.5" y1="6.5" x2="17.51" y2="6.5" />"###
};
#[cfg(any(FiItalic, icondata_include_all))]
const FI_ITALIC: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="19" y1="4" x2="10" y2="4" />
<line x1="14" y1="20" x2="5" y2="20" />
<line x1="15" y1="4" x2="9" y2="20" />"###
};
#[cfg(any(FiKey, icondata_include_all))]
const FI_KEY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 2l-2 2m-7.61 7.61a5.5 5.5 0 1 1-7.778 7.778 5.5 5.5 0 0 1 7.777-7.777zm0 0L15.5 7.5m0 0l3 3L22 7l-3-3m-3.5 3.5L19 4" />"###
};
#[cfg(any(FiLayers, icondata_include_all))]
const FI_LAYERS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="12 2 2 7 12 12 22 7 12 2" />
<polyline points="2 17 12 22 22 17" />
<polyline points="2 12 12 17 22 12" />"###
};
#[cfg(any(FiLayout, icondata_include_all))]
const FI_LAYOUT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
<line x1="3" y1="9" x2="21" y2="9" />
<line x1="9" y1="21" x2="9" y2="9" />"###
};
#[cfg(any(FiLifeBuoy, icondata_include_all))]
const FI_LIFE_BUOY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />
<circle cx="12" cy="12" r="4" />
<line x1="4.93" y1="4.93" x2="9.17" y2="9.17" />
<line x1="14.83" y1="14.83" x2="19.07" y2="19.07" />
<line x1="14.83" y1="9.17" x2="19.07" y2="4.93" />
<line x1="14.83" y1="9.17" x2="18.36" y2="5.64" />
<line x1="4.93" y1="19.07" x2="9.17" y2="14.83" />"###
};
#[cfg(any(FiLink, icondata_include_all))]
const FI_LINK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" />
<path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" />"###
};
#[cfg(any(FiLink2, icondata_include_all))]
const FI_LINK2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M15 7h3a5 5 0 0 1 5 5 5 5 0 0 1-5 5h-3m-6 0H6a5 5 0 0 1-5-5 5 5 0 0 1 5-5h3" />
<line x1="8" y1="12" x2="16" y2="12" />"###
};
#[cfg(any(FiLinkedin, icondata_include_all))]
const FI_LINKEDIN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z" />
<rect x="2" y="9" width="4" height="12" />
<circle cx="4" cy="4" r="2" />"###
};
#[cfg(any(FiList, icondata_include_all))]
const FI_LIST: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="8" y1="6" x2="21" y2="6" />
<line x1="8" y1="12" x2="21" y2="12" />
<line x1="8" y1="18" x2="21" y2="18" />
<line x1="3" y1="6" x2="3.01" y2="6" />
<line x1="3" y1="12" x2="3.01" y2="12" />
<line x1="3" y1="18" x2="3.01" y2="18" />"###
};
#[cfg(any(FiLoader, icondata_include_all))]
const FI_LOADER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="12" y1="2" x2="12" y2="6" />
<line x1="12" y1="18" x2="12" y2="22" />
<line x1="4.93" y1="4.93" x2="7.76" y2="7.76" />
<line x1="16.24" y1="16.24" x2="19.07" y2="19.07" />
<line x1="2" y1="12" x2="6" y2="12" />
<line x1="18" y1="12" x2="22" y2="12" />
<line x1="4.93" y1="19.07" x2="7.76" y2="16.24" />
<line x1="16.24" y1="7.76" x2="19.07" y2="4.93" />"###
};
#[cfg(any(FiLock, icondata_include_all))]
const FI_LOCK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="3" y="11" width="18" height="11" rx="2" ry="2" />
<path d="M7 11V7a5 5 0 0 1 10 0v4" />"###
};
#[cfg(any(FiLogIn, icondata_include_all))]
const FI_LOG_IN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4" />
<polyline points="10 17 15 12 10 7" />
<line x1="15" y1="12" x2="3" y2="12" />"###
};
#[cfg(any(FiLogOut, icondata_include_all))]
const FI_LOG_OUT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" />
<polyline points="16 17 21 12 16 7" />
<line x1="21" y1="12" x2="9" y2="12" />"###
};
#[cfg(any(FiMail, icondata_include_all))]
const FI_MAIL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z" />
<polyline points="22,6 12,13 2,6" />"###
};
#[cfg(any(FiMap, icondata_include_all))]
const FI_MAP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="1 6 1 22 8 18 16 22 23 18 23 2 16 6 8 2 1 6" />
<line x1="8" y1="2" x2="8" y2="18" />
<line x1="16" y1="6" x2="16" y2="22" />"###
};
#[cfg(any(FiMapPin, icondata_include_all))]
const FI_MAP_PIN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z" />
<circle cx="12" cy="10" r="3" />"###
};
#[cfg(any(FiMaximize, icondata_include_all))]
const FI_MAXIMIZE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3" />"###
};
#[cfg(any(FiMaximize2, icondata_include_all))]
const FI_MAXIMIZE2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="15 3 21 3 21 9" />
<polyline points="9 21 3 21 3 15" />
<line x1="21" y1="3" x2="14" y2="10" />
<line x1="3" y1="21" x2="10" y2="14" />"###
};
#[cfg(any(FiMeh, icondata_include_all))]
const FI_MEH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />
<line x1="8" y1="15" x2="16" y2="15" />
<line x1="9" y1="9" x2="9.01" y2="9" />
<line x1="15" y1="9" x2="15.01" y2="9" />"###
};
#[cfg(any(FiMenu, icondata_include_all))]
const FI_MENU: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="3" y1="12" x2="21" y2="12" />
<line x1="3" y1="6" x2="21" y2="6" />
<line x1="3" y1="18" x2="21" y2="18" />"###
};
#[cfg(any(FiMessageCircle, icondata_include_all))]
const FI_MESSAGE_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z" />"###
};
#[cfg(any(FiMessageSquare, icondata_include_all))]
const FI_MESSAGE_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />"###
};
#[cfg(any(FiMic, icondata_include_all))]
const FI_MIC: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z" />
<path d="M19 10v2a7 7 0 0 1-14 0v-2" />
<line x1="12" y1="19" x2="12" y2="23" />
<line x1="8" y1="23" x2="16" y2="23" />"###
};
#[cfg(any(FiMicOff, icondata_include_all))]
const FI_MIC_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="1" y1="1" x2="23" y2="23" />
<path d="M9 9v3a3 3 0 0 0 5.12 2.12M15 9.34V4a3 3 0 0 0-5.94-.6" />
<path d="M17 16.95A7 7 0 0 1 5 12v-2m14 0v2a7 7 0 0 1-.11 1.23" />
<line x1="12" y1="19" x2="12" y2="23" />
<line x1="8" y1="23" x2="16" y2="23" />"###
};
#[cfg(any(FiMinimize, icondata_include_all))]
const FI_MINIMIZE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 3v3a2 2 0 0 1-2 2H3m18 0h-3a2 2 0 0 1-2-2V3m0 18v-3a2 2 0 0 1 2-2h3M3 16h3a2 2 0 0 1 2 2v3" />"###
};
#[cfg(any(FiMinimize2, icondata_include_all))]
const FI_MINIMIZE2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="4 14 10 14 10 20" />
<polyline points="20 10 14 10 14 4" />
<line x1="14" y1="10" x2="21" y2="3" />
<line x1="3" y1="21" x2="10" y2="14" />"###
};
#[cfg(any(FiMinus, icondata_include_all))]
const FI_MINUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="5" y1="12" x2="19" y2="12" />"###
};
#[cfg(any(FiMinusCircle, icondata_include_all))]
const FI_MINUS_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />
<line x1="8" y1="12" x2="16" y2="12" />"###
};
#[cfg(any(FiMinusSquare, icondata_include_all))]
const FI_MINUS_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
<line x1="8" y1="12" x2="16" y2="12" />"###
};
#[cfg(any(FiMonitor, icondata_include_all))]
const FI_MONITOR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="2" y="3" width="20" height="14" rx="2" ry="2" />
<line x1="8" y1="21" x2="16" y2="21" />
<line x1="12" y1="17" x2="12" y2="21" />"###
};
#[cfg(any(FiMoon, icondata_include_all))]
const FI_MOON: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" />"###
};
#[cfg(any(FiMoreHorizontal, icondata_include_all))]
const FI_MORE_HORIZONTAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="1" />
<circle cx="19" cy="12" r="1" />
<circle cx="5" cy="12" r="1" />"###
};
#[cfg(any(FiMoreVertical, icondata_include_all))]
const FI_MORE_VERTICAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="1" />
<circle cx="12" cy="5" r="1" />
<circle cx="12" cy="19" r="1" />"###
};
#[cfg(any(FiMousePointer, icondata_include_all))]
const FI_MOUSE_POINTER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 3l7.07 16.97 2.51-7.39 7.39-2.51L3 3z" />
<path d="M13 13l6 6" />"###
};
#[cfg(any(FiMove, icondata_include_all))]
const FI_MOVE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="5 9 2 12 5 15" />
<polyline points="9 5 12 2 15 5" />
<polyline points="15 19 12 22 9 19" />
<polyline points="19 9 22 12 19 15" />
<line x1="2" y1="12" x2="22" y2="12" />
<line x1="12" y1="2" x2="12" y2="22" />"###
};
#[cfg(any(FiMusic, icondata_include_all))]
const FI_MUSIC: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M9 18V5l12-2v13" />
<circle cx="6" cy="18" r="3" />
<circle cx="18" cy="16" r="3" />"###
};
#[cfg(any(FiNavigation, icondata_include_all))]
const FI_NAVIGATION: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="3 11 22 2 13 21 11 13 3 11" />"###
};
#[cfg(any(FiNavigation2, icondata_include_all))]
const FI_NAVIGATION2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="12 2 19 21 12 17 5 21 12 2" />"###
};
#[cfg(any(FiOctagon, icondata_include_all))]
const FI_OCTAGON: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2" />"###
};
#[cfg(any(FiPackage, icondata_include_all))]
const FI_PACKAGE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="16.5" y1="9.4" x2="7.5" y2="4.21" />
<path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" />
<polyline points="3.27 6.96 12 12.01 20.73 6.96" />
<line x1="12" y1="22.08" x2="12" y2="12" />"###
};
#[cfg(any(FiPaperclip, icondata_include_all))]
const FI_PAPERCLIP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48" />"###
};
#[cfg(any(FiPause, icondata_include_all))]
const FI_PAUSE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="6" y="4" width="4" height="16" />
<rect x="14" y="4" width="4" height="16" />"###
};
#[cfg(any(FiPauseCircle, icondata_include_all))]
const FI_PAUSE_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />
<line x1="10" y1="15" x2="10" y2="9" />
<line x1="14" y1="15" x2="14" y2="9" />"###
};
#[cfg(any(FiPenTool, icondata_include_all))]
const FI_PEN_TOOL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 19l7-7 3 3-7 7-3-3z" />
<path d="M18 13l-1.5-7.5L2 2l3.5 14.5L13 18l5-5z" />
<path d="M2 2l7.586 7.586" />
<circle cx="11" cy="11" r="2" />"###
};
#[cfg(any(FiPercent, icondata_include_all))]
const FI_PERCENT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="19" y1="5" x2="5" y2="19" />
<circle cx="6.5" cy="6.5" r="2.5" />
<circle cx="17.5" cy="17.5" r="2.5" />"###
};
#[cfg(any(FiPhone, icondata_include_all))]
const FI_PHONE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z" />"###
};
#[cfg(any(FiPhoneCall, icondata_include_all))]
const FI_PHONE_CALL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M15.05 5A5 5 0 0 1 19 8.95M15.05 1A9 9 0 0 1 23 8.94m-1 7.98v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z" />"###
};
#[cfg(any(FiPhoneForwarded, icondata_include_all))]
const FI_PHONE_FORWARDED: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="19 1 23 5 19 9" />
<line x1="15" y1="5" x2="23" y2="5" />
<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z" />"###
};
#[cfg(any(FiPhoneIncoming, icondata_include_all))]
const FI_PHONE_INCOMING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="16 2 16 8 22 8" />
<line x1="23" y1="1" x2="16" y2="8" />
<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z" />"###
};
#[cfg(any(FiPhoneMissed, icondata_include_all))]
const FI_PHONE_MISSED: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="23" y1="1" x2="17" y2="7" />
<line x1="17" y1="1" x2="23" y2="7" />
<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z" />"###
};
#[cfg(any(FiPhoneOff, icondata_include_all))]
const FI_PHONE_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10.68 13.31a16 16 0 0 0 3.41 2.6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7 2 2 0 0 1 1.72 2v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.42 19.42 0 0 1-3.33-2.67m-2.67-3.34a19.79 19.79 0 0 1-3.07-8.63A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91" />
<line x1="23" y1="1" x2="1" y2="23" />"###
};
#[cfg(any(FiPhoneOutgoing, icondata_include_all))]
const FI_PHONE_OUTGOING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="23 7 23 1 17 1" />
<line x1="16" y1="8" x2="23" y2="1" />
<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z" />"###
};
#[cfg(any(FiPieChart, icondata_include_all))]
const FI_PIE_CHART: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21.21 15.89A10 10 0 1 1 8 2.83" />
<path d="M22 12A10 10 0 0 0 12 2v10z" />"###
};
#[cfg(any(FiPlay, icondata_include_all))]
const FI_PLAY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="5 3 19 12 5 21 5 3" />"###
};
#[cfg(any(FiPlayCircle, icondata_include_all))]
const FI_PLAY_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />
<polygon points="10 8 16 12 10 16 10 8" />"###
};
#[cfg(any(FiPlus, icondata_include_all))]
const FI_PLUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="12" y1="5" x2="12" y2="19" />
<line x1="5" y1="12" x2="19" y2="12" />"###
};
#[cfg(any(FiPlusCircle, icondata_include_all))]
const FI_PLUS_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />
<line x1="12" y1="8" x2="12" y2="16" />
<line x1="8" y1="12" x2="16" y2="12" />"###
};
#[cfg(any(FiPlusSquare, icondata_include_all))]
const FI_PLUS_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
<line x1="12" y1="8" x2="12" y2="16" />
<line x1="8" y1="12" x2="16" y2="12" />"###
};
#[cfg(any(FiPocket, icondata_include_all))]
const FI_POCKET: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 3h16a2 2 0 0 1 2 2v6a10 10 0 0 1-10 10A10 10 0 0 1 2 11V5a2 2 0 0 1 2-2z" />
<polyline points="8 10 12 14 16 10" />"###
};
#[cfg(any(FiPower, icondata_include_all))]
const FI_POWER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18.36 6.64a9 9 0 1 1-12.73 0" />
<line x1="12" y1="2" x2="12" y2="12" />"###
};
#[cfg(any(FiPrinter, icondata_include_all))]
const FI_PRINTER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="6 9 6 2 18 2 18 9" />
<path d="M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2" />
<rect x="6" y="14" width="12" height="8" />"###
};
#[cfg(any(FiRadio, icondata_include_all))]
const FI_RADIO: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="2" />
<path d="M16.24 7.76a6 6 0 0 1 0 8.49m-8.48-.01a6 6 0 0 1 0-8.49m11.31-2.82a10 10 0 0 1 0 14.14m-14.14 0a10 10 0 0 1 0-14.14" />"###
};
#[cfg(any(FiRefreshCcw, icondata_include_all))]
const FI_REFRESH_CCW: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="1 4 1 10 7 10" />
<polyline points="23 20 23 14 17 14" />
<path d="M20.49 9A9 9 0 0 0 5.64 5.64L1 10m22 4l-4.64 4.36A9 9 0 0 1 3.51 15" />"###
};
#[cfg(any(FiRefreshCw, icondata_include_all))]
const FI_REFRESH_CW: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="23 4 23 10 17 10" />
<polyline points="1 20 1 14 7 14" />
<path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15" />"###
};
#[cfg(any(FiRepeat, icondata_include_all))]
const FI_REPEAT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="17 1 21 5 17 9" />
<path d="M3 11V9a4 4 0 0 1 4-4h14" />
<polyline points="7 23 3 19 7 15" />
<path d="M21 13v2a4 4 0 0 1-4 4H3" />"###
};
#[cfg(any(FiRewind, icondata_include_all))]
const FI_REWIND: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="11 19 2 12 11 5 11 19" />
<polygon points="22 19 13 12 22 5 22 19" />"###
};
#[cfg(any(FiRotateCcw, icondata_include_all))]
const FI_ROTATE_CCW: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="1 4 1 10 7 10" />
<path d="M3.51 15a9 9 0 1 0 2.13-9.36L1 10" />"###
};
#[cfg(any(FiRotateCw, icondata_include_all))]
const FI_ROTATE_CW: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="23 4 23 10 17 10" />
<path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10" />"###
};
#[cfg(any(FiRss, icondata_include_all))]
const FI_RSS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 11a9 9 0 0 1 9 9" />
<path d="M4 4a16 16 0 0 1 16 16" />
<circle cx="5" cy="19" r="1" />"###
};
#[cfg(any(FiSave, icondata_include_all))]
const FI_SAVE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z" />
<polyline points="17 21 17 13 7 13 7 21" />
<polyline points="7 3 7 8 15 8" />"###
};
#[cfg(any(FiScissors, icondata_include_all))]
const FI_SCISSORS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="6" cy="6" r="3" />
<circle cx="6" cy="18" r="3" />
<line x1="20" y1="4" x2="8.12" y2="15.88" />
<line x1="14.47" y1="14.48" x2="20" y2="20" />
<line x1="8.12" y1="8.12" x2="12" y2="12" />"###
};
#[cfg(any(FiSearch, icondata_include_all))]
const FI_SEARCH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="11" cy="11" r="8" />
<line x1="21" y1="21" x2="16.65" y2="16.65" />"###
};
#[cfg(any(FiSend, icondata_include_all))]
const FI_SEND: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="22" y1="2" x2="11" y2="13" />
<polygon points="22 2 15 22 11 13 2 9 22 2" />"###
};
#[cfg(any(FiServer, icondata_include_all))]
const FI_SERVER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="2" y="2" width="20" height="8" rx="2" ry="2" />
<rect x="2" y="14" width="20" height="8" rx="2" ry="2" />
<line x1="6" y1="6" x2="6.01" y2="6" />
<line x1="6" y1="18" x2="6.01" y2="18" />"###
};
#[cfg(any(FiSettings, icondata_include_all))]
const FI_SETTINGS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="3" />
<path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />"###
};
#[cfg(any(FiShare, icondata_include_all))]
const FI_SHARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8" />
<polyline points="16 6 12 2 8 6" />
<line x1="12" y1="2" x2="12" y2="15" />"###
};
#[cfg(any(FiShare2, icondata_include_all))]
const FI_SHARE2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="18" cy="5" r="3" />
<circle cx="6" cy="12" r="3" />
<circle cx="18" cy="19" r="3" />
<line x1="8.59" y1="13.51" x2="15.42" y2="17.49" />
<line x1="15.41" y1="6.51" x2="8.59" y2="10.49" />"###
};
#[cfg(any(FiShield, icondata_include_all))]
const FI_SHIELD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" />"###
};
#[cfg(any(FiShieldOff, icondata_include_all))]
const FI_SHIELD_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M19.69 14a6.9 6.9 0 0 0 .31-2V5l-8-3-3.16 1.18" />
<path d="M4.73 4.73L4 5v7c0 6 8 10 8 10a20.29 20.29 0 0 0 5.62-4.38" />
<line x1="1" y1="1" x2="23" y2="23" />"###
};
#[cfg(any(FiShoppingBag, icondata_include_all))]
const FI_SHOPPING_BAG: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 2L3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6l-3-4z" />
<line x1="3" y1="6" x2="21" y2="6" />
<path d="M16 10a4 4 0 0 1-8 0" />"###
};
#[cfg(any(FiShoppingCart, icondata_include_all))]
const FI_SHOPPING_CART: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="9" cy="21" r="1" />
<circle cx="20" cy="21" r="1" />
<path d="M1 1h4l2.68 13.39a2 2 0 0 0 2 1.61h9.72a2 2 0 0 0 2-1.61L23 6H6" />"###
};
#[cfg(any(FiShuffle, icondata_include_all))]
const FI_SHUFFLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="16 3 21 3 21 8" />
<line x1="4" y1="20" x2="21" y2="3" />
<polyline points="21 16 21 21 16 21" />
<line x1="15" y1="15" x2="21" y2="21" />
<line x1="4" y1="4" x2="9" y2="9" />"###
};
#[cfg(any(FiSidebar, icondata_include_all))]
const FI_SIDEBAR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
<line x1="9" y1="3" x2="9" y2="21" />"###
};
#[cfg(any(FiSkipBack, icondata_include_all))]
const FI_SKIP_BACK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="19 20 9 12 19 4 19 20" />
<line x1="5" y1="19" x2="5" y2="5" />"###
};
#[cfg(any(FiSkipForward, icondata_include_all))]
const FI_SKIP_FORWARD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="5 4 15 12 5 20 5 4" />
<line x1="19" y1="5" x2="19" y2="19" />"###
};
#[cfg(any(FiSlack, icondata_include_all))]
const FI_SLACK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 10c-.83 0-1.5-.67-1.5-1.5v-5c0-.83.67-1.5 1.5-1.5s1.5.67 1.5 1.5v5c0 .83-.67 1.5-1.5 1.5z" />
<path d="M20.5 10H19V8.5c0-.83.67-1.5 1.5-1.5s1.5.67 1.5 1.5-.67 1.5-1.5 1.5z" />
<path d="M9.5 14c.83 0 1.5.67 1.5 1.5v5c0 .83-.67 1.5-1.5 1.5S8 21.33 8 20.5v-5c0-.83.67-1.5 1.5-1.5z" />
<path d="M3.5 14H5v1.5c0 .83-.67 1.5-1.5 1.5S2 16.33 2 15.5 2.67 14 3.5 14z" />
<path d="M14 14.5c0-.83.67-1.5 1.5-1.5h5c.83 0 1.5.67 1.5 1.5s-.67 1.5-1.5 1.5h-5c-.83 0-1.5-.67-1.5-1.5z" />
<path d="M15.5 19H14v1.5c0 .83.67 1.5 1.5 1.5s1.5-.67 1.5-1.5-.67-1.5-1.5-1.5z" />
<path d="M10 9.5C10 8.67 9.33 8 8.5 8h-5C2.67 8 2 8.67 2 9.5S2.67 11 3.5 11h5c.83 0 1.5-.67 1.5-1.5z" />
<path d="M8.5 5H10V3.5C10 2.67 9.33 2 8.5 2S7 2.67 7 3.5 7.67 5 8.5 5z" />"###
};
#[cfg(any(FiSlash, icondata_include_all))]
const FI_SLASH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />
<line x1="4.93" y1="4.93" x2="19.07" y2="19.07" />"###
};
#[cfg(any(FiSliders, icondata_include_all))]
const FI_SLIDERS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="4" y1="21" x2="4" y2="14" />
<line x1="4" y1="10" x2="4" y2="3" />
<line x1="12" y1="21" x2="12" y2="12" />
<line x1="12" y1="8" x2="12" y2="3" />
<line x1="20" y1="21" x2="20" y2="16" />
<line x1="20" y1="12" x2="20" y2="3" />
<line x1="1" y1="14" x2="7" y2="14" />
<line x1="9" y1="8" x2="15" y2="8" />
<line x1="17" y1="16" x2="23" y2="16" />"###
};
#[cfg(any(FiSmartphone, icondata_include_all))]
const FI_SMARTPHONE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="5" y="2" width="14" height="20" rx="2" ry="2" />
<line x1="12" y1="18" x2="12.01" y2="18" />"###
};
#[cfg(any(FiSmile, icondata_include_all))]
const FI_SMILE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />
<path d="M8 14s1.5 2 4 2 4-2 4-2" />
<line x1="9" y1="9" x2="9.01" y2="9" />
<line x1="15" y1="9" x2="15.01" y2="9" />"###
};
#[cfg(any(FiSpeaker, icondata_include_all))]
const FI_SPEAKER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="4" y="2" width="16" height="20" rx="2" ry="2" />
<circle cx="12" cy="14" r="4" />
<line x1="12" y1="6" x2="12.01" y2="6" />"###
};
#[cfg(any(FiSquare, icondata_include_all))]
const FI_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="3" y="3" width="18" height="18" rx="2" ry="2" />"###
};
#[cfg(any(FiStar, icondata_include_all))]
const FI_STAR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" />"###
};
#[cfg(any(FiStopCircle, icondata_include_all))]
const FI_STOP_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />
<rect x="9" y="9" width="6" height="6" />"###
};
#[cfg(any(FiSun, icondata_include_all))]
const FI_SUN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="5" />
<line x1="12" y1="1" x2="12" y2="3" />
<line x1="12" y1="21" x2="12" y2="23" />
<line x1="4.22" y1="4.22" x2="5.64" y2="5.64" />
<line x1="18.36" y1="18.36" x2="19.78" y2="19.78" />
<line x1="1" y1="12" x2="3" y2="12" />
<line x1="21" y1="12" x2="23" y2="12" />
<line x1="4.22" y1="19.78" x2="5.64" y2="18.36" />
<line x1="18.36" y1="5.64" x2="19.78" y2="4.22" />"###
};
#[cfg(any(FiSunrise, icondata_include_all))]
const FI_SUNRISE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17 18a5 5 0 0 0-10 0" />
<line x1="12" y1="2" x2="12" y2="9" />
<line x1="4.22" y1="10.22" x2="5.64" y2="11.64" />
<line x1="1" y1="18" x2="3" y2="18" />
<line x1="21" y1="18" x2="23" y2="18" />
<line x1="18.36" y1="11.64" x2="19.78" y2="10.22" />
<line x1="23" y1="22" x2="1" y2="22" />
<polyline points="8 6 12 2 16 6" />"###
};
#[cfg(any(FiSunset, icondata_include_all))]
const FI_SUNSET: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17 18a5 5 0 0 0-10 0" />
<line x1="12" y1="9" x2="12" y2="2" />
<line x1="4.22" y1="10.22" x2="5.64" y2="11.64" />
<line x1="1" y1="18" x2="3" y2="18" />
<line x1="21" y1="18" x2="23" y2="18" />
<line x1="18.36" y1="11.64" x2="19.78" y2="10.22" />
<line x1="23" y1="22" x2="1" y2="22" />
<polyline points="16 5 12 9 8 5" />"###
};
#[cfg(any(FiTable, icondata_include_all))]
const FI_TABLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M9 3H5a2 2 0 0 0-2 2v4m6-6h10a2 2 0 0 1 2 2v4M9 3v18m0 0h10a2 2 0 0 0 2-2V9M9 21H5a2 2 0 0 1-2-2V9m0 0h18" />"###
};
#[cfg(any(FiTablet, icondata_include_all))]
const FI_TABLET: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="4" y="2" width="16" height="20" rx="2" ry="2" />
<line x1="12" y1="18" x2="12.01" y2="18" />"###
};
#[cfg(any(FiTag, icondata_include_all))]
const FI_TAG: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M20.59 13.41l-7.17 7.17a2 2 0 0 1-2.83 0L2 12V2h10l8.59 8.59a2 2 0 0 1 0 2.82z" />
<line x1="7" y1="7" x2="7.01" y2="7" />"###
};
#[cfg(any(FiTarget, icondata_include_all))]
const FI_TARGET: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />
<circle cx="12" cy="12" r="6" />
<circle cx="12" cy="12" r="2" />"###
};
#[cfg(any(FiTerminal, icondata_include_all))]
const FI_TERMINAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="4 17 10 11 4 5" />
<line x1="12" y1="19" x2="20" y2="19" />"###
};
#[cfg(any(FiThermometer, icondata_include_all))]
const FI_THERMOMETER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14 14.76V3.5a2.5 2.5 0 0 0-5 0v11.26a4.5 4.5 0 1 0 5 0z" />"###
};
#[cfg(any(FiThumbsDown, icondata_include_all))]
const FI_THUMBS_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10 15v4a3 3 0 0 0 3 3l4-9V2H5.72a2 2 0 0 0-2 1.7l-1.38 9a2 2 0 0 0 2 2.3zm7-13h2.67A2.31 2.31 0 0 1 22 4v7a2.31 2.31 0 0 1-2.33 2H17" />"###
};
#[cfg(any(FiThumbsUp, icondata_include_all))]
const FI_THUMBS_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14 9V5a3 3 0 0 0-3-3l-4 9v11h11.28a2 2 0 0 0 2-1.7l1.38-9a2 2 0 0 0-2-2.3zM7 22H4a2 2 0 0 1-2-2v-7a2 2 0 0 1 2-2h3" />"###
};
#[cfg(any(FiToggleLeft, icondata_include_all))]
const FI_TOGGLE_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="1" y="5" width="22" height="14" rx="7" ry="7" />
<circle cx="8" cy="12" r="3" />"###
};
#[cfg(any(FiToggleRight, icondata_include_all))]
const FI_TOGGLE_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="1" y="5" width="22" height="14" rx="7" ry="7" />
<circle cx="16" cy="12" r="3" />"###
};
#[cfg(any(FiTool, icondata_include_all))]
const FI_TOOL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z" />"###
};
#[cfg(any(FiTrash, icondata_include_all))]
const FI_TRASH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="3 6 5 6 21 6" />
<path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />"###
};
#[cfg(any(FiTrash2, icondata_include_all))]
const FI_TRASH2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="3 6 5 6 21 6" />
<path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
<line x1="10" y1="11" x2="10" y2="17" />
<line x1="14" y1="11" x2="14" y2="17" />"###
};
#[cfg(any(FiTrello, icondata_include_all))]
const FI_TRELLO: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
<rect x="7" y="7" width="3" height="9" />
<rect x="14" y="7" width="3" height="5" />"###
};
#[cfg(any(FiTrendingDown, icondata_include_all))]
const FI_TRENDING_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="23 18 13.5 8.5 8.5 13.5 1 6" />
<polyline points="17 18 23 18 23 12" />"###
};
#[cfg(any(FiTrendingUp, icondata_include_all))]
const FI_TRENDING_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="23 6 13.5 15.5 8.5 10.5 1 18" />
<polyline points="17 6 23 6 23 12" />"###
};
#[cfg(any(FiTriangle, icondata_include_all))]
const FI_TRIANGLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" />"###
};
#[cfg(any(FiTruck, icondata_include_all))]
const FI_TRUCK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="1" y="3" width="15" height="13" />
<polygon points="16 8 20 8 23 11 23 16 16 16 16 8" />
<circle cx="5.5" cy="18.5" r="2.5" />
<circle cx="18.5" cy="18.5" r="2.5" />"###
};
#[cfg(any(FiTv, icondata_include_all))]
const FI_TV: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="2" y="7" width="20" height="15" rx="2" ry="2" />
<polyline points="17 2 12 7 7 2" />"###
};
#[cfg(any(FiTwitch, icondata_include_all))]
const FI_TWITCH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 2H3v16h5v4l4-4h5l4-4V2zM11 11V7M16 11V7" />"###
};
#[cfg(any(FiTwitter, icondata_include_all))]
const FI_TWITTER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M23 3a10.9 10.9 0 0 1-3.14 1.53 4.48 4.48 0 0 0-7.86 3v1A10.66 10.66 0 0 1 3 4s-4 9 5 13a11.64 11.64 0 0 1-7 2c9 5 20 0 20-11.5a4.5 4.5 0 0 0-.08-.83A7.72 7.72 0 0 0 23 3z" />"###
};
#[cfg(any(FiType, icondata_include_all))]
const FI_TYPE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="4 7 4 4 20 4 20 7" />
<line x1="9" y1="20" x2="15" y2="20" />
<line x1="12" y1="4" x2="12" y2="20" />"###
};
#[cfg(any(FiUmbrella, icondata_include_all))]
const FI_UMBRELLA: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M23 12a11.05 11.05 0 0 0-22 0zm-5 7a3 3 0 0 1-6 0v-7" />"###
};
#[cfg(any(FiUnderline, icondata_include_all))]
const FI_UNDERLINE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 3v7a6 6 0 0 0 6 6 6 6 0 0 0 6-6V3" />
<line x1="4" y1="21" x2="20" y2="21" />"###
};
#[cfg(any(FiUnlock, icondata_include_all))]
const FI_UNLOCK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="3" y="11" width="18" height="11" rx="2" ry="2" />
<path d="M7 11V7a5 5 0 0 1 9.9-1" />"###
};
#[cfg(any(FiUpload, icondata_include_all))]
const FI_UPLOAD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
<polyline points="17 8 12 3 7 8" />
<line x1="12" y1="3" x2="12" y2="15" />"###
};
#[cfg(any(FiUploadCloud, icondata_include_all))]
const FI_UPLOAD_CLOUD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="16 16 12 12 8 16" />
<line x1="12" y1="12" x2="12" y2="21" />
<path d="M20.39 18.39A5 5 0 0 0 18 9h-1.26A8 8 0 1 0 3 16.3" />
<polyline points="16 16 12 12 8 16" />"###
};
#[cfg(any(FiUser, icondata_include_all))]
const FI_USER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" />
<circle cx="12" cy="7" r="4" />"###
};
#[cfg(any(FiUserCheck, icondata_include_all))]
const FI_USER_CHECK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" />
<circle cx="8.5" cy="7" r="4" />
<polyline points="17 11 19 13 23 9" />"###
};
#[cfg(any(FiUserMinus, icondata_include_all))]
const FI_USER_MINUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" />
<circle cx="8.5" cy="7" r="4" />
<line x1="23" y1="11" x2="17" y2="11" />"###
};
#[cfg(any(FiUserPlus, icondata_include_all))]
const FI_USER_PLUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" />
<circle cx="8.5" cy="7" r="4" />
<line x1="20" y1="8" x2="20" y2="14" />
<line x1="23" y1="11" x2="17" y2="11" />"###
};
#[cfg(any(FiUserX, icondata_include_all))]
const FI_USER_X: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" />
<circle cx="8.5" cy="7" r="4" />
<line x1="18" y1="8" x2="23" y2="13" />
<line x1="23" y1="8" x2="18" y2="13" />"###
};
#[cfg(any(FiUsers, icondata_include_all))]
const FI_USERS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" />
<circle cx="9" cy="7" r="4" />
<path d="M23 21v-2a4 4 0 0 0-3-3.87" />
<path d="M16 3.13a4 4 0 0 1 0 7.75" />"###
};
#[cfg(any(FiVideo, icondata_include_all))]
const FI_VIDEO: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="23 7 16 12 23 17 23 7" />
<rect x="1" y="5" width="15" height="14" rx="2" ry="2" />"###
};
#[cfg(any(FiVideoOff, icondata_include_all))]
const FI_VIDEO_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16 16v1a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h2m5.66 0H14a2 2 0 0 1 2 2v3.34l1 1L23 7v10" />
<line x1="1" y1="1" x2="23" y2="23" />"###
};
#[cfg(any(FiVoicemail, icondata_include_all))]
const FI_VOICEMAIL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="5.5" cy="11.5" r="4.5" />
<circle cx="18.5" cy="11.5" r="4.5" />
<line x1="5.5" y1="16" x2="18.5" y2="16" />"###
};
#[cfg(any(FiVolume, icondata_include_all))]
const FI_VOLUME: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />"###
};
#[cfg(any(FiVolume1, icondata_include_all))]
const FI_VOLUME1: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
<path d="M15.54 8.46a5 5 0 0 1 0 7.07" />"###
};
#[cfg(any(FiVolume2, icondata_include_all))]
const FI_VOLUME2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
<path d="M19.07 4.93a10 10 0 0 1 0 14.14M15.54 8.46a5 5 0 0 1 0 7.07" />"###
};
#[cfg(any(FiVolumeX, icondata_include_all))]
const FI_VOLUME_X: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
<line x1="23" y1="9" x2="17" y2="15" />
<line x1="17" y1="9" x2="23" y2="15" />"###
};
#[cfg(any(FiWatch, icondata_include_all))]
const FI_WATCH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="7" />
<polyline points="12 9 12 12 13.5 13.5" />
<path d="M16.51 17.35l-.35 3.83a2 2 0 0 1-2 1.82H9.83a2 2 0 0 1-2-1.82l-.35-3.83m.01-10.7l.35-3.83A2 2 0 0 1 9.83 1h4.35a2 2 0 0 1 2 1.82l.35 3.83" />"###
};
#[cfg(any(FiWifi, icondata_include_all))]
const FI_WIFI: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 12.55a11 11 0 0 1 14.08 0" />
<path d="M1.42 9a16 16 0 0 1 21.16 0" />
<path d="M8.53 16.11a6 6 0 0 1 6.95 0" />
<line x1="12" y1="20" x2="12.01" y2="20" />"###
};
#[cfg(any(FiWifiOff, icondata_include_all))]
const FI_WIFI_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="1" y1="1" x2="23" y2="23" />
<path d="M16.72 11.06A10.94 10.94 0 0 1 19 12.55" />
<path d="M5 12.55a10.94 10.94 0 0 1 5.17-2.39" />
<path d="M10.71 5.05A16 16 0 0 1 22.58 9" />
<path d="M1.42 9a15.91 15.91 0 0 1 4.7-2.88" />
<path d="M8.53 16.11a6 6 0 0 1 6.95 0" />
<line x1="12" y1="20" x2="12.01" y2="20" />"###
};
#[cfg(any(FiWind, icondata_include_all))]
const FI_WIND: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M9.59 4.59A2 2 0 1 1 11 8H2m10.59 11.41A2 2 0 1 0 14 16H2m15.73-8.27A2.5 2.5 0 1 1 19.5 12H2" />"###
};
#[cfg(any(FiX, icondata_include_all))]
const FI_X: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="18" y1="6" x2="6" y2="18" />
<line x1="6" y1="6" x2="18" y2="18" />"###
};
#[cfg(any(FiXCircle, icondata_include_all))]
const FI_X_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="10" />
<line x1="15" y1="9" x2="9" y2="15" />
<line x1="9" y1="9" x2="15" y2="15" />"###
};
#[cfg(any(FiXOctagon, icondata_include_all))]
const FI_X_OCTAGON: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2" />
<line x1="15" y1="9" x2="9" y2="15" />
<line x1="9" y1="9" x2="15" y2="15" />"###
};
#[cfg(any(FiXSquare, icondata_include_all))]
const FI_X_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
<line x1="9" y1="9" x2="15" y2="15" />
<line x1="15" y1="9" x2="9" y2="15" />"###
};
#[cfg(any(FiYoutube, icondata_include_all))]
const FI_YOUTUBE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22.54 6.42a2.78 2.78 0 0 0-1.94-2C18.88 4 12 4 12 4s-6.88 0-8.6.46a2.78 2.78 0 0 0-1.94 2A29 29 0 0 0 1 11.75a29 29 0 0 0 .46 5.33A2.78 2.78 0 0 0 3.4 19c1.72.46 8.6.46 8.6.46s6.88 0 8.6-.46a2.78 2.78 0 0 0 1.94-2 29 29 0 0 0 .46-5.25 29 29 0 0 0-.46-5.33z" />
<polygon points="9.75 15.02 15.5 11.75 9.75 8.48 9.75 15.02" />"###
};
#[cfg(any(FiZap, icondata_include_all))]
const FI_ZAP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2" />"###
};
#[cfg(any(FiZapOff, icondata_include_all))]
const FI_ZAP_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="12.41 6.75 13 2 10.57 4.92" />
<polyline points="18.57 12.91 21 10 15.66 10" />
<polyline points="8 8 3 14 12 14 11 22 16 16" />
<line x1="1" y1="1" x2="23" y2="23" />"###
};
#[cfg(any(FiZoomIn, icondata_include_all))]
const FI_ZOOM_IN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="11" cy="11" r="8" />
<line x1="21" y1="21" x2="16.65" y2="16.65" />
<line x1="11" y1="8" x2="11" y2="14" />
<line x1="8" y1="11" x2="14" y2="11" />"###
};
#[cfg(any(FiZoomOut, icondata_include_all))]
const FI_ZOOM_OUT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="11" cy="11" r="8" />
<line x1="21" y1="21" x2="16.65" y2="16.65" />
<line x1="8" y1="11" x2="14" y2="11" />"###
};

impl From<FiIcon> for icondata_core::IconData {
    fn from(icon: FiIcon) -> icondata_core::IconData {
        match icon {
            #[cfg(any(FiActivity, icondata_include_all))]
            FiIcon::FiActivity => FI_ACTIVITY,
            #[cfg(any(FiAirplay, icondata_include_all))]
            FiIcon::FiAirplay => FI_AIRPLAY,
            #[cfg(any(FiAlertCircle, icondata_include_all))]
            FiIcon::FiAlertCircle => FI_ALERT_CIRCLE,
            #[cfg(any(FiAlertOctagon, icondata_include_all))]
            FiIcon::FiAlertOctagon => FI_ALERT_OCTAGON,
            #[cfg(any(FiAlertTriangle, icondata_include_all))]
            FiIcon::FiAlertTriangle => FI_ALERT_TRIANGLE,
            #[cfg(any(FiAlignCenter, icondata_include_all))]
            FiIcon::FiAlignCenter => FI_ALIGN_CENTER,
            #[cfg(any(FiAlignJustify, icondata_include_all))]
            FiIcon::FiAlignJustify => FI_ALIGN_JUSTIFY,
            #[cfg(any(FiAlignLeft, icondata_include_all))]
            FiIcon::FiAlignLeft => FI_ALIGN_LEFT,
            #[cfg(any(FiAlignRight, icondata_include_all))]
            FiIcon::FiAlignRight => FI_ALIGN_RIGHT,
            #[cfg(any(FiAnchor, icondata_include_all))]
            FiIcon::FiAnchor => FI_ANCHOR,
            #[cfg(any(FiAperture, icondata_include_all))]
            FiIcon::FiAperture => FI_APERTURE,
            #[cfg(any(FiArchive, icondata_include_all))]
            FiIcon::FiArchive => FI_ARCHIVE,
            #[cfg(any(FiArrowDown, icondata_include_all))]
            FiIcon::FiArrowDown => FI_ARROW_DOWN,
            #[cfg(any(FiArrowDownCircle, icondata_include_all))]
            FiIcon::FiArrowDownCircle => FI_ARROW_DOWN_CIRCLE,
            #[cfg(any(FiArrowDownLeft, icondata_include_all))]
            FiIcon::FiArrowDownLeft => FI_ARROW_DOWN_LEFT,
            #[cfg(any(FiArrowDownRight, icondata_include_all))]
            FiIcon::FiArrowDownRight => FI_ARROW_DOWN_RIGHT,
            #[cfg(any(FiArrowLeft, icondata_include_all))]
            FiIcon::FiArrowLeft => FI_ARROW_LEFT,
            #[cfg(any(FiArrowLeftCircle, icondata_include_all))]
            FiIcon::FiArrowLeftCircle => FI_ARROW_LEFT_CIRCLE,
            #[cfg(any(FiArrowRight, icondata_include_all))]
            FiIcon::FiArrowRight => FI_ARROW_RIGHT,
            #[cfg(any(FiArrowRightCircle, icondata_include_all))]
            FiIcon::FiArrowRightCircle => FI_ARROW_RIGHT_CIRCLE,
            #[cfg(any(FiArrowUp, icondata_include_all))]
            FiIcon::FiArrowUp => FI_ARROW_UP,
            #[cfg(any(FiArrowUpCircle, icondata_include_all))]
            FiIcon::FiArrowUpCircle => FI_ARROW_UP_CIRCLE,
            #[cfg(any(FiArrowUpLeft, icondata_include_all))]
            FiIcon::FiArrowUpLeft => FI_ARROW_UP_LEFT,
            #[cfg(any(FiArrowUpRight, icondata_include_all))]
            FiIcon::FiArrowUpRight => FI_ARROW_UP_RIGHT,
            #[cfg(any(FiAtSign, icondata_include_all))]
            FiIcon::FiAtSign => FI_AT_SIGN,
            #[cfg(any(FiAward, icondata_include_all))]
            FiIcon::FiAward => FI_AWARD,
            #[cfg(any(FiBarChart, icondata_include_all))]
            FiIcon::FiBarChart => FI_BAR_CHART,
            #[cfg(any(FiBarChart2, icondata_include_all))]
            FiIcon::FiBarChart2 => FI_BAR_CHART2,
            #[cfg(any(FiBattery, icondata_include_all))]
            FiIcon::FiBattery => FI_BATTERY,
            #[cfg(any(FiBatteryCharging, icondata_include_all))]
            FiIcon::FiBatteryCharging => FI_BATTERY_CHARGING,
            #[cfg(any(FiBell, icondata_include_all))]
            FiIcon::FiBell => FI_BELL,
            #[cfg(any(FiBellOff, icondata_include_all))]
            FiIcon::FiBellOff => FI_BELL_OFF,
            #[cfg(any(FiBluetooth, icondata_include_all))]
            FiIcon::FiBluetooth => FI_BLUETOOTH,
            #[cfg(any(FiBold, icondata_include_all))]
            FiIcon::FiBold => FI_BOLD,
            #[cfg(any(FiBook, icondata_include_all))]
            FiIcon::FiBook => FI_BOOK,
            #[cfg(any(FiBookOpen, icondata_include_all))]
            FiIcon::FiBookOpen => FI_BOOK_OPEN,
            #[cfg(any(FiBookmark, icondata_include_all))]
            FiIcon::FiBookmark => FI_BOOKMARK,
            #[cfg(any(FiBox, icondata_include_all))]
            FiIcon::FiBox => FI_BOX,
            #[cfg(any(FiBriefcase, icondata_include_all))]
            FiIcon::FiBriefcase => FI_BRIEFCASE,
            #[cfg(any(FiCalendar, icondata_include_all))]
            FiIcon::FiCalendar => FI_CALENDAR,
            #[cfg(any(FiCamera, icondata_include_all))]
            FiIcon::FiCamera => FI_CAMERA,
            #[cfg(any(FiCameraOff, icondata_include_all))]
            FiIcon::FiCameraOff => FI_CAMERA_OFF,
            #[cfg(any(FiCast, icondata_include_all))]
            FiIcon::FiCast => FI_CAST,
            #[cfg(any(FiCheck, icondata_include_all))]
            FiIcon::FiCheck => FI_CHECK,
            #[cfg(any(FiCheckCircle, icondata_include_all))]
            FiIcon::FiCheckCircle => FI_CHECK_CIRCLE,
            #[cfg(any(FiCheckSquare, icondata_include_all))]
            FiIcon::FiCheckSquare => FI_CHECK_SQUARE,
            #[cfg(any(FiChevronDown, icondata_include_all))]
            FiIcon::FiChevronDown => FI_CHEVRON_DOWN,
            #[cfg(any(FiChevronLeft, icondata_include_all))]
            FiIcon::FiChevronLeft => FI_CHEVRON_LEFT,
            #[cfg(any(FiChevronRight, icondata_include_all))]
            FiIcon::FiChevronRight => FI_CHEVRON_RIGHT,
            #[cfg(any(FiChevronUp, icondata_include_all))]
            FiIcon::FiChevronUp => FI_CHEVRON_UP,
            #[cfg(any(FiChevronsDown, icondata_include_all))]
            FiIcon::FiChevronsDown => FI_CHEVRONS_DOWN,
            #[cfg(any(FiChevronsLeft, icondata_include_all))]
            FiIcon::FiChevronsLeft => FI_CHEVRONS_LEFT,
            #[cfg(any(FiChevronsRight, icondata_include_all))]
            FiIcon::FiChevronsRight => FI_CHEVRONS_RIGHT,
            #[cfg(any(FiChevronsUp, icondata_include_all))]
            FiIcon::FiChevronsUp => FI_CHEVRONS_UP,
            #[cfg(any(FiChrome, icondata_include_all))]
            FiIcon::FiChrome => FI_CHROME,
            #[cfg(any(FiCircle, icondata_include_all))]
            FiIcon::FiCircle => FI_CIRCLE,
            #[cfg(any(FiClipboard, icondata_include_all))]
            FiIcon::FiClipboard => FI_CLIPBOARD,
            #[cfg(any(FiClock, icondata_include_all))]
            FiIcon::FiClock => FI_CLOCK,
            #[cfg(any(FiCloud, icondata_include_all))]
            FiIcon::FiCloud => FI_CLOUD,
            #[cfg(any(FiCloudDrizzle, icondata_include_all))]
            FiIcon::FiCloudDrizzle => FI_CLOUD_DRIZZLE,
            #[cfg(any(FiCloudLightning, icondata_include_all))]
            FiIcon::FiCloudLightning => FI_CLOUD_LIGHTNING,
            #[cfg(any(FiCloudOff, icondata_include_all))]
            FiIcon::FiCloudOff => FI_CLOUD_OFF,
            #[cfg(any(FiCloudRain, icondata_include_all))]
            FiIcon::FiCloudRain => FI_CLOUD_RAIN,
            #[cfg(any(FiCloudSnow, icondata_include_all))]
            FiIcon::FiCloudSnow => FI_CLOUD_SNOW,
            #[cfg(any(FiCode, icondata_include_all))]
            FiIcon::FiCode => FI_CODE,
            #[cfg(any(FiCodepen, icondata_include_all))]
            FiIcon::FiCodepen => FI_CODEPEN,
            #[cfg(any(FiCodesandbox, icondata_include_all))]
            FiIcon::FiCodesandbox => FI_CODESANDBOX,
            #[cfg(any(FiCoffee, icondata_include_all))]
            FiIcon::FiCoffee => FI_COFFEE,
            #[cfg(any(FiColumns, icondata_include_all))]
            FiIcon::FiColumns => FI_COLUMNS,
            #[cfg(any(FiCommand, icondata_include_all))]
            FiIcon::FiCommand => FI_COMMAND,
            #[cfg(any(FiCompass, icondata_include_all))]
            FiIcon::FiCompass => FI_COMPASS,
            #[cfg(any(FiCopy, icondata_include_all))]
            FiIcon::FiCopy => FI_COPY,
            #[cfg(any(FiCornerDownLeft, icondata_include_all))]
            FiIcon::FiCornerDownLeft => FI_CORNER_DOWN_LEFT,
            #[cfg(any(FiCornerDownRight, icondata_include_all))]
            FiIcon::FiCornerDownRight => FI_CORNER_DOWN_RIGHT,
            #[cfg(any(FiCornerLeftDown, icondata_include_all))]
            FiIcon::FiCornerLeftDown => FI_CORNER_LEFT_DOWN,
            #[cfg(any(FiCornerLeftUp, icondata_include_all))]
            FiIcon::FiCornerLeftUp => FI_CORNER_LEFT_UP,
            #[cfg(any(FiCornerRightDown, icondata_include_all))]
            FiIcon::FiCornerRightDown => FI_CORNER_RIGHT_DOWN,
            #[cfg(any(FiCornerRightUp, icondata_include_all))]
            FiIcon::FiCornerRightUp => FI_CORNER_RIGHT_UP,
            #[cfg(any(FiCornerUpLeft, icondata_include_all))]
            FiIcon::FiCornerUpLeft => FI_CORNER_UP_LEFT,
            #[cfg(any(FiCornerUpRight, icondata_include_all))]
            FiIcon::FiCornerUpRight => FI_CORNER_UP_RIGHT,
            #[cfg(any(FiCpu, icondata_include_all))]
            FiIcon::FiCpu => FI_CPU,
            #[cfg(any(FiCreditCard, icondata_include_all))]
            FiIcon::FiCreditCard => FI_CREDIT_CARD,
            #[cfg(any(FiCrop, icondata_include_all))]
            FiIcon::FiCrop => FI_CROP,
            #[cfg(any(FiCrosshair, icondata_include_all))]
            FiIcon::FiCrosshair => FI_CROSSHAIR,
            #[cfg(any(FiDatabase, icondata_include_all))]
            FiIcon::FiDatabase => FI_DATABASE,
            #[cfg(any(FiDelete, icondata_include_all))]
            FiIcon::FiDelete => FI_DELETE,
            #[cfg(any(FiDisc, icondata_include_all))]
            FiIcon::FiDisc => FI_DISC,
            #[cfg(any(FiDivide, icondata_include_all))]
            FiIcon::FiDivide => FI_DIVIDE,
            #[cfg(any(FiDivideCircle, icondata_include_all))]
            FiIcon::FiDivideCircle => FI_DIVIDE_CIRCLE,
            #[cfg(any(FiDivideSquare, icondata_include_all))]
            FiIcon::FiDivideSquare => FI_DIVIDE_SQUARE,
            #[cfg(any(FiDollarSign, icondata_include_all))]
            FiIcon::FiDollarSign => FI_DOLLAR_SIGN,
            #[cfg(any(FiDownload, icondata_include_all))]
            FiIcon::FiDownload => FI_DOWNLOAD,
            #[cfg(any(FiDownloadCloud, icondata_include_all))]
            FiIcon::FiDownloadCloud => FI_DOWNLOAD_CLOUD,
            #[cfg(any(FiDribbble, icondata_include_all))]
            FiIcon::FiDribbble => FI_DRIBBBLE,
            #[cfg(any(FiDroplet, icondata_include_all))]
            FiIcon::FiDroplet => FI_DROPLET,
            #[cfg(any(FiEdit, icondata_include_all))]
            FiIcon::FiEdit => FI_EDIT,
            #[cfg(any(FiEdit2, icondata_include_all))]
            FiIcon::FiEdit2 => FI_EDIT2,
            #[cfg(any(FiEdit3, icondata_include_all))]
            FiIcon::FiEdit3 => FI_EDIT3,
            #[cfg(any(FiExternalLink, icondata_include_all))]
            FiIcon::FiExternalLink => FI_EXTERNAL_LINK,
            #[cfg(any(FiEye, icondata_include_all))]
            FiIcon::FiEye => FI_EYE,
            #[cfg(any(FiEyeOff, icondata_include_all))]
            FiIcon::FiEyeOff => FI_EYE_OFF,
            #[cfg(any(FiFacebook, icondata_include_all))]
            FiIcon::FiFacebook => FI_FACEBOOK,
            #[cfg(any(FiFastForward, icondata_include_all))]
            FiIcon::FiFastForward => FI_FAST_FORWARD,
            #[cfg(any(FiFeather, icondata_include_all))]
            FiIcon::FiFeather => FI_FEATHER,
            #[cfg(any(FiFigma, icondata_include_all))]
            FiIcon::FiFigma => FI_FIGMA,
            #[cfg(any(FiFile, icondata_include_all))]
            FiIcon::FiFile => FI_FILE,
            #[cfg(any(FiFileMinus, icondata_include_all))]
            FiIcon::FiFileMinus => FI_FILE_MINUS,
            #[cfg(any(FiFilePlus, icondata_include_all))]
            FiIcon::FiFilePlus => FI_FILE_PLUS,
            #[cfg(any(FiFileText, icondata_include_all))]
            FiIcon::FiFileText => FI_FILE_TEXT,
            #[cfg(any(FiFilm, icondata_include_all))]
            FiIcon::FiFilm => FI_FILM,
            #[cfg(any(FiFilter, icondata_include_all))]
            FiIcon::FiFilter => FI_FILTER,
            #[cfg(any(FiFlag, icondata_include_all))]
            FiIcon::FiFlag => FI_FLAG,
            #[cfg(any(FiFolder, icondata_include_all))]
            FiIcon::FiFolder => FI_FOLDER,
            #[cfg(any(FiFolderMinus, icondata_include_all))]
            FiIcon::FiFolderMinus => FI_FOLDER_MINUS,
            #[cfg(any(FiFolderPlus, icondata_include_all))]
            FiIcon::FiFolderPlus => FI_FOLDER_PLUS,
            #[cfg(any(FiFramer, icondata_include_all))]
            FiIcon::FiFramer => FI_FRAMER,
            #[cfg(any(FiFrown, icondata_include_all))]
            FiIcon::FiFrown => FI_FROWN,
            #[cfg(any(FiGift, icondata_include_all))]
            FiIcon::FiGift => FI_GIFT,
            #[cfg(any(FiGitBranch, icondata_include_all))]
            FiIcon::FiGitBranch => FI_GIT_BRANCH,
            #[cfg(any(FiGitCommit, icondata_include_all))]
            FiIcon::FiGitCommit => FI_GIT_COMMIT,
            #[cfg(any(FiGitMerge, icondata_include_all))]
            FiIcon::FiGitMerge => FI_GIT_MERGE,
            #[cfg(any(FiGitPullRequest, icondata_include_all))]
            FiIcon::FiGitPullRequest => FI_GIT_PULL_REQUEST,
            #[cfg(any(FiGithub, icondata_include_all))]
            FiIcon::FiGithub => FI_GITHUB,
            #[cfg(any(FiGitlab, icondata_include_all))]
            FiIcon::FiGitlab => FI_GITLAB,
            #[cfg(any(FiGlobe, icondata_include_all))]
            FiIcon::FiGlobe => FI_GLOBE,
            #[cfg(any(FiGrid, icondata_include_all))]
            FiIcon::FiGrid => FI_GRID,
            #[cfg(any(FiHardDrive, icondata_include_all))]
            FiIcon::FiHardDrive => FI_HARD_DRIVE,
            #[cfg(any(FiHash, icondata_include_all))]
            FiIcon::FiHash => FI_HASH,
            #[cfg(any(FiHeadphones, icondata_include_all))]
            FiIcon::FiHeadphones => FI_HEADPHONES,
            #[cfg(any(FiHeart, icondata_include_all))]
            FiIcon::FiHeart => FI_HEART,
            #[cfg(any(FiHelpCircle, icondata_include_all))]
            FiIcon::FiHelpCircle => FI_HELP_CIRCLE,
            #[cfg(any(FiHexagon, icondata_include_all))]
            FiIcon::FiHexagon => FI_HEXAGON,
            #[cfg(any(FiHome, icondata_include_all))]
            FiIcon::FiHome => FI_HOME,
            #[cfg(any(FiImage, icondata_include_all))]
            FiIcon::FiImage => FI_IMAGE,
            #[cfg(any(FiInbox, icondata_include_all))]
            FiIcon::FiInbox => FI_INBOX,
            #[cfg(any(FiInfo, icondata_include_all))]
            FiIcon::FiInfo => FI_INFO,
            #[cfg(any(FiInstagram, icondata_include_all))]
            FiIcon::FiInstagram => FI_INSTAGRAM,
            #[cfg(any(FiItalic, icondata_include_all))]
            FiIcon::FiItalic => FI_ITALIC,
            #[cfg(any(FiKey, icondata_include_all))]
            FiIcon::FiKey => FI_KEY,
            #[cfg(any(FiLayers, icondata_include_all))]
            FiIcon::FiLayers => FI_LAYERS,
            #[cfg(any(FiLayout, icondata_include_all))]
            FiIcon::FiLayout => FI_LAYOUT,
            #[cfg(any(FiLifeBuoy, icondata_include_all))]
            FiIcon::FiLifeBuoy => FI_LIFE_BUOY,
            #[cfg(any(FiLink, icondata_include_all))]
            FiIcon::FiLink => FI_LINK,
            #[cfg(any(FiLink2, icondata_include_all))]
            FiIcon::FiLink2 => FI_LINK2,
            #[cfg(any(FiLinkedin, icondata_include_all))]
            FiIcon::FiLinkedin => FI_LINKEDIN,
            #[cfg(any(FiList, icondata_include_all))]
            FiIcon::FiList => FI_LIST,
            #[cfg(any(FiLoader, icondata_include_all))]
            FiIcon::FiLoader => FI_LOADER,
            #[cfg(any(FiLock, icondata_include_all))]
            FiIcon::FiLock => FI_LOCK,
            #[cfg(any(FiLogIn, icondata_include_all))]
            FiIcon::FiLogIn => FI_LOG_IN,
            #[cfg(any(FiLogOut, icondata_include_all))]
            FiIcon::FiLogOut => FI_LOG_OUT,
            #[cfg(any(FiMail, icondata_include_all))]
            FiIcon::FiMail => FI_MAIL,
            #[cfg(any(FiMap, icondata_include_all))]
            FiIcon::FiMap => FI_MAP,
            #[cfg(any(FiMapPin, icondata_include_all))]
            FiIcon::FiMapPin => FI_MAP_PIN,
            #[cfg(any(FiMaximize, icondata_include_all))]
            FiIcon::FiMaximize => FI_MAXIMIZE,
            #[cfg(any(FiMaximize2, icondata_include_all))]
            FiIcon::FiMaximize2 => FI_MAXIMIZE2,
            #[cfg(any(FiMeh, icondata_include_all))]
            FiIcon::FiMeh => FI_MEH,
            #[cfg(any(FiMenu, icondata_include_all))]
            FiIcon::FiMenu => FI_MENU,
            #[cfg(any(FiMessageCircle, icondata_include_all))]
            FiIcon::FiMessageCircle => FI_MESSAGE_CIRCLE,
            #[cfg(any(FiMessageSquare, icondata_include_all))]
            FiIcon::FiMessageSquare => FI_MESSAGE_SQUARE,
            #[cfg(any(FiMic, icondata_include_all))]
            FiIcon::FiMic => FI_MIC,
            #[cfg(any(FiMicOff, icondata_include_all))]
            FiIcon::FiMicOff => FI_MIC_OFF,
            #[cfg(any(FiMinimize, icondata_include_all))]
            FiIcon::FiMinimize => FI_MINIMIZE,
            #[cfg(any(FiMinimize2, icondata_include_all))]
            FiIcon::FiMinimize2 => FI_MINIMIZE2,
            #[cfg(any(FiMinus, icondata_include_all))]
            FiIcon::FiMinus => FI_MINUS,
            #[cfg(any(FiMinusCircle, icondata_include_all))]
            FiIcon::FiMinusCircle => FI_MINUS_CIRCLE,
            #[cfg(any(FiMinusSquare, icondata_include_all))]
            FiIcon::FiMinusSquare => FI_MINUS_SQUARE,
            #[cfg(any(FiMonitor, icondata_include_all))]
            FiIcon::FiMonitor => FI_MONITOR,
            #[cfg(any(FiMoon, icondata_include_all))]
            FiIcon::FiMoon => FI_MOON,
            #[cfg(any(FiMoreHorizontal, icondata_include_all))]
            FiIcon::FiMoreHorizontal => FI_MORE_HORIZONTAL,
            #[cfg(any(FiMoreVertical, icondata_include_all))]
            FiIcon::FiMoreVertical => FI_MORE_VERTICAL,
            #[cfg(any(FiMousePointer, icondata_include_all))]
            FiIcon::FiMousePointer => FI_MOUSE_POINTER,
            #[cfg(any(FiMove, icondata_include_all))]
            FiIcon::FiMove => FI_MOVE,
            #[cfg(any(FiMusic, icondata_include_all))]
            FiIcon::FiMusic => FI_MUSIC,
            #[cfg(any(FiNavigation, icondata_include_all))]
            FiIcon::FiNavigation => FI_NAVIGATION,
            #[cfg(any(FiNavigation2, icondata_include_all))]
            FiIcon::FiNavigation2 => FI_NAVIGATION2,
            #[cfg(any(FiOctagon, icondata_include_all))]
            FiIcon::FiOctagon => FI_OCTAGON,
            #[cfg(any(FiPackage, icondata_include_all))]
            FiIcon::FiPackage => FI_PACKAGE,
            #[cfg(any(FiPaperclip, icondata_include_all))]
            FiIcon::FiPaperclip => FI_PAPERCLIP,
            #[cfg(any(FiPause, icondata_include_all))]
            FiIcon::FiPause => FI_PAUSE,
            #[cfg(any(FiPauseCircle, icondata_include_all))]
            FiIcon::FiPauseCircle => FI_PAUSE_CIRCLE,
            #[cfg(any(FiPenTool, icondata_include_all))]
            FiIcon::FiPenTool => FI_PEN_TOOL,
            #[cfg(any(FiPercent, icondata_include_all))]
            FiIcon::FiPercent => FI_PERCENT,
            #[cfg(any(FiPhone, icondata_include_all))]
            FiIcon::FiPhone => FI_PHONE,
            #[cfg(any(FiPhoneCall, icondata_include_all))]
            FiIcon::FiPhoneCall => FI_PHONE_CALL,
            #[cfg(any(FiPhoneForwarded, icondata_include_all))]
            FiIcon::FiPhoneForwarded => FI_PHONE_FORWARDED,
            #[cfg(any(FiPhoneIncoming, icondata_include_all))]
            FiIcon::FiPhoneIncoming => FI_PHONE_INCOMING,
            #[cfg(any(FiPhoneMissed, icondata_include_all))]
            FiIcon::FiPhoneMissed => FI_PHONE_MISSED,
            #[cfg(any(FiPhoneOff, icondata_include_all))]
            FiIcon::FiPhoneOff => FI_PHONE_OFF,
            #[cfg(any(FiPhoneOutgoing, icondata_include_all))]
            FiIcon::FiPhoneOutgoing => FI_PHONE_OUTGOING,
            #[cfg(any(FiPieChart, icondata_include_all))]
            FiIcon::FiPieChart => FI_PIE_CHART,
            #[cfg(any(FiPlay, icondata_include_all))]
            FiIcon::FiPlay => FI_PLAY,
            #[cfg(any(FiPlayCircle, icondata_include_all))]
            FiIcon::FiPlayCircle => FI_PLAY_CIRCLE,
            #[cfg(any(FiPlus, icondata_include_all))]
            FiIcon::FiPlus => FI_PLUS,
            #[cfg(any(FiPlusCircle, icondata_include_all))]
            FiIcon::FiPlusCircle => FI_PLUS_CIRCLE,
            #[cfg(any(FiPlusSquare, icondata_include_all))]
            FiIcon::FiPlusSquare => FI_PLUS_SQUARE,
            #[cfg(any(FiPocket, icondata_include_all))]
            FiIcon::FiPocket => FI_POCKET,
            #[cfg(any(FiPower, icondata_include_all))]
            FiIcon::FiPower => FI_POWER,
            #[cfg(any(FiPrinter, icondata_include_all))]
            FiIcon::FiPrinter => FI_PRINTER,
            #[cfg(any(FiRadio, icondata_include_all))]
            FiIcon::FiRadio => FI_RADIO,
            #[cfg(any(FiRefreshCcw, icondata_include_all))]
            FiIcon::FiRefreshCcw => FI_REFRESH_CCW,
            #[cfg(any(FiRefreshCw, icondata_include_all))]
            FiIcon::FiRefreshCw => FI_REFRESH_CW,
            #[cfg(any(FiRepeat, icondata_include_all))]
            FiIcon::FiRepeat => FI_REPEAT,
            #[cfg(any(FiRewind, icondata_include_all))]
            FiIcon::FiRewind => FI_REWIND,
            #[cfg(any(FiRotateCcw, icondata_include_all))]
            FiIcon::FiRotateCcw => FI_ROTATE_CCW,
            #[cfg(any(FiRotateCw, icondata_include_all))]
            FiIcon::FiRotateCw => FI_ROTATE_CW,
            #[cfg(any(FiRss, icondata_include_all))]
            FiIcon::FiRss => FI_RSS,
            #[cfg(any(FiSave, icondata_include_all))]
            FiIcon::FiSave => FI_SAVE,
            #[cfg(any(FiScissors, icondata_include_all))]
            FiIcon::FiScissors => FI_SCISSORS,
            #[cfg(any(FiSearch, icondata_include_all))]
            FiIcon::FiSearch => FI_SEARCH,
            #[cfg(any(FiSend, icondata_include_all))]
            FiIcon::FiSend => FI_SEND,
            #[cfg(any(FiServer, icondata_include_all))]
            FiIcon::FiServer => FI_SERVER,
            #[cfg(any(FiSettings, icondata_include_all))]
            FiIcon::FiSettings => FI_SETTINGS,
            #[cfg(any(FiShare, icondata_include_all))]
            FiIcon::FiShare => FI_SHARE,
            #[cfg(any(FiShare2, icondata_include_all))]
            FiIcon::FiShare2 => FI_SHARE2,
            #[cfg(any(FiShield, icondata_include_all))]
            FiIcon::FiShield => FI_SHIELD,
            #[cfg(any(FiShieldOff, icondata_include_all))]
            FiIcon::FiShieldOff => FI_SHIELD_OFF,
            #[cfg(any(FiShoppingBag, icondata_include_all))]
            FiIcon::FiShoppingBag => FI_SHOPPING_BAG,
            #[cfg(any(FiShoppingCart, icondata_include_all))]
            FiIcon::FiShoppingCart => FI_SHOPPING_CART,
            #[cfg(any(FiShuffle, icondata_include_all))]
            FiIcon::FiShuffle => FI_SHUFFLE,
            #[cfg(any(FiSidebar, icondata_include_all))]
            FiIcon::FiSidebar => FI_SIDEBAR,
            #[cfg(any(FiSkipBack, icondata_include_all))]
            FiIcon::FiSkipBack => FI_SKIP_BACK,
            #[cfg(any(FiSkipForward, icondata_include_all))]
            FiIcon::FiSkipForward => FI_SKIP_FORWARD,
            #[cfg(any(FiSlack, icondata_include_all))]
            FiIcon::FiSlack => FI_SLACK,
            #[cfg(any(FiSlash, icondata_include_all))]
            FiIcon::FiSlash => FI_SLASH,
            #[cfg(any(FiSliders, icondata_include_all))]
            FiIcon::FiSliders => FI_SLIDERS,
            #[cfg(any(FiSmartphone, icondata_include_all))]
            FiIcon::FiSmartphone => FI_SMARTPHONE,
            #[cfg(any(FiSmile, icondata_include_all))]
            FiIcon::FiSmile => FI_SMILE,
            #[cfg(any(FiSpeaker, icondata_include_all))]
            FiIcon::FiSpeaker => FI_SPEAKER,
            #[cfg(any(FiSquare, icondata_include_all))]
            FiIcon::FiSquare => FI_SQUARE,
            #[cfg(any(FiStar, icondata_include_all))]
            FiIcon::FiStar => FI_STAR,
            #[cfg(any(FiStopCircle, icondata_include_all))]
            FiIcon::FiStopCircle => FI_STOP_CIRCLE,
            #[cfg(any(FiSun, icondata_include_all))]
            FiIcon::FiSun => FI_SUN,
            #[cfg(any(FiSunrise, icondata_include_all))]
            FiIcon::FiSunrise => FI_SUNRISE,
            #[cfg(any(FiSunset, icondata_include_all))]
            FiIcon::FiSunset => FI_SUNSET,
            #[cfg(any(FiTable, icondata_include_all))]
            FiIcon::FiTable => FI_TABLE,
            #[cfg(any(FiTablet, icondata_include_all))]
            FiIcon::FiTablet => FI_TABLET,
            #[cfg(any(FiTag, icondata_include_all))]
            FiIcon::FiTag => FI_TAG,
            #[cfg(any(FiTarget, icondata_include_all))]
            FiIcon::FiTarget => FI_TARGET,
            #[cfg(any(FiTerminal, icondata_include_all))]
            FiIcon::FiTerminal => FI_TERMINAL,
            #[cfg(any(FiThermometer, icondata_include_all))]
            FiIcon::FiThermometer => FI_THERMOMETER,
            #[cfg(any(FiThumbsDown, icondata_include_all))]
            FiIcon::FiThumbsDown => FI_THUMBS_DOWN,
            #[cfg(any(FiThumbsUp, icondata_include_all))]
            FiIcon::FiThumbsUp => FI_THUMBS_UP,
            #[cfg(any(FiToggleLeft, icondata_include_all))]
            FiIcon::FiToggleLeft => FI_TOGGLE_LEFT,
            #[cfg(any(FiToggleRight, icondata_include_all))]
            FiIcon::FiToggleRight => FI_TOGGLE_RIGHT,
            #[cfg(any(FiTool, icondata_include_all))]
            FiIcon::FiTool => FI_TOOL,
            #[cfg(any(FiTrash, icondata_include_all))]
            FiIcon::FiTrash => FI_TRASH,
            #[cfg(any(FiTrash2, icondata_include_all))]
            FiIcon::FiTrash2 => FI_TRASH2,
            #[cfg(any(FiTrello, icondata_include_all))]
            FiIcon::FiTrello => FI_TRELLO,
            #[cfg(any(FiTrendingDown, icondata_include_all))]
            FiIcon::FiTrendingDown => FI_TRENDING_DOWN,
            #[cfg(any(FiTrendingUp, icondata_include_all))]
            FiIcon::FiTrendingUp => FI_TRENDING_UP,
            #[cfg(any(FiTriangle, icondata_include_all))]
            FiIcon::FiTriangle => FI_TRIANGLE,
            #[cfg(any(FiTruck, icondata_include_all))]
            FiIcon::FiTruck => FI_TRUCK,
            #[cfg(any(FiTv, icondata_include_all))]
            FiIcon::FiTv => FI_TV,
            #[cfg(any(FiTwitch, icondata_include_all))]
            FiIcon::FiTwitch => FI_TWITCH,
            #[cfg(any(FiTwitter, icondata_include_all))]
            FiIcon::FiTwitter => FI_TWITTER,
            #[cfg(any(FiType, icondata_include_all))]
            FiIcon::FiType => FI_TYPE,
            #[cfg(any(FiUmbrella, icondata_include_all))]
            FiIcon::FiUmbrella => FI_UMBRELLA,
            #[cfg(any(FiUnderline, icondata_include_all))]
            FiIcon::FiUnderline => FI_UNDERLINE,
            #[cfg(any(FiUnlock, icondata_include_all))]
            FiIcon::FiUnlock => FI_UNLOCK,
            #[cfg(any(FiUpload, icondata_include_all))]
            FiIcon::FiUpload => FI_UPLOAD,
            #[cfg(any(FiUploadCloud, icondata_include_all))]
            FiIcon::FiUploadCloud => FI_UPLOAD_CLOUD,
            #[cfg(any(FiUser, icondata_include_all))]
            FiIcon::FiUser => FI_USER,
            #[cfg(any(FiUserCheck, icondata_include_all))]
            FiIcon::FiUserCheck => FI_USER_CHECK,
            #[cfg(any(FiUserMinus, icondata_include_all))]
            FiIcon::FiUserMinus => FI_USER_MINUS,
            #[cfg(any(FiUserPlus, icondata_include_all))]
            FiIcon::FiUserPlus => FI_USER_PLUS,
            #[cfg(any(FiUserX, icondata_include_all))]
            FiIcon::FiUserX => FI_USER_X,
            #[cfg(any(FiUsers, icondata_include_all))]
            FiIcon::FiUsers => FI_USERS,
            #[cfg(any(FiVideo, icondata_include_all))]
            FiIcon::FiVideo => FI_VIDEO,
            #[cfg(any(FiVideoOff, icondata_include_all))]
            FiIcon::FiVideoOff => FI_VIDEO_OFF,
            #[cfg(any(FiVoicemail, icondata_include_all))]
            FiIcon::FiVoicemail => FI_VOICEMAIL,
            #[cfg(any(FiVolume, icondata_include_all))]
            FiIcon::FiVolume => FI_VOLUME,
            #[cfg(any(FiVolume1, icondata_include_all))]
            FiIcon::FiVolume1 => FI_VOLUME1,
            #[cfg(any(FiVolume2, icondata_include_all))]
            FiIcon::FiVolume2 => FI_VOLUME2,
            #[cfg(any(FiVolumeX, icondata_include_all))]
            FiIcon::FiVolumeX => FI_VOLUME_X,
            #[cfg(any(FiWatch, icondata_include_all))]
            FiIcon::FiWatch => FI_WATCH,
            #[cfg(any(FiWifi, icondata_include_all))]
            FiIcon::FiWifi => FI_WIFI,
            #[cfg(any(FiWifiOff, icondata_include_all))]
            FiIcon::FiWifiOff => FI_WIFI_OFF,
            #[cfg(any(FiWind, icondata_include_all))]
            FiIcon::FiWind => FI_WIND,
            #[cfg(any(FiX, icondata_include_all))]
            FiIcon::FiX => FI_X,
            #[cfg(any(FiXCircle, icondata_include_all))]
            FiIcon::FiXCircle => FI_X_CIRCLE,
            #[cfg(any(FiXOctagon, icondata_include_all))]
            FiIcon::FiXOctagon => FI_X_OCTAGON,
            #[cfg(any(FiXSquare, icondata_include_all))]
            FiIcon::FiXSquare => FI_X_SQUARE,
            #[cfg(any(FiYoutube, icondata_include_all))]
            FiIcon::FiYoutube => FI_YOUTUBE,
            #[cfg(any(FiZap, icondata_include_all))]
            FiIcon::FiZap => FI_ZAP,
            #[cfg(any(FiZapOff, icondata_include_all))]
            FiIcon::FiZapOff => FI_ZAP_OFF,
            #[cfg(any(FiZoomIn, icondata_include_all))]
            FiIcon::FiZoomIn => FI_ZOOM_IN,
            #[cfg(any(FiZoomOut, icondata_include_all))]
            FiIcon::FiZoomOut => FI_ZOOM_OUT,
        }
    }
}