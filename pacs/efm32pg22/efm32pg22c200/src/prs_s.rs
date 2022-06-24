#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub ipversion: crate::Reg<ipversion::IPVERSION_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - No Description"]
    pub async_swpulse: crate::Reg<async_swpulse::ASYNC_SWPULSE_SPEC>,
    #[doc = "0x0c - No Description"]
    pub async_swlevel: crate::Reg<async_swlevel::ASYNC_SWLEVEL_SPEC>,
    #[doc = "0x10 - No Description"]
    pub async_peek: crate::Reg<async_peek::ASYNC_PEEK_SPEC>,
    #[doc = "0x14 - No Description"]
    pub sync_peek: crate::Reg<sync_peek::SYNC_PEEK_SPEC>,
    #[doc = "0x18 - No Description"]
    pub async_ch0_ctrl: crate::Reg<async_ch0_ctrl::ASYNC_CH0_CTRL_SPEC>,
    #[doc = "0x1c - No Description"]
    pub async_ch1_ctrl: crate::Reg<async_ch1_ctrl::ASYNC_CH1_CTRL_SPEC>,
    #[doc = "0x20 - No Description"]
    pub async_ch2_ctrl: crate::Reg<async_ch2_ctrl::ASYNC_CH2_CTRL_SPEC>,
    #[doc = "0x24 - No Description"]
    pub async_ch3_ctrl: crate::Reg<async_ch3_ctrl::ASYNC_CH3_CTRL_SPEC>,
    #[doc = "0x28 - No Description"]
    pub async_ch4_ctrl: crate::Reg<async_ch4_ctrl::ASYNC_CH4_CTRL_SPEC>,
    #[doc = "0x2c - No Description"]
    pub async_ch5_ctrl: crate::Reg<async_ch5_ctrl::ASYNC_CH5_CTRL_SPEC>,
    #[doc = "0x30 - No Description"]
    pub async_ch6_ctrl: crate::Reg<async_ch6_ctrl::ASYNC_CH6_CTRL_SPEC>,
    #[doc = "0x34 - No Description"]
    pub async_ch7_ctrl: crate::Reg<async_ch7_ctrl::ASYNC_CH7_CTRL_SPEC>,
    #[doc = "0x38 - No Description"]
    pub async_ch8_ctrl: crate::Reg<async_ch8_ctrl::ASYNC_CH8_CTRL_SPEC>,
    #[doc = "0x3c - No Description"]
    pub async_ch9_ctrl: crate::Reg<async_ch9_ctrl::ASYNC_CH9_CTRL_SPEC>,
    #[doc = "0x40 - No Description"]
    pub async_ch10_ctrl: crate::Reg<async_ch10_ctrl::ASYNC_CH10_CTRL_SPEC>,
    #[doc = "0x44 - No Description"]
    pub async_ch11_ctrl: crate::Reg<async_ch11_ctrl::ASYNC_CH11_CTRL_SPEC>,
    #[doc = "0x48 - No Description"]
    pub sync_ch0_ctrl: crate::Reg<sync_ch0_ctrl::SYNC_CH0_CTRL_SPEC>,
    #[doc = "0x4c - No Description"]
    pub sync_ch1_ctrl: crate::Reg<sync_ch1_ctrl::SYNC_CH1_CTRL_SPEC>,
    #[doc = "0x50 - No Description"]
    pub sync_ch2_ctrl: crate::Reg<sync_ch2_ctrl::SYNC_CH2_CTRL_SPEC>,
    #[doc = "0x54 - No Description"]
    pub sync_ch3_ctrl: crate::Reg<sync_ch3_ctrl::SYNC_CH3_CTRL_SPEC>,
    #[doc = "0x58 - CALDN Consumer Register"]
    pub consumer_cmu_caldn: crate::Reg<consumer_cmu_caldn::CONSUMER_CMU_CALDN_SPEC>,
    #[doc = "0x5c - CALUP Consumer Register"]
    pub consumer_cmu_calup: crate::Reg<consumer_cmu_calup::CONSUMER_CMU_CALUP_SPEC>,
    _reserved23: [u8; 0x04],
    #[doc = "0x64 - SCAN Consumer Register"]
    pub consumer_iadc0_scantrigger:
        crate::Reg<consumer_iadc0_scantrigger::CONSUMER_IADC0_SCANTRIGGER_SPEC>,
    #[doc = "0x68 - SINGLE Consumer Register"]
    pub consumer_iadc0_singletrigger:
        crate::Reg<consumer_iadc0_singletrigger::CONSUMER_IADC0_SINGLETRIGGER_SPEC>,
    #[doc = "0x6c - DMAREQ0 Consumer Register"]
    pub consumer_ldmaxbar_dmareq0:
        crate::Reg<consumer_ldmaxbar_dmareq0::CONSUMER_LDMAXBAR_DMAREQ0_SPEC>,
    #[doc = "0x70 - DMAREQ1 Consumer Register"]
    pub consumer_ldmaxbar_dmareq1:
        crate::Reg<consumer_ldmaxbar_dmareq1::CONSUMER_LDMAXBAR_DMAREQ1_SPEC>,
    #[doc = "0x74 - CLEAR Consumer Register"]
    pub consumer_letimer0_clear: crate::Reg<consumer_letimer0_clear::CONSUMER_LETIMER0_CLEAR_SPEC>,
    #[doc = "0x78 - START Consumer Register"]
    pub consumer_letimer0_start: crate::Reg<consumer_letimer0_start::CONSUMER_LETIMER0_START_SPEC>,
    #[doc = "0x7c - STOP Consumer Register"]
    pub consumer_letimer0_stop: crate::Reg<consumer_letimer0_stop::CONSUMER_LETIMER0_STOP_SPEC>,
    #[doc = "0x80 - RX Consumer Register"]
    pub consumer_euart0_rx: crate::Reg<consumer_euart0_rx::CONSUMER_EUART0_RX_SPEC>,
    #[doc = "0x84 - TRIGGER Consumer Register"]
    pub consumer_euart0_trigger: crate::Reg<consumer_euart0_trigger::CONSUMER_EUART0_TRIGGER_SPEC>,
    _reserved32: [u8; 0x60],
    #[doc = "0xe8 - CC0 Consumer Register"]
    pub consumer_rtcc_cc0: crate::Reg<consumer_rtcc_cc0::CONSUMER_RTCC_CC0_SPEC>,
    #[doc = "0xec - CC1 Consumer Register"]
    pub consumer_rtcc_cc1: crate::Reg<consumer_rtcc_cc1::CONSUMER_RTCC_CC1_SPEC>,
    #[doc = "0xf0 - CC2 Consumer Register"]
    pub consumer_rtcc_cc2: crate::Reg<consumer_rtcc_cc2::CONSUMER_RTCC_CC2_SPEC>,
    _reserved35: [u8; 0x04],
    #[doc = "0xf8 - CTI Consumer Register"]
    pub consumer_core_ctiin0: crate::Reg<consumer_core_ctiin0::CONSUMER_CORE_CTIIN0_SPEC>,
    #[doc = "0xfc - CTI Consumer Register"]
    pub consumer_core_ctiin1: crate::Reg<consumer_core_ctiin1::CONSUMER_CORE_CTIIN1_SPEC>,
    #[doc = "0x100 - CTI Consumer Register"]
    pub consumer_core_ctiin2: crate::Reg<consumer_core_ctiin2::CONSUMER_CORE_CTIIN2_SPEC>,
    #[doc = "0x104 - CTI Consumer Register"]
    pub consumer_core_ctiin3: crate::Reg<consumer_core_ctiin3::CONSUMER_CORE_CTIIN3_SPEC>,
    #[doc = "0x108 - M33 Consumer Register"]
    pub consumer_core_m33rxev: crate::Reg<consumer_core_m33rxev::CONSUMER_CORE_M33RXEV_SPEC>,
    #[doc = "0x10c - CC0 Consumer Register"]
    pub consumer_timer0_cc0: crate::Reg<consumer_timer0_cc0::CONSUMER_TIMER0_CC0_SPEC>,
    #[doc = "0x110 - CC1 Consumer Register"]
    pub consumer_timer0_cc1: crate::Reg<consumer_timer0_cc1::CONSUMER_TIMER0_CC1_SPEC>,
    #[doc = "0x114 - CC2 Consumer Register"]
    pub consumer_timer0_cc2: crate::Reg<consumer_timer0_cc2::CONSUMER_TIMER0_CC2_SPEC>,
    #[doc = "0x118 - DTI Consumer Register"]
    pub consumer_timer0_dti: crate::Reg<consumer_timer0_dti::CONSUMER_TIMER0_DTI_SPEC>,
    #[doc = "0x11c - DTI Consumer Register"]
    pub consumer_timer0_dtifs1: crate::Reg<consumer_timer0_dtifs1::CONSUMER_TIMER0_DTIFS1_SPEC>,
    #[doc = "0x120 - DTI Consumer Register"]
    pub consumer_timer0_dtifs2: crate::Reg<consumer_timer0_dtifs2::CONSUMER_TIMER0_DTIFS2_SPEC>,
    #[doc = "0x124 - CC0 Consumer Register"]
    pub consumer_timer1_cc0: crate::Reg<consumer_timer1_cc0::CONSUMER_TIMER1_CC0_SPEC>,
    #[doc = "0x128 - CC1 Consumer Register"]
    pub consumer_timer1_cc1: crate::Reg<consumer_timer1_cc1::CONSUMER_TIMER1_CC1_SPEC>,
    #[doc = "0x12c - CC2 Consumer Register"]
    pub consumer_timer1_cc2: crate::Reg<consumer_timer1_cc2::CONSUMER_TIMER1_CC2_SPEC>,
    #[doc = "0x130 - DTI Consumer Register"]
    pub consumer_timer1_dti: crate::Reg<consumer_timer1_dti::CONSUMER_TIMER1_DTI_SPEC>,
    #[doc = "0x134 - DTI Consumer Register"]
    pub consumer_timer1_dtifs1: crate::Reg<consumer_timer1_dtifs1::CONSUMER_TIMER1_DTIFS1_SPEC>,
    #[doc = "0x138 - DTI Consumer Register"]
    pub consumer_timer1_dtifs2: crate::Reg<consumer_timer1_dtifs2::CONSUMER_TIMER1_DTIFS2_SPEC>,
    #[doc = "0x13c - CC0 Consumer Register"]
    pub consumer_timer2_cc0: crate::Reg<consumer_timer2_cc0::CONSUMER_TIMER2_CC0_SPEC>,
    #[doc = "0x140 - CC1 Consumer Register"]
    pub consumer_timer2_cc1: crate::Reg<consumer_timer2_cc1::CONSUMER_TIMER2_CC1_SPEC>,
    #[doc = "0x144 - CC2 Consumer Register"]
    pub consumer_timer2_cc2: crate::Reg<consumer_timer2_cc2::CONSUMER_TIMER2_CC2_SPEC>,
    #[doc = "0x148 - DTI Consumer Register"]
    pub consumer_timer2_dti: crate::Reg<consumer_timer2_dti::CONSUMER_TIMER2_DTI_SPEC>,
    #[doc = "0x14c - DTI Consumer Register"]
    pub consumer_timer2_dtifs1: crate::Reg<consumer_timer2_dtifs1::CONSUMER_TIMER2_DTIFS1_SPEC>,
    #[doc = "0x150 - DTI Consumer Register"]
    pub consumer_timer2_dtifs2: crate::Reg<consumer_timer2_dtifs2::CONSUMER_TIMER2_DTIFS2_SPEC>,
    #[doc = "0x154 - CC0 Consumer Register"]
    pub consumer_timer3_cc0: crate::Reg<consumer_timer3_cc0::CONSUMER_TIMER3_CC0_SPEC>,
    #[doc = "0x158 - CC1 Consumer Register"]
    pub consumer_timer3_cc1: crate::Reg<consumer_timer3_cc1::CONSUMER_TIMER3_CC1_SPEC>,
    #[doc = "0x15c - CC2 Consumer Register"]
    pub consumer_timer3_cc2: crate::Reg<consumer_timer3_cc2::CONSUMER_TIMER3_CC2_SPEC>,
    #[doc = "0x160 - DTI Consumer Register"]
    pub consumer_timer3_dti: crate::Reg<consumer_timer3_dti::CONSUMER_TIMER3_DTI_SPEC>,
    #[doc = "0x164 - DTI Consumer Register"]
    pub consumer_timer3_dtifs1: crate::Reg<consumer_timer3_dtifs1::CONSUMER_TIMER3_DTIFS1_SPEC>,
    #[doc = "0x168 - DTI Consumer Register"]
    pub consumer_timer3_dtifs2: crate::Reg<consumer_timer3_dtifs2::CONSUMER_TIMER3_DTIFS2_SPEC>,
    #[doc = "0x16c - CC0 Consumer Register"]
    pub consumer_timer4_cc0: crate::Reg<consumer_timer4_cc0::CONSUMER_TIMER4_CC0_SPEC>,
    #[doc = "0x170 - CC1 Consumer Register"]
    pub consumer_timer4_cc1: crate::Reg<consumer_timer4_cc1::CONSUMER_TIMER4_CC1_SPEC>,
    #[doc = "0x174 - CC2 Consumer Register"]
    pub consumer_timer4_cc2: crate::Reg<consumer_timer4_cc2::CONSUMER_TIMER4_CC2_SPEC>,
    #[doc = "0x178 - DTI Consumer Register"]
    pub consumer_timer4_dti: crate::Reg<consumer_timer4_dti::CONSUMER_TIMER4_DTI_SPEC>,
    #[doc = "0x17c - DTI Consumer Register"]
    pub consumer_timer4_dtifs1: crate::Reg<consumer_timer4_dtifs1::CONSUMER_TIMER4_DTIFS1_SPEC>,
    #[doc = "0x180 - DTI Consumer Register"]
    pub consumer_timer4_dtifs2: crate::Reg<consumer_timer4_dtifs2::CONSUMER_TIMER4_DTIFS2_SPEC>,
    #[doc = "0x184 - CLK Consumer Register"]
    pub consumer_usart0_clk: crate::Reg<consumer_usart0_clk::CONSUMER_USART0_CLK_SPEC>,
    #[doc = "0x188 - IR Consumer Register"]
    pub consumer_usart0_ir: crate::Reg<consumer_usart0_ir::CONSUMER_USART0_IR_SPEC>,
    #[doc = "0x18c - RX Consumer Register"]
    pub consumer_usart0_rx: crate::Reg<consumer_usart0_rx::CONSUMER_USART0_RX_SPEC>,
    #[doc = "0x190 - TRIGGER Consumer Register"]
    pub consumer_usart0_trigger: crate::Reg<consumer_usart0_trigger::CONSUMER_USART0_TRIGGER_SPEC>,
    #[doc = "0x194 - CLK Consumer Register"]
    pub consumer_usart1_clk: crate::Reg<consumer_usart1_clk::CONSUMER_USART1_CLK_SPEC>,
    #[doc = "0x198 - IR Consumer Register"]
    pub consumer_usart1_ir: crate::Reg<consumer_usart1_ir::CONSUMER_USART1_IR_SPEC>,
    #[doc = "0x19c - RX Consumer Register"]
    pub consumer_usart1_rx: crate::Reg<consumer_usart1_rx::CONSUMER_USART1_RX_SPEC>,
    #[doc = "0x1a0 - TRIGGER Consumer Register"]
    pub consumer_usart1_trigger: crate::Reg<consumer_usart1_trigger::CONSUMER_USART1_TRIGGER_SPEC>,
    #[doc = "0x1a4 - SRC0 Consumer Register"]
    pub consumer_wdog0_src0: crate::Reg<consumer_wdog0_src0::CONSUMER_WDOG0_SRC0_SPEC>,
    #[doc = "0x1a8 - SRC1 Consumer Register"]
    pub consumer_wdog0_src1: crate::Reg<consumer_wdog0_src1::CONSUMER_WDOG0_SRC1_SPEC>,
}
#[doc = "IPVERSION register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "ASYNC_SWPULSE register accessor: an alias for `Reg<ASYNC_SWPULSE_SPEC>`"]
pub type ASYNC_SWPULSE = crate::Reg<async_swpulse::ASYNC_SWPULSE_SPEC>;
#[doc = "No Description"]
pub mod async_swpulse;
#[doc = "ASYNC_SWLEVEL register accessor: an alias for `Reg<ASYNC_SWLEVEL_SPEC>`"]
pub type ASYNC_SWLEVEL = crate::Reg<async_swlevel::ASYNC_SWLEVEL_SPEC>;
#[doc = "No Description"]
pub mod async_swlevel;
#[doc = "ASYNC_PEEK register accessor: an alias for `Reg<ASYNC_PEEK_SPEC>`"]
pub type ASYNC_PEEK = crate::Reg<async_peek::ASYNC_PEEK_SPEC>;
#[doc = "No Description"]
pub mod async_peek;
#[doc = "SYNC_PEEK register accessor: an alias for `Reg<SYNC_PEEK_SPEC>`"]
pub type SYNC_PEEK = crate::Reg<sync_peek::SYNC_PEEK_SPEC>;
#[doc = "No Description"]
pub mod sync_peek;
#[doc = "ASYNC_CH0_CTRL register accessor: an alias for `Reg<ASYNC_CH0_CTRL_SPEC>`"]
pub type ASYNC_CH0_CTRL = crate::Reg<async_ch0_ctrl::ASYNC_CH0_CTRL_SPEC>;
#[doc = "No Description"]
pub mod async_ch0_ctrl;
#[doc = "ASYNC_CH1_CTRL register accessor: an alias for `Reg<ASYNC_CH1_CTRL_SPEC>`"]
pub type ASYNC_CH1_CTRL = crate::Reg<async_ch1_ctrl::ASYNC_CH1_CTRL_SPEC>;
#[doc = "No Description"]
pub mod async_ch1_ctrl;
#[doc = "ASYNC_CH2_CTRL register accessor: an alias for `Reg<ASYNC_CH2_CTRL_SPEC>`"]
pub type ASYNC_CH2_CTRL = crate::Reg<async_ch2_ctrl::ASYNC_CH2_CTRL_SPEC>;
#[doc = "No Description"]
pub mod async_ch2_ctrl;
#[doc = "ASYNC_CH3_CTRL register accessor: an alias for `Reg<ASYNC_CH3_CTRL_SPEC>`"]
pub type ASYNC_CH3_CTRL = crate::Reg<async_ch3_ctrl::ASYNC_CH3_CTRL_SPEC>;
#[doc = "No Description"]
pub mod async_ch3_ctrl;
#[doc = "ASYNC_CH4_CTRL register accessor: an alias for `Reg<ASYNC_CH4_CTRL_SPEC>`"]
pub type ASYNC_CH4_CTRL = crate::Reg<async_ch4_ctrl::ASYNC_CH4_CTRL_SPEC>;
#[doc = "No Description"]
pub mod async_ch4_ctrl;
#[doc = "ASYNC_CH5_CTRL register accessor: an alias for `Reg<ASYNC_CH5_CTRL_SPEC>`"]
pub type ASYNC_CH5_CTRL = crate::Reg<async_ch5_ctrl::ASYNC_CH5_CTRL_SPEC>;
#[doc = "No Description"]
pub mod async_ch5_ctrl;
#[doc = "ASYNC_CH6_CTRL register accessor: an alias for `Reg<ASYNC_CH6_CTRL_SPEC>`"]
pub type ASYNC_CH6_CTRL = crate::Reg<async_ch6_ctrl::ASYNC_CH6_CTRL_SPEC>;
#[doc = "No Description"]
pub mod async_ch6_ctrl;
#[doc = "ASYNC_CH7_CTRL register accessor: an alias for `Reg<ASYNC_CH7_CTRL_SPEC>`"]
pub type ASYNC_CH7_CTRL = crate::Reg<async_ch7_ctrl::ASYNC_CH7_CTRL_SPEC>;
#[doc = "No Description"]
pub mod async_ch7_ctrl;
#[doc = "ASYNC_CH8_CTRL register accessor: an alias for `Reg<ASYNC_CH8_CTRL_SPEC>`"]
pub type ASYNC_CH8_CTRL = crate::Reg<async_ch8_ctrl::ASYNC_CH8_CTRL_SPEC>;
#[doc = "No Description"]
pub mod async_ch8_ctrl;
#[doc = "ASYNC_CH9_CTRL register accessor: an alias for `Reg<ASYNC_CH9_CTRL_SPEC>`"]
pub type ASYNC_CH9_CTRL = crate::Reg<async_ch9_ctrl::ASYNC_CH9_CTRL_SPEC>;
#[doc = "No Description"]
pub mod async_ch9_ctrl;
#[doc = "ASYNC_CH10_CTRL register accessor: an alias for `Reg<ASYNC_CH10_CTRL_SPEC>`"]
pub type ASYNC_CH10_CTRL = crate::Reg<async_ch10_ctrl::ASYNC_CH10_CTRL_SPEC>;
#[doc = "No Description"]
pub mod async_ch10_ctrl;
#[doc = "ASYNC_CH11_CTRL register accessor: an alias for `Reg<ASYNC_CH11_CTRL_SPEC>`"]
pub type ASYNC_CH11_CTRL = crate::Reg<async_ch11_ctrl::ASYNC_CH11_CTRL_SPEC>;
#[doc = "No Description"]
pub mod async_ch11_ctrl;
#[doc = "SYNC_CH0_CTRL register accessor: an alias for `Reg<SYNC_CH0_CTRL_SPEC>`"]
pub type SYNC_CH0_CTRL = crate::Reg<sync_ch0_ctrl::SYNC_CH0_CTRL_SPEC>;
#[doc = "No Description"]
pub mod sync_ch0_ctrl;
#[doc = "SYNC_CH1_CTRL register accessor: an alias for `Reg<SYNC_CH1_CTRL_SPEC>`"]
pub type SYNC_CH1_CTRL = crate::Reg<sync_ch1_ctrl::SYNC_CH1_CTRL_SPEC>;
#[doc = "No Description"]
pub mod sync_ch1_ctrl;
#[doc = "SYNC_CH2_CTRL register accessor: an alias for `Reg<SYNC_CH2_CTRL_SPEC>`"]
pub type SYNC_CH2_CTRL = crate::Reg<sync_ch2_ctrl::SYNC_CH2_CTRL_SPEC>;
#[doc = "No Description"]
pub mod sync_ch2_ctrl;
#[doc = "SYNC_CH3_CTRL register accessor: an alias for `Reg<SYNC_CH3_CTRL_SPEC>`"]
pub type SYNC_CH3_CTRL = crate::Reg<sync_ch3_ctrl::SYNC_CH3_CTRL_SPEC>;
#[doc = "No Description"]
pub mod sync_ch3_ctrl;
#[doc = "CONSUMER_CMU_CALDN register accessor: an alias for `Reg<CONSUMER_CMU_CALDN_SPEC>`"]
pub type CONSUMER_CMU_CALDN = crate::Reg<consumer_cmu_caldn::CONSUMER_CMU_CALDN_SPEC>;
#[doc = "CALDN Consumer Register"]
pub mod consumer_cmu_caldn;
#[doc = "CONSUMER_CMU_CALUP register accessor: an alias for `Reg<CONSUMER_CMU_CALUP_SPEC>`"]
pub type CONSUMER_CMU_CALUP = crate::Reg<consumer_cmu_calup::CONSUMER_CMU_CALUP_SPEC>;
#[doc = "CALUP Consumer Register"]
pub mod consumer_cmu_calup;
#[doc = "CONSUMER_IADC0_SCANTRIGGER register accessor: an alias for `Reg<CONSUMER_IADC0_SCANTRIGGER_SPEC>`"]
pub type CONSUMER_IADC0_SCANTRIGGER =
    crate::Reg<consumer_iadc0_scantrigger::CONSUMER_IADC0_SCANTRIGGER_SPEC>;
