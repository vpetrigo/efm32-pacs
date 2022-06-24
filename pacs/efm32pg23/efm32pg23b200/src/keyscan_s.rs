#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IPVERSION"]
    pub ipversion: crate::Reg<ipversion::IPVERSION_SPEC>,
    #[doc = "0x04 - Enable"]
    pub en: crate::Reg<en::EN_SPEC>,
    #[doc = "0x08 - Software Reset"]
    pub swrst: crate::Reg<swrst::SWRST_SPEC>,
    #[doc = "0x0c - Config"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x10 - Command"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x14 - Delay"]
    pub delay: crate::Reg<delay::DELAY_SPEC>,
    #[doc = "0x18 - Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x1c - Interrupt Flags"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x20 - Interrupt Enables"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
}
#[doc = "IPVERSION register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "IPVERSION"]
pub mod ipversion;
#[doc = "EN register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "Enable"]
pub mod en;
#[doc = "SWRST register accessor: an alias for `Reg<SWRST_SPEC>`"]
pub type SWRST = crate::Reg<swrst::SWRST_SPEC>;
#[doc = "Software Reset"]
pub mod swrst;
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Config"]
pub mod cfg;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command"]
pub mod cmd;
#[doc = "DELAY register accessor: an alias for `Reg<DELAY_SPEC>`"]
pub type DELAY = crate::Reg<delay::DELAY_SPEC>;
#[doc = "Delay"]
pub mod delay;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "IF register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt Flags"]
pub mod if_;
#[doc = "IEN register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enables"]
pub mod ien;
