#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub ipversion: crate::Reg<ipversion::IPVERSION_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - No Description"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - No Description"]
    pub lock: crate::Reg<lock::LOCK_SPEC>,
    #[doc = "0x14 - No Description"]
    pub wdoglock: crate::Reg<wdoglock::WDOGLOCK_SPEC>,
    _reserved4: [u8; 0x08],
    #[doc = "0x20 - No Description"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x24 - No Description"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    _reserved6: [u8; 0x28],
    #[doc = "0x50 - No Description"]
    pub calcmd: crate::Reg<calcmd::CALCMD_SPEC>,
    #[doc = "0x54 - No Description"]
    pub calctrl: crate::Reg<calctrl::CALCTRL_SPEC>,
    #[doc = "0x58 - No Description"]
    pub calcnt: crate::Reg<calcnt::CALCNT_SPEC>,
    _reserved9: [u8; 0x08],
    #[doc = "0x64 - No Description"]
    pub clken0: crate::Reg<clken0::CLKEN0_SPEC>,
    #[doc = "0x68 - No Description"]
    pub clken1: crate::Reg<clken1::CLKEN1_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x70 - No Description"]
    pub sysclkctrl: crate::Reg<sysclkctrl::SYSCLKCTRL_SPEC>,
    _reserved12: [u8; 0x0c],
    #[doc = "0x80 - No Description"]
    pub traceclkctrl: crate::Reg<traceclkctrl::TRACECLKCTRL_SPEC>,
    _reserved13: [u8; 0x0c],
    #[doc = "0x90 - No Description"]
    pub exportclkctrl: crate::Reg<exportclkctrl::EXPORTCLKCTRL_SPEC>,
    _reserved14: [u8; 0x6c],
    #[doc = "0x100 - No Description"]
    pub dpllrefclkctrl: crate::Reg<dpllrefclkctrl::DPLLREFCLKCTRL_SPEC>,
    _reserved15: [u8; 0x1c],
    #[doc = "0x120 - No Description"]
    pub em01grpaclkctrl: crate::Reg<em01grpaclkctrl::EM01GRPACLKCTRL_SPEC>,
    #[doc = "0x124 - No Description"]
    pub em01grpbclkctrl: crate::Reg<em01grpbclkctrl::EM01GRPBCLKCTRL_SPEC>,
    _reserved17: [u8; 0x18],
    #[doc = "0x140 - No Description"]
    pub em23grpaclkctrl: crate::Reg<em23grpaclkctrl::EM23GRPACLKCTRL_SPEC>,
    _reserved18: [u8; 0x1c],
    #[doc = "0x160 - No Description"]
    pub em4grpaclkctrl: crate::Reg<em4grpaclkctrl::EM4GRPACLKCTRL_SPEC>,
    _reserved19: [u8; 0x1c],
    #[doc = "0x180 - No Description"]
    pub iadcclkctrl: crate::Reg<iadcclkctrl::IADCCLKCTRL_SPEC>,
    _reserved20: [u8; 0x7c],
    #[doc = "0x200 - No Description"]
    pub wdog0clkctrl: crate::Reg<wdog0clkctrl::WDOG0CLKCTRL_SPEC>,
    _reserved21: [u8; 0x1c],
    #[doc = "0x220 - No Description"]
    pub euart0clkctrl: crate::Reg<euart0clkctrl::EUART0CLKCTRL_SPEC>,
    _reserved22: [u8; 0x1c],
    #[doc = "0x240 - No Description"]
    pub rtccclkctrl: crate::Reg<rtccclkctrl::RTCCCLKCTRL_SPEC>,
    _reserved23: [u8; 0x1c],
    #[doc = "0x260 - No Description"]
    pub cryptoaccclkctrl: crate::Reg<cryptoaccclkctrl::CRYPTOACCCLKCTRL_SPEC>,
}
#[doc = "IPVERSION register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "No Description"]
pub mod status;
#[doc = "LOCK register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "No Description"]
pub mod lock;
#[doc = "WDOGLOCK register accessor: an alias for `Reg<WDOGLOCK_SPEC>`"]
pub type WDOGLOCK = crate::Reg<wdoglock::WDOGLOCK_SPEC>;
#[doc = "No Description"]
pub mod wdoglock;
#[doc = "IF register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "No Description"]
pub mod if_;
#[doc = "IEN register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "No Description"]
pub mod ien;
#[doc = "CALCMD register accessor: an alias for `Reg<CALCMD_SPEC>`"]
pub type CALCMD = crate::Reg<calcmd::CALCMD_SPEC>;
#[doc = "No Description"]
pub mod calcmd;
#[doc = "CALCTRL register accessor: an alias for `Reg<CALCTRL_SPEC>`"]
pub type CALCTRL = crate::Reg<calctrl::CALCTRL_SPEC>;
#[doc = "No Description"]
pub mod calctrl;
#[doc = "CALCNT register accessor: an alias for `Reg<CALCNT_SPEC>`"]
pub type CALCNT = crate::Reg<calcnt::CALCNT_SPEC>;
#[doc = "No Description"]
pub mod calcnt;
#[doc = "CLKEN0 register accessor: an alias for `Reg<CLKEN0_SPEC>`"]
pub type CLKEN0 = crate::Reg<clken0::CLKEN0_SPEC>;
#[doc = "No Description"]
pub mod clken0;
#[doc = "CLKEN1 register accessor: an alias for `Reg<CLKEN1_SPEC>`"]
pub type CLKEN1 = crate::Reg<clken1::CLKEN1_SPEC>;
#[doc = "No Description"]
pub mod clken1;
#[doc = "SYSCLKCTRL register accessor: an alias for `Reg<SYSCLKCTRL_SPEC>`"]
pub type SYSCLKCTRL = crate::Reg<sysclkctrl::SYSCLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod sysclkctrl;
#[doc = "TRACECLKCTRL register accessor: an alias for `Reg<TRACECLKCTRL_SPEC>`"]
pub type TRACECLKCTRL = crate::Reg<traceclkctrl::TRACECLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod traceclkctrl;
#[doc = "EXPORTCLKCTRL register accessor: an alias for `Reg<EXPORTCLKCTRL_SPEC>`"]
pub type EXPORTCLKCTRL = crate::Reg<exportclkctrl::EXPORTCLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod exportclkctrl;
#[doc = "DPLLREFCLKCTRL register accessor: an alias for `Reg<DPLLREFCLKCTRL_SPEC>`"]
pub type DPLLREFCLKCTRL = crate::Reg<dpllrefclkctrl::DPLLREFCLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod dpllrefclkctrl;
#[doc = "EM01GRPACLKCTRL register accessor: an alias for `Reg<EM01GRPACLKCTRL_SPEC>`"]
pub type EM01GRPACLKCTRL = crate::Reg<em01grpaclkctrl::EM01GRPACLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod em01grpaclkctrl;
#[doc = "EM01GRPBCLKCTRL register accessor: an alias for `Reg<EM01GRPBCLKCTRL_SPEC>`"]
pub type EM01GRPBCLKCTRL = crate::Reg<em01grpbclkctrl::EM01GRPBCLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod em01grpbclkctrl;
#[doc = "EM23GRPACLKCTRL register accessor: an alias for `Reg<EM23GRPACLKCTRL_SPEC>`"]
pub type EM23GRPACLKCTRL = crate::Reg<em23grpaclkctrl::EM23GRPACLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod em23grpaclkctrl;
#[doc = "EM4GRPACLKCTRL register accessor: an alias for `Reg<EM4GRPACLKCTRL_SPEC>`"]
pub type EM4GRPACLKCTRL = crate::Reg<em4grpaclkctrl::EM4GRPACLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod em4grpaclkctrl;
#[doc = "IADCCLKCTRL register accessor: an alias for `Reg<IADCCLKCTRL_SPEC>`"]
pub type IADCCLKCTRL = crate::Reg<iadcclkctrl::IADCCLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod iadcclkctrl;
#[doc = "WDOG0CLKCTRL register accessor: an alias for `Reg<WDOG0CLKCTRL_SPEC>`"]
pub type WDOG0CLKCTRL = crate::Reg<wdog0clkctrl::WDOG0CLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod wdog0clkctrl;
#[doc = "EUART0CLKCTRL register accessor: an alias for `Reg<EUART0CLKCTRL_SPEC>`"]
pub type EUART0CLKCTRL = crate::Reg<euart0clkctrl::EUART0CLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod euart0clkctrl;
#[doc = "RTCCCLKCTRL register accessor: an alias for `Reg<RTCCCLKCTRL_SPEC>`"]
pub type RTCCCLKCTRL = crate::Reg<rtccclkctrl::RTCCCLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod rtccclkctrl;
#[doc = "CRYPTOACCCLKCTRL register accessor: an alias for `Reg<CRYPTOACCCLKCTRL_SPEC>`"]
pub type CRYPTOACCCLKCTRL = crate::Reg<cryptoaccclkctrl::CRYPTOACCCLKCTRL_SPEC>;
#[doc = "No Description"]
pub mod cryptoaccclkctrl;