#[doc = "SCAN Consumer Register"]
pub mod consumer_iadc0_scantrigger;
#[doc = "CONSUMER_IADC0_SINGLETRIGGER register accessor: an alias for `Reg<CONSUMER_IADC0_SINGLETRIGGER_SPEC>`"]
pub type CONSUMER_IADC0_SINGLETRIGGER =
    crate::Reg<consumer_iadc0_singletrigger::CONSUMER_IADC0_SINGLETRIGGER_SPEC>;
#[doc = "SINGLE Consumer Register"]
pub mod consumer_iadc0_singletrigger;
#[doc = "CONSUMER_LDMAXBAR_DMAREQ0 register accessor: an alias for `Reg<CONSUMER_LDMAXBAR_DMAREQ0_SPEC>`"]
pub type CONSUMER_LDMAXBAR_DMAREQ0 =
    crate::Reg<consumer_ldmaxbar_dmareq0::CONSUMER_LDMAXBAR_DMAREQ0_SPEC>;
#[doc = "DMAREQ0 Consumer Register"]
pub mod consumer_ldmaxbar_dmareq0;
#[doc = "CONSUMER_LDMAXBAR_DMAREQ1 register accessor: an alias for `Reg<CONSUMER_LDMAXBAR_DMAREQ1_SPEC>`"]
pub type CONSUMER_LDMAXBAR_DMAREQ1 =
    crate::Reg<consumer_ldmaxbar_dmareq1::CONSUMER_LDMAXBAR_DMAREQ1_SPEC>;
