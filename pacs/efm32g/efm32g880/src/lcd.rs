#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Display Control Register"]
    pub dispctrl: crate::Reg<dispctrl::DISPCTRL_SPEC>,
    #[doc = "0x08 - Segment Enable Register"]
    pub segen: crate::Reg<segen::SEGEN_SPEC>,
    #[doc = "0x0c - Blink and Animation Control Register"]
    pub bactrl: crate::Reg<bactrl::BACTRL_SPEC>,
    #[doc = "0x10 - Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x14 - Animation Register A"]
    pub arega: crate::Reg<arega::AREGA_SPEC>,
    #[doc = "0x18 - Animation Register B"]
    pub aregb: crate::Reg<aregb::AREGB_SPEC>,
    #[doc = "0x1c - Interrupt Flag Register"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x20 - Interrupt Flag Set Register"]
    pub ifs: crate::Reg<ifs::IFS_SPEC>,
    #[doc = "0x24 - Interrupt Flag Clear Register"]
    pub ifc: crate::Reg<ifc::IFC_SPEC>,
    #[doc = "0x28 - Interrupt Enable Register"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    _reserved11: [u8; 0x14],
    #[doc = "0x40 - Segment Data Low Register 0"]
    pub segd0l: crate::Reg<segd0l::SEGD0L_SPEC>,
    #[doc = "0x44 - Segment Data Low Register 1"]
    pub segd1l: crate::Reg<segd1l::SEGD1L_SPEC>,
    #[doc = "0x48 - Segment Data Low Register 2"]
    pub segd2l: crate::Reg<segd2l::SEGD2L_SPEC>,
    #[doc = "0x4c - Segment Data Low Register 3"]
    pub segd3l: crate::Reg<segd3l::SEGD3L_SPEC>,
    #[doc = "0x50 - Segment Data High Register 0"]
    pub segd0h: crate::Reg<segd0h::SEGD0H_SPEC>,
    #[doc = "0x54 - Segment Data High Register 1"]
    pub segd1h: crate::Reg<segd1h::SEGD1H_SPEC>,
    #[doc = "0x58 - Segment Data High Register 2"]
    pub segd2h: crate::Reg<segd2h::SEGD2H_SPEC>,
    #[doc = "0x5c - Segment Data High Register 3"]
    pub segd3h: crate::Reg<segd3h::SEGD3H_SPEC>,
    #[doc = "0x60 - Freeze Register"]
    pub freeze: crate::Reg<freeze::FREEZE_SPEC>,
    #[doc = "0x64 - Synchronization Busy Register"]
    pub syncbusy: crate::Reg<syncbusy::SYNCBUSY_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "DISPCTRL register accessor: an alias for `Reg<DISPCTRL_SPEC>`"]
pub type DISPCTRL = crate::Reg<dispctrl::DISPCTRL_SPEC>;
#[doc = "Display Control Register"]
pub mod dispctrl;
#[doc = "SEGEN register accessor: an alias for `Reg<SEGEN_SPEC>`"]
pub type SEGEN = crate::Reg<segen::SEGEN_SPEC>;
#[doc = "Segment Enable Register"]
pub mod segen;
#[doc = "BACTRL register accessor: an alias for `Reg<BACTRL_SPEC>`"]
pub type BACTRL = crate::Reg<bactrl::BACTRL_SPEC>;
#[doc = "Blink and Animation Control Register"]
pub mod bactrl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "AREGA register accessor: an alias for `Reg<AREGA_SPEC>`"]
pub type AREGA = crate::Reg<arega::AREGA_SPEC>;
#[doc = "Animation Register A"]
pub mod arega;
#[doc = "AREGB register accessor: an alias for `Reg<AREGB_SPEC>`"]
pub type AREGB = crate::Reg<aregb::AREGB_SPEC>;
#[doc = "Animation Register B"]
pub mod aregb;
#[doc = "IF register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "IFS register accessor: an alias for `Reg<IFS_SPEC>`"]
pub type IFS = crate::Reg<ifs::IFS_SPEC>;
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "IFC register accessor: an alias for `Reg<IFC_SPEC>`"]
pub type IFC = crate::Reg<ifc::IFC_SPEC>;
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "IEN register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "SEGD0L register accessor: an alias for `Reg<SEGD0L_SPEC>`"]
pub type SEGD0L = crate::Reg<segd0l::SEGD0L_SPEC>;
#[doc = "Segment Data Low Register 0"]
pub mod segd0l;
#[doc = "SEGD1L register accessor: an alias for `Reg<SEGD1L_SPEC>`"]
pub type SEGD1L = crate::Reg<segd1l::SEGD1L_SPEC>;
#[doc = "Segment Data Low Register 1"]
pub mod segd1l;
#[doc = "SEGD2L register accessor: an alias for `Reg<SEGD2L_SPEC>`"]
pub type SEGD2L = crate::Reg<segd2l::SEGD2L_SPEC>;
#[doc = "Segment Data Low Register 2"]
pub mod segd2l;
#[doc = "SEGD3L register accessor: an alias for `Reg<SEGD3L_SPEC>`"]
pub type SEGD3L = crate::Reg<segd3l::SEGD3L_SPEC>;
#[doc = "Segment Data Low Register 3"]
pub mod segd3l;
#[doc = "SEGD0H register accessor: an alias for `Reg<SEGD0H_SPEC>`"]
pub type SEGD0H = crate::Reg<segd0h::SEGD0H_SPEC>;
#[doc = "Segment Data High Register 0"]
pub mod segd0h;
#[doc = "SEGD1H register accessor: an alias for `Reg<SEGD1H_SPEC>`"]
pub type SEGD1H = crate::Reg<segd1h::SEGD1H_SPEC>;
#[doc = "Segment Data High Register 1"]
pub mod segd1h;
#[doc = "SEGD2H register accessor: an alias for `Reg<SEGD2H_SPEC>`"]
pub type SEGD2H = crate::Reg<segd2h::SEGD2H_SPEC>;
#[doc = "Segment Data High Register 2"]
pub mod segd2h;
#[doc = "SEGD3H register accessor: an alias for `Reg<SEGD3H_SPEC>`"]
pub type SEGD3H = crate::Reg<segd3h::SEGD3H_SPEC>;
#[doc = "Segment Data High Register 3"]
pub mod segd3h;
#[doc = "FREEZE register accessor: an alias for `Reg<FREEZE_SPEC>`"]
pub type FREEZE = crate::Reg<freeze::FREEZE_SPEC>;
#[doc = "Freeze Register"]
pub mod freeze;
#[doc = "SYNCBUSY register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
