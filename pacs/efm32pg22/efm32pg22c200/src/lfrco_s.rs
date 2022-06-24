#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Contains the LFRCO ip version"]
    pub ipversion: crate::Reg<ipversion::IPVERSION_SPEC>,
    #[doc = "0x04 - Control register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x08 - Status register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    _reserved3: [u8; 0x08],
    #[doc = "0x14 - Interrupt flag register"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x18 - Interrupt enable register"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x20 - Configuration lock register. Locks and unlocks access to configuration registers."]
    pub lock: crate::Reg<lock::LOCK_SPEC>,
    #[doc = "0x24 - Configuration register"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x2c - Nominal calibration register"]
    pub nomcal: crate::Reg<nomcal::NOMCAL_SPEC>,
    #[doc = "0x30 - Nominal calibration inverted register"]
    pub nomcalinv: crate::Reg<nomcalinv::NOMCALINV_SPEC>,
    #[doc = "0x34 - Command register"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
}
#[doc = "IPVERSION register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "Contains the LFRCO ip version"]
pub mod ipversion;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status register"]
pub mod status;
#[doc = "IF register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt flag register"]
pub mod if_;
#[doc = "IEN register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt enable register"]
pub mod ien;
#[doc = "LOCK register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Configuration lock register. Locks and unlocks access to configuration registers."]
pub mod lock;
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Configuration register"]
pub mod cfg;
#[doc = "NOMCAL register accessor: an alias for `Reg<NOMCAL_SPEC>`"]
pub type NOMCAL = crate::Reg<nomcal::NOMCAL_SPEC>;
#[doc = "Nominal calibration register"]
pub mod nomcal;
#[doc = "NOMCALINV register accessor: an alias for `Reg<NOMCALINV_SPEC>`"]
pub type NOMCALINV = crate::Reg<nomcalinv::NOMCALINV_SPEC>;
#[doc = "Nominal calibration inverted register"]
pub mod nomcalinv;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command register"]
pub mod cmd;
