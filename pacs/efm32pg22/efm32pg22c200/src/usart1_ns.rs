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
    pub frame: crate::Reg<frame::FRAME_SPEC>,
    #[doc = "0x10 - No Description"]
    pub trigctrl: crate::Reg<trigctrl::TRIGCTRL_SPEC>,
    #[doc = "0x14 - No Description"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x18 - No Description"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x1c - No Description"]
    pub clkdiv: crate::Reg<clkdiv::CLKDIV_SPEC>,
    #[doc = "0x20 - No Description"]
    pub rxdatax: crate::Reg<rxdatax::RXDATAX_SPEC>,
    #[doc = "0x24 - No Description"]
    pub rxdata: crate::Reg<rxdata::RXDATA_SPEC>,
    #[doc = "0x28 - No Description"]
    pub rxdoublex: crate::Reg<rxdoublex::RXDOUBLEX_SPEC>,
    #[doc = "0x2c - No Description"]
    pub rxdouble: crate::Reg<rxdouble::RXDOUBLE_SPEC>,
    #[doc = "0x30 - No Description"]
    pub rxdataxp: crate::Reg<rxdataxp::RXDATAXP_SPEC>,
    #[doc = "0x34 - No Description"]
    pub rxdoublexp: crate::Reg<rxdoublexp::RXDOUBLEXP_SPEC>,
    #[doc = "0x38 - No Description"]
    pub txdatax: crate::Reg<txdatax::TXDATAX_SPEC>,
    #[doc = "0x3c - No Description"]
    pub txdata: crate::Reg<txdata::TXDATA_SPEC>,
    #[doc = "0x40 - No Description"]
    pub txdoublex: crate::Reg<txdoublex::TXDOUBLEX_SPEC>,
    #[doc = "0x44 - No Description"]
    pub txdouble: crate::Reg<txdouble::TXDOUBLE_SPEC>,
    #[doc = "0x48 - No Description"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x4c - No Description"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    #[doc = "0x50 - No Description"]
    pub irctrl: crate::Reg<irctrl::IRCTRL_SPEC>,
    #[doc = "0x54 - No Description"]
    pub i2sctrl: crate::Reg<i2sctrl::I2SCTRL_SPEC>,
    #[doc = "0x58 - No Description"]
    pub timing: crate::Reg<timing::TIMING_SPEC>,
    #[doc = "0x5c - No Description"]
    pub ctrlx: crate::Reg<ctrlx::CTRLX_SPEC>,
    #[doc = "0x60 - No Description"]
    pub timecmp0: crate::Reg<timecmp0::TIMECMP0_SPEC>,
    #[doc = "0x64 - No Description"]
    pub timecmp1: crate::Reg<timecmp1::TIMECMP1_SPEC>,
    #[doc = "0x68 - No Description"]
    pub timecmp2: crate::Reg<timecmp2::TIMECMP2_SPEC>,
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
#[doc = "FRAME register accessor: an alias for `Reg<FRAME_SPEC>`"]
pub type FRAME = crate::Reg<frame::FRAME_SPEC>;
#[doc = "No Description"]
pub mod frame;
#[doc = "TRIGCTRL register accessor: an alias for `Reg<TRIGCTRL_SPEC>`"]
pub type TRIGCTRL = crate::Reg<trigctrl::TRIGCTRL_SPEC>;
#[doc = "No Description"]
pub mod trigctrl;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "No Description"]
pub mod status;
#[doc = "CLKDIV register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "No Description"]
pub mod clkdiv;
#[doc = "RXDATAX register accessor: an alias for `Reg<RXDATAX_SPEC>`"]
pub type RXDATAX = crate::Reg<rxdatax::RXDATAX_SPEC>;
#[doc = "No Description"]
pub mod rxdatax;
#[doc = "RXDATA register accessor: an alias for `Reg<RXDATA_SPEC>`"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "No Description"]
pub mod rxdata;
#[doc = "RXDOUBLEX register accessor: an alias for `Reg<RXDOUBLEX_SPEC>`"]
pub type RXDOUBLEX = crate::Reg<rxdoublex::RXDOUBLEX_SPEC>;
#[doc = "No Description"]
pub mod rxdoublex;
#[doc = "RXDOUBLE register accessor: an alias for `Reg<RXDOUBLE_SPEC>`"]
pub type RXDOUBLE = crate::Reg<rxdouble::RXDOUBLE_SPEC>;
#[doc = "No Description"]
pub mod rxdouble;
#[doc = "RXDATAXP register accessor: an alias for `Reg<RXDATAXP_SPEC>`"]
pub type RXDATAXP = crate::Reg<rxdataxp::RXDATAXP_SPEC>;
#[doc = "No Description"]
pub mod rxdataxp;
#[doc = "RXDOUBLEXP register accessor: an alias for `Reg<RXDOUBLEXP_SPEC>`"]
pub type RXDOUBLEXP = crate::Reg<rxdoublexp::RXDOUBLEXP_SPEC>;
#[doc = "No Description"]
pub mod rxdoublexp;
#[doc = "TXDATAX register accessor: an alias for `Reg<TXDATAX_SPEC>`"]
pub type TXDATAX = crate::Reg<txdatax::TXDATAX_SPEC>;
#[doc = "No Description"]
pub mod txdatax;
#[doc = "TXDATA register accessor: an alias for `Reg<TXDATA_SPEC>`"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "No Description"]
pub mod txdata;
#[doc = "TXDOUBLEX register accessor: an alias for `Reg<TXDOUBLEX_SPEC>`"]
pub type TXDOUBLEX = crate::Reg<txdoublex::TXDOUBLEX_SPEC>;
#[doc = "No Description"]
pub mod txdoublex;
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
#[doc = "IRCTRL register accessor: an alias for `Reg<IRCTRL_SPEC>`"]
pub type IRCTRL = crate::Reg<irctrl::IRCTRL_SPEC>;
#[doc = "No Description"]
pub mod irctrl;
#[doc = "I2SCTRL register accessor: an alias for `Reg<I2SCTRL_SPEC>`"]
pub type I2SCTRL = crate::Reg<i2sctrl::I2SCTRL_SPEC>;
#[doc = "No Description"]
pub mod i2sctrl;
#[doc = "TIMING register accessor: an alias for `Reg<TIMING_SPEC>`"]
pub type TIMING = crate::Reg<timing::TIMING_SPEC>;
#[doc = "No Description"]
pub mod timing;
#[doc = "CTRLX register accessor: an alias for `Reg<CTRLX_SPEC>`"]
pub type CTRLX = crate::Reg<ctrlx::CTRLX_SPEC>;
#[doc = "No Description"]
pub mod ctrlx;
#[doc = "TIMECMP0 register accessor: an alias for `Reg<TIMECMP0_SPEC>`"]
pub type TIMECMP0 = crate::Reg<timecmp0::TIMECMP0_SPEC>;
#[doc = "No Description"]
pub mod timecmp0;
#[doc = "TIMECMP1 register accessor: an alias for `Reg<TIMECMP1_SPEC>`"]
pub type TIMECMP1 = crate::Reg<timecmp1::TIMECMP1_SPEC>;
#[doc = "No Description"]
pub mod timecmp1;
#[doc = "TIMECMP2 register accessor: an alias for `Reg<TIMECMP2_SPEC>`"]
pub type TIMECMP2 = crate::Reg<timecmp2::TIMECMP2_SPEC>;
#[doc = "No Description"]
pub mod timecmp2;
