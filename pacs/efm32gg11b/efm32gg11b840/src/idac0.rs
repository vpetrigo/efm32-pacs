#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Current Programming Register"]
    pub curprog: crate::Reg<curprog::CURPROG_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Duty Cycle Configuration Register"]
    pub dutyconfig: crate::Reg<dutyconfig::DUTYCONFIG_SPEC>,
    _reserved3: [u8; 0x08],
    #[doc = "0x18 - Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x20 - Interrupt Flag Register"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x24 - Interrupt Flag Set Register"]
    pub ifs: crate::Reg<ifs::IFS_SPEC>,
    #[doc = "0x28 - Interrupt Flag Clear Register"]
    pub ifc: crate::Reg<ifc::IFC_SPEC>,
    #[doc = "0x2c - Interrupt Enable Register"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x34 - APORT Request Status Register"]
    pub aportreq: crate::Reg<aportreq::APORTREQ_SPEC>,
    #[doc = "0x38 - APORT Request Status Register"]
    pub aportconflict: crate::Reg<aportconflict::APORTCONFLICT_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CURPROG register accessor: an alias for `Reg<CURPROG_SPEC>`"]
pub type CURPROG = crate::Reg<curprog::CURPROG_SPEC>;
#[doc = "Current Programming Register"]
pub mod curprog;
#[doc = "DUTYCONFIG register accessor: an alias for `Reg<DUTYCONFIG_SPEC>`"]
pub type DUTYCONFIG = crate::Reg<dutyconfig::DUTYCONFIG_SPEC>;
#[doc = "Duty Cycle Configuration Register"]
pub mod dutyconfig;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
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
#[doc = "APORTREQ register accessor: an alias for `Reg<APORTREQ_SPEC>`"]
pub type APORTREQ = crate::Reg<aportreq::APORTREQ_SPEC>;
#[doc = "APORT Request Status Register"]
pub mod aportreq;
#[doc = "APORTCONFLICT register accessor: an alias for `Reg<APORTCONFLICT_SPEC>`"]
pub type APORTCONFLICT = crate::Reg<aportconflict::APORTCONFLICT_SPEC>;
#[doc = "APORT Request Status Register"]
pub mod aportconflict;
