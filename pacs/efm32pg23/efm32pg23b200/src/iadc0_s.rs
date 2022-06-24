#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IPVERSION"]
    pub ipversion: crate::Reg<ipversion::IPVERSION_SPEC>,
    #[doc = "0x04 - Enable"]
    pub en: crate::Reg<en::EN_SPEC>,
    #[doc = "0x08 - Control"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x0c - Command"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x10 - Timer"]
    pub timer: crate::Reg<timer::TIMER_SPEC>,
    #[doc = "0x14 - Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x18 - Mask Request"]
    pub maskreq: crate::Reg<maskreq::MASKREQ_SPEC>,
    #[doc = "0x1c - Scan Table Mask"]
    pub stmask: crate::Reg<stmask::STMASK_SPEC>,
    #[doc = "0x20 - Comparator Threshold"]
    pub cmpthr: crate::Reg<cmpthr::CMPTHR_SPEC>,
    #[doc = "0x24 - Interrupt Flag"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x28 - Interrupt Enable"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    #[doc = "0x2c - Trigger"]
    pub trigger: crate::Reg<trigger::TRIGGER_SPEC>,
    _reserved12: [u8; 0x18],
    #[doc = "0x48 - Configration"]
    pub cfg0: crate::Reg<cfg0::CFG0_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x50 - Scale"]
    pub scale0: crate::Reg<scale0::SCALE0_SPEC>,
    #[doc = "0x54 - Scheduling"]
    pub sched0: crate::Reg<sched0::SCHED0_SPEC>,
    #[doc = "0x58 - Configration"]
    pub cfg1: crate::Reg<cfg1::CFG1_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x60 - Scale"]
    pub scale1: crate::Reg<scale1::SCALE1_SPEC>,
    #[doc = "0x64 - Scheduling"]
    pub sched1: crate::Reg<sched1::SCHED1_SPEC>,
    _reserved18: [u8; 0x08],
    #[doc = "0x70 - Single FIFO Configuration"]
    pub singlefifocfg: crate::Reg<singlefifocfg::SINGLEFIFOCFG_SPEC>,
    #[doc = "0x74 - Read the oldest valid data from the single FIFO and pop the FIFO"]
    pub singlefifodata: crate::Reg<singlefifodata::SINGLEFIFODATA_SPEC>,
    #[doc = "0x78 - Single FIFO status"]
    pub singlefifostat: crate::Reg<singlefifostat::SINGLEFIFOSTAT_SPEC>,
    #[doc = "0x7c - latest single queue conversion data"]
    pub singledata: crate::Reg<singledata::SINGLEDATA_SPEC>,
    #[doc = "0x80 - SCAN FIFO configuration"]
    pub scanfifocfg: crate::Reg<scanfifocfg::SCANFIFOCFG_SPEC>,
    #[doc = "0x84 - Read the oldest valid data from the scan FIFO and pop the FIFO"]
    pub scanfifodata: crate::Reg<scanfifodata::SCANFIFODATA_SPEC>,
    #[doc = "0x88 - Scan FIFO status"]
    pub scanfifostat: crate::Reg<scanfifostat::SCANFIFOSTAT_SPEC>,
    #[doc = "0x8c - Most recent data data from scan queue conversion"]
    pub scandata: crate::Reg<scandata::SCANDATA_SPEC>,
    _reserved26: [u8; 0x08],
    #[doc = "0x98 - No Description"]
    pub single: crate::Reg<single::SINGLE_SPEC>,
    _reserved27: [u8; 0x04],
    #[doc = "0xa0 - No Description"]
    pub scan0: crate::Reg<scan0::SCAN0_SPEC>,
    #[doc = "0xa4 - No Description"]
    pub scan1: crate::Reg<scan1::SCAN1_SPEC>,
    #[doc = "0xa8 - No Description"]
    pub scan2: crate::Reg<scan2::SCAN2_SPEC>,
    #[doc = "0xac - No Description"]
    pub scan3: crate::Reg<scan3::SCAN3_SPEC>,
    #[doc = "0xb0 - No Description"]
    pub scan4: crate::Reg<scan4::SCAN4_SPEC>,
    #[doc = "0xb4 - No Description"]
    pub scan5: crate::Reg<scan5::SCAN5_SPEC>,
    #[doc = "0xb8 - No Description"]
    pub scan6: crate::Reg<scan6::SCAN6_SPEC>,
    #[doc = "0xbc - No Description"]
    pub scan7: crate::Reg<scan7::SCAN7_SPEC>,
    #[doc = "0xc0 - No Description"]
    pub scan8: crate::Reg<scan8::SCAN8_SPEC>,
    #[doc = "0xc4 - No Description"]
    pub scan9: crate::Reg<scan9::SCAN9_SPEC>,
    #[doc = "0xc8 - No Description"]
    pub scan10: crate::Reg<scan10::SCAN10_SPEC>,
    #[doc = "0xcc - No Description"]
    pub scan11: crate::Reg<scan11::SCAN11_SPEC>,
    #[doc = "0xd0 - No Description"]
    pub scan12: crate::Reg<scan12::SCAN12_SPEC>,
    #[doc = "0xd4 - No Description"]
    pub scan13: crate::Reg<scan13::SCAN13_SPEC>,
    #[doc = "0xd8 - No Description"]
    pub scan14: crate::Reg<scan14::SCAN14_SPEC>,
    #[doc = "0xdc - No Description"]
    pub scan15: crate::Reg<scan15::SCAN15_SPEC>,
}
#[doc = "IPVERSION register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "IPVERSION"]
pub mod ipversion;
#[doc = "EN register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "Enable"]
pub mod en;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command"]
pub mod cmd;
#[doc = "TIMER register accessor: an alias for `Reg<TIMER_SPEC>`"]
pub type TIMER = crate::Reg<timer::TIMER_SPEC>;
#[doc = "Timer"]
pub mod timer;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "MASKREQ register accessor: an alias for `Reg<MASKREQ_SPEC>`"]
pub type MASKREQ = crate::Reg<maskreq::MASKREQ_SPEC>;
#[doc = "Mask Request"]
pub mod maskreq;
#[doc = "STMASK register accessor: an alias for `Reg<STMASK_SPEC>`"]
pub type STMASK = crate::Reg<stmask::STMASK_SPEC>;
#[doc = "Scan Table Mask"]
pub mod stmask;
#[doc = "CMPTHR register accessor: an alias for `Reg<CMPTHR_SPEC>`"]
pub type CMPTHR = crate::Reg<cmpthr::CMPTHR_SPEC>;
#[doc = "Comparator Threshold"]
pub mod cmpthr;
#[doc = "IF register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt Flag"]
pub mod if_;
#[doc = "IEN register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enable"]
pub mod ien;
#[doc = "TRIGGER register accessor: an alias for `Reg<TRIGGER_SPEC>`"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "Trigger"]
pub mod trigger;
#[doc = "CFG0 register accessor: an alias for `Reg<CFG0_SPEC>`"]
pub type CFG0 = crate::Reg<cfg0::CFG0_SPEC>;
#[doc = "Configration"]
pub mod cfg0;
#[doc = "SCALE0 register accessor: an alias for `Reg<SCALE0_SPEC>`"]
pub type SCALE0 = crate::Reg<scale0::SCALE0_SPEC>;
#[doc = "Scale"]
pub mod scale0;
#[doc = "SCHED0 register accessor: an alias for `Reg<SCHED0_SPEC>`"]
pub type SCHED0 = crate::Reg<sched0::SCHED0_SPEC>;
#[doc = "Scheduling"]
pub mod sched0;
#[doc = "CFG1 register accessor: an alias for `Reg<CFG1_SPEC>`"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "Configration"]
pub mod cfg1;
#[doc = "SCALE1 register accessor: an alias for `Reg<SCALE1_SPEC>`"]
pub type SCALE1 = crate::Reg<scale1::SCALE1_SPEC>;
#[doc = "Scale"]
pub mod scale1;
#[doc = "SCHED1 register accessor: an alias for `Reg<SCHED1_SPEC>`"]
pub type SCHED1 = crate::Reg<sched1::SCHED1_SPEC>;
#[doc = "Scheduling"]
pub mod sched1;
#[doc = "SINGLEFIFOCFG register accessor: an alias for `Reg<SINGLEFIFOCFG_SPEC>`"]
pub type SINGLEFIFOCFG = crate::Reg<singlefifocfg::SINGLEFIFOCFG_SPEC>;
#[doc = "Single FIFO Configuration"]
pub mod singlefifocfg;
#[doc = "SINGLEFIFODATA register accessor: an alias for `Reg<SINGLEFIFODATA_SPEC>`"]
pub type SINGLEFIFODATA = crate::Reg<singlefifodata::SINGLEFIFODATA_SPEC>;
#[doc = "Read the oldest valid data from the single FIFO and pop the FIFO"]
pub mod singlefifodata;
#[doc = "SINGLEFIFOSTAT register accessor: an alias for `Reg<SINGLEFIFOSTAT_SPEC>`"]
pub type SINGLEFIFOSTAT = crate::Reg<singlefifostat::SINGLEFIFOSTAT_SPEC>;
#[doc = "Single FIFO status"]
pub mod singlefifostat;
#[doc = "SINGLEDATA register accessor: an alias for `Reg<SINGLEDATA_SPEC>`"]
pub type SINGLEDATA = crate::Reg<singledata::SINGLEDATA_SPEC>;
#[doc = "latest single queue conversion data"]
pub mod singledata;
#[doc = "SCANFIFOCFG register accessor: an alias for `Reg<SCANFIFOCFG_SPEC>`"]
pub type SCANFIFOCFG = crate::Reg<scanfifocfg::SCANFIFOCFG_SPEC>;
#[doc = "SCAN FIFO configuration"]
pub mod scanfifocfg;
#[doc = "SCANFIFODATA register accessor: an alias for `Reg<SCANFIFODATA_SPEC>`"]
pub type SCANFIFODATA = crate::Reg<scanfifodata::SCANFIFODATA_SPEC>;
#[doc = "Read the oldest valid data from the scan FIFO and pop the FIFO"]
pub mod scanfifodata;
#[doc = "SCANFIFOSTAT register accessor: an alias for `Reg<SCANFIFOSTAT_SPEC>`"]
pub type SCANFIFOSTAT = crate::Reg<scanfifostat::SCANFIFOSTAT_SPEC>;
#[doc = "Scan FIFO status"]
pub mod scanfifostat;
#[doc = "SCANDATA register accessor: an alias for `Reg<SCANDATA_SPEC>`"]
pub type SCANDATA = crate::Reg<scandata::SCANDATA_SPEC>;
#[doc = "Most recent data data from scan queue conversion"]
pub mod scandata;
#[doc = "SINGLE register accessor: an alias for `Reg<SINGLE_SPEC>`"]
pub type SINGLE = crate::Reg<single::SINGLE_SPEC>;
#[doc = "No Description"]
pub mod single;
#[doc = "SCAN0 register accessor: an alias for `Reg<SCAN0_SPEC>`"]
pub type SCAN0 = crate::Reg<scan0::SCAN0_SPEC>;
#[doc = "No Description"]
pub mod scan0;
#[doc = "SCAN1 register accessor: an alias for `Reg<SCAN1_SPEC>`"]
pub type SCAN1 = crate::Reg<scan1::SCAN1_SPEC>;
#[doc = "No Description"]
pub mod scan1;
#[doc = "SCAN2 register accessor: an alias for `Reg<SCAN2_SPEC>`"]
pub type SCAN2 = crate::Reg<scan2::SCAN2_SPEC>;
#[doc = "No Description"]
pub mod scan2;
#[doc = "SCAN3 register accessor: an alias for `Reg<SCAN3_SPEC>`"]
pub type SCAN3 = crate::Reg<scan3::SCAN3_SPEC>;
#[doc = "No Description"]
pub mod scan3;
#[doc = "SCAN4 register accessor: an alias for `Reg<SCAN4_SPEC>`"]
pub type SCAN4 = crate::Reg<scan4::SCAN4_SPEC>;
#[doc = "No Description"]
pub mod scan4;
#[doc = "SCAN5 register accessor: an alias for `Reg<SCAN5_SPEC>`"]
pub type SCAN5 = crate::Reg<scan5::SCAN5_SPEC>;
#[doc = "No Description"]
pub mod scan5;
#[doc = "SCAN6 register accessor: an alias for `Reg<SCAN6_SPEC>`"]
pub type SCAN6 = crate::Reg<scan6::SCAN6_SPEC>;
#[doc = "No Description"]
pub mod scan6;
#[doc = "SCAN7 register accessor: an alias for `Reg<SCAN7_SPEC>`"]
pub type SCAN7 = crate::Reg<scan7::SCAN7_SPEC>;
#[doc = "No Description"]
pub mod scan7;
#[doc = "SCAN8 register accessor: an alias for `Reg<SCAN8_SPEC>`"]
pub type SCAN8 = crate::Reg<scan8::SCAN8_SPEC>;
#[doc = "No Description"]
pub mod scan8;
#[doc = "SCAN9 register accessor: an alias for `Reg<SCAN9_SPEC>`"]
pub type SCAN9 = crate::Reg<scan9::SCAN9_SPEC>;
#[doc = "No Description"]
pub mod scan9;
#[doc = "SCAN10 register accessor: an alias for `Reg<SCAN10_SPEC>`"]
pub type SCAN10 = crate::Reg<scan10::SCAN10_SPEC>;
#[doc = "No Description"]
pub mod scan10;
#[doc = "SCAN11 register accessor: an alias for `Reg<SCAN11_SPEC>`"]
pub type SCAN11 = crate::Reg<scan11::SCAN11_SPEC>;
#[doc = "No Description"]
pub mod scan11;
#[doc = "SCAN12 register accessor: an alias for `Reg<SCAN12_SPEC>`"]
pub type SCAN12 = crate::Reg<scan12::SCAN12_SPEC>;
#[doc = "No Description"]
pub mod scan12;
#[doc = "SCAN13 register accessor: an alias for `Reg<SCAN13_SPEC>`"]
pub type SCAN13 = crate::Reg<scan13::SCAN13_SPEC>;
#[doc = "No Description"]
pub mod scan13;
#[doc = "SCAN14 register accessor: an alias for `Reg<SCAN14_SPEC>`"]
pub type SCAN14 = crate::Reg<scan14::SCAN14_SPEC>;
#[doc = "No Description"]
pub mod scan14;
#[doc = "SCAN15 register accessor: an alias for `Reg<SCAN15_SPEC>`"]
pub type SCAN15 = crate::Reg<scan15::SCAN15_SPEC>;
#[doc = "No Description"]
pub mod scan15;
