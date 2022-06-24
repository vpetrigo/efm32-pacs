#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Fetcher: Start address of data block. In direct mode, this register is written by the software. In scatter-gather mode, this register is updated after each processed descriptor."]
    pub fetchaddr: crate::Reg<fetchaddr::FETCHADDR_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Fetcher: Length of data block. In direct mode, this register is written by the software. In scatter-gather mode, this register is not used."]
    pub fetchlen: crate::Reg<fetchlen::FETCHLEN_SPEC>,
    #[doc = "0x0c - Fetcher: User tag. In direct mode, this register is written by the software. In scatter-gather mode, this register is not used."]
    pub fetchtag: crate::Reg<fetchtag::FETCHTAG_SPEC>,
    #[doc = "0x10 - Pusher: Start address of data block (LSB). In direct mode, this register is written by the software. In scatter-gather mode, this register is updated after each processed descriptor."]
    pub pushaddr: crate::Reg<pushaddr::PUSHADDR_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x18 - Pusher: Length of data block. In direct mode, this register is written by the software. In scatter-gather mode, this register is not used."]
    pub pushlen: crate::Reg<pushlen::PUSHLEN_SPEC>,
    #[doc = "0x1c - Interrupt enable"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x28 - Interrupt flag register"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x30 - Writing a '1' clears the interrupt status. Writing a '0' has no effect."]
    pub if_clr: crate::Reg<if_clr::IF_CLR_SPEC>,
    #[doc = "0x34 - Control register, called CONFIG in Barco datasheet."]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x38 - Command register for starting the fetcher and pusher"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x3c - Status register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    _reserved11: [u8; 0x03c0],
    #[doc = "0x400 - No Description"]
    pub incl_ips_hw_cfg: crate::Reg<incl_ips_hw_cfg::INCL_IPS_HW_CFG_SPEC>,
    #[doc = "0x404 - No Description"]
    pub ba411e_hw_cfg_1: crate::Reg<ba411e_hw_cfg_1::BA411E_HW_CFG_1_SPEC>,
    #[doc = "0x408 - No Description"]
    pub ba411e_hw_cfg_2: crate::Reg<ba411e_hw_cfg_2::BA411E_HW_CFG_2_SPEC>,
    #[doc = "0x40c - No Description"]
    pub ba413_hw_cfg: crate::Reg<ba413_hw_cfg::BA413_HW_CFG_SPEC>,
    #[doc = "0x410 - No Description"]
    pub ba418_hw_cfg: crate::Reg<ba418_hw_cfg::BA418_HW_CFG_SPEC>,
    #[doc = "0x414 - No Description"]
    pub ba419_hw_cfg: crate::Reg<ba419_hw_cfg::BA419_HW_CFG_SPEC>,
}
#[doc = "FETCHADDR register accessor: an alias for `Reg<FETCHADDR_SPEC>`"]
pub type FETCHADDR = crate::Reg<fetchaddr::FETCHADDR_SPEC>;
#[doc = "Fetcher: Start address of data block. In direct mode, this register is written by the software. In scatter-gather mode, this register is updated after each processed descriptor."]
pub mod fetchaddr;
#[doc = "FETCHLEN register accessor: an alias for `Reg<FETCHLEN_SPEC>`"]
pub type FETCHLEN = crate::Reg<fetchlen::FETCHLEN_SPEC>;
#[doc = "Fetcher: Length of data block. In direct mode, this register is written by the software. In scatter-gather mode, this register is not used."]
pub mod fetchlen;
#[doc = "FETCHTAG register accessor: an alias for `Reg<FETCHTAG_SPEC>`"]
pub type FETCHTAG = crate::Reg<fetchtag::FETCHTAG_SPEC>;
#[doc = "Fetcher: User tag. In direct mode, this register is written by the software. In scatter-gather mode, this register is not used."]
pub mod fetchtag;
#[doc = "PUSHADDR register accessor: an alias for `Reg<PUSHADDR_SPEC>`"]
pub type PUSHADDR = crate::Reg<pushaddr::PUSHADDR_SPEC>;
#[doc = "Pusher: Start address of data block (LSB). In direct mode, this register is written by the software. In scatter-gather mode, this register is updated after each processed descriptor."]
pub mod pushaddr;
#[doc = "PUSHLEN register accessor: an alias for `Reg<PUSHLEN_SPEC>`"]
pub type PUSHLEN = crate::Reg<pushlen::PUSHLEN_SPEC>;
#[doc = "Pusher: Length of data block. In direct mode, this register is written by the software. In scatter-gather mode, this register is not used."]
pub mod pushlen;
#[doc = "IEN register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt enable"]
pub mod ien;
#[doc = "IF register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt flag register"]
pub mod if_;
#[doc = "IF_CLR register accessor: an alias for `Reg<IF_CLR_SPEC>`"]
pub type IF_CLR = crate::Reg<if_clr::IF_CLR_SPEC>;
#[doc = "Writing a '1' clears the interrupt status. Writing a '0' has no effect."]
pub mod if_clr;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register, called CONFIG in Barco datasheet."]
pub mod ctrl;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command register for starting the fetcher and pusher"]
pub mod cmd;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status register"]
pub mod status;
#[doc = "INCL_IPS_HW_CFG register accessor: an alias for `Reg<INCL_IPS_HW_CFG_SPEC>`"]
pub type INCL_IPS_HW_CFG = crate::Reg<incl_ips_hw_cfg::INCL_IPS_HW_CFG_SPEC>;
#[doc = "No Description"]
pub mod incl_ips_hw_cfg;
#[doc = "BA411E_HW_CFG_1 register accessor: an alias for `Reg<BA411E_HW_CFG_1_SPEC>`"]
pub type BA411E_HW_CFG_1 = crate::Reg<ba411e_hw_cfg_1::BA411E_HW_CFG_1_SPEC>;
#[doc = "No Description"]
pub mod ba411e_hw_cfg_1;
#[doc = "BA411E_HW_CFG_2 register accessor: an alias for `Reg<BA411E_HW_CFG_2_SPEC>`"]
pub type BA411E_HW_CFG_2 = crate::Reg<ba411e_hw_cfg_2::BA411E_HW_CFG_2_SPEC>;
#[doc = "No Description"]
pub mod ba411e_hw_cfg_2;
#[doc = "BA413_HW_CFG register accessor: an alias for `Reg<BA413_HW_CFG_SPEC>`"]
pub type BA413_HW_CFG = crate::Reg<ba413_hw_cfg::BA413_HW_CFG_SPEC>;
#[doc = "No Description"]
pub mod ba413_hw_cfg;
#[doc = "BA418_HW_CFG register accessor: an alias for `Reg<BA418_HW_CFG_SPEC>`"]
pub type BA418_HW_CFG = crate::Reg<ba418_hw_cfg::BA418_HW_CFG_SPEC>;
#[doc = "No Description"]
pub mod ba418_hw_cfg;
#[doc = "BA419_HW_CFG register accessor: an alias for `Reg<BA419_HW_CFG_SPEC>`"]
pub type BA419_HW_CFG = crate::Reg<ba419_hw_cfg::BA419_HW_CFG_SPEC>;
#[doc = "No Description"]
pub mod ba419_hw_cfg;