#[doc = "DMAREQ1 Consumer Register"]
pub mod consumer_ldmaxbar_dmareq1;
#[doc = "CONSUMER_LETIMER0_CLEAR register accessor: an alias for `Reg<CONSUMER_LETIMER0_CLEAR_SPEC>`"]
pub type CONSUMER_LETIMER0_CLEAR =
    crate::Reg<consumer_letimer0_clear::CONSUMER_LETIMER0_CLEAR_SPEC>;
#[doc = "CLEAR Consumer Register"]
pub mod consumer_letimer0_clear;
#[doc = "CONSUMER_LETIMER0_START register accessor: an alias for `Reg<CONSUMER_LETIMER0_START_SPEC>`"]
pub type CONSUMER_LETIMER0_START =
    crate::Reg<consumer_letimer0_start::CONSUMER_LETIMER0_START_SPEC>;
#[doc = "START Consumer Register"]
pub mod consumer_letimer0_start;
#[doc = "CONSUMER_LETIMER0_STOP register accessor: an alias for `Reg<CONSUMER_LETIMER0_STOP_SPEC>`"]
pub type CONSUMER_LETIMER0_STOP = crate::Reg<consumer_letimer0_stop::CONSUMER_LETIMER0_STOP_SPEC>;
#[doc = "STOP Consumer Register"]
pub mod consumer_letimer0_stop;
#[doc = "CONSUMER_EUART0_RX register accessor: an alias for `Reg<CONSUMER_EUART0_RX_SPEC>`"]
pub type CONSUMER_EUART0_RX = crate::Reg<consumer_euart0_rx::CONSUMER_EUART0_RX_SPEC>;
#[doc = "RX Consumer Register"]
pub mod consumer_euart0_rx;
#[doc = "CONSUMER_EUART0_TRIGGER register accessor: an alias for `Reg<CONSUMER_EUART0_TRIGGER_SPEC>`"]
pub type CONSUMER_EUART0_TRIGGER =
    crate::Reg<consumer_euart0_trigger::CONSUMER_EUART0_TRIGGER_SPEC>;
