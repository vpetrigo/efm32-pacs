#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub ipversion: crate::Reg<ipversion::IPVERSION_SPEC>,
    #[doc = "0x04 - No Description"]
    pub readctrl: crate::Reg<readctrl::READCTRL_SPEC>,
    #[doc = "0x08 - No Description"]
    pub rdatactrl: crate::Reg<rdatactrl::RDATACTRL_SPEC>,
    #[doc = "0x0c - No Description"]
    pub writectrl: crate::Reg<writectrl::WRITECTRL_SPEC>,
    #[doc = "0x10 - No Description"]
    pub writecmd: crate::Reg<writecmd::WRITECMD_SPEC>,
    #[doc = "0x14 - No Description"]
    pub addrb: crate::Reg<addrb::ADDRB_SPEC>,
    #[doc = "0x18 - No Description"]
    pub wdata: crate::Reg<wdata::WDATA_SPEC>,
    #[doc = "0x1c - No Description"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x20 - No Description"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x24 - No Description"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    _reserved10: [u8; 0x0c],
    #[doc = "0x34 - No Description"]
    pub userdatasize: crate::Reg<userdatasize::USERDATASIZE_SPEC>,
    #[doc = "0x38 - No Description"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x3c - No Description"]
    pub lock: crate::Reg<lock::LOCK_SPEC>,
    #[doc = "0x40 - No Description"]
    pub misclockword: crate::Reg<misclockword::MISCLOCKWORD_SPEC>,
    _reserved14: [u8; 0x0c],
    #[doc = "0x50 - No Description"]
    pub pwrctrl: crate::Reg<pwrctrl::PWRCTRL_SPEC>,
    _reserved15: [u8; 0xcc],
    #[doc = "0x120 - No Description"]
    pub pagelock0: crate::Reg<pagelock0::PAGELOCK0_SPEC>,
    #[doc = "0x124 - No Description"]
    pub pagelock1: crate::Reg<pagelock1::PAGELOCK1_SPEC>,
}
#[doc = "IPVERSION register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "READCTRL register accessor: an alias for `Reg<READCTRL_SPEC>`"]
pub type READCTRL = crate::Reg<readctrl::READCTRL_SPEC>;
#[doc = "No Description"]
pub mod readctrl;
#[doc = "RDATACTRL register accessor: an alias for `Reg<RDATACTRL_SPEC>`"]
pub type RDATACTRL = crate::Reg<rdatactrl::RDATACTRL_SPEC>;
#[doc = "No Description"]
pub mod rdatactrl;
#[doc = "WRITECTRL register accessor: an alias for `Reg<WRITECTRL_SPEC>`"]
pub type WRITECTRL = crate::Reg<writectrl::WRITECTRL_SPEC>;
#[doc = "No Description"]
pub mod writectrl;
#[doc = "WRITECMD register accessor: an alias for `Reg<WRITECMD_SPEC>`"]
pub type WRITECMD = crate::Reg<writecmd::WRITECMD_SPEC>;
#[doc = "No Description"]
pub mod writecmd;
#[doc = "ADDRB register accessor: an alias for `Reg<ADDRB_SPEC>`"]
pub type ADDRB = crate::Reg<addrb::ADDRB_SPEC>;
#[doc = "No Description"]
pub mod addrb;
#[doc = "WDATA register accessor: an alias for `Reg<WDATA_SPEC>`"]
pub type WDATA = crate::Reg<wdata::WDATA_SPEC>;
#[doc = "No Description"]
pub mod wdata;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "No Description"]
pub mod status;
#[doc = "IF register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "No Description"]
pub mod if_;
#[doc = "IEN register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "No Description"]
pub mod ien;
#[doc = "USERDATASIZE register accessor: an alias for `Reg<USERDATASIZE_SPEC>`"]
pub type USERDATASIZE = crate::Reg<userdatasize::USERDATASIZE_SPEC>;
#[doc = "No Description"]
pub mod userdatasize;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "LOCK register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "No Description"]
pub mod lock;
#[doc = "MISCLOCKWORD register accessor: an alias for `Reg<MISCLOCKWORD_SPEC>`"]
pub type MISCLOCKWORD = crate::Reg<misclockword::MISCLOCKWORD_SPEC>;
#[doc = "No Description"]
pub mod misclockword;
#[doc = "PWRCTRL register accessor: an alias for `Reg<PWRCTRL_SPEC>`"]
pub type PWRCTRL = crate::Reg<pwrctrl::PWRCTRL_SPEC>;
#[doc = "No Description"]
pub mod pwrctrl;
#[doc = "PAGELOCK0 register accessor: an alias for `Reg<PAGELOCK0_SPEC>`"]
pub type PAGELOCK0 = crate::Reg<pagelock0::PAGELOCK0_SPEC>;
#[doc = "No Description"]
pub mod pagelock0;
#[doc = "PAGELOCK1 register accessor: an alias for `Reg<PAGELOCK1_SPEC>`"]
pub type PAGELOCK1 = crate::Reg<pagelock1::PAGELOCK1_SPEC>;
#[doc = "No Description"]
pub mod pagelock1;
