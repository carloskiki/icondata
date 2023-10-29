//! This crate provides a collection of icons in the form of SVG data
//! and an enum to select them.
//!
//! ## Usage
//!
//! Every icon is shipped as its own feature; the enum variant and their corresponding feature name are
//! identical.
//!

/// Icons from [__Lucide__](https://github.com/lucide-icons/lucide)
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "strum", derive(strum::EnumIter, strum::EnumVariantNames))]
pub enum LuIcon {
    #[cfg(any(LuAccessibility, icondata_include_all))]
    LuAccessibility,
    #[cfg(any(LuActivity, icondata_include_all))]
    LuActivity,
    #[cfg(any(LuActivitySquare, icondata_include_all))]
    LuActivitySquare,
    #[cfg(any(LuAirVent, icondata_include_all))]
    LuAirVent,
    #[cfg(any(LuAirplay, icondata_include_all))]
    LuAirplay,
    #[cfg(any(LuAlarmCheck, icondata_include_all))]
    LuAlarmCheck,
    #[cfg(any(LuAlarmClock, icondata_include_all))]
    LuAlarmClock,
    #[cfg(any(LuAlarmClockOff, icondata_include_all))]
    LuAlarmClockOff,
    #[cfg(any(LuAlarmMinus, icondata_include_all))]
    LuAlarmMinus,
    #[cfg(any(LuAlarmPlus, icondata_include_all))]
    LuAlarmPlus,
    #[cfg(any(LuAlbum, icondata_include_all))]
    LuAlbum,
    #[cfg(any(LuAlertCircle, icondata_include_all))]
    LuAlertCircle,
    #[cfg(any(LuAlertOctagon, icondata_include_all))]
    LuAlertOctagon,
    #[cfg(any(LuAlertTriangle, icondata_include_all))]
    LuAlertTriangle,
    #[cfg(any(LuAlignCenter, icondata_include_all))]
    LuAlignCenter,
    #[cfg(any(LuAlignCenterHorizontal, icondata_include_all))]
    LuAlignCenterHorizontal,
    #[cfg(any(LuAlignCenterVertical, icondata_include_all))]
    LuAlignCenterVertical,
    #[cfg(any(LuAlignEndHorizontal, icondata_include_all))]
    LuAlignEndHorizontal,
    #[cfg(any(LuAlignEndVertical, icondata_include_all))]
    LuAlignEndVertical,
    #[cfg(any(LuAlignHorizontalDistributeCenter, icondata_include_all))]
    LuAlignHorizontalDistributeCenter,
    #[cfg(any(LuAlignHorizontalDistributeEnd, icondata_include_all))]
    LuAlignHorizontalDistributeEnd,
    #[cfg(any(LuAlignHorizontalDistributeStart, icondata_include_all))]
    LuAlignHorizontalDistributeStart,
    #[cfg(any(LuAlignHorizontalJustifyCenter, icondata_include_all))]
    LuAlignHorizontalJustifyCenter,
    #[cfg(any(LuAlignHorizontalJustifyEnd, icondata_include_all))]
    LuAlignHorizontalJustifyEnd,
    #[cfg(any(LuAlignHorizontalJustifyStart, icondata_include_all))]
    LuAlignHorizontalJustifyStart,
    #[cfg(any(LuAlignHorizontalSpaceAround, icondata_include_all))]
    LuAlignHorizontalSpaceAround,
    #[cfg(any(LuAlignHorizontalSpaceBetween, icondata_include_all))]
    LuAlignHorizontalSpaceBetween,
    #[cfg(any(LuAlignJustify, icondata_include_all))]
    LuAlignJustify,
    #[cfg(any(LuAlignLeft, icondata_include_all))]
    LuAlignLeft,
    #[cfg(any(LuAlignRight, icondata_include_all))]
    LuAlignRight,
    #[cfg(any(LuAlignStartHorizontal, icondata_include_all))]
    LuAlignStartHorizontal,
    #[cfg(any(LuAlignStartVertical, icondata_include_all))]
    LuAlignStartVertical,
    #[cfg(any(LuAlignVerticalDistributeCenter, icondata_include_all))]
    LuAlignVerticalDistributeCenter,
    #[cfg(any(LuAlignVerticalDistributeEnd, icondata_include_all))]
    LuAlignVerticalDistributeEnd,
    #[cfg(any(LuAlignVerticalDistributeStart, icondata_include_all))]
    LuAlignVerticalDistributeStart,
    #[cfg(any(LuAlignVerticalJustifyCenter, icondata_include_all))]
    LuAlignVerticalJustifyCenter,
    #[cfg(any(LuAlignVerticalJustifyEnd, icondata_include_all))]
    LuAlignVerticalJustifyEnd,
    #[cfg(any(LuAlignVerticalJustifyStart, icondata_include_all))]
    LuAlignVerticalJustifyStart,
    #[cfg(any(LuAlignVerticalSpaceAround, icondata_include_all))]
    LuAlignVerticalSpaceAround,
    #[cfg(any(LuAlignVerticalSpaceBetween, icondata_include_all))]
    LuAlignVerticalSpaceBetween,
    #[cfg(any(LuAmpersand, icondata_include_all))]
    LuAmpersand,
    #[cfg(any(LuAmpersands, icondata_include_all))]
    LuAmpersands,
    #[cfg(any(LuAnchor, icondata_include_all))]
    LuAnchor,
    #[cfg(any(LuAngry, icondata_include_all))]
    LuAngry,
    #[cfg(any(LuAnnoyed, icondata_include_all))]
    LuAnnoyed,
    #[cfg(any(LuAntenna, icondata_include_all))]
    LuAntenna,
    #[cfg(any(LuAperture, icondata_include_all))]
    LuAperture,
    #[cfg(any(LuAppWindow, icondata_include_all))]
    LuAppWindow,
    #[cfg(any(LuApple, icondata_include_all))]
    LuApple,
    #[cfg(any(LuArchive, icondata_include_all))]
    LuArchive,
    #[cfg(any(LuArchiveRestore, icondata_include_all))]
    LuArchiveRestore,
    #[cfg(any(LuAreaChart, icondata_include_all))]
    LuAreaChart,
    #[cfg(any(LuArmchair, icondata_include_all))]
    LuArmchair,
    #[cfg(any(LuArrowBigDown, icondata_include_all))]
    LuArrowBigDown,
    #[cfg(any(LuArrowBigDownDash, icondata_include_all))]
    LuArrowBigDownDash,
    #[cfg(any(LuArrowBigLeft, icondata_include_all))]
    LuArrowBigLeft,
    #[cfg(any(LuArrowBigLeftDash, icondata_include_all))]
    LuArrowBigLeftDash,
    #[cfg(any(LuArrowBigRight, icondata_include_all))]
    LuArrowBigRight,
    #[cfg(any(LuArrowBigRightDash, icondata_include_all))]
    LuArrowBigRightDash,
    #[cfg(any(LuArrowBigUp, icondata_include_all))]
    LuArrowBigUp,
    #[cfg(any(LuArrowBigUpDash, icondata_include_all))]
    LuArrowBigUpDash,
    #[cfg(any(LuArrowDown, icondata_include_all))]
    LuArrowDown,
    #[cfg(any(LuArrowDown01, icondata_include_all))]
    LuArrowDown01,
    #[cfg(any(LuArrowDown10, icondata_include_all))]
    LuArrowDown10,
    #[cfg(any(LuArrowDownAZ, icondata_include_all))]
    LuArrowDownAZ,
    #[cfg(any(LuArrowDownCircle, icondata_include_all))]
    LuArrowDownCircle,
    #[cfg(any(LuArrowDownFromLine, icondata_include_all))]
    LuArrowDownFromLine,
    #[cfg(any(LuArrowDownLeft, icondata_include_all))]
    LuArrowDownLeft,
    #[cfg(any(LuArrowDownLeftFromCircle, icondata_include_all))]
    LuArrowDownLeftFromCircle,
    #[cfg(any(LuArrowDownLeftSquare, icondata_include_all))]
    LuArrowDownLeftSquare,
    #[cfg(any(LuArrowDownNarrowWide, icondata_include_all))]
    LuArrowDownNarrowWide,
    #[cfg(any(LuArrowDownRight, icondata_include_all))]
    LuArrowDownRight,
    #[cfg(any(LuArrowDownRightFromCircle, icondata_include_all))]
    LuArrowDownRightFromCircle,
    #[cfg(any(LuArrowDownRightSquare, icondata_include_all))]
    LuArrowDownRightSquare,
    #[cfg(any(LuArrowDownSquare, icondata_include_all))]
    LuArrowDownSquare,
    #[cfg(any(LuArrowDownToDot, icondata_include_all))]
    LuArrowDownToDot,
    #[cfg(any(LuArrowDownToLine, icondata_include_all))]
    LuArrowDownToLine,
    #[cfg(any(LuArrowDownUp, icondata_include_all))]
    LuArrowDownUp,
    #[cfg(any(LuArrowDownWideNarrow, icondata_include_all))]
    LuArrowDownWideNarrow,
    #[cfg(any(LuArrowDownZA, icondata_include_all))]
    LuArrowDownZA,
    #[cfg(any(LuArrowLeft, icondata_include_all))]
    LuArrowLeft,
    #[cfg(any(LuArrowLeftCircle, icondata_include_all))]
    LuArrowLeftCircle,
    #[cfg(any(LuArrowLeftFromLine, icondata_include_all))]
    LuArrowLeftFromLine,
    #[cfg(any(LuArrowLeftRight, icondata_include_all))]
    LuArrowLeftRight,
    #[cfg(any(LuArrowLeftSquare, icondata_include_all))]
    LuArrowLeftSquare,
    #[cfg(any(LuArrowLeftToLine, icondata_include_all))]
    LuArrowLeftToLine,
    #[cfg(any(LuArrowRight, icondata_include_all))]
    LuArrowRight,
    #[cfg(any(LuArrowRightCircle, icondata_include_all))]
    LuArrowRightCircle,
    #[cfg(any(LuArrowRightFromLine, icondata_include_all))]
    LuArrowRightFromLine,
    #[cfg(any(LuArrowRightLeft, icondata_include_all))]
    LuArrowRightLeft,
    #[cfg(any(LuArrowRightSquare, icondata_include_all))]
    LuArrowRightSquare,
    #[cfg(any(LuArrowRightToLine, icondata_include_all))]
    LuArrowRightToLine,
    #[cfg(any(LuArrowUp, icondata_include_all))]
    LuArrowUp,
    #[cfg(any(LuArrowUp01, icondata_include_all))]
    LuArrowUp01,
    #[cfg(any(LuArrowUp10, icondata_include_all))]
    LuArrowUp10,
    #[cfg(any(LuArrowUpAZ, icondata_include_all))]
    LuArrowUpAZ,
    #[cfg(any(LuArrowUpCircle, icondata_include_all))]
    LuArrowUpCircle,
    #[cfg(any(LuArrowUpDown, icondata_include_all))]
    LuArrowUpDown,
    #[cfg(any(LuArrowUpFromDot, icondata_include_all))]
    LuArrowUpFromDot,
    #[cfg(any(LuArrowUpFromLine, icondata_include_all))]
    LuArrowUpFromLine,
    #[cfg(any(LuArrowUpLeft, icondata_include_all))]
    LuArrowUpLeft,
    #[cfg(any(LuArrowUpLeftFromCircle, icondata_include_all))]
    LuArrowUpLeftFromCircle,
    #[cfg(any(LuArrowUpLeftSquare, icondata_include_all))]
    LuArrowUpLeftSquare,
    #[cfg(any(LuArrowUpNarrowWide, icondata_include_all))]
    LuArrowUpNarrowWide,
    #[cfg(any(LuArrowUpRight, icondata_include_all))]
    LuArrowUpRight,
    #[cfg(any(LuArrowUpRightFromCircle, icondata_include_all))]
    LuArrowUpRightFromCircle,
    #[cfg(any(LuArrowUpRightSquare, icondata_include_all))]
    LuArrowUpRightSquare,
    #[cfg(any(LuArrowUpSquare, icondata_include_all))]
    LuArrowUpSquare,
    #[cfg(any(LuArrowUpToLine, icondata_include_all))]
    LuArrowUpToLine,
    #[cfg(any(LuArrowUpWideNarrow, icondata_include_all))]
    LuArrowUpWideNarrow,
    #[cfg(any(LuArrowUpZA, icondata_include_all))]
    LuArrowUpZA,
    #[cfg(any(LuArrowsUpFromLine, icondata_include_all))]
    LuArrowsUpFromLine,
    #[cfg(any(LuAsterisk, icondata_include_all))]
    LuAsterisk,
    #[cfg(any(LuAtSign, icondata_include_all))]
    LuAtSign,
    #[cfg(any(LuAtom, icondata_include_all))]
    LuAtom,
    #[cfg(any(LuAward, icondata_include_all))]
    LuAward,
    #[cfg(any(LuAxe, icondata_include_all))]
    LuAxe,
    #[cfg(any(LuAxis3d, icondata_include_all))]
    LuAxis3d,
    #[cfg(any(LuBaby, icondata_include_all))]
    LuBaby,
    #[cfg(any(LuBackpack, icondata_include_all))]
    LuBackpack,
    #[cfg(any(LuBadge, icondata_include_all))]
    LuBadge,
    #[cfg(any(LuBadgeAlert, icondata_include_all))]
    LuBadgeAlert,
    #[cfg(any(LuBadgeCheck, icondata_include_all))]
    LuBadgeCheck,
    #[cfg(any(LuBadgeDollarSign, icondata_include_all))]
    LuBadgeDollarSign,
    #[cfg(any(LuBadgeHelp, icondata_include_all))]
    LuBadgeHelp,
    #[cfg(any(LuBadgeInfo, icondata_include_all))]
    LuBadgeInfo,
    #[cfg(any(LuBadgeMinus, icondata_include_all))]
    LuBadgeMinus,
    #[cfg(any(LuBadgePercent, icondata_include_all))]
    LuBadgePercent,
    #[cfg(any(LuBadgePlus, icondata_include_all))]
    LuBadgePlus,
    #[cfg(any(LuBadgeX, icondata_include_all))]
    LuBadgeX,
    #[cfg(any(LuBaggageClaim, icondata_include_all))]
    LuBaggageClaim,
    #[cfg(any(LuBan, icondata_include_all))]
    LuBan,
    #[cfg(any(LuBanana, icondata_include_all))]
    LuBanana,
    #[cfg(any(LuBanknote, icondata_include_all))]
    LuBanknote,
    #[cfg(any(LuBarChart, icondata_include_all))]
    LuBarChart,
    #[cfg(any(LuBarChart2, icondata_include_all))]
    LuBarChart2,
    #[cfg(any(LuBarChart3, icondata_include_all))]
    LuBarChart3,
    #[cfg(any(LuBarChart4, icondata_include_all))]
    LuBarChart4,
    #[cfg(any(LuBarChartBig, icondata_include_all))]
    LuBarChartBig,
    #[cfg(any(LuBarChartHorizontal, icondata_include_all))]
    LuBarChartHorizontal,
    #[cfg(any(LuBarChartHorizontalBig, icondata_include_all))]
    LuBarChartHorizontalBig,
    #[cfg(any(LuBaseline, icondata_include_all))]
    LuBaseline,
    #[cfg(any(LuBath, icondata_include_all))]
    LuBath,
    #[cfg(any(LuBattery, icondata_include_all))]
    LuBattery,
    #[cfg(any(LuBatteryCharging, icondata_include_all))]
    LuBatteryCharging,
    #[cfg(any(LuBatteryFull, icondata_include_all))]
    LuBatteryFull,
    #[cfg(any(LuBatteryLow, icondata_include_all))]
    LuBatteryLow,
    #[cfg(any(LuBatteryMedium, icondata_include_all))]
    LuBatteryMedium,
    #[cfg(any(LuBatteryWarning, icondata_include_all))]
    LuBatteryWarning,
    #[cfg(any(LuBeaker, icondata_include_all))]
    LuBeaker,
    #[cfg(any(LuBean, icondata_include_all))]
    LuBean,
    #[cfg(any(LuBeanOff, icondata_include_all))]
    LuBeanOff,
    #[cfg(any(LuBed, icondata_include_all))]
    LuBed,
    #[cfg(any(LuBedDouble, icondata_include_all))]
    LuBedDouble,
    #[cfg(any(LuBedSingle, icondata_include_all))]
    LuBedSingle,
    #[cfg(any(LuBeef, icondata_include_all))]
    LuBeef,
    #[cfg(any(LuBeer, icondata_include_all))]
    LuBeer,
    #[cfg(any(LuBell, icondata_include_all))]
    LuBell,
    #[cfg(any(LuBellDot, icondata_include_all))]
    LuBellDot,
    #[cfg(any(LuBellMinus, icondata_include_all))]
    LuBellMinus,
    #[cfg(any(LuBellOff, icondata_include_all))]
    LuBellOff,
    #[cfg(any(LuBellPlus, icondata_include_all))]
    LuBellPlus,
    #[cfg(any(LuBellRing, icondata_include_all))]
    LuBellRing,
    #[cfg(any(LuBike, icondata_include_all))]
    LuBike,
    #[cfg(any(LuBinary, icondata_include_all))]
    LuBinary,
    #[cfg(any(LuBiohazard, icondata_include_all))]
    LuBiohazard,
    #[cfg(any(LuBird, icondata_include_all))]
    LuBird,
    #[cfg(any(LuBitcoin, icondata_include_all))]
    LuBitcoin,
    #[cfg(any(LuBlinds, icondata_include_all))]
    LuBlinds,
    #[cfg(any(LuBlocks, icondata_include_all))]
    LuBlocks,
    #[cfg(any(LuBluetooth, icondata_include_all))]
    LuBluetooth,
    #[cfg(any(LuBluetoothConnected, icondata_include_all))]
    LuBluetoothConnected,
    #[cfg(any(LuBluetoothOff, icondata_include_all))]
    LuBluetoothOff,
    #[cfg(any(LuBluetoothSearching, icondata_include_all))]
    LuBluetoothSearching,
    #[cfg(any(LuBold, icondata_include_all))]
    LuBold,
    #[cfg(any(LuBomb, icondata_include_all))]
    LuBomb,
    #[cfg(any(LuBone, icondata_include_all))]
    LuBone,
    #[cfg(any(LuBook, icondata_include_all))]
    LuBook,
    #[cfg(any(LuBookCopy, icondata_include_all))]
    LuBookCopy,
    #[cfg(any(LuBookDown, icondata_include_all))]
    LuBookDown,
    #[cfg(any(LuBookKey, icondata_include_all))]
    LuBookKey,
    #[cfg(any(LuBookLock, icondata_include_all))]
    LuBookLock,
    #[cfg(any(LuBookMarked, icondata_include_all))]
    LuBookMarked,
    #[cfg(any(LuBookMinus, icondata_include_all))]
    LuBookMinus,
    #[cfg(any(LuBookOpen, icondata_include_all))]
    LuBookOpen,
    #[cfg(any(LuBookOpenCheck, icondata_include_all))]
    LuBookOpenCheck,
    #[cfg(any(LuBookPlus, icondata_include_all))]
    LuBookPlus,
    #[cfg(any(LuBookTemplate, icondata_include_all))]
    LuBookTemplate,
    #[cfg(any(LuBookUp, icondata_include_all))]
    LuBookUp,
    #[cfg(any(LuBookUp2, icondata_include_all))]
    LuBookUp2,
    #[cfg(any(LuBookX, icondata_include_all))]
    LuBookX,
    #[cfg(any(LuBookmark, icondata_include_all))]
    LuBookmark,
    #[cfg(any(LuBookmarkMinus, icondata_include_all))]
    LuBookmarkMinus,
    #[cfg(any(LuBookmarkPlus, icondata_include_all))]
    LuBookmarkPlus,
    #[cfg(any(LuBoomBox, icondata_include_all))]
    LuBoomBox,
    #[cfg(any(LuBot, icondata_include_all))]
    LuBot,
    #[cfg(any(LuBox, icondata_include_all))]
    LuBox,
    #[cfg(any(LuBoxSelect, icondata_include_all))]
    LuBoxSelect,
    #[cfg(any(LuBoxes, icondata_include_all))]
    LuBoxes,
    #[cfg(any(LuBraces, icondata_include_all))]
    LuBraces,
    #[cfg(any(LuBrackets, icondata_include_all))]
    LuBrackets,
    #[cfg(any(LuBrain, icondata_include_all))]
    LuBrain,
    #[cfg(any(LuBrainCircuit, icondata_include_all))]
    LuBrainCircuit,
    #[cfg(any(LuBrainCog, icondata_include_all))]
    LuBrainCog,
    #[cfg(any(LuBriefcase, icondata_include_all))]
    LuBriefcase,
    #[cfg(any(LuBringToFront, icondata_include_all))]
    LuBringToFront,
    #[cfg(any(LuBrush, icondata_include_all))]
    LuBrush,
    #[cfg(any(LuBug, icondata_include_all))]
    LuBug,
    #[cfg(any(LuBuilding, icondata_include_all))]
    LuBuilding,
    #[cfg(any(LuBuilding2, icondata_include_all))]
    LuBuilding2,
    #[cfg(any(LuBus, icondata_include_all))]
    LuBus,
    #[cfg(any(LuBusFront, icondata_include_all))]
    LuBusFront,
    #[cfg(any(LuCable, icondata_include_all))]
    LuCable,
    #[cfg(any(LuCableCar, icondata_include_all))]
    LuCableCar,
    #[cfg(any(LuCake, icondata_include_all))]
    LuCake,
    #[cfg(any(LuCakeSlice, icondata_include_all))]
    LuCakeSlice,
    #[cfg(any(LuCalculator, icondata_include_all))]
    LuCalculator,
    #[cfg(any(LuCalendar, icondata_include_all))]
    LuCalendar,
    #[cfg(any(LuCalendarCheck, icondata_include_all))]
    LuCalendarCheck,
    #[cfg(any(LuCalendarCheck2, icondata_include_all))]
    LuCalendarCheck2,
    #[cfg(any(LuCalendarClock, icondata_include_all))]
    LuCalendarClock,
    #[cfg(any(LuCalendarDays, icondata_include_all))]
    LuCalendarDays,
    #[cfg(any(LuCalendarHeart, icondata_include_all))]
    LuCalendarHeart,
    #[cfg(any(LuCalendarMinus, icondata_include_all))]
    LuCalendarMinus,
    #[cfg(any(LuCalendarOff, icondata_include_all))]
    LuCalendarOff,
    #[cfg(any(LuCalendarPlus, icondata_include_all))]
    LuCalendarPlus,
    #[cfg(any(LuCalendarRange, icondata_include_all))]
    LuCalendarRange,
    #[cfg(any(LuCalendarSearch, icondata_include_all))]
    LuCalendarSearch,
    #[cfg(any(LuCalendarX, icondata_include_all))]
    LuCalendarX,
    #[cfg(any(LuCalendarX2, icondata_include_all))]
    LuCalendarX2,
    #[cfg(any(LuCamera, icondata_include_all))]
    LuCamera,
    #[cfg(any(LuCameraOff, icondata_include_all))]
    LuCameraOff,
    #[cfg(any(LuCandlestickChart, icondata_include_all))]
    LuCandlestickChart,
    #[cfg(any(LuCandy, icondata_include_all))]
    LuCandy,
    #[cfg(any(LuCandyCane, icondata_include_all))]
    LuCandyCane,
    #[cfg(any(LuCandyOff, icondata_include_all))]
    LuCandyOff,
    #[cfg(any(LuCar, icondata_include_all))]
    LuCar,
    #[cfg(any(LuCarFront, icondata_include_all))]
    LuCarFront,
    #[cfg(any(LuCarTaxiFront, icondata_include_all))]
    LuCarTaxiFront,
    #[cfg(any(LuCarrot, icondata_include_all))]
    LuCarrot,
    #[cfg(any(LuCaseLower, icondata_include_all))]
    LuCaseLower,
    #[cfg(any(LuCaseSensitive, icondata_include_all))]
    LuCaseSensitive,
    #[cfg(any(LuCaseUpper, icondata_include_all))]
    LuCaseUpper,
    #[cfg(any(LuCassetteTape, icondata_include_all))]
    LuCassetteTape,
    #[cfg(any(LuCast, icondata_include_all))]
    LuCast,
    #[cfg(any(LuCastle, icondata_include_all))]
    LuCastle,
    #[cfg(any(LuCat, icondata_include_all))]
    LuCat,
    #[cfg(any(LuCheck, icondata_include_all))]
    LuCheck,
    #[cfg(any(LuCheckCheck, icondata_include_all))]
    LuCheckCheck,
    #[cfg(any(LuCheckCircle, icondata_include_all))]
    LuCheckCircle,
    #[cfg(any(LuCheckCircle2, icondata_include_all))]
    LuCheckCircle2,
    #[cfg(any(LuCheckSquare, icondata_include_all))]
    LuCheckSquare,
    #[cfg(any(LuChefHat, icondata_include_all))]
    LuChefHat,
    #[cfg(any(LuCherry, icondata_include_all))]
    LuCherry,
    #[cfg(any(LuChevronDown, icondata_include_all))]
    LuChevronDown,
    #[cfg(any(LuChevronDownCircle, icondata_include_all))]
    LuChevronDownCircle,
    #[cfg(any(LuChevronDownSquare, icondata_include_all))]
    LuChevronDownSquare,
    #[cfg(any(LuChevronFirst, icondata_include_all))]
    LuChevronFirst,
    #[cfg(any(LuChevronLast, icondata_include_all))]
    LuChevronLast,
    #[cfg(any(LuChevronLeft, icondata_include_all))]
    LuChevronLeft,
    #[cfg(any(LuChevronLeftCircle, icondata_include_all))]
    LuChevronLeftCircle,
    #[cfg(any(LuChevronLeftSquare, icondata_include_all))]
    LuChevronLeftSquare,
    #[cfg(any(LuChevronRight, icondata_include_all))]
    LuChevronRight,
    #[cfg(any(LuChevronRightCircle, icondata_include_all))]
    LuChevronRightCircle,
    #[cfg(any(LuChevronRightSquare, icondata_include_all))]
    LuChevronRightSquare,
    #[cfg(any(LuChevronUp, icondata_include_all))]
    LuChevronUp,
    #[cfg(any(LuChevronUpCircle, icondata_include_all))]
    LuChevronUpCircle,
    #[cfg(any(LuChevronUpSquare, icondata_include_all))]
    LuChevronUpSquare,
    #[cfg(any(LuChevronsDown, icondata_include_all))]
    LuChevronsDown,
    #[cfg(any(LuChevronsDownUp, icondata_include_all))]
    LuChevronsDownUp,
    #[cfg(any(LuChevronsLeft, icondata_include_all))]
    LuChevronsLeft,
    #[cfg(any(LuChevronsLeftRight, icondata_include_all))]
    LuChevronsLeftRight,
    #[cfg(any(LuChevronsRight, icondata_include_all))]
    LuChevronsRight,
    #[cfg(any(LuChevronsRightLeft, icondata_include_all))]
    LuChevronsRightLeft,
    #[cfg(any(LuChevronsUp, icondata_include_all))]
    LuChevronsUp,
    #[cfg(any(LuChevronsUpDown, icondata_include_all))]
    LuChevronsUpDown,
    #[cfg(any(LuChrome, icondata_include_all))]
    LuChrome,
    #[cfg(any(LuChurch, icondata_include_all))]
    LuChurch,
    #[cfg(any(LuCigarette, icondata_include_all))]
    LuCigarette,
    #[cfg(any(LuCigaretteOff, icondata_include_all))]
    LuCigaretteOff,
    #[cfg(any(LuCircle, icondata_include_all))]
    LuCircle,
    #[cfg(any(LuCircleDashed, icondata_include_all))]
    LuCircleDashed,
    #[cfg(any(LuCircleDollarSign, icondata_include_all))]
    LuCircleDollarSign,
    #[cfg(any(LuCircleDot, icondata_include_all))]
    LuCircleDot,
    #[cfg(any(LuCircleDotDashed, icondata_include_all))]
    LuCircleDotDashed,
    #[cfg(any(LuCircleEllipsis, icondata_include_all))]
    LuCircleEllipsis,
    #[cfg(any(LuCircleEqual, icondata_include_all))]
    LuCircleEqual,
    #[cfg(any(LuCircleOff, icondata_include_all))]
    LuCircleOff,
    #[cfg(any(LuCircleSlash, icondata_include_all))]
    LuCircleSlash,
    #[cfg(any(LuCircleSlash2, icondata_include_all))]
    LuCircleSlash2,
    #[cfg(any(LuCircuitBoard, icondata_include_all))]
    LuCircuitBoard,
    #[cfg(any(LuCitrus, icondata_include_all))]
    LuCitrus,
    #[cfg(any(LuClapperboard, icondata_include_all))]
    LuClapperboard,
    #[cfg(any(LuClipboard, icondata_include_all))]
    LuClipboard,
    #[cfg(any(LuClipboardCheck, icondata_include_all))]
    LuClipboardCheck,
    #[cfg(any(LuClipboardCopy, icondata_include_all))]
    LuClipboardCopy,
    #[cfg(any(LuClipboardEdit, icondata_include_all))]
    LuClipboardEdit,
    #[cfg(any(LuClipboardList, icondata_include_all))]
    LuClipboardList,
    #[cfg(any(LuClipboardPaste, icondata_include_all))]
    LuClipboardPaste,
    #[cfg(any(LuClipboardSignature, icondata_include_all))]
    LuClipboardSignature,
    #[cfg(any(LuClipboardType, icondata_include_all))]
    LuClipboardType,
    #[cfg(any(LuClipboardX, icondata_include_all))]
    LuClipboardX,
    #[cfg(any(LuClock, icondata_include_all))]
    LuClock,
    #[cfg(any(LuClock1, icondata_include_all))]
    LuClock1,
    #[cfg(any(LuClock10, icondata_include_all))]
    LuClock10,
    #[cfg(any(LuClock11, icondata_include_all))]
    LuClock11,
    #[cfg(any(LuClock12, icondata_include_all))]
    LuClock12,
    #[cfg(any(LuClock2, icondata_include_all))]
    LuClock2,
    #[cfg(any(LuClock3, icondata_include_all))]
    LuClock3,
    #[cfg(any(LuClock4, icondata_include_all))]
    LuClock4,
    #[cfg(any(LuClock5, icondata_include_all))]
    LuClock5,
    #[cfg(any(LuClock6, icondata_include_all))]
    LuClock6,
    #[cfg(any(LuClock7, icondata_include_all))]
    LuClock7,
    #[cfg(any(LuClock8, icondata_include_all))]
    LuClock8,
    #[cfg(any(LuClock9, icondata_include_all))]
    LuClock9,
    #[cfg(any(LuCloud, icondata_include_all))]
    LuCloud,
    #[cfg(any(LuCloudCog, icondata_include_all))]
    LuCloudCog,
    #[cfg(any(LuCloudDrizzle, icondata_include_all))]
    LuCloudDrizzle,
    #[cfg(any(LuCloudFog, icondata_include_all))]
    LuCloudFog,
    #[cfg(any(LuCloudHail, icondata_include_all))]
    LuCloudHail,
    #[cfg(any(LuCloudLightning, icondata_include_all))]
    LuCloudLightning,
    #[cfg(any(LuCloudMoon, icondata_include_all))]
    LuCloudMoon,
    #[cfg(any(LuCloudMoonRain, icondata_include_all))]
    LuCloudMoonRain,
    #[cfg(any(LuCloudOff, icondata_include_all))]
    LuCloudOff,
    #[cfg(any(LuCloudRain, icondata_include_all))]
    LuCloudRain,
    #[cfg(any(LuCloudRainWind, icondata_include_all))]
    LuCloudRainWind,
    #[cfg(any(LuCloudSnow, icondata_include_all))]
    LuCloudSnow,
    #[cfg(any(LuCloudSun, icondata_include_all))]
    LuCloudSun,
    #[cfg(any(LuCloudSunRain, icondata_include_all))]
    LuCloudSunRain,
    #[cfg(any(LuCloudy, icondata_include_all))]
    LuCloudy,
    #[cfg(any(LuClover, icondata_include_all))]
    LuClover,
    #[cfg(any(LuClub, icondata_include_all))]
    LuClub,
    #[cfg(any(LuCode, icondata_include_all))]
    LuCode,
    #[cfg(any(LuCode2, icondata_include_all))]
    LuCode2,
    #[cfg(any(LuCodepen, icondata_include_all))]
    LuCodepen,
    #[cfg(any(LuCodesandbox, icondata_include_all))]
    LuCodesandbox,
    #[cfg(any(LuCoffee, icondata_include_all))]
    LuCoffee,
    #[cfg(any(LuCog, icondata_include_all))]
    LuCog,
    #[cfg(any(LuCoins, icondata_include_all))]
    LuCoins,
    #[cfg(any(LuColumns, icondata_include_all))]
    LuColumns,
    #[cfg(any(LuCombine, icondata_include_all))]
    LuCombine,
    #[cfg(any(LuCommand, icondata_include_all))]
    LuCommand,
    #[cfg(any(LuCompass, icondata_include_all))]
    LuCompass,
    #[cfg(any(LuComponent, icondata_include_all))]
    LuComponent,
    #[cfg(any(LuComputer, icondata_include_all))]
    LuComputer,
    #[cfg(any(LuConciergeBell, icondata_include_all))]
    LuConciergeBell,
    #[cfg(any(LuConstruction, icondata_include_all))]
    LuConstruction,
    #[cfg(any(LuContact, icondata_include_all))]
    LuContact,
    #[cfg(any(LuContact2, icondata_include_all))]
    LuContact2,
    #[cfg(any(LuContainer, icondata_include_all))]
    LuContainer,
    #[cfg(any(LuContrast, icondata_include_all))]
    LuContrast,
    #[cfg(any(LuCookie, icondata_include_all))]
    LuCookie,
    #[cfg(any(LuCopy, icondata_include_all))]
    LuCopy,
    #[cfg(any(LuCopyCheck, icondata_include_all))]
    LuCopyCheck,
    #[cfg(any(LuCopyMinus, icondata_include_all))]
    LuCopyMinus,
    #[cfg(any(LuCopyPlus, icondata_include_all))]
    LuCopyPlus,
    #[cfg(any(LuCopySlash, icondata_include_all))]
    LuCopySlash,
    #[cfg(any(LuCopyX, icondata_include_all))]
    LuCopyX,
    #[cfg(any(LuCopyleft, icondata_include_all))]
    LuCopyleft,
    #[cfg(any(LuCopyright, icondata_include_all))]
    LuCopyright,
    #[cfg(any(LuCornerDownLeft, icondata_include_all))]
    LuCornerDownLeft,
    #[cfg(any(LuCornerDownRight, icondata_include_all))]
    LuCornerDownRight,
    #[cfg(any(LuCornerLeftDown, icondata_include_all))]
    LuCornerLeftDown,
    #[cfg(any(LuCornerLeftUp, icondata_include_all))]
    LuCornerLeftUp,
    #[cfg(any(LuCornerRightDown, icondata_include_all))]
    LuCornerRightDown,
    #[cfg(any(LuCornerRightUp, icondata_include_all))]
    LuCornerRightUp,
    #[cfg(any(LuCornerUpLeft, icondata_include_all))]
    LuCornerUpLeft,
    #[cfg(any(LuCornerUpRight, icondata_include_all))]
    LuCornerUpRight,
    #[cfg(any(LuCpu, icondata_include_all))]
    LuCpu,
    #[cfg(any(LuCreativeCommons, icondata_include_all))]
    LuCreativeCommons,
    #[cfg(any(LuCreditCard, icondata_include_all))]
    LuCreditCard,
    #[cfg(any(LuCroissant, icondata_include_all))]
    LuCroissant,
    #[cfg(any(LuCrop, icondata_include_all))]
    LuCrop,
    #[cfg(any(LuCross, icondata_include_all))]
    LuCross,
    #[cfg(any(LuCrosshair, icondata_include_all))]
    LuCrosshair,
    #[cfg(any(LuCrown, icondata_include_all))]
    LuCrown,
    #[cfg(any(LuCupSoda, icondata_include_all))]
    LuCupSoda,
    #[cfg(any(LuCurrency, icondata_include_all))]
    LuCurrency,
    #[cfg(any(LuDatabase, icondata_include_all))]
    LuDatabase,
    #[cfg(any(LuDatabaseBackup, icondata_include_all))]
    LuDatabaseBackup,
    #[cfg(any(LuDelete, icondata_include_all))]
    LuDelete,
    #[cfg(any(LuDessert, icondata_include_all))]
    LuDessert,
    #[cfg(any(LuDiamond, icondata_include_all))]
    LuDiamond,
    #[cfg(any(LuDice1, icondata_include_all))]
    LuDice1,
    #[cfg(any(LuDice2, icondata_include_all))]
    LuDice2,
    #[cfg(any(LuDice3, icondata_include_all))]
    LuDice3,
    #[cfg(any(LuDice4, icondata_include_all))]
    LuDice4,
    #[cfg(any(LuDice5, icondata_include_all))]
    LuDice5,
    #[cfg(any(LuDice6, icondata_include_all))]
    LuDice6,
    #[cfg(any(LuDices, icondata_include_all))]
    LuDices,
    #[cfg(any(LuDiff, icondata_include_all))]
    LuDiff,
    #[cfg(any(LuDisc, icondata_include_all))]
    LuDisc,
    #[cfg(any(LuDisc2, icondata_include_all))]
    LuDisc2,
    #[cfg(any(LuDisc3, icondata_include_all))]
    LuDisc3,
    #[cfg(any(LuDivide, icondata_include_all))]
    LuDivide,
    #[cfg(any(LuDivideCircle, icondata_include_all))]
    LuDivideCircle,
    #[cfg(any(LuDivideSquare, icondata_include_all))]
    LuDivideSquare,
    #[cfg(any(LuDna, icondata_include_all))]
    LuDna,
    #[cfg(any(LuDnaOff, icondata_include_all))]
    LuDnaOff,
    #[cfg(any(LuDog, icondata_include_all))]
    LuDog,
    #[cfg(any(LuDollarSign, icondata_include_all))]
    LuDollarSign,
    #[cfg(any(LuDonut, icondata_include_all))]
    LuDonut,
    #[cfg(any(LuDoorClosed, icondata_include_all))]
    LuDoorClosed,
    #[cfg(any(LuDoorOpen, icondata_include_all))]
    LuDoorOpen,
    #[cfg(any(LuDot, icondata_include_all))]
    LuDot,
    #[cfg(any(LuDownload, icondata_include_all))]
    LuDownload,
    #[cfg(any(LuDownloadCloud, icondata_include_all))]
    LuDownloadCloud,
    #[cfg(any(LuDribbble, icondata_include_all))]
    LuDribbble,
    #[cfg(any(LuDroplet, icondata_include_all))]
    LuDroplet,
    #[cfg(any(LuDroplets, icondata_include_all))]
    LuDroplets,
    #[cfg(any(LuDrumstick, icondata_include_all))]
    LuDrumstick,
    #[cfg(any(LuDumbbell, icondata_include_all))]
    LuDumbbell,
    #[cfg(any(LuEar, icondata_include_all))]
    LuEar,
    #[cfg(any(LuEarOff, icondata_include_all))]
    LuEarOff,
    #[cfg(any(LuEgg, icondata_include_all))]
    LuEgg,
    #[cfg(any(LuEggFried, icondata_include_all))]
    LuEggFried,
    #[cfg(any(LuEggOff, icondata_include_all))]
    LuEggOff,
    #[cfg(any(LuEqual, icondata_include_all))]
    LuEqual,
    #[cfg(any(LuEqualNot, icondata_include_all))]
    LuEqualNot,
    #[cfg(any(LuEraser, icondata_include_all))]
    LuEraser,
    #[cfg(any(LuEuro, icondata_include_all))]
    LuEuro,
    #[cfg(any(LuExpand, icondata_include_all))]
    LuExpand,
    #[cfg(any(LuExternalLink, icondata_include_all))]
    LuExternalLink,
    #[cfg(any(LuEye, icondata_include_all))]
    LuEye,
    #[cfg(any(LuEyeOff, icondata_include_all))]
    LuEyeOff,
    #[cfg(any(LuFacebook, icondata_include_all))]
    LuFacebook,
    #[cfg(any(LuFactory, icondata_include_all))]
    LuFactory,
    #[cfg(any(LuFan, icondata_include_all))]
    LuFan,
    #[cfg(any(LuFastForward, icondata_include_all))]
    LuFastForward,
    #[cfg(any(LuFeather, icondata_include_all))]
    LuFeather,
    #[cfg(any(LuFerrisWheel, icondata_include_all))]
    LuFerrisWheel,
    #[cfg(any(LuFigma, icondata_include_all))]
    LuFigma,
    #[cfg(any(LuFile, icondata_include_all))]
    LuFile,
    #[cfg(any(LuFileArchive, icondata_include_all))]
    LuFileArchive,
    #[cfg(any(LuFileAudio, icondata_include_all))]
    LuFileAudio,
    #[cfg(any(LuFileAudio2, icondata_include_all))]
    LuFileAudio2,
    #[cfg(any(LuFileAxis3d, icondata_include_all))]
    LuFileAxis3d,
    #[cfg(any(LuFileBadge, icondata_include_all))]
    LuFileBadge,
    #[cfg(any(LuFileBadge2, icondata_include_all))]
    LuFileBadge2,
    #[cfg(any(LuFileBarChart, icondata_include_all))]
    LuFileBarChart,
    #[cfg(any(LuFileBarChart2, icondata_include_all))]
    LuFileBarChart2,
    #[cfg(any(LuFileBox, icondata_include_all))]
    LuFileBox,
    #[cfg(any(LuFileCheck, icondata_include_all))]
    LuFileCheck,
    #[cfg(any(LuFileCheck2, icondata_include_all))]
    LuFileCheck2,
    #[cfg(any(LuFileClock, icondata_include_all))]
    LuFileClock,
    #[cfg(any(LuFileCode, icondata_include_all))]
    LuFileCode,
    #[cfg(any(LuFileCode2, icondata_include_all))]
    LuFileCode2,
    #[cfg(any(LuFileCog, icondata_include_all))]
    LuFileCog,
    #[cfg(any(LuFileCog2, icondata_include_all))]
    LuFileCog2,
    #[cfg(any(LuFileDiff, icondata_include_all))]
    LuFileDiff,
    #[cfg(any(LuFileDigit, icondata_include_all))]
    LuFileDigit,
    #[cfg(any(LuFileDown, icondata_include_all))]
    LuFileDown,
    #[cfg(any(LuFileEdit, icondata_include_all))]
    LuFileEdit,
    #[cfg(any(LuFileHeart, icondata_include_all))]
    LuFileHeart,
    #[cfg(any(LuFileImage, icondata_include_all))]
    LuFileImage,
    #[cfg(any(LuFileInput, icondata_include_all))]
    LuFileInput,
    #[cfg(any(LuFileJson, icondata_include_all))]
    LuFileJson,
    #[cfg(any(LuFileJson2, icondata_include_all))]
    LuFileJson2,
    #[cfg(any(LuFileKey, icondata_include_all))]
    LuFileKey,
    #[cfg(any(LuFileKey2, icondata_include_all))]
    LuFileKey2,
    #[cfg(any(LuFileLineChart, icondata_include_all))]
    LuFileLineChart,
    #[cfg(any(LuFileLock, icondata_include_all))]
    LuFileLock,
    #[cfg(any(LuFileLock2, icondata_include_all))]
    LuFileLock2,
    #[cfg(any(LuFileMinus, icondata_include_all))]
    LuFileMinus,
    #[cfg(any(LuFileMinus2, icondata_include_all))]
    LuFileMinus2,
    #[cfg(any(LuFileOutput, icondata_include_all))]
    LuFileOutput,
    #[cfg(any(LuFilePieChart, icondata_include_all))]
    LuFilePieChart,
    #[cfg(any(LuFilePlus, icondata_include_all))]
    LuFilePlus,
    #[cfg(any(LuFilePlus2, icondata_include_all))]
    LuFilePlus2,
    #[cfg(any(LuFileQuestion, icondata_include_all))]
    LuFileQuestion,
    #[cfg(any(LuFileScan, icondata_include_all))]
    LuFileScan,
    #[cfg(any(LuFileSearch, icondata_include_all))]
    LuFileSearch,
    #[cfg(any(LuFileSearch2, icondata_include_all))]
    LuFileSearch2,
    #[cfg(any(LuFileSignature, icondata_include_all))]
    LuFileSignature,
    #[cfg(any(LuFileSpreadsheet, icondata_include_all))]
    LuFileSpreadsheet,
    #[cfg(any(LuFileStack, icondata_include_all))]
    LuFileStack,
    #[cfg(any(LuFileSymlink, icondata_include_all))]
    LuFileSymlink,
    #[cfg(any(LuFileTerminal, icondata_include_all))]
    LuFileTerminal,
    #[cfg(any(LuFileText, icondata_include_all))]
    LuFileText,
    #[cfg(any(LuFileType, icondata_include_all))]
    LuFileType,
    #[cfg(any(LuFileType2, icondata_include_all))]
    LuFileType2,
    #[cfg(any(LuFileUp, icondata_include_all))]
    LuFileUp,
    #[cfg(any(LuFileVideo, icondata_include_all))]
    LuFileVideo,
    #[cfg(any(LuFileVideo2, icondata_include_all))]
    LuFileVideo2,
    #[cfg(any(LuFileVolume, icondata_include_all))]
    LuFileVolume,
    #[cfg(any(LuFileVolume2, icondata_include_all))]
    LuFileVolume2,
    #[cfg(any(LuFileWarning, icondata_include_all))]
    LuFileWarning,
    #[cfg(any(LuFileX, icondata_include_all))]
    LuFileX,
    #[cfg(any(LuFileX2, icondata_include_all))]
    LuFileX2,
    #[cfg(any(LuFiles, icondata_include_all))]
    LuFiles,
    #[cfg(any(LuFilm, icondata_include_all))]
    LuFilm,
    #[cfg(any(LuFilter, icondata_include_all))]
    LuFilter,
    #[cfg(any(LuFilterX, icondata_include_all))]
    LuFilterX,
    #[cfg(any(LuFingerprint, icondata_include_all))]
    LuFingerprint,
    #[cfg(any(LuFish, icondata_include_all))]
    LuFish,
    #[cfg(any(LuFishOff, icondata_include_all))]
    LuFishOff,
    #[cfg(any(LuFishSymbol, icondata_include_all))]
    LuFishSymbol,
    #[cfg(any(LuFlag, icondata_include_all))]
    LuFlag,
    #[cfg(any(LuFlagOff, icondata_include_all))]
    LuFlagOff,
    #[cfg(any(LuFlagTriangleLeft, icondata_include_all))]
    LuFlagTriangleLeft,
    #[cfg(any(LuFlagTriangleRight, icondata_include_all))]
    LuFlagTriangleRight,
    #[cfg(any(LuFlame, icondata_include_all))]
    LuFlame,
    #[cfg(any(LuFlashlight, icondata_include_all))]
    LuFlashlight,
    #[cfg(any(LuFlashlightOff, icondata_include_all))]
    LuFlashlightOff,
    #[cfg(any(LuFlaskConical, icondata_include_all))]
    LuFlaskConical,
    #[cfg(any(LuFlaskConicalOff, icondata_include_all))]
    LuFlaskConicalOff,
    #[cfg(any(LuFlaskRound, icondata_include_all))]
    LuFlaskRound,
    #[cfg(any(LuFlipHorizontal, icondata_include_all))]
    LuFlipHorizontal,
    #[cfg(any(LuFlipHorizontal2, icondata_include_all))]
    LuFlipHorizontal2,
    #[cfg(any(LuFlipVertical, icondata_include_all))]
    LuFlipVertical,
    #[cfg(any(LuFlipVertical2, icondata_include_all))]
    LuFlipVertical2,
    #[cfg(any(LuFlower, icondata_include_all))]
    LuFlower,
    #[cfg(any(LuFlower2, icondata_include_all))]
    LuFlower2,
    #[cfg(any(LuFocus, icondata_include_all))]
    LuFocus,
    #[cfg(any(LuFoldHorizontal, icondata_include_all))]
    LuFoldHorizontal,
    #[cfg(any(LuFoldVertical, icondata_include_all))]
    LuFoldVertical,
    #[cfg(any(LuFolder, icondata_include_all))]
    LuFolder,
    #[cfg(any(LuFolderArchive, icondata_include_all))]
    LuFolderArchive,
    #[cfg(any(LuFolderCheck, icondata_include_all))]
    LuFolderCheck,
    #[cfg(any(LuFolderClock, icondata_include_all))]
    LuFolderClock,
    #[cfg(any(LuFolderClosed, icondata_include_all))]
    LuFolderClosed,
    #[cfg(any(LuFolderCog, icondata_include_all))]
    LuFolderCog,
    #[cfg(any(LuFolderCog2, icondata_include_all))]
    LuFolderCog2,
    #[cfg(any(LuFolderDot, icondata_include_all))]
    LuFolderDot,
    #[cfg(any(LuFolderDown, icondata_include_all))]
    LuFolderDown,
    #[cfg(any(LuFolderEdit, icondata_include_all))]
    LuFolderEdit,
    #[cfg(any(LuFolderGit, icondata_include_all))]
    LuFolderGit,
    #[cfg(any(LuFolderGit2, icondata_include_all))]
    LuFolderGit2,
    #[cfg(any(LuFolderHeart, icondata_include_all))]
    LuFolderHeart,
    #[cfg(any(LuFolderInput, icondata_include_all))]
    LuFolderInput,
    #[cfg(any(LuFolderKanban, icondata_include_all))]
    LuFolderKanban,
    #[cfg(any(LuFolderKey, icondata_include_all))]
    LuFolderKey,
    #[cfg(any(LuFolderLock, icondata_include_all))]
    LuFolderLock,
    #[cfg(any(LuFolderMinus, icondata_include_all))]
    LuFolderMinus,
    #[cfg(any(LuFolderOpen, icondata_include_all))]
    LuFolderOpen,
    #[cfg(any(LuFolderOpenDot, icondata_include_all))]
    LuFolderOpenDot,
    #[cfg(any(LuFolderOutput, icondata_include_all))]
    LuFolderOutput,
    #[cfg(any(LuFolderPlus, icondata_include_all))]
    LuFolderPlus,
    #[cfg(any(LuFolderRoot, icondata_include_all))]
    LuFolderRoot,
    #[cfg(any(LuFolderSearch, icondata_include_all))]
    LuFolderSearch,
    #[cfg(any(LuFolderSearch2, icondata_include_all))]
    LuFolderSearch2,
    #[cfg(any(LuFolderSymlink, icondata_include_all))]
    LuFolderSymlink,
    #[cfg(any(LuFolderSync, icondata_include_all))]
    LuFolderSync,
    #[cfg(any(LuFolderTree, icondata_include_all))]
    LuFolderTree,
    #[cfg(any(LuFolderUp, icondata_include_all))]
    LuFolderUp,
    #[cfg(any(LuFolderX, icondata_include_all))]
    LuFolderX,
    #[cfg(any(LuFolders, icondata_include_all))]
    LuFolders,
    #[cfg(any(LuFootprints, icondata_include_all))]
    LuFootprints,
    #[cfg(any(LuForklift, icondata_include_all))]
    LuForklift,
    #[cfg(any(LuFormInput, icondata_include_all))]
    LuFormInput,
    #[cfg(any(LuForward, icondata_include_all))]
    LuForward,
    #[cfg(any(LuFrame, icondata_include_all))]
    LuFrame,
    #[cfg(any(LuFramer, icondata_include_all))]
    LuFramer,
    #[cfg(any(LuFrown, icondata_include_all))]
    LuFrown,
    #[cfg(any(LuFuel, icondata_include_all))]
    LuFuel,
    #[cfg(any(LuFunctionSquare, icondata_include_all))]
    LuFunctionSquare,
    #[cfg(any(LuGalleryHorizontal, icondata_include_all))]
    LuGalleryHorizontal,
    #[cfg(any(LuGalleryHorizontalEnd, icondata_include_all))]
    LuGalleryHorizontalEnd,
    #[cfg(any(LuGalleryThumbnails, icondata_include_all))]
    LuGalleryThumbnails,
    #[cfg(any(LuGalleryVertical, icondata_include_all))]
    LuGalleryVertical,
    #[cfg(any(LuGalleryVerticalEnd, icondata_include_all))]
    LuGalleryVerticalEnd,
    #[cfg(any(LuGamepad, icondata_include_all))]
    LuGamepad,
    #[cfg(any(LuGamepad2, icondata_include_all))]
    LuGamepad2,
    #[cfg(any(LuGanttChart, icondata_include_all))]
    LuGanttChart,
    #[cfg(any(LuGanttChartSquare, icondata_include_all))]
    LuGanttChartSquare,
    #[cfg(any(LuGauge, icondata_include_all))]
    LuGauge,
    #[cfg(any(LuGaugeCircle, icondata_include_all))]
    LuGaugeCircle,
    #[cfg(any(LuGavel, icondata_include_all))]
    LuGavel,
    #[cfg(any(LuGem, icondata_include_all))]
    LuGem,
    #[cfg(any(LuGhost, icondata_include_all))]
    LuGhost,
    #[cfg(any(LuGift, icondata_include_all))]
    LuGift,
    #[cfg(any(LuGitBranch, icondata_include_all))]
    LuGitBranch,
    #[cfg(any(LuGitBranchPlus, icondata_include_all))]
    LuGitBranchPlus,
    #[cfg(any(LuGitCommit, icondata_include_all))]
    LuGitCommit,
    #[cfg(any(LuGitCompare, icondata_include_all))]
    LuGitCompare,
    #[cfg(any(LuGitFork, icondata_include_all))]
    LuGitFork,
    #[cfg(any(LuGitMerge, icondata_include_all))]
    LuGitMerge,
    #[cfg(any(LuGitPullRequest, icondata_include_all))]
    LuGitPullRequest,
    #[cfg(any(LuGitPullRequestClosed, icondata_include_all))]
    LuGitPullRequestClosed,
    #[cfg(any(LuGitPullRequestDraft, icondata_include_all))]
    LuGitPullRequestDraft,
    #[cfg(any(LuGithub, icondata_include_all))]
    LuGithub,
    #[cfg(any(LuGitlab, icondata_include_all))]
    LuGitlab,
    #[cfg(any(LuGlassWater, icondata_include_all))]
    LuGlassWater,
    #[cfg(any(LuGlasses, icondata_include_all))]
    LuGlasses,
    #[cfg(any(LuGlobe, icondata_include_all))]
    LuGlobe,
    #[cfg(any(LuGlobe2, icondata_include_all))]
    LuGlobe2,
    #[cfg(any(LuGoal, icondata_include_all))]
    LuGoal,
    #[cfg(any(LuGrab, icondata_include_all))]
    LuGrab,
    #[cfg(any(LuGraduationCap, icondata_include_all))]
    LuGraduationCap,
    #[cfg(any(LuGrape, icondata_include_all))]
    LuGrape,
    #[cfg(any(LuGrid2x2, icondata_include_all))]
    LuGrid2x2,
    #[cfg(any(LuGrid3x3, icondata_include_all))]
    LuGrid3x3,
    #[cfg(any(LuGrip, icondata_include_all))]
    LuGrip,
    #[cfg(any(LuGripHorizontal, icondata_include_all))]
    LuGripHorizontal,
    #[cfg(any(LuGripVertical, icondata_include_all))]
    LuGripVertical,
    #[cfg(any(LuGroup, icondata_include_all))]
    LuGroup,
    #[cfg(any(LuHammer, icondata_include_all))]
    LuHammer,
    #[cfg(any(LuHand, icondata_include_all))]
    LuHand,
    #[cfg(any(LuHandMetal, icondata_include_all))]
    LuHandMetal,
    #[cfg(any(LuHardDrive, icondata_include_all))]
    LuHardDrive,
    #[cfg(any(LuHardDriveDownload, icondata_include_all))]
    LuHardDriveDownload,
    #[cfg(any(LuHardDriveUpload, icondata_include_all))]
    LuHardDriveUpload,
    #[cfg(any(LuHardHat, icondata_include_all))]
    LuHardHat,
    #[cfg(any(LuHash, icondata_include_all))]
    LuHash,
    #[cfg(any(LuHaze, icondata_include_all))]
    LuHaze,
    #[cfg(any(LuHdmiPort, icondata_include_all))]
    LuHdmiPort,
    #[cfg(any(LuHeading, icondata_include_all))]
    LuHeading,
    #[cfg(any(LuHeading1, icondata_include_all))]
    LuHeading1,
    #[cfg(any(LuHeading2, icondata_include_all))]
    LuHeading2,
    #[cfg(any(LuHeading3, icondata_include_all))]
    LuHeading3,
    #[cfg(any(LuHeading4, icondata_include_all))]
    LuHeading4,
    #[cfg(any(LuHeading5, icondata_include_all))]
    LuHeading5,
    #[cfg(any(LuHeading6, icondata_include_all))]
    LuHeading6,
    #[cfg(any(LuHeadphones, icondata_include_all))]
    LuHeadphones,
    #[cfg(any(LuHeart, icondata_include_all))]
    LuHeart,
    #[cfg(any(LuHeartCrack, icondata_include_all))]
    LuHeartCrack,
    #[cfg(any(LuHeartHandshake, icondata_include_all))]
    LuHeartHandshake,
    #[cfg(any(LuHeartOff, icondata_include_all))]
    LuHeartOff,
    #[cfg(any(LuHeartPulse, icondata_include_all))]
    LuHeartPulse,
    #[cfg(any(LuHelpCircle, icondata_include_all))]
    LuHelpCircle,
    #[cfg(any(LuHelpingHand, icondata_include_all))]
    LuHelpingHand,
    #[cfg(any(LuHexagon, icondata_include_all))]
    LuHexagon,
    #[cfg(any(LuHighlighter, icondata_include_all))]
    LuHighlighter,
    #[cfg(any(LuHistory, icondata_include_all))]
    LuHistory,
    #[cfg(any(LuHome, icondata_include_all))]
    LuHome,
    #[cfg(any(LuHop, icondata_include_all))]
    LuHop,
    #[cfg(any(LuHopOff, icondata_include_all))]
    LuHopOff,
    #[cfg(any(LuHotel, icondata_include_all))]
    LuHotel,
    #[cfg(any(LuHourglass, icondata_include_all))]
    LuHourglass,
    #[cfg(any(LuIceCream, icondata_include_all))]
    LuIceCream,
    #[cfg(any(LuIceCream2, icondata_include_all))]
    LuIceCream2,
    #[cfg(any(LuImage, icondata_include_all))]
    LuImage,
    #[cfg(any(LuImageMinus, icondata_include_all))]
    LuImageMinus,
    #[cfg(any(LuImageOff, icondata_include_all))]
    LuImageOff,
    #[cfg(any(LuImagePlus, icondata_include_all))]
    LuImagePlus,
    #[cfg(any(LuImport, icondata_include_all))]
    LuImport,
    #[cfg(any(LuInbox, icondata_include_all))]
    LuInbox,
    #[cfg(any(LuIndent, icondata_include_all))]
    LuIndent,
    #[cfg(any(LuIndianRupee, icondata_include_all))]
    LuIndianRupee,
    #[cfg(any(LuInfinity, icondata_include_all))]
    LuInfinity,
    #[cfg(any(LuInfo, icondata_include_all))]
    LuInfo,
    #[cfg(any(LuInspect, icondata_include_all))]
    LuInspect,
    #[cfg(any(LuInstagram, icondata_include_all))]
    LuInstagram,
    #[cfg(any(LuItalic, icondata_include_all))]
    LuItalic,
    #[cfg(any(LuIterationCcw, icondata_include_all))]
    LuIterationCcw,
    #[cfg(any(LuIterationCw, icondata_include_all))]
    LuIterationCw,
    #[cfg(any(LuJapaneseYen, icondata_include_all))]
    LuJapaneseYen,
    #[cfg(any(LuJoystick, icondata_include_all))]
    LuJoystick,
    #[cfg(any(LuKanban, icondata_include_all))]
    LuKanban,
    #[cfg(any(LuKanbanSquare, icondata_include_all))]
    LuKanbanSquare,
    #[cfg(any(LuKanbanSquareDashed, icondata_include_all))]
    LuKanbanSquareDashed,
    #[cfg(any(LuKey, icondata_include_all))]
    LuKey,
    #[cfg(any(LuKeyRound, icondata_include_all))]
    LuKeyRound,
    #[cfg(any(LuKeySquare, icondata_include_all))]
    LuKeySquare,
    #[cfg(any(LuKeyboard, icondata_include_all))]
    LuKeyboard,
    #[cfg(any(LuLamp, icondata_include_all))]
    LuLamp,
    #[cfg(any(LuLampCeiling, icondata_include_all))]
    LuLampCeiling,
    #[cfg(any(LuLampDesk, icondata_include_all))]
    LuLampDesk,
    #[cfg(any(LuLampFloor, icondata_include_all))]
    LuLampFloor,
    #[cfg(any(LuLampWallDown, icondata_include_all))]
    LuLampWallDown,
    #[cfg(any(LuLampWallUp, icondata_include_all))]
    LuLampWallUp,
    #[cfg(any(LuLandmark, icondata_include_all))]
    LuLandmark,
    #[cfg(any(LuLanguages, icondata_include_all))]
    LuLanguages,
    #[cfg(any(LuLaptop, icondata_include_all))]
    LuLaptop,
    #[cfg(any(LuLaptop2, icondata_include_all))]
    LuLaptop2,
    #[cfg(any(LuLasso, icondata_include_all))]
    LuLasso,
    #[cfg(any(LuLassoSelect, icondata_include_all))]
    LuLassoSelect,
    #[cfg(any(LuLaugh, icondata_include_all))]
    LuLaugh,
    #[cfg(any(LuLayers, icondata_include_all))]
    LuLayers,
    #[cfg(any(LuLayout, icondata_include_all))]
    LuLayout,
    #[cfg(any(LuLayoutDashboard, icondata_include_all))]
    LuLayoutDashboard,
    #[cfg(any(LuLayoutGrid, icondata_include_all))]
    LuLayoutGrid,
    #[cfg(any(LuLayoutList, icondata_include_all))]
    LuLayoutList,
    #[cfg(any(LuLayoutPanelLeft, icondata_include_all))]
    LuLayoutPanelLeft,
    #[cfg(any(LuLayoutPanelTop, icondata_include_all))]
    LuLayoutPanelTop,
    #[cfg(any(LuLayoutTemplate, icondata_include_all))]
    LuLayoutTemplate,
    #[cfg(any(LuLeaf, icondata_include_all))]
    LuLeaf,
    #[cfg(any(LuLeafyGreen, icondata_include_all))]
    LuLeafyGreen,
    #[cfg(any(LuLibrary, icondata_include_all))]
    LuLibrary,
    #[cfg(any(LuLifeBuoy, icondata_include_all))]
    LuLifeBuoy,
    #[cfg(any(LuLigature, icondata_include_all))]
    LuLigature,
    #[cfg(any(LuLightbulb, icondata_include_all))]
    LuLightbulb,
    #[cfg(any(LuLightbulbOff, icondata_include_all))]
    LuLightbulbOff,
    #[cfg(any(LuLineChart, icondata_include_all))]
    LuLineChart,
    #[cfg(any(LuLink, icondata_include_all))]
    LuLink,
    #[cfg(any(LuLink2, icondata_include_all))]
    LuLink2,
    #[cfg(any(LuLink2Off, icondata_include_all))]
    LuLink2Off,
    #[cfg(any(LuLinkedin, icondata_include_all))]
    LuLinkedin,
    #[cfg(any(LuList, icondata_include_all))]
    LuList,
    #[cfg(any(LuListChecks, icondata_include_all))]
    LuListChecks,
    #[cfg(any(LuListEnd, icondata_include_all))]
    LuListEnd,
    #[cfg(any(LuListFilter, icondata_include_all))]
    LuListFilter,
    #[cfg(any(LuListMinus, icondata_include_all))]
    LuListMinus,
    #[cfg(any(LuListMusic, icondata_include_all))]
    LuListMusic,
    #[cfg(any(LuListOrdered, icondata_include_all))]
    LuListOrdered,
    #[cfg(any(LuListPlus, icondata_include_all))]
    LuListPlus,
    #[cfg(any(LuListRestart, icondata_include_all))]
    LuListRestart,
    #[cfg(any(LuListStart, icondata_include_all))]
    LuListStart,
    #[cfg(any(LuListTodo, icondata_include_all))]
    LuListTodo,
    #[cfg(any(LuListTree, icondata_include_all))]
    LuListTree,
    #[cfg(any(LuListVideo, icondata_include_all))]
    LuListVideo,
    #[cfg(any(LuListX, icondata_include_all))]
    LuListX,
    #[cfg(any(LuLoader, icondata_include_all))]
    LuLoader,
    #[cfg(any(LuLoader2, icondata_include_all))]
    LuLoader2,
    #[cfg(any(LuLocate, icondata_include_all))]
    LuLocate,
    #[cfg(any(LuLocateFixed, icondata_include_all))]
    LuLocateFixed,
    #[cfg(any(LuLocateOff, icondata_include_all))]
    LuLocateOff,
    #[cfg(any(LuLock, icondata_include_all))]
    LuLock,
    #[cfg(any(LuLogIn, icondata_include_all))]
    LuLogIn,
    #[cfg(any(LuLogOut, icondata_include_all))]
    LuLogOut,
    #[cfg(any(LuLollipop, icondata_include_all))]
    LuLollipop,
    #[cfg(any(LuLuggage, icondata_include_all))]
    LuLuggage,
    #[cfg(any(LuMSquare, icondata_include_all))]
    LuMSquare,
    #[cfg(any(LuMagnet, icondata_include_all))]
    LuMagnet,
    #[cfg(any(LuMail, icondata_include_all))]
    LuMail,
    #[cfg(any(LuMailCheck, icondata_include_all))]
    LuMailCheck,
    #[cfg(any(LuMailMinus, icondata_include_all))]
    LuMailMinus,
    #[cfg(any(LuMailOpen, icondata_include_all))]
    LuMailOpen,
    #[cfg(any(LuMailPlus, icondata_include_all))]
    LuMailPlus,
    #[cfg(any(LuMailQuestion, icondata_include_all))]
    LuMailQuestion,
    #[cfg(any(LuMailSearch, icondata_include_all))]
    LuMailSearch,
    #[cfg(any(LuMailWarning, icondata_include_all))]
    LuMailWarning,
    #[cfg(any(LuMailX, icondata_include_all))]
    LuMailX,
    #[cfg(any(LuMailbox, icondata_include_all))]
    LuMailbox,
    #[cfg(any(LuMails, icondata_include_all))]
    LuMails,
    #[cfg(any(LuMap, icondata_include_all))]
    LuMap,
    #[cfg(any(LuMapPin, icondata_include_all))]
    LuMapPin,
    #[cfg(any(LuMapPinOff, icondata_include_all))]
    LuMapPinOff,
    #[cfg(any(LuMartini, icondata_include_all))]
    LuMartini,
    #[cfg(any(LuMaximize, icondata_include_all))]
    LuMaximize,
    #[cfg(any(LuMaximize2, icondata_include_all))]
    LuMaximize2,
    #[cfg(any(LuMedal, icondata_include_all))]
    LuMedal,
    #[cfg(any(LuMegaphone, icondata_include_all))]
    LuMegaphone,
    #[cfg(any(LuMegaphoneOff, icondata_include_all))]
    LuMegaphoneOff,
    #[cfg(any(LuMeh, icondata_include_all))]
    LuMeh,
    #[cfg(any(LuMemoryStick, icondata_include_all))]
    LuMemoryStick,
    #[cfg(any(LuMenu, icondata_include_all))]
    LuMenu,
    #[cfg(any(LuMenuSquare, icondata_include_all))]
    LuMenuSquare,
    #[cfg(any(LuMerge, icondata_include_all))]
    LuMerge,
    #[cfg(any(LuMessageCircle, icondata_include_all))]
    LuMessageCircle,
    #[cfg(any(LuMessageSquare, icondata_include_all))]
    LuMessageSquare,
    #[cfg(any(LuMessageSquareDashed, icondata_include_all))]
    LuMessageSquareDashed,
    #[cfg(any(LuMessageSquarePlus, icondata_include_all))]
    LuMessageSquarePlus,
    #[cfg(any(LuMessagesSquare, icondata_include_all))]
    LuMessagesSquare,
    #[cfg(any(LuMic, icondata_include_all))]
    LuMic,
    #[cfg(any(LuMic2, icondata_include_all))]
    LuMic2,
    #[cfg(any(LuMicOff, icondata_include_all))]
    LuMicOff,
    #[cfg(any(LuMicroscope, icondata_include_all))]
    LuMicroscope,
    #[cfg(any(LuMicrowave, icondata_include_all))]
    LuMicrowave,
    #[cfg(any(LuMilestone, icondata_include_all))]
    LuMilestone,
    #[cfg(any(LuMilk, icondata_include_all))]
    LuMilk,
    #[cfg(any(LuMilkOff, icondata_include_all))]
    LuMilkOff,
    #[cfg(any(LuMinimize, icondata_include_all))]
    LuMinimize,
    #[cfg(any(LuMinimize2, icondata_include_all))]
    LuMinimize2,
    #[cfg(any(LuMinus, icondata_include_all))]
    LuMinus,
    #[cfg(any(LuMinusCircle, icondata_include_all))]
    LuMinusCircle,
    #[cfg(any(LuMinusSquare, icondata_include_all))]
    LuMinusSquare,
    #[cfg(any(LuMonitor, icondata_include_all))]
    LuMonitor,
    #[cfg(any(LuMonitorCheck, icondata_include_all))]
    LuMonitorCheck,
    #[cfg(any(LuMonitorDot, icondata_include_all))]
    LuMonitorDot,
    #[cfg(any(LuMonitorDown, icondata_include_all))]
    LuMonitorDown,
    #[cfg(any(LuMonitorOff, icondata_include_all))]
    LuMonitorOff,
    #[cfg(any(LuMonitorPause, icondata_include_all))]
    LuMonitorPause,
    #[cfg(any(LuMonitorPlay, icondata_include_all))]
    LuMonitorPlay,
    #[cfg(any(LuMonitorSmartphone, icondata_include_all))]
    LuMonitorSmartphone,
    #[cfg(any(LuMonitorSpeaker, icondata_include_all))]
    LuMonitorSpeaker,
    #[cfg(any(LuMonitorStop, icondata_include_all))]
    LuMonitorStop,
    #[cfg(any(LuMonitorUp, icondata_include_all))]
    LuMonitorUp,
    #[cfg(any(LuMonitorX, icondata_include_all))]
    LuMonitorX,
    #[cfg(any(LuMoon, icondata_include_all))]
    LuMoon,
    #[cfg(any(LuMoonStar, icondata_include_all))]
    LuMoonStar,
    #[cfg(any(LuMoreHorizontal, icondata_include_all))]
    LuMoreHorizontal,
    #[cfg(any(LuMoreVertical, icondata_include_all))]
    LuMoreVertical,
    #[cfg(any(LuMountain, icondata_include_all))]
    LuMountain,
    #[cfg(any(LuMountainSnow, icondata_include_all))]
    LuMountainSnow,
    #[cfg(any(LuMouse, icondata_include_all))]
    LuMouse,
    #[cfg(any(LuMousePointer, icondata_include_all))]
    LuMousePointer,
    #[cfg(any(LuMousePointer2, icondata_include_all))]
    LuMousePointer2,
    #[cfg(any(LuMousePointerClick, icondata_include_all))]
    LuMousePointerClick,
    #[cfg(any(LuMove, icondata_include_all))]
    LuMove,
    #[cfg(any(LuMove3d, icondata_include_all))]
    LuMove3d,
    #[cfg(any(LuMoveDiagonal, icondata_include_all))]
    LuMoveDiagonal,
    #[cfg(any(LuMoveDiagonal2, icondata_include_all))]
    LuMoveDiagonal2,
    #[cfg(any(LuMoveDown, icondata_include_all))]
    LuMoveDown,
    #[cfg(any(LuMoveDownLeft, icondata_include_all))]
    LuMoveDownLeft,
    #[cfg(any(LuMoveDownRight, icondata_include_all))]
    LuMoveDownRight,
    #[cfg(any(LuMoveHorizontal, icondata_include_all))]
    LuMoveHorizontal,
    #[cfg(any(LuMoveLeft, icondata_include_all))]
    LuMoveLeft,
    #[cfg(any(LuMoveRight, icondata_include_all))]
    LuMoveRight,
    #[cfg(any(LuMoveUp, icondata_include_all))]
    LuMoveUp,
    #[cfg(any(LuMoveUpLeft, icondata_include_all))]
    LuMoveUpLeft,
    #[cfg(any(LuMoveUpRight, icondata_include_all))]
    LuMoveUpRight,
    #[cfg(any(LuMoveVertical, icondata_include_all))]
    LuMoveVertical,
    #[cfg(any(LuMusic, icondata_include_all))]
    LuMusic,
    #[cfg(any(LuMusic2, icondata_include_all))]
    LuMusic2,
    #[cfg(any(LuMusic3, icondata_include_all))]
    LuMusic3,
    #[cfg(any(LuMusic4, icondata_include_all))]
    LuMusic4,
    #[cfg(any(LuNavigation, icondata_include_all))]
    LuNavigation,
    #[cfg(any(LuNavigation2, icondata_include_all))]
    LuNavigation2,
    #[cfg(any(LuNavigation2Off, icondata_include_all))]
    LuNavigation2Off,
    #[cfg(any(LuNavigationOff, icondata_include_all))]
    LuNavigationOff,
    #[cfg(any(LuNetwork, icondata_include_all))]
    LuNetwork,
    #[cfg(any(LuNewspaper, icondata_include_all))]
    LuNewspaper,
    #[cfg(any(LuNfc, icondata_include_all))]
    LuNfc,
    #[cfg(any(LuNut, icondata_include_all))]
    LuNut,
    #[cfg(any(LuNutOff, icondata_include_all))]
    LuNutOff,
    #[cfg(any(LuOctagon, icondata_include_all))]
    LuOctagon,
    #[cfg(any(LuOption, icondata_include_all))]
    LuOption,
    #[cfg(any(LuOrbit, icondata_include_all))]
    LuOrbit,
    #[cfg(any(LuOutdent, icondata_include_all))]
    LuOutdent,
    #[cfg(any(LuPackage, icondata_include_all))]
    LuPackage,
    #[cfg(any(LuPackage2, icondata_include_all))]
    LuPackage2,
    #[cfg(any(LuPackageCheck, icondata_include_all))]
    LuPackageCheck,
    #[cfg(any(LuPackageMinus, icondata_include_all))]
    LuPackageMinus,
    #[cfg(any(LuPackageOpen, icondata_include_all))]
    LuPackageOpen,
    #[cfg(any(LuPackagePlus, icondata_include_all))]
    LuPackagePlus,
    #[cfg(any(LuPackageSearch, icondata_include_all))]
    LuPackageSearch,
    #[cfg(any(LuPackageX, icondata_include_all))]
    LuPackageX,
    #[cfg(any(LuPaintBucket, icondata_include_all))]
    LuPaintBucket,
    #[cfg(any(LuPaintbrush, icondata_include_all))]
    LuPaintbrush,
    #[cfg(any(LuPaintbrush2, icondata_include_all))]
    LuPaintbrush2,
    #[cfg(any(LuPalette, icondata_include_all))]
    LuPalette,
    #[cfg(any(LuPalmtree, icondata_include_all))]
    LuPalmtree,
    #[cfg(any(LuPanelBottom, icondata_include_all))]
    LuPanelBottom,
    #[cfg(any(LuPanelBottomClose, icondata_include_all))]
    LuPanelBottomClose,
    #[cfg(any(LuPanelBottomInactive, icondata_include_all))]
    LuPanelBottomInactive,
    #[cfg(any(LuPanelBottomOpen, icondata_include_all))]
    LuPanelBottomOpen,
    #[cfg(any(LuPanelLeft, icondata_include_all))]
    LuPanelLeft,
    #[cfg(any(LuPanelLeftClose, icondata_include_all))]
    LuPanelLeftClose,
    #[cfg(any(LuPanelLeftInactive, icondata_include_all))]
    LuPanelLeftInactive,
    #[cfg(any(LuPanelLeftOpen, icondata_include_all))]
    LuPanelLeftOpen,
    #[cfg(any(LuPanelRight, icondata_include_all))]
    LuPanelRight,
    #[cfg(any(LuPanelRightClose, icondata_include_all))]
    LuPanelRightClose,
    #[cfg(any(LuPanelRightInactive, icondata_include_all))]
    LuPanelRightInactive,
    #[cfg(any(LuPanelRightOpen, icondata_include_all))]
    LuPanelRightOpen,
    #[cfg(any(LuPanelTop, icondata_include_all))]
    LuPanelTop,
    #[cfg(any(LuPanelTopClose, icondata_include_all))]
    LuPanelTopClose,
    #[cfg(any(LuPanelTopInactive, icondata_include_all))]
    LuPanelTopInactive,
    #[cfg(any(LuPanelTopOpen, icondata_include_all))]
    LuPanelTopOpen,
    #[cfg(any(LuPaperclip, icondata_include_all))]
    LuPaperclip,
    #[cfg(any(LuParentheses, icondata_include_all))]
    LuParentheses,
    #[cfg(any(LuParkingCircle, icondata_include_all))]
    LuParkingCircle,
    #[cfg(any(LuParkingCircleOff, icondata_include_all))]
    LuParkingCircleOff,
    #[cfg(any(LuParkingMeter, icondata_include_all))]
    LuParkingMeter,
    #[cfg(any(LuParkingSquare, icondata_include_all))]
    LuParkingSquare,
    #[cfg(any(LuParkingSquareOff, icondata_include_all))]
    LuParkingSquareOff,
    #[cfg(any(LuPartyPopper, icondata_include_all))]
    LuPartyPopper,
    #[cfg(any(LuPause, icondata_include_all))]
    LuPause,
    #[cfg(any(LuPauseCircle, icondata_include_all))]
    LuPauseCircle,
    #[cfg(any(LuPauseOctagon, icondata_include_all))]
    LuPauseOctagon,
    #[cfg(any(LuPawPrint, icondata_include_all))]
    LuPawPrint,
    #[cfg(any(LuPcCase, icondata_include_all))]
    LuPcCase,
    #[cfg(any(LuPen, icondata_include_all))]
    LuPen,
    #[cfg(any(LuPenLine, icondata_include_all))]
    LuPenLine,
    #[cfg(any(LuPenSquare, icondata_include_all))]
    LuPenSquare,
    #[cfg(any(LuPenTool, icondata_include_all))]
    LuPenTool,
    #[cfg(any(LuPencil, icondata_include_all))]
    LuPencil,
    #[cfg(any(LuPencilLine, icondata_include_all))]
    LuPencilLine,
    #[cfg(any(LuPencilRuler, icondata_include_all))]
    LuPencilRuler,
    #[cfg(any(LuPercent, icondata_include_all))]
    LuPercent,
    #[cfg(any(LuPersonStanding, icondata_include_all))]
    LuPersonStanding,
    #[cfg(any(LuPhone, icondata_include_all))]
    LuPhone,
    #[cfg(any(LuPhoneCall, icondata_include_all))]
    LuPhoneCall,
    #[cfg(any(LuPhoneForwarded, icondata_include_all))]
    LuPhoneForwarded,
    #[cfg(any(LuPhoneIncoming, icondata_include_all))]
    LuPhoneIncoming,
    #[cfg(any(LuPhoneMissed, icondata_include_all))]
    LuPhoneMissed,
    #[cfg(any(LuPhoneOff, icondata_include_all))]
    LuPhoneOff,
    #[cfg(any(LuPhoneOutgoing, icondata_include_all))]
    LuPhoneOutgoing,
    #[cfg(any(LuPi, icondata_include_all))]
    LuPi,
    #[cfg(any(LuPiSquare, icondata_include_all))]
    LuPiSquare,
    #[cfg(any(LuPictureInPicture, icondata_include_all))]
    LuPictureInPicture,
    #[cfg(any(LuPictureInPicture2, icondata_include_all))]
    LuPictureInPicture2,
    #[cfg(any(LuPieChart, icondata_include_all))]
    LuPieChart,
    #[cfg(any(LuPiggyBank, icondata_include_all))]
    LuPiggyBank,
    #[cfg(any(LuPilcrow, icondata_include_all))]
    LuPilcrow,
    #[cfg(any(LuPilcrowSquare, icondata_include_all))]
    LuPilcrowSquare,
    #[cfg(any(LuPill, icondata_include_all))]
    LuPill,
    #[cfg(any(LuPin, icondata_include_all))]
    LuPin,
    #[cfg(any(LuPinOff, icondata_include_all))]
    LuPinOff,
    #[cfg(any(LuPipette, icondata_include_all))]
    LuPipette,
    #[cfg(any(LuPizza, icondata_include_all))]
    LuPizza,
    #[cfg(any(LuPlane, icondata_include_all))]
    LuPlane,
    #[cfg(any(LuPlaneLanding, icondata_include_all))]
    LuPlaneLanding,
    #[cfg(any(LuPlaneTakeoff, icondata_include_all))]
    LuPlaneTakeoff,
    #[cfg(any(LuPlay, icondata_include_all))]
    LuPlay,
    #[cfg(any(LuPlayCircle, icondata_include_all))]
    LuPlayCircle,
    #[cfg(any(LuPlaySquare, icondata_include_all))]
    LuPlaySquare,
    #[cfg(any(LuPlug, icondata_include_all))]
    LuPlug,
    #[cfg(any(LuPlug2, icondata_include_all))]
    LuPlug2,
    #[cfg(any(LuPlugZap, icondata_include_all))]
    LuPlugZap,
    #[cfg(any(LuPlugZap2, icondata_include_all))]
    LuPlugZap2,
    #[cfg(any(LuPlus, icondata_include_all))]
    LuPlus,
    #[cfg(any(LuPlusCircle, icondata_include_all))]
    LuPlusCircle,
    #[cfg(any(LuPlusSquare, icondata_include_all))]
    LuPlusSquare,
    #[cfg(any(LuPocket, icondata_include_all))]
    LuPocket,
    #[cfg(any(LuPocketKnife, icondata_include_all))]
    LuPocketKnife,
    #[cfg(any(LuPodcast, icondata_include_all))]
    LuPodcast,
    #[cfg(any(LuPointer, icondata_include_all))]
    LuPointer,
    #[cfg(any(LuPopcorn, icondata_include_all))]
    LuPopcorn,
    #[cfg(any(LuPopsicle, icondata_include_all))]
    LuPopsicle,
    #[cfg(any(LuPoundSterling, icondata_include_all))]
    LuPoundSterling,
    #[cfg(any(LuPower, icondata_include_all))]
    LuPower,
    #[cfg(any(LuPowerOff, icondata_include_all))]
    LuPowerOff,
    #[cfg(any(LuPresentation, icondata_include_all))]
    LuPresentation,
    #[cfg(any(LuPrinter, icondata_include_all))]
    LuPrinter,
    #[cfg(any(LuProjector, icondata_include_all))]
    LuProjector,
    #[cfg(any(LuPuzzle, icondata_include_all))]
    LuPuzzle,
    #[cfg(any(LuQrCode, icondata_include_all))]
    LuQrCode,
    #[cfg(any(LuQuote, icondata_include_all))]
    LuQuote,
    #[cfg(any(LuRabbit, icondata_include_all))]
    LuRabbit,
    #[cfg(any(LuRadar, icondata_include_all))]
    LuRadar,
    #[cfg(any(LuRadiation, icondata_include_all))]
    LuRadiation,
    #[cfg(any(LuRadio, icondata_include_all))]
    LuRadio,
    #[cfg(any(LuRadioReceiver, icondata_include_all))]
    LuRadioReceiver,
    #[cfg(any(LuRadioTower, icondata_include_all))]
    LuRadioTower,
    #[cfg(any(LuRailSymbol, icondata_include_all))]
    LuRailSymbol,
    #[cfg(any(LuRainbow, icondata_include_all))]
    LuRainbow,
    #[cfg(any(LuRat, icondata_include_all))]
    LuRat,
    #[cfg(any(LuRatio, icondata_include_all))]
    LuRatio,
    #[cfg(any(LuReceipt, icondata_include_all))]
    LuReceipt,
    #[cfg(any(LuRectangleHorizontal, icondata_include_all))]
    LuRectangleHorizontal,
    #[cfg(any(LuRectangleVertical, icondata_include_all))]
    LuRectangleVertical,
    #[cfg(any(LuRecycle, icondata_include_all))]
    LuRecycle,
    #[cfg(any(LuRedo, icondata_include_all))]
    LuRedo,
    #[cfg(any(LuRedo2, icondata_include_all))]
    LuRedo2,
    #[cfg(any(LuRedoDot, icondata_include_all))]
    LuRedoDot,
    #[cfg(any(LuRefreshCcw, icondata_include_all))]
    LuRefreshCcw,
    #[cfg(any(LuRefreshCcwDot, icondata_include_all))]
    LuRefreshCcwDot,
    #[cfg(any(LuRefreshCw, icondata_include_all))]
    LuRefreshCw,
    #[cfg(any(LuRefreshCwOff, icondata_include_all))]
    LuRefreshCwOff,
    #[cfg(any(LuRefrigerator, icondata_include_all))]
    LuRefrigerator,
    #[cfg(any(LuRegex, icondata_include_all))]
    LuRegex,
    #[cfg(any(LuRemoveFormatting, icondata_include_all))]
    LuRemoveFormatting,
    #[cfg(any(LuRepeat, icondata_include_all))]
    LuRepeat,
    #[cfg(any(LuRepeat1, icondata_include_all))]
    LuRepeat1,
    #[cfg(any(LuRepeat2, icondata_include_all))]
    LuRepeat2,
    #[cfg(any(LuReplace, icondata_include_all))]
    LuReplace,
    #[cfg(any(LuReplaceAll, icondata_include_all))]
    LuReplaceAll,
    #[cfg(any(LuReply, icondata_include_all))]
    LuReply,
    #[cfg(any(LuReplyAll, icondata_include_all))]
    LuReplyAll,
    #[cfg(any(LuRewind, icondata_include_all))]
    LuRewind,
    #[cfg(any(LuRocket, icondata_include_all))]
    LuRocket,
    #[cfg(any(LuRockingChair, icondata_include_all))]
    LuRockingChair,
    #[cfg(any(LuRollerCoaster, icondata_include_all))]
    LuRollerCoaster,
    #[cfg(any(LuRotate3d, icondata_include_all))]
    LuRotate3d,
    #[cfg(any(LuRotateCcw, icondata_include_all))]
    LuRotateCcw,
    #[cfg(any(LuRotateCw, icondata_include_all))]
    LuRotateCw,
    #[cfg(any(LuRouter, icondata_include_all))]
    LuRouter,
    #[cfg(any(LuRows, icondata_include_all))]
    LuRows,
    #[cfg(any(LuRss, icondata_include_all))]
    LuRss,
    #[cfg(any(LuRuler, icondata_include_all))]
    LuRuler,
    #[cfg(any(LuRussianRuble, icondata_include_all))]
    LuRussianRuble,
    #[cfg(any(LuSailboat, icondata_include_all))]
    LuSailboat,
    #[cfg(any(LuSalad, icondata_include_all))]
    LuSalad,
    #[cfg(any(LuSandwich, icondata_include_all))]
    LuSandwich,
    #[cfg(any(LuSatellite, icondata_include_all))]
    LuSatellite,
    #[cfg(any(LuSatelliteDish, icondata_include_all))]
    LuSatelliteDish,
    #[cfg(any(LuSave, icondata_include_all))]
    LuSave,
    #[cfg(any(LuSaveAll, icondata_include_all))]
    LuSaveAll,
    #[cfg(any(LuScale, icondata_include_all))]
    LuScale,
    #[cfg(any(LuScale3d, icondata_include_all))]
    LuScale3d,
    #[cfg(any(LuScaling, icondata_include_all))]
    LuScaling,
    #[cfg(any(LuScan, icondata_include_all))]
    LuScan,
    #[cfg(any(LuScanFace, icondata_include_all))]
    LuScanFace,
    #[cfg(any(LuScanLine, icondata_include_all))]
    LuScanLine,
    #[cfg(any(LuScatterChart, icondata_include_all))]
    LuScatterChart,
    #[cfg(any(LuSchool, icondata_include_all))]
    LuSchool,
    #[cfg(any(LuSchool2, icondata_include_all))]
    LuSchool2,
    #[cfg(any(LuScissors, icondata_include_all))]
    LuScissors,
    #[cfg(any(LuScissorsLineDashed, icondata_include_all))]
    LuScissorsLineDashed,
    #[cfg(any(LuScissorsSquare, icondata_include_all))]
    LuScissorsSquare,
    #[cfg(any(LuScissorsSquareDashedBottom, icondata_include_all))]
    LuScissorsSquareDashedBottom,
    #[cfg(any(LuScreenShare, icondata_include_all))]
    LuScreenShare,
    #[cfg(any(LuScreenShareOff, icondata_include_all))]
    LuScreenShareOff,
    #[cfg(any(LuScroll, icondata_include_all))]
    LuScroll,
    #[cfg(any(LuScrollText, icondata_include_all))]
    LuScrollText,
    #[cfg(any(LuSearch, icondata_include_all))]
    LuSearch,
    #[cfg(any(LuSearchCheck, icondata_include_all))]
    LuSearchCheck,
    #[cfg(any(LuSearchCode, icondata_include_all))]
    LuSearchCode,
    #[cfg(any(LuSearchSlash, icondata_include_all))]
    LuSearchSlash,
    #[cfg(any(LuSearchX, icondata_include_all))]
    LuSearchX,
    #[cfg(any(LuSend, icondata_include_all))]
    LuSend,
    #[cfg(any(LuSendHorizonal, icondata_include_all))]
    LuSendHorizonal,
    #[cfg(any(LuSendToBack, icondata_include_all))]
    LuSendToBack,
    #[cfg(any(LuSeparatorHorizontal, icondata_include_all))]
    LuSeparatorHorizontal,
    #[cfg(any(LuSeparatorVertical, icondata_include_all))]
    LuSeparatorVertical,
    #[cfg(any(LuServer, icondata_include_all))]
    LuServer,
    #[cfg(any(LuServerCog, icondata_include_all))]
    LuServerCog,
    #[cfg(any(LuServerCrash, icondata_include_all))]
    LuServerCrash,
    #[cfg(any(LuServerOff, icondata_include_all))]
    LuServerOff,
    #[cfg(any(LuSettings, icondata_include_all))]
    LuSettings,
    #[cfg(any(LuSettings2, icondata_include_all))]
    LuSettings2,
    #[cfg(any(LuShapes, icondata_include_all))]
    LuShapes,
    #[cfg(any(LuShare, icondata_include_all))]
    LuShare,
    #[cfg(any(LuShare2, icondata_include_all))]
    LuShare2,
    #[cfg(any(LuSheet, icondata_include_all))]
    LuSheet,
    #[cfg(any(LuShell, icondata_include_all))]
    LuShell,
    #[cfg(any(LuShield, icondata_include_all))]
    LuShield,
    #[cfg(any(LuShieldAlert, icondata_include_all))]
    LuShieldAlert,
    #[cfg(any(LuShieldCheck, icondata_include_all))]
    LuShieldCheck,
    #[cfg(any(LuShieldClose, icondata_include_all))]
    LuShieldClose,
    #[cfg(any(LuShieldOff, icondata_include_all))]
    LuShieldOff,
    #[cfg(any(LuShieldQuestion, icondata_include_all))]
    LuShieldQuestion,
    #[cfg(any(LuShip, icondata_include_all))]
    LuShip,
    #[cfg(any(LuShipWheel, icondata_include_all))]
    LuShipWheel,
    #[cfg(any(LuShirt, icondata_include_all))]
    LuShirt,
    #[cfg(any(LuShoppingBag, icondata_include_all))]
    LuShoppingBag,
    #[cfg(any(LuShoppingBasket, icondata_include_all))]
    LuShoppingBasket,
    #[cfg(any(LuShoppingCart, icondata_include_all))]
    LuShoppingCart,
    #[cfg(any(LuShovel, icondata_include_all))]
    LuShovel,
    #[cfg(any(LuShowerHead, icondata_include_all))]
    LuShowerHead,
    #[cfg(any(LuShrink, icondata_include_all))]
    LuShrink,
    #[cfg(any(LuShrub, icondata_include_all))]
    LuShrub,
    #[cfg(any(LuShuffle, icondata_include_all))]
    LuShuffle,
    #[cfg(any(LuSigma, icondata_include_all))]
    LuSigma,
    #[cfg(any(LuSigmaSquare, icondata_include_all))]
    LuSigmaSquare,
    #[cfg(any(LuSignal, icondata_include_all))]
    LuSignal,
    #[cfg(any(LuSignalHigh, icondata_include_all))]
    LuSignalHigh,
    #[cfg(any(LuSignalLow, icondata_include_all))]
    LuSignalLow,
    #[cfg(any(LuSignalMedium, icondata_include_all))]
    LuSignalMedium,
    #[cfg(any(LuSignalZero, icondata_include_all))]
    LuSignalZero,
    #[cfg(any(LuSiren, icondata_include_all))]
    LuSiren,
    #[cfg(any(LuSkipBack, icondata_include_all))]
    LuSkipBack,
    #[cfg(any(LuSkipForward, icondata_include_all))]
    LuSkipForward,
    #[cfg(any(LuSkull, icondata_include_all))]
    LuSkull,
    #[cfg(any(LuSlack, icondata_include_all))]
    LuSlack,
    #[cfg(any(LuSlice, icondata_include_all))]
    LuSlice,
    #[cfg(any(LuSliders, icondata_include_all))]
    LuSliders,
    #[cfg(any(LuSlidersHorizontal, icondata_include_all))]
    LuSlidersHorizontal,
    #[cfg(any(LuSmartphone, icondata_include_all))]
    LuSmartphone,
    #[cfg(any(LuSmartphoneCharging, icondata_include_all))]
    LuSmartphoneCharging,
    #[cfg(any(LuSmartphoneNfc, icondata_include_all))]
    LuSmartphoneNfc,
    #[cfg(any(LuSmile, icondata_include_all))]
    LuSmile,
    #[cfg(any(LuSmilePlus, icondata_include_all))]
    LuSmilePlus,
    #[cfg(any(LuSnail, icondata_include_all))]
    LuSnail,
    #[cfg(any(LuSnowflake, icondata_include_all))]
    LuSnowflake,
    #[cfg(any(LuSofa, icondata_include_all))]
    LuSofa,
    #[cfg(any(LuSoup, icondata_include_all))]
    LuSoup,
    #[cfg(any(LuSpace, icondata_include_all))]
    LuSpace,
    #[cfg(any(LuSpade, icondata_include_all))]
    LuSpade,
    #[cfg(any(LuSparkle, icondata_include_all))]
    LuSparkle,
    #[cfg(any(LuSparkles, icondata_include_all))]
    LuSparkles,
    #[cfg(any(LuSpeaker, icondata_include_all))]
    LuSpeaker,
    #[cfg(any(LuSpellCheck, icondata_include_all))]
    LuSpellCheck,
    #[cfg(any(LuSpellCheck2, icondata_include_all))]
    LuSpellCheck2,
    #[cfg(any(LuSpline, icondata_include_all))]
    LuSpline,
    #[cfg(any(LuSplit, icondata_include_all))]
    LuSplit,
    #[cfg(any(LuSplitSquareHorizontal, icondata_include_all))]
    LuSplitSquareHorizontal,
    #[cfg(any(LuSplitSquareVertical, icondata_include_all))]
    LuSplitSquareVertical,
    #[cfg(any(LuSprayCan, icondata_include_all))]
    LuSprayCan,
    #[cfg(any(LuSprout, icondata_include_all))]
    LuSprout,
    #[cfg(any(LuSquare, icondata_include_all))]
    LuSquare,
    #[cfg(any(LuSquareAsterisk, icondata_include_all))]
    LuSquareAsterisk,
    #[cfg(any(LuSquareCode, icondata_include_all))]
    LuSquareCode,
    #[cfg(any(LuSquareDashedBottom, icondata_include_all))]
    LuSquareDashedBottom,
    #[cfg(any(LuSquareDashedBottomCode, icondata_include_all))]
    LuSquareDashedBottomCode,
    #[cfg(any(LuSquareDot, icondata_include_all))]
    LuSquareDot,
    #[cfg(any(LuSquareEqual, icondata_include_all))]
    LuSquareEqual,
    #[cfg(any(LuSquareSlash, icondata_include_all))]
    LuSquareSlash,
    #[cfg(any(LuSquareStack, icondata_include_all))]
    LuSquareStack,
    #[cfg(any(LuSquirrel, icondata_include_all))]
    LuSquirrel,
    #[cfg(any(LuStamp, icondata_include_all))]
    LuStamp,
    #[cfg(any(LuStar, icondata_include_all))]
    LuStar,
    #[cfg(any(LuStarHalf, icondata_include_all))]
    LuStarHalf,
    #[cfg(any(LuStarOff, icondata_include_all))]
    LuStarOff,
    #[cfg(any(LuStepBack, icondata_include_all))]
    LuStepBack,
    #[cfg(any(LuStepForward, icondata_include_all))]
    LuStepForward,
    #[cfg(any(LuStethoscope, icondata_include_all))]
    LuStethoscope,
    #[cfg(any(LuSticker, icondata_include_all))]
    LuSticker,
    #[cfg(any(LuStickyNote, icondata_include_all))]
    LuStickyNote,
    #[cfg(any(LuStopCircle, icondata_include_all))]
    LuStopCircle,
    #[cfg(any(LuStore, icondata_include_all))]
    LuStore,
    #[cfg(any(LuStretchHorizontal, icondata_include_all))]
    LuStretchHorizontal,
    #[cfg(any(LuStretchVertical, icondata_include_all))]
    LuStretchVertical,
    #[cfg(any(LuStrikethrough, icondata_include_all))]
    LuStrikethrough,
    #[cfg(any(LuSubscript, icondata_include_all))]
    LuSubscript,
    #[cfg(any(LuSubtitles, icondata_include_all))]
    LuSubtitles,
    #[cfg(any(LuSun, icondata_include_all))]
    LuSun,
    #[cfg(any(LuSunDim, icondata_include_all))]
    LuSunDim,
    #[cfg(any(LuSunMedium, icondata_include_all))]
    LuSunMedium,
    #[cfg(any(LuSunMoon, icondata_include_all))]
    LuSunMoon,
    #[cfg(any(LuSunSnow, icondata_include_all))]
    LuSunSnow,
    #[cfg(any(LuSunrise, icondata_include_all))]
    LuSunrise,
    #[cfg(any(LuSunset, icondata_include_all))]
    LuSunset,
    #[cfg(any(LuSuperscript, icondata_include_all))]
    LuSuperscript,
    #[cfg(any(LuSwissFranc, icondata_include_all))]
    LuSwissFranc,
    #[cfg(any(LuSwitchCamera, icondata_include_all))]
    LuSwitchCamera,
    #[cfg(any(LuSword, icondata_include_all))]
    LuSword,
    #[cfg(any(LuSwords, icondata_include_all))]
    LuSwords,
    #[cfg(any(LuSyringe, icondata_include_all))]
    LuSyringe,
    #[cfg(any(LuTable, icondata_include_all))]
    LuTable,
    #[cfg(any(LuTable2, icondata_include_all))]
    LuTable2,
    #[cfg(any(LuTableProperties, icondata_include_all))]
    LuTableProperties,
    #[cfg(any(LuTablet, icondata_include_all))]
    LuTablet,
    #[cfg(any(LuTablets, icondata_include_all))]
    LuTablets,
    #[cfg(any(LuTag, icondata_include_all))]
    LuTag,
    #[cfg(any(LuTags, icondata_include_all))]
    LuTags,
    #[cfg(any(LuTally1, icondata_include_all))]
    LuTally1,
    #[cfg(any(LuTally2, icondata_include_all))]
    LuTally2,
    #[cfg(any(LuTally3, icondata_include_all))]
    LuTally3,
    #[cfg(any(LuTally4, icondata_include_all))]
    LuTally4,
    #[cfg(any(LuTally5, icondata_include_all))]
    LuTally5,
    #[cfg(any(LuTarget, icondata_include_all))]
    LuTarget,
    #[cfg(any(LuTent, icondata_include_all))]
    LuTent,
    #[cfg(any(LuTerminal, icondata_include_all))]
    LuTerminal,
    #[cfg(any(LuTerminalSquare, icondata_include_all))]
    LuTerminalSquare,
    #[cfg(any(LuTestTube, icondata_include_all))]
    LuTestTube,
    #[cfg(any(LuTestTube2, icondata_include_all))]
    LuTestTube2,
    #[cfg(any(LuTestTubes, icondata_include_all))]
    LuTestTubes,
    #[cfg(any(LuText, icondata_include_all))]
    LuText,
    #[cfg(any(LuTextCursor, icondata_include_all))]
    LuTextCursor,
    #[cfg(any(LuTextCursorInput, icondata_include_all))]
    LuTextCursorInput,
    #[cfg(any(LuTextQuote, icondata_include_all))]
    LuTextQuote,
    #[cfg(any(LuTextSelect, icondata_include_all))]
    LuTextSelect,
    #[cfg(any(LuThermometer, icondata_include_all))]
    LuThermometer,
    #[cfg(any(LuThermometerSnowflake, icondata_include_all))]
    LuThermometerSnowflake,
    #[cfg(any(LuThermometerSun, icondata_include_all))]
    LuThermometerSun,
    #[cfg(any(LuThumbsDown, icondata_include_all))]
    LuThumbsDown,
    #[cfg(any(LuThumbsUp, icondata_include_all))]
    LuThumbsUp,
    #[cfg(any(LuTicket, icondata_include_all))]
    LuTicket,
    #[cfg(any(LuTimer, icondata_include_all))]
    LuTimer,
    #[cfg(any(LuTimerOff, icondata_include_all))]
    LuTimerOff,
    #[cfg(any(LuTimerReset, icondata_include_all))]
    LuTimerReset,
    #[cfg(any(LuToggleLeft, icondata_include_all))]
    LuToggleLeft,
    #[cfg(any(LuToggleRight, icondata_include_all))]
    LuToggleRight,
    #[cfg(any(LuTornado, icondata_include_all))]
    LuTornado,
    #[cfg(any(LuTouchpad, icondata_include_all))]
    LuTouchpad,
    #[cfg(any(LuTouchpadOff, icondata_include_all))]
    LuTouchpadOff,
    #[cfg(any(LuTowerControl, icondata_include_all))]
    LuTowerControl,
    #[cfg(any(LuToyBrick, icondata_include_all))]
    LuToyBrick,
    #[cfg(any(LuTractor, icondata_include_all))]
    LuTractor,
    #[cfg(any(LuTrafficCone, icondata_include_all))]
    LuTrafficCone,
    #[cfg(any(LuTrainFront, icondata_include_all))]
    LuTrainFront,
    #[cfg(any(LuTrainFrontTunnel, icondata_include_all))]
    LuTrainFrontTunnel,
    #[cfg(any(LuTrainTrack, icondata_include_all))]
    LuTrainTrack,
    #[cfg(any(LuTramFront, icondata_include_all))]
    LuTramFront,
    #[cfg(any(LuTrash, icondata_include_all))]
    LuTrash,
    #[cfg(any(LuTrash2, icondata_include_all))]
    LuTrash2,
    #[cfg(any(LuTreeDeciduous, icondata_include_all))]
    LuTreeDeciduous,
    #[cfg(any(LuTreePine, icondata_include_all))]
    LuTreePine,
    #[cfg(any(LuTrees, icondata_include_all))]
    LuTrees,
    #[cfg(any(LuTrello, icondata_include_all))]
    LuTrello,
    #[cfg(any(LuTrendingDown, icondata_include_all))]
    LuTrendingDown,
    #[cfg(any(LuTrendingUp, icondata_include_all))]
    LuTrendingUp,
    #[cfg(any(LuTriangle, icondata_include_all))]
    LuTriangle,
    #[cfg(any(LuTriangleRight, icondata_include_all))]
    LuTriangleRight,
    #[cfg(any(LuTrophy, icondata_include_all))]
    LuTrophy,
    #[cfg(any(LuTruck, icondata_include_all))]
    LuTruck,
    #[cfg(any(LuTurtle, icondata_include_all))]
    LuTurtle,
    #[cfg(any(LuTv, icondata_include_all))]
    LuTv,
    #[cfg(any(LuTv2, icondata_include_all))]
    LuTv2,
    #[cfg(any(LuTwitch, icondata_include_all))]
    LuTwitch,
    #[cfg(any(LuTwitter, icondata_include_all))]
    LuTwitter,
    #[cfg(any(LuType, icondata_include_all))]
    LuType,
    #[cfg(any(LuUmbrella, icondata_include_all))]
    LuUmbrella,
    #[cfg(any(LuUnderline, icondata_include_all))]
    LuUnderline,
    #[cfg(any(LuUndo, icondata_include_all))]
    LuUndo,
    #[cfg(any(LuUndo2, icondata_include_all))]
    LuUndo2,
    #[cfg(any(LuUndoDot, icondata_include_all))]
    LuUndoDot,
    #[cfg(any(LuUnfoldHorizontal, icondata_include_all))]
    LuUnfoldHorizontal,
    #[cfg(any(LuUnfoldVertical, icondata_include_all))]
    LuUnfoldVertical,
    #[cfg(any(LuUngroup, icondata_include_all))]
    LuUngroup,
    #[cfg(any(LuUnlink, icondata_include_all))]
    LuUnlink,
    #[cfg(any(LuUnlink2, icondata_include_all))]
    LuUnlink2,
    #[cfg(any(LuUnlock, icondata_include_all))]
    LuUnlock,
    #[cfg(any(LuUnplug, icondata_include_all))]
    LuUnplug,
    #[cfg(any(LuUpload, icondata_include_all))]
    LuUpload,
    #[cfg(any(LuUploadCloud, icondata_include_all))]
    LuUploadCloud,
    #[cfg(any(LuUsb, icondata_include_all))]
    LuUsb,
    #[cfg(any(LuUser, icondata_include_all))]
    LuUser,
    #[cfg(any(LuUser2, icondata_include_all))]
    LuUser2,
    #[cfg(any(LuUserCheck, icondata_include_all))]
    LuUserCheck,
    #[cfg(any(LuUserCheck2, icondata_include_all))]
    LuUserCheck2,
    #[cfg(any(LuUserCircle, icondata_include_all))]
    LuUserCircle,
    #[cfg(any(LuUserCircle2, icondata_include_all))]
    LuUserCircle2,
    #[cfg(any(LuUserCog, icondata_include_all))]
    LuUserCog,
    #[cfg(any(LuUserCog2, icondata_include_all))]
    LuUserCog2,
    #[cfg(any(LuUserMinus, icondata_include_all))]
    LuUserMinus,
    #[cfg(any(LuUserMinus2, icondata_include_all))]
    LuUserMinus2,
    #[cfg(any(LuUserPlus, icondata_include_all))]
    LuUserPlus,
    #[cfg(any(LuUserPlus2, icondata_include_all))]
    LuUserPlus2,
    #[cfg(any(LuUserSquare, icondata_include_all))]
    LuUserSquare,
    #[cfg(any(LuUserSquare2, icondata_include_all))]
    LuUserSquare2,
    #[cfg(any(LuUserX, icondata_include_all))]
    LuUserX,
    #[cfg(any(LuUserX2, icondata_include_all))]
    LuUserX2,
    #[cfg(any(LuUsers, icondata_include_all))]
    LuUsers,
    #[cfg(any(LuUsers2, icondata_include_all))]
    LuUsers2,
    #[cfg(any(LuUtensils, icondata_include_all))]
    LuUtensils,
    #[cfg(any(LuUtensilsCrossed, icondata_include_all))]
    LuUtensilsCrossed,
    #[cfg(any(LuUtilityPole, icondata_include_all))]
    LuUtilityPole,
    #[cfg(any(LuVariable, icondata_include_all))]
    LuVariable,
    #[cfg(any(LuVegan, icondata_include_all))]
    LuVegan,
    #[cfg(any(LuVenetianMask, icondata_include_all))]
    LuVenetianMask,
    #[cfg(any(LuVibrate, icondata_include_all))]
    LuVibrate,
    #[cfg(any(LuVibrateOff, icondata_include_all))]
    LuVibrateOff,
    #[cfg(any(LuVideo, icondata_include_all))]
    LuVideo,
    #[cfg(any(LuVideoOff, icondata_include_all))]
    LuVideoOff,
    #[cfg(any(LuVideotape, icondata_include_all))]
    LuVideotape,
    #[cfg(any(LuView, icondata_include_all))]
    LuView,
    #[cfg(any(LuVoicemail, icondata_include_all))]
    LuVoicemail,
    #[cfg(any(LuVolume, icondata_include_all))]
    LuVolume,
    #[cfg(any(LuVolume1, icondata_include_all))]
    LuVolume1,
    #[cfg(any(LuVolume2, icondata_include_all))]
    LuVolume2,
    #[cfg(any(LuVolumeX, icondata_include_all))]
    LuVolumeX,
    #[cfg(any(LuVote, icondata_include_all))]
    LuVote,
    #[cfg(any(LuWallet, icondata_include_all))]
    LuWallet,
    #[cfg(any(LuWallet2, icondata_include_all))]
    LuWallet2,
    #[cfg(any(LuWalletCards, icondata_include_all))]
    LuWalletCards,
    #[cfg(any(LuWallpaper, icondata_include_all))]
    LuWallpaper,
    #[cfg(any(LuWand, icondata_include_all))]
    LuWand,
    #[cfg(any(LuWand2, icondata_include_all))]
    LuWand2,
    #[cfg(any(LuWarehouse, icondata_include_all))]
    LuWarehouse,
    #[cfg(any(LuWatch, icondata_include_all))]
    LuWatch,
    #[cfg(any(LuWaves, icondata_include_all))]
    LuWaves,
    #[cfg(any(LuWebcam, icondata_include_all))]
    LuWebcam,
    #[cfg(any(LuWebhook, icondata_include_all))]
    LuWebhook,
    #[cfg(any(LuWheat, icondata_include_all))]
    LuWheat,
    #[cfg(any(LuWheatOff, icondata_include_all))]
    LuWheatOff,
    #[cfg(any(LuWholeWord, icondata_include_all))]
    LuWholeWord,
    #[cfg(any(LuWifi, icondata_include_all))]
    LuWifi,
    #[cfg(any(LuWifiOff, icondata_include_all))]
    LuWifiOff,
    #[cfg(any(LuWind, icondata_include_all))]
    LuWind,
    #[cfg(any(LuWine, icondata_include_all))]
    LuWine,
    #[cfg(any(LuWineOff, icondata_include_all))]
    LuWineOff,
    #[cfg(any(LuWorkflow, icondata_include_all))]
    LuWorkflow,
    #[cfg(any(LuWrapText, icondata_include_all))]
    LuWrapText,
    #[cfg(any(LuWrench, icondata_include_all))]
    LuWrench,
    #[cfg(any(LuX, icondata_include_all))]
    LuX,
    #[cfg(any(LuXCircle, icondata_include_all))]
    LuXCircle,
    #[cfg(any(LuXOctagon, icondata_include_all))]
    LuXOctagon,
    #[cfg(any(LuXSquare, icondata_include_all))]
    LuXSquare,
    #[cfg(any(LuYoutube, icondata_include_all))]
    LuYoutube,
    #[cfg(any(LuZap, icondata_include_all))]
    LuZap,
    #[cfg(any(LuZapOff, icondata_include_all))]
    LuZapOff,
    #[cfg(any(LuZoomIn, icondata_include_all))]
    LuZoomIn,
    #[cfg(any(LuZoomOut, icondata_include_all))]
    LuZoomOut,
}

#[cfg(any(LuAccessibility, icondata_include_all))]
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
#[cfg(any(LuActivity, icondata_include_all))]
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
#[cfg(any(LuActivitySquare, icondata_include_all))]
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
#[cfg(any(LuAirVent, icondata_include_all))]
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
#[cfg(any(LuAirplay, icondata_include_all))]
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
#[cfg(any(LuAlarmCheck, icondata_include_all))]
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
#[cfg(any(LuAlarmClock, icondata_include_all))]
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
#[cfg(any(LuAlarmClockOff, icondata_include_all))]
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
#[cfg(any(LuAlarmMinus, icondata_include_all))]
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
#[cfg(any(LuAlarmPlus, icondata_include_all))]
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
#[cfg(any(LuAlbum, icondata_include_all))]
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
#[cfg(any(LuAlertCircle, icondata_include_all))]
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
#[cfg(any(LuAlertOctagon, icondata_include_all))]
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
#[cfg(any(LuAlertTriangle, icondata_include_all))]
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
#[cfg(any(LuAlignCenter, icondata_include_all))]
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
#[cfg(any(LuAlignCenterHorizontal, icondata_include_all))]
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
#[cfg(any(LuAlignCenterVertical, icondata_include_all))]
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
#[cfg(any(LuAlignEndHorizontal, icondata_include_all))]
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
#[cfg(any(LuAlignEndVertical, icondata_include_all))]
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
#[cfg(any(LuAlignHorizontalDistributeCenter, icondata_include_all))]
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
#[cfg(any(LuAlignHorizontalDistributeEnd, icondata_include_all))]
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
#[cfg(any(LuAlignHorizontalDistributeStart, icondata_include_all))]
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
#[cfg(any(LuAlignHorizontalJustifyCenter, icondata_include_all))]
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
#[cfg(any(LuAlignHorizontalJustifyEnd, icondata_include_all))]
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
#[cfg(any(LuAlignHorizontalJustifyStart, icondata_include_all))]
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
#[cfg(any(LuAlignHorizontalSpaceAround, icondata_include_all))]
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
#[cfg(any(LuAlignHorizontalSpaceBetween, icondata_include_all))]
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
#[cfg(any(LuAlignJustify, icondata_include_all))]
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
#[cfg(any(LuAlignLeft, icondata_include_all))]
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
#[cfg(any(LuAlignRight, icondata_include_all))]
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
#[cfg(any(LuAlignStartHorizontal, icondata_include_all))]
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
#[cfg(any(LuAlignStartVertical, icondata_include_all))]
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
#[cfg(any(LuAlignVerticalDistributeCenter, icondata_include_all))]
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
#[cfg(any(LuAlignVerticalDistributeEnd, icondata_include_all))]
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
#[cfg(any(LuAlignVerticalDistributeStart, icondata_include_all))]
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
#[cfg(any(LuAlignVerticalJustifyCenter, icondata_include_all))]
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
#[cfg(any(LuAlignVerticalJustifyEnd, icondata_include_all))]
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
#[cfg(any(LuAlignVerticalJustifyStart, icondata_include_all))]
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
#[cfg(any(LuAlignVerticalSpaceAround, icondata_include_all))]
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
#[cfg(any(LuAlignVerticalSpaceBetween, icondata_include_all))]
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
#[cfg(any(LuAmpersand, icondata_include_all))]
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
#[cfg(any(LuAmpersands, icondata_include_all))]
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
#[cfg(any(LuAnchor, icondata_include_all))]
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
#[cfg(any(LuAngry, icondata_include_all))]
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
#[cfg(any(LuAnnoyed, icondata_include_all))]
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
#[cfg(any(LuAntenna, icondata_include_all))]
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
#[cfg(any(LuAperture, icondata_include_all))]
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
#[cfg(any(LuAppWindow, icondata_include_all))]
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
#[cfg(any(LuApple, icondata_include_all))]
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
#[cfg(any(LuArchive, icondata_include_all))]
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
#[cfg(any(LuArchiveRestore, icondata_include_all))]
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
#[cfg(any(LuAreaChart, icondata_include_all))]
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
#[cfg(any(LuArmchair, icondata_include_all))]
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
#[cfg(any(LuArrowBigDown, icondata_include_all))]
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
#[cfg(any(LuArrowBigDownDash, icondata_include_all))]
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
#[cfg(any(LuArrowBigLeft, icondata_include_all))]
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
#[cfg(any(LuArrowBigLeftDash, icondata_include_all))]
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
#[cfg(any(LuArrowBigRight, icondata_include_all))]
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
#[cfg(any(LuArrowBigRightDash, icondata_include_all))]
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
#[cfg(any(LuArrowBigUp, icondata_include_all))]
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
#[cfg(any(LuArrowBigUpDash, icondata_include_all))]
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
#[cfg(any(LuArrowDown, icondata_include_all))]
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
#[cfg(any(LuArrowDown01, icondata_include_all))]
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
#[cfg(any(LuArrowDown10, icondata_include_all))]
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
#[cfg(any(LuArrowDownAZ, icondata_include_all))]
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
#[cfg(any(LuArrowDownCircle, icondata_include_all))]
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
#[cfg(any(LuArrowDownFromLine, icondata_include_all))]
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
#[cfg(any(LuArrowDownLeft, icondata_include_all))]
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
#[cfg(any(LuArrowDownLeftFromCircle, icondata_include_all))]
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
#[cfg(any(LuArrowDownLeftSquare, icondata_include_all))]
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
#[cfg(any(LuArrowDownNarrowWide, icondata_include_all))]
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
#[cfg(any(LuArrowDownRight, icondata_include_all))]
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
#[cfg(any(LuArrowDownRightFromCircle, icondata_include_all))]
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
#[cfg(any(LuArrowDownRightSquare, icondata_include_all))]
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
#[cfg(any(LuArrowDownSquare, icondata_include_all))]
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
#[cfg(any(LuArrowDownToDot, icondata_include_all))]
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
#[cfg(any(LuArrowDownToLine, icondata_include_all))]
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
#[cfg(any(LuArrowDownUp, icondata_include_all))]
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
#[cfg(any(LuArrowDownWideNarrow, icondata_include_all))]
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
#[cfg(any(LuArrowDownZA, icondata_include_all))]
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
#[cfg(any(LuArrowLeft, icondata_include_all))]
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
#[cfg(any(LuArrowLeftCircle, icondata_include_all))]
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
#[cfg(any(LuArrowLeftFromLine, icondata_include_all))]
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
#[cfg(any(LuArrowLeftRight, icondata_include_all))]
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
#[cfg(any(LuArrowLeftSquare, icondata_include_all))]
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
#[cfg(any(LuArrowLeftToLine, icondata_include_all))]
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
#[cfg(any(LuArrowRight, icondata_include_all))]
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
#[cfg(any(LuArrowRightCircle, icondata_include_all))]
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
#[cfg(any(LuArrowRightFromLine, icondata_include_all))]
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
#[cfg(any(LuArrowRightLeft, icondata_include_all))]
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
#[cfg(any(LuArrowRightSquare, icondata_include_all))]
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
#[cfg(any(LuArrowRightToLine, icondata_include_all))]
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
#[cfg(any(LuArrowUp, icondata_include_all))]
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
#[cfg(any(LuArrowUp01, icondata_include_all))]
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
#[cfg(any(LuArrowUp10, icondata_include_all))]
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
#[cfg(any(LuArrowUpAZ, icondata_include_all))]
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
#[cfg(any(LuArrowUpCircle, icondata_include_all))]
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
#[cfg(any(LuArrowUpDown, icondata_include_all))]
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
#[cfg(any(LuArrowUpFromDot, icondata_include_all))]
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
#[cfg(any(LuArrowUpFromLine, icondata_include_all))]
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
#[cfg(any(LuArrowUpLeft, icondata_include_all))]
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
#[cfg(any(LuArrowUpLeftFromCircle, icondata_include_all))]
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
#[cfg(any(LuArrowUpLeftSquare, icondata_include_all))]
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
#[cfg(any(LuArrowUpNarrowWide, icondata_include_all))]
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
#[cfg(any(LuArrowUpRight, icondata_include_all))]
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
#[cfg(any(LuArrowUpRightFromCircle, icondata_include_all))]
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
#[cfg(any(LuArrowUpRightSquare, icondata_include_all))]
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
#[cfg(any(LuArrowUpSquare, icondata_include_all))]
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
#[cfg(any(LuArrowUpToLine, icondata_include_all))]
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
#[cfg(any(LuArrowUpWideNarrow, icondata_include_all))]
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
#[cfg(any(LuArrowUpZA, icondata_include_all))]
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
#[cfg(any(LuArrowsUpFromLine, icondata_include_all))]
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
#[cfg(any(LuAsterisk, icondata_include_all))]
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
#[cfg(any(LuAtSign, icondata_include_all))]
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
#[cfg(any(LuAtom, icondata_include_all))]
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
#[cfg(any(LuAward, icondata_include_all))]
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
#[cfg(any(LuAxe, icondata_include_all))]
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
#[cfg(any(LuAxis3d, icondata_include_all))]
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
#[cfg(any(LuBaby, icondata_include_all))]
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
#[cfg(any(LuBackpack, icondata_include_all))]
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
#[cfg(any(LuBadge, icondata_include_all))]
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
#[cfg(any(LuBadgeAlert, icondata_include_all))]
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
#[cfg(any(LuBadgeCheck, icondata_include_all))]
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
#[cfg(any(LuBadgeDollarSign, icondata_include_all))]
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
#[cfg(any(LuBadgeHelp, icondata_include_all))]
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
#[cfg(any(LuBadgeInfo, icondata_include_all))]
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
#[cfg(any(LuBadgeMinus, icondata_include_all))]
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
#[cfg(any(LuBadgePercent, icondata_include_all))]
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
#[cfg(any(LuBadgePlus, icondata_include_all))]
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
#[cfg(any(LuBadgeX, icondata_include_all))]
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
#[cfg(any(LuBaggageClaim, icondata_include_all))]
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
#[cfg(any(LuBan, icondata_include_all))]
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
#[cfg(any(LuBanana, icondata_include_all))]
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
#[cfg(any(LuBanknote, icondata_include_all))]
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
#[cfg(any(LuBarChart, icondata_include_all))]
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
#[cfg(any(LuBarChart2, icondata_include_all))]
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
#[cfg(any(LuBarChart3, icondata_include_all))]
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
#[cfg(any(LuBarChart4, icondata_include_all))]
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
#[cfg(any(LuBarChartBig, icondata_include_all))]
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
#[cfg(any(LuBarChartHorizontal, icondata_include_all))]
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
#[cfg(any(LuBarChartHorizontalBig, icondata_include_all))]
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
#[cfg(any(LuBaseline, icondata_include_all))]
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
#[cfg(any(LuBath, icondata_include_all))]
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
#[cfg(any(LuBattery, icondata_include_all))]
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
#[cfg(any(LuBatteryCharging, icondata_include_all))]
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
#[cfg(any(LuBatteryFull, icondata_include_all))]
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
#[cfg(any(LuBatteryLow, icondata_include_all))]
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
#[cfg(any(LuBatteryMedium, icondata_include_all))]
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
#[cfg(any(LuBatteryWarning, icondata_include_all))]
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
#[cfg(any(LuBeaker, icondata_include_all))]
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
#[cfg(any(LuBean, icondata_include_all))]
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
#[cfg(any(LuBeanOff, icondata_include_all))]
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
#[cfg(any(LuBed, icondata_include_all))]
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
#[cfg(any(LuBedDouble, icondata_include_all))]
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
#[cfg(any(LuBedSingle, icondata_include_all))]
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
#[cfg(any(LuBeef, icondata_include_all))]
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
#[cfg(any(LuBeer, icondata_include_all))]
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
#[cfg(any(LuBell, icondata_include_all))]
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
#[cfg(any(LuBellDot, icondata_include_all))]
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
#[cfg(any(LuBellMinus, icondata_include_all))]
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
#[cfg(any(LuBellOff, icondata_include_all))]
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
#[cfg(any(LuBellPlus, icondata_include_all))]
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
#[cfg(any(LuBellRing, icondata_include_all))]
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
#[cfg(any(LuBike, icondata_include_all))]
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
#[cfg(any(LuBinary, icondata_include_all))]
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
#[cfg(any(LuBiohazard, icondata_include_all))]
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
#[cfg(any(LuBird, icondata_include_all))]
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
#[cfg(any(LuBitcoin, icondata_include_all))]
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
#[cfg(any(LuBlinds, icondata_include_all))]
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
#[cfg(any(LuBlocks, icondata_include_all))]
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
#[cfg(any(LuBluetooth, icondata_include_all))]
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
#[cfg(any(LuBluetoothConnected, icondata_include_all))]
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
#[cfg(any(LuBluetoothOff, icondata_include_all))]
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
#[cfg(any(LuBluetoothSearching, icondata_include_all))]
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
#[cfg(any(LuBold, icondata_include_all))]
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
#[cfg(any(LuBomb, icondata_include_all))]
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
#[cfg(any(LuBone, icondata_include_all))]
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
#[cfg(any(LuBook, icondata_include_all))]
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
#[cfg(any(LuBookCopy, icondata_include_all))]
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
#[cfg(any(LuBookDown, icondata_include_all))]
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
#[cfg(any(LuBookKey, icondata_include_all))]
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
#[cfg(any(LuBookLock, icondata_include_all))]
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
#[cfg(any(LuBookMarked, icondata_include_all))]
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
#[cfg(any(LuBookMinus, icondata_include_all))]
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
#[cfg(any(LuBookOpen, icondata_include_all))]
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
#[cfg(any(LuBookOpenCheck, icondata_include_all))]
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
#[cfg(any(LuBookPlus, icondata_include_all))]
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
#[cfg(any(LuBookTemplate, icondata_include_all))]
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
#[cfg(any(LuBookUp, icondata_include_all))]
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
#[cfg(any(LuBookUp2, icondata_include_all))]
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
#[cfg(any(LuBookX, icondata_include_all))]
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
#[cfg(any(LuBookmark, icondata_include_all))]
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
#[cfg(any(LuBookmarkMinus, icondata_include_all))]
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
#[cfg(any(LuBookmarkPlus, icondata_include_all))]
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
#[cfg(any(LuBoomBox, icondata_include_all))]
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
#[cfg(any(LuBot, icondata_include_all))]
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
#[cfg(any(LuBox, icondata_include_all))]
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
#[cfg(any(LuBoxSelect, icondata_include_all))]
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
#[cfg(any(LuBoxes, icondata_include_all))]
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
#[cfg(any(LuBraces, icondata_include_all))]
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
#[cfg(any(LuBrackets, icondata_include_all))]
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
#[cfg(any(LuBrain, icondata_include_all))]
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
#[cfg(any(LuBrainCircuit, icondata_include_all))]
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
#[cfg(any(LuBrainCog, icondata_include_all))]
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
#[cfg(any(LuBriefcase, icondata_include_all))]
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
#[cfg(any(LuBringToFront, icondata_include_all))]
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
#[cfg(any(LuBrush, icondata_include_all))]
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
#[cfg(any(LuBug, icondata_include_all))]
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
#[cfg(any(LuBuilding, icondata_include_all))]
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
#[cfg(any(LuBuilding2, icondata_include_all))]
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
#[cfg(any(LuBus, icondata_include_all))]
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
#[cfg(any(LuBusFront, icondata_include_all))]
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
#[cfg(any(LuCable, icondata_include_all))]
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
#[cfg(any(LuCableCar, icondata_include_all))]
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
#[cfg(any(LuCake, icondata_include_all))]
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
#[cfg(any(LuCakeSlice, icondata_include_all))]
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
#[cfg(any(LuCalculator, icondata_include_all))]
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
#[cfg(any(LuCalendar, icondata_include_all))]
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
#[cfg(any(LuCalendarCheck, icondata_include_all))]
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
#[cfg(any(LuCalendarCheck2, icondata_include_all))]
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
#[cfg(any(LuCalendarClock, icondata_include_all))]
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
#[cfg(any(LuCalendarDays, icondata_include_all))]
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
#[cfg(any(LuCalendarHeart, icondata_include_all))]
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
#[cfg(any(LuCalendarMinus, icondata_include_all))]
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
#[cfg(any(LuCalendarOff, icondata_include_all))]
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
#[cfg(any(LuCalendarPlus, icondata_include_all))]
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
#[cfg(any(LuCalendarRange, icondata_include_all))]
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
#[cfg(any(LuCalendarSearch, icondata_include_all))]
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
#[cfg(any(LuCalendarX, icondata_include_all))]
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
#[cfg(any(LuCalendarX2, icondata_include_all))]
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
#[cfg(any(LuCamera, icondata_include_all))]
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
#[cfg(any(LuCameraOff, icondata_include_all))]
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
#[cfg(any(LuCandlestickChart, icondata_include_all))]
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
#[cfg(any(LuCandy, icondata_include_all))]
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
#[cfg(any(LuCandyCane, icondata_include_all))]
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
#[cfg(any(LuCandyOff, icondata_include_all))]
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
#[cfg(any(LuCar, icondata_include_all))]
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
#[cfg(any(LuCarFront, icondata_include_all))]
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
#[cfg(any(LuCarTaxiFront, icondata_include_all))]
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
#[cfg(any(LuCarrot, icondata_include_all))]
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
#[cfg(any(LuCaseLower, icondata_include_all))]
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
#[cfg(any(LuCaseSensitive, icondata_include_all))]
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
#[cfg(any(LuCaseUpper, icondata_include_all))]
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
#[cfg(any(LuCassetteTape, icondata_include_all))]
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
#[cfg(any(LuCast, icondata_include_all))]
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
#[cfg(any(LuCastle, icondata_include_all))]
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
#[cfg(any(LuCat, icondata_include_all))]
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
#[cfg(any(LuCheck, icondata_include_all))]
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
#[cfg(any(LuCheckCheck, icondata_include_all))]
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
#[cfg(any(LuCheckCircle, icondata_include_all))]
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
#[cfg(any(LuCheckCircle2, icondata_include_all))]
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
#[cfg(any(LuCheckSquare, icondata_include_all))]
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
#[cfg(any(LuChefHat, icondata_include_all))]
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
#[cfg(any(LuCherry, icondata_include_all))]
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
#[cfg(any(LuChevronDown, icondata_include_all))]
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
#[cfg(any(LuChevronDownCircle, icondata_include_all))]
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
#[cfg(any(LuChevronDownSquare, icondata_include_all))]
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
#[cfg(any(LuChevronFirst, icondata_include_all))]
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
#[cfg(any(LuChevronLast, icondata_include_all))]
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
#[cfg(any(LuChevronLeft, icondata_include_all))]
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
#[cfg(any(LuChevronLeftCircle, icondata_include_all))]
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
#[cfg(any(LuChevronLeftSquare, icondata_include_all))]
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
#[cfg(any(LuChevronRight, icondata_include_all))]
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
#[cfg(any(LuChevronRightCircle, icondata_include_all))]
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
#[cfg(any(LuChevronRightSquare, icondata_include_all))]
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
#[cfg(any(LuChevronUp, icondata_include_all))]
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
#[cfg(any(LuChevronUpCircle, icondata_include_all))]
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
#[cfg(any(LuChevronUpSquare, icondata_include_all))]
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
#[cfg(any(LuChevronsDown, icondata_include_all))]
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
#[cfg(any(LuChevronsDownUp, icondata_include_all))]
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
#[cfg(any(LuChevronsLeft, icondata_include_all))]
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
#[cfg(any(LuChevronsLeftRight, icondata_include_all))]
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
#[cfg(any(LuChevronsRight, icondata_include_all))]
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
#[cfg(any(LuChevronsRightLeft, icondata_include_all))]
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
#[cfg(any(LuChevronsUp, icondata_include_all))]
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
#[cfg(any(LuChevronsUpDown, icondata_include_all))]
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
#[cfg(any(LuChrome, icondata_include_all))]
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
#[cfg(any(LuChurch, icondata_include_all))]
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
#[cfg(any(LuCigarette, icondata_include_all))]
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
#[cfg(any(LuCigaretteOff, icondata_include_all))]
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
#[cfg(any(LuCircle, icondata_include_all))]
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
#[cfg(any(LuCircleDashed, icondata_include_all))]
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
#[cfg(any(LuCircleDollarSign, icondata_include_all))]
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
#[cfg(any(LuCircleDot, icondata_include_all))]
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
#[cfg(any(LuCircleDotDashed, icondata_include_all))]
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
#[cfg(any(LuCircleEllipsis, icondata_include_all))]
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
#[cfg(any(LuCircleEqual, icondata_include_all))]
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
#[cfg(any(LuCircleOff, icondata_include_all))]
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
#[cfg(any(LuCircleSlash, icondata_include_all))]
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
#[cfg(any(LuCircleSlash2, icondata_include_all))]
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
#[cfg(any(LuCircuitBoard, icondata_include_all))]
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
#[cfg(any(LuCitrus, icondata_include_all))]
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
#[cfg(any(LuClapperboard, icondata_include_all))]
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
#[cfg(any(LuClipboard, icondata_include_all))]
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
#[cfg(any(LuClipboardCheck, icondata_include_all))]
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
#[cfg(any(LuClipboardCopy, icondata_include_all))]
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
#[cfg(any(LuClipboardEdit, icondata_include_all))]
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
#[cfg(any(LuClipboardList, icondata_include_all))]
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
#[cfg(any(LuClipboardPaste, icondata_include_all))]
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
#[cfg(any(LuClipboardSignature, icondata_include_all))]
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
#[cfg(any(LuClipboardType, icondata_include_all))]
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
#[cfg(any(LuClipboardX, icondata_include_all))]
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
#[cfg(any(LuClock, icondata_include_all))]
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
#[cfg(any(LuClock1, icondata_include_all))]
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
#[cfg(any(LuClock10, icondata_include_all))]
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
#[cfg(any(LuClock11, icondata_include_all))]
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
#[cfg(any(LuClock12, icondata_include_all))]
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
#[cfg(any(LuClock2, icondata_include_all))]
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
#[cfg(any(LuClock3, icondata_include_all))]
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
#[cfg(any(LuClock4, icondata_include_all))]
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
#[cfg(any(LuClock5, icondata_include_all))]
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
#[cfg(any(LuClock6, icondata_include_all))]
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
#[cfg(any(LuClock7, icondata_include_all))]
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
#[cfg(any(LuClock8, icondata_include_all))]
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
#[cfg(any(LuClock9, icondata_include_all))]
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
#[cfg(any(LuCloud, icondata_include_all))]
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
#[cfg(any(LuCloudCog, icondata_include_all))]
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
#[cfg(any(LuCloudDrizzle, icondata_include_all))]
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
#[cfg(any(LuCloudFog, icondata_include_all))]
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
#[cfg(any(LuCloudHail, icondata_include_all))]
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
#[cfg(any(LuCloudLightning, icondata_include_all))]
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
#[cfg(any(LuCloudMoon, icondata_include_all))]
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
#[cfg(any(LuCloudMoonRain, icondata_include_all))]
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
#[cfg(any(LuCloudOff, icondata_include_all))]
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
#[cfg(any(LuCloudRain, icondata_include_all))]
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
#[cfg(any(LuCloudRainWind, icondata_include_all))]
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
#[cfg(any(LuCloudSnow, icondata_include_all))]
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
#[cfg(any(LuCloudSun, icondata_include_all))]
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
#[cfg(any(LuCloudSunRain, icondata_include_all))]
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
#[cfg(any(LuCloudy, icondata_include_all))]
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
#[cfg(any(LuClover, icondata_include_all))]
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
#[cfg(any(LuClub, icondata_include_all))]
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
#[cfg(any(LuCode, icondata_include_all))]
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
#[cfg(any(LuCode2, icondata_include_all))]
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
#[cfg(any(LuCodepen, icondata_include_all))]
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
#[cfg(any(LuCodesandbox, icondata_include_all))]
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
#[cfg(any(LuCoffee, icondata_include_all))]
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
#[cfg(any(LuCog, icondata_include_all))]
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
#[cfg(any(LuCoins, icondata_include_all))]
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
#[cfg(any(LuColumns, icondata_include_all))]
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
#[cfg(any(LuCombine, icondata_include_all))]
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
#[cfg(any(LuCommand, icondata_include_all))]
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
#[cfg(any(LuCompass, icondata_include_all))]
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
#[cfg(any(LuComponent, icondata_include_all))]
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
#[cfg(any(LuComputer, icondata_include_all))]
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
#[cfg(any(LuConciergeBell, icondata_include_all))]
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
#[cfg(any(LuConstruction, icondata_include_all))]
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
#[cfg(any(LuContact, icondata_include_all))]
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
#[cfg(any(LuContact2, icondata_include_all))]
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
#[cfg(any(LuContainer, icondata_include_all))]
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
#[cfg(any(LuContrast, icondata_include_all))]
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
#[cfg(any(LuCookie, icondata_include_all))]
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
#[cfg(any(LuCopy, icondata_include_all))]
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
#[cfg(any(LuCopyCheck, icondata_include_all))]
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
#[cfg(any(LuCopyMinus, icondata_include_all))]
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
#[cfg(any(LuCopyPlus, icondata_include_all))]
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
#[cfg(any(LuCopySlash, icondata_include_all))]
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
#[cfg(any(LuCopyX, icondata_include_all))]
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
#[cfg(any(LuCopyleft, icondata_include_all))]
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
#[cfg(any(LuCopyright, icondata_include_all))]
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
#[cfg(any(LuCornerDownLeft, icondata_include_all))]
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
#[cfg(any(LuCornerDownRight, icondata_include_all))]
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
#[cfg(any(LuCornerLeftDown, icondata_include_all))]
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
#[cfg(any(LuCornerLeftUp, icondata_include_all))]
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
#[cfg(any(LuCornerRightDown, icondata_include_all))]
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
#[cfg(any(LuCornerRightUp, icondata_include_all))]
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
#[cfg(any(LuCornerUpLeft, icondata_include_all))]
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
#[cfg(any(LuCornerUpRight, icondata_include_all))]
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
#[cfg(any(LuCpu, icondata_include_all))]
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
#[cfg(any(LuCreativeCommons, icondata_include_all))]
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
#[cfg(any(LuCreditCard, icondata_include_all))]
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
#[cfg(any(LuCroissant, icondata_include_all))]
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
#[cfg(any(LuCrop, icondata_include_all))]
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
#[cfg(any(LuCross, icondata_include_all))]
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
#[cfg(any(LuCrosshair, icondata_include_all))]
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
#[cfg(any(LuCrown, icondata_include_all))]
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
#[cfg(any(LuCupSoda, icondata_include_all))]
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
#[cfg(any(LuCurrency, icondata_include_all))]
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
#[cfg(any(LuDatabase, icondata_include_all))]
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
#[cfg(any(LuDatabaseBackup, icondata_include_all))]
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
#[cfg(any(LuDelete, icondata_include_all))]
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
#[cfg(any(LuDessert, icondata_include_all))]
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
#[cfg(any(LuDiamond, icondata_include_all))]
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
#[cfg(any(LuDice1, icondata_include_all))]
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
#[cfg(any(LuDice2, icondata_include_all))]
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
#[cfg(any(LuDice3, icondata_include_all))]
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
#[cfg(any(LuDice4, icondata_include_all))]
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
#[cfg(any(LuDice5, icondata_include_all))]
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
#[cfg(any(LuDice6, icondata_include_all))]
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
#[cfg(any(LuDices, icondata_include_all))]
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
#[cfg(any(LuDiff, icondata_include_all))]
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
#[cfg(any(LuDisc, icondata_include_all))]
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
#[cfg(any(LuDisc2, icondata_include_all))]
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
#[cfg(any(LuDisc3, icondata_include_all))]
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
#[cfg(any(LuDivide, icondata_include_all))]
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
#[cfg(any(LuDivideCircle, icondata_include_all))]
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
#[cfg(any(LuDivideSquare, icondata_include_all))]
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
#[cfg(any(LuDna, icondata_include_all))]
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
#[cfg(any(LuDnaOff, icondata_include_all))]
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
#[cfg(any(LuDog, icondata_include_all))]
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
#[cfg(any(LuDollarSign, icondata_include_all))]
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
#[cfg(any(LuDonut, icondata_include_all))]
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
#[cfg(any(LuDoorClosed, icondata_include_all))]
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
#[cfg(any(LuDoorOpen, icondata_include_all))]
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
#[cfg(any(LuDot, icondata_include_all))]
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
#[cfg(any(LuDownload, icondata_include_all))]
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
#[cfg(any(LuDownloadCloud, icondata_include_all))]
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
#[cfg(any(LuDribbble, icondata_include_all))]
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
#[cfg(any(LuDroplet, icondata_include_all))]
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
#[cfg(any(LuDroplets, icondata_include_all))]
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
#[cfg(any(LuDrumstick, icondata_include_all))]
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
#[cfg(any(LuDumbbell, icondata_include_all))]
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
#[cfg(any(LuEar, icondata_include_all))]
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
#[cfg(any(LuEarOff, icondata_include_all))]
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
#[cfg(any(LuEgg, icondata_include_all))]
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
#[cfg(any(LuEggFried, icondata_include_all))]
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
#[cfg(any(LuEggOff, icondata_include_all))]
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
#[cfg(any(LuEqual, icondata_include_all))]
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
#[cfg(any(LuEqualNot, icondata_include_all))]
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
#[cfg(any(LuEraser, icondata_include_all))]
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
#[cfg(any(LuEuro, icondata_include_all))]
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
#[cfg(any(LuExpand, icondata_include_all))]
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
#[cfg(any(LuExternalLink, icondata_include_all))]
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
#[cfg(any(LuEye, icondata_include_all))]
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
#[cfg(any(LuEyeOff, icondata_include_all))]
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
#[cfg(any(LuFacebook, icondata_include_all))]
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
#[cfg(any(LuFactory, icondata_include_all))]
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
#[cfg(any(LuFan, icondata_include_all))]
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
#[cfg(any(LuFastForward, icondata_include_all))]
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
#[cfg(any(LuFeather, icondata_include_all))]
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
#[cfg(any(LuFerrisWheel, icondata_include_all))]
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
#[cfg(any(LuFigma, icondata_include_all))]
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
#[cfg(any(LuFile, icondata_include_all))]
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
#[cfg(any(LuFileArchive, icondata_include_all))]
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
#[cfg(any(LuFileAudio, icondata_include_all))]
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
#[cfg(any(LuFileAudio2, icondata_include_all))]
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
#[cfg(any(LuFileAxis3d, icondata_include_all))]
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
#[cfg(any(LuFileBadge, icondata_include_all))]
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
#[cfg(any(LuFileBadge2, icondata_include_all))]
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
#[cfg(any(LuFileBarChart, icondata_include_all))]
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
#[cfg(any(LuFileBarChart2, icondata_include_all))]
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
#[cfg(any(LuFileBox, icondata_include_all))]
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
#[cfg(any(LuFileCheck, icondata_include_all))]
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
#[cfg(any(LuFileCheck2, icondata_include_all))]
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
#[cfg(any(LuFileClock, icondata_include_all))]
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
#[cfg(any(LuFileCode, icondata_include_all))]
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
#[cfg(any(LuFileCode2, icondata_include_all))]
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
#[cfg(any(LuFileCog, icondata_include_all))]
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
#[cfg(any(LuFileCog2, icondata_include_all))]
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
#[cfg(any(LuFileDiff, icondata_include_all))]
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
#[cfg(any(LuFileDigit, icondata_include_all))]
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
#[cfg(any(LuFileDown, icondata_include_all))]
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
#[cfg(any(LuFileEdit, icondata_include_all))]
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
#[cfg(any(LuFileHeart, icondata_include_all))]
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
#[cfg(any(LuFileImage, icondata_include_all))]
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
#[cfg(any(LuFileInput, icondata_include_all))]
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
#[cfg(any(LuFileJson, icondata_include_all))]
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
#[cfg(any(LuFileJson2, icondata_include_all))]
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
#[cfg(any(LuFileKey, icondata_include_all))]
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
#[cfg(any(LuFileKey2, icondata_include_all))]
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
#[cfg(any(LuFileLineChart, icondata_include_all))]
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
#[cfg(any(LuFileLock, icondata_include_all))]
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
#[cfg(any(LuFileLock2, icondata_include_all))]
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
#[cfg(any(LuFileMinus, icondata_include_all))]
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
#[cfg(any(LuFileMinus2, icondata_include_all))]
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
#[cfg(any(LuFileOutput, icondata_include_all))]
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
#[cfg(any(LuFilePieChart, icondata_include_all))]
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
#[cfg(any(LuFilePlus, icondata_include_all))]
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
#[cfg(any(LuFilePlus2, icondata_include_all))]
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
#[cfg(any(LuFileQuestion, icondata_include_all))]
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
#[cfg(any(LuFileScan, icondata_include_all))]
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
#[cfg(any(LuFileSearch, icondata_include_all))]
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
#[cfg(any(LuFileSearch2, icondata_include_all))]
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
#[cfg(any(LuFileSignature, icondata_include_all))]
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
#[cfg(any(LuFileSpreadsheet, icondata_include_all))]
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
#[cfg(any(LuFileStack, icondata_include_all))]
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
#[cfg(any(LuFileSymlink, icondata_include_all))]
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
#[cfg(any(LuFileTerminal, icondata_include_all))]
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
#[cfg(any(LuFileText, icondata_include_all))]
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
#[cfg(any(LuFileType, icondata_include_all))]
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
#[cfg(any(LuFileType2, icondata_include_all))]
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
#[cfg(any(LuFileUp, icondata_include_all))]
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
#[cfg(any(LuFileVideo, icondata_include_all))]
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
#[cfg(any(LuFileVideo2, icondata_include_all))]
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
#[cfg(any(LuFileVolume, icondata_include_all))]
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
#[cfg(any(LuFileVolume2, icondata_include_all))]
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
#[cfg(any(LuFileWarning, icondata_include_all))]
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
#[cfg(any(LuFileX, icondata_include_all))]
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
#[cfg(any(LuFileX2, icondata_include_all))]
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
#[cfg(any(LuFiles, icondata_include_all))]
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
#[cfg(any(LuFilm, icondata_include_all))]
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
#[cfg(any(LuFilter, icondata_include_all))]
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
#[cfg(any(LuFilterX, icondata_include_all))]
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
#[cfg(any(LuFingerprint, icondata_include_all))]
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
#[cfg(any(LuFish, icondata_include_all))]
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
#[cfg(any(LuFishOff, icondata_include_all))]
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
#[cfg(any(LuFishSymbol, icondata_include_all))]
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
#[cfg(any(LuFlag, icondata_include_all))]
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
#[cfg(any(LuFlagOff, icondata_include_all))]
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
#[cfg(any(LuFlagTriangleLeft, icondata_include_all))]
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
#[cfg(any(LuFlagTriangleRight, icondata_include_all))]
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
#[cfg(any(LuFlame, icondata_include_all))]
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
#[cfg(any(LuFlashlight, icondata_include_all))]
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
#[cfg(any(LuFlashlightOff, icondata_include_all))]
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
#[cfg(any(LuFlaskConical, icondata_include_all))]
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
#[cfg(any(LuFlaskConicalOff, icondata_include_all))]
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
#[cfg(any(LuFlaskRound, icondata_include_all))]
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
#[cfg(any(LuFlipHorizontal, icondata_include_all))]
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
#[cfg(any(LuFlipHorizontal2, icondata_include_all))]
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
#[cfg(any(LuFlipVertical, icondata_include_all))]
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
#[cfg(any(LuFlipVertical2, icondata_include_all))]
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
#[cfg(any(LuFlower, icondata_include_all))]
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
#[cfg(any(LuFlower2, icondata_include_all))]
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
#[cfg(any(LuFocus, icondata_include_all))]
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
#[cfg(any(LuFoldHorizontal, icondata_include_all))]
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
#[cfg(any(LuFoldVertical, icondata_include_all))]
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
#[cfg(any(LuFolder, icondata_include_all))]
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
#[cfg(any(LuFolderArchive, icondata_include_all))]
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
#[cfg(any(LuFolderCheck, icondata_include_all))]
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
#[cfg(any(LuFolderClock, icondata_include_all))]
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
#[cfg(any(LuFolderClosed, icondata_include_all))]
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
#[cfg(any(LuFolderCog, icondata_include_all))]
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
#[cfg(any(LuFolderCog2, icondata_include_all))]
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
#[cfg(any(LuFolderDot, icondata_include_all))]
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
#[cfg(any(LuFolderDown, icondata_include_all))]
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
#[cfg(any(LuFolderEdit, icondata_include_all))]
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
#[cfg(any(LuFolderGit, icondata_include_all))]
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
#[cfg(any(LuFolderGit2, icondata_include_all))]
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
#[cfg(any(LuFolderHeart, icondata_include_all))]
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
#[cfg(any(LuFolderInput, icondata_include_all))]
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
#[cfg(any(LuFolderKanban, icondata_include_all))]
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
#[cfg(any(LuFolderKey, icondata_include_all))]
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
#[cfg(any(LuFolderLock, icondata_include_all))]
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
#[cfg(any(LuFolderMinus, icondata_include_all))]
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
#[cfg(any(LuFolderOpen, icondata_include_all))]
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
#[cfg(any(LuFolderOpenDot, icondata_include_all))]
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
#[cfg(any(LuFolderOutput, icondata_include_all))]
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
#[cfg(any(LuFolderPlus, icondata_include_all))]
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
#[cfg(any(LuFolderRoot, icondata_include_all))]
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
#[cfg(any(LuFolderSearch, icondata_include_all))]
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
#[cfg(any(LuFolderSearch2, icondata_include_all))]
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
#[cfg(any(LuFolderSymlink, icondata_include_all))]
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
#[cfg(any(LuFolderSync, icondata_include_all))]
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
#[cfg(any(LuFolderTree, icondata_include_all))]
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
#[cfg(any(LuFolderUp, icondata_include_all))]
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
#[cfg(any(LuFolderX, icondata_include_all))]
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
#[cfg(any(LuFolders, icondata_include_all))]
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
#[cfg(any(LuFootprints, icondata_include_all))]
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
#[cfg(any(LuForklift, icondata_include_all))]
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
#[cfg(any(LuFormInput, icondata_include_all))]
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
#[cfg(any(LuForward, icondata_include_all))]
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
#[cfg(any(LuFrame, icondata_include_all))]
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
#[cfg(any(LuFramer, icondata_include_all))]
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
#[cfg(any(LuFrown, icondata_include_all))]
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
#[cfg(any(LuFuel, icondata_include_all))]
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
#[cfg(any(LuFunctionSquare, icondata_include_all))]
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
#[cfg(any(LuGalleryHorizontal, icondata_include_all))]
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
#[cfg(any(LuGalleryHorizontalEnd, icondata_include_all))]
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
#[cfg(any(LuGalleryThumbnails, icondata_include_all))]
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
#[cfg(any(LuGalleryVertical, icondata_include_all))]
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
#[cfg(any(LuGalleryVerticalEnd, icondata_include_all))]
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
#[cfg(any(LuGamepad, icondata_include_all))]
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
#[cfg(any(LuGamepad2, icondata_include_all))]
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
#[cfg(any(LuGanttChart, icondata_include_all))]
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
#[cfg(any(LuGanttChartSquare, icondata_include_all))]
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
#[cfg(any(LuGauge, icondata_include_all))]
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
#[cfg(any(LuGaugeCircle, icondata_include_all))]
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
#[cfg(any(LuGavel, icondata_include_all))]
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
#[cfg(any(LuGem, icondata_include_all))]
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
#[cfg(any(LuGhost, icondata_include_all))]
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
#[cfg(any(LuGift, icondata_include_all))]
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
#[cfg(any(LuGitBranch, icondata_include_all))]
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
#[cfg(any(LuGitBranchPlus, icondata_include_all))]
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
#[cfg(any(LuGitCommit, icondata_include_all))]
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
#[cfg(any(LuGitCompare, icondata_include_all))]
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
#[cfg(any(LuGitFork, icondata_include_all))]
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
#[cfg(any(LuGitMerge, icondata_include_all))]
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
#[cfg(any(LuGitPullRequest, icondata_include_all))]
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
#[cfg(any(LuGitPullRequestClosed, icondata_include_all))]
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
#[cfg(any(LuGitPullRequestDraft, icondata_include_all))]
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
#[cfg(any(LuGithub, icondata_include_all))]
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
#[cfg(any(LuGitlab, icondata_include_all))]
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
#[cfg(any(LuGlassWater, icondata_include_all))]
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
#[cfg(any(LuGlasses, icondata_include_all))]
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
#[cfg(any(LuGlobe, icondata_include_all))]
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
#[cfg(any(LuGlobe2, icondata_include_all))]
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
#[cfg(any(LuGoal, icondata_include_all))]
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
#[cfg(any(LuGrab, icondata_include_all))]
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
#[cfg(any(LuGraduationCap, icondata_include_all))]
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
#[cfg(any(LuGrape, icondata_include_all))]
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
#[cfg(any(LuGrid2x2, icondata_include_all))]
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
#[cfg(any(LuGrid3x3, icondata_include_all))]
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
#[cfg(any(LuGrip, icondata_include_all))]
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
#[cfg(any(LuGripHorizontal, icondata_include_all))]
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
#[cfg(any(LuGripVertical, icondata_include_all))]
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
#[cfg(any(LuGroup, icondata_include_all))]
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
#[cfg(any(LuHammer, icondata_include_all))]
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
#[cfg(any(LuHand, icondata_include_all))]
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
#[cfg(any(LuHandMetal, icondata_include_all))]
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
#[cfg(any(LuHardDrive, icondata_include_all))]
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
#[cfg(any(LuHardDriveDownload, icondata_include_all))]
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
#[cfg(any(LuHardDriveUpload, icondata_include_all))]
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
#[cfg(any(LuHardHat, icondata_include_all))]
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
#[cfg(any(LuHash, icondata_include_all))]
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
#[cfg(any(LuHaze, icondata_include_all))]
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
#[cfg(any(LuHdmiPort, icondata_include_all))]
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
#[cfg(any(LuHeading, icondata_include_all))]
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
#[cfg(any(LuHeading1, icondata_include_all))]
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
#[cfg(any(LuHeading2, icondata_include_all))]
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
#[cfg(any(LuHeading3, icondata_include_all))]
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
#[cfg(any(LuHeading4, icondata_include_all))]
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
#[cfg(any(LuHeading5, icondata_include_all))]
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
#[cfg(any(LuHeading6, icondata_include_all))]
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
#[cfg(any(LuHeadphones, icondata_include_all))]
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
#[cfg(any(LuHeart, icondata_include_all))]
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
#[cfg(any(LuHeartCrack, icondata_include_all))]
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
#[cfg(any(LuHeartHandshake, icondata_include_all))]
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
#[cfg(any(LuHeartOff, icondata_include_all))]
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
#[cfg(any(LuHeartPulse, icondata_include_all))]
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
#[cfg(any(LuHelpCircle, icondata_include_all))]
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
#[cfg(any(LuHelpingHand, icondata_include_all))]
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
#[cfg(any(LuHexagon, icondata_include_all))]
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
#[cfg(any(LuHighlighter, icondata_include_all))]
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
#[cfg(any(LuHistory, icondata_include_all))]
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
#[cfg(any(LuHome, icondata_include_all))]
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
#[cfg(any(LuHop, icondata_include_all))]
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
#[cfg(any(LuHopOff, icondata_include_all))]
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
#[cfg(any(LuHotel, icondata_include_all))]
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
#[cfg(any(LuHourglass, icondata_include_all))]
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
#[cfg(any(LuIceCream, icondata_include_all))]
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
#[cfg(any(LuIceCream2, icondata_include_all))]
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
#[cfg(any(LuImage, icondata_include_all))]
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
#[cfg(any(LuImageMinus, icondata_include_all))]
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
#[cfg(any(LuImageOff, icondata_include_all))]
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
#[cfg(any(LuImagePlus, icondata_include_all))]
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
#[cfg(any(LuImport, icondata_include_all))]
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
#[cfg(any(LuInbox, icondata_include_all))]
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
#[cfg(any(LuIndent, icondata_include_all))]
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
#[cfg(any(LuIndianRupee, icondata_include_all))]
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
#[cfg(any(LuInfinity, icondata_include_all))]
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
#[cfg(any(LuInfo, icondata_include_all))]
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
#[cfg(any(LuInspect, icondata_include_all))]
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
#[cfg(any(LuInstagram, icondata_include_all))]
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
#[cfg(any(LuItalic, icondata_include_all))]
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
#[cfg(any(LuIterationCcw, icondata_include_all))]
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
#[cfg(any(LuIterationCw, icondata_include_all))]
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
#[cfg(any(LuJapaneseYen, icondata_include_all))]
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
#[cfg(any(LuJoystick, icondata_include_all))]
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
#[cfg(any(LuKanban, icondata_include_all))]
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
#[cfg(any(LuKanbanSquare, icondata_include_all))]
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
#[cfg(any(LuKanbanSquareDashed, icondata_include_all))]
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
#[cfg(any(LuKey, icondata_include_all))]
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
#[cfg(any(LuKeyRound, icondata_include_all))]
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
#[cfg(any(LuKeySquare, icondata_include_all))]
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
#[cfg(any(LuKeyboard, icondata_include_all))]
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
#[cfg(any(LuLamp, icondata_include_all))]
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
#[cfg(any(LuLampCeiling, icondata_include_all))]
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
#[cfg(any(LuLampDesk, icondata_include_all))]
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
#[cfg(any(LuLampFloor, icondata_include_all))]
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
#[cfg(any(LuLampWallDown, icondata_include_all))]
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
#[cfg(any(LuLampWallUp, icondata_include_all))]
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
#[cfg(any(LuLandmark, icondata_include_all))]
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
#[cfg(any(LuLanguages, icondata_include_all))]
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
#[cfg(any(LuLaptop, icondata_include_all))]
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
#[cfg(any(LuLaptop2, icondata_include_all))]
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
#[cfg(any(LuLasso, icondata_include_all))]
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
#[cfg(any(LuLassoSelect, icondata_include_all))]
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
#[cfg(any(LuLaugh, icondata_include_all))]
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
#[cfg(any(LuLayers, icondata_include_all))]
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
#[cfg(any(LuLayout, icondata_include_all))]
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
#[cfg(any(LuLayoutDashboard, icondata_include_all))]
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
#[cfg(any(LuLayoutGrid, icondata_include_all))]
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
#[cfg(any(LuLayoutList, icondata_include_all))]
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
#[cfg(any(LuLayoutPanelLeft, icondata_include_all))]
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
#[cfg(any(LuLayoutPanelTop, icondata_include_all))]
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
#[cfg(any(LuLayoutTemplate, icondata_include_all))]
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
#[cfg(any(LuLeaf, icondata_include_all))]
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
#[cfg(any(LuLeafyGreen, icondata_include_all))]
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
#[cfg(any(LuLibrary, icondata_include_all))]
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
#[cfg(any(LuLifeBuoy, icondata_include_all))]
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
#[cfg(any(LuLigature, icondata_include_all))]
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
#[cfg(any(LuLightbulb, icondata_include_all))]
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
#[cfg(any(LuLightbulbOff, icondata_include_all))]
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
#[cfg(any(LuLineChart, icondata_include_all))]
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
#[cfg(any(LuLink, icondata_include_all))]
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
#[cfg(any(LuLink2, icondata_include_all))]
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
#[cfg(any(LuLink2Off, icondata_include_all))]
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
#[cfg(any(LuLinkedin, icondata_include_all))]
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
#[cfg(any(LuList, icondata_include_all))]
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
#[cfg(any(LuListChecks, icondata_include_all))]
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
#[cfg(any(LuListEnd, icondata_include_all))]
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
#[cfg(any(LuListFilter, icondata_include_all))]
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
#[cfg(any(LuListMinus, icondata_include_all))]
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
#[cfg(any(LuListMusic, icondata_include_all))]
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
#[cfg(any(LuListOrdered, icondata_include_all))]
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
#[cfg(any(LuListPlus, icondata_include_all))]
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
#[cfg(any(LuListRestart, icondata_include_all))]
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
#[cfg(any(LuListStart, icondata_include_all))]
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
#[cfg(any(LuListTodo, icondata_include_all))]
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
#[cfg(any(LuListTree, icondata_include_all))]
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
#[cfg(any(LuListVideo, icondata_include_all))]
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
#[cfg(any(LuListX, icondata_include_all))]
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
#[cfg(any(LuLoader, icondata_include_all))]
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
#[cfg(any(LuLoader2, icondata_include_all))]
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
#[cfg(any(LuLocate, icondata_include_all))]
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
#[cfg(any(LuLocateFixed, icondata_include_all))]
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
#[cfg(any(LuLocateOff, icondata_include_all))]
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
#[cfg(any(LuLock, icondata_include_all))]
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
#[cfg(any(LuLogIn, icondata_include_all))]
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
#[cfg(any(LuLogOut, icondata_include_all))]
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
#[cfg(any(LuLollipop, icondata_include_all))]
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
#[cfg(any(LuLuggage, icondata_include_all))]
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
#[cfg(any(LuMSquare, icondata_include_all))]
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
#[cfg(any(LuMagnet, icondata_include_all))]
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
#[cfg(any(LuMail, icondata_include_all))]
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
#[cfg(any(LuMailCheck, icondata_include_all))]
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
#[cfg(any(LuMailMinus, icondata_include_all))]
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
#[cfg(any(LuMailOpen, icondata_include_all))]
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
#[cfg(any(LuMailPlus, icondata_include_all))]
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
#[cfg(any(LuMailQuestion, icondata_include_all))]
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
#[cfg(any(LuMailSearch, icondata_include_all))]
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
#[cfg(any(LuMailWarning, icondata_include_all))]
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
#[cfg(any(LuMailX, icondata_include_all))]
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
#[cfg(any(LuMailbox, icondata_include_all))]
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
#[cfg(any(LuMails, icondata_include_all))]
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
#[cfg(any(LuMap, icondata_include_all))]
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
#[cfg(any(LuMapPin, icondata_include_all))]
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
#[cfg(any(LuMapPinOff, icondata_include_all))]
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
#[cfg(any(LuMartini, icondata_include_all))]
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
#[cfg(any(LuMaximize, icondata_include_all))]
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
#[cfg(any(LuMaximize2, icondata_include_all))]
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
#[cfg(any(LuMedal, icondata_include_all))]
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
#[cfg(any(LuMegaphone, icondata_include_all))]
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
#[cfg(any(LuMegaphoneOff, icondata_include_all))]
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
#[cfg(any(LuMeh, icondata_include_all))]
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
#[cfg(any(LuMemoryStick, icondata_include_all))]
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
#[cfg(any(LuMenu, icondata_include_all))]
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
#[cfg(any(LuMenuSquare, icondata_include_all))]
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
#[cfg(any(LuMerge, icondata_include_all))]
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
#[cfg(any(LuMessageCircle, icondata_include_all))]
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
#[cfg(any(LuMessageSquare, icondata_include_all))]
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
#[cfg(any(LuMessageSquareDashed, icondata_include_all))]
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
#[cfg(any(LuMessageSquarePlus, icondata_include_all))]
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
#[cfg(any(LuMessagesSquare, icondata_include_all))]
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
#[cfg(any(LuMic, icondata_include_all))]
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
#[cfg(any(LuMic2, icondata_include_all))]
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
#[cfg(any(LuMicOff, icondata_include_all))]
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
#[cfg(any(LuMicroscope, icondata_include_all))]
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
#[cfg(any(LuMicrowave, icondata_include_all))]
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
#[cfg(any(LuMilestone, icondata_include_all))]
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
#[cfg(any(LuMilk, icondata_include_all))]
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
#[cfg(any(LuMilkOff, icondata_include_all))]
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
#[cfg(any(LuMinimize, icondata_include_all))]
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
#[cfg(any(LuMinimize2, icondata_include_all))]
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
#[cfg(any(LuMinus, icondata_include_all))]
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
#[cfg(any(LuMinusCircle, icondata_include_all))]
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
#[cfg(any(LuMinusSquare, icondata_include_all))]
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
#[cfg(any(LuMonitor, icondata_include_all))]
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
#[cfg(any(LuMonitorCheck, icondata_include_all))]
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
#[cfg(any(LuMonitorDot, icondata_include_all))]
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
#[cfg(any(LuMonitorDown, icondata_include_all))]
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
#[cfg(any(LuMonitorOff, icondata_include_all))]
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
#[cfg(any(LuMonitorPause, icondata_include_all))]
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
#[cfg(any(LuMonitorPlay, icondata_include_all))]
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
#[cfg(any(LuMonitorSmartphone, icondata_include_all))]
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
#[cfg(any(LuMonitorSpeaker, icondata_include_all))]
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
#[cfg(any(LuMonitorStop, icondata_include_all))]
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
#[cfg(any(LuMonitorUp, icondata_include_all))]
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
#[cfg(any(LuMonitorX, icondata_include_all))]
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
#[cfg(any(LuMoon, icondata_include_all))]
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
#[cfg(any(LuMoonStar, icondata_include_all))]
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
#[cfg(any(LuMoreHorizontal, icondata_include_all))]
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
#[cfg(any(LuMoreVertical, icondata_include_all))]
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
#[cfg(any(LuMountain, icondata_include_all))]
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
#[cfg(any(LuMountainSnow, icondata_include_all))]
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
#[cfg(any(LuMouse, icondata_include_all))]
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
#[cfg(any(LuMousePointer, icondata_include_all))]
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
#[cfg(any(LuMousePointer2, icondata_include_all))]
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
#[cfg(any(LuMousePointerClick, icondata_include_all))]
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
#[cfg(any(LuMove, icondata_include_all))]
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
#[cfg(any(LuMove3d, icondata_include_all))]
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
#[cfg(any(LuMoveDiagonal, icondata_include_all))]
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
#[cfg(any(LuMoveDiagonal2, icondata_include_all))]
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
#[cfg(any(LuMoveDown, icondata_include_all))]
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
#[cfg(any(LuMoveDownLeft, icondata_include_all))]
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
#[cfg(any(LuMoveDownRight, icondata_include_all))]
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
#[cfg(any(LuMoveHorizontal, icondata_include_all))]
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
#[cfg(any(LuMoveLeft, icondata_include_all))]
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
#[cfg(any(LuMoveRight, icondata_include_all))]
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
#[cfg(any(LuMoveUp, icondata_include_all))]
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
#[cfg(any(LuMoveUpLeft, icondata_include_all))]
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
#[cfg(any(LuMoveUpRight, icondata_include_all))]
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
#[cfg(any(LuMoveVertical, icondata_include_all))]
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
#[cfg(any(LuMusic, icondata_include_all))]
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
#[cfg(any(LuMusic2, icondata_include_all))]
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
#[cfg(any(LuMusic3, icondata_include_all))]
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
#[cfg(any(LuMusic4, icondata_include_all))]
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
#[cfg(any(LuNavigation, icondata_include_all))]
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
#[cfg(any(LuNavigation2, icondata_include_all))]
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
#[cfg(any(LuNavigation2Off, icondata_include_all))]
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
#[cfg(any(LuNavigationOff, icondata_include_all))]
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
#[cfg(any(LuNetwork, icondata_include_all))]
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
#[cfg(any(LuNewspaper, icondata_include_all))]
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
#[cfg(any(LuNfc, icondata_include_all))]
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
#[cfg(any(LuNut, icondata_include_all))]
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
#[cfg(any(LuNutOff, icondata_include_all))]
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
#[cfg(any(LuOctagon, icondata_include_all))]
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
#[cfg(any(LuOption, icondata_include_all))]
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
#[cfg(any(LuOrbit, icondata_include_all))]
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
#[cfg(any(LuOutdent, icondata_include_all))]
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
#[cfg(any(LuPackage, icondata_include_all))]
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
#[cfg(any(LuPackage2, icondata_include_all))]
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
#[cfg(any(LuPackageCheck, icondata_include_all))]
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
#[cfg(any(LuPackageMinus, icondata_include_all))]
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
#[cfg(any(LuPackageOpen, icondata_include_all))]
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
#[cfg(any(LuPackagePlus, icondata_include_all))]
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
#[cfg(any(LuPackageSearch, icondata_include_all))]
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
#[cfg(any(LuPackageX, icondata_include_all))]
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
#[cfg(any(LuPaintBucket, icondata_include_all))]
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
#[cfg(any(LuPaintbrush, icondata_include_all))]
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
#[cfg(any(LuPaintbrush2, icondata_include_all))]
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
#[cfg(any(LuPalette, icondata_include_all))]
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
#[cfg(any(LuPalmtree, icondata_include_all))]
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
#[cfg(any(LuPanelBottom, icondata_include_all))]
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
#[cfg(any(LuPanelBottomClose, icondata_include_all))]
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
#[cfg(any(LuPanelBottomInactive, icondata_include_all))]
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
#[cfg(any(LuPanelBottomOpen, icondata_include_all))]
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
#[cfg(any(LuPanelLeft, icondata_include_all))]
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
#[cfg(any(LuPanelLeftClose, icondata_include_all))]
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
#[cfg(any(LuPanelLeftInactive, icondata_include_all))]
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
#[cfg(any(LuPanelLeftOpen, icondata_include_all))]
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
#[cfg(any(LuPanelRight, icondata_include_all))]
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
#[cfg(any(LuPanelRightClose, icondata_include_all))]
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
#[cfg(any(LuPanelRightInactive, icondata_include_all))]
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
#[cfg(any(LuPanelRightOpen, icondata_include_all))]
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
#[cfg(any(LuPanelTop, icondata_include_all))]
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
#[cfg(any(LuPanelTopClose, icondata_include_all))]
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
#[cfg(any(LuPanelTopInactive, icondata_include_all))]
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
#[cfg(any(LuPanelTopOpen, icondata_include_all))]
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
#[cfg(any(LuPaperclip, icondata_include_all))]
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
#[cfg(any(LuParentheses, icondata_include_all))]
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
#[cfg(any(LuParkingCircle, icondata_include_all))]
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
#[cfg(any(LuParkingCircleOff, icondata_include_all))]
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
#[cfg(any(LuParkingMeter, icondata_include_all))]
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
#[cfg(any(LuParkingSquare, icondata_include_all))]
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
#[cfg(any(LuParkingSquareOff, icondata_include_all))]
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
#[cfg(any(LuPartyPopper, icondata_include_all))]
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
#[cfg(any(LuPause, icondata_include_all))]
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
#[cfg(any(LuPauseCircle, icondata_include_all))]
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
#[cfg(any(LuPauseOctagon, icondata_include_all))]
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
#[cfg(any(LuPawPrint, icondata_include_all))]
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
#[cfg(any(LuPcCase, icondata_include_all))]
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
#[cfg(any(LuPen, icondata_include_all))]
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
#[cfg(any(LuPenLine, icondata_include_all))]
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
#[cfg(any(LuPenSquare, icondata_include_all))]
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
#[cfg(any(LuPenTool, icondata_include_all))]
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
#[cfg(any(LuPencil, icondata_include_all))]
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
#[cfg(any(LuPencilLine, icondata_include_all))]
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
#[cfg(any(LuPencilRuler, icondata_include_all))]
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
#[cfg(any(LuPercent, icondata_include_all))]
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
#[cfg(any(LuPersonStanding, icondata_include_all))]
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
#[cfg(any(LuPhone, icondata_include_all))]
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
#[cfg(any(LuPhoneCall, icondata_include_all))]
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
#[cfg(any(LuPhoneForwarded, icondata_include_all))]
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
#[cfg(any(LuPhoneIncoming, icondata_include_all))]
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
#[cfg(any(LuPhoneMissed, icondata_include_all))]
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
#[cfg(any(LuPhoneOff, icondata_include_all))]
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
#[cfg(any(LuPhoneOutgoing, icondata_include_all))]
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
#[cfg(any(LuPi, icondata_include_all))]
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
#[cfg(any(LuPiSquare, icondata_include_all))]
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
#[cfg(any(LuPictureInPicture, icondata_include_all))]
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
#[cfg(any(LuPictureInPicture2, icondata_include_all))]
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
#[cfg(any(LuPieChart, icondata_include_all))]
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
#[cfg(any(LuPiggyBank, icondata_include_all))]
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
#[cfg(any(LuPilcrow, icondata_include_all))]
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
#[cfg(any(LuPilcrowSquare, icondata_include_all))]
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
#[cfg(any(LuPill, icondata_include_all))]
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
#[cfg(any(LuPin, icondata_include_all))]
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
#[cfg(any(LuPinOff, icondata_include_all))]
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
#[cfg(any(LuPipette, icondata_include_all))]
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
#[cfg(any(LuPizza, icondata_include_all))]
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
#[cfg(any(LuPlane, icondata_include_all))]
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
#[cfg(any(LuPlaneLanding, icondata_include_all))]
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
#[cfg(any(LuPlaneTakeoff, icondata_include_all))]
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
#[cfg(any(LuPlay, icondata_include_all))]
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
#[cfg(any(LuPlayCircle, icondata_include_all))]
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
#[cfg(any(LuPlaySquare, icondata_include_all))]
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
#[cfg(any(LuPlug, icondata_include_all))]
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
#[cfg(any(LuPlug2, icondata_include_all))]
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
#[cfg(any(LuPlugZap, icondata_include_all))]
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
#[cfg(any(LuPlugZap2, icondata_include_all))]
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
#[cfg(any(LuPlus, icondata_include_all))]
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
#[cfg(any(LuPlusCircle, icondata_include_all))]
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
#[cfg(any(LuPlusSquare, icondata_include_all))]
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
#[cfg(any(LuPocket, icondata_include_all))]
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
#[cfg(any(LuPocketKnife, icondata_include_all))]
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
#[cfg(any(LuPodcast, icondata_include_all))]
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
#[cfg(any(LuPointer, icondata_include_all))]
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
#[cfg(any(LuPopcorn, icondata_include_all))]
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
#[cfg(any(LuPopsicle, icondata_include_all))]
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
#[cfg(any(LuPoundSterling, icondata_include_all))]
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
#[cfg(any(LuPower, icondata_include_all))]
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
#[cfg(any(LuPowerOff, icondata_include_all))]
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
#[cfg(any(LuPresentation, icondata_include_all))]
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
#[cfg(any(LuPrinter, icondata_include_all))]
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
#[cfg(any(LuProjector, icondata_include_all))]
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
#[cfg(any(LuPuzzle, icondata_include_all))]
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
#[cfg(any(LuQrCode, icondata_include_all))]
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
#[cfg(any(LuQuote, icondata_include_all))]
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
#[cfg(any(LuRabbit, icondata_include_all))]
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
#[cfg(any(LuRadar, icondata_include_all))]
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
#[cfg(any(LuRadiation, icondata_include_all))]
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
#[cfg(any(LuRadio, icondata_include_all))]
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
#[cfg(any(LuRadioReceiver, icondata_include_all))]
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
#[cfg(any(LuRadioTower, icondata_include_all))]
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
#[cfg(any(LuRailSymbol, icondata_include_all))]
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
#[cfg(any(LuRainbow, icondata_include_all))]
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
#[cfg(any(LuRat, icondata_include_all))]
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
#[cfg(any(LuRatio, icondata_include_all))]
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
#[cfg(any(LuReceipt, icondata_include_all))]
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
#[cfg(any(LuRectangleHorizontal, icondata_include_all))]
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
#[cfg(any(LuRectangleVertical, icondata_include_all))]
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
#[cfg(any(LuRecycle, icondata_include_all))]
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
#[cfg(any(LuRedo, icondata_include_all))]
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
#[cfg(any(LuRedo2, icondata_include_all))]
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
#[cfg(any(LuRedoDot, icondata_include_all))]
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
#[cfg(any(LuRefreshCcw, icondata_include_all))]
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
#[cfg(any(LuRefreshCcwDot, icondata_include_all))]
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
#[cfg(any(LuRefreshCw, icondata_include_all))]
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
#[cfg(any(LuRefreshCwOff, icondata_include_all))]
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
#[cfg(any(LuRefrigerator, icondata_include_all))]
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
#[cfg(any(LuRegex, icondata_include_all))]
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
#[cfg(any(LuRemoveFormatting, icondata_include_all))]
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
#[cfg(any(LuRepeat, icondata_include_all))]
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
#[cfg(any(LuRepeat1, icondata_include_all))]
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
#[cfg(any(LuRepeat2, icondata_include_all))]
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
#[cfg(any(LuReplace, icondata_include_all))]
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
#[cfg(any(LuReplaceAll, icondata_include_all))]
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
#[cfg(any(LuReply, icondata_include_all))]
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
#[cfg(any(LuReplyAll, icondata_include_all))]
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
#[cfg(any(LuRewind, icondata_include_all))]
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
#[cfg(any(LuRocket, icondata_include_all))]
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
#[cfg(any(LuRockingChair, icondata_include_all))]
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
#[cfg(any(LuRollerCoaster, icondata_include_all))]
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
#[cfg(any(LuRotate3d, icondata_include_all))]
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
#[cfg(any(LuRotateCcw, icondata_include_all))]
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
#[cfg(any(LuRotateCw, icondata_include_all))]
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
#[cfg(any(LuRouter, icondata_include_all))]
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
#[cfg(any(LuRows, icondata_include_all))]
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
#[cfg(any(LuRss, icondata_include_all))]
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
#[cfg(any(LuRuler, icondata_include_all))]
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
#[cfg(any(LuRussianRuble, icondata_include_all))]
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
#[cfg(any(LuSailboat, icondata_include_all))]
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
#[cfg(any(LuSalad, icondata_include_all))]
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
#[cfg(any(LuSandwich, icondata_include_all))]
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
#[cfg(any(LuSatellite, icondata_include_all))]
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
#[cfg(any(LuSatelliteDish, icondata_include_all))]
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
#[cfg(any(LuSave, icondata_include_all))]
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
#[cfg(any(LuSaveAll, icondata_include_all))]
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
#[cfg(any(LuScale, icondata_include_all))]
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
#[cfg(any(LuScale3d, icondata_include_all))]
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
#[cfg(any(LuScaling, icondata_include_all))]
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
#[cfg(any(LuScan, icondata_include_all))]
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
#[cfg(any(LuScanFace, icondata_include_all))]
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
#[cfg(any(LuScanLine, icondata_include_all))]
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
#[cfg(any(LuScatterChart, icondata_include_all))]
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
#[cfg(any(LuSchool, icondata_include_all))]
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
#[cfg(any(LuSchool2, icondata_include_all))]
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
#[cfg(any(LuScissors, icondata_include_all))]
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
#[cfg(any(LuScissorsLineDashed, icondata_include_all))]
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
#[cfg(any(LuScissorsSquare, icondata_include_all))]
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
#[cfg(any(LuScissorsSquareDashedBottom, icondata_include_all))]
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
#[cfg(any(LuScreenShare, icondata_include_all))]
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
#[cfg(any(LuScreenShareOff, icondata_include_all))]
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
#[cfg(any(LuScroll, icondata_include_all))]
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
#[cfg(any(LuScrollText, icondata_include_all))]
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
#[cfg(any(LuSearch, icondata_include_all))]
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
#[cfg(any(LuSearchCheck, icondata_include_all))]
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
#[cfg(any(LuSearchCode, icondata_include_all))]
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
#[cfg(any(LuSearchSlash, icondata_include_all))]
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
#[cfg(any(LuSearchX, icondata_include_all))]
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
#[cfg(any(LuSend, icondata_include_all))]
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
#[cfg(any(LuSendHorizonal, icondata_include_all))]
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
#[cfg(any(LuSendToBack, icondata_include_all))]
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
#[cfg(any(LuSeparatorHorizontal, icondata_include_all))]
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
#[cfg(any(LuSeparatorVertical, icondata_include_all))]
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
#[cfg(any(LuServer, icondata_include_all))]
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
#[cfg(any(LuServerCog, icondata_include_all))]
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
#[cfg(any(LuServerCrash, icondata_include_all))]
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
#[cfg(any(LuServerOff, icondata_include_all))]
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
#[cfg(any(LuSettings, icondata_include_all))]
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
#[cfg(any(LuSettings2, icondata_include_all))]
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
#[cfg(any(LuShapes, icondata_include_all))]
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
#[cfg(any(LuShare, icondata_include_all))]
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
#[cfg(any(LuShare2, icondata_include_all))]
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
#[cfg(any(LuSheet, icondata_include_all))]
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
#[cfg(any(LuShell, icondata_include_all))]
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
#[cfg(any(LuShield, icondata_include_all))]
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
#[cfg(any(LuShieldAlert, icondata_include_all))]
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
#[cfg(any(LuShieldCheck, icondata_include_all))]
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
#[cfg(any(LuShieldClose, icondata_include_all))]
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
#[cfg(any(LuShieldOff, icondata_include_all))]
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
#[cfg(any(LuShieldQuestion, icondata_include_all))]
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
#[cfg(any(LuShip, icondata_include_all))]
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
#[cfg(any(LuShipWheel, icondata_include_all))]
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
#[cfg(any(LuShirt, icondata_include_all))]
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
#[cfg(any(LuShoppingBag, icondata_include_all))]
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
#[cfg(any(LuShoppingBasket, icondata_include_all))]
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
#[cfg(any(LuShoppingCart, icondata_include_all))]
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
#[cfg(any(LuShovel, icondata_include_all))]
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
#[cfg(any(LuShowerHead, icondata_include_all))]
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
#[cfg(any(LuShrink, icondata_include_all))]
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
#[cfg(any(LuShrub, icondata_include_all))]
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
#[cfg(any(LuShuffle, icondata_include_all))]
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
#[cfg(any(LuSigma, icondata_include_all))]
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
#[cfg(any(LuSigmaSquare, icondata_include_all))]
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
#[cfg(any(LuSignal, icondata_include_all))]
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
#[cfg(any(LuSignalHigh, icondata_include_all))]
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
#[cfg(any(LuSignalLow, icondata_include_all))]
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
#[cfg(any(LuSignalMedium, icondata_include_all))]
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
#[cfg(any(LuSignalZero, icondata_include_all))]
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
#[cfg(any(LuSiren, icondata_include_all))]
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
#[cfg(any(LuSkipBack, icondata_include_all))]
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
#[cfg(any(LuSkipForward, icondata_include_all))]
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
#[cfg(any(LuSkull, icondata_include_all))]
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
#[cfg(any(LuSlack, icondata_include_all))]
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
#[cfg(any(LuSlice, icondata_include_all))]
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
#[cfg(any(LuSliders, icondata_include_all))]
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
#[cfg(any(LuSlidersHorizontal, icondata_include_all))]
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
#[cfg(any(LuSmartphone, icondata_include_all))]
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
#[cfg(any(LuSmartphoneCharging, icondata_include_all))]
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
#[cfg(any(LuSmartphoneNfc, icondata_include_all))]
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
#[cfg(any(LuSmile, icondata_include_all))]
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
#[cfg(any(LuSmilePlus, icondata_include_all))]
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
#[cfg(any(LuSnail, icondata_include_all))]
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
#[cfg(any(LuSnowflake, icondata_include_all))]
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
#[cfg(any(LuSofa, icondata_include_all))]
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
#[cfg(any(LuSoup, icondata_include_all))]
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
#[cfg(any(LuSpace, icondata_include_all))]
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
#[cfg(any(LuSpade, icondata_include_all))]
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
#[cfg(any(LuSparkle, icondata_include_all))]
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
#[cfg(any(LuSparkles, icondata_include_all))]
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
#[cfg(any(LuSpeaker, icondata_include_all))]
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
#[cfg(any(LuSpellCheck, icondata_include_all))]
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
#[cfg(any(LuSpellCheck2, icondata_include_all))]
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
#[cfg(any(LuSpline, icondata_include_all))]
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
#[cfg(any(LuSplit, icondata_include_all))]
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
#[cfg(any(LuSplitSquareHorizontal, icondata_include_all))]
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
#[cfg(any(LuSplitSquareVertical, icondata_include_all))]
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
#[cfg(any(LuSprayCan, icondata_include_all))]
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
#[cfg(any(LuSprout, icondata_include_all))]
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
#[cfg(any(LuSquare, icondata_include_all))]
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
#[cfg(any(LuSquareAsterisk, icondata_include_all))]
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
#[cfg(any(LuSquareCode, icondata_include_all))]
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
#[cfg(any(LuSquareDashedBottom, icondata_include_all))]
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
#[cfg(any(LuSquareDashedBottomCode, icondata_include_all))]
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
#[cfg(any(LuSquareDot, icondata_include_all))]
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
#[cfg(any(LuSquareEqual, icondata_include_all))]
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
#[cfg(any(LuSquareSlash, icondata_include_all))]
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
#[cfg(any(LuSquareStack, icondata_include_all))]
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
#[cfg(any(LuSquirrel, icondata_include_all))]
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
#[cfg(any(LuStamp, icondata_include_all))]
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
#[cfg(any(LuStar, icondata_include_all))]
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
#[cfg(any(LuStarHalf, icondata_include_all))]
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
#[cfg(any(LuStarOff, icondata_include_all))]
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
#[cfg(any(LuStepBack, icondata_include_all))]
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
#[cfg(any(LuStepForward, icondata_include_all))]
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
#[cfg(any(LuStethoscope, icondata_include_all))]
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
#[cfg(any(LuSticker, icondata_include_all))]
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
#[cfg(any(LuStickyNote, icondata_include_all))]
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
#[cfg(any(LuStopCircle, icondata_include_all))]
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
#[cfg(any(LuStore, icondata_include_all))]
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
#[cfg(any(LuStretchHorizontal, icondata_include_all))]
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
#[cfg(any(LuStretchVertical, icondata_include_all))]
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
#[cfg(any(LuStrikethrough, icondata_include_all))]
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
#[cfg(any(LuSubscript, icondata_include_all))]
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
#[cfg(any(LuSubtitles, icondata_include_all))]
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
#[cfg(any(LuSun, icondata_include_all))]
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
#[cfg(any(LuSunDim, icondata_include_all))]
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
#[cfg(any(LuSunMedium, icondata_include_all))]
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
#[cfg(any(LuSunMoon, icondata_include_all))]
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
#[cfg(any(LuSunSnow, icondata_include_all))]
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
#[cfg(any(LuSunrise, icondata_include_all))]
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
#[cfg(any(LuSunset, icondata_include_all))]
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
#[cfg(any(LuSuperscript, icondata_include_all))]
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
#[cfg(any(LuSwissFranc, icondata_include_all))]
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
#[cfg(any(LuSwitchCamera, icondata_include_all))]
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
#[cfg(any(LuSword, icondata_include_all))]
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
#[cfg(any(LuSwords, icondata_include_all))]
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
#[cfg(any(LuSyringe, icondata_include_all))]
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
#[cfg(any(LuTable, icondata_include_all))]
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
#[cfg(any(LuTable2, icondata_include_all))]
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
#[cfg(any(LuTableProperties, icondata_include_all))]
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
#[cfg(any(LuTablet, icondata_include_all))]
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
#[cfg(any(LuTablets, icondata_include_all))]
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
#[cfg(any(LuTag, icondata_include_all))]
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
#[cfg(any(LuTags, icondata_include_all))]
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
#[cfg(any(LuTally1, icondata_include_all))]
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
#[cfg(any(LuTally2, icondata_include_all))]
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
#[cfg(any(LuTally3, icondata_include_all))]
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
#[cfg(any(LuTally4, icondata_include_all))]
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
#[cfg(any(LuTally5, icondata_include_all))]
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
#[cfg(any(LuTarget, icondata_include_all))]
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
#[cfg(any(LuTent, icondata_include_all))]
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
#[cfg(any(LuTerminal, icondata_include_all))]
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
#[cfg(any(LuTerminalSquare, icondata_include_all))]
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
#[cfg(any(LuTestTube, icondata_include_all))]
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
#[cfg(any(LuTestTube2, icondata_include_all))]
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
#[cfg(any(LuTestTubes, icondata_include_all))]
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
#[cfg(any(LuText, icondata_include_all))]
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
#[cfg(any(LuTextCursor, icondata_include_all))]
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
#[cfg(any(LuTextCursorInput, icondata_include_all))]
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
#[cfg(any(LuTextQuote, icondata_include_all))]
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
#[cfg(any(LuTextSelect, icondata_include_all))]
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
#[cfg(any(LuThermometer, icondata_include_all))]
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
#[cfg(any(LuThermometerSnowflake, icondata_include_all))]
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
#[cfg(any(LuThermometerSun, icondata_include_all))]
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
#[cfg(any(LuThumbsDown, icondata_include_all))]
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
#[cfg(any(LuThumbsUp, icondata_include_all))]
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
#[cfg(any(LuTicket, icondata_include_all))]
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
#[cfg(any(LuTimer, icondata_include_all))]
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
#[cfg(any(LuTimerOff, icondata_include_all))]
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
#[cfg(any(LuTimerReset, icondata_include_all))]
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
#[cfg(any(LuToggleLeft, icondata_include_all))]
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
#[cfg(any(LuToggleRight, icondata_include_all))]
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
#[cfg(any(LuTornado, icondata_include_all))]
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
#[cfg(any(LuTouchpad, icondata_include_all))]
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
#[cfg(any(LuTouchpadOff, icondata_include_all))]
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
#[cfg(any(LuTowerControl, icondata_include_all))]
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
#[cfg(any(LuToyBrick, icondata_include_all))]
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
#[cfg(any(LuTractor, icondata_include_all))]
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
#[cfg(any(LuTrafficCone, icondata_include_all))]
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
#[cfg(any(LuTrainFront, icondata_include_all))]
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
#[cfg(any(LuTrainFrontTunnel, icondata_include_all))]
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
#[cfg(any(LuTrainTrack, icondata_include_all))]
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
#[cfg(any(LuTramFront, icondata_include_all))]
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
#[cfg(any(LuTrash, icondata_include_all))]
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
#[cfg(any(LuTrash2, icondata_include_all))]
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
#[cfg(any(LuTreeDeciduous, icondata_include_all))]
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
#[cfg(any(LuTreePine, icondata_include_all))]
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
#[cfg(any(LuTrees, icondata_include_all))]
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
#[cfg(any(LuTrello, icondata_include_all))]
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
#[cfg(any(LuTrendingDown, icondata_include_all))]
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
#[cfg(any(LuTrendingUp, icondata_include_all))]
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
#[cfg(any(LuTriangle, icondata_include_all))]
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
#[cfg(any(LuTriangleRight, icondata_include_all))]
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
#[cfg(any(LuTrophy, icondata_include_all))]
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
#[cfg(any(LuTruck, icondata_include_all))]
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
#[cfg(any(LuTurtle, icondata_include_all))]
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
#[cfg(any(LuTv, icondata_include_all))]
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
#[cfg(any(LuTv2, icondata_include_all))]
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
#[cfg(any(LuTwitch, icondata_include_all))]
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
#[cfg(any(LuTwitter, icondata_include_all))]
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
#[cfg(any(LuType, icondata_include_all))]
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
#[cfg(any(LuUmbrella, icondata_include_all))]
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
#[cfg(any(LuUnderline, icondata_include_all))]
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
#[cfg(any(LuUndo, icondata_include_all))]
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
#[cfg(any(LuUndo2, icondata_include_all))]
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
#[cfg(any(LuUndoDot, icondata_include_all))]
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
#[cfg(any(LuUnfoldHorizontal, icondata_include_all))]
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
#[cfg(any(LuUnfoldVertical, icondata_include_all))]
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
#[cfg(any(LuUngroup, icondata_include_all))]
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
#[cfg(any(LuUnlink, icondata_include_all))]
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
#[cfg(any(LuUnlink2, icondata_include_all))]
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
#[cfg(any(LuUnlock, icondata_include_all))]
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
#[cfg(any(LuUnplug, icondata_include_all))]
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
#[cfg(any(LuUpload, icondata_include_all))]
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
#[cfg(any(LuUploadCloud, icondata_include_all))]
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
#[cfg(any(LuUsb, icondata_include_all))]
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
#[cfg(any(LuUser, icondata_include_all))]
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
#[cfg(any(LuUser2, icondata_include_all))]
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
#[cfg(any(LuUserCheck, icondata_include_all))]
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
#[cfg(any(LuUserCheck2, icondata_include_all))]
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
#[cfg(any(LuUserCircle, icondata_include_all))]
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
#[cfg(any(LuUserCircle2, icondata_include_all))]
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
#[cfg(any(LuUserCog, icondata_include_all))]
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
#[cfg(any(LuUserCog2, icondata_include_all))]
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
#[cfg(any(LuUserMinus, icondata_include_all))]
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
#[cfg(any(LuUserMinus2, icondata_include_all))]
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
#[cfg(any(LuUserPlus, icondata_include_all))]
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
#[cfg(any(LuUserPlus2, icondata_include_all))]
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
#[cfg(any(LuUserSquare, icondata_include_all))]
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
#[cfg(any(LuUserSquare2, icondata_include_all))]
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
#[cfg(any(LuUserX, icondata_include_all))]
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
#[cfg(any(LuUserX2, icondata_include_all))]
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
#[cfg(any(LuUsers, icondata_include_all))]
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
#[cfg(any(LuUsers2, icondata_include_all))]
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
#[cfg(any(LuUtensils, icondata_include_all))]
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
#[cfg(any(LuUtensilsCrossed, icondata_include_all))]
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
#[cfg(any(LuUtilityPole, icondata_include_all))]
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
#[cfg(any(LuVariable, icondata_include_all))]
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
#[cfg(any(LuVegan, icondata_include_all))]
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
#[cfg(any(LuVenetianMask, icondata_include_all))]
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
#[cfg(any(LuVibrate, icondata_include_all))]
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
#[cfg(any(LuVibrateOff, icondata_include_all))]
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
#[cfg(any(LuVideo, icondata_include_all))]
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
#[cfg(any(LuVideoOff, icondata_include_all))]
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
#[cfg(any(LuVideotape, icondata_include_all))]
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
#[cfg(any(LuView, icondata_include_all))]
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
#[cfg(any(LuVoicemail, icondata_include_all))]
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
#[cfg(any(LuVolume, icondata_include_all))]
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
#[cfg(any(LuVolume1, icondata_include_all))]
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
#[cfg(any(LuVolume2, icondata_include_all))]
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
#[cfg(any(LuVolumeX, icondata_include_all))]
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
#[cfg(any(LuVote, icondata_include_all))]
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
#[cfg(any(LuWallet, icondata_include_all))]
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
#[cfg(any(LuWallet2, icondata_include_all))]
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
#[cfg(any(LuWalletCards, icondata_include_all))]
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
#[cfg(any(LuWallpaper, icondata_include_all))]
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
#[cfg(any(LuWand, icondata_include_all))]
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
#[cfg(any(LuWand2, icondata_include_all))]
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
#[cfg(any(LuWarehouse, icondata_include_all))]
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
#[cfg(any(LuWatch, icondata_include_all))]
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
#[cfg(any(LuWaves, icondata_include_all))]
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
#[cfg(any(LuWebcam, icondata_include_all))]
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
#[cfg(any(LuWebhook, icondata_include_all))]
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
#[cfg(any(LuWheat, icondata_include_all))]
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
#[cfg(any(LuWheatOff, icondata_include_all))]
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
#[cfg(any(LuWholeWord, icondata_include_all))]
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
#[cfg(any(LuWifi, icondata_include_all))]
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
#[cfg(any(LuWifiOff, icondata_include_all))]
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
#[cfg(any(LuWind, icondata_include_all))]
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
#[cfg(any(LuWine, icondata_include_all))]
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
#[cfg(any(LuWineOff, icondata_include_all))]
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
#[cfg(any(LuWorkflow, icondata_include_all))]
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
#[cfg(any(LuWrapText, icondata_include_all))]
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
#[cfg(any(LuWrench, icondata_include_all))]
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
#[cfg(any(LuX, icondata_include_all))]
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
#[cfg(any(LuXCircle, icondata_include_all))]
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
#[cfg(any(LuXOctagon, icondata_include_all))]
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
#[cfg(any(LuXSquare, icondata_include_all))]
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
#[cfg(any(LuYoutube, icondata_include_all))]
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
#[cfg(any(LuZap, icondata_include_all))]
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
#[cfg(any(LuZapOff, icondata_include_all))]
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
#[cfg(any(LuZoomIn, icondata_include_all))]
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
#[cfg(any(LuZoomOut, icondata_include_all))]
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
            #[cfg(any(LuAccessibility, icondata_include_all))]
            LuIcon::LuAccessibility => LU_ACCESSIBILITY,
            #[cfg(any(LuActivity, icondata_include_all))]
            LuIcon::LuActivity => LU_ACTIVITY,
            #[cfg(any(LuActivitySquare, icondata_include_all))]
            LuIcon::LuActivitySquare => LU_ACTIVITY_SQUARE,
            #[cfg(any(LuAirVent, icondata_include_all))]
            LuIcon::LuAirVent => LU_AIR_VENT,
            #[cfg(any(LuAirplay, icondata_include_all))]
            LuIcon::LuAirplay => LU_AIRPLAY,
            #[cfg(any(LuAlarmCheck, icondata_include_all))]
            LuIcon::LuAlarmCheck => LU_ALARM_CHECK,
            #[cfg(any(LuAlarmClock, icondata_include_all))]
            LuIcon::LuAlarmClock => LU_ALARM_CLOCK,
            #[cfg(any(LuAlarmClockOff, icondata_include_all))]
            LuIcon::LuAlarmClockOff => LU_ALARM_CLOCK_OFF,
            #[cfg(any(LuAlarmMinus, icondata_include_all))]
            LuIcon::LuAlarmMinus => LU_ALARM_MINUS,
            #[cfg(any(LuAlarmPlus, icondata_include_all))]
            LuIcon::LuAlarmPlus => LU_ALARM_PLUS,
            #[cfg(any(LuAlbum, icondata_include_all))]
            LuIcon::LuAlbum => LU_ALBUM,
            #[cfg(any(LuAlertCircle, icondata_include_all))]
            LuIcon::LuAlertCircle => LU_ALERT_CIRCLE,
            #[cfg(any(LuAlertOctagon, icondata_include_all))]
            LuIcon::LuAlertOctagon => LU_ALERT_OCTAGON,
            #[cfg(any(LuAlertTriangle, icondata_include_all))]
            LuIcon::LuAlertTriangle => LU_ALERT_TRIANGLE,
            #[cfg(any(LuAlignCenter, icondata_include_all))]
            LuIcon::LuAlignCenter => LU_ALIGN_CENTER,
            #[cfg(any(LuAlignCenterHorizontal, icondata_include_all))]
            LuIcon::LuAlignCenterHorizontal => LU_ALIGN_CENTER_HORIZONTAL,
            #[cfg(any(LuAlignCenterVertical, icondata_include_all))]
            LuIcon::LuAlignCenterVertical => LU_ALIGN_CENTER_VERTICAL,
            #[cfg(any(LuAlignEndHorizontal, icondata_include_all))]
            LuIcon::LuAlignEndHorizontal => LU_ALIGN_END_HORIZONTAL,
            #[cfg(any(LuAlignEndVertical, icondata_include_all))]
            LuIcon::LuAlignEndVertical => LU_ALIGN_END_VERTICAL,
            #[cfg(any(LuAlignHorizontalDistributeCenter, icondata_include_all))]
            LuIcon::LuAlignHorizontalDistributeCenter => LU_ALIGN_HORIZONTAL_DISTRIBUTE_CENTER,
            #[cfg(any(LuAlignHorizontalDistributeEnd, icondata_include_all))]
            LuIcon::LuAlignHorizontalDistributeEnd => LU_ALIGN_HORIZONTAL_DISTRIBUTE_END,
            #[cfg(any(LuAlignHorizontalDistributeStart, icondata_include_all))]
            LuIcon::LuAlignHorizontalDistributeStart => LU_ALIGN_HORIZONTAL_DISTRIBUTE_START,
            #[cfg(any(LuAlignHorizontalJustifyCenter, icondata_include_all))]
            LuIcon::LuAlignHorizontalJustifyCenter => LU_ALIGN_HORIZONTAL_JUSTIFY_CENTER,
            #[cfg(any(LuAlignHorizontalJustifyEnd, icondata_include_all))]
            LuIcon::LuAlignHorizontalJustifyEnd => LU_ALIGN_HORIZONTAL_JUSTIFY_END,
            #[cfg(any(LuAlignHorizontalJustifyStart, icondata_include_all))]
            LuIcon::LuAlignHorizontalJustifyStart => LU_ALIGN_HORIZONTAL_JUSTIFY_START,
            #[cfg(any(LuAlignHorizontalSpaceAround, icondata_include_all))]
            LuIcon::LuAlignHorizontalSpaceAround => LU_ALIGN_HORIZONTAL_SPACE_AROUND,
            #[cfg(any(LuAlignHorizontalSpaceBetween, icondata_include_all))]
            LuIcon::LuAlignHorizontalSpaceBetween => LU_ALIGN_HORIZONTAL_SPACE_BETWEEN,
            #[cfg(any(LuAlignJustify, icondata_include_all))]
            LuIcon::LuAlignJustify => LU_ALIGN_JUSTIFY,
            #[cfg(any(LuAlignLeft, icondata_include_all))]
            LuIcon::LuAlignLeft => LU_ALIGN_LEFT,
            #[cfg(any(LuAlignRight, icondata_include_all))]
            LuIcon::LuAlignRight => LU_ALIGN_RIGHT,
            #[cfg(any(LuAlignStartHorizontal, icondata_include_all))]
            LuIcon::LuAlignStartHorizontal => LU_ALIGN_START_HORIZONTAL,
            #[cfg(any(LuAlignStartVertical, icondata_include_all))]
            LuIcon::LuAlignStartVertical => LU_ALIGN_START_VERTICAL,
            #[cfg(any(LuAlignVerticalDistributeCenter, icondata_include_all))]
            LuIcon::LuAlignVerticalDistributeCenter => LU_ALIGN_VERTICAL_DISTRIBUTE_CENTER,
            #[cfg(any(LuAlignVerticalDistributeEnd, icondata_include_all))]
            LuIcon::LuAlignVerticalDistributeEnd => LU_ALIGN_VERTICAL_DISTRIBUTE_END,
            #[cfg(any(LuAlignVerticalDistributeStart, icondata_include_all))]
            LuIcon::LuAlignVerticalDistributeStart => LU_ALIGN_VERTICAL_DISTRIBUTE_START,
            #[cfg(any(LuAlignVerticalJustifyCenter, icondata_include_all))]
            LuIcon::LuAlignVerticalJustifyCenter => LU_ALIGN_VERTICAL_JUSTIFY_CENTER,
            #[cfg(any(LuAlignVerticalJustifyEnd, icondata_include_all))]
            LuIcon::LuAlignVerticalJustifyEnd => LU_ALIGN_VERTICAL_JUSTIFY_END,
            #[cfg(any(LuAlignVerticalJustifyStart, icondata_include_all))]
            LuIcon::LuAlignVerticalJustifyStart => LU_ALIGN_VERTICAL_JUSTIFY_START,
            #[cfg(any(LuAlignVerticalSpaceAround, icondata_include_all))]
            LuIcon::LuAlignVerticalSpaceAround => LU_ALIGN_VERTICAL_SPACE_AROUND,
            #[cfg(any(LuAlignVerticalSpaceBetween, icondata_include_all))]
            LuIcon::LuAlignVerticalSpaceBetween => LU_ALIGN_VERTICAL_SPACE_BETWEEN,
            #[cfg(any(LuAmpersand, icondata_include_all))]
            LuIcon::LuAmpersand => LU_AMPERSAND,
            #[cfg(any(LuAmpersands, icondata_include_all))]
            LuIcon::LuAmpersands => LU_AMPERSANDS,
            #[cfg(any(LuAnchor, icondata_include_all))]
            LuIcon::LuAnchor => LU_ANCHOR,
            #[cfg(any(LuAngry, icondata_include_all))]
            LuIcon::LuAngry => LU_ANGRY,
            #[cfg(any(LuAnnoyed, icondata_include_all))]
            LuIcon::LuAnnoyed => LU_ANNOYED,
            #[cfg(any(LuAntenna, icondata_include_all))]
            LuIcon::LuAntenna => LU_ANTENNA,
            #[cfg(any(LuAperture, icondata_include_all))]
            LuIcon::LuAperture => LU_APERTURE,
            #[cfg(any(LuAppWindow, icondata_include_all))]
            LuIcon::LuAppWindow => LU_APP_WINDOW,
            #[cfg(any(LuApple, icondata_include_all))]
            LuIcon::LuApple => LU_APPLE,
            #[cfg(any(LuArchive, icondata_include_all))]
            LuIcon::LuArchive => LU_ARCHIVE,
            #[cfg(any(LuArchiveRestore, icondata_include_all))]
            LuIcon::LuArchiveRestore => LU_ARCHIVE_RESTORE,
            #[cfg(any(LuAreaChart, icondata_include_all))]
            LuIcon::LuAreaChart => LU_AREA_CHART,
            #[cfg(any(LuArmchair, icondata_include_all))]
            LuIcon::LuArmchair => LU_ARMCHAIR,
            #[cfg(any(LuArrowBigDown, icondata_include_all))]
            LuIcon::LuArrowBigDown => LU_ARROW_BIG_DOWN,
            #[cfg(any(LuArrowBigDownDash, icondata_include_all))]
            LuIcon::LuArrowBigDownDash => LU_ARROW_BIG_DOWN_DASH,
            #[cfg(any(LuArrowBigLeft, icondata_include_all))]
            LuIcon::LuArrowBigLeft => LU_ARROW_BIG_LEFT,
            #[cfg(any(LuArrowBigLeftDash, icondata_include_all))]
            LuIcon::LuArrowBigLeftDash => LU_ARROW_BIG_LEFT_DASH,
            #[cfg(any(LuArrowBigRight, icondata_include_all))]
            LuIcon::LuArrowBigRight => LU_ARROW_BIG_RIGHT,
            #[cfg(any(LuArrowBigRightDash, icondata_include_all))]
            LuIcon::LuArrowBigRightDash => LU_ARROW_BIG_RIGHT_DASH,
            #[cfg(any(LuArrowBigUp, icondata_include_all))]
            LuIcon::LuArrowBigUp => LU_ARROW_BIG_UP,
            #[cfg(any(LuArrowBigUpDash, icondata_include_all))]
            LuIcon::LuArrowBigUpDash => LU_ARROW_BIG_UP_DASH,
            #[cfg(any(LuArrowDown, icondata_include_all))]
            LuIcon::LuArrowDown => LU_ARROW_DOWN,
            #[cfg(any(LuArrowDown01, icondata_include_all))]
            LuIcon::LuArrowDown01 => LU_ARROW_DOWN01,
            #[cfg(any(LuArrowDown10, icondata_include_all))]
            LuIcon::LuArrowDown10 => LU_ARROW_DOWN10,
            #[cfg(any(LuArrowDownAZ, icondata_include_all))]
            LuIcon::LuArrowDownAZ => LU_ARROW_DOWN_AZ,
            #[cfg(any(LuArrowDownCircle, icondata_include_all))]
            LuIcon::LuArrowDownCircle => LU_ARROW_DOWN_CIRCLE,
            #[cfg(any(LuArrowDownFromLine, icondata_include_all))]
            LuIcon::LuArrowDownFromLine => LU_ARROW_DOWN_FROM_LINE,
            #[cfg(any(LuArrowDownLeft, icondata_include_all))]
            LuIcon::LuArrowDownLeft => LU_ARROW_DOWN_LEFT,
            #[cfg(any(LuArrowDownLeftFromCircle, icondata_include_all))]
            LuIcon::LuArrowDownLeftFromCircle => LU_ARROW_DOWN_LEFT_FROM_CIRCLE,
            #[cfg(any(LuArrowDownLeftSquare, icondata_include_all))]
            LuIcon::LuArrowDownLeftSquare => LU_ARROW_DOWN_LEFT_SQUARE,
            #[cfg(any(LuArrowDownNarrowWide, icondata_include_all))]
            LuIcon::LuArrowDownNarrowWide => LU_ARROW_DOWN_NARROW_WIDE,
            #[cfg(any(LuArrowDownRight, icondata_include_all))]
            LuIcon::LuArrowDownRight => LU_ARROW_DOWN_RIGHT,
            #[cfg(any(LuArrowDownRightFromCircle, icondata_include_all))]
            LuIcon::LuArrowDownRightFromCircle => LU_ARROW_DOWN_RIGHT_FROM_CIRCLE,
            #[cfg(any(LuArrowDownRightSquare, icondata_include_all))]
            LuIcon::LuArrowDownRightSquare => LU_ARROW_DOWN_RIGHT_SQUARE,
            #[cfg(any(LuArrowDownSquare, icondata_include_all))]
            LuIcon::LuArrowDownSquare => LU_ARROW_DOWN_SQUARE,
            #[cfg(any(LuArrowDownToDot, icondata_include_all))]
            LuIcon::LuArrowDownToDot => LU_ARROW_DOWN_TO_DOT,
            #[cfg(any(LuArrowDownToLine, icondata_include_all))]
            LuIcon::LuArrowDownToLine => LU_ARROW_DOWN_TO_LINE,
            #[cfg(any(LuArrowDownUp, icondata_include_all))]
            LuIcon::LuArrowDownUp => LU_ARROW_DOWN_UP,
            #[cfg(any(LuArrowDownWideNarrow, icondata_include_all))]
            LuIcon::LuArrowDownWideNarrow => LU_ARROW_DOWN_WIDE_NARROW,
            #[cfg(any(LuArrowDownZA, icondata_include_all))]
            LuIcon::LuArrowDownZA => LU_ARROW_DOWN_ZA,
            #[cfg(any(LuArrowLeft, icondata_include_all))]
            LuIcon::LuArrowLeft => LU_ARROW_LEFT,
            #[cfg(any(LuArrowLeftCircle, icondata_include_all))]
            LuIcon::LuArrowLeftCircle => LU_ARROW_LEFT_CIRCLE,
            #[cfg(any(LuArrowLeftFromLine, icondata_include_all))]
            LuIcon::LuArrowLeftFromLine => LU_ARROW_LEFT_FROM_LINE,
            #[cfg(any(LuArrowLeftRight, icondata_include_all))]
            LuIcon::LuArrowLeftRight => LU_ARROW_LEFT_RIGHT,
            #[cfg(any(LuArrowLeftSquare, icondata_include_all))]
            LuIcon::LuArrowLeftSquare => LU_ARROW_LEFT_SQUARE,
            #[cfg(any(LuArrowLeftToLine, icondata_include_all))]
            LuIcon::LuArrowLeftToLine => LU_ARROW_LEFT_TO_LINE,
            #[cfg(any(LuArrowRight, icondata_include_all))]
            LuIcon::LuArrowRight => LU_ARROW_RIGHT,
            #[cfg(any(LuArrowRightCircle, icondata_include_all))]
            LuIcon::LuArrowRightCircle => LU_ARROW_RIGHT_CIRCLE,
            #[cfg(any(LuArrowRightFromLine, icondata_include_all))]
            LuIcon::LuArrowRightFromLine => LU_ARROW_RIGHT_FROM_LINE,
            #[cfg(any(LuArrowRightLeft, icondata_include_all))]
            LuIcon::LuArrowRightLeft => LU_ARROW_RIGHT_LEFT,
            #[cfg(any(LuArrowRightSquare, icondata_include_all))]
            LuIcon::LuArrowRightSquare => LU_ARROW_RIGHT_SQUARE,
            #[cfg(any(LuArrowRightToLine, icondata_include_all))]
            LuIcon::LuArrowRightToLine => LU_ARROW_RIGHT_TO_LINE,
            #[cfg(any(LuArrowUp, icondata_include_all))]
            LuIcon::LuArrowUp => LU_ARROW_UP,
            #[cfg(any(LuArrowUp01, icondata_include_all))]
            LuIcon::LuArrowUp01 => LU_ARROW_UP01,
            #[cfg(any(LuArrowUp10, icondata_include_all))]
            LuIcon::LuArrowUp10 => LU_ARROW_UP10,
            #[cfg(any(LuArrowUpAZ, icondata_include_all))]
            LuIcon::LuArrowUpAZ => LU_ARROW_UP_AZ,
            #[cfg(any(LuArrowUpCircle, icondata_include_all))]
            LuIcon::LuArrowUpCircle => LU_ARROW_UP_CIRCLE,
            #[cfg(any(LuArrowUpDown, icondata_include_all))]
            LuIcon::LuArrowUpDown => LU_ARROW_UP_DOWN,
            #[cfg(any(LuArrowUpFromDot, icondata_include_all))]
            LuIcon::LuArrowUpFromDot => LU_ARROW_UP_FROM_DOT,
            #[cfg(any(LuArrowUpFromLine, icondata_include_all))]
            LuIcon::LuArrowUpFromLine => LU_ARROW_UP_FROM_LINE,
            #[cfg(any(LuArrowUpLeft, icondata_include_all))]
            LuIcon::LuArrowUpLeft => LU_ARROW_UP_LEFT,
            #[cfg(any(LuArrowUpLeftFromCircle, icondata_include_all))]
            LuIcon::LuArrowUpLeftFromCircle => LU_ARROW_UP_LEFT_FROM_CIRCLE,
            #[cfg(any(LuArrowUpLeftSquare, icondata_include_all))]
            LuIcon::LuArrowUpLeftSquare => LU_ARROW_UP_LEFT_SQUARE,
            #[cfg(any(LuArrowUpNarrowWide, icondata_include_all))]
            LuIcon::LuArrowUpNarrowWide => LU_ARROW_UP_NARROW_WIDE,
            #[cfg(any(LuArrowUpRight, icondata_include_all))]
            LuIcon::LuArrowUpRight => LU_ARROW_UP_RIGHT,
            #[cfg(any(LuArrowUpRightFromCircle, icondata_include_all))]
            LuIcon::LuArrowUpRightFromCircle => LU_ARROW_UP_RIGHT_FROM_CIRCLE,
            #[cfg(any(LuArrowUpRightSquare, icondata_include_all))]
            LuIcon::LuArrowUpRightSquare => LU_ARROW_UP_RIGHT_SQUARE,
            #[cfg(any(LuArrowUpSquare, icondata_include_all))]
            LuIcon::LuArrowUpSquare => LU_ARROW_UP_SQUARE,
            #[cfg(any(LuArrowUpToLine, icondata_include_all))]
            LuIcon::LuArrowUpToLine => LU_ARROW_UP_TO_LINE,
            #[cfg(any(LuArrowUpWideNarrow, icondata_include_all))]
            LuIcon::LuArrowUpWideNarrow => LU_ARROW_UP_WIDE_NARROW,
            #[cfg(any(LuArrowUpZA, icondata_include_all))]
            LuIcon::LuArrowUpZA => LU_ARROW_UP_ZA,
            #[cfg(any(LuArrowsUpFromLine, icondata_include_all))]
            LuIcon::LuArrowsUpFromLine => LU_ARROWS_UP_FROM_LINE,
            #[cfg(any(LuAsterisk, icondata_include_all))]
            LuIcon::LuAsterisk => LU_ASTERISK,
            #[cfg(any(LuAtSign, icondata_include_all))]
            LuIcon::LuAtSign => LU_AT_SIGN,
            #[cfg(any(LuAtom, icondata_include_all))]
            LuIcon::LuAtom => LU_ATOM,
            #[cfg(any(LuAward, icondata_include_all))]
            LuIcon::LuAward => LU_AWARD,
            #[cfg(any(LuAxe, icondata_include_all))]
            LuIcon::LuAxe => LU_AXE,
            #[cfg(any(LuAxis3d, icondata_include_all))]
            LuIcon::LuAxis3d => LU_AXIS3D,
            #[cfg(any(LuBaby, icondata_include_all))]
            LuIcon::LuBaby => LU_BABY,
            #[cfg(any(LuBackpack, icondata_include_all))]
            LuIcon::LuBackpack => LU_BACKPACK,
            #[cfg(any(LuBadge, icondata_include_all))]
            LuIcon::LuBadge => LU_BADGE,
            #[cfg(any(LuBadgeAlert, icondata_include_all))]
            LuIcon::LuBadgeAlert => LU_BADGE_ALERT,
            #[cfg(any(LuBadgeCheck, icondata_include_all))]
            LuIcon::LuBadgeCheck => LU_BADGE_CHECK,
            #[cfg(any(LuBadgeDollarSign, icondata_include_all))]
            LuIcon::LuBadgeDollarSign => LU_BADGE_DOLLAR_SIGN,
            #[cfg(any(LuBadgeHelp, icondata_include_all))]
            LuIcon::LuBadgeHelp => LU_BADGE_HELP,
            #[cfg(any(LuBadgeInfo, icondata_include_all))]
            LuIcon::LuBadgeInfo => LU_BADGE_INFO,
            #[cfg(any(LuBadgeMinus, icondata_include_all))]
            LuIcon::LuBadgeMinus => LU_BADGE_MINUS,
            #[cfg(any(LuBadgePercent, icondata_include_all))]
            LuIcon::LuBadgePercent => LU_BADGE_PERCENT,
            #[cfg(any(LuBadgePlus, icondata_include_all))]
            LuIcon::LuBadgePlus => LU_BADGE_PLUS,
            #[cfg(any(LuBadgeX, icondata_include_all))]
            LuIcon::LuBadgeX => LU_BADGE_X,
            #[cfg(any(LuBaggageClaim, icondata_include_all))]
            LuIcon::LuBaggageClaim => LU_BAGGAGE_CLAIM,
            #[cfg(any(LuBan, icondata_include_all))]
            LuIcon::LuBan => LU_BAN,
            #[cfg(any(LuBanana, icondata_include_all))]
            LuIcon::LuBanana => LU_BANANA,
            #[cfg(any(LuBanknote, icondata_include_all))]
            LuIcon::LuBanknote => LU_BANKNOTE,
            #[cfg(any(LuBarChart, icondata_include_all))]
            LuIcon::LuBarChart => LU_BAR_CHART,
            #[cfg(any(LuBarChart2, icondata_include_all))]
            LuIcon::LuBarChart2 => LU_BAR_CHART2,
            #[cfg(any(LuBarChart3, icondata_include_all))]
            LuIcon::LuBarChart3 => LU_BAR_CHART3,
            #[cfg(any(LuBarChart4, icondata_include_all))]
            LuIcon::LuBarChart4 => LU_BAR_CHART4,
            #[cfg(any(LuBarChartBig, icondata_include_all))]
            LuIcon::LuBarChartBig => LU_BAR_CHART_BIG,
            #[cfg(any(LuBarChartHorizontal, icondata_include_all))]
            LuIcon::LuBarChartHorizontal => LU_BAR_CHART_HORIZONTAL,
            #[cfg(any(LuBarChartHorizontalBig, icondata_include_all))]
            LuIcon::LuBarChartHorizontalBig => LU_BAR_CHART_HORIZONTAL_BIG,
            #[cfg(any(LuBaseline, icondata_include_all))]
            LuIcon::LuBaseline => LU_BASELINE,
            #[cfg(any(LuBath, icondata_include_all))]
            LuIcon::LuBath => LU_BATH,
            #[cfg(any(LuBattery, icondata_include_all))]
            LuIcon::LuBattery => LU_BATTERY,
            #[cfg(any(LuBatteryCharging, icondata_include_all))]
            LuIcon::LuBatteryCharging => LU_BATTERY_CHARGING,
            #[cfg(any(LuBatteryFull, icondata_include_all))]
            LuIcon::LuBatteryFull => LU_BATTERY_FULL,
            #[cfg(any(LuBatteryLow, icondata_include_all))]
            LuIcon::LuBatteryLow => LU_BATTERY_LOW,
            #[cfg(any(LuBatteryMedium, icondata_include_all))]
            LuIcon::LuBatteryMedium => LU_BATTERY_MEDIUM,
            #[cfg(any(LuBatteryWarning, icondata_include_all))]
            LuIcon::LuBatteryWarning => LU_BATTERY_WARNING,
            #[cfg(any(LuBeaker, icondata_include_all))]
            LuIcon::LuBeaker => LU_BEAKER,
            #[cfg(any(LuBean, icondata_include_all))]
            LuIcon::LuBean => LU_BEAN,
            #[cfg(any(LuBeanOff, icondata_include_all))]
            LuIcon::LuBeanOff => LU_BEAN_OFF,
            #[cfg(any(LuBed, icondata_include_all))]
            LuIcon::LuBed => LU_BED,
            #[cfg(any(LuBedDouble, icondata_include_all))]
            LuIcon::LuBedDouble => LU_BED_DOUBLE,
            #[cfg(any(LuBedSingle, icondata_include_all))]
            LuIcon::LuBedSingle => LU_BED_SINGLE,
            #[cfg(any(LuBeef, icondata_include_all))]
            LuIcon::LuBeef => LU_BEEF,
            #[cfg(any(LuBeer, icondata_include_all))]
            LuIcon::LuBeer => LU_BEER,
            #[cfg(any(LuBell, icondata_include_all))]
            LuIcon::LuBell => LU_BELL,
            #[cfg(any(LuBellDot, icondata_include_all))]
            LuIcon::LuBellDot => LU_BELL_DOT,
            #[cfg(any(LuBellMinus, icondata_include_all))]
            LuIcon::LuBellMinus => LU_BELL_MINUS,
            #[cfg(any(LuBellOff, icondata_include_all))]
            LuIcon::LuBellOff => LU_BELL_OFF,
            #[cfg(any(LuBellPlus, icondata_include_all))]
            LuIcon::LuBellPlus => LU_BELL_PLUS,
            #[cfg(any(LuBellRing, icondata_include_all))]
            LuIcon::LuBellRing => LU_BELL_RING,
            #[cfg(any(LuBike, icondata_include_all))]
            LuIcon::LuBike => LU_BIKE,
            #[cfg(any(LuBinary, icondata_include_all))]
            LuIcon::LuBinary => LU_BINARY,
            #[cfg(any(LuBiohazard, icondata_include_all))]
            LuIcon::LuBiohazard => LU_BIOHAZARD,
            #[cfg(any(LuBird, icondata_include_all))]
            LuIcon::LuBird => LU_BIRD,
            #[cfg(any(LuBitcoin, icondata_include_all))]
            LuIcon::LuBitcoin => LU_BITCOIN,
            #[cfg(any(LuBlinds, icondata_include_all))]
            LuIcon::LuBlinds => LU_BLINDS,
            #[cfg(any(LuBlocks, icondata_include_all))]
            LuIcon::LuBlocks => LU_BLOCKS,
            #[cfg(any(LuBluetooth, icondata_include_all))]
            LuIcon::LuBluetooth => LU_BLUETOOTH,
            #[cfg(any(LuBluetoothConnected, icondata_include_all))]
            LuIcon::LuBluetoothConnected => LU_BLUETOOTH_CONNECTED,
            #[cfg(any(LuBluetoothOff, icondata_include_all))]
            LuIcon::LuBluetoothOff => LU_BLUETOOTH_OFF,
            #[cfg(any(LuBluetoothSearching, icondata_include_all))]
            LuIcon::LuBluetoothSearching => LU_BLUETOOTH_SEARCHING,
            #[cfg(any(LuBold, icondata_include_all))]
            LuIcon::LuBold => LU_BOLD,
            #[cfg(any(LuBomb, icondata_include_all))]
            LuIcon::LuBomb => LU_BOMB,
            #[cfg(any(LuBone, icondata_include_all))]
            LuIcon::LuBone => LU_BONE,
            #[cfg(any(LuBook, icondata_include_all))]
            LuIcon::LuBook => LU_BOOK,
            #[cfg(any(LuBookCopy, icondata_include_all))]
            LuIcon::LuBookCopy => LU_BOOK_COPY,
            #[cfg(any(LuBookDown, icondata_include_all))]
            LuIcon::LuBookDown => LU_BOOK_DOWN,
            #[cfg(any(LuBookKey, icondata_include_all))]
            LuIcon::LuBookKey => LU_BOOK_KEY,
            #[cfg(any(LuBookLock, icondata_include_all))]
            LuIcon::LuBookLock => LU_BOOK_LOCK,
            #[cfg(any(LuBookMarked, icondata_include_all))]
            LuIcon::LuBookMarked => LU_BOOK_MARKED,
            #[cfg(any(LuBookMinus, icondata_include_all))]
            LuIcon::LuBookMinus => LU_BOOK_MINUS,
            #[cfg(any(LuBookOpen, icondata_include_all))]
            LuIcon::LuBookOpen => LU_BOOK_OPEN,
            #[cfg(any(LuBookOpenCheck, icondata_include_all))]
            LuIcon::LuBookOpenCheck => LU_BOOK_OPEN_CHECK,
            #[cfg(any(LuBookPlus, icondata_include_all))]
            LuIcon::LuBookPlus => LU_BOOK_PLUS,
            #[cfg(any(LuBookTemplate, icondata_include_all))]
            LuIcon::LuBookTemplate => LU_BOOK_TEMPLATE,
            #[cfg(any(LuBookUp, icondata_include_all))]
            LuIcon::LuBookUp => LU_BOOK_UP,
            #[cfg(any(LuBookUp2, icondata_include_all))]
            LuIcon::LuBookUp2 => LU_BOOK_UP2,
            #[cfg(any(LuBookX, icondata_include_all))]
            LuIcon::LuBookX => LU_BOOK_X,
            #[cfg(any(LuBookmark, icondata_include_all))]
            LuIcon::LuBookmark => LU_BOOKMARK,
            #[cfg(any(LuBookmarkMinus, icondata_include_all))]
            LuIcon::LuBookmarkMinus => LU_BOOKMARK_MINUS,
            #[cfg(any(LuBookmarkPlus, icondata_include_all))]
            LuIcon::LuBookmarkPlus => LU_BOOKMARK_PLUS,
            #[cfg(any(LuBoomBox, icondata_include_all))]
            LuIcon::LuBoomBox => LU_BOOM_BOX,
            #[cfg(any(LuBot, icondata_include_all))]
            LuIcon::LuBot => LU_BOT,
            #[cfg(any(LuBox, icondata_include_all))]
            LuIcon::LuBox => LU_BOX,
            #[cfg(any(LuBoxSelect, icondata_include_all))]
            LuIcon::LuBoxSelect => LU_BOX_SELECT,
            #[cfg(any(LuBoxes, icondata_include_all))]
            LuIcon::LuBoxes => LU_BOXES,
            #[cfg(any(LuBraces, icondata_include_all))]
            LuIcon::LuBraces => LU_BRACES,
            #[cfg(any(LuBrackets, icondata_include_all))]
            LuIcon::LuBrackets => LU_BRACKETS,
            #[cfg(any(LuBrain, icondata_include_all))]
            LuIcon::LuBrain => LU_BRAIN,
            #[cfg(any(LuBrainCircuit, icondata_include_all))]
            LuIcon::LuBrainCircuit => LU_BRAIN_CIRCUIT,
            #[cfg(any(LuBrainCog, icondata_include_all))]
            LuIcon::LuBrainCog => LU_BRAIN_COG,
            #[cfg(any(LuBriefcase, icondata_include_all))]
            LuIcon::LuBriefcase => LU_BRIEFCASE,
            #[cfg(any(LuBringToFront, icondata_include_all))]
            LuIcon::LuBringToFront => LU_BRING_TO_FRONT,
            #[cfg(any(LuBrush, icondata_include_all))]
            LuIcon::LuBrush => LU_BRUSH,
            #[cfg(any(LuBug, icondata_include_all))]
            LuIcon::LuBug => LU_BUG,
            #[cfg(any(LuBuilding, icondata_include_all))]
            LuIcon::LuBuilding => LU_BUILDING,
            #[cfg(any(LuBuilding2, icondata_include_all))]
            LuIcon::LuBuilding2 => LU_BUILDING2,
            #[cfg(any(LuBus, icondata_include_all))]
            LuIcon::LuBus => LU_BUS,
            #[cfg(any(LuBusFront, icondata_include_all))]
            LuIcon::LuBusFront => LU_BUS_FRONT,
            #[cfg(any(LuCable, icondata_include_all))]
            LuIcon::LuCable => LU_CABLE,
            #[cfg(any(LuCableCar, icondata_include_all))]
            LuIcon::LuCableCar => LU_CABLE_CAR,
            #[cfg(any(LuCake, icondata_include_all))]
            LuIcon::LuCake => LU_CAKE,
            #[cfg(any(LuCakeSlice, icondata_include_all))]
            LuIcon::LuCakeSlice => LU_CAKE_SLICE,
            #[cfg(any(LuCalculator, icondata_include_all))]
            LuIcon::LuCalculator => LU_CALCULATOR,
            #[cfg(any(LuCalendar, icondata_include_all))]
            LuIcon::LuCalendar => LU_CALENDAR,
            #[cfg(any(LuCalendarCheck, icondata_include_all))]
            LuIcon::LuCalendarCheck => LU_CALENDAR_CHECK,
            #[cfg(any(LuCalendarCheck2, icondata_include_all))]
            LuIcon::LuCalendarCheck2 => LU_CALENDAR_CHECK2,
            #[cfg(any(LuCalendarClock, icondata_include_all))]
            LuIcon::LuCalendarClock => LU_CALENDAR_CLOCK,
            #[cfg(any(LuCalendarDays, icondata_include_all))]
            LuIcon::LuCalendarDays => LU_CALENDAR_DAYS,
            #[cfg(any(LuCalendarHeart, icondata_include_all))]
            LuIcon::LuCalendarHeart => LU_CALENDAR_HEART,
            #[cfg(any(LuCalendarMinus, icondata_include_all))]
            LuIcon::LuCalendarMinus => LU_CALENDAR_MINUS,
            #[cfg(any(LuCalendarOff, icondata_include_all))]
            LuIcon::LuCalendarOff => LU_CALENDAR_OFF,
            #[cfg(any(LuCalendarPlus, icondata_include_all))]
            LuIcon::LuCalendarPlus => LU_CALENDAR_PLUS,
            #[cfg(any(LuCalendarRange, icondata_include_all))]
            LuIcon::LuCalendarRange => LU_CALENDAR_RANGE,
            #[cfg(any(LuCalendarSearch, icondata_include_all))]
            LuIcon::LuCalendarSearch => LU_CALENDAR_SEARCH,
            #[cfg(any(LuCalendarX, icondata_include_all))]
            LuIcon::LuCalendarX => LU_CALENDAR_X,
            #[cfg(any(LuCalendarX2, icondata_include_all))]
            LuIcon::LuCalendarX2 => LU_CALENDAR_X2,
            #[cfg(any(LuCamera, icondata_include_all))]
            LuIcon::LuCamera => LU_CAMERA,
            #[cfg(any(LuCameraOff, icondata_include_all))]
            LuIcon::LuCameraOff => LU_CAMERA_OFF,
            #[cfg(any(LuCandlestickChart, icondata_include_all))]
            LuIcon::LuCandlestickChart => LU_CANDLESTICK_CHART,
            #[cfg(any(LuCandy, icondata_include_all))]
            LuIcon::LuCandy => LU_CANDY,
            #[cfg(any(LuCandyCane, icondata_include_all))]
            LuIcon::LuCandyCane => LU_CANDY_CANE,
            #[cfg(any(LuCandyOff, icondata_include_all))]
            LuIcon::LuCandyOff => LU_CANDY_OFF,
            #[cfg(any(LuCar, icondata_include_all))]
            LuIcon::LuCar => LU_CAR,
            #[cfg(any(LuCarFront, icondata_include_all))]
            LuIcon::LuCarFront => LU_CAR_FRONT,
            #[cfg(any(LuCarTaxiFront, icondata_include_all))]
            LuIcon::LuCarTaxiFront => LU_CAR_TAXI_FRONT,
            #[cfg(any(LuCarrot, icondata_include_all))]
            LuIcon::LuCarrot => LU_CARROT,
            #[cfg(any(LuCaseLower, icondata_include_all))]
            LuIcon::LuCaseLower => LU_CASE_LOWER,
            #[cfg(any(LuCaseSensitive, icondata_include_all))]
            LuIcon::LuCaseSensitive => LU_CASE_SENSITIVE,
            #[cfg(any(LuCaseUpper, icondata_include_all))]
            LuIcon::LuCaseUpper => LU_CASE_UPPER,
            #[cfg(any(LuCassetteTape, icondata_include_all))]
            LuIcon::LuCassetteTape => LU_CASSETTE_TAPE,
            #[cfg(any(LuCast, icondata_include_all))]
            LuIcon::LuCast => LU_CAST,
            #[cfg(any(LuCastle, icondata_include_all))]
            LuIcon::LuCastle => LU_CASTLE,
            #[cfg(any(LuCat, icondata_include_all))]
            LuIcon::LuCat => LU_CAT,
            #[cfg(any(LuCheck, icondata_include_all))]
            LuIcon::LuCheck => LU_CHECK,
            #[cfg(any(LuCheckCheck, icondata_include_all))]
            LuIcon::LuCheckCheck => LU_CHECK_CHECK,
            #[cfg(any(LuCheckCircle, icondata_include_all))]
            LuIcon::LuCheckCircle => LU_CHECK_CIRCLE,
            #[cfg(any(LuCheckCircle2, icondata_include_all))]
            LuIcon::LuCheckCircle2 => LU_CHECK_CIRCLE2,
            #[cfg(any(LuCheckSquare, icondata_include_all))]
            LuIcon::LuCheckSquare => LU_CHECK_SQUARE,
            #[cfg(any(LuChefHat, icondata_include_all))]
            LuIcon::LuChefHat => LU_CHEF_HAT,
            #[cfg(any(LuCherry, icondata_include_all))]
            LuIcon::LuCherry => LU_CHERRY,
            #[cfg(any(LuChevronDown, icondata_include_all))]
            LuIcon::LuChevronDown => LU_CHEVRON_DOWN,
            #[cfg(any(LuChevronDownCircle, icondata_include_all))]
            LuIcon::LuChevronDownCircle => LU_CHEVRON_DOWN_CIRCLE,
            #[cfg(any(LuChevronDownSquare, icondata_include_all))]
            LuIcon::LuChevronDownSquare => LU_CHEVRON_DOWN_SQUARE,
            #[cfg(any(LuChevronFirst, icondata_include_all))]
            LuIcon::LuChevronFirst => LU_CHEVRON_FIRST,
            #[cfg(any(LuChevronLast, icondata_include_all))]
            LuIcon::LuChevronLast => LU_CHEVRON_LAST,
            #[cfg(any(LuChevronLeft, icondata_include_all))]
            LuIcon::LuChevronLeft => LU_CHEVRON_LEFT,
            #[cfg(any(LuChevronLeftCircle, icondata_include_all))]
            LuIcon::LuChevronLeftCircle => LU_CHEVRON_LEFT_CIRCLE,
            #[cfg(any(LuChevronLeftSquare, icondata_include_all))]
            LuIcon::LuChevronLeftSquare => LU_CHEVRON_LEFT_SQUARE,
            #[cfg(any(LuChevronRight, icondata_include_all))]
            LuIcon::LuChevronRight => LU_CHEVRON_RIGHT,
            #[cfg(any(LuChevronRightCircle, icondata_include_all))]
            LuIcon::LuChevronRightCircle => LU_CHEVRON_RIGHT_CIRCLE,
            #[cfg(any(LuChevronRightSquare, icondata_include_all))]
            LuIcon::LuChevronRightSquare => LU_CHEVRON_RIGHT_SQUARE,
            #[cfg(any(LuChevronUp, icondata_include_all))]
            LuIcon::LuChevronUp => LU_CHEVRON_UP,
            #[cfg(any(LuChevronUpCircle, icondata_include_all))]
            LuIcon::LuChevronUpCircle => LU_CHEVRON_UP_CIRCLE,
            #[cfg(any(LuChevronUpSquare, icondata_include_all))]
            LuIcon::LuChevronUpSquare => LU_CHEVRON_UP_SQUARE,
            #[cfg(any(LuChevronsDown, icondata_include_all))]
            LuIcon::LuChevronsDown => LU_CHEVRONS_DOWN,
            #[cfg(any(LuChevronsDownUp, icondata_include_all))]
            LuIcon::LuChevronsDownUp => LU_CHEVRONS_DOWN_UP,
            #[cfg(any(LuChevronsLeft, icondata_include_all))]
            LuIcon::LuChevronsLeft => LU_CHEVRONS_LEFT,
            #[cfg(any(LuChevronsLeftRight, icondata_include_all))]
            LuIcon::LuChevronsLeftRight => LU_CHEVRONS_LEFT_RIGHT,
            #[cfg(any(LuChevronsRight, icondata_include_all))]
            LuIcon::LuChevronsRight => LU_CHEVRONS_RIGHT,
            #[cfg(any(LuChevronsRightLeft, icondata_include_all))]
            LuIcon::LuChevronsRightLeft => LU_CHEVRONS_RIGHT_LEFT,
            #[cfg(any(LuChevronsUp, icondata_include_all))]
            LuIcon::LuChevronsUp => LU_CHEVRONS_UP,
            #[cfg(any(LuChevronsUpDown, icondata_include_all))]
            LuIcon::LuChevronsUpDown => LU_CHEVRONS_UP_DOWN,
            #[cfg(any(LuChrome, icondata_include_all))]
            LuIcon::LuChrome => LU_CHROME,
            #[cfg(any(LuChurch, icondata_include_all))]
            LuIcon::LuChurch => LU_CHURCH,
            #[cfg(any(LuCigarette, icondata_include_all))]
            LuIcon::LuCigarette => LU_CIGARETTE,
            #[cfg(any(LuCigaretteOff, icondata_include_all))]
            LuIcon::LuCigaretteOff => LU_CIGARETTE_OFF,
            #[cfg(any(LuCircle, icondata_include_all))]
            LuIcon::LuCircle => LU_CIRCLE,
            #[cfg(any(LuCircleDashed, icondata_include_all))]
            LuIcon::LuCircleDashed => LU_CIRCLE_DASHED,
            #[cfg(any(LuCircleDollarSign, icondata_include_all))]
            LuIcon::LuCircleDollarSign => LU_CIRCLE_DOLLAR_SIGN,
            #[cfg(any(LuCircleDot, icondata_include_all))]
            LuIcon::LuCircleDot => LU_CIRCLE_DOT,
            #[cfg(any(LuCircleDotDashed, icondata_include_all))]
            LuIcon::LuCircleDotDashed => LU_CIRCLE_DOT_DASHED,
            #[cfg(any(LuCircleEllipsis, icondata_include_all))]
            LuIcon::LuCircleEllipsis => LU_CIRCLE_ELLIPSIS,
            #[cfg(any(LuCircleEqual, icondata_include_all))]
            LuIcon::LuCircleEqual => LU_CIRCLE_EQUAL,
            #[cfg(any(LuCircleOff, icondata_include_all))]
            LuIcon::LuCircleOff => LU_CIRCLE_OFF,
            #[cfg(any(LuCircleSlash, icondata_include_all))]
            LuIcon::LuCircleSlash => LU_CIRCLE_SLASH,
            #[cfg(any(LuCircleSlash2, icondata_include_all))]
            LuIcon::LuCircleSlash2 => LU_CIRCLE_SLASH2,
            #[cfg(any(LuCircuitBoard, icondata_include_all))]
            LuIcon::LuCircuitBoard => LU_CIRCUIT_BOARD,
            #[cfg(any(LuCitrus, icondata_include_all))]
            LuIcon::LuCitrus => LU_CITRUS,
            #[cfg(any(LuClapperboard, icondata_include_all))]
            LuIcon::LuClapperboard => LU_CLAPPERBOARD,
            #[cfg(any(LuClipboard, icondata_include_all))]
            LuIcon::LuClipboard => LU_CLIPBOARD,
            #[cfg(any(LuClipboardCheck, icondata_include_all))]
            LuIcon::LuClipboardCheck => LU_CLIPBOARD_CHECK,
            #[cfg(any(LuClipboardCopy, icondata_include_all))]
            LuIcon::LuClipboardCopy => LU_CLIPBOARD_COPY,
            #[cfg(any(LuClipboardEdit, icondata_include_all))]
            LuIcon::LuClipboardEdit => LU_CLIPBOARD_EDIT,
            #[cfg(any(LuClipboardList, icondata_include_all))]
            LuIcon::LuClipboardList => LU_CLIPBOARD_LIST,
            #[cfg(any(LuClipboardPaste, icondata_include_all))]
            LuIcon::LuClipboardPaste => LU_CLIPBOARD_PASTE,
            #[cfg(any(LuClipboardSignature, icondata_include_all))]
            LuIcon::LuClipboardSignature => LU_CLIPBOARD_SIGNATURE,
            #[cfg(any(LuClipboardType, icondata_include_all))]
            LuIcon::LuClipboardType => LU_CLIPBOARD_TYPE,
            #[cfg(any(LuClipboardX, icondata_include_all))]
            LuIcon::LuClipboardX => LU_CLIPBOARD_X,
            #[cfg(any(LuClock, icondata_include_all))]
            LuIcon::LuClock => LU_CLOCK,
            #[cfg(any(LuClock1, icondata_include_all))]
            LuIcon::LuClock1 => LU_CLOCK1,
            #[cfg(any(LuClock10, icondata_include_all))]
            LuIcon::LuClock10 => LU_CLOCK10,
            #[cfg(any(LuClock11, icondata_include_all))]
            LuIcon::LuClock11 => LU_CLOCK11,
            #[cfg(any(LuClock12, icondata_include_all))]
            LuIcon::LuClock12 => LU_CLOCK12,
            #[cfg(any(LuClock2, icondata_include_all))]
            LuIcon::LuClock2 => LU_CLOCK2,
            #[cfg(any(LuClock3, icondata_include_all))]
            LuIcon::LuClock3 => LU_CLOCK3,
            #[cfg(any(LuClock4, icondata_include_all))]
            LuIcon::LuClock4 => LU_CLOCK4,
            #[cfg(any(LuClock5, icondata_include_all))]
            LuIcon::LuClock5 => LU_CLOCK5,
            #[cfg(any(LuClock6, icondata_include_all))]
            LuIcon::LuClock6 => LU_CLOCK6,
            #[cfg(any(LuClock7, icondata_include_all))]
            LuIcon::LuClock7 => LU_CLOCK7,
            #[cfg(any(LuClock8, icondata_include_all))]
            LuIcon::LuClock8 => LU_CLOCK8,
            #[cfg(any(LuClock9, icondata_include_all))]
            LuIcon::LuClock9 => LU_CLOCK9,
            #[cfg(any(LuCloud, icondata_include_all))]
            LuIcon::LuCloud => LU_CLOUD,
            #[cfg(any(LuCloudCog, icondata_include_all))]
            LuIcon::LuCloudCog => LU_CLOUD_COG,
            #[cfg(any(LuCloudDrizzle, icondata_include_all))]
            LuIcon::LuCloudDrizzle => LU_CLOUD_DRIZZLE,
            #[cfg(any(LuCloudFog, icondata_include_all))]
            LuIcon::LuCloudFog => LU_CLOUD_FOG,
            #[cfg(any(LuCloudHail, icondata_include_all))]
            LuIcon::LuCloudHail => LU_CLOUD_HAIL,
            #[cfg(any(LuCloudLightning, icondata_include_all))]
            LuIcon::LuCloudLightning => LU_CLOUD_LIGHTNING,
            #[cfg(any(LuCloudMoon, icondata_include_all))]
            LuIcon::LuCloudMoon => LU_CLOUD_MOON,
            #[cfg(any(LuCloudMoonRain, icondata_include_all))]
            LuIcon::LuCloudMoonRain => LU_CLOUD_MOON_RAIN,
            #[cfg(any(LuCloudOff, icondata_include_all))]
            LuIcon::LuCloudOff => LU_CLOUD_OFF,
            #[cfg(any(LuCloudRain, icondata_include_all))]
            LuIcon::LuCloudRain => LU_CLOUD_RAIN,
            #[cfg(any(LuCloudRainWind, icondata_include_all))]
            LuIcon::LuCloudRainWind => LU_CLOUD_RAIN_WIND,
            #[cfg(any(LuCloudSnow, icondata_include_all))]
            LuIcon::LuCloudSnow => LU_CLOUD_SNOW,
            #[cfg(any(LuCloudSun, icondata_include_all))]
            LuIcon::LuCloudSun => LU_CLOUD_SUN,
            #[cfg(any(LuCloudSunRain, icondata_include_all))]
            LuIcon::LuCloudSunRain => LU_CLOUD_SUN_RAIN,
            #[cfg(any(LuCloudy, icondata_include_all))]
            LuIcon::LuCloudy => LU_CLOUDY,
            #[cfg(any(LuClover, icondata_include_all))]
            LuIcon::LuClover => LU_CLOVER,
            #[cfg(any(LuClub, icondata_include_all))]
            LuIcon::LuClub => LU_CLUB,
            #[cfg(any(LuCode, icondata_include_all))]
            LuIcon::LuCode => LU_CODE,
            #[cfg(any(LuCode2, icondata_include_all))]
            LuIcon::LuCode2 => LU_CODE2,
            #[cfg(any(LuCodepen, icondata_include_all))]
            LuIcon::LuCodepen => LU_CODEPEN,
            #[cfg(any(LuCodesandbox, icondata_include_all))]
            LuIcon::LuCodesandbox => LU_CODESANDBOX,
            #[cfg(any(LuCoffee, icondata_include_all))]
            LuIcon::LuCoffee => LU_COFFEE,
            #[cfg(any(LuCog, icondata_include_all))]
            LuIcon::LuCog => LU_COG,
            #[cfg(any(LuCoins, icondata_include_all))]
            LuIcon::LuCoins => LU_COINS,
            #[cfg(any(LuColumns, icondata_include_all))]
            LuIcon::LuColumns => LU_COLUMNS,
            #[cfg(any(LuCombine, icondata_include_all))]
            LuIcon::LuCombine => LU_COMBINE,
            #[cfg(any(LuCommand, icondata_include_all))]
            LuIcon::LuCommand => LU_COMMAND,
            #[cfg(any(LuCompass, icondata_include_all))]
            LuIcon::LuCompass => LU_COMPASS,
            #[cfg(any(LuComponent, icondata_include_all))]
            LuIcon::LuComponent => LU_COMPONENT,
            #[cfg(any(LuComputer, icondata_include_all))]
            LuIcon::LuComputer => LU_COMPUTER,
            #[cfg(any(LuConciergeBell, icondata_include_all))]
            LuIcon::LuConciergeBell => LU_CONCIERGE_BELL,
            #[cfg(any(LuConstruction, icondata_include_all))]
            LuIcon::LuConstruction => LU_CONSTRUCTION,
            #[cfg(any(LuContact, icondata_include_all))]
            LuIcon::LuContact => LU_CONTACT,
            #[cfg(any(LuContact2, icondata_include_all))]
            LuIcon::LuContact2 => LU_CONTACT2,
            #[cfg(any(LuContainer, icondata_include_all))]
            LuIcon::LuContainer => LU_CONTAINER,
            #[cfg(any(LuContrast, icondata_include_all))]
            LuIcon::LuContrast => LU_CONTRAST,
            #[cfg(any(LuCookie, icondata_include_all))]
            LuIcon::LuCookie => LU_COOKIE,
            #[cfg(any(LuCopy, icondata_include_all))]
            LuIcon::LuCopy => LU_COPY,
            #[cfg(any(LuCopyCheck, icondata_include_all))]
            LuIcon::LuCopyCheck => LU_COPY_CHECK,
            #[cfg(any(LuCopyMinus, icondata_include_all))]
            LuIcon::LuCopyMinus => LU_COPY_MINUS,
            #[cfg(any(LuCopyPlus, icondata_include_all))]
            LuIcon::LuCopyPlus => LU_COPY_PLUS,
            #[cfg(any(LuCopySlash, icondata_include_all))]
            LuIcon::LuCopySlash => LU_COPY_SLASH,
            #[cfg(any(LuCopyX, icondata_include_all))]
            LuIcon::LuCopyX => LU_COPY_X,
            #[cfg(any(LuCopyleft, icondata_include_all))]
            LuIcon::LuCopyleft => LU_COPYLEFT,
            #[cfg(any(LuCopyright, icondata_include_all))]
            LuIcon::LuCopyright => LU_COPYRIGHT,
            #[cfg(any(LuCornerDownLeft, icondata_include_all))]
            LuIcon::LuCornerDownLeft => LU_CORNER_DOWN_LEFT,
            #[cfg(any(LuCornerDownRight, icondata_include_all))]
            LuIcon::LuCornerDownRight => LU_CORNER_DOWN_RIGHT,
            #[cfg(any(LuCornerLeftDown, icondata_include_all))]
            LuIcon::LuCornerLeftDown => LU_CORNER_LEFT_DOWN,
            #[cfg(any(LuCornerLeftUp, icondata_include_all))]
            LuIcon::LuCornerLeftUp => LU_CORNER_LEFT_UP,
            #[cfg(any(LuCornerRightDown, icondata_include_all))]
            LuIcon::LuCornerRightDown => LU_CORNER_RIGHT_DOWN,
            #[cfg(any(LuCornerRightUp, icondata_include_all))]
            LuIcon::LuCornerRightUp => LU_CORNER_RIGHT_UP,
            #[cfg(any(LuCornerUpLeft, icondata_include_all))]
            LuIcon::LuCornerUpLeft => LU_CORNER_UP_LEFT,
            #[cfg(any(LuCornerUpRight, icondata_include_all))]
            LuIcon::LuCornerUpRight => LU_CORNER_UP_RIGHT,
            #[cfg(any(LuCpu, icondata_include_all))]
            LuIcon::LuCpu => LU_CPU,
            #[cfg(any(LuCreativeCommons, icondata_include_all))]
            LuIcon::LuCreativeCommons => LU_CREATIVE_COMMONS,
            #[cfg(any(LuCreditCard, icondata_include_all))]
            LuIcon::LuCreditCard => LU_CREDIT_CARD,
            #[cfg(any(LuCroissant, icondata_include_all))]
            LuIcon::LuCroissant => LU_CROISSANT,
            #[cfg(any(LuCrop, icondata_include_all))]
            LuIcon::LuCrop => LU_CROP,
            #[cfg(any(LuCross, icondata_include_all))]
            LuIcon::LuCross => LU_CROSS,
            #[cfg(any(LuCrosshair, icondata_include_all))]
            LuIcon::LuCrosshair => LU_CROSSHAIR,
            #[cfg(any(LuCrown, icondata_include_all))]
            LuIcon::LuCrown => LU_CROWN,
            #[cfg(any(LuCupSoda, icondata_include_all))]
            LuIcon::LuCupSoda => LU_CUP_SODA,
            #[cfg(any(LuCurrency, icondata_include_all))]
            LuIcon::LuCurrency => LU_CURRENCY,
            #[cfg(any(LuDatabase, icondata_include_all))]
            LuIcon::LuDatabase => LU_DATABASE,
            #[cfg(any(LuDatabaseBackup, icondata_include_all))]
            LuIcon::LuDatabaseBackup => LU_DATABASE_BACKUP,
            #[cfg(any(LuDelete, icondata_include_all))]
            LuIcon::LuDelete => LU_DELETE,
            #[cfg(any(LuDessert, icondata_include_all))]
            LuIcon::LuDessert => LU_DESSERT,
            #[cfg(any(LuDiamond, icondata_include_all))]
            LuIcon::LuDiamond => LU_DIAMOND,
            #[cfg(any(LuDice1, icondata_include_all))]
            LuIcon::LuDice1 => LU_DICE1,
            #[cfg(any(LuDice2, icondata_include_all))]
            LuIcon::LuDice2 => LU_DICE2,
            #[cfg(any(LuDice3, icondata_include_all))]
            LuIcon::LuDice3 => LU_DICE3,
            #[cfg(any(LuDice4, icondata_include_all))]
            LuIcon::LuDice4 => LU_DICE4,
            #[cfg(any(LuDice5, icondata_include_all))]
            LuIcon::LuDice5 => LU_DICE5,
            #[cfg(any(LuDice6, icondata_include_all))]
            LuIcon::LuDice6 => LU_DICE6,
            #[cfg(any(LuDices, icondata_include_all))]
            LuIcon::LuDices => LU_DICES,
            #[cfg(any(LuDiff, icondata_include_all))]
            LuIcon::LuDiff => LU_DIFF,
            #[cfg(any(LuDisc, icondata_include_all))]
            LuIcon::LuDisc => LU_DISC,
            #[cfg(any(LuDisc2, icondata_include_all))]
            LuIcon::LuDisc2 => LU_DISC2,
            #[cfg(any(LuDisc3, icondata_include_all))]
            LuIcon::LuDisc3 => LU_DISC3,
            #[cfg(any(LuDivide, icondata_include_all))]
            LuIcon::LuDivide => LU_DIVIDE,
            #[cfg(any(LuDivideCircle, icondata_include_all))]
            LuIcon::LuDivideCircle => LU_DIVIDE_CIRCLE,
            #[cfg(any(LuDivideSquare, icondata_include_all))]
            LuIcon::LuDivideSquare => LU_DIVIDE_SQUARE,
            #[cfg(any(LuDna, icondata_include_all))]
            LuIcon::LuDna => LU_DNA,
            #[cfg(any(LuDnaOff, icondata_include_all))]
            LuIcon::LuDnaOff => LU_DNA_OFF,
            #[cfg(any(LuDog, icondata_include_all))]
            LuIcon::LuDog => LU_DOG,
            #[cfg(any(LuDollarSign, icondata_include_all))]
            LuIcon::LuDollarSign => LU_DOLLAR_SIGN,
            #[cfg(any(LuDonut, icondata_include_all))]
            LuIcon::LuDonut => LU_DONUT,
            #[cfg(any(LuDoorClosed, icondata_include_all))]
            LuIcon::LuDoorClosed => LU_DOOR_CLOSED,
            #[cfg(any(LuDoorOpen, icondata_include_all))]
            LuIcon::LuDoorOpen => LU_DOOR_OPEN,
            #[cfg(any(LuDot, icondata_include_all))]
            LuIcon::LuDot => LU_DOT,
            #[cfg(any(LuDownload, icondata_include_all))]
            LuIcon::LuDownload => LU_DOWNLOAD,
            #[cfg(any(LuDownloadCloud, icondata_include_all))]
            LuIcon::LuDownloadCloud => LU_DOWNLOAD_CLOUD,
            #[cfg(any(LuDribbble, icondata_include_all))]
            LuIcon::LuDribbble => LU_DRIBBBLE,
            #[cfg(any(LuDroplet, icondata_include_all))]
            LuIcon::LuDroplet => LU_DROPLET,
            #[cfg(any(LuDroplets, icondata_include_all))]
            LuIcon::LuDroplets => LU_DROPLETS,
            #[cfg(any(LuDrumstick, icondata_include_all))]
            LuIcon::LuDrumstick => LU_DRUMSTICK,
            #[cfg(any(LuDumbbell, icondata_include_all))]
            LuIcon::LuDumbbell => LU_DUMBBELL,
            #[cfg(any(LuEar, icondata_include_all))]
            LuIcon::LuEar => LU_EAR,
            #[cfg(any(LuEarOff, icondata_include_all))]
            LuIcon::LuEarOff => LU_EAR_OFF,
            #[cfg(any(LuEgg, icondata_include_all))]
            LuIcon::LuEgg => LU_EGG,
            #[cfg(any(LuEggFried, icondata_include_all))]
            LuIcon::LuEggFried => LU_EGG_FRIED,
            #[cfg(any(LuEggOff, icondata_include_all))]
            LuIcon::LuEggOff => LU_EGG_OFF,
            #[cfg(any(LuEqual, icondata_include_all))]
            LuIcon::LuEqual => LU_EQUAL,
            #[cfg(any(LuEqualNot, icondata_include_all))]
            LuIcon::LuEqualNot => LU_EQUAL_NOT,
            #[cfg(any(LuEraser, icondata_include_all))]
            LuIcon::LuEraser => LU_ERASER,
            #[cfg(any(LuEuro, icondata_include_all))]
            LuIcon::LuEuro => LU_EURO,
            #[cfg(any(LuExpand, icondata_include_all))]
            LuIcon::LuExpand => LU_EXPAND,
            #[cfg(any(LuExternalLink, icondata_include_all))]
            LuIcon::LuExternalLink => LU_EXTERNAL_LINK,
            #[cfg(any(LuEye, icondata_include_all))]
            LuIcon::LuEye => LU_EYE,
            #[cfg(any(LuEyeOff, icondata_include_all))]
            LuIcon::LuEyeOff => LU_EYE_OFF,
            #[cfg(any(LuFacebook, icondata_include_all))]
            LuIcon::LuFacebook => LU_FACEBOOK,
            #[cfg(any(LuFactory, icondata_include_all))]
            LuIcon::LuFactory => LU_FACTORY,
            #[cfg(any(LuFan, icondata_include_all))]
            LuIcon::LuFan => LU_FAN,
            #[cfg(any(LuFastForward, icondata_include_all))]
            LuIcon::LuFastForward => LU_FAST_FORWARD,
            #[cfg(any(LuFeather, icondata_include_all))]
            LuIcon::LuFeather => LU_FEATHER,
            #[cfg(any(LuFerrisWheel, icondata_include_all))]
            LuIcon::LuFerrisWheel => LU_FERRIS_WHEEL,
            #[cfg(any(LuFigma, icondata_include_all))]
            LuIcon::LuFigma => LU_FIGMA,
            #[cfg(any(LuFile, icondata_include_all))]
            LuIcon::LuFile => LU_FILE,
            #[cfg(any(LuFileArchive, icondata_include_all))]
            LuIcon::LuFileArchive => LU_FILE_ARCHIVE,
            #[cfg(any(LuFileAudio, icondata_include_all))]
            LuIcon::LuFileAudio => LU_FILE_AUDIO,
            #[cfg(any(LuFileAudio2, icondata_include_all))]
            LuIcon::LuFileAudio2 => LU_FILE_AUDIO2,
            #[cfg(any(LuFileAxis3d, icondata_include_all))]
            LuIcon::LuFileAxis3d => LU_FILE_AXIS3D,
            #[cfg(any(LuFileBadge, icondata_include_all))]
            LuIcon::LuFileBadge => LU_FILE_BADGE,
            #[cfg(any(LuFileBadge2, icondata_include_all))]
            LuIcon::LuFileBadge2 => LU_FILE_BADGE2,
            #[cfg(any(LuFileBarChart, icondata_include_all))]
            LuIcon::LuFileBarChart => LU_FILE_BAR_CHART,
            #[cfg(any(LuFileBarChart2, icondata_include_all))]
            LuIcon::LuFileBarChart2 => LU_FILE_BAR_CHART2,
            #[cfg(any(LuFileBox, icondata_include_all))]
            LuIcon::LuFileBox => LU_FILE_BOX,
            #[cfg(any(LuFileCheck, icondata_include_all))]
            LuIcon::LuFileCheck => LU_FILE_CHECK,
            #[cfg(any(LuFileCheck2, icondata_include_all))]
            LuIcon::LuFileCheck2 => LU_FILE_CHECK2,
            #[cfg(any(LuFileClock, icondata_include_all))]
            LuIcon::LuFileClock => LU_FILE_CLOCK,
            #[cfg(any(LuFileCode, icondata_include_all))]
            LuIcon::LuFileCode => LU_FILE_CODE,
            #[cfg(any(LuFileCode2, icondata_include_all))]
            LuIcon::LuFileCode2 => LU_FILE_CODE2,
            #[cfg(any(LuFileCog, icondata_include_all))]
            LuIcon::LuFileCog => LU_FILE_COG,
            #[cfg(any(LuFileCog2, icondata_include_all))]
            LuIcon::LuFileCog2 => LU_FILE_COG2,
            #[cfg(any(LuFileDiff, icondata_include_all))]
            LuIcon::LuFileDiff => LU_FILE_DIFF,
            #[cfg(any(LuFileDigit, icondata_include_all))]
            LuIcon::LuFileDigit => LU_FILE_DIGIT,
            #[cfg(any(LuFileDown, icondata_include_all))]
            LuIcon::LuFileDown => LU_FILE_DOWN,
            #[cfg(any(LuFileEdit, icondata_include_all))]
            LuIcon::LuFileEdit => LU_FILE_EDIT,
            #[cfg(any(LuFileHeart, icondata_include_all))]
            LuIcon::LuFileHeart => LU_FILE_HEART,
            #[cfg(any(LuFileImage, icondata_include_all))]
            LuIcon::LuFileImage => LU_FILE_IMAGE,
            #[cfg(any(LuFileInput, icondata_include_all))]
            LuIcon::LuFileInput => LU_FILE_INPUT,
            #[cfg(any(LuFileJson, icondata_include_all))]
            LuIcon::LuFileJson => LU_FILE_JSON,
            #[cfg(any(LuFileJson2, icondata_include_all))]
            LuIcon::LuFileJson2 => LU_FILE_JSON2,
            #[cfg(any(LuFileKey, icondata_include_all))]
            LuIcon::LuFileKey => LU_FILE_KEY,
            #[cfg(any(LuFileKey2, icondata_include_all))]
            LuIcon::LuFileKey2 => LU_FILE_KEY2,
            #[cfg(any(LuFileLineChart, icondata_include_all))]
            LuIcon::LuFileLineChart => LU_FILE_LINE_CHART,
            #[cfg(any(LuFileLock, icondata_include_all))]
            LuIcon::LuFileLock => LU_FILE_LOCK,
            #[cfg(any(LuFileLock2, icondata_include_all))]
            LuIcon::LuFileLock2 => LU_FILE_LOCK2,
            #[cfg(any(LuFileMinus, icondata_include_all))]
            LuIcon::LuFileMinus => LU_FILE_MINUS,
            #[cfg(any(LuFileMinus2, icondata_include_all))]
            LuIcon::LuFileMinus2 => LU_FILE_MINUS2,
            #[cfg(any(LuFileOutput, icondata_include_all))]
            LuIcon::LuFileOutput => LU_FILE_OUTPUT,
            #[cfg(any(LuFilePieChart, icondata_include_all))]
            LuIcon::LuFilePieChart => LU_FILE_PIE_CHART,
            #[cfg(any(LuFilePlus, icondata_include_all))]
            LuIcon::LuFilePlus => LU_FILE_PLUS,
            #[cfg(any(LuFilePlus2, icondata_include_all))]
            LuIcon::LuFilePlus2 => LU_FILE_PLUS2,
            #[cfg(any(LuFileQuestion, icondata_include_all))]
            LuIcon::LuFileQuestion => LU_FILE_QUESTION,
            #[cfg(any(LuFileScan, icondata_include_all))]
            LuIcon::LuFileScan => LU_FILE_SCAN,
            #[cfg(any(LuFileSearch, icondata_include_all))]
            LuIcon::LuFileSearch => LU_FILE_SEARCH,
            #[cfg(any(LuFileSearch2, icondata_include_all))]
            LuIcon::LuFileSearch2 => LU_FILE_SEARCH2,
            #[cfg(any(LuFileSignature, icondata_include_all))]
            LuIcon::LuFileSignature => LU_FILE_SIGNATURE,
            #[cfg(any(LuFileSpreadsheet, icondata_include_all))]
            LuIcon::LuFileSpreadsheet => LU_FILE_SPREADSHEET,
            #[cfg(any(LuFileStack, icondata_include_all))]
            LuIcon::LuFileStack => LU_FILE_STACK,
            #[cfg(any(LuFileSymlink, icondata_include_all))]
            LuIcon::LuFileSymlink => LU_FILE_SYMLINK,
            #[cfg(any(LuFileTerminal, icondata_include_all))]
            LuIcon::LuFileTerminal => LU_FILE_TERMINAL,
            #[cfg(any(LuFileText, icondata_include_all))]
            LuIcon::LuFileText => LU_FILE_TEXT,
            #[cfg(any(LuFileType, icondata_include_all))]
            LuIcon::LuFileType => LU_FILE_TYPE,
            #[cfg(any(LuFileType2, icondata_include_all))]
            LuIcon::LuFileType2 => LU_FILE_TYPE2,
            #[cfg(any(LuFileUp, icondata_include_all))]
            LuIcon::LuFileUp => LU_FILE_UP,
            #[cfg(any(LuFileVideo, icondata_include_all))]
            LuIcon::LuFileVideo => LU_FILE_VIDEO,
            #[cfg(any(LuFileVideo2, icondata_include_all))]
            LuIcon::LuFileVideo2 => LU_FILE_VIDEO2,
            #[cfg(any(LuFileVolume, icondata_include_all))]
            LuIcon::LuFileVolume => LU_FILE_VOLUME,
            #[cfg(any(LuFileVolume2, icondata_include_all))]
            LuIcon::LuFileVolume2 => LU_FILE_VOLUME2,
            #[cfg(any(LuFileWarning, icondata_include_all))]
            LuIcon::LuFileWarning => LU_FILE_WARNING,
            #[cfg(any(LuFileX, icondata_include_all))]
            LuIcon::LuFileX => LU_FILE_X,
            #[cfg(any(LuFileX2, icondata_include_all))]
            LuIcon::LuFileX2 => LU_FILE_X2,
            #[cfg(any(LuFiles, icondata_include_all))]
            LuIcon::LuFiles => LU_FILES,
            #[cfg(any(LuFilm, icondata_include_all))]
            LuIcon::LuFilm => LU_FILM,
            #[cfg(any(LuFilter, icondata_include_all))]
            LuIcon::LuFilter => LU_FILTER,
            #[cfg(any(LuFilterX, icondata_include_all))]
            LuIcon::LuFilterX => LU_FILTER_X,
            #[cfg(any(LuFingerprint, icondata_include_all))]
            LuIcon::LuFingerprint => LU_FINGERPRINT,
            #[cfg(any(LuFish, icondata_include_all))]
            LuIcon::LuFish => LU_FISH,
            #[cfg(any(LuFishOff, icondata_include_all))]
            LuIcon::LuFishOff => LU_FISH_OFF,
            #[cfg(any(LuFishSymbol, icondata_include_all))]
            LuIcon::LuFishSymbol => LU_FISH_SYMBOL,
            #[cfg(any(LuFlag, icondata_include_all))]
            LuIcon::LuFlag => LU_FLAG,
            #[cfg(any(LuFlagOff, icondata_include_all))]
            LuIcon::LuFlagOff => LU_FLAG_OFF,
            #[cfg(any(LuFlagTriangleLeft, icondata_include_all))]
            LuIcon::LuFlagTriangleLeft => LU_FLAG_TRIANGLE_LEFT,
            #[cfg(any(LuFlagTriangleRight, icondata_include_all))]
            LuIcon::LuFlagTriangleRight => LU_FLAG_TRIANGLE_RIGHT,
            #[cfg(any(LuFlame, icondata_include_all))]
            LuIcon::LuFlame => LU_FLAME,
            #[cfg(any(LuFlashlight, icondata_include_all))]
            LuIcon::LuFlashlight => LU_FLASHLIGHT,
            #[cfg(any(LuFlashlightOff, icondata_include_all))]
            LuIcon::LuFlashlightOff => LU_FLASHLIGHT_OFF,
            #[cfg(any(LuFlaskConical, icondata_include_all))]
            LuIcon::LuFlaskConical => LU_FLASK_CONICAL,
            #[cfg(any(LuFlaskConicalOff, icondata_include_all))]
            LuIcon::LuFlaskConicalOff => LU_FLASK_CONICAL_OFF,
            #[cfg(any(LuFlaskRound, icondata_include_all))]
            LuIcon::LuFlaskRound => LU_FLASK_ROUND,
            #[cfg(any(LuFlipHorizontal, icondata_include_all))]
            LuIcon::LuFlipHorizontal => LU_FLIP_HORIZONTAL,
            #[cfg(any(LuFlipHorizontal2, icondata_include_all))]
            LuIcon::LuFlipHorizontal2 => LU_FLIP_HORIZONTAL2,
            #[cfg(any(LuFlipVertical, icondata_include_all))]
            LuIcon::LuFlipVertical => LU_FLIP_VERTICAL,
            #[cfg(any(LuFlipVertical2, icondata_include_all))]
            LuIcon::LuFlipVertical2 => LU_FLIP_VERTICAL2,
            #[cfg(any(LuFlower, icondata_include_all))]
            LuIcon::LuFlower => LU_FLOWER,
            #[cfg(any(LuFlower2, icondata_include_all))]
            LuIcon::LuFlower2 => LU_FLOWER2,
            #[cfg(any(LuFocus, icondata_include_all))]
            LuIcon::LuFocus => LU_FOCUS,
            #[cfg(any(LuFoldHorizontal, icondata_include_all))]
            LuIcon::LuFoldHorizontal => LU_FOLD_HORIZONTAL,
            #[cfg(any(LuFoldVertical, icondata_include_all))]
            LuIcon::LuFoldVertical => LU_FOLD_VERTICAL,
            #[cfg(any(LuFolder, icondata_include_all))]
            LuIcon::LuFolder => LU_FOLDER,
            #[cfg(any(LuFolderArchive, icondata_include_all))]
            LuIcon::LuFolderArchive => LU_FOLDER_ARCHIVE,
            #[cfg(any(LuFolderCheck, icondata_include_all))]
            LuIcon::LuFolderCheck => LU_FOLDER_CHECK,
            #[cfg(any(LuFolderClock, icondata_include_all))]
            LuIcon::LuFolderClock => LU_FOLDER_CLOCK,
            #[cfg(any(LuFolderClosed, icondata_include_all))]
            LuIcon::LuFolderClosed => LU_FOLDER_CLOSED,
            #[cfg(any(LuFolderCog, icondata_include_all))]
            LuIcon::LuFolderCog => LU_FOLDER_COG,
            #[cfg(any(LuFolderCog2, icondata_include_all))]
            LuIcon::LuFolderCog2 => LU_FOLDER_COG2,
            #[cfg(any(LuFolderDot, icondata_include_all))]
            LuIcon::LuFolderDot => LU_FOLDER_DOT,
            #[cfg(any(LuFolderDown, icondata_include_all))]
            LuIcon::LuFolderDown => LU_FOLDER_DOWN,
            #[cfg(any(LuFolderEdit, icondata_include_all))]
            LuIcon::LuFolderEdit => LU_FOLDER_EDIT,
            #[cfg(any(LuFolderGit, icondata_include_all))]
            LuIcon::LuFolderGit => LU_FOLDER_GIT,
            #[cfg(any(LuFolderGit2, icondata_include_all))]
            LuIcon::LuFolderGit2 => LU_FOLDER_GIT2,
            #[cfg(any(LuFolderHeart, icondata_include_all))]
            LuIcon::LuFolderHeart => LU_FOLDER_HEART,
            #[cfg(any(LuFolderInput, icondata_include_all))]
            LuIcon::LuFolderInput => LU_FOLDER_INPUT,
            #[cfg(any(LuFolderKanban, icondata_include_all))]
            LuIcon::LuFolderKanban => LU_FOLDER_KANBAN,
            #[cfg(any(LuFolderKey, icondata_include_all))]
            LuIcon::LuFolderKey => LU_FOLDER_KEY,
            #[cfg(any(LuFolderLock, icondata_include_all))]
            LuIcon::LuFolderLock => LU_FOLDER_LOCK,
            #[cfg(any(LuFolderMinus, icondata_include_all))]
            LuIcon::LuFolderMinus => LU_FOLDER_MINUS,
            #[cfg(any(LuFolderOpen, icondata_include_all))]
            LuIcon::LuFolderOpen => LU_FOLDER_OPEN,
            #[cfg(any(LuFolderOpenDot, icondata_include_all))]
            LuIcon::LuFolderOpenDot => LU_FOLDER_OPEN_DOT,
            #[cfg(any(LuFolderOutput, icondata_include_all))]
            LuIcon::LuFolderOutput => LU_FOLDER_OUTPUT,
            #[cfg(any(LuFolderPlus, icondata_include_all))]
            LuIcon::LuFolderPlus => LU_FOLDER_PLUS,
            #[cfg(any(LuFolderRoot, icondata_include_all))]
            LuIcon::LuFolderRoot => LU_FOLDER_ROOT,
            #[cfg(any(LuFolderSearch, icondata_include_all))]
            LuIcon::LuFolderSearch => LU_FOLDER_SEARCH,
            #[cfg(any(LuFolderSearch2, icondata_include_all))]
            LuIcon::LuFolderSearch2 => LU_FOLDER_SEARCH2,
            #[cfg(any(LuFolderSymlink, icondata_include_all))]
            LuIcon::LuFolderSymlink => LU_FOLDER_SYMLINK,
            #[cfg(any(LuFolderSync, icondata_include_all))]
            LuIcon::LuFolderSync => LU_FOLDER_SYNC,
            #[cfg(any(LuFolderTree, icondata_include_all))]
            LuIcon::LuFolderTree => LU_FOLDER_TREE,
            #[cfg(any(LuFolderUp, icondata_include_all))]
            LuIcon::LuFolderUp => LU_FOLDER_UP,
            #[cfg(any(LuFolderX, icondata_include_all))]
            LuIcon::LuFolderX => LU_FOLDER_X,
            #[cfg(any(LuFolders, icondata_include_all))]
            LuIcon::LuFolders => LU_FOLDERS,
            #[cfg(any(LuFootprints, icondata_include_all))]
            LuIcon::LuFootprints => LU_FOOTPRINTS,
            #[cfg(any(LuForklift, icondata_include_all))]
            LuIcon::LuForklift => LU_FORKLIFT,
            #[cfg(any(LuFormInput, icondata_include_all))]
            LuIcon::LuFormInput => LU_FORM_INPUT,
            #[cfg(any(LuForward, icondata_include_all))]
            LuIcon::LuForward => LU_FORWARD,
            #[cfg(any(LuFrame, icondata_include_all))]
            LuIcon::LuFrame => LU_FRAME,
            #[cfg(any(LuFramer, icondata_include_all))]
            LuIcon::LuFramer => LU_FRAMER,
            #[cfg(any(LuFrown, icondata_include_all))]
            LuIcon::LuFrown => LU_FROWN,
            #[cfg(any(LuFuel, icondata_include_all))]
            LuIcon::LuFuel => LU_FUEL,
            #[cfg(any(LuFunctionSquare, icondata_include_all))]
            LuIcon::LuFunctionSquare => LU_FUNCTION_SQUARE,
            #[cfg(any(LuGalleryHorizontal, icondata_include_all))]
            LuIcon::LuGalleryHorizontal => LU_GALLERY_HORIZONTAL,
            #[cfg(any(LuGalleryHorizontalEnd, icondata_include_all))]
            LuIcon::LuGalleryHorizontalEnd => LU_GALLERY_HORIZONTAL_END,
            #[cfg(any(LuGalleryThumbnails, icondata_include_all))]
            LuIcon::LuGalleryThumbnails => LU_GALLERY_THUMBNAILS,
            #[cfg(any(LuGalleryVertical, icondata_include_all))]
            LuIcon::LuGalleryVertical => LU_GALLERY_VERTICAL,
            #[cfg(any(LuGalleryVerticalEnd, icondata_include_all))]
            LuIcon::LuGalleryVerticalEnd => LU_GALLERY_VERTICAL_END,
            #[cfg(any(LuGamepad, icondata_include_all))]
            LuIcon::LuGamepad => LU_GAMEPAD,
            #[cfg(any(LuGamepad2, icondata_include_all))]
            LuIcon::LuGamepad2 => LU_GAMEPAD2,
            #[cfg(any(LuGanttChart, icondata_include_all))]
            LuIcon::LuGanttChart => LU_GANTT_CHART,
            #[cfg(any(LuGanttChartSquare, icondata_include_all))]
            LuIcon::LuGanttChartSquare => LU_GANTT_CHART_SQUARE,
            #[cfg(any(LuGauge, icondata_include_all))]
            LuIcon::LuGauge => LU_GAUGE,
            #[cfg(any(LuGaugeCircle, icondata_include_all))]
            LuIcon::LuGaugeCircle => LU_GAUGE_CIRCLE,
            #[cfg(any(LuGavel, icondata_include_all))]
            LuIcon::LuGavel => LU_GAVEL,
            #[cfg(any(LuGem, icondata_include_all))]
            LuIcon::LuGem => LU_GEM,
            #[cfg(any(LuGhost, icondata_include_all))]
            LuIcon::LuGhost => LU_GHOST,
            #[cfg(any(LuGift, icondata_include_all))]
            LuIcon::LuGift => LU_GIFT,
            #[cfg(any(LuGitBranch, icondata_include_all))]
            LuIcon::LuGitBranch => LU_GIT_BRANCH,
            #[cfg(any(LuGitBranchPlus, icondata_include_all))]
            LuIcon::LuGitBranchPlus => LU_GIT_BRANCH_PLUS,
            #[cfg(any(LuGitCommit, icondata_include_all))]
            LuIcon::LuGitCommit => LU_GIT_COMMIT,
            #[cfg(any(LuGitCompare, icondata_include_all))]
            LuIcon::LuGitCompare => LU_GIT_COMPARE,
            #[cfg(any(LuGitFork, icondata_include_all))]
            LuIcon::LuGitFork => LU_GIT_FORK,
            #[cfg(any(LuGitMerge, icondata_include_all))]
            LuIcon::LuGitMerge => LU_GIT_MERGE,
            #[cfg(any(LuGitPullRequest, icondata_include_all))]
            LuIcon::LuGitPullRequest => LU_GIT_PULL_REQUEST,
            #[cfg(any(LuGitPullRequestClosed, icondata_include_all))]
            LuIcon::LuGitPullRequestClosed => LU_GIT_PULL_REQUEST_CLOSED,
            #[cfg(any(LuGitPullRequestDraft, icondata_include_all))]
            LuIcon::LuGitPullRequestDraft => LU_GIT_PULL_REQUEST_DRAFT,
            #[cfg(any(LuGithub, icondata_include_all))]
            LuIcon::LuGithub => LU_GITHUB,
            #[cfg(any(LuGitlab, icondata_include_all))]
            LuIcon::LuGitlab => LU_GITLAB,
            #[cfg(any(LuGlassWater, icondata_include_all))]
            LuIcon::LuGlassWater => LU_GLASS_WATER,
            #[cfg(any(LuGlasses, icondata_include_all))]
            LuIcon::LuGlasses => LU_GLASSES,
            #[cfg(any(LuGlobe, icondata_include_all))]
            LuIcon::LuGlobe => LU_GLOBE,
            #[cfg(any(LuGlobe2, icondata_include_all))]
            LuIcon::LuGlobe2 => LU_GLOBE2,
            #[cfg(any(LuGoal, icondata_include_all))]
            LuIcon::LuGoal => LU_GOAL,
            #[cfg(any(LuGrab, icondata_include_all))]
            LuIcon::LuGrab => LU_GRAB,
            #[cfg(any(LuGraduationCap, icondata_include_all))]
            LuIcon::LuGraduationCap => LU_GRADUATION_CAP,
            #[cfg(any(LuGrape, icondata_include_all))]
            LuIcon::LuGrape => LU_GRAPE,
            #[cfg(any(LuGrid2x2, icondata_include_all))]
            LuIcon::LuGrid2x2 => LU_GRID2X2,
            #[cfg(any(LuGrid3x3, icondata_include_all))]
            LuIcon::LuGrid3x3 => LU_GRID3X3,
            #[cfg(any(LuGrip, icondata_include_all))]
            LuIcon::LuGrip => LU_GRIP,
            #[cfg(any(LuGripHorizontal, icondata_include_all))]
            LuIcon::LuGripHorizontal => LU_GRIP_HORIZONTAL,
            #[cfg(any(LuGripVertical, icondata_include_all))]
            LuIcon::LuGripVertical => LU_GRIP_VERTICAL,
            #[cfg(any(LuGroup, icondata_include_all))]
            LuIcon::LuGroup => LU_GROUP,
            #[cfg(any(LuHammer, icondata_include_all))]
            LuIcon::LuHammer => LU_HAMMER,
            #[cfg(any(LuHand, icondata_include_all))]
            LuIcon::LuHand => LU_HAND,
            #[cfg(any(LuHandMetal, icondata_include_all))]
            LuIcon::LuHandMetal => LU_HAND_METAL,
            #[cfg(any(LuHardDrive, icondata_include_all))]
            LuIcon::LuHardDrive => LU_HARD_DRIVE,
            #[cfg(any(LuHardDriveDownload, icondata_include_all))]
            LuIcon::LuHardDriveDownload => LU_HARD_DRIVE_DOWNLOAD,
            #[cfg(any(LuHardDriveUpload, icondata_include_all))]
            LuIcon::LuHardDriveUpload => LU_HARD_DRIVE_UPLOAD,
            #[cfg(any(LuHardHat, icondata_include_all))]
            LuIcon::LuHardHat => LU_HARD_HAT,
            #[cfg(any(LuHash, icondata_include_all))]
            LuIcon::LuHash => LU_HASH,
            #[cfg(any(LuHaze, icondata_include_all))]
            LuIcon::LuHaze => LU_HAZE,
            #[cfg(any(LuHdmiPort, icondata_include_all))]
            LuIcon::LuHdmiPort => LU_HDMI_PORT,
            #[cfg(any(LuHeading, icondata_include_all))]
            LuIcon::LuHeading => LU_HEADING,
            #[cfg(any(LuHeading1, icondata_include_all))]
            LuIcon::LuHeading1 => LU_HEADING1,
            #[cfg(any(LuHeading2, icondata_include_all))]
            LuIcon::LuHeading2 => LU_HEADING2,
            #[cfg(any(LuHeading3, icondata_include_all))]
            LuIcon::LuHeading3 => LU_HEADING3,
            #[cfg(any(LuHeading4, icondata_include_all))]
            LuIcon::LuHeading4 => LU_HEADING4,
            #[cfg(any(LuHeading5, icondata_include_all))]
            LuIcon::LuHeading5 => LU_HEADING5,
            #[cfg(any(LuHeading6, icondata_include_all))]
            LuIcon::LuHeading6 => LU_HEADING6,
            #[cfg(any(LuHeadphones, icondata_include_all))]
            LuIcon::LuHeadphones => LU_HEADPHONES,
            #[cfg(any(LuHeart, icondata_include_all))]
            LuIcon::LuHeart => LU_HEART,
            #[cfg(any(LuHeartCrack, icondata_include_all))]
            LuIcon::LuHeartCrack => LU_HEART_CRACK,
            #[cfg(any(LuHeartHandshake, icondata_include_all))]
            LuIcon::LuHeartHandshake => LU_HEART_HANDSHAKE,
            #[cfg(any(LuHeartOff, icondata_include_all))]
            LuIcon::LuHeartOff => LU_HEART_OFF,
            #[cfg(any(LuHeartPulse, icondata_include_all))]
            LuIcon::LuHeartPulse => LU_HEART_PULSE,
            #[cfg(any(LuHelpCircle, icondata_include_all))]
            LuIcon::LuHelpCircle => LU_HELP_CIRCLE,
            #[cfg(any(LuHelpingHand, icondata_include_all))]
            LuIcon::LuHelpingHand => LU_HELPING_HAND,
            #[cfg(any(LuHexagon, icondata_include_all))]
            LuIcon::LuHexagon => LU_HEXAGON,
            #[cfg(any(LuHighlighter, icondata_include_all))]
            LuIcon::LuHighlighter => LU_HIGHLIGHTER,
            #[cfg(any(LuHistory, icondata_include_all))]
            LuIcon::LuHistory => LU_HISTORY,
            #[cfg(any(LuHome, icondata_include_all))]
            LuIcon::LuHome => LU_HOME,
            #[cfg(any(LuHop, icondata_include_all))]
            LuIcon::LuHop => LU_HOP,
            #[cfg(any(LuHopOff, icondata_include_all))]
            LuIcon::LuHopOff => LU_HOP_OFF,
            #[cfg(any(LuHotel, icondata_include_all))]
            LuIcon::LuHotel => LU_HOTEL,
            #[cfg(any(LuHourglass, icondata_include_all))]
            LuIcon::LuHourglass => LU_HOURGLASS,
            #[cfg(any(LuIceCream, icondata_include_all))]
            LuIcon::LuIceCream => LU_ICE_CREAM,
            #[cfg(any(LuIceCream2, icondata_include_all))]
            LuIcon::LuIceCream2 => LU_ICE_CREAM2,
            #[cfg(any(LuImage, icondata_include_all))]
            LuIcon::LuImage => LU_IMAGE,
            #[cfg(any(LuImageMinus, icondata_include_all))]
            LuIcon::LuImageMinus => LU_IMAGE_MINUS,
            #[cfg(any(LuImageOff, icondata_include_all))]
            LuIcon::LuImageOff => LU_IMAGE_OFF,
            #[cfg(any(LuImagePlus, icondata_include_all))]
            LuIcon::LuImagePlus => LU_IMAGE_PLUS,
            #[cfg(any(LuImport, icondata_include_all))]
            LuIcon::LuImport => LU_IMPORT,
            #[cfg(any(LuInbox, icondata_include_all))]
            LuIcon::LuInbox => LU_INBOX,
            #[cfg(any(LuIndent, icondata_include_all))]
            LuIcon::LuIndent => LU_INDENT,
            #[cfg(any(LuIndianRupee, icondata_include_all))]
            LuIcon::LuIndianRupee => LU_INDIAN_RUPEE,
            #[cfg(any(LuInfinity, icondata_include_all))]
            LuIcon::LuInfinity => LU_INFINITY,
            #[cfg(any(LuInfo, icondata_include_all))]
            LuIcon::LuInfo => LU_INFO,
            #[cfg(any(LuInspect, icondata_include_all))]
            LuIcon::LuInspect => LU_INSPECT,
            #[cfg(any(LuInstagram, icondata_include_all))]
            LuIcon::LuInstagram => LU_INSTAGRAM,
            #[cfg(any(LuItalic, icondata_include_all))]
            LuIcon::LuItalic => LU_ITALIC,
            #[cfg(any(LuIterationCcw, icondata_include_all))]
            LuIcon::LuIterationCcw => LU_ITERATION_CCW,
            #[cfg(any(LuIterationCw, icondata_include_all))]
            LuIcon::LuIterationCw => LU_ITERATION_CW,
            #[cfg(any(LuJapaneseYen, icondata_include_all))]
            LuIcon::LuJapaneseYen => LU_JAPANESE_YEN,
            #[cfg(any(LuJoystick, icondata_include_all))]
            LuIcon::LuJoystick => LU_JOYSTICK,
            #[cfg(any(LuKanban, icondata_include_all))]
            LuIcon::LuKanban => LU_KANBAN,
            #[cfg(any(LuKanbanSquare, icondata_include_all))]
            LuIcon::LuKanbanSquare => LU_KANBAN_SQUARE,
            #[cfg(any(LuKanbanSquareDashed, icondata_include_all))]
            LuIcon::LuKanbanSquareDashed => LU_KANBAN_SQUARE_DASHED,
            #[cfg(any(LuKey, icondata_include_all))]
            LuIcon::LuKey => LU_KEY,
            #[cfg(any(LuKeyRound, icondata_include_all))]
            LuIcon::LuKeyRound => LU_KEY_ROUND,
            #[cfg(any(LuKeySquare, icondata_include_all))]
            LuIcon::LuKeySquare => LU_KEY_SQUARE,
            #[cfg(any(LuKeyboard, icondata_include_all))]
            LuIcon::LuKeyboard => LU_KEYBOARD,
            #[cfg(any(LuLamp, icondata_include_all))]
            LuIcon::LuLamp => LU_LAMP,
            #[cfg(any(LuLampCeiling, icondata_include_all))]
            LuIcon::LuLampCeiling => LU_LAMP_CEILING,
            #[cfg(any(LuLampDesk, icondata_include_all))]
            LuIcon::LuLampDesk => LU_LAMP_DESK,
            #[cfg(any(LuLampFloor, icondata_include_all))]
            LuIcon::LuLampFloor => LU_LAMP_FLOOR,
            #[cfg(any(LuLampWallDown, icondata_include_all))]
            LuIcon::LuLampWallDown => LU_LAMP_WALL_DOWN,
            #[cfg(any(LuLampWallUp, icondata_include_all))]
            LuIcon::LuLampWallUp => LU_LAMP_WALL_UP,
            #[cfg(any(LuLandmark, icondata_include_all))]
            LuIcon::LuLandmark => LU_LANDMARK,
            #[cfg(any(LuLanguages, icondata_include_all))]
            LuIcon::LuLanguages => LU_LANGUAGES,
            #[cfg(any(LuLaptop, icondata_include_all))]
            LuIcon::LuLaptop => LU_LAPTOP,
            #[cfg(any(LuLaptop2, icondata_include_all))]
            LuIcon::LuLaptop2 => LU_LAPTOP2,
            #[cfg(any(LuLasso, icondata_include_all))]
            LuIcon::LuLasso => LU_LASSO,
            #[cfg(any(LuLassoSelect, icondata_include_all))]
            LuIcon::LuLassoSelect => LU_LASSO_SELECT,
            #[cfg(any(LuLaugh, icondata_include_all))]
            LuIcon::LuLaugh => LU_LAUGH,
            #[cfg(any(LuLayers, icondata_include_all))]
            LuIcon::LuLayers => LU_LAYERS,
            #[cfg(any(LuLayout, icondata_include_all))]
            LuIcon::LuLayout => LU_LAYOUT,
            #[cfg(any(LuLayoutDashboard, icondata_include_all))]
            LuIcon::LuLayoutDashboard => LU_LAYOUT_DASHBOARD,
            #[cfg(any(LuLayoutGrid, icondata_include_all))]
            LuIcon::LuLayoutGrid => LU_LAYOUT_GRID,
            #[cfg(any(LuLayoutList, icondata_include_all))]
            LuIcon::LuLayoutList => LU_LAYOUT_LIST,
            #[cfg(any(LuLayoutPanelLeft, icondata_include_all))]
            LuIcon::LuLayoutPanelLeft => LU_LAYOUT_PANEL_LEFT,
            #[cfg(any(LuLayoutPanelTop, icondata_include_all))]
            LuIcon::LuLayoutPanelTop => LU_LAYOUT_PANEL_TOP,
            #[cfg(any(LuLayoutTemplate, icondata_include_all))]
            LuIcon::LuLayoutTemplate => LU_LAYOUT_TEMPLATE,
            #[cfg(any(LuLeaf, icondata_include_all))]
            LuIcon::LuLeaf => LU_LEAF,
            #[cfg(any(LuLeafyGreen, icondata_include_all))]
            LuIcon::LuLeafyGreen => LU_LEAFY_GREEN,
            #[cfg(any(LuLibrary, icondata_include_all))]
            LuIcon::LuLibrary => LU_LIBRARY,
            #[cfg(any(LuLifeBuoy, icondata_include_all))]
            LuIcon::LuLifeBuoy => LU_LIFE_BUOY,
            #[cfg(any(LuLigature, icondata_include_all))]
            LuIcon::LuLigature => LU_LIGATURE,
            #[cfg(any(LuLightbulb, icondata_include_all))]
            LuIcon::LuLightbulb => LU_LIGHTBULB,
            #[cfg(any(LuLightbulbOff, icondata_include_all))]
            LuIcon::LuLightbulbOff => LU_LIGHTBULB_OFF,
            #[cfg(any(LuLineChart, icondata_include_all))]
            LuIcon::LuLineChart => LU_LINE_CHART,
            #[cfg(any(LuLink, icondata_include_all))]
            LuIcon::LuLink => LU_LINK,
            #[cfg(any(LuLink2, icondata_include_all))]
            LuIcon::LuLink2 => LU_LINK2,
            #[cfg(any(LuLink2Off, icondata_include_all))]
            LuIcon::LuLink2Off => LU_LINK2_OFF,
            #[cfg(any(LuLinkedin, icondata_include_all))]
            LuIcon::LuLinkedin => LU_LINKEDIN,
            #[cfg(any(LuList, icondata_include_all))]
            LuIcon::LuList => LU_LIST,
            #[cfg(any(LuListChecks, icondata_include_all))]
            LuIcon::LuListChecks => LU_LIST_CHECKS,
            #[cfg(any(LuListEnd, icondata_include_all))]
            LuIcon::LuListEnd => LU_LIST_END,
            #[cfg(any(LuListFilter, icondata_include_all))]
            LuIcon::LuListFilter => LU_LIST_FILTER,
            #[cfg(any(LuListMinus, icondata_include_all))]
            LuIcon::LuListMinus => LU_LIST_MINUS,
            #[cfg(any(LuListMusic, icondata_include_all))]
            LuIcon::LuListMusic => LU_LIST_MUSIC,
            #[cfg(any(LuListOrdered, icondata_include_all))]
            LuIcon::LuListOrdered => LU_LIST_ORDERED,
            #[cfg(any(LuListPlus, icondata_include_all))]
            LuIcon::LuListPlus => LU_LIST_PLUS,
            #[cfg(any(LuListRestart, icondata_include_all))]
            LuIcon::LuListRestart => LU_LIST_RESTART,
            #[cfg(any(LuListStart, icondata_include_all))]
            LuIcon::LuListStart => LU_LIST_START,
            #[cfg(any(LuListTodo, icondata_include_all))]
            LuIcon::LuListTodo => LU_LIST_TODO,
            #[cfg(any(LuListTree, icondata_include_all))]
            LuIcon::LuListTree => LU_LIST_TREE,
            #[cfg(any(LuListVideo, icondata_include_all))]
            LuIcon::LuListVideo => LU_LIST_VIDEO,
            #[cfg(any(LuListX, icondata_include_all))]
            LuIcon::LuListX => LU_LIST_X,
            #[cfg(any(LuLoader, icondata_include_all))]
            LuIcon::LuLoader => LU_LOADER,
            #[cfg(any(LuLoader2, icondata_include_all))]
            LuIcon::LuLoader2 => LU_LOADER2,
            #[cfg(any(LuLocate, icondata_include_all))]
            LuIcon::LuLocate => LU_LOCATE,
            #[cfg(any(LuLocateFixed, icondata_include_all))]
            LuIcon::LuLocateFixed => LU_LOCATE_FIXED,
            #[cfg(any(LuLocateOff, icondata_include_all))]
            LuIcon::LuLocateOff => LU_LOCATE_OFF,
            #[cfg(any(LuLock, icondata_include_all))]
            LuIcon::LuLock => LU_LOCK,
            #[cfg(any(LuLogIn, icondata_include_all))]
            LuIcon::LuLogIn => LU_LOG_IN,
            #[cfg(any(LuLogOut, icondata_include_all))]
            LuIcon::LuLogOut => LU_LOG_OUT,
            #[cfg(any(LuLollipop, icondata_include_all))]
            LuIcon::LuLollipop => LU_LOLLIPOP,
            #[cfg(any(LuLuggage, icondata_include_all))]
            LuIcon::LuLuggage => LU_LUGGAGE,
            #[cfg(any(LuMSquare, icondata_include_all))]
            LuIcon::LuMSquare => LU_M_SQUARE,
            #[cfg(any(LuMagnet, icondata_include_all))]
            LuIcon::LuMagnet => LU_MAGNET,
            #[cfg(any(LuMail, icondata_include_all))]
            LuIcon::LuMail => LU_MAIL,
            #[cfg(any(LuMailCheck, icondata_include_all))]
            LuIcon::LuMailCheck => LU_MAIL_CHECK,
            #[cfg(any(LuMailMinus, icondata_include_all))]
            LuIcon::LuMailMinus => LU_MAIL_MINUS,
            #[cfg(any(LuMailOpen, icondata_include_all))]
            LuIcon::LuMailOpen => LU_MAIL_OPEN,
            #[cfg(any(LuMailPlus, icondata_include_all))]
            LuIcon::LuMailPlus => LU_MAIL_PLUS,
            #[cfg(any(LuMailQuestion, icondata_include_all))]
            LuIcon::LuMailQuestion => LU_MAIL_QUESTION,
            #[cfg(any(LuMailSearch, icondata_include_all))]
            LuIcon::LuMailSearch => LU_MAIL_SEARCH,
            #[cfg(any(LuMailWarning, icondata_include_all))]
            LuIcon::LuMailWarning => LU_MAIL_WARNING,
            #[cfg(any(LuMailX, icondata_include_all))]
            LuIcon::LuMailX => LU_MAIL_X,
            #[cfg(any(LuMailbox, icondata_include_all))]
            LuIcon::LuMailbox => LU_MAILBOX,
            #[cfg(any(LuMails, icondata_include_all))]
            LuIcon::LuMails => LU_MAILS,
            #[cfg(any(LuMap, icondata_include_all))]
            LuIcon::LuMap => LU_MAP,
            #[cfg(any(LuMapPin, icondata_include_all))]
            LuIcon::LuMapPin => LU_MAP_PIN,
            #[cfg(any(LuMapPinOff, icondata_include_all))]
            LuIcon::LuMapPinOff => LU_MAP_PIN_OFF,
            #[cfg(any(LuMartini, icondata_include_all))]
            LuIcon::LuMartini => LU_MARTINI,
            #[cfg(any(LuMaximize, icondata_include_all))]
            LuIcon::LuMaximize => LU_MAXIMIZE,
            #[cfg(any(LuMaximize2, icondata_include_all))]
            LuIcon::LuMaximize2 => LU_MAXIMIZE2,
            #[cfg(any(LuMedal, icondata_include_all))]
            LuIcon::LuMedal => LU_MEDAL,
            #[cfg(any(LuMegaphone, icondata_include_all))]
            LuIcon::LuMegaphone => LU_MEGAPHONE,
            #[cfg(any(LuMegaphoneOff, icondata_include_all))]
            LuIcon::LuMegaphoneOff => LU_MEGAPHONE_OFF,
            #[cfg(any(LuMeh, icondata_include_all))]
            LuIcon::LuMeh => LU_MEH,
            #[cfg(any(LuMemoryStick, icondata_include_all))]
            LuIcon::LuMemoryStick => LU_MEMORY_STICK,
            #[cfg(any(LuMenu, icondata_include_all))]
            LuIcon::LuMenu => LU_MENU,
            #[cfg(any(LuMenuSquare, icondata_include_all))]
            LuIcon::LuMenuSquare => LU_MENU_SQUARE,
            #[cfg(any(LuMerge, icondata_include_all))]
            LuIcon::LuMerge => LU_MERGE,
            #[cfg(any(LuMessageCircle, icondata_include_all))]
            LuIcon::LuMessageCircle => LU_MESSAGE_CIRCLE,
            #[cfg(any(LuMessageSquare, icondata_include_all))]
            LuIcon::LuMessageSquare => LU_MESSAGE_SQUARE,
            #[cfg(any(LuMessageSquareDashed, icondata_include_all))]
            LuIcon::LuMessageSquareDashed => LU_MESSAGE_SQUARE_DASHED,
            #[cfg(any(LuMessageSquarePlus, icondata_include_all))]
            LuIcon::LuMessageSquarePlus => LU_MESSAGE_SQUARE_PLUS,
            #[cfg(any(LuMessagesSquare, icondata_include_all))]
            LuIcon::LuMessagesSquare => LU_MESSAGES_SQUARE,
            #[cfg(any(LuMic, icondata_include_all))]
            LuIcon::LuMic => LU_MIC,
            #[cfg(any(LuMic2, icondata_include_all))]
            LuIcon::LuMic2 => LU_MIC2,
            #[cfg(any(LuMicOff, icondata_include_all))]
            LuIcon::LuMicOff => LU_MIC_OFF,
            #[cfg(any(LuMicroscope, icondata_include_all))]
            LuIcon::LuMicroscope => LU_MICROSCOPE,
            #[cfg(any(LuMicrowave, icondata_include_all))]
            LuIcon::LuMicrowave => LU_MICROWAVE,
            #[cfg(any(LuMilestone, icondata_include_all))]
            LuIcon::LuMilestone => LU_MILESTONE,
            #[cfg(any(LuMilk, icondata_include_all))]
            LuIcon::LuMilk => LU_MILK,
            #[cfg(any(LuMilkOff, icondata_include_all))]
            LuIcon::LuMilkOff => LU_MILK_OFF,
            #[cfg(any(LuMinimize, icondata_include_all))]
            LuIcon::LuMinimize => LU_MINIMIZE,
            #[cfg(any(LuMinimize2, icondata_include_all))]
            LuIcon::LuMinimize2 => LU_MINIMIZE2,
            #[cfg(any(LuMinus, icondata_include_all))]
            LuIcon::LuMinus => LU_MINUS,
            #[cfg(any(LuMinusCircle, icondata_include_all))]
            LuIcon::LuMinusCircle => LU_MINUS_CIRCLE,
            #[cfg(any(LuMinusSquare, icondata_include_all))]
            LuIcon::LuMinusSquare => LU_MINUS_SQUARE,
            #[cfg(any(LuMonitor, icondata_include_all))]
            LuIcon::LuMonitor => LU_MONITOR,
            #[cfg(any(LuMonitorCheck, icondata_include_all))]
            LuIcon::LuMonitorCheck => LU_MONITOR_CHECK,
            #[cfg(any(LuMonitorDot, icondata_include_all))]
            LuIcon::LuMonitorDot => LU_MONITOR_DOT,
            #[cfg(any(LuMonitorDown, icondata_include_all))]
            LuIcon::LuMonitorDown => LU_MONITOR_DOWN,
            #[cfg(any(LuMonitorOff, icondata_include_all))]
            LuIcon::LuMonitorOff => LU_MONITOR_OFF,
            #[cfg(any(LuMonitorPause, icondata_include_all))]
            LuIcon::LuMonitorPause => LU_MONITOR_PAUSE,
            #[cfg(any(LuMonitorPlay, icondata_include_all))]
            LuIcon::LuMonitorPlay => LU_MONITOR_PLAY,
            #[cfg(any(LuMonitorSmartphone, icondata_include_all))]
            LuIcon::LuMonitorSmartphone => LU_MONITOR_SMARTPHONE,
            #[cfg(any(LuMonitorSpeaker, icondata_include_all))]
            LuIcon::LuMonitorSpeaker => LU_MONITOR_SPEAKER,
            #[cfg(any(LuMonitorStop, icondata_include_all))]
            LuIcon::LuMonitorStop => LU_MONITOR_STOP,
            #[cfg(any(LuMonitorUp, icondata_include_all))]
            LuIcon::LuMonitorUp => LU_MONITOR_UP,
            #[cfg(any(LuMonitorX, icondata_include_all))]
            LuIcon::LuMonitorX => LU_MONITOR_X,
            #[cfg(any(LuMoon, icondata_include_all))]
            LuIcon::LuMoon => LU_MOON,
            #[cfg(any(LuMoonStar, icondata_include_all))]
            LuIcon::LuMoonStar => LU_MOON_STAR,
            #[cfg(any(LuMoreHorizontal, icondata_include_all))]
            LuIcon::LuMoreHorizontal => LU_MORE_HORIZONTAL,
            #[cfg(any(LuMoreVertical, icondata_include_all))]
            LuIcon::LuMoreVertical => LU_MORE_VERTICAL,
            #[cfg(any(LuMountain, icondata_include_all))]
            LuIcon::LuMountain => LU_MOUNTAIN,
            #[cfg(any(LuMountainSnow, icondata_include_all))]
            LuIcon::LuMountainSnow => LU_MOUNTAIN_SNOW,
            #[cfg(any(LuMouse, icondata_include_all))]
            LuIcon::LuMouse => LU_MOUSE,
            #[cfg(any(LuMousePointer, icondata_include_all))]
            LuIcon::LuMousePointer => LU_MOUSE_POINTER,
            #[cfg(any(LuMousePointer2, icondata_include_all))]
            LuIcon::LuMousePointer2 => LU_MOUSE_POINTER2,
            #[cfg(any(LuMousePointerClick, icondata_include_all))]
            LuIcon::LuMousePointerClick => LU_MOUSE_POINTER_CLICK,
            #[cfg(any(LuMove, icondata_include_all))]
            LuIcon::LuMove => LU_MOVE,
            #[cfg(any(LuMove3d, icondata_include_all))]
            LuIcon::LuMove3d => LU_MOVE3D,
            #[cfg(any(LuMoveDiagonal, icondata_include_all))]
            LuIcon::LuMoveDiagonal => LU_MOVE_DIAGONAL,
            #[cfg(any(LuMoveDiagonal2, icondata_include_all))]
            LuIcon::LuMoveDiagonal2 => LU_MOVE_DIAGONAL2,
            #[cfg(any(LuMoveDown, icondata_include_all))]
            LuIcon::LuMoveDown => LU_MOVE_DOWN,
            #[cfg(any(LuMoveDownLeft, icondata_include_all))]
            LuIcon::LuMoveDownLeft => LU_MOVE_DOWN_LEFT,
            #[cfg(any(LuMoveDownRight, icondata_include_all))]
            LuIcon::LuMoveDownRight => LU_MOVE_DOWN_RIGHT,
            #[cfg(any(LuMoveHorizontal, icondata_include_all))]
            LuIcon::LuMoveHorizontal => LU_MOVE_HORIZONTAL,
            #[cfg(any(LuMoveLeft, icondata_include_all))]
            LuIcon::LuMoveLeft => LU_MOVE_LEFT,
            #[cfg(any(LuMoveRight, icondata_include_all))]
            LuIcon::LuMoveRight => LU_MOVE_RIGHT,
            #[cfg(any(LuMoveUp, icondata_include_all))]
            LuIcon::LuMoveUp => LU_MOVE_UP,
            #[cfg(any(LuMoveUpLeft, icondata_include_all))]
            LuIcon::LuMoveUpLeft => LU_MOVE_UP_LEFT,
            #[cfg(any(LuMoveUpRight, icondata_include_all))]
            LuIcon::LuMoveUpRight => LU_MOVE_UP_RIGHT,
            #[cfg(any(LuMoveVertical, icondata_include_all))]
            LuIcon::LuMoveVertical => LU_MOVE_VERTICAL,
            #[cfg(any(LuMusic, icondata_include_all))]
            LuIcon::LuMusic => LU_MUSIC,
            #[cfg(any(LuMusic2, icondata_include_all))]
            LuIcon::LuMusic2 => LU_MUSIC2,
            #[cfg(any(LuMusic3, icondata_include_all))]
            LuIcon::LuMusic3 => LU_MUSIC3,
            #[cfg(any(LuMusic4, icondata_include_all))]
            LuIcon::LuMusic4 => LU_MUSIC4,
            #[cfg(any(LuNavigation, icondata_include_all))]
            LuIcon::LuNavigation => LU_NAVIGATION,
            #[cfg(any(LuNavigation2, icondata_include_all))]
            LuIcon::LuNavigation2 => LU_NAVIGATION2,
            #[cfg(any(LuNavigation2Off, icondata_include_all))]
            LuIcon::LuNavigation2Off => LU_NAVIGATION2_OFF,
            #[cfg(any(LuNavigationOff, icondata_include_all))]
            LuIcon::LuNavigationOff => LU_NAVIGATION_OFF,
            #[cfg(any(LuNetwork, icondata_include_all))]
            LuIcon::LuNetwork => LU_NETWORK,
            #[cfg(any(LuNewspaper, icondata_include_all))]
            LuIcon::LuNewspaper => LU_NEWSPAPER,
            #[cfg(any(LuNfc, icondata_include_all))]
            LuIcon::LuNfc => LU_NFC,
            #[cfg(any(LuNut, icondata_include_all))]
            LuIcon::LuNut => LU_NUT,
            #[cfg(any(LuNutOff, icondata_include_all))]
            LuIcon::LuNutOff => LU_NUT_OFF,
            #[cfg(any(LuOctagon, icondata_include_all))]
            LuIcon::LuOctagon => LU_OCTAGON,
            #[cfg(any(LuOption, icondata_include_all))]
            LuIcon::LuOption => LU_OPTION,
            #[cfg(any(LuOrbit, icondata_include_all))]
            LuIcon::LuOrbit => LU_ORBIT,
            #[cfg(any(LuOutdent, icondata_include_all))]
            LuIcon::LuOutdent => LU_OUTDENT,
            #[cfg(any(LuPackage, icondata_include_all))]
            LuIcon::LuPackage => LU_PACKAGE,
            #[cfg(any(LuPackage2, icondata_include_all))]
            LuIcon::LuPackage2 => LU_PACKAGE2,
            #[cfg(any(LuPackageCheck, icondata_include_all))]
            LuIcon::LuPackageCheck => LU_PACKAGE_CHECK,
            #[cfg(any(LuPackageMinus, icondata_include_all))]
            LuIcon::LuPackageMinus => LU_PACKAGE_MINUS,
            #[cfg(any(LuPackageOpen, icondata_include_all))]
            LuIcon::LuPackageOpen => LU_PACKAGE_OPEN,
            #[cfg(any(LuPackagePlus, icondata_include_all))]
            LuIcon::LuPackagePlus => LU_PACKAGE_PLUS,
            #[cfg(any(LuPackageSearch, icondata_include_all))]
            LuIcon::LuPackageSearch => LU_PACKAGE_SEARCH,
            #[cfg(any(LuPackageX, icondata_include_all))]
            LuIcon::LuPackageX => LU_PACKAGE_X,
            #[cfg(any(LuPaintBucket, icondata_include_all))]
            LuIcon::LuPaintBucket => LU_PAINT_BUCKET,
            #[cfg(any(LuPaintbrush, icondata_include_all))]
            LuIcon::LuPaintbrush => LU_PAINTBRUSH,
            #[cfg(any(LuPaintbrush2, icondata_include_all))]
            LuIcon::LuPaintbrush2 => LU_PAINTBRUSH2,
            #[cfg(any(LuPalette, icondata_include_all))]
            LuIcon::LuPalette => LU_PALETTE,
            #[cfg(any(LuPalmtree, icondata_include_all))]
            LuIcon::LuPalmtree => LU_PALMTREE,
            #[cfg(any(LuPanelBottom, icondata_include_all))]
            LuIcon::LuPanelBottom => LU_PANEL_BOTTOM,
            #[cfg(any(LuPanelBottomClose, icondata_include_all))]
            LuIcon::LuPanelBottomClose => LU_PANEL_BOTTOM_CLOSE,
            #[cfg(any(LuPanelBottomInactive, icondata_include_all))]
            LuIcon::LuPanelBottomInactive => LU_PANEL_BOTTOM_INACTIVE,
            #[cfg(any(LuPanelBottomOpen, icondata_include_all))]
            LuIcon::LuPanelBottomOpen => LU_PANEL_BOTTOM_OPEN,
            #[cfg(any(LuPanelLeft, icondata_include_all))]
            LuIcon::LuPanelLeft => LU_PANEL_LEFT,
            #[cfg(any(LuPanelLeftClose, icondata_include_all))]
            LuIcon::LuPanelLeftClose => LU_PANEL_LEFT_CLOSE,
            #[cfg(any(LuPanelLeftInactive, icondata_include_all))]
            LuIcon::LuPanelLeftInactive => LU_PANEL_LEFT_INACTIVE,
            #[cfg(any(LuPanelLeftOpen, icondata_include_all))]
            LuIcon::LuPanelLeftOpen => LU_PANEL_LEFT_OPEN,
            #[cfg(any(LuPanelRight, icondata_include_all))]
            LuIcon::LuPanelRight => LU_PANEL_RIGHT,
            #[cfg(any(LuPanelRightClose, icondata_include_all))]
            LuIcon::LuPanelRightClose => LU_PANEL_RIGHT_CLOSE,
            #[cfg(any(LuPanelRightInactive, icondata_include_all))]
            LuIcon::LuPanelRightInactive => LU_PANEL_RIGHT_INACTIVE,
            #[cfg(any(LuPanelRightOpen, icondata_include_all))]
            LuIcon::LuPanelRightOpen => LU_PANEL_RIGHT_OPEN,
            #[cfg(any(LuPanelTop, icondata_include_all))]
            LuIcon::LuPanelTop => LU_PANEL_TOP,
            #[cfg(any(LuPanelTopClose, icondata_include_all))]
            LuIcon::LuPanelTopClose => LU_PANEL_TOP_CLOSE,
            #[cfg(any(LuPanelTopInactive, icondata_include_all))]
            LuIcon::LuPanelTopInactive => LU_PANEL_TOP_INACTIVE,
            #[cfg(any(LuPanelTopOpen, icondata_include_all))]
            LuIcon::LuPanelTopOpen => LU_PANEL_TOP_OPEN,
            #[cfg(any(LuPaperclip, icondata_include_all))]
            LuIcon::LuPaperclip => LU_PAPERCLIP,
            #[cfg(any(LuParentheses, icondata_include_all))]
            LuIcon::LuParentheses => LU_PARENTHESES,
            #[cfg(any(LuParkingCircle, icondata_include_all))]
            LuIcon::LuParkingCircle => LU_PARKING_CIRCLE,
            #[cfg(any(LuParkingCircleOff, icondata_include_all))]
            LuIcon::LuParkingCircleOff => LU_PARKING_CIRCLE_OFF,
            #[cfg(any(LuParkingMeter, icondata_include_all))]
            LuIcon::LuParkingMeter => LU_PARKING_METER,
            #[cfg(any(LuParkingSquare, icondata_include_all))]
            LuIcon::LuParkingSquare => LU_PARKING_SQUARE,
            #[cfg(any(LuParkingSquareOff, icondata_include_all))]
            LuIcon::LuParkingSquareOff => LU_PARKING_SQUARE_OFF,
            #[cfg(any(LuPartyPopper, icondata_include_all))]
            LuIcon::LuPartyPopper => LU_PARTY_POPPER,
            #[cfg(any(LuPause, icondata_include_all))]
            LuIcon::LuPause => LU_PAUSE,
            #[cfg(any(LuPauseCircle, icondata_include_all))]
            LuIcon::LuPauseCircle => LU_PAUSE_CIRCLE,
            #[cfg(any(LuPauseOctagon, icondata_include_all))]
            LuIcon::LuPauseOctagon => LU_PAUSE_OCTAGON,
            #[cfg(any(LuPawPrint, icondata_include_all))]
            LuIcon::LuPawPrint => LU_PAW_PRINT,
            #[cfg(any(LuPcCase, icondata_include_all))]
            LuIcon::LuPcCase => LU_PC_CASE,
            #[cfg(any(LuPen, icondata_include_all))]
            LuIcon::LuPen => LU_PEN,
            #[cfg(any(LuPenLine, icondata_include_all))]
            LuIcon::LuPenLine => LU_PEN_LINE,
            #[cfg(any(LuPenSquare, icondata_include_all))]
            LuIcon::LuPenSquare => LU_PEN_SQUARE,
            #[cfg(any(LuPenTool, icondata_include_all))]
            LuIcon::LuPenTool => LU_PEN_TOOL,
            #[cfg(any(LuPencil, icondata_include_all))]
            LuIcon::LuPencil => LU_PENCIL,
            #[cfg(any(LuPencilLine, icondata_include_all))]
            LuIcon::LuPencilLine => LU_PENCIL_LINE,
            #[cfg(any(LuPencilRuler, icondata_include_all))]
            LuIcon::LuPencilRuler => LU_PENCIL_RULER,
            #[cfg(any(LuPercent, icondata_include_all))]
            LuIcon::LuPercent => LU_PERCENT,
            #[cfg(any(LuPersonStanding, icondata_include_all))]
            LuIcon::LuPersonStanding => LU_PERSON_STANDING,
            #[cfg(any(LuPhone, icondata_include_all))]
            LuIcon::LuPhone => LU_PHONE,
            #[cfg(any(LuPhoneCall, icondata_include_all))]
            LuIcon::LuPhoneCall => LU_PHONE_CALL,
            #[cfg(any(LuPhoneForwarded, icondata_include_all))]
            LuIcon::LuPhoneForwarded => LU_PHONE_FORWARDED,
            #[cfg(any(LuPhoneIncoming, icondata_include_all))]
            LuIcon::LuPhoneIncoming => LU_PHONE_INCOMING,
            #[cfg(any(LuPhoneMissed, icondata_include_all))]
            LuIcon::LuPhoneMissed => LU_PHONE_MISSED,
            #[cfg(any(LuPhoneOff, icondata_include_all))]
            LuIcon::LuPhoneOff => LU_PHONE_OFF,
            #[cfg(any(LuPhoneOutgoing, icondata_include_all))]
            LuIcon::LuPhoneOutgoing => LU_PHONE_OUTGOING,
            #[cfg(any(LuPi, icondata_include_all))]
            LuIcon::LuPi => LU_PI,
            #[cfg(any(LuPiSquare, icondata_include_all))]
            LuIcon::LuPiSquare => LU_PI_SQUARE,
            #[cfg(any(LuPictureInPicture, icondata_include_all))]
            LuIcon::LuPictureInPicture => LU_PICTURE_IN_PICTURE,
            #[cfg(any(LuPictureInPicture2, icondata_include_all))]
            LuIcon::LuPictureInPicture2 => LU_PICTURE_IN_PICTURE2,
            #[cfg(any(LuPieChart, icondata_include_all))]
            LuIcon::LuPieChart => LU_PIE_CHART,
            #[cfg(any(LuPiggyBank, icondata_include_all))]
            LuIcon::LuPiggyBank => LU_PIGGY_BANK,
            #[cfg(any(LuPilcrow, icondata_include_all))]
            LuIcon::LuPilcrow => LU_PILCROW,
            #[cfg(any(LuPilcrowSquare, icondata_include_all))]
            LuIcon::LuPilcrowSquare => LU_PILCROW_SQUARE,
            #[cfg(any(LuPill, icondata_include_all))]
            LuIcon::LuPill => LU_PILL,
            #[cfg(any(LuPin, icondata_include_all))]
            LuIcon::LuPin => LU_PIN,
            #[cfg(any(LuPinOff, icondata_include_all))]
            LuIcon::LuPinOff => LU_PIN_OFF,
            #[cfg(any(LuPipette, icondata_include_all))]
            LuIcon::LuPipette => LU_PIPETTE,
            #[cfg(any(LuPizza, icondata_include_all))]
            LuIcon::LuPizza => LU_PIZZA,
            #[cfg(any(LuPlane, icondata_include_all))]
            LuIcon::LuPlane => LU_PLANE,
            #[cfg(any(LuPlaneLanding, icondata_include_all))]
            LuIcon::LuPlaneLanding => LU_PLANE_LANDING,
            #[cfg(any(LuPlaneTakeoff, icondata_include_all))]
            LuIcon::LuPlaneTakeoff => LU_PLANE_TAKEOFF,
            #[cfg(any(LuPlay, icondata_include_all))]
            LuIcon::LuPlay => LU_PLAY,
            #[cfg(any(LuPlayCircle, icondata_include_all))]
            LuIcon::LuPlayCircle => LU_PLAY_CIRCLE,
            #[cfg(any(LuPlaySquare, icondata_include_all))]
            LuIcon::LuPlaySquare => LU_PLAY_SQUARE,
            #[cfg(any(LuPlug, icondata_include_all))]
            LuIcon::LuPlug => LU_PLUG,
            #[cfg(any(LuPlug2, icondata_include_all))]
            LuIcon::LuPlug2 => LU_PLUG2,
            #[cfg(any(LuPlugZap, icondata_include_all))]
            LuIcon::LuPlugZap => LU_PLUG_ZAP,
            #[cfg(any(LuPlugZap2, icondata_include_all))]
            LuIcon::LuPlugZap2 => LU_PLUG_ZAP2,
            #[cfg(any(LuPlus, icondata_include_all))]
            LuIcon::LuPlus => LU_PLUS,
            #[cfg(any(LuPlusCircle, icondata_include_all))]
            LuIcon::LuPlusCircle => LU_PLUS_CIRCLE,
            #[cfg(any(LuPlusSquare, icondata_include_all))]
            LuIcon::LuPlusSquare => LU_PLUS_SQUARE,
            #[cfg(any(LuPocket, icondata_include_all))]
            LuIcon::LuPocket => LU_POCKET,
            #[cfg(any(LuPocketKnife, icondata_include_all))]
            LuIcon::LuPocketKnife => LU_POCKET_KNIFE,
            #[cfg(any(LuPodcast, icondata_include_all))]
            LuIcon::LuPodcast => LU_PODCAST,
            #[cfg(any(LuPointer, icondata_include_all))]
            LuIcon::LuPointer => LU_POINTER,
            #[cfg(any(LuPopcorn, icondata_include_all))]
            LuIcon::LuPopcorn => LU_POPCORN,
            #[cfg(any(LuPopsicle, icondata_include_all))]
            LuIcon::LuPopsicle => LU_POPSICLE,
            #[cfg(any(LuPoundSterling, icondata_include_all))]
            LuIcon::LuPoundSterling => LU_POUND_STERLING,
            #[cfg(any(LuPower, icondata_include_all))]
            LuIcon::LuPower => LU_POWER,
            #[cfg(any(LuPowerOff, icondata_include_all))]
            LuIcon::LuPowerOff => LU_POWER_OFF,
            #[cfg(any(LuPresentation, icondata_include_all))]
            LuIcon::LuPresentation => LU_PRESENTATION,
            #[cfg(any(LuPrinter, icondata_include_all))]
            LuIcon::LuPrinter => LU_PRINTER,
            #[cfg(any(LuProjector, icondata_include_all))]
            LuIcon::LuProjector => LU_PROJECTOR,
            #[cfg(any(LuPuzzle, icondata_include_all))]
            LuIcon::LuPuzzle => LU_PUZZLE,
            #[cfg(any(LuQrCode, icondata_include_all))]
            LuIcon::LuQrCode => LU_QR_CODE,
            #[cfg(any(LuQuote, icondata_include_all))]
            LuIcon::LuQuote => LU_QUOTE,
            #[cfg(any(LuRabbit, icondata_include_all))]
            LuIcon::LuRabbit => LU_RABBIT,
            #[cfg(any(LuRadar, icondata_include_all))]
            LuIcon::LuRadar => LU_RADAR,
            #[cfg(any(LuRadiation, icondata_include_all))]
            LuIcon::LuRadiation => LU_RADIATION,
            #[cfg(any(LuRadio, icondata_include_all))]
            LuIcon::LuRadio => LU_RADIO,
            #[cfg(any(LuRadioReceiver, icondata_include_all))]
            LuIcon::LuRadioReceiver => LU_RADIO_RECEIVER,
            #[cfg(any(LuRadioTower, icondata_include_all))]
            LuIcon::LuRadioTower => LU_RADIO_TOWER,
            #[cfg(any(LuRailSymbol, icondata_include_all))]
            LuIcon::LuRailSymbol => LU_RAIL_SYMBOL,
            #[cfg(any(LuRainbow, icondata_include_all))]
            LuIcon::LuRainbow => LU_RAINBOW,
            #[cfg(any(LuRat, icondata_include_all))]
            LuIcon::LuRat => LU_RAT,
            #[cfg(any(LuRatio, icondata_include_all))]
            LuIcon::LuRatio => LU_RATIO,
            #[cfg(any(LuReceipt, icondata_include_all))]
            LuIcon::LuReceipt => LU_RECEIPT,
            #[cfg(any(LuRectangleHorizontal, icondata_include_all))]
            LuIcon::LuRectangleHorizontal => LU_RECTANGLE_HORIZONTAL,
            #[cfg(any(LuRectangleVertical, icondata_include_all))]
            LuIcon::LuRectangleVertical => LU_RECTANGLE_VERTICAL,
            #[cfg(any(LuRecycle, icondata_include_all))]
            LuIcon::LuRecycle => LU_RECYCLE,
            #[cfg(any(LuRedo, icondata_include_all))]
            LuIcon::LuRedo => LU_REDO,
            #[cfg(any(LuRedo2, icondata_include_all))]
            LuIcon::LuRedo2 => LU_REDO2,
            #[cfg(any(LuRedoDot, icondata_include_all))]
            LuIcon::LuRedoDot => LU_REDO_DOT,
            #[cfg(any(LuRefreshCcw, icondata_include_all))]
            LuIcon::LuRefreshCcw => LU_REFRESH_CCW,
            #[cfg(any(LuRefreshCcwDot, icondata_include_all))]
            LuIcon::LuRefreshCcwDot => LU_REFRESH_CCW_DOT,
            #[cfg(any(LuRefreshCw, icondata_include_all))]
            LuIcon::LuRefreshCw => LU_REFRESH_CW,
            #[cfg(any(LuRefreshCwOff, icondata_include_all))]
            LuIcon::LuRefreshCwOff => LU_REFRESH_CW_OFF,
            #[cfg(any(LuRefrigerator, icondata_include_all))]
            LuIcon::LuRefrigerator => LU_REFRIGERATOR,
            #[cfg(any(LuRegex, icondata_include_all))]
            LuIcon::LuRegex => LU_REGEX,
            #[cfg(any(LuRemoveFormatting, icondata_include_all))]
            LuIcon::LuRemoveFormatting => LU_REMOVE_FORMATTING,
            #[cfg(any(LuRepeat, icondata_include_all))]
            LuIcon::LuRepeat => LU_REPEAT,
            #[cfg(any(LuRepeat1, icondata_include_all))]
            LuIcon::LuRepeat1 => LU_REPEAT1,
            #[cfg(any(LuRepeat2, icondata_include_all))]
            LuIcon::LuRepeat2 => LU_REPEAT2,
            #[cfg(any(LuReplace, icondata_include_all))]
            LuIcon::LuReplace => LU_REPLACE,
            #[cfg(any(LuReplaceAll, icondata_include_all))]
            LuIcon::LuReplaceAll => LU_REPLACE_ALL,
            #[cfg(any(LuReply, icondata_include_all))]
            LuIcon::LuReply => LU_REPLY,
            #[cfg(any(LuReplyAll, icondata_include_all))]
            LuIcon::LuReplyAll => LU_REPLY_ALL,
            #[cfg(any(LuRewind, icondata_include_all))]
            LuIcon::LuRewind => LU_REWIND,
            #[cfg(any(LuRocket, icondata_include_all))]
            LuIcon::LuRocket => LU_ROCKET,
            #[cfg(any(LuRockingChair, icondata_include_all))]
            LuIcon::LuRockingChair => LU_ROCKING_CHAIR,
            #[cfg(any(LuRollerCoaster, icondata_include_all))]
            LuIcon::LuRollerCoaster => LU_ROLLER_COASTER,
            #[cfg(any(LuRotate3d, icondata_include_all))]
            LuIcon::LuRotate3d => LU_ROTATE3D,
            #[cfg(any(LuRotateCcw, icondata_include_all))]
            LuIcon::LuRotateCcw => LU_ROTATE_CCW,
            #[cfg(any(LuRotateCw, icondata_include_all))]
            LuIcon::LuRotateCw => LU_ROTATE_CW,
            #[cfg(any(LuRouter, icondata_include_all))]
            LuIcon::LuRouter => LU_ROUTER,
            #[cfg(any(LuRows, icondata_include_all))]
            LuIcon::LuRows => LU_ROWS,
            #[cfg(any(LuRss, icondata_include_all))]
            LuIcon::LuRss => LU_RSS,
            #[cfg(any(LuRuler, icondata_include_all))]
            LuIcon::LuRuler => LU_RULER,
            #[cfg(any(LuRussianRuble, icondata_include_all))]
            LuIcon::LuRussianRuble => LU_RUSSIAN_RUBLE,
            #[cfg(any(LuSailboat, icondata_include_all))]
            LuIcon::LuSailboat => LU_SAILBOAT,
            #[cfg(any(LuSalad, icondata_include_all))]
            LuIcon::LuSalad => LU_SALAD,
            #[cfg(any(LuSandwich, icondata_include_all))]
            LuIcon::LuSandwich => LU_SANDWICH,
            #[cfg(any(LuSatellite, icondata_include_all))]
            LuIcon::LuSatellite => LU_SATELLITE,
            #[cfg(any(LuSatelliteDish, icondata_include_all))]
            LuIcon::LuSatelliteDish => LU_SATELLITE_DISH,
            #[cfg(any(LuSave, icondata_include_all))]
            LuIcon::LuSave => LU_SAVE,
            #[cfg(any(LuSaveAll, icondata_include_all))]
            LuIcon::LuSaveAll => LU_SAVE_ALL,
            #[cfg(any(LuScale, icondata_include_all))]
            LuIcon::LuScale => LU_SCALE,
            #[cfg(any(LuScale3d, icondata_include_all))]
            LuIcon::LuScale3d => LU_SCALE3D,
            #[cfg(any(LuScaling, icondata_include_all))]
            LuIcon::LuScaling => LU_SCALING,
            #[cfg(any(LuScan, icondata_include_all))]
            LuIcon::LuScan => LU_SCAN,
            #[cfg(any(LuScanFace, icondata_include_all))]
            LuIcon::LuScanFace => LU_SCAN_FACE,
            #[cfg(any(LuScanLine, icondata_include_all))]
            LuIcon::LuScanLine => LU_SCAN_LINE,
            #[cfg(any(LuScatterChart, icondata_include_all))]
            LuIcon::LuScatterChart => LU_SCATTER_CHART,
            #[cfg(any(LuSchool, icondata_include_all))]
            LuIcon::LuSchool => LU_SCHOOL,
            #[cfg(any(LuSchool2, icondata_include_all))]
            LuIcon::LuSchool2 => LU_SCHOOL2,
            #[cfg(any(LuScissors, icondata_include_all))]
            LuIcon::LuScissors => LU_SCISSORS,
            #[cfg(any(LuScissorsLineDashed, icondata_include_all))]
            LuIcon::LuScissorsLineDashed => LU_SCISSORS_LINE_DASHED,
            #[cfg(any(LuScissorsSquare, icondata_include_all))]
            LuIcon::LuScissorsSquare => LU_SCISSORS_SQUARE,
            #[cfg(any(LuScissorsSquareDashedBottom, icondata_include_all))]
            LuIcon::LuScissorsSquareDashedBottom => LU_SCISSORS_SQUARE_DASHED_BOTTOM,
            #[cfg(any(LuScreenShare, icondata_include_all))]
            LuIcon::LuScreenShare => LU_SCREEN_SHARE,
            #[cfg(any(LuScreenShareOff, icondata_include_all))]
            LuIcon::LuScreenShareOff => LU_SCREEN_SHARE_OFF,
            #[cfg(any(LuScroll, icondata_include_all))]
            LuIcon::LuScroll => LU_SCROLL,
            #[cfg(any(LuScrollText, icondata_include_all))]
            LuIcon::LuScrollText => LU_SCROLL_TEXT,
            #[cfg(any(LuSearch, icondata_include_all))]
            LuIcon::LuSearch => LU_SEARCH,
            #[cfg(any(LuSearchCheck, icondata_include_all))]
            LuIcon::LuSearchCheck => LU_SEARCH_CHECK,
            #[cfg(any(LuSearchCode, icondata_include_all))]
            LuIcon::LuSearchCode => LU_SEARCH_CODE,
            #[cfg(any(LuSearchSlash, icondata_include_all))]
            LuIcon::LuSearchSlash => LU_SEARCH_SLASH,
            #[cfg(any(LuSearchX, icondata_include_all))]
            LuIcon::LuSearchX => LU_SEARCH_X,
            #[cfg(any(LuSend, icondata_include_all))]
            LuIcon::LuSend => LU_SEND,
            #[cfg(any(LuSendHorizonal, icondata_include_all))]
            LuIcon::LuSendHorizonal => LU_SEND_HORIZONAL,
            #[cfg(any(LuSendToBack, icondata_include_all))]
            LuIcon::LuSendToBack => LU_SEND_TO_BACK,
            #[cfg(any(LuSeparatorHorizontal, icondata_include_all))]
            LuIcon::LuSeparatorHorizontal => LU_SEPARATOR_HORIZONTAL,
            #[cfg(any(LuSeparatorVertical, icondata_include_all))]
            LuIcon::LuSeparatorVertical => LU_SEPARATOR_VERTICAL,
            #[cfg(any(LuServer, icondata_include_all))]
            LuIcon::LuServer => LU_SERVER,
            #[cfg(any(LuServerCog, icondata_include_all))]
            LuIcon::LuServerCog => LU_SERVER_COG,
            #[cfg(any(LuServerCrash, icondata_include_all))]
            LuIcon::LuServerCrash => LU_SERVER_CRASH,
            #[cfg(any(LuServerOff, icondata_include_all))]
            LuIcon::LuServerOff => LU_SERVER_OFF,
            #[cfg(any(LuSettings, icondata_include_all))]
            LuIcon::LuSettings => LU_SETTINGS,
            #[cfg(any(LuSettings2, icondata_include_all))]
            LuIcon::LuSettings2 => LU_SETTINGS2,
            #[cfg(any(LuShapes, icondata_include_all))]
            LuIcon::LuShapes => LU_SHAPES,
            #[cfg(any(LuShare, icondata_include_all))]
            LuIcon::LuShare => LU_SHARE,
            #[cfg(any(LuShare2, icondata_include_all))]
            LuIcon::LuShare2 => LU_SHARE2,
            #[cfg(any(LuSheet, icondata_include_all))]
            LuIcon::LuSheet => LU_SHEET,
            #[cfg(any(LuShell, icondata_include_all))]
            LuIcon::LuShell => LU_SHELL,
            #[cfg(any(LuShield, icondata_include_all))]
            LuIcon::LuShield => LU_SHIELD,
            #[cfg(any(LuShieldAlert, icondata_include_all))]
            LuIcon::LuShieldAlert => LU_SHIELD_ALERT,
            #[cfg(any(LuShieldCheck, icondata_include_all))]
            LuIcon::LuShieldCheck => LU_SHIELD_CHECK,
            #[cfg(any(LuShieldClose, icondata_include_all))]
            LuIcon::LuShieldClose => LU_SHIELD_CLOSE,
            #[cfg(any(LuShieldOff, icondata_include_all))]
            LuIcon::LuShieldOff => LU_SHIELD_OFF,
            #[cfg(any(LuShieldQuestion, icondata_include_all))]
            LuIcon::LuShieldQuestion => LU_SHIELD_QUESTION,
            #[cfg(any(LuShip, icondata_include_all))]
            LuIcon::LuShip => LU_SHIP,
            #[cfg(any(LuShipWheel, icondata_include_all))]
            LuIcon::LuShipWheel => LU_SHIP_WHEEL,
            #[cfg(any(LuShirt, icondata_include_all))]
            LuIcon::LuShirt => LU_SHIRT,
            #[cfg(any(LuShoppingBag, icondata_include_all))]
            LuIcon::LuShoppingBag => LU_SHOPPING_BAG,
            #[cfg(any(LuShoppingBasket, icondata_include_all))]
            LuIcon::LuShoppingBasket => LU_SHOPPING_BASKET,
            #[cfg(any(LuShoppingCart, icondata_include_all))]
            LuIcon::LuShoppingCart => LU_SHOPPING_CART,
            #[cfg(any(LuShovel, icondata_include_all))]
            LuIcon::LuShovel => LU_SHOVEL,
            #[cfg(any(LuShowerHead, icondata_include_all))]
            LuIcon::LuShowerHead => LU_SHOWER_HEAD,
            #[cfg(any(LuShrink, icondata_include_all))]
            LuIcon::LuShrink => LU_SHRINK,
            #[cfg(any(LuShrub, icondata_include_all))]
            LuIcon::LuShrub => LU_SHRUB,
            #[cfg(any(LuShuffle, icondata_include_all))]
            LuIcon::LuShuffle => LU_SHUFFLE,
            #[cfg(any(LuSigma, icondata_include_all))]
            LuIcon::LuSigma => LU_SIGMA,
            #[cfg(any(LuSigmaSquare, icondata_include_all))]
            LuIcon::LuSigmaSquare => LU_SIGMA_SQUARE,
            #[cfg(any(LuSignal, icondata_include_all))]
            LuIcon::LuSignal => LU_SIGNAL,
            #[cfg(any(LuSignalHigh, icondata_include_all))]
            LuIcon::LuSignalHigh => LU_SIGNAL_HIGH,
            #[cfg(any(LuSignalLow, icondata_include_all))]
            LuIcon::LuSignalLow => LU_SIGNAL_LOW,
            #[cfg(any(LuSignalMedium, icondata_include_all))]
            LuIcon::LuSignalMedium => LU_SIGNAL_MEDIUM,
            #[cfg(any(LuSignalZero, icondata_include_all))]
            LuIcon::LuSignalZero => LU_SIGNAL_ZERO,
            #[cfg(any(LuSiren, icondata_include_all))]
            LuIcon::LuSiren => LU_SIREN,
            #[cfg(any(LuSkipBack, icondata_include_all))]
            LuIcon::LuSkipBack => LU_SKIP_BACK,
            #[cfg(any(LuSkipForward, icondata_include_all))]
            LuIcon::LuSkipForward => LU_SKIP_FORWARD,
            #[cfg(any(LuSkull, icondata_include_all))]
            LuIcon::LuSkull => LU_SKULL,
            #[cfg(any(LuSlack, icondata_include_all))]
            LuIcon::LuSlack => LU_SLACK,
            #[cfg(any(LuSlice, icondata_include_all))]
            LuIcon::LuSlice => LU_SLICE,
            #[cfg(any(LuSliders, icondata_include_all))]
            LuIcon::LuSliders => LU_SLIDERS,
            #[cfg(any(LuSlidersHorizontal, icondata_include_all))]
            LuIcon::LuSlidersHorizontal => LU_SLIDERS_HORIZONTAL,
            #[cfg(any(LuSmartphone, icondata_include_all))]
            LuIcon::LuSmartphone => LU_SMARTPHONE,
            #[cfg(any(LuSmartphoneCharging, icondata_include_all))]
            LuIcon::LuSmartphoneCharging => LU_SMARTPHONE_CHARGING,
            #[cfg(any(LuSmartphoneNfc, icondata_include_all))]
            LuIcon::LuSmartphoneNfc => LU_SMARTPHONE_NFC,
            #[cfg(any(LuSmile, icondata_include_all))]
            LuIcon::LuSmile => LU_SMILE,
            #[cfg(any(LuSmilePlus, icondata_include_all))]
            LuIcon::LuSmilePlus => LU_SMILE_PLUS,
            #[cfg(any(LuSnail, icondata_include_all))]
            LuIcon::LuSnail => LU_SNAIL,
            #[cfg(any(LuSnowflake, icondata_include_all))]
            LuIcon::LuSnowflake => LU_SNOWFLAKE,
            #[cfg(any(LuSofa, icondata_include_all))]
            LuIcon::LuSofa => LU_SOFA,
            #[cfg(any(LuSoup, icondata_include_all))]
            LuIcon::LuSoup => LU_SOUP,
            #[cfg(any(LuSpace, icondata_include_all))]
            LuIcon::LuSpace => LU_SPACE,
            #[cfg(any(LuSpade, icondata_include_all))]
            LuIcon::LuSpade => LU_SPADE,
            #[cfg(any(LuSparkle, icondata_include_all))]
            LuIcon::LuSparkle => LU_SPARKLE,
            #[cfg(any(LuSparkles, icondata_include_all))]
            LuIcon::LuSparkles => LU_SPARKLES,
            #[cfg(any(LuSpeaker, icondata_include_all))]
            LuIcon::LuSpeaker => LU_SPEAKER,
            #[cfg(any(LuSpellCheck, icondata_include_all))]
            LuIcon::LuSpellCheck => LU_SPELL_CHECK,
            #[cfg(any(LuSpellCheck2, icondata_include_all))]
            LuIcon::LuSpellCheck2 => LU_SPELL_CHECK2,
            #[cfg(any(LuSpline, icondata_include_all))]
            LuIcon::LuSpline => LU_SPLINE,
            #[cfg(any(LuSplit, icondata_include_all))]
            LuIcon::LuSplit => LU_SPLIT,
            #[cfg(any(LuSplitSquareHorizontal, icondata_include_all))]
            LuIcon::LuSplitSquareHorizontal => LU_SPLIT_SQUARE_HORIZONTAL,
            #[cfg(any(LuSplitSquareVertical, icondata_include_all))]
            LuIcon::LuSplitSquareVertical => LU_SPLIT_SQUARE_VERTICAL,
            #[cfg(any(LuSprayCan, icondata_include_all))]
            LuIcon::LuSprayCan => LU_SPRAY_CAN,
            #[cfg(any(LuSprout, icondata_include_all))]
            LuIcon::LuSprout => LU_SPROUT,
            #[cfg(any(LuSquare, icondata_include_all))]
            LuIcon::LuSquare => LU_SQUARE,
            #[cfg(any(LuSquareAsterisk, icondata_include_all))]
            LuIcon::LuSquareAsterisk => LU_SQUARE_ASTERISK,
            #[cfg(any(LuSquareCode, icondata_include_all))]
            LuIcon::LuSquareCode => LU_SQUARE_CODE,
            #[cfg(any(LuSquareDashedBottom, icondata_include_all))]
            LuIcon::LuSquareDashedBottom => LU_SQUARE_DASHED_BOTTOM,
            #[cfg(any(LuSquareDashedBottomCode, icondata_include_all))]
            LuIcon::LuSquareDashedBottomCode => LU_SQUARE_DASHED_BOTTOM_CODE,
            #[cfg(any(LuSquareDot, icondata_include_all))]
            LuIcon::LuSquareDot => LU_SQUARE_DOT,
            #[cfg(any(LuSquareEqual, icondata_include_all))]
            LuIcon::LuSquareEqual => LU_SQUARE_EQUAL,
            #[cfg(any(LuSquareSlash, icondata_include_all))]
            LuIcon::LuSquareSlash => LU_SQUARE_SLASH,
            #[cfg(any(LuSquareStack, icondata_include_all))]
            LuIcon::LuSquareStack => LU_SQUARE_STACK,
            #[cfg(any(LuSquirrel, icondata_include_all))]
            LuIcon::LuSquirrel => LU_SQUIRREL,
            #[cfg(any(LuStamp, icondata_include_all))]
            LuIcon::LuStamp => LU_STAMP,
            #[cfg(any(LuStar, icondata_include_all))]
            LuIcon::LuStar => LU_STAR,
            #[cfg(any(LuStarHalf, icondata_include_all))]
            LuIcon::LuStarHalf => LU_STAR_HALF,
            #[cfg(any(LuStarOff, icondata_include_all))]
            LuIcon::LuStarOff => LU_STAR_OFF,
            #[cfg(any(LuStepBack, icondata_include_all))]
            LuIcon::LuStepBack => LU_STEP_BACK,
            #[cfg(any(LuStepForward, icondata_include_all))]
            LuIcon::LuStepForward => LU_STEP_FORWARD,
            #[cfg(any(LuStethoscope, icondata_include_all))]
            LuIcon::LuStethoscope => LU_STETHOSCOPE,
            #[cfg(any(LuSticker, icondata_include_all))]
            LuIcon::LuSticker => LU_STICKER,
            #[cfg(any(LuStickyNote, icondata_include_all))]
            LuIcon::LuStickyNote => LU_STICKY_NOTE,
            #[cfg(any(LuStopCircle, icondata_include_all))]
            LuIcon::LuStopCircle => LU_STOP_CIRCLE,
            #[cfg(any(LuStore, icondata_include_all))]
            LuIcon::LuStore => LU_STORE,
            #[cfg(any(LuStretchHorizontal, icondata_include_all))]
            LuIcon::LuStretchHorizontal => LU_STRETCH_HORIZONTAL,
            #[cfg(any(LuStretchVertical, icondata_include_all))]
            LuIcon::LuStretchVertical => LU_STRETCH_VERTICAL,
            #[cfg(any(LuStrikethrough, icondata_include_all))]
            LuIcon::LuStrikethrough => LU_STRIKETHROUGH,
            #[cfg(any(LuSubscript, icondata_include_all))]
            LuIcon::LuSubscript => LU_SUBSCRIPT,
            #[cfg(any(LuSubtitles, icondata_include_all))]
            LuIcon::LuSubtitles => LU_SUBTITLES,
            #[cfg(any(LuSun, icondata_include_all))]
            LuIcon::LuSun => LU_SUN,
            #[cfg(any(LuSunDim, icondata_include_all))]
            LuIcon::LuSunDim => LU_SUN_DIM,
            #[cfg(any(LuSunMedium, icondata_include_all))]
            LuIcon::LuSunMedium => LU_SUN_MEDIUM,
            #[cfg(any(LuSunMoon, icondata_include_all))]
            LuIcon::LuSunMoon => LU_SUN_MOON,
            #[cfg(any(LuSunSnow, icondata_include_all))]
            LuIcon::LuSunSnow => LU_SUN_SNOW,
            #[cfg(any(LuSunrise, icondata_include_all))]
            LuIcon::LuSunrise => LU_SUNRISE,
            #[cfg(any(LuSunset, icondata_include_all))]
            LuIcon::LuSunset => LU_SUNSET,
            #[cfg(any(LuSuperscript, icondata_include_all))]
            LuIcon::LuSuperscript => LU_SUPERSCRIPT,
            #[cfg(any(LuSwissFranc, icondata_include_all))]
            LuIcon::LuSwissFranc => LU_SWISS_FRANC,
            #[cfg(any(LuSwitchCamera, icondata_include_all))]
            LuIcon::LuSwitchCamera => LU_SWITCH_CAMERA,
            #[cfg(any(LuSword, icondata_include_all))]
            LuIcon::LuSword => LU_SWORD,
            #[cfg(any(LuSwords, icondata_include_all))]
            LuIcon::LuSwords => LU_SWORDS,
            #[cfg(any(LuSyringe, icondata_include_all))]
            LuIcon::LuSyringe => LU_SYRINGE,
            #[cfg(any(LuTable, icondata_include_all))]
            LuIcon::LuTable => LU_TABLE,
            #[cfg(any(LuTable2, icondata_include_all))]
            LuIcon::LuTable2 => LU_TABLE2,
            #[cfg(any(LuTableProperties, icondata_include_all))]
            LuIcon::LuTableProperties => LU_TABLE_PROPERTIES,
            #[cfg(any(LuTablet, icondata_include_all))]
            LuIcon::LuTablet => LU_TABLET,
            #[cfg(any(LuTablets, icondata_include_all))]
            LuIcon::LuTablets => LU_TABLETS,
            #[cfg(any(LuTag, icondata_include_all))]
            LuIcon::LuTag => LU_TAG,
            #[cfg(any(LuTags, icondata_include_all))]
            LuIcon::LuTags => LU_TAGS,
            #[cfg(any(LuTally1, icondata_include_all))]
            LuIcon::LuTally1 => LU_TALLY1,
            #[cfg(any(LuTally2, icondata_include_all))]
            LuIcon::LuTally2 => LU_TALLY2,
            #[cfg(any(LuTally3, icondata_include_all))]
            LuIcon::LuTally3 => LU_TALLY3,
            #[cfg(any(LuTally4, icondata_include_all))]
            LuIcon::LuTally4 => LU_TALLY4,
            #[cfg(any(LuTally5, icondata_include_all))]
            LuIcon::LuTally5 => LU_TALLY5,
            #[cfg(any(LuTarget, icondata_include_all))]
            LuIcon::LuTarget => LU_TARGET,
            #[cfg(any(LuTent, icondata_include_all))]
            LuIcon::LuTent => LU_TENT,
            #[cfg(any(LuTerminal, icondata_include_all))]
            LuIcon::LuTerminal => LU_TERMINAL,
            #[cfg(any(LuTerminalSquare, icondata_include_all))]
            LuIcon::LuTerminalSquare => LU_TERMINAL_SQUARE,
            #[cfg(any(LuTestTube, icondata_include_all))]
            LuIcon::LuTestTube => LU_TEST_TUBE,
            #[cfg(any(LuTestTube2, icondata_include_all))]
            LuIcon::LuTestTube2 => LU_TEST_TUBE2,
            #[cfg(any(LuTestTubes, icondata_include_all))]
            LuIcon::LuTestTubes => LU_TEST_TUBES,
            #[cfg(any(LuText, icondata_include_all))]
            LuIcon::LuText => LU_TEXT,
            #[cfg(any(LuTextCursor, icondata_include_all))]
            LuIcon::LuTextCursor => LU_TEXT_CURSOR,
            #[cfg(any(LuTextCursorInput, icondata_include_all))]
            LuIcon::LuTextCursorInput => LU_TEXT_CURSOR_INPUT,
            #[cfg(any(LuTextQuote, icondata_include_all))]
            LuIcon::LuTextQuote => LU_TEXT_QUOTE,
            #[cfg(any(LuTextSelect, icondata_include_all))]
            LuIcon::LuTextSelect => LU_TEXT_SELECT,
            #[cfg(any(LuThermometer, icondata_include_all))]
            LuIcon::LuThermometer => LU_THERMOMETER,
            #[cfg(any(LuThermometerSnowflake, icondata_include_all))]
            LuIcon::LuThermometerSnowflake => LU_THERMOMETER_SNOWFLAKE,
            #[cfg(any(LuThermometerSun, icondata_include_all))]
            LuIcon::LuThermometerSun => LU_THERMOMETER_SUN,
            #[cfg(any(LuThumbsDown, icondata_include_all))]
            LuIcon::LuThumbsDown => LU_THUMBS_DOWN,
            #[cfg(any(LuThumbsUp, icondata_include_all))]
            LuIcon::LuThumbsUp => LU_THUMBS_UP,
            #[cfg(any(LuTicket, icondata_include_all))]
            LuIcon::LuTicket => LU_TICKET,
            #[cfg(any(LuTimer, icondata_include_all))]
            LuIcon::LuTimer => LU_TIMER,
            #[cfg(any(LuTimerOff, icondata_include_all))]
            LuIcon::LuTimerOff => LU_TIMER_OFF,
            #[cfg(any(LuTimerReset, icondata_include_all))]
            LuIcon::LuTimerReset => LU_TIMER_RESET,
            #[cfg(any(LuToggleLeft, icondata_include_all))]
            LuIcon::LuToggleLeft => LU_TOGGLE_LEFT,
            #[cfg(any(LuToggleRight, icondata_include_all))]
            LuIcon::LuToggleRight => LU_TOGGLE_RIGHT,
            #[cfg(any(LuTornado, icondata_include_all))]
            LuIcon::LuTornado => LU_TORNADO,
            #[cfg(any(LuTouchpad, icondata_include_all))]
            LuIcon::LuTouchpad => LU_TOUCHPAD,
            #[cfg(any(LuTouchpadOff, icondata_include_all))]
            LuIcon::LuTouchpadOff => LU_TOUCHPAD_OFF,
            #[cfg(any(LuTowerControl, icondata_include_all))]
            LuIcon::LuTowerControl => LU_TOWER_CONTROL,
            #[cfg(any(LuToyBrick, icondata_include_all))]
            LuIcon::LuToyBrick => LU_TOY_BRICK,
            #[cfg(any(LuTractor, icondata_include_all))]
            LuIcon::LuTractor => LU_TRACTOR,
            #[cfg(any(LuTrafficCone, icondata_include_all))]
            LuIcon::LuTrafficCone => LU_TRAFFIC_CONE,
            #[cfg(any(LuTrainFront, icondata_include_all))]
            LuIcon::LuTrainFront => LU_TRAIN_FRONT,
            #[cfg(any(LuTrainFrontTunnel, icondata_include_all))]
            LuIcon::LuTrainFrontTunnel => LU_TRAIN_FRONT_TUNNEL,
            #[cfg(any(LuTrainTrack, icondata_include_all))]
            LuIcon::LuTrainTrack => LU_TRAIN_TRACK,
            #[cfg(any(LuTramFront, icondata_include_all))]
            LuIcon::LuTramFront => LU_TRAM_FRONT,
            #[cfg(any(LuTrash, icondata_include_all))]
            LuIcon::LuTrash => LU_TRASH,
            #[cfg(any(LuTrash2, icondata_include_all))]
            LuIcon::LuTrash2 => LU_TRASH2,
            #[cfg(any(LuTreeDeciduous, icondata_include_all))]
            LuIcon::LuTreeDeciduous => LU_TREE_DECIDUOUS,
            #[cfg(any(LuTreePine, icondata_include_all))]
            LuIcon::LuTreePine => LU_TREE_PINE,
            #[cfg(any(LuTrees, icondata_include_all))]
            LuIcon::LuTrees => LU_TREES,
            #[cfg(any(LuTrello, icondata_include_all))]
            LuIcon::LuTrello => LU_TRELLO,
            #[cfg(any(LuTrendingDown, icondata_include_all))]
            LuIcon::LuTrendingDown => LU_TRENDING_DOWN,
            #[cfg(any(LuTrendingUp, icondata_include_all))]
            LuIcon::LuTrendingUp => LU_TRENDING_UP,
            #[cfg(any(LuTriangle, icondata_include_all))]
            LuIcon::LuTriangle => LU_TRIANGLE,
            #[cfg(any(LuTriangleRight, icondata_include_all))]
            LuIcon::LuTriangleRight => LU_TRIANGLE_RIGHT,
            #[cfg(any(LuTrophy, icondata_include_all))]
            LuIcon::LuTrophy => LU_TROPHY,
            #[cfg(any(LuTruck, icondata_include_all))]
            LuIcon::LuTruck => LU_TRUCK,
            #[cfg(any(LuTurtle, icondata_include_all))]
            LuIcon::LuTurtle => LU_TURTLE,
            #[cfg(any(LuTv, icondata_include_all))]
            LuIcon::LuTv => LU_TV,
            #[cfg(any(LuTv2, icondata_include_all))]
            LuIcon::LuTv2 => LU_TV2,
            #[cfg(any(LuTwitch, icondata_include_all))]
            LuIcon::LuTwitch => LU_TWITCH,
            #[cfg(any(LuTwitter, icondata_include_all))]
            LuIcon::LuTwitter => LU_TWITTER,
            #[cfg(any(LuType, icondata_include_all))]
            LuIcon::LuType => LU_TYPE,
            #[cfg(any(LuUmbrella, icondata_include_all))]
            LuIcon::LuUmbrella => LU_UMBRELLA,
            #[cfg(any(LuUnderline, icondata_include_all))]
            LuIcon::LuUnderline => LU_UNDERLINE,
            #[cfg(any(LuUndo, icondata_include_all))]
            LuIcon::LuUndo => LU_UNDO,
            #[cfg(any(LuUndo2, icondata_include_all))]
            LuIcon::LuUndo2 => LU_UNDO2,
            #[cfg(any(LuUndoDot, icondata_include_all))]
            LuIcon::LuUndoDot => LU_UNDO_DOT,
            #[cfg(any(LuUnfoldHorizontal, icondata_include_all))]
            LuIcon::LuUnfoldHorizontal => LU_UNFOLD_HORIZONTAL,
            #[cfg(any(LuUnfoldVertical, icondata_include_all))]
            LuIcon::LuUnfoldVertical => LU_UNFOLD_VERTICAL,
            #[cfg(any(LuUngroup, icondata_include_all))]
            LuIcon::LuUngroup => LU_UNGROUP,
            #[cfg(any(LuUnlink, icondata_include_all))]
            LuIcon::LuUnlink => LU_UNLINK,
            #[cfg(any(LuUnlink2, icondata_include_all))]
            LuIcon::LuUnlink2 => LU_UNLINK2,
            #[cfg(any(LuUnlock, icondata_include_all))]
            LuIcon::LuUnlock => LU_UNLOCK,
            #[cfg(any(LuUnplug, icondata_include_all))]
            LuIcon::LuUnplug => LU_UNPLUG,
            #[cfg(any(LuUpload, icondata_include_all))]
            LuIcon::LuUpload => LU_UPLOAD,
            #[cfg(any(LuUploadCloud, icondata_include_all))]
            LuIcon::LuUploadCloud => LU_UPLOAD_CLOUD,
            #[cfg(any(LuUsb, icondata_include_all))]
            LuIcon::LuUsb => LU_USB,
            #[cfg(any(LuUser, icondata_include_all))]
            LuIcon::LuUser => LU_USER,
            #[cfg(any(LuUser2, icondata_include_all))]
            LuIcon::LuUser2 => LU_USER2,
            #[cfg(any(LuUserCheck, icondata_include_all))]
            LuIcon::LuUserCheck => LU_USER_CHECK,
            #[cfg(any(LuUserCheck2, icondata_include_all))]
            LuIcon::LuUserCheck2 => LU_USER_CHECK2,
            #[cfg(any(LuUserCircle, icondata_include_all))]
            LuIcon::LuUserCircle => LU_USER_CIRCLE,
            #[cfg(any(LuUserCircle2, icondata_include_all))]
            LuIcon::LuUserCircle2 => LU_USER_CIRCLE2,
            #[cfg(any(LuUserCog, icondata_include_all))]
            LuIcon::LuUserCog => LU_USER_COG,
            #[cfg(any(LuUserCog2, icondata_include_all))]
            LuIcon::LuUserCog2 => LU_USER_COG2,
            #[cfg(any(LuUserMinus, icondata_include_all))]
            LuIcon::LuUserMinus => LU_USER_MINUS,
            #[cfg(any(LuUserMinus2, icondata_include_all))]
            LuIcon::LuUserMinus2 => LU_USER_MINUS2,
            #[cfg(any(LuUserPlus, icondata_include_all))]
            LuIcon::LuUserPlus => LU_USER_PLUS,
            #[cfg(any(LuUserPlus2, icondata_include_all))]
            LuIcon::LuUserPlus2 => LU_USER_PLUS2,
            #[cfg(any(LuUserSquare, icondata_include_all))]
            LuIcon::LuUserSquare => LU_USER_SQUARE,
            #[cfg(any(LuUserSquare2, icondata_include_all))]
            LuIcon::LuUserSquare2 => LU_USER_SQUARE2,
            #[cfg(any(LuUserX, icondata_include_all))]
            LuIcon::LuUserX => LU_USER_X,
            #[cfg(any(LuUserX2, icondata_include_all))]
            LuIcon::LuUserX2 => LU_USER_X2,
            #[cfg(any(LuUsers, icondata_include_all))]
            LuIcon::LuUsers => LU_USERS,
            #[cfg(any(LuUsers2, icondata_include_all))]
            LuIcon::LuUsers2 => LU_USERS2,
            #[cfg(any(LuUtensils, icondata_include_all))]
            LuIcon::LuUtensils => LU_UTENSILS,
            #[cfg(any(LuUtensilsCrossed, icondata_include_all))]
            LuIcon::LuUtensilsCrossed => LU_UTENSILS_CROSSED,
            #[cfg(any(LuUtilityPole, icondata_include_all))]
            LuIcon::LuUtilityPole => LU_UTILITY_POLE,
            #[cfg(any(LuVariable, icondata_include_all))]
            LuIcon::LuVariable => LU_VARIABLE,
            #[cfg(any(LuVegan, icondata_include_all))]
            LuIcon::LuVegan => LU_VEGAN,
            #[cfg(any(LuVenetianMask, icondata_include_all))]
            LuIcon::LuVenetianMask => LU_VENETIAN_MASK,
            #[cfg(any(LuVibrate, icondata_include_all))]
            LuIcon::LuVibrate => LU_VIBRATE,
            #[cfg(any(LuVibrateOff, icondata_include_all))]
            LuIcon::LuVibrateOff => LU_VIBRATE_OFF,
            #[cfg(any(LuVideo, icondata_include_all))]
            LuIcon::LuVideo => LU_VIDEO,
            #[cfg(any(LuVideoOff, icondata_include_all))]
            LuIcon::LuVideoOff => LU_VIDEO_OFF,
            #[cfg(any(LuVideotape, icondata_include_all))]
            LuIcon::LuVideotape => LU_VIDEOTAPE,
            #[cfg(any(LuView, icondata_include_all))]
            LuIcon::LuView => LU_VIEW,
            #[cfg(any(LuVoicemail, icondata_include_all))]
            LuIcon::LuVoicemail => LU_VOICEMAIL,
            #[cfg(any(LuVolume, icondata_include_all))]
            LuIcon::LuVolume => LU_VOLUME,
            #[cfg(any(LuVolume1, icondata_include_all))]
            LuIcon::LuVolume1 => LU_VOLUME1,
            #[cfg(any(LuVolume2, icondata_include_all))]
            LuIcon::LuVolume2 => LU_VOLUME2,
            #[cfg(any(LuVolumeX, icondata_include_all))]
            LuIcon::LuVolumeX => LU_VOLUME_X,
            #[cfg(any(LuVote, icondata_include_all))]
            LuIcon::LuVote => LU_VOTE,
            #[cfg(any(LuWallet, icondata_include_all))]
            LuIcon::LuWallet => LU_WALLET,
            #[cfg(any(LuWallet2, icondata_include_all))]
            LuIcon::LuWallet2 => LU_WALLET2,
            #[cfg(any(LuWalletCards, icondata_include_all))]
            LuIcon::LuWalletCards => LU_WALLET_CARDS,
            #[cfg(any(LuWallpaper, icondata_include_all))]
            LuIcon::LuWallpaper => LU_WALLPAPER,
            #[cfg(any(LuWand, icondata_include_all))]
            LuIcon::LuWand => LU_WAND,
            #[cfg(any(LuWand2, icondata_include_all))]
            LuIcon::LuWand2 => LU_WAND2,
            #[cfg(any(LuWarehouse, icondata_include_all))]
            LuIcon::LuWarehouse => LU_WAREHOUSE,
            #[cfg(any(LuWatch, icondata_include_all))]
            LuIcon::LuWatch => LU_WATCH,
            #[cfg(any(LuWaves, icondata_include_all))]
            LuIcon::LuWaves => LU_WAVES,
            #[cfg(any(LuWebcam, icondata_include_all))]
            LuIcon::LuWebcam => LU_WEBCAM,
            #[cfg(any(LuWebhook, icondata_include_all))]
            LuIcon::LuWebhook => LU_WEBHOOK,
            #[cfg(any(LuWheat, icondata_include_all))]
            LuIcon::LuWheat => LU_WHEAT,
            #[cfg(any(LuWheatOff, icondata_include_all))]
            LuIcon::LuWheatOff => LU_WHEAT_OFF,
            #[cfg(any(LuWholeWord, icondata_include_all))]
            LuIcon::LuWholeWord => LU_WHOLE_WORD,
            #[cfg(any(LuWifi, icondata_include_all))]
            LuIcon::LuWifi => LU_WIFI,
            #[cfg(any(LuWifiOff, icondata_include_all))]
            LuIcon::LuWifiOff => LU_WIFI_OFF,
            #[cfg(any(LuWind, icondata_include_all))]
            LuIcon::LuWind => LU_WIND,
            #[cfg(any(LuWine, icondata_include_all))]
            LuIcon::LuWine => LU_WINE,
            #[cfg(any(LuWineOff, icondata_include_all))]
            LuIcon::LuWineOff => LU_WINE_OFF,
            #[cfg(any(LuWorkflow, icondata_include_all))]
            LuIcon::LuWorkflow => LU_WORKFLOW,
            #[cfg(any(LuWrapText, icondata_include_all))]
            LuIcon::LuWrapText => LU_WRAP_TEXT,
            #[cfg(any(LuWrench, icondata_include_all))]
            LuIcon::LuWrench => LU_WRENCH,
            #[cfg(any(LuX, icondata_include_all))]
            LuIcon::LuX => LU_X,
            #[cfg(any(LuXCircle, icondata_include_all))]
            LuIcon::LuXCircle => LU_X_CIRCLE,
            #[cfg(any(LuXOctagon, icondata_include_all))]
            LuIcon::LuXOctagon => LU_X_OCTAGON,
            #[cfg(any(LuXSquare, icondata_include_all))]
            LuIcon::LuXSquare => LU_X_SQUARE,
            #[cfg(any(LuYoutube, icondata_include_all))]
            LuIcon::LuYoutube => LU_YOUTUBE,
            #[cfg(any(LuZap, icondata_include_all))]
            LuIcon::LuZap => LU_ZAP,
            #[cfg(any(LuZapOff, icondata_include_all))]
            LuIcon::LuZapOff => LU_ZAP_OFF,
            #[cfg(any(LuZoomIn, icondata_include_all))]
            LuIcon::LuZoomIn => LU_ZOOM_IN,
            #[cfg(any(LuZoomOut, icondata_include_all))]
            LuIcon::LuZoomOut => LU_ZOOM_OUT,
        }
    }
}