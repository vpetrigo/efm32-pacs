#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Software Pulse Register"]
    pub swpulse: crate::Reg<swpulse::SWPULSE_SPEC>,
    #[doc = "0x04 - Software Level Register"]
    pub swlevel: crate::Reg<swlevel::SWLEVEL_SPEC>,
    #[doc = "0x08 - I/O Routing Pin Enable Register"]
    pub routepen: crate::Reg<routepen::ROUTEPEN_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - I/O Routing Location Register"]
    pub routeloc0: crate::Reg<routeloc0::ROUTELOC0_SPEC>,
    #[doc = "0x14 - I/O Routing Location Register"]
    pub routeloc1: crate::Reg<routeloc1::ROUTELOC1_SPEC>,
    #[doc = "0x18 - I/O Routing Location Register"]
    pub routeloc2: crate::Reg<routeloc2::ROUTELOC2_SPEC>,
    #[doc = "0x1c - I/O Routing Location Register"]
    pub routeloc3: crate::Reg<routeloc3::ROUTELOC3_SPEC>,
    _reserved7: [u8; 0x10],
    #[doc = "0x30 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x34 - DMA Request 0 Register"]
    pub dmareq0: crate::Reg<dmareq0::DMAREQ0_SPEC>,
    #[doc = "0x38 - DMA Request 1 Register"]
    pub dmareq1: crate::Reg<dmareq1::DMAREQ1_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x40 - PRS Channel Values"]
    pub peek: crate::Reg<peek::PEEK_SPEC>,
    _reserved11: [u8; 0x0c],
    #[doc = "0x50 - Channel Control Register"]
    pub ch0_ctrl: crate::Reg<ch0_ctrl::CH0_CTRL_SPEC>,
    #[doc = "0x54 - Channel Control Register"]
    pub ch1_ctrl: crate::Reg<ch1_ctrl::CH1_CTRL_SPEC>,
    #[doc = "0x58 - Channel Control Register"]
    pub ch2_ctrl: crate::Reg<ch2_ctrl::CH2_CTRL_SPEC>,
    #[doc = "0x5c - Channel Control Register"]
    pub ch3_ctrl: crate::Reg<ch3_ctrl::CH3_CTRL_SPEC>,
    #[doc = "0x60 - Channel Control Register"]
    pub ch4_ctrl: crate::Reg<ch4_ctrl::CH4_CTRL_SPEC>,
    #[doc = "0x64 - Channel Control Register"]
    pub ch5_ctrl: crate::Reg<ch5_ctrl::CH5_CTRL_SPEC>,
    #[doc = "0x68 - Channel Control Register"]
    pub ch6_ctrl: crate::Reg<ch6_ctrl::CH6_CTRL_SPEC>,
    #[doc = "0x6c - Channel Control Register"]
    pub ch7_ctrl: crate::Reg<ch7_ctrl::CH7_CTRL_SPEC>,
    #[doc = "0x70 - Channel Control Register"]
    pub ch8_ctrl: crate::Reg<ch8_ctrl::CH8_CTRL_SPEC>,
    #[doc = "0x74 - Channel Control Register"]
    pub ch9_ctrl: crate::Reg<ch9_ctrl::CH9_CTRL_SPEC>,
    #[doc = "0x78 - Channel Control Register"]
    pub ch10_ctrl: crate::Reg<ch10_ctrl::CH10_CTRL_SPEC>,
    #[doc = "0x7c - Channel Control Register"]
    pub ch11_ctrl: crate::Reg<ch11_ctrl::CH11_CTRL_SPEC>,
    #[doc = "0x80 - Channel Control Register"]
    pub ch12_ctrl: crate::Reg<ch12_ctrl::CH12_CTRL_SPEC>,
    #[doc = "0x84 - Channel Control Register"]
    pub ch13_ctrl: crate::Reg<ch13_ctrl::CH13_CTRL_SPEC>,
    #[doc = "0x88 - Channel Control Register"]
    pub ch14_ctrl: crate::Reg<ch14_ctrl::CH14_CTRL_SPEC>,
    #[doc = "0x8c - Channel Control Register"]
    pub ch15_ctrl: crate::Reg<ch15_ctrl::CH15_CTRL_SPEC>,
}
#[doc = "SWPULSE register accessor: an alias for `Reg<SWPULSE_SPEC>`"]
pub type SWPULSE = crate::Reg<swpulse::SWPULSE_SPEC>;
#[doc = "Software Pulse Register"]
pub mod swpulse;
#[doc = "SWLEVEL register accessor: an alias for `Reg<SWLEVEL_SPEC>`"]
pub type SWLEVEL = crate::Reg<swlevel::SWLEVEL_SPEC>;
#[doc = "Software Level Register"]
pub mod swlevel;
#[doc = "ROUTEPEN register accessor: an alias for `Reg<ROUTEPEN_SPEC>`"]
pub type ROUTEPEN = crate::Reg<routepen::ROUTEPEN_SPEC>;
#[doc = "I/O Routing Pin Enable Register"]
pub mod routepen;
#[doc = "ROUTELOC0 register accessor: an alias for `Reg<ROUTELOC0_SPEC>`"]
pub type ROUTELOC0 = crate::Reg<routeloc0::ROUTELOC0_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
#[doc = "ROUTELOC1 register accessor: an alias for `Reg<ROUTELOC1_SPEC>`"]
pub type ROUTELOC1 = crate::Reg<routeloc1::ROUTELOC1_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc1;
#[doc = "ROUTELOC2 register accessor: an alias for `Reg<ROUTELOC2_SPEC>`"]
pub type ROUTELOC2 = crate::Reg<routeloc2::ROUTELOC2_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc2;
#[doc = "ROUTELOC3 register accessor: an alias for `Reg<ROUTELOC3_SPEC>`"]
pub type ROUTELOC3 = crate::Reg<routeloc3::ROUTELOC3_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc3;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "DMAREQ0 register accessor: an alias for `Reg<DMAREQ0_SPEC>`"]
pub type DMAREQ0 = crate::Reg<dmareq0::DMAREQ0_SPEC>;
#[doc = "DMA Request 0 Register"]
pub mod dmareq0;
#[doc = "DMAREQ1 register accessor: an alias for `Reg<DMAREQ1_SPEC>`"]
pub type DMAREQ1 = crate::Reg<dmareq1::DMAREQ1_SPEC>;
#[doc = "DMA Request 1 Register"]
pub mod dmareq1;
#[doc = "PEEK register accessor: an alias for `Reg<PEEK_SPEC>`"]
pub type PEEK = crate::Reg<peek::PEEK_SPEC>;
#[doc = "PRS Channel Values"]
pub mod peek;
#[doc = "CH0_CTRL register accessor: an alias for `Reg<CH0_CTRL_SPEC>`"]
pub type CH0_CTRL = crate::Reg<ch0_ctrl::CH0_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch0_ctrl;
#[doc = "CH1_CTRL register accessor: an alias for `Reg<CH1_CTRL_SPEC>`"]
pub type CH1_CTRL = crate::Reg<ch1_ctrl::CH1_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch1_ctrl;
#[doc = "CH2_CTRL register accessor: an alias for `Reg<CH2_CTRL_SPEC>`"]
pub type CH2_CTRL = crate::Reg<ch2_ctrl::CH2_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch2_ctrl;
#[doc = "CH3_CTRL register accessor: an alias for `Reg<CH3_CTRL_SPEC>`"]
pub type CH3_CTRL = crate::Reg<ch3_ctrl::CH3_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch3_ctrl;
#[doc = "CH4_CTRL register accessor: an alias for `Reg<CH4_CTRL_SPEC>`"]
pub type CH4_CTRL = crate::Reg<ch4_ctrl::CH4_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch4_ctrl;
#[doc = "CH5_CTRL register accessor: an alias for `Reg<CH5_CTRL_SPEC>`"]
pub type CH5_CTRL = crate::Reg<ch5_ctrl::CH5_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch5_ctrl;
#[doc = "CH6_CTRL register accessor: an alias for `Reg<CH6_CTRL_SPEC>`"]
pub type CH6_CTRL = crate::Reg<ch6_ctrl::CH6_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch6_ctrl;
#[doc = "CH7_CTRL register accessor: an alias for `Reg<CH7_CTRL_SPEC>`"]
pub type CH7_CTRL = crate::Reg<ch7_ctrl::CH7_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch7_ctrl;
#[doc = "CH8_CTRL register accessor: an alias for `Reg<CH8_CTRL_SPEC>`"]
pub type CH8_CTRL = crate::Reg<ch8_ctrl::CH8_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch8_ctrl;
#[doc = "CH9_CTRL register accessor: an alias for `Reg<CH9_CTRL_SPEC>`"]
pub type CH9_CTRL = crate::Reg<ch9_ctrl::CH9_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch9_ctrl;
#[doc = "CH10_CTRL register accessor: an alias for `Reg<CH10_CTRL_SPEC>`"]
pub type CH10_CTRL = crate::Reg<ch10_ctrl::CH10_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch10_ctrl;
#[doc = "CH11_CTRL register accessor: an alias for `Reg<CH11_CTRL_SPEC>`"]
pub type CH11_CTRL = crate::Reg<ch11_ctrl::CH11_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch11_ctrl;
#[doc = "CH12_CTRL register accessor: an alias for `Reg<CH12_CTRL_SPEC>`"]
pub type CH12_CTRL = crate::Reg<ch12_ctrl::CH12_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch12_ctrl;
#[doc = "CH13_CTRL register accessor: an alias for `Reg<CH13_CTRL_SPEC>`"]
pub type CH13_CTRL = crate::Reg<ch13_ctrl::CH13_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch13_ctrl;
#[doc = "CH14_CTRL register accessor: an alias for `Reg<CH14_CTRL_SPEC>`"]
pub type CH14_CTRL = crate::Reg<ch14_ctrl::CH14_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch14_ctrl;
#[doc = "CH15_CTRL register accessor: an alias for `Reg<CH15_CTRL_SPEC>`"]
pub type CH15_CTRL = crate::Reg<ch15_ctrl::CH15_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch15_ctrl;
