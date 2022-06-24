#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub ipversion: crate::Reg<ipversion::IPVERSION_SPEC>,
    #[doc = "0x04 - No Description"]
    pub en: crate::Reg<en::EN_SPEC>,
    #[doc = "0x08 - No Description"]
    pub swrst: crate::Reg<swrst::SWRST_SPEC>,
    #[doc = "0x0c - No Description"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x10 - No Description"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x14 - No Description"]
    pub dispctrl: crate::Reg<dispctrl::DISPCTRL_SPEC>,
    #[doc = "0x18 - No Description"]
    pub bacfg: crate::Reg<bacfg::BACFG_SPEC>,
    #[doc = "0x1c - No Description"]
    pub bactrl: crate::Reg<bactrl::BACTRL_SPEC>,
    #[doc = "0x20 - No Description"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x24 - No Description"]
    pub arega: crate::Reg<arega::AREGA_SPEC>,
    #[doc = "0x28 - No Description"]
    pub aregb: crate::Reg<aregb::AREGB_SPEC>,
    #[doc = "0x2c - No Description"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x30 - No Description"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    #[doc = "0x34 - No Description"]
    pub biasctrl: crate::Reg<biasctrl::BIASCTRL_SPEC>,
    #[doc = "0x38 - No Description"]
    pub dispctrlx: crate::Reg<dispctrlx::DISPCTRLX_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x40 - No Description"]
    pub segd0: crate::Reg<segd0::SEGD0_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x48 - No Description"]
    pub segd1: crate::Reg<segd1::SEGD1_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0x50 - No Description"]
    pub segd2: crate::Reg<segd2::SEGD2_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x58 - No Description"]
    pub segd3: crate::Reg<segd3::SEGD3_SPEC>,
    _reserved19: [u8; 0x64],
    #[doc = "0xc0 - No Description"]
    pub updatectrl: crate::Reg<updatectrl::UPDATECTRL_SPEC>,
    _reserved20: [u8; 0x2c],
    #[doc = "0xf0 - No Description"]
    pub framerate: crate::Reg<framerate::FRAMERATE_SPEC>,
}
#[doc = "IPVERSION register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "EN register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "No Description"]
pub mod en;
#[doc = "SWRST register accessor: an alias for `Reg<SWRST_SPEC>`"]
pub type SWRST = crate::Reg<swrst::SWRST_SPEC>;
#[doc = "No Description"]
pub mod swrst;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "No Description"]
pub mod ctrl;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "DISPCTRL register accessor: an alias for `Reg<DISPCTRL_SPEC>`"]
pub type DISPCTRL = crate::Reg<dispctrl::DISPCTRL_SPEC>;
#[doc = "No Description"]
pub mod dispctrl;
#[doc = "BACFG register accessor: an alias for `Reg<BACFG_SPEC>`"]
pub type BACFG = crate::Reg<bacfg::BACFG_SPEC>;
#[doc = "No Description"]
pub mod bacfg;
#[doc = "BACTRL register accessor: an alias for `Reg<BACTRL_SPEC>`"]
pub type BACTRL = crate::Reg<bactrl::BACTRL_SPEC>;
#[doc = "No Description"]
pub mod bactrl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "No Description"]
pub mod status;
#[doc = "AREGA register accessor: an alias for `Reg<AREGA_SPEC>`"]
pub type AREGA = crate::Reg<arega::AREGA_SPEC>;
#[doc = "No Description"]
pub mod arega;
#[doc = "AREGB register accessor: an alias for `Reg<AREGB_SPEC>`"]
pub type AREGB = crate::Reg<aregb::AREGB_SPEC>;
#[doc = "No Description"]
pub mod aregb;
#[doc = "IF register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "No Description"]
pub mod if_;
#[doc = "IEN register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "No Description"]
pub mod ien;
#[doc = "BIASCTRL register accessor: an alias for `Reg<BIASCTRL_SPEC>`"]
pub type BIASCTRL = crate::Reg<biasctrl::BIASCTRL_SPEC>;
#[doc = "No Description"]
pub mod biasctrl;
#[doc = "DISPCTRLX register accessor: an alias for `Reg<DISPCTRLX_SPEC>`"]
pub type DISPCTRLX = crate::Reg<dispctrlx::DISPCTRLX_SPEC>;
#[doc = "No Description"]
pub mod dispctrlx;
#[doc = "SEGD0 register accessor: an alias for `Reg<SEGD0_SPEC>`"]
pub type SEGD0 = crate::Reg<segd0::SEGD0_SPEC>;
#[doc = "No Description"]
pub mod segd0;
#[doc = "SEGD1 register accessor: an alias for `Reg<SEGD1_SPEC>`"]
pub type SEGD1 = crate::Reg<segd1::SEGD1_SPEC>;
#[doc = "No Description"]
pub mod segd1;
#[doc = "SEGD2 register accessor: an alias for `Reg<SEGD2_SPEC>`"]
pub type SEGD2 = crate::Reg<segd2::SEGD2_SPEC>;
#[doc = "No Description"]
pub mod segd2;
#[doc = "SEGD3 register accessor: an alias for `Reg<SEGD3_SPEC>`"]
pub type SEGD3 = crate::Reg<segd3::SEGD3_SPEC>;
#[doc = "No Description"]
pub mod segd3;
#[doc = "UPDATECTRL register accessor: an alias for `Reg<UPDATECTRL_SPEC>`"]
pub type UPDATECTRL = crate::Reg<updatectrl::UPDATECTRL_SPEC>;
#[doc = "No Description"]
pub mod updatectrl;
#[doc = "FRAMERATE register accessor: an alias for `Reg<FRAMERATE_SPEC>`"]
pub type FRAMERATE = crate::Reg<framerate::FRAMERATE_SPEC>;
#[doc = "No Description"]
pub mod framerate;
