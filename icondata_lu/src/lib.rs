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
pub enum LuIcon {
    #[cfg(feature = "LuAccessibility")]
    LuAccessibility,
    #[cfg(feature = "LuActivity")]
    LuActivity,
    #[cfg(feature = "LuActivitySquare")]
    LuActivitySquare,
    #[cfg(feature = "LuAirVent")]
    LuAirVent,
    #[cfg(feature = "LuAirplay")]
    LuAirplay,
    #[cfg(feature = "LuAlarmCheck")]
    LuAlarmCheck,
    #[cfg(feature = "LuAlarmClock")]
    LuAlarmClock,
    #[cfg(feature = "LuAlarmClockOff")]
    LuAlarmClockOff,
    #[cfg(feature = "LuAlarmMinus")]
    LuAlarmMinus,
    #[cfg(feature = "LuAlarmPlus")]
    LuAlarmPlus,
    #[cfg(feature = "LuAlbum")]
    LuAlbum,
    #[cfg(feature = "LuAlertCircle")]
    LuAlertCircle,
    #[cfg(feature = "LuAlertOctagon")]
    LuAlertOctagon,
    #[cfg(feature = "LuAlertTriangle")]
    LuAlertTriangle,
    #[cfg(feature = "LuAlignCenter")]
    LuAlignCenter,
    #[cfg(feature = "LuAlignCenterHorizontal")]
    LuAlignCenterHorizontal,
    #[cfg(feature = "LuAlignCenterVertical")]
    LuAlignCenterVertical,
    #[cfg(feature = "LuAlignEndHorizontal")]
    LuAlignEndHorizontal,
    #[cfg(feature = "LuAlignEndVertical")]
    LuAlignEndVertical,
    #[cfg(feature = "LuAlignHorizontalDistributeCenter")]
    LuAlignHorizontalDistributeCenter,
    #[cfg(feature = "LuAlignHorizontalDistributeEnd")]
    LuAlignHorizontalDistributeEnd,
    #[cfg(feature = "LuAlignHorizontalDistributeStart")]
    LuAlignHorizontalDistributeStart,
    #[cfg(feature = "LuAlignHorizontalJustifyCenter")]
    LuAlignHorizontalJustifyCenter,
    #[cfg(feature = "LuAlignHorizontalJustifyEnd")]
    LuAlignHorizontalJustifyEnd,
    #[cfg(feature = "LuAlignHorizontalJustifyStart")]
    LuAlignHorizontalJustifyStart,
    #[cfg(feature = "LuAlignHorizontalSpaceAround")]
    LuAlignHorizontalSpaceAround,
    #[cfg(feature = "LuAlignHorizontalSpaceBetween")]
    LuAlignHorizontalSpaceBetween,
    #[cfg(feature = "LuAlignJustify")]
    LuAlignJustify,
    #[cfg(feature = "LuAlignLeft")]
    LuAlignLeft,
    #[cfg(feature = "LuAlignRight")]
    LuAlignRight,
    #[cfg(feature = "LuAlignStartHorizontal")]
    LuAlignStartHorizontal,
    #[cfg(feature = "LuAlignStartVertical")]
    LuAlignStartVertical,
    #[cfg(feature = "LuAlignVerticalDistributeCenter")]
    LuAlignVerticalDistributeCenter,
    #[cfg(feature = "LuAlignVerticalDistributeEnd")]
    LuAlignVerticalDistributeEnd,
    #[cfg(feature = "LuAlignVerticalDistributeStart")]
    LuAlignVerticalDistributeStart,
    #[cfg(feature = "LuAlignVerticalJustifyCenter")]
    LuAlignVerticalJustifyCenter,
    #[cfg(feature = "LuAlignVerticalJustifyEnd")]
    LuAlignVerticalJustifyEnd,
    #[cfg(feature = "LuAlignVerticalJustifyStart")]
    LuAlignVerticalJustifyStart,
    #[cfg(feature = "LuAlignVerticalSpaceAround")]
    LuAlignVerticalSpaceAround,
    #[cfg(feature = "LuAlignVerticalSpaceBetween")]
    LuAlignVerticalSpaceBetween,
    #[cfg(feature = "LuAmpersand")]
    LuAmpersand,
    #[cfg(feature = "LuAmpersands")]
    LuAmpersands,
    #[cfg(feature = "LuAnchor")]
    LuAnchor,
    #[cfg(feature = "LuAngry")]
    LuAngry,
    #[cfg(feature = "LuAnnoyed")]
    LuAnnoyed,
    #[cfg(feature = "LuAntenna")]
    LuAntenna,
    #[cfg(feature = "LuAperture")]
    LuAperture,
    #[cfg(feature = "LuAppWindow")]
    LuAppWindow,
    #[cfg(feature = "LuApple")]
    LuApple,
    #[cfg(feature = "LuArchive")]
    LuArchive,
    #[cfg(feature = "LuArchiveRestore")]
    LuArchiveRestore,
    #[cfg(feature = "LuAreaChart")]
    LuAreaChart,
    #[cfg(feature = "LuArmchair")]
    LuArmchair,
    #[cfg(feature = "LuArrowBigDown")]
    LuArrowBigDown,
    #[cfg(feature = "LuArrowBigDownDash")]
    LuArrowBigDownDash,
    #[cfg(feature = "LuArrowBigLeft")]
    LuArrowBigLeft,
    #[cfg(feature = "LuArrowBigLeftDash")]
    LuArrowBigLeftDash,
    #[cfg(feature = "LuArrowBigRight")]
    LuArrowBigRight,
    #[cfg(feature = "LuArrowBigRightDash")]
    LuArrowBigRightDash,
    #[cfg(feature = "LuArrowBigUp")]
    LuArrowBigUp,
    #[cfg(feature = "LuArrowBigUpDash")]
    LuArrowBigUpDash,
    #[cfg(feature = "LuArrowDown")]
    LuArrowDown,
    #[cfg(feature = "LuArrowDown01")]
    LuArrowDown01,
    #[cfg(feature = "LuArrowDown10")]
    LuArrowDown10,
    #[cfg(feature = "LuArrowDownAZ")]
    LuArrowDownAZ,
    #[cfg(feature = "LuArrowDownCircle")]
    LuArrowDownCircle,
    #[cfg(feature = "LuArrowDownFromLine")]
    LuArrowDownFromLine,
    #[cfg(feature = "LuArrowDownLeft")]
    LuArrowDownLeft,
    #[cfg(feature = "LuArrowDownLeftFromCircle")]
    LuArrowDownLeftFromCircle,
    #[cfg(feature = "LuArrowDownLeftSquare")]
    LuArrowDownLeftSquare,
    #[cfg(feature = "LuArrowDownNarrowWide")]
    LuArrowDownNarrowWide,
    #[cfg(feature = "LuArrowDownRight")]
    LuArrowDownRight,
    #[cfg(feature = "LuArrowDownRightFromCircle")]
    LuArrowDownRightFromCircle,
    #[cfg(feature = "LuArrowDownRightSquare")]
    LuArrowDownRightSquare,
    #[cfg(feature = "LuArrowDownSquare")]
    LuArrowDownSquare,
    #[cfg(feature = "LuArrowDownToDot")]
    LuArrowDownToDot,
    #[cfg(feature = "LuArrowDownToLine")]
    LuArrowDownToLine,
    #[cfg(feature = "LuArrowDownUp")]
    LuArrowDownUp,
    #[cfg(feature = "LuArrowDownWideNarrow")]
    LuArrowDownWideNarrow,
    #[cfg(feature = "LuArrowDownZA")]
    LuArrowDownZA,
    #[cfg(feature = "LuArrowLeft")]
    LuArrowLeft,
    #[cfg(feature = "LuArrowLeftCircle")]
    LuArrowLeftCircle,
    #[cfg(feature = "LuArrowLeftFromLine")]
    LuArrowLeftFromLine,
    #[cfg(feature = "LuArrowLeftRight")]
    LuArrowLeftRight,
    #[cfg(feature = "LuArrowLeftSquare")]
    LuArrowLeftSquare,
    #[cfg(feature = "LuArrowLeftToLine")]
    LuArrowLeftToLine,
    #[cfg(feature = "LuArrowRight")]
    LuArrowRight,
    #[cfg(feature = "LuArrowRightCircle")]
    LuArrowRightCircle,
    #[cfg(feature = "LuArrowRightFromLine")]
    LuArrowRightFromLine,
    #[cfg(feature = "LuArrowRightLeft")]
    LuArrowRightLeft,
    #[cfg(feature = "LuArrowRightSquare")]
    LuArrowRightSquare,
    #[cfg(feature = "LuArrowRightToLine")]
    LuArrowRightToLine,
    #[cfg(feature = "LuArrowUp")]
    LuArrowUp,
    #[cfg(feature = "LuArrowUp01")]
    LuArrowUp01,
    #[cfg(feature = "LuArrowUp10")]
    LuArrowUp10,
    #[cfg(feature = "LuArrowUpAZ")]
    LuArrowUpAZ,
    #[cfg(feature = "LuArrowUpCircle")]
    LuArrowUpCircle,
    #[cfg(feature = "LuArrowUpDown")]
    LuArrowUpDown,
    #[cfg(feature = "LuArrowUpFromDot")]
    LuArrowUpFromDot,
    #[cfg(feature = "LuArrowUpFromLine")]
    LuArrowUpFromLine,
    #[cfg(feature = "LuArrowUpLeft")]
    LuArrowUpLeft,
    #[cfg(feature = "LuArrowUpLeftFromCircle")]
    LuArrowUpLeftFromCircle,
    #[cfg(feature = "LuArrowUpLeftSquare")]
    LuArrowUpLeftSquare,
    #[cfg(feature = "LuArrowUpNarrowWide")]
    LuArrowUpNarrowWide,
    #[cfg(feature = "LuArrowUpRight")]
    LuArrowUpRight,
    #[cfg(feature = "LuArrowUpRightFromCircle")]
    LuArrowUpRightFromCircle,
    #[cfg(feature = "LuArrowUpRightSquare")]
    LuArrowUpRightSquare,
    #[cfg(feature = "LuArrowUpSquare")]
    LuArrowUpSquare,
    #[cfg(feature = "LuArrowUpToLine")]
    LuArrowUpToLine,
    #[cfg(feature = "LuArrowUpWideNarrow")]
    LuArrowUpWideNarrow,
    #[cfg(feature = "LuArrowUpZA")]
    LuArrowUpZA,
    #[cfg(feature = "LuArrowsUpFromLine")]
    LuArrowsUpFromLine,
    #[cfg(feature = "LuAsterisk")]
    LuAsterisk,
    #[cfg(feature = "LuAtSign")]
    LuAtSign,
    #[cfg(feature = "LuAtom")]
    LuAtom,
    #[cfg(feature = "LuAward")]
    LuAward,
    #[cfg(feature = "LuAxe")]
    LuAxe,
    #[cfg(feature = "LuAxis3d")]
    LuAxis3d,
    #[cfg(feature = "LuBaby")]
    LuBaby,
    #[cfg(feature = "LuBackpack")]
    LuBackpack,
    #[cfg(feature = "LuBadge")]
    LuBadge,
    #[cfg(feature = "LuBadgeAlert")]
    LuBadgeAlert,
    #[cfg(feature = "LuBadgeCheck")]
    LuBadgeCheck,
    #[cfg(feature = "LuBadgeDollarSign")]
    LuBadgeDollarSign,
    #[cfg(feature = "LuBadgeHelp")]
    LuBadgeHelp,
    #[cfg(feature = "LuBadgeInfo")]
    LuBadgeInfo,
    #[cfg(feature = "LuBadgeMinus")]
    LuBadgeMinus,
    #[cfg(feature = "LuBadgePercent")]
    LuBadgePercent,
    #[cfg(feature = "LuBadgePlus")]
    LuBadgePlus,
    #[cfg(feature = "LuBadgeX")]
    LuBadgeX,
    #[cfg(feature = "LuBaggageClaim")]
    LuBaggageClaim,
    #[cfg(feature = "LuBan")]
    LuBan,
    #[cfg(feature = "LuBanana")]
    LuBanana,
    #[cfg(feature = "LuBanknote")]
    LuBanknote,
    #[cfg(feature = "LuBarChart")]
    LuBarChart,
    #[cfg(feature = "LuBarChart2")]
    LuBarChart2,
    #[cfg(feature = "LuBarChart3")]
    LuBarChart3,
    #[cfg(feature = "LuBarChart4")]
    LuBarChart4,
    #[cfg(feature = "LuBarChartBig")]
    LuBarChartBig,
    #[cfg(feature = "LuBarChartHorizontal")]
    LuBarChartHorizontal,
    #[cfg(feature = "LuBarChartHorizontalBig")]
    LuBarChartHorizontalBig,
    #[cfg(feature = "LuBaseline")]
    LuBaseline,
    #[cfg(feature = "LuBath")]
    LuBath,
    #[cfg(feature = "LuBattery")]
    LuBattery,
    #[cfg(feature = "LuBatteryCharging")]
    LuBatteryCharging,
    #[cfg(feature = "LuBatteryFull")]
    LuBatteryFull,
    #[cfg(feature = "LuBatteryLow")]
    LuBatteryLow,
    #[cfg(feature = "LuBatteryMedium")]
    LuBatteryMedium,
    #[cfg(feature = "LuBatteryWarning")]
    LuBatteryWarning,
    #[cfg(feature = "LuBeaker")]
    LuBeaker,
    #[cfg(feature = "LuBean")]
    LuBean,
    #[cfg(feature = "LuBeanOff")]
    LuBeanOff,
    #[cfg(feature = "LuBed")]
    LuBed,
    #[cfg(feature = "LuBedDouble")]
    LuBedDouble,
    #[cfg(feature = "LuBedSingle")]
    LuBedSingle,
    #[cfg(feature = "LuBeef")]
    LuBeef,
    #[cfg(feature = "LuBeer")]
    LuBeer,
    #[cfg(feature = "LuBell")]
    LuBell,
    #[cfg(feature = "LuBellDot")]
    LuBellDot,
    #[cfg(feature = "LuBellMinus")]
    LuBellMinus,
    #[cfg(feature = "LuBellOff")]
    LuBellOff,
    #[cfg(feature = "LuBellPlus")]
    LuBellPlus,
    #[cfg(feature = "LuBellRing")]
    LuBellRing,
    #[cfg(feature = "LuBike")]
    LuBike,
    #[cfg(feature = "LuBinary")]
    LuBinary,
    #[cfg(feature = "LuBiohazard")]
    LuBiohazard,
    #[cfg(feature = "LuBird")]
    LuBird,
    #[cfg(feature = "LuBitcoin")]
    LuBitcoin,
    #[cfg(feature = "LuBlinds")]
    LuBlinds,
    #[cfg(feature = "LuBlocks")]
    LuBlocks,
    #[cfg(feature = "LuBluetooth")]
    LuBluetooth,
    #[cfg(feature = "LuBluetoothConnected")]
    LuBluetoothConnected,
    #[cfg(feature = "LuBluetoothOff")]
    LuBluetoothOff,
    #[cfg(feature = "LuBluetoothSearching")]
    LuBluetoothSearching,
    #[cfg(feature = "LuBold")]
    LuBold,
    #[cfg(feature = "LuBomb")]
    LuBomb,
    #[cfg(feature = "LuBone")]
    LuBone,
    #[cfg(feature = "LuBook")]
    LuBook,
    #[cfg(feature = "LuBookCopy")]
    LuBookCopy,
    #[cfg(feature = "LuBookDown")]
    LuBookDown,
    #[cfg(feature = "LuBookKey")]
    LuBookKey,
    #[cfg(feature = "LuBookLock")]
    LuBookLock,
    #[cfg(feature = "LuBookMarked")]
    LuBookMarked,
    #[cfg(feature = "LuBookMinus")]
    LuBookMinus,
    #[cfg(feature = "LuBookOpen")]
    LuBookOpen,
    #[cfg(feature = "LuBookOpenCheck")]
    LuBookOpenCheck,
    #[cfg(feature = "LuBookPlus")]
    LuBookPlus,
    #[cfg(feature = "LuBookTemplate")]
    LuBookTemplate,
    #[cfg(feature = "LuBookUp")]
    LuBookUp,
    #[cfg(feature = "LuBookUp2")]
    LuBookUp2,
    #[cfg(feature = "LuBookX")]
    LuBookX,
    #[cfg(feature = "LuBookmark")]
    LuBookmark,
    #[cfg(feature = "LuBookmarkMinus")]
    LuBookmarkMinus,
    #[cfg(feature = "LuBookmarkPlus")]
    LuBookmarkPlus,
    #[cfg(feature = "LuBoomBox")]
    LuBoomBox,
    #[cfg(feature = "LuBot")]
    LuBot,
    #[cfg(feature = "LuBox")]
    LuBox,
    #[cfg(feature = "LuBoxSelect")]
    LuBoxSelect,
    #[cfg(feature = "LuBoxes")]
    LuBoxes,
    #[cfg(feature = "LuBraces")]
    LuBraces,
    #[cfg(feature = "LuBrackets")]
    LuBrackets,
    #[cfg(feature = "LuBrain")]
    LuBrain,
    #[cfg(feature = "LuBrainCircuit")]
    LuBrainCircuit,
    #[cfg(feature = "LuBrainCog")]
    LuBrainCog,
    #[cfg(feature = "LuBriefcase")]
    LuBriefcase,
    #[cfg(feature = "LuBringToFront")]
    LuBringToFront,
    #[cfg(feature = "LuBrush")]
    LuBrush,
    #[cfg(feature = "LuBug")]
    LuBug,
    #[cfg(feature = "LuBuilding")]
    LuBuilding,
    #[cfg(feature = "LuBuilding2")]
    LuBuilding2,
    #[cfg(feature = "LuBus")]
    LuBus,
    #[cfg(feature = "LuBusFront")]
    LuBusFront,
    #[cfg(feature = "LuCable")]
    LuCable,
    #[cfg(feature = "LuCableCar")]
    LuCableCar,
    #[cfg(feature = "LuCake")]
    LuCake,
    #[cfg(feature = "LuCakeSlice")]
    LuCakeSlice,
    #[cfg(feature = "LuCalculator")]
    LuCalculator,
    #[cfg(feature = "LuCalendar")]
    LuCalendar,
    #[cfg(feature = "LuCalendarCheck")]
    LuCalendarCheck,
    #[cfg(feature = "LuCalendarCheck2")]
    LuCalendarCheck2,
    #[cfg(feature = "LuCalendarClock")]
    LuCalendarClock,
    #[cfg(feature = "LuCalendarDays")]
    LuCalendarDays,
    #[cfg(feature = "LuCalendarHeart")]
    LuCalendarHeart,
    #[cfg(feature = "LuCalendarMinus")]
    LuCalendarMinus,
    #[cfg(feature = "LuCalendarOff")]
    LuCalendarOff,
    #[cfg(feature = "LuCalendarPlus")]
    LuCalendarPlus,
    #[cfg(feature = "LuCalendarRange")]
    LuCalendarRange,
    #[cfg(feature = "LuCalendarSearch")]
    LuCalendarSearch,
    #[cfg(feature = "LuCalendarX")]
    LuCalendarX,
    #[cfg(feature = "LuCalendarX2")]
    LuCalendarX2,
    #[cfg(feature = "LuCamera")]
    LuCamera,
    #[cfg(feature = "LuCameraOff")]
    LuCameraOff,
    #[cfg(feature = "LuCandlestickChart")]
    LuCandlestickChart,
    #[cfg(feature = "LuCandy")]
    LuCandy,
    #[cfg(feature = "LuCandyCane")]
    LuCandyCane,
    #[cfg(feature = "LuCandyOff")]
    LuCandyOff,
    #[cfg(feature = "LuCar")]
    LuCar,
    #[cfg(feature = "LuCarFront")]
    LuCarFront,
    #[cfg(feature = "LuCarTaxiFront")]
    LuCarTaxiFront,
    #[cfg(feature = "LuCarrot")]
    LuCarrot,
    #[cfg(feature = "LuCaseLower")]
    LuCaseLower,
    #[cfg(feature = "LuCaseSensitive")]
    LuCaseSensitive,
    #[cfg(feature = "LuCaseUpper")]
    LuCaseUpper,
    #[cfg(feature = "LuCassetteTape")]
    LuCassetteTape,
    #[cfg(feature = "LuCast")]
    LuCast,
    #[cfg(feature = "LuCastle")]
    LuCastle,
    #[cfg(feature = "LuCat")]
    LuCat,
    #[cfg(feature = "LuCheck")]
    LuCheck,
    #[cfg(feature = "LuCheckCheck")]
    LuCheckCheck,
    #[cfg(feature = "LuCheckCircle")]
    LuCheckCircle,
    #[cfg(feature = "LuCheckCircle2")]
    LuCheckCircle2,
    #[cfg(feature = "LuCheckSquare")]
    LuCheckSquare,
    #[cfg(feature = "LuChefHat")]
    LuChefHat,
    #[cfg(feature = "LuCherry")]
    LuCherry,
    #[cfg(feature = "LuChevronDown")]
    LuChevronDown,
    #[cfg(feature = "LuChevronDownCircle")]
    LuChevronDownCircle,
    #[cfg(feature = "LuChevronDownSquare")]
    LuChevronDownSquare,
    #[cfg(feature = "LuChevronFirst")]
    LuChevronFirst,
    #[cfg(feature = "LuChevronLast")]
    LuChevronLast,
    #[cfg(feature = "LuChevronLeft")]
    LuChevronLeft,
    #[cfg(feature = "LuChevronLeftCircle")]
    LuChevronLeftCircle,
    #[cfg(feature = "LuChevronLeftSquare")]
    LuChevronLeftSquare,
    #[cfg(feature = "LuChevronRight")]
    LuChevronRight,
    #[cfg(feature = "LuChevronRightCircle")]
    LuChevronRightCircle,
    #[cfg(feature = "LuChevronRightSquare")]
    LuChevronRightSquare,
    #[cfg(feature = "LuChevronUp")]
    LuChevronUp,
    #[cfg(feature = "LuChevronUpCircle")]
    LuChevronUpCircle,
    #[cfg(feature = "LuChevronUpSquare")]
    LuChevronUpSquare,
    #[cfg(feature = "LuChevronsDown")]
    LuChevronsDown,
    #[cfg(feature = "LuChevronsDownUp")]
    LuChevronsDownUp,
    #[cfg(feature = "LuChevronsLeft")]
    LuChevronsLeft,
    #[cfg(feature = "LuChevronsLeftRight")]
    LuChevronsLeftRight,
    #[cfg(feature = "LuChevronsRight")]
    LuChevronsRight,
    #[cfg(feature = "LuChevronsRightLeft")]
    LuChevronsRightLeft,
    #[cfg(feature = "LuChevronsUp")]
    LuChevronsUp,
    #[cfg(feature = "LuChevronsUpDown")]
    LuChevronsUpDown,
    #[cfg(feature = "LuChrome")]
    LuChrome,
    #[cfg(feature = "LuChurch")]
    LuChurch,
    #[cfg(feature = "LuCigarette")]
    LuCigarette,
    #[cfg(feature = "LuCigaretteOff")]
    LuCigaretteOff,
    #[cfg(feature = "LuCircle")]
    LuCircle,
    #[cfg(feature = "LuCircleDashed")]
    LuCircleDashed,
    #[cfg(feature = "LuCircleDollarSign")]
    LuCircleDollarSign,
    #[cfg(feature = "LuCircleDot")]
    LuCircleDot,
    #[cfg(feature = "LuCircleDotDashed")]
    LuCircleDotDashed,
    #[cfg(feature = "LuCircleEllipsis")]
    LuCircleEllipsis,
    #[cfg(feature = "LuCircleEqual")]
    LuCircleEqual,
    #[cfg(feature = "LuCircleOff")]
    LuCircleOff,
    #[cfg(feature = "LuCircleSlash")]
    LuCircleSlash,
    #[cfg(feature = "LuCircleSlash2")]
    LuCircleSlash2,
    #[cfg(feature = "LuCircuitBoard")]
    LuCircuitBoard,
    #[cfg(feature = "LuCitrus")]
    LuCitrus,
    #[cfg(feature = "LuClapperboard")]
    LuClapperboard,
    #[cfg(feature = "LuClipboard")]
    LuClipboard,
    #[cfg(feature = "LuClipboardCheck")]
    LuClipboardCheck,
    #[cfg(feature = "LuClipboardCopy")]
    LuClipboardCopy,
    #[cfg(feature = "LuClipboardEdit")]
    LuClipboardEdit,
    #[cfg(feature = "LuClipboardList")]
    LuClipboardList,
    #[cfg(feature = "LuClipboardPaste")]
    LuClipboardPaste,
    #[cfg(feature = "LuClipboardSignature")]
    LuClipboardSignature,
    #[cfg(feature = "LuClipboardType")]
    LuClipboardType,
    #[cfg(feature = "LuClipboardX")]
    LuClipboardX,
    #[cfg(feature = "LuClock")]
    LuClock,
    #[cfg(feature = "LuClock1")]
    LuClock1,
    #[cfg(feature = "LuClock10")]
    LuClock10,
    #[cfg(feature = "LuClock11")]
    LuClock11,
    #[cfg(feature = "LuClock12")]
    LuClock12,
    #[cfg(feature = "LuClock2")]
    LuClock2,
    #[cfg(feature = "LuClock3")]
    LuClock3,
    #[cfg(feature = "LuClock4")]
    LuClock4,
    #[cfg(feature = "LuClock5")]
    LuClock5,
    #[cfg(feature = "LuClock6")]
    LuClock6,
    #[cfg(feature = "LuClock7")]
    LuClock7,
    #[cfg(feature = "LuClock8")]
    LuClock8,
    #[cfg(feature = "LuClock9")]
    LuClock9,
    #[cfg(feature = "LuCloud")]
    LuCloud,
    #[cfg(feature = "LuCloudCog")]
    LuCloudCog,
    #[cfg(feature = "LuCloudDrizzle")]
    LuCloudDrizzle,
    #[cfg(feature = "LuCloudFog")]
    LuCloudFog,
    #[cfg(feature = "LuCloudHail")]
    LuCloudHail,
    #[cfg(feature = "LuCloudLightning")]
    LuCloudLightning,
    #[cfg(feature = "LuCloudMoon")]
    LuCloudMoon,
    #[cfg(feature = "LuCloudMoonRain")]
    LuCloudMoonRain,
    #[cfg(feature = "LuCloudOff")]
    LuCloudOff,
    #[cfg(feature = "LuCloudRain")]
    LuCloudRain,
    #[cfg(feature = "LuCloudRainWind")]
    LuCloudRainWind,
    #[cfg(feature = "LuCloudSnow")]
    LuCloudSnow,
    #[cfg(feature = "LuCloudSun")]
    LuCloudSun,
    #[cfg(feature = "LuCloudSunRain")]
    LuCloudSunRain,
    #[cfg(feature = "LuCloudy")]
    LuCloudy,
    #[cfg(feature = "LuClover")]
    LuClover,
    #[cfg(feature = "LuClub")]
    LuClub,
    #[cfg(feature = "LuCode")]
    LuCode,
    #[cfg(feature = "LuCode2")]
    LuCode2,
    #[cfg(feature = "LuCodepen")]
    LuCodepen,
    #[cfg(feature = "LuCodesandbox")]
    LuCodesandbox,
    #[cfg(feature = "LuCoffee")]
    LuCoffee,
    #[cfg(feature = "LuCog")]
    LuCog,
    #[cfg(feature = "LuCoins")]
    LuCoins,
    #[cfg(feature = "LuColumns")]
    LuColumns,
    #[cfg(feature = "LuCombine")]
    LuCombine,
    #[cfg(feature = "LuCommand")]
    LuCommand,
    #[cfg(feature = "LuCompass")]
    LuCompass,
    #[cfg(feature = "LuComponent")]
    LuComponent,
    #[cfg(feature = "LuComputer")]
    LuComputer,
    #[cfg(feature = "LuConciergeBell")]
    LuConciergeBell,
    #[cfg(feature = "LuConstruction")]
    LuConstruction,
    #[cfg(feature = "LuContact")]
    LuContact,
    #[cfg(feature = "LuContact2")]
    LuContact2,
    #[cfg(feature = "LuContainer")]
    LuContainer,
    #[cfg(feature = "LuContrast")]
    LuContrast,
    #[cfg(feature = "LuCookie")]
    LuCookie,
    #[cfg(feature = "LuCopy")]
    LuCopy,
    #[cfg(feature = "LuCopyCheck")]
    LuCopyCheck,
    #[cfg(feature = "LuCopyMinus")]
    LuCopyMinus,
    #[cfg(feature = "LuCopyPlus")]
    LuCopyPlus,
    #[cfg(feature = "LuCopySlash")]
    LuCopySlash,
    #[cfg(feature = "LuCopyX")]
    LuCopyX,
    #[cfg(feature = "LuCopyleft")]
    LuCopyleft,
    #[cfg(feature = "LuCopyright")]
    LuCopyright,
    #[cfg(feature = "LuCornerDownLeft")]
    LuCornerDownLeft,
    #[cfg(feature = "LuCornerDownRight")]
    LuCornerDownRight,
    #[cfg(feature = "LuCornerLeftDown")]
    LuCornerLeftDown,
    #[cfg(feature = "LuCornerLeftUp")]
    LuCornerLeftUp,
    #[cfg(feature = "LuCornerRightDown")]
    LuCornerRightDown,
    #[cfg(feature = "LuCornerRightUp")]
    LuCornerRightUp,
    #[cfg(feature = "LuCornerUpLeft")]
    LuCornerUpLeft,
    #[cfg(feature = "LuCornerUpRight")]
    LuCornerUpRight,
    #[cfg(feature = "LuCpu")]
    LuCpu,
    #[cfg(feature = "LuCreativeCommons")]
    LuCreativeCommons,
    #[cfg(feature = "LuCreditCard")]
    LuCreditCard,
    #[cfg(feature = "LuCroissant")]
    LuCroissant,
    #[cfg(feature = "LuCrop")]
    LuCrop,
    #[cfg(feature = "LuCross")]
    LuCross,
    #[cfg(feature = "LuCrosshair")]
    LuCrosshair,
    #[cfg(feature = "LuCrown")]
    LuCrown,
    #[cfg(feature = "LuCupSoda")]
    LuCupSoda,
    #[cfg(feature = "LuCurrency")]
    LuCurrency,
    #[cfg(feature = "LuDatabase")]
    LuDatabase,
    #[cfg(feature = "LuDatabaseBackup")]
    LuDatabaseBackup,
    #[cfg(feature = "LuDelete")]
    LuDelete,
    #[cfg(feature = "LuDessert")]
    LuDessert,
    #[cfg(feature = "LuDiamond")]
    LuDiamond,
    #[cfg(feature = "LuDice1")]
    LuDice1,
    #[cfg(feature = "LuDice2")]
    LuDice2,
    #[cfg(feature = "LuDice3")]
    LuDice3,
    #[cfg(feature = "LuDice4")]
    LuDice4,
    #[cfg(feature = "LuDice5")]
    LuDice5,
    #[cfg(feature = "LuDice6")]
    LuDice6,
    #[cfg(feature = "LuDices")]
    LuDices,
    #[cfg(feature = "LuDiff")]
    LuDiff,
    #[cfg(feature = "LuDisc")]
    LuDisc,
    #[cfg(feature = "LuDisc2")]
    LuDisc2,
    #[cfg(feature = "LuDisc3")]
    LuDisc3,
    #[cfg(feature = "LuDivide")]
    LuDivide,
    #[cfg(feature = "LuDivideCircle")]
    LuDivideCircle,
    #[cfg(feature = "LuDivideSquare")]
    LuDivideSquare,
    #[cfg(feature = "LuDna")]
    LuDna,
    #[cfg(feature = "LuDnaOff")]
    LuDnaOff,
    #[cfg(feature = "LuDog")]
    LuDog,
    #[cfg(feature = "LuDollarSign")]
    LuDollarSign,
    #[cfg(feature = "LuDonut")]
    LuDonut,
    #[cfg(feature = "LuDoorClosed")]
    LuDoorClosed,
    #[cfg(feature = "LuDoorOpen")]
    LuDoorOpen,
    #[cfg(feature = "LuDot")]
    LuDot,
    #[cfg(feature = "LuDownload")]
    LuDownload,
    #[cfg(feature = "LuDownloadCloud")]
    LuDownloadCloud,
    #[cfg(feature = "LuDribbble")]
    LuDribbble,
    #[cfg(feature = "LuDroplet")]
    LuDroplet,
    #[cfg(feature = "LuDroplets")]
    LuDroplets,
    #[cfg(feature = "LuDrumstick")]
    LuDrumstick,
    #[cfg(feature = "LuDumbbell")]
    LuDumbbell,
    #[cfg(feature = "LuEar")]
    LuEar,
    #[cfg(feature = "LuEarOff")]
    LuEarOff,
    #[cfg(feature = "LuEgg")]
    LuEgg,
    #[cfg(feature = "LuEggFried")]
    LuEggFried,
    #[cfg(feature = "LuEggOff")]
    LuEggOff,
    #[cfg(feature = "LuEqual")]
    LuEqual,
    #[cfg(feature = "LuEqualNot")]
    LuEqualNot,
    #[cfg(feature = "LuEraser")]
    LuEraser,
    #[cfg(feature = "LuEuro")]
    LuEuro,
    #[cfg(feature = "LuExpand")]
    LuExpand,
    #[cfg(feature = "LuExternalLink")]
    LuExternalLink,
    #[cfg(feature = "LuEye")]
    LuEye,
    #[cfg(feature = "LuEyeOff")]
    LuEyeOff,
    #[cfg(feature = "LuFacebook")]
    LuFacebook,
    #[cfg(feature = "LuFactory")]
    LuFactory,
    #[cfg(feature = "LuFan")]
    LuFan,
    #[cfg(feature = "LuFastForward")]
    LuFastForward,
    #[cfg(feature = "LuFeather")]
    LuFeather,
    #[cfg(feature = "LuFerrisWheel")]
    LuFerrisWheel,
    #[cfg(feature = "LuFigma")]
    LuFigma,
    #[cfg(feature = "LuFile")]
    LuFile,
    #[cfg(feature = "LuFileArchive")]
    LuFileArchive,
    #[cfg(feature = "LuFileAudio")]
    LuFileAudio,
    #[cfg(feature = "LuFileAudio2")]
    LuFileAudio2,
    #[cfg(feature = "LuFileAxis3d")]
    LuFileAxis3d,
    #[cfg(feature = "LuFileBadge")]
    LuFileBadge,
    #[cfg(feature = "LuFileBadge2")]
    LuFileBadge2,
    #[cfg(feature = "LuFileBarChart")]
    LuFileBarChart,
    #[cfg(feature = "LuFileBarChart2")]
    LuFileBarChart2,
    #[cfg(feature = "LuFileBox")]
    LuFileBox,
    #[cfg(feature = "LuFileCheck")]
    LuFileCheck,
    #[cfg(feature = "LuFileCheck2")]
    LuFileCheck2,
    #[cfg(feature = "LuFileClock")]
    LuFileClock,
    #[cfg(feature = "LuFileCode")]
    LuFileCode,
    #[cfg(feature = "LuFileCode2")]
    LuFileCode2,
    #[cfg(feature = "LuFileCog")]
    LuFileCog,
    #[cfg(feature = "LuFileCog2")]
    LuFileCog2,
    #[cfg(feature = "LuFileDiff")]
    LuFileDiff,
    #[cfg(feature = "LuFileDigit")]
    LuFileDigit,
    #[cfg(feature = "LuFileDown")]
    LuFileDown,
    #[cfg(feature = "LuFileEdit")]
    LuFileEdit,
    #[cfg(feature = "LuFileHeart")]
    LuFileHeart,
    #[cfg(feature = "LuFileImage")]
    LuFileImage,
    #[cfg(feature = "LuFileInput")]
    LuFileInput,
    #[cfg(feature = "LuFileJson")]
    LuFileJson,
    #[cfg(feature = "LuFileJson2")]
    LuFileJson2,
    #[cfg(feature = "LuFileKey")]
    LuFileKey,
    #[cfg(feature = "LuFileKey2")]
    LuFileKey2,
    #[cfg(feature = "LuFileLineChart")]
    LuFileLineChart,
    #[cfg(feature = "LuFileLock")]
    LuFileLock,
    #[cfg(feature = "LuFileLock2")]
    LuFileLock2,
    #[cfg(feature = "LuFileMinus")]
    LuFileMinus,
    #[cfg(feature = "LuFileMinus2")]
    LuFileMinus2,
    #[cfg(feature = "LuFileOutput")]
    LuFileOutput,
    #[cfg(feature = "LuFilePieChart")]
    LuFilePieChart,
    #[cfg(feature = "LuFilePlus")]
    LuFilePlus,
    #[cfg(feature = "LuFilePlus2")]
    LuFilePlus2,
    #[cfg(feature = "LuFileQuestion")]
    LuFileQuestion,
    #[cfg(feature = "LuFileScan")]
    LuFileScan,
    #[cfg(feature = "LuFileSearch")]
    LuFileSearch,
    #[cfg(feature = "LuFileSearch2")]
    LuFileSearch2,
    #[cfg(feature = "LuFileSignature")]
    LuFileSignature,
    #[cfg(feature = "LuFileSpreadsheet")]
    LuFileSpreadsheet,
    #[cfg(feature = "LuFileStack")]
    LuFileStack,
    #[cfg(feature = "LuFileSymlink")]
    LuFileSymlink,
    #[cfg(feature = "LuFileTerminal")]
    LuFileTerminal,
    #[cfg(feature = "LuFileText")]
    LuFileText,
    #[cfg(feature = "LuFileType")]
    LuFileType,
    #[cfg(feature = "LuFileType2")]
    LuFileType2,
    #[cfg(feature = "LuFileUp")]
    LuFileUp,
    #[cfg(feature = "LuFileVideo")]
    LuFileVideo,
    #[cfg(feature = "LuFileVideo2")]
    LuFileVideo2,
    #[cfg(feature = "LuFileVolume")]
    LuFileVolume,
    #[cfg(feature = "LuFileVolume2")]
    LuFileVolume2,
    #[cfg(feature = "LuFileWarning")]
    LuFileWarning,
    #[cfg(feature = "LuFileX")]
    LuFileX,
    #[cfg(feature = "LuFileX2")]
    LuFileX2,
    #[cfg(feature = "LuFiles")]
    LuFiles,
    #[cfg(feature = "LuFilm")]
    LuFilm,
    #[cfg(feature = "LuFilter")]
    LuFilter,
    #[cfg(feature = "LuFilterX")]
    LuFilterX,
    #[cfg(feature = "LuFingerprint")]
    LuFingerprint,
    #[cfg(feature = "LuFish")]
    LuFish,
    #[cfg(feature = "LuFishOff")]
    LuFishOff,
    #[cfg(feature = "LuFishSymbol")]
    LuFishSymbol,
    #[cfg(feature = "LuFlag")]
    LuFlag,
    #[cfg(feature = "LuFlagOff")]
    LuFlagOff,
    #[cfg(feature = "LuFlagTriangleLeft")]
    LuFlagTriangleLeft,
    #[cfg(feature = "LuFlagTriangleRight")]
    LuFlagTriangleRight,
    #[cfg(feature = "LuFlame")]
    LuFlame,
    #[cfg(feature = "LuFlashlight")]
    LuFlashlight,
    #[cfg(feature = "LuFlashlightOff")]
    LuFlashlightOff,
    #[cfg(feature = "LuFlaskConical")]
    LuFlaskConical,
    #[cfg(feature = "LuFlaskConicalOff")]
    LuFlaskConicalOff,
    #[cfg(feature = "LuFlaskRound")]
    LuFlaskRound,
    #[cfg(feature = "LuFlipHorizontal")]
    LuFlipHorizontal,
    #[cfg(feature = "LuFlipHorizontal2")]
    LuFlipHorizontal2,
    #[cfg(feature = "LuFlipVertical")]
    LuFlipVertical,
    #[cfg(feature = "LuFlipVertical2")]
    LuFlipVertical2,
    #[cfg(feature = "LuFlower")]
    LuFlower,
    #[cfg(feature = "LuFlower2")]
    LuFlower2,
    #[cfg(feature = "LuFocus")]
    LuFocus,
    #[cfg(feature = "LuFoldHorizontal")]
    LuFoldHorizontal,
    #[cfg(feature = "LuFoldVertical")]
    LuFoldVertical,
    #[cfg(feature = "LuFolder")]
    LuFolder,
    #[cfg(feature = "LuFolderArchive")]
    LuFolderArchive,
    #[cfg(feature = "LuFolderCheck")]
    LuFolderCheck,
    #[cfg(feature = "LuFolderClock")]
    LuFolderClock,
    #[cfg(feature = "LuFolderClosed")]
    LuFolderClosed,
    #[cfg(feature = "LuFolderCog")]
    LuFolderCog,
    #[cfg(feature = "LuFolderCog2")]
    LuFolderCog2,
    #[cfg(feature = "LuFolderDot")]
    LuFolderDot,
    #[cfg(feature = "LuFolderDown")]
    LuFolderDown,
    #[cfg(feature = "LuFolderEdit")]
    LuFolderEdit,
    #[cfg(feature = "LuFolderGit")]
    LuFolderGit,
    #[cfg(feature = "LuFolderGit2")]
    LuFolderGit2,
    #[cfg(feature = "LuFolderHeart")]
    LuFolderHeart,
    #[cfg(feature = "LuFolderInput")]
    LuFolderInput,
    #[cfg(feature = "LuFolderKanban")]
    LuFolderKanban,
    #[cfg(feature = "LuFolderKey")]
    LuFolderKey,
    #[cfg(feature = "LuFolderLock")]
    LuFolderLock,
    #[cfg(feature = "LuFolderMinus")]
    LuFolderMinus,
    #[cfg(feature = "LuFolderOpen")]
    LuFolderOpen,
    #[cfg(feature = "LuFolderOpenDot")]
    LuFolderOpenDot,
    #[cfg(feature = "LuFolderOutput")]
    LuFolderOutput,
    #[cfg(feature = "LuFolderPlus")]
    LuFolderPlus,
    #[cfg(feature = "LuFolderRoot")]
    LuFolderRoot,
    #[cfg(feature = "LuFolderSearch")]
    LuFolderSearch,
    #[cfg(feature = "LuFolderSearch2")]
    LuFolderSearch2,
    #[cfg(feature = "LuFolderSymlink")]
    LuFolderSymlink,
    #[cfg(feature = "LuFolderSync")]
    LuFolderSync,
    #[cfg(feature = "LuFolderTree")]
    LuFolderTree,
    #[cfg(feature = "LuFolderUp")]
    LuFolderUp,
    #[cfg(feature = "LuFolderX")]
    LuFolderX,
    #[cfg(feature = "LuFolders")]
    LuFolders,
    #[cfg(feature = "LuFootprints")]
    LuFootprints,
    #[cfg(feature = "LuForklift")]
    LuForklift,
    #[cfg(feature = "LuFormInput")]
    LuFormInput,
    #[cfg(feature = "LuForward")]
    LuForward,
    #[cfg(feature = "LuFrame")]
    LuFrame,
    #[cfg(feature = "LuFramer")]
    LuFramer,
    #[cfg(feature = "LuFrown")]
    LuFrown,
    #[cfg(feature = "LuFuel")]
    LuFuel,
    #[cfg(feature = "LuFunctionSquare")]
    LuFunctionSquare,
    #[cfg(feature = "LuGalleryHorizontal")]
    LuGalleryHorizontal,
    #[cfg(feature = "LuGalleryHorizontalEnd")]
    LuGalleryHorizontalEnd,
    #[cfg(feature = "LuGalleryThumbnails")]
    LuGalleryThumbnails,
    #[cfg(feature = "LuGalleryVertical")]
    LuGalleryVertical,
    #[cfg(feature = "LuGalleryVerticalEnd")]
    LuGalleryVerticalEnd,
    #[cfg(feature = "LuGamepad")]
    LuGamepad,
    #[cfg(feature = "LuGamepad2")]
    LuGamepad2,
    #[cfg(feature = "LuGanttChart")]
    LuGanttChart,
    #[cfg(feature = "LuGanttChartSquare")]
    LuGanttChartSquare,
    #[cfg(feature = "LuGauge")]
    LuGauge,
    #[cfg(feature = "LuGaugeCircle")]
    LuGaugeCircle,
    #[cfg(feature = "LuGavel")]
    LuGavel,
    #[cfg(feature = "LuGem")]
    LuGem,
    #[cfg(feature = "LuGhost")]
    LuGhost,
    #[cfg(feature = "LuGift")]
    LuGift,
    #[cfg(feature = "LuGitBranch")]
    LuGitBranch,
    #[cfg(feature = "LuGitBranchPlus")]
    LuGitBranchPlus,
    #[cfg(feature = "LuGitCommit")]
    LuGitCommit,
    #[cfg(feature = "LuGitCompare")]
    LuGitCompare,
    #[cfg(feature = "LuGitFork")]
    LuGitFork,
    #[cfg(feature = "LuGitMerge")]
    LuGitMerge,
    #[cfg(feature = "LuGitPullRequest")]
    LuGitPullRequest,
    #[cfg(feature = "LuGitPullRequestClosed")]
    LuGitPullRequestClosed,
    #[cfg(feature = "LuGitPullRequestDraft")]
    LuGitPullRequestDraft,
    #[cfg(feature = "LuGithub")]
    LuGithub,
    #[cfg(feature = "LuGitlab")]
    LuGitlab,
    #[cfg(feature = "LuGlassWater")]
    LuGlassWater,
    #[cfg(feature = "LuGlasses")]
    LuGlasses,
    #[cfg(feature = "LuGlobe")]
    LuGlobe,
    #[cfg(feature = "LuGlobe2")]
    LuGlobe2,
    #[cfg(feature = "LuGoal")]
    LuGoal,
    #[cfg(feature = "LuGrab")]
    LuGrab,
    #[cfg(feature = "LuGraduationCap")]
    LuGraduationCap,
    #[cfg(feature = "LuGrape")]
    LuGrape,
    #[cfg(feature = "LuGrid2x2")]
    LuGrid2x2,
    #[cfg(feature = "LuGrid3x3")]
    LuGrid3x3,
    #[cfg(feature = "LuGrip")]
    LuGrip,
    #[cfg(feature = "LuGripHorizontal")]
    LuGripHorizontal,
    #[cfg(feature = "LuGripVertical")]
    LuGripVertical,
    #[cfg(feature = "LuGroup")]
    LuGroup,
    #[cfg(feature = "LuHammer")]
    LuHammer,
    #[cfg(feature = "LuHand")]
    LuHand,
    #[cfg(feature = "LuHandMetal")]
    LuHandMetal,
    #[cfg(feature = "LuHardDrive")]
    LuHardDrive,
    #[cfg(feature = "LuHardDriveDownload")]
    LuHardDriveDownload,
    #[cfg(feature = "LuHardDriveUpload")]
    LuHardDriveUpload,
    #[cfg(feature = "LuHardHat")]
    LuHardHat,
    #[cfg(feature = "LuHash")]
    LuHash,
    #[cfg(feature = "LuHaze")]
    LuHaze,
    #[cfg(feature = "LuHdmiPort")]
    LuHdmiPort,
    #[cfg(feature = "LuHeading")]
    LuHeading,
    #[cfg(feature = "LuHeading1")]
    LuHeading1,
    #[cfg(feature = "LuHeading2")]
    LuHeading2,
    #[cfg(feature = "LuHeading3")]
    LuHeading3,
    #[cfg(feature = "LuHeading4")]
    LuHeading4,
    #[cfg(feature = "LuHeading5")]
    LuHeading5,
    #[cfg(feature = "LuHeading6")]
    LuHeading6,
    #[cfg(feature = "LuHeadphones")]
    LuHeadphones,
    #[cfg(feature = "LuHeart")]
    LuHeart,
    #[cfg(feature = "LuHeartCrack")]
    LuHeartCrack,
    #[cfg(feature = "LuHeartHandshake")]
    LuHeartHandshake,
    #[cfg(feature = "LuHeartOff")]
    LuHeartOff,
    #[cfg(feature = "LuHeartPulse")]
    LuHeartPulse,
    #[cfg(feature = "LuHelpCircle")]
    LuHelpCircle,
    #[cfg(feature = "LuHelpingHand")]
    LuHelpingHand,
    #[cfg(feature = "LuHexagon")]
    LuHexagon,
    #[cfg(feature = "LuHighlighter")]
    LuHighlighter,
    #[cfg(feature = "LuHistory")]
    LuHistory,
    #[cfg(feature = "LuHome")]
    LuHome,
    #[cfg(feature = "LuHop")]
    LuHop,
    #[cfg(feature = "LuHopOff")]
    LuHopOff,
    #[cfg(feature = "LuHotel")]
    LuHotel,
    #[cfg(feature = "LuHourglass")]
    LuHourglass,
    #[cfg(feature = "LuIceCream")]
    LuIceCream,
    #[cfg(feature = "LuIceCream2")]
    LuIceCream2,
    #[cfg(feature = "LuImage")]
    LuImage,
    #[cfg(feature = "LuImageMinus")]
    LuImageMinus,
    #[cfg(feature = "LuImageOff")]
    LuImageOff,
    #[cfg(feature = "LuImagePlus")]
    LuImagePlus,
    #[cfg(feature = "LuImport")]
    LuImport,
    #[cfg(feature = "LuInbox")]
    LuInbox,
    #[cfg(feature = "LuIndent")]
    LuIndent,
    #[cfg(feature = "LuIndianRupee")]
    LuIndianRupee,
    #[cfg(feature = "LuInfinity")]
    LuInfinity,
    #[cfg(feature = "LuInfo")]
    LuInfo,
    #[cfg(feature = "LuInspect")]
    LuInspect,
    #[cfg(feature = "LuInstagram")]
    LuInstagram,
    #[cfg(feature = "LuItalic")]
    LuItalic,
    #[cfg(feature = "LuIterationCcw")]
    LuIterationCcw,
    #[cfg(feature = "LuIterationCw")]
    LuIterationCw,
    #[cfg(feature = "LuJapaneseYen")]
    LuJapaneseYen,
    #[cfg(feature = "LuJoystick")]
    LuJoystick,
    #[cfg(feature = "LuKanban")]
    LuKanban,
    #[cfg(feature = "LuKanbanSquare")]
    LuKanbanSquare,
    #[cfg(feature = "LuKanbanSquareDashed")]
    LuKanbanSquareDashed,
    #[cfg(feature = "LuKey")]
    LuKey,
    #[cfg(feature = "LuKeyRound")]
    LuKeyRound,
    #[cfg(feature = "LuKeySquare")]
    LuKeySquare,
    #[cfg(feature = "LuKeyboard")]
    LuKeyboard,
    #[cfg(feature = "LuLamp")]
    LuLamp,
    #[cfg(feature = "LuLampCeiling")]
    LuLampCeiling,
    #[cfg(feature = "LuLampDesk")]
    LuLampDesk,
    #[cfg(feature = "LuLampFloor")]
    LuLampFloor,
    #[cfg(feature = "LuLampWallDown")]
    LuLampWallDown,
    #[cfg(feature = "LuLampWallUp")]
    LuLampWallUp,
    #[cfg(feature = "LuLandmark")]
    LuLandmark,
    #[cfg(feature = "LuLanguages")]
    LuLanguages,
    #[cfg(feature = "LuLaptop")]
    LuLaptop,
    #[cfg(feature = "LuLaptop2")]
    LuLaptop2,
    #[cfg(feature = "LuLasso")]
    LuLasso,
    #[cfg(feature = "LuLassoSelect")]
    LuLassoSelect,
    #[cfg(feature = "LuLaugh")]
    LuLaugh,
    #[cfg(feature = "LuLayers")]
    LuLayers,
    #[cfg(feature = "LuLayout")]
    LuLayout,
    #[cfg(feature = "LuLayoutDashboard")]
    LuLayoutDashboard,
    #[cfg(feature = "LuLayoutGrid")]
    LuLayoutGrid,
    #[cfg(feature = "LuLayoutList")]
    LuLayoutList,
    #[cfg(feature = "LuLayoutPanelLeft")]
    LuLayoutPanelLeft,
    #[cfg(feature = "LuLayoutPanelTop")]
    LuLayoutPanelTop,
    #[cfg(feature = "LuLayoutTemplate")]
    LuLayoutTemplate,
    #[cfg(feature = "LuLeaf")]
    LuLeaf,
    #[cfg(feature = "LuLeafyGreen")]
    LuLeafyGreen,
    #[cfg(feature = "LuLibrary")]
    LuLibrary,
    #[cfg(feature = "LuLifeBuoy")]
    LuLifeBuoy,
    #[cfg(feature = "LuLigature")]
    LuLigature,
    #[cfg(feature = "LuLightbulb")]
    LuLightbulb,
    #[cfg(feature = "LuLightbulbOff")]
    LuLightbulbOff,
    #[cfg(feature = "LuLineChart")]
    LuLineChart,
    #[cfg(feature = "LuLink")]
    LuLink,
    #[cfg(feature = "LuLink2")]
    LuLink2,
    #[cfg(feature = "LuLink2Off")]
    LuLink2Off,
    #[cfg(feature = "LuLinkedin")]
    LuLinkedin,
    #[cfg(feature = "LuList")]
    LuList,
    #[cfg(feature = "LuListChecks")]
    LuListChecks,
    #[cfg(feature = "LuListEnd")]
    LuListEnd,
    #[cfg(feature = "LuListFilter")]
    LuListFilter,
    #[cfg(feature = "LuListMinus")]
    LuListMinus,
    #[cfg(feature = "LuListMusic")]
    LuListMusic,
    #[cfg(feature = "LuListOrdered")]
    LuListOrdered,
    #[cfg(feature = "LuListPlus")]
    LuListPlus,
    #[cfg(feature = "LuListRestart")]
    LuListRestart,
    #[cfg(feature = "LuListStart")]
    LuListStart,
    #[cfg(feature = "LuListTodo")]
    LuListTodo,
    #[cfg(feature = "LuListTree")]
    LuListTree,
    #[cfg(feature = "LuListVideo")]
    LuListVideo,
    #[cfg(feature = "LuListX")]
    LuListX,
    #[cfg(feature = "LuLoader")]
    LuLoader,
    #[cfg(feature = "LuLoader2")]
    LuLoader2,
    #[cfg(feature = "LuLocate")]
    LuLocate,
    #[cfg(feature = "LuLocateFixed")]
    LuLocateFixed,
    #[cfg(feature = "LuLocateOff")]
    LuLocateOff,
    #[cfg(feature = "LuLock")]
    LuLock,
    #[cfg(feature = "LuLogIn")]
    LuLogIn,
    #[cfg(feature = "LuLogOut")]
    LuLogOut,
    #[cfg(feature = "LuLollipop")]
    LuLollipop,
    #[cfg(feature = "LuLuggage")]
    LuLuggage,
    #[cfg(feature = "LuMSquare")]
    LuMSquare,
    #[cfg(feature = "LuMagnet")]
    LuMagnet,
    #[cfg(feature = "LuMail")]
    LuMail,
    #[cfg(feature = "LuMailCheck")]
    LuMailCheck,
    #[cfg(feature = "LuMailMinus")]
    LuMailMinus,
    #[cfg(feature = "LuMailOpen")]
    LuMailOpen,
    #[cfg(feature = "LuMailPlus")]
    LuMailPlus,
    #[cfg(feature = "LuMailQuestion")]
    LuMailQuestion,
    #[cfg(feature = "LuMailSearch")]
    LuMailSearch,
    #[cfg(feature = "LuMailWarning")]
    LuMailWarning,
    #[cfg(feature = "LuMailX")]
    LuMailX,
    #[cfg(feature = "LuMailbox")]
    LuMailbox,
    #[cfg(feature = "LuMails")]
    LuMails,
    #[cfg(feature = "LuMap")]
    LuMap,
    #[cfg(feature = "LuMapPin")]
    LuMapPin,
    #[cfg(feature = "LuMapPinOff")]
    LuMapPinOff,
    #[cfg(feature = "LuMartini")]
    LuMartini,
    #[cfg(feature = "LuMaximize")]
    LuMaximize,
    #[cfg(feature = "LuMaximize2")]
    LuMaximize2,
    #[cfg(feature = "LuMedal")]
    LuMedal,
    #[cfg(feature = "LuMegaphone")]
    LuMegaphone,
    #[cfg(feature = "LuMegaphoneOff")]
    LuMegaphoneOff,
    #[cfg(feature = "LuMeh")]
    LuMeh,
    #[cfg(feature = "LuMemoryStick")]
    LuMemoryStick,
    #[cfg(feature = "LuMenu")]
    LuMenu,
    #[cfg(feature = "LuMenuSquare")]
    LuMenuSquare,
    #[cfg(feature = "LuMerge")]
    LuMerge,
    #[cfg(feature = "LuMessageCircle")]
    LuMessageCircle,
    #[cfg(feature = "LuMessageSquare")]
    LuMessageSquare,
    #[cfg(feature = "LuMessageSquareDashed")]
    LuMessageSquareDashed,
    #[cfg(feature = "LuMessageSquarePlus")]
    LuMessageSquarePlus,
    #[cfg(feature = "LuMessagesSquare")]
    LuMessagesSquare,
    #[cfg(feature = "LuMic")]
    LuMic,
    #[cfg(feature = "LuMic2")]
    LuMic2,
    #[cfg(feature = "LuMicOff")]
    LuMicOff,
    #[cfg(feature = "LuMicroscope")]
    LuMicroscope,
    #[cfg(feature = "LuMicrowave")]
    LuMicrowave,
    #[cfg(feature = "LuMilestone")]
    LuMilestone,
    #[cfg(feature = "LuMilk")]
    LuMilk,
    #[cfg(feature = "LuMilkOff")]
    LuMilkOff,
    #[cfg(feature = "LuMinimize")]
    LuMinimize,
    #[cfg(feature = "LuMinimize2")]
    LuMinimize2,
    #[cfg(feature = "LuMinus")]
    LuMinus,
    #[cfg(feature = "LuMinusCircle")]
    LuMinusCircle,
    #[cfg(feature = "LuMinusSquare")]
    LuMinusSquare,
    #[cfg(feature = "LuMonitor")]
    LuMonitor,
    #[cfg(feature = "LuMonitorCheck")]
    LuMonitorCheck,
    #[cfg(feature = "LuMonitorDot")]
    LuMonitorDot,
    #[cfg(feature = "LuMonitorDown")]
    LuMonitorDown,
    #[cfg(feature = "LuMonitorOff")]
    LuMonitorOff,
    #[cfg(feature = "LuMonitorPause")]
    LuMonitorPause,
    #[cfg(feature = "LuMonitorPlay")]
    LuMonitorPlay,
    #[cfg(feature = "LuMonitorSmartphone")]
    LuMonitorSmartphone,
    #[cfg(feature = "LuMonitorSpeaker")]
    LuMonitorSpeaker,
    #[cfg(feature = "LuMonitorStop")]
    LuMonitorStop,
    #[cfg(feature = "LuMonitorUp")]
    LuMonitorUp,
    #[cfg(feature = "LuMonitorX")]
    LuMonitorX,
    #[cfg(feature = "LuMoon")]
    LuMoon,
    #[cfg(feature = "LuMoonStar")]
    LuMoonStar,
    #[cfg(feature = "LuMoreHorizontal")]
    LuMoreHorizontal,
    #[cfg(feature = "LuMoreVertical")]
    LuMoreVertical,
    #[cfg(feature = "LuMountain")]
    LuMountain,
    #[cfg(feature = "LuMountainSnow")]
    LuMountainSnow,
    #[cfg(feature = "LuMouse")]
    LuMouse,
    #[cfg(feature = "LuMousePointer")]
    LuMousePointer,
    #[cfg(feature = "LuMousePointer2")]
    LuMousePointer2,
    #[cfg(feature = "LuMousePointerClick")]
    LuMousePointerClick,
    #[cfg(feature = "LuMove")]
    LuMove,
    #[cfg(feature = "LuMove3d")]
    LuMove3d,
    #[cfg(feature = "LuMoveDiagonal")]
    LuMoveDiagonal,
    #[cfg(feature = "LuMoveDiagonal2")]
    LuMoveDiagonal2,
    #[cfg(feature = "LuMoveDown")]
    LuMoveDown,
    #[cfg(feature = "LuMoveDownLeft")]
    LuMoveDownLeft,
    #[cfg(feature = "LuMoveDownRight")]
    LuMoveDownRight,
    #[cfg(feature = "LuMoveHorizontal")]
    LuMoveHorizontal,
    #[cfg(feature = "LuMoveLeft")]
    LuMoveLeft,
    #[cfg(feature = "LuMoveRight")]
    LuMoveRight,
    #[cfg(feature = "LuMoveUp")]
    LuMoveUp,
    #[cfg(feature = "LuMoveUpLeft")]
    LuMoveUpLeft,
    #[cfg(feature = "LuMoveUpRight")]
    LuMoveUpRight,
    #[cfg(feature = "LuMoveVertical")]
    LuMoveVertical,
    #[cfg(feature = "LuMusic")]
    LuMusic,
    #[cfg(feature = "LuMusic2")]
    LuMusic2,
    #[cfg(feature = "LuMusic3")]
    LuMusic3,
    #[cfg(feature = "LuMusic4")]
    LuMusic4,
    #[cfg(feature = "LuNavigation")]
    LuNavigation,
    #[cfg(feature = "LuNavigation2")]
    LuNavigation2,
    #[cfg(feature = "LuNavigation2Off")]
    LuNavigation2Off,
    #[cfg(feature = "LuNavigationOff")]
    LuNavigationOff,
    #[cfg(feature = "LuNetwork")]
    LuNetwork,
    #[cfg(feature = "LuNewspaper")]
    LuNewspaper,
    #[cfg(feature = "LuNfc")]
    LuNfc,
    #[cfg(feature = "LuNut")]
    LuNut,
    #[cfg(feature = "LuNutOff")]
    LuNutOff,
    #[cfg(feature = "LuOctagon")]
    LuOctagon,
    #[cfg(feature = "LuOption")]
    LuOption,
    #[cfg(feature = "LuOrbit")]
    LuOrbit,
    #[cfg(feature = "LuOutdent")]
    LuOutdent,
    #[cfg(feature = "LuPackage")]
    LuPackage,
    #[cfg(feature = "LuPackage2")]
    LuPackage2,
    #[cfg(feature = "LuPackageCheck")]
    LuPackageCheck,
    #[cfg(feature = "LuPackageMinus")]
    LuPackageMinus,
    #[cfg(feature = "LuPackageOpen")]
    LuPackageOpen,
    #[cfg(feature = "LuPackagePlus")]
    LuPackagePlus,
    #[cfg(feature = "LuPackageSearch")]
    LuPackageSearch,
    #[cfg(feature = "LuPackageX")]
    LuPackageX,
    #[cfg(feature = "LuPaintBucket")]
    LuPaintBucket,
    #[cfg(feature = "LuPaintbrush")]
    LuPaintbrush,
    #[cfg(feature = "LuPaintbrush2")]
    LuPaintbrush2,
    #[cfg(feature = "LuPalette")]
    LuPalette,
    #[cfg(feature = "LuPalmtree")]
    LuPalmtree,
    #[cfg(feature = "LuPanelBottom")]
    LuPanelBottom,
    #[cfg(feature = "LuPanelBottomClose")]
    LuPanelBottomClose,
    #[cfg(feature = "LuPanelBottomInactive")]
    LuPanelBottomInactive,
    #[cfg(feature = "LuPanelBottomOpen")]
    LuPanelBottomOpen,
    #[cfg(feature = "LuPanelLeft")]
    LuPanelLeft,
    #[cfg(feature = "LuPanelLeftClose")]
    LuPanelLeftClose,
    #[cfg(feature = "LuPanelLeftInactive")]
    LuPanelLeftInactive,
    #[cfg(feature = "LuPanelLeftOpen")]
    LuPanelLeftOpen,
    #[cfg(feature = "LuPanelRight")]
    LuPanelRight,
    #[cfg(feature = "LuPanelRightClose")]
    LuPanelRightClose,
    #[cfg(feature = "LuPanelRightInactive")]
    LuPanelRightInactive,
    #[cfg(feature = "LuPanelRightOpen")]
    LuPanelRightOpen,
    #[cfg(feature = "LuPanelTop")]
    LuPanelTop,
    #[cfg(feature = "LuPanelTopClose")]
    LuPanelTopClose,
    #[cfg(feature = "LuPanelTopInactive")]
    LuPanelTopInactive,
    #[cfg(feature = "LuPanelTopOpen")]
    LuPanelTopOpen,
    #[cfg(feature = "LuPaperclip")]
    LuPaperclip,
    #[cfg(feature = "LuParentheses")]
    LuParentheses,
    #[cfg(feature = "LuParkingCircle")]
    LuParkingCircle,
    #[cfg(feature = "LuParkingCircleOff")]
    LuParkingCircleOff,
    #[cfg(feature = "LuParkingMeter")]
    LuParkingMeter,
    #[cfg(feature = "LuParkingSquare")]
    LuParkingSquare,
    #[cfg(feature = "LuParkingSquareOff")]
    LuParkingSquareOff,
    #[cfg(feature = "LuPartyPopper")]
    LuPartyPopper,
    #[cfg(feature = "LuPause")]
    LuPause,
    #[cfg(feature = "LuPauseCircle")]
    LuPauseCircle,
    #[cfg(feature = "LuPauseOctagon")]
    LuPauseOctagon,
    #[cfg(feature = "LuPawPrint")]
    LuPawPrint,
    #[cfg(feature = "LuPcCase")]
    LuPcCase,
    #[cfg(feature = "LuPen")]
    LuPen,
    #[cfg(feature = "LuPenLine")]
    LuPenLine,
    #[cfg(feature = "LuPenSquare")]
    LuPenSquare,
    #[cfg(feature = "LuPenTool")]
    LuPenTool,
    #[cfg(feature = "LuPencil")]
    LuPencil,
    #[cfg(feature = "LuPencilLine")]
    LuPencilLine,
    #[cfg(feature = "LuPencilRuler")]
    LuPencilRuler,
    #[cfg(feature = "LuPercent")]
    LuPercent,
    #[cfg(feature = "LuPersonStanding")]
    LuPersonStanding,
    #[cfg(feature = "LuPhone")]
    LuPhone,
    #[cfg(feature = "LuPhoneCall")]
    LuPhoneCall,
    #[cfg(feature = "LuPhoneForwarded")]
    LuPhoneForwarded,
    #[cfg(feature = "LuPhoneIncoming")]
    LuPhoneIncoming,
    #[cfg(feature = "LuPhoneMissed")]
    LuPhoneMissed,
    #[cfg(feature = "LuPhoneOff")]
    LuPhoneOff,
    #[cfg(feature = "LuPhoneOutgoing")]
    LuPhoneOutgoing,
    #[cfg(feature = "LuPi")]
    LuPi,
    #[cfg(feature = "LuPiSquare")]
    LuPiSquare,
    #[cfg(feature = "LuPictureInPicture")]
    LuPictureInPicture,
    #[cfg(feature = "LuPictureInPicture2")]
    LuPictureInPicture2,
    #[cfg(feature = "LuPieChart")]
    LuPieChart,
    #[cfg(feature = "LuPiggyBank")]
    LuPiggyBank,
    #[cfg(feature = "LuPilcrow")]
    LuPilcrow,
    #[cfg(feature = "LuPilcrowSquare")]
    LuPilcrowSquare,
    #[cfg(feature = "LuPill")]
    LuPill,
    #[cfg(feature = "LuPin")]
    LuPin,
    #[cfg(feature = "LuPinOff")]
    LuPinOff,
    #[cfg(feature = "LuPipette")]
    LuPipette,
    #[cfg(feature = "LuPizza")]
    LuPizza,
    #[cfg(feature = "LuPlane")]
    LuPlane,
    #[cfg(feature = "LuPlaneLanding")]
    LuPlaneLanding,
    #[cfg(feature = "LuPlaneTakeoff")]
    LuPlaneTakeoff,
    #[cfg(feature = "LuPlay")]
    LuPlay,
    #[cfg(feature = "LuPlayCircle")]
    LuPlayCircle,
    #[cfg(feature = "LuPlaySquare")]
    LuPlaySquare,
    #[cfg(feature = "LuPlug")]
    LuPlug,
    #[cfg(feature = "LuPlug2")]
    LuPlug2,
    #[cfg(feature = "LuPlugZap")]
    LuPlugZap,
    #[cfg(feature = "LuPlugZap2")]
    LuPlugZap2,
    #[cfg(feature = "LuPlus")]
    LuPlus,
    #[cfg(feature = "LuPlusCircle")]
    LuPlusCircle,
    #[cfg(feature = "LuPlusSquare")]
    LuPlusSquare,
    #[cfg(feature = "LuPocket")]
    LuPocket,
    #[cfg(feature = "LuPocketKnife")]
    LuPocketKnife,
    #[cfg(feature = "LuPodcast")]
    LuPodcast,
    #[cfg(feature = "LuPointer")]
    LuPointer,
    #[cfg(feature = "LuPopcorn")]
    LuPopcorn,
    #[cfg(feature = "LuPopsicle")]
    LuPopsicle,
    #[cfg(feature = "LuPoundSterling")]
    LuPoundSterling,
    #[cfg(feature = "LuPower")]
    LuPower,
    #[cfg(feature = "LuPowerOff")]
    LuPowerOff,
    #[cfg(feature = "LuPresentation")]
    LuPresentation,
    #[cfg(feature = "LuPrinter")]
    LuPrinter,
    #[cfg(feature = "LuProjector")]
    LuProjector,
    #[cfg(feature = "LuPuzzle")]
    LuPuzzle,
    #[cfg(feature = "LuQrCode")]
    LuQrCode,
    #[cfg(feature = "LuQuote")]
    LuQuote,
    #[cfg(feature = "LuRabbit")]
    LuRabbit,
    #[cfg(feature = "LuRadar")]
    LuRadar,
    #[cfg(feature = "LuRadiation")]
    LuRadiation,
    #[cfg(feature = "LuRadio")]
    LuRadio,
    #[cfg(feature = "LuRadioReceiver")]
    LuRadioReceiver,
    #[cfg(feature = "LuRadioTower")]
    LuRadioTower,
    #[cfg(feature = "LuRailSymbol")]
    LuRailSymbol,
    #[cfg(feature = "LuRainbow")]
    LuRainbow,
    #[cfg(feature = "LuRat")]
    LuRat,
    #[cfg(feature = "LuRatio")]
    LuRatio,
    #[cfg(feature = "LuReceipt")]
    LuReceipt,
    #[cfg(feature = "LuRectangleHorizontal")]
    LuRectangleHorizontal,
    #[cfg(feature = "LuRectangleVertical")]
    LuRectangleVertical,
    #[cfg(feature = "LuRecycle")]
    LuRecycle,
    #[cfg(feature = "LuRedo")]
    LuRedo,
    #[cfg(feature = "LuRedo2")]
    LuRedo2,
    #[cfg(feature = "LuRedoDot")]
    LuRedoDot,
    #[cfg(feature = "LuRefreshCcw")]
    LuRefreshCcw,
    #[cfg(feature = "LuRefreshCcwDot")]
    LuRefreshCcwDot,
    #[cfg(feature = "LuRefreshCw")]
    LuRefreshCw,
    #[cfg(feature = "LuRefreshCwOff")]
    LuRefreshCwOff,
    #[cfg(feature = "LuRefrigerator")]
    LuRefrigerator,
    #[cfg(feature = "LuRegex")]
    LuRegex,
    #[cfg(feature = "LuRemoveFormatting")]
    LuRemoveFormatting,
    #[cfg(feature = "LuRepeat")]
    LuRepeat,
    #[cfg(feature = "LuRepeat1")]
    LuRepeat1,
    #[cfg(feature = "LuRepeat2")]
    LuRepeat2,
    #[cfg(feature = "LuReplace")]
    LuReplace,
    #[cfg(feature = "LuReplaceAll")]
    LuReplaceAll,
    #[cfg(feature = "LuReply")]
    LuReply,
    #[cfg(feature = "LuReplyAll")]
    LuReplyAll,
    #[cfg(feature = "LuRewind")]
    LuRewind,
    #[cfg(feature = "LuRocket")]
    LuRocket,
    #[cfg(feature = "LuRockingChair")]
    LuRockingChair,
    #[cfg(feature = "LuRollerCoaster")]
    LuRollerCoaster,
    #[cfg(feature = "LuRotate3d")]
    LuRotate3d,
    #[cfg(feature = "LuRotateCcw")]
    LuRotateCcw,
    #[cfg(feature = "LuRotateCw")]
    LuRotateCw,
    #[cfg(feature = "LuRouter")]
    LuRouter,
    #[cfg(feature = "LuRows")]
    LuRows,
    #[cfg(feature = "LuRss")]
    LuRss,
    #[cfg(feature = "LuRuler")]
    LuRuler,
    #[cfg(feature = "LuRussianRuble")]
    LuRussianRuble,
    #[cfg(feature = "LuSailboat")]
    LuSailboat,
    #[cfg(feature = "LuSalad")]
    LuSalad,
    #[cfg(feature = "LuSandwich")]
    LuSandwich,
    #[cfg(feature = "LuSatellite")]
    LuSatellite,
    #[cfg(feature = "LuSatelliteDish")]
    LuSatelliteDish,
    #[cfg(feature = "LuSave")]
    LuSave,
    #[cfg(feature = "LuSaveAll")]
    LuSaveAll,
    #[cfg(feature = "LuScale")]
    LuScale,
    #[cfg(feature = "LuScale3d")]
    LuScale3d,
    #[cfg(feature = "LuScaling")]
    LuScaling,
    #[cfg(feature = "LuScan")]
    LuScan,
    #[cfg(feature = "LuScanFace")]
    LuScanFace,
    #[cfg(feature = "LuScanLine")]
    LuScanLine,
    #[cfg(feature = "LuScatterChart")]
    LuScatterChart,
    #[cfg(feature = "LuSchool")]
    LuSchool,
    #[cfg(feature = "LuSchool2")]
    LuSchool2,
    #[cfg(feature = "LuScissors")]
    LuScissors,
    #[cfg(feature = "LuScissorsLineDashed")]
    LuScissorsLineDashed,
    #[cfg(feature = "LuScissorsSquare")]
    LuScissorsSquare,
    #[cfg(feature = "LuScissorsSquareDashedBottom")]
    LuScissorsSquareDashedBottom,
    #[cfg(feature = "LuScreenShare")]
    LuScreenShare,
    #[cfg(feature = "LuScreenShareOff")]
    LuScreenShareOff,
    #[cfg(feature = "LuScroll")]
    LuScroll,
    #[cfg(feature = "LuScrollText")]
    LuScrollText,
    #[cfg(feature = "LuSearch")]
    LuSearch,
    #[cfg(feature = "LuSearchCheck")]
    LuSearchCheck,
    #[cfg(feature = "LuSearchCode")]
    LuSearchCode,
    #[cfg(feature = "LuSearchSlash")]
    LuSearchSlash,
    #[cfg(feature = "LuSearchX")]
    LuSearchX,
    #[cfg(feature = "LuSend")]
    LuSend,
    #[cfg(feature = "LuSendHorizonal")]
    LuSendHorizonal,
    #[cfg(feature = "LuSendToBack")]
    LuSendToBack,
    #[cfg(feature = "LuSeparatorHorizontal")]
    LuSeparatorHorizontal,
    #[cfg(feature = "LuSeparatorVertical")]
    LuSeparatorVertical,
    #[cfg(feature = "LuServer")]
    LuServer,
    #[cfg(feature = "LuServerCog")]
    LuServerCog,
    #[cfg(feature = "LuServerCrash")]
    LuServerCrash,
    #[cfg(feature = "LuServerOff")]
    LuServerOff,
    #[cfg(feature = "LuSettings")]
    LuSettings,
    #[cfg(feature = "LuSettings2")]
    LuSettings2,
    #[cfg(feature = "LuShapes")]
    LuShapes,
    #[cfg(feature = "LuShare")]
    LuShare,
    #[cfg(feature = "LuShare2")]
    LuShare2,
    #[cfg(feature = "LuSheet")]
    LuSheet,
    #[cfg(feature = "LuShell")]
    LuShell,
    #[cfg(feature = "LuShield")]
    LuShield,
    #[cfg(feature = "LuShieldAlert")]
    LuShieldAlert,
    #[cfg(feature = "LuShieldCheck")]
    LuShieldCheck,
    #[cfg(feature = "LuShieldClose")]
    LuShieldClose,
    #[cfg(feature = "LuShieldOff")]
    LuShieldOff,
    #[cfg(feature = "LuShieldQuestion")]
    LuShieldQuestion,
    #[cfg(feature = "LuShip")]
    LuShip,
    #[cfg(feature = "LuShipWheel")]
    LuShipWheel,
    #[cfg(feature = "LuShirt")]
    LuShirt,
    #[cfg(feature = "LuShoppingBag")]
    LuShoppingBag,
    #[cfg(feature = "LuShoppingBasket")]
    LuShoppingBasket,
    #[cfg(feature = "LuShoppingCart")]
    LuShoppingCart,
    #[cfg(feature = "LuShovel")]
    LuShovel,
    #[cfg(feature = "LuShowerHead")]
    LuShowerHead,
    #[cfg(feature = "LuShrink")]
    LuShrink,
    #[cfg(feature = "LuShrub")]
    LuShrub,
    #[cfg(feature = "LuShuffle")]
    LuShuffle,
    #[cfg(feature = "LuSigma")]
    LuSigma,
    #[cfg(feature = "LuSigmaSquare")]
    LuSigmaSquare,
    #[cfg(feature = "LuSignal")]
    LuSignal,
    #[cfg(feature = "LuSignalHigh")]
    LuSignalHigh,
    #[cfg(feature = "LuSignalLow")]
    LuSignalLow,
    #[cfg(feature = "LuSignalMedium")]
    LuSignalMedium,
    #[cfg(feature = "LuSignalZero")]
    LuSignalZero,
    #[cfg(feature = "LuSiren")]
    LuSiren,
    #[cfg(feature = "LuSkipBack")]
    LuSkipBack,
    #[cfg(feature = "LuSkipForward")]
    LuSkipForward,
    #[cfg(feature = "LuSkull")]
    LuSkull,
    #[cfg(feature = "LuSlack")]
    LuSlack,
    #[cfg(feature = "LuSlice")]
    LuSlice,
    #[cfg(feature = "LuSliders")]
    LuSliders,
    #[cfg(feature = "LuSlidersHorizontal")]
    LuSlidersHorizontal,
    #[cfg(feature = "LuSmartphone")]
    LuSmartphone,
    #[cfg(feature = "LuSmartphoneCharging")]
    LuSmartphoneCharging,
    #[cfg(feature = "LuSmartphoneNfc")]
    LuSmartphoneNfc,
    #[cfg(feature = "LuSmile")]
    LuSmile,
    #[cfg(feature = "LuSmilePlus")]
    LuSmilePlus,
    #[cfg(feature = "LuSnail")]
    LuSnail,
    #[cfg(feature = "LuSnowflake")]
    LuSnowflake,
    #[cfg(feature = "LuSofa")]
    LuSofa,
    #[cfg(feature = "LuSoup")]
    LuSoup,
    #[cfg(feature = "LuSpace")]
    LuSpace,
    #[cfg(feature = "LuSpade")]
    LuSpade,
    #[cfg(feature = "LuSparkle")]
    LuSparkle,
    #[cfg(feature = "LuSparkles")]
    LuSparkles,
    #[cfg(feature = "LuSpeaker")]
    LuSpeaker,
    #[cfg(feature = "LuSpellCheck")]
    LuSpellCheck,
    #[cfg(feature = "LuSpellCheck2")]
    LuSpellCheck2,
    #[cfg(feature = "LuSpline")]
    LuSpline,
    #[cfg(feature = "LuSplit")]
    LuSplit,
    #[cfg(feature = "LuSplitSquareHorizontal")]
    LuSplitSquareHorizontal,
    #[cfg(feature = "LuSplitSquareVertical")]
    LuSplitSquareVertical,
    #[cfg(feature = "LuSprayCan")]
    LuSprayCan,
    #[cfg(feature = "LuSprout")]
    LuSprout,
    #[cfg(feature = "LuSquare")]
    LuSquare,
    #[cfg(feature = "LuSquareAsterisk")]
    LuSquareAsterisk,
    #[cfg(feature = "LuSquareCode")]
    LuSquareCode,
    #[cfg(feature = "LuSquareDashedBottom")]
    LuSquareDashedBottom,
    #[cfg(feature = "LuSquareDashedBottomCode")]
    LuSquareDashedBottomCode,
    #[cfg(feature = "LuSquareDot")]
    LuSquareDot,
    #[cfg(feature = "LuSquareEqual")]
    LuSquareEqual,
    #[cfg(feature = "LuSquareSlash")]
    LuSquareSlash,
    #[cfg(feature = "LuSquareStack")]
    LuSquareStack,
    #[cfg(feature = "LuSquirrel")]
    LuSquirrel,
    #[cfg(feature = "LuStamp")]
    LuStamp,
    #[cfg(feature = "LuStar")]
    LuStar,
    #[cfg(feature = "LuStarHalf")]
    LuStarHalf,
    #[cfg(feature = "LuStarOff")]
    LuStarOff,
    #[cfg(feature = "LuStepBack")]
    LuStepBack,
    #[cfg(feature = "LuStepForward")]
    LuStepForward,
    #[cfg(feature = "LuStethoscope")]
    LuStethoscope,
    #[cfg(feature = "LuSticker")]
    LuSticker,
    #[cfg(feature = "LuStickyNote")]
    LuStickyNote,
    #[cfg(feature = "LuStopCircle")]
    LuStopCircle,
    #[cfg(feature = "LuStore")]
    LuStore,
    #[cfg(feature = "LuStretchHorizontal")]
    LuStretchHorizontal,
    #[cfg(feature = "LuStretchVertical")]
    LuStretchVertical,
    #[cfg(feature = "LuStrikethrough")]
    LuStrikethrough,
    #[cfg(feature = "LuSubscript")]
    LuSubscript,
    #[cfg(feature = "LuSubtitles")]
    LuSubtitles,
    #[cfg(feature = "LuSun")]
    LuSun,
    #[cfg(feature = "LuSunDim")]
    LuSunDim,
    #[cfg(feature = "LuSunMedium")]
    LuSunMedium,
    #[cfg(feature = "LuSunMoon")]
    LuSunMoon,
    #[cfg(feature = "LuSunSnow")]
    LuSunSnow,
    #[cfg(feature = "LuSunrise")]
    LuSunrise,
    #[cfg(feature = "LuSunset")]
    LuSunset,
    #[cfg(feature = "LuSuperscript")]
    LuSuperscript,
    #[cfg(feature = "LuSwissFranc")]
    LuSwissFranc,
    #[cfg(feature = "LuSwitchCamera")]
    LuSwitchCamera,
    #[cfg(feature = "LuSword")]
    LuSword,
    #[cfg(feature = "LuSwords")]
    LuSwords,
    #[cfg(feature = "LuSyringe")]
    LuSyringe,
    #[cfg(feature = "LuTable")]
    LuTable,
    #[cfg(feature = "LuTable2")]
    LuTable2,
    #[cfg(feature = "LuTableProperties")]
    LuTableProperties,
    #[cfg(feature = "LuTablet")]
    LuTablet,
    #[cfg(feature = "LuTablets")]
    LuTablets,
    #[cfg(feature = "LuTag")]
    LuTag,
    #[cfg(feature = "LuTags")]
    LuTags,
    #[cfg(feature = "LuTally1")]
    LuTally1,
    #[cfg(feature = "LuTally2")]
    LuTally2,
    #[cfg(feature = "LuTally3")]
    LuTally3,
    #[cfg(feature = "LuTally4")]
    LuTally4,
    #[cfg(feature = "LuTally5")]
    LuTally5,
    #[cfg(feature = "LuTarget")]
    LuTarget,
    #[cfg(feature = "LuTent")]
    LuTent,
    #[cfg(feature = "LuTerminal")]
    LuTerminal,
    #[cfg(feature = "LuTerminalSquare")]
    LuTerminalSquare,
    #[cfg(feature = "LuTestTube")]
    LuTestTube,
    #[cfg(feature = "LuTestTube2")]
    LuTestTube2,
    #[cfg(feature = "LuTestTubes")]
    LuTestTubes,
    #[cfg(feature = "LuText")]
    LuText,
    #[cfg(feature = "LuTextCursor")]
    LuTextCursor,
    #[cfg(feature = "LuTextCursorInput")]
    LuTextCursorInput,
    #[cfg(feature = "LuTextQuote")]
    LuTextQuote,
    #[cfg(feature = "LuTextSelect")]
    LuTextSelect,
    #[cfg(feature = "LuThermometer")]
    LuThermometer,
    #[cfg(feature = "LuThermometerSnowflake")]
    LuThermometerSnowflake,
    #[cfg(feature = "LuThermometerSun")]
    LuThermometerSun,
    #[cfg(feature = "LuThumbsDown")]
    LuThumbsDown,
    #[cfg(feature = "LuThumbsUp")]
    LuThumbsUp,
    #[cfg(feature = "LuTicket")]
    LuTicket,
    #[cfg(feature = "LuTimer")]
    LuTimer,
    #[cfg(feature = "LuTimerOff")]
    LuTimerOff,
    #[cfg(feature = "LuTimerReset")]
    LuTimerReset,
    #[cfg(feature = "LuToggleLeft")]
    LuToggleLeft,
    #[cfg(feature = "LuToggleRight")]
    LuToggleRight,
    #[cfg(feature = "LuTornado")]
    LuTornado,
    #[cfg(feature = "LuTouchpad")]
    LuTouchpad,
    #[cfg(feature = "LuTouchpadOff")]
    LuTouchpadOff,
    #[cfg(feature = "LuTowerControl")]
    LuTowerControl,
    #[cfg(feature = "LuToyBrick")]
    LuToyBrick,
    #[cfg(feature = "LuTractor")]
    LuTractor,
    #[cfg(feature = "LuTrafficCone")]
    LuTrafficCone,
    #[cfg(feature = "LuTrainFront")]
    LuTrainFront,
    #[cfg(feature = "LuTrainFrontTunnel")]
    LuTrainFrontTunnel,
    #[cfg(feature = "LuTrainTrack")]
    LuTrainTrack,
    #[cfg(feature = "LuTramFront")]
    LuTramFront,
    #[cfg(feature = "LuTrash")]
    LuTrash,
    #[cfg(feature = "LuTrash2")]
    LuTrash2,
    #[cfg(feature = "LuTreeDeciduous")]
    LuTreeDeciduous,
    #[cfg(feature = "LuTreePine")]
    LuTreePine,
    #[cfg(feature = "LuTrees")]
    LuTrees,
    #[cfg(feature = "LuTrello")]
    LuTrello,
    #[cfg(feature = "LuTrendingDown")]
    LuTrendingDown,
    #[cfg(feature = "LuTrendingUp")]
    LuTrendingUp,
    #[cfg(feature = "LuTriangle")]
    LuTriangle,
    #[cfg(feature = "LuTriangleRight")]
    LuTriangleRight,
    #[cfg(feature = "LuTrophy")]
    LuTrophy,
    #[cfg(feature = "LuTruck")]
    LuTruck,
    #[cfg(feature = "LuTurtle")]
    LuTurtle,
    #[cfg(feature = "LuTv")]
    LuTv,
    #[cfg(feature = "LuTv2")]
    LuTv2,
    #[cfg(feature = "LuTwitch")]
    LuTwitch,
    #[cfg(feature = "LuTwitter")]
    LuTwitter,
    #[cfg(feature = "LuType")]
    LuType,
    #[cfg(feature = "LuUmbrella")]
    LuUmbrella,
    #[cfg(feature = "LuUnderline")]
    LuUnderline,
    #[cfg(feature = "LuUndo")]
    LuUndo,
    #[cfg(feature = "LuUndo2")]
    LuUndo2,
    #[cfg(feature = "LuUndoDot")]
    LuUndoDot,
    #[cfg(feature = "LuUnfoldHorizontal")]
    LuUnfoldHorizontal,
    #[cfg(feature = "LuUnfoldVertical")]
    LuUnfoldVertical,
    #[cfg(feature = "LuUngroup")]
    LuUngroup,
    #[cfg(feature = "LuUnlink")]
    LuUnlink,
    #[cfg(feature = "LuUnlink2")]
    LuUnlink2,
    #[cfg(feature = "LuUnlock")]
    LuUnlock,
    #[cfg(feature = "LuUnplug")]
    LuUnplug,
    #[cfg(feature = "LuUpload")]
    LuUpload,
    #[cfg(feature = "LuUploadCloud")]
    LuUploadCloud,
    #[cfg(feature = "LuUsb")]
    LuUsb,
    #[cfg(feature = "LuUser")]
    LuUser,
    #[cfg(feature = "LuUser2")]
    LuUser2,
    #[cfg(feature = "LuUserCheck")]
    LuUserCheck,
    #[cfg(feature = "LuUserCheck2")]
    LuUserCheck2,
    #[cfg(feature = "LuUserCircle")]
    LuUserCircle,
    #[cfg(feature = "LuUserCircle2")]
    LuUserCircle2,
    #[cfg(feature = "LuUserCog")]
    LuUserCog,
    #[cfg(feature = "LuUserCog2")]
    LuUserCog2,
    #[cfg(feature = "LuUserMinus")]
    LuUserMinus,
    #[cfg(feature = "LuUserMinus2")]
    LuUserMinus2,
    #[cfg(feature = "LuUserPlus")]
    LuUserPlus,
    #[cfg(feature = "LuUserPlus2")]
    LuUserPlus2,
    #[cfg(feature = "LuUserSquare")]
    LuUserSquare,
    #[cfg(feature = "LuUserSquare2")]
    LuUserSquare2,
    #[cfg(feature = "LuUserX")]
    LuUserX,
    #[cfg(feature = "LuUserX2")]
    LuUserX2,
    #[cfg(feature = "LuUsers")]
    LuUsers,
    #[cfg(feature = "LuUsers2")]
    LuUsers2,
    #[cfg(feature = "LuUtensils")]
    LuUtensils,
    #[cfg(feature = "LuUtensilsCrossed")]
    LuUtensilsCrossed,
    #[cfg(feature = "LuUtilityPole")]
    LuUtilityPole,
    #[cfg(feature = "LuVariable")]
    LuVariable,
    #[cfg(feature = "LuVegan")]
    LuVegan,
    #[cfg(feature = "LuVenetianMask")]
    LuVenetianMask,
    #[cfg(feature = "LuVibrate")]
    LuVibrate,
    #[cfg(feature = "LuVibrateOff")]
    LuVibrateOff,
    #[cfg(feature = "LuVideo")]
    LuVideo,
    #[cfg(feature = "LuVideoOff")]
    LuVideoOff,
    #[cfg(feature = "LuVideotape")]
    LuVideotape,
    #[cfg(feature = "LuView")]
    LuView,
    #[cfg(feature = "LuVoicemail")]
    LuVoicemail,
    #[cfg(feature = "LuVolume")]
    LuVolume,
    #[cfg(feature = "LuVolume1")]
    LuVolume1,
    #[cfg(feature = "LuVolume2")]
    LuVolume2,
    #[cfg(feature = "LuVolumeX")]
    LuVolumeX,
    #[cfg(feature = "LuVote")]
    LuVote,
    #[cfg(feature = "LuWallet")]
    LuWallet,
    #[cfg(feature = "LuWallet2")]
    LuWallet2,
    #[cfg(feature = "LuWalletCards")]
    LuWalletCards,
    #[cfg(feature = "LuWallpaper")]
    LuWallpaper,
    #[cfg(feature = "LuWand")]
    LuWand,
    #[cfg(feature = "LuWand2")]
    LuWand2,
    #[cfg(feature = "LuWarehouse")]
    LuWarehouse,
    #[cfg(feature = "LuWatch")]
    LuWatch,
    #[cfg(feature = "LuWaves")]
    LuWaves,
    #[cfg(feature = "LuWebcam")]
    LuWebcam,
    #[cfg(feature = "LuWebhook")]
    LuWebhook,
    #[cfg(feature = "LuWheat")]
    LuWheat,
    #[cfg(feature = "LuWheatOff")]
    LuWheatOff,
    #[cfg(feature = "LuWholeWord")]
    LuWholeWord,
    #[cfg(feature = "LuWifi")]
    LuWifi,
    #[cfg(feature = "LuWifiOff")]
    LuWifiOff,
    #[cfg(feature = "LuWind")]
    LuWind,
    #[cfg(feature = "LuWine")]
    LuWine,
    #[cfg(feature = "LuWineOff")]
    LuWineOff,
    #[cfg(feature = "LuWorkflow")]
    LuWorkflow,
    #[cfg(feature = "LuWrapText")]
    LuWrapText,
    #[cfg(feature = "LuWrench")]
    LuWrench,
    #[cfg(feature = "LuX")]
    LuX,
    #[cfg(feature = "LuXCircle")]
    LuXCircle,
    #[cfg(feature = "LuXOctagon")]
    LuXOctagon,
    #[cfg(feature = "LuXSquare")]
    LuXSquare,
    #[cfg(feature = "LuYoutube")]
    LuYoutube,
    #[cfg(feature = "LuZap")]
    LuZap,
    #[cfg(feature = "LuZapOff")]
    LuZapOff,
    #[cfg(feature = "LuZoomIn")]
    LuZoomIn,
    #[cfg(feature = "LuZoomOut")]
    LuZoomOut,
}

#[cfg(feature = "LuAccessibility")]
const LU_ACCESSIBILITY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="16" cy="4" r="1" />
<path d="m18 19 1-7-6 1" />
<path d="m5 8 3-3 5.5 3-2.36 3.5" />
<path d="M4.24 14.5a5 5 0 0 0 6.88 6" />
<path d="M13.76 17.5a5 5 0 0 0-6.88-6" />"###
};
#[cfg(feature = "LuActivity")]
const LU_ACTIVITY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 12h-4l-3 9L9 3l-3 9H2" />"###
};
#[cfg(feature = "LuActivitySquare")]
const LU_ACTIVITY_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="M17 12h-2l-2 5-2-10-2 5H7" />"###
};
#[cfg(feature = "LuAirVent")]
const LU_AIR_VENT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 12H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2" />
<path d="M6 8h12" />
<path d="M18.3 17.7a2.5 2.5 0 0 1-3.16 3.83 2.53 2.53 0 0 1-1.14-2V12" />
<path d="M6.6 15.6A2 2 0 1 0 10 17v-5" />"###
};
#[cfg(feature = "LuAirplay")]
const LU_AIRPLAY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuAlarmCheck")]
const LU_ALARM_CHECK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="13" r="8" />
<path d="M5 3 2 6" />
<path d="m22 6-3-3" />
<path d="M6.38 18.7 4 21" />
<path d="M17.64 18.67 20 21" />
<path d="m9 13 2 2 4-4" />"###
};
#[cfg(feature = "LuAlarmClock")]
const LU_ALARM_CLOCK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="13" r="8" />
<path d="M12 9v4l2 2" />
<path d="M5 3 2 6" />
<path d="m22 6-3-3" />
<path d="M6.38 18.7 4 21" />
<path d="M17.64 18.67 20 21" />"###
};
#[cfg(feature = "LuAlarmClockOff")]
const LU_ALARM_CLOCK_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6.87 6.87a8 8 0 1 0 11.26 11.26" />
<path d="M19.9 14.25a8 8 0 0 0-9.15-9.15" />
<path d="m22 6-3-3" />
<path d="M6.26 18.67 4 21" />
<path d="m2 2 20 20" />
<path d="M4 4 2 6" />"###
};
#[cfg(feature = "LuAlarmMinus")]
const LU_ALARM_MINUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="13" r="8" />
<path d="M5 3 2 6" />
<path d="m22 6-3-3" />
<path d="M6.38 18.7 4 21" />
<path d="M17.64 18.67 20 21" />
<path d="M9 13h6" />"###
};
#[cfg(feature = "LuAlarmPlus")]
const LU_ALARM_PLUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="13" r="8" />
<path d="M5 3 2 6" />
<path d="m22 6-3-3" />
<path d="M6.38 18.7 4 21" />
<path d="M17.64 18.67 20 21" />
<path d="M12 10v6" />
<path d="M9 13h6" />"###
};
#[cfg(feature = "LuAlbum")]
const LU_ALBUM: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<polyline points="11 3 11 11 14 8 17 11 17 3" />"###
};
#[cfg(feature = "LuAlertCircle")]
const LU_ALERT_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="12" x2="12" y1="8" y2="12" />
<line x1="12" x2="12.01" y1="16" y2="16" />"###
};
#[cfg(feature = "LuAlertOctagon")]
const LU_ALERT_OCTAGON: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="12" x2="12" y1="8" y2="12" />
<line x1="12" x2="12.01" y1="16" y2="16" />"###
};
#[cfg(feature = "LuAlertTriangle")]
const LU_ALERT_TRIANGLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z" />
<path d="M12 9v4" />
<path d="M12 17h.01" />"###
};
#[cfg(feature = "LuAlignCenter")]
const LU_ALIGN_CENTER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="21" x2="3" y1="6" y2="6" />
<line x1="17" x2="7" y1="12" y2="12" />
<line x1="19" x2="5" y1="18" y2="18" />"###
};
#[cfg(feature = "LuAlignCenterHorizontal")]
const LU_ALIGN_CENTER_HORIZONTAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 12h20" />
<path d="M10 16v4a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2v-4" />
<path d="M10 8V4a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v4" />
<path d="M20 16v1a2 2 0 0 1-2 2h-2a2 2 0 0 1-2-2v-1" />
<path d="M14 8V7c0-1.1.9-2 2-2h2a2 2 0 0 1 2 2v1" />"###
};
#[cfg(feature = "LuAlignCenterVertical")]
const LU_ALIGN_CENTER_VERTICAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 2v20" />
<path d="M8 10H4a2 2 0 0 1-2-2V6c0-1.1.9-2 2-2h4" />
<path d="M16 10h4a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2h-4" />
<path d="M8 20H7a2 2 0 0 1-2-2v-2c0-1.1.9-2 2-2h1" />
<path d="M16 14h1a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2h-1" />"###
};
#[cfg(feature = "LuAlignEndHorizontal")]
const LU_ALIGN_END_HORIZONTAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="6" height="16" x="4" y="2" rx="2" />
<rect width="6" height="9" x="14" y="9" rx="2" />
<path d="M22 22H2" />"###
};
#[cfg(feature = "LuAlignEndVertical")]
const LU_ALIGN_END_VERTICAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="16" height="6" x="2" y="4" rx="2" />
<rect width="9" height="6" x="9" y="14" rx="2" />
<path d="M22 22V2" />"###
};
#[cfg(feature = "LuAlignHorizontalDistributeCenter")]
const LU_ALIGN_HORIZONTAL_DISTRIBUTE_CENTER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="6" height="14" x="4" y="5" rx="2" />
<rect width="6" height="10" x="14" y="7" rx="2" />
<path d="M17 22v-5" />
<path d="M17 7V2" />
<path d="M7 22v-3" />
<path d="M7 5V2" />"###
};
#[cfg(feature = "LuAlignHorizontalDistributeEnd")]
const LU_ALIGN_HORIZONTAL_DISTRIBUTE_END: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="6" height="14" x="4" y="5" rx="2" />
<rect width="6" height="10" x="14" y="7" rx="2" />
<path d="M10 2v20" />
<path d="M20 2v20" />"###
};
#[cfg(feature = "LuAlignHorizontalDistributeStart")]
const LU_ALIGN_HORIZONTAL_DISTRIBUTE_START: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="6" height="14" x="4" y="5" rx="2" />
<rect width="6" height="10" x="14" y="7" rx="2" />
<path d="M4 2v20" />
<path d="M14 2v20" />"###
};
#[cfg(feature = "LuAlignHorizontalJustifyCenter")]
const LU_ALIGN_HORIZONTAL_JUSTIFY_CENTER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="6" height="14" x="2" y="5" rx="2" />
<rect width="6" height="10" x="16" y="7" rx="2" />
<path d="M12 2v20" />"###
};
#[cfg(feature = "LuAlignHorizontalJustifyEnd")]
const LU_ALIGN_HORIZONTAL_JUSTIFY_END: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="6" height="14" x="2" y="5" rx="2" />
<rect width="6" height="10" x="12" y="7" rx="2" />
<path d="M22 2v20" />"###
};
#[cfg(feature = "LuAlignHorizontalJustifyStart")]
const LU_ALIGN_HORIZONTAL_JUSTIFY_START: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="6" height="14" x="6" y="5" rx="2" />
<rect width="6" height="10" x="16" y="7" rx="2" />
<path d="M2 2v20" />"###
};
#[cfg(feature = "LuAlignHorizontalSpaceAround")]
const LU_ALIGN_HORIZONTAL_SPACE_AROUND: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="6" height="10" x="9" y="7" rx="2" />
<path d="M4 22V2" />
<path d="M20 22V2" />"###
};
#[cfg(feature = "LuAlignHorizontalSpaceBetween")]
const LU_ALIGN_HORIZONTAL_SPACE_BETWEEN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="6" height="14" x="3" y="5" rx="2" />
<rect width="6" height="10" x="15" y="7" rx="2" />
<path d="M3 2v20" />
<path d="M21 2v20" />"###
};
#[cfg(feature = "LuAlignJustify")]
const LU_ALIGN_JUSTIFY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="3" x2="21" y1="6" y2="6" />
<line x1="3" x2="21" y1="12" y2="12" />
<line x1="3" x2="21" y1="18" y2="18" />"###
};
#[cfg(feature = "LuAlignLeft")]
const LU_ALIGN_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="21" x2="3" y1="6" y2="6" />
<line x1="15" x2="3" y1="12" y2="12" />
<line x1="17" x2="3" y1="18" y2="18" />"###
};
#[cfg(feature = "LuAlignRight")]
const LU_ALIGN_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="21" x2="3" y1="6" y2="6" />
<line x1="21" x2="9" y1="12" y2="12" />
<line x1="21" x2="7" y1="18" y2="18" />"###
};
#[cfg(feature = "LuAlignStartHorizontal")]
const LU_ALIGN_START_HORIZONTAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="6" height="16" x="4" y="6" rx="2" />
<rect width="6" height="9" x="14" y="6" rx="2" />
<path d="M22 2H2" />"###
};
#[cfg(feature = "LuAlignStartVertical")]
const LU_ALIGN_START_VERTICAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="9" height="6" x="6" y="14" rx="2" />
<rect width="16" height="6" x="6" y="4" rx="2" />
<path d="M2 2v20" />"###
};
#[cfg(feature = "LuAlignVerticalDistributeCenter")]
const LU_ALIGN_VERTICAL_DISTRIBUTE_CENTER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="14" height="6" x="5" y="14" rx="2" />
<rect width="10" height="6" x="7" y="4" rx="2" />
<path d="M22 7h-5" />
<path d="M7 7H1" />
<path d="M22 17h-3" />
<path d="M5 17H2" />"###
};
#[cfg(feature = "LuAlignVerticalDistributeEnd")]
const LU_ALIGN_VERTICAL_DISTRIBUTE_END: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="14" height="6" x="5" y="14" rx="2" />
<rect width="10" height="6" x="7" y="4" rx="2" />
<path d="M2 20h20" />
<path d="M2 10h20" />"###
};
#[cfg(feature = "LuAlignVerticalDistributeStart")]
const LU_ALIGN_VERTICAL_DISTRIBUTE_START: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="14" height="6" x="5" y="14" rx="2" />
<rect width="10" height="6" x="7" y="4" rx="2" />
<path d="M2 14h20" />
<path d="M2 4h20" />"###
};
#[cfg(feature = "LuAlignVerticalJustifyCenter")]
const LU_ALIGN_VERTICAL_JUSTIFY_CENTER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="14" height="6" x="5" y="16" rx="2" />
<rect width="10" height="6" x="7" y="2" rx="2" />
<path d="M2 12h20" />"###
};
#[cfg(feature = "LuAlignVerticalJustifyEnd")]
const LU_ALIGN_VERTICAL_JUSTIFY_END: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="14" height="6" x="5" y="12" rx="2" />
<rect width="10" height="6" x="7" y="2" rx="2" />
<path d="M2 22h20" />"###
};
#[cfg(feature = "LuAlignVerticalJustifyStart")]
const LU_ALIGN_VERTICAL_JUSTIFY_START: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="14" height="6" x="5" y="16" rx="2" />
<rect width="10" height="6" x="7" y="6" rx="2" />
<path d="M2 2h20" />"###
};
#[cfg(feature = "LuAlignVerticalSpaceAround")]
const LU_ALIGN_VERTICAL_SPACE_AROUND: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="10" height="6" x="7" y="9" rx="2" />
<path d="M22 20H2" />
<path d="M22 4H2" />"###
};
#[cfg(feature = "LuAlignVerticalSpaceBetween")]
const LU_ALIGN_VERTICAL_SPACE_BETWEEN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="14" height="6" x="5" y="15" rx="2" />
<rect width="10" height="6" x="7" y="3" rx="2" />
<path d="M2 21h20" />
<path d="M2 3h20" />"###
};
#[cfg(feature = "LuAmpersand")]
const LU_AMPERSAND: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17.5 12c0 4.4-3.6 8-8 8A4.5 4.5 0 0 1 5 15.5c0-6 8-4 8-8.5a3 3 0 1 0-6 0c0 3 2.5 8.5 12 13" />
<path d="M16 12h3" />"###
};
#[cfg(feature = "LuAmpersands")]
const LU_AMPERSANDS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10 17c-5-3-7-7-7-9a2 2 0 0 1 4 0c0 2.5-5 2.5-5 6 0 1.7 1.3 3 3 3 2.8 0 5-2.2 5-5" />
<path d="M22 17c-5-3-7-7-7-9a2 2 0 0 1 4 0c0 2.5-5 2.5-5 6 0 1.7 1.3 3 3 3 2.8 0 5-2.2 5-5" />"###
};
#[cfg(feature = "LuAnchor")]
const LU_ANCHOR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="12" x2="12" y1="22" y2="8" />
<path d="M5 12H2a10 10 0 0 0 20 0h-3" />"###
};
#[cfg(feature = "LuAngry")]
const LU_ANGRY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M7.5 8 10 9" />
<path d="m14 9 2.5-1" />
<path d="M9 10h0" />
<path d="M15 10h0" />"###
};
#[cfg(feature = "LuAnnoyed")]
const LU_ANNOYED: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M8 15h8" />
<path d="M8 9h2" />
<path d="M14 9h2" />"###
};
#[cfg(feature = "LuAntenna")]
const LU_ANTENNA: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 12 7 2" />
<path d="m7 12 5-10" />
<path d="m12 12 5-10" />
<path d="m17 12 5-10" />
<path d="M4.5 7h15" />
<path d="M12 16v6" />"###
};
#[cfg(feature = "LuAperture")]
const LU_APERTURE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="14.31" x2="20.05" y1="8" y2="17.94" />
<line x1="9.69" x2="21.17" y1="8" y2="8" />
<line x1="7.38" x2="13.12" y1="12" y2="2.06" />
<line x1="9.69" x2="3.95" y1="16" y2="6.06" />
<line x1="14.31" x2="2.83" y1="16" y2="16" />
<line x1="16.62" x2="10.88" y1="12" y2="21.94" />"###
};
#[cfg(feature = "LuAppWindow")]
const LU_APP_WINDOW: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="2" y="4" width="20" height="16" rx="2" />
<path d="M10 4v4" />
<path d="M2 8h20" />
<path d="M6 4v4" />"###
};
#[cfg(feature = "LuApple")]
const LU_APPLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 20.94c1.5 0 2.75 1.06 4 1.06 3 0 6-8 6-12.22A4.91 4.91 0 0 0 17 5c-2.22 0-4 1.44-5 2-1-.56-2.78-2-5-2a4.9 4.9 0 0 0-5 4.78C2 14 5 22 8 22c1.25 0 2.5-1.06 4-1.06Z" />
<path d="M10 2c1 .5 2 2 2 5" />"###
};
#[cfg(feature = "LuArchive")]
const LU_ARCHIVE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="20" height="5" x="2" y="4" rx="2" />
<path d="M4 9v9a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9" />
<path d="M10 13h4" />"###
};
#[cfg(feature = "LuArchiveRestore")]
const LU_ARCHIVE_RESTORE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="20" height="5" x="2" y="4" rx="2" />
<path d="M12 13v7" />
<path d="m9 16 3-3 3 3" />
<path d="M4 9v9a2 2 0 0 0 2 2h2" />
<path d="M20 9v9a2 2 0 0 1-2 2h-2" />"###
};
#[cfg(feature = "LuAreaChart")]
const LU_AREA_CHART: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 3v18h18" />
<path d="M7 12v5h12V8l-5 5-4-4Z" />"###
};
#[cfg(feature = "LuArmchair")]
const LU_ARMCHAIR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M19 9V6a2 2 0 0 0-2-2H7a2 2 0 0 0-2 2v3" />
<path d="M3 11v5a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-5a2 2 0 0 0-4 0v2H7v-2a2 2 0 0 0-4 0Z" />
<path d="M5 18v2" />
<path d="M19 18v2" />"###
};
#[cfg(feature = "LuArrowBigDown")]
const LU_ARROW_BIG_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M15 6v6h4l-7 7-7-7h4V6h6z" />"###
};
#[cfg(feature = "LuArrowBigDownDash")]
const LU_ARROW_BIG_DOWN_DASH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M15 5H9" />
<path d="M15 9v3h4l-7 7-7-7h4V9h6z" />"###
};
#[cfg(feature = "LuArrowBigLeft")]
const LU_ARROW_BIG_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18 15h-6v4l-7-7 7-7v4h6v6z" />"###
};
#[cfg(feature = "LuArrowBigLeftDash")]
const LU_ARROW_BIG_LEFT_DASH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M19 15V9" />
<path d="M15 15h-3v4l-7-7 7-7v4h3v6z" />"###
};
#[cfg(feature = "LuArrowBigRight")]
const LU_ARROW_BIG_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 9h6V5l7 7-7 7v-4H6V9z" />"###
};
#[cfg(feature = "LuArrowBigRightDash")]
const LU_ARROW_BIG_RIGHT_DASH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 9v6" />
<path d="M9 9h3V5l7 7-7 7v-4H9V9z" />"###
};
#[cfg(feature = "LuArrowBigUp")]
const LU_ARROW_BIG_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M9 18v-6H5l7-7 7 7h-4v6H9z" />"###
};
#[cfg(feature = "LuArrowBigUpDash")]
const LU_ARROW_BIG_UP_DASH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M9 19h6" />
<path d="M9 15v-3H5l7-7 7 7h-4v3H9z" />"###
};
#[cfg(feature = "LuArrowDown")]
const LU_ARROW_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 5v14" />
<path d="m19 12-7 7-7-7" />"###
};
#[cfg(feature = "LuArrowDown01")]
const LU_ARROW_DOWN01: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3 16 4 4 4-4" />
<path d="M7 20V4" />
<rect x="15" y="4" width="4" height="6" ry="2" />
<path d="M17 20v-6h-2" />
<path d="M15 20h4" />"###
};
#[cfg(feature = "LuArrowDown10")]
const LU_ARROW_DOWN10: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3 16 4 4 4-4" />
<path d="M7 20V4" />
<path d="M17 10V4h-2" />
<path d="M15 10h4" />
<rect x="15" y="14" width="4" height="6" ry="2" />"###
};
#[cfg(feature = "LuArrowDownAZ")]
const LU_ARROW_DOWN_AZ: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3 16 4 4 4-4" />
<path d="M7 20V4" />
<path d="M20 8h-5" />
<path d="M15 10V6.5a2.5 2.5 0 0 1 5 0V10" />
<path d="M15 14h5l-5 6h5" />"###
};
#[cfg(feature = "LuArrowDownCircle")]
const LU_ARROW_DOWN_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M12 8v8" />
<path d="m8 12 4 4 4-4" />"###
};
#[cfg(feature = "LuArrowDownFromLine")]
const LU_ARROW_DOWN_FROM_LINE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M19 3H5" />
<path d="M12 21V7" />
<path d="m6 15 6 6 6-6" />"###
};
#[cfg(feature = "LuArrowDownLeft")]
const LU_ARROW_DOWN_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17 7 7 17" />
<path d="M17 17H7V7" />"###
};
#[cfg(feature = "LuArrowDownLeftFromCircle")]
const LU_ARROW_DOWN_LEFT_FROM_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 12a10 10 0 1 1 10 10" />
<path d="m2 22 10-10" />
<path d="M8 22H2v-6" />"###
};
#[cfg(feature = "LuArrowDownLeftSquare")]
const LU_ARROW_DOWN_LEFT_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="m16 8-8 8" />
<path d="M16 16H8V8" />"###
};
#[cfg(feature = "LuArrowDownNarrowWide")]
const LU_ARROW_DOWN_NARROW_WIDE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3 16 4 4 4-4" />
<path d="M7 20V4" />
<path d="M11 4h4" />
<path d="M11 8h7" />
<path d="M11 12h10" />"###
};
#[cfg(feature = "LuArrowDownRight")]
const LU_ARROW_DOWN_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m7 7 10 10" />
<path d="M17 7v10H7" />"###
};
#[cfg(feature = "LuArrowDownRightFromCircle")]
const LU_ARROW_DOWN_RIGHT_FROM_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 22a10 10 0 1 1 10-10" />
<path d="M22 22 12 12" />
<path d="M22 16v6h-6" />"###
};
#[cfg(feature = "LuArrowDownRightSquare")]
const LU_ARROW_DOWN_RIGHT_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="m8 8 8 8" />
<path d="M16 8v8H8" />"###
};
#[cfg(feature = "LuArrowDownSquare")]
const LU_ARROW_DOWN_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="M12 8v8" />
<path d="m8 12 4 4 4-4" />"###
};
#[cfg(feature = "LuArrowDownToDot")]
const LU_ARROW_DOWN_TO_DOT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 2v14" />
<path d="m19 9-7 7-7-7" />
<circle cx="12" cy="21" r="1" />"###
};
#[cfg(feature = "LuArrowDownToLine")]
const LU_ARROW_DOWN_TO_LINE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 17V3" />
<path d="m6 11 6 6 6-6" />
<path d="M19 21H5" />"###
};
#[cfg(feature = "LuArrowDownUp")]
const LU_ARROW_DOWN_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3 16 4 4 4-4" />
<path d="M7 20V4" />
<path d="m21 8-4-4-4 4" />
<path d="M17 4v16" />"###
};
#[cfg(feature = "LuArrowDownWideNarrow")]
const LU_ARROW_DOWN_WIDE_NARROW: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3 16 4 4 4-4" />
<path d="M7 20V4" />
<path d="M11 4h10" />
<path d="M11 8h7" />
<path d="M11 12h4" />"###
};
#[cfg(feature = "LuArrowDownZA")]
const LU_ARROW_DOWN_ZA: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3 16 4 4 4-4" />
<path d="M7 4v16" />
<path d="M15 4h5l-5 6h5" />
<path d="M15 20v-3.5a2.5 2.5 0 0 1 5 0V20" />
<path d="M20 18h-5" />"###
};
#[cfg(feature = "LuArrowLeft")]
const LU_ARROW_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m12 19-7-7 7-7" />
<path d="M19 12H5" />"###
};
#[cfg(feature = "LuArrowLeftCircle")]
const LU_ARROW_LEFT_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M16 12H8" />
<path d="m12 8-4 4 4 4" />"###
};
#[cfg(feature = "LuArrowLeftFromLine")]
const LU_ARROW_LEFT_FROM_LINE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m9 6-6 6 6 6" />
<path d="M3 12h14" />
<path d="M21 19V5" />"###
};
#[cfg(feature = "LuArrowLeftRight")]
const LU_ARROW_LEFT_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 3 4 7l4 4" />
<path d="M4 7h16" />
<path d="m16 21 4-4-4-4" />
<path d="M20 17H4" />"###
};
#[cfg(feature = "LuArrowLeftSquare")]
const LU_ARROW_LEFT_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="m12 8-4 4 4 4" />
<path d="M16 12H8" />"###
};
#[cfg(feature = "LuArrowLeftToLine")]
const LU_ARROW_LEFT_TO_LINE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 19V5" />
<path d="m13 6-6 6 6 6" />
<path d="M7 12h14" />"###
};
#[cfg(feature = "LuArrowRight")]
const LU_ARROW_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 12h14" />
<path d="m12 5 7 7-7 7" />"###
};
#[cfg(feature = "LuArrowRightCircle")]
const LU_ARROW_RIGHT_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M8 12h8" />
<path d="m12 16 4-4-4-4" />"###
};
#[cfg(feature = "LuArrowRightFromLine")]
const LU_ARROW_RIGHT_FROM_LINE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 5v14" />
<path d="M21 12H7" />
<path d="m15 18 6-6-6-6" />"###
};
#[cfg(feature = "LuArrowRightLeft")]
const LU_ARROW_RIGHT_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m16 3 4 4-4 4" />
<path d="M20 7H4" />
<path d="m8 21-4-4 4-4" />
<path d="M4 17h16" />"###
};
#[cfg(feature = "LuArrowRightSquare")]
const LU_ARROW_RIGHT_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="M8 12h8" />
<path d="m12 16 4-4-4-4" />"###
};
#[cfg(feature = "LuArrowRightToLine")]
const LU_ARROW_RIGHT_TO_LINE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17 12H3" />
<path d="m11 18 6-6-6-6" />
<path d="M21 5v14" />"###
};
#[cfg(feature = "LuArrowUp")]
const LU_ARROW_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m5 12 7-7 7 7" />
<path d="M12 19V5" />"###
};
#[cfg(feature = "LuArrowUp01")]
const LU_ARROW_UP01: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3 8 4-4 4 4" />
<path d="M7 4v16" />
<rect x="15" y="4" width="4" height="6" ry="2" />
<path d="M17 20v-6h-2" />
<path d="M15 20h4" />"###
};
#[cfg(feature = "LuArrowUp10")]
const LU_ARROW_UP10: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3 8 4-4 4 4" />
<path d="M7 4v16" />
<path d="M17 10V4h-2" />
<path d="M15 10h4" />
<rect x="15" y="14" width="4" height="6" ry="2" />"###
};
#[cfg(feature = "LuArrowUpAZ")]
const LU_ARROW_UP_AZ: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3 8 4-4 4 4" />
<path d="M7 4v16" />
<path d="M20 8h-5" />
<path d="M15 10V6.5a2.5 2.5 0 0 1 5 0V10" />
<path d="M15 14h5l-5 6h5" />"###
};
#[cfg(feature = "LuArrowUpCircle")]
const LU_ARROW_UP_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="m16 12-4-4-4 4" />
<path d="M12 16V8" />"###
};
#[cfg(feature = "LuArrowUpDown")]
const LU_ARROW_UP_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m21 16-4 4-4-4" />
<path d="M17 20V4" />
<path d="m3 8 4-4 4 4" />
<path d="M7 4v16" />"###
};
#[cfg(feature = "LuArrowUpFromDot")]
const LU_ARROW_UP_FROM_DOT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m5 9 7-7 7 7" />
<path d="M12 16V2" />
<circle cx="12" cy="21" r="1" />"###
};
#[cfg(feature = "LuArrowUpFromLine")]
const LU_ARROW_UP_FROM_LINE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m18 9-6-6-6 6" />
<path d="M12 3v14" />
<path d="M5 21h14" />"###
};
#[cfg(feature = "LuArrowUpLeft")]
const LU_ARROW_UP_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M7 17V7h10" />
<path d="M17 17 7 7" />"###
};
#[cfg(feature = "LuArrowUpLeftFromCircle")]
const LU_ARROW_UP_LEFT_FROM_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 8V2h6" />
<path d="m2 2 10 10" />
<path d="M12 2A10 10 0 1 1 2 12" />"###
};
#[cfg(feature = "LuArrowUpLeftSquare")]
const LU_ARROW_UP_LEFT_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="M8 16V8h8" />
<path d="M16 16 8 8" />"###
};
#[cfg(feature = "LuArrowUpNarrowWide")]
const LU_ARROW_UP_NARROW_WIDE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3 8 4-4 4 4" />
<path d="M7 4v16" />
<path d="M11 12h4" />
<path d="M11 16h7" />
<path d="M11 20h10" />"###
};
#[cfg(feature = "LuArrowUpRight")]
const LU_ARROW_UP_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M7 7h10v10" />
<path d="M7 17 17 7" />"###
};
#[cfg(feature = "LuArrowUpRightFromCircle")]
const LU_ARROW_UP_RIGHT_FROM_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 12A10 10 0 1 1 12 2" />
<path d="M22 2 12 12" />
<path d="M16 2h6v6" />"###
};
#[cfg(feature = "LuArrowUpRightSquare")]
const LU_ARROW_UP_RIGHT_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="M8 8h8v8" />
<path d="m8 16 8-8" />"###
};
#[cfg(feature = "LuArrowUpSquare")]
const LU_ARROW_UP_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="m16 12-4-4-4 4" />
<path d="M12 16V8" />"###
};
#[cfg(feature = "LuArrowUpToLine")]
const LU_ARROW_UP_TO_LINE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 3h14" />
<path d="m18 13-6-6-6 6" />
<path d="M12 7v14" />"###
};
#[cfg(feature = "LuArrowUpWideNarrow")]
const LU_ARROW_UP_WIDE_NARROW: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3 8 4-4 4 4" />
<path d="M7 4v16" />
<path d="M11 12h10" />
<path d="M11 16h7" />
<path d="M11 20h4" />"###
};
#[cfg(feature = "LuArrowUpZA")]
const LU_ARROW_UP_ZA: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3 8 4-4 4 4" />
<path d="M7 4v16" />
<path d="M15 4h5l-5 6h5" />
<path d="M15 20v-3.5a2.5 2.5 0 0 1 5 0V20" />
<path d="M20 18h-5" />"###
};
#[cfg(feature = "LuArrowsUpFromLine")]
const LU_ARROWS_UP_FROM_LINE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m4 6 3-3 3 3" />
<path d="M7 17V3" />
<path d="m14 6 3-3 3 3" />
<path d="M17 17V3" />
<path d="M4 21h16" />"###
};
#[cfg(feature = "LuAsterisk")]
const LU_ASTERISK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 6v12" />
<path d="M17.196 9 6.804 15" />
<path d="m6.804 9 10.392 6" />"###
};
#[cfg(feature = "LuAtSign")]
const LU_AT_SIGN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M16 8v5a3 3 0 0 0 6 0v-1a10 10 0 1 0-4 8" />"###
};
#[cfg(feature = "LuAtom")]
const LU_ATOM: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M20.2 20.2c2.04-2.03.02-7.36-4.5-11.9-4.54-4.52-9.87-6.54-11.9-4.5-2.04 2.03-.02 7.36 4.5 11.9 4.54 4.52 9.87 6.54 11.9 4.5Z" />
<path d="M15.7 15.7c4.52-4.54 6.54-9.87 4.5-11.9-2.03-2.04-7.36-.02-11.9 4.5-4.52 4.54-6.54 9.87-4.5 11.9 2.03 2.04 7.36.02 11.9-4.5Z" />"###
};
#[cfg(feature = "LuAward")]
const LU_AWARD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="8" r="6" />
<path d="M15.477 12.89 17 22l-5-3-5 3 1.523-9.11" />"###
};
#[cfg(feature = "LuAxe")]
const LU_AXE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m14 12-8.5 8.5a2.12 2.12 0 1 1-3-3L11 9" />
<path d="M15 13 9 7l4-4 6 6h3a8 8 0 0 1-7 7z" />"###
};
#[cfg(feature = "LuAxis3d")]
const LU_AXIS3D: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 4v16h16" />
<path d="m4 20 7-7" />"###
};
#[cfg(feature = "LuBaby")]
const LU_BABY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M9 12h.01" />
<path d="M15 12h.01" />
<path d="M10 16c.5.3 1.2.5 2 .5s1.5-.2 2-.5" />
<path d="M19 6.3a9 9 0 0 1 1.8 3.9 2 2 0 0 1 0 3.6 9 9 0 0 1-17.6 0 2 2 0 0 1 0-3.6A9 9 0 0 1 12 3c2 0 3.5 1.1 3.5 2.5s-.9 2.5-2 2.5c-.8 0-1.5-.4-1.5-1" />"###
};
#[cfg(feature = "LuBackpack")]
const LU_BACKPACK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 20V10a4 4 0 0 1 4-4h8a4 4 0 0 1 4 4v10a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2Z" />
<path d="M9 6V4a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2v2" />
<path d="M8 21v-5a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v5" />
<path d="M8 10h8" />
<path d="M8 18h8" />"###
};
#[cfg(feature = "LuBadge")]
const LU_BADGE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />"###
};
#[cfg(feature = "LuBadgeAlert")]
const LU_BADGE_ALERT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />
<line x1="12" x2="12" y1="8" y2="12" />
<line x1="12" x2="12.01" y1="16" y2="16" />"###
};
#[cfg(feature = "LuBadgeCheck")]
const LU_BADGE_CHECK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />
<path d="m9 12 2 2 4-4" />"###
};
#[cfg(feature = "LuBadgeDollarSign")]
const LU_BADGE_DOLLAR_SIGN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />
<path d="M16 8h-6a2 2 0 1 0 0 4h4a2 2 0 1 1 0 4H8" />
<path d="M12 18V6" />"###
};
#[cfg(feature = "LuBadgeHelp")]
const LU_BADGE_HELP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />
<path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" />
<line x1="12" x2="12.01" y1="17" y2="17" />"###
};
#[cfg(feature = "LuBadgeInfo")]
const LU_BADGE_INFO: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />
<line x1="12" x2="12" y1="16" y2="12" />
<line x1="12" x2="12.01" y1="8" y2="8" />"###
};
#[cfg(feature = "LuBadgeMinus")]
const LU_BADGE_MINUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />
<line x1="8" x2="16" y1="12" y2="12" />"###
};
#[cfg(feature = "LuBadgePercent")]
const LU_BADGE_PERCENT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />
<path d="m15 9-6 6" />
<path d="M9 9h.01" />
<path d="M15 15h.01" />"###
};
#[cfg(feature = "LuBadgePlus")]
const LU_BADGE_PLUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />
<line x1="12" x2="12" y1="8" y2="16" />
<line x1="8" x2="16" y1="12" y2="12" />"###
};
#[cfg(feature = "LuBadgeX")]
const LU_BADGE_X: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />
<line x1="15" x2="9" y1="9" y2="15" />
<line x1="9" x2="15" y1="9" y2="15" />"###
};
#[cfg(feature = "LuBaggageClaim")]
const LU_BAGGAGE_CLAIM: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 18H6a2 2 0 0 1-2-2V7a2 2 0 0 0-2-2" />
<path d="M17 14V4a2 2 0 0 0-2-2h-1a2 2 0 0 0-2 2v10" />
<rect width="13" height="8" x="8" y="6" rx="1" />
<circle cx="18" cy="20" r="2" />
<circle cx="9" cy="20" r="2" />"###
};
#[cfg(feature = "LuBan")]
const LU_BAN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="m4.9 4.9 14.2 14.2" />"###
};
#[cfg(feature = "LuBanana")]
const LU_BANANA: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 13c3.5-2 8-2 10 2a5.5 5.5 0 0 1 8 5" />
<path d="M5.15 17.89c5.52-1.52 8.65-6.89 7-12C11.55 4 11.5 2 13 2c3.22 0 5 5.5 5 8 0 6.5-4.2 12-10.49 12C5.11 22 2 22 2 20c0-1.5 1.14-1.55 3.15-2.11Z" />"###
};
#[cfg(feature = "LuBanknote")]
const LU_BANKNOTE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="20" height="12" x="2" y="6" rx="2" />
<circle cx="12" cy="12" r="2" />
<path d="M6 12h.01M18 12h.01" />"###
};
#[cfg(feature = "LuBarChart")]
const LU_BAR_CHART: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="12" x2="12" y1="20" y2="10" />
<line x1="18" x2="18" y1="20" y2="4" />
<line x1="6" x2="6" y1="20" y2="16" />"###
};
#[cfg(feature = "LuBarChart2")]
const LU_BAR_CHART2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="18" x2="18" y1="20" y2="10" />
<line x1="12" x2="12" y1="20" y2="4" />
<line x1="6" x2="6" y1="20" y2="14" />"###
};
#[cfg(feature = "LuBarChart3")]
const LU_BAR_CHART3: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 3v18h18" />
<path d="M18 17V9" />
<path d="M13 17V5" />
<path d="M8 17v-3" />"###
};
#[cfg(feature = "LuBarChart4")]
const LU_BAR_CHART4: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 3v18h18" />
<path d="M13 17V9" />
<path d="M18 17V5" />
<path d="M8 17v-3" />"###
};
#[cfg(feature = "LuBarChartBig")]
const LU_BAR_CHART_BIG: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 3v18h18" />
<rect width="4" height="7" x="7" y="10" rx="1" />
<rect width="4" height="12" x="15" y="5" rx="1" />"###
};
#[cfg(feature = "LuBarChartHorizontal")]
const LU_BAR_CHART_HORIZONTAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 3v18h18" />
<path d="M7 16h8" />
<path d="M7 11h12" />
<path d="M7 6h3" />"###
};
#[cfg(feature = "LuBarChartHorizontalBig")]
const LU_BAR_CHART_HORIZONTAL_BIG: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 3v18h18" />
<rect width="12" height="4" x="7" y="5" rx="1" />
<rect width="7" height="4" x="7" y="13" rx="1" />"###
};
#[cfg(feature = "LuBaseline")]
const LU_BASELINE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 20h16" />
<path d="m6 16 6-12 6 12" />
<path d="M8 12h8" />"###
};
#[cfg(feature = "LuBath")]
const LU_BATH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M9 6 6.5 3.5a1.5 1.5 0 0 0-1-.5C4.683 3 4 3.683 4 4.5V17a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-5" />
<line x1="10" x2="8" y1="5" y2="7" />
<line x1="2" x2="22" y1="12" y2="12" />
<line x1="7" x2="7" y1="19" y2="21" />
<line x1="17" x2="17" y1="19" y2="21" />"###
};
#[cfg(feature = "LuBattery")]
const LU_BATTERY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="16" height="10" x="2" y="7" rx="2" ry="2" />
<line x1="22" x2="22" y1="11" y2="13" />"###
};
#[cfg(feature = "LuBatteryCharging")]
const LU_BATTERY_CHARGING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M15 7h1a2 2 0 0 1 2 2v6a2 2 0 0 1-2 2h-2" />
<path d="M6 7H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h1" />
<path d="m11 7-3 5h4l-3 5" />
<line x1="22" x2="22" y1="11" y2="13" />"###
};
#[cfg(feature = "LuBatteryFull")]
const LU_BATTERY_FULL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="16" height="10" x="2" y="7" rx="2" ry="2" />
<line x1="22" x2="22" y1="11" y2="13" />
<line x1="6" x2="6" y1="11" y2="13" />
<line x1="10" x2="10" y1="11" y2="13" />
<line x1="14" x2="14" y1="11" y2="13" />"###
};
#[cfg(feature = "LuBatteryLow")]
const LU_BATTERY_LOW: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="16" height="10" x="2" y="7" rx="2" ry="2" />
<line x1="22" x2="22" y1="11" y2="13" />
<line x1="6" x2="6" y1="11" y2="13" />"###
};
#[cfg(feature = "LuBatteryMedium")]
const LU_BATTERY_MEDIUM: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="16" height="10" x="2" y="7" rx="2" ry="2" />
<line x1="22" x2="22" y1="11" y2="13" />
<line x1="6" x2="6" y1="11" y2="13" />
<line x1="10" x2="10" y1="11" y2="13" />"###
};
#[cfg(feature = "LuBatteryWarning")]
const LU_BATTERY_WARNING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14 7h2a2 2 0 0 1 2 2v6c0 1-1 2-2 2h-2" />
<path d="M6 7H4a2 2 0 0 0-2 2v6c0 1 1 2 2 2h2" />
<line x1="22" x2="22" y1="11" y2="13" />
<line x1="10" x2="10" y1="7" y2="13" />
<line x1="10" x2="10" y1="17" y2="17.01" />"###
};
#[cfg(feature = "LuBeaker")]
const LU_BEAKER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4.5 3h15" />
<path d="M6 3v16a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V3" />
<path d="M6 14h12" />"###
};
#[cfg(feature = "LuBean")]
const LU_BEAN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10.165 6.598C9.954 7.478 9.64 8.36 9 9c-.64.64-1.521.954-2.402 1.165A6 6 0 0 0 8 22c7.732 0 14-6.268 14-14a6 6 0 0 0-11.835-1.402Z" />
<path d="M5.341 10.62a4 4 0 1 0 5.279-5.28" />"###
};
#[cfg(feature = "LuBeanOff")]
const LU_BEAN_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M9 9c-.64.64-1.521.954-2.402 1.165A6 6 0 0 0 8 22a13.96 13.96 0 0 0 9.9-4.1" />
<path d="M10.75 5.093A6 6 0 0 1 22 8c0 2.411-.61 4.68-1.683 6.66" />
<path d="M5.341 10.62a4 4 0 0 0 6.487 1.208M10.62 5.341a4.015 4.015 0 0 1 2.039 2.04" />
<line x1="2" x2="22" y1="2" y2="22" />"###
};
#[cfg(feature = "LuBed")]
const LU_BED: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 4v16" />
<path d="M2 8h18a2 2 0 0 1 2 2v10" />
<path d="M2 17h20" />
<path d="M6 8v9" />"###
};
#[cfg(feature = "LuBedDouble")]
const LU_BED_DOUBLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 20v-8a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v8" />
<path d="M4 10V6a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v4" />
<path d="M12 4v6" />
<path d="M2 18h20" />"###
};
#[cfg(feature = "LuBedSingle")]
const LU_BED_SINGLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 20v-8a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v8" />
<path d="M5 10V6a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v4" />
<path d="M3 18h18" />"###
};
#[cfg(feature = "LuBeef")]
const LU_BEEF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12.5" cy="8.5" r="2.5" />
<path d="M12.5 2a6.5 6.5 0 0 0-6.22 4.6c-1.1 3.13-.78 3.9-3.18 6.08A3 3 0 0 0 5 18c4 0 8.4-1.8 11.4-4.3A6.5 6.5 0 0 0 12.5 2Z" />
<path d="m18.5 6 2.19 4.5a6.48 6.48 0 0 1 .31 2 6.49 6.49 0 0 1-2.6 5.2C15.4 20.2 11 22 7 22a3 3 0 0 1-2.68-1.66L2.4 16.5" />"###
};
#[cfg(feature = "LuBeer")]
const LU_BEER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17 11h1a3 3 0 0 1 0 6h-1" />
<path d="M9 12v6" />
<path d="M13 12v6" />
<path d="M14 7.5c-1 0-1.44.5-3 .5s-2-.5-3-.5-1.72.5-2.5.5a2.5 2.5 0 0 1 0-5c.78 0 1.57.5 2.5.5S9.44 2 11 2s2 1.5 3 1.5 1.72-.5 2.5-.5a2.5 2.5 0 0 1 0 5c-.78 0-1.5-.5-2.5-.5Z" />
<path d="M5 8v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V8" />"###
};
#[cfg(feature = "LuBell")]
const LU_BELL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9" />
<path d="M10.3 21a1.94 1.94 0 0 0 3.4 0" />"###
};
#[cfg(feature = "LuBellDot")]
const LU_BELL_DOT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M19.4 14.9C20.2 16.4 21 17 21 17H3s3-2 3-9c0-3.3 2.7-6 6-6 .7 0 1.3.1 1.9.3" />
<path d="M10.3 21a1.94 1.94 0 0 0 3.4 0" />
<circle cx="18" cy="8" r="3" />"###
};
#[cfg(feature = "LuBellMinus")]
const LU_BELL_MINUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18.4 12c.8 3.8 2.6 5 2.6 5H3s3-2 3-9c0-3.3 2.7-6 6-6 1.8 0 3.4.8 4.5 2" />
<path d="M10.3 21a1.94 1.94 0 0 0 3.4 0" />
<path d="M15 8h6" />"###
};
#[cfg(feature = "LuBellOff")]
const LU_BELL_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8.7 3A6 6 0 0 1 18 8a21.3 21.3 0 0 0 .6 5" />
<path d="M17 17H3s3-2 3-9a4.67 4.67 0 0 1 .3-1.7" />
<path d="M10.3 21a1.94 1.94 0 0 0 3.4 0" />
<path d="m2 2 20 20" />"###
};
#[cfg(feature = "LuBellPlus")]
const LU_BELL_PLUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M19.3 14.8C20.1 16.4 21 17 21 17H3s3-2 3-9c0-3.3 2.7-6 6-6 1 0 1.9.2 2.8.7" />
<path d="M10.3 21a1.94 1.94 0 0 0 3.4 0" />
<path d="M15 8h6" />
<path d="M18 5v6" />"###
};
#[cfg(feature = "LuBellRing")]
const LU_BELL_RING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9" />
<path d="M10.3 21a1.94 1.94 0 0 0 3.4 0" />
<path d="M4 2C2.8 3.7 2 5.7 2 8" />
<path d="M22 8c0-2.3-.8-4.3-2-6" />"###
};
#[cfg(feature = "LuBike")]
const LU_BIKE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="18.5" cy="17.5" r="3.5" />
<circle cx="5.5" cy="17.5" r="3.5" />
<circle cx="15" cy="5" r="1" />
<path d="M12 17.5V14l-3-3 4-3 2 3h2" />"###
};
#[cfg(feature = "LuBinary")]
const LU_BINARY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="14" y="14" width="4" height="6" rx="2" />
<rect x="6" y="4" width="4" height="6" rx="2" />
<path d="M6 20h4" />
<path d="M14 10h4" />
<path d="M6 14h2v6" />
<path d="M14 4h2v6" />"###
};
#[cfg(feature = "LuBiohazard")]
const LU_BIOHAZARD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="11.9" r="2" />
<path d="M6.7 3.4c-.9 2.5 0 5.2 2.2 6.7C6.5 9 3.7 9.6 2 11.6" />
<path d="m8.9 10.1 1.4.8" />
<path d="M17.3 3.4c.9 2.5 0 5.2-2.2 6.7 2.4-1.2 5.2-.6 6.9 1.5" />
<path d="m15.1 10.1-1.4.8" />
<path d="M16.7 20.8c-2.6-.4-4.6-2.6-4.7-5.3-.2 2.6-2.1 4.8-4.7 5.2" />
<path d="M12 13.9v1.6" />
<path d="M13.5 5.4c-1-.2-2-.2-3 0" />
<path d="M17 16.4c.7-.7 1.2-1.6 1.5-2.5" />
<path d="M5.5 13.9c.3.9.8 1.8 1.5 2.5" />"###
};
#[cfg(feature = "LuBird")]
const LU_BIRD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16 7h.01" />
<path d="M3.4 18H12a8 8 0 0 0 8-8V7a4 4 0 0 0-7.28-2.3L2 20" />
<path d="m20 7 2 .5-2 .5" />
<path d="M10 18v3" />
<path d="M14 17.75V21" />
<path d="M7 18a6 6 0 0 0 3.84-10.61" />"###
};
#[cfg(feature = "LuBitcoin")]
const LU_BITCOIN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M11.767 19.089c4.924.868 6.14-6.025 1.216-6.894m-1.216 6.894L5.86 18.047m5.908 1.042-.347 1.97m1.563-8.864c4.924.869 6.14-6.025 1.215-6.893m-1.215 6.893-3.94-.694m5.155-6.2L8.29 4.26m5.908 1.042.348-1.97M7.48 20.364l3.126-17.727" />"###
};
#[cfg(feature = "LuBlinds")]
const LU_BLINDS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 3h18" />
<path d="M20 7H8" />
<path d="M20 11H8" />
<path d="M10 19h10" />
<path d="M8 15h12" />
<path d="M4 3v14" />
<circle cx="4" cy="19" r="2" />"###
};
#[cfg(feature = "LuBlocks")]
const LU_BLOCKS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="7" height="7" x="14" y="3" rx="1" />
<path d="M10 21V8a1 1 0 0 0-1-1H4a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-5a1 1 0 0 0-1-1H3" />"###
};
#[cfg(feature = "LuBluetooth")]
const LU_BLUETOOTH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m7 7 10 10-5 5V2l5 5L7 17" />"###
};
#[cfg(feature = "LuBluetoothConnected")]
const LU_BLUETOOTH_CONNECTED: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m7 7 10 10-5 5V2l5 5L7 17" />
<line x1="18" x2="21" y1="12" y2="12" />
<line x1="3" x2="6" y1="12" y2="12" />"###
};
#[cfg(feature = "LuBluetoothOff")]
const LU_BLUETOOTH_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m17 17-5 5V12l-5 5" />
<path d="m2 2 20 20" />
<path d="M14.5 9.5 17 7l-5-5v4.5" />"###
};
#[cfg(feature = "LuBluetoothSearching")]
const LU_BLUETOOTH_SEARCHING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m7 7 10 10-5 5V2l5 5L7 17" />
<path d="M20.83 14.83a4 4 0 0 0 0-5.66" />
<path d="M18 12h.01" />"###
};
#[cfg(feature = "LuBold")]
const LU_BOLD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14 12a4 4 0 0 0 0-8H6v8" />
<path d="M15 20a4 4 0 0 0 0-8H6v8Z" />"###
};
#[cfg(feature = "LuBomb")]
const LU_BOMB: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="11" cy="13" r="9" />
<path d="m19.5 9.5 1.8-1.8a2.4 2.4 0 0 0 0-3.4l-1.6-1.6a2.41 2.41 0 0 0-3.4 0l-1.8 1.8" />
<path d="m22 2-1.5 1.5" />"###
};
#[cfg(feature = "LuBone")]
const LU_BONE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17 10c.7-.7 1.69 0 2.5 0a2.5 2.5 0 1 0 0-5 .5.5 0 0 1-.5-.5 2.5 2.5 0 1 0-5 0c0 .81.7 1.8 0 2.5l-7 7c-.7.7-1.69 0-2.5 0a2.5 2.5 0 0 0 0 5c.28 0 .5.22.5.5a2.5 2.5 0 1 0 5 0c0-.81-.7-1.8 0-2.5Z" />"###
};
#[cfg(feature = "LuBook")]
const LU_BOOK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" />"###
};
#[cfg(feature = "LuBookCopy")]
const LU_BOOK_COPY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 16V4a2 2 0 0 1 2-2h11" />
<path d="M5 14H4a2 2 0 1 0 0 4h1" />
<path d="M22 18H11a2 2 0 1 0 0 4h11V6H11a2 2 0 0 0-2 2v12" />"###
};
#[cfg(feature = "LuBookDown")]
const LU_BOOK_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" />
<path d="M12 13V7" />
<path d="m9 10 3 3 3-3" />"###
};
#[cfg(feature = "LuBookKey")]
const LU_BOOK_KEY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H14" />
<path d="M20 8v14H6.5a2.5 2.5 0 0 1 0-5H20" />
<circle cx="14" cy="8" r="2" />
<path d="m20 2-4.5 4.5" />
<path d="m19 3 1 1" />"###
};
#[cfg(feature = "LuBookLock")]
const LU_BOOK_LOCK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H10" />
<path d="M20 15v7H6.5a2.5 2.5 0 0 1 0-5H20" />
<rect width="8" height="5" x="12" y="6" rx="1" />
<path d="M18 6V4a2 2 0 1 0-4 0v2" />"###
};
#[cfg(feature = "LuBookMarked")]
const LU_BOOK_MARKED: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" />
<polyline points="10 2 10 10 13 7 16 10 16 2" />"###
};
#[cfg(feature = "LuBookMinus")]
const LU_BOOK_MINUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" />
<path d="M9 10h6" />"###
};
#[cfg(feature = "LuBookOpen")]
const LU_BOOK_OPEN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuBookOpenCheck")]
const LU_BOOK_OPEN_CHECK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 3H2v15h7c1.7 0 3 1.3 3 3V7c0-2.2-1.8-4-4-4Z" />
<path d="m16 12 2 2 4-4" />
<path d="M22 6V3h-6c-2.2 0-4 1.8-4 4v14c0-1.7 1.3-3 3-3h7v-2.3" />"###
};
#[cfg(feature = "LuBookPlus")]
const LU_BOOK_PLUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" />
<path d="M9 10h6" />
<path d="M12 7v6" />"###
};
#[cfg(feature = "LuBookTemplate")]
const LU_BOOK_TEMPLATE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M20 22h-2" />
<path d="M20 15v2h-2" />
<path d="M4 19.5V15" />
<path d="M20 8v3" />
<path d="M18 2h2v2" />
<path d="M4 11V9" />
<path d="M12 2h2" />
<path d="M12 22h2" />
<path d="M12 17h2" />
<path d="M8 22H6.5a2.5 2.5 0 0 1 0-5H8" />
<path d="M4 5v-.5A2.5 2.5 0 0 1 6.5 2H8" />"###
};
#[cfg(feature = "LuBookUp")]
const LU_BOOK_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" />
<path d="M12 13V7" />
<path d="m9 10 3-3 3 3" />"###
};
#[cfg(feature = "LuBookUp2")]
const LU_BOOK_UP2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2" />
<path d="M18 2h2v20H6.5a2.5 2.5 0 0 1 0-5H20" />
<path d="M12 13V7" />
<path d="m9 10 3-3 3 3" />
<path d="m9 5 3-3 3 3" />"###
};
#[cfg(feature = "LuBookX")]
const LU_BOOK_X: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" />
<path d="m14.5 7-5 5" />
<path d="m9.5 7 5 5" />"###
};
#[cfg(feature = "LuBookmark")]
const LU_BOOKMARK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z" />"###
};
#[cfg(feature = "LuBookmarkMinus")]
const LU_BOOKMARK_MINUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z" />
<line x1="15" x2="9" y1="10" y2="10" />"###
};
#[cfg(feature = "LuBookmarkPlus")]
const LU_BOOKMARK_PLUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z" />
<line x1="12" x2="12" y1="7" y2="13" />
<line x1="15" x2="9" y1="10" y2="10" />"###
};
#[cfg(feature = "LuBoomBox")]
const LU_BOOM_BOX: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 9V5a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v4" />
<path d="M8 8v1" />
<path d="M12 8v1" />
<path d="M16 8v1" />
<rect width="20" height="12" x="2" y="9" rx="2" />
<circle cx="8" cy="15" r="2" />
<circle cx="16" cy="15" r="2" />"###
};
#[cfg(feature = "LuBot")]
const LU_BOT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="10" x="3" y="11" rx="2" />
<circle cx="12" cy="5" r="2" />
<path d="M12 7v4" />
<line x1="8" x2="8" y1="16" y2="16" />
<line x1="16" x2="16" y1="16" y2="16" />"###
};
#[cfg(feature = "LuBox")]
const LU_BOX: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<polyline points="3.29 7 12 12 20.71 7" />
<line x1="12" x2="12" y1="22" y2="12" />"###
};
#[cfg(feature = "LuBoxSelect")]
const LU_BOX_SELECT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 3a2 2 0 0 0-2 2" />
<path d="M19 3a2 2 0 0 1 2 2" />
<path d="M21 19a2 2 0 0 1-2 2" />
<path d="M5 21a2 2 0 0 1-2-2" />
<path d="M9 3h1" />
<path d="M9 21h1" />
<path d="M14 3h1" />
<path d="M14 21h1" />
<path d="M3 9v1" />
<path d="M21 9v1" />
<path d="M3 14v1" />
<path d="M21 14v1" />"###
};
#[cfg(feature = "LuBoxes")]
const LU_BOXES: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2.97 12.92A2 2 0 0 0 2 14.63v3.24a2 2 0 0 0 .97 1.71l3 1.8a2 2 0 0 0 2.06 0L12 19v-5.5l-5-3-4.03 2.42Z" />
<path d="m7 16.5-4.74-2.85" />
<path d="m7 16.5 5-3" />
<path d="M7 16.5v5.17" />
<path d="M12 13.5V19l3.97 2.38a2 2 0 0 0 2.06 0l3-1.8a2 2 0 0 0 .97-1.71v-3.24a2 2 0 0 0-.97-1.71L17 10.5l-5 3Z" />
<path d="m17 16.5-5-3" />
<path d="m17 16.5 4.74-2.85" />
<path d="M17 16.5v5.17" />
<path d="M7.97 4.42A2 2 0 0 0 7 6.13v4.37l5 3 5-3V6.13a2 2 0 0 0-.97-1.71l-3-1.8a2 2 0 0 0-2.06 0l-3 1.8Z" />
<path d="M12 8 7.26 5.15" />
<path d="m12 8 4.74-2.85" />
<path d="M12 13.5V8" />"###
};
#[cfg(feature = "LuBraces")]
const LU_BRACES: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 3H7a2 2 0 0 0-2 2v5a2 2 0 0 1-2 2 2 2 0 0 1 2 2v5c0 1.1.9 2 2 2h1" />
<path d="M16 21h1a2 2 0 0 0 2-2v-5c0-1.1.9-2 2-2a2 2 0 0 1-2-2V5a2 2 0 0 0-2-2h-1" />"###
};
#[cfg(feature = "LuBrackets")]
const LU_BRACKETS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16 3h3v18h-3" />
<path d="M8 21H5V3h3" />"###
};
#[cfg(feature = "LuBrain")]
const LU_BRAIN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M9.5 2A2.5 2.5 0 0 1 12 4.5v15a2.5 2.5 0 0 1-4.96.44 2.5 2.5 0 0 1-2.96-3.08 3 3 0 0 1-.34-5.58 2.5 2.5 0 0 1 1.32-4.24 2.5 2.5 0 0 1 1.98-3A2.5 2.5 0 0 1 9.5 2Z" />
<path d="M14.5 2A2.5 2.5 0 0 0 12 4.5v15a2.5 2.5 0 0 0 4.96.44 2.5 2.5 0 0 0 2.96-3.08 3 3 0 0 0 .34-5.58 2.5 2.5 0 0 0-1.32-4.24 2.5 2.5 0 0 0-1.98-3A2.5 2.5 0 0 0 14.5 2Z" />"###
};
#[cfg(feature = "LuBrainCircuit")]
const LU_BRAIN_CIRCUIT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 4.5a2.5 2.5 0 0 0-4.96-.46 2.5 2.5 0 0 0-1.98 3 2.5 2.5 0 0 0-1.32 4.24 3 3 0 0 0 .34 5.58 2.5 2.5 0 0 0 2.96 3.08 2.5 2.5 0 0 0 4.91.05L12 20V4.5Z" />
<path d="M16 8V5c0-1.1.9-2 2-2" />
<path d="M12 13h4" />
<path d="M12 18h6a2 2 0 0 1 2 2v1" />
<path d="M12 8h8" />
<path d="M20.5 8a.5.5 0 1 1-1 0 .5.5 0 0 1 1 0Z" />
<path d="M16.5 13a.5.5 0 1 1-1 0 .5.5 0 0 1 1 0Z" />
<path d="M20.5 21a.5.5 0 1 1-1 0 .5.5 0 0 1 1 0Z" />
<path d="M18.5 3a.5.5 0 1 1-1 0 .5.5 0 0 1 1 0Z" />"###
};
#[cfg(feature = "LuBrainCog")]
const LU_BRAIN_COG: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 4.5a2.5 2.5 0 0 0-4.96-.46 2.5 2.5 0 0 0-1.98 3 2.5 2.5 0 0 0-1.32 4.24 3 3 0 0 0 .34 5.58 2.5 2.5 0 0 0 2.96 3.08A2.5 2.5 0 0 0 9.5 22c1.21 0 2.5-.74 2.5-2.5m0-15a2.5 2.5 0 0 1 4.96-.46 2.5 2.5 0 0 1 1.98 3 2.5 2.5 0 0 1 1.32 4.24 3 3 0 0 1-.34 5.58 2.5 2.5 0 0 1-2.96 3.08A2.5 2.5 0 0 1 14.5 22c-1.21 0-2.5-.74-2.5-2.5m0-15V5m0 14.5V19" />
<circle cx="12" cy="12" r="2" />
<path d="M12 9v1" />
<path d="M12 14v1" />
<path d="m14.6 10.5-.87.5" />
<path d="m10.27 13-.87.5" />
<path d="m14.6 13.5-.87-.5" />
<path d="m10.27 11-.87-.5" />"###
};
#[cfg(feature = "LuBriefcase")]
const LU_BRIEFCASE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="20" height="14" x="2" y="7" rx="2" ry="2" />
<path d="M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16" />"###
};
#[cfg(feature = "LuBringToFront")]
const LU_BRING_TO_FRONT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="8" y="8" width="8" height="8" rx="2" />
<path d="M4 10a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2" />
<path d="M14 20a2 2 0 0 0 2 2h4a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2" />"###
};
#[cfg(feature = "LuBrush")]
const LU_BRUSH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m9.06 11.9 8.07-8.06a2.85 2.85 0 1 1 4.03 4.03l-8.06 8.08" />
<path d="M7.07 14.94c-1.66 0-3 1.35-3 3.02 0 1.33-2.5 1.52-2 2.02 1.08 1.1 2.49 2.02 4 2.02 2.2 0 4-1.8 4-4.04a3.01 3.01 0 0 0-3-3.02z" />"###
};
#[cfg(feature = "LuBug")]
const LU_BUG: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="8" height="14" x="8" y="6" rx="4" />
<path d="m19 7-3 2" />
<path d="m5 7 3 2" />
<path d="m19 19-3-2" />
<path d="m5 19 3-2" />
<path d="M20 13h-4" />
<path d="M4 13h4" />
<path d="m10 4 1 2" />
<path d="m14 4-1 2" />"###
};
#[cfg(feature = "LuBuilding")]
const LU_BUILDING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="16" height="20" x="4" y="2" rx="2" ry="2" />
<path d="M9 22v-4h6v4" />
<path d="M8 6h.01" />
<path d="M16 6h.01" />
<path d="M12 6h.01" />
<path d="M12 10h.01" />
<path d="M12 14h.01" />
<path d="M16 10h.01" />
<path d="M16 14h.01" />
<path d="M8 10h.01" />
<path d="M8 14h.01" />"###
};
#[cfg(feature = "LuBuilding2")]
const LU_BUILDING2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 22V4a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v18Z" />
<path d="M6 12H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h2" />
<path d="M18 9h2a2 2 0 0 1 2 2v9a2 2 0 0 1-2 2h-2" />
<path d="M10 6h4" />
<path d="M10 10h4" />
<path d="M10 14h4" />
<path d="M10 18h4" />"###
};
#[cfg(feature = "LuBus")]
const LU_BUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 6v6" />
<path d="M15 6v6" />
<path d="M2 12h19.6" />
<path d="M18 18h3s.5-1.7.8-2.8c.1-.4.2-.8.2-1.2 0-.4-.1-.8-.2-1.2l-1.4-5C20.1 6.8 19.1 6 18 6H4a2 2 0 0 0-2 2v10h3" />
<circle cx="7" cy="18" r="2" />
<path d="M9 18h5" />
<circle cx="16" cy="18" r="2" />"###
};
#[cfg(feature = "LuBusFront")]
const LU_BUS_FRONT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 6 2 7" />
<path d="M10 6h4" />
<path d="m22 7-2-1" />
<rect width="16" height="16" x="4" y="3" rx="2" />
<path d="M4 11h16" />
<path d="M8 15h.01" />
<path d="M16 15h.01" />
<path d="M6 19v2" />
<path d="M18 21v-2" />"###
};
#[cfg(feature = "LuCable")]
const LU_CABLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 9a2 2 0 0 1-2-2V5h6v2a2 2 0 0 1-2 2Z" />
<path d="M3 5V3" />
<path d="M7 5V3" />
<path d="M19 15V6.5a3.5 3.5 0 0 0-7 0v11a3.5 3.5 0 0 1-7 0V9" />
<path d="M17 21v-2" />
<path d="M21 21v-2" />
<path d="M22 19h-6v-2a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2Z" />"###
};
#[cfg(feature = "LuCableCar")]
const LU_CABLE_CAR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10 3h.01" />
<path d="M14 2h.01" />
<path d="m2 9 20-5" />
<path d="M12 12V6.5" />
<rect width="16" height="10" x="4" y="12" rx="3" />
<path d="M9 12v5" />
<path d="M15 12v5" />
<path d="M4 17h16" />"###
};
#[cfg(feature = "LuCake")]
const LU_CAKE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M20 21v-8a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v8" />
<path d="M4 16s.5-1 2-1 2.5 2 4 2 2.5-2 4-2 2.5 2 4 2 2-1 2-1" />
<path d="M2 21h20" />
<path d="M7 8v2" />
<path d="M12 8v2" />
<path d="M17 8v2" />
<path d="M7 4h.01" />
<path d="M12 4h.01" />
<path d="M17 4h.01" />"###
};
#[cfg(feature = "LuCakeSlice")]
const LU_CAKE_SLICE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="9" cy="7" r="2" />
<path d="M7.2 7.9 3 11v9c0 .6.4 1 1 1h16c.6 0 1-.4 1-1v-9c0-2-3-6-7-8l-3.6 2.6" />
<path d="M16 13H3" />
<path d="M16 17H3" />"###
};
#[cfg(feature = "LuCalculator")]
const LU_CALCULATOR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="16" height="20" x="4" y="2" rx="2" />
<line x1="8" x2="16" y1="6" y2="6" />
<line x1="16" x2="16" y1="14" y2="18" />
<path d="M16 10h.01" />
<path d="M12 10h.01" />
<path d="M8 10h.01" />
<path d="M12 14h.01" />
<path d="M8 14h.01" />
<path d="M12 18h.01" />
<path d="M8 18h.01" />"###
};
#[cfg(feature = "LuCalendar")]
const LU_CALENDAR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="4" rx="2" ry="2" />
<line x1="16" x2="16" y1="2" y2="6" />
<line x1="8" x2="8" y1="2" y2="6" />
<line x1="3" x2="21" y1="10" y2="10" />"###
};
#[cfg(feature = "LuCalendarCheck")]
const LU_CALENDAR_CHECK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="4" rx="2" ry="2" />
<line x1="16" x2="16" y1="2" y2="6" />
<line x1="8" x2="8" y1="2" y2="6" />
<line x1="3" x2="21" y1="10" y2="10" />
<path d="m9 16 2 2 4-4" />"###
};
#[cfg(feature = "LuCalendarCheck2")]
const LU_CALENDAR_CHECK2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 14V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h8" />
<line x1="16" x2="16" y1="2" y2="6" />
<line x1="8" x2="8" y1="2" y2="6" />
<line x1="3" x2="21" y1="10" y2="10" />
<path d="m16 20 2 2 4-4" />"###
};
#[cfg(feature = "LuCalendarClock")]
const LU_CALENDAR_CLOCK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 7.5V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h3.5" />
<path d="M16 2v4" />
<path d="M8 2v4" />
<path d="M3 10h5" />
<path d="M17.5 17.5 16 16.25V14" />
<path d="M22 16a6 6 0 1 1-12 0 6 6 0 0 1 12 0Z" />"###
};
#[cfg(feature = "LuCalendarDays")]
const LU_CALENDAR_DAYS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="4" rx="2" ry="2" />
<line x1="16" x2="16" y1="2" y2="6" />
<line x1="8" x2="8" y1="2" y2="6" />
<line x1="3" x2="21" y1="10" y2="10" />
<path d="M8 14h.01" />
<path d="M12 14h.01" />
<path d="M16 14h.01" />
<path d="M8 18h.01" />
<path d="M12 18h.01" />
<path d="M16 18h.01" />"###
};
#[cfg(feature = "LuCalendarHeart")]
const LU_CALENDAR_HEART: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 10V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14c0 1.1.9 2 2 2h7" />
<path d="M16 2v4" />
<path d="M8 2v4" />
<path d="M3 10h18" />
<path d="M21.29 14.7a2.43 2.43 0 0 0-2.65-.52c-.3.12-.57.3-.8.53l-.34.34-.35-.34a2.43 2.43 0 0 0-2.65-.53c-.3.12-.56.3-.79.53-.95.94-1 2.53.2 3.74L17.5 22l3.6-3.55c1.2-1.21 1.14-2.8.19-3.74Z" />"###
};
#[cfg(feature = "LuCalendarMinus")]
const LU_CALENDAR_MINUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 13V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h8" />
<line x1="16" x2="16" y1="2" y2="6" />
<line x1="8" x2="8" y1="2" y2="6" />
<line x1="3" x2="21" y1="10" y2="10" />
<line x1="16" x2="22" y1="19" y2="19" />"###
};
#[cfg(feature = "LuCalendarOff")]
const LU_CALENDAR_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4.18 4.18A2 2 0 0 0 3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 1.82-1.18" />
<path d="M21 15.5V6a2 2 0 0 0-2-2H9.5" />
<path d="M16 2v4" />
<path d="M3 10h7" />
<path d="M21 10h-5.5" />
<line x1="2" x2="22" y1="2" y2="22" />"###
};
#[cfg(feature = "LuCalendarPlus")]
const LU_CALENDAR_PLUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 13V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h8" />
<line x1="16" x2="16" y1="2" y2="6" />
<line x1="8" x2="8" y1="2" y2="6" />
<line x1="3" x2="21" y1="10" y2="10" />
<line x1="19" x2="19" y1="16" y2="22" />
<line x1="16" x2="22" y1="19" y2="19" />"###
};
#[cfg(feature = "LuCalendarRange")]
const LU_CALENDAR_RANGE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="4" rx="2" ry="2" />
<line x1="16" x2="16" y1="2" y2="6" />
<line x1="8" x2="8" y1="2" y2="6" />
<line x1="3" x2="21" y1="10" y2="10" />
<path d="M17 14h-6" />
<path d="M13 18H7" />
<path d="M7 14h.01" />
<path d="M17 18h.01" />"###
};
#[cfg(feature = "LuCalendarSearch")]
const LU_CALENDAR_SEARCH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 12V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14c0 1.1.9 2 2 2h7.5" />
<path d="M16 2v4" />
<path d="M8 2v4" />
<path d="M3 10h18" />
<path d="M18 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6v0Z" />
<path d="m22 22-1.5-1.5" />"###
};
#[cfg(feature = "LuCalendarX")]
const LU_CALENDAR_X: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="4" rx="2" ry="2" />
<line x1="16" x2="16" y1="2" y2="6" />
<line x1="8" x2="8" y1="2" y2="6" />
<line x1="3" x2="21" y1="10" y2="10" />
<line x1="10" x2="14" y1="14" y2="18" />
<line x1="14" x2="10" y1="14" y2="18" />"###
};
#[cfg(feature = "LuCalendarX2")]
const LU_CALENDAR_X2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 13V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h8" />
<line x1="16" x2="16" y1="2" y2="6" />
<line x1="8" x2="8" y1="2" y2="6" />
<line x1="3" x2="21" y1="10" y2="10" />
<line x1="17" x2="22" y1="17" y2="22" />
<line x1="17" x2="22" y1="22" y2="17" />"###
};
#[cfg(feature = "LuCamera")]
const LU_CAMERA: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 4h-5L7 7H4a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-3l-2.5-3z" />
<circle cx="12" cy="13" r="3" />"###
};
#[cfg(feature = "LuCameraOff")]
const LU_CAMERA_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="2" x2="22" y1="2" y2="22" />
<path d="M7 7H4a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h16" />
<path d="M9.5 4h5L17 7h3a2 2 0 0 1 2 2v7.5" />
<path d="M14.121 15.121A3 3 0 1 1 9.88 10.88" />"###
};
#[cfg(feature = "LuCandlestickChart")]
const LU_CANDLESTICK_CHART: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M9 5v4" />
<rect width="4" height="6" x="7" y="9" rx="1" />
<path d="M9 15v2" />
<path d="M17 3v2" />
<rect width="4" height="8" x="15" y="5" rx="1" />
<path d="M17 13v3" />
<path d="M3 3v18h18" />"###
};
#[cfg(feature = "LuCandy")]
const LU_CANDY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m9.5 7.5-2 2a4.95 4.95 0 1 0 7 7l2-2a4.95 4.95 0 1 0-7-7Z" />
<path d="M14 6.5v10" />
<path d="M10 7.5v10" />
<path d="m16 7 1-5 1.37.68A3 3 0 0 0 19.7 3H21v1.3c0 .46.1.92.32 1.33L22 7l-5 1" />
<path d="m8 17-1 5-1.37-.68A3 3 0 0 0 4.3 21H3v-1.3a3 3 0 0 0-.32-1.33L2 17l5-1" />"###
};
#[cfg(feature = "LuCandyCane")]
const LU_CANDY_CANE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5.7 21a2 2 0 0 1-3.5-2l8.6-14a6 6 0 0 1 10.4 6 2 2 0 1 1-3.464-2 2 2 0 1 0-3.464-2Z" />
<path d="M17.75 7 15 2.1" />
<path d="M10.9 4.8 13 9" />
<path d="m7.9 9.7 2 4.4" />
<path d="M4.9 14.7 7 18.9" />"###
};
#[cfg(feature = "LuCandyOff")]
const LU_CANDY_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m8.5 8.5-1 1a4.95 4.95 0 0 0 7 7l1-1" />
<path d="M11.843 6.187A4.947 4.947 0 0 1 16.5 7.5a4.947 4.947 0 0 1 1.313 4.657" />
<path d="M14 16.5V14" />
<path d="M14 6.5v1.843" />
<path d="M10 10v7.5" />
<path d="m16 7 1-5 1.367.683A3 3 0 0 0 19.708 3H21v1.292a3 3 0 0 0 .317 1.341L22 7l-5 1" />
<path d="m8 17-1 5-1.367-.683A3 3 0 0 0 4.292 21H3v-1.292a3 3 0 0 0-.317-1.341L2 17l5-1" />
<line x1="2" x2="22" y1="2" y2="22" />"###
};
#[cfg(feature = "LuCar")]
const LU_CAR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M19 17h2c.6 0 1-.4 1-1v-3c0-.9-.7-1.7-1.5-1.9C18.7 10.6 16 10 16 10s-1.3-1.4-2.2-2.3c-.5-.4-1.1-.7-1.8-.7H5c-.6 0-1.1.4-1.4.9l-1.4 2.9A3.7 3.7 0 0 0 2 12v4c0 .6.4 1 1 1h2" />
<circle cx="7" cy="17" r="2" />
<path d="M9 17h6" />
<circle cx="17" cy="17" r="2" />"###
};
#[cfg(feature = "LuCarFront")]
const LU_CAR_FRONT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m21 8-2 2-1.5-3.7A2 2 0 0 0 15.646 5H8.4a2 2 0 0 0-1.903 1.257L5 10 3 8" />
<path d="M7 14h.01" />
<path d="M17 14h.01" />
<rect width="18" height="8" x="3" y="10" rx="2" />
<path d="M5 18v2" />
<path d="M19 18v2" />"###
};
#[cfg(feature = "LuCarTaxiFront")]
const LU_CAR_TAXI_FRONT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10 2h4" />
<path d="m21 8-2 2-1.5-3.7A2 2 0 0 0 15.646 5H8.4a2 2 0 0 0-1.903 1.257L5 10 3 8" />
<path d="M7 14h.01" />
<path d="M17 14h.01" />
<rect width="18" height="8" x="3" y="10" rx="2" />
<path d="M5 18v2" />
<path d="M19 18v2" />"###
};
#[cfg(feature = "LuCarrot")]
const LU_CARROT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2.27 21.7s9.87-3.5 12.73-6.36a4.5 4.5 0 0 0-6.36-6.37C5.77 11.84 2.27 21.7 2.27 21.7zM8.64 14l-2.05-2.04M15.34 15l-2.46-2.46" />
<path d="M22 9s-1.33-2-3.5-2C16.86 7 15 9 15 9s1.33 2 3.5 2S22 9 22 9z" />
<path d="M15 2s-2 1.33-2 3.5S15 9 15 9s2-1.84 2-3.5C17 3.33 15 2 15 2z" />"###
};
#[cfg(feature = "LuCaseLower")]
const LU_CASE_LOWER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="7" cy="12" r="3" />
<path d="M10 9v6" />
<circle cx="17" cy="12" r="3" />
<path d="M14 7v8" />"###
};
#[cfg(feature = "LuCaseSensitive")]
const LU_CASE_SENSITIVE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3 15 4-8 4 8" />
<path d="M4 13h6" />
<circle cx="18" cy="12" r="3" />
<path d="M21 9v6" />"###
};
#[cfg(feature = "LuCaseUpper")]
const LU_CASE_UPPER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3 15 4-8 4 8" />
<path d="M4 13h6" />
<path d="M15 11h4.5a2 2 0 0 1 0 4H15V7h4a2 2 0 0 1 0 4" />"###
};
#[cfg(feature = "LuCassetteTape")]
const LU_CASSETTE_TAPE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="20" height="16" x="2" y="4" rx="2" />
<circle cx="8" cy="10" r="2" />
<path d="M8 12h8" />
<circle cx="16" cy="10" r="2" />
<path d="m6 20 .7-2.9A1.4 1.4 0 0 1 8.1 16h7.8a1.4 1.4 0 0 1 1.4 1l.7 3" />"###
};
#[cfg(feature = "LuCast")]
const LU_CAST: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 8V6a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2h-6" />
<path d="M2 12a9 9 0 0 1 8 8" />
<path d="M2 16a5 5 0 0 1 4 4" />
<line x1="2" x2="2.01" y1="20" y2="20" />"###
};
#[cfg(feature = "LuCastle")]
const LU_CASTLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 20v-9H2v9a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2Z" />
<path d="M18 11V4H6v7" />
<path d="M15 22v-4a3 3 0 0 0-3-3v0a3 3 0 0 0-3 3v4" />
<path d="M22 11V9" />
<path d="M2 11V9" />
<path d="M6 4V2" />
<path d="M18 4V2" />
<path d="M10 4V2" />
<path d="M14 4V2" />"###
};
#[cfg(feature = "LuCat")]
const LU_CAT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 5c.67 0 1.35.09 2 .26 1.78-2 5.03-2.84 6.42-2.26 1.4.58-.42 7-.42 7 .57 1.07 1 2.24 1 3.44C21 17.9 16.97 21 12 21s-9-3-9-7.56c0-1.25.5-2.4 1-3.44 0 0-1.89-6.42-.5-7 1.39-.58 4.72.23 6.5 2.23A9.04 9.04 0 0 1 12 5Z" />
<path d="M8 14v.5" />
<path d="M16 14v.5" />
<path d="M11.25 16.25h1.5L12 17l-.75-.75Z" />"###
};
#[cfg(feature = "LuCheck")]
const LU_CHECK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuCheckCheck")]
const LU_CHECK_CHECK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18 6 7 17l-5-5" />
<path d="m22 10-7.5 7.5L13 16" />"###
};
#[cfg(feature = "LuCheckCircle")]
const LU_CHECK_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuCheckCircle2")]
const LU_CHECK_CIRCLE2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10z" />
<path d="m9 12 2 2 4-4" />"###
};
#[cfg(feature = "LuCheckSquare")]
const LU_CHECK_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuChefHat")]
const LU_CHEF_HAT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 13.87A4 4 0 0 1 7.41 6a5.11 5.11 0 0 1 1.05-1.54 5 5 0 0 1 7.08 0A5.11 5.11 0 0 1 16.59 6 4 4 0 0 1 18 13.87V21H6Z" />
<line x1="6" x2="18" y1="17" y2="17" />"###
};
#[cfg(feature = "LuCherry")]
const LU_CHERRY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 17a5 5 0 0 0 10 0c0-2.76-2.5-5-5-3-2.5-2-5 .24-5 3Z" />
<path d="M12 17a5 5 0 0 0 10 0c0-2.76-2.5-5-5-3-2.5-2-5 .24-5 3Z" />
<path d="M7 14c3.22-2.91 4.29-8.75 5-12 1.66 2.38 4.94 9 5 12" />
<path d="M22 9c-4.29 0-7.14-2.33-10-7 5.71 0 10 4.67 10 7Z" />"###
};
#[cfg(feature = "LuChevronDown")]
const LU_CHEVRON_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m6 9 6 6 6-6" />"###
};
#[cfg(feature = "LuChevronDownCircle")]
const LU_CHEVRON_DOWN_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="m16 10-4 4-4-4" />"###
};
#[cfg(feature = "LuChevronDownSquare")]
const LU_CHEVRON_DOWN_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="m16 10-4 4-4-4" />"###
};
#[cfg(feature = "LuChevronFirst")]
const LU_CHEVRON_FIRST: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m17 18-6-6 6-6" />
<path d="M7 6v12" />"###
};
#[cfg(feature = "LuChevronLast")]
const LU_CHEVRON_LAST: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m7 18 6-6-6-6" />
<path d="M17 6v12" />"###
};
#[cfg(feature = "LuChevronLeft")]
const LU_CHEVRON_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m15 18-6-6 6-6" />"###
};
#[cfg(feature = "LuChevronLeftCircle")]
const LU_CHEVRON_LEFT_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="m14 16-4-4 4-4" />"###
};
#[cfg(feature = "LuChevronLeftSquare")]
const LU_CHEVRON_LEFT_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="m14 16-4-4 4-4" />"###
};
#[cfg(feature = "LuChevronRight")]
const LU_CHEVRON_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m9 18 6-6-6-6" />"###
};
#[cfg(feature = "LuChevronRightCircle")]
const LU_CHEVRON_RIGHT_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="m10 8 4 4-4 4" />"###
};
#[cfg(feature = "LuChevronRightSquare")]
const LU_CHEVRON_RIGHT_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="m10 8 4 4-4 4" />"###
};
#[cfg(feature = "LuChevronUp")]
const LU_CHEVRON_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m18 15-6-6-6 6" />"###
};
#[cfg(feature = "LuChevronUpCircle")]
const LU_CHEVRON_UP_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="m8 14 4-4 4 4" />"###
};
#[cfg(feature = "LuChevronUpSquare")]
const LU_CHEVRON_UP_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="m8 14 4-4 4 4" />"###
};
#[cfg(feature = "LuChevronsDown")]
const LU_CHEVRONS_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m7 6 5 5 5-5" />
<path d="m7 13 5 5 5-5" />"###
};
#[cfg(feature = "LuChevronsDownUp")]
const LU_CHEVRONS_DOWN_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m7 20 5-5 5 5" />
<path d="m7 4 5 5 5-5" />"###
};
#[cfg(feature = "LuChevronsLeft")]
const LU_CHEVRONS_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m11 17-5-5 5-5" />
<path d="m18 17-5-5 5-5" />"###
};
#[cfg(feature = "LuChevronsLeftRight")]
const LU_CHEVRONS_LEFT_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m9 7-5 5 5 5" />
<path d="m15 7 5 5-5 5" />"###
};
#[cfg(feature = "LuChevronsRight")]
const LU_CHEVRONS_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m6 17 5-5-5-5" />
<path d="m13 17 5-5-5-5" />"###
};
#[cfg(feature = "LuChevronsRightLeft")]
const LU_CHEVRONS_RIGHT_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m20 17-5-5 5-5" />
<path d="m4 17 5-5-5-5" />"###
};
#[cfg(feature = "LuChevronsUp")]
const LU_CHEVRONS_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m17 11-5-5-5 5" />
<path d="m17 18-5-5-5 5" />"###
};
#[cfg(feature = "LuChevronsUpDown")]
const LU_CHEVRONS_UP_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m7 15 5 5 5-5" />
<path d="m7 9 5-5 5 5" />"###
};
#[cfg(feature = "LuChrome")]
const LU_CHROME: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="21.17" x2="12" y1="8" y2="8" />
<line x1="3.95" x2="8.54" y1="6.06" y2="14" />
<line x1="10.88" x2="15.46" y1="21.94" y2="14" />"###
};
#[cfg(feature = "LuChurch")]
const LU_CHURCH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m18 7 4 2v11a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V9l4-2" />
<path d="M14 22v-4a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v4" />
<path d="M18 22V5l-6-3-6 3v17" />
<path d="M12 7v5" />
<path d="M10 9h4" />"###
};
#[cfg(feature = "LuCigarette")]
const LU_CIGARETTE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18 12H2v4h16" />
<path d="M22 12v4" />
<path d="M7 12v4" />
<path d="M18 8c0-2.5-2-2.5-2-5" />
<path d="M22 8c0-2.5-2-2.5-2-5" />"###
};
#[cfg(feature = "LuCigaretteOff")]
const LU_CIGARETTE_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="2" x2="22" y1="2" y2="22" />
<path d="M12 12H2v4h14" />
<path d="M22 12v4" />
<path d="M18 12h-.5" />
<path d="M7 12v4" />
<path d="M18 8c0-2.5-2-2.5-2-5" />
<path d="M22 8c0-2.5-2-2.5-2-5" />"###
};
#[cfg(feature = "LuCircle")]
const LU_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuCircleDashed")]
const LU_CIRCLE_DASHED: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10.1 2.18a9.93 9.93 0 0 1 3.8 0" />
<path d="M17.6 3.71a9.95 9.95 0 0 1 2.69 2.7" />
<path d="M21.82 10.1a9.93 9.93 0 0 1 0 3.8" />
<path d="M20.29 17.6a9.95 9.95 0 0 1-2.7 2.69" />
<path d="M13.9 21.82a9.94 9.94 0 0 1-3.8 0" />
<path d="M6.4 20.29a9.95 9.95 0 0 1-2.69-2.7" />
<path d="M2.18 13.9a9.93 9.93 0 0 1 0-3.8" />
<path d="M3.71 6.4a9.95 9.95 0 0 1 2.7-2.69" />"###
};
#[cfg(feature = "LuCircleDollarSign")]
const LU_CIRCLE_DOLLAR_SIGN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M16 8h-6a2 2 0 1 0 0 4h4a2 2 0 1 1 0 4H8" />
<path d="M12 18V6" />"###
};
#[cfg(feature = "LuCircleDot")]
const LU_CIRCLE_DOT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<circle cx="12" cy="12" r="1" />"###
};
#[cfg(feature = "LuCircleDotDashed")]
const LU_CIRCLE_DOT_DASHED: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10.1 2.18a9.93 9.93 0 0 1 3.8 0" />
<path d="M17.6 3.71a9.95 9.95 0 0 1 2.69 2.7" />
<path d="M21.82 10.1a9.93 9.93 0 0 1 0 3.8" />
<path d="M20.29 17.6a9.95 9.95 0 0 1-2.7 2.69" />
<path d="M13.9 21.82a9.94 9.94 0 0 1-3.8 0" />
<path d="M6.4 20.29a9.95 9.95 0 0 1-2.69-2.7" />
<path d="M2.18 13.9a9.93 9.93 0 0 1 0-3.8" />
<path d="M3.71 6.4a9.95 9.95 0 0 1 2.7-2.69" />
<circle cx="12" cy="12" r="1" />"###
};
#[cfg(feature = "LuCircleEllipsis")]
const LU_CIRCLE_ELLIPSIS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M17 12h.01" />
<path d="M12 12h.01" />
<path d="M7 12h.01" />"###
};
#[cfg(feature = "LuCircleEqual")]
const LU_CIRCLE_EQUAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M7 10h10" />
<path d="M7 14h10" />
<circle cx="12" cy="12" r="10" />"###
};
#[cfg(feature = "LuCircleOff")]
const LU_CIRCLE_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m2 2 20 20" />
<path d="M8.35 2.69A10 10 0 0 1 21.3 15.65" />
<path d="M19.08 19.08A10 10 0 1 1 4.92 4.92" />"###
};
#[cfg(feature = "LuCircleSlash")]
const LU_CIRCLE_SLASH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="9" x2="15" y1="15" y2="9" />
<circle cx="12" cy="12" r="10" />"###
};
#[cfg(feature = "LuCircleSlash2")]
const LU_CIRCLE_SLASH2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M22 2 2 22" />"###
};
#[cfg(feature = "LuCircuitBoard")]
const LU_CIRCUIT_BOARD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="M11 9h4a2 2 0 0 0 2-2V3" />
<circle cx="9" cy="9" r="2" />
<path d="M7 21v-4a2 2 0 0 1 2-2h4" />
<circle cx="15" cy="15" r="2" />"###
};
#[cfg(feature = "LuCitrus")]
const LU_CITRUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21.66 17.67a1.08 1.08 0 0 1-.04 1.6A12 12 0 0 1 4.73 2.38a1.1 1.1 0 0 1 1.61-.04z" />
<path d="M19.65 15.66A8 8 0 0 1 8.35 4.34" />
<path d="m14 10-5.5 5.5" />
<path d="M14 17.85V10H6.15" />"###
};
#[cfg(feature = "LuClapperboard")]
const LU_CLAPPERBOARD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 11v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8H4Z" />
<path d="m4 11-.88-2.87a2 2 0 0 1 1.33-2.5l11.48-3.5a2 2 0 0 1 2.5 1.32l.87 2.87L4 11.01Z" />
<path d="m6.6 4.99 3.38 4.2" />
<path d="m11.86 3.38 3.38 4.2" />"###
};
#[cfg(feature = "LuClipboard")]
const LU_CLIPBOARD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="8" height="4" x="8" y="2" rx="1" ry="1" />
<path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2" />"###
};
#[cfg(feature = "LuClipboardCheck")]
const LU_CLIPBOARD_CHECK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="8" height="4" x="8" y="2" rx="1" ry="1" />
<path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2" />
<path d="m9 14 2 2 4-4" />"###
};
#[cfg(feature = "LuClipboardCopy")]
const LU_CLIPBOARD_COPY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="8" height="4" x="8" y="2" rx="1" ry="1" />
<path d="M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-2" />
<path d="M16 4h2a2 2 0 0 1 2 2v4" />
<path d="M21 14H11" />
<path d="m15 10-4 4 4 4" />"###
};
#[cfg(feature = "LuClipboardEdit")]
const LU_CLIPBOARD_EDIT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="8" height="4" x="8" y="2" rx="1" ry="1" />
<path d="M10.42 12.61a2.1 2.1 0 1 1 2.97 2.97L7.95 21 4 22l.99-3.95 5.43-5.44Z" />
<path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-5.5" />
<path d="M4 13.5V6a2 2 0 0 1 2-2h2" />"###
};
#[cfg(feature = "LuClipboardList")]
const LU_CLIPBOARD_LIST: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="8" height="4" x="8" y="2" rx="1" ry="1" />
<path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2" />
<path d="M12 11h4" />
<path d="M12 16h4" />
<path d="M8 11h.01" />
<path d="M8 16h.01" />"###
};
#[cfg(feature = "LuClipboardPaste")]
const LU_CLIPBOARD_PASTE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M15 2H9a1 1 0 0 0-1 1v2c0 .6.4 1 1 1h6c.6 0 1-.4 1-1V3c0-.6-.4-1-1-1Z" />
<path d="M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2M16 4h2a2 2 0 0 1 2 2v2M11 14h10" />
<path d="m17 10 4 4-4 4" />"###
};
#[cfg(feature = "LuClipboardSignature")]
const LU_CLIPBOARD_SIGNATURE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="8" height="4" x="8" y="2" rx="1" ry="1" />
<path d="M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-.5" />
<path d="M16 4h2a2 2 0 0 1 1.73 1" />
<path d="M18.42 9.61a2.1 2.1 0 1 1 2.97 2.97L16.95 17 13 18l.99-3.95 4.43-4.44Z" />
<path d="M8 18h1" />"###
};
#[cfg(feature = "LuClipboardType")]
const LU_CLIPBOARD_TYPE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="8" height="4" x="8" y="2" rx="1" ry="1" />
<path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2" />
<path d="M9 12v-1h6v1" />
<path d="M11 17h2" />
<path d="M12 11v6" />"###
};
#[cfg(feature = "LuClipboardX")]
const LU_CLIPBOARD_X: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="8" height="4" x="8" y="2" rx="1" ry="1" />
<path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2" />
<path d="m15 11-6 6" />
<path d="m9 11 6 6" />"###
};
#[cfg(feature = "LuClock")]
const LU_CLOCK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuClock1")]
const LU_CLOCK1: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<polyline points="12 6 12 12 14.5 8" />"###
};
#[cfg(feature = "LuClock10")]
const LU_CLOCK10: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<polyline points="12 6 12 12 8 10" />"###
};
#[cfg(feature = "LuClock11")]
const LU_CLOCK11: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<polyline points="12 6 12 12 9.5 8" />"###
};
#[cfg(feature = "LuClock12")]
const LU_CLOCK12: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<polyline points="12 6 12 12" />"###
};
#[cfg(feature = "LuClock2")]
const LU_CLOCK2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<polyline points="12 6 12 12 16 10" />"###
};
#[cfg(feature = "LuClock3")]
const LU_CLOCK3: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<polyline points="12 6 12 12 16.5 12" />"###
};
#[cfg(feature = "LuClock4")]
const LU_CLOCK4: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuClock5")]
const LU_CLOCK5: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<polyline points="12 6 12 12 14.5 16" />"###
};
#[cfg(feature = "LuClock6")]
const LU_CLOCK6: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<polyline points="12 6 12 12 12 16.5" />"###
};
#[cfg(feature = "LuClock7")]
const LU_CLOCK7: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<polyline points="12 6 12 12 9.5 16" />"###
};
#[cfg(feature = "LuClock8")]
const LU_CLOCK8: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<polyline points="12 6 12 12 8 14" />"###
};
#[cfg(feature = "LuClock9")]
const LU_CLOCK9: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<polyline points="12 6 12 12 7.5 12" />"###
};
#[cfg(feature = "LuCloud")]
const LU_CLOUD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17.5 19H9a7 7 0 1 1 6.71-9h1.79a4.5 4.5 0 1 1 0 9Z" />"###
};
#[cfg(feature = "LuCloudCog")]
const LU_CLOUD_COG: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M20 16.2A4.5 4.5 0 0 0 17.5 8h-1.8A7 7 0 1 0 4 14.9" />
<circle cx="12" cy="17" r="3" />
<path d="M12 13v1" />
<path d="M12 20v1" />
<path d="M16 17h-1" />
<path d="M9 17H8" />
<path d="m15 14-.88.88" />
<path d="M9.88 19.12 9 20" />
<path d="m15 20-.88-.88" />
<path d="M9.88 14.88 9 14" />"###
};
#[cfg(feature = "LuCloudDrizzle")]
const LU_CLOUD_DRIZZLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" />
<path d="M8 19v1" />
<path d="M8 14v1" />
<path d="M16 19v1" />
<path d="M16 14v1" />
<path d="M12 21v1" />
<path d="M12 16v1" />"###
};
#[cfg(feature = "LuCloudFog")]
const LU_CLOUD_FOG: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" />
<path d="M16 17H7" />
<path d="M17 21H9" />"###
};
#[cfg(feature = "LuCloudHail")]
const LU_CLOUD_HAIL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" />
<path d="M16 14v2" />
<path d="M8 14v2" />
<path d="M16 20h.01" />
<path d="M8 20h.01" />
<path d="M12 16v2" />
<path d="M12 22h.01" />"###
};
#[cfg(feature = "LuCloudLightning")]
const LU_CLOUD_LIGHTNING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 16.326A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 .5 8.973" />
<path d="m13 12-3 5h4l-3 5" />"###
};
#[cfg(feature = "LuCloudMoon")]
const LU_CLOUD_MOON: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M13 16a3 3 0 1 1 0 6H7a5 5 0 1 1 4.9-6Z" />
<path d="M10.1 9A6 6 0 0 1 16 4a4.24 4.24 0 0 0 6 6 6 6 0 0 1-3 5.197" />"###
};
#[cfg(feature = "LuCloudMoonRain")]
const LU_CLOUD_MOON_RAIN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10.083 9A6.002 6.002 0 0 1 16 4a4.243 4.243 0 0 0 6 6c0 2.22-1.206 4.16-3 5.197" />
<path d="M3 20a5 5 0 1 1 8.9-4H13a3 3 0 0 1 2 5.24" />
<path d="M11 20v2" />
<path d="M7 19v2" />"###
};
#[cfg(feature = "LuCloudOff")]
const LU_CLOUD_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m2 2 20 20" />
<path d="M5.782 5.782A7 7 0 0 0 9 19h8.5a4.5 4.5 0 0 0 1.307-.193" />
<path d="M21.532 16.5A4.5 4.5 0 0 0 17.5 10h-1.79A7.008 7.008 0 0 0 10 5.07" />"###
};
#[cfg(feature = "LuCloudRain")]
const LU_CLOUD_RAIN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" />
<path d="M16 14v6" />
<path d="M8 14v6" />
<path d="M12 16v6" />"###
};
#[cfg(feature = "LuCloudRainWind")]
const LU_CLOUD_RAIN_WIND: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" />
<path d="m9.2 22 3-7" />
<path d="m9 13-3 7" />
<path d="m17 13-3 7" />"###
};
#[cfg(feature = "LuCloudSnow")]
const LU_CLOUD_SNOW: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" />
<path d="M8 15h.01" />
<path d="M8 19h.01" />
<path d="M12 17h.01" />
<path d="M12 21h.01" />
<path d="M16 15h.01" />
<path d="M16 19h.01" />"###
};
#[cfg(feature = "LuCloudSun")]
const LU_CLOUD_SUN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 2v2" />
<path d="m4.93 4.93 1.41 1.41" />
<path d="M20 12h2" />
<path d="m19.07 4.93-1.41 1.41" />
<path d="M15.947 12.65a4 4 0 0 0-5.925-4.128" />
<path d="M13 22H7a5 5 0 1 1 4.9-6H13a3 3 0 0 1 0 6Z" />"###
};
#[cfg(feature = "LuCloudSunRain")]
const LU_CLOUD_SUN_RAIN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 2v2" />
<path d="m4.93 4.93 1.41 1.41" />
<path d="M20 12h2" />
<path d="m19.07 4.93-1.41 1.41" />
<path d="M15.947 12.65a4 4 0 0 0-5.925-4.128" />
<path d="M3 20a5 5 0 1 1 8.9-4H13a3 3 0 0 1 2 5.24" />
<path d="M11 20v2" />
<path d="M7 19v2" />"###
};
#[cfg(feature = "LuCloudy")]
const LU_CLOUDY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17.5 21H9a7 7 0 1 1 6.71-9h1.79a4.5 4.5 0 1 1 0 9Z" />
<path d="M22 10a3 3 0 0 0-3-3h-2.207a5.502 5.502 0 0 0-10.702.5" />"###
};
#[cfg(feature = "LuClover")]
const LU_CLOVER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16.2 3.8a2.7 2.7 0 0 0-3.81 0l-.4.38-.4-.4a2.7 2.7 0 0 0-3.82 0C6.73 4.85 6.67 6.64 8 8l4 4 4-4c1.33-1.36 1.27-3.15.2-4.2z" />
<path d="M8 8c-1.36-1.33-3.15-1.27-4.2-.2a2.7 2.7 0 0 0 0 3.81l.38.4-.4.4a2.7 2.7 0 0 0 0 3.82C4.85 17.27 6.64 17.33 8 16" />
<path d="M16 16c1.36 1.33 3.15 1.27 4.2.2a2.7 2.7 0 0 0 0-3.81l-.38-.4.4-.4a2.7 2.7 0 0 0 0-3.82C19.15 6.73 17.36 6.67 16 8" />
<path d="M7.8 20.2a2.7 2.7 0 0 0 3.81 0l.4-.38.4.4a2.7 2.7 0 0 0 3.82 0c1.06-1.06 1.12-2.85-.21-4.21l-4-4-4 4c-1.33 1.36-1.27 3.15-.2 4.2z" />
<path d="m7 17-5 5" />"###
};
#[cfg(feature = "LuClub")]
const LU_CLUB: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17.28 9.05a5.5 5.5 0 1 0-10.56 0A5.5 5.5 0 1 0 12 17.66a5.5 5.5 0 1 0 5.28-8.6Z" />
<path d="M12 17.66L12 22" />"###
};
#[cfg(feature = "LuCode")]
const LU_CODE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuCode2")]
const LU_CODE2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m18 16 4-4-4-4" />
<path d="m6 8-4 4 4 4" />
<path d="m14.5 4-5 16" />"###
};
#[cfg(feature = "LuCodepen")]
const LU_CODEPEN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="12" x2="12" y1="22" y2="15.5" />
<polyline points="22 8.5 12 15.5 2 8.5" />
<polyline points="2 15.5 12 8.5 22 15.5" />
<line x1="12" x2="12" y1="2" y2="8.5" />"###
};
#[cfg(feature = "LuCodesandbox")]
const LU_CODESANDBOX: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="12" x2="12" y1="22.08" y2="12" />"###
};
#[cfg(feature = "LuCoffee")]
const LU_COFFEE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17 8h1a4 4 0 1 1 0 8h-1" />
<path d="M3 8h14v9a4 4 0 0 1-4 4H7a4 4 0 0 1-4-4Z" />
<line x1="6" x2="6" y1="2" y2="4" />
<line x1="10" x2="10" y1="2" y2="4" />
<line x1="14" x2="14" y1="2" y2="4" />"###
};
#[cfg(feature = "LuCog")]
const LU_COG: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 20a8 8 0 1 0 0-16 8 8 0 0 0 0 16Z" />
<path d="M12 14a2 2 0 1 0 0-4 2 2 0 0 0 0 4Z" />
<path d="M12 2v2" />
<path d="M12 22v-2" />
<path d="m17 20.66-1-1.73" />
<path d="M11 10.27 7 3.34" />
<path d="m20.66 17-1.73-1" />
<path d="m3.34 7 1.73 1" />
<path d="M14 12h8" />
<path d="M2 12h2" />
<path d="m20.66 7-1.73 1" />
<path d="m3.34 17 1.73-1" />
<path d="m17 3.34-1 1.73" />
<path d="m11 13.73-4 6.93" />"###
};
#[cfg(feature = "LuCoins")]
const LU_COINS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="8" cy="8" r="6" />
<path d="M18.09 10.37A6 6 0 1 1 10.34 18" />
<path d="M7 6h1v4" />
<path d="m16.71 13.88.7.71-2.82 2.82" />"###
};
#[cfg(feature = "LuColumns")]
const LU_COLUMNS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<line x1="12" x2="12" y1="3" y2="21" />"###
};
#[cfg(feature = "LuCombine")]
const LU_COMBINE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="8" height="8" x="2" y="2" rx="2" />
<path d="M14 2c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2" />
<path d="M20 2c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2" />
<path d="M10 18H5c-1.7 0-3-1.3-3-3v-1" />
<polyline points="7 21 10 18 7 15" />
<rect width="8" height="8" x="14" y="14" rx="2" />"###
};
#[cfg(feature = "LuCommand")]
const LU_COMMAND: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M15 6v12a3 3 0 1 0 3-3H6a3 3 0 1 0 3 3V6a3 3 0 1 0-3 3h12a3 3 0 1 0-3-3" />"###
};
#[cfg(feature = "LuCompass")]
const LU_COMPASS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuComponent")]
const LU_COMPONENT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5.5 8.5 9 12l-3.5 3.5L2 12l3.5-3.5Z" />
<path d="m12 2 3.5 3.5L12 9 8.5 5.5 12 2Z" />
<path d="M18.5 8.5 22 12l-3.5 3.5L15 12l3.5-3.5Z" />
<path d="m12 15 3.5 3.5L12 22l-3.5-3.5L12 15Z" />"###
};
#[cfg(feature = "LuComputer")]
const LU_COMPUTER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="14" height="8" x="5" y="2" rx="2" />
<rect width="20" height="8" x="2" y="14" rx="2" />
<path d="M6 18h2" />
<path d="M12 18h6" />"###
};
#[cfg(feature = "LuConciergeBell")]
const LU_CONCIERGE_BELL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 18a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v2H2v-2Z" />
<path d="M20 16a8 8 0 1 0-16 0" />
<path d="M12 4v4" />
<path d="M10 4h4" />"###
};
#[cfg(feature = "LuConstruction")]
const LU_CONSTRUCTION: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="2" y="6" width="20" height="8" rx="1" />
<path d="M17 14v7" />
<path d="M7 14v7" />
<path d="M17 3v3" />
<path d="M7 3v3" />
<path d="M10 14 2.3 6.3" />
<path d="m14 6 7.7 7.7" />
<path d="m8 6 8 8" />"###
};
#[cfg(feature = "LuContact")]
const LU_CONTACT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17 18a2 2 0 0 0-2-2H9a2 2 0 0 0-2 2" />
<rect width="18" height="18" x="3" y="4" rx="2" />
<circle cx="12" cy="10" r="2" />
<line x1="8" x2="8" y1="2" y2="4" />
<line x1="16" x2="16" y1="2" y2="4" />"###
};
#[cfg(feature = "LuContact2")]
const LU_CONTACT2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16 18a4 4 0 0 0-8 0" />
<circle cx="12" cy="11" r="3" />
<rect width="18" height="18" x="3" y="4" rx="2" />
<line x1="8" x2="8" y1="2" y2="4" />
<line x1="16" x2="16" y1="2" y2="4" />"###
};
#[cfg(feature = "LuContainer")]
const LU_CONTAINER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 7.7c0-.6-.4-1.2-.8-1.5l-6.3-3.9a1.72 1.72 0 0 0-1.7 0l-10.3 6c-.5.2-.9.8-.9 1.4v6.6c0 .5.4 1.2.8 1.5l6.3 3.9a1.72 1.72 0 0 0 1.7 0l10.3-6c.5-.3.9-1 .9-1.5Z" />
<path d="M10 21.9V14L2.1 9.1" />
<path d="m10 14 11.9-6.9" />
<path d="M14 19.8v-8.1" />
<path d="M18 17.5V9.4" />"###
};
#[cfg(feature = "LuContrast")]
const LU_CONTRAST: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M12 18a6 6 0 0 0 0-12v12z" />"###
};
#[cfg(feature = "LuCookie")]
const LU_COOKIE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 2a10 10 0 1 0 10 10 4 4 0 0 1-5-5 4 4 0 0 1-5-5" />
<path d="M8.5 8.5v.01" />
<path d="M16 15.5v.01" />
<path d="M12 12v.01" />
<path d="M11 17v.01" />
<path d="M7 14v.01" />"###
};
#[cfg(feature = "LuCopy")]
const LU_COPY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="14" height="14" x="8" y="8" rx="2" ry="2" />
<path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" />"###
};
#[cfg(feature = "LuCopyCheck")]
const LU_COPY_CHECK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m12 15 2 2 4-4" />
<rect width="14" height="14" x="8" y="8" rx="2" ry="2" />
<path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" />"###
};
#[cfg(feature = "LuCopyMinus")]
const LU_COPY_MINUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="12" x2="18" y1="15" y2="15" />
<rect width="14" height="14" x="8" y="8" rx="2" ry="2" />
<path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" />"###
};
#[cfg(feature = "LuCopyPlus")]
const LU_COPY_PLUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="15" x2="15" y1="12" y2="18" />
<line x1="12" x2="18" y1="15" y2="15" />
<rect width="14" height="14" x="8" y="8" rx="2" ry="2" />
<path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" />"###
};
#[cfg(feature = "LuCopySlash")]
const LU_COPY_SLASH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="12" x2="18" y1="18" y2="12" />
<rect width="14" height="14" x="8" y="8" rx="2" ry="2" />
<path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" />"###
};
#[cfg(feature = "LuCopyX")]
const LU_COPY_X: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="12" x2="18" y1="12" y2="18" />
<line x1="12" x2="18" y1="18" y2="12" />
<rect width="14" height="14" x="8" y="8" rx="2" ry="2" />
<path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" />"###
};
#[cfg(feature = "LuCopyleft")]
const LU_COPYLEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M9 9.35a4 4 0 1 1 0 5.3" />"###
};
#[cfg(feature = "LuCopyright")]
const LU_COPYRIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M15 9.354a4 4 0 1 0 0 5.292" />"###
};
#[cfg(feature = "LuCornerDownLeft")]
const LU_CORNER_DOWN_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuCornerDownRight")]
const LU_CORNER_DOWN_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuCornerLeftDown")]
const LU_CORNER_LEFT_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuCornerLeftUp")]
const LU_CORNER_LEFT_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuCornerRightDown")]
const LU_CORNER_RIGHT_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuCornerRightUp")]
const LU_CORNER_RIGHT_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuCornerUpLeft")]
const LU_CORNER_UP_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuCornerUpRight")]
const LU_CORNER_UP_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuCpu")]
const LU_CPU: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="4" y="4" width="16" height="16" rx="2" />
<rect x="9" y="9" width="6" height="6" />
<path d="M15 2v2" />
<path d="M15 20v2" />
<path d="M2 15h2" />
<path d="M2 9h2" />
<path d="M20 15h2" />
<path d="M20 9h2" />
<path d="M9 2v2" />
<path d="M9 20v2" />"###
};
#[cfg(feature = "LuCreativeCommons")]
const LU_CREATIVE_COMMONS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M10 9.3a2.8 2.8 0 0 0-3.5 1 3.1 3.1 0 0 0 0 3.4 2.7 2.7 0 0 0 3.5 1" />
<path d="M17 9.3a2.8 2.8 0 0 0-3.5 1 3.1 3.1 0 0 0 0 3.4 2.7 2.7 0 0 0 3.5 1" />"###
};
#[cfg(feature = "LuCreditCard")]
const LU_CREDIT_CARD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="20" height="14" x="2" y="5" rx="2" />
<line x1="2" x2="22" y1="10" y2="10" />"###
};
#[cfg(feature = "LuCroissant")]
const LU_CROISSANT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m4.6 13.11 5.79-3.21c1.89-1.05 4.79 1.78 3.71 3.71l-3.22 5.81C8.8 23.16.79 15.23 4.6 13.11Z" />
<path d="m10.5 9.5-1-2.29C9.2 6.48 8.8 6 8 6H4.5C2.79 6 2 6.5 2 8.5a7.71 7.71 0 0 0 2 4.83" />
<path d="M8 6c0-1.55.24-4-2-4-2 0-2.5 2.17-2.5 4" />
<path d="m14.5 13.5 2.29 1c.73.3 1.21.7 1.21 1.5v3.5c0 1.71-.5 2.5-2.5 2.5a7.71 7.71 0 0 1-4.83-2" />
<path d="M18 16c1.55 0 4-.24 4 2 0 2-2.17 2.5-4 2.5" />"###
};
#[cfg(feature = "LuCrop")]
const LU_CROP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 2v14a2 2 0 0 0 2 2h14" />
<path d="M18 22V8a2 2 0 0 0-2-2H2" />"###
};
#[cfg(feature = "LuCross")]
const LU_CROSS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M11 2a2 2 0 0 0-2 2v5H4a2 2 0 0 0-2 2v2c0 1.1.9 2 2 2h5v5c0 1.1.9 2 2 2h2a2 2 0 0 0 2-2v-5h5a2 2 0 0 0 2-2v-2a2 2 0 0 0-2-2h-5V4a2 2 0 0 0-2-2h-2z" />"###
};
#[cfg(feature = "LuCrosshair")]
const LU_CROSSHAIR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="22" x2="18" y1="12" y2="12" />
<line x1="6" x2="2" y1="12" y2="12" />
<line x1="12" x2="12" y1="6" y2="2" />
<line x1="12" x2="12" y1="22" y2="18" />"###
};
#[cfg(feature = "LuCrown")]
const LU_CROWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m2 4 3 12h14l3-12-6 7-4-7-4 7-6-7zm3 16h14" />"###
};
#[cfg(feature = "LuCupSoda")]
const LU_CUP_SODA: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m6 8 1.75 12.28a2 2 0 0 0 2 1.72h4.54a2 2 0 0 0 2-1.72L18 8" />
<path d="M5 8h14" />
<path d="M7 15a6.47 6.47 0 0 1 5 0 6.47 6.47 0 0 0 5 0" />
<path d="m12 8 1-6h2" />"###
};
#[cfg(feature = "LuCurrency")]
const LU_CURRENCY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="8" />
<line x1="3" x2="6" y1="3" y2="6" />
<line x1="21" x2="18" y1="3" y2="6" />
<line x1="3" x2="6" y1="21" y2="18" />
<line x1="21" x2="18" y1="21" y2="18" />"###
};
#[cfg(feature = "LuDatabase")]
const LU_DATABASE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M3 5V19A9 3 0 0 0 21 19V5" />
<path d="M3 12A9 3 0 0 0 21 12" />"###
};
#[cfg(feature = "LuDatabaseBackup")]
const LU_DATABASE_BACKUP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M3 5v14c0 1.4 3 2.7 7 3" />
<path d="M3 12c0 1.2 2 2.5 5 3" />
<path d="M21 5v4" />
<path d="M13 20a5 5 0 0 0 9-3 4.5 4.5 0 0 0-4.5-4.5c-1.33 0-2.54.54-3.41 1.41L12 16" />
<path d="M12 12v4h4" />"###
};
#[cfg(feature = "LuDelete")]
const LU_DELETE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M20 5H9l-7 7 7 7h11a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2Z" />
<line x1="18" x2="12" y1="9" y2="15" />
<line x1="12" x2="18" y1="9" y2="15" />"###
};
#[cfg(feature = "LuDessert")]
const LU_DESSERT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="4" r="2" />
<path d="M10.2 3.2C5.5 4 2 8.1 2 13a2 2 0 0 0 4 0v-1a2 2 0 0 1 4 0v4a2 2 0 0 0 4 0v-4a2 2 0 0 1 4 0v1a2 2 0 0 0 4 0c0-4.9-3.5-9-8.2-9.8" />
<path d="M3.2 14.8a9 9 0 0 0 17.6 0" />"###
};
#[cfg(feature = "LuDiamond")]
const LU_DIAMOND: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2.7 10.3a2.41 2.41 0 0 0 0 3.41l7.59 7.59a2.41 2.41 0 0 0 3.41 0l7.59-7.59a2.41 2.41 0 0 0 0-3.41l-7.59-7.59a2.41 2.41 0 0 0-3.41 0Z" />"###
};
#[cfg(feature = "LuDice1")]
const LU_DICE1: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<path d="M12 12h.01" />"###
};
#[cfg(feature = "LuDice2")]
const LU_DICE2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<path d="M15 9h.01" />
<path d="M9 15h.01" />"###
};
#[cfg(feature = "LuDice3")]
const LU_DICE3: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<path d="M16 8h.01" />
<path d="M12 12h.01" />
<path d="M8 16h.01" />"###
};
#[cfg(feature = "LuDice4")]
const LU_DICE4: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<path d="M16 8h.01" />
<path d="M8 8h.01" />
<path d="M8 16h.01" />
<path d="M16 16h.01" />"###
};
#[cfg(feature = "LuDice5")]
const LU_DICE5: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<path d="M16 8h.01" />
<path d="M8 8h.01" />
<path d="M8 16h.01" />
<path d="M16 16h.01" />
<path d="M12 12h.01" />"###
};
#[cfg(feature = "LuDice6")]
const LU_DICE6: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<path d="M16 8h.01" />
<path d="M16 12h.01" />
<path d="M16 16h.01" />
<path d="M8 8h.01" />
<path d="M8 12h.01" />
<path d="M8 16h.01" />"###
};
#[cfg(feature = "LuDices")]
const LU_DICES: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="12" height="12" x="2" y="10" rx="2" ry="2" />
<path d="m17.92 14 3.5-3.5a2.24 2.24 0 0 0 0-3l-5-4.92a2.24 2.24 0 0 0-3 0L10 6" />
<path d="M6 18h.01" />
<path d="M10 14h.01" />
<path d="M15 6h.01" />
<path d="M18 9h.01" />"###
};
#[cfg(feature = "LuDiff")]
const LU_DIFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 3v14" />
<path d="M5 10h14" />
<path d="M5 21h14" />"###
};
#[cfg(feature = "LuDisc")]
const LU_DISC: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<circle cx="12" cy="12" r="2" />"###
};
#[cfg(feature = "LuDisc2")]
const LU_DISC2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M12 12h.01" />"###
};
#[cfg(feature = "LuDisc3")]
const LU_DISC3: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M6 12c0-1.7.7-3.2 1.8-4.2" />
<circle cx="12" cy="12" r="2" />
<path d="M18 12c0 1.7-.7 3.2-1.8 4.2" />"###
};
#[cfg(feature = "LuDivide")]
const LU_DIVIDE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="6" r="1" />
<line x1="5" x2="19" y1="12" y2="12" />
<circle cx="12" cy="18" r="1" />"###
};
#[cfg(feature = "LuDivideCircle")]
const LU_DIVIDE_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="8" x2="16" y1="12" y2="12" />
<line x1="12" x2="12" y1="16" y2="16" />
<line x1="12" x2="12" y1="8" y2="8" />
<circle cx="12" cy="12" r="10" />"###
};
#[cfg(feature = "LuDivideSquare")]
const LU_DIVIDE_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<line x1="8" x2="16" y1="12" y2="12" />
<line x1="12" x2="12" y1="16" y2="16" />
<line x1="12" x2="12" y1="8" y2="8" />"###
};
#[cfg(feature = "LuDna")]
const LU_DNA: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 15c6.667-6 13.333 0 20-6" />
<path d="M9 22c1.798-1.998 2.518-3.995 2.807-5.993" />
<path d="M15 2c-1.798 1.998-2.518 3.995-2.807 5.993" />
<path d="m17 6-2.5-2.5" />
<path d="m14 8-1-1" />
<path d="m7 18 2.5 2.5" />
<path d="m3.5 14.5.5.5" />
<path d="m20 9 .5.5" />
<path d="m6.5 12.5 1 1" />
<path d="m16.5 10.5 1 1" />
<path d="m10 16 1.5 1.5" />"###
};
#[cfg(feature = "LuDnaOff")]
const LU_DNA_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M15 2c-1.35 1.5-2.092 3-2.5 4.5M9 22c1.35-1.5 2.092-3 2.5-4.5" />
<path d="M2 15c3.333-3 6.667-3 10-3m10-3c-1.5 1.35-3 2.092-4.5 2.5" />
<path d="m17 6-2.5-2.5" />
<path d="m14 8-1.5-1.5" />
<path d="m7 18 2.5 2.5" />
<path d="m3.5 14.5.5.5" />
<path d="m20 9 .5.5" />
<path d="m6.5 12.5 1 1" />
<path d="m16.5 10.5 1 1" />
<path d="m10 16 1.5 1.5" />
<line x1="2" x2="22" y1="2" y2="22" />"###
};
#[cfg(feature = "LuDog")]
const LU_DOG: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10 5.172C10 3.782 8.423 2.679 6.5 3c-2.823.47-4.113 6.006-4 7 .08.703 1.725 1.722 3.656 1 1.261-.472 1.96-1.45 2.344-2.5" />
<path d="M14.267 5.172c0-1.39 1.577-2.493 3.5-2.172 2.823.47 4.113 6.006 4 7-.08.703-1.725 1.722-3.656 1-1.261-.472-1.855-1.45-2.239-2.5" />
<path d="M8 14v.5" />
<path d="M16 14v.5" />
<path d="M11.25 16.25h1.5L12 17l-.75-.75Z" />
<path d="M4.42 11.247A13.152 13.152 0 0 0 4 14.556C4 18.728 7.582 21 12 21s8-2.272 8-6.444c0-1.061-.162-2.2-.493-3.309m-9.243-6.082A8.801 8.801 0 0 1 12 5c.78 0 1.5.108 2.161.306" />"###
};
#[cfg(feature = "LuDollarSign")]
const LU_DOLLAR_SIGN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="12" x2="12" y1="2" y2="22" />
<path d="M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6" />"###
};
#[cfg(feature = "LuDonut")]
const LU_DONUT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M20.5 10a2.5 2.5 0 0 1-2.4-3H18a2.95 2.95 0 0 1-2.6-4.4 10 10 0 1 0 6.3 7.1c-.3.2-.8.3-1.2.3" />
<circle cx="12" cy="12" r="3" />"###
};
#[cfg(feature = "LuDoorClosed")]
const LU_DOOR_CLOSED: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18 20V6a2 2 0 0 0-2-2H8a2 2 0 0 0-2 2v14" />
<path d="M2 20h20" />
<path d="M14 12v.01" />"###
};
#[cfg(feature = "LuDoorOpen")]
const LU_DOOR_OPEN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M13 4h3a2 2 0 0 1 2 2v14" />
<path d="M2 20h3" />
<path d="M13 20h9" />
<path d="M10 12v.01" />
<path d="M13 4.562v16.157a1 1 0 0 1-1.242.97L5 20V5.562a2 2 0 0 1 1.515-1.94l4-1A2 2 0 0 1 13 4.561Z" />"###
};
#[cfg(feature = "LuDot")]
const LU_DOT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12.1" cy="12.1" r="1" />"###
};
#[cfg(feature = "LuDownload")]
const LU_DOWNLOAD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="12" x2="12" y1="15" y2="3" />"###
};
#[cfg(feature = "LuDownloadCloud")]
const LU_DOWNLOAD_CLOUD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" />
<path d="M12 12v9" />
<path d="m8 17 4 4 4-4" />"###
};
#[cfg(feature = "LuDribbble")]
const LU_DRIBBBLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M19.13 5.09C15.22 9.14 10 10.44 2.25 10.94" />
<path d="M21.75 12.84c-6.62-1.41-12.14 1-16.38 6.32" />
<path d="M8.56 2.75c4.37 6 6 9.42 8 17.72" />"###
};
#[cfg(feature = "LuDroplet")]
const LU_DROPLET: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 22a7 7 0 0 0 7-7c0-2-1-3.9-3-5.5s-3.5-4-4-6.5c-.5 2.5-2 4.9-4 6.5C6 11.1 5 13 5 15a7 7 0 0 0 7 7z" />"###
};
#[cfg(feature = "LuDroplets")]
const LU_DROPLETS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M7 16.3c2.2 0 4-1.83 4-4.05 0-1.16-.57-2.26-1.71-3.19S7.29 6.75 7 5.3c-.29 1.45-1.14 2.84-2.29 3.76S3 11.1 3 12.25c0 2.22 1.8 4.05 4 4.05z" />
<path d="M12.56 6.6A10.97 10.97 0 0 0 14 3.02c.5 2.5 2 4.9 4 6.5s3 3.5 3 5.5a6.98 6.98 0 0 1-11.91 4.97" />"###
};
#[cfg(feature = "LuDrumstick")]
const LU_DRUMSTICK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M15.45 15.4c-2.13.65-4.3.32-5.7-1.1-2.29-2.27-1.76-6.5 1.17-9.42 2.93-2.93 7.15-3.46 9.43-1.18 1.41 1.41 1.74 3.57 1.1 5.71-1.4-.51-3.26-.02-4.64 1.36-1.38 1.38-1.87 3.23-1.36 4.63z" />
<path d="m11.25 15.6-2.16 2.16a2.5 2.5 0 1 1-4.56 1.73 2.49 2.49 0 0 1-1.41-4.24 2.5 2.5 0 0 1 3.14-.32l2.16-2.16" />"###
};
#[cfg(feature = "LuDumbbell")]
const LU_DUMBBELL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m6.5 6.5 11 11" />
<path d="m21 21-1-1" />
<path d="m3 3 1 1" />
<path d="m18 22 4-4" />
<path d="m2 6 4-4" />
<path d="m3 10 7-7" />
<path d="m14 21 7-7" />"###
};
#[cfg(feature = "LuEar")]
const LU_EAR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 8.5a6.5 6.5 0 1 1 13 0c0 6-6 6-6 10a3.5 3.5 0 1 1-7 0" />
<path d="M15 8.5a2.5 2.5 0 0 0-5 0v1a2 2 0 1 1 0 4" />"###
};
#[cfg(feature = "LuEarOff")]
const LU_EAR_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 18.5a3.5 3.5 0 1 0 7 0c0-1.57.92-2.52 2.04-3.46" />
<path d="M6 8.5c0-.75.13-1.47.36-2.14" />
<path d="M8.8 3.15A6.5 6.5 0 0 1 19 8.5c0 1.63-.44 2.81-1.09 3.76" />
<path d="M12.5 6A2.5 2.5 0 0 1 15 8.5M10 13a2 2 0 0 0 1.82-1.18" />
<line x1="2" x2="22" y1="2" y2="22" />"###
};
#[cfg(feature = "LuEgg")]
const LU_EGG: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 22c6.23-.05 7.87-5.57 7.5-10-.36-4.34-3.95-9.96-7.5-10-3.55.04-7.14 5.66-7.5 10-.37 4.43 1.27 9.95 7.5 10z" />"###
};
#[cfg(feature = "LuEggFried")]
const LU_EGG_FRIED: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="11.5" cy="12.5" r="3.5" />
<path d="M3 8c0-3.5 2.5-6 6.5-6 5 0 4.83 3 7.5 5s5 2 5 6c0 4.5-2.5 6.5-7 6.5-2.5 0-2.5 2.5-6 2.5s-7-2-7-5.5c0-3 1.5-3 1.5-5C3.5 10 3 9 3 8Z" />"###
};
#[cfg(feature = "LuEggOff")]
const LU_EGG_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6.399 6.399C5.362 8.157 4.65 10.189 4.5 12c-.37 4.43 1.27 9.95 7.5 10 3.256-.026 5.259-1.547 6.375-3.625" />
<path d="M19.532 13.875A14.07 14.07 0 0 0 19.5 12c-.36-4.34-3.95-9.96-7.5-10-1.04.012-2.082.502-3.046 1.297" />
<line x1="2" x2="22" y1="2" y2="22" />"###
};
#[cfg(feature = "LuEqual")]
const LU_EQUAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="5" x2="19" y1="9" y2="9" />
<line x1="5" x2="19" y1="15" y2="15" />"###
};
#[cfg(feature = "LuEqualNot")]
const LU_EQUAL_NOT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="5" x2="19" y1="9" y2="9" />
<line x1="5" x2="19" y1="15" y2="15" />
<line x1="19" x2="5" y1="5" y2="19" />"###
};
#[cfg(feature = "LuEraser")]
const LU_ERASER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m7 21-4.3-4.3c-1-1-1-2.5 0-3.4l9.6-9.6c1-1 2.5-1 3.4 0l5.6 5.6c1 1 1 2.5 0 3.4L13 21" />
<path d="M22 21H7" />
<path d="m5 11 9 9" />"###
};
#[cfg(feature = "LuEuro")]
const LU_EURO: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 10h12" />
<path d="M4 14h9" />
<path d="M19 6a7.7 7.7 0 0 0-5.2-2A7.9 7.9 0 0 0 6 12c0 4.4 3.5 8 7.8 8 2 0 3.8-.8 5.2-2" />"###
};
#[cfg(feature = "LuExpand")]
const LU_EXPAND: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m21 21-6-6m6 6v-4.8m0 4.8h-4.8" />
<path d="M3 16.2V21m0 0h4.8M3 21l6-6" />
<path d="M21 7.8V3m0 0h-4.8M21 3l-6 6" />
<path d="M3 7.8V3m0 0h4.8M3 3l6 6" />"###
};
#[cfg(feature = "LuExternalLink")]
const LU_EXTERNAL_LINK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="10" x2="21" y1="14" y2="3" />"###
};
#[cfg(feature = "LuEye")]
const LU_EYE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z" />
<circle cx="12" cy="12" r="3" />"###
};
#[cfg(feature = "LuEyeOff")]
const LU_EYE_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M9.88 9.88a3 3 0 1 0 4.24 4.24" />
<path d="M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68" />
<path d="M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61" />
<line x1="2" x2="22" y1="2" y2="22" />"###
};
#[cfg(feature = "LuFacebook")]
const LU_FACEBOOK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuFactory")]
const LU_FACTORY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 20a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V8l-7 5V8l-7 5V4a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z" />
<path d="M17 18h1" />
<path d="M12 18h1" />
<path d="M7 18h1" />"###
};
#[cfg(feature = "LuFan")]
const LU_FAN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10.827 16.379a6.082 6.082 0 0 1-8.618-7.002l5.412 1.45a6.082 6.082 0 0 1 7.002-8.618l-1.45 5.412a6.082 6.082 0 0 1 8.618 7.002l-5.412-1.45a6.082 6.082 0 0 1-7.002 8.618l1.45-5.412Z" />
<path d="M12 12v.01" />"###
};
#[cfg(feature = "LuFastForward")]
const LU_FAST_FORWARD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuFeather")]
const LU_FEATHER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="16" x2="2" y1="8" y2="22" />
<line x1="17.5" x2="9" y1="15" y2="15" />"###
};
#[cfg(feature = "LuFerrisWheel")]
const LU_FERRIS_WHEEL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M12 2v4" />
<path d="m6.8 15-3.5 2" />
<path d="m20.7 7-3.5 2" />
<path d="M6.8 9 3.3 7" />
<path d="m20.7 17-3.5-2" />
<path d="m9 22 3-8 3 8" />
<path d="M8 22h8" />
<path d="M18 18.7a9 9 0 1 0-12 0" />"###
};
#[cfg(feature = "LuFigma")]
const LU_FIGMA: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuFile")]
const LU_FILE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<polyline points="14 2 14 8 20 8" />"###
};
#[cfg(feature = "LuFileArchive")]
const LU_FILE_ARCHIVE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 22V4c0-.5.2-1 .6-1.4C5 2.2 5.5 2 6 2h8.5L20 7.5V20c0 .5-.2 1-.6 1.4-.4.4-.9.6-1.4.6h-2" />
<polyline points="14 2 14 8 20 8" />
<circle cx="10" cy="20" r="2" />
<path d="M10 7V6" />
<path d="M10 12v-1" />
<path d="M10 18v-2" />"###
};
#[cfg(feature = "LuFileAudio")]
const LU_FILE_AUDIO: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17.5 22h.5c.5 0 1-.2 1.4-.6.4-.4.6-.9.6-1.4V7.5L14.5 2H6c-.5 0-1 .2-1.4.6C4.2 3 4 3.5 4 4v3" />
<polyline points="14 2 14 8 20 8" />
<path d="M10 20v-1a2 2 0 1 1 4 0v1a2 2 0 1 1-4 0Z" />
<path d="M6 20v-1a2 2 0 1 0-4 0v1a2 2 0 1 0 4 0Z" />
<path d="M2 19v-3a6 6 0 0 1 12 0v3" />"###
};
#[cfg(feature = "LuFileAudio2")]
const LU_FILE_AUDIO2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v2" />
<polyline points="14 2 14 8 20 8" />
<path d="M2 17v-3a4 4 0 0 1 8 0v3" />
<circle cx="9" cy="17" r="1" />
<circle cx="3" cy="17" r="1" />"###
};
#[cfg(feature = "LuFileAxis3d")]
const LU_FILE_AXIS3D: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<polyline points="14 2 14 8 20 8" />
<path d="M8 10v8h8" />
<path d="m8 18 4-4" />"###
};
#[cfg(feature = "LuFileBadge")]
const LU_FILE_BADGE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 7V4a2 2 0 0 1 2-2h8.5L20 7.5V20a2 2 0 0 1-2 2h-6" />
<polyline points="14 2 14 8 20 8" />
<path d="M5 17a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z" />
<path d="M7 16.5 8 22l-3-1-3 1 1-5.5" />"###
};
#[cfg(feature = "LuFileBadge2")]
const LU_FILE_BADGE2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<path d="M12 13a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z" />
<path d="m14 12.5 1 5.5-3-1-3 1 1-5.5" />"###
};
#[cfg(feature = "LuFileBarChart")]
const LU_FILE_BAR_CHART: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<polyline points="14 2 14 8 20 8" />
<path d="M12 18v-4" />
<path d="M8 18v-2" />
<path d="M16 18v-6" />"###
};
#[cfg(feature = "LuFileBarChart2")]
const LU_FILE_BAR_CHART2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<polyline points="14 2 14 8 20 8" />
<path d="M12 18v-6" />
<path d="M8 18v-1" />
<path d="M16 18v-3" />"###
};
#[cfg(feature = "LuFileBox")]
const LU_FILE_BOX: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 22H18a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4" />
<polyline points="14 2 14 8 20 8" />
<path d="M2.97 13.12c-.6.36-.97 1.02-.97 1.74v3.28c0 .72.37 1.38.97 1.74l3 1.83c.63.39 1.43.39 2.06 0l3-1.83c.6-.36.97-1.02.97-1.74v-3.28c0-.72-.37-1.38-.97-1.74l-3-1.83a1.97 1.97 0 0 0-2.06 0l-3 1.83Z" />
<path d="m7 17-4.74-2.85" />
<path d="m7 17 4.74-2.85" />
<path d="M7 17v5" />"###
};
#[cfg(feature = "LuFileCheck")]
const LU_FILE_CHECK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<polyline points="14 2 14 8 20 8" />
<path d="m9 15 2 2 4-4" />"###
};
#[cfg(feature = "LuFileCheck2")]
const LU_FILE_CHECK2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4" />
<polyline points="14 2 14 8 20 8" />
<path d="m3 15 2 2 4-4" />"###
};
#[cfg(feature = "LuFileClock")]
const LU_FILE_CLOCK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16 22h2c.5 0 1-.2 1.4-.6.4-.4.6-.9.6-1.4V7.5L14.5 2H6c-.5 0-1 .2-1.4.6C4.2 3 4 3.5 4 4v3" />
<polyline points="14 2 14 8 20 8" />
<circle cx="8" cy="16" r="6" />
<path d="M9.5 17.5 8 16.25V14" />"###
};
#[cfg(feature = "LuFileCode")]
const LU_FILE_CODE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<polyline points="14 2 14 8 20 8" />
<path d="m10 13-2 2 2 2" />
<path d="m14 17 2-2-2-2" />"###
};
#[cfg(feature = "LuFileCode2")]
const LU_FILE_CODE2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4" />
<polyline points="14 2 14 8 20 8" />
<path d="m9 18 3-3-3-3" />
<path d="m5 12-3 3 3 3" />"###
};
#[cfg(feature = "LuFileCog")]
const LU_FILE_COG: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 6V4a2 2 0 0 1 2-2h8.5L20 7.5V20a2 2 0 0 1-2 2H4" />
<polyline points="14 2 14 8 20 8" />
<circle cx="6" cy="14" r="3" />
<path d="M6 10v1" />
<path d="M6 17v1" />
<path d="M10 14H9" />
<path d="M3 14H2" />
<path d="m9 11-.88.88" />
<path d="M3.88 16.12 3 17" />
<path d="m9 17-.88-.88" />
<path d="M3.88 11.88 3 11" />"###
};
#[cfg(feature = "LuFileCog2")]
const LU_FILE_COG2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<polyline points="14 2 14 8 20 8" />
<circle cx="12" cy="15" r="2" />
<path d="M12 12v1" />
<path d="M12 17v1" />
<path d="m14.6 13.5-.87.5" />
<path d="m10.27 16-.87.5" />
<path d="m14.6 16.5-.87-.5" />
<path d="m10.27 14-.87-.5" />"###
};
#[cfg(feature = "LuFileDiff")]
const LU_FILE_DIFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<path d="M12 13V7" />
<path d="M9 10h6" />
<path d="M9 17h6" />"###
};
#[cfg(feature = "LuFileDigit")]
const LU_FILE_DIGIT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="4" height="6" x="2" y="12" rx="2" />
<path d="M14 2v6h6" />
<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4" />
<path d="M10 12h2v6" />
<path d="M10 18h4" />"###
};
#[cfg(feature = "LuFileDown")]
const LU_FILE_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<polyline points="14 2 14 8 20 8" />
<path d="M12 18v-6" />
<path d="m9 15 3 3 3-3" />"###
};
#[cfg(feature = "LuFileEdit")]
const LU_FILE_EDIT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 13.5V4a2 2 0 0 1 2-2h8.5L20 7.5V20a2 2 0 0 1-2 2h-5.5" />
<polyline points="14 2 14 8 20 8" />
<path d="M10.42 12.61a2.1 2.1 0 1 1 2.97 2.97L7.95 21 4 22l.99-3.95 5.43-5.44Z" />"###
};
#[cfg(feature = "LuFileHeart")]
const LU_FILE_HEART: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 6V4a2 2 0 0 1 2-2h8.5L20 7.5V20a2 2 0 0 1-2 2H4" />
<polyline points="14 2 14 8 20 8" />
<path d="M10.29 10.7a2.43 2.43 0 0 0-2.66-.52c-.29.12-.56.3-.78.53l-.35.34-.35-.34a2.43 2.43 0 0 0-2.65-.53c-.3.12-.56.3-.79.53-.95.94-1 2.53.2 3.74L6.5 18l3.6-3.55c1.2-1.21 1.14-2.8.19-3.74Z" />"###
};
#[cfg(feature = "LuFileImage")]
const LU_FILE_IMAGE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<polyline points="14 2 14 8 20 8" />
<circle cx="10" cy="13" r="2" />
<path d="m20 17-1.09-1.09a2 2 0 0 0-2.82 0L10 22" />"###
};
#[cfg(feature = "LuFileInput")]
const LU_FILE_INPUT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4" />
<polyline points="14 2 14 8 20 8" />
<path d="M2 15h10" />
<path d="m9 18 3-3-3-3" />"###
};
#[cfg(feature = "LuFileJson")]
const LU_FILE_JSON: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<polyline points="14 2 14 8 20 8" />
<path d="M10 12a1 1 0 0 0-1 1v1a1 1 0 0 1-1 1 1 1 0 0 1 1 1v1a1 1 0 0 0 1 1" />
<path d="M14 18a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1 1 1 0 0 1-1-1v-1a1 1 0 0 0-1-1" />"###
};
#[cfg(feature = "LuFileJson2")]
const LU_FILE_JSON2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4" />
<polyline points="14 2 14 8 20 8" />
<path d="M4 12a1 1 0 0 0-1 1v1a1 1 0 0 1-1 1 1 1 0 0 1 1 1v1a1 1 0 0 0 1 1" />
<path d="M8 18a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1 1 1 0 0 1-1-1v-1a1 1 0 0 0-1-1" />"###
};
#[cfg(feature = "LuFileKey")]
const LU_FILE_KEY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<circle cx="10" cy="16" r="2" />
<path d="m16 10-4.5 4.5" />
<path d="m15 11 1 1" />"###
};
#[cfg(feature = "LuFileKey2")]
const LU_FILE_KEY2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 10V4a2 2 0 0 1 2-2h8.5L20 7.5V20a2 2 0 0 1-2 2H4" />
<polyline points="14 2 14 8 20 8" />
<circle cx="4" cy="16" r="2" />
<path d="m10 10-4.5 4.5" />
<path d="m9 11 1 1" />"###
};
#[cfg(feature = "LuFileLineChart")]
const LU_FILE_LINE_CHART: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<polyline points="14 2 14 8 20 8" />
<path d="m16 13-3.5 3.5-2-2L8 17" />"###
};
#[cfg(feature = "LuFileLock")]
const LU_FILE_LOCK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<rect width="8" height="6" x="8" y="12" rx="1" />
<path d="M15 12v-2a3 3 0 1 0-6 0v2" />"###
};
#[cfg(feature = "LuFileLock2")]
const LU_FILE_LOCK2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 5V4a2 2 0 0 1 2-2h8.5L20 7.5V20a2 2 0 0 1-2 2H4" />
<polyline points="14 2 14 8 20 8" />
<rect width="8" height="5" x="2" y="13" rx="1" />
<path d="M8 13v-2a2 2 0 1 0-4 0v2" />"###
};
#[cfg(feature = "LuFileMinus")]
const LU_FILE_MINUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<polyline points="14 2 14 8 20 8" />
<line x1="9" x2="15" y1="15" y2="15" />"###
};
#[cfg(feature = "LuFileMinus2")]
const LU_FILE_MINUS2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4" />
<polyline points="14 2 14 8 20 8" />
<path d="M3 15h6" />"###
};
#[cfg(feature = "LuFileOutput")]
const LU_FILE_OUTPUT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4" />
<polyline points="14 2 14 8 20 8" />
<path d="M2 15h10" />
<path d="m5 12-3 3 3 3" />"###
};
#[cfg(feature = "LuFilePieChart")]
const LU_FILE_PIE_CHART: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16 22h2a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v3" />
<polyline points="14 2 14 8 20 8" />
<path d="M4.04 11.71a5.84 5.84 0 1 0 8.2 8.29" />
<path d="M13.83 16A5.83 5.83 0 0 0 8 10.17V16h5.83Z" />"###
};
#[cfg(feature = "LuFilePlus")]
const LU_FILE_PLUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<polyline points="14 2 14 8 20 8" />
<line x1="12" x2="12" y1="18" y2="12" />
<line x1="9" x2="15" y1="15" y2="15" />"###
};
#[cfg(feature = "LuFilePlus2")]
const LU_FILE_PLUS2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4" />
<polyline points="14 2 14 8 20 8" />
<path d="M3 15h6" />
<path d="M6 12v6" />"###
};
#[cfg(feature = "LuFileQuestion")]
const LU_FILE_QUESTION: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<path d="M10 10.3c.2-.4.5-.8.9-1a2.1 2.1 0 0 1 2.6.4c.3.4.5.8.5 1.3 0 1.3-2 2-2 2" />
<path d="M12 17h.01" />"###
};
#[cfg(feature = "LuFileScan")]
const LU_FILE_SCAN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M20 10V7.5L14.5 2H6a2 2 0 0 0-2 2v16c0 1.1.9 2 2 2h4.5" />
<polyline points="14 2 14 8 20 8" />
<path d="M16 22a2 2 0 0 1-2-2" />
<path d="M20 22a2 2 0 0 0 2-2" />
<path d="M20 14a2 2 0 0 1 2 2" />
<path d="M16 14a2 2 0 0 0-2 2" />"###
};
#[cfg(feature = "LuFileSearch")]
const LU_FILE_SEARCH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v3" />
<polyline points="14 2 14 8 20 8" />
<path d="M5 17a3 3 0 1 0 0-6 3 3 0 0 0 0 6z" />
<path d="m9 18-1.5-1.5" />"###
};
#[cfg(feature = "LuFileSearch2")]
const LU_FILE_SEARCH2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<polyline points="14 2 14 8 20 8" />
<circle cx="11.5" cy="14.5" r="2.5" />
<path d="M13.25 16.25 15 18" />"###
};
#[cfg(feature = "LuFileSignature")]
const LU_FILE_SIGNATURE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M20 19.5v.5a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8.5L18 5.5" />
<path d="M8 18h1" />
<path d="M18.42 9.61a2.1 2.1 0 1 1 2.97 2.97L16.95 17 13 18l.99-3.95 4.43-4.44Z" />"###
};
#[cfg(feature = "LuFileSpreadsheet")]
const LU_FILE_SPREADSHEET: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<polyline points="14 2 14 8 20 8" />
<path d="M8 13h2" />
<path d="M8 17h2" />
<path d="M14 13h2" />
<path d="M14 17h2" />"###
};
#[cfg(feature = "LuFileStack")]
const LU_FILE_STACK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16 2v5h5" />
<path d="M21 6v6.5c0 .8-.7 1.5-1.5 1.5h-7c-.8 0-1.5-.7-1.5-1.5v-9c0-.8.7-1.5 1.5-1.5H17l4 4z" />
<path d="M7 8v8.8c0 .3.2.6.4.8.2.2.5.4.8.4H15" />
<path d="M3 12v8.8c0 .3.2.6.4.8.2.2.5.4.8.4H11" />"###
};
#[cfg(feature = "LuFileSymlink")]
const LU_FILE_SYMLINK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v7" />
<polyline points="14 2 14 8 20 8" />
<path d="m10 18 3-3-3-3" />
<path d="M4 18v-1a2 2 0 0 1 2-2h6" />"###
};
#[cfg(feature = "LuFileTerminal")]
const LU_FILE_TERMINAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<polyline points="14 2 14 8 20 8" />
<path d="m8 16 2-2-2-2" />
<path d="M12 18h4" />"###
};
#[cfg(feature = "LuFileText")]
const LU_FILE_TEXT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<polyline points="14 2 14 8 20 8" />
<line x1="16" x2="8" y1="13" y2="13" />
<line x1="16" x2="8" y1="17" y2="17" />
<line x1="10" x2="8" y1="9" y2="9" />"###
};
#[cfg(feature = "LuFileType")]
const LU_FILE_TYPE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<polyline points="14 2 14 8 20 8" />
<path d="M9 13v-1h6v1" />
<path d="M11 18h2" />
<path d="M12 12v6" />"###
};
#[cfg(feature = "LuFileType2")]
const LU_FILE_TYPE2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4" />
<polyline points="14 2 14 8 20 8" />
<path d="M2 13v-1h6v1" />
<path d="M4 18h2" />
<path d="M5 12v6" />"###
};
#[cfg(feature = "LuFileUp")]
const LU_FILE_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<polyline points="14 2 14 8 20 8" />
<path d="M12 12v6" />
<path d="m15 15-3-3-3 3" />"###
};
#[cfg(feature = "LuFileVideo")]
const LU_FILE_VIDEO: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<polyline points="14 2 14 8 20 8" />
<path d="m10 11 5 3-5 3v-6Z" />"###
};
#[cfg(feature = "LuFileVideo2")]
const LU_FILE_VIDEO2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 8V4a2 2 0 0 1 2-2h8.5L20 7.5V20a2 2 0 0 1-2 2H4" />
<polyline points="14 2 14 8 20 8" />
<path d="m10 15.5 4 2.5v-6l-4 2.5" />
<rect width="8" height="6" x="2" y="12" rx="1" />"###
};
#[cfg(feature = "LuFileVolume")]
const LU_FILE_VOLUME: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v3" />
<polyline points="14 2 14 8 20 8" />
<path d="m7 10-3 2H2v4h2l3 2v-8Z" />
<path d="M11 11c.64.8 1 1.87 1 3s-.36 2.2-1 3" />"###
};
#[cfg(feature = "LuFileVolume2")]
const LU_FILE_VOLUME2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<polyline points="14 2 14 8 20 8" />
<path d="M11.5 13.5c.32.4.5.94.5 1.5s-.18 1.1-.5 1.5" />
<path d="M15 12c.64.8 1 1.87 1 3s-.36 2.2-1 3" />
<path d="M8 15h.01" />"###
};
#[cfg(feature = "LuFileWarning")]
const LU_FILE_WARNING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<path d="M12 9v4" />
<path d="M12 17h.01" />"###
};
#[cfg(feature = "LuFileX")]
const LU_FILE_X: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
<polyline points="14 2 14 8 20 8" />
<line x1="9.5" x2="14.5" y1="12.5" y2="17.5" />
<line x1="14.5" x2="9.5" y1="12.5" y2="17.5" />"###
};
#[cfg(feature = "LuFileX2")]
const LU_FILE_X2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4" />
<path d="M14 2v6h6" />
<path d="m3 12.5 5 5" />
<path d="m8 12.5-5 5" />"###
};
#[cfg(feature = "LuFiles")]
const LU_FILES: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M15.5 2H8.6c-.4 0-.8.2-1.1.5-.3.3-.5.7-.5 1.1v12.8c0 .4.2.8.5 1.1.3.3.7.5 1.1.5h9.8c.4 0 .8-.2 1.1-.5.3-.3.5-.7.5-1.1V6.5L15.5 2z" />
<path d="M3 7.6v12.8c0 .4.2.8.5 1.1.3.3.7.5 1.1.5h9.8" />
<path d="M15 2v5h5" />"###
};
#[cfg(feature = "LuFilm")]
const LU_FILM: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="20" height="20" x="2" y="2" rx="2.18" ry="2.18" />
<line x1="7" x2="7" y1="2" y2="22" />
<line x1="17" x2="17" y1="2" y2="22" />
<line x1="2" x2="22" y1="12" y2="12" />
<line x1="2" x2="7" y1="7" y2="7" />
<line x1="2" x2="7" y1="17" y2="17" />
<line x1="17" x2="22" y1="17" y2="17" />
<line x1="17" x2="22" y1="7" y2="7" />"###
};
#[cfg(feature = "LuFilter")]
const LU_FILTER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuFilterX")]
const LU_FILTER_X: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M13.013 3H2l8 9.46V19l4 2v-8.54l.9-1.055" />
<path d="m22 3-5 5" />
<path d="m17 3 5 5" />"###
};
#[cfg(feature = "LuFingerprint")]
const LU_FINGERPRINT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 12C2 6.5 6.5 2 12 2a10 10 0 0 1 8 4" />
<path d="M5 19.5C5.5 18 6 15 6 12c0-.7.12-1.37.34-2" />
<path d="M17.29 21.02c.12-.6.43-2.3.5-3.02" />
<path d="M12 10a2 2 0 0 0-2 2c0 1.02-.1 2.51-.26 4" />
<path d="M8.65 22c.21-.66.45-1.32.57-2" />
<path d="M14 13.12c0 2.38 0 6.38-1 8.88" />
<path d="M2 16h.01" />
<path d="M21.8 16c.2-2 .131-5.354 0-6" />
<path d="M9 6.8a6 6 0 0 1 9 5.2c0 .47 0 1.17-.02 2" />"###
};
#[cfg(feature = "LuFish")]
const LU_FISH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6.5 12c.94-3.46 4.94-6 8.5-6 3.56 0 6.06 2.54 7 6-.94 3.47-3.44 6-7 6s-7.56-2.53-8.5-6Z" />
<path d="M18 12v.5" />
<path d="M16 17.93a9.77 9.77 0 0 1 0-11.86" />
<path d="M7 10.67C7 8 5.58 5.97 2.73 5.5c-1 1.5-1 5 .23 6.5-1.24 1.5-1.24 5-.23 6.5C5.58 18.03 7 16 7 13.33" />
<path d="M10.46 7.26C10.2 5.88 9.17 4.24 8 3h5.8a2 2 0 0 1 1.98 1.67l.23 1.4" />
<path d="m16.01 17.93-.23 1.4A2 2 0 0 1 13.8 21H9.5a5.96 5.96 0 0 0 1.49-3.98" />"###
};
#[cfg(feature = "LuFishOff")]
const LU_FISH_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18 12.47v.03m0-.5v.47m-.475 5.056A6.744 6.744 0 0 1 15 18c-3.56 0-7.56-2.53-8.5-6 .348-1.28 1.114-2.433 2.121-3.38m3.444-2.088A8.802 8.802 0 0 1 15 6c3.56 0 6.06 2.54 7 6-.309 1.14-.786 2.177-1.413 3.058" />
<path d="M7 10.67C7 8 5.58 5.97 2.73 5.5c-1 1.5-1 5 .23 6.5-1.24 1.5-1.24 5-.23 6.5C5.58 18.03 7 16 7 13.33m7.48-4.372A9.77 9.77 0 0 1 16 6.07m0 11.86a9.77 9.77 0 0 1-1.728-3.618" />
<path d="m16.01 17.93-.23 1.4A2 2 0 0 1 13.8 21H9.5a5.96 5.96 0 0 0 1.49-3.98M8.53 3h5.27a2 2 0 0 1 1.98 1.67l.23 1.4M2 2l20 20" />"###
};
#[cfg(feature = "LuFishSymbol")]
const LU_FISH_SYMBOL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 16s9-15 20-4C11 23 2 8 2 8" />"###
};
#[cfg(feature = "LuFlag")]
const LU_FLAG: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="4" x2="4" y1="22" y2="15" />"###
};
#[cfg(feature = "LuFlagOff")]
const LU_FLAG_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 2c3 0 5 2 8 2s4-1 4-1v11" />
<path d="M4 22V4" />
<path d="M4 15s1-1 4-1 5 2 8 2" />
<line x1="2" x2="22" y1="2" y2="22" />"###
};
#[cfg(feature = "LuFlagTriangleLeft")]
const LU_FLAG_TRIANGLE_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17 22V2L7 7l10 5" />"###
};
#[cfg(feature = "LuFlagTriangleRight")]
const LU_FLAG_TRIANGLE_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M7 22V2l10 5-10 5" />"###
};
#[cfg(feature = "LuFlame")]
const LU_FLAME: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8.5 14.5A2.5 2.5 0 0 0 11 12c0-1.38-.5-2-1-3-1.072-2.143-.224-4.054 2-6 .5 2.5 2 4.9 4 6.5 2 1.6 3 3.5 3 5.5a7 7 0 1 1-14 0c0-1.153.433-2.294 1-3a2.5 2.5 0 0 0 2.5 2.5z" />"###
};
#[cfg(feature = "LuFlashlight")]
const LU_FLASHLIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18 6c0 2-2 2-2 4v10a2 2 0 0 1-2 2h-4a2 2 0 0 1-2-2V10c0-2-2-2-2-4V2h12z" />
<line x1="6" x2="18" y1="6" y2="6" />
<line x1="12" x2="12" y1="12" y2="12" />"###
};
#[cfg(feature = "LuFlashlightOff")]
const LU_FLASHLIGHT_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16 16v4a2 2 0 0 1-2 2h-4a2 2 0 0 1-2-2V10c0-2-2-2-2-4" />
<path d="M7 2h11v4c0 2-2 2-2 4v1" />
<line x1="11" x2="18" y1="6" y2="6" />
<line x1="2" x2="22" y1="2" y2="22" />"###
};
#[cfg(feature = "LuFlaskConical")]
const LU_FLASK_CONICAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10 2v7.527a2 2 0 0 1-.211.896L4.72 20.55a1 1 0 0 0 .9 1.45h12.76a1 1 0 0 0 .9-1.45l-5.069-10.127A2 2 0 0 1 14 9.527V2" />
<path d="M8.5 2h7" />
<path d="M7 16h10" />"###
};
#[cfg(feature = "LuFlaskConicalOff")]
const LU_FLASK_CONICAL_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10 10 4.72 20.55a1 1 0 0 0 .9 1.45h12.76a1 1 0 0 0 .9-1.45l-1.272-2.542" />
<path d="M10 2v2.343" />
<path d="M14 2v6.343" />
<path d="M8.5 2h7" />
<path d="M7 16h9" />
<line x1="2" x2="22" y1="2" y2="22" />"###
};
#[cfg(feature = "LuFlaskRound")]
const LU_FLASK_ROUND: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10 2v7.31" />
<path d="M14 9.3V1.99" />
<path d="M8.5 2h7" />
<path d="M14 9.3a6.5 6.5 0 1 1-4 0" />
<path d="M5.52 16h12.96" />"###
};
#[cfg(feature = "LuFlipHorizontal")]
const LU_FLIP_HORIZONTAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 3H5a2 2 0 0 0-2 2v14c0 1.1.9 2 2 2h3" />
<path d="M16 3h3a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-3" />
<path d="M12 20v2" />
<path d="M12 14v2" />
<path d="M12 8v2" />
<path d="M12 2v2" />"###
};
#[cfg(feature = "LuFlipHorizontal2")]
const LU_FLIP_HORIZONTAL2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3 7 5 5-5 5V7" />
<path d="m21 7-5 5 5 5V7" />
<path d="M12 20v2" />
<path d="M12 14v2" />
<path d="M12 8v2" />
<path d="M12 2v2" />"###
};
#[cfg(feature = "LuFlipVertical")]
const LU_FLIP_VERTICAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 8V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v3" />
<path d="M21 16v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-3" />
<path d="M4 12H2" />
<path d="M10 12H8" />
<path d="M16 12h-2" />
<path d="M22 12h-2" />"###
};
#[cfg(feature = "LuFlipVertical2")]
const LU_FLIP_VERTICAL2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m17 3-5 5-5-5h10" />
<path d="m17 21-5-5-5 5h10" />
<path d="M4 12H2" />
<path d="M10 12H8" />
<path d="M16 12h-2" />
<path d="M22 12h-2" />"###
};
#[cfg(feature = "LuFlower")]
const LU_FLOWER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 7.5a4.5 4.5 0 1 1 4.5 4.5M12 7.5A4.5 4.5 0 1 0 7.5 12M12 7.5V9m-4.5 3a4.5 4.5 0 1 0 4.5 4.5M7.5 12H9m7.5 0a4.5 4.5 0 1 1-4.5 4.5m4.5-4.5H15m-3 4.5V15" />
<circle cx="12" cy="12" r="3" />
<path d="m8 16 1.5-1.5" />
<path d="M14.5 9.5 16 8" />
<path d="m8 8 1.5 1.5" />
<path d="M14.5 14.5 16 16" />"###
};
#[cfg(feature = "LuFlower2")]
const LU_FLOWER2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 5a3 3 0 1 1 3 3m-3-3a3 3 0 1 0-3 3m3-3v1M9 8a3 3 0 1 0 3 3M9 8h1m5 0a3 3 0 1 1-3 3m3-3h-1m-2 3v-1" />
<circle cx="12" cy="8" r="2" />
<path d="M12 10v12" />
<path d="M12 22c4.2 0 7-1.667 7-5-4.2 0-7 1.667-7 5Z" />
<path d="M12 22c-4.2 0-7-1.667-7-5 4.2 0 7 1.667 7 5Z" />"###
};
#[cfg(feature = "LuFocus")]
const LU_FOCUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M3 7V5a2 2 0 0 1 2-2h2" />
<path d="M17 3h2a2 2 0 0 1 2 2v2" />
<path d="M21 17v2a2 2 0 0 1-2 2h-2" />
<path d="M7 21H5a2 2 0 0 1-2-2v-2" />"###
};
#[cfg(feature = "LuFoldHorizontal")]
const LU_FOLD_HORIZONTAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 12h6" />
<path d="M22 12h-6" />
<path d="M12 2v2" />
<path d="M12 8v2" />
<path d="M12 14v2" />
<path d="M12 20v2" />
<path d="m19 9-3 3 3 3" />
<path d="m5 15 3-3-3-3" />"###
};
#[cfg(feature = "LuFoldVertical")]
const LU_FOLD_VERTICAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 22v-6" />
<path d="M12 8V2" />
<path d="M4 12H2" />
<path d="M10 12H8" />
<path d="M16 12h-2" />
<path d="M22 12h-2" />
<path d="m15 19-3-3-3 3" />
<path d="m15 5-3 3-3-3" />"###
};
#[cfg(feature = "LuFolder")]
const LU_FOLDER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z" />"###
};
#[cfg(feature = "LuFolderArchive")]
const LU_FOLDER_ARCHIVE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 20V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2h6" />
<circle cx="16" cy="19" r="2" />
<path d="M16 11v-1" />
<path d="M16 17v-2" />"###
};
#[cfg(feature = "LuFolderCheck")]
const LU_FOLDER_CHECK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z" />
<path d="m9 13 2 2 4-4" />"###
};
#[cfg(feature = "LuFolderClock")]
const LU_FOLDER_CLOCK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M7 20H4a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2" />
<circle cx="16" cy="16" r="6" />
<path d="M16 14v2l1 1" />"###
};
#[cfg(feature = "LuFolderClosed")]
const LU_FOLDER_CLOSED: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z" />
<path d="M2 10h20" />"###
};
#[cfg(feature = "LuFolderCog")]
const LU_FOLDER_COG: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10.5 20H4a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2v3" />
<circle cx="18" cy="18" r="3" />
<path d="M18 14v1" />
<path d="M18 21v1" />
<path d="M22 18h-1" />
<path d="M15 18h-1" />
<path d="m21 15-.88.88" />
<path d="M15.88 20.12 15 21" />
<path d="m21 21-.88-.88" />
<path d="M15.88 15.88 15 15" />"###
};
#[cfg(feature = "LuFolderCog2")]
const LU_FOLDER_COG2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z" />
<circle cx="12" cy="13" r="2" />
<path d="M12 10v1" />
<path d="M12 15v1" />
<path d="m14.6 11.5-.87.5" />
<path d="m10.27 14-.87.5" />
<path d="m14.6 14.5-.87-.5" />
<path d="m10.27 12-.87-.5" />"###
};
#[cfg(feature = "LuFolderDot")]
const LU_FOLDER_DOT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z" />
<circle cx="12" cy="13" r="1" />"###
};
#[cfg(feature = "LuFolderDown")]
const LU_FOLDER_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z" />
<path d="M12 10v6" />
<path d="m15 13-3 3-3-3" />"###
};
#[cfg(feature = "LuFolderEdit")]
const LU_FOLDER_EDIT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8.42 10.61a2.1 2.1 0 1 1 2.97 2.97L5.95 19 2 20l.99-3.95 5.43-5.44Z" />
<path d="M2 11.5V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-9.5" />"###
};
#[cfg(feature = "LuFolderGit")]
const LU_FOLDER_GIT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z" />
<circle cx="12" cy="13" r="2" />
<path d="M14 13h3" />
<path d="M7 13h3" />"###
};
#[cfg(feature = "LuFolderGit2")]
const LU_FOLDER_GIT2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M9 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v5" />
<circle cx="13" cy="12" r="2" />
<path d="M18 19c-2.8 0-5-2.2-5-5v8" />
<circle cx="20" cy="19" r="2" />"###
};
#[cfg(feature = "LuFolderHeart")]
const LU_FOLDER_HEART: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M11 20H4a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2v1.5" />
<path d="M21.29 13.7a2.43 2.43 0 0 0-2.65-.52c-.3.12-.57.3-.8.53l-.34.34-.35-.34a2.43 2.43 0 0 0-2.65-.53c-.3.12-.56.3-.79.53-.95.94-1 2.53.2 3.74L17.5 21l3.6-3.55c1.2-1.21 1.14-2.8.19-3.74Z" />"###
};
#[cfg(feature = "LuFolderInput")]
const LU_FOLDER_INPUT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 9V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-1" />
<path d="M2 13h10" />
<path d="m9 16 3-3-3-3" />"###
};
#[cfg(feature = "LuFolderKanban")]
const LU_FOLDER_KANBAN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z" />
<path d="M8 10v4" />
<path d="M12 10v2" />
<path d="M16 10v6" />"###
};
#[cfg(feature = "LuFolderKey")]
const LU_FOLDER_KEY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10 20H4a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2v2" />
<circle cx="16" cy="20" r="2" />
<path d="m22 14-4.5 4.5" />
<path d="m21 15 1 1" />"###
};
#[cfg(feature = "LuFolderLock")]
const LU_FOLDER_LOCK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10 20H4a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2v2.5" />
<rect width="8" height="5" x="14" y="17" rx="1" />
<path d="M20 17v-2a2 2 0 1 0-4 0v2" />"###
};
#[cfg(feature = "LuFolderMinus")]
const LU_FOLDER_MINUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z" />
<line x1="9" x2="15" y1="13" y2="13" />"###
};
#[cfg(feature = "LuFolderOpen")]
const LU_FOLDER_OPEN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m6 14 1.45-2.9A2 2 0 0 1 9.24 10H20a2 2 0 0 1 1.94 2.5l-1.55 6a2 2 0 0 1-1.94 1.5H4a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H18a2 2 0 0 1 2 2v2" />"###
};
#[cfg(feature = "LuFolderOpenDot")]
const LU_FOLDER_OPEN_DOT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m6 14 1.45-2.9A2 2 0 0 1 9.24 10H20a2 2 0 0 1 1.94 2.5l-1.55 6a2 2 0 0 1-1.94 1.5H4a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H18a2 2 0 0 1 2 2v2" />
<circle cx="14" cy="15" r="1" />"###
};
#[cfg(feature = "LuFolderOutput")]
const LU_FOLDER_OUTPUT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 7.5V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H2" />
<path d="M2 13h10" />
<path d="m5 10-3 3 3 3" />"###
};
#[cfg(feature = "LuFolderPlus")]
const LU_FOLDER_PLUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z" />
<line x1="12" x2="12" y1="10" y2="16" />
<line x1="9" x2="15" y1="13" y2="13" />"###
};
#[cfg(feature = "LuFolderRoot")]
const LU_FOLDER_ROOT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z" />
<circle cx="12" cy="13" r="2" />
<path d="M12 15v5" />"###
};
#[cfg(feature = "LuFolderSearch")]
const LU_FOLDER_SEARCH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M11 20H4a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2v4" />
<circle cx="17" cy="17" r="3" />
<path d="m21 21-1.5-1.5" />"###
};
#[cfg(feature = "LuFolderSearch2")]
const LU_FOLDER_SEARCH2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z" />
<circle cx="11.5" cy="12.5" r="2.5" />
<path d="M13.27 14.27 15 16" />"###
};
#[cfg(feature = "LuFolderSymlink")]
const LU_FOLDER_SYMLINK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 9V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H20a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H2" />
<path d="m8 16 3-3-3-3" />
<path d="M2 16v-1a2 2 0 0 1 2-2h6" />"###
};
#[cfg(feature = "LuFolderSync")]
const LU_FOLDER_SYNC: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M9 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v1" />
<path d="M12 10v4h4" />
<path d="m12 14 1.5-1.5c.9-.9 2.2-1.5 3.5-1.5s2.6.6 3.5 1.5c.4.4.8 1 1 1.5" />
<path d="M22 22v-4h-4" />
<path d="m22 18-1.5 1.5c-.9.9-2.1 1.5-3.5 1.5s-2.6-.6-3.5-1.5c-.4-.4-.8-1-1-1.5" />"###
};
#[cfg(feature = "LuFolderTree")]
const LU_FOLDER_TREE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M13 10h7a1 1 0 0 0 1-1V6a1 1 0 0 0-1-1h-2.5a1 1 0 0 1-.8-.4l-.9-1.2A1 1 0 0 0 15 3h-2a1 1 0 0 0-1 1v5a1 1 0 0 0 1 1Z" />
<path d="M13 21h7a1 1 0 0 0 1-1v-3a1 1 0 0 0-1-1h-2.88a1 1 0 0 1-.9-.55l-.44-.9a1 1 0 0 0-.9-.55H13a1 1 0 0 0-1 1v5a1 1 0 0 0 1 1Z" />
<path d="M3 3v2c0 1.1.9 2 2 2h3" />
<path d="M3 3v13c0 1.1.9 2 2 2h3" />"###
};
#[cfg(feature = "LuFolderUp")]
const LU_FOLDER_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z" />
<path d="M12 10v6" />
<path d="m9 13 3-3 3 3" />"###
};
#[cfg(feature = "LuFolderX")]
const LU_FOLDER_X: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z" />
<path d="m9.5 10.5 5 5" />
<path d="m14.5 10.5-5 5" />"###
};
#[cfg(feature = "LuFolders")]
const LU_FOLDERS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 17h12a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-3.93a2 2 0 0 1-1.66-.9l-.82-1.2a2 2 0 0 0-1.66-.9H8a2 2 0 0 0-2 2v9c0 1.1.9 2 2 2Z" />
<path d="M2 8v11c0 1.1.9 2 2 2h14" />"###
};
#[cfg(feature = "LuFootprints")]
const LU_FOOTPRINTS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 16v-2.38C4 11.5 2.97 10.5 3 8c.03-2.72 1.49-6 4.5-6C9.37 2 10 3.8 10 5.5c0 3.11-2 5.66-2 8.68V16a2 2 0 1 1-4 0Z" />
<path d="M20 20v-2.38c0-2.12 1.03-3.12 1-5.62-.03-2.72-1.49-6-4.5-6C14.63 6 14 7.8 14 9.5c0 3.11 2 5.66 2 8.68V20a2 2 0 1 0 4 0Z" />
<path d="M16 17h4" />
<path d="M4 13h4" />"###
};
#[cfg(feature = "LuForklift")]
const LU_FORKLIFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 12H5a2 2 0 0 0-2 2v5" />
<circle cx="13" cy="19" r="2" />
<circle cx="5" cy="19" r="2" />
<path d="M8 19h3m5-17v17h6M6 12V7c0-1.1.9-2 2-2h3l5 5" />"###
};
#[cfg(feature = "LuFormInput")]
const LU_FORM_INPUT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="20" height="12" x="2" y="6" rx="2" />
<path d="M12 12h.01" />
<path d="M17 12h.01" />
<path d="M7 12h.01" />"###
};
#[cfg(feature = "LuForward")]
const LU_FORWARD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="15 17 20 12 15 7" />
<path d="M4 18v-2a4 4 0 0 1 4-4h12" />"###
};
#[cfg(feature = "LuFrame")]
const LU_FRAME: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="22" x2="2" y1="6" y2="6" />
<line x1="22" x2="2" y1="18" y2="18" />
<line x1="6" x2="6" y1="2" y2="22" />
<line x1="18" x2="18" y1="2" y2="22" />"###
};
#[cfg(feature = "LuFramer")]
const LU_FRAMER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 16V9h14V2H5l14 14h-7m-7 0 7 7v-7m-7 0h7" />"###
};
#[cfg(feature = "LuFrown")]
const LU_FROWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="9" x2="9.01" y1="9" y2="9" />
<line x1="15" x2="15.01" y1="9" y2="9" />"###
};
#[cfg(feature = "LuFuel")]
const LU_FUEL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="3" x2="15" y1="22" y2="22" />
<line x1="4" x2="14" y1="9" y2="9" />
<path d="M14 22V4a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v18" />
<path d="M14 13h2a2 2 0 0 1 2 2v2a2 2 0 0 0 2 2h0a2 2 0 0 0 2-2V9.83a2 2 0 0 0-.59-1.42L18 5" />"###
};
#[cfg(feature = "LuFunctionSquare")]
const LU_FUNCTION_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<path d="M9 17c2 0 2.8-1 2.8-2.8V10c0-2 1-3.3 3.2-3" />
<path d="M9 11.2h5.7" />"###
};
#[cfg(feature = "LuGalleryHorizontal")]
const LU_GALLERY_HORIZONTAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 3v18" />
<rect width="12" height="18" x="6" y="3" rx="2" />
<path d="M22 3v18" />"###
};
#[cfg(feature = "LuGalleryHorizontalEnd")]
const LU_GALLERY_HORIZONTAL_END: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 7v10" />
<path d="M6 5v14" />
<rect width="12" height="18" x="10" y="3" rx="2" />"###
};
#[cfg(feature = "LuGalleryThumbnails")]
const LU_GALLERY_THUMBNAILS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="14" x="3" y="3" rx="2" />
<path d="M4 21h1" />
<path d="M9 21h1" />
<path d="M14 21h1" />
<path d="M19 21h1" />"###
};
#[cfg(feature = "LuGalleryVertical")]
const LU_GALLERY_VERTICAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 2h18" />
<rect width="18" height="12" x="3" y="6" rx="2" />
<path d="M3 22h18" />"###
};
#[cfg(feature = "LuGalleryVerticalEnd")]
const LU_GALLERY_VERTICAL_END: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M7 2h10" />
<path d="M5 6h14" />
<rect width="18" height="12" x="3" y="10" rx="2" />"###
};
#[cfg(feature = "LuGamepad")]
const LU_GAMEPAD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="6" x2="10" y1="12" y2="12" />
<line x1="8" x2="8" y1="10" y2="14" />
<line x1="15" x2="15.01" y1="13" y2="13" />
<line x1="18" x2="18.01" y1="11" y2="11" />
<rect width="20" height="12" x="2" y="6" rx="2" />"###
};
#[cfg(feature = "LuGamepad2")]
const LU_GAMEPAD2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="6" x2="10" y1="11" y2="11" />
<line x1="8" x2="8" y1="9" y2="13" />
<line x1="15" x2="15.01" y1="12" y2="12" />
<line x1="18" x2="18.01" y1="10" y2="10" />
<path d="M17.32 5H6.68a4 4 0 0 0-3.978 3.59c-.006.052-.01.101-.017.152C2.604 9.416 2 14.456 2 16a3 3 0 0 0 3 3c1 0 1.5-.5 2-1l1.414-1.414A2 2 0 0 1 9.828 16h4.344a2 2 0 0 1 1.414.586L17 18c.5.5 1 1 2 1a3 3 0 0 0 3-3c0-1.545-.604-6.584-.685-7.258-.007-.05-.011-.1-.017-.151A4 4 0 0 0 17.32 5z" />"###
};
#[cfg(feature = "LuGanttChart")]
const LU_GANTT_CHART: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 6h10" />
<path d="M6 12h9" />
<path d="M11 18h7" />"###
};
#[cfg(feature = "LuGanttChartSquare")]
const LU_GANTT_CHART_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="M9 8h7" />
<path d="M8 12h6" />
<path d="M11 16h5" />"###
};
#[cfg(feature = "LuGauge")]
const LU_GAUGE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m12 14 4-4" />
<path d="M3.34 19a10 10 0 1 1 17.32 0" />"###
};
#[cfg(feature = "LuGaugeCircle")]
const LU_GAUGE_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M15.6 2.7a10 10 0 1 0 5.7 5.7" />
<circle cx="12" cy="12" r="2" />
<path d="M13.4 10.6 19 5" />"###
};
#[cfg(feature = "LuGavel")]
const LU_GAVEL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m14 13-7.5 7.5c-.83.83-2.17.83-3 0 0 0 0 0 0 0a2.12 2.12 0 0 1 0-3L11 10" />
<path d="m16 16 6-6" />
<path d="m8 8 6-6" />
<path d="m9 7 8 8" />
<path d="m21 11-8-8" />"###
};
#[cfg(feature = "LuGem")]
const LU_GEM: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 3h12l4 6-10 13L2 9Z" />
<path d="M11 3 8 9l4 13 4-13-3-6" />
<path d="M2 9h20" />"###
};
#[cfg(feature = "LuGhost")]
const LU_GHOST: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M9 10h.01" />
<path d="M15 10h.01" />
<path d="M12 2a8 8 0 0 0-8 8v12l3-3 2.5 2.5L12 19l2.5 2.5L17 19l3 3V10a8 8 0 0 0-8-8z" />"###
};
#[cfg(feature = "LuGift")]
const LU_GIFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<rect width="20" height="5" x="2" y="7" />
<line x1="12" x2="12" y1="22" y2="7" />
<path d="M12 7H7.5a2.5 2.5 0 0 1 0-5C11 2 12 7 12 7z" />
<path d="M12 7h4.5a2.5 2.5 0 0 0 0-5C13 2 12 7 12 7z" />"###
};
#[cfg(feature = "LuGitBranch")]
const LU_GIT_BRANCH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="6" x2="6" y1="3" y2="15" />
<circle cx="18" cy="6" r="3" />
<circle cx="6" cy="18" r="3" />
<path d="M18 9a9 9 0 0 1-9 9" />"###
};
#[cfg(feature = "LuGitBranchPlus")]
const LU_GIT_BRANCH_PLUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 3v12" />
<path d="M18 9a3 3 0 1 0 0-6 3 3 0 0 0 0 6z" />
<path d="M6 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6z" />
<path d="M15 6a9 9 0 0 0-9 9" />
<path d="M18 15v6" />
<path d="M21 18h-6" />"###
};
#[cfg(feature = "LuGitCommit")]
const LU_GIT_COMMIT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="3" x2="9" y1="12" y2="12" />
<line x1="15" x2="21" y1="12" y2="12" />"###
};
#[cfg(feature = "LuGitCompare")]
const LU_GIT_COMPARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M11 18H8a2 2 0 0 1-2-2V9" />"###
};
#[cfg(feature = "LuGitFork")]
const LU_GIT_FORK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="18" r="3" />
<circle cx="6" cy="6" r="3" />
<circle cx="18" cy="6" r="3" />
<path d="M18 9v1a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2V9" />
<path d="M12 12v3" />"###
};
#[cfg(feature = "LuGitMerge")]
const LU_GIT_MERGE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuGitPullRequest")]
const LU_GIT_PULL_REQUEST: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="6" x2="6" y1="9" y2="21" />"###
};
#[cfg(feature = "LuGitPullRequestClosed")]
const LU_GIT_PULL_REQUEST_CLOSED: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M18 11.5V15" />
<path d="m21 3-6 6" />
<path d="m21 9-6-6" />
<line x1="6" x2="6" y1="9" y2="21" />"###
};
#[cfg(feature = "LuGitPullRequestDraft")]
const LU_GIT_PULL_REQUEST_DRAFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M18 6V5" />
<path d="M18 11v-1" />
<line x1="6" x2="6" y1="9" y2="21" />"###
};
#[cfg(feature = "LuGithub")]
const LU_GITHUB: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4" />
<path d="M9 18c-4.51 2-5-2-7-2" />"###
};
#[cfg(feature = "LuGitlab")]
const LU_GITLAB: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m22 13.29-3.33-10a.42.42 0 0 0-.14-.18.38.38 0 0 0-.22-.11.39.39 0 0 0-.23.07.42.42 0 0 0-.14.18l-2.26 6.67H8.32L6.1 3.26a.42.42 0 0 0-.1-.18.38.38 0 0 0-.26-.08.39.39 0 0 0-.23.07.42.42 0 0 0-.14.18L2 13.29a.74.74 0 0 0 .27.83L12 21l9.69-6.88a.71.71 0 0 0 .31-.83Z" />"###
};
#[cfg(feature = "LuGlassWater")]
const LU_GLASS_WATER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M15.2 22H8.8a2 2 0 0 1-2-1.79L5 3h14l-1.81 17.21A2 2 0 0 1 15.2 22Z" />
<path d="M6 12a5 5 0 0 1 6 0 5 5 0 0 0 6 0" />"###
};
#[cfg(feature = "LuGlasses")]
const LU_GLASSES: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="6" cy="15" r="4" />
<circle cx="18" cy="15" r="4" />
<path d="M14 15a2 2 0 0 0-2-2 2 2 0 0 0-2 2" />
<path d="M2.5 13 5 7c.7-1.3 1.4-2 3-2" />
<path d="M21.5 13 19 7c-.7-1.3-1.5-2-3-2" />"###
};
#[cfg(feature = "LuGlobe")]
const LU_GLOBE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="2" x2="22" y1="12" y2="12" />
<path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z" />"###
};
#[cfg(feature = "LuGlobe2")]
const LU_GLOBE2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21.54 15H17a2 2 0 0 0-2 2v4.54" />
<path d="M7 3.34V5a3 3 0 0 0 3 3v0a2 2 0 0 1 2 2v0c0 1.1.9 2 2 2v0a2 2 0 0 0 2-2v0c0-1.1.9-2 2-2h3.17" />
<path d="M11 21.95V18a2 2 0 0 0-2-2v0a2 2 0 0 1-2-2v-1a2 2 0 0 0-2-2H2.05" />
<circle cx="12" cy="12" r="10" />"###
};
#[cfg(feature = "LuGoal")]
const LU_GOAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 13V2l8 4-8 4" />
<path d="M20.55 10.23A9 9 0 1 1 8 4.94" />
<path d="M8 10a5 5 0 1 0 8.9 2.02" />"###
};
#[cfg(feature = "LuGrab")]
const LU_GRAB: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18 11.5V9a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v1.4" />
<path d="M14 10V8a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v2" />
<path d="M10 9.9V9a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v5" />
<path d="M6 14v0a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v0" />
<path d="M18 11v0a2 2 0 1 1 4 0v3a8 8 0 0 1-8 8h-4a8 8 0 0 1-8-8 2 2 0 1 1 4 0" />"###
};
#[cfg(feature = "LuGraduationCap")]
const LU_GRADUATION_CAP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 10v6M2 10l10-5 10 5-10 5z" />
<path d="M6 12v5c3 3 9 3 12 0v-5" />"###
};
#[cfg(feature = "LuGrape")]
const LU_GRAPE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 5V2l-5.89 5.89" />
<circle cx="16.6" cy="15.89" r="3" />
<circle cx="8.11" cy="7.4" r="3" />
<circle cx="12.35" cy="11.65" r="3" />
<circle cx="13.91" cy="5.85" r="3" />
<circle cx="18.15" cy="10.09" r="3" />
<circle cx="6.56" cy="13.2" r="3" />
<circle cx="10.8" cy="17.44" r="3" />
<circle cx="5" cy="19" r="3" />"###
};
#[cfg(feature = "LuGrid2x2")]
const LU_GRID2X2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="M3 12h18" />
<path d="M12 3v18" />"###
};
#[cfg(feature = "LuGrid3x3")]
const LU_GRID3X3: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="M3 9h18" />
<path d="M3 15h18" />
<path d="M9 3v18" />
<path d="M15 3v18" />"###
};
#[cfg(feature = "LuGrip")]
const LU_GRIP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="5" r="1" />
<circle cx="19" cy="5" r="1" />
<circle cx="5" cy="5" r="1" />
<circle cx="12" cy="12" r="1" />
<circle cx="19" cy="12" r="1" />
<circle cx="5" cy="12" r="1" />
<circle cx="12" cy="19" r="1" />
<circle cx="19" cy="19" r="1" />
<circle cx="5" cy="19" r="1" />"###
};
#[cfg(feature = "LuGripHorizontal")]
const LU_GRIP_HORIZONTAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="9" r="1" />
<circle cx="19" cy="9" r="1" />
<circle cx="5" cy="9" r="1" />
<circle cx="12" cy="15" r="1" />
<circle cx="19" cy="15" r="1" />
<circle cx="5" cy="15" r="1" />"###
};
#[cfg(feature = "LuGripVertical")]
const LU_GRIP_VERTICAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="9" cy="12" r="1" />
<circle cx="9" cy="5" r="1" />
<circle cx="9" cy="19" r="1" />
<circle cx="15" cy="12" r="1" />
<circle cx="15" cy="5" r="1" />
<circle cx="15" cy="19" r="1" />"###
};
#[cfg(feature = "LuGroup")]
const LU_GROUP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 7V5c0-1.1.9-2 2-2h2" />
<path d="M17 3h2c1.1 0 2 .9 2 2v2" />
<path d="M21 17v2c0 1.1-.9 2-2 2h-2" />
<path d="M7 21H5c-1.1 0-2-.9-2-2v-2" />
<rect width="7" height="5" x="7" y="7" rx="1" />
<rect width="7" height="5" x="10" y="12" rx="1" />"###
};
#[cfg(feature = "LuHammer")]
const LU_HAMMER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m15 12-8.5 8.5c-.83.83-2.17.83-3 0 0 0 0 0 0 0a2.12 2.12 0 0 1 0-3L12 9" />
<path d="M17.64 15 22 10.64" />
<path d="m20.91 11.7-1.25-1.25c-.6-.6-.93-1.4-.93-2.25v-.86L16.01 4.6a5.56 5.56 0 0 0-3.94-1.64H9l.92.82A6.18 6.18 0 0 1 12 8.4v1.56l2 2h2.47l2.26 1.91" />"###
};
#[cfg(feature = "LuHand")]
const LU_HAND: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18 11V6a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v0" />
<path d="M14 10V4a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v2" />
<path d="M10 10.5V6a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v8" />
<path d="M18 8a2 2 0 1 1 4 0v6a8 8 0 0 1-8 8h-2c-2.8 0-4.5-.86-5.99-2.34l-3.6-3.6a2 2 0 0 1 2.83-2.82L7 15" />"###
};
#[cfg(feature = "LuHandMetal")]
const LU_HAND_METAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18 12.5V10a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v1.4" />
<path d="M14 11V9a2 2 0 1 0-4 0v2" />
<path d="M10 10.5V5a2 2 0 1 0-4 0v9" />
<path d="m7 15-1.76-1.76a2 2 0 0 0-2.83 2.82l3.6 3.6C7.5 21.14 9.2 22 12 22h2a8 8 0 0 0 8-8V7a2 2 0 1 0-4 0v5" />"###
};
#[cfg(feature = "LuHardDrive")]
const LU_HARD_DRIVE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="22" x2="2" y1="12" y2="12" />
<path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z" />
<line x1="6" x2="6.01" y1="16" y2="16" />
<line x1="10" x2="10.01" y1="16" y2="16" />"###
};
#[cfg(feature = "LuHardDriveDownload")]
const LU_HARD_DRIVE_DOWNLOAD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 2v8" />
<path d="m16 6-4 4-4-4" />
<rect width="20" height="8" x="2" y="14" rx="2" />
<path d="M6 18h.01" />
<path d="M10 18h.01" />"###
};
#[cfg(feature = "LuHardDriveUpload")]
const LU_HARD_DRIVE_UPLOAD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m16 6-4-4-4 4" />
<path d="M12 2v8" />
<rect width="20" height="8" x="2" y="14" rx="2" />
<path d="M6 18h.01" />
<path d="M10 18h.01" />"###
};
#[cfg(feature = "LuHardHat")]
const LU_HARD_HAT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 18a1 1 0 0 0 1 1h18a1 1 0 0 0 1-1v-2a1 1 0 0 0-1-1H3a1 1 0 0 0-1 1v2z" />
<path d="M10 10V5a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1v5" />
<path d="M4 15v-3a6 6 0 0 1 6-6h0" />
<path d="M14 6h0a6 6 0 0 1 6 6v3" />"###
};
#[cfg(feature = "LuHash")]
const LU_HASH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="4" x2="20" y1="9" y2="9" />
<line x1="4" x2="20" y1="15" y2="15" />
<line x1="10" x2="8" y1="3" y2="21" />
<line x1="16" x2="14" y1="3" y2="21" />"###
};
#[cfg(feature = "LuHaze")]
const LU_HAZE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m5.2 6.2 1.4 1.4" />
<path d="M2 13h2" />
<path d="M20 13h2" />
<path d="m17.4 7.6 1.4-1.4" />
<path d="M22 17H2" />
<path d="M22 21H2" />
<path d="M16 13a4 4 0 0 0-8 0" />
<path d="M12 5V2.5" />"###
};
#[cfg(feature = "LuHdmiPort")]
const LU_HDMI_PORT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 9a1 1 0 0 0-1-1H3a1 1 0 0 0-1 1v4a1 1 0 0 0 1 1h1l2 2h12l2-2h1a1 1 0 0 0 1-1Z" />
<path d="M7.5 12h9" />"###
};
#[cfg(feature = "LuHeading")]
const LU_HEADING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 12h12" />
<path d="M6 20V4" />
<path d="M18 20V4" />"###
};
#[cfg(feature = "LuHeading1")]
const LU_HEADING1: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 12h8" />
<path d="M4 18V6" />
<path d="M12 18V6" />
<path d="m17 12 3-2v8" />"###
};
#[cfg(feature = "LuHeading2")]
const LU_HEADING2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 12h8" />
<path d="M4 18V6" />
<path d="M12 18V6" />
<path d="M21 18h-4c0-4 4-3 4-6 0-1.5-2-2.5-4-1" />"###
};
#[cfg(feature = "LuHeading3")]
const LU_HEADING3: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 12h8" />
<path d="M4 18V6" />
<path d="M12 18V6" />
<path d="M17.5 10.5c1.7-1 3.5 0 3.5 1.5a2 2 0 0 1-2 2" />
<path d="M17 17.5c2 1.5 4 .3 4-1.5a2 2 0 0 0-2-2" />"###
};
#[cfg(feature = "LuHeading4")]
const LU_HEADING4: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 12h8" />
<path d="M4 18V6" />
<path d="M12 18V6" />
<path d="M17 10v4h4" />
<path d="M21 10v8" />"###
};
#[cfg(feature = "LuHeading5")]
const LU_HEADING5: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 12h8" />
<path d="M4 18V6" />
<path d="M12 18V6" />
<path d="M17 13v-3h4" />
<path d="M17 17.7c.4.2.8.3 1.3.3 1.5 0 2.7-1.1 2.7-2.5S19.8 13 18.3 13H17" />"###
};
#[cfg(feature = "LuHeading6")]
const LU_HEADING6: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 12h8" />
<path d="M4 18V6" />
<path d="M12 18V6" />
<circle cx="19" cy="16" r="2" />
<path d="M20 10c-2 2-3 3.5-3 6" />"###
};
#[cfg(feature = "LuHeadphones")]
const LU_HEADPHONES: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 14h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-7a9 9 0 0 1 18 0v7a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3" />"###
};
#[cfg(feature = "LuHeart")]
const LU_HEART: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z" />"###
};
#[cfg(feature = "LuHeartCrack")]
const LU_HEART_CRACK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z" />
<path d="m12 13-1-1 2-2-3-3 2-2" />"###
};
#[cfg(feature = "LuHeartHandshake")]
const LU_HEART_HANDSHAKE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z" />
<path d="M12 5 9.04 7.96a2.17 2.17 0 0 0 0 3.08v0c.82.82 2.13.85 3 .07l2.07-1.9a2.82 2.82 0 0 1 3.79 0l2.96 2.66" />
<path d="m18 15-2-2" />
<path d="m15 18-2-2" />"###
};
#[cfg(feature = "LuHeartOff")]
const LU_HEART_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="2" y1="2" x2="22" y2="22" />
<path d="M16.5 16.5 12 21l-7-7c-1.5-1.45-3-3.2-3-5.5a5.5 5.5 0 0 1 2.14-4.35" />
<path d="M8.76 3.1c1.15.22 2.13.78 3.24 1.9 1.5-1.5 2.74-2 4.5-2A5.5 5.5 0 0 1 22 8.5c0 2.12-1.3 3.78-2.67 5.17" />"###
};
#[cfg(feature = "LuHeartPulse")]
const LU_HEART_PULSE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z" />
<path d="M3.22 12H9.5l.5-1 2 4.5 2-7 1.5 3.5h5.27" />"###
};
#[cfg(feature = "LuHelpCircle")]
const LU_HELP_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M12 17h.01" />"###
};
#[cfg(feature = "LuHelpingHand")]
const LU_HELPING_HAND: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3 15 5.12-5.12A3 3 0 0 1 10.24 9H13a2 2 0 1 1 0 4h-2.5m4-.68 4.17-4.89a1.88 1.88 0 0 1 2.92 2.36l-4.2 5.94A3 3 0 0 1 14.96 17H9.83a2 2 0 0 0-1.42.59L7 19" />
<path d="m2 14 6 6" />"###
};
#[cfg(feature = "LuHexagon")]
const LU_HEXAGON: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuHighlighter")]
const LU_HIGHLIGHTER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m9 11-6 6v3h9l3-3" />
<path d="m22 12-4.6 4.6a2 2 0 0 1-2.8 0l-5.2-5.2a2 2 0 0 1 0-2.8L14 4" />"###
};
#[cfg(feature = "LuHistory")]
const LU_HISTORY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
<path d="M3 3v5h5" />
<path d="M12 7v5l4 2" />"###
};
#[cfg(feature = "LuHome")]
const LU_HOME: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" />
<polyline points="9 22 9 12 15 12 15 22" />"###
};
#[cfg(feature = "LuHop")]
const LU_HOP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17.5 5.5C19 7 20.5 9 21 11c-2.5.5-5 .5-8.5-1" />
<path d="M5.5 17.5C7 19 9 20.5 11 21c.5-2.5.5-5-1-8.5" />
<path d="M16.5 11.5c1 2 1 3.5 1 6-2.5 0-4 0-6-1" />
<path d="M20 11.5c1 1.5 2 3.5 2 4.5-1.5.5-3 0-4.5-.5" />
<path d="M11.5 20c1.5 1 3.5 2 4.5 2 .5-1.5 0-3-.5-4.5" />
<path d="M20.5 16.5c1 2 1.5 3.5 1.5 5.5-2 0-3.5-.5-5.5-1.5" />
<path d="M4.783 4.782C8.493 1.072 14.5 1 18 5c-1 1-4.5 2-6.5 1.5 1 1.5 1 4 .5 5.5-1.5.5-4 .5-5.5-.5C7 13.5 6 17 5 18c-4-3.5-3.927-9.508-.217-13.218Z" />
<path d="M4.5 4.5 3 3c-.184-.185-.184-.816 0-1" />"###
};
#[cfg(feature = "LuHopOff")]
const LU_HOP_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17.5 5.5C19 7 20.5 9 21 11c-1.323.265-2.646.39-4.118.226" />
<path d="M5.5 17.5C7 19 9 20.5 11 21c.5-2.5.5-5-1-8.5" />
<path d="M17.5 17.5c-2.5 0-4 0-6-1" />
<path d="M20 11.5c1 1.5 2 3.5 2 4.5" />
<path d="M11.5 20c1.5 1 3.5 2 4.5 2 .5-1.5 0-3-.5-4.5" />
<path d="M22 22c-2 0-3.5-.5-5.5-1.5" />
<path d="M4.783 4.782C1.073 8.492 1 14.5 5 18c1-1 2-4.5 1.5-6.5 1.5 1 4 1 5.5.5M8.227 2.57C11.578 1.335 15.453 2.089 18 5c-.88.88-3.7 1.761-5.726 1.618" />
<line x1="2" x2="22" y1="2" y2="22" />"###
};
#[cfg(feature = "LuHotel")]
const LU_HOTEL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V4a2 2 0 0 0-2-2Z" />
<path d="m9 16 .348-.24c1.465-1.013 3.84-1.013 5.304 0L15 16" />
<path d="M8 7h.01" />
<path d="M16 7h.01" />
<path d="M12 7h.01" />
<path d="M12 11h.01" />
<path d="M16 11h.01" />
<path d="M8 11h.01" />
<path d="M10 22v-6.5m4 0V22" />"###
};
#[cfg(feature = "LuHourglass")]
const LU_HOURGLASS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 22h14" />
<path d="M5 2h14" />
<path d="M17 22v-4.172a2 2 0 0 0-.586-1.414L12 12l-4.414 4.414A2 2 0 0 0 7 17.828V22" />
<path d="M7 2v4.172a2 2 0 0 0 .586 1.414L12 12l4.414-4.414A2 2 0 0 0 17 6.172V2" />"###
};
#[cfg(feature = "LuIceCream")]
const LU_ICE_CREAM: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m7 11 4.08 10.35a1 1 0 0 0 1.84 0L17 11" />
<path d="M17 7A5 5 0 0 0 7 7" />
<path d="M17 7a2 2 0 0 1 0 4H7a2 2 0 0 1 0-4" />"###
};
#[cfg(feature = "LuIceCream2")]
const LU_ICE_CREAM2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 17c5 0 8-2.69 8-6H4c0 3.31 3 6 8 6Zm-4 4h8m-4-3v3M5.14 11a3.5 3.5 0 1 1 6.71 0" />
<path d="M12.14 11a3.5 3.5 0 1 1 6.71 0" />
<path d="M15.5 6.5a3.5 3.5 0 1 0-7 0" />"###
};
#[cfg(feature = "LuImage")]
const LU_IMAGE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<circle cx="9" cy="9" r="2" />
<path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21" />"###
};
#[cfg(feature = "LuImageMinus")]
const LU_IMAGE_MINUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 9v10a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h7" />
<line x1="16" x2="22" y1="5" y2="5" />
<circle cx="9" cy="9" r="2" />
<path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21" />"###
};
#[cfg(feature = "LuImageOff")]
const LU_IMAGE_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="2" x2="22" y1="2" y2="22" />
<path d="M10.41 10.41a2 2 0 1 1-2.83-2.83" />
<line x1="13.5" x2="6" y1="13.5" y2="21" />
<line x1="18" x2="21" y1="12" y2="15" />
<path d="M3.59 3.59A1.99 1.99 0 0 0 3 5v14a2 2 0 0 0 2 2h14c.55 0 1.052-.22 1.41-.59" />
<path d="M21 15V5a2 2 0 0 0-2-2H9" />"###
};
#[cfg(feature = "LuImagePlus")]
const LU_IMAGE_PLUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h7" />
<line x1="16" x2="22" y1="5" y2="5" />
<line x1="19" x2="19" y1="2" y2="8" />
<circle cx="9" cy="9" r="2" />
<path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21" />"###
};
#[cfg(feature = "LuImport")]
const LU_IMPORT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 3v12" />
<path d="m8 11 4 4 4-4" />
<path d="M8 5H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2h-4" />"###
};
#[cfg(feature = "LuInbox")]
const LU_INBOX: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z" />"###
};
#[cfg(feature = "LuIndent")]
const LU_INDENT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="3 8 7 12 3 16" />
<line x1="21" x2="11" y1="12" y2="12" />
<line x1="21" x2="11" y1="6" y2="6" />
<line x1="21" x2="11" y1="18" y2="18" />"###
};
#[cfg(feature = "LuIndianRupee")]
const LU_INDIAN_RUPEE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 3h12" />
<path d="M6 8h12" />
<path d="m6 13 8.5 8" />
<path d="M6 13h3" />
<path d="M9 13c6.667 0 6.667-10 0-10" />"###
};
#[cfg(feature = "LuInfinity")]
const LU_INFINITY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 12c-2-2.67-4-4-6-4a4 4 0 1 0 0 8c2 0 4-1.33 6-4Zm0 0c2 2.67 4 4 6 4a4 4 0 0 0 0-8c-2 0-4 1.33-6 4Z" />"###
};
#[cfg(feature = "LuInfo")]
const LU_INFO: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M12 16v-4" />
<path d="M12 8h.01" />"###
};
#[cfg(feature = "LuInspect")]
const LU_INSPECT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 11V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h6" />
<path d="m12 12 4 10 1.7-4.3L22 16Z" />"###
};
#[cfg(feature = "LuInstagram")]
const LU_INSTAGRAM: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="20" height="20" x="2" y="2" rx="5" ry="5" />
<path d="M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z" />
<line x1="17.5" x2="17.51" y1="6.5" y2="6.5" />"###
};
#[cfg(feature = "LuItalic")]
const LU_ITALIC: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="19" x2="10" y1="4" y2="4" />
<line x1="14" x2="5" y1="20" y2="20" />
<line x1="15" x2="9" y1="4" y2="20" />"###
};
#[cfg(feature = "LuIterationCcw")]
const LU_ITERATION_CCW: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M20 10c0-4.4-3.6-8-8-8s-8 3.6-8 8 3.6 8 8 8h8" />
<polyline points="16 14 20 18 16 22" />"###
};
#[cfg(feature = "LuIterationCw")]
const LU_ITERATION_CW: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 10c0-4.4 3.6-8 8-8s8 3.6 8 8-3.6 8-8 8H4" />
<polyline points="8 22 4 18 8 14" />"###
};
#[cfg(feature = "LuJapaneseYen")]
const LU_JAPANESE_YEN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 9.5V21m0-11.5L6 3m6 6.5L18 3" />
<path d="M6 15h12" />
<path d="M6 11h12" />"###
};
#[cfg(feature = "LuJoystick")]
const LU_JOYSTICK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 17a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v2a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-2Z" />
<path d="M6 15v-2" />
<path d="M12 15V9" />
<circle cx="12" cy="6" r="3" />"###
};
#[cfg(feature = "LuKanban")]
const LU_KANBAN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 5v11" />
<path d="M12 5v6" />
<path d="M18 5v14" />"###
};
#[cfg(feature = "LuKanbanSquare")]
const LU_KANBAN_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="M8 7v7" />
<path d="M12 7v4" />
<path d="M16 7v9" />"###
};
#[cfg(feature = "LuKanbanSquareDashed")]
const LU_KANBAN_SQUARE_DASHED: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 7v7" />
<path d="M12 7v4" />
<path d="M16 7v9" />
<path d="M5 3a2 2 0 0 0-2 2" />
<path d="M9 3h1" />
<path d="M14 3h1" />
<path d="M19 3a2 2 0 0 1 2 2" />
<path d="M21 9v1" />
<path d="M21 14v1" />
<path d="M21 19a2 2 0 0 1-2 2" />
<path d="M14 21h1" />
<path d="M9 21h1" />
<path d="M5 21a2 2 0 0 1-2-2" />
<path d="M3 14v1" />
<path d="M3 9v1" />"###
};
#[cfg(feature = "LuKey")]
const LU_KEY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="7.5" cy="15.5" r="5.5" />
<path d="m21 2-9.6 9.6" />
<path d="m15.5 7.5 3 3L22 7l-3-3" />"###
};
#[cfg(feature = "LuKeyRound")]
const LU_KEY_ROUND: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 18v3c0 .6.4 1 1 1h4v-3h3v-3h2l1.4-1.4a6.5 6.5 0 1 0-4-4Z" />
<circle cx="16.5" cy="7.5" r=".5" />"###
};
#[cfg(feature = "LuKeySquare")]
const LU_KEY_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12.4 2.7c.9-.9 2.5-.9 3.4 0l5.5 5.5c.9.9.9 2.5 0 3.4l-3.7 3.7c-.9.9-2.5.9-3.4 0L8.7 9.8c-.9-.9-.9-2.5 0-3.4Z" />
<path d="m14 7 3 3" />
<path d="M9.4 10.6 2 18v3c0 .6.4 1 1 1h4v-3h3v-3h2l1.4-1.4" />"###
};
#[cfg(feature = "LuKeyboard")]
const LU_KEYBOARD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="20" height="16" x="2" y="4" rx="2" ry="2" />
<path d="M6 8h.001" />
<path d="M10 8h.001" />
<path d="M14 8h.001" />
<path d="M18 8h.001" />
<path d="M8 12h.001" />
<path d="M12 12h.001" />
<path d="M16 12h.001" />
<path d="M7 16h10" />"###
};
#[cfg(feature = "LuLamp")]
const LU_LAMP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 2h8l4 10H4L8 2Z" />
<path d="M12 12v6" />
<path d="M8 22v-2c0-1.1.9-2 2-2h4a2 2 0 0 1 2 2v2H8Z" />"###
};
#[cfg(feature = "LuLampCeiling")]
const LU_LAMP_CEILING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 2v5" />
<path d="M6 7h12l4 9H2l4-9Z" />
<path d="M9.17 16a3 3 0 1 0 5.66 0" />"###
};
#[cfg(feature = "LuLampDesk")]
const LU_LAMP_DESK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m14 5-3 3 2 7 8-8-7-2Z" />
<path d="m14 5-3 3-3-3 3-3 3 3Z" />
<path d="M9.5 6.5 4 12l3 6" />
<path d="M3 22v-2c0-1.1.9-2 2-2h4a2 2 0 0 1 2 2v2H3Z" />"###
};
#[cfg(feature = "LuLampFloor")]
const LU_LAMP_FLOOR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M9 2h6l3 7H6l3-7Z" />
<path d="M12 9v13" />
<path d="M9 22h6" />"###
};
#[cfg(feature = "LuLampWallDown")]
const LU_LAMP_WALL_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M11 13h6l3 7H8l3-7Z" />
<path d="M14 13V8a2 2 0 0 0-2-2H8" />
<path d="M4 9h2a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2H4v6Z" />"###
};
#[cfg(feature = "LuLampWallUp")]
const LU_LAMP_WALL_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M11 4h6l3 7H8l3-7Z" />
<path d="M14 11v5a2 2 0 0 1-2 2H8" />
<path d="M4 15h2a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2H4v-6Z" />"###
};
#[cfg(feature = "LuLandmark")]
const LU_LANDMARK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="3" x2="21" y1="22" y2="22" />
<line x1="6" x2="6" y1="18" y2="11" />
<line x1="10" x2="10" y1="18" y2="11" />
<line x1="14" x2="14" y1="18" y2="11" />
<line x1="18" x2="18" y1="18" y2="11" />
<polygon points="12 2 20 7 4 7" />"###
};
#[cfg(feature = "LuLanguages")]
const LU_LANGUAGES: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m5 8 6 6" />
<path d="m4 14 6-6 2-3" />
<path d="M2 5h12" />
<path d="M7 2h1" />
<path d="m22 22-5-10-5 10" />
<path d="M14 18h6" />"###
};
#[cfg(feature = "LuLaptop")]
const LU_LAPTOP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M20 16V7a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v9m16 0H4m16 0 1.28 2.55a1 1 0 0 1-.9 1.45H3.62a1 1 0 0 1-.9-1.45L4 16" />"###
};
#[cfg(feature = "LuLaptop2")]
const LU_LAPTOP2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="12" x="3" y="4" rx="2" ry="2" />
<line x1="2" x2="22" y1="20" y2="20" />"###
};
#[cfg(feature = "LuLasso")]
const LU_LASSO: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M7 22a5 5 0 0 1-2-4" />
<path d="M3.3 14A6.8 6.8 0 0 1 2 10c0-4.4 4.5-8 10-8s10 3.6 10 8-4.5 8-10 8a12 12 0 0 1-5-1" />
<path d="M5 18a2 2 0 1 0 0-4 2 2 0 0 0 0 4z" />"###
};
#[cfg(feature = "LuLassoSelect")]
const LU_LASSO_SELECT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M7 22a5 5 0 0 1-2-4" />
<path d="M7 16.93c.96.43 1.96.74 2.99.91" />
<path d="M3.34 14A6.8 6.8 0 0 1 2 10c0-4.42 4.48-8 10-8s10 3.58 10 8a7.19 7.19 0 0 1-.33 2" />
<path d="M5 18a2 2 0 1 0 0-4 2 2 0 0 0 0 4z" />
<path d="M14.33 22h-.09a.35.35 0 0 1-.24-.32v-10a.34.34 0 0 1 .33-.34c.08 0 .15.03.21.08l7.34 6a.33.33 0 0 1-.21.59h-4.49l-2.57 3.85a.35.35 0 0 1-.28.14v0z" />"###
};
#[cfg(feature = "LuLaugh")]
const LU_LAUGH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M18 13a6 6 0 0 1-6 5 6 6 0 0 1-6-5h12Z" />
<line x1="9" x2="9.01" y1="9" y2="9" />
<line x1="15" x2="15.01" y1="9" y2="9" />"###
};
#[cfg(feature = "LuLayers")]
const LU_LAYERS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuLayout")]
const LU_LAYOUT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<line x1="3" x2="21" y1="9" y2="9" />
<line x1="9" x2="9" y1="21" y2="9" />"###
};
#[cfg(feature = "LuLayoutDashboard")]
const LU_LAYOUT_DASHBOARD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="7" height="9" x="3" y="3" rx="1" />
<rect width="7" height="5" x="14" y="3" rx="1" />
<rect width="7" height="9" x="14" y="12" rx="1" />
<rect width="7" height="5" x="3" y="16" rx="1" />"###
};
#[cfg(feature = "LuLayoutGrid")]
const LU_LAYOUT_GRID: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="7" height="7" x="3" y="3" rx="1" />
<rect width="7" height="7" x="14" y="3" rx="1" />
<rect width="7" height="7" x="14" y="14" rx="1" />
<rect width="7" height="7" x="3" y="14" rx="1" />"###
};
#[cfg(feature = "LuLayoutList")]
const LU_LAYOUT_LIST: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="7" height="7" x="3" y="3" rx="1" />
<rect width="7" height="7" x="3" y="14" rx="1" />
<path d="M14 4h7" />
<path d="M14 9h7" />
<path d="M14 15h7" />
<path d="M14 20h7" />"###
};
#[cfg(feature = "LuLayoutPanelLeft")]
const LU_LAYOUT_PANEL_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="7" height="18" x="3" y="3" rx="1" />
<rect width="7" height="7" x="14" y="3" rx="1" />
<rect width="7" height="7" x="14" y="14" rx="1" />"###
};
#[cfg(feature = "LuLayoutPanelTop")]
const LU_LAYOUT_PANEL_TOP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="7" x="3" y="3" rx="1" />
<rect width="7" height="7" x="3" y="14" rx="1" />
<rect width="7" height="7" x="14" y="14" rx="1" />"###
};
#[cfg(feature = "LuLayoutTemplate")]
const LU_LAYOUT_TEMPLATE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="7" x="3" y="3" rx="1" />
<rect width="9" height="7" x="3" y="14" rx="1" />
<rect width="5" height="7" x="16" y="14" rx="1" />"###
};
#[cfg(feature = "LuLeaf")]
const LU_LEAF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M11 20A7 7 0 0 1 9.8 6.1C15.5 5 17 4.48 19 2c1 2 2 4.18 2 8 0 5.5-4.78 10-10 10Z" />
<path d="M2 21c0-3 1.85-5.36 5.08-6C9.5 14.52 12 13 13 12" />"###
};
#[cfg(feature = "LuLeafyGreen")]
const LU_LEAFY_GREEN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 22c1.25-.987 2.27-1.975 3.9-2.2a5.56 5.56 0 0 1 3.8 1.5 4 4 0 0 0 6.187-2.353 3.5 3.5 0 0 0 3.69-5.116A3.5 3.5 0 0 0 20.95 8 3.5 3.5 0 1 0 16 3.05a3.5 3.5 0 0 0-5.831 1.373 3.5 3.5 0 0 0-5.116 3.69 4 4 0 0 0-2.348 6.155C3.499 15.42 4.409 16.712 4.2 18.1 3.926 19.743 3.014 20.732 2 22" />
<path d="M2 22 17 7" />"###
};
#[cfg(feature = "LuLibrary")]
const LU_LIBRARY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m16 6 4 14" />
<path d="M12 6v14" />
<path d="M8 8v12" />
<path d="M4 4v16" />"###
};
#[cfg(feature = "LuLifeBuoy")]
const LU_LIFE_BUOY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="m4.93 4.93 4.24 4.24" />
<path d="m14.83 9.17 4.24-4.24" />
<path d="m14.83 14.83 4.24 4.24" />
<path d="m9.17 14.83-4.24 4.24" />
<circle cx="12" cy="12" r="4" />"###
};
#[cfg(feature = "LuLigature")]
const LU_LIGATURE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 20V8c0-2.2 1.8-4 4-4 1.5 0 2.8.8 3.5 2" />
<path d="M6 12h4" />
<path d="M14 12h2v8" />
<path d="M6 20h4" />
<path d="M14 20h4" />"###
};
#[cfg(feature = "LuLightbulb")]
const LU_LIGHTBULB: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M15 14c.2-1 .7-1.7 1.5-2.5 1-.9 1.5-2.2 1.5-3.5A6 6 0 0 0 6 8c0 1 .2 2.2 1.5 3.5.7.7 1.3 1.5 1.5 2.5" />
<path d="M9 18h6" />
<path d="M10 22h4" />"###
};
#[cfg(feature = "LuLightbulbOff")]
const LU_LIGHTBULB_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16.8 11.2c.8-.9 1.2-2 1.2-3.2a6 6 0 0 0-9.3-5" />
<path d="m2 2 20 20" />
<path d="M6.3 6.3a4.67 4.67 0 0 0 1.2 5.2c.7.7 1.3 1.5 1.5 2.5" />
<path d="M9 18h6" />
<path d="M10 22h4" />"###
};
#[cfg(feature = "LuLineChart")]
const LU_LINE_CHART: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 3v18h18" />
<path d="m19 9-5 5-4-4-3 3" />"###
};
#[cfg(feature = "LuLink")]
const LU_LINK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuLink2")]
const LU_LINK2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M9 17H7A5 5 0 0 1 7 7h2" />
<path d="M15 7h2a5 5 0 1 1 0 10h-2" />
<line x1="8" x2="16" y1="12" y2="12" />"###
};
#[cfg(feature = "LuLink2Off")]
const LU_LINK2_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M9 17H7A5 5 0 0 1 7 7" />
<path d="M15 7h2a5 5 0 0 1 4 8" />
<line x1="8" x2="12" y1="12" y2="12" />
<line x1="2" x2="22" y1="2" y2="22" />"###
};
#[cfg(feature = "LuLinkedin")]
const LU_LINKEDIN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<rect width="4" height="12" x="2" y="9" />
<circle cx="4" cy="4" r="2" />"###
};
#[cfg(feature = "LuList")]
const LU_LIST: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="8" x2="21" y1="6" y2="6" />
<line x1="8" x2="21" y1="12" y2="12" />
<line x1="8" x2="21" y1="18" y2="18" />
<line x1="3" x2="3.01" y1="6" y2="6" />
<line x1="3" x2="3.01" y1="12" y2="12" />
<line x1="3" x2="3.01" y1="18" y2="18" />"###
};
#[cfg(feature = "LuListChecks")]
const LU_LIST_CHECKS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3 17 2 2 4-4" />
<path d="m3 7 2 2 4-4" />
<path d="M13 6h8" />
<path d="M13 12h8" />
<path d="M13 18h8" />"###
};
#[cfg(feature = "LuListEnd")]
const LU_LIST_END: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16 12H3" />
<path d="M16 6H3" />
<path d="M10 18H3" />
<path d="M21 6v10a2 2 0 0 1-2 2h-5" />
<path d="m16 16-2 2 2 2" />"###
};
#[cfg(feature = "LuListFilter")]
const LU_LIST_FILTER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 6h18" />
<path d="M7 12h10" />
<path d="M10 18h4" />"###
};
#[cfg(feature = "LuListMinus")]
const LU_LIST_MINUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M11 12H3" />
<path d="M16 6H3" />
<path d="M16 18H3" />
<path d="M21 12h-6" />"###
};
#[cfg(feature = "LuListMusic")]
const LU_LIST_MUSIC: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 15V6" />
<path d="M18.5 18a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5Z" />
<path d="M12 12H3" />
<path d="M16 6H3" />
<path d="M12 18H3" />"###
};
#[cfg(feature = "LuListOrdered")]
const LU_LIST_ORDERED: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="10" x2="21" y1="6" y2="6" />
<line x1="10" x2="21" y1="12" y2="12" />
<line x1="10" x2="21" y1="18" y2="18" />
<path d="M4 6h1v4" />
<path d="M4 10h2" />
<path d="M6 18H4c0-1 2-2 2-3s-1-1.5-2-1" />"###
};
#[cfg(feature = "LuListPlus")]
const LU_LIST_PLUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M11 12H3" />
<path d="M16 6H3" />
<path d="M16 18H3" />
<path d="M18 9v6" />
<path d="M21 12h-6" />"###
};
#[cfg(feature = "LuListRestart")]
const LU_LIST_RESTART: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 6H3" />
<path d="M7 12H3" />
<path d="M7 18H3" />
<path d="M12 18a5 5 0 0 0 9-3 4.5 4.5 0 0 0-4.5-4.5c-1.33 0-2.54.54-3.41 1.41L11 14" />
<path d="M11 10v4h4" />"###
};
#[cfg(feature = "LuListStart")]
const LU_LIST_START: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16 12H3" />
<path d="M16 18H3" />
<path d="M10 6H3" />
<path d="M21 18V8a2 2 0 0 0-2-2h-5" />
<path d="m16 8-2-2 2-2" />"###
};
#[cfg(feature = "LuListTodo")]
const LU_LIST_TODO: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="3" y="5" width="6" height="6" rx="1" />
<path d="m3 17 2 2 4-4" />
<path d="M13 6h8" />
<path d="M13 12h8" />
<path d="M13 18h8" />"###
};
#[cfg(feature = "LuListTree")]
const LU_LIST_TREE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 12h-8" />
<path d="M21 6H8" />
<path d="M21 18h-8" />
<path d="M3 6v4c0 1.1.9 2 2 2h3" />
<path d="M3 10v6c0 1.1.9 2 2 2h3" />"###
};
#[cfg(feature = "LuListVideo")]
const LU_LIST_VIDEO: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 12H3" />
<path d="M16 6H3" />
<path d="M12 18H3" />
<path d="m16 12 5 3-5 3v-6Z" />"###
};
#[cfg(feature = "LuListX")]
const LU_LIST_X: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M11 12H3" />
<path d="M16 6H3" />
<path d="M16 18H3" />
<path d="m19 10-4 4" />
<path d="m15 10 4 4" />"###
};
#[cfg(feature = "LuLoader")]
const LU_LOADER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="12" x2="12" y1="2" y2="6" />
<line x1="12" x2="12" y1="18" y2="22" />
<line x1="4.93" x2="7.76" y1="4.93" y2="7.76" />
<line x1="16.24" x2="19.07" y1="16.24" y2="19.07" />
<line x1="2" x2="6" y1="12" y2="12" />
<line x1="18" x2="22" y1="12" y2="12" />
<line x1="4.93" x2="7.76" y1="19.07" y2="16.24" />
<line x1="16.24" x2="19.07" y1="7.76" y2="4.93" />"###
};
#[cfg(feature = "LuLoader2")]
const LU_LOADER2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 12a9 9 0 1 1-6.219-8.56" />"###
};
#[cfg(feature = "LuLocate")]
const LU_LOCATE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="2" x2="5" y1="12" y2="12" />
<line x1="19" x2="22" y1="12" y2="12" />
<line x1="12" x2="12" y1="2" y2="5" />
<line x1="12" x2="12" y1="19" y2="22" />
<circle cx="12" cy="12" r="7" />"###
};
#[cfg(feature = "LuLocateFixed")]
const LU_LOCATE_FIXED: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="2" x2="5" y1="12" y2="12" />
<line x1="19" x2="22" y1="12" y2="12" />
<line x1="12" x2="12" y1="2" y2="5" />
<line x1="12" x2="12" y1="19" y2="22" />
<circle cx="12" cy="12" r="7" />
<circle cx="12" cy="12" r="3" />"###
};
#[cfg(feature = "LuLocateOff")]
const LU_LOCATE_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="2" x2="5" y1="12" y2="12" />
<line x1="19" x2="22" y1="12" y2="12" />
<line x1="12" x2="12" y1="2" y2="5" />
<line x1="12" x2="12" y1="19" y2="22" />
<path d="M7.11 7.11C5.83 8.39 5 10.1 5 12c0 3.87 3.13 7 7 7 1.9 0 3.61-.83 4.89-2.11" />
<path d="M18.71 13.96c.19-.63.29-1.29.29-1.96 0-3.87-3.13-7-7-7-.67 0-1.33.1-1.96.29" />
<line x1="2" x2="22" y1="2" y2="22" />"###
};
#[cfg(feature = "LuLock")]
const LU_LOCK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="11" x="3" y="11" rx="2" ry="2" />
<path d="M7 11V7a5 5 0 0 1 10 0v4" />"###
};
#[cfg(feature = "LuLogIn")]
const LU_LOG_IN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="15" x2="3" y1="12" y2="12" />"###
};
#[cfg(feature = "LuLogOut")]
const LU_LOG_OUT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="21" x2="9" y1="12" y2="12" />"###
};
#[cfg(feature = "LuLollipop")]
const LU_LOLLIPOP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="m21 21-4.3-4.3" />
<path d="M11 11a2 2 0 0 0 4 0 4 4 0 0 0-8 0 6 6 0 0 0 12 0" />"###
};
#[cfg(feature = "LuLuggage")]
const LU_LUGGAGE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 20h0a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h0" />
<path d="M8 18V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v14" />
<path d="M10 20h4" />
<circle cx="16" cy="20" r="2" />
<circle cx="8" cy="20" r="2" />"###
};
#[cfg(feature = "LuMSquare")]
const LU_M_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="M8 16V8l4 4 4-4v8" />"###
};
#[cfg(feature = "LuMagnet")]
const LU_MAGNET: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m6 15-4-4 6.75-6.77a7.79 7.79 0 0 1 11 11L13 22l-4-4 6.39-6.36a2.14 2.14 0 0 0-3-3L6 15" />
<path d="m5 8 4 4" />
<path d="m12 15 4 4" />"###
};
#[cfg(feature = "LuMail")]
const LU_MAIL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="20" height="16" x="2" y="4" rx="2" />
<path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" />"###
};
#[cfg(feature = "LuMailCheck")]
const LU_MAIL_CHECK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 13V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h8" />
<path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" />
<path d="m16 19 2 2 4-4" />"###
};
#[cfg(feature = "LuMailMinus")]
const LU_MAIL_MINUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 15V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h8" />
<path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" />
<path d="M16 19h6" />"###
};
#[cfg(feature = "LuMailOpen")]
const LU_MAIL_OPEN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21.2 8.4c.5.38.8.97.8 1.6v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V10a2 2 0 0 1 .8-1.6l8-6a2 2 0 0 1 2.4 0l8 6Z" />
<path d="m22 10-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 10" />"###
};
#[cfg(feature = "LuMailPlus")]
const LU_MAIL_PLUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 13V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h8" />
<path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" />
<path d="M19 16v6" />
<path d="M16 19h6" />"###
};
#[cfg(feature = "LuMailQuestion")]
const LU_MAIL_QUESTION: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 10.5V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h12.5" />
<path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" />
<path d="M18 15.28c.2-.4.5-.8.9-1a2.1 2.1 0 0 1 2.6.4c.3.4.5.8.5 1.3 0 1.3-2 2-2 2" />
<path d="M20 22v.01" />"###
};
#[cfg(feature = "LuMailSearch")]
const LU_MAIL_SEARCH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 12.5V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h7.5" />
<path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" />
<path d="M18 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6v0Z" />
<circle cx="18" cy="18" r="3" />
<path d="m22 22-1.5-1.5" />"###
};
#[cfg(feature = "LuMailWarning")]
const LU_MAIL_WARNING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 10.5V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h12.5" />
<path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" />
<path d="M20 14v4" />
<path d="M20 22v.01" />"###
};
#[cfg(feature = "LuMailX")]
const LU_MAIL_X: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 13V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h9" />
<path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" />
<path d="m17 17 4 4" />
<path d="m21 17-4 4" />"###
};
#[cfg(feature = "LuMailbox")]
const LU_MAILBOX: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 17a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V9.5C2 7 4 5 6.5 5H18c2.2 0 4 1.8 4 4v8Z" />
<polyline points="15,9 18,9 18,11" />
<path d="M6.5 5C9 5 11 7 11 9.5V17a2 2 0 0 1-2 2v0" />
<line x1="6" x2="7" y1="10" y2="10" />"###
};
#[cfg(feature = "LuMails")]
const LU_MAILS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="16" height="13" x="6" y="4" rx="2" />
<path d="m22 7-7.1 3.78c-.57.3-1.23.3-1.8 0L6 7" />
<path d="M2 8v11c0 1.1.9 2 2 2h14" />"###
};
#[cfg(feature = "LuMap")]
const LU_MAP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polygon points="3 6 9 3 15 6 21 3 21 18 15 21 9 18 3 21" />
<line x1="9" x2="9" y1="3" y2="18" />
<line x1="15" x2="15" y1="6" y2="21" />"###
};
#[cfg(feature = "LuMapPin")]
const LU_MAP_PIN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M20 10c0 6-8 12-8 12s-8-6-8-12a8 8 0 0 1 16 0Z" />
<circle cx="12" cy="10" r="3" />"###
};
#[cfg(feature = "LuMapPinOff")]
const LU_MAP_PIN_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5.43 5.43A8.06 8.06 0 0 0 4 10c0 6 8 12 8 12a29.94 29.94 0 0 0 5-5" />
<path d="M19.18 13.52A8.66 8.66 0 0 0 20 10a8 8 0 0 0-8-8 7.88 7.88 0 0 0-3.52.82" />
<path d="M9.13 9.13A2.78 2.78 0 0 0 9 10a3 3 0 0 0 3 3 2.78 2.78 0 0 0 .87-.13" />
<path d="M14.9 9.25a3 3 0 0 0-2.15-2.16" />
<line x1="2" x2="22" y1="2" y2="22" />"###
};
#[cfg(feature = "LuMartini")]
const LU_MARTINI: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 22h8" />
<path d="M12 11v11" />
<path d="m19 3-7 8-7-8Z" />"###
};
#[cfg(feature = "LuMaximize")]
const LU_MAXIMIZE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 3H5a2 2 0 0 0-2 2v3" />
<path d="M21 8V5a2 2 0 0 0-2-2h-3" />
<path d="M3 16v3a2 2 0 0 0 2 2h3" />
<path d="M16 21h3a2 2 0 0 0 2-2v-3" />"###
};
#[cfg(feature = "LuMaximize2")]
const LU_MAXIMIZE2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="21" x2="14" y1="3" y2="10" />
<line x1="3" x2="10" y1="21" y2="14" />"###
};
#[cfg(feature = "LuMedal")]
const LU_MEDAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M7.21 15 2.66 7.14a2 2 0 0 1 .13-2.2L4.4 2.8A2 2 0 0 1 6 2h12a2 2 0 0 1 1.6.8l1.6 2.14a2 2 0 0 1 .14 2.2L16.79 15" />
<path d="M11 12 5.12 2.2" />
<path d="m13 12 5.88-9.8" />
<path d="M8 7h8" />
<circle cx="12" cy="17" r="5" />
<path d="M12 18v-2h-.5" />"###
};
#[cfg(feature = "LuMegaphone")]
const LU_MEGAPHONE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3 11 18-5v12L3 14v-3z" />
<path d="M11.6 16.8a3 3 0 1 1-5.8-1.6" />"###
};
#[cfg(feature = "LuMegaphoneOff")]
const LU_MEGAPHONE_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M9.26 9.26 3 11v3l14.14 3.14" />
<path d="M21 15.34V6l-7.31 2.03" />
<path d="M11.6 16.8a3 3 0 1 1-5.8-1.6" />
<line x1="2" x2="22" y1="2" y2="22" />"###
};
#[cfg(feature = "LuMeh")]
const LU_MEH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="8" x2="16" y1="15" y2="15" />
<line x1="9" x2="9.01" y1="9" y2="9" />
<line x1="15" x2="15.01" y1="9" y2="9" />"###
};
#[cfg(feature = "LuMemoryStick")]
const LU_MEMORY_STICK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 19v-3" />
<path d="M10 19v-3" />
<path d="M14 19v-3" />
<path d="M18 19v-3" />
<path d="M8 11V9" />
<path d="M16 11V9" />
<path d="M12 11V9" />
<path d="M2 15h20" />
<path d="M2 7a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v1.1a2 2 0 0 0 0 3.837V17a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-5.1a2 2 0 0 0 0-3.837Z" />"###
};
#[cfg(feature = "LuMenu")]
const LU_MENU: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="4" x2="20" y1="12" y2="12" />
<line x1="4" x2="20" y1="6" y2="6" />
<line x1="4" x2="20" y1="18" y2="18" />"###
};
#[cfg(feature = "LuMenuSquare")]
const LU_MENU_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="M7 8h10" />
<path d="M7 12h10" />
<path d="M7 16h10" />"###
};
#[cfg(feature = "LuMerge")]
const LU_MERGE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m8 6 4-4 4 4" />
<path d="M12 2v10.3a4 4 0 0 1-1.172 2.872L4 22" />
<path d="m20 22-5-5" />"###
};
#[cfg(feature = "LuMessageCircle")]
const LU_MESSAGE_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3 21 1.9-5.7a8.5 8.5 0 1 1 3.8 3.8z" />"###
};
#[cfg(feature = "LuMessageSquare")]
const LU_MESSAGE_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuMessageSquareDashed")]
const LU_MESSAGE_SQUARE_DASHED: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 6V5c0-1.1.9-2 2-2h2" />
<path d="M11 3h3" />
<path d="M18 3h1c1.1 0 2 .9 2 2" />
<path d="M21 9v2" />
<path d="M21 15c0 1.1-.9 2-2 2h-1" />
<path d="M14 17h-3" />
<path d="m7 17-4 4v-5" />
<path d="M3 12v-2" />"###
};
#[cfg(feature = "LuMessageSquarePlus")]
const LU_MESSAGE_SQUARE_PLUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
<line x1="9" x2="15" y1="10" y2="10" />
<line x1="12" x2="12" y1="7" y2="13" />"###
};
#[cfg(feature = "LuMessagesSquare")]
const LU_MESSAGES_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14 9a2 2 0 0 1-2 2H6l-4 4V4c0-1.1.9-2 2-2h8a2 2 0 0 1 2 2v5Z" />
<path d="M18 9h2a2 2 0 0 1 2 2v11l-4-4h-6a2 2 0 0 1-2-2v-1" />"###
};
#[cfg(feature = "LuMic")]
const LU_MIC: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 2a3 3 0 0 0-3 3v7a3 3 0 0 0 6 0V5a3 3 0 0 0-3-3Z" />
<path d="M19 10v2a7 7 0 0 1-14 0v-2" />
<line x1="12" x2="12" y1="19" y2="22" />"###
};
#[cfg(feature = "LuMic2")]
const LU_MIC2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m12 8-9.04 9.06a2.82 2.82 0 1 0 3.98 3.98L16 12" />
<circle cx="17" cy="7" r="5" />"###
};
#[cfg(feature = "LuMicOff")]
const LU_MIC_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="2" x2="22" y1="2" y2="22" />
<path d="M18.89 13.23A7.12 7.12 0 0 0 19 12v-2" />
<path d="M5 10v2a7 7 0 0 0 12 5" />
<path d="M15 9.34V5a3 3 0 0 0-5.68-1.33" />
<path d="M9 9v3a3 3 0 0 0 5.12 2.12" />
<line x1="12" x2="12" y1="19" y2="22" />"###
};
#[cfg(feature = "LuMicroscope")]
const LU_MICROSCOPE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 18h8" />
<path d="M3 22h18" />
<path d="M14 22a7 7 0 1 0 0-14h-1" />
<path d="M9 14h2" />
<path d="M9 12a2 2 0 0 1-2-2V6h6v4a2 2 0 0 1-2 2Z" />
<path d="M12 6V3a1 1 0 0 0-1-1H9a1 1 0 0 0-1 1v3" />"###
};
#[cfg(feature = "LuMicrowave")]
const LU_MICROWAVE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="20" height="15" x="2" y="4" rx="2" />
<rect width="8" height="7" x="6" y="8" rx="1" />
<path d="M18 8v7" />
<path d="M6 19v2" />
<path d="M18 19v2" />"###
};
#[cfg(feature = "LuMilestone")]
const LU_MILESTONE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18 6H5a2 2 0 0 0-2 2v3a2 2 0 0 0 2 2h13l4-3.5L18 6Z" />
<path d="M12 13v8" />
<path d="M12 3v3" />"###
};
#[cfg(feature = "LuMilk")]
const LU_MILK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 2h8" />
<path d="M9 2v2.789a4 4 0 0 1-.672 2.219l-.656.984A4 4 0 0 0 7 10.212V20a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2v-9.789a4 4 0 0 0-.672-2.219l-.656-.984A4 4 0 0 1 15 4.788V2" />
<path d="M7 15a6.472 6.472 0 0 1 5 0 6.47 6.47 0 0 0 5 0" />"###
};
#[cfg(feature = "LuMilkOff")]
const LU_MILK_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 2h8" />
<path d="M9 2v1.343M15 2v2.789a4 4 0 0 0 .672 2.219l.656.984a4 4 0 0 1 .672 2.22v1.131M7.8 7.8l-.128.192A4 4 0 0 0 7 10.212V20a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2v-3" />
<path d="M7 15a6.47 6.47 0 0 1 5 0 6.472 6.472 0 0 0 3.435.435" />
<line x1="2" x2="22" y1="2" y2="22" />"###
};
#[cfg(feature = "LuMinimize")]
const LU_MINIMIZE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 3v3a2 2 0 0 1-2 2H3" />
<path d="M21 8h-3a2 2 0 0 1-2-2V3" />
<path d="M3 16h3a2 2 0 0 1 2 2v3" />
<path d="M16 21v-3a2 2 0 0 1 2-2h3" />"###
};
#[cfg(feature = "LuMinimize2")]
const LU_MINIMIZE2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="14" x2="21" y1="10" y2="3" />
<line x1="3" x2="10" y1="21" y2="14" />"###
};
#[cfg(feature = "LuMinus")]
const LU_MINUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 12h14" />"###
};
#[cfg(feature = "LuMinusCircle")]
const LU_MINUS_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M8 12h8" />"###
};
#[cfg(feature = "LuMinusSquare")]
const LU_MINUS_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="M8 12h8" />"###
};
#[cfg(feature = "LuMonitor")]
const LU_MONITOR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="20" height="14" x="2" y="3" rx="2" />
<line x1="8" x2="16" y1="21" y2="21" />
<line x1="12" x2="12" y1="17" y2="21" />"###
};
#[cfg(feature = "LuMonitorCheck")]
const LU_MONITOR_CHECK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m9 10 2 2 4-4" />
<rect width="20" height="14" x="2" y="3" rx="2" />
<path d="M12 17v4" />
<path d="M8 21h8" />"###
};
#[cfg(feature = "LuMonitorDot")]
const LU_MONITOR_DOT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="19" cy="6" r="3" />
<path d="M22 12v3a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h9" />
<path d="M12 17v4" />
<path d="M8 21h8" />"###
};
#[cfg(feature = "LuMonitorDown")]
const LU_MONITOR_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 13V7" />
<path d="m15 10-3 3-3-3" />
<rect width="20" height="14" x="2" y="3" rx="2" />
<path d="M12 17v4" />
<path d="M8 21h8" />"###
};
#[cfg(feature = "LuMonitorOff")]
const LU_MONITOR_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17 17H4a2 2 0 0 1-2-2V5c0-1.5 1-2 1-2" />
<path d="M22 15V5a2 2 0 0 0-2-2H9" />
<path d="M8 21h8" />
<path d="M12 17v4" />
<path d="m2 2 20 20" />"###
};
#[cfg(feature = "LuMonitorPause")]
const LU_MONITOR_PAUSE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10 13V7" />
<path d="M14 13V7" />
<rect width="20" height="14" x="2" y="3" rx="2" />
<path d="M12 17v4" />
<path d="M8 21h8" />"###
};
#[cfg(feature = "LuMonitorPlay")]
const LU_MONITOR_PLAY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m10 7 5 3-5 3Z" />
<rect width="20" height="14" x="2" y="3" rx="2" />
<path d="M12 17v4" />
<path d="M8 21h8" />"###
};
#[cfg(feature = "LuMonitorSmartphone")]
const LU_MONITOR_SMARTPHONE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18 8V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h8" />
<path d="M10 19v-3.96 3.15" />
<path d="M7 19h5" />
<rect width="6" height="10" x="16" y="12" rx="2" />"###
};
#[cfg(feature = "LuMonitorSpeaker")]
const LU_MONITOR_SPEAKER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5.5 20H8" />
<path d="M17 9h.01" />
<rect width="10" height="16" x="12" y="4" rx="2" />
<path d="M8 6H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h4" />
<circle cx="17" cy="15" r="1" />"###
};
#[cfg(feature = "LuMonitorStop")]
const LU_MONITOR_STOP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="9" y="7" width="6" height="6" />
<rect width="20" height="14" x="2" y="3" rx="2" />
<path d="M12 17v4" />
<path d="M8 21h8" />"###
};
#[cfg(feature = "LuMonitorUp")]
const LU_MONITOR_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m9 10 3-3 3 3" />
<path d="M12 13V7" />
<rect width="20" height="14" x="2" y="3" rx="2" />
<path d="M12 17v4" />
<path d="M8 21h8" />"###
};
#[cfg(feature = "LuMonitorX")]
const LU_MONITOR_X: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m14.5 12.5-5-5" />
<path d="m9.5 12.5 5-5" />
<rect width="20" height="14" x="2" y="3" rx="2" />
<path d="M12 17v4" />
<path d="M8 21h8" />"###
};
#[cfg(feature = "LuMoon")]
const LU_MOON: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" />"###
};
#[cfg(feature = "LuMoonStar")]
const LU_MOON_STAR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" />
<path d="M19 3v4" />
<path d="M21 5h-4" />"###
};
#[cfg(feature = "LuMoreHorizontal")]
const LU_MORE_HORIZONTAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuMoreVertical")]
const LU_MORE_VERTICAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuMountain")]
const LU_MOUNTAIN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m8 3 4 8 5-5 5 15H2L8 3z" />"###
};
#[cfg(feature = "LuMountainSnow")]
const LU_MOUNTAIN_SNOW: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m8 3 4 8 5-5 5 15H2L8 3z" />
<path d="M4.14 15.08c2.62-1.57 5.24-1.43 7.86.42 2.74 1.94 5.49 2 8.23.19" />"###
};
#[cfg(feature = "LuMouse")]
const LU_MOUSE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="5" y="2" width="14" height="20" rx="7" />
<path d="M12 6v4" />"###
};
#[cfg(feature = "LuMousePointer")]
const LU_MOUSE_POINTER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3 3 7.07 16.97 2.51-7.39 7.39-2.51L3 3z" />
<path d="m13 13 6 6" />"###
};
#[cfg(feature = "LuMousePointer2")]
const LU_MOUSE_POINTER2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m4 4 7.07 17 2.51-7.39L21 11.07z" />"###
};
#[cfg(feature = "LuMousePointerClick")]
const LU_MOUSE_POINTER_CLICK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m9 9 5 12 1.774-5.226L21 14 9 9z" />
<path d="m16.071 16.071 4.243 4.243" />
<path d="m7.188 2.239.777 2.897M5.136 7.965l-2.898-.777M13.95 4.05l-2.122 2.122m-5.657 5.656-2.12 2.122" />"###
};
#[cfg(feature = "LuMove")]
const LU_MOVE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="2" x2="22" y1="12" y2="12" />
<line x1="12" x2="12" y1="2" y2="22" />"###
};
#[cfg(feature = "LuMove3d")]
const LU_MOVE3D: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 3v16h16" />
<path d="m5 19 6-6" />
<path d="m2 6 3-3 3 3" />
<path d="m18 16 3 3-3 3" />"###
};
#[cfg(feature = "LuMoveDiagonal")]
const LU_MOVE_DIAGONAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="13 5 19 5 19 11" />
<polyline points="11 19 5 19 5 13" />
<line x1="19" x2="5" y1="5" y2="19" />"###
};
#[cfg(feature = "LuMoveDiagonal2")]
const LU_MOVE_DIAGONAL2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="5 11 5 5 11 5" />
<polyline points="19 13 19 19 13 19" />
<line x1="5" x2="19" y1="5" y2="19" />"###
};
#[cfg(feature = "LuMoveDown")]
const LU_MOVE_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 18L12 22L16 18" />
<path d="M12 2V22" />"###
};
#[cfg(feature = "LuMoveDownLeft")]
const LU_MOVE_DOWN_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M11 19H5V13" />
<path d="M19 5L5 19" />"###
};
#[cfg(feature = "LuMoveDownRight")]
const LU_MOVE_DOWN_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M19 13V19H13" />
<path d="M5 5L19 19" />"###
};
#[cfg(feature = "LuMoveHorizontal")]
const LU_MOVE_HORIZONTAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="18 8 22 12 18 16" />
<polyline points="6 8 2 12 6 16" />
<line x1="2" x2="22" y1="12" y2="12" />"###
};
#[cfg(feature = "LuMoveLeft")]
const LU_MOVE_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 8L2 12L6 16" />
<path d="M2 12H22" />"###
};
#[cfg(feature = "LuMoveRight")]
const LU_MOVE_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18 8L22 12L18 16" />
<path d="M2 12H22" />"###
};
#[cfg(feature = "LuMoveUp")]
const LU_MOVE_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 6L12 2L16 6" />
<path d="M12 2V22" />"###
};
#[cfg(feature = "LuMoveUpLeft")]
const LU_MOVE_UP_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 11V5H11" />
<path d="M5 5L19 19" />"###
};
#[cfg(feature = "LuMoveUpRight")]
const LU_MOVE_UP_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M13 5H19V11" />
<path d="M19 5L5 19" />"###
};
#[cfg(feature = "LuMoveVertical")]
const LU_MOVE_VERTICAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="8 18 12 22 16 18" />
<polyline points="8 6 12 2 16 6" />
<line x1="12" x2="12" y1="2" y2="22" />"###
};
#[cfg(feature = "LuMusic")]
const LU_MUSIC: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuMusic2")]
const LU_MUSIC2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="8" cy="18" r="4" />
<path d="M12 18V2l7 4" />"###
};
#[cfg(feature = "LuMusic3")]
const LU_MUSIC3: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="18" r="4" />
<path d="M16 18V2" />"###
};
#[cfg(feature = "LuMusic4")]
const LU_MUSIC4: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="m9 9 12-2" />
<circle cx="6" cy="18" r="3" />
<circle cx="18" cy="16" r="3" />"###
};
#[cfg(feature = "LuNavigation")]
const LU_NAVIGATION: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuNavigation2")]
const LU_NAVIGATION2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuNavigation2Off")]
const LU_NAVIGATION2_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M9.31 9.31 5 21l7-4 7 4-1.17-3.17" />
<path d="M14.53 8.88 12 2l-1.17 3.17" />
<line x1="2" x2="22" y1="2" y2="22" />"###
};
#[cfg(feature = "LuNavigationOff")]
const LU_NAVIGATION_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8.43 8.43 3 11l8 2 2 8 2.57-5.43" />
<path d="M17.39 11.73 22 2l-9.73 4.61" />
<line x1="2" x2="22" y1="2" y2="22" />"###
};
#[cfg(feature = "LuNetwork")]
const LU_NETWORK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="16" y="16" width="6" height="6" rx="1" />
<rect x="2" y="16" width="6" height="6" rx="1" />
<rect x="9" y="2" width="6" height="6" rx="1" />
<path d="M5 16v-3a1 1 0 0 1 1-1h12a1 1 0 0 1 1 1v3" />
<path d="M12 12V8" />"###
};
#[cfg(feature = "LuNewspaper")]
const LU_NEWSPAPER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 22h16a2 2 0 0 0 2-2V4a2 2 0 0 0-2-2H8a2 2 0 0 0-2 2v16a2 2 0 0 1-2 2Zm0 0a2 2 0 0 1-2-2v-9c0-1.1.9-2 2-2h2" />
<path d="M18 14h-8" />
<path d="M15 18h-5" />
<path d="M10 6h8v4h-8V6Z" />"###
};
#[cfg(feature = "LuNfc")]
const LU_NFC: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 8.32a7.43 7.43 0 0 1 0 7.36" />
<path d="M9.46 6.21a11.76 11.76 0 0 1 0 11.58" />
<path d="M12.91 4.1a15.91 15.91 0 0 1 .01 15.8" />
<path d="M16.37 2a20.16 20.16 0 0 1 0 20" />"###
};
#[cfg(feature = "LuNut")]
const LU_NUT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 4V2" />
<path d="M5 10v4a7.004 7.004 0 0 0 5.277 6.787c.412.104.802.292 1.102.592L12 22l.621-.621c.3-.3.69-.488 1.102-.592A7.003 7.003 0 0 0 19 14v-4" />
<path d="M12 4C8 4 4.5 6 4 8c-.243.97-.919 1.952-2 3 1.31-.082 1.972-.29 3-1 .54.92.982 1.356 2 2 1.452-.647 1.954-1.098 2.5-2 .595.995 1.151 1.427 2.5 2 1.31-.621 1.862-1.058 2.5-2 .629.977 1.162 1.423 2.5 2 1.209-.548 1.68-.967 2-2 1.032.916 1.683 1.157 3 1-1.297-1.036-1.758-2.03-2-3-.5-2-4-4-8-4Z" />"###
};
#[cfg(feature = "LuNutOff")]
const LU_NUT_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 4V2" />
<path d="M5 10v4a7.004 7.004 0 0 0 5.277 6.787c.412.104.802.292 1.102.592L12 22l.621-.621c.3-.3.69-.488 1.102-.592a7.01 7.01 0 0 0 4.125-2.939" />
<path d="M19 10v3.343" />
<path d="M12 12c-1.349-.573-1.905-1.005-2.5-2-.546.902-1.048 1.353-2.5 2-1.018-.644-1.46-1.08-2-2-1.028.71-1.69.918-3 1 1.081-1.048 1.757-2.03 2-3 .194-.776.84-1.551 1.79-2.21m11.654 5.997c.887-.457 1.28-.891 1.556-1.787 1.032.916 1.683 1.157 3 1-1.297-1.036-1.758-2.03-2-3-.5-2-4-4-8-4-.74 0-1.461.068-2.15.192" />
<line x1="2" x2="22" y1="2" y2="22" />"###
};
#[cfg(feature = "LuOctagon")]
const LU_OCTAGON: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuOption")]
const LU_OPTION: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 3h6l6 18h6" />
<path d="M14 3h7" />"###
};
#[cfg(feature = "LuOrbit")]
const LU_ORBIT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<circle cx="19" cy="5" r="2" />
<circle cx="5" cy="19" r="2" />
<path d="M10.4 21.9a10 10 0 0 0 9.941-15.416" />
<path d="M13.5 2.1a10 10 0 0 0-9.841 15.416" />"###
};
#[cfg(feature = "LuOutdent")]
const LU_OUTDENT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="7 8 3 12 7 16" />
<line x1="21" x2="11" y1="12" y2="12" />
<line x1="21" x2="11" y1="6" y2="6" />
<line x1="21" x2="11" y1="18" y2="18" />"###
};
#[cfg(feature = "LuPackage")]
const LU_PACKAGE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16.5 9.4 7.55 4.24" />
<path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" />
<polyline points="3.29 7 12 12 20.71 7" />
<line x1="12" x2="12" y1="22" y2="12" />"###
};
#[cfg(feature = "LuPackage2")]
const LU_PACKAGE2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 9h18v10a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V9Z" />
<path d="m3 9 2.45-4.9A2 2 0 0 1 7.24 3h9.52a2 2 0 0 1 1.8 1.1L21 9" />
<path d="M12 3v6" />"###
};
#[cfg(feature = "LuPackageCheck")]
const LU_PACKAGE_CHECK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m16 16 2 2 4-4" />
<path d="M21 10V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l2-1.14" />
<path d="M16.5 9.4 7.55 4.24" />
<polyline points="3.29 7 12 12 20.71 7" />
<line x1="12" x2="12" y1="22" y2="12" />"###
};
#[cfg(feature = "LuPackageMinus")]
const LU_PACKAGE_MINUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16 16h6" />
<path d="M21 10V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l2-1.14" />
<path d="M16.5 9.4 7.55 4.24" />
<polyline points="3.29 7 12 12 20.71 7" />
<line x1="12" x2="12" y1="22" y2="12" />"###
};
#[cfg(feature = "LuPackageOpen")]
const LU_PACKAGE_OPEN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M20.91 8.84 8.56 2.23a1.93 1.93 0 0 0-1.81 0L3.1 4.13a2.12 2.12 0 0 0-.05 3.69l12.22 6.93a2 2 0 0 0 1.94 0L21 12.51a2.12 2.12 0 0 0-.09-3.67Z" />
<path d="m3.09 8.84 12.35-6.61a1.93 1.93 0 0 1 1.81 0l3.65 1.9a2.12 2.12 0 0 1 .1 3.69L8.73 14.75a2 2 0 0 1-1.94 0L3 12.51a2.12 2.12 0 0 1 .09-3.67Z" />
<line x1="12" x2="12" y1="22" y2="13" />
<path d="M20 13.5v3.37a2.06 2.06 0 0 1-1.11 1.83l-6 3.08a1.93 1.93 0 0 1-1.78 0l-6-3.08A2.06 2.06 0 0 1 4 16.87V13.5" />"###
};
#[cfg(feature = "LuPackagePlus")]
const LU_PACKAGE_PLUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16 16h6" />
<path d="M19 13v6" />
<path d="M21 10V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l2-1.14" />
<path d="M16.5 9.4 7.55 4.24" />
<polyline points="3.29 7 12 12 20.71 7" />
<line x1="12" x2="12" y1="22" y2="12" />"###
};
#[cfg(feature = "LuPackageSearch")]
const LU_PACKAGE_SEARCH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 10V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l2-1.14" />
<path d="M16.5 9.4 7.55 4.24" />
<polyline points="3.29 7 12 12 20.71 7" />
<line x1="12" x2="12" y1="22" y2="12" />
<circle cx="18.5" cy="15.5" r="2.5" />
<path d="M20.27 17.27 22 19" />"###
};
#[cfg(feature = "LuPackageX")]
const LU_PACKAGE_X: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 10V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l2-1.14" />
<path d="M16.5 9.4 7.55 4.24" />
<polyline points="3.29 7 12 12 20.71 7" />
<line x1="12" x2="12" y1="22" y2="12" />
<path d="m17 13 5 5m-5 0 5-5" />"###
};
#[cfg(feature = "LuPaintBucket")]
const LU_PAINT_BUCKET: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m19 11-8-8-8.6 8.6a2 2 0 0 0 0 2.8l5.2 5.2c.8.8 2 .8 2.8 0L19 11Z" />
<path d="m5 2 5 5" />
<path d="M2 13h15" />
<path d="M22 20a2 2 0 1 1-4 0c0-1.6 1.7-2.4 2-4 .3 1.6 2 2.4 2 4Z" />"###
};
#[cfg(feature = "LuPaintbrush")]
const LU_PAINTBRUSH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18.37 2.63 14 7l-1.59-1.59a2 2 0 0 0-2.82 0L8 7l9 9 1.59-1.59a2 2 0 0 0 0-2.82L17 10l4.37-4.37a2.12 2.12 0 1 0-3-3Z" />
<path d="M9 8c-2 3-4 3.5-7 4l8 10c2-1 6-5 6-7" />
<path d="M14.5 17.5 4.5 15" />"###
};
#[cfg(feature = "LuPaintbrush2")]
const LU_PAINTBRUSH2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14 19.9V16h3a2 2 0 0 0 2-2v-2H5v2c0 1.1.9 2 2 2h3v3.9a2 2 0 1 0 4 0Z" />
<path d="M6 12V2h12v10" />
<path d="M14 2v4" />
<path d="M10 2v2" />"###
};
#[cfg(feature = "LuPalette")]
const LU_PALETTE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="13.5" cy="6.5" r=".5" />
<circle cx="17.5" cy="10.5" r=".5" />
<circle cx="8.5" cy="7.5" r=".5" />
<circle cx="6.5" cy="12.5" r=".5" />
<path d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.554C21.965 6.012 17.461 2 12 2z" />"###
};
#[cfg(feature = "LuPalmtree")]
const LU_PALMTREE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M13 8c0-2.76-2.46-5-5.5-5S2 5.24 2 8h2l1-1 1 1h4" />
<path d="M13 7.14A5.82 5.82 0 0 1 16.5 6c3.04 0 5.5 2.24 5.5 5h-3l-1-1-1 1h-3" />
<path d="M5.89 9.71c-2.15 2.15-2.3 5.47-.35 7.43l4.24-4.25.7-.7.71-.71 2.12-2.12c-1.95-1.96-5.27-1.8-7.42.35z" />
<path d="M11 15.5c.5 2.5-.17 4.5-1 6.5h4c2-5.5-.5-12-1-14" />"###
};
#[cfg(feature = "LuPanelBottom")]
const LU_PANEL_BOTTOM: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<line x1="3" x2="21" y1="15" y2="15" />"###
};
#[cfg(feature = "LuPanelBottomClose")]
const LU_PANEL_BOTTOM_CLOSE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<line x1="3" x2="21" y1="15" y2="15" />
<path d="m15 8-3 3-3-3" />"###
};
#[cfg(feature = "LuPanelBottomInactive")]
const LU_PANEL_BOTTOM_INACTIVE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="M14 15h1" />
<path d="M19 15h2" />
<path d="M3 15h2" />
<path d="M9 15h1" />"###
};
#[cfg(feature = "LuPanelBottomOpen")]
const LU_PANEL_BOTTOM_OPEN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<line x1="3" x2="21" y1="15" y2="15" />
<path d="m9 10 3-3 3 3" />"###
};
#[cfg(feature = "LuPanelLeft")]
const LU_PANEL_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<line x1="9" x2="9" y1="3" y2="21" />"###
};
#[cfg(feature = "LuPanelLeftClose")]
const LU_PANEL_LEFT_CLOSE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<path d="M9 3v18" />
<path d="m16 15-3-3 3-3" />"###
};
#[cfg(feature = "LuPanelLeftInactive")]
const LU_PANEL_LEFT_INACTIVE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="M9 14v1" />
<path d="M9 19v2" />
<path d="M9 3v2" />
<path d="M9 9v1" />"###
};
#[cfg(feature = "LuPanelLeftOpen")]
const LU_PANEL_LEFT_OPEN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<path d="M9 3v18" />
<path d="m14 9 3 3-3 3" />"###
};
#[cfg(feature = "LuPanelRight")]
const LU_PANEL_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<line x1="15" x2="15" y1="3" y2="21" />"###
};
#[cfg(feature = "LuPanelRightClose")]
const LU_PANEL_RIGHT_CLOSE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<line x1="15" x2="15" y1="3" y2="21" />
<path d="m8 9 3 3-3 3" />"###
};
#[cfg(feature = "LuPanelRightInactive")]
const LU_PANEL_RIGHT_INACTIVE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="M15 14v1" />
<path d="M15 19v2" />
<path d="M15 3v2" />
<path d="M15 9v1" />"###
};
#[cfg(feature = "LuPanelRightOpen")]
const LU_PANEL_RIGHT_OPEN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<line x1="15" x2="15" y1="3" y2="21" />
<path d="m10 15-3-3 3-3" />"###
};
#[cfg(feature = "LuPanelTop")]
const LU_PANEL_TOP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<line x1="3" x2="21" y1="9" y2="9" />"###
};
#[cfg(feature = "LuPanelTopClose")]
const LU_PANEL_TOP_CLOSE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<line x1="3" x2="21" y1="9" y2="9" />
<path d="m9 16 3-3 3 3" />"###
};
#[cfg(feature = "LuPanelTopInactive")]
const LU_PANEL_TOP_INACTIVE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="M14 9h1" />
<path d="M19 9h2" />
<path d="M3 9h2" />
<path d="M9 9h1" />"###
};
#[cfg(feature = "LuPanelTopOpen")]
const LU_PANEL_TOP_OPEN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<line x1="3" x2="21" y1="9" y2="9" />
<path d="m15 14-3 3-3-3" />"###
};
#[cfg(feature = "LuPaperclip")]
const LU_PAPERCLIP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m21.44 11.05-9.19 9.19a6 6 0 0 1-8.49-8.49l8.57-8.57A4 4 0 1 1 18 8.84l-8.59 8.57a2 2 0 0 1-2.83-2.83l8.49-8.48" />"###
};
#[cfg(feature = "LuParentheses")]
const LU_PARENTHESES: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 21s-4-3-4-9 4-9 4-9" />
<path d="M16 3s4 3 4 9-4 9-4 9" />"###
};
#[cfg(feature = "LuParkingCircle")]
const LU_PARKING_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M9 17V7h4a3 3 0 0 1 0 6H9" />"###
};
#[cfg(feature = "LuParkingCircleOff")]
const LU_PARKING_CIRCLE_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="m5 5 14 14" />
<path d="M13 13a3 3 0 1 0 0-6H9v2" />
<path d="M9 17v-2.34" />"###
};
#[cfg(feature = "LuParkingMeter")]
const LU_PARKING_METER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M9 9a3 3 0 1 1 6 0" />
<path d="M12 12v3" />
<path d="M11 15h2" />
<path d="M19 9a7 7 0 1 0-13.6 2.3C6.4 14.4 8 19 8 19h8s1.6-4.6 2.6-7.7c.3-.8.4-1.5.4-2.3" />
<path d="M12 19v3" />"###
};
#[cfg(feature = "LuParkingSquare")]
const LU_PARKING_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="M9 17V7h4a3 3 0 0 1 0 6H9" />"###
};
#[cfg(feature = "LuParkingSquareOff")]
const LU_PARKING_SQUARE_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3.6 3.6A2 2 0 0 1 5 3h14a2 2 0 0 1 2 2v14a2 2 0 0 1-.59 1.41" />
<path d="M3 8.7V19a2 2 0 0 0 2 2h10.3" />
<path d="m2 2 20 20" />
<path d="M13 13a3 3 0 1 0 0-6H9v2" />
<path d="M9 17v-2.3" />"###
};
#[cfg(feature = "LuPartyPopper")]
const LU_PARTY_POPPER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5.8 11.3 2 22l10.7-3.79" />
<path d="M4 3h.01" />
<path d="M22 8h.01" />
<path d="M15 2h.01" />
<path d="M22 20h.01" />
<path d="m22 2-2.24.75a2.9 2.9 0 0 0-1.96 3.12v0c.1.86-.57 1.63-1.45 1.63h-.38c-.86 0-1.6.6-1.76 1.44L14 10" />
<path d="m22 13-.82-.33c-.86-.34-1.82.2-1.98 1.11v0c-.11.7-.72 1.22-1.43 1.22H17" />
<path d="m11 2 .33.82c.34.86-.2 1.82-1.11 1.98v0C9.52 4.9 9 5.52 9 6.23V7" />
<path d="M11 13c1.93 1.93 2.83 4.17 2 5-.83.83-3.07-.07-5-2-1.93-1.93-2.83-4.17-2-5 .83-.83 3.07.07 5 2Z" />"###
};
#[cfg(feature = "LuPause")]
const LU_PAUSE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="4" height="16" x="6" y="4" />
<rect width="4" height="16" x="14" y="4" />"###
};
#[cfg(feature = "LuPauseCircle")]
const LU_PAUSE_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="10" x2="10" y1="15" y2="9" />
<line x1="14" x2="14" y1="15" y2="9" />"###
};
#[cfg(feature = "LuPauseOctagon")]
const LU_PAUSE_OCTAGON: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10 15V9" />
<path d="M14 15V9" />
<path d="M7.714 2h8.572L22 7.714v8.572L16.286 22H7.714L2 16.286V7.714L7.714 2z" />"###
};
#[cfg(feature = "LuPawPrint")]
const LU_PAW_PRINT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="11" cy="4" r="2" />
<circle cx="18" cy="8" r="2" />
<circle cx="20" cy="16" r="2" />
<path d="M9 10a5 5 0 0 1 5 5v3.5a3.5 3.5 0 0 1-6.84 1.045Q6.52 17.48 4.46 16.84A3.5 3.5 0 0 1 5.5 10Z" />"###
};
#[cfg(feature = "LuPcCase")]
const LU_PC_CASE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="14" height="20" x="5" y="2" rx="2" />
<path d="M15 14h.01" />
<path d="M9 6h6" />
<path d="M9 10h6" />"###
};
#[cfg(feature = "LuPen")]
const LU_PEN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z" />"###
};
#[cfg(feature = "LuPenLine")]
const LU_PEN_LINE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z" />"###
};
#[cfg(feature = "LuPenSquare")]
const LU_PEN_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M18.5 2.5a2.12 2.12 0 0 1 3 3L12 15l-4 1 1-4Z" />"###
};
#[cfg(feature = "LuPenTool")]
const LU_PEN_TOOL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m12 19 7-7 3 3-7 7-3-3z" />
<path d="m18 13-1.5-7.5L2 2l3.5 14.5L13 18l5-5z" />
<path d="m2 2 7.586 7.586" />
<circle cx="11" cy="11" r="2" />"###
};
#[cfg(feature = "LuPencil")]
const LU_PENCIL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z" />
<path d="m15 5 4 4" />"###
};
#[cfg(feature = "LuPencilLine")]
const LU_PENCIL_LINE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z" />
<path d="m15 5 3 3" />"###
};
#[cfg(feature = "LuPencilRuler")]
const LU_PENCIL_RULER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m15 5 4 4" />
<path d="M13 7 8.7 2.7a2.41 2.41 0 0 0-3.4 0L2.7 5.3a2.41 2.41 0 0 0 0 3.4L7 13" />
<path d="m8 6 2-2" />
<path d="m2 22 5.5-1.5L21.17 6.83a2.82 2.82 0 0 0-4-4L3.5 16.5Z" />
<path d="m18 16 2-2" />
<path d="m17 11 4.3 4.3c.94.94.94 2.46 0 3.4l-2.6 2.6c-.94.94-2.46.94-3.4 0L11 17" />"###
};
#[cfg(feature = "LuPercent")]
const LU_PERCENT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="19" x2="5" y1="5" y2="19" />
<circle cx="6.5" cy="6.5" r="2.5" />
<circle cx="17.5" cy="17.5" r="2.5" />"###
};
#[cfg(feature = "LuPersonStanding")]
const LU_PERSON_STANDING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="5" r="1" />
<path d="m9 20 3-6 3 6" />
<path d="m6 8 6 2 6-2" />
<path d="M12 10v4" />"###
};
#[cfg(feature = "LuPhone")]
const LU_PHONE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuPhoneCall")]
const LU_PHONE_CALL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z" />
<path d="M14.05 2a9 9 0 0 1 8 7.94" />
<path d="M14.05 6A5 5 0 0 1 18 10" />"###
};
#[cfg(feature = "LuPhoneForwarded")]
const LU_PHONE_FORWARDED: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="18 2 22 6 18 10" />
<line x1="14" x2="22" y1="6" y2="6" />
<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z" />"###
};
#[cfg(feature = "LuPhoneIncoming")]
const LU_PHONE_INCOMING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="22" x2="16" y1="2" y2="8" />
<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z" />"###
};
#[cfg(feature = "LuPhoneMissed")]
const LU_PHONE_MISSED: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="22" x2="16" y1="2" y2="8" />
<line x1="16" x2="22" y1="2" y2="8" />
<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z" />"###
};
#[cfg(feature = "LuPhoneOff")]
const LU_PHONE_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="22" x2="2" y1="2" y2="22" />"###
};
#[cfg(feature = "LuPhoneOutgoing")]
const LU_PHONE_OUTGOING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="22 8 22 2 16 2" />
<line x1="16" x2="22" y1="8" y2="2" />
<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z" />"###
};
#[cfg(feature = "LuPi")]
const LU_PI: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="9" x2="9" y1="4" y2="20" />
<path d="M4 7c0-1.7 1.3-3 3-3h13" />
<path d="M18 20c-1.7 0-3-1.3-3-3V4" />"###
};
#[cfg(feature = "LuPiSquare")]
const LU_PI_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="M7 7h10" />
<path d="M10 7v10" />
<path d="M16 17a2 2 0 0 1-2-2V7" />"###
};
#[cfg(feature = "LuPictureInPicture")]
const LU_PICTURE_IN_PICTURE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 4.5v5H3m-1-6 6 6m13 0v-3c0-1.16-.84-2-2-2h-7m-9 9v2c0 1.05.95 2 2 2h3" />
<rect width="10" height="7" x="12" y="13.5" ry="2" />"###
};
#[cfg(feature = "LuPictureInPicture2")]
const LU_PICTURE_IN_PICTURE2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 9V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v10c0 1.1.9 2 2 2h4" />
<rect width="10" height="7" x="12" y="13" rx="2" />"###
};
#[cfg(feature = "LuPieChart")]
const LU_PIE_CHART: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuPiggyBank")]
const LU_PIGGY_BANK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M19 5c-1.5 0-2.8 1.4-3 2-3.5-1.5-11-.3-11 5 0 1.8 0 3 2 4.5V20h4v-2h3v2h4v-4c1-.5 1.7-1 2-2h2v-4h-2c0-1-.5-1.5-1-2h0V5z" />
<path d="M2 9v1c0 1.1.9 2 2 2h1" />
<path d="M16 11h0" />"###
};
#[cfg(feature = "LuPilcrow")]
const LU_PILCROW: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M13 4v16" />
<path d="M17 4v16" />
<path d="M19 4H9.5a4.5 4.5 0 0 0 0 9H13" />"###
};
#[cfg(feature = "LuPilcrowSquare")]
const LU_PILCROW_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="M12 12H9.5a2.5 2.5 0 0 1 0-5H17" />
<path d="M12 7v10" />
<path d="M16 7v10" />"###
};
#[cfg(feature = "LuPill")]
const LU_PILL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m10.5 20.5 10-10a4.95 4.95 0 1 0-7-7l-10 10a4.95 4.95 0 1 0 7 7Z" />
<path d="m8.5 8.5 7 7" />"###
};
#[cfg(feature = "LuPin")]
const LU_PIN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="12" x2="12" y1="17" y2="22" />
<path d="M5 17h14v-1.76a2 2 0 0 0-1.11-1.79l-1.78-.9A2 2 0 0 1 15 10.76V6h1a2 2 0 0 0 0-4H8a2 2 0 0 0 0 4h1v4.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24Z" />"###
};
#[cfg(feature = "LuPinOff")]
const LU_PIN_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="2" x2="22" y1="2" y2="22" />
<line x1="12" x2="12" y1="17" y2="22" />
<path d="M9 9v1.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24V17h12" />
<path d="M15 9.34V6h1a2 2 0 0 0 0-4H7.89" />"###
};
#[cfg(feature = "LuPipette")]
const LU_PIPETTE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m2 22 1-1h3l9-9" />
<path d="M3 21v-3l9-9" />
<path d="m15 6 3.4-3.4a2.1 2.1 0 1 1 3 3L18 9l.4.4a2.1 2.1 0 1 1-3 3l-3.8-3.8a2.1 2.1 0 1 1 3-3l.4.4Z" />"###
};
#[cfg(feature = "LuPizza")]
const LU_PIZZA: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M15 11h.01" />
<path d="M11 15h.01" />
<path d="M16 16h.01" />
<path d="m2 16 20 6-6-20A20 20 0 0 0 2 16" />
<path d="M5.71 17.11a17.04 17.04 0 0 1 11.4-11.4" />"###
};
#[cfg(feature = "LuPlane")]
const LU_PLANE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17.8 19.2 16 11l3.5-3.5C21 6 21.5 4 21 3c-1-.5-3 0-4.5 1.5L13 8 4.8 6.2c-.5-.1-.9.1-1.1.5l-.3.5c-.2.5-.1 1 .3 1.3L9 12l-2 3H4l-1 1 3 2 2 3 1-1v-3l3-2 3.5 5.3c.3.4.8.5 1.3.3l.5-.2c.4-.3.6-.7.5-1.2z" />"###
};
#[cfg(feature = "LuPlaneLanding")]
const LU_PLANE_LANDING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 22h20" />
<path d="M3.77 10.77 2 9l2-4.5 1.1.55c.55.28.9.84.9 1.45s.35 1.17.9 1.45L8 8.5l3-6 1.05.53a2 2 0 0 1 1.09 1.52l.72 5.4a2 2 0 0 0 1.09 1.52l4.4 2.2c.42.22.78.55 1.01.96l.6 1.03c.49.88-.06 1.98-1.06 2.1l-1.18.15c-.47.06-.95-.02-1.37-.24L4.29 11.15a2 2 0 0 1-.52-.38Z" />"###
};
#[cfg(feature = "LuPlaneTakeoff")]
const LU_PLANE_TAKEOFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 22h20" />
<path d="M6.36 17.4 4 17l-2-4 1.1-.55a2 2 0 0 1 1.8 0l.17.1a2 2 0 0 0 1.8 0L8 12 5 6l.9-.45a2 2 0 0 1 2.09.2l4.02 3a2 2 0 0 0 2.1.2l4.19-2.06a2.41 2.41 0 0 1 1.73-.17L21 7a1.4 1.4 0 0 1 .87 1.99l-.38.76c-.23.46-.6.84-1.07 1.08L7.58 17.2a2 2 0 0 1-1.22.18Z" />"###
};
#[cfg(feature = "LuPlay")]
const LU_PLAY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuPlayCircle")]
const LU_PLAY_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuPlaySquare")]
const LU_PLAY_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="m9 8 6 4-6 4Z" />"###
};
#[cfg(feature = "LuPlug")]
const LU_PLUG: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 22v-5" />
<path d="M9 8V2" />
<path d="M15 8V2" />
<path d="M18 8v5a4 4 0 0 1-4 4h-4a4 4 0 0 1-4-4V8Z" />"###
};
#[cfg(feature = "LuPlug2")]
const LU_PLUG2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M9 2v6" />
<path d="M15 2v6" />
<path d="M12 17v5" />
<path d="M5 8h14" />
<path d="M6 11V8h12v3a6 6 0 1 1-12 0v0Z" />"###
};
#[cfg(feature = "LuPlugZap")]
const LU_PLUG_ZAP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6.3 20.3a2.4 2.4 0 0 0 3.4 0L12 18l-6-6-2.3 2.3a2.4 2.4 0 0 0 0 3.4Z" />
<path d="m2 22 3-3" />
<path d="M7.5 13.5 10 11" />
<path d="M10.5 16.5 13 14" />
<path d="m18 3-4 4h6l-4 4" />"###
};
#[cfg(feature = "LuPlugZap2")]
const LU_PLUG_ZAP2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m13 2-2 2.5h3L12 7" />
<path d="M10 14v-3" />
<path d="M14 14v-3" />
<path d="M11 19c-1.7 0-3-1.3-3-3v-2h8v2c0 1.7-1.3 3-3 3Z" />
<path d="M12 22v-3" />"###
};
#[cfg(feature = "LuPlus")]
const LU_PLUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 12h14" />
<path d="M12 5v14" />"###
};
#[cfg(feature = "LuPlusCircle")]
const LU_PLUS_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M8 12h8" />
<path d="M12 8v8" />"###
};
#[cfg(feature = "LuPlusSquare")]
const LU_PLUS_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="M8 12h8" />
<path d="M12 8v8" />"###
};
#[cfg(feature = "LuPocket")]
const LU_POCKET: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuPocketKnife")]
const LU_POCKET_KNIFE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 2v1c0 1 2 1 2 2S3 6 3 7s2 1 2 2-2 1-2 2 2 1 2 2" />
<path d="M18 6h.01" />
<path d="M6 18h.01" />
<path d="M20.83 8.83a4 4 0 0 0-5.66-5.66l-12 12a4 4 0 1 0 5.66 5.66Z" />
<path d="M18 11.66V22a4 4 0 0 0 4-4V6" />"###
};
#[cfg(feature = "LuPodcast")]
const LU_PODCAST: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="11" r="1" />
<path d="M11 17a1 1 0 0 1 2 0c0 .5-.34 3-.5 4.5a.5.5 0 0 1-1 0c-.16-1.5-.5-4-.5-4.5Z" />
<path d="M8 14a5 5 0 1 1 8 0" />
<path d="M17 18.5a9 9 0 1 0-10 0" />"###
};
#[cfg(feature = "LuPointer")]
const LU_POINTER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 14a8 8 0 0 1-8 8" />
<path d="M18 11v-1a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v0" />
<path d="M14 10V9a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v1" />
<path d="M10 9.5V4a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v10" />
<path d="M18 11a2 2 0 1 1 4 0v3a8 8 0 0 1-8 8h-2c-2.8 0-4.5-.86-5.99-2.34l-3.6-3.6a2 2 0 0 1 2.83-2.82L7 15" />"###
};
#[cfg(feature = "LuPopcorn")]
const LU_POPCORN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18 8a2 2 0 0 0 0-4 2 2 0 0 0-4 0 2 2 0 0 0-4 0 2 2 0 0 0-4 0 2 2 0 0 0 0 4" />
<path d="M10 22 9 8" />
<path d="m14 22 1-14" />
<path d="M20 8c.5 0 .9.4.8 1l-2.6 12c-.1.5-.7 1-1.2 1H7c-.6 0-1.1-.4-1.2-1L3.2 9c-.1-.6.3-1 .8-1Z" />"###
};
#[cfg(feature = "LuPopsicle")]
const LU_POPSICLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18.6 14.4c.8-.8.8-2 0-2.8l-8.1-8.1a4.95 4.95 0 1 0-7.1 7.1l8.1 8.1c.9.7 2.1.7 2.9-.1Z" />
<path d="m22 22-5.5-5.5" />"###
};
#[cfg(feature = "LuPoundSterling")]
const LU_POUND_STERLING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18 7c0-5.333-8-5.333-8 0" />
<path d="M10 7v14" />
<path d="M6 21h12" />
<path d="M6 13h10" />"###
};
#[cfg(feature = "LuPower")]
const LU_POWER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="12" x2="12" y1="2" y2="12" />"###
};
#[cfg(feature = "LuPowerOff")]
const LU_POWER_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18.36 6.64A9 9 0 0 1 20.77 15" />
<path d="M6.16 6.16a9 9 0 1 0 12.68 12.68" />
<path d="M12 2v4" />
<path d="m2 2 20 20" />"###
};
#[cfg(feature = "LuPresentation")]
const LU_PRESENTATION: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 3h20" />
<path d="M21 3v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V3" />
<path d="m7 21 5-5 5 5" />"###
};
#[cfg(feature = "LuPrinter")]
const LU_PRINTER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<rect width="12" height="8" x="6" y="14" />"###
};
#[cfg(feature = "LuProjector")]
const LU_PROJECTOR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 7 3 5" />
<path d="M9 6V3" />
<path d="m13 7 2-2" />
<circle cx="9" cy="13" r="3" />
<path d="M11.83 12H20a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-4a2 2 0 0 1 2-2h2.17" />
<path d="M16 16h2" />"###
};
#[cfg(feature = "LuPuzzle")]
const LU_PUZZLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M19.439 7.85c-.049.322.059.648.289.878l1.568 1.568c.47.47.706 1.087.706 1.704s-.235 1.233-.706 1.704l-1.611 1.611a.98.98 0 0 1-.837.276c-.47-.07-.802-.48-.968-.925a2.501 2.501 0 1 0-3.214 3.214c.446.166.855.497.925.968a.979.979 0 0 1-.276.837l-1.61 1.61a2.404 2.404 0 0 1-1.705.707 2.402 2.402 0 0 1-1.704-.706l-1.568-1.568a1.026 1.026 0 0 0-.877-.29c-.493.074-.84.504-1.02.968a2.5 2.5 0 1 1-3.237-3.237c.464-.18.894-.527.967-1.02a1.026 1.026 0 0 0-.289-.877l-1.568-1.568A2.402 2.402 0 0 1 1.998 12c0-.617.236-1.234.706-1.704L4.23 8.77c.24-.24.581-.353.917-.303.515.077.877.528 1.073 1.01a2.5 2.5 0 1 0 3.259-3.259c-.482-.196-.933-.558-1.01-1.073-.05-.336.062-.676.303-.917l1.525-1.525A2.402 2.402 0 0 1 12 1.998c.617 0 1.234.236 1.704.706l1.568 1.568c.23.23.556.338.877.29.493-.074.84-.504 1.02-.968a2.5 2.5 0 1 1 3.237 3.237c-.464.18-.894.527-.967 1.02Z" />"###
};
#[cfg(feature = "LuQrCode")]
const LU_QR_CODE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="5" height="5" x="3" y="3" rx="1" />
<rect width="5" height="5" x="16" y="3" rx="1" />
<rect width="5" height="5" x="3" y="16" rx="1" />
<path d="M21 16h-3a2 2 0 0 0-2 2v3" />
<path d="M21 21v.01" />
<path d="M12 7v3a2 2 0 0 1-2 2H7" />
<path d="M3 12h.01" />
<path d="M12 3h.01" />
<path d="M12 16v.01" />
<path d="M16 12h1" />
<path d="M21 12v.01" />
<path d="M12 21v-1" />"###
};
#[cfg(feature = "LuQuote")]
const LU_QUOTE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 21c3 0 7-1 7-8V5c0-1.25-.756-2.017-2-2H4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2 1 0 1 0 1 1v1c0 1-1 2-2 2s-1 .008-1 1.031V20c0 1 0 1 1 1z" />
<path d="M15 21c3 0 7-1 7-8V5c0-1.25-.757-2.017-2-2h-4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2h.75c0 2.25.25 4-2.75 4v3c0 1 0 1 1 1z" />"###
};
#[cfg(feature = "LuRabbit")]
const LU_RABBIT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M20 8.54V4a2 2 0 1 0-4 0v3" />
<path d="M18 21h-8a4 4 0 0 1-4-4 7 7 0 0 1 7-7h.2L9.6 6.4a1.93 1.93 0 1 1 2.8-2.8L15.8 7h.2c3.3 0 6 2.7 6 6v1a2 2 0 0 1-2 2h-1c-1.7 0-3 1.3-3 3" />
<path d="M7.61 12.53a3 3 0 1 0-1.6 4.3" />
<path d="M13 16a3 3 0 0 1 2.24 5" />
<path d="M18 12h.01" />"###
};
#[cfg(feature = "LuRadar")]
const LU_RADAR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M19.07 4.93A10 10 0 0 0 6.99 3.34" />
<path d="M4 6h.01" />
<path d="M2.29 9.62A10 10 0 1 0 21.31 8.35" />
<path d="M16.24 7.76A6 6 0 1 0 8.23 16.67" />
<path d="M12 18h.01" />
<path d="M17.99 11.66A6 6 0 0 1 15.77 16.67" />
<circle cx="12" cy="12" r="2" />
<path d="m13.41 10.59 5.66-5.66" />"###
};
#[cfg(feature = "LuRadiation")]
const LU_RADIATION: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 12h0" />
<path d="M7.5 4.2c-.3-.5-.9-.7-1.3-.4C3.9 5.5 2.3 8.1 2 11c-.1.5.4 1 1 1h5c0-1.5.8-2.8 2-3.4-1.1-1.9-2-3.5-2.5-4.4z" />
<path d="M21 12c.6 0 1-.4 1-1-.3-2.9-1.8-5.5-4.1-7.1-.4-.3-1.1-.2-1.3.3-.6.9-1.5 2.5-2.6 4.3 1.2.7 2 2 2 3.5h5z" />
<path d="M7.5 19.8c-.3.5-.1 1.1.4 1.3 2.6 1.2 5.6 1.2 8.2 0 .5-.2.7-.8.4-1.3-.5-.9-1.4-2.5-2.5-4.3-1.2.7-2.8.7-4 0-1.1 1.8-2 3.4-2.5 4.3z" />"###
};
#[cfg(feature = "LuRadio")]
const LU_RADIO: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4.9 19.1C1 15.2 1 8.8 4.9 4.9" />
<path d="M7.8 16.2c-2.3-2.3-2.3-6.1 0-8.5" />
<circle cx="12" cy="12" r="2" />
<path d="M16.2 7.8c2.3 2.3 2.3 6.1 0 8.5" />
<path d="M19.1 4.9C23 8.8 23 15.1 19.1 19" />"###
};
#[cfg(feature = "LuRadioReceiver")]
const LU_RADIO_RECEIVER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 16v2" />
<path d="M19 16v2" />
<rect width="20" height="8" x="2" y="8" rx="2" />
<path d="M18 12h0" />"###
};
#[cfg(feature = "LuRadioTower")]
const LU_RADIO_TOWER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4.9 16.1C1 12.2 1 5.8 4.9 1.9" />
<path d="M7.8 4.7a6.14 6.14 0 0 0-.8 7.5" />
<circle cx="12" cy="9" r="2" />
<path d="M16.2 4.8c2 2 2.26 5.11.8 7.47" />
<path d="M19.1 1.9a9.96 9.96 0 0 1 0 14.1" />
<path d="M9.5 18h5" />
<path d="m8 22 4-11 4 11" />"###
};
#[cfg(feature = "LuRailSymbol")]
const LU_RAIL_SYMBOL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 15h14" />
<path d="M5 9h14" />
<path d="m14 20-5-5 6-6-5-5" />"###
};
#[cfg(feature = "LuRainbow")]
const LU_RAINBOW: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 17a10 10 0 0 0-20 0" />
<path d="M6 17a6 6 0 0 1 12 0" />
<path d="M10 17a2 2 0 0 1 4 0" />"###
};
#[cfg(feature = "LuRat")]
const LU_RAT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17 5c0-1.7-1.3-3-3-3s-3 1.3-3 3c0 .8.3 1.5.8 2H11c-3.9 0-7 3.1-7 7v0c0 2.2 1.8 4 4 4" />
<path d="M16.8 3.9c.3-.3.6-.5 1-.7 1.5-.6 3.3.1 3.9 1.6.6 1.5-.1 3.3-1.6 3.9l1.6 2.8c.2.3.2.7.2 1-.2.8-.9 1.2-1.7 1.1 0 0-1.6-.3-2.7-.6H17c-1.7 0-3 1.3-3 3" />
<path d="M13.2 18a3 3 0 0 0-2.2-5" />
<path d="M13 22H4a2 2 0 0 1 0-4h12" />
<path d="M16 9h.01" />"###
};
#[cfg(feature = "LuRatio")]
const LU_RATIO: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="12" height="20" x="6" y="2" rx="2" />
<rect width="20" height="12" x="2" y="6" rx="2" />"###
};
#[cfg(feature = "LuReceipt")]
const LU_RECEIPT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 2v20l2-1 2 1 2-1 2 1 2-1 2 1 2-1 2 1V2l-2 1-2-1-2 1-2-1-2 1-2-1-2 1-2-1Z" />
<path d="M16 8h-6a2 2 0 1 0 0 4h4a2 2 0 1 1 0 4H8" />
<path d="M12 17V7" />"###
};
#[cfg(feature = "LuRectangleHorizontal")]
const LU_RECTANGLE_HORIZONTAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="20" height="12" x="2" y="6" rx="2" />"###
};
#[cfg(feature = "LuRectangleVertical")]
const LU_RECTANGLE_VERTICAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="12" height="20" x="6" y="2" rx="2" />"###
};
#[cfg(feature = "LuRecycle")]
const LU_RECYCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M7 19H4.815a1.83 1.83 0 0 1-1.57-.881 1.785 1.785 0 0 1-.004-1.784L7.196 9.5" />
<path d="M11 19h8.203a1.83 1.83 0 0 0 1.556-.89 1.784 1.784 0 0 0 0-1.775l-1.226-2.12" />
<path d="m14 16-3 3 3 3" />
<path d="M8.293 13.596 7.196 9.5 3.1 10.598" />
<path d="m9.344 5.811 1.093-1.892A1.83 1.83 0 0 1 11.985 3a1.784 1.784 0 0 1 1.546.888l3.943 6.843" />
<path d="m13.378 9.633 4.096 1.098 1.097-4.096" />"###
};
#[cfg(feature = "LuRedo")]
const LU_REDO: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 7v6h-6" />
<path d="M3 17a9 9 0 0 1 9-9 9 9 0 0 1 6 2.3l3 2.7" />"###
};
#[cfg(feature = "LuRedo2")]
const LU_REDO2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m15 14 5-5-5-5" />
<path d="M20 9H9.5A5.5 5.5 0 0 0 4 14.5v0A5.5 5.5 0 0 0 9.5 20H13" />"###
};
#[cfg(feature = "LuRedoDot")]
const LU_REDO_DOT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="17" r="1" />
<path d="M21 7v6h-6" />
<path d="M3 17a9 9 0 0 1 9-9 9 9 0 0 1 6 2.3l3 2.7" />"###
};
#[cfg(feature = "LuRefreshCcw")]
const LU_REFRESH_CCW: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
<path d="M3 3v5h5" />
<path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16" />
<path d="M16 16h5v5" />"###
};
#[cfg(feature = "LuRefreshCcwDot")]
const LU_REFRESH_CCW_DOT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 2v6h6" />
<path d="M21 12A9 9 0 0 0 6 5.3L3 8" />
<path d="M21 22v-6h-6" />
<path d="M3 12a9 9 0 0 0 15 6.7l3-2.7" />
<circle cx="12" cy="12" r="1" />"###
};
#[cfg(feature = "LuRefreshCw")]
const LU_REFRESH_CW: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8" />
<path d="M21 3v5h-5" />
<path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16" />
<path d="M8 16H3v5" />"###
};
#[cfg(feature = "LuRefreshCwOff")]
const LU_REFRESH_CW_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 8L18.74 5.74A9.75 9.75 0 0 0 12 3C11 3 10.03 3.16 9.13 3.47" />
<path d="M8 16H3v5" />
<path d="M3 12C3 9.51 4 7.26 5.64 5.64" />
<path d="m3 16 2.26 2.26A9.75 9.75 0 0 0 12 21c2.49 0 4.74-1 6.36-2.64" />
<path d="M21 12c0 1-.16 1.97-.47 2.87" />
<path d="M21 3v5h-5" />
<path d="M22 22 2 2" />"###
};
#[cfg(feature = "LuRefrigerator")]
const LU_REFRIGERATOR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 6a4 4 0 0 1 4-4h6a4 4 0 0 1 4 4v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6Z" />
<path d="M5 10h14" />
<path d="M15 7v6" />"###
};
#[cfg(feature = "LuRegex")]
const LU_REGEX: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17 3v10" />
<path d="m12.67 5.5 8.66 5" />
<path d="m12.67 10.5 8.66-5" />
<path d="M9 17a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v2a2 2 0 0 0 2 2h2a2 2 0 0 0 2-2v-2z" />"###
};
#[cfg(feature = "LuRemoveFormatting")]
const LU_REMOVE_FORMATTING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 7V4h16v3" />
<path d="M5 20h6" />
<path d="M13 4 8 20" />
<path d="m15 15 5 5" />
<path d="m20 15-5 5" />"###
};
#[cfg(feature = "LuRepeat")]
const LU_REPEAT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m17 2 4 4-4 4" />
<path d="M3 11v-1a4 4 0 0 1 4-4h14" />
<path d="m7 22-4-4 4-4" />
<path d="M21 13v1a4 4 0 0 1-4 4H3" />"###
};
#[cfg(feature = "LuRepeat1")]
const LU_REPEAT1: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m17 2 4 4-4 4" />
<path d="M3 11v-1a4 4 0 0 1 4-4h14" />
<path d="m7 22-4-4 4-4" />
<path d="M21 13v1a4 4 0 0 1-4 4H3" />
<path d="M11 10h1v4" />"###
};
#[cfg(feature = "LuRepeat2")]
const LU_REPEAT2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m2 9 3-3 3 3" />
<path d="M13 18H7a2 2 0 0 1-2-2V6" />
<path d="m22 15-3 3-3-3" />
<path d="M11 6h6a2 2 0 0 1 2 2v10" />"###
};
#[cfg(feature = "LuReplace")]
const LU_REPLACE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14 4c0-1.1.9-2 2-2" />
<path d="M20 2c1.1 0 2 .9 2 2" />
<path d="M22 8c0 1.1-.9 2-2 2" />
<path d="M16 10c-1.1 0-2-.9-2-2" />
<path d="m3 7 3 3 3-3" />
<path d="M6 10V5c0-1.7 1.3-3 3-3h1" />
<rect width="8" height="8" x="2" y="14" rx="2" />"###
};
#[cfg(feature = "LuReplaceAll")]
const LU_REPLACE_ALL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14 4c0-1.1.9-2 2-2" />
<path d="M20 2c1.1 0 2 .9 2 2" />
<path d="M22 8c0 1.1-.9 2-2 2" />
<path d="M16 10c-1.1 0-2-.9-2-2" />
<path d="m3 7 3 3 3-3" />
<path d="M6 10V5c0-1.7 1.3-3 3-3h1" />
<rect width="8" height="8" x="2" y="14" rx="2" />
<path d="M14 14c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2" />
<path d="M20 14c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2" />"###
};
#[cfg(feature = "LuReply")]
const LU_REPLY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="9 17 4 12 9 7" />
<path d="M20 18v-2a4 4 0 0 0-4-4H4" />"###
};
#[cfg(feature = "LuReplyAll")]
const LU_REPLY_ALL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="7 17 2 12 7 7" />
<polyline points="12 17 7 12 12 7" />
<path d="M22 18v-2a4 4 0 0 0-4-4H7" />"###
};
#[cfg(feature = "LuRewind")]
const LU_REWIND: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuRocket")]
const LU_ROCKET: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4.5 16.5c-1.5 1.26-2 5-2 5s3.74-.5 5-2c.71-.84.7-2.13-.09-2.91a2.18 2.18 0 0 0-2.91-.09z" />
<path d="m12 15-3-3a22 22 0 0 1 2-3.95A12.88 12.88 0 0 1 22 2c0 2.72-.78 7.5-6 11a22.35 22.35 0 0 1-4 2z" />
<path d="M9 12H4s.55-3.03 2-4c1.62-1.08 5 0 5 0" />
<path d="M12 15v5s3.03-.55 4-2c1.08-1.62 0-5 0-5" />"###
};
#[cfg(feature = "LuRockingChair")]
const LU_ROCKING_CHAIR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="3.5 2 6.5 12.5 18 12.5" />
<line x1="9.5" x2="5.5" y1="12.5" y2="20" />
<line x1="15" x2="18.5" y1="12.5" y2="20" />
<path d="M2.75 18a13 13 0 0 0 18.5 0" />"###
};
#[cfg(feature = "LuRollerCoaster")]
const LU_ROLLER_COASTER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 19V5" />
<path d="M10 19V6.8" />
<path d="M14 19v-7.8" />
<path d="M18 5v4" />
<path d="M18 19v-6" />
<path d="M22 19V9" />
<path d="M2 19V9a4 4 0 0 1 4-4c2 0 4 1.33 6 4s4 4 6 4a4 4 0 1 0-3-6.65" />"###
};
#[cfg(feature = "LuRotate3d")]
const LU_ROTATE3D: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16.466 7.5C15.643 4.237 13.952 2 12 2 9.239 2 7 6.477 7 12s2.239 10 5 10c.342 0 .677-.069 1-.2" />
<path d="m15.194 13.707 3.814 1.86-1.86 3.814" />
<path d="M19 15.57c-1.804.885-4.274 1.43-7 1.43-5.523 0-10-2.239-10-5s4.477-5 10-5c4.838 0 8.873 1.718 9.8 4" />"###
};
#[cfg(feature = "LuRotateCcw")]
const LU_ROTATE_CCW: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
<path d="M3 3v5h5" />"###
};
#[cfg(feature = "LuRotateCw")]
const LU_ROTATE_CW: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8" />
<path d="M21 3v5h-5" />"###
};
#[cfg(feature = "LuRouter")]
const LU_ROUTER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="20" height="8" x="2" y="14" rx="2" />
<path d="M6.01 18H6" />
<path d="M10.01 18H10" />
<path d="M15 10v4" />
<path d="M17.84 7.17a4 4 0 0 0-5.66 0" />
<path d="M20.66 4.34a8 8 0 0 0-11.31 0" />"###
};
#[cfg(feature = "LuRows")]
const LU_ROWS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<line x1="3" x2="21" y1="12" y2="12" />"###
};
#[cfg(feature = "LuRss")]
const LU_RSS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuRuler")]
const LU_RULER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21.3 15.3a2.4 2.4 0 0 1 0 3.4l-2.6 2.6a2.4 2.4 0 0 1-3.4 0L2.7 8.7a2.41 2.41 0 0 1 0-3.4l2.6-2.6a2.41 2.41 0 0 1 3.4 0Z" />
<path d="m14.5 12.5 2-2" />
<path d="m11.5 9.5 2-2" />
<path d="m8.5 6.5 2-2" />
<path d="m17.5 15.5 2-2" />"###
};
#[cfg(feature = "LuRussianRuble")]
const LU_RUSSIAN_RUBLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14 11c5.333 0 5.333-8 0-8" />
<path d="M6 11h8" />
<path d="M6 15h8" />
<path d="M9 21V3" />
<path d="M9 3h5" />"###
};
#[cfg(feature = "LuSailboat")]
const LU_SAILBOAT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 18H2a4 4 0 0 0 4 4h12a4 4 0 0 0 4-4Z" />
<path d="M21 14 10 2 3 14h18Z" />
<path d="M10 2v16" />"###
};
#[cfg(feature = "LuSalad")]
const LU_SALAD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M7 21h10" />
<path d="M12 21a9 9 0 0 0 9-9H3a9 9 0 0 0 9 9Z" />
<path d="M11.38 12a2.4 2.4 0 0 1-.4-4.77 2.4 2.4 0 0 1 3.2-2.77 2.4 2.4 0 0 1 3.47-.63 2.4 2.4 0 0 1 3.37 3.37 2.4 2.4 0 0 1-1.1 3.7 2.51 2.51 0 0 1 .03 1.1" />
<path d="m13 12 4-4" />
<path d="M10.9 7.25A3.99 3.99 0 0 0 4 10c0 .73.2 1.41.54 2" />"###
};
#[cfg(feature = "LuSandwich")]
const LU_SANDWICH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 11v3a1 1 0 0 0 1 1h16a1 1 0 0 0 1-1v-3" />
<path d="M12 19H4a1 1 0 0 1-1-1v-2a1 1 0 0 1 1-1h16a1 1 0 0 1 1 1v2a1 1 0 0 1-1 1h-3.83" />
<path d="m3 11 7.77-6.04a2 2 0 0 1 2.46 0L21 11H3Z" />
<path d="M12.97 19.77 7 15h12.5l-3.75 4.5a2 2 0 0 1-2.78.27Z" />"###
};
#[cfg(feature = "LuSatellite")]
const LU_SATELLITE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M13 7 9 3 5 7l4 4" />
<path d="m17 11 4 4-4 4-4-4" />
<path d="m8 12 4 4 6-6-4-4Z" />
<path d="m16 8 3-3" />
<path d="M9 21a6 6 0 0 0-6-6" />"###
};
#[cfg(feature = "LuSatelliteDish")]
const LU_SATELLITE_DISH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 10a7.31 7.31 0 0 0 10 10Z" />
<path d="m9 15 3-3" />
<path d="M17 13a6 6 0 0 0-6-6" />
<path d="M21 13A10 10 0 0 0 11 3" />"###
};
#[cfg(feature = "LuSave")]
const LU_SAVE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuSaveAll")]
const LU_SAVE_ALL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 4a2 2 0 0 1 2-2h10l4 4v10.2a2 2 0 0 1-2 1.8H8a2 2 0 0 1-2-2Z" />
<path d="M10 2v4h6" />
<path d="M18 18v-7h-8v7" />
<path d="M18 22H4a2 2 0 0 1-2-2V6" />"###
};
#[cfg(feature = "LuScale")]
const LU_SCALE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m16 16 3-8 3 8c-.87.65-1.92 1-3 1s-2.13-.35-3-1Z" />
<path d="m2 16 3-8 3 8c-.87.65-1.92 1-3 1s-2.13-.35-3-1Z" />
<path d="M7 21h10" />
<path d="M12 3v18" />
<path d="M3 7h2c2 0 5-1 7-2 2 1 5 2 7 2h2" />"###
};
#[cfg(feature = "LuScale3d")]
const LU_SCALE3D: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="19" cy="19" r="2" />
<circle cx="5" cy="5" r="2" />
<path d="M5 7v12h12" />
<path d="m5 19 6-6" />"###
};
#[cfg(feature = "LuScaling")]
const LU_SCALING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 3 9 15" />
<path d="M12 3H3v18h18v-9" />
<path d="M16 3h5v5" />
<path d="M14 15H9v-5" />"###
};
#[cfg(feature = "LuScan")]
const LU_SCAN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 7V5a2 2 0 0 1 2-2h2" />
<path d="M17 3h2a2 2 0 0 1 2 2v2" />
<path d="M21 17v2a2 2 0 0 1-2 2h-2" />
<path d="M7 21H5a2 2 0 0 1-2-2v-2" />"###
};
#[cfg(feature = "LuScanFace")]
const LU_SCAN_FACE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 7V5a2 2 0 0 1 2-2h2" />
<path d="M17 3h2a2 2 0 0 1 2 2v2" />
<path d="M21 17v2a2 2 0 0 1-2 2h-2" />
<path d="M7 21H5a2 2 0 0 1-2-2v-2" />
<path d="M8 14s1.5 2 4 2 4-2 4-2" />
<path d="M9 9h.01" />
<path d="M15 9h.01" />"###
};
#[cfg(feature = "LuScanLine")]
const LU_SCAN_LINE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 7V5a2 2 0 0 1 2-2h2" />
<path d="M17 3h2a2 2 0 0 1 2 2v2" />
<path d="M21 17v2a2 2 0 0 1-2 2h-2" />
<path d="M7 21H5a2 2 0 0 1-2-2v-2" />
<line x1="7" x2="17" y1="12" y2="12" />"###
};
#[cfg(feature = "LuScatterChart")]
const LU_SCATTER_CHART: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="7.5" cy="7.5" r=".5" />
<circle cx="18.5" cy="5.5" r=".5" />
<circle cx="11.5" cy="11.5" r=".5" />
<circle cx="7.5" cy="16.5" r=".5" />
<circle cx="17.5" cy="14.5" r=".5" />
<path d="M3 3v18h18" />"###
};
#[cfg(feature = "LuSchool")]
const LU_SCHOOL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m4 6 8-4 8 4" />
<path d="m18 10 4 2v8a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-8l4-2" />
<path d="M14 22v-4a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v4" />
<path d="M18 5v17" />
<path d="M6 5v17" />
<circle cx="12" cy="9" r="2" />"###
};
#[cfg(feature = "LuSchool2")]
const LU_SCHOOL2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="10" r="1" />
<path d="M22 20V8h-4l-6-4-6 4H2v12a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2Z" />
<path d="M6 17v.01" />
<path d="M6 13v.01" />
<path d="M18 17v.01" />
<path d="M18 13v.01" />
<path d="M14 22v-5a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v5" />"###
};
#[cfg(feature = "LuScissors")]
const LU_SCISSORS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M8.12 8.12 12 12" />
<path d="M20 4 8.12 15.88" />
<circle cx="6" cy="18" r="3" />
<path d="M14.8 14.8 20 20" />"###
};
#[cfg(feature = "LuScissorsLineDashed")]
const LU_SCISSORS_LINE_DASHED: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5.42 9.42 8 12" />
<circle cx="4" cy="8" r="2" />
<path d="m14 6-8.58 8.58" />
<circle cx="4" cy="16" r="2" />
<path d="M10.8 14.8 14 18" />
<path d="M16 12h-2" />
<path d="M22 12h-2" />"###
};
#[cfg(feature = "LuScissorsSquare")]
const LU_SCISSORS_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="20" height="20" x="2" y="2" rx="2" />
<circle cx="8" cy="8" r="2" />
<path d="M9.414 9.414 12 12" />
<path d="M14.8 14.8 18 18" />
<circle cx="8" cy="16" r="2" />
<path d="m18 6-8.586 8.586" />"###
};
#[cfg(feature = "LuScissorsSquareDashedBottom")]
const LU_SCISSORS_SQUARE_DASHED_BOTTOM: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v16a2 2 0 0 1-2 2" />
<path d="M10 22H8" />
<path d="M16 22h-2" />
<circle cx="8" cy="8" r="2" />
<path d="M9.414 9.414 12 12" />
<path d="M14.8 14.8 18 18" />
<circle cx="8" cy="16" r="2" />
<path d="m18 6-8.586 8.586" />"###
};
#[cfg(feature = "LuScreenShare")]
const LU_SCREEN_SHARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M13 3H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-3" />
<path d="M8 21h8" />
<path d="M12 17v4" />
<path d="m17 8 5-5" />
<path d="M17 3h5v5" />"###
};
#[cfg(feature = "LuScreenShareOff")]
const LU_SCREEN_SHARE_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M13 3H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-3" />
<path d="M8 21h8" />
<path d="M12 17v4" />
<path d="m22 3-5 5" />
<path d="m17 3 5 5" />"###
};
#[cfg(feature = "LuScroll")]
const LU_SCROLL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 21h12a2 2 0 0 0 2-2v-2H10v2a2 2 0 1 1-4 0V5a2 2 0 1 0-4 0v3h4" />
<path d="M19 17V5a2 2 0 0 0-2-2H4" />"###
};
#[cfg(feature = "LuScrollText")]
const LU_SCROLL_TEXT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 21h12a2 2 0 0 0 2-2v-2H10v2a2 2 0 1 1-4 0V5a2 2 0 1 0-4 0v3h4" />
<path d="M19 17V5a2 2 0 0 0-2-2H4" />
<path d="M15 8h-5" />
<path d="M15 12h-5" />"###
};
#[cfg(feature = "LuSearch")]
const LU_SEARCH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="m21 21-4.3-4.3" />"###
};
#[cfg(feature = "LuSearchCheck")]
const LU_SEARCH_CHECK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m8 11 2 2 4-4" />
<circle cx="11" cy="11" r="8" />
<path d="m21 21-4.3-4.3" />"###
};
#[cfg(feature = "LuSearchCode")]
const LU_SEARCH_CODE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m9 9-2 2 2 2" />
<path d="m13 13 2-2-2-2" />
<circle cx="11" cy="11" r="8" />
<path d="m21 21-4.3-4.3" />"###
};
#[cfg(feature = "LuSearchSlash")]
const LU_SEARCH_SLASH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m13.5 8.5-5 5" />
<circle cx="11" cy="11" r="8" />
<path d="m21 21-4.3-4.3" />"###
};
#[cfg(feature = "LuSearchX")]
const LU_SEARCH_X: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m13.5 8.5-5 5" />
<path d="m8.5 8.5 5 5" />
<circle cx="11" cy="11" r="8" />
<path d="m21 21-4.3-4.3" />"###
};
#[cfg(feature = "LuSend")]
const LU_SEND: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m22 2-7 20-4-9-9-4Z" />
<path d="M22 2 11 13" />"###
};
#[cfg(feature = "LuSendHorizonal")]
const LU_SEND_HORIZONAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m3 3 3 9-3 9 19-9Z" />
<path d="M6 12h16" />"###
};
#[cfg(feature = "LuSendToBack")]
const LU_SEND_TO_BACK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect x="14" y="14" width="8" height="8" rx="2" />
<rect x="2" y="2" width="8" height="8" rx="2" />
<path d="M7 14v1a2 2 0 0 0 2 2h1" />
<path d="M14 7h1a2 2 0 0 1 2 2v1" />"###
};
#[cfg(feature = "LuSeparatorHorizontal")]
const LU_SEPARATOR_HORIZONTAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="3" x2="21" y1="12" y2="12" />
<polyline points="8 8 12 4 16 8" />
<polyline points="16 16 12 20 8 16" />"###
};
#[cfg(feature = "LuSeparatorVertical")]
const LU_SEPARATOR_VERTICAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="12" x2="12" y1="3" y2="21" />
<polyline points="8 8 4 12 8 16" />
<polyline points="16 16 20 12 16 8" />"###
};
#[cfg(feature = "LuServer")]
const LU_SERVER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="20" height="8" x="2" y="2" rx="2" ry="2" />
<rect width="20" height="8" x="2" y="14" rx="2" ry="2" />
<line x1="6" x2="6.01" y1="6" y2="6" />
<line x1="6" x2="6.01" y1="18" y2="18" />"###
};
#[cfg(feature = "LuServerCog")]
const LU_SERVER_COG: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 10H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-1" />
<path d="M5 14H4a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2h-1" />
<path d="M6 6h.01" />
<path d="M6 18h.01" />
<circle cx="12" cy="12" r="3" />
<path d="M12 8v1" />
<path d="M12 15v1" />
<path d="M16 12h-1" />
<path d="M9 12H8" />
<path d="m15 9-.88.88" />
<path d="M9.88 14.12 9 15" />
<path d="m15 15-.88-.88" />
<path d="M9.88 9.88 9 9" />"###
};
#[cfg(feature = "LuServerCrash")]
const LU_SERVER_CRASH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 10H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-2" />
<path d="M6 14H4a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2h-2" />
<path d="M6 6h.01" />
<path d="M6 18h.01" />
<path d="m13 6-4 6h6l-4 6" />"###
};
#[cfg(feature = "LuServerOff")]
const LU_SERVER_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M7 2h13a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-5" />
<path d="M10 10 2.5 2.5C2 2 2 2.5 2 5v3a2 2 0 0 0 2 2h6z" />
<path d="M22 17v-1a2 2 0 0 0-2-2h-1" />
<path d="M4 14a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h16.5l1-.5.5.5-8-8H4z" />
<path d="M6 18h.01" />
<path d="m2 2 20 20" />"###
};
#[cfg(feature = "LuSettings")]
const LU_SETTINGS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" />
<circle cx="12" cy="12" r="3" />"###
};
#[cfg(feature = "LuSettings2")]
const LU_SETTINGS2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M20 7h-9" />
<path d="M14 17H5" />
<circle cx="17" cy="17" r="3" />
<circle cx="7" cy="7" r="3" />"###
};
#[cfg(feature = "LuShapes")]
const LU_SHAPES: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8.3 10a.7.7 0 0 1-.626-1.079L11.4 3a.7.7 0 0 1 1.198-.043L16.3 8.9a.7.7 0 0 1-.572 1.1Z" />
<rect x="3" y="14" width="7" height="7" rx="1" />
<circle cx="17.5" cy="17.5" r="3.5" />"###
};
#[cfg(feature = "LuShare")]
const LU_SHARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="12" x2="12" y1="2" y2="15" />"###
};
#[cfg(feature = "LuShare2")]
const LU_SHARE2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="8.59" x2="15.42" y1="13.51" y2="17.49" />
<line x1="15.41" x2="8.59" y1="6.51" y2="10.49" />"###
};
#[cfg(feature = "LuSheet")]
const LU_SHEET: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<line x1="3" x2="21" y1="9" y2="9" />
<line x1="3" x2="21" y1="15" y2="15" />
<line x1="9" x2="9" y1="9" y2="21" />
<line x1="15" x2="15" y1="9" y2="21" />"###
};
#[cfg(feature = "LuShell")]
const LU_SHELL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14 11a2 2 0 1 1-4 0 4 4 0 0 1 8 0 6 6 0 0 1-12 0 8 8 0 0 1 16 0 10 10 0 1 1-20 0 11.93 11.93 0 0 1 2.42-7.22 2 2 0 1 1 3.16 2.44" />"###
};
#[cfg(feature = "LuShield")]
const LU_SHIELD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuShieldAlert")]
const LU_SHIELD_ALERT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" />
<path d="M12 8v4" />
<path d="M12 16h.01" />"###
};
#[cfg(feature = "LuShieldCheck")]
const LU_SHIELD_CHECK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" />
<path d="m9 12 2 2 4-4" />"###
};
#[cfg(feature = "LuShieldClose")]
const LU_SHIELD_CLOSE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" />
<line x1="9.5" x2="14.5" y1="9" y2="14" />
<line x1="14.5" x2="9.5" y1="9" y2="14" />"###
};
#[cfg(feature = "LuShieldOff")]
const LU_SHIELD_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M4.73 4.73 4 5v7c0 6 8 10 8 10a20.29 20.29 0 0 0 5.62-4.38" />
<line x1="2" x2="22" y1="2" y2="22" />"###
};
#[cfg(feature = "LuShieldQuestion")]
const LU_SHIELD_QUESTION: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 17h.01" />
<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10" />
<path d="M9.1 9a3 3 0 0 1 5.82 1c0 2-3 3-3 3" />"###
};
#[cfg(feature = "LuShip")]
const LU_SHIP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 21c.6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1 .6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1" />
<path d="M19.38 20A11.6 11.6 0 0 0 21 14l-9-4-9 4c0 2.9.94 5.34 2.81 7.76" />
<path d="M19 13V7a2 2 0 0 0-2-2H7a2 2 0 0 0-2 2v6" />
<path d="M12 10v4" />
<path d="M12 2v3" />"###
};
#[cfg(feature = "LuShipWheel")]
const LU_SHIP_WHEEL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="8" />
<path d="M12 2v7.5" />
<path d="m19 5-5.23 5.23" />
<path d="M22 12h-7.5" />
<path d="m19 19-5.23-5.23" />
<path d="M12 14.5V22" />
<path d="M10.23 13.77 5 19" />
<path d="M9.5 12H2" />
<path d="M10.23 10.23 5 5" />
<circle cx="12" cy="12" r="2.5" />"###
};
#[cfg(feature = "LuShirt")]
const LU_SHIRT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M20.38 3.46 16 2a4 4 0 0 1-8 0L3.62 3.46a2 2 0 0 0-1.34 2.23l.58 3.47a1 1 0 0 0 .99.84H6v10c0 1.1.9 2 2 2h8a2 2 0 0 0 2-2V10h2.15a1 1 0 0 0 .99-.84l.58-3.47a2 2 0 0 0-1.34-2.23z" />"###
};
#[cfg(feature = "LuShoppingBag")]
const LU_SHOPPING_BAG: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 2 3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6l-3-4Z" />
<path d="M3 6h18" />
<path d="M16 10a4 4 0 0 1-8 0" />"###
};
#[cfg(feature = "LuShoppingBasket")]
const LU_SHOPPING_BASKET: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m5 11 4-7" />
<path d="m19 11-4-7" />
<path d="M2 11h20" />
<path d="m3.5 11 1.6 7.4a2 2 0 0 0 2 1.6h9.8c.9 0 1.8-.7 2-1.6l1.7-7.4" />
<path d="m9 11 1 9" />
<path d="M4.5 15.5h15" />
<path d="m15 11-1 9" />"###
};
#[cfg(feature = "LuShoppingCart")]
const LU_SHOPPING_CART: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="8" cy="21" r="1" />
<circle cx="19" cy="21" r="1" />
<path d="M2.05 2.05h2l2.66 12.42a2 2 0 0 0 2 1.58h9.78a2 2 0 0 0 1.95-1.57l1.65-7.43H5.12" />"###
};
#[cfg(feature = "LuShovel")]
const LU_SHOVEL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 22v-5l5-5 5 5-5 5z" />
<path d="M9.5 14.5 16 8" />
<path d="m17 2 5 5-.5.5a3.53 3.53 0 0 1-5 0s0 0 0 0a3.53 3.53 0 0 1 0-5L17 2" />"###
};
#[cfg(feature = "LuShowerHead")]
const LU_SHOWER_HEAD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m4 4 2.5 2.5" />
<path d="M13.5 6.5a4.95 4.95 0 0 0-7 7" />
<path d="M15 5 5 15" />
<path d="M14 17v.01" />
<path d="M10 16v.01" />
<path d="M13 13v.01" />
<path d="M16 10v.01" />
<path d="M11 20v.01" />
<path d="M17 14v.01" />
<path d="M20 11v.01" />"###
};
#[cfg(feature = "LuShrink")]
const LU_SHRINK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m15 15 6 6m-6-6v4.8m0-4.8h4.8" />
<path d="M9 19.8V15m0 0H4.2M9 15l-6 6" />
<path d="M15 4.2V9m0 0h4.8M15 9l6-6" />
<path d="M9 4.2V9m0 0H4.2M9 9 3 3" />"###
};
#[cfg(feature = "LuShrub")]
const LU_SHRUB: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 22v-7l-2-2" />
<path d="M17 8v.8A6 6 0 0 1 13.8 20v0H10v0A6.5 6.5 0 0 1 7 8h0a5 5 0 0 1 10 0Z" />
<path d="m14 14-2 2" />"###
};
#[cfg(feature = "LuShuffle")]
const LU_SHUFFLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 18h1.4c1.3 0 2.5-.6 3.3-1.7l6.1-8.6c.7-1.1 2-1.7 3.3-1.7H22" />
<path d="m18 2 4 4-4 4" />
<path d="M2 6h1.9c1.5 0 2.9.9 3.6 2.2" />
<path d="M22 18h-5.9c-1.3 0-2.6-.7-3.3-1.8l-.5-.8" />
<path d="m18 14 4 4-4 4" />"###
};
#[cfg(feature = "LuSigma")]
const LU_SIGMA: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18 7V4H6l6 8-6 8h12v-3" />"###
};
#[cfg(feature = "LuSigmaSquare")]
const LU_SIGMA_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="M16 8.9V7H8l4 5-4 5h8v-1.9" />"###
};
#[cfg(feature = "LuSignal")]
const LU_SIGNAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 20h.01" />
<path d="M7 20v-4" />
<path d="M12 20v-8" />
<path d="M17 20V8" />
<path d="M22 4v16" />"###
};
#[cfg(feature = "LuSignalHigh")]
const LU_SIGNAL_HIGH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 20h.01" />
<path d="M7 20v-4" />
<path d="M12 20v-8" />
<path d="M17 20V8" />"###
};
#[cfg(feature = "LuSignalLow")]
const LU_SIGNAL_LOW: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 20h.01" />
<path d="M7 20v-4" />"###
};
#[cfg(feature = "LuSignalMedium")]
const LU_SIGNAL_MEDIUM: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 20h.01" />
<path d="M7 20v-4" />
<path d="M12 20v-8" />"###
};
#[cfg(feature = "LuSignalZero")]
const LU_SIGNAL_ZERO: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 20h.01" />"###
};
#[cfg(feature = "LuSiren")]
const LU_SIREN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M7 12a5 5 0 0 1 5-5v0a5 5 0 0 1 5 5v6H7v-6Z" />
<path d="M5 20a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v2H5v-2Z" />
<path d="M21 12h1" />
<path d="M18.5 4.5 18 5" />
<path d="M2 12h1" />
<path d="M12 2v1" />
<path d="m4.929 4.929.707.707" />
<path d="M12 12v6" />"###
};
#[cfg(feature = "LuSkipBack")]
const LU_SKIP_BACK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="5" x2="5" y1="19" y2="5" />"###
};
#[cfg(feature = "LuSkipForward")]
const LU_SKIP_FORWARD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="19" x2="19" y1="5" y2="19" />"###
};
#[cfg(feature = "LuSkull")]
const LU_SKULL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="9" cy="12" r="1" />
<circle cx="15" cy="12" r="1" />
<path d="M8 20v2h8v-2" />
<path d="m12.5 17-.5-1-.5 1h1z" />
<path d="M16 20a2 2 0 0 0 1.56-3.25 8 8 0 1 0-11.12 0A2 2 0 0 0 8 20" />"###
};
#[cfg(feature = "LuSlack")]
const LU_SLACK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="3" height="8" x="13" y="2" rx="1.5" />
<path d="M19 8.5V10h1.5A1.5 1.5 0 1 0 19 8.5" />
<rect width="3" height="8" x="8" y="14" rx="1.5" />
<path d="M5 15.5V14H3.5A1.5 1.5 0 1 0 5 15.5" />
<rect width="8" height="3" x="14" y="13" rx="1.5" />
<path d="M15.5 19H14v1.5a1.5 1.5 0 1 0 1.5-1.5" />
<rect width="8" height="3" x="2" y="8" rx="1.5" />
<path d="M8.5 5H10V3.5A1.5 1.5 0 1 0 8.5 5" />"###
};
#[cfg(feature = "LuSlice")]
const LU_SLICE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m8 14-6 6h9v-3" />
<path d="M18.37 3.63 8 14l3 3L21.37 6.63a2.12 2.12 0 1 0-3-3Z" />"###
};
#[cfg(feature = "LuSliders")]
const LU_SLIDERS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="4" x2="4" y1="21" y2="14" />
<line x1="4" x2="4" y1="10" y2="3" />
<line x1="12" x2="12" y1="21" y2="12" />
<line x1="12" x2="12" y1="8" y2="3" />
<line x1="20" x2="20" y1="21" y2="16" />
<line x1="20" x2="20" y1="12" y2="3" />
<line x1="2" x2="6" y1="14" y2="14" />
<line x1="10" x2="14" y1="8" y2="8" />
<line x1="18" x2="22" y1="16" y2="16" />"###
};
#[cfg(feature = "LuSlidersHorizontal")]
const LU_SLIDERS_HORIZONTAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="21" x2="14" y1="4" y2="4" />
<line x1="10" x2="3" y1="4" y2="4" />
<line x1="21" x2="12" y1="12" y2="12" />
<line x1="8" x2="3" y1="12" y2="12" />
<line x1="21" x2="16" y1="20" y2="20" />
<line x1="12" x2="3" y1="20" y2="20" />
<line x1="14" x2="14" y1="2" y2="6" />
<line x1="8" x2="8" y1="10" y2="14" />
<line x1="16" x2="16" y1="18" y2="22" />"###
};
#[cfg(feature = "LuSmartphone")]
const LU_SMARTPHONE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="14" height="20" x="5" y="2" rx="2" ry="2" />
<path d="M12 18h.01" />"###
};
#[cfg(feature = "LuSmartphoneCharging")]
const LU_SMARTPHONE_CHARGING: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="14" height="20" x="5" y="2" rx="2" ry="2" />
<path d="M12.667 8 10 12h4l-2.667 4" />"###
};
#[cfg(feature = "LuSmartphoneNfc")]
const LU_SMARTPHONE_NFC: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="7" height="12" x="2" y="6" rx="1" />
<path d="M13 8.32a7.43 7.43 0 0 1 0 7.36" />
<path d="M16.46 6.21a11.76 11.76 0 0 1 0 11.58" />
<path d="M19.91 4.1a15.91 15.91 0 0 1 .01 15.8" />"###
};
#[cfg(feature = "LuSmile")]
const LU_SMILE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="9" x2="9.01" y1="9" y2="9" />
<line x1="15" x2="15.01" y1="9" y2="9" />"###
};
#[cfg(feature = "LuSmilePlus")]
const LU_SMILE_PLUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 11v1a10 10 0 1 1-9-10" />
<path d="M8 14s1.5 2 4 2 4-2 4-2" />
<line x1="9" x2="9.01" y1="9" y2="9" />
<line x1="15" x2="15.01" y1="9" y2="9" />
<path d="M16 5h6" />
<path d="M19 2v6" />"###
};
#[cfg(feature = "LuSnail")]
const LU_SNAIL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 13a6 6 0 1 0 12 0 4 4 0 1 0-8 0 2 2 0 0 0 4 0" />
<circle cx="10" cy="13" r="8" />
<path d="M2 21h12c4.4 0 8-3.6 8-8V7a2 2 0 1 0-4 0v6" />
<path d="M18 3 19.1 5.2" />
<path d="M22 3 20.9 5.2" />"###
};
#[cfg(feature = "LuSnowflake")]
const LU_SNOWFLAKE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="2" x2="22" y1="12" y2="12" />
<line x1="12" x2="12" y1="2" y2="22" />
<path d="m20 16-4-4 4-4" />
<path d="m4 8 4 4-4 4" />
<path d="m16 4-4 4-4-4" />
<path d="m8 20 4-4 4 4" />"###
};
#[cfg(feature = "LuSofa")]
const LU_SOFA: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M20 9V6a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v3" />
<path d="M2 11v5a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-5a2 2 0 0 0-4 0v2H6v-2a2 2 0 0 0-4 0Z" />
<path d="M4 18v2" />
<path d="M20 18v2" />
<path d="M12 4v9" />"###
};
#[cfg(feature = "LuSoup")]
const LU_SOUP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 21a9 9 0 0 0 9-9H3a9 9 0 0 0 9 9Z" />
<path d="M7 21h10" />
<path d="M19.5 12 22 6" />
<path d="M16.25 3c.27.1.8.53.75 1.36-.06.83-.93 1.2-1 2.02-.05.78.34 1.24.73 1.62" />
<path d="M11.25 3c.27.1.8.53.74 1.36-.05.83-.93 1.2-.98 2.02-.06.78.33 1.24.72 1.62" />
<path d="M6.25 3c.27.1.8.53.75 1.36-.06.83-.93 1.2-1 2.02-.05.78.34 1.24.74 1.62" />"###
};
#[cfg(feature = "LuSpace")]
const LU_SPACE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 17v1c0 .5-.5 1-1 1H3c-.5 0-1-.5-1-1v-1" />"###
};
#[cfg(feature = "LuSpade")]
const LU_SPADE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 9c-1.5 1.5-3 3.2-3 5.5A5.5 5.5 0 0 0 7.5 20c1.8 0 3-.5 4.5-2 1.5 1.5 2.7 2 4.5 2a5.5 5.5 0 0 0 5.5-5.5c0-2.3-1.5-4-3-5.5l-7-7-7 7Z" />
<path d="M12 18v4" />"###
};
#[cfg(feature = "LuSparkle")]
const LU_SPARKLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m12 3-1.9 5.8a2 2 0 0 1-1.287 1.288L3 12l5.8 1.9a2 2 0 0 1 1.288 1.287L12 21l1.9-5.8a2 2 0 0 1 1.287-1.288L21 12l-5.8-1.9a2 2 0 0 1-1.288-1.287Z" />"###
};
#[cfg(feature = "LuSparkles")]
const LU_SPARKLES: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m12 3-1.912 5.813a2 2 0 0 1-1.275 1.275L3 12l5.813 1.912a2 2 0 0 1 1.275 1.275L12 21l1.912-5.813a2 2 0 0 1 1.275-1.275L21 12l-5.813-1.912a2 2 0 0 1-1.275-1.275L12 3Z" />
<path d="M5 3v4" />
<path d="M19 17v4" />
<path d="M3 5h4" />
<path d="M17 19h4" />"###
};
#[cfg(feature = "LuSpeaker")]
const LU_SPEAKER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="16" height="20" x="4" y="2" rx="2" ry="2" />
<circle cx="12" cy="14" r="4" />
<line x1="12" x2="12.01" y1="6" y2="6" />"###
};
#[cfg(feature = "LuSpellCheck")]
const LU_SPELL_CHECK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m6 16 6-12 6 12" />
<path d="M8 12h8" />
<path d="m16 20 2 2 4-4" />"###
};
#[cfg(feature = "LuSpellCheck2")]
const LU_SPELL_CHECK2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m6 16 6-12 6 12" />
<path d="M8 12h8" />
<path d="M4 21c1.1 0 1.1-1 2.3-1s1.1 1 2.3 1c1.1 0 1.1-1 2.3-1 1.1 0 1.1 1 2.3 1 1.1 0 1.1-1 2.3-1 1.1 0 1.1 1 2.3 1 1.1 0 1.1-1 2.3-1" />"###
};
#[cfg(feature = "LuSpline")]
const LU_SPLINE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="19" cy="5" r="2" />
<circle cx="5" cy="19" r="2" />
<path d="M5 17A12 12 0 0 1 17 5" />"###
};
#[cfg(feature = "LuSplit")]
const LU_SPLIT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16 3h5v5" />
<path d="M8 3H3v5" />
<path d="M12 22v-8.3a4 4 0 0 0-1.172-2.872L3 3" />
<path d="m15 9 6-6" />"###
};
#[cfg(feature = "LuSplitSquareHorizontal")]
const LU_SPLIT_SQUARE_HORIZONTAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 19H5c-1 0-2-1-2-2V7c0-1 1-2 2-2h3" />
<path d="M16 5h3c1 0 2 1 2 2v10c0 1-1 2-2 2h-3" />
<line x1="12" x2="12" y1="4" y2="20" />"###
};
#[cfg(feature = "LuSplitSquareVertical")]
const LU_SPLIT_SQUARE_VERTICAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 8V5c0-1 1-2 2-2h10c1 0 2 1 2 2v3" />
<path d="M19 16v3c0 1-1 2-2 2H7c-1 0-2-1-2-2v-3" />
<line x1="4" x2="20" y1="12" y2="12" />"###
};
#[cfg(feature = "LuSprayCan")]
const LU_SPRAY_CAN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 3h.01" />
<path d="M7 5h.01" />
<path d="M11 7h.01" />
<path d="M3 7h.01" />
<path d="M7 9h.01" />
<path d="M3 11h.01" />
<rect width="4" height="4" x="15" y="5" />
<path d="m19 9 2 2v10c0 .6-.4 1-1 1h-6c-.6 0-1-.4-1-1V11l2-2" />
<path d="m13 14 8-2" />
<path d="m13 19 8-2" />"###
};
#[cfg(feature = "LuSprout")]
const LU_SPROUT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M7 20h10" />
<path d="M10 20c5.5-2.5.8-6.4 3-10" />
<path d="M9.5 9.4c1.1.8 1.8 2.2 2.3 3.7-2 .4-3.5.4-4.8-.3-1.2-.6-2.3-1.9-3-4.2 2.8-.5 4.4 0 5.5.8z" />
<path d="M14.1 6a7 7 0 0 0-1.1 4c1.9-.1 3.3-.6 4.3-1.4 1-1 1.6-2.3 1.7-4.6-2.7.1-4 1-4.9 2z" />"###
};
#[cfg(feature = "LuSquare")]
const LU_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />"###
};
#[cfg(feature = "LuSquareAsterisk")]
const LU_SQUARE_ASTERISK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="M12 8v8" />
<path d="m8.5 14 7-4" />
<path d="m8.5 10 7 4" />"###
};
#[cfg(feature = "LuSquareCode")]
const LU_SQUARE_CODE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="m10 10-2 2 2 2" />
<path d="m14 14 2-2-2-2" />"###
};
#[cfg(feature = "LuSquareDashedBottom")]
const LU_SQUARE_DASHED_BOTTOM: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 21a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2" />
<path d="M9 21h1" />
<path d="M14 21h1" />"###
};
#[cfg(feature = "LuSquareDashedBottomCode")]
const LU_SQUARE_DASHED_BOTTOM_CODE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m10 10-2 2 2 2" />
<path d="m14 14 2-2-2-2" />
<path d="M5 21a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2" />
<path d="M9 21h1" />
<path d="M14 21h1" />"###
};
#[cfg(feature = "LuSquareDot")]
const LU_SQUARE_DOT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<circle cx="12" cy="12" r="1" />"###
};
#[cfg(feature = "LuSquareEqual")]
const LU_SQUARE_EQUAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="M7 10h10" />
<path d="M7 14h10" />"###
};
#[cfg(feature = "LuSquareSlash")]
const LU_SQUARE_SLASH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<line x1="9" x2="15" y1="15" y2="9" />"###
};
#[cfg(feature = "LuSquareStack")]
const LU_SQUARE_STACK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 10c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h4c1.1 0 2 .9 2 2" />
<path d="M10 16c-1.1 0-2-.9-2-2v-4c0-1.1.9-2 2-2h4c1.1 0 2 .9 2 2" />
<rect width="8" height="8" x="14" y="14" rx="2" />"###
};
#[cfg(feature = "LuSquirrel")]
const LU_SQUIRREL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18 6a4 4 0 0 0-4 4 7 7 0 0 0-7 7c0-5 4-5 4-10.5a4.5 4.5 0 1 0-9 0 2.5 2.5 0 0 0 5 0C7 10 3 11 3 17c0 2.8 2.2 5 5 5h10" />
<path d="M16 20c0-1.7 1.3-3 3-3h1a2 2 0 0 0 2-2v-2a4 4 0 0 0-4-4V4" />
<path d="M15.2 22a3 3 0 0 0-2.2-5" />
<path d="M18 13h.01" />"###
};
#[cfg(feature = "LuStamp")]
const LU_STAMP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 22h14" />
<path d="M19.27 13.73A2.5 2.5 0 0 0 17.5 13h-11A2.5 2.5 0 0 0 4 15.5V17a1 1 0 0 0 1 1h14a1 1 0 0 0 1-1v-1.5c0-.66-.26-1.3-.73-1.77Z" />
<path d="M14 13V8.5C14 7 15 7 15 5a3 3 0 0 0-3-3c-1.66 0-3 1-3 3s1 2 1 3.5V13" />"###
};
#[cfg(feature = "LuStar")]
const LU_STAR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuStarHalf")]
const LU_STAR_HALF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 17.8 5.8 21 7 14.1 2 9.3l7-1L12 2" />"###
};
#[cfg(feature = "LuStarOff")]
const LU_STAR_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8.34 8.34 2 9.27l5 4.87L5.82 21 12 17.77 18.18 21l-.59-3.43" />
<path d="M18.42 12.76 22 9.27l-6.91-1L12 2l-1.44 2.91" />
<line x1="2" x2="22" y1="2" y2="22" />"###
};
#[cfg(feature = "LuStepBack")]
const LU_STEP_BACK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="18" x2="18" y1="20" y2="4" />
<polygon points="14,20 4,12 14,4" />"###
};
#[cfg(feature = "LuStepForward")]
const LU_STEP_FORWARD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="6" x2="6" y1="4" y2="20" />
<polygon points="10,4 20,12 10,20" />"###
};
#[cfg(feature = "LuStethoscope")]
const LU_STETHOSCOPE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4.8 2.3A.3.3 0 1 0 5 2H4a2 2 0 0 0-2 2v5a6 6 0 0 0 6 6v0a6 6 0 0 0 6-6V4a2 2 0 0 0-2-2h-1a.2.2 0 1 0 .3.3" />
<path d="M8 15v1a6 6 0 0 0 6 6v0a6 6 0 0 0 6-6v-4" />
<circle cx="20" cy="10" r="2" />"###
};
#[cfg(feature = "LuSticker")]
const LU_STICKER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M15.5 3H5a2 2 0 0 0-2 2v14c0 1.1.9 2 2 2h14a2 2 0 0 0 2-2V8.5L15.5 3Z" />
<path d="M15 3v6h6" />
<path d="M10 16s.8 1 2 1c1.3 0 2-1 2-1" />
<path d="M8 13h0" />
<path d="M16 13h0" />"###
};
#[cfg(feature = "LuStickyNote")]
const LU_STICKY_NOTE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M15.5 3H5a2 2 0 0 0-2 2v14c0 1.1.9 2 2 2h14a2 2 0 0 0 2-2V8.5L15.5 3Z" />
<path d="M15 3v6h6" />"###
};
#[cfg(feature = "LuStopCircle")]
const LU_STOP_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<rect width="6" height="6" x="9" y="9" />"###
};
#[cfg(feature = "LuStore")]
const LU_STORE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m2 7 4.41-4.41A2 2 0 0 1 7.83 2h8.34a2 2 0 0 1 1.42.59L22 7" />
<path d="M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8" />
<path d="M15 22v-4a2 2 0 0 0-2-2h-2a2 2 0 0 0-2 2v4" />
<path d="M2 7h20" />
<path d="M22 7v3a2 2 0 0 1-2 2v0a2.7 2.7 0 0 1-1.59-.63.7.7 0 0 0-.82 0A2.7 2.7 0 0 1 16 12a2.7 2.7 0 0 1-1.59-.63.7.7 0 0 0-.82 0A2.7 2.7 0 0 1 12 12a2.7 2.7 0 0 1-1.59-.63.7.7 0 0 0-.82 0A2.7 2.7 0 0 1 8 12a2.7 2.7 0 0 1-1.59-.63.7.7 0 0 0-.82 0A2.7 2.7 0 0 1 4 12v0a2 2 0 0 1-2-2V7" />"###
};
#[cfg(feature = "LuStretchHorizontal")]
const LU_STRETCH_HORIZONTAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="20" height="6" x="2" y="4" rx="2" />
<rect width="20" height="6" x="2" y="14" rx="2" />"###
};
#[cfg(feature = "LuStretchVertical")]
const LU_STRETCH_VERTICAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="6" height="20" x="4" y="2" rx="2" />
<rect width="6" height="20" x="14" y="2" rx="2" />"###
};
#[cfg(feature = "LuStrikethrough")]
const LU_STRIKETHROUGH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16 4H9a3 3 0 0 0-2.83 4" />
<path d="M14 12a4 4 0 0 1 0 8H6" />
<line x1="4" x2="20" y1="12" y2="12" />"###
};
#[cfg(feature = "LuSubscript")]
const LU_SUBSCRIPT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m4 5 8 8" />
<path d="m12 5-8 8" />
<path d="M20 19h-4c0-1.5.44-2 1.5-2.5S20 15.33 20 14c0-.47-.17-.93-.48-1.29a2.11 2.11 0 0 0-2.62-.44c-.42.24-.74.62-.9 1.07" />"###
};
#[cfg(feature = "LuSubtitles")]
const LU_SUBTITLES: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M7 13h4" />
<path d="M15 13h2" />
<path d="M7 9h2" />
<path d="M13 9h4" />
<path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v10Z" />"###
};
#[cfg(feature = "LuSun")]
const LU_SUN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M12 2v2" />
<path d="M12 20v2" />
<path d="m4.93 4.93 1.41 1.41" />
<path d="m17.66 17.66 1.41 1.41" />
<path d="M2 12h2" />
<path d="M20 12h2" />
<path d="m6.34 17.66-1.41 1.41" />
<path d="m19.07 4.93-1.41 1.41" />"###
};
#[cfg(feature = "LuSunDim")]
const LU_SUN_DIM: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M12 4h.01" />
<path d="M20 12h.01" />
<path d="M12 20h.01" />
<path d="M4 12h.01" />
<path d="M17.657 6.343h.01" />
<path d="M17.657 17.657h.01" />
<path d="M6.343 17.657h.01" />
<path d="M6.343 6.343h.01" />"###
};
#[cfg(feature = "LuSunMedium")]
const LU_SUN_MEDIUM: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M12 3v1" />
<path d="M12 20v1" />
<path d="M3 12h1" />
<path d="M20 12h1" />
<path d="m18.364 5.636-.707.707" />
<path d="m6.343 17.657-.707.707" />
<path d="m5.636 5.636.707.707" />
<path d="m17.657 17.657.707.707" />"###
};
#[cfg(feature = "LuSunMoon")]
const LU_SUN_MOON: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M12 8a2 2 0 1 0 4 4" />
<path d="M12 2v2" />
<path d="M12 20v2" />
<path d="m4.93 4.93 1.41 1.41" />
<path d="m17.66 17.66 1.41 1.41" />
<path d="M2 12h2" />
<path d="M20 12h2" />
<path d="m6.34 17.66-1.41 1.41" />
<path d="m19.07 4.93-1.41 1.41" />"###
};
#[cfg(feature = "LuSunSnow")]
const LU_SUN_SNOW: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10 9a3 3 0 1 0 0 6" />
<path d="M2 12h1" />
<path d="M14 21V3" />
<path d="M10 4V3" />
<path d="M10 21v-1" />
<path d="m3.64 18.36.7-.7" />
<path d="m4.34 6.34-.7-.7" />
<path d="M14 12h8" />
<path d="m17 4-3 3" />
<path d="m14 17 3 3" />
<path d="m21 15-3-3 3-3" />"###
};
#[cfg(feature = "LuSunrise")]
const LU_SUNRISE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 2v8" />
<path d="m4.93 10.93 1.41 1.41" />
<path d="M2 18h2" />
<path d="M20 18h2" />
<path d="m19.07 10.93-1.41 1.41" />
<path d="M22 22H2" />
<path d="m8 6 4-4 4 4" />
<path d="M16 18a4 4 0 0 0-8 0" />"###
};
#[cfg(feature = "LuSunset")]
const LU_SUNSET: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 10V2" />
<path d="m4.93 10.93 1.41 1.41" />
<path d="M2 18h2" />
<path d="M20 18h2" />
<path d="m19.07 10.93-1.41 1.41" />
<path d="M22 22H2" />
<path d="m16 6-4 4-4-4" />
<path d="M16 18a4 4 0 0 0-8 0" />"###
};
#[cfg(feature = "LuSuperscript")]
const LU_SUPERSCRIPT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m4 19 8-8" />
<path d="m12 19-8-8" />
<path d="M20 12h-4c0-1.5.442-2 1.5-2.5S20 8.334 20 7.002c0-.472-.17-.93-.484-1.29a2.105 2.105 0 0 0-2.617-.436c-.42.239-.738.614-.899 1.06" />"###
};
#[cfg(feature = "LuSwissFranc")]
const LU_SWISS_FRANC: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10 21V3h8" />
<path d="M6 16h9" />
<path d="M10 9.5h7" />"###
};
#[cfg(feature = "LuSwitchCamera")]
const LU_SWITCH_CAMERA: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M11 19H4a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h5" />
<path d="M13 5h7a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-5" />
<circle cx="12" cy="12" r="3" />
<path d="m18 22-3-3 3-3" />
<path d="m6 2 3 3-3 3" />"###
};
#[cfg(feature = "LuSword")]
const LU_SWORD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="14.5 17.5 3 6 3 3 6 3 17.5 14.5" />
<line x1="13" x2="19" y1="19" y2="13" />
<line x1="16" x2="20" y1="16" y2="20" />
<line x1="19" x2="21" y1="21" y2="19" />"###
};
#[cfg(feature = "LuSwords")]
const LU_SWORDS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="14.5 17.5 3 6 3 3 6 3 17.5 14.5" />
<line x1="13" x2="19" y1="19" y2="13" />
<line x1="16" x2="20" y1="16" y2="20" />
<line x1="19" x2="21" y1="21" y2="19" />
<polyline points="14.5 6.5 18 3 21 3 21 6 17.5 9.5" />
<line x1="5" x2="9" y1="14" y2="18" />
<line x1="7" x2="4" y1="17" y2="20" />
<line x1="3" x2="5" y1="19" y2="21" />"###
};
#[cfg(feature = "LuSyringe")]
const LU_SYRINGE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m18 2 4 4" />
<path d="m17 7 3-3" />
<path d="M19 9 8.7 19.3c-1 1-2.5 1-3.4 0l-.6-.6c-1-1-1-2.5 0-3.4L15 5" />
<path d="m9 11 4 4" />
<path d="m5 19-3 3" />
<path d="m14 4 6 6" />"###
};
#[cfg(feature = "LuTable")]
const LU_TABLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 3v18" />
<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="M3 9h18" />
<path d="M3 15h18" />"###
};
#[cfg(feature = "LuTable2")]
const LU_TABLE2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuTableProperties")]
const LU_TABLE_PROPERTIES: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M15 3v18" />
<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="M21 9H3" />
<path d="M21 15H3" />"###
};
#[cfg(feature = "LuTablet")]
const LU_TABLET: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="16" height="20" x="4" y="2" rx="2" ry="2" />
<line x1="12" x2="12.01" y1="18" y2="18" />"###
};
#[cfg(feature = "LuTablets")]
const LU_TABLETS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="7" cy="7" r="5" />
<circle cx="17" cy="17" r="5" />
<path d="M12 17h10" />
<path d="m3.46 10.54 7.08-7.08" />"###
};
#[cfg(feature = "LuTag")]
const LU_TAG: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 2H2v10l9.29 9.29c.94.94 2.48.94 3.42 0l6.58-6.58c.94-.94.94-2.48 0-3.42L12 2Z" />
<path d="M7 7h.01" />"###
};
#[cfg(feature = "LuTags")]
const LU_TAGS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M9 5H2v7l6.29 6.29c.94.94 2.48.94 3.42 0l3.58-3.58c.94-.94.94-2.48 0-3.42L9 5Z" />
<path d="M6 9.01V9" />
<path d="m15 5 6.3 6.3a2.4 2.4 0 0 1 0 3.4L17 19" />"###
};
#[cfg(feature = "LuTally1")]
const LU_TALLY1: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 4v16" />"###
};
#[cfg(feature = "LuTally2")]
const LU_TALLY2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 4v16" />
<path d="M9 4v16" />"###
};
#[cfg(feature = "LuTally3")]
const LU_TALLY3: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 4v16" />
<path d="M9 4v16" />
<path d="M14 4v16" />"###
};
#[cfg(feature = "LuTally4")]
const LU_TALLY4: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 4v16" />
<path d="M9 4v16" />
<path d="M14 4v16" />
<path d="M19 4v16" />"###
};
#[cfg(feature = "LuTally5")]
const LU_TALLY5: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 4v16" />
<path d="M9 4v16" />
<path d="M14 4v16" />
<path d="M19 4v16" />
<path d="M22 6 2 18" />"###
};
#[cfg(feature = "LuTarget")]
const LU_TARGET: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuTent")]
const LU_TENT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M19 20 10 4" />
<path d="m5 20 9-16" />
<path d="M3 20h18" />
<path d="m12 15-3 5" />
<path d="m12 15 3 5" />"###
};
#[cfg(feature = "LuTerminal")]
const LU_TERMINAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="12" x2="20" y1="19" y2="19" />"###
};
#[cfg(feature = "LuTerminalSquare")]
const LU_TERMINAL_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m7 11 2-2-2-2" />
<path d="M11 13h4" />
<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />"###
};
#[cfg(feature = "LuTestTube")]
const LU_TEST_TUBE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14.5 2v17.5c0 1.4-1.1 2.5-2.5 2.5h0c-1.4 0-2.5-1.1-2.5-2.5V2" />
<path d="M8.5 2h7" />
<path d="M14.5 16h-5" />"###
};
#[cfg(feature = "LuTestTube2")]
const LU_TEST_TUBE2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 7 6.82 21.18a2.83 2.83 0 0 1-3.99-.01v0a2.83 2.83 0 0 1 0-4L17 3" />
<path d="m16 2 6 6" />
<path d="M12 16H4" />"###
};
#[cfg(feature = "LuTestTubes")]
const LU_TEST_TUBES: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M9 2v17.5A2.5 2.5 0 0 1 6.5 22v0A2.5 2.5 0 0 1 4 19.5V2" />
<path d="M20 2v17.5a2.5 2.5 0 0 1-2.5 2.5v0a2.5 2.5 0 0 1-2.5-2.5V2" />
<path d="M3 2h7" />
<path d="M14 2h7" />
<path d="M9 16H4" />
<path d="M20 16h-5" />"###
};
#[cfg(feature = "LuText")]
const LU_TEXT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17 6.1H3" />
<path d="M21 12.1H3" />
<path d="M15.1 18H3" />"###
};
#[cfg(feature = "LuTextCursor")]
const LU_TEXT_CURSOR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17 22h-1a4 4 0 0 1-4-4V6a4 4 0 0 1 4-4h1" />
<path d="M7 22h1a4 4 0 0 0 4-4v-1" />
<path d="M7 2h1a4 4 0 0 1 4 4v1" />"###
};
#[cfg(feature = "LuTextCursorInput")]
const LU_TEXT_CURSOR_INPUT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 4h1a3 3 0 0 1 3 3 3 3 0 0 1 3-3h1" />
<path d="M13 20h-1a3 3 0 0 1-3-3 3 3 0 0 1-3 3H5" />
<path d="M5 16H4a2 2 0 0 1-2-2v-4a2 2 0 0 1 2-2h1" />
<path d="M13 8h7a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-7" />
<path d="M9 7v10" />"###
};
#[cfg(feature = "LuTextQuote")]
const LU_TEXT_QUOTE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17 6H3" />
<path d="M21 12H8" />
<path d="M21 18H8" />
<path d="M3 12v6" />"###
};
#[cfg(feature = "LuTextSelect")]
const LU_TEXT_SELECT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 3a2 2 0 0 0-2 2" />
<path d="M19 3a2 2 0 0 1 2 2" />
<path d="M21 19a2 2 0 0 1-2 2" />
<path d="M5 21a2 2 0 0 1-2-2" />
<path d="M9 3h1" />
<path d="M9 21h1" />
<path d="M14 3h1" />
<path d="M14 21h1" />
<path d="M3 9v1" />
<path d="M21 9v1" />
<path d="M3 14v1" />
<path d="M21 14v1" />
<line x1="7" x2="15" y1="8" y2="8" />
<line x1="7" x2="17" y1="12" y2="12" />
<line x1="7" x2="13" y1="16" y2="16" />"###
};
#[cfg(feature = "LuThermometer")]
const LU_THERMOMETER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14 4v10.54a4 4 0 1 1-4 0V4a2 2 0 0 1 4 0Z" />"###
};
#[cfg(feature = "LuThermometerSnowflake")]
const LU_THERMOMETER_SNOWFLAKE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 12h10" />
<path d="M9 4v16" />
<path d="m3 9 3 3-3 3" />
<path d="M12 6 9 9 6 6" />
<path d="m6 18 3-3 1.5 1.5" />
<path d="M20 4v10.54a4 4 0 1 1-4 0V4a2 2 0 0 1 4 0Z" />"###
};
#[cfg(feature = "LuThermometerSun")]
const LU_THERMOMETER_SUN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 9a4 4 0 0 0-2 7.5" />
<path d="M12 3v2" />
<path d="m6.6 18.4-1.4 1.4" />
<path d="M20 4v10.54a4 4 0 1 1-4 0V4a2 2 0 0 1 4 0Z" />
<path d="M4 13H2" />
<path d="M6.34 7.34 4.93 5.93" />"###
};
#[cfg(feature = "LuThumbsDown")]
const LU_THUMBS_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17 14V2" />
<path d="M9 18.12 10 14H4.17a2 2 0 0 1-1.92-2.56l2.33-8A2 2 0 0 1 6.5 2H20a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.76a2 2 0 0 0-1.79 1.11L12 22h0a3.13 3.13 0 0 1-3-3.88Z" />"###
};
#[cfg(feature = "LuThumbsUp")]
const LU_THUMBS_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M7 10v12" />
<path d="M15 5.88 14 10h5.83a2 2 0 0 1 1.92 2.56l-2.33 8A2 2 0 0 1 17.5 22H4a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2h2.76a2 2 0 0 0 1.79-1.11L12 2h0a3.13 3.13 0 0 1 3 3.88Z" />"###
};
#[cfg(feature = "LuTicket")]
const LU_TICKET: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 9a3 3 0 0 1 0 6v2a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-2a3 3 0 0 1 0-6V7a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z" />
<path d="M13 5v2" />
<path d="M13 17v2" />
<path d="M13 11v2" />"###
};
#[cfg(feature = "LuTimer")]
const LU_TIMER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="10" x2="14" y1="2" y2="2" />
<line x1="12" x2="15" y1="14" y2="11" />
<circle cx="12" cy="14" r="8" />"###
};
#[cfg(feature = "LuTimerOff")]
const LU_TIMER_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10 2h4" />
<path d="M4.6 11a8 8 0 0 0 1.7 8.7 8 8 0 0 0 8.7 1.7" />
<path d="M7.4 7.4a8 8 0 0 1 10.3 1 8 8 0 0 1 .9 10.2" />
<path d="m2 2 20 20" />
<path d="M12 12v-2" />"###
};
#[cfg(feature = "LuTimerReset")]
const LU_TIMER_RESET: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10 2h4" />
<path d="M12 14v-4" />
<path d="M4 13a8 8 0 0 1 8-7 8 8 0 1 1-5.3 14L4 17.6" />
<path d="M9 17H4v5" />"###
};
#[cfg(feature = "LuToggleLeft")]
const LU_TOGGLE_LEFT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="20" height="12" x="2" y="6" rx="6" ry="6" />
<circle cx="8" cy="12" r="2" />"###
};
#[cfg(feature = "LuToggleRight")]
const LU_TOGGLE_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="20" height="12" x="2" y="6" rx="6" ry="6" />
<circle cx="16" cy="12" r="2" />"###
};
#[cfg(feature = "LuTornado")]
const LU_TORNADO: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 4H3" />
<path d="M18 8H6" />
<path d="M19 12H9" />
<path d="M16 16h-6" />
<path d="M11 20H9" />"###
};
#[cfg(feature = "LuTouchpad")]
const LU_TOUCHPAD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="20" height="16" x="2" y="4" rx="2" />
<path d="M2 14h20" />
<path d="M12 20v-6" />"###
};
#[cfg(feature = "LuTouchpadOff")]
const LU_TOUCHPAD_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h16" />
<path d="M2 14h12" />
<path d="M22 14h-2" />
<path d="M12 20v-6" />
<path d="m2 2 20 20" />
<path d="M22 16V6a2 2 0 0 0-2-2H10" />"###
};
#[cfg(feature = "LuTowerControl")]
const LU_TOWER_CONTROL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18.2 12.27 20 6H4l1.8 6.27a1 1 0 0 0 .95.73h10.5a1 1 0 0 0 .96-.73Z" />
<path d="M8 13v9" />
<path d="M16 22v-9" />
<path d="m9 6 1 7" />
<path d="m15 6-1 7" />
<path d="M12 6V2" />
<path d="M13 2h-2" />"###
};
#[cfg(feature = "LuToyBrick")]
const LU_TOY_BRICK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="12" x="3" y="8" rx="1" />
<path d="M10 8V5c0-.6-.4-1-1-1H6a1 1 0 0 0-1 1v3" />
<path d="M19 8V5c0-.6-.4-1-1-1h-3a1 1 0 0 0-1 1v3" />"###
};
#[cfg(feature = "LuTractor")]
const LU_TRACTOR: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 4h9l1 7" />
<path d="M4 11V4" />
<path d="M8 10V4" />
<path d="M18 5c-.6 0-1 .4-1 1v5.6" />
<path d="m10 11 11 .9c.6 0 .9.5.8 1.1l-.8 5h-1" />
<circle cx="7" cy="15" r=".5" />
<circle cx="7" cy="15" r="5" />
<path d="M16 18h-5" />
<circle cx="18" cy="18" r="2" />"###
};
#[cfg(feature = "LuTrafficCone")]
const LU_TRAFFIC_CONE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M9.3 6.2a4.55 4.55 0 0 0 5.4 0" />
<path d="M7.9 10.7c.9.8 2.4 1.3 4.1 1.3s3.2-.5 4.1-1.3" />
<path d="M13.9 3.5a1.93 1.93 0 0 0-3.8-.1l-3 10c-.1.2-.1.4-.1.6 0 1.7 2.2 3 5 3s5-1.3 5-3c0-.2 0-.4-.1-.5Z" />
<path d="m7.5 12.2-4.7 2.7c-.5.3-.8.7-.8 1.1s.3.8.8 1.1l7.6 4.5c.9.5 2.1.5 3 0l7.6-4.5c.7-.3 1-.7 1-1.1s-.3-.8-.8-1.1l-4.7-2.8" />"###
};
#[cfg(feature = "LuTrainFront")]
const LU_TRAIN_FRONT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 3.1V7a4 4 0 0 0 8 0V3.1" />
<path d="m9 15-1-1" />
<path d="m15 15 1-1" />
<path d="M9 19c-2.8 0-5-2.2-5-5v-4a8 8 0 0 1 16 0v4c0 2.8-2.2 5-5 5Z" />
<path d="m8 19-2 3" />
<path d="m16 19 2 3" />"###
};
#[cfg(feature = "LuTrainFrontTunnel")]
const LU_TRAIN_FRONT_TUNNEL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 22V12a10 10 0 1 1 20 0v10" />
<path d="M15 6.8v1.4a3 2.8 0 1 1-6 0V6.8" />
<path d="M10 15h.01" />
<path d="M14 15h.01" />
<path d="M10 19a4 4 0 0 1-4-4v-3a6 6 0 1 1 12 0v3a4 4 0 0 1-4 4Z" />
<path d="m9 19-2 3" />
<path d="m15 19 2 3" />"###
};
#[cfg(feature = "LuTrainTrack")]
const LU_TRAIN_TRACK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 17 17 2" />
<path d="m2 14 8 8" />
<path d="m5 11 8 8" />
<path d="m8 8 8 8" />
<path d="m11 5 8 8" />
<path d="m14 2 8 8" />
<path d="M7 22 22 7" />"###
};
#[cfg(feature = "LuTramFront")]
const LU_TRAM_FRONT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="16" height="16" x="4" y="3" rx="2" />
<path d="M4 11h16" />
<path d="M12 3v8" />
<path d="m8 19-2 3" />
<path d="m18 22-2-3" />
<path d="M8 15h0" />
<path d="M16 15h0" />"###
};
#[cfg(feature = "LuTrash")]
const LU_TRASH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 6h18" />
<path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" />
<path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" />"###
};
#[cfg(feature = "LuTrash2")]
const LU_TRASH2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 6h18" />
<path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" />
<path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" />
<line x1="10" x2="10" y1="11" y2="17" />
<line x1="14" x2="14" y1="11" y2="17" />"###
};
#[cfg(feature = "LuTreeDeciduous")]
const LU_TREE_DECIDUOUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 19h8a4 4 0 0 0 3.8-2.8 4 4 0 0 0-1.6-4.5c1-1.1 1-2.7.4-4-.7-1.2-2.2-2-3.6-1.7a3 3 0 0 0-3-3 3 3 0 0 0-3 3c-1.4-.2-2.9.5-3.6 1.7-.7 1.3-.5 2.9.4 4a4 4 0 0 0-1.6 4.5A4 4 0 0 0 8 19Z" />
<path d="M12 19v3" />"###
};
#[cfg(feature = "LuTreePine")]
const LU_TREE_PINE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m17 14 3 3.3a1 1 0 0 1-.7 1.7H4.7a1 1 0 0 1-.7-1.7L7 14h-.3a1 1 0 0 1-.7-1.7L9 9h-.2A1 1 0 0 1 8 7.3L12 3l4 4.3a1 1 0 0 1-.8 1.7H15l3 3.3a1 1 0 0 1-.7 1.7H17Z" />
<path d="M12 22v-3" />"###
};
#[cfg(feature = "LuTrees")]
const LU_TREES: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10 10v.2A3 3 0 0 1 8.9 16v0H5v0h0a3 3 0 0 1-1-5.8V10a3 3 0 0 1 6 0Z" />
<path d="M7 16v6" />
<path d="M13 19v3" />
<path d="M12 19h8.3a1 1 0 0 0 .7-1.7L18 14h.3a1 1 0 0 0 .7-1.7L16 9h.2a1 1 0 0 0 .8-1.7L13 3l-1.4 1.5" />"###
};
#[cfg(feature = "LuTrello")]
const LU_TRELLO: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<rect width="3" height="9" x="7" y="7" />
<rect width="3" height="5" x="14" y="7" />"###
};
#[cfg(feature = "LuTrendingDown")]
const LU_TRENDING_DOWN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="22 17 13.5 8.5 8.5 13.5 2 7" />
<polyline points="16 17 22 17 22 11" />"###
};
#[cfg(feature = "LuTrendingUp")]
const LU_TRENDING_UP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<polyline points="22 7 13.5 15.5 8.5 10.5 2 17" />
<polyline points="16 7 22 7 22 13" />"###
};
#[cfg(feature = "LuTriangle")]
const LU_TRIANGLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z" />"###
};
#[cfg(feature = "LuTriangleRight")]
const LU_TRIANGLE_RIGHT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 18a2 2 0 0 1-2 2H3c-1.1 0-1.3-.6-.4-1.3L20.4 4.3c.9-.7 1.6-.4 1.6.7Z" />"###
};
#[cfg(feature = "LuTrophy")]
const LU_TROPHY: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 9H4.5a2.5 2.5 0 0 1 0-5H6" />
<path d="M18 9h1.5a2.5 2.5 0 0 0 0-5H18" />
<path d="M4 22h16" />
<path d="M10 14.66V17c0 .55-.47.98-.97 1.21C7.85 18.75 7 20.24 7 22" />
<path d="M14 14.66V17c0 .55.47.98.97 1.21C16.15 18.75 17 20.24 17 22" />
<path d="M18 2H6v7a6 6 0 0 0 12 0V2Z" />"###
};
#[cfg(feature = "LuTruck")]
const LU_TRUCK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 18H3c-.6 0-1-.4-1-1V7c0-.6.4-1 1-1h10c.6 0 1 .4 1 1v11" />
<path d="M14 9h4l4 4v4c0 .6-.4 1-1 1h-2" />
<circle cx="7" cy="18" r="2" />
<path d="M15 18H9" />
<circle cx="17" cy="18" r="2" />"###
};
#[cfg(feature = "LuTurtle")]
const LU_TURTLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m12 10 2 4v3a1 1 0 0 0 1 1h2a1 1 0 0 0 1-1v-3a8 8 0 1 0-16 0v3a1 1 0 0 0 1 1h2a1 1 0 0 0 1-1v-3l2-4h4Z" />
<path d="M4.82 7.9 8 10" />
<path d="M15.18 7.9 12 10" />
<path d="M16.93 10H20a2 2 0 0 1 0 4H2" />"###
};
#[cfg(feature = "LuTv")]
const LU_TV: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="20" height="15" x="2" y="7" rx="2" ry="2" />
<polyline points="17 2 12 7 7 2" />"###
};
#[cfg(feature = "LuTv2")]
const LU_TV2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M7 21h10" />
<rect width="20" height="14" x="2" y="3" rx="2" />"###
};
#[cfg(feature = "LuTwitch")]
const LU_TWITCH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 2H3v16h5v4l4-4h5l4-4V2zm-10 9V7m5 4V7" />"###
};
#[cfg(feature = "LuTwitter")]
const LU_TWITTER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 4s-.7 2.1-2 3.4c1.6 10-9.4 17.3-18 11.6 2.2.1 4.4-.6 6-2C3 15.5.5 9.6 3 5c2.2 2.6 5.6 4.1 9 4-.9-4.2 4-6.6 7-3.8 1.1 0 3-1.2 3-1.2z" />"###
};
#[cfg(feature = "LuType")]
const LU_TYPE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="9" x2="15" y1="20" y2="20" />
<line x1="12" x2="12" y1="4" y2="20" />"###
};
#[cfg(feature = "LuUmbrella")]
const LU_UMBRELLA: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 12a10.06 10.06 1 0 0-20 0Z" />
<path d="M12 12v8a2 2 0 0 0 4 0" />
<path d="M12 2v1" />"###
};
#[cfg(feature = "LuUnderline")]
const LU_UNDERLINE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M6 4v6a6 6 0 0 0 12 0V4" />
<line x1="4" x2="20" y1="20" y2="20" />"###
};
#[cfg(feature = "LuUndo")]
const LU_UNDO: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 7v6h6" />
<path d="M21 17a9 9 0 0 0-9-9 9 9 0 0 0-6 2.3L3 13" />"###
};
#[cfg(feature = "LuUndo2")]
const LU_UNDO2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M9 14 4 9l5-5" />
<path d="M4 9h10.5a5.5 5.5 0 0 1 5.5 5.5v0a5.5 5.5 0 0 1-5.5 5.5H11" />"###
};
#[cfg(feature = "LuUndoDot")]
const LU_UNDO_DOT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="17" r="1" />
<path d="M3 7v6h6" />
<path d="M21 17a9 9 0 0 0-9-9 9 9 0 0 0-6 2.3L3 13" />"###
};
#[cfg(feature = "LuUnfoldHorizontal")]
const LU_UNFOLD_HORIZONTAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16 12h6" />
<path d="M8 12H2" />
<path d="M12 2v2" />
<path d="M12 8v2" />
<path d="M12 14v2" />
<path d="M12 20v2" />
<path d="m19 15 3-3-3-3" />
<path d="m5 9-3 3 3 3" />"###
};
#[cfg(feature = "LuUnfoldVertical")]
const LU_UNFOLD_VERTICAL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 22v-6" />
<path d="M12 8V2" />
<path d="M4 12H2" />
<path d="M10 12H8" />
<path d="M16 12h-2" />
<path d="M22 12h-2" />
<path d="m15 19-3 3-3-3" />
<path d="m15 5-3-3-3 3" />"###
};
#[cfg(feature = "LuUngroup")]
const LU_UNGROUP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="8" height="6" x="5" y="4" rx="1" />
<rect width="8" height="6" x="11" y="14" rx="1" />"###
};
#[cfg(feature = "LuUnlink")]
const LU_UNLINK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m18.84 12.25 1.72-1.71h-.02a5.004 5.004 0 0 0-.12-7.07 5.006 5.006 0 0 0-6.95 0l-1.72 1.71" />
<path d="m5.17 11.75-1.71 1.71a5.004 5.004 0 0 0 .12 7.07 5.006 5.006 0 0 0 6.95 0l1.71-1.71" />
<line x1="8" x2="8" y1="2" y2="5" />
<line x1="2" x2="5" y1="8" y2="8" />
<line x1="16" x2="16" y1="19" y2="22" />
<line x1="19" x2="22" y1="16" y2="16" />"###
};
#[cfg(feature = "LuUnlink2")]
const LU_UNLINK2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M15 7h2a5 5 0 0 1 0 10h-2m-6 0H7A5 5 0 0 1 7 7h2" />"###
};
#[cfg(feature = "LuUnlock")]
const LU_UNLOCK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="11" x="3" y="11" rx="2" ry="2" />
<path d="M7 11V7a5 5 0 0 1 9.9-1" />"###
};
#[cfg(feature = "LuUnplug")]
const LU_UNPLUG: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m19 5 3-3" />
<path d="m2 22 3-3" />
<path d="M6.3 20.3a2.4 2.4 0 0 0 3.4 0L12 18l-6-6-2.3 2.3a2.4 2.4 0 0 0 0 3.4Z" />
<path d="M7.5 13.5 10 11" />
<path d="M10.5 16.5 13 14" />
<path d="m12 6 6 6 2.3-2.3a2.4 2.4 0 0 0 0-3.4l-2.6-2.6a2.4 2.4 0 0 0-3.4 0Z" />"###
};
#[cfg(feature = "LuUpload")]
const LU_UPLOAD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="12" x2="12" y1="3" y2="15" />"###
};
#[cfg(feature = "LuUploadCloud")]
const LU_UPLOAD_CLOUD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" />
<path d="M12 12v9" />
<path d="m16 16-4-4-4 4" />"###
};
#[cfg(feature = "LuUsb")]
const LU_USB: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="10" cy="7" r="1" />
<circle cx="4" cy="20" r="1" />
<path d="M4.7 19.3 19 5" />
<path d="m21 3-3 1 2 2Z" />
<path d="M9.26 7.68 5 12l2 5" />
<path d="m10 14 5 2 3.5-3.5" />
<path d="m18 12 1-1 1 1-1 1Z" />"###
};
#[cfg(feature = "LuUser")]
const LU_USER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2" />
<circle cx="12" cy="7" r="4" />"###
};
#[cfg(feature = "LuUser2")]
const LU_USER2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="8" r="5" />
<path d="M20 21a8 8 0 1 0-16 0" />"###
};
#[cfg(feature = "LuUserCheck")]
const LU_USER_CHECK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" />
<circle cx="9" cy="7" r="4" />
<polyline points="16 11 18 13 22 9" />"###
};
#[cfg(feature = "LuUserCheck2")]
const LU_USER_CHECK2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14 19a6 6 0 0 0-12 0" />
<circle cx="8" cy="9" r="4" />
<polyline points="16 11 18 13 22 9" />"###
};
#[cfg(feature = "LuUserCircle")]
const LU_USER_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<circle cx="12" cy="10" r="3" />
<path d="M7 20.662V19a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v1.662" />"###
};
#[cfg(feature = "LuUserCircle2")]
const LU_USER_CIRCLE2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18 20a6 6 0 0 0-12 0" />
<circle cx="12" cy="10" r="4" />
<circle cx="12" cy="12" r="10" />"###
};
#[cfg(feature = "LuUserCog")]
const LU_USER_COG: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" />
<circle cx="9" cy="7" r="4" />
<circle cx="19" cy="11" r="2" />
<path d="M19 8v1" />
<path d="M19 13v1" />
<path d="m21.6 9.5-.87.5" />
<path d="m17.27 12-.87.5" />
<path d="m21.6 12.5-.87-.5" />
<path d="m17.27 10-.87-.5" />"###
};
#[cfg(feature = "LuUserCog2")]
const LU_USER_COG2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14 19a6 6 0 0 0-12 0" />
<circle cx="8" cy="9" r="4" />
<circle cx="19" cy="11" r="2" />
<path d="M19 8v1" />
<path d="M19 13v1" />
<path d="m21.6 9.5-.87.5" />
<path d="m17.27 12-.87.5" />
<path d="m21.6 12.5-.87-.5" />
<path d="m17.27 10-.87-.5" />"###
};
#[cfg(feature = "LuUserMinus")]
const LU_USER_MINUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" />
<circle cx="9" cy="7" r="4" />
<line x1="22" x2="16" y1="11" y2="11" />"###
};
#[cfg(feature = "LuUserMinus2")]
const LU_USER_MINUS2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14 19a6 6 0 0 0-12 0" />
<circle cx="8" cy="9" r="4" />
<line x1="22" x2="16" y1="11" y2="11" />"###
};
#[cfg(feature = "LuUserPlus")]
const LU_USER_PLUS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" />
<circle cx="9" cy="7" r="4" />
<line x1="19" x2="19" y1="8" y2="14" />
<line x1="22" x2="16" y1="11" y2="11" />"###
};
#[cfg(feature = "LuUserPlus2")]
const LU_USER_PLUS2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14 19a6 6 0 0 0-12 0" />
<circle cx="8" cy="9" r="4" />
<line x1="19" x2="19" y1="8" y2="14" />
<line x1="22" x2="16" y1="11" y2="11" />"###
};
#[cfg(feature = "LuUserSquare")]
const LU_USER_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<circle cx="12" cy="10" r="3" />
<path d="M7 21v-2a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v2" />"###
};
#[cfg(feature = "LuUserSquare2")]
const LU_USER_SQUARE2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18 21a6 6 0 0 0-12 0" />
<circle cx="12" cy="11" r="4" />
<rect width="18" height="18" x="3" y="3" rx="2" />"###
};
#[cfg(feature = "LuUserX")]
const LU_USER_X: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" />
<circle cx="9" cy="7" r="4" />
<line x1="17" x2="22" y1="8" y2="13" />
<line x1="22" x2="17" y1="8" y2="13" />"###
};
#[cfg(feature = "LuUserX2")]
const LU_USER_X2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14 19a6 6 0 0 0-12 0" />
<circle cx="8" cy="9" r="4" />
<line x1="17" x2="22" y1="8" y2="13" />
<line x1="22" x2="17" y1="8" y2="13" />"###
};
#[cfg(feature = "LuUsers")]
const LU_USERS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" />
<circle cx="9" cy="7" r="4" />
<path d="M22 21v-2a4 4 0 0 0-3-3.87" />
<path d="M16 3.13a4 4 0 0 1 0 7.75" />"###
};
#[cfg(feature = "LuUsers2")]
const LU_USERS2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M14 19a6 6 0 0 0-12 0" />
<circle cx="8" cy="9" r="4" />
<path d="M22 19a6 6 0 0 0-6-6 4 4 0 1 0 0-8" />"###
};
#[cfg(feature = "LuUtensils")]
const LU_UTENSILS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M3 2v7c0 1.1.9 2 2 2h4a2 2 0 0 0 2-2V2" />
<path d="M7 2v20" />
<path d="M21 15V2v0a5 5 0 0 0-5 5v6c0 1.1.9 2 2 2h3Zm0 0v7" />"###
};
#[cfg(feature = "LuUtensilsCrossed")]
const LU_UTENSILS_CROSSED: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m16 2-2.3 2.3a3 3 0 0 0 0 4.2l1.8 1.8a3 3 0 0 0 4.2 0L22 8" />
<path d="M15 15 3.3 3.3a4.2 4.2 0 0 0 0 6l7.3 7.3c.7.7 2 .7 2.8 0L15 15Zm0 0 7 7" />
<path d="m2.1 21.8 6.4-6.3" />
<path d="m19 5-7 7" />"###
};
#[cfg(feature = "LuUtilityPole")]
const LU_UTILITY_POLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M12 2v20" />
<path d="M2 5h20" />
<path d="M3 3v2" />
<path d="M7 3v2" />
<path d="M17 3v2" />
<path d="M21 3v2" />
<path d="m19 5-7 7-7-7" />"###
};
#[cfg(feature = "LuVariable")]
const LU_VARIABLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 21s-4-3-4-9 4-9 4-9" />
<path d="M16 3s4 3 4 9-4 9-4 9" />
<line x1="15" x2="9" y1="9" y2="15" />
<line x1="9" x2="15" y1="9" y2="15" />"###
};
#[cfg(feature = "LuVegan")]
const LU_VEGAN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 2a26.6 26.6 0 0 1 10 20c.9-6.82 1.5-9.5 4-14" />
<path d="M16 8c4 0 6-2 6-6-4 0-6 2-6 6" />
<path d="M17.41 3.6a10 10 0 1 0 3 3" />"###
};
#[cfg(feature = "LuVenetianMask")]
const LU_VENETIAN_MASK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 12a5 5 0 0 0 5 5 8 8 0 0 1 5 2 8 8 0 0 1 5-2 5 5 0 0 0 5-5V7h-5a8 8 0 0 0-5 2 8 8 0 0 0-5-2H2Z" />
<path d="M6 11c1.5 0 3 .5 3 2-2 0-3 0-3-2Z" />
<path d="M18 11c-1.5 0-3 .5-3 2 2 0 3 0 3-2Z" />"###
};
#[cfg(feature = "LuVibrate")]
const LU_VIBRATE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m2 8 2 2-2 2 2 2-2 2" />
<path d="m22 8-2 2 2 2-2 2 2 2" />
<rect width="8" height="14" x="8" y="5" rx="1" />"###
};
#[cfg(feature = "LuVibrateOff")]
const LU_VIBRATE_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m2 8 2 2-2 2 2 2-2 2" />
<path d="m22 8-2 2 2 2-2 2 2 2" />
<path d="M8 8v10c0 .55.45 1 1 1h6c.55 0 1-.45 1-1v-2" />
<path d="M16 10.34V6c0-.55-.45-1-1-1h-4.34" />
<line x1="2" x2="22" y1="2" y2="22" />"###
};
#[cfg(feature = "LuVideo")]
const LU_VIDEO: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m22 8-6 4 6 4V8Z" />
<rect width="14" height="12" x="2" y="6" rx="2" ry="2" />"###
};
#[cfg(feature = "LuVideoOff")]
const LU_VIDEO_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M10.66 6H14a2 2 0 0 1 2 2v2.34l1 1L22 8v8" />
<path d="M16 16a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h2l10 10Z" />
<line x1="2" x2="22" y1="2" y2="22" />"###
};
#[cfg(feature = "LuVideotape")]
const LU_VIDEOTAPE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="20" height="16" x="2" y="4" rx="2" />
<path d="M2 8h20" />
<circle cx="8" cy="14" r="2" />
<path d="M8 12h8" />
<circle cx="16" cy="14" r="2" />"###
};
#[cfg(feature = "LuView")]
const LU_VIEW: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 12s2.545-5 7-5c4.454 0 7 5 7 5s-2.546 5-7 5c-4.455 0-7-5-7-5z" />
<path d="M12 13a1 1 0 1 0 0-2 1 1 0 0 0 0 2z" />
<path d="M21 17v2a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-2" />
<path d="M21 7V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v2" />"###
};
#[cfg(feature = "LuVoicemail")]
const LU_VOICEMAIL: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="6" cy="12" r="4" />
<circle cx="18" cy="12" r="4" />
<line x1="6" x2="18" y1="16" y2="16" />"###
};
#[cfg(feature = "LuVolume")]
const LU_VOLUME: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuVolume1")]
const LU_VOLUME1: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuVolume2")]
const LU_VOLUME2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="M15.54 8.46a5 5 0 0 1 0 7.07" />
<path d="M19.07 4.93a10 10 0 0 1 0 14.14" />"###
};
#[cfg(feature = "LuVolumeX")]
const LU_VOLUME_X: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="22" x2="16" y1="9" y2="15" />
<line x1="16" x2="22" y1="9" y2="15" />"###
};
#[cfg(feature = "LuVote")]
const LU_VOTE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m9 12 2 2 4-4" />
<path d="M5 7c0-1.1.9-2 2-2h10a2 2 0 0 1 2 2v12H5V7Z" />
<path d="M22 19H2" />"###
};
#[cfg(feature = "LuWallet")]
const LU_WALLET: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M21 12V7H5a2 2 0 0 1 0-4h14v4" />
<path d="M3 5v14a2 2 0 0 0 2 2h16v-5" />
<path d="M18 12a2 2 0 0 0 0 4h4v-4Z" />"###
};
#[cfg(feature = "LuWallet2")]
const LU_WALLET2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17 14h.01" />
<path d="M7 7h12a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14" />"###
};
#[cfg(feature = "LuWalletCards")]
const LU_WALLET_CARDS: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" />
<path d="M3 9a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2" />
<path d="M3 11h3c.8 0 1.6.3 2.1.9l1.1.9c1.6 1.6 4.1 1.6 5.7 0l1.1-.9c.5-.5 1.3-.9 2.1-.9H21" />"###
};
#[cfg(feature = "LuWallpaper")]
const LU_WALLPAPER: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="8" cy="9" r="2" />
<path d="m9 17 6.1-6.1a2 2 0 0 1 2.81.01L22 15V5a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2" />
<path d="M8 21h8" />
<path d="M12 17v4" />"###
};
#[cfg(feature = "LuWand")]
const LU_WAND: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M15 4V2" />
<path d="M15 16v-2" />
<path d="M8 9h2" />
<path d="M20 9h2" />
<path d="M17.8 11.8 19 13" />
<path d="M15 9h0" />
<path d="M17.8 6.2 19 5" />
<path d="m3 21 9-9" />
<path d="M12.2 6.2 11 5" />"###
};
#[cfg(feature = "LuWand2")]
const LU_WAND2: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m21.64 3.64-1.28-1.28a1.21 1.21 0 0 0-1.72 0L2.36 18.64a1.21 1.21 0 0 0 0 1.72l1.28 1.28a1.2 1.2 0 0 0 1.72 0L21.64 5.36a1.2 1.2 0 0 0 0-1.72Z" />
<path d="m14 7 3 3" />
<path d="M5 6v4" />
<path d="M19 14v4" />
<path d="M10 2v2" />
<path d="M7 8H3" />
<path d="M21 16h-4" />
<path d="M11 3H9" />"###
};
#[cfg(feature = "LuWarehouse")]
const LU_WAREHOUSE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M22 8.35V20a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V8.35A2 2 0 0 1 3.26 6.5l8-3.2a2 2 0 0 1 1.48 0l8 3.2A2 2 0 0 1 22 8.35Z" />
<path d="M6 18h12" />
<path d="M6 14h12" />
<rect width="12" height="12" x="6" y="10" />"###
};
#[cfg(feature = "LuWatch")]
const LU_WATCH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="12" r="6" />
<polyline points="12 10 12 12 13 13" />
<path d="m16.13 7.66-.81-4.05a2 2 0 0 0-2-1.61h-2.68a2 2 0 0 0-2 1.61l-.78 4.05" />
<path d="m7.88 16.36.8 4a2 2 0 0 0 2 1.61h2.72a2 2 0 0 0 2-1.61l.81-4.05" />"###
};
#[cfg(feature = "LuWaves")]
const LU_WAVES: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 6c.6.5 1.2 1 2.5 1C7 7 7 5 9.5 5c2.6 0 2.4 2 5 2 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1" />
<path d="M2 12c.6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 2.6 0 2.4 2 5 2 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1" />
<path d="M2 18c.6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 2.6 0 2.4 2 5 2 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1" />"###
};
#[cfg(feature = "LuWebcam")]
const LU_WEBCAM: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="12" cy="10" r="8" />
<circle cx="12" cy="10" r="3" />
<path d="M7 22h10" />
<path d="M12 22v-4" />"###
};
#[cfg(feature = "LuWebhook")]
const LU_WEBHOOK: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18 16.98h-5.99c-1.1 0-1.95.94-2.48 1.9A4 4 0 0 1 2 17c.01-.7.2-1.4.57-2" />
<path d="m6 17 3.13-5.78c.53-.97.1-2.18-.5-3.1a4 4 0 1 1 6.89-4.06" />
<path d="m12 6 3.13 5.73C15.66 12.7 16.9 13 18 13a4 4 0 0 1 0 8" />"###
};
#[cfg(feature = "LuWheat")]
const LU_WHEAT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2 22 16 8" />
<path d="M3.47 12.53 5 11l1.53 1.53a3.5 3.5 0 0 1 0 4.94L5 19l-1.53-1.53a3.5 3.5 0 0 1 0-4.94Z" />
<path d="M7.47 8.53 9 7l1.53 1.53a3.5 3.5 0 0 1 0 4.94L9 15l-1.53-1.53a3.5 3.5 0 0 1 0-4.94Z" />
<path d="M11.47 4.53 13 3l1.53 1.53a3.5 3.5 0 0 1 0 4.94L13 11l-1.53-1.53a3.5 3.5 0 0 1 0-4.94Z" />
<path d="M20 2h2v2a4 4 0 0 1-4 4h-2V6a4 4 0 0 1 4-4Z" />
<path d="M11.47 17.47 13 19l-1.53 1.53a3.5 3.5 0 0 1-4.94 0L5 19l1.53-1.53a3.5 3.5 0 0 1 4.94 0Z" />
<path d="M15.47 13.47 17 15l-1.53 1.53a3.5 3.5 0 0 1-4.94 0L9 15l1.53-1.53a3.5 3.5 0 0 1 4.94 0Z" />
<path d="M19.47 9.47 21 11l-1.53 1.53a3.5 3.5 0 0 1-4.94 0L13 11l1.53-1.53a3.5 3.5 0 0 1 4.94 0Z" />"###
};
#[cfg(feature = "LuWheatOff")]
const LU_WHEAT_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="m2 22 10-10" />
<path d="m16 8-1.17 1.17" />
<path d="M3.47 12.53 5 11l1.53 1.53a3.5 3.5 0 0 1 0 4.94L5 19l-1.53-1.53a3.5 3.5 0 0 1 0-4.94Z" />
<path d="m8 8-.53.53a3.5 3.5 0 0 0 0 4.94L9 15l1.53-1.53c.55-.55.88-1.25.98-1.97" />
<path d="M10.91 5.26c.15-.26.34-.51.56-.73L13 3l1.53 1.53a3.5 3.5 0 0 1 .28 4.62" />
<path d="M20 2h2v2a4 4 0 0 1-4 4h-2V6a4 4 0 0 1 4-4Z" />
<path d="M11.47 17.47 13 19l-1.53 1.53a3.5 3.5 0 0 1-4.94 0L5 19l1.53-1.53a3.5 3.5 0 0 1 4.94 0Z" />
<path d="m16 16-.53.53a3.5 3.5 0 0 1-4.94 0L9 15l1.53-1.53a3.49 3.49 0 0 1 1.97-.98" />
<path d="M18.74 13.09c.26-.15.51-.34.73-.56L21 11l-1.53-1.53a3.5 3.5 0 0 0-4.62-.28" />
<line x1="2" x2="22" y1="2" y2="22" />"###
};
#[cfg(feature = "LuWholeWord")]
const LU_WHOLE_WORD: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="7" cy="12" r="3" />
<path d="M10 9v6" />
<circle cx="17" cy="12" r="3" />
<path d="M14 7v8" />
<path d="M22 17v1c0 .5-.5 1-1 1H3c-.5 0-1-.5-1-1v-1" />"###
};
#[cfg(feature = "LuWifi")]
const LU_WIFI: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M5 13a10 10 0 0 1 14 0" />
<path d="M8.5 16.5a5 5 0 0 1 7 0" />
<path d="M2 8.82a15 15 0 0 1 20 0" />
<line x1="12" x2="12.01" y1="20" y2="20" />"###
};
#[cfg(feature = "LuWifiOff")]
const LU_WIFI_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="2" x2="22" y1="2" y2="22" />
<path d="M8.5 16.5a5 5 0 0 1 7 0" />
<path d="M2 8.82a15 15 0 0 1 4.17-2.65" />
<path d="M10.66 5c4.01-.36 8.14.9 11.34 3.76" />
<path d="M16.85 11.25a10 10 0 0 1 2.22 1.68" />
<path d="M5 13a10 10 0 0 1 5.24-2.76" />
<line x1="12" x2="12.01" y1="20" y2="20" />"###
};
#[cfg(feature = "LuWind")]
const LU_WIND: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M17.7 7.7a2.5 2.5 0 1 1 1.8 4.3H2" />
<path d="M9.6 4.6A2 2 0 1 1 11 8H2" />
<path d="M12.6 19.4A2 2 0 1 0 14 16H2" />"###
};
#[cfg(feature = "LuWine")]
const LU_WINE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 22h8" />
<path d="M7 10h10" />
<path d="M12 15v7" />
<path d="M12 15a5 5 0 0 0 5-5c0-2-.5-4-2-8H9c-1.5 4-2 6-2 8a5 5 0 0 0 5 5Z" />"###
};
#[cfg(feature = "LuWineOff")]
const LU_WINE_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M8 22h8" />
<path d="M7 10h3m7 0h-1.343" />
<path d="M12 15v7" />
<path d="M7.307 7.307A12.33 12.33 0 0 0 7 10a5 5 0 0 0 7.391 4.391M8.638 2.981C8.75 2.668 8.872 2.34 9 2h6c1.5 4 2 6 2 8 0 .407-.05.809-.145 1.198" />
<line x1="2" x2="22" y1="2" y2="22" />"###
};
#[cfg(feature = "LuWorkflow")]
const LU_WORKFLOW: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="8" height="8" x="3" y="3" rx="2" />
<path d="M7 11v4a2 2 0 0 0 2 2h4" />
<rect width="8" height="8" x="13" y="13" rx="2" />"###
};
#[cfg(feature = "LuWrapText")]
const LU_WRAP_TEXT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<line x1="3" x2="21" y1="6" y2="6" />
<path d="M3 12h15a3 3 0 1 1 0 6h-4" />
<polyline points="16 16 14 18 16 20" />
<line x1="3" x2="10" y1="18" y2="18" />"###
};
#[cfg(feature = "LuWrench")]
const LU_WRENCH: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuX")]
const LU_X: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M18 6 6 18" />
<path d="m6 6 12 12" />"###
};
#[cfg(feature = "LuXCircle")]
const LU_X_CIRCLE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="m15 9-6 6" />
<path d="m9 9 6 6" />"###
};
#[cfg(feature = "LuXOctagon")]
const LU_X_OCTAGON: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<path d="m15 9-6 6" />
<path d="m9 9 6 6" />"###
};
#[cfg(feature = "LuXSquare")]
const LU_X_SQUARE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
<path d="m15 9-6 6" />
<path d="m9 9 6 6" />"###
};
#[cfg(feature = "LuYoutube")]
const LU_YOUTUBE: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<path d="M2.5 17a24.12 24.12 0 0 1 0-10 2 2 0 0 1 1.4-1.4 49.56 49.56 0 0 1 16.2 0A2 2 0 0 1 21.5 7a24.12 24.12 0 0 1 0 10 2 2 0 0 1-1.4 1.4 49.55 49.55 0 0 1-16.2 0A2 2 0 0 1 2.5 17" />
<path d="m10 15 5-3-5-3z" />"###
};
#[cfg(feature = "LuZap")]
const LU_ZAP: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
#[cfg(feature = "LuZapOff")]
const LU_ZAP_OFF: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="2" x2="22" y1="2" y2="22" />"###
};
#[cfg(feature = "LuZoomIn")]
const LU_ZOOM_IN: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="21" x2="16.65" y1="21" y2="16.65" />
<line x1="11" x2="11" y1="8" y2="14" />
<line x1="8" x2="14" y1="11" y2="11" />"###
};
#[cfg(feature = "LuZoomOut")]
const LU_ZOOM_OUT: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
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
<line x1="21" x2="16.65" y1="21" y2="16.65" />
<line x1="8" x2="14" y1="11" y2="11" />"###
};

impl From<LuIcon> for icondata_core::IconData {
    fn from(icon: LuIcon) -> icondata_core::IconData {
        match icon {
            #[cfg(feature = "LuAccessibility")]
            LuIcon::LuAccessibility => LU_ACCESSIBILITY,
            #[cfg(feature = "LuActivity")]
            LuIcon::LuActivity => LU_ACTIVITY,
            #[cfg(feature = "LuActivitySquare")]
            LuIcon::LuActivitySquare => LU_ACTIVITY_SQUARE,
            #[cfg(feature = "LuAirVent")]
            LuIcon::LuAirVent => LU_AIR_VENT,
            #[cfg(feature = "LuAirplay")]
            LuIcon::LuAirplay => LU_AIRPLAY,
            #[cfg(feature = "LuAlarmCheck")]
            LuIcon::LuAlarmCheck => LU_ALARM_CHECK,
            #[cfg(feature = "LuAlarmClock")]
            LuIcon::LuAlarmClock => LU_ALARM_CLOCK,
            #[cfg(feature = "LuAlarmClockOff")]
            LuIcon::LuAlarmClockOff => LU_ALARM_CLOCK_OFF,
            #[cfg(feature = "LuAlarmMinus")]
            LuIcon::LuAlarmMinus => LU_ALARM_MINUS,
            #[cfg(feature = "LuAlarmPlus")]
            LuIcon::LuAlarmPlus => LU_ALARM_PLUS,
            #[cfg(feature = "LuAlbum")]
            LuIcon::LuAlbum => LU_ALBUM,
            #[cfg(feature = "LuAlertCircle")]
            LuIcon::LuAlertCircle => LU_ALERT_CIRCLE,
            #[cfg(feature = "LuAlertOctagon")]
            LuIcon::LuAlertOctagon => LU_ALERT_OCTAGON,
            #[cfg(feature = "LuAlertTriangle")]
            LuIcon::LuAlertTriangle => LU_ALERT_TRIANGLE,
            #[cfg(feature = "LuAlignCenter")]
            LuIcon::LuAlignCenter => LU_ALIGN_CENTER,
            #[cfg(feature = "LuAlignCenterHorizontal")]
            LuIcon::LuAlignCenterHorizontal => LU_ALIGN_CENTER_HORIZONTAL,
            #[cfg(feature = "LuAlignCenterVertical")]
            LuIcon::LuAlignCenterVertical => LU_ALIGN_CENTER_VERTICAL,
            #[cfg(feature = "LuAlignEndHorizontal")]
            LuIcon::LuAlignEndHorizontal => LU_ALIGN_END_HORIZONTAL,
            #[cfg(feature = "LuAlignEndVertical")]
            LuIcon::LuAlignEndVertical => LU_ALIGN_END_VERTICAL,
            #[cfg(feature = "LuAlignHorizontalDistributeCenter")]
            LuIcon::LuAlignHorizontalDistributeCenter => LU_ALIGN_HORIZONTAL_DISTRIBUTE_CENTER,
            #[cfg(feature = "LuAlignHorizontalDistributeEnd")]
            LuIcon::LuAlignHorizontalDistributeEnd => LU_ALIGN_HORIZONTAL_DISTRIBUTE_END,
            #[cfg(feature = "LuAlignHorizontalDistributeStart")]
            LuIcon::LuAlignHorizontalDistributeStart => LU_ALIGN_HORIZONTAL_DISTRIBUTE_START,
            #[cfg(feature = "LuAlignHorizontalJustifyCenter")]
            LuIcon::LuAlignHorizontalJustifyCenter => LU_ALIGN_HORIZONTAL_JUSTIFY_CENTER,
            #[cfg(feature = "LuAlignHorizontalJustifyEnd")]
            LuIcon::LuAlignHorizontalJustifyEnd => LU_ALIGN_HORIZONTAL_JUSTIFY_END,
            #[cfg(feature = "LuAlignHorizontalJustifyStart")]
            LuIcon::LuAlignHorizontalJustifyStart => LU_ALIGN_HORIZONTAL_JUSTIFY_START,
            #[cfg(feature = "LuAlignHorizontalSpaceAround")]
            LuIcon::LuAlignHorizontalSpaceAround => LU_ALIGN_HORIZONTAL_SPACE_AROUND,
            #[cfg(feature = "LuAlignHorizontalSpaceBetween")]
            LuIcon::LuAlignHorizontalSpaceBetween => LU_ALIGN_HORIZONTAL_SPACE_BETWEEN,
            #[cfg(feature = "LuAlignJustify")]
            LuIcon::LuAlignJustify => LU_ALIGN_JUSTIFY,
            #[cfg(feature = "LuAlignLeft")]
            LuIcon::LuAlignLeft => LU_ALIGN_LEFT,
            #[cfg(feature = "LuAlignRight")]
            LuIcon::LuAlignRight => LU_ALIGN_RIGHT,
            #[cfg(feature = "LuAlignStartHorizontal")]
            LuIcon::LuAlignStartHorizontal => LU_ALIGN_START_HORIZONTAL,
            #[cfg(feature = "LuAlignStartVertical")]
            LuIcon::LuAlignStartVertical => LU_ALIGN_START_VERTICAL,
            #[cfg(feature = "LuAlignVerticalDistributeCenter")]
            LuIcon::LuAlignVerticalDistributeCenter => LU_ALIGN_VERTICAL_DISTRIBUTE_CENTER,
            #[cfg(feature = "LuAlignVerticalDistributeEnd")]
            LuIcon::LuAlignVerticalDistributeEnd => LU_ALIGN_VERTICAL_DISTRIBUTE_END,
            #[cfg(feature = "LuAlignVerticalDistributeStart")]
            LuIcon::LuAlignVerticalDistributeStart => LU_ALIGN_VERTICAL_DISTRIBUTE_START,
            #[cfg(feature = "LuAlignVerticalJustifyCenter")]
            LuIcon::LuAlignVerticalJustifyCenter => LU_ALIGN_VERTICAL_JUSTIFY_CENTER,
            #[cfg(feature = "LuAlignVerticalJustifyEnd")]
            LuIcon::LuAlignVerticalJustifyEnd => LU_ALIGN_VERTICAL_JUSTIFY_END,
            #[cfg(feature = "LuAlignVerticalJustifyStart")]
            LuIcon::LuAlignVerticalJustifyStart => LU_ALIGN_VERTICAL_JUSTIFY_START,
            #[cfg(feature = "LuAlignVerticalSpaceAround")]
            LuIcon::LuAlignVerticalSpaceAround => LU_ALIGN_VERTICAL_SPACE_AROUND,
            #[cfg(feature = "LuAlignVerticalSpaceBetween")]
            LuIcon::LuAlignVerticalSpaceBetween => LU_ALIGN_VERTICAL_SPACE_BETWEEN,
            #[cfg(feature = "LuAmpersand")]
            LuIcon::LuAmpersand => LU_AMPERSAND,
            #[cfg(feature = "LuAmpersands")]
            LuIcon::LuAmpersands => LU_AMPERSANDS,
            #[cfg(feature = "LuAnchor")]
            LuIcon::LuAnchor => LU_ANCHOR,
            #[cfg(feature = "LuAngry")]
            LuIcon::LuAngry => LU_ANGRY,
            #[cfg(feature = "LuAnnoyed")]
            LuIcon::LuAnnoyed => LU_ANNOYED,
            #[cfg(feature = "LuAntenna")]
            LuIcon::LuAntenna => LU_ANTENNA,
            #[cfg(feature = "LuAperture")]
            LuIcon::LuAperture => LU_APERTURE,
            #[cfg(feature = "LuAppWindow")]
            LuIcon::LuAppWindow => LU_APP_WINDOW,
            #[cfg(feature = "LuApple")]
            LuIcon::LuApple => LU_APPLE,
            #[cfg(feature = "LuArchive")]
            LuIcon::LuArchive => LU_ARCHIVE,
            #[cfg(feature = "LuArchiveRestore")]
            LuIcon::LuArchiveRestore => LU_ARCHIVE_RESTORE,
            #[cfg(feature = "LuAreaChart")]
            LuIcon::LuAreaChart => LU_AREA_CHART,
            #[cfg(feature = "LuArmchair")]
            LuIcon::LuArmchair => LU_ARMCHAIR,
            #[cfg(feature = "LuArrowBigDown")]
            LuIcon::LuArrowBigDown => LU_ARROW_BIG_DOWN,
            #[cfg(feature = "LuArrowBigDownDash")]
            LuIcon::LuArrowBigDownDash => LU_ARROW_BIG_DOWN_DASH,
            #[cfg(feature = "LuArrowBigLeft")]
            LuIcon::LuArrowBigLeft => LU_ARROW_BIG_LEFT,
            #[cfg(feature = "LuArrowBigLeftDash")]
            LuIcon::LuArrowBigLeftDash => LU_ARROW_BIG_LEFT_DASH,
            #[cfg(feature = "LuArrowBigRight")]
            LuIcon::LuArrowBigRight => LU_ARROW_BIG_RIGHT,
            #[cfg(feature = "LuArrowBigRightDash")]
            LuIcon::LuArrowBigRightDash => LU_ARROW_BIG_RIGHT_DASH,
            #[cfg(feature = "LuArrowBigUp")]
            LuIcon::LuArrowBigUp => LU_ARROW_BIG_UP,
            #[cfg(feature = "LuArrowBigUpDash")]
            LuIcon::LuArrowBigUpDash => LU_ARROW_BIG_UP_DASH,
            #[cfg(feature = "LuArrowDown")]
            LuIcon::LuArrowDown => LU_ARROW_DOWN,
            #[cfg(feature = "LuArrowDown01")]
            LuIcon::LuArrowDown01 => LU_ARROW_DOWN01,
            #[cfg(feature = "LuArrowDown10")]
            LuIcon::LuArrowDown10 => LU_ARROW_DOWN10,
            #[cfg(feature = "LuArrowDownAZ")]
            LuIcon::LuArrowDownAZ => LU_ARROW_DOWN_AZ,
            #[cfg(feature = "LuArrowDownCircle")]
            LuIcon::LuArrowDownCircle => LU_ARROW_DOWN_CIRCLE,
            #[cfg(feature = "LuArrowDownFromLine")]
            LuIcon::LuArrowDownFromLine => LU_ARROW_DOWN_FROM_LINE,
            #[cfg(feature = "LuArrowDownLeft")]
            LuIcon::LuArrowDownLeft => LU_ARROW_DOWN_LEFT,
            #[cfg(feature = "LuArrowDownLeftFromCircle")]
            LuIcon::LuArrowDownLeftFromCircle => LU_ARROW_DOWN_LEFT_FROM_CIRCLE,
            #[cfg(feature = "LuArrowDownLeftSquare")]
            LuIcon::LuArrowDownLeftSquare => LU_ARROW_DOWN_LEFT_SQUARE,
            #[cfg(feature = "LuArrowDownNarrowWide")]
            LuIcon::LuArrowDownNarrowWide => LU_ARROW_DOWN_NARROW_WIDE,
            #[cfg(feature = "LuArrowDownRight")]
            LuIcon::LuArrowDownRight => LU_ARROW_DOWN_RIGHT,
            #[cfg(feature = "LuArrowDownRightFromCircle")]
            LuIcon::LuArrowDownRightFromCircle => LU_ARROW_DOWN_RIGHT_FROM_CIRCLE,
            #[cfg(feature = "LuArrowDownRightSquare")]
            LuIcon::LuArrowDownRightSquare => LU_ARROW_DOWN_RIGHT_SQUARE,
            #[cfg(feature = "LuArrowDownSquare")]
            LuIcon::LuArrowDownSquare => LU_ARROW_DOWN_SQUARE,
            #[cfg(feature = "LuArrowDownToDot")]
            LuIcon::LuArrowDownToDot => LU_ARROW_DOWN_TO_DOT,
            #[cfg(feature = "LuArrowDownToLine")]
            LuIcon::LuArrowDownToLine => LU_ARROW_DOWN_TO_LINE,
            #[cfg(feature = "LuArrowDownUp")]
            LuIcon::LuArrowDownUp => LU_ARROW_DOWN_UP,
            #[cfg(feature = "LuArrowDownWideNarrow")]
            LuIcon::LuArrowDownWideNarrow => LU_ARROW_DOWN_WIDE_NARROW,
            #[cfg(feature = "LuArrowDownZA")]
            LuIcon::LuArrowDownZA => LU_ARROW_DOWN_ZA,
            #[cfg(feature = "LuArrowLeft")]
            LuIcon::LuArrowLeft => LU_ARROW_LEFT,
            #[cfg(feature = "LuArrowLeftCircle")]
            LuIcon::LuArrowLeftCircle => LU_ARROW_LEFT_CIRCLE,
            #[cfg(feature = "LuArrowLeftFromLine")]
            LuIcon::LuArrowLeftFromLine => LU_ARROW_LEFT_FROM_LINE,
            #[cfg(feature = "LuArrowLeftRight")]
            LuIcon::LuArrowLeftRight => LU_ARROW_LEFT_RIGHT,
            #[cfg(feature = "LuArrowLeftSquare")]
            LuIcon::LuArrowLeftSquare => LU_ARROW_LEFT_SQUARE,
            #[cfg(feature = "LuArrowLeftToLine")]
            LuIcon::LuArrowLeftToLine => LU_ARROW_LEFT_TO_LINE,
            #[cfg(feature = "LuArrowRight")]
            LuIcon::LuArrowRight => LU_ARROW_RIGHT,
            #[cfg(feature = "LuArrowRightCircle")]
            LuIcon::LuArrowRightCircle => LU_ARROW_RIGHT_CIRCLE,
            #[cfg(feature = "LuArrowRightFromLine")]
            LuIcon::LuArrowRightFromLine => LU_ARROW_RIGHT_FROM_LINE,
            #[cfg(feature = "LuArrowRightLeft")]
            LuIcon::LuArrowRightLeft => LU_ARROW_RIGHT_LEFT,
            #[cfg(feature = "LuArrowRightSquare")]
            LuIcon::LuArrowRightSquare => LU_ARROW_RIGHT_SQUARE,
            #[cfg(feature = "LuArrowRightToLine")]
            LuIcon::LuArrowRightToLine => LU_ARROW_RIGHT_TO_LINE,
            #[cfg(feature = "LuArrowUp")]
            LuIcon::LuArrowUp => LU_ARROW_UP,
            #[cfg(feature = "LuArrowUp01")]
            LuIcon::LuArrowUp01 => LU_ARROW_UP01,
            #[cfg(feature = "LuArrowUp10")]
            LuIcon::LuArrowUp10 => LU_ARROW_UP10,
            #[cfg(feature = "LuArrowUpAZ")]
            LuIcon::LuArrowUpAZ => LU_ARROW_UP_AZ,
            #[cfg(feature = "LuArrowUpCircle")]
            LuIcon::LuArrowUpCircle => LU_ARROW_UP_CIRCLE,
            #[cfg(feature = "LuArrowUpDown")]
            LuIcon::LuArrowUpDown => LU_ARROW_UP_DOWN,
            #[cfg(feature = "LuArrowUpFromDot")]
            LuIcon::LuArrowUpFromDot => LU_ARROW_UP_FROM_DOT,
            #[cfg(feature = "LuArrowUpFromLine")]
            LuIcon::LuArrowUpFromLine => LU_ARROW_UP_FROM_LINE,
            #[cfg(feature = "LuArrowUpLeft")]
            LuIcon::LuArrowUpLeft => LU_ARROW_UP_LEFT,
            #[cfg(feature = "LuArrowUpLeftFromCircle")]
            LuIcon::LuArrowUpLeftFromCircle => LU_ARROW_UP_LEFT_FROM_CIRCLE,
            #[cfg(feature = "LuArrowUpLeftSquare")]
            LuIcon::LuArrowUpLeftSquare => LU_ARROW_UP_LEFT_SQUARE,
            #[cfg(feature = "LuArrowUpNarrowWide")]
            LuIcon::LuArrowUpNarrowWide => LU_ARROW_UP_NARROW_WIDE,
            #[cfg(feature = "LuArrowUpRight")]
            LuIcon::LuArrowUpRight => LU_ARROW_UP_RIGHT,
            #[cfg(feature = "LuArrowUpRightFromCircle")]
            LuIcon::LuArrowUpRightFromCircle => LU_ARROW_UP_RIGHT_FROM_CIRCLE,
            #[cfg(feature = "LuArrowUpRightSquare")]
            LuIcon::LuArrowUpRightSquare => LU_ARROW_UP_RIGHT_SQUARE,
            #[cfg(feature = "LuArrowUpSquare")]
            LuIcon::LuArrowUpSquare => LU_ARROW_UP_SQUARE,
            #[cfg(feature = "LuArrowUpToLine")]
            LuIcon::LuArrowUpToLine => LU_ARROW_UP_TO_LINE,
            #[cfg(feature = "LuArrowUpWideNarrow")]
            LuIcon::LuArrowUpWideNarrow => LU_ARROW_UP_WIDE_NARROW,
            #[cfg(feature = "LuArrowUpZA")]
            LuIcon::LuArrowUpZA => LU_ARROW_UP_ZA,
            #[cfg(feature = "LuArrowsUpFromLine")]
            LuIcon::LuArrowsUpFromLine => LU_ARROWS_UP_FROM_LINE,
            #[cfg(feature = "LuAsterisk")]
            LuIcon::LuAsterisk => LU_ASTERISK,
            #[cfg(feature = "LuAtSign")]
            LuIcon::LuAtSign => LU_AT_SIGN,
            #[cfg(feature = "LuAtom")]
            LuIcon::LuAtom => LU_ATOM,
            #[cfg(feature = "LuAward")]
            LuIcon::LuAward => LU_AWARD,
            #[cfg(feature = "LuAxe")]
            LuIcon::LuAxe => LU_AXE,
            #[cfg(feature = "LuAxis3d")]
            LuIcon::LuAxis3d => LU_AXIS3D,
            #[cfg(feature = "LuBaby")]
            LuIcon::LuBaby => LU_BABY,
            #[cfg(feature = "LuBackpack")]
            LuIcon::LuBackpack => LU_BACKPACK,
            #[cfg(feature = "LuBadge")]
            LuIcon::LuBadge => LU_BADGE,
            #[cfg(feature = "LuBadgeAlert")]
            LuIcon::LuBadgeAlert => LU_BADGE_ALERT,
            #[cfg(feature = "LuBadgeCheck")]
            LuIcon::LuBadgeCheck => LU_BADGE_CHECK,
            #[cfg(feature = "LuBadgeDollarSign")]
            LuIcon::LuBadgeDollarSign => LU_BADGE_DOLLAR_SIGN,
            #[cfg(feature = "LuBadgeHelp")]
            LuIcon::LuBadgeHelp => LU_BADGE_HELP,
            #[cfg(feature = "LuBadgeInfo")]
            LuIcon::LuBadgeInfo => LU_BADGE_INFO,
            #[cfg(feature = "LuBadgeMinus")]
            LuIcon::LuBadgeMinus => LU_BADGE_MINUS,
            #[cfg(feature = "LuBadgePercent")]
            LuIcon::LuBadgePercent => LU_BADGE_PERCENT,
            #[cfg(feature = "LuBadgePlus")]
            LuIcon::LuBadgePlus => LU_BADGE_PLUS,
            #[cfg(feature = "LuBadgeX")]
            LuIcon::LuBadgeX => LU_BADGE_X,
            #[cfg(feature = "LuBaggageClaim")]
            LuIcon::LuBaggageClaim => LU_BAGGAGE_CLAIM,
            #[cfg(feature = "LuBan")]
            LuIcon::LuBan => LU_BAN,
            #[cfg(feature = "LuBanana")]
            LuIcon::LuBanana => LU_BANANA,
            #[cfg(feature = "LuBanknote")]
            LuIcon::LuBanknote => LU_BANKNOTE,
            #[cfg(feature = "LuBarChart")]
            LuIcon::LuBarChart => LU_BAR_CHART,
            #[cfg(feature = "LuBarChart2")]
            LuIcon::LuBarChart2 => LU_BAR_CHART2,
            #[cfg(feature = "LuBarChart3")]
            LuIcon::LuBarChart3 => LU_BAR_CHART3,
            #[cfg(feature = "LuBarChart4")]
            LuIcon::LuBarChart4 => LU_BAR_CHART4,
            #[cfg(feature = "LuBarChartBig")]
            LuIcon::LuBarChartBig => LU_BAR_CHART_BIG,
            #[cfg(feature = "LuBarChartHorizontal")]
            LuIcon::LuBarChartHorizontal => LU_BAR_CHART_HORIZONTAL,
            #[cfg(feature = "LuBarChartHorizontalBig")]
            LuIcon::LuBarChartHorizontalBig => LU_BAR_CHART_HORIZONTAL_BIG,
            #[cfg(feature = "LuBaseline")]
            LuIcon::LuBaseline => LU_BASELINE,
            #[cfg(feature = "LuBath")]
            LuIcon::LuBath => LU_BATH,
            #[cfg(feature = "LuBattery")]
            LuIcon::LuBattery => LU_BATTERY,
            #[cfg(feature = "LuBatteryCharging")]
            LuIcon::LuBatteryCharging => LU_BATTERY_CHARGING,
            #[cfg(feature = "LuBatteryFull")]
            LuIcon::LuBatteryFull => LU_BATTERY_FULL,
            #[cfg(feature = "LuBatteryLow")]
            LuIcon::LuBatteryLow => LU_BATTERY_LOW,
            #[cfg(feature = "LuBatteryMedium")]
            LuIcon::LuBatteryMedium => LU_BATTERY_MEDIUM,
            #[cfg(feature = "LuBatteryWarning")]
            LuIcon::LuBatteryWarning => LU_BATTERY_WARNING,
            #[cfg(feature = "LuBeaker")]
            LuIcon::LuBeaker => LU_BEAKER,
            #[cfg(feature = "LuBean")]
            LuIcon::LuBean => LU_BEAN,
            #[cfg(feature = "LuBeanOff")]
            LuIcon::LuBeanOff => LU_BEAN_OFF,
            #[cfg(feature = "LuBed")]
            LuIcon::LuBed => LU_BED,
            #[cfg(feature = "LuBedDouble")]
            LuIcon::LuBedDouble => LU_BED_DOUBLE,
            #[cfg(feature = "LuBedSingle")]
            LuIcon::LuBedSingle => LU_BED_SINGLE,
            #[cfg(feature = "LuBeef")]
            LuIcon::LuBeef => LU_BEEF,
            #[cfg(feature = "LuBeer")]
            LuIcon::LuBeer => LU_BEER,
            #[cfg(feature = "LuBell")]
            LuIcon::LuBell => LU_BELL,
            #[cfg(feature = "LuBellDot")]
            LuIcon::LuBellDot => LU_BELL_DOT,
            #[cfg(feature = "LuBellMinus")]
            LuIcon::LuBellMinus => LU_BELL_MINUS,
            #[cfg(feature = "LuBellOff")]
            LuIcon::LuBellOff => LU_BELL_OFF,
            #[cfg(feature = "LuBellPlus")]
            LuIcon::LuBellPlus => LU_BELL_PLUS,
            #[cfg(feature = "LuBellRing")]
            LuIcon::LuBellRing => LU_BELL_RING,
            #[cfg(feature = "LuBike")]
            LuIcon::LuBike => LU_BIKE,
            #[cfg(feature = "LuBinary")]
            LuIcon::LuBinary => LU_BINARY,
            #[cfg(feature = "LuBiohazard")]
            LuIcon::LuBiohazard => LU_BIOHAZARD,
            #[cfg(feature = "LuBird")]
            LuIcon::LuBird => LU_BIRD,
            #[cfg(feature = "LuBitcoin")]
            LuIcon::LuBitcoin => LU_BITCOIN,
            #[cfg(feature = "LuBlinds")]
            LuIcon::LuBlinds => LU_BLINDS,
            #[cfg(feature = "LuBlocks")]
            LuIcon::LuBlocks => LU_BLOCKS,
            #[cfg(feature = "LuBluetooth")]
            LuIcon::LuBluetooth => LU_BLUETOOTH,
            #[cfg(feature = "LuBluetoothConnected")]
            LuIcon::LuBluetoothConnected => LU_BLUETOOTH_CONNECTED,
            #[cfg(feature = "LuBluetoothOff")]
            LuIcon::LuBluetoothOff => LU_BLUETOOTH_OFF,
            #[cfg(feature = "LuBluetoothSearching")]
            LuIcon::LuBluetoothSearching => LU_BLUETOOTH_SEARCHING,
            #[cfg(feature = "LuBold")]
            LuIcon::LuBold => LU_BOLD,
            #[cfg(feature = "LuBomb")]
            LuIcon::LuBomb => LU_BOMB,
            #[cfg(feature = "LuBone")]
            LuIcon::LuBone => LU_BONE,
            #[cfg(feature = "LuBook")]
            LuIcon::LuBook => LU_BOOK,
            #[cfg(feature = "LuBookCopy")]
            LuIcon::LuBookCopy => LU_BOOK_COPY,
            #[cfg(feature = "LuBookDown")]
            LuIcon::LuBookDown => LU_BOOK_DOWN,
            #[cfg(feature = "LuBookKey")]
            LuIcon::LuBookKey => LU_BOOK_KEY,
            #[cfg(feature = "LuBookLock")]
            LuIcon::LuBookLock => LU_BOOK_LOCK,
            #[cfg(feature = "LuBookMarked")]
            LuIcon::LuBookMarked => LU_BOOK_MARKED,
            #[cfg(feature = "LuBookMinus")]
            LuIcon::LuBookMinus => LU_BOOK_MINUS,
            #[cfg(feature = "LuBookOpen")]
            LuIcon::LuBookOpen => LU_BOOK_OPEN,
            #[cfg(feature = "LuBookOpenCheck")]
            LuIcon::LuBookOpenCheck => LU_BOOK_OPEN_CHECK,
            #[cfg(feature = "LuBookPlus")]
            LuIcon::LuBookPlus => LU_BOOK_PLUS,
            #[cfg(feature = "LuBookTemplate")]
            LuIcon::LuBookTemplate => LU_BOOK_TEMPLATE,
            #[cfg(feature = "LuBookUp")]
            LuIcon::LuBookUp => LU_BOOK_UP,
            #[cfg(feature = "LuBookUp2")]
            LuIcon::LuBookUp2 => LU_BOOK_UP2,
            #[cfg(feature = "LuBookX")]
            LuIcon::LuBookX => LU_BOOK_X,
            #[cfg(feature = "LuBookmark")]
            LuIcon::LuBookmark => LU_BOOKMARK,
            #[cfg(feature = "LuBookmarkMinus")]
            LuIcon::LuBookmarkMinus => LU_BOOKMARK_MINUS,
            #[cfg(feature = "LuBookmarkPlus")]
            LuIcon::LuBookmarkPlus => LU_BOOKMARK_PLUS,
            #[cfg(feature = "LuBoomBox")]
            LuIcon::LuBoomBox => LU_BOOM_BOX,
            #[cfg(feature = "LuBot")]
            LuIcon::LuBot => LU_BOT,
            #[cfg(feature = "LuBox")]
            LuIcon::LuBox => LU_BOX,
            #[cfg(feature = "LuBoxSelect")]
            LuIcon::LuBoxSelect => LU_BOX_SELECT,
            #[cfg(feature = "LuBoxes")]
            LuIcon::LuBoxes => LU_BOXES,
            #[cfg(feature = "LuBraces")]
            LuIcon::LuBraces => LU_BRACES,
            #[cfg(feature = "LuBrackets")]
            LuIcon::LuBrackets => LU_BRACKETS,
            #[cfg(feature = "LuBrain")]
            LuIcon::LuBrain => LU_BRAIN,
            #[cfg(feature = "LuBrainCircuit")]
            LuIcon::LuBrainCircuit => LU_BRAIN_CIRCUIT,
            #[cfg(feature = "LuBrainCog")]
            LuIcon::LuBrainCog => LU_BRAIN_COG,
            #[cfg(feature = "LuBriefcase")]
            LuIcon::LuBriefcase => LU_BRIEFCASE,
            #[cfg(feature = "LuBringToFront")]
            LuIcon::LuBringToFront => LU_BRING_TO_FRONT,
            #[cfg(feature = "LuBrush")]
            LuIcon::LuBrush => LU_BRUSH,
            #[cfg(feature = "LuBug")]
            LuIcon::LuBug => LU_BUG,
            #[cfg(feature = "LuBuilding")]
            LuIcon::LuBuilding => LU_BUILDING,
            #[cfg(feature = "LuBuilding2")]
            LuIcon::LuBuilding2 => LU_BUILDING2,
            #[cfg(feature = "LuBus")]
            LuIcon::LuBus => LU_BUS,
            #[cfg(feature = "LuBusFront")]
            LuIcon::LuBusFront => LU_BUS_FRONT,
            #[cfg(feature = "LuCable")]
            LuIcon::LuCable => LU_CABLE,
            #[cfg(feature = "LuCableCar")]
            LuIcon::LuCableCar => LU_CABLE_CAR,
            #[cfg(feature = "LuCake")]
            LuIcon::LuCake => LU_CAKE,
            #[cfg(feature = "LuCakeSlice")]
            LuIcon::LuCakeSlice => LU_CAKE_SLICE,
            #[cfg(feature = "LuCalculator")]
            LuIcon::LuCalculator => LU_CALCULATOR,
            #[cfg(feature = "LuCalendar")]
            LuIcon::LuCalendar => LU_CALENDAR,
            #[cfg(feature = "LuCalendarCheck")]
            LuIcon::LuCalendarCheck => LU_CALENDAR_CHECK,
            #[cfg(feature = "LuCalendarCheck2")]
            LuIcon::LuCalendarCheck2 => LU_CALENDAR_CHECK2,
            #[cfg(feature = "LuCalendarClock")]
            LuIcon::LuCalendarClock => LU_CALENDAR_CLOCK,
            #[cfg(feature = "LuCalendarDays")]
            LuIcon::LuCalendarDays => LU_CALENDAR_DAYS,
            #[cfg(feature = "LuCalendarHeart")]
            LuIcon::LuCalendarHeart => LU_CALENDAR_HEART,
            #[cfg(feature = "LuCalendarMinus")]
            LuIcon::LuCalendarMinus => LU_CALENDAR_MINUS,
            #[cfg(feature = "LuCalendarOff")]
            LuIcon::LuCalendarOff => LU_CALENDAR_OFF,
            #[cfg(feature = "LuCalendarPlus")]
            LuIcon::LuCalendarPlus => LU_CALENDAR_PLUS,
            #[cfg(feature = "LuCalendarRange")]
            LuIcon::LuCalendarRange => LU_CALENDAR_RANGE,
            #[cfg(feature = "LuCalendarSearch")]
            LuIcon::LuCalendarSearch => LU_CALENDAR_SEARCH,
            #[cfg(feature = "LuCalendarX")]
            LuIcon::LuCalendarX => LU_CALENDAR_X,
            #[cfg(feature = "LuCalendarX2")]
            LuIcon::LuCalendarX2 => LU_CALENDAR_X2,
            #[cfg(feature = "LuCamera")]
            LuIcon::LuCamera => LU_CAMERA,
            #[cfg(feature = "LuCameraOff")]
            LuIcon::LuCameraOff => LU_CAMERA_OFF,
            #[cfg(feature = "LuCandlestickChart")]
            LuIcon::LuCandlestickChart => LU_CANDLESTICK_CHART,
            #[cfg(feature = "LuCandy")]
            LuIcon::LuCandy => LU_CANDY,
            #[cfg(feature = "LuCandyCane")]
            LuIcon::LuCandyCane => LU_CANDY_CANE,
            #[cfg(feature = "LuCandyOff")]
            LuIcon::LuCandyOff => LU_CANDY_OFF,
            #[cfg(feature = "LuCar")]
            LuIcon::LuCar => LU_CAR,
            #[cfg(feature = "LuCarFront")]
            LuIcon::LuCarFront => LU_CAR_FRONT,
            #[cfg(feature = "LuCarTaxiFront")]
            LuIcon::LuCarTaxiFront => LU_CAR_TAXI_FRONT,
            #[cfg(feature = "LuCarrot")]
            LuIcon::LuCarrot => LU_CARROT,
            #[cfg(feature = "LuCaseLower")]
            LuIcon::LuCaseLower => LU_CASE_LOWER,
            #[cfg(feature = "LuCaseSensitive")]
            LuIcon::LuCaseSensitive => LU_CASE_SENSITIVE,
            #[cfg(feature = "LuCaseUpper")]
            LuIcon::LuCaseUpper => LU_CASE_UPPER,
            #[cfg(feature = "LuCassetteTape")]
            LuIcon::LuCassetteTape => LU_CASSETTE_TAPE,
            #[cfg(feature = "LuCast")]
            LuIcon::LuCast => LU_CAST,
            #[cfg(feature = "LuCastle")]
            LuIcon::LuCastle => LU_CASTLE,
            #[cfg(feature = "LuCat")]
            LuIcon::LuCat => LU_CAT,
            #[cfg(feature = "LuCheck")]
            LuIcon::LuCheck => LU_CHECK,
            #[cfg(feature = "LuCheckCheck")]
            LuIcon::LuCheckCheck => LU_CHECK_CHECK,
            #[cfg(feature = "LuCheckCircle")]
            LuIcon::LuCheckCircle => LU_CHECK_CIRCLE,
            #[cfg(feature = "LuCheckCircle2")]
            LuIcon::LuCheckCircle2 => LU_CHECK_CIRCLE2,
            #[cfg(feature = "LuCheckSquare")]
            LuIcon::LuCheckSquare => LU_CHECK_SQUARE,
            #[cfg(feature = "LuChefHat")]
            LuIcon::LuChefHat => LU_CHEF_HAT,
            #[cfg(feature = "LuCherry")]
            LuIcon::LuCherry => LU_CHERRY,
            #[cfg(feature = "LuChevronDown")]
            LuIcon::LuChevronDown => LU_CHEVRON_DOWN,
            #[cfg(feature = "LuChevronDownCircle")]
            LuIcon::LuChevronDownCircle => LU_CHEVRON_DOWN_CIRCLE,
            #[cfg(feature = "LuChevronDownSquare")]
            LuIcon::LuChevronDownSquare => LU_CHEVRON_DOWN_SQUARE,
            #[cfg(feature = "LuChevronFirst")]
            LuIcon::LuChevronFirst => LU_CHEVRON_FIRST,
            #[cfg(feature = "LuChevronLast")]
            LuIcon::LuChevronLast => LU_CHEVRON_LAST,
            #[cfg(feature = "LuChevronLeft")]
            LuIcon::LuChevronLeft => LU_CHEVRON_LEFT,
            #[cfg(feature = "LuChevronLeftCircle")]
            LuIcon::LuChevronLeftCircle => LU_CHEVRON_LEFT_CIRCLE,
            #[cfg(feature = "LuChevronLeftSquare")]
            LuIcon::LuChevronLeftSquare => LU_CHEVRON_LEFT_SQUARE,
            #[cfg(feature = "LuChevronRight")]
            LuIcon::LuChevronRight => LU_CHEVRON_RIGHT,
            #[cfg(feature = "LuChevronRightCircle")]
            LuIcon::LuChevronRightCircle => LU_CHEVRON_RIGHT_CIRCLE,
            #[cfg(feature = "LuChevronRightSquare")]
            LuIcon::LuChevronRightSquare => LU_CHEVRON_RIGHT_SQUARE,
            #[cfg(feature = "LuChevronUp")]
            LuIcon::LuChevronUp => LU_CHEVRON_UP,
            #[cfg(feature = "LuChevronUpCircle")]
            LuIcon::LuChevronUpCircle => LU_CHEVRON_UP_CIRCLE,
            #[cfg(feature = "LuChevronUpSquare")]
            LuIcon::LuChevronUpSquare => LU_CHEVRON_UP_SQUARE,
            #[cfg(feature = "LuChevronsDown")]
            LuIcon::LuChevronsDown => LU_CHEVRONS_DOWN,
            #[cfg(feature = "LuChevronsDownUp")]
            LuIcon::LuChevronsDownUp => LU_CHEVRONS_DOWN_UP,
            #[cfg(feature = "LuChevronsLeft")]
            LuIcon::LuChevronsLeft => LU_CHEVRONS_LEFT,
            #[cfg(feature = "LuChevronsLeftRight")]
            LuIcon::LuChevronsLeftRight => LU_CHEVRONS_LEFT_RIGHT,
            #[cfg(feature = "LuChevronsRight")]
            LuIcon::LuChevronsRight => LU_CHEVRONS_RIGHT,
            #[cfg(feature = "LuChevronsRightLeft")]
            LuIcon::LuChevronsRightLeft => LU_CHEVRONS_RIGHT_LEFT,
            #[cfg(feature = "LuChevronsUp")]
            LuIcon::LuChevronsUp => LU_CHEVRONS_UP,
            #[cfg(feature = "LuChevronsUpDown")]
            LuIcon::LuChevronsUpDown => LU_CHEVRONS_UP_DOWN,
            #[cfg(feature = "LuChrome")]
            LuIcon::LuChrome => LU_CHROME,
            #[cfg(feature = "LuChurch")]
            LuIcon::LuChurch => LU_CHURCH,
            #[cfg(feature = "LuCigarette")]
            LuIcon::LuCigarette => LU_CIGARETTE,
            #[cfg(feature = "LuCigaretteOff")]
            LuIcon::LuCigaretteOff => LU_CIGARETTE_OFF,
            #[cfg(feature = "LuCircle")]
            LuIcon::LuCircle => LU_CIRCLE,
            #[cfg(feature = "LuCircleDashed")]
            LuIcon::LuCircleDashed => LU_CIRCLE_DASHED,
            #[cfg(feature = "LuCircleDollarSign")]
            LuIcon::LuCircleDollarSign => LU_CIRCLE_DOLLAR_SIGN,
            #[cfg(feature = "LuCircleDot")]
            LuIcon::LuCircleDot => LU_CIRCLE_DOT,
            #[cfg(feature = "LuCircleDotDashed")]
            LuIcon::LuCircleDotDashed => LU_CIRCLE_DOT_DASHED,
            #[cfg(feature = "LuCircleEllipsis")]
            LuIcon::LuCircleEllipsis => LU_CIRCLE_ELLIPSIS,
            #[cfg(feature = "LuCircleEqual")]
            LuIcon::LuCircleEqual => LU_CIRCLE_EQUAL,
            #[cfg(feature = "LuCircleOff")]
            LuIcon::LuCircleOff => LU_CIRCLE_OFF,
            #[cfg(feature = "LuCircleSlash")]
            LuIcon::LuCircleSlash => LU_CIRCLE_SLASH,
            #[cfg(feature = "LuCircleSlash2")]
            LuIcon::LuCircleSlash2 => LU_CIRCLE_SLASH2,
            #[cfg(feature = "LuCircuitBoard")]
            LuIcon::LuCircuitBoard => LU_CIRCUIT_BOARD,
            #[cfg(feature = "LuCitrus")]
            LuIcon::LuCitrus => LU_CITRUS,
            #[cfg(feature = "LuClapperboard")]
            LuIcon::LuClapperboard => LU_CLAPPERBOARD,
            #[cfg(feature = "LuClipboard")]
            LuIcon::LuClipboard => LU_CLIPBOARD,
            #[cfg(feature = "LuClipboardCheck")]
            LuIcon::LuClipboardCheck => LU_CLIPBOARD_CHECK,
            #[cfg(feature = "LuClipboardCopy")]
            LuIcon::LuClipboardCopy => LU_CLIPBOARD_COPY,
            #[cfg(feature = "LuClipboardEdit")]
            LuIcon::LuClipboardEdit => LU_CLIPBOARD_EDIT,
            #[cfg(feature = "LuClipboardList")]
            LuIcon::LuClipboardList => LU_CLIPBOARD_LIST,
            #[cfg(feature = "LuClipboardPaste")]
            LuIcon::LuClipboardPaste => LU_CLIPBOARD_PASTE,
            #[cfg(feature = "LuClipboardSignature")]
            LuIcon::LuClipboardSignature => LU_CLIPBOARD_SIGNATURE,
            #[cfg(feature = "LuClipboardType")]
            LuIcon::LuClipboardType => LU_CLIPBOARD_TYPE,
            #[cfg(feature = "LuClipboardX")]
            LuIcon::LuClipboardX => LU_CLIPBOARD_X,
            #[cfg(feature = "LuClock")]
            LuIcon::LuClock => LU_CLOCK,
            #[cfg(feature = "LuClock1")]
            LuIcon::LuClock1 => LU_CLOCK1,
            #[cfg(feature = "LuClock10")]
            LuIcon::LuClock10 => LU_CLOCK10,
            #[cfg(feature = "LuClock11")]
            LuIcon::LuClock11 => LU_CLOCK11,
            #[cfg(feature = "LuClock12")]
            LuIcon::LuClock12 => LU_CLOCK12,
            #[cfg(feature = "LuClock2")]
            LuIcon::LuClock2 => LU_CLOCK2,
            #[cfg(feature = "LuClock3")]
            LuIcon::LuClock3 => LU_CLOCK3,
            #[cfg(feature = "LuClock4")]
            LuIcon::LuClock4 => LU_CLOCK4,
            #[cfg(feature = "LuClock5")]
            LuIcon::LuClock5 => LU_CLOCK5,
            #[cfg(feature = "LuClock6")]
            LuIcon::LuClock6 => LU_CLOCK6,
            #[cfg(feature = "LuClock7")]
            LuIcon::LuClock7 => LU_CLOCK7,
            #[cfg(feature = "LuClock8")]
            LuIcon::LuClock8 => LU_CLOCK8,
            #[cfg(feature = "LuClock9")]
            LuIcon::LuClock9 => LU_CLOCK9,
            #[cfg(feature = "LuCloud")]
            LuIcon::LuCloud => LU_CLOUD,
            #[cfg(feature = "LuCloudCog")]
            LuIcon::LuCloudCog => LU_CLOUD_COG,
            #[cfg(feature = "LuCloudDrizzle")]
            LuIcon::LuCloudDrizzle => LU_CLOUD_DRIZZLE,
            #[cfg(feature = "LuCloudFog")]
            LuIcon::LuCloudFog => LU_CLOUD_FOG,
            #[cfg(feature = "LuCloudHail")]
            LuIcon::LuCloudHail => LU_CLOUD_HAIL,
            #[cfg(feature = "LuCloudLightning")]
            LuIcon::LuCloudLightning => LU_CLOUD_LIGHTNING,
            #[cfg(feature = "LuCloudMoon")]
            LuIcon::LuCloudMoon => LU_CLOUD_MOON,
            #[cfg(feature = "LuCloudMoonRain")]
            LuIcon::LuCloudMoonRain => LU_CLOUD_MOON_RAIN,
            #[cfg(feature = "LuCloudOff")]
            LuIcon::LuCloudOff => LU_CLOUD_OFF,
            #[cfg(feature = "LuCloudRain")]
            LuIcon::LuCloudRain => LU_CLOUD_RAIN,
            #[cfg(feature = "LuCloudRainWind")]
            LuIcon::LuCloudRainWind => LU_CLOUD_RAIN_WIND,
            #[cfg(feature = "LuCloudSnow")]
            LuIcon::LuCloudSnow => LU_CLOUD_SNOW,
            #[cfg(feature = "LuCloudSun")]
            LuIcon::LuCloudSun => LU_CLOUD_SUN,
            #[cfg(feature = "LuCloudSunRain")]
            LuIcon::LuCloudSunRain => LU_CLOUD_SUN_RAIN,
            #[cfg(feature = "LuCloudy")]
            LuIcon::LuCloudy => LU_CLOUDY,
            #[cfg(feature = "LuClover")]
            LuIcon::LuClover => LU_CLOVER,
            #[cfg(feature = "LuClub")]
            LuIcon::LuClub => LU_CLUB,
            #[cfg(feature = "LuCode")]
            LuIcon::LuCode => LU_CODE,
            #[cfg(feature = "LuCode2")]
            LuIcon::LuCode2 => LU_CODE2,
            #[cfg(feature = "LuCodepen")]
            LuIcon::LuCodepen => LU_CODEPEN,
            #[cfg(feature = "LuCodesandbox")]
            LuIcon::LuCodesandbox => LU_CODESANDBOX,
            #[cfg(feature = "LuCoffee")]
            LuIcon::LuCoffee => LU_COFFEE,
            #[cfg(feature = "LuCog")]
            LuIcon::LuCog => LU_COG,
            #[cfg(feature = "LuCoins")]
            LuIcon::LuCoins => LU_COINS,
            #[cfg(feature = "LuColumns")]
            LuIcon::LuColumns => LU_COLUMNS,
            #[cfg(feature = "LuCombine")]
            LuIcon::LuCombine => LU_COMBINE,
            #[cfg(feature = "LuCommand")]
            LuIcon::LuCommand => LU_COMMAND,
            #[cfg(feature = "LuCompass")]
            LuIcon::LuCompass => LU_COMPASS,
            #[cfg(feature = "LuComponent")]
            LuIcon::LuComponent => LU_COMPONENT,
            #[cfg(feature = "LuComputer")]
            LuIcon::LuComputer => LU_COMPUTER,
            #[cfg(feature = "LuConciergeBell")]
            LuIcon::LuConciergeBell => LU_CONCIERGE_BELL,
            #[cfg(feature = "LuConstruction")]
            LuIcon::LuConstruction => LU_CONSTRUCTION,
            #[cfg(feature = "LuContact")]
            LuIcon::LuContact => LU_CONTACT,
            #[cfg(feature = "LuContact2")]
            LuIcon::LuContact2 => LU_CONTACT2,
            #[cfg(feature = "LuContainer")]
            LuIcon::LuContainer => LU_CONTAINER,
            #[cfg(feature = "LuContrast")]
            LuIcon::LuContrast => LU_CONTRAST,
            #[cfg(feature = "LuCookie")]
            LuIcon::LuCookie => LU_COOKIE,
            #[cfg(feature = "LuCopy")]
            LuIcon::LuCopy => LU_COPY,
            #[cfg(feature = "LuCopyCheck")]
            LuIcon::LuCopyCheck => LU_COPY_CHECK,
            #[cfg(feature = "LuCopyMinus")]
            LuIcon::LuCopyMinus => LU_COPY_MINUS,
            #[cfg(feature = "LuCopyPlus")]
            LuIcon::LuCopyPlus => LU_COPY_PLUS,
            #[cfg(feature = "LuCopySlash")]
            LuIcon::LuCopySlash => LU_COPY_SLASH,
            #[cfg(feature = "LuCopyX")]
            LuIcon::LuCopyX => LU_COPY_X,
            #[cfg(feature = "LuCopyleft")]
            LuIcon::LuCopyleft => LU_COPYLEFT,
            #[cfg(feature = "LuCopyright")]
            LuIcon::LuCopyright => LU_COPYRIGHT,
            #[cfg(feature = "LuCornerDownLeft")]
            LuIcon::LuCornerDownLeft => LU_CORNER_DOWN_LEFT,
            #[cfg(feature = "LuCornerDownRight")]
            LuIcon::LuCornerDownRight => LU_CORNER_DOWN_RIGHT,
            #[cfg(feature = "LuCornerLeftDown")]
            LuIcon::LuCornerLeftDown => LU_CORNER_LEFT_DOWN,
            #[cfg(feature = "LuCornerLeftUp")]
            LuIcon::LuCornerLeftUp => LU_CORNER_LEFT_UP,
            #[cfg(feature = "LuCornerRightDown")]
            LuIcon::LuCornerRightDown => LU_CORNER_RIGHT_DOWN,
            #[cfg(feature = "LuCornerRightUp")]
            LuIcon::LuCornerRightUp => LU_CORNER_RIGHT_UP,
            #[cfg(feature = "LuCornerUpLeft")]
            LuIcon::LuCornerUpLeft => LU_CORNER_UP_LEFT,
            #[cfg(feature = "LuCornerUpRight")]
            LuIcon::LuCornerUpRight => LU_CORNER_UP_RIGHT,
            #[cfg(feature = "LuCpu")]
            LuIcon::LuCpu => LU_CPU,
            #[cfg(feature = "LuCreativeCommons")]
            LuIcon::LuCreativeCommons => LU_CREATIVE_COMMONS,
            #[cfg(feature = "LuCreditCard")]
            LuIcon::LuCreditCard => LU_CREDIT_CARD,
            #[cfg(feature = "LuCroissant")]
            LuIcon::LuCroissant => LU_CROISSANT,
            #[cfg(feature = "LuCrop")]
            LuIcon::LuCrop => LU_CROP,
            #[cfg(feature = "LuCross")]
            LuIcon::LuCross => LU_CROSS,
            #[cfg(feature = "LuCrosshair")]
            LuIcon::LuCrosshair => LU_CROSSHAIR,
            #[cfg(feature = "LuCrown")]
            LuIcon::LuCrown => LU_CROWN,
            #[cfg(feature = "LuCupSoda")]
            LuIcon::LuCupSoda => LU_CUP_SODA,
            #[cfg(feature = "LuCurrency")]
            LuIcon::LuCurrency => LU_CURRENCY,
            #[cfg(feature = "LuDatabase")]
            LuIcon::LuDatabase => LU_DATABASE,
            #[cfg(feature = "LuDatabaseBackup")]
            LuIcon::LuDatabaseBackup => LU_DATABASE_BACKUP,
            #[cfg(feature = "LuDelete")]
            LuIcon::LuDelete => LU_DELETE,
            #[cfg(feature = "LuDessert")]
            LuIcon::LuDessert => LU_DESSERT,
            #[cfg(feature = "LuDiamond")]
            LuIcon::LuDiamond => LU_DIAMOND,
            #[cfg(feature = "LuDice1")]
            LuIcon::LuDice1 => LU_DICE1,
            #[cfg(feature = "LuDice2")]
            LuIcon::LuDice2 => LU_DICE2,
            #[cfg(feature = "LuDice3")]
            LuIcon::LuDice3 => LU_DICE3,
            #[cfg(feature = "LuDice4")]
            LuIcon::LuDice4 => LU_DICE4,
            #[cfg(feature = "LuDice5")]
            LuIcon::LuDice5 => LU_DICE5,
            #[cfg(feature = "LuDice6")]
            LuIcon::LuDice6 => LU_DICE6,
            #[cfg(feature = "LuDices")]
            LuIcon::LuDices => LU_DICES,
            #[cfg(feature = "LuDiff")]
            LuIcon::LuDiff => LU_DIFF,
            #[cfg(feature = "LuDisc")]
            LuIcon::LuDisc => LU_DISC,
            #[cfg(feature = "LuDisc2")]
            LuIcon::LuDisc2 => LU_DISC2,
            #[cfg(feature = "LuDisc3")]
            LuIcon::LuDisc3 => LU_DISC3,
            #[cfg(feature = "LuDivide")]
            LuIcon::LuDivide => LU_DIVIDE,
            #[cfg(feature = "LuDivideCircle")]
            LuIcon::LuDivideCircle => LU_DIVIDE_CIRCLE,
            #[cfg(feature = "LuDivideSquare")]
            LuIcon::LuDivideSquare => LU_DIVIDE_SQUARE,
            #[cfg(feature = "LuDna")]
            LuIcon::LuDna => LU_DNA,
            #[cfg(feature = "LuDnaOff")]
            LuIcon::LuDnaOff => LU_DNA_OFF,
            #[cfg(feature = "LuDog")]
            LuIcon::LuDog => LU_DOG,
            #[cfg(feature = "LuDollarSign")]
            LuIcon::LuDollarSign => LU_DOLLAR_SIGN,
            #[cfg(feature = "LuDonut")]
            LuIcon::LuDonut => LU_DONUT,
            #[cfg(feature = "LuDoorClosed")]
            LuIcon::LuDoorClosed => LU_DOOR_CLOSED,
            #[cfg(feature = "LuDoorOpen")]
            LuIcon::LuDoorOpen => LU_DOOR_OPEN,
            #[cfg(feature = "LuDot")]
            LuIcon::LuDot => LU_DOT,
            #[cfg(feature = "LuDownload")]
            LuIcon::LuDownload => LU_DOWNLOAD,
            #[cfg(feature = "LuDownloadCloud")]
            LuIcon::LuDownloadCloud => LU_DOWNLOAD_CLOUD,
            #[cfg(feature = "LuDribbble")]
            LuIcon::LuDribbble => LU_DRIBBBLE,
            #[cfg(feature = "LuDroplet")]
            LuIcon::LuDroplet => LU_DROPLET,
            #[cfg(feature = "LuDroplets")]
            LuIcon::LuDroplets => LU_DROPLETS,
            #[cfg(feature = "LuDrumstick")]
            LuIcon::LuDrumstick => LU_DRUMSTICK,
            #[cfg(feature = "LuDumbbell")]
            LuIcon::LuDumbbell => LU_DUMBBELL,
            #[cfg(feature = "LuEar")]
            LuIcon::LuEar => LU_EAR,
            #[cfg(feature = "LuEarOff")]
            LuIcon::LuEarOff => LU_EAR_OFF,
            #[cfg(feature = "LuEgg")]
            LuIcon::LuEgg => LU_EGG,
            #[cfg(feature = "LuEggFried")]
            LuIcon::LuEggFried => LU_EGG_FRIED,
            #[cfg(feature = "LuEggOff")]
            LuIcon::LuEggOff => LU_EGG_OFF,
            #[cfg(feature = "LuEqual")]
            LuIcon::LuEqual => LU_EQUAL,
            #[cfg(feature = "LuEqualNot")]
            LuIcon::LuEqualNot => LU_EQUAL_NOT,
            #[cfg(feature = "LuEraser")]
            LuIcon::LuEraser => LU_ERASER,
            #[cfg(feature = "LuEuro")]
            LuIcon::LuEuro => LU_EURO,
            #[cfg(feature = "LuExpand")]
            LuIcon::LuExpand => LU_EXPAND,
            #[cfg(feature = "LuExternalLink")]
            LuIcon::LuExternalLink => LU_EXTERNAL_LINK,
            #[cfg(feature = "LuEye")]
            LuIcon::LuEye => LU_EYE,
            #[cfg(feature = "LuEyeOff")]
            LuIcon::LuEyeOff => LU_EYE_OFF,
            #[cfg(feature = "LuFacebook")]
            LuIcon::LuFacebook => LU_FACEBOOK,
            #[cfg(feature = "LuFactory")]
            LuIcon::LuFactory => LU_FACTORY,
            #[cfg(feature = "LuFan")]
            LuIcon::LuFan => LU_FAN,
            #[cfg(feature = "LuFastForward")]
            LuIcon::LuFastForward => LU_FAST_FORWARD,
            #[cfg(feature = "LuFeather")]
            LuIcon::LuFeather => LU_FEATHER,
            #[cfg(feature = "LuFerrisWheel")]
            LuIcon::LuFerrisWheel => LU_FERRIS_WHEEL,
            #[cfg(feature = "LuFigma")]
            LuIcon::LuFigma => LU_FIGMA,
            #[cfg(feature = "LuFile")]
            LuIcon::LuFile => LU_FILE,
            #[cfg(feature = "LuFileArchive")]
            LuIcon::LuFileArchive => LU_FILE_ARCHIVE,
            #[cfg(feature = "LuFileAudio")]
            LuIcon::LuFileAudio => LU_FILE_AUDIO,
            #[cfg(feature = "LuFileAudio2")]
            LuIcon::LuFileAudio2 => LU_FILE_AUDIO2,
            #[cfg(feature = "LuFileAxis3d")]
            LuIcon::LuFileAxis3d => LU_FILE_AXIS3D,
            #[cfg(feature = "LuFileBadge")]
            LuIcon::LuFileBadge => LU_FILE_BADGE,
            #[cfg(feature = "LuFileBadge2")]
            LuIcon::LuFileBadge2 => LU_FILE_BADGE2,
            #[cfg(feature = "LuFileBarChart")]
            LuIcon::LuFileBarChart => LU_FILE_BAR_CHART,
            #[cfg(feature = "LuFileBarChart2")]
            LuIcon::LuFileBarChart2 => LU_FILE_BAR_CHART2,
            #[cfg(feature = "LuFileBox")]
            LuIcon::LuFileBox => LU_FILE_BOX,
            #[cfg(feature = "LuFileCheck")]
            LuIcon::LuFileCheck => LU_FILE_CHECK,
            #[cfg(feature = "LuFileCheck2")]
            LuIcon::LuFileCheck2 => LU_FILE_CHECK2,
            #[cfg(feature = "LuFileClock")]
            LuIcon::LuFileClock => LU_FILE_CLOCK,
            #[cfg(feature = "LuFileCode")]
            LuIcon::LuFileCode => LU_FILE_CODE,
            #[cfg(feature = "LuFileCode2")]
            LuIcon::LuFileCode2 => LU_FILE_CODE2,
            #[cfg(feature = "LuFileCog")]
            LuIcon::LuFileCog => LU_FILE_COG,
            #[cfg(feature = "LuFileCog2")]
            LuIcon::LuFileCog2 => LU_FILE_COG2,
            #[cfg(feature = "LuFileDiff")]
            LuIcon::LuFileDiff => LU_FILE_DIFF,
            #[cfg(feature = "LuFileDigit")]
            LuIcon::LuFileDigit => LU_FILE_DIGIT,
            #[cfg(feature = "LuFileDown")]
            LuIcon::LuFileDown => LU_FILE_DOWN,
            #[cfg(feature = "LuFileEdit")]
            LuIcon::LuFileEdit => LU_FILE_EDIT,
            #[cfg(feature = "LuFileHeart")]
            LuIcon::LuFileHeart => LU_FILE_HEART,
            #[cfg(feature = "LuFileImage")]
            LuIcon::LuFileImage => LU_FILE_IMAGE,
            #[cfg(feature = "LuFileInput")]
            LuIcon::LuFileInput => LU_FILE_INPUT,
            #[cfg(feature = "LuFileJson")]
            LuIcon::LuFileJson => LU_FILE_JSON,
            #[cfg(feature = "LuFileJson2")]
            LuIcon::LuFileJson2 => LU_FILE_JSON2,
            #[cfg(feature = "LuFileKey")]
            LuIcon::LuFileKey => LU_FILE_KEY,
            #[cfg(feature = "LuFileKey2")]
            LuIcon::LuFileKey2 => LU_FILE_KEY2,
            #[cfg(feature = "LuFileLineChart")]
            LuIcon::LuFileLineChart => LU_FILE_LINE_CHART,
            #[cfg(feature = "LuFileLock")]
            LuIcon::LuFileLock => LU_FILE_LOCK,
            #[cfg(feature = "LuFileLock2")]
            LuIcon::LuFileLock2 => LU_FILE_LOCK2,
            #[cfg(feature = "LuFileMinus")]
            LuIcon::LuFileMinus => LU_FILE_MINUS,
            #[cfg(feature = "LuFileMinus2")]
            LuIcon::LuFileMinus2 => LU_FILE_MINUS2,
            #[cfg(feature = "LuFileOutput")]
            LuIcon::LuFileOutput => LU_FILE_OUTPUT,
            #[cfg(feature = "LuFilePieChart")]
            LuIcon::LuFilePieChart => LU_FILE_PIE_CHART,
            #[cfg(feature = "LuFilePlus")]
            LuIcon::LuFilePlus => LU_FILE_PLUS,
            #[cfg(feature = "LuFilePlus2")]
            LuIcon::LuFilePlus2 => LU_FILE_PLUS2,
            #[cfg(feature = "LuFileQuestion")]
            LuIcon::LuFileQuestion => LU_FILE_QUESTION,
            #[cfg(feature = "LuFileScan")]
            LuIcon::LuFileScan => LU_FILE_SCAN,
            #[cfg(feature = "LuFileSearch")]
            LuIcon::LuFileSearch => LU_FILE_SEARCH,
            #[cfg(feature = "LuFileSearch2")]
            LuIcon::LuFileSearch2 => LU_FILE_SEARCH2,
            #[cfg(feature = "LuFileSignature")]
            LuIcon::LuFileSignature => LU_FILE_SIGNATURE,
            #[cfg(feature = "LuFileSpreadsheet")]
            LuIcon::LuFileSpreadsheet => LU_FILE_SPREADSHEET,
            #[cfg(feature = "LuFileStack")]
            LuIcon::LuFileStack => LU_FILE_STACK,
            #[cfg(feature = "LuFileSymlink")]
            LuIcon::LuFileSymlink => LU_FILE_SYMLINK,
            #[cfg(feature = "LuFileTerminal")]
            LuIcon::LuFileTerminal => LU_FILE_TERMINAL,
            #[cfg(feature = "LuFileText")]
            LuIcon::LuFileText => LU_FILE_TEXT,
            #[cfg(feature = "LuFileType")]
            LuIcon::LuFileType => LU_FILE_TYPE,
            #[cfg(feature = "LuFileType2")]
            LuIcon::LuFileType2 => LU_FILE_TYPE2,
            #[cfg(feature = "LuFileUp")]
            LuIcon::LuFileUp => LU_FILE_UP,
            #[cfg(feature = "LuFileVideo")]
            LuIcon::LuFileVideo => LU_FILE_VIDEO,
            #[cfg(feature = "LuFileVideo2")]
            LuIcon::LuFileVideo2 => LU_FILE_VIDEO2,
            #[cfg(feature = "LuFileVolume")]
            LuIcon::LuFileVolume => LU_FILE_VOLUME,
            #[cfg(feature = "LuFileVolume2")]
            LuIcon::LuFileVolume2 => LU_FILE_VOLUME2,
            #[cfg(feature = "LuFileWarning")]
            LuIcon::LuFileWarning => LU_FILE_WARNING,
            #[cfg(feature = "LuFileX")]
            LuIcon::LuFileX => LU_FILE_X,
            #[cfg(feature = "LuFileX2")]
            LuIcon::LuFileX2 => LU_FILE_X2,
            #[cfg(feature = "LuFiles")]
            LuIcon::LuFiles => LU_FILES,
            #[cfg(feature = "LuFilm")]
            LuIcon::LuFilm => LU_FILM,
            #[cfg(feature = "LuFilter")]
            LuIcon::LuFilter => LU_FILTER,
            #[cfg(feature = "LuFilterX")]
            LuIcon::LuFilterX => LU_FILTER_X,
            #[cfg(feature = "LuFingerprint")]
            LuIcon::LuFingerprint => LU_FINGERPRINT,
            #[cfg(feature = "LuFish")]
            LuIcon::LuFish => LU_FISH,
            #[cfg(feature = "LuFishOff")]
            LuIcon::LuFishOff => LU_FISH_OFF,
            #[cfg(feature = "LuFishSymbol")]
            LuIcon::LuFishSymbol => LU_FISH_SYMBOL,
            #[cfg(feature = "LuFlag")]
            LuIcon::LuFlag => LU_FLAG,
            #[cfg(feature = "LuFlagOff")]
            LuIcon::LuFlagOff => LU_FLAG_OFF,
            #[cfg(feature = "LuFlagTriangleLeft")]
            LuIcon::LuFlagTriangleLeft => LU_FLAG_TRIANGLE_LEFT,
            #[cfg(feature = "LuFlagTriangleRight")]
            LuIcon::LuFlagTriangleRight => LU_FLAG_TRIANGLE_RIGHT,
            #[cfg(feature = "LuFlame")]
            LuIcon::LuFlame => LU_FLAME,
            #[cfg(feature = "LuFlashlight")]
            LuIcon::LuFlashlight => LU_FLASHLIGHT,
            #[cfg(feature = "LuFlashlightOff")]
            LuIcon::LuFlashlightOff => LU_FLASHLIGHT_OFF,
            #[cfg(feature = "LuFlaskConical")]
            LuIcon::LuFlaskConical => LU_FLASK_CONICAL,
            #[cfg(feature = "LuFlaskConicalOff")]
            LuIcon::LuFlaskConicalOff => LU_FLASK_CONICAL_OFF,
            #[cfg(feature = "LuFlaskRound")]
            LuIcon::LuFlaskRound => LU_FLASK_ROUND,
            #[cfg(feature = "LuFlipHorizontal")]
            LuIcon::LuFlipHorizontal => LU_FLIP_HORIZONTAL,
            #[cfg(feature = "LuFlipHorizontal2")]
            LuIcon::LuFlipHorizontal2 => LU_FLIP_HORIZONTAL2,
            #[cfg(feature = "LuFlipVertical")]
            LuIcon::LuFlipVertical => LU_FLIP_VERTICAL,
            #[cfg(feature = "LuFlipVertical2")]
            LuIcon::LuFlipVertical2 => LU_FLIP_VERTICAL2,
            #[cfg(feature = "LuFlower")]
            LuIcon::LuFlower => LU_FLOWER,
            #[cfg(feature = "LuFlower2")]
            LuIcon::LuFlower2 => LU_FLOWER2,
            #[cfg(feature = "LuFocus")]
            LuIcon::LuFocus => LU_FOCUS,
            #[cfg(feature = "LuFoldHorizontal")]
            LuIcon::LuFoldHorizontal => LU_FOLD_HORIZONTAL,
            #[cfg(feature = "LuFoldVertical")]
            LuIcon::LuFoldVertical => LU_FOLD_VERTICAL,
            #[cfg(feature = "LuFolder")]
            LuIcon::LuFolder => LU_FOLDER,
            #[cfg(feature = "LuFolderArchive")]
            LuIcon::LuFolderArchive => LU_FOLDER_ARCHIVE,
            #[cfg(feature = "LuFolderCheck")]
            LuIcon::LuFolderCheck => LU_FOLDER_CHECK,
            #[cfg(feature = "LuFolderClock")]
            LuIcon::LuFolderClock => LU_FOLDER_CLOCK,
            #[cfg(feature = "LuFolderClosed")]
            LuIcon::LuFolderClosed => LU_FOLDER_CLOSED,
            #[cfg(feature = "LuFolderCog")]
            LuIcon::LuFolderCog => LU_FOLDER_COG,
            #[cfg(feature = "LuFolderCog2")]
            LuIcon::LuFolderCog2 => LU_FOLDER_COG2,
            #[cfg(feature = "LuFolderDot")]
            LuIcon::LuFolderDot => LU_FOLDER_DOT,
            #[cfg(feature = "LuFolderDown")]
            LuIcon::LuFolderDown => LU_FOLDER_DOWN,
            #[cfg(feature = "LuFolderEdit")]
            LuIcon::LuFolderEdit => LU_FOLDER_EDIT,
            #[cfg(feature = "LuFolderGit")]
            LuIcon::LuFolderGit => LU_FOLDER_GIT,
            #[cfg(feature = "LuFolderGit2")]
            LuIcon::LuFolderGit2 => LU_FOLDER_GIT2,
            #[cfg(feature = "LuFolderHeart")]
            LuIcon::LuFolderHeart => LU_FOLDER_HEART,
            #[cfg(feature = "LuFolderInput")]
            LuIcon::LuFolderInput => LU_FOLDER_INPUT,
            #[cfg(feature = "LuFolderKanban")]
            LuIcon::LuFolderKanban => LU_FOLDER_KANBAN,
            #[cfg(feature = "LuFolderKey")]
            LuIcon::LuFolderKey => LU_FOLDER_KEY,
            #[cfg(feature = "LuFolderLock")]
            LuIcon::LuFolderLock => LU_FOLDER_LOCK,
            #[cfg(feature = "LuFolderMinus")]
            LuIcon::LuFolderMinus => LU_FOLDER_MINUS,
            #[cfg(feature = "LuFolderOpen")]
            LuIcon::LuFolderOpen => LU_FOLDER_OPEN,
            #[cfg(feature = "LuFolderOpenDot")]
            LuIcon::LuFolderOpenDot => LU_FOLDER_OPEN_DOT,
            #[cfg(feature = "LuFolderOutput")]
            LuIcon::LuFolderOutput => LU_FOLDER_OUTPUT,
            #[cfg(feature = "LuFolderPlus")]
            LuIcon::LuFolderPlus => LU_FOLDER_PLUS,
            #[cfg(feature = "LuFolderRoot")]
            LuIcon::LuFolderRoot => LU_FOLDER_ROOT,
            #[cfg(feature = "LuFolderSearch")]
            LuIcon::LuFolderSearch => LU_FOLDER_SEARCH,
            #[cfg(feature = "LuFolderSearch2")]
            LuIcon::LuFolderSearch2 => LU_FOLDER_SEARCH2,
            #[cfg(feature = "LuFolderSymlink")]
            LuIcon::LuFolderSymlink => LU_FOLDER_SYMLINK,
            #[cfg(feature = "LuFolderSync")]
            LuIcon::LuFolderSync => LU_FOLDER_SYNC,
            #[cfg(feature = "LuFolderTree")]
            LuIcon::LuFolderTree => LU_FOLDER_TREE,
            #[cfg(feature = "LuFolderUp")]
            LuIcon::LuFolderUp => LU_FOLDER_UP,
            #[cfg(feature = "LuFolderX")]
            LuIcon::LuFolderX => LU_FOLDER_X,
            #[cfg(feature = "LuFolders")]
            LuIcon::LuFolders => LU_FOLDERS,
            #[cfg(feature = "LuFootprints")]
            LuIcon::LuFootprints => LU_FOOTPRINTS,
            #[cfg(feature = "LuForklift")]
            LuIcon::LuForklift => LU_FORKLIFT,
            #[cfg(feature = "LuFormInput")]
            LuIcon::LuFormInput => LU_FORM_INPUT,
            #[cfg(feature = "LuForward")]
            LuIcon::LuForward => LU_FORWARD,
            #[cfg(feature = "LuFrame")]
            LuIcon::LuFrame => LU_FRAME,
            #[cfg(feature = "LuFramer")]
            LuIcon::LuFramer => LU_FRAMER,
            #[cfg(feature = "LuFrown")]
            LuIcon::LuFrown => LU_FROWN,
            #[cfg(feature = "LuFuel")]
            LuIcon::LuFuel => LU_FUEL,
            #[cfg(feature = "LuFunctionSquare")]
            LuIcon::LuFunctionSquare => LU_FUNCTION_SQUARE,
            #[cfg(feature = "LuGalleryHorizontal")]
            LuIcon::LuGalleryHorizontal => LU_GALLERY_HORIZONTAL,
            #[cfg(feature = "LuGalleryHorizontalEnd")]
            LuIcon::LuGalleryHorizontalEnd => LU_GALLERY_HORIZONTAL_END,
            #[cfg(feature = "LuGalleryThumbnails")]
            LuIcon::LuGalleryThumbnails => LU_GALLERY_THUMBNAILS,
            #[cfg(feature = "LuGalleryVertical")]
            LuIcon::LuGalleryVertical => LU_GALLERY_VERTICAL,
            #[cfg(feature = "LuGalleryVerticalEnd")]
            LuIcon::LuGalleryVerticalEnd => LU_GALLERY_VERTICAL_END,
            #[cfg(feature = "LuGamepad")]
            LuIcon::LuGamepad => LU_GAMEPAD,
            #[cfg(feature = "LuGamepad2")]
            LuIcon::LuGamepad2 => LU_GAMEPAD2,
            #[cfg(feature = "LuGanttChart")]
            LuIcon::LuGanttChart => LU_GANTT_CHART,
            #[cfg(feature = "LuGanttChartSquare")]
            LuIcon::LuGanttChartSquare => LU_GANTT_CHART_SQUARE,
            #[cfg(feature = "LuGauge")]
            LuIcon::LuGauge => LU_GAUGE,
            #[cfg(feature = "LuGaugeCircle")]
            LuIcon::LuGaugeCircle => LU_GAUGE_CIRCLE,
            #[cfg(feature = "LuGavel")]
            LuIcon::LuGavel => LU_GAVEL,
            #[cfg(feature = "LuGem")]
            LuIcon::LuGem => LU_GEM,
            #[cfg(feature = "LuGhost")]
            LuIcon::LuGhost => LU_GHOST,
            #[cfg(feature = "LuGift")]
            LuIcon::LuGift => LU_GIFT,
            #[cfg(feature = "LuGitBranch")]
            LuIcon::LuGitBranch => LU_GIT_BRANCH,
            #[cfg(feature = "LuGitBranchPlus")]
            LuIcon::LuGitBranchPlus => LU_GIT_BRANCH_PLUS,
            #[cfg(feature = "LuGitCommit")]
            LuIcon::LuGitCommit => LU_GIT_COMMIT,
            #[cfg(feature = "LuGitCompare")]
            LuIcon::LuGitCompare => LU_GIT_COMPARE,
            #[cfg(feature = "LuGitFork")]
            LuIcon::LuGitFork => LU_GIT_FORK,
            #[cfg(feature = "LuGitMerge")]
            LuIcon::LuGitMerge => LU_GIT_MERGE,
            #[cfg(feature = "LuGitPullRequest")]
            LuIcon::LuGitPullRequest => LU_GIT_PULL_REQUEST,
            #[cfg(feature = "LuGitPullRequestClosed")]
            LuIcon::LuGitPullRequestClosed => LU_GIT_PULL_REQUEST_CLOSED,
            #[cfg(feature = "LuGitPullRequestDraft")]
            LuIcon::LuGitPullRequestDraft => LU_GIT_PULL_REQUEST_DRAFT,
            #[cfg(feature = "LuGithub")]
            LuIcon::LuGithub => LU_GITHUB,
            #[cfg(feature = "LuGitlab")]
            LuIcon::LuGitlab => LU_GITLAB,
            #[cfg(feature = "LuGlassWater")]
            LuIcon::LuGlassWater => LU_GLASS_WATER,
            #[cfg(feature = "LuGlasses")]
            LuIcon::LuGlasses => LU_GLASSES,
            #[cfg(feature = "LuGlobe")]
            LuIcon::LuGlobe => LU_GLOBE,
            #[cfg(feature = "LuGlobe2")]
            LuIcon::LuGlobe2 => LU_GLOBE2,
            #[cfg(feature = "LuGoal")]
            LuIcon::LuGoal => LU_GOAL,
            #[cfg(feature = "LuGrab")]
            LuIcon::LuGrab => LU_GRAB,
            #[cfg(feature = "LuGraduationCap")]
            LuIcon::LuGraduationCap => LU_GRADUATION_CAP,
            #[cfg(feature = "LuGrape")]
            LuIcon::LuGrape => LU_GRAPE,
            #[cfg(feature = "LuGrid2x2")]
            LuIcon::LuGrid2x2 => LU_GRID2X2,
            #[cfg(feature = "LuGrid3x3")]
            LuIcon::LuGrid3x3 => LU_GRID3X3,
            #[cfg(feature = "LuGrip")]
            LuIcon::LuGrip => LU_GRIP,
            #[cfg(feature = "LuGripHorizontal")]
            LuIcon::LuGripHorizontal => LU_GRIP_HORIZONTAL,
            #[cfg(feature = "LuGripVertical")]
            LuIcon::LuGripVertical => LU_GRIP_VERTICAL,
            #[cfg(feature = "LuGroup")]
            LuIcon::LuGroup => LU_GROUP,
            #[cfg(feature = "LuHammer")]
            LuIcon::LuHammer => LU_HAMMER,
            #[cfg(feature = "LuHand")]
            LuIcon::LuHand => LU_HAND,
            #[cfg(feature = "LuHandMetal")]
            LuIcon::LuHandMetal => LU_HAND_METAL,
            #[cfg(feature = "LuHardDrive")]
            LuIcon::LuHardDrive => LU_HARD_DRIVE,
            #[cfg(feature = "LuHardDriveDownload")]
            LuIcon::LuHardDriveDownload => LU_HARD_DRIVE_DOWNLOAD,
            #[cfg(feature = "LuHardDriveUpload")]
            LuIcon::LuHardDriveUpload => LU_HARD_DRIVE_UPLOAD,
            #[cfg(feature = "LuHardHat")]
            LuIcon::LuHardHat => LU_HARD_HAT,
            #[cfg(feature = "LuHash")]
            LuIcon::LuHash => LU_HASH,
            #[cfg(feature = "LuHaze")]
            LuIcon::LuHaze => LU_HAZE,
            #[cfg(feature = "LuHdmiPort")]
            LuIcon::LuHdmiPort => LU_HDMI_PORT,
            #[cfg(feature = "LuHeading")]
            LuIcon::LuHeading => LU_HEADING,
            #[cfg(feature = "LuHeading1")]
            LuIcon::LuHeading1 => LU_HEADING1,
            #[cfg(feature = "LuHeading2")]
            LuIcon::LuHeading2 => LU_HEADING2,
            #[cfg(feature = "LuHeading3")]
            LuIcon::LuHeading3 => LU_HEADING3,
            #[cfg(feature = "LuHeading4")]
            LuIcon::LuHeading4 => LU_HEADING4,
            #[cfg(feature = "LuHeading5")]
            LuIcon::LuHeading5 => LU_HEADING5,
            #[cfg(feature = "LuHeading6")]
            LuIcon::LuHeading6 => LU_HEADING6,
            #[cfg(feature = "LuHeadphones")]
            LuIcon::LuHeadphones => LU_HEADPHONES,
            #[cfg(feature = "LuHeart")]
            LuIcon::LuHeart => LU_HEART,
            #[cfg(feature = "LuHeartCrack")]
            LuIcon::LuHeartCrack => LU_HEART_CRACK,
            #[cfg(feature = "LuHeartHandshake")]
            LuIcon::LuHeartHandshake => LU_HEART_HANDSHAKE,
            #[cfg(feature = "LuHeartOff")]
            LuIcon::LuHeartOff => LU_HEART_OFF,
            #[cfg(feature = "LuHeartPulse")]
            LuIcon::LuHeartPulse => LU_HEART_PULSE,
            #[cfg(feature = "LuHelpCircle")]
            LuIcon::LuHelpCircle => LU_HELP_CIRCLE,
            #[cfg(feature = "LuHelpingHand")]
            LuIcon::LuHelpingHand => LU_HELPING_HAND,
            #[cfg(feature = "LuHexagon")]
            LuIcon::LuHexagon => LU_HEXAGON,
            #[cfg(feature = "LuHighlighter")]
            LuIcon::LuHighlighter => LU_HIGHLIGHTER,
            #[cfg(feature = "LuHistory")]
            LuIcon::LuHistory => LU_HISTORY,
            #[cfg(feature = "LuHome")]
            LuIcon::LuHome => LU_HOME,
            #[cfg(feature = "LuHop")]
            LuIcon::LuHop => LU_HOP,
            #[cfg(feature = "LuHopOff")]
            LuIcon::LuHopOff => LU_HOP_OFF,
            #[cfg(feature = "LuHotel")]
            LuIcon::LuHotel => LU_HOTEL,
            #[cfg(feature = "LuHourglass")]
            LuIcon::LuHourglass => LU_HOURGLASS,
            #[cfg(feature = "LuIceCream")]
            LuIcon::LuIceCream => LU_ICE_CREAM,
            #[cfg(feature = "LuIceCream2")]
            LuIcon::LuIceCream2 => LU_ICE_CREAM2,
            #[cfg(feature = "LuImage")]
            LuIcon::LuImage => LU_IMAGE,
            #[cfg(feature = "LuImageMinus")]
            LuIcon::LuImageMinus => LU_IMAGE_MINUS,
            #[cfg(feature = "LuImageOff")]
            LuIcon::LuImageOff => LU_IMAGE_OFF,
            #[cfg(feature = "LuImagePlus")]
            LuIcon::LuImagePlus => LU_IMAGE_PLUS,
            #[cfg(feature = "LuImport")]
            LuIcon::LuImport => LU_IMPORT,
            #[cfg(feature = "LuInbox")]
            LuIcon::LuInbox => LU_INBOX,
            #[cfg(feature = "LuIndent")]
            LuIcon::LuIndent => LU_INDENT,
            #[cfg(feature = "LuIndianRupee")]
            LuIcon::LuIndianRupee => LU_INDIAN_RUPEE,
            #[cfg(feature = "LuInfinity")]
            LuIcon::LuInfinity => LU_INFINITY,
            #[cfg(feature = "LuInfo")]
            LuIcon::LuInfo => LU_INFO,
            #[cfg(feature = "LuInspect")]
            LuIcon::LuInspect => LU_INSPECT,
            #[cfg(feature = "LuInstagram")]
            LuIcon::LuInstagram => LU_INSTAGRAM,
            #[cfg(feature = "LuItalic")]
            LuIcon::LuItalic => LU_ITALIC,
            #[cfg(feature = "LuIterationCcw")]
            LuIcon::LuIterationCcw => LU_ITERATION_CCW,
            #[cfg(feature = "LuIterationCw")]
            LuIcon::LuIterationCw => LU_ITERATION_CW,
            #[cfg(feature = "LuJapaneseYen")]
            LuIcon::LuJapaneseYen => LU_JAPANESE_YEN,
            #[cfg(feature = "LuJoystick")]
            LuIcon::LuJoystick => LU_JOYSTICK,
            #[cfg(feature = "LuKanban")]
            LuIcon::LuKanban => LU_KANBAN,
            #[cfg(feature = "LuKanbanSquare")]
            LuIcon::LuKanbanSquare => LU_KANBAN_SQUARE,
            #[cfg(feature = "LuKanbanSquareDashed")]
            LuIcon::LuKanbanSquareDashed => LU_KANBAN_SQUARE_DASHED,
            #[cfg(feature = "LuKey")]
            LuIcon::LuKey => LU_KEY,
            #[cfg(feature = "LuKeyRound")]
            LuIcon::LuKeyRound => LU_KEY_ROUND,
            #[cfg(feature = "LuKeySquare")]
            LuIcon::LuKeySquare => LU_KEY_SQUARE,
            #[cfg(feature = "LuKeyboard")]
            LuIcon::LuKeyboard => LU_KEYBOARD,
            #[cfg(feature = "LuLamp")]
            LuIcon::LuLamp => LU_LAMP,
            #[cfg(feature = "LuLampCeiling")]
            LuIcon::LuLampCeiling => LU_LAMP_CEILING,
            #[cfg(feature = "LuLampDesk")]
            LuIcon::LuLampDesk => LU_LAMP_DESK,
            #[cfg(feature = "LuLampFloor")]
            LuIcon::LuLampFloor => LU_LAMP_FLOOR,
            #[cfg(feature = "LuLampWallDown")]
            LuIcon::LuLampWallDown => LU_LAMP_WALL_DOWN,
            #[cfg(feature = "LuLampWallUp")]
            LuIcon::LuLampWallUp => LU_LAMP_WALL_UP,
            #[cfg(feature = "LuLandmark")]
            LuIcon::LuLandmark => LU_LANDMARK,
            #[cfg(feature = "LuLanguages")]
            LuIcon::LuLanguages => LU_LANGUAGES,
            #[cfg(feature = "LuLaptop")]
            LuIcon::LuLaptop => LU_LAPTOP,
            #[cfg(feature = "LuLaptop2")]
            LuIcon::LuLaptop2 => LU_LAPTOP2,
            #[cfg(feature = "LuLasso")]
            LuIcon::LuLasso => LU_LASSO,
            #[cfg(feature = "LuLassoSelect")]
            LuIcon::LuLassoSelect => LU_LASSO_SELECT,
            #[cfg(feature = "LuLaugh")]
            LuIcon::LuLaugh => LU_LAUGH,
            #[cfg(feature = "LuLayers")]
            LuIcon::LuLayers => LU_LAYERS,
            #[cfg(feature = "LuLayout")]
            LuIcon::LuLayout => LU_LAYOUT,
            #[cfg(feature = "LuLayoutDashboard")]
            LuIcon::LuLayoutDashboard => LU_LAYOUT_DASHBOARD,
            #[cfg(feature = "LuLayoutGrid")]
            LuIcon::LuLayoutGrid => LU_LAYOUT_GRID,
            #[cfg(feature = "LuLayoutList")]
            LuIcon::LuLayoutList => LU_LAYOUT_LIST,
            #[cfg(feature = "LuLayoutPanelLeft")]
            LuIcon::LuLayoutPanelLeft => LU_LAYOUT_PANEL_LEFT,
            #[cfg(feature = "LuLayoutPanelTop")]
            LuIcon::LuLayoutPanelTop => LU_LAYOUT_PANEL_TOP,
            #[cfg(feature = "LuLayoutTemplate")]
            LuIcon::LuLayoutTemplate => LU_LAYOUT_TEMPLATE,
            #[cfg(feature = "LuLeaf")]
            LuIcon::LuLeaf => LU_LEAF,
            #[cfg(feature = "LuLeafyGreen")]
            LuIcon::LuLeafyGreen => LU_LEAFY_GREEN,
            #[cfg(feature = "LuLibrary")]
            LuIcon::LuLibrary => LU_LIBRARY,
            #[cfg(feature = "LuLifeBuoy")]
            LuIcon::LuLifeBuoy => LU_LIFE_BUOY,
            #[cfg(feature = "LuLigature")]
            LuIcon::LuLigature => LU_LIGATURE,
            #[cfg(feature = "LuLightbulb")]
            LuIcon::LuLightbulb => LU_LIGHTBULB,
            #[cfg(feature = "LuLightbulbOff")]
            LuIcon::LuLightbulbOff => LU_LIGHTBULB_OFF,
            #[cfg(feature = "LuLineChart")]
            LuIcon::LuLineChart => LU_LINE_CHART,
            #[cfg(feature = "LuLink")]
            LuIcon::LuLink => LU_LINK,
            #[cfg(feature = "LuLink2")]
            LuIcon::LuLink2 => LU_LINK2,
            #[cfg(feature = "LuLink2Off")]
            LuIcon::LuLink2Off => LU_LINK2_OFF,
            #[cfg(feature = "LuLinkedin")]
            LuIcon::LuLinkedin => LU_LINKEDIN,
            #[cfg(feature = "LuList")]
            LuIcon::LuList => LU_LIST,
            #[cfg(feature = "LuListChecks")]
            LuIcon::LuListChecks => LU_LIST_CHECKS,
            #[cfg(feature = "LuListEnd")]
            LuIcon::LuListEnd => LU_LIST_END,
            #[cfg(feature = "LuListFilter")]
            LuIcon::LuListFilter => LU_LIST_FILTER,
            #[cfg(feature = "LuListMinus")]
            LuIcon::LuListMinus => LU_LIST_MINUS,
            #[cfg(feature = "LuListMusic")]
            LuIcon::LuListMusic => LU_LIST_MUSIC,
            #[cfg(feature = "LuListOrdered")]
            LuIcon::LuListOrdered => LU_LIST_ORDERED,
            #[cfg(feature = "LuListPlus")]
            LuIcon::LuListPlus => LU_LIST_PLUS,
            #[cfg(feature = "LuListRestart")]
            LuIcon::LuListRestart => LU_LIST_RESTART,
            #[cfg(feature = "LuListStart")]
            LuIcon::LuListStart => LU_LIST_START,
            #[cfg(feature = "LuListTodo")]
            LuIcon::LuListTodo => LU_LIST_TODO,
            #[cfg(feature = "LuListTree")]
            LuIcon::LuListTree => LU_LIST_TREE,
            #[cfg(feature = "LuListVideo")]
            LuIcon::LuListVideo => LU_LIST_VIDEO,
            #[cfg(feature = "LuListX")]
            LuIcon::LuListX => LU_LIST_X,
            #[cfg(feature = "LuLoader")]
            LuIcon::LuLoader => LU_LOADER,
            #[cfg(feature = "LuLoader2")]
            LuIcon::LuLoader2 => LU_LOADER2,
            #[cfg(feature = "LuLocate")]
            LuIcon::LuLocate => LU_LOCATE,
            #[cfg(feature = "LuLocateFixed")]
            LuIcon::LuLocateFixed => LU_LOCATE_FIXED,
            #[cfg(feature = "LuLocateOff")]
            LuIcon::LuLocateOff => LU_LOCATE_OFF,
            #[cfg(feature = "LuLock")]
            LuIcon::LuLock => LU_LOCK,
            #[cfg(feature = "LuLogIn")]
            LuIcon::LuLogIn => LU_LOG_IN,
            #[cfg(feature = "LuLogOut")]
            LuIcon::LuLogOut => LU_LOG_OUT,
            #[cfg(feature = "LuLollipop")]
            LuIcon::LuLollipop => LU_LOLLIPOP,
            #[cfg(feature = "LuLuggage")]
            LuIcon::LuLuggage => LU_LUGGAGE,
            #[cfg(feature = "LuMSquare")]
            LuIcon::LuMSquare => LU_M_SQUARE,
            #[cfg(feature = "LuMagnet")]
            LuIcon::LuMagnet => LU_MAGNET,
            #[cfg(feature = "LuMail")]
            LuIcon::LuMail => LU_MAIL,
            #[cfg(feature = "LuMailCheck")]
            LuIcon::LuMailCheck => LU_MAIL_CHECK,
            #[cfg(feature = "LuMailMinus")]
            LuIcon::LuMailMinus => LU_MAIL_MINUS,
            #[cfg(feature = "LuMailOpen")]
            LuIcon::LuMailOpen => LU_MAIL_OPEN,
            #[cfg(feature = "LuMailPlus")]
            LuIcon::LuMailPlus => LU_MAIL_PLUS,
            #[cfg(feature = "LuMailQuestion")]
            LuIcon::LuMailQuestion => LU_MAIL_QUESTION,
            #[cfg(feature = "LuMailSearch")]
            LuIcon::LuMailSearch => LU_MAIL_SEARCH,
            #[cfg(feature = "LuMailWarning")]
            LuIcon::LuMailWarning => LU_MAIL_WARNING,
            #[cfg(feature = "LuMailX")]
            LuIcon::LuMailX => LU_MAIL_X,
            #[cfg(feature = "LuMailbox")]
            LuIcon::LuMailbox => LU_MAILBOX,
            #[cfg(feature = "LuMails")]
            LuIcon::LuMails => LU_MAILS,
            #[cfg(feature = "LuMap")]
            LuIcon::LuMap => LU_MAP,
            #[cfg(feature = "LuMapPin")]
            LuIcon::LuMapPin => LU_MAP_PIN,
            #[cfg(feature = "LuMapPinOff")]
            LuIcon::LuMapPinOff => LU_MAP_PIN_OFF,
            #[cfg(feature = "LuMartini")]
            LuIcon::LuMartini => LU_MARTINI,
            #[cfg(feature = "LuMaximize")]
            LuIcon::LuMaximize => LU_MAXIMIZE,
            #[cfg(feature = "LuMaximize2")]
            LuIcon::LuMaximize2 => LU_MAXIMIZE2,
            #[cfg(feature = "LuMedal")]
            LuIcon::LuMedal => LU_MEDAL,
            #[cfg(feature = "LuMegaphone")]
            LuIcon::LuMegaphone => LU_MEGAPHONE,
            #[cfg(feature = "LuMegaphoneOff")]
            LuIcon::LuMegaphoneOff => LU_MEGAPHONE_OFF,
            #[cfg(feature = "LuMeh")]
            LuIcon::LuMeh => LU_MEH,
            #[cfg(feature = "LuMemoryStick")]
            LuIcon::LuMemoryStick => LU_MEMORY_STICK,
            #[cfg(feature = "LuMenu")]
            LuIcon::LuMenu => LU_MENU,
            #[cfg(feature = "LuMenuSquare")]
            LuIcon::LuMenuSquare => LU_MENU_SQUARE,
            #[cfg(feature = "LuMerge")]
            LuIcon::LuMerge => LU_MERGE,
            #[cfg(feature = "LuMessageCircle")]
            LuIcon::LuMessageCircle => LU_MESSAGE_CIRCLE,
            #[cfg(feature = "LuMessageSquare")]
            LuIcon::LuMessageSquare => LU_MESSAGE_SQUARE,
            #[cfg(feature = "LuMessageSquareDashed")]
            LuIcon::LuMessageSquareDashed => LU_MESSAGE_SQUARE_DASHED,
            #[cfg(feature = "LuMessageSquarePlus")]
            LuIcon::LuMessageSquarePlus => LU_MESSAGE_SQUARE_PLUS,
            #[cfg(feature = "LuMessagesSquare")]
            LuIcon::LuMessagesSquare => LU_MESSAGES_SQUARE,
            #[cfg(feature = "LuMic")]
            LuIcon::LuMic => LU_MIC,
            #[cfg(feature = "LuMic2")]
            LuIcon::LuMic2 => LU_MIC2,
            #[cfg(feature = "LuMicOff")]
            LuIcon::LuMicOff => LU_MIC_OFF,
            #[cfg(feature = "LuMicroscope")]
            LuIcon::LuMicroscope => LU_MICROSCOPE,
            #[cfg(feature = "LuMicrowave")]
            LuIcon::LuMicrowave => LU_MICROWAVE,
            #[cfg(feature = "LuMilestone")]
            LuIcon::LuMilestone => LU_MILESTONE,
            #[cfg(feature = "LuMilk")]
            LuIcon::LuMilk => LU_MILK,
            #[cfg(feature = "LuMilkOff")]
            LuIcon::LuMilkOff => LU_MILK_OFF,
            #[cfg(feature = "LuMinimize")]
            LuIcon::LuMinimize => LU_MINIMIZE,
            #[cfg(feature = "LuMinimize2")]
            LuIcon::LuMinimize2 => LU_MINIMIZE2,
            #[cfg(feature = "LuMinus")]
            LuIcon::LuMinus => LU_MINUS,
            #[cfg(feature = "LuMinusCircle")]
            LuIcon::LuMinusCircle => LU_MINUS_CIRCLE,
            #[cfg(feature = "LuMinusSquare")]
            LuIcon::LuMinusSquare => LU_MINUS_SQUARE,
            #[cfg(feature = "LuMonitor")]
            LuIcon::LuMonitor => LU_MONITOR,
            #[cfg(feature = "LuMonitorCheck")]
            LuIcon::LuMonitorCheck => LU_MONITOR_CHECK,
            #[cfg(feature = "LuMonitorDot")]
            LuIcon::LuMonitorDot => LU_MONITOR_DOT,
            #[cfg(feature = "LuMonitorDown")]
            LuIcon::LuMonitorDown => LU_MONITOR_DOWN,
            #[cfg(feature = "LuMonitorOff")]
            LuIcon::LuMonitorOff => LU_MONITOR_OFF,
            #[cfg(feature = "LuMonitorPause")]
            LuIcon::LuMonitorPause => LU_MONITOR_PAUSE,
            #[cfg(feature = "LuMonitorPlay")]
            LuIcon::LuMonitorPlay => LU_MONITOR_PLAY,
            #[cfg(feature = "LuMonitorSmartphone")]
            LuIcon::LuMonitorSmartphone => LU_MONITOR_SMARTPHONE,
            #[cfg(feature = "LuMonitorSpeaker")]
            LuIcon::LuMonitorSpeaker => LU_MONITOR_SPEAKER,
            #[cfg(feature = "LuMonitorStop")]
            LuIcon::LuMonitorStop => LU_MONITOR_STOP,
            #[cfg(feature = "LuMonitorUp")]
            LuIcon::LuMonitorUp => LU_MONITOR_UP,
            #[cfg(feature = "LuMonitorX")]
            LuIcon::LuMonitorX => LU_MONITOR_X,
            #[cfg(feature = "LuMoon")]
            LuIcon::LuMoon => LU_MOON,
            #[cfg(feature = "LuMoonStar")]
            LuIcon::LuMoonStar => LU_MOON_STAR,
            #[cfg(feature = "LuMoreHorizontal")]
            LuIcon::LuMoreHorizontal => LU_MORE_HORIZONTAL,
            #[cfg(feature = "LuMoreVertical")]
            LuIcon::LuMoreVertical => LU_MORE_VERTICAL,
            #[cfg(feature = "LuMountain")]
            LuIcon::LuMountain => LU_MOUNTAIN,
            #[cfg(feature = "LuMountainSnow")]
            LuIcon::LuMountainSnow => LU_MOUNTAIN_SNOW,
            #[cfg(feature = "LuMouse")]
            LuIcon::LuMouse => LU_MOUSE,
            #[cfg(feature = "LuMousePointer")]
            LuIcon::LuMousePointer => LU_MOUSE_POINTER,
            #[cfg(feature = "LuMousePointer2")]
            LuIcon::LuMousePointer2 => LU_MOUSE_POINTER2,
            #[cfg(feature = "LuMousePointerClick")]
            LuIcon::LuMousePointerClick => LU_MOUSE_POINTER_CLICK,
            #[cfg(feature = "LuMove")]
            LuIcon::LuMove => LU_MOVE,
            #[cfg(feature = "LuMove3d")]
            LuIcon::LuMove3d => LU_MOVE3D,
            #[cfg(feature = "LuMoveDiagonal")]
            LuIcon::LuMoveDiagonal => LU_MOVE_DIAGONAL,
            #[cfg(feature = "LuMoveDiagonal2")]
            LuIcon::LuMoveDiagonal2 => LU_MOVE_DIAGONAL2,
            #[cfg(feature = "LuMoveDown")]
            LuIcon::LuMoveDown => LU_MOVE_DOWN,
            #[cfg(feature = "LuMoveDownLeft")]
            LuIcon::LuMoveDownLeft => LU_MOVE_DOWN_LEFT,
            #[cfg(feature = "LuMoveDownRight")]
            LuIcon::LuMoveDownRight => LU_MOVE_DOWN_RIGHT,
            #[cfg(feature = "LuMoveHorizontal")]
            LuIcon::LuMoveHorizontal => LU_MOVE_HORIZONTAL,
            #[cfg(feature = "LuMoveLeft")]
            LuIcon::LuMoveLeft => LU_MOVE_LEFT,
            #[cfg(feature = "LuMoveRight")]
            LuIcon::LuMoveRight => LU_MOVE_RIGHT,
            #[cfg(feature = "LuMoveUp")]
            LuIcon::LuMoveUp => LU_MOVE_UP,
            #[cfg(feature = "LuMoveUpLeft")]
            LuIcon::LuMoveUpLeft => LU_MOVE_UP_LEFT,
            #[cfg(feature = "LuMoveUpRight")]
            LuIcon::LuMoveUpRight => LU_MOVE_UP_RIGHT,
            #[cfg(feature = "LuMoveVertical")]
            LuIcon::LuMoveVertical => LU_MOVE_VERTICAL,
            #[cfg(feature = "LuMusic")]
            LuIcon::LuMusic => LU_MUSIC,
            #[cfg(feature = "LuMusic2")]
            LuIcon::LuMusic2 => LU_MUSIC2,
            #[cfg(feature = "LuMusic3")]
            LuIcon::LuMusic3 => LU_MUSIC3,
            #[cfg(feature = "LuMusic4")]
            LuIcon::LuMusic4 => LU_MUSIC4,
            #[cfg(feature = "LuNavigation")]
            LuIcon::LuNavigation => LU_NAVIGATION,
            #[cfg(feature = "LuNavigation2")]
            LuIcon::LuNavigation2 => LU_NAVIGATION2,
            #[cfg(feature = "LuNavigation2Off")]
            LuIcon::LuNavigation2Off => LU_NAVIGATION2_OFF,
            #[cfg(feature = "LuNavigationOff")]
            LuIcon::LuNavigationOff => LU_NAVIGATION_OFF,
            #[cfg(feature = "LuNetwork")]
            LuIcon::LuNetwork => LU_NETWORK,
            #[cfg(feature = "LuNewspaper")]
            LuIcon::LuNewspaper => LU_NEWSPAPER,
            #[cfg(feature = "LuNfc")]
            LuIcon::LuNfc => LU_NFC,
            #[cfg(feature = "LuNut")]
            LuIcon::LuNut => LU_NUT,
            #[cfg(feature = "LuNutOff")]
            LuIcon::LuNutOff => LU_NUT_OFF,
            #[cfg(feature = "LuOctagon")]
            LuIcon::LuOctagon => LU_OCTAGON,
            #[cfg(feature = "LuOption")]
            LuIcon::LuOption => LU_OPTION,
            #[cfg(feature = "LuOrbit")]
            LuIcon::LuOrbit => LU_ORBIT,
            #[cfg(feature = "LuOutdent")]
            LuIcon::LuOutdent => LU_OUTDENT,
            #[cfg(feature = "LuPackage")]
            LuIcon::LuPackage => LU_PACKAGE,
            #[cfg(feature = "LuPackage2")]
            LuIcon::LuPackage2 => LU_PACKAGE2,
            #[cfg(feature = "LuPackageCheck")]
            LuIcon::LuPackageCheck => LU_PACKAGE_CHECK,
            #[cfg(feature = "LuPackageMinus")]
            LuIcon::LuPackageMinus => LU_PACKAGE_MINUS,
            #[cfg(feature = "LuPackageOpen")]
            LuIcon::LuPackageOpen => LU_PACKAGE_OPEN,
            #[cfg(feature = "LuPackagePlus")]
            LuIcon::LuPackagePlus => LU_PACKAGE_PLUS,
            #[cfg(feature = "LuPackageSearch")]
            LuIcon::LuPackageSearch => LU_PACKAGE_SEARCH,
            #[cfg(feature = "LuPackageX")]
            LuIcon::LuPackageX => LU_PACKAGE_X,
            #[cfg(feature = "LuPaintBucket")]
            LuIcon::LuPaintBucket => LU_PAINT_BUCKET,
            #[cfg(feature = "LuPaintbrush")]
            LuIcon::LuPaintbrush => LU_PAINTBRUSH,
            #[cfg(feature = "LuPaintbrush2")]
            LuIcon::LuPaintbrush2 => LU_PAINTBRUSH2,
            #[cfg(feature = "LuPalette")]
            LuIcon::LuPalette => LU_PALETTE,
            #[cfg(feature = "LuPalmtree")]
            LuIcon::LuPalmtree => LU_PALMTREE,
            #[cfg(feature = "LuPanelBottom")]
            LuIcon::LuPanelBottom => LU_PANEL_BOTTOM,
            #[cfg(feature = "LuPanelBottomClose")]
            LuIcon::LuPanelBottomClose => LU_PANEL_BOTTOM_CLOSE,
            #[cfg(feature = "LuPanelBottomInactive")]
            LuIcon::LuPanelBottomInactive => LU_PANEL_BOTTOM_INACTIVE,
            #[cfg(feature = "LuPanelBottomOpen")]
            LuIcon::LuPanelBottomOpen => LU_PANEL_BOTTOM_OPEN,
            #[cfg(feature = "LuPanelLeft")]
            LuIcon::LuPanelLeft => LU_PANEL_LEFT,
            #[cfg(feature = "LuPanelLeftClose")]
            LuIcon::LuPanelLeftClose => LU_PANEL_LEFT_CLOSE,
            #[cfg(feature = "LuPanelLeftInactive")]
            LuIcon::LuPanelLeftInactive => LU_PANEL_LEFT_INACTIVE,
            #[cfg(feature = "LuPanelLeftOpen")]
            LuIcon::LuPanelLeftOpen => LU_PANEL_LEFT_OPEN,
            #[cfg(feature = "LuPanelRight")]
            LuIcon::LuPanelRight => LU_PANEL_RIGHT,
            #[cfg(feature = "LuPanelRightClose")]
            LuIcon::LuPanelRightClose => LU_PANEL_RIGHT_CLOSE,
            #[cfg(feature = "LuPanelRightInactive")]
            LuIcon::LuPanelRightInactive => LU_PANEL_RIGHT_INACTIVE,
            #[cfg(feature = "LuPanelRightOpen")]
            LuIcon::LuPanelRightOpen => LU_PANEL_RIGHT_OPEN,
            #[cfg(feature = "LuPanelTop")]
            LuIcon::LuPanelTop => LU_PANEL_TOP,
            #[cfg(feature = "LuPanelTopClose")]
            LuIcon::LuPanelTopClose => LU_PANEL_TOP_CLOSE,
            #[cfg(feature = "LuPanelTopInactive")]
            LuIcon::LuPanelTopInactive => LU_PANEL_TOP_INACTIVE,
            #[cfg(feature = "LuPanelTopOpen")]
            LuIcon::LuPanelTopOpen => LU_PANEL_TOP_OPEN,
            #[cfg(feature = "LuPaperclip")]
            LuIcon::LuPaperclip => LU_PAPERCLIP,
            #[cfg(feature = "LuParentheses")]
            LuIcon::LuParentheses => LU_PARENTHESES,
            #[cfg(feature = "LuParkingCircle")]
            LuIcon::LuParkingCircle => LU_PARKING_CIRCLE,
            #[cfg(feature = "LuParkingCircleOff")]
            LuIcon::LuParkingCircleOff => LU_PARKING_CIRCLE_OFF,
            #[cfg(feature = "LuParkingMeter")]
            LuIcon::LuParkingMeter => LU_PARKING_METER,
            #[cfg(feature = "LuParkingSquare")]
            LuIcon::LuParkingSquare => LU_PARKING_SQUARE,
            #[cfg(feature = "LuParkingSquareOff")]
            LuIcon::LuParkingSquareOff => LU_PARKING_SQUARE_OFF,
            #[cfg(feature = "LuPartyPopper")]
            LuIcon::LuPartyPopper => LU_PARTY_POPPER,
            #[cfg(feature = "LuPause")]
            LuIcon::LuPause => LU_PAUSE,
            #[cfg(feature = "LuPauseCircle")]
            LuIcon::LuPauseCircle => LU_PAUSE_CIRCLE,
            #[cfg(feature = "LuPauseOctagon")]
            LuIcon::LuPauseOctagon => LU_PAUSE_OCTAGON,
            #[cfg(feature = "LuPawPrint")]
            LuIcon::LuPawPrint => LU_PAW_PRINT,
            #[cfg(feature = "LuPcCase")]
            LuIcon::LuPcCase => LU_PC_CASE,
            #[cfg(feature = "LuPen")]
            LuIcon::LuPen => LU_PEN,
            #[cfg(feature = "LuPenLine")]
            LuIcon::LuPenLine => LU_PEN_LINE,
            #[cfg(feature = "LuPenSquare")]
            LuIcon::LuPenSquare => LU_PEN_SQUARE,
            #[cfg(feature = "LuPenTool")]
            LuIcon::LuPenTool => LU_PEN_TOOL,
            #[cfg(feature = "LuPencil")]
            LuIcon::LuPencil => LU_PENCIL,
            #[cfg(feature = "LuPencilLine")]
            LuIcon::LuPencilLine => LU_PENCIL_LINE,
            #[cfg(feature = "LuPencilRuler")]
            LuIcon::LuPencilRuler => LU_PENCIL_RULER,
            #[cfg(feature = "LuPercent")]
            LuIcon::LuPercent => LU_PERCENT,
            #[cfg(feature = "LuPersonStanding")]
            LuIcon::LuPersonStanding => LU_PERSON_STANDING,
            #[cfg(feature = "LuPhone")]
            LuIcon::LuPhone => LU_PHONE,
            #[cfg(feature = "LuPhoneCall")]
            LuIcon::LuPhoneCall => LU_PHONE_CALL,
            #[cfg(feature = "LuPhoneForwarded")]
            LuIcon::LuPhoneForwarded => LU_PHONE_FORWARDED,
            #[cfg(feature = "LuPhoneIncoming")]
            LuIcon::LuPhoneIncoming => LU_PHONE_INCOMING,
            #[cfg(feature = "LuPhoneMissed")]
            LuIcon::LuPhoneMissed => LU_PHONE_MISSED,
            #[cfg(feature = "LuPhoneOff")]
            LuIcon::LuPhoneOff => LU_PHONE_OFF,
            #[cfg(feature = "LuPhoneOutgoing")]
            LuIcon::LuPhoneOutgoing => LU_PHONE_OUTGOING,
            #[cfg(feature = "LuPi")]
            LuIcon::LuPi => LU_PI,
            #[cfg(feature = "LuPiSquare")]
            LuIcon::LuPiSquare => LU_PI_SQUARE,
            #[cfg(feature = "LuPictureInPicture")]
            LuIcon::LuPictureInPicture => LU_PICTURE_IN_PICTURE,
            #[cfg(feature = "LuPictureInPicture2")]
            LuIcon::LuPictureInPicture2 => LU_PICTURE_IN_PICTURE2,
            #[cfg(feature = "LuPieChart")]
            LuIcon::LuPieChart => LU_PIE_CHART,
            #[cfg(feature = "LuPiggyBank")]
            LuIcon::LuPiggyBank => LU_PIGGY_BANK,
            #[cfg(feature = "LuPilcrow")]
            LuIcon::LuPilcrow => LU_PILCROW,
            #[cfg(feature = "LuPilcrowSquare")]
            LuIcon::LuPilcrowSquare => LU_PILCROW_SQUARE,
            #[cfg(feature = "LuPill")]
            LuIcon::LuPill => LU_PILL,
            #[cfg(feature = "LuPin")]
            LuIcon::LuPin => LU_PIN,
            #[cfg(feature = "LuPinOff")]
            LuIcon::LuPinOff => LU_PIN_OFF,
            #[cfg(feature = "LuPipette")]
            LuIcon::LuPipette => LU_PIPETTE,
            #[cfg(feature = "LuPizza")]
            LuIcon::LuPizza => LU_PIZZA,
            #[cfg(feature = "LuPlane")]
            LuIcon::LuPlane => LU_PLANE,
            #[cfg(feature = "LuPlaneLanding")]
            LuIcon::LuPlaneLanding => LU_PLANE_LANDING,
            #[cfg(feature = "LuPlaneTakeoff")]
            LuIcon::LuPlaneTakeoff => LU_PLANE_TAKEOFF,
            #[cfg(feature = "LuPlay")]
            LuIcon::LuPlay => LU_PLAY,
            #[cfg(feature = "LuPlayCircle")]
            LuIcon::LuPlayCircle => LU_PLAY_CIRCLE,
            #[cfg(feature = "LuPlaySquare")]
            LuIcon::LuPlaySquare => LU_PLAY_SQUARE,
            #[cfg(feature = "LuPlug")]
            LuIcon::LuPlug => LU_PLUG,
            #[cfg(feature = "LuPlug2")]
            LuIcon::LuPlug2 => LU_PLUG2,
            #[cfg(feature = "LuPlugZap")]
            LuIcon::LuPlugZap => LU_PLUG_ZAP,
            #[cfg(feature = "LuPlugZap2")]
            LuIcon::LuPlugZap2 => LU_PLUG_ZAP2,
            #[cfg(feature = "LuPlus")]
            LuIcon::LuPlus => LU_PLUS,
            #[cfg(feature = "LuPlusCircle")]
            LuIcon::LuPlusCircle => LU_PLUS_CIRCLE,
            #[cfg(feature = "LuPlusSquare")]
            LuIcon::LuPlusSquare => LU_PLUS_SQUARE,
            #[cfg(feature = "LuPocket")]
            LuIcon::LuPocket => LU_POCKET,
            #[cfg(feature = "LuPocketKnife")]
            LuIcon::LuPocketKnife => LU_POCKET_KNIFE,
            #[cfg(feature = "LuPodcast")]
            LuIcon::LuPodcast => LU_PODCAST,
            #[cfg(feature = "LuPointer")]
            LuIcon::LuPointer => LU_POINTER,
            #[cfg(feature = "LuPopcorn")]
            LuIcon::LuPopcorn => LU_POPCORN,
            #[cfg(feature = "LuPopsicle")]
            LuIcon::LuPopsicle => LU_POPSICLE,
            #[cfg(feature = "LuPoundSterling")]
            LuIcon::LuPoundSterling => LU_POUND_STERLING,
            #[cfg(feature = "LuPower")]
            LuIcon::LuPower => LU_POWER,
            #[cfg(feature = "LuPowerOff")]
            LuIcon::LuPowerOff => LU_POWER_OFF,
            #[cfg(feature = "LuPresentation")]
            LuIcon::LuPresentation => LU_PRESENTATION,
            #[cfg(feature = "LuPrinter")]
            LuIcon::LuPrinter => LU_PRINTER,
            #[cfg(feature = "LuProjector")]
            LuIcon::LuProjector => LU_PROJECTOR,
            #[cfg(feature = "LuPuzzle")]
            LuIcon::LuPuzzle => LU_PUZZLE,
            #[cfg(feature = "LuQrCode")]
            LuIcon::LuQrCode => LU_QR_CODE,
            #[cfg(feature = "LuQuote")]
            LuIcon::LuQuote => LU_QUOTE,
            #[cfg(feature = "LuRabbit")]
            LuIcon::LuRabbit => LU_RABBIT,
            #[cfg(feature = "LuRadar")]
            LuIcon::LuRadar => LU_RADAR,
            #[cfg(feature = "LuRadiation")]
            LuIcon::LuRadiation => LU_RADIATION,
            #[cfg(feature = "LuRadio")]
            LuIcon::LuRadio => LU_RADIO,
            #[cfg(feature = "LuRadioReceiver")]
            LuIcon::LuRadioReceiver => LU_RADIO_RECEIVER,
            #[cfg(feature = "LuRadioTower")]
            LuIcon::LuRadioTower => LU_RADIO_TOWER,
            #[cfg(feature = "LuRailSymbol")]
            LuIcon::LuRailSymbol => LU_RAIL_SYMBOL,
            #[cfg(feature = "LuRainbow")]
            LuIcon::LuRainbow => LU_RAINBOW,
            #[cfg(feature = "LuRat")]
            LuIcon::LuRat => LU_RAT,
            #[cfg(feature = "LuRatio")]
            LuIcon::LuRatio => LU_RATIO,
            #[cfg(feature = "LuReceipt")]
            LuIcon::LuReceipt => LU_RECEIPT,
            #[cfg(feature = "LuRectangleHorizontal")]
            LuIcon::LuRectangleHorizontal => LU_RECTANGLE_HORIZONTAL,
            #[cfg(feature = "LuRectangleVertical")]
            LuIcon::LuRectangleVertical => LU_RECTANGLE_VERTICAL,
            #[cfg(feature = "LuRecycle")]
            LuIcon::LuRecycle => LU_RECYCLE,
            #[cfg(feature = "LuRedo")]
            LuIcon::LuRedo => LU_REDO,
            #[cfg(feature = "LuRedo2")]
            LuIcon::LuRedo2 => LU_REDO2,
            #[cfg(feature = "LuRedoDot")]
            LuIcon::LuRedoDot => LU_REDO_DOT,
            #[cfg(feature = "LuRefreshCcw")]
            LuIcon::LuRefreshCcw => LU_REFRESH_CCW,
            #[cfg(feature = "LuRefreshCcwDot")]
            LuIcon::LuRefreshCcwDot => LU_REFRESH_CCW_DOT,
            #[cfg(feature = "LuRefreshCw")]
            LuIcon::LuRefreshCw => LU_REFRESH_CW,
            #[cfg(feature = "LuRefreshCwOff")]
            LuIcon::LuRefreshCwOff => LU_REFRESH_CW_OFF,
            #[cfg(feature = "LuRefrigerator")]
            LuIcon::LuRefrigerator => LU_REFRIGERATOR,
            #[cfg(feature = "LuRegex")]
            LuIcon::LuRegex => LU_REGEX,
            #[cfg(feature = "LuRemoveFormatting")]
            LuIcon::LuRemoveFormatting => LU_REMOVE_FORMATTING,
            #[cfg(feature = "LuRepeat")]
            LuIcon::LuRepeat => LU_REPEAT,
            #[cfg(feature = "LuRepeat1")]
            LuIcon::LuRepeat1 => LU_REPEAT1,
            #[cfg(feature = "LuRepeat2")]
            LuIcon::LuRepeat2 => LU_REPEAT2,
            #[cfg(feature = "LuReplace")]
            LuIcon::LuReplace => LU_REPLACE,
            #[cfg(feature = "LuReplaceAll")]
            LuIcon::LuReplaceAll => LU_REPLACE_ALL,
            #[cfg(feature = "LuReply")]
            LuIcon::LuReply => LU_REPLY,
            #[cfg(feature = "LuReplyAll")]
            LuIcon::LuReplyAll => LU_REPLY_ALL,
            #[cfg(feature = "LuRewind")]
            LuIcon::LuRewind => LU_REWIND,
            #[cfg(feature = "LuRocket")]
            LuIcon::LuRocket => LU_ROCKET,
            #[cfg(feature = "LuRockingChair")]
            LuIcon::LuRockingChair => LU_ROCKING_CHAIR,
            #[cfg(feature = "LuRollerCoaster")]
            LuIcon::LuRollerCoaster => LU_ROLLER_COASTER,
            #[cfg(feature = "LuRotate3d")]
            LuIcon::LuRotate3d => LU_ROTATE3D,
            #[cfg(feature = "LuRotateCcw")]
            LuIcon::LuRotateCcw => LU_ROTATE_CCW,
            #[cfg(feature = "LuRotateCw")]
            LuIcon::LuRotateCw => LU_ROTATE_CW,
            #[cfg(feature = "LuRouter")]
            LuIcon::LuRouter => LU_ROUTER,
            #[cfg(feature = "LuRows")]
            LuIcon::LuRows => LU_ROWS,
            #[cfg(feature = "LuRss")]
            LuIcon::LuRss => LU_RSS,
            #[cfg(feature = "LuRuler")]
            LuIcon::LuRuler => LU_RULER,
            #[cfg(feature = "LuRussianRuble")]
            LuIcon::LuRussianRuble => LU_RUSSIAN_RUBLE,
            #[cfg(feature = "LuSailboat")]
            LuIcon::LuSailboat => LU_SAILBOAT,
            #[cfg(feature = "LuSalad")]
            LuIcon::LuSalad => LU_SALAD,
            #[cfg(feature = "LuSandwich")]
            LuIcon::LuSandwich => LU_SANDWICH,
            #[cfg(feature = "LuSatellite")]
            LuIcon::LuSatellite => LU_SATELLITE,
            #[cfg(feature = "LuSatelliteDish")]
            LuIcon::LuSatelliteDish => LU_SATELLITE_DISH,
            #[cfg(feature = "LuSave")]
            LuIcon::LuSave => LU_SAVE,
            #[cfg(feature = "LuSaveAll")]
            LuIcon::LuSaveAll => LU_SAVE_ALL,
            #[cfg(feature = "LuScale")]
            LuIcon::LuScale => LU_SCALE,
            #[cfg(feature = "LuScale3d")]
            LuIcon::LuScale3d => LU_SCALE3D,
            #[cfg(feature = "LuScaling")]
            LuIcon::LuScaling => LU_SCALING,
            #[cfg(feature = "LuScan")]
            LuIcon::LuScan => LU_SCAN,
            #[cfg(feature = "LuScanFace")]
            LuIcon::LuScanFace => LU_SCAN_FACE,
            #[cfg(feature = "LuScanLine")]
            LuIcon::LuScanLine => LU_SCAN_LINE,
            #[cfg(feature = "LuScatterChart")]
            LuIcon::LuScatterChart => LU_SCATTER_CHART,
            #[cfg(feature = "LuSchool")]
            LuIcon::LuSchool => LU_SCHOOL,
            #[cfg(feature = "LuSchool2")]
            LuIcon::LuSchool2 => LU_SCHOOL2,
            #[cfg(feature = "LuScissors")]
            LuIcon::LuScissors => LU_SCISSORS,
            #[cfg(feature = "LuScissorsLineDashed")]
            LuIcon::LuScissorsLineDashed => LU_SCISSORS_LINE_DASHED,
            #[cfg(feature = "LuScissorsSquare")]
            LuIcon::LuScissorsSquare => LU_SCISSORS_SQUARE,
            #[cfg(feature = "LuScissorsSquareDashedBottom")]
            LuIcon::LuScissorsSquareDashedBottom => LU_SCISSORS_SQUARE_DASHED_BOTTOM,
            #[cfg(feature = "LuScreenShare")]
            LuIcon::LuScreenShare => LU_SCREEN_SHARE,
            #[cfg(feature = "LuScreenShareOff")]
            LuIcon::LuScreenShareOff => LU_SCREEN_SHARE_OFF,
            #[cfg(feature = "LuScroll")]
            LuIcon::LuScroll => LU_SCROLL,
            #[cfg(feature = "LuScrollText")]
            LuIcon::LuScrollText => LU_SCROLL_TEXT,
            #[cfg(feature = "LuSearch")]
            LuIcon::LuSearch => LU_SEARCH,
            #[cfg(feature = "LuSearchCheck")]
            LuIcon::LuSearchCheck => LU_SEARCH_CHECK,
            #[cfg(feature = "LuSearchCode")]
            LuIcon::LuSearchCode => LU_SEARCH_CODE,
            #[cfg(feature = "LuSearchSlash")]
            LuIcon::LuSearchSlash => LU_SEARCH_SLASH,
            #[cfg(feature = "LuSearchX")]
            LuIcon::LuSearchX => LU_SEARCH_X,
            #[cfg(feature = "LuSend")]
            LuIcon::LuSend => LU_SEND,
            #[cfg(feature = "LuSendHorizonal")]
            LuIcon::LuSendHorizonal => LU_SEND_HORIZONAL,
            #[cfg(feature = "LuSendToBack")]
            LuIcon::LuSendToBack => LU_SEND_TO_BACK,
            #[cfg(feature = "LuSeparatorHorizontal")]
            LuIcon::LuSeparatorHorizontal => LU_SEPARATOR_HORIZONTAL,
            #[cfg(feature = "LuSeparatorVertical")]
            LuIcon::LuSeparatorVertical => LU_SEPARATOR_VERTICAL,
            #[cfg(feature = "LuServer")]
            LuIcon::LuServer => LU_SERVER,
            #[cfg(feature = "LuServerCog")]
            LuIcon::LuServerCog => LU_SERVER_COG,
            #[cfg(feature = "LuServerCrash")]
            LuIcon::LuServerCrash => LU_SERVER_CRASH,
            #[cfg(feature = "LuServerOff")]
            LuIcon::LuServerOff => LU_SERVER_OFF,
            #[cfg(feature = "LuSettings")]
            LuIcon::LuSettings => LU_SETTINGS,
            #[cfg(feature = "LuSettings2")]
            LuIcon::LuSettings2 => LU_SETTINGS2,
            #[cfg(feature = "LuShapes")]
            LuIcon::LuShapes => LU_SHAPES,
            #[cfg(feature = "LuShare")]
            LuIcon::LuShare => LU_SHARE,
            #[cfg(feature = "LuShare2")]
            LuIcon::LuShare2 => LU_SHARE2,
            #[cfg(feature = "LuSheet")]
            LuIcon::LuSheet => LU_SHEET,
            #[cfg(feature = "LuShell")]
            LuIcon::LuShell => LU_SHELL,
            #[cfg(feature = "LuShield")]
            LuIcon::LuShield => LU_SHIELD,
            #[cfg(feature = "LuShieldAlert")]
            LuIcon::LuShieldAlert => LU_SHIELD_ALERT,
            #[cfg(feature = "LuShieldCheck")]
            LuIcon::LuShieldCheck => LU_SHIELD_CHECK,
            #[cfg(feature = "LuShieldClose")]
            LuIcon::LuShieldClose => LU_SHIELD_CLOSE,
            #[cfg(feature = "LuShieldOff")]
            LuIcon::LuShieldOff => LU_SHIELD_OFF,
            #[cfg(feature = "LuShieldQuestion")]
            LuIcon::LuShieldQuestion => LU_SHIELD_QUESTION,
            #[cfg(feature = "LuShip")]
            LuIcon::LuShip => LU_SHIP,
            #[cfg(feature = "LuShipWheel")]
            LuIcon::LuShipWheel => LU_SHIP_WHEEL,
            #[cfg(feature = "LuShirt")]
            LuIcon::LuShirt => LU_SHIRT,
            #[cfg(feature = "LuShoppingBag")]
            LuIcon::LuShoppingBag => LU_SHOPPING_BAG,
            #[cfg(feature = "LuShoppingBasket")]
            LuIcon::LuShoppingBasket => LU_SHOPPING_BASKET,
            #[cfg(feature = "LuShoppingCart")]
            LuIcon::LuShoppingCart => LU_SHOPPING_CART,
            #[cfg(feature = "LuShovel")]
            LuIcon::LuShovel => LU_SHOVEL,
            #[cfg(feature = "LuShowerHead")]
            LuIcon::LuShowerHead => LU_SHOWER_HEAD,
            #[cfg(feature = "LuShrink")]
            LuIcon::LuShrink => LU_SHRINK,
            #[cfg(feature = "LuShrub")]
            LuIcon::LuShrub => LU_SHRUB,
            #[cfg(feature = "LuShuffle")]
            LuIcon::LuShuffle => LU_SHUFFLE,
            #[cfg(feature = "LuSigma")]
            LuIcon::LuSigma => LU_SIGMA,
            #[cfg(feature = "LuSigmaSquare")]
            LuIcon::LuSigmaSquare => LU_SIGMA_SQUARE,
            #[cfg(feature = "LuSignal")]
            LuIcon::LuSignal => LU_SIGNAL,
            #[cfg(feature = "LuSignalHigh")]
            LuIcon::LuSignalHigh => LU_SIGNAL_HIGH,
            #[cfg(feature = "LuSignalLow")]
            LuIcon::LuSignalLow => LU_SIGNAL_LOW,
            #[cfg(feature = "LuSignalMedium")]
            LuIcon::LuSignalMedium => LU_SIGNAL_MEDIUM,
            #[cfg(feature = "LuSignalZero")]
            LuIcon::LuSignalZero => LU_SIGNAL_ZERO,
            #[cfg(feature = "LuSiren")]
            LuIcon::LuSiren => LU_SIREN,
            #[cfg(feature = "LuSkipBack")]
            LuIcon::LuSkipBack => LU_SKIP_BACK,
            #[cfg(feature = "LuSkipForward")]
            LuIcon::LuSkipForward => LU_SKIP_FORWARD,
            #[cfg(feature = "LuSkull")]
            LuIcon::LuSkull => LU_SKULL,
            #[cfg(feature = "LuSlack")]
            LuIcon::LuSlack => LU_SLACK,
            #[cfg(feature = "LuSlice")]
            LuIcon::LuSlice => LU_SLICE,
            #[cfg(feature = "LuSliders")]
            LuIcon::LuSliders => LU_SLIDERS,
            #[cfg(feature = "LuSlidersHorizontal")]
            LuIcon::LuSlidersHorizontal => LU_SLIDERS_HORIZONTAL,
            #[cfg(feature = "LuSmartphone")]
            LuIcon::LuSmartphone => LU_SMARTPHONE,
            #[cfg(feature = "LuSmartphoneCharging")]
            LuIcon::LuSmartphoneCharging => LU_SMARTPHONE_CHARGING,
            #[cfg(feature = "LuSmartphoneNfc")]
            LuIcon::LuSmartphoneNfc => LU_SMARTPHONE_NFC,
            #[cfg(feature = "LuSmile")]
            LuIcon::LuSmile => LU_SMILE,
            #[cfg(feature = "LuSmilePlus")]
            LuIcon::LuSmilePlus => LU_SMILE_PLUS,
            #[cfg(feature = "LuSnail")]
            LuIcon::LuSnail => LU_SNAIL,
            #[cfg(feature = "LuSnowflake")]
            LuIcon::LuSnowflake => LU_SNOWFLAKE,
            #[cfg(feature = "LuSofa")]
            LuIcon::LuSofa => LU_SOFA,
            #[cfg(feature = "LuSoup")]
            LuIcon::LuSoup => LU_SOUP,
            #[cfg(feature = "LuSpace")]
            LuIcon::LuSpace => LU_SPACE,
            #[cfg(feature = "LuSpade")]
            LuIcon::LuSpade => LU_SPADE,
            #[cfg(feature = "LuSparkle")]
            LuIcon::LuSparkle => LU_SPARKLE,
            #[cfg(feature = "LuSparkles")]
            LuIcon::LuSparkles => LU_SPARKLES,
            #[cfg(feature = "LuSpeaker")]
            LuIcon::LuSpeaker => LU_SPEAKER,
            #[cfg(feature = "LuSpellCheck")]
            LuIcon::LuSpellCheck => LU_SPELL_CHECK,
            #[cfg(feature = "LuSpellCheck2")]
            LuIcon::LuSpellCheck2 => LU_SPELL_CHECK2,
            #[cfg(feature = "LuSpline")]
            LuIcon::LuSpline => LU_SPLINE,
            #[cfg(feature = "LuSplit")]
            LuIcon::LuSplit => LU_SPLIT,
            #[cfg(feature = "LuSplitSquareHorizontal")]
            LuIcon::LuSplitSquareHorizontal => LU_SPLIT_SQUARE_HORIZONTAL,
            #[cfg(feature = "LuSplitSquareVertical")]
            LuIcon::LuSplitSquareVertical => LU_SPLIT_SQUARE_VERTICAL,
            #[cfg(feature = "LuSprayCan")]
            LuIcon::LuSprayCan => LU_SPRAY_CAN,
            #[cfg(feature = "LuSprout")]
            LuIcon::LuSprout => LU_SPROUT,
            #[cfg(feature = "LuSquare")]
            LuIcon::LuSquare => LU_SQUARE,
            #[cfg(feature = "LuSquareAsterisk")]
            LuIcon::LuSquareAsterisk => LU_SQUARE_ASTERISK,
            #[cfg(feature = "LuSquareCode")]
            LuIcon::LuSquareCode => LU_SQUARE_CODE,
            #[cfg(feature = "LuSquareDashedBottom")]
            LuIcon::LuSquareDashedBottom => LU_SQUARE_DASHED_BOTTOM,
            #[cfg(feature = "LuSquareDashedBottomCode")]
            LuIcon::LuSquareDashedBottomCode => LU_SQUARE_DASHED_BOTTOM_CODE,
            #[cfg(feature = "LuSquareDot")]
            LuIcon::LuSquareDot => LU_SQUARE_DOT,
            #[cfg(feature = "LuSquareEqual")]
            LuIcon::LuSquareEqual => LU_SQUARE_EQUAL,
            #[cfg(feature = "LuSquareSlash")]
            LuIcon::LuSquareSlash => LU_SQUARE_SLASH,
            #[cfg(feature = "LuSquareStack")]
            LuIcon::LuSquareStack => LU_SQUARE_STACK,
            #[cfg(feature = "LuSquirrel")]
            LuIcon::LuSquirrel => LU_SQUIRREL,
            #[cfg(feature = "LuStamp")]
            LuIcon::LuStamp => LU_STAMP,
            #[cfg(feature = "LuStar")]
            LuIcon::LuStar => LU_STAR,
            #[cfg(feature = "LuStarHalf")]
            LuIcon::LuStarHalf => LU_STAR_HALF,
            #[cfg(feature = "LuStarOff")]
            LuIcon::LuStarOff => LU_STAR_OFF,
            #[cfg(feature = "LuStepBack")]
            LuIcon::LuStepBack => LU_STEP_BACK,
            #[cfg(feature = "LuStepForward")]
            LuIcon::LuStepForward => LU_STEP_FORWARD,
            #[cfg(feature = "LuStethoscope")]
            LuIcon::LuStethoscope => LU_STETHOSCOPE,
            #[cfg(feature = "LuSticker")]
            LuIcon::LuSticker => LU_STICKER,
            #[cfg(feature = "LuStickyNote")]
            LuIcon::LuStickyNote => LU_STICKY_NOTE,
            #[cfg(feature = "LuStopCircle")]
            LuIcon::LuStopCircle => LU_STOP_CIRCLE,
            #[cfg(feature = "LuStore")]
            LuIcon::LuStore => LU_STORE,
            #[cfg(feature = "LuStretchHorizontal")]
            LuIcon::LuStretchHorizontal => LU_STRETCH_HORIZONTAL,
            #[cfg(feature = "LuStretchVertical")]
            LuIcon::LuStretchVertical => LU_STRETCH_VERTICAL,
            #[cfg(feature = "LuStrikethrough")]
            LuIcon::LuStrikethrough => LU_STRIKETHROUGH,
            #[cfg(feature = "LuSubscript")]
            LuIcon::LuSubscript => LU_SUBSCRIPT,
            #[cfg(feature = "LuSubtitles")]
            LuIcon::LuSubtitles => LU_SUBTITLES,
            #[cfg(feature = "LuSun")]
            LuIcon::LuSun => LU_SUN,
            #[cfg(feature = "LuSunDim")]
            LuIcon::LuSunDim => LU_SUN_DIM,
            #[cfg(feature = "LuSunMedium")]
            LuIcon::LuSunMedium => LU_SUN_MEDIUM,
            #[cfg(feature = "LuSunMoon")]
            LuIcon::LuSunMoon => LU_SUN_MOON,
            #[cfg(feature = "LuSunSnow")]
            LuIcon::LuSunSnow => LU_SUN_SNOW,
            #[cfg(feature = "LuSunrise")]
            LuIcon::LuSunrise => LU_SUNRISE,
            #[cfg(feature = "LuSunset")]
            LuIcon::LuSunset => LU_SUNSET,
            #[cfg(feature = "LuSuperscript")]
            LuIcon::LuSuperscript => LU_SUPERSCRIPT,
            #[cfg(feature = "LuSwissFranc")]
            LuIcon::LuSwissFranc => LU_SWISS_FRANC,
            #[cfg(feature = "LuSwitchCamera")]
            LuIcon::LuSwitchCamera => LU_SWITCH_CAMERA,
            #[cfg(feature = "LuSword")]
            LuIcon::LuSword => LU_SWORD,
            #[cfg(feature = "LuSwords")]
            LuIcon::LuSwords => LU_SWORDS,
            #[cfg(feature = "LuSyringe")]
            LuIcon::LuSyringe => LU_SYRINGE,
            #[cfg(feature = "LuTable")]
            LuIcon::LuTable => LU_TABLE,
            #[cfg(feature = "LuTable2")]
            LuIcon::LuTable2 => LU_TABLE2,
            #[cfg(feature = "LuTableProperties")]
            LuIcon::LuTableProperties => LU_TABLE_PROPERTIES,
            #[cfg(feature = "LuTablet")]
            LuIcon::LuTablet => LU_TABLET,
            #[cfg(feature = "LuTablets")]
            LuIcon::LuTablets => LU_TABLETS,
            #[cfg(feature = "LuTag")]
            LuIcon::LuTag => LU_TAG,
            #[cfg(feature = "LuTags")]
            LuIcon::LuTags => LU_TAGS,
            #[cfg(feature = "LuTally1")]
            LuIcon::LuTally1 => LU_TALLY1,
            #[cfg(feature = "LuTally2")]
            LuIcon::LuTally2 => LU_TALLY2,
            #[cfg(feature = "LuTally3")]
            LuIcon::LuTally3 => LU_TALLY3,
            #[cfg(feature = "LuTally4")]
            LuIcon::LuTally4 => LU_TALLY4,
            #[cfg(feature = "LuTally5")]
            LuIcon::LuTally5 => LU_TALLY5,
            #[cfg(feature = "LuTarget")]
            LuIcon::LuTarget => LU_TARGET,
            #[cfg(feature = "LuTent")]
            LuIcon::LuTent => LU_TENT,
            #[cfg(feature = "LuTerminal")]
            LuIcon::LuTerminal => LU_TERMINAL,
            #[cfg(feature = "LuTerminalSquare")]
            LuIcon::LuTerminalSquare => LU_TERMINAL_SQUARE,
            #[cfg(feature = "LuTestTube")]
            LuIcon::LuTestTube => LU_TEST_TUBE,
            #[cfg(feature = "LuTestTube2")]
            LuIcon::LuTestTube2 => LU_TEST_TUBE2,
            #[cfg(feature = "LuTestTubes")]
            LuIcon::LuTestTubes => LU_TEST_TUBES,
            #[cfg(feature = "LuText")]
            LuIcon::LuText => LU_TEXT,
            #[cfg(feature = "LuTextCursor")]
            LuIcon::LuTextCursor => LU_TEXT_CURSOR,
            #[cfg(feature = "LuTextCursorInput")]
            LuIcon::LuTextCursorInput => LU_TEXT_CURSOR_INPUT,
            #[cfg(feature = "LuTextQuote")]
            LuIcon::LuTextQuote => LU_TEXT_QUOTE,
            #[cfg(feature = "LuTextSelect")]
            LuIcon::LuTextSelect => LU_TEXT_SELECT,
            #[cfg(feature = "LuThermometer")]
            LuIcon::LuThermometer => LU_THERMOMETER,
            #[cfg(feature = "LuThermometerSnowflake")]
            LuIcon::LuThermometerSnowflake => LU_THERMOMETER_SNOWFLAKE,
            #[cfg(feature = "LuThermometerSun")]
            LuIcon::LuThermometerSun => LU_THERMOMETER_SUN,
            #[cfg(feature = "LuThumbsDown")]
            LuIcon::LuThumbsDown => LU_THUMBS_DOWN,
            #[cfg(feature = "LuThumbsUp")]
            LuIcon::LuThumbsUp => LU_THUMBS_UP,
            #[cfg(feature = "LuTicket")]
            LuIcon::LuTicket => LU_TICKET,
            #[cfg(feature = "LuTimer")]
            LuIcon::LuTimer => LU_TIMER,
            #[cfg(feature = "LuTimerOff")]
            LuIcon::LuTimerOff => LU_TIMER_OFF,
            #[cfg(feature = "LuTimerReset")]
            LuIcon::LuTimerReset => LU_TIMER_RESET,
            #[cfg(feature = "LuToggleLeft")]
            LuIcon::LuToggleLeft => LU_TOGGLE_LEFT,
            #[cfg(feature = "LuToggleRight")]
            LuIcon::LuToggleRight => LU_TOGGLE_RIGHT,
            #[cfg(feature = "LuTornado")]
            LuIcon::LuTornado => LU_TORNADO,
            #[cfg(feature = "LuTouchpad")]
            LuIcon::LuTouchpad => LU_TOUCHPAD,
            #[cfg(feature = "LuTouchpadOff")]
            LuIcon::LuTouchpadOff => LU_TOUCHPAD_OFF,
            #[cfg(feature = "LuTowerControl")]
            LuIcon::LuTowerControl => LU_TOWER_CONTROL,
            #[cfg(feature = "LuToyBrick")]
            LuIcon::LuToyBrick => LU_TOY_BRICK,
            #[cfg(feature = "LuTractor")]
            LuIcon::LuTractor => LU_TRACTOR,
            #[cfg(feature = "LuTrafficCone")]
            LuIcon::LuTrafficCone => LU_TRAFFIC_CONE,
            #[cfg(feature = "LuTrainFront")]
            LuIcon::LuTrainFront => LU_TRAIN_FRONT,
            #[cfg(feature = "LuTrainFrontTunnel")]
            LuIcon::LuTrainFrontTunnel => LU_TRAIN_FRONT_TUNNEL,
            #[cfg(feature = "LuTrainTrack")]
            LuIcon::LuTrainTrack => LU_TRAIN_TRACK,
            #[cfg(feature = "LuTramFront")]
            LuIcon::LuTramFront => LU_TRAM_FRONT,
            #[cfg(feature = "LuTrash")]
            LuIcon::LuTrash => LU_TRASH,
            #[cfg(feature = "LuTrash2")]
            LuIcon::LuTrash2 => LU_TRASH2,
            #[cfg(feature = "LuTreeDeciduous")]
            LuIcon::LuTreeDeciduous => LU_TREE_DECIDUOUS,
            #[cfg(feature = "LuTreePine")]
            LuIcon::LuTreePine => LU_TREE_PINE,
            #[cfg(feature = "LuTrees")]
            LuIcon::LuTrees => LU_TREES,
            #[cfg(feature = "LuTrello")]
            LuIcon::LuTrello => LU_TRELLO,
            #[cfg(feature = "LuTrendingDown")]
            LuIcon::LuTrendingDown => LU_TRENDING_DOWN,
            #[cfg(feature = "LuTrendingUp")]
            LuIcon::LuTrendingUp => LU_TRENDING_UP,
            #[cfg(feature = "LuTriangle")]
            LuIcon::LuTriangle => LU_TRIANGLE,
            #[cfg(feature = "LuTriangleRight")]
            LuIcon::LuTriangleRight => LU_TRIANGLE_RIGHT,
            #[cfg(feature = "LuTrophy")]
            LuIcon::LuTrophy => LU_TROPHY,
            #[cfg(feature = "LuTruck")]
            LuIcon::LuTruck => LU_TRUCK,
            #[cfg(feature = "LuTurtle")]
            LuIcon::LuTurtle => LU_TURTLE,
            #[cfg(feature = "LuTv")]
            LuIcon::LuTv => LU_TV,
            #[cfg(feature = "LuTv2")]
            LuIcon::LuTv2 => LU_TV2,
            #[cfg(feature = "LuTwitch")]
            LuIcon::LuTwitch => LU_TWITCH,
            #[cfg(feature = "LuTwitter")]
            LuIcon::LuTwitter => LU_TWITTER,
            #[cfg(feature = "LuType")]
            LuIcon::LuType => LU_TYPE,
            #[cfg(feature = "LuUmbrella")]
            LuIcon::LuUmbrella => LU_UMBRELLA,
            #[cfg(feature = "LuUnderline")]
            LuIcon::LuUnderline => LU_UNDERLINE,
            #[cfg(feature = "LuUndo")]
            LuIcon::LuUndo => LU_UNDO,
            #[cfg(feature = "LuUndo2")]
            LuIcon::LuUndo2 => LU_UNDO2,
            #[cfg(feature = "LuUndoDot")]
            LuIcon::LuUndoDot => LU_UNDO_DOT,
            #[cfg(feature = "LuUnfoldHorizontal")]
            LuIcon::LuUnfoldHorizontal => LU_UNFOLD_HORIZONTAL,
            #[cfg(feature = "LuUnfoldVertical")]
            LuIcon::LuUnfoldVertical => LU_UNFOLD_VERTICAL,
            #[cfg(feature = "LuUngroup")]
            LuIcon::LuUngroup => LU_UNGROUP,
            #[cfg(feature = "LuUnlink")]
            LuIcon::LuUnlink => LU_UNLINK,
            #[cfg(feature = "LuUnlink2")]
            LuIcon::LuUnlink2 => LU_UNLINK2,
            #[cfg(feature = "LuUnlock")]
            LuIcon::LuUnlock => LU_UNLOCK,
            #[cfg(feature = "LuUnplug")]
            LuIcon::LuUnplug => LU_UNPLUG,
            #[cfg(feature = "LuUpload")]
            LuIcon::LuUpload => LU_UPLOAD,
            #[cfg(feature = "LuUploadCloud")]
            LuIcon::LuUploadCloud => LU_UPLOAD_CLOUD,
            #[cfg(feature = "LuUsb")]
            LuIcon::LuUsb => LU_USB,
            #[cfg(feature = "LuUser")]
            LuIcon::LuUser => LU_USER,
            #[cfg(feature = "LuUser2")]
            LuIcon::LuUser2 => LU_USER2,
            #[cfg(feature = "LuUserCheck")]
            LuIcon::LuUserCheck => LU_USER_CHECK,
            #[cfg(feature = "LuUserCheck2")]
            LuIcon::LuUserCheck2 => LU_USER_CHECK2,
            #[cfg(feature = "LuUserCircle")]
            LuIcon::LuUserCircle => LU_USER_CIRCLE,
            #[cfg(feature = "LuUserCircle2")]
            LuIcon::LuUserCircle2 => LU_USER_CIRCLE2,
            #[cfg(feature = "LuUserCog")]
            LuIcon::LuUserCog => LU_USER_COG,
            #[cfg(feature = "LuUserCog2")]
            LuIcon::LuUserCog2 => LU_USER_COG2,
            #[cfg(feature = "LuUserMinus")]
            LuIcon::LuUserMinus => LU_USER_MINUS,
            #[cfg(feature = "LuUserMinus2")]
            LuIcon::LuUserMinus2 => LU_USER_MINUS2,
            #[cfg(feature = "LuUserPlus")]
            LuIcon::LuUserPlus => LU_USER_PLUS,
            #[cfg(feature = "LuUserPlus2")]
            LuIcon::LuUserPlus2 => LU_USER_PLUS2,
            #[cfg(feature = "LuUserSquare")]
            LuIcon::LuUserSquare => LU_USER_SQUARE,
            #[cfg(feature = "LuUserSquare2")]
            LuIcon::LuUserSquare2 => LU_USER_SQUARE2,
            #[cfg(feature = "LuUserX")]
            LuIcon::LuUserX => LU_USER_X,
            #[cfg(feature = "LuUserX2")]
            LuIcon::LuUserX2 => LU_USER_X2,
            #[cfg(feature = "LuUsers")]
            LuIcon::LuUsers => LU_USERS,
            #[cfg(feature = "LuUsers2")]
            LuIcon::LuUsers2 => LU_USERS2,
            #[cfg(feature = "LuUtensils")]
            LuIcon::LuUtensils => LU_UTENSILS,
            #[cfg(feature = "LuUtensilsCrossed")]
            LuIcon::LuUtensilsCrossed => LU_UTENSILS_CROSSED,
            #[cfg(feature = "LuUtilityPole")]
            LuIcon::LuUtilityPole => LU_UTILITY_POLE,
            #[cfg(feature = "LuVariable")]
            LuIcon::LuVariable => LU_VARIABLE,
            #[cfg(feature = "LuVegan")]
            LuIcon::LuVegan => LU_VEGAN,
            #[cfg(feature = "LuVenetianMask")]
            LuIcon::LuVenetianMask => LU_VENETIAN_MASK,
            #[cfg(feature = "LuVibrate")]
            LuIcon::LuVibrate => LU_VIBRATE,
            #[cfg(feature = "LuVibrateOff")]
            LuIcon::LuVibrateOff => LU_VIBRATE_OFF,
            #[cfg(feature = "LuVideo")]
            LuIcon::LuVideo => LU_VIDEO,
            #[cfg(feature = "LuVideoOff")]
            LuIcon::LuVideoOff => LU_VIDEO_OFF,
            #[cfg(feature = "LuVideotape")]
            LuIcon::LuVideotape => LU_VIDEOTAPE,
            #[cfg(feature = "LuView")]
            LuIcon::LuView => LU_VIEW,
            #[cfg(feature = "LuVoicemail")]
            LuIcon::LuVoicemail => LU_VOICEMAIL,
            #[cfg(feature = "LuVolume")]
            LuIcon::LuVolume => LU_VOLUME,
            #[cfg(feature = "LuVolume1")]
            LuIcon::LuVolume1 => LU_VOLUME1,
            #[cfg(feature = "LuVolume2")]
            LuIcon::LuVolume2 => LU_VOLUME2,
            #[cfg(feature = "LuVolumeX")]
            LuIcon::LuVolumeX => LU_VOLUME_X,
            #[cfg(feature = "LuVote")]
            LuIcon::LuVote => LU_VOTE,
            #[cfg(feature = "LuWallet")]
            LuIcon::LuWallet => LU_WALLET,
            #[cfg(feature = "LuWallet2")]
            LuIcon::LuWallet2 => LU_WALLET2,
            #[cfg(feature = "LuWalletCards")]
            LuIcon::LuWalletCards => LU_WALLET_CARDS,
            #[cfg(feature = "LuWallpaper")]
            LuIcon::LuWallpaper => LU_WALLPAPER,
            #[cfg(feature = "LuWand")]
            LuIcon::LuWand => LU_WAND,
            #[cfg(feature = "LuWand2")]
            LuIcon::LuWand2 => LU_WAND2,
            #[cfg(feature = "LuWarehouse")]
            LuIcon::LuWarehouse => LU_WAREHOUSE,
            #[cfg(feature = "LuWatch")]
            LuIcon::LuWatch => LU_WATCH,
            #[cfg(feature = "LuWaves")]
            LuIcon::LuWaves => LU_WAVES,
            #[cfg(feature = "LuWebcam")]
            LuIcon::LuWebcam => LU_WEBCAM,
            #[cfg(feature = "LuWebhook")]
            LuIcon::LuWebhook => LU_WEBHOOK,
            #[cfg(feature = "LuWheat")]
            LuIcon::LuWheat => LU_WHEAT,
            #[cfg(feature = "LuWheatOff")]
            LuIcon::LuWheatOff => LU_WHEAT_OFF,
            #[cfg(feature = "LuWholeWord")]
            LuIcon::LuWholeWord => LU_WHOLE_WORD,
            #[cfg(feature = "LuWifi")]
            LuIcon::LuWifi => LU_WIFI,
            #[cfg(feature = "LuWifiOff")]
            LuIcon::LuWifiOff => LU_WIFI_OFF,
            #[cfg(feature = "LuWind")]
            LuIcon::LuWind => LU_WIND,
            #[cfg(feature = "LuWine")]
            LuIcon::LuWine => LU_WINE,
            #[cfg(feature = "LuWineOff")]
            LuIcon::LuWineOff => LU_WINE_OFF,
            #[cfg(feature = "LuWorkflow")]
            LuIcon::LuWorkflow => LU_WORKFLOW,
            #[cfg(feature = "LuWrapText")]
            LuIcon::LuWrapText => LU_WRAP_TEXT,
            #[cfg(feature = "LuWrench")]
            LuIcon::LuWrench => LU_WRENCH,
            #[cfg(feature = "LuX")]
            LuIcon::LuX => LU_X,
            #[cfg(feature = "LuXCircle")]
            LuIcon::LuXCircle => LU_X_CIRCLE,
            #[cfg(feature = "LuXOctagon")]
            LuIcon::LuXOctagon => LU_X_OCTAGON,
            #[cfg(feature = "LuXSquare")]
            LuIcon::LuXSquare => LU_X_SQUARE,
            #[cfg(feature = "LuYoutube")]
            LuIcon::LuYoutube => LU_YOUTUBE,
            #[cfg(feature = "LuZap")]
            LuIcon::LuZap => LU_ZAP,
            #[cfg(feature = "LuZapOff")]
            LuIcon::LuZapOff => LU_ZAP_OFF,
            #[cfg(feature = "LuZoomIn")]
            LuIcon::LuZoomIn => LU_ZOOM_IN,
            #[cfg(feature = "LuZoomOut")]
            LuIcon::LuZoomOut => LU_ZOOM_OUT,
        }
    }
}