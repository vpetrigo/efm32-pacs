#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub ipversion: crate::Reg<ipversion::IPVERSION_SPEC>,
    #[doc = "0x04 - No Description"]
    pub en: crate::Reg<en::EN_SPEC>,
    #[doc = "0x08 - No Description"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x0c - No Description"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x10 - No Description"]
    pub state: crate::Reg<state::STATE_SPEC>,
    #[doc = "0x14 - No Description"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x18 - No Description"]
    pub clkdiv: crate::Reg<clkdiv::CLKDIV_SPEC>,
    #[doc = "0x1c - No Description"]
    pub saddr: crate::Reg<saddr::SADDR_SPEC>,
    #[doc = "0x20 - No Description"]
    pub saddrmask: crate::Reg<saddrmask::SADDRMASK_SPEC>,
    #[doc = "0x24 - No Description"]
    pub rxdata: crate::Reg<rxdata::RXDATA_SPEC>,
    #[doc = "0x28 - No Description"]
    pub rxdouble: crate::Reg<rxdouble::RXDOUBLE_SPEC>,
    #[doc = "0x2c - No Description"]
    pub rxdatap: crate::Reg<rxdatap::RXDATAP_SPEC>,
    #[doc = "0x30 - No Description"]
    pub rxdoublep: crate::Reg<rxdoublep::RXDOUBLEP_SPEC>,
    #[doc = "0x34 - No Description"]
    pub txdata: crate::Reg<txdata::TXDATA_SPEC>,
    #[doc = "0x38 - No Description"]
    pub txdouble: crate::Reg<txdouble::TXDOUBLE_SPEC>,
    #[doc = "0x3c - No Description"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x40 - No Description"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
}
#[doc = "IPVERSION register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "EN register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "No Description"]
pub mod en;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "No Description"]
pub mod ctrl;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "STATE register accessor: an alias for `Reg<STATE_SPEC>`"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "No Description"]
pub mod state;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "No Description"]
pub mod status;
#[doc = "CLKDIV register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "No Description"]
pub mod clkdiv;
#[doc = "SADDR register accessor: an alias for `Reg<SADDR_SPEC>`"]
pub type SADDR = crate::Reg<saddr::SADDR_SPEC>;
#[doc = "No Description"]
pub mod saddr;
#[doc = "SADDRMASK register accessor: an alias for `Reg<SADDRMASK_SPEC>`"]
pub type SADDRMASK = crate::Reg<saddrmask::SADDRMASK_SPEC>;
#[doc = "No Description"]
pub mod saddrmask;
#[doc = "RXDATA register accessor: an alias for `Reg<RXDATA_SPEC>`"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "No Description"]
pub mod rxdata;
#[doc = "RXDOUBLE register accessor: an alias for `Reg<RXDOUBLE_SPEC>`"]
pub type RXDOUBLE = crate::Reg<rxdouble::RXDOUBLE_SPEC>;
#[doc = "No Description"]
pub mod rxdouble;
#[doc = "RXDATAP register accessor: an alias for `Reg<RXDATAP_SPEC>`"]
pub type RXDATAP = crate::Reg<rxdatap::RXDATAP_SPEC>;
#[doc = "No Description"]
pub mod rxdatap;
#[doc = "RXDOUBLEP register accessor: an alias for `Reg<RXDOUBLEP_SPEC>`"]
pub type RXDOUBLEP = crate::Reg<rxdoublep::RXDOUBLEP_SPEC>;
#[doc = "No Description"]
pub mod rxdoublep;
#[doc = "TXDATA register accessor: an alias for `Reg<TXDATA_SPEC>`"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "No Description"]
pub mod txdata;
#[doc = "TXDOUBLE register accessor: an alias for `Reg<TXDOUBLE_SPEC>`"]
pub type TXDOUBLE = crate::Reg<txdouble::TXDOUBLE_SPEC>;
#[doc = "No Description"]
pub mod txdouble;
#[doc = "IF register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "No Description"]
pub mod if_;
#[doc = "IEN register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "No Description"]
pub mod ien;
