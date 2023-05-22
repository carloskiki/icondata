//! This crate provides a collection of icons in the form of SVG data
//! and an enum to select them.
//!
//! ## Usage
//!
//! Every icon is shipped as its own feature; the enum variant and their corresponding feature name are
//! identical.
//!
pub use icondata_core::IconData;

#[cfg!(feature = "Ai")]
pub use icondata_ai::*;
#[cfg!(feature = "Bi")]
pub use icondata_bi::*;
#[cfg!(feature = "Bs")]
pub use icondata_bs::*;
#[cfg!(feature = "Cg")]
pub use icondata_cg::*;
#[cfg!(feature = "Ch")]
pub use icondata_ch::*;
#[cfg!(feature = "Fa")]
pub use icondata_fa::*;
#[cfg!(feature = "Fi")]
pub use icondata_fi::*;
#[cfg!(feature = "Hi")]
pub use icondata_hi::*;
#[cfg!(feature = "Im")]
pub use icondata_im::*;
#[cfg!(feature = "Io")]
pub use icondata_io::*;
#[cfg!(feature = "Lu")]
pub use icondata_lu::*;
#[cfg!(feature = "Oc")]
pub use icondata_oc::*;
#[cfg!(feature = "Ri")]
pub use icondata_ri::*;
#[cfg!(feature = "Si")]
pub use icondata_si::*;
#[cfg!(feature = "Tb")]
pub use icondata_tb::*;
#[cfg!(feature = "Ti")]
pub use icondata_ti::*;
#[cfg!(feature = "Vs")]
pub use icondata_vs::*;
#[cfg!(feature = "Wi")]
pub use icondata_wi::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Icon {
    #[cfg(feature = "Ai")]
    Ai(AiIcon),
    #[cfg(feature = "Bi")]
    Bi(BiIcon),
    #[cfg(feature = "Bs")]
    Bs(BsIcon),
    #[cfg(feature = "Cg")]
    Cg(CgIcon),
    #[cfg(feature = "Ch")]
    Ch(ChIcon),
    #[cfg(feature = "Fa")]
    Fa(FaIcon),
    #[cfg(feature = "Fi")]
    Fi(FiIcon),
    #[cfg(feature = "Hi")]
    Hi(HiIcon),
    #[cfg(feature = "Im")]
    Im(ImIcon),
    #[cfg(feature = "Io")]
    Io(IoIcon),
    #[cfg(feature = "Lu")]
    Lu(LuIcon),
    #[cfg(feature = "Oc")]
    Oc(OcIcon),
    #[cfg(feature = "Ri")]
    Ri(RiIcon),
    #[cfg(feature = "Si")]
    Si(SiIcon),
    #[cfg(feature = "Tb")]
    Tb(TbIcon),
    #[cfg(feature = "Ti")]
    Ti(TiIcon),
    #[cfg(feature = "Vs")]
    Vs(VsIcon),
    #[cfg(feature = "Wi")]
    Wi(WiIcon),
    
}
impl<'a> icondata_core::IconData<'a> for crate::Icon {
    fn data(self) -> &'a icondata_core::Data {
        match self {
            #[cfg(feature = "Ai")]
            Self::Ai(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Bi")]
            Self::Bi(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Bs")]
            Self::Bs(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Cg")]
            Self::Cg(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Ch")]
            Self::Ch(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Fa")]
            Self::Fa(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Fi")]
            Self::Fi(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Hi")]
            Self::Hi(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Im")]
            Self::Im(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Io")]
            Self::Io(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Lu")]
            Self::Lu(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Oc")]
            Self::Oc(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Ri")]
            Self::Ri(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Si")]
            Self::Si(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Tb")]
            Self::Tb(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Ti")]
            Self::Ti(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Vs")]
            Self::Vs(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Wi")]
            Self::Wi(icon) => icondata_core::IconData::from(icon),
            
        }
    }
}

#[cfg(feature = "Ai")]
impl From<AiIcon> for Icon {
    fn from(value: AiIcon) -> Self {
        Self::Ai(value)
    }
}

#[cfg(feature = "Bi")]
impl From<BiIcon> for Icon {
    fn from(value: BiIcon) -> Self {
        Self::Bi(value)
    }
}

#[cfg(feature = "Bs")]
impl From<BsIcon> for Icon {
    fn from(value: BsIcon) -> Self {
        Self::Bs(value)
    }
}

#[cfg(feature = "Cg")]
impl From<CgIcon> for Icon {
    fn from(value: CgIcon) -> Self {
        Self::Cg(value)
    }
}

#[cfg(feature = "Ch")]
impl From<ChIcon> for Icon {
    fn from(value: ChIcon) -> Self {
        Self::Ch(value)
    }
}

#[cfg(feature = "Fa")]
impl From<FaIcon> for Icon {
    fn from(value: FaIcon) -> Self {
        Self::Fa(value)
    }
}

#[cfg(feature = "Fi")]
impl From<FiIcon> for Icon {
    fn from(value: FiIcon) -> Self {
        Self::Fi(value)
    }
}

#[cfg(feature = "Hi")]
impl From<HiIcon> for Icon {
    fn from(value: HiIcon) -> Self {
        Self::Hi(value)
    }
}

#[cfg(feature = "Im")]
impl From<ImIcon> for Icon {
    fn from(value: ImIcon) -> Self {
        Self::Im(value)
    }
}

#[cfg(feature = "Io")]
impl From<IoIcon> for Icon {
    fn from(value: IoIcon) -> Self {
        Self::Io(value)
    }
}

#[cfg(feature = "Lu")]
impl From<LuIcon> for Icon {
    fn from(value: LuIcon) -> Self {
        Self::Lu(value)
    }
}

#[cfg(feature = "Oc")]
impl From<OcIcon> for Icon {
    fn from(value: OcIcon) -> Self {
        Self::Oc(value)
    }
}

#[cfg(feature = "Ri")]
impl From<RiIcon> for Icon {
    fn from(value: RiIcon) -> Self {
        Self::Ri(value)
    }
}

#[cfg(feature = "Si")]
impl From<SiIcon> for Icon {
    fn from(value: SiIcon) -> Self {
        Self::Si(value)
    }
}

#[cfg(feature = "Tb")]
impl From<TbIcon> for Icon {
    fn from(value: TbIcon) -> Self {
        Self::Tb(value)
    }
}

#[cfg(feature = "Ti")]
impl From<TiIcon> for Icon {
    fn from(value: TiIcon) -> Self {
        Self::Ti(value)
    }
}

#[cfg(feature = "Vs")]
impl From<VsIcon> for Icon {
    fn from(value: VsIcon) -> Self {
        Self::Vs(value)
    }
}

#[cfg(feature = "Wi")]
impl From<WiIcon> for Icon {
    fn from(value: WiIcon) -> Self {
        Self::Wi(value)
    }
}
