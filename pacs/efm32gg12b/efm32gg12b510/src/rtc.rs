#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Counter Value Register"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    #[doc = "0x08 - Interrupt Flag Register"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x0c - Interrupt Flag Set Register"]
    pub ifs: crate::Reg<ifs::IFS_SPEC>,
    #[doc = "0x10 - Interrupt Flag Clear Register"]
    pub ifc: crate::Reg<ifc::IFC_SPEC>,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - Compare Value Register X"]
    pub compa_comp: crate::Reg<compa_comp::COMPA_COMP_SPEC>,
    #[doc = "0x24 - Compare Value Register X"]
    pub compb_comp: crate::Reg<compb_comp::COMPB_COMP_SPEC>,
    #[doc = "0x28 - Compare Value Register X"]
    pub compc_comp: crate::Reg<compc_comp::COMPC_COMP_SPEC>,
    #[doc = "0x2c - Compare Value Register X"]
    pub compd_comp: crate::Reg<compd_comp::COMPD_COMP_SPEC>,
    #[doc = "0x30 - Compare Value Register X"]
    pub compe_comp: crate::Reg<compe_comp::COMPE_COMP_SPEC>,
    #[doc = "0x34 - Compare Value Register X"]
    pub compf_comp: crate::Reg<compf_comp::COMPF_COMP_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Counter Value Register"]
pub mod cnt;
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
#[doc = "COMPA_COMP register accessor: an alias for `Reg<COMPA_COMP_SPEC>`"]
pub type COMPA_COMP = crate::Reg<compa_comp::COMPA_COMP_SPEC>;
#[doc = "Compare Value Register X"]
pub mod compa_comp;
#[doc = "COMPB_COMP register accessor: an alias for `Reg<COMPB_COMP_SPEC>`"]
pub type COMPB_COMP = crate::Reg<compb_comp::COMPB_COMP_SPEC>;
#[doc = "Compare Value Register X"]
pub mod compb_comp;
#[doc = "COMPC_COMP register accessor: an alias for `Reg<COMPC_COMP_SPEC>`"]
pub type COMPC_COMP = crate::Reg<compc_comp::COMPC_COMP_SPEC>;
#[doc = "Compare Value Register X"]
pub mod compc_comp;
#[doc = "COMPD_COMP register accessor: an alias for `Reg<COMPD_COMP_SPEC>`"]
pub type COMPD_COMP = crate::Reg<compd_comp::COMPD_COMP_SPEC>;
#[doc = "Compare Value Register X"]
pub mod compd_comp;
#[doc = "COMPE_COMP register accessor: an alias for `Reg<COMPE_COMP_SPEC>`"]
pub type COMPE_COMP = crate::Reg<compe_comp::COMPE_COMP_SPEC>;
#[doc = "Compare Value Register X"]
pub mod compe_comp;
#[doc = "COMPF_COMP register accessor: an alias for `Reg<COMPF_COMP_SPEC>`"]
pub type COMPF_COMP = crate::Reg<compf_comp::COMPF_COMP_SPEC>;
#[doc = "Compare Value Register X"]
pub mod compf_comp;
