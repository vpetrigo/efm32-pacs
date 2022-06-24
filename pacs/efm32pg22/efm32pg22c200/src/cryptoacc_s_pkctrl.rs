#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub pointer: crate::Reg<pointer::POINTER_SPEC>,
    #[doc = "0x04 - No Description"]
    pub command: crate::Reg<command::COMMAND_SPEC>,
    #[doc = "0x08 - No Description"]
    pub pkctrl: crate::Reg<pkctrl::PKCTRL_SPEC>,
    #[doc = "0x0c - No Description"]
    pub pkstatus: crate::Reg<pkstatus::PKSTATUS_SPEC>,
    #[doc = "0x10 - No Description"]
    pub version: crate::Reg<version::VERSION_SPEC>,
    #[doc = "0x14 - No Description"]
    pub timer: crate::Reg<timer::TIMER_SPEC>,
}
#[doc = "POINTER register accessor: an alias for `Reg<POINTER_SPEC>`"]
pub type POINTER = crate::Reg<pointer::POINTER_SPEC>;
#[doc = "No Description"]
pub mod pointer;
#[doc = "COMMAND register accessor: an alias for `Reg<COMMAND_SPEC>`"]
pub type COMMAND = crate::Reg<command::COMMAND_SPEC>;
#[doc = "No Description"]
pub mod command;
#[doc = "PKCTRL register accessor: an alias for `Reg<PKCTRL_SPEC>`"]
pub type PKCTRL = crate::Reg<pkctrl::PKCTRL_SPEC>;
#[doc = "No Description"]
pub mod pkctrl;
#[doc = "PKSTATUS register accessor: an alias for `Reg<PKSTATUS_SPEC>`"]
pub type PKSTATUS = crate::Reg<pkstatus::PKSTATUS_SPEC>;
#[doc = "No Description"]
pub mod pkstatus;
#[doc = "VERSION register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "No Description"]
pub mod version;
#[doc = "TIMER register accessor: an alias for `Reg<TIMER_SPEC>`"]
pub type TIMER = crate::Reg<timer::TIMER_SPEC>;
#[doc = "No Description"]
pub mod timer;
