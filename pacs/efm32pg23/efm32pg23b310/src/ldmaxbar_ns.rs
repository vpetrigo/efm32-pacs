#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub ipversion: crate::Reg<ipversion::IPVERSION_SPEC>,
    #[doc = "0x04 - No Description"]
    pub ch0_reqsel: crate::Reg<ch0_reqsel::CH0_REQSEL_SPEC>,
    #[doc = "0x08 - No Description"]
    pub ch1_reqsel: crate::Reg<ch1_reqsel::CH1_REQSEL_SPEC>,
    #[doc = "0x0c - No Description"]
    pub ch2_reqsel: crate::Reg<ch2_reqsel::CH2_REQSEL_SPEC>,
    #[doc = "0x10 - No Description"]
    pub ch3_reqsel: crate::Reg<ch3_reqsel::CH3_REQSEL_SPEC>,
    #[doc = "0x14 - No Description"]
    pub ch4_reqsel: crate::Reg<ch4_reqsel::CH4_REQSEL_SPEC>,
    #[doc = "0x18 - No Description"]
    pub ch5_reqsel: crate::Reg<ch5_reqsel::CH5_REQSEL_SPEC>,
    #[doc = "0x1c - No Description"]
    pub ch6_reqsel: crate::Reg<ch6_reqsel::CH6_REQSEL_SPEC>,
    #[doc = "0x20 - No Description"]
    pub ch7_reqsel: crate::Reg<ch7_reqsel::CH7_REQSEL_SPEC>,
}
#[doc = "IPVERSION register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "CH0_REQSEL register accessor: an alias for `Reg<CH0_REQSEL_SPEC>`"]
pub type CH0_REQSEL = crate::Reg<ch0_reqsel::CH0_REQSEL_SPEC>;
#[doc = "No Description"]
pub mod ch0_reqsel;
#[doc = "CH1_REQSEL register accessor: an alias for `Reg<CH1_REQSEL_SPEC>`"]
pub type CH1_REQSEL = crate::Reg<ch1_reqsel::CH1_REQSEL_SPEC>;
#[doc = "No Description"]
pub mod ch1_reqsel;
#[doc = "CH2_REQSEL register accessor: an alias for `Reg<CH2_REQSEL_SPEC>`"]
pub type CH2_REQSEL = crate::Reg<ch2_reqsel::CH2_REQSEL_SPEC>;
#[doc = "No Description"]
pub mod ch2_reqsel;
#[doc = "CH3_REQSEL register accessor: an alias for `Reg<CH3_REQSEL_SPEC>`"]
pub type CH3_REQSEL = crate::Reg<ch3_reqsel::CH3_REQSEL_SPEC>;
#[doc = "No Description"]
pub mod ch3_reqsel;
#[doc = "CH4_REQSEL register accessor: an alias for `Reg<CH4_REQSEL_SPEC>`"]
pub type CH4_REQSEL = crate::Reg<ch4_reqsel::CH4_REQSEL_SPEC>;
#[doc = "No Description"]
pub mod ch4_reqsel;
#[doc = "CH5_REQSEL register accessor: an alias for `Reg<CH5_REQSEL_SPEC>`"]
pub type CH5_REQSEL = crate::Reg<ch5_reqsel::CH5_REQSEL_SPEC>;
#[doc = "No Description"]
pub mod ch5_reqsel;
#[doc = "CH6_REQSEL register accessor: an alias for `Reg<CH6_REQSEL_SPEC>`"]
pub type CH6_REQSEL = crate::Reg<ch6_reqsel::CH6_REQSEL_SPEC>;
#[doc = "No Description"]
pub mod ch6_reqsel;
#[doc = "CH7_REQSEL register accessor: an alias for `Reg<CH7_REQSEL_SPEC>`"]
pub type CH7_REQSEL = crate::Reg<ch7_reqsel::CH7_REQSEL_SPEC>;
#[doc = "No Description"]
pub mod ch7_reqsel;
