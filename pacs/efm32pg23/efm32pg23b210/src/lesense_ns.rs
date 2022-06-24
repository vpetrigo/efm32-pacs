#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IPVERSION"]
    pub ipversion: crate::Reg<ipversion::IPVERSION_SPEC>,
    #[doc = "0x04 - Global Enable of LESENSE functions"]
    pub en: crate::Reg<en::EN_SPEC>,
    #[doc = "0x08 - No Description"]
    pub swrst: crate::Reg<swrst::SWRST_SPEC>,
    #[doc = "0x0c - Configuration Register"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x10 - Timing Control Register"]
    pub timctrl: crate::Reg<timctrl::TIMCTRL_SPEC>,
    #[doc = "0x14 - Peripheral Control Register"]
    pub perctrl: crate::Reg<perctrl::PERCTRL_SPEC>,
    #[doc = "0x18 - Decoder control Register"]
    pub decctrl: crate::Reg<decctrl::DECCTRL_SPEC>,
    #[doc = "0x1c - LESENSE evaluation control"]
    pub evalctrl: crate::Reg<evalctrl::EVALCTRL_SPEC>,
    #[doc = "0x20 - PRS control register"]
    pub prsctrl: crate::Reg<prsctrl::PRSCTRL_SPEC>,
    #[doc = "0x24 - Command Register"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x28 - Channel enable Register"]
    pub chen: crate::Reg<chen::CHEN_SPEC>,
    #[doc = "0x2c - Scan result register"]
    pub scanres: crate::Reg<scanres::SCANRES_SPEC>,
    #[doc = "0x30 - Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x34 - Result FIFO Count"]
    pub rescount: crate::Reg<rescount::RESCOUNT_SPEC>,
    #[doc = "0x38 - Result Fifo"]
    pub resfifo: crate::Reg<resfifo::RESFIFO_SPEC>,
    #[doc = "0x3c - Current channel index"]
    pub curch: crate::Reg<curch::CURCH_SPEC>,
    #[doc = "0x40 - Current decoder state"]
    pub decstate: crate::Reg<decstate::DECSTATE_SPEC>,
    #[doc = "0x44 - Decoder input register"]
    pub sensorstate: crate::Reg<sensorstate::SENSORSTATE_SPEC>,
    #[doc = "0x48 - GPIO Idle phase configuration"]
    pub idleconf: crate::Reg<idleconf::IDLECONF_SPEC>,
    _reserved19: [u8; 0x04],
    #[doc = "0x50 - Synchronization Busy Register"]
    pub syncbusy: crate::Reg<syncbusy::SYNCBUSY_SPEC>,
    _reserved20: [u8; 0x0c],
    #[doc = "0x60 - Interrupt Flags"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x64 - Interrupt Enables"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    _reserved22: [u8; 0x98],
    #[doc = "0x100 - No Description"]
    pub ch0_timing: crate::Reg<ch0_timing::CH0_TIMING_SPEC>,
    #[doc = "0x104 - No Description"]
    pub ch0_interact: crate::Reg<ch0_interact::CH0_INTERACT_SPEC>,
    #[doc = "0x108 - No Description"]
    pub ch0_evalcfg: crate::Reg<ch0_evalcfg::CH0_EVALCFG_SPEC>,
    #[doc = "0x10c - No Description"]
    pub ch0_evalthres: crate::Reg<ch0_evalthres::CH0_EVALTHRES_SPEC>,
    #[doc = "0x110 - No Description"]
    pub ch1_timing: crate::Reg<ch1_timing::CH1_TIMING_SPEC>,
    #[doc = "0x114 - No Description"]
    pub ch1_interact: crate::Reg<ch1_interact::CH1_INTERACT_SPEC>,
    #[doc = "0x118 - No Description"]
    pub ch1_evalcfg: crate::Reg<ch1_evalcfg::CH1_EVALCFG_SPEC>,
    #[doc = "0x11c - No Description"]
    pub ch1_evalthres: crate::Reg<ch1_evalthres::CH1_EVALTHRES_SPEC>,
    #[doc = "0x120 - No Description"]
    pub ch2_timing: crate::Reg<ch2_timing::CH2_TIMING_SPEC>,
    #[doc = "0x124 - No Description"]
    pub ch2_interact: crate::Reg<ch2_interact::CH2_INTERACT_SPEC>,
    #[doc = "0x128 - No Description"]
    pub ch2_evalcfg: crate::Reg<ch2_evalcfg::CH2_EVALCFG_SPEC>,
    #[doc = "0x12c - No Description"]
    pub ch2_evalthres: crate::Reg<ch2_evalthres::CH2_EVALTHRES_SPEC>,
    #[doc = "0x130 - No Description"]
    pub ch3_timing: crate::Reg<ch3_timing::CH3_TIMING_SPEC>,
    #[doc = "0x134 - No Description"]
    pub ch3_interact: crate::Reg<ch3_interact::CH3_INTERACT_SPEC>,
    #[doc = "0x138 - No Description"]
    pub ch3_evalcfg: crate::Reg<ch3_evalcfg::CH3_EVALCFG_SPEC>,
    #[doc = "0x13c - No Description"]
    pub ch3_evalthres: crate::Reg<ch3_evalthres::CH3_EVALTHRES_SPEC>,
    #[doc = "0x140 - No Description"]
    pub ch4_timing: crate::Reg<ch4_timing::CH4_TIMING_SPEC>,
    #[doc = "0x144 - No Description"]
    pub ch4_interact: crate::Reg<ch4_interact::CH4_INTERACT_SPEC>,
    #[doc = "0x148 - No Description"]
    pub ch4_evalcfg: crate::Reg<ch4_evalcfg::CH4_EVALCFG_SPEC>,
    #[doc = "0x14c - No Description"]
    pub ch4_evalthres: crate::Reg<ch4_evalthres::CH4_EVALTHRES_SPEC>,
    #[doc = "0x150 - No Description"]
    pub ch5_timing: crate::Reg<ch5_timing::CH5_TIMING_SPEC>,
    #[doc = "0x154 - No Description"]
    pub ch5_interact: crate::Reg<ch5_interact::CH5_INTERACT_SPEC>,
    #[doc = "0x158 - No Description"]
    pub ch5_evalcfg: crate::Reg<ch5_evalcfg::CH5_EVALCFG_SPEC>,
    #[doc = "0x15c - No Description"]
    pub ch5_evalthres: crate::Reg<ch5_evalthres::CH5_EVALTHRES_SPEC>,
    #[doc = "0x160 - No Description"]
    pub ch6_timing: crate::Reg<ch6_timing::CH6_TIMING_SPEC>,
    #[doc = "0x164 - No Description"]
    pub ch6_interact: crate::Reg<ch6_interact::CH6_INTERACT_SPEC>,
    #[doc = "0x168 - No Description"]
    pub ch6_evalcfg: crate::Reg<ch6_evalcfg::CH6_EVALCFG_SPEC>,
    #[doc = "0x16c - No Description"]
    pub ch6_evalthres: crate::Reg<ch6_evalthres::CH6_EVALTHRES_SPEC>,
    #[doc = "0x170 - No Description"]
    pub ch7_timing: crate::Reg<ch7_timing::CH7_TIMING_SPEC>,
    #[doc = "0x174 - No Description"]
    pub ch7_interact: crate::Reg<ch7_interact::CH7_INTERACT_SPEC>,
    #[doc = "0x178 - No Description"]
    pub ch7_evalcfg: crate::Reg<ch7_evalcfg::CH7_EVALCFG_SPEC>,
    #[doc = "0x17c - No Description"]
    pub ch7_evalthres: crate::Reg<ch7_evalthres::CH7_EVALTHRES_SPEC>,
    #[doc = "0x180 - No Description"]
    pub ch8_timing: crate::Reg<ch8_timing::CH8_TIMING_SPEC>,
    #[doc = "0x184 - No Description"]
    pub ch8_interact: crate::Reg<ch8_interact::CH8_INTERACT_SPEC>,
    #[doc = "0x188 - No Description"]
    pub ch8_evalcfg: crate::Reg<ch8_evalcfg::CH8_EVALCFG_SPEC>,
    #[doc = "0x18c - No Description"]
    pub ch8_evalthres: crate::Reg<ch8_evalthres::CH8_EVALTHRES_SPEC>,
    #[doc = "0x190 - No Description"]
    pub ch9_timing: crate::Reg<ch9_timing::CH9_TIMING_SPEC>,
    #[doc = "0x194 - No Description"]
    pub ch9_interact: crate::Reg<ch9_interact::CH9_INTERACT_SPEC>,
    #[doc = "0x198 - No Description"]
    pub ch9_evalcfg: crate::Reg<ch9_evalcfg::CH9_EVALCFG_SPEC>,
    #[doc = "0x19c - No Description"]
    pub ch9_evalthres: crate::Reg<ch9_evalthres::CH9_EVALTHRES_SPEC>,
    #[doc = "0x1a0 - No Description"]
    pub ch10_timing: crate::Reg<ch10_timing::CH10_TIMING_SPEC>,
    #[doc = "0x1a4 - No Description"]
    pub ch10_interact: crate::Reg<ch10_interact::CH10_INTERACT_SPEC>,
    #[doc = "0x1a8 - No Description"]
    pub ch10_evalcfg: crate::Reg<ch10_evalcfg::CH10_EVALCFG_SPEC>,
    #[doc = "0x1ac - No Description"]
    pub ch10_evalthres: crate::Reg<ch10_evalthres::CH10_EVALTHRES_SPEC>,
    #[doc = "0x1b0 - No Description"]
    pub ch11_timing: crate::Reg<ch11_timing::CH11_TIMING_SPEC>,
    #[doc = "0x1b4 - No Description"]
    pub ch11_interact: crate::Reg<ch11_interact::CH11_INTERACT_SPEC>,
    #[doc = "0x1b8 - No Description"]
    pub ch11_evalcfg: crate::Reg<ch11_evalcfg::CH11_EVALCFG_SPEC>,
    #[doc = "0x1bc - No Description"]
    pub ch11_evalthres: crate::Reg<ch11_evalthres::CH11_EVALTHRES_SPEC>,
    #[doc = "0x1c0 - No Description"]
    pub ch12_timing: crate::Reg<ch12_timing::CH12_TIMING_SPEC>,
    #[doc = "0x1c4 - No Description"]
    pub ch12_interact: crate::Reg<ch12_interact::CH12_INTERACT_SPEC>,
    #[doc = "0x1c8 - No Description"]
    pub ch12_evalcfg: crate::Reg<ch12_evalcfg::CH12_EVALCFG_SPEC>,
    #[doc = "0x1cc - No Description"]
    pub ch12_evalthres: crate::Reg<ch12_evalthres::CH12_EVALTHRES_SPEC>,
    #[doc = "0x1d0 - No Description"]
    pub ch13_timing: crate::Reg<ch13_timing::CH13_TIMING_SPEC>,
    #[doc = "0x1d4 - No Description"]
    pub ch13_interact: crate::Reg<ch13_interact::CH13_INTERACT_SPEC>,
    #[doc = "0x1d8 - No Description"]
    pub ch13_evalcfg: crate::Reg<ch13_evalcfg::CH13_EVALCFG_SPEC>,
    #[doc = "0x1dc - No Description"]
    pub ch13_evalthres: crate::Reg<ch13_evalthres::CH13_EVALTHRES_SPEC>,
    #[doc = "0x1e0 - No Description"]
    pub ch14_timing: crate::Reg<ch14_timing::CH14_TIMING_SPEC>,
    #[doc = "0x1e4 - No Description"]
    pub ch14_interact: crate::Reg<ch14_interact::CH14_INTERACT_SPEC>,
    #[doc = "0x1e8 - No Description"]
    pub ch14_evalcfg: crate::Reg<ch14_evalcfg::CH14_EVALCFG_SPEC>,
    #[doc = "0x1ec - No Description"]
    pub ch14_evalthres: crate::Reg<ch14_evalthres::CH14_EVALTHRES_SPEC>,
    #[doc = "0x1f0 - No Description"]
    pub ch15_timing: crate::Reg<ch15_timing::CH15_TIMING_SPEC>,
    #[doc = "0x1f4 - No Description"]
    pub ch15_interact: crate::Reg<ch15_interact::CH15_INTERACT_SPEC>,
    #[doc = "0x1f8 - No Description"]
    pub ch15_evalcfg: crate::Reg<ch15_evalcfg::CH15_EVALCFG_SPEC>,
    #[doc = "0x1fc - No Description"]
    pub ch15_evalthres: crate::Reg<ch15_evalthres::CH15_EVALTHRES_SPEC>,
    #[doc = "0x200 - No Description"]
    pub st0_arc: crate::Reg<st0_arc::ST0_ARC_SPEC>,
    #[doc = "0x204 - No Description"]
    pub st1_arc: crate::Reg<st1_arc::ST1_ARC_SPEC>,
    #[doc = "0x208 - No Description"]
    pub st2_arc: crate::Reg<st2_arc::ST2_ARC_SPEC>,
    #[doc = "0x20c - No Description"]
    pub st3_arc: crate::Reg<st3_arc::ST3_ARC_SPEC>,
    #[doc = "0x210 - No Description"]
    pub st4_arc: crate::Reg<st4_arc::ST4_ARC_SPEC>,
    #[doc = "0x214 - No Description"]
    pub st5_arc: crate::Reg<st5_arc::ST5_ARC_SPEC>,
    #[doc = "0x218 - No Description"]
    pub st6_arc: crate::Reg<st6_arc::ST6_ARC_SPEC>,
    #[doc = "0x21c - No Description"]
    pub st7_arc: crate::Reg<st7_arc::ST7_ARC_SPEC>,
    #[doc = "0x220 - No Description"]
    pub st8_arc: crate::Reg<st8_arc::ST8_ARC_SPEC>,
    #[doc = "0x224 - No Description"]
    pub st9_arc: crate::Reg<st9_arc::ST9_ARC_SPEC>,
    #[doc = "0x228 - No Description"]
    pub st10_arc: crate::Reg<st10_arc::ST10_ARC_SPEC>,
    #[doc = "0x22c - No Description"]
    pub st11_arc: crate::Reg<st11_arc::ST11_ARC_SPEC>,
    #[doc = "0x230 - No Description"]
    pub st12_arc: crate::Reg<st12_arc::ST12_ARC_SPEC>,
    #[doc = "0x234 - No Description"]
    pub st13_arc: crate::Reg<st13_arc::ST13_ARC_SPEC>,
    #[doc = "0x238 - No Description"]
    pub st14_arc: crate::Reg<st14_arc::ST14_ARC_SPEC>,
    #[doc = "0x23c - No Description"]
    pub st15_arc: crate::Reg<st15_arc::ST15_ARC_SPEC>,
    #[doc = "0x240 - No Description"]
    pub st16_arc: crate::Reg<st16_arc::ST16_ARC_SPEC>,
    #[doc = "0x244 - No Description"]
    pub st17_arc: crate::Reg<st17_arc::ST17_ARC_SPEC>,
    #[doc = "0x248 - No Description"]
    pub st18_arc: crate::Reg<st18_arc::ST18_ARC_SPEC>,
    #[doc = "0x24c - No Description"]
    pub st19_arc: crate::Reg<st19_arc::ST19_ARC_SPEC>,
    #[doc = "0x250 - No Description"]
    pub st20_arc: crate::Reg<st20_arc::ST20_ARC_SPEC>,
    #[doc = "0x254 - No Description"]
    pub st21_arc: crate::Reg<st21_arc::ST21_ARC_SPEC>,
    #[doc = "0x258 - No Description"]
    pub st22_arc: crate::Reg<st22_arc::ST22_ARC_SPEC>,
    #[doc = "0x25c - No Description"]
    pub st23_arc: crate::Reg<st23_arc::ST23_ARC_SPEC>,
    #[doc = "0x260 - No Description"]
    pub st24_arc: crate::Reg<st24_arc::ST24_ARC_SPEC>,
    #[doc = "0x264 - No Description"]
    pub st25_arc: crate::Reg<st25_arc::ST25_ARC_SPEC>,
    #[doc = "0x268 - No Description"]
    pub st26_arc: crate::Reg<st26_arc::ST26_ARC_SPEC>,
    #[doc = "0x26c - No Description"]
    pub st27_arc: crate::Reg<st27_arc::ST27_ARC_SPEC>,
    #[doc = "0x270 - No Description"]
    pub st28_arc: crate::Reg<st28_arc::ST28_ARC_SPEC>,
    #[doc = "0x274 - No Description"]
    pub st29_arc: crate::Reg<st29_arc::ST29_ARC_SPEC>,
    #[doc = "0x278 - No Description"]
    pub st30_arc: crate::Reg<st30_arc::ST30_ARC_SPEC>,
    #[doc = "0x27c - No Description"]
    pub st31_arc: crate::Reg<st31_arc::ST31_ARC_SPEC>,
    #[doc = "0x280 - No Description"]
    pub st32_arc: crate::Reg<st32_arc::ST32_ARC_SPEC>,
    #[doc = "0x284 - No Description"]
    pub st33_arc: crate::Reg<st33_arc::ST33_ARC_SPEC>,
    #[doc = "0x288 - No Description"]
    pub st34_arc: crate::Reg<st34_arc::ST34_ARC_SPEC>,
    #[doc = "0x28c - No Description"]
    pub st35_arc: crate::Reg<st35_arc::ST35_ARC_SPEC>,
    #[doc = "0x290 - No Description"]
    pub st36_arc: crate::Reg<st36_arc::ST36_ARC_SPEC>,
    #[doc = "0x294 - No Description"]
    pub st37_arc: crate::Reg<st37_arc::ST37_ARC_SPEC>,
    #[doc = "0x298 - No Description"]
    pub st38_arc: crate::Reg<st38_arc::ST38_ARC_SPEC>,
    #[doc = "0x29c - No Description"]
    pub st39_arc: crate::Reg<st39_arc::ST39_ARC_SPEC>,
    #[doc = "0x2a0 - No Description"]
    pub st40_arc: crate::Reg<st40_arc::ST40_ARC_SPEC>,
    #[doc = "0x2a4 - No Description"]
    pub st41_arc: crate::Reg<st41_arc::ST41_ARC_SPEC>,
    #[doc = "0x2a8 - No Description"]
    pub st42_arc: crate::Reg<st42_arc::ST42_ARC_SPEC>,
    #[doc = "0x2ac - No Description"]
    pub st43_arc: crate::Reg<st43_arc::ST43_ARC_SPEC>,
    #[doc = "0x2b0 - No Description"]
    pub st44_arc: crate::Reg<st44_arc::ST44_ARC_SPEC>,
    #[doc = "0x2b4 - No Description"]
    pub st45_arc: crate::Reg<st45_arc::ST45_ARC_SPEC>,
    #[doc = "0x2b8 - No Description"]
    pub st46_arc: crate::Reg<st46_arc::ST46_ARC_SPEC>,
    #[doc = "0x2bc - No Description"]
    pub st47_arc: crate::Reg<st47_arc::ST47_ARC_SPEC>,
    #[doc = "0x2c0 - No Description"]
    pub st48_arc: crate::Reg<st48_arc::ST48_ARC_SPEC>,
    #[doc = "0x2c4 - No Description"]
    pub st49_arc: crate::Reg<st49_arc::ST49_ARC_SPEC>,
    #[doc = "0x2c8 - No Description"]
    pub st50_arc: crate::Reg<st50_arc::ST50_ARC_SPEC>,
    #[doc = "0x2cc - No Description"]
    pub st51_arc: crate::Reg<st51_arc::ST51_ARC_SPEC>,
    #[doc = "0x2d0 - No Description"]
    pub st52_arc: crate::Reg<st52_arc::ST52_ARC_SPEC>,
    #[doc = "0x2d4 - No Description"]
    pub st53_arc: crate::Reg<st53_arc::ST53_ARC_SPEC>,
    #[doc = "0x2d8 - No Description"]
    pub st54_arc: crate::Reg<st54_arc::ST54_ARC_SPEC>,
    #[doc = "0x2dc - No Description"]
    pub st55_arc: crate::Reg<st55_arc::ST55_ARC_SPEC>,
    #[doc = "0x2e0 - No Description"]
    pub st56_arc: crate::Reg<st56_arc::ST56_ARC_SPEC>,
    #[doc = "0x2e4 - No Description"]
    pub st57_arc: crate::Reg<st57_arc::ST57_ARC_SPEC>,
    #[doc = "0x2e8 - No Description"]
    pub st58_arc: crate::Reg<st58_arc::ST58_ARC_SPEC>,
    #[doc = "0x2ec - No Description"]
    pub st59_arc: crate::Reg<st59_arc::ST59_ARC_SPEC>,
    #[doc = "0x2f0 - No Description"]
    pub st60_arc: crate::Reg<st60_arc::ST60_ARC_SPEC>,
    #[doc = "0x2f4 - No Description"]
    pub st61_arc: crate::Reg<st61_arc::ST61_ARC_SPEC>,
    #[doc = "0x2f8 - No Description"]
    pub st62_arc: crate::Reg<st62_arc::ST62_ARC_SPEC>,
    #[doc = "0x2fc - No Description"]
    pub st63_arc: crate::Reg<st63_arc::ST63_ARC_SPEC>,
}
#[doc = "IPVERSION register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "IPVERSION"]
pub mod ipversion;
#[doc = "EN register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "Global Enable of LESENSE functions"]
pub mod en;
#[doc = "SWRST register accessor: an alias for `Reg<SWRST_SPEC>`"]
pub type SWRST = crate::Reg<swrst::SWRST_SPEC>;
#[doc = "No Description"]
pub mod swrst;
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "TIMCTRL register accessor: an alias for `Reg<TIMCTRL_SPEC>`"]
pub type TIMCTRL = crate::Reg<timctrl::TIMCTRL_SPEC>;
#[doc = "Timing Control Register"]
pub mod timctrl;
#[doc = "PERCTRL register accessor: an alias for `Reg<PERCTRL_SPEC>`"]
pub type PERCTRL = crate::Reg<perctrl::PERCTRL_SPEC>;
#[doc = "Peripheral Control Register"]
pub mod perctrl;
#[doc = "DECCTRL register accessor: an alias for `Reg<DECCTRL_SPEC>`"]
pub type DECCTRL = crate::Reg<decctrl::DECCTRL_SPEC>;
#[doc = "Decoder control Register"]
pub mod decctrl;
#[doc = "EVALCTRL register accessor: an alias for `Reg<EVALCTRL_SPEC>`"]
pub type EVALCTRL = crate::Reg<evalctrl::EVALCTRL_SPEC>;
#[doc = "LESENSE evaluation control"]
pub mod evalctrl;
#[doc = "PRSCTRL register accessor: an alias for `Reg<PRSCTRL_SPEC>`"]
pub type PRSCTRL = crate::Reg<prsctrl::PRSCTRL_SPEC>;
#[doc = "PRS control register"]
pub mod prsctrl;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "CHEN register accessor: an alias for `Reg<CHEN_SPEC>`"]
pub type CHEN = crate::Reg<chen::CHEN_SPEC>;
#[doc = "Channel enable Register"]
pub mod chen;
#[doc = "SCANRES register accessor: an alias for `Reg<SCANRES_SPEC>`"]
pub type SCANRES = crate::Reg<scanres::SCANRES_SPEC>;
#[doc = "Scan result register"]
pub mod scanres;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "RESCOUNT register accessor: an alias for `Reg<RESCOUNT_SPEC>`"]
pub type RESCOUNT = crate::Reg<rescount::RESCOUNT_SPEC>;
#[doc = "Result FIFO Count"]
pub mod rescount;
#[doc = "RESFIFO register accessor: an alias for `Reg<RESFIFO_SPEC>`"]
pub type RESFIFO = crate::Reg<resfifo::RESFIFO_SPEC>;
#[doc = "Result Fifo"]
pub mod resfifo;
#[doc = "CURCH register accessor: an alias for `Reg<CURCH_SPEC>`"]
pub type CURCH = crate::Reg<curch::CURCH_SPEC>;
#[doc = "Current channel index"]
pub mod curch;
#[doc = "DECSTATE register accessor: an alias for `Reg<DECSTATE_SPEC>`"]
pub type DECSTATE = crate::Reg<decstate::DECSTATE_SPEC>;
#[doc = "Current decoder state"]
pub mod decstate;
#[doc = "SENSORSTATE register accessor: an alias for `Reg<SENSORSTATE_SPEC>`"]
pub type SENSORSTATE = crate::Reg<sensorstate::SENSORSTATE_SPEC>;
#[doc = "Decoder input register"]
pub mod sensorstate;
#[doc = "IDLECONF register accessor: an alias for `Reg<IDLECONF_SPEC>`"]
pub type IDLECONF = crate::Reg<idleconf::IDLECONF_SPEC>;
#[doc = "GPIO Idle phase configuration"]
pub mod idleconf;
#[doc = "SYNCBUSY register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "IF register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt Flags"]
pub mod if_;
#[doc = "IEN register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enables"]
pub mod ien;
#[doc = "CH0_TIMING register accessor: an alias for `Reg<CH0_TIMING_SPEC>`"]
pub type CH0_TIMING = crate::Reg<ch0_timing::CH0_TIMING_SPEC>;
#[doc = "No Description"]
pub mod ch0_timing;
#[doc = "CH0_INTERACT register accessor: an alias for `Reg<CH0_INTERACT_SPEC>`"]
pub type CH0_INTERACT = crate::Reg<ch0_interact::CH0_INTERACT_SPEC>;
#[doc = "No Description"]
pub mod ch0_interact;
#[doc = "CH0_EVALCFG register accessor: an alias for `Reg<CH0_EVALCFG_SPEC>`"]
pub type CH0_EVALCFG = crate::Reg<ch0_evalcfg::CH0_EVALCFG_SPEC>;
#[doc = "No Description"]
pub mod ch0_evalcfg;
#[doc = "CH0_EVALTHRES register accessor: an alias for `Reg<CH0_EVALTHRES_SPEC>`"]
pub type CH0_EVALTHRES = crate::Reg<ch0_evalthres::CH0_EVALTHRES_SPEC>;
#[doc = "No Description"]
pub mod ch0_evalthres;
#[doc = "CH1_TIMING register accessor: an alias for `Reg<CH1_TIMING_SPEC>`"]
pub type CH1_TIMING = crate::Reg<ch1_timing::CH1_TIMING_SPEC>;
#[doc = "No Description"]
pub mod ch1_timing;
#[doc = "CH1_INTERACT register accessor: an alias for `Reg<CH1_INTERACT_SPEC>`"]
pub type CH1_INTERACT = crate::Reg<ch1_interact::CH1_INTERACT_SPEC>;
#[doc = "No Description"]
pub mod ch1_interact;
#[doc = "CH1_EVALCFG register accessor: an alias for `Reg<CH1_EVALCFG_SPEC>`"]
pub type CH1_EVALCFG = crate::Reg<ch1_evalcfg::CH1_EVALCFG_SPEC>;
#[doc = "No Description"]
pub mod ch1_evalcfg;
#[doc = "CH1_EVALTHRES register accessor: an alias for `Reg<CH1_EVALTHRES_SPEC>`"]
pub type CH1_EVALTHRES = crate::Reg<ch1_evalthres::CH1_EVALTHRES_SPEC>;
#[doc = "No Description"]
pub mod ch1_evalthres;
#[doc = "CH2_TIMING register accessor: an alias for `Reg<CH2_TIMING_SPEC>`"]
pub type CH2_TIMING = crate::Reg<ch2_timing::CH2_TIMING_SPEC>;
#[doc = "No Description"]
pub mod ch2_timing;
#[doc = "CH2_INTERACT register accessor: an alias for `Reg<CH2_INTERACT_SPEC>`"]
pub type CH2_INTERACT = crate::Reg<ch2_interact::CH2_INTERACT_SPEC>;
#[doc = "No Description"]
pub mod ch2_interact;
#[doc = "CH2_EVALCFG register accessor: an alias for `Reg<CH2_EVALCFG_SPEC>`"]
pub type CH2_EVALCFG = crate::Reg<ch2_evalcfg::CH2_EVALCFG_SPEC>;
#[doc = "No Description"]
pub mod ch2_evalcfg;
#[doc = "CH2_EVALTHRES register accessor: an alias for `Reg<CH2_EVALTHRES_SPEC>`"]
pub type CH2_EVALTHRES = crate::Reg<ch2_evalthres::CH2_EVALTHRES_SPEC>;
#[doc = "No Description"]
pub mod ch2_evalthres;
#[doc = "CH3_TIMING register accessor: an alias for `Reg<CH3_TIMING_SPEC>`"]
pub type CH3_TIMING = crate::Reg<ch3_timing::CH3_TIMING_SPEC>;
#[doc = "No Description"]
pub mod ch3_timing;
#[doc = "CH3_INTERACT register accessor: an alias for `Reg<CH3_INTERACT_SPEC>`"]
pub type CH3_INTERACT = crate::Reg<ch3_interact::CH3_INTERACT_SPEC>;
#[doc = "No Description"]
pub mod ch3_interact;
#[doc = "CH3_EVALCFG register accessor: an alias for `Reg<CH3_EVALCFG_SPEC>`"]
pub type CH3_EVALCFG = crate::Reg<ch3_evalcfg::CH3_EVALCFG_SPEC>;
#[doc = "No Description"]
pub mod ch3_evalcfg;
#[doc = "CH3_EVALTHRES register accessor: an alias for `Reg<CH3_EVALTHRES_SPEC>`"]
pub type CH3_EVALTHRES = crate::Reg<ch3_evalthres::CH3_EVALTHRES_SPEC>;
#[doc = "No Description"]
pub mod ch3_evalthres;
#[doc = "CH4_TIMING register accessor: an alias for `Reg<CH4_TIMING_SPEC>`"]
pub type CH4_TIMING = crate::Reg<ch4_timing::CH4_TIMING_SPEC>;
#[doc = "No Description"]
pub mod ch4_timing;
#[doc = "CH4_INTERACT register accessor: an alias for `Reg<CH4_INTERACT_SPEC>`"]
pub type CH4_INTERACT = crate::Reg<ch4_interact::CH4_INTERACT_SPEC>;
#[doc = "No Description"]
pub mod ch4_interact;
#[doc = "CH4_EVALCFG register accessor: an alias for `Reg<CH4_EVALCFG_SPEC>`"]
pub type CH4_EVALCFG = crate::Reg<ch4_evalcfg::CH4_EVALCFG_SPEC>;
#[doc = "No Description"]
pub mod ch4_evalcfg;
#[doc = "CH4_EVALTHRES register accessor: an alias for `Reg<CH4_EVALTHRES_SPEC>`"]
pub type CH4_EVALTHRES = crate::Reg<ch4_evalthres::CH4_EVALTHRES_SPEC>;
#[doc = "No Description"]
pub mod ch4_evalthres;
#[doc = "CH5_TIMING register accessor: an alias for `Reg<CH5_TIMING_SPEC>`"]
pub type CH5_TIMING = crate::Reg<ch5_timing::CH5_TIMING_SPEC>;
#[doc = "No Description"]
pub mod ch5_timing;
#[doc = "CH5_INTERACT register accessor: an alias for `Reg<CH5_INTERACT_SPEC>`"]
pub type CH5_INTERACT = crate::Reg<ch5_interact::CH5_INTERACT_SPEC>;
#[doc = "No Description"]
pub mod ch5_interact;
#[doc = "CH5_EVALCFG register accessor: an alias for `Reg<CH5_EVALCFG_SPEC>`"]
pub type CH5_EVALCFG = crate::Reg<ch5_evalcfg::CH5_EVALCFG_SPEC>;
#[doc = "No Description"]
pub mod ch5_evalcfg;
#[doc = "CH5_EVALTHRES register accessor: an alias for `Reg<CH5_EVALTHRES_SPEC>`"]
pub type CH5_EVALTHRES = crate::Reg<ch5_evalthres::CH5_EVALTHRES_SPEC>;
#[doc = "No Description"]
pub mod ch5_evalthres;
#[doc = "CH6_TIMING register accessor: an alias for `Reg<CH6_TIMING_SPEC>`"]
pub type CH6_TIMING = crate::Reg<ch6_timing::CH6_TIMING_SPEC>;
#[doc = "No Description"]
pub mod ch6_timing;
#[doc = "CH6_INTERACT register accessor: an alias for `Reg<CH6_INTERACT_SPEC>`"]
pub type CH6_INTERACT = crate::Reg<ch6_interact::CH6_INTERACT_SPEC>;
#[doc = "No Description"]
pub mod ch6_interact;
#[doc = "CH6_EVALCFG register accessor: an alias for `Reg<CH6_EVALCFG_SPEC>`"]
pub type CH6_EVALCFG = crate::Reg<ch6_evalcfg::CH6_EVALCFG_SPEC>;
#[doc = "No Description"]
pub mod ch6_evalcfg;
#[doc = "CH6_EVALTHRES register accessor: an alias for `Reg<CH6_EVALTHRES_SPEC>`"]
pub type CH6_EVALTHRES = crate::Reg<ch6_evalthres::CH6_EVALTHRES_SPEC>;
#[doc = "No Description"]
pub mod ch6_evalthres;
#[doc = "CH7_TIMING register accessor: an alias for `Reg<CH7_TIMING_SPEC>`"]
pub type CH7_TIMING = crate::Reg<ch7_timing::CH7_TIMING_SPEC>;
#[doc = "No Description"]
pub mod ch7_timing;
#[doc = "CH7_INTERACT register accessor: an alias for `Reg<CH7_INTERACT_SPEC>`"]
pub type CH7_INTERACT = crate::Reg<ch7_interact::CH7_INTERACT_SPEC>;
#[doc = "No Description"]
pub mod ch7_interact;
#[doc = "CH7_EVALCFG register accessor: an alias for `Reg<CH7_EVALCFG_SPEC>`"]
pub type CH7_EVALCFG = crate::Reg<ch7_evalcfg::CH7_EVALCFG_SPEC>;
#[doc = "No Description"]
pub mod ch7_evalcfg;
#[doc = "CH7_EVALTHRES register accessor: an alias for `Reg<CH7_EVALTHRES_SPEC>`"]
pub type CH7_EVALTHRES = crate::Reg<ch7_evalthres::CH7_EVALTHRES_SPEC>;
#[doc = "No Description"]
pub mod ch7_evalthres;
#[doc = "CH8_TIMING register accessor: an alias for `Reg<CH8_TIMING_SPEC>`"]
pub type CH8_TIMING = crate::Reg<ch8_timing::CH8_TIMING_SPEC>;
#[doc = "No Description"]
pub mod ch8_timing;
#[doc = "CH8_INTERACT register accessor: an alias for `Reg<CH8_INTERACT_SPEC>`"]
pub type CH8_INTERACT = crate::Reg<ch8_interact::CH8_INTERACT_SPEC>;
#[doc = "No Description"]
pub mod ch8_interact;
#[doc = "CH8_EVALCFG register accessor: an alias for `Reg<CH8_EVALCFG_SPEC>`"]
pub type CH8_EVALCFG = crate::Reg<ch8_evalcfg::CH8_EVALCFG_SPEC>;
#[doc = "No Description"]
pub mod ch8_evalcfg;
#[doc = "CH8_EVALTHRES register accessor: an alias for `Reg<CH8_EVALTHRES_SPEC>`"]
pub type CH8_EVALTHRES = crate::Reg<ch8_evalthres::CH8_EVALTHRES_SPEC>;
#[doc = "No Description"]
pub mod ch8_evalthres;
#[doc = "CH9_TIMING register accessor: an alias for `Reg<CH9_TIMING_SPEC>`"]
pub type CH9_TIMING = crate::Reg<ch9_timing::CH9_TIMING_SPEC>;
#[doc = "No Description"]
pub mod ch9_timing;
#[doc = "CH9_INTERACT register accessor: an alias for `Reg<CH9_INTERACT_SPEC>`"]
pub type CH9_INTERACT = crate::Reg<ch9_interact::CH9_INTERACT_SPEC>;
#[doc = "No Description"]
pub mod ch9_interact;
#[doc = "CH9_EVALCFG register accessor: an alias for `Reg<CH9_EVALCFG_SPEC>`"]
pub type CH9_EVALCFG = crate::Reg<ch9_evalcfg::CH9_EVALCFG_SPEC>;
#[doc = "No Description"]
pub mod ch9_evalcfg;
#[doc = "CH9_EVALTHRES register accessor: an alias for `Reg<CH9_EVALTHRES_SPEC>`"]
pub type CH9_EVALTHRES = crate::Reg<ch9_evalthres::CH9_EVALTHRES_SPEC>;
#[doc = "No Description"]
pub mod ch9_evalthres;
#[doc = "CH10_TIMING register accessor: an alias for `Reg<CH10_TIMING_SPEC>`"]
pub type CH10_TIMING = crate::Reg<ch10_timing::CH10_TIMING_SPEC>;
#[doc = "No Description"]
pub mod ch10_timing;
#[doc = "CH10_INTERACT register accessor: an alias for `Reg<CH10_INTERACT_SPEC>`"]
pub type CH10_INTERACT = crate::Reg<ch10_interact::CH10_INTERACT_SPEC>;
#[doc = "No Description"]
pub mod ch10_interact;
#[doc = "CH10_EVALCFG register accessor: an alias for `Reg<CH10_EVALCFG_SPEC>`"]
pub type CH10_EVALCFG = crate::Reg<ch10_evalcfg::CH10_EVALCFG_SPEC>;
#[doc = "No Description"]
pub mod ch10_evalcfg;
#[doc = "CH10_EVALTHRES register accessor: an alias for `Reg<CH10_EVALTHRES_SPEC>`"]
pub type CH10_EVALTHRES = crate::Reg<ch10_evalthres::CH10_EVALTHRES_SPEC>;
#[doc = "No Description"]
pub mod ch10_evalthres;
#[doc = "CH11_TIMING register accessor: an alias for `Reg<CH11_TIMING_SPEC>`"]
pub type CH11_TIMING = crate::Reg<ch11_timing::CH11_TIMING_SPEC>;
#[doc = "No Description"]
pub mod ch11_timing;
#[doc = "CH11_INTERACT register accessor: an alias for `Reg<CH11_INTERACT_SPEC>`"]
pub type CH11_INTERACT = crate::Reg<ch11_interact::CH11_INTERACT_SPEC>;
#[doc = "No Description"]
pub mod ch11_interact;
#[doc = "CH11_EVALCFG register accessor: an alias for `Reg<CH11_EVALCFG_SPEC>`"]
pub type CH11_EVALCFG = crate::Reg<ch11_evalcfg::CH11_EVALCFG_SPEC>;
#[doc = "No Description"]
pub mod ch11_evalcfg;
#[doc = "CH11_EVALTHRES register accessor: an alias for `Reg<CH11_EVALTHRES_SPEC>`"]
pub type CH11_EVALTHRES = crate::Reg<ch11_evalthres::CH11_EVALTHRES_SPEC>;
#[doc = "No Description"]
pub mod ch11_evalthres;
#[doc = "CH12_TIMING register accessor: an alias for `Reg<CH12_TIMING_SPEC>`"]
pub type CH12_TIMING = crate::Reg<ch12_timing::CH12_TIMING_SPEC>;
#[doc = "No Description"]
pub mod ch12_timing;
#[doc = "CH12_INTERACT register accessor: an alias for `Reg<CH12_INTERACT_SPEC>`"]
pub type CH12_INTERACT = crate::Reg<ch12_interact::CH12_INTERACT_SPEC>;
#[doc = "No Description"]
pub mod ch12_interact;
#[doc = "CH12_EVALCFG register accessor: an alias for `Reg<CH12_EVALCFG_SPEC>`"]
pub type CH12_EVALCFG = crate::Reg<ch12_evalcfg::CH12_EVALCFG_SPEC>;
#[doc = "No Description"]
pub mod ch12_evalcfg;
#[doc = "CH12_EVALTHRES register accessor: an alias for `Reg<CH12_EVALTHRES_SPEC>`"]
pub type CH12_EVALTHRES = crate::Reg<ch12_evalthres::CH12_EVALTHRES_SPEC>;
#[doc = "No Description"]
pub mod ch12_evalthres;
#[doc = "CH13_TIMING register accessor: an alias for `Reg<CH13_TIMING_SPEC>`"]
pub type CH13_TIMING = crate::Reg<ch13_timing::CH13_TIMING_SPEC>;
#[doc = "No Description"]
pub mod ch13_timing;
#[doc = "CH13_INTERACT register accessor: an alias for `Reg<CH13_INTERACT_SPEC>`"]
pub type CH13_INTERACT = crate::Reg<ch13_interact::CH13_INTERACT_SPEC>;
#[doc = "No Description"]
pub mod ch13_interact;
#[doc = "CH13_EVALCFG register accessor: an alias for `Reg<CH13_EVALCFG_SPEC>`"]
pub type CH13_EVALCFG = crate::Reg<ch13_evalcfg::CH13_EVALCFG_SPEC>;
#[doc = "No Description"]
pub mod ch13_evalcfg;
#[doc = "CH13_EVALTHRES register accessor: an alias for `Reg<CH13_EVALTHRES_SPEC>`"]
pub type CH13_EVALTHRES = crate::Reg<ch13_evalthres::CH13_EVALTHRES_SPEC>;
#[doc = "No Description"]
pub mod ch13_evalthres;
#[doc = "CH14_TIMING register accessor: an alias for `Reg<CH14_TIMING_SPEC>`"]
pub type CH14_TIMING = crate::Reg<ch14_timing::CH14_TIMING_SPEC>;
#[doc = "No Description"]
pub mod ch14_timing;
#[doc = "CH14_INTERACT register accessor: an alias for `Reg<CH14_INTERACT_SPEC>`"]
pub type CH14_INTERACT = crate::Reg<ch14_interact::CH14_INTERACT_SPEC>;
#[doc = "No Description"]
pub mod ch14_interact;
#[doc = "CH14_EVALCFG register accessor: an alias for `Reg<CH14_EVALCFG_SPEC>`"]
pub type CH14_EVALCFG = crate::Reg<ch14_evalcfg::CH14_EVALCFG_SPEC>;
#[doc = "No Description"]
pub mod ch14_evalcfg;
#[doc = "CH14_EVALTHRES register accessor: an alias for `Reg<CH14_EVALTHRES_SPEC>`"]
pub type CH14_EVALTHRES = crate::Reg<ch14_evalthres::CH14_EVALTHRES_SPEC>;
#[doc = "No Description"]
pub mod ch14_evalthres;
#[doc = "CH15_TIMING register accessor: an alias for `Reg<CH15_TIMING_SPEC>`"]
pub type CH15_TIMING = crate::Reg<ch15_timing::CH15_TIMING_SPEC>;
#[doc = "No Description"]
pub mod ch15_timing;
#[doc = "CH15_INTERACT register accessor: an alias for `Reg<CH15_INTERACT_SPEC>`"]
pub type CH15_INTERACT = crate::Reg<ch15_interact::CH15_INTERACT_SPEC>;
#[doc = "No Description"]
pub mod ch15_interact;
#[doc = "CH15_EVALCFG register accessor: an alias for `Reg<CH15_EVALCFG_SPEC>`"]
pub type CH15_EVALCFG = crate::Reg<ch15_evalcfg::CH15_EVALCFG_SPEC>;
#[doc = "No Description"]
pub mod ch15_evalcfg;
#[doc = "CH15_EVALTHRES register accessor: an alias for `Reg<CH15_EVALTHRES_SPEC>`"]
pub type CH15_EVALTHRES = crate::Reg<ch15_evalthres::CH15_EVALTHRES_SPEC>;
#[doc = "No Description"]
pub mod ch15_evalthres;
#[doc = "ST0_ARC register accessor: an alias for `Reg<ST0_ARC_SPEC>`"]
pub type ST0_ARC = crate::Reg<st0_arc::ST0_ARC_SPEC>;
#[doc = "No Description"]
pub mod st0_arc;
#[doc = "ST1_ARC register accessor: an alias for `Reg<ST1_ARC_SPEC>`"]
pub type ST1_ARC = crate::Reg<st1_arc::ST1_ARC_SPEC>;
#[doc = "No Description"]
pub mod st1_arc;
#[doc = "ST2_ARC register accessor: an alias for `Reg<ST2_ARC_SPEC>`"]
pub type ST2_ARC = crate::Reg<st2_arc::ST2_ARC_SPEC>;
#[doc = "No Description"]
pub mod st2_arc;
#[doc = "ST3_ARC register accessor: an alias for `Reg<ST3_ARC_SPEC>`"]
pub type ST3_ARC = crate::Reg<st3_arc::ST3_ARC_SPEC>;
#[doc = "No Description"]
pub mod st3_arc;
#[doc = "ST4_ARC register accessor: an alias for `Reg<ST4_ARC_SPEC>`"]
pub type ST4_ARC = crate::Reg<st4_arc::ST4_ARC_SPEC>;
#[doc = "No Description"]
pub mod st4_arc;
#[doc = "ST5_ARC register accessor: an alias for `Reg<ST5_ARC_SPEC>`"]
pub type ST5_ARC = crate::Reg<st5_arc::ST5_ARC_SPEC>;
#[doc = "No Description"]
pub mod st5_arc;
#[doc = "ST6_ARC register accessor: an alias for `Reg<ST6_ARC_SPEC>`"]
pub type ST6_ARC = crate::Reg<st6_arc::ST6_ARC_SPEC>;
#[doc = "No Description"]
pub mod st6_arc;
#[doc = "ST7_ARC register accessor: an alias for `Reg<ST7_ARC_SPEC>`"]
pub type ST7_ARC = crate::Reg<st7_arc::ST7_ARC_SPEC>;
#[doc = "No Description"]
pub mod st7_arc;
#[doc = "ST8_ARC register accessor: an alias for `Reg<ST8_ARC_SPEC>`"]
pub type ST8_ARC = crate::Reg<st8_arc::ST8_ARC_SPEC>;
#[doc = "No Description"]
pub mod st8_arc;
#[doc = "ST9_ARC register accessor: an alias for `Reg<ST9_ARC_SPEC>`"]
pub type ST9_ARC = crate::Reg<st9_arc::ST9_ARC_SPEC>;
#[doc = "No Description"]
pub mod st9_arc;
#[doc = "ST10_ARC register accessor: an alias for `Reg<ST10_ARC_SPEC>`"]
pub type ST10_ARC = crate::Reg<st10_arc::ST10_ARC_SPEC>;
#[doc = "No Description"]
pub mod st10_arc;
#[doc = "ST11_ARC register accessor: an alias for `Reg<ST11_ARC_SPEC>`"]
pub type ST11_ARC = crate::Reg<st11_arc::ST11_ARC_SPEC>;
#[doc = "No Description"]
pub mod st11_arc;
#[doc = "ST12_ARC register accessor: an alias for `Reg<ST12_ARC_SPEC>`"]
pub type ST12_ARC = crate::Reg<st12_arc::ST12_ARC_SPEC>;
#[doc = "No Description"]
pub mod st12_arc;
#[doc = "ST13_ARC register accessor: an alias for `Reg<ST13_ARC_SPEC>`"]
pub type ST13_ARC = crate::Reg<st13_arc::ST13_ARC_SPEC>;
#[doc = "No Description"]
pub mod st13_arc;
#[doc = "ST14_ARC register accessor: an alias for `Reg<ST14_ARC_SPEC>`"]
pub type ST14_ARC = crate::Reg<st14_arc::ST14_ARC_SPEC>;
#[doc = "No Description"]
pub mod st14_arc;
#[doc = "ST15_ARC register accessor: an alias for `Reg<ST15_ARC_SPEC>`"]
pub type ST15_ARC = crate::Reg<st15_arc::ST15_ARC_SPEC>;
#[doc = "No Description"]
pub mod st15_arc;
#[doc = "ST16_ARC register accessor: an alias for `Reg<ST16_ARC_SPEC>`"]
pub type ST16_ARC = crate::Reg<st16_arc::ST16_ARC_SPEC>;
#[doc = "No Description"]
pub mod st16_arc;
#[doc = "ST17_ARC register accessor: an alias for `Reg<ST17_ARC_SPEC>`"]
pub type ST17_ARC = crate::Reg<st17_arc::ST17_ARC_SPEC>;
#[doc = "No Description"]
pub mod st17_arc;
#[doc = "ST18_ARC register accessor: an alias for `Reg<ST18_ARC_SPEC>`"]
pub type ST18_ARC = crate::Reg<st18_arc::ST18_ARC_SPEC>;
#[doc = "No Description"]
pub mod st18_arc;
#[doc = "ST19_ARC register accessor: an alias for `Reg<ST19_ARC_SPEC>`"]
pub type ST19_ARC = crate::Reg<st19_arc::ST19_ARC_SPEC>;
#[doc = "No Description"]
pub mod st19_arc;
#[doc = "ST20_ARC register accessor: an alias for `Reg<ST20_ARC_SPEC>`"]
pub type ST20_ARC = crate::Reg<st20_arc::ST20_ARC_SPEC>;
#[doc = "No Description"]
pub mod st20_arc;
#[doc = "ST21_ARC register accessor: an alias for `Reg<ST21_ARC_SPEC>`"]
pub type ST21_ARC = crate::Reg<st21_arc::ST21_ARC_SPEC>;
#[doc = "No Description"]
pub mod st21_arc;
#[doc = "ST22_ARC register accessor: an alias for `Reg<ST22_ARC_SPEC>`"]
pub type ST22_ARC = crate::Reg<st22_arc::ST22_ARC_SPEC>;
#[doc = "No Description"]
pub mod st22_arc;
#[doc = "ST23_ARC register accessor: an alias for `Reg<ST23_ARC_SPEC>`"]
pub type ST23_ARC = crate::Reg<st23_arc::ST23_ARC_SPEC>;
#[doc = "No Description"]
pub mod st23_arc;
#[doc = "ST24_ARC register accessor: an alias for `Reg<ST24_ARC_SPEC>`"]
pub type ST24_ARC = crate::Reg<st24_arc::ST24_ARC_SPEC>;
#[doc = "No Description"]
pub mod st24_arc;
#[doc = "ST25_ARC register accessor: an alias for `Reg<ST25_ARC_SPEC>`"]
pub type ST25_ARC = crate::Reg<st25_arc::ST25_ARC_SPEC>;
#[doc = "No Description"]
pub mod st25_arc;
#[doc = "ST26_ARC register accessor: an alias for `Reg<ST26_ARC_SPEC>`"]
pub type ST26_ARC = crate::Reg<st26_arc::ST26_ARC_SPEC>;
#[doc = "No Description"]
pub mod st26_arc;
#[doc = "ST27_ARC register accessor: an alias for `Reg<ST27_ARC_SPEC>`"]
pub type ST27_ARC = crate::Reg<st27_arc::ST27_ARC_SPEC>;
#[doc = "No Description"]
pub mod st27_arc;
#[doc = "ST28_ARC register accessor: an alias for `Reg<ST28_ARC_SPEC>`"]
pub type ST28_ARC = crate::Reg<st28_arc::ST28_ARC_SPEC>;
#[doc = "No Description"]
pub mod st28_arc;
#[doc = "ST29_ARC register accessor: an alias for `Reg<ST29_ARC_SPEC>`"]
pub type ST29_ARC = crate::Reg<st29_arc::ST29_ARC_SPEC>;
#[doc = "No Description"]
pub mod st29_arc;
#[doc = "ST30_ARC register accessor: an alias for `Reg<ST30_ARC_SPEC>`"]
pub type ST30_ARC = crate::Reg<st30_arc::ST30_ARC_SPEC>;
#[doc = "No Description"]
pub mod st30_arc;
#[doc = "ST31_ARC register accessor: an alias for `Reg<ST31_ARC_SPEC>`"]
pub type ST31_ARC = crate::Reg<st31_arc::ST31_ARC_SPEC>;
#[doc = "No Description"]
pub mod st31_arc;
#[doc = "ST32_ARC register accessor: an alias for `Reg<ST32_ARC_SPEC>`"]
pub type ST32_ARC = crate::Reg<st32_arc::ST32_ARC_SPEC>;
#[doc = "No Description"]
pub mod st32_arc;
#[doc = "ST33_ARC register accessor: an alias for `Reg<ST33_ARC_SPEC>`"]
pub type ST33_ARC = crate::Reg<st33_arc::ST33_ARC_SPEC>;
#[doc = "No Description"]
pub mod st33_arc;
#[doc = "ST34_ARC register accessor: an alias for `Reg<ST34_ARC_SPEC>`"]
pub type ST34_ARC = crate::Reg<st34_arc::ST34_ARC_SPEC>;
#[doc = "No Description"]
pub mod st34_arc;
#[doc = "ST35_ARC register accessor: an alias for `Reg<ST35_ARC_SPEC>`"]
pub type ST35_ARC = crate::Reg<st35_arc::ST35_ARC_SPEC>;
#[doc = "No Description"]
pub mod st35_arc;
#[doc = "ST36_ARC register accessor: an alias for `Reg<ST36_ARC_SPEC>`"]
pub type ST36_ARC = crate::Reg<st36_arc::ST36_ARC_SPEC>;
#[doc = "No Description"]
pub mod st36_arc;
#[doc = "ST37_ARC register accessor: an alias for `Reg<ST37_ARC_SPEC>`"]
pub type ST37_ARC = crate::Reg<st37_arc::ST37_ARC_SPEC>;
#[doc = "No Description"]
pub mod st37_arc;
#[doc = "ST38_ARC register accessor: an alias for `Reg<ST38_ARC_SPEC>`"]
pub type ST38_ARC = crate::Reg<st38_arc::ST38_ARC_SPEC>;
#[doc = "No Description"]
pub mod st38_arc;
#[doc = "ST39_ARC register accessor: an alias for `Reg<ST39_ARC_SPEC>`"]
pub type ST39_ARC = crate::Reg<st39_arc::ST39_ARC_SPEC>;
#[doc = "No Description"]
pub mod st39_arc;
#[doc = "ST40_ARC register accessor: an alias for `Reg<ST40_ARC_SPEC>`"]
pub type ST40_ARC = crate::Reg<st40_arc::ST40_ARC_SPEC>;
#[doc = "No Description"]
pub mod st40_arc;
#[doc = "ST41_ARC register accessor: an alias for `Reg<ST41_ARC_SPEC>`"]
pub type ST41_ARC = crate::Reg<st41_arc::ST41_ARC_SPEC>;
#[doc = "No Description"]
pub mod st41_arc;
#[doc = "ST42_ARC register accessor: an alias for `Reg<ST42_ARC_SPEC>`"]
pub type ST42_ARC = crate::Reg<st42_arc::ST42_ARC_SPEC>;
#[doc = "No Description"]
pub mod st42_arc;
#[doc = "ST43_ARC register accessor: an alias for `Reg<ST43_ARC_SPEC>`"]
pub type ST43_ARC = crate::Reg<st43_arc::ST43_ARC_SPEC>;
#[doc = "No Description"]
pub mod st43_arc;
#[doc = "ST44_ARC register accessor: an alias for `Reg<ST44_ARC_SPEC>`"]
pub type ST44_ARC = crate::Reg<st44_arc::ST44_ARC_SPEC>;
#[doc = "No Description"]
pub mod st44_arc;
#[doc = "ST45_ARC register accessor: an alias for `Reg<ST45_ARC_SPEC>`"]
pub type ST45_ARC = crate::Reg<st45_arc::ST45_ARC_SPEC>;
#[doc = "No Description"]
pub mod st45_arc;
#[doc = "ST46_ARC register accessor: an alias for `Reg<ST46_ARC_SPEC>`"]
pub type ST46_ARC = crate::Reg<st46_arc::ST46_ARC_SPEC>;
#[doc = "No Description"]
pub mod st46_arc;
#[doc = "ST47_ARC register accessor: an alias for `Reg<ST47_ARC_SPEC>`"]
pub type ST47_ARC = crate::Reg<st47_arc::ST47_ARC_SPEC>;
#[doc = "No Description"]
pub mod st47_arc;
#[doc = "ST48_ARC register accessor: an alias for `Reg<ST48_ARC_SPEC>`"]
pub type ST48_ARC = crate::Reg<st48_arc::ST48_ARC_SPEC>;
#[doc = "No Description"]
pub mod st48_arc;
#[doc = "ST49_ARC register accessor: an alias for `Reg<ST49_ARC_SPEC>`"]
pub type ST49_ARC = crate::Reg<st49_arc::ST49_ARC_SPEC>;
#[doc = "No Description"]
pub mod st49_arc;
#[doc = "ST50_ARC register accessor: an alias for `Reg<ST50_ARC_SPEC>`"]
pub type ST50_ARC = crate::Reg<st50_arc::ST50_ARC_SPEC>;
#[doc = "No Description"]
pub mod st50_arc;
#[doc = "ST51_ARC register accessor: an alias for `Reg<ST51_ARC_SPEC>`"]
pub type ST51_ARC = crate::Reg<st51_arc::ST51_ARC_SPEC>;
#[doc = "No Description"]
pub mod st51_arc;
#[doc = "ST52_ARC register accessor: an alias for `Reg<ST52_ARC_SPEC>`"]
pub type ST52_ARC = crate::Reg<st52_arc::ST52_ARC_SPEC>;
#[doc = "No Description"]
pub mod st52_arc;
#[doc = "ST53_ARC register accessor: an alias for `Reg<ST53_ARC_SPEC>`"]
pub type ST53_ARC = crate::Reg<st53_arc::ST53_ARC_SPEC>;
#[doc = "No Description"]
pub mod st53_arc;
#[doc = "ST54_ARC register accessor: an alias for `Reg<ST54_ARC_SPEC>`"]
pub type ST54_ARC = crate::Reg<st54_arc::ST54_ARC_SPEC>;
#[doc = "No Description"]
pub mod st54_arc;
#[doc = "ST55_ARC register accessor: an alias for `Reg<ST55_ARC_SPEC>`"]
pub type ST55_ARC = crate::Reg<st55_arc::ST55_ARC_SPEC>;
#[doc = "No Description"]
pub mod st55_arc;
#[doc = "ST56_ARC register accessor: an alias for `Reg<ST56_ARC_SPEC>`"]
pub type ST56_ARC = crate::Reg<st56_arc::ST56_ARC_SPEC>;
#[doc = "No Description"]
pub mod st56_arc;
#[doc = "ST57_ARC register accessor: an alias for `Reg<ST57_ARC_SPEC>`"]
pub type ST57_ARC = crate::Reg<st57_arc::ST57_ARC_SPEC>;
#[doc = "No Description"]
pub mod st57_arc;
#[doc = "ST58_ARC register accessor: an alias for `Reg<ST58_ARC_SPEC>`"]
pub type ST58_ARC = crate::Reg<st58_arc::ST58_ARC_SPEC>;
#[doc = "No Description"]
pub mod st58_arc;
#[doc = "ST59_ARC register accessor: an alias for `Reg<ST59_ARC_SPEC>`"]
pub type ST59_ARC = crate::Reg<st59_arc::ST59_ARC_SPEC>;
#[doc = "No Description"]
pub mod st59_arc;
#[doc = "ST60_ARC register accessor: an alias for `Reg<ST60_ARC_SPEC>`"]
pub type ST60_ARC = crate::Reg<st60_arc::ST60_ARC_SPEC>;
#[doc = "No Description"]
pub mod st60_arc;
#[doc = "ST61_ARC register accessor: an alias for `Reg<ST61_ARC_SPEC>`"]
pub type ST61_ARC = crate::Reg<st61_arc::ST61_ARC_SPEC>;
#[doc = "No Description"]
pub mod st61_arc;
#[doc = "ST62_ARC register accessor: an alias for `Reg<ST62_ARC_SPEC>`"]
pub type ST62_ARC = crate::Reg<st62_arc::ST62_ARC_SPEC>;
#[doc = "No Description"]
pub mod st62_arc;
#[doc = "ST63_ARC register accessor: an alias for `Reg<ST63_ARC_SPEC>`"]
pub type ST63_ARC = crate::Reg<st63_arc::ST63_ARC_SPEC>;
#[doc = "No Description"]
pub mod st63_arc;