#[doc = "TRIGGER Consumer Register"]
pub mod consumer_euart0_trigger;
#[doc = "CONSUMER_RTCC_CC0 register accessor: an alias for `Reg<CONSUMER_RTCC_CC0_SPEC>`"]
pub type CONSUMER_RTCC_CC0 = crate::Reg<consumer_rtcc_cc0::CONSUMER_RTCC_CC0_SPEC>;
#[doc = "CC0 Consumer Register"]
pub mod consumer_rtcc_cc0;
#[doc = "CONSUMER_RTCC_CC1 register accessor: an alias for `Reg<CONSUMER_RTCC_CC1_SPEC>`"]
pub type CONSUMER_RTCC_CC1 = crate::Reg<consumer_rtcc_cc1::CONSUMER_RTCC_CC1_SPEC>;
#[doc = "CC1 Consumer Register"]
pub mod consumer_rtcc_cc1;
#[doc = "CONSUMER_RTCC_CC2 register accessor: an alias for `Reg<CONSUMER_RTCC_CC2_SPEC>`"]
pub type CONSUMER_RTCC_CC2 = crate::Reg<consumer_rtcc_cc2::CONSUMER_RTCC_CC2_SPEC>;
#[doc = "CC2 Consumer Register"]
pub mod consumer_rtcc_cc2;
#[doc = "CONSUMER_CORE_CTIIN0 register accessor: an alias for `Reg<CONSUMER_CORE_CTIIN0_SPEC>`"]
pub type CONSUMER_CORE_CTIIN0 = crate::Reg<consumer_core_ctiin0::CONSUMER_CORE_CTIIN0_SPEC>;
#[doc = "CTI Consumer Register"]
pub mod consumer_core_ctiin0;
#[doc = "CONSUMER_CORE_CTIIN1 register accessor: an alias for `Reg<CONSUMER_CORE_CTIIN1_SPEC>`"]
pub type CONSUMER_CORE_CTIIN1 = crate::Reg<consumer_core_ctiin1::CONSUMER_CORE_CTIIN1_SPEC>;
#[doc = "CTI Consumer Register"]
pub mod consumer_core_ctiin1;
#[doc = "CONSUMER_CORE_CTIIN2 register accessor: an alias for `Reg<CONSUMER_CORE_CTIIN2_SPEC>`"]
pub type CONSUMER_CORE_CTIIN2 = crate::Reg<consumer_core_ctiin2::CONSUMER_CORE_CTIIN2_SPEC>;
#[doc = "CTI Consumer Register"]
pub mod consumer_core_ctiin2;
#[doc = "CONSUMER_CORE_CTIIN3 register accessor: an alias for `Reg<CONSUMER_CORE_CTIIN3_SPEC>`"]
pub type CONSUMER_CORE_CTIIN3 = crate::Reg<consumer_core_ctiin3::CONSUMER_CORE_CTIIN3_SPEC>;
#[doc = "CTI Consumer Register"]
pub mod consumer_core_ctiin3;
#[doc = "CONSUMER_CORE_M33RXEV register accessor: an alias for `Reg<CONSUMER_CORE_M33RXEV_SPEC>`"]
pub type CONSUMER_CORE_M33RXEV = crate::Reg<consumer_core_m33rxev::CONSUMER_CORE_M33RXEV_SPEC>;
#[doc = "M33 Consumer Register"]
pub mod consumer_core_m33rxev;
#[doc = "CONSUMER_TIMER0_CC0 register accessor: an alias for `Reg<CONSUMER_TIMER0_CC0_SPEC>`"]
pub type CONSUMER_TIMER0_CC0 = crate::Reg<consumer_timer0_cc0::CONSUMER_TIMER0_CC0_SPEC>;
#[doc = "CC0 Consumer Register"]
pub mod consumer_timer0_cc0;
#[doc = "CONSUMER_TIMER0_CC1 register accessor: an alias for `Reg<CONSUMER_TIMER0_CC1_SPEC>`"]
pub type CONSUMER_TIMER0_CC1 = crate::Reg<consumer_timer0_cc1::CONSUMER_TIMER0_CC1_SPEC>;
#[doc = "CC1 Consumer Register"]
pub mod consumer_timer0_cc1;
#[doc = "CONSUMER_TIMER0_CC2 register accessor: an alias for `Reg<CONSUMER_TIMER0_CC2_SPEC>`"]
pub type CONSUMER_TIMER0_CC2 = crate::Reg<consumer_timer0_cc2::CONSUMER_TIMER0_CC2_SPEC>;
#[doc = "CC2 Consumer Register"]
pub mod consumer_timer0_cc2;
#[doc = "CONSUMER_TIMER0_DTI register accessor: an alias for `Reg<CONSUMER_TIMER0_DTI_SPEC>`"]
pub type CONSUMER_TIMER0_DTI = crate::Reg<consumer_timer0_dti::CONSUMER_TIMER0_DTI_SPEC>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer0_dti;
#[doc = "CONSUMER_TIMER0_DTIFS1 register accessor: an alias for `Reg<CONSUMER_TIMER0_DTIFS1_SPEC>`"]
pub type CONSUMER_TIMER0_DTIFS1 = crate::Reg<consumer_timer0_dtifs1::CONSUMER_TIMER0_DTIFS1_SPEC>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer0_dtifs1;
#[doc = "CONSUMER_TIMER0_DTIFS2 register accessor: an alias for `Reg<CONSUMER_TIMER0_DTIFS2_SPEC>`"]
pub type CONSUMER_TIMER0_DTIFS2 = crate::Reg<consumer_timer0_dtifs2::CONSUMER_TIMER0_DTIFS2_SPEC>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer0_dtifs2;
#[doc = "CONSUMER_TIMER1_CC0 register accessor: an alias for `Reg<CONSUMER_TIMER1_CC0_SPEC>`"]
pub type CONSUMER_TIMER1_CC0 = crate::Reg<consumer_timer1_cc0::CONSUMER_TIMER1_CC0_SPEC>;
#[doc = "CC0 Consumer Register"]
pub mod consumer_timer1_cc0;
#[doc = "CONSUMER_TIMER1_CC1 register accessor: an alias for `Reg<CONSUMER_TIMER1_CC1_SPEC>`"]
pub type CONSUMER_TIMER1_CC1 = crate::Reg<consumer_timer1_cc1::CONSUMER_TIMER1_CC1_SPEC>;
#[doc = "CC1 Consumer Register"]
pub mod consumer_timer1_cc1;
#[doc = "CONSUMER_TIMER1_CC2 register accessor: an alias for `Reg<CONSUMER_TIMER1_CC2_SPEC>`"]
pub type CONSUMER_TIMER1_CC2 = crate::Reg<consumer_timer1_cc2::CONSUMER_TIMER1_CC2_SPEC>;
#[doc = "CC2 Consumer Register"]
pub mod consumer_timer1_cc2;
#[doc = "CONSUMER_TIMER1_DTI register accessor: an alias for `Reg<CONSUMER_TIMER1_DTI_SPEC>`"]
pub type CONSUMER_TIMER1_DTI = crate::Reg<consumer_timer1_dti::CONSUMER_TIMER1_DTI_SPEC>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer1_dti;
#[doc = "CONSUMER_TIMER1_DTIFS1 register accessor: an alias for `Reg<CONSUMER_TIMER1_DTIFS1_SPEC>`"]
pub type CONSUMER_TIMER1_DTIFS1 = crate::Reg<consumer_timer1_dtifs1::CONSUMER_TIMER1_DTIFS1_SPEC>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer1_dtifs1;
#[doc = "CONSUMER_TIMER1_DTIFS2 register accessor: an alias for `Reg<CONSUMER_TIMER1_DTIFS2_SPEC>`"]
pub type CONSUMER_TIMER1_DTIFS2 = crate::Reg<consumer_timer1_dtifs2::CONSUMER_TIMER1_DTIFS2_SPEC>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer1_dtifs2;
#[doc = "CONSUMER_TIMER2_CC0 register accessor: an alias for `Reg<CONSUMER_TIMER2_CC0_SPEC>`"]
pub type CONSUMER_TIMER2_CC0 = crate::Reg<consumer_timer2_cc0::CONSUMER_TIMER2_CC0_SPEC>;
#[doc = "CC0 Consumer Register"]
pub mod consumer_timer2_cc0;
#[doc = "CONSUMER_TIMER2_CC1 register accessor: an alias for `Reg<CONSUMER_TIMER2_CC1_SPEC>`"]
pub type CONSUMER_TIMER2_CC1 = crate::Reg<consumer_timer2_cc1::CONSUMER_TIMER2_CC1_SPEC>;
#[doc = "CC1 Consumer Register"]
pub mod consumer_timer2_cc1;
#[doc = "CONSUMER_TIMER2_CC2 register accessor: an alias for `Reg<CONSUMER_TIMER2_CC2_SPEC>`"]
pub type CONSUMER_TIMER2_CC2 = crate::Reg<consumer_timer2_cc2::CONSUMER_TIMER2_CC2_SPEC>;
#[doc = "CC2 Consumer Register"]
pub mod consumer_timer2_cc2;
#[doc = "CONSUMER_TIMER2_DTI register accessor: an alias for `Reg<CONSUMER_TIMER2_DTI_SPEC>`"]
pub type CONSUMER_TIMER2_DTI = crate::Reg<consumer_timer2_dti::CONSUMER_TIMER2_DTI_SPEC>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer2_dti;
#[doc = "CONSUMER_TIMER2_DTIFS1 register accessor: an alias for `Reg<CONSUMER_TIMER2_DTIFS1_SPEC>`"]
pub type CONSUMER_TIMER2_DTIFS1 = crate::Reg<consumer_timer2_dtifs1::CONSUMER_TIMER2_DTIFS1_SPEC>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer2_dtifs1;
#[doc = "CONSUMER_TIMER2_DTIFS2 register accessor: an alias for `Reg<CONSUMER_TIMER2_DTIFS2_SPEC>`"]
pub type CONSUMER_TIMER2_DTIFS2 = crate::Reg<consumer_timer2_dtifs2::CONSUMER_TIMER2_DTIFS2_SPEC>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer2_dtifs2;
#[doc = "CONSUMER_TIMER3_CC0 register accessor: an alias for `Reg<CONSUMER_TIMER3_CC0_SPEC>`"]
pub type CONSUMER_TIMER3_CC0 = crate::Reg<consumer_timer3_cc0::CONSUMER_TIMER3_CC0_SPEC>;
#[doc = "CC0 Consumer Register"]
pub mod consumer_timer3_cc0;
#[doc = "CONSUMER_TIMER3_CC1 register accessor: an alias for `Reg<CONSUMER_TIMER3_CC1_SPEC>`"]
pub type CONSUMER_TIMER3_CC1 = crate::Reg<consumer_timer3_cc1::CONSUMER_TIMER3_CC1_SPEC>;
#[doc = "CC1 Consumer Register"]
pub mod consumer_timer3_cc1;
#[doc = "CONSUMER_TIMER3_CC2 register accessor: an alias for `Reg<CONSUMER_TIMER3_CC2_SPEC>`"]
pub type CONSUMER_TIMER3_CC2 = crate::Reg<consumer_timer3_cc2::CONSUMER_TIMER3_CC2_SPEC>;
#[doc = "CC2 Consumer Register"]
pub mod consumer_timer3_cc2;
#[doc = "CONSUMER_TIMER3_DTI register accessor: an alias for `Reg<CONSUMER_TIMER3_DTI_SPEC>`"]
pub type CONSUMER_TIMER3_DTI = crate::Reg<consumer_timer3_dti::CONSUMER_TIMER3_DTI_SPEC>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer3_dti;
#[doc = "CONSUMER_TIMER3_DTIFS1 register accessor: an alias for `Reg<CONSUMER_TIMER3_DTIFS1_SPEC>`"]
pub type CONSUMER_TIMER3_DTIFS1 = crate::Reg<consumer_timer3_dtifs1::CONSUMER_TIMER3_DTIFS1_SPEC>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer3_dtifs1;
#[doc = "CONSUMER_TIMER3_DTIFS2 register accessor: an alias for `Reg<CONSUMER_TIMER3_DTIFS2_SPEC>`"]
pub type CONSUMER_TIMER3_DTIFS2 = crate::Reg<consumer_timer3_dtifs2::CONSUMER_TIMER3_DTIFS2_SPEC>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer3_dtifs2;
#[doc = "CONSUMER_TIMER4_CC0 register accessor: an alias for `Reg<CONSUMER_TIMER4_CC0_SPEC>`"]
pub type CONSUMER_TIMER4_CC0 = crate::Reg<consumer_timer4_cc0::CONSUMER_TIMER4_CC0_SPEC>;
#[doc = "CC0 Consumer Register"]
pub mod consumer_timer4_cc0;
#[doc = "CONSUMER_TIMER4_CC1 register accessor: an alias for `Reg<CONSUMER_TIMER4_CC1_SPEC>`"]
pub type CONSUMER_TIMER4_CC1 = crate::Reg<consumer_timer4_cc1::CONSUMER_TIMER4_CC1_SPEC>;
#[doc = "CC1 Consumer Register"]
pub mod consumer_timer4_cc1;
#[doc = "CONSUMER_TIMER4_CC2 register accessor: an alias for `Reg<CONSUMER_TIMER4_CC2_SPEC>`"]
pub type CONSUMER_TIMER4_CC2 = crate::Reg<consumer_timer4_cc2::CONSUMER_TIMER4_CC2_SPEC>;
#[doc = "CC2 Consumer Register"]
pub mod consumer_timer4_cc2;
#[doc = "CONSUMER_TIMER4_DTI register accessor: an alias for `Reg<CONSUMER_TIMER4_DTI_SPEC>`"]
pub type CONSUMER_TIMER4_DTI = crate::Reg<consumer_timer4_dti::CONSUMER_TIMER4_DTI_SPEC>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer4_dti;
#[doc = "CONSUMER_TIMER4_DTIFS1 register accessor: an alias for `Reg<CONSUMER_TIMER4_DTIFS1_SPEC>`"]
pub type CONSUMER_TIMER4_DTIFS1 = crate::Reg<consumer_timer4_dtifs1::CONSUMER_TIMER4_DTIFS1_SPEC>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer4_dtifs1;
#[doc = "CONSUMER_TIMER4_DTIFS2 register accessor: an alias for `Reg<CONSUMER_TIMER4_DTIFS2_SPEC>`"]
pub type CONSUMER_TIMER4_DTIFS2 = crate::Reg<consumer_timer4_dtifs2::CONSUMER_TIMER4_DTIFS2_SPEC>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer4_dtifs2;
#[doc = "CONSUMER_USART0_CLK register accessor: an alias for `Reg<CONSUMER_USART0_CLK_SPEC>`"]
pub type CONSUMER_USART0_CLK = crate::Reg<consumer_usart0_clk::CONSUMER_USART0_CLK_SPEC>;
#[doc = "CLK Consumer Register"]
pub mod consumer_usart0_clk;
#[doc = "CONSUMER_USART0_IR register accessor: an alias for `Reg<CONSUMER_USART0_IR_SPEC>`"]
pub type CONSUMER_USART0_IR = crate::Reg<consumer_usart0_ir::CONSUMER_USART0_IR_SPEC>;
#[doc = "IR Consumer Register"]
pub mod consumer_usart0_ir;
#[doc = "CONSUMER_USART0_RX register accessor: an alias for `Reg<CONSUMER_USART0_RX_SPEC>`"]
pub type CONSUMER_USART0_RX = crate::Reg<consumer_usart0_rx::CONSUMER_USART0_RX_SPEC>;
#[doc = "RX Consumer Register"]
pub mod consumer_usart0_rx;
#[doc = "CONSUMER_USART0_TRIGGER register accessor: an alias for `Reg<CONSUMER_USART0_TRIGGER_SPEC>`"]
pub type CONSUMER_USART0_TRIGGER =
    crate::Reg<consumer_usart0_trigger::CONSUMER_USART0_TRIGGER_SPEC>;
