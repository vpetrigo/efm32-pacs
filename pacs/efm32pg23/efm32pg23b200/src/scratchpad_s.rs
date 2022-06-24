#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Used for SIMCTRL Pointer in Verification Environment"]
    pub sreg0: crate::Reg<sreg0::SREG0_SPEC>,
    #[doc = "0x04 - Used for SIMCTRL Data Access in Verification Environment"]
    pub sreg1: crate::Reg<sreg1::SREG1_SPEC>,
}
#[doc = "SREG0 register accessor: an alias for `Reg<SREG0_SPEC>`"]
pub type SREG0 = crate::Reg<sreg0::SREG0_SPEC>;
#[doc = "Used for SIMCTRL Pointer in Verification Environment"]
pub mod sreg0;
#[doc = "SREG1 register accessor: an alias for `Reg<SREG1_SPEC>`"]
pub type SREG1 = crate::Reg<sreg1::SREG1_SPEC>;
#[doc = "Used for SIMCTRL Data Access in Verification Environment"]
pub mod sreg1;
