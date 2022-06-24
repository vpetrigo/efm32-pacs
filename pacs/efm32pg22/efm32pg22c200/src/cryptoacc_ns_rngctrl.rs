#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub rngctrl: crate::Reg<rngctrl::RNGCTRL_SPEC>,
    #[doc = "0x04 - Number of 32 bits words of random available in the FIFO. Writing to this register clears the FIFO full interrupt"]
    pub fifolevel: crate::Reg<fifolevel::FIFOLEVEL_SPEC>,
    #[doc = "0x08 - FIFO level at which the rings are restarted when in the FIFOFull_Off state, expressed in number of 128bit blocks"]
    pub fifothresh: crate::Reg<fifothresh::FIFOTHRESH_SPEC>,
    #[doc = "0x0c - Maximum number of 32 bits words that can be stored in the FIFO: 2^g_fifodepth"]
    pub fifodepth: crate::Reg<fifodepth::FIFODEPTH_SPEC>,
    #[doc = "0x10 - This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011..."]
    pub key0: crate::Reg<key0::KEY0_SPEC>,
    #[doc = "0x14 - This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011..."]
    pub key1: crate::Reg<key1::KEY1_SPEC>,
    #[doc = "0x18 - This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011..."]
    pub key2: crate::Reg<key2::KEY2_SPEC>,
    #[doc = "0x1c - This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011..."]
    pub key3: crate::Reg<key3::KEY3_SPEC>,
    #[doc = "0x20 - This register is used to feed known data to the conditioning function or to the continuous tests. See manual"]
    pub testdata: crate::Reg<testdata::TESTDATA_SPEC>,
    _reserved9: [u8; 0x0c],
    #[doc = "0x30 - No Description"]
    pub rngstatus: crate::Reg<rngstatus::RNGSTATUS_SPEC>,
    #[doc = "0x34 - No Description"]
    pub initwaitval: crate::Reg<initwaitval::INITWAITVAL_SPEC>,
    _reserved11: [u8; 0x08],
    #[doc = "0x40 - Number of clk cycles to wait before stopping the rings after the FIFO is full"]
    pub swofftmrval: crate::Reg<swofftmrval::SWOFFTMRVAL_SPEC>,
    #[doc = "0x44 - Sample clock divider. The frequency at which the outputs of the rings are sampled is given by Fs = Fpclk/(ClkDiv + 1)"]
    pub clkdiv: crate::Reg<clkdiv::CLKDIV_SPEC>,
    #[doc = "0x48 - No Description"]
    pub ais31conf0: crate::Reg<ais31conf0::AIS31CONF0_SPEC>,
    #[doc = "0x4c - No Description"]
    pub ais31conf1: crate::Reg<ais31conf1::AIS31CONF1_SPEC>,
    #[doc = "0x50 - No Description"]
    pub ais31conf2: crate::Reg<ais31conf2::AIS31CONF2_SPEC>,
    #[doc = "0x54 - This register is used to obtain diagnostic information about the AIS31 start-up and online tests when g_AIS31=True. Writing to this register clears all fields"]
    pub ais31status: crate::Reg<ais31status::AIS31STATUS_SPEC>,
}
#[doc = "RNGCTRL register accessor: an alias for `Reg<RNGCTRL_SPEC>`"]
pub type RNGCTRL = crate::Reg<rngctrl::RNGCTRL_SPEC>;
#[doc = "No Description"]
pub mod rngctrl;
#[doc = "FIFOLEVEL register accessor: an alias for `Reg<FIFOLEVEL_SPEC>`"]
pub type FIFOLEVEL = crate::Reg<fifolevel::FIFOLEVEL_SPEC>;
#[doc = "Number of 32 bits words of random available in the FIFO. Writing to this register clears the FIFO full interrupt"]
pub mod fifolevel;
#[doc = "FIFOTHRESH register accessor: an alias for `Reg<FIFOTHRESH_SPEC>`"]
pub type FIFOTHRESH = crate::Reg<fifothresh::FIFOTHRESH_SPEC>;
#[doc = "FIFO level at which the rings are restarted when in the FIFOFull_Off state, expressed in number of 128bit blocks"]
pub mod fifothresh;
#[doc = "FIFODEPTH register accessor: an alias for `Reg<FIFODEPTH_SPEC>`"]
pub type FIFODEPTH = crate::Reg<fifodepth::FIFODEPTH_SPEC>;
#[doc = "Maximum number of 32 bits words that can be stored in the FIFO: 2^g_fifodepth"]
pub mod fifodepth;
#[doc = "KEY0 register accessor: an alias for `Reg<KEY0_SPEC>`"]
pub type KEY0 = crate::Reg<key0::KEY0_SPEC>;
#[doc = "This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011..."]
pub mod key0;
#[doc = "KEY1 register accessor: an alias for `Reg<KEY1_SPEC>`"]
pub type KEY1 = crate::Reg<key1::KEY1_SPEC>;
#[doc = "This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011..."]
pub mod key1;
#[doc = "KEY2 register accessor: an alias for `Reg<KEY2_SPEC>`"]
pub type KEY2 = crate::Reg<key2::KEY2_SPEC>;
#[doc = "This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011..."]
pub mod key2;
#[doc = "KEY3 register accessor: an alias for `Reg<KEY3_SPEC>`"]
pub type KEY3 = crate::Reg<key3::KEY3_SPEC>;
#[doc = "This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011..."]
pub mod key3;
#[doc = "TESTDATA register accessor: an alias for `Reg<TESTDATA_SPEC>`"]
pub type TESTDATA = crate::Reg<testdata::TESTDATA_SPEC>;
#[doc = "This register is used to feed known data to the conditioning function or to the continuous tests. See manual"]
pub mod testdata;
#[doc = "RNGSTATUS register accessor: an alias for `Reg<RNGSTATUS_SPEC>`"]
pub type RNGSTATUS = crate::Reg<rngstatus::RNGSTATUS_SPEC>;
#[doc = "No Description"]
pub mod rngstatus;
#[doc = "INITWAITVAL register accessor: an alias for `Reg<INITWAITVAL_SPEC>`"]
pub type INITWAITVAL = crate::Reg<initwaitval::INITWAITVAL_SPEC>;
#[doc = "No Description"]
pub mod initwaitval;
#[doc = "SWOFFTMRVAL register accessor: an alias for `Reg<SWOFFTMRVAL_SPEC>`"]
pub type SWOFFTMRVAL = crate::Reg<swofftmrval::SWOFFTMRVAL_SPEC>;
#[doc = "Number of clk cycles to wait before stopping the rings after the FIFO is full"]
pub mod swofftmrval;
#[doc = "CLKDIV register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "Sample clock divider. The frequency at which the outputs of the rings are sampled is given by Fs = Fpclk/(ClkDiv + 1)"]
pub mod clkdiv;
#[doc = "AIS31CONF0 register accessor: an alias for `Reg<AIS31CONF0_SPEC>`"]
pub type AIS31CONF0 = crate::Reg<ais31conf0::AIS31CONF0_SPEC>;
#[doc = "No Description"]
pub mod ais31conf0;
#[doc = "AIS31CONF1 register accessor: an alias for `Reg<AIS31CONF1_SPEC>`"]
pub type AIS31CONF1 = crate::Reg<ais31conf1::AIS31CONF1_SPEC>;
#[doc = "No Description"]
pub mod ais31conf1;
#[doc = "AIS31CONF2 register accessor: an alias for `Reg<AIS31CONF2_SPEC>`"]
pub type AIS31CONF2 = crate::Reg<ais31conf2::AIS31CONF2_SPEC>;
#[doc = "No Description"]
pub mod ais31conf2;
#[doc = "AIS31STATUS register accessor: an alias for `Reg<AIS31STATUS_SPEC>`"]
pub type AIS31STATUS = crate::Reg<ais31status::AIS31STATUS_SPEC>;
#[doc = "This register is used to obtain diagnostic information about the AIS31 start-up and online tests when g_AIS31=True. Writing to this register clears all fields"]
pub mod ais31status;