#[doc = "TRIGGER Consumer Register"]
pub mod consumer_usart0_trigger;
#[doc = "CONSUMER_USART1_CLK register accessor: an alias for `Reg<CONSUMER_USART1_CLK_SPEC>`"]
pub type CONSUMER_USART1_CLK = crate::Reg<consumer_usart1_clk::CONSUMER_USART1_CLK_SPEC>;
#[doc = "CLK Consumer Register"]
pub mod consumer_usart1_clk;
#[doc = "CONSUMER_USART1_IR register accessor: an alias for `Reg<CONSUMER_USART1_IR_SPEC>`"]
pub type CONSUMER_USART1_IR = crate::Reg<consumer_usart1_ir::CONSUMER_USART1_IR_SPEC>;
#[doc = "IR Consumer Register"]
pub mod consumer_usart1_ir;
#[doc = "CONSUMER_USART1_RX register accessor: an alias for `Reg<CONSUMER_USART1_RX_SPEC>`"]
pub type CONSUMER_USART1_RX = crate::Reg<consumer_usart1_rx::CONSUMER_USART1_RX_SPEC>;
#[doc = "RX Consumer Register"]
pub mod consumer_usart1_rx;
#[doc = "CONSUMER_USART1_TRIGGER register accessor: an alias for `Reg<CONSUMER_USART1_TRIGGER_SPEC>`"]
pub type CONSUMER_USART1_TRIGGER =
    crate::Reg<consumer_usart1_trigger::CONSUMER_USART1_TRIGGER_SPEC>;
#[doc = "TRIGGER Consumer Register"]
pub mod consumer_usart1_trigger;
#[doc = "CONSUMER_WDOG0_SRC0 register accessor: an alias for `Reg<CONSUMER_WDOG0_SRC0_SPEC>`"]
pub type CONSUMER_WDOG0_SRC0 = crate::Reg<consumer_wdog0_src0::CONSUMER_WDOG0_SRC0_SPEC>;
#[doc = "SRC0 Consumer Register"]
pub mod consumer_wdog0_src0;
#[doc = "CONSUMER_WDOG0_SRC1 register accessor: an alias for `Reg<CONSUMER_WDOG0_SRC1_SPEC>`"]
pub type CONSUMER_WDOG0_SRC1 = crate::Reg<consumer_wdog0_src1::CONSUMER_WDOG0_SRC1_SPEC>;
#[doc = "SRC1 Consumer Register"]
pub mod consumer_wdog0_src1;
