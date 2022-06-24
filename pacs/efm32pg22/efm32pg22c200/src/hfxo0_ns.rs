#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub ipversion: crate::Reg<ipversion::IPVERSION_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - No Description"]
    pub xtalcfg: crate::Reg<xtalcfg::XTALCFG_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x18 - No Description"]
    pub xtalctrl: crate::Reg<xtalctrl::XTALCTRL_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x20 - No Description"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x28 - No Description"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    _reserved5: [u8; 0x24],
    #[doc = "0x50 - No Description"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x58 - No Description"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    _reserved7: [u8; 0x14],
    #[doc = "0x70 - No Description"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x74 - No Description"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    _reserved9: [u8; 0x08],
    #[doc = "0x80 - No Description"]
    pub lock: crate::Reg<lock::LOCK_SPEC>,
}
#[doc = "IPVERSION register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "XTALCFG register accessor: an alias for `Reg<XTALCFG_SPEC>`"]
pub type XTALCFG = crate::Reg<xtalcfg::XTALCFG_SPEC>;
#[doc = "No Description"]
pub mod xtalcfg;
#[doc = "XTALCTRL register accessor: an alias for `Reg<XTALCTRL_SPEC>`"]
pub type XTALCTRL = crate::Reg<xtalctrl::XTALCTRL_SPEC>;
#[doc = "No Description"]
pub mod xtalctrl;
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "No Description"]
pub mod cfg;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "No Description"]
pub mod ctrl;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "No Description"]
pub mod status;
#[doc = "IF register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "No Description"]
pub mod if_;
#[doc = "IEN register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "No Description"]
pub mod ien;
#[doc = "LOCK register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "No Description"]
pub mod lock;
