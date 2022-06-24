#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub ipversion: crate::Reg<ipversion::IPVERSION_SPEC>,
    #[doc = "0x04 - No Description"]
    pub en: crate::Reg<en::EN_SPEC>,
    #[doc = "0x08 - No Description"]
    pub swrst: crate::Reg<swrst::SWRST_SPEC>,
    #[doc = "0x0c - No Description"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x10 - No Description"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x14 - No Description"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x18 - No Description"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x1c - No Description"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x20 - No Description"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    #[doc = "0x24 - No Description"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    #[doc = "0x28 - No Description"]
    pub auxcnt: crate::Reg<auxcnt::AUXCNT_SPEC>,
    #[doc = "0x2c - No Description"]
    pub top: crate::Reg<top::TOP_SPEC>,
    #[doc = "0x30 - No Description"]
    pub topb: crate::Reg<topb::TOPB_SPEC>,
    #[doc = "0x34 - No Description"]
    pub ovsctrl: crate::Reg<ovsctrl::OVSCTRL_SPEC>,
    #[doc = "0x38 - No Description"]
    pub syncbusy: crate::Reg<syncbusy::SYNCBUSY_SPEC>,
    #[doc = "0x3c - No Description"]
    pub lock: crate::Reg<lock::LOCK_SPEC>,
}
#[doc = "IPVERSION register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "EN register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "No Description"]
pub mod en;
#[doc = "SWRST register accessor: an alias for `Reg<SWRST_SPEC>`"]
pub type SWRST = crate::Reg<swrst::SWRST_SPEC>;
#[doc = "No Description"]
pub mod swrst;
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "No Description"]
pub mod cfg;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "No Description"]
pub mod ctrl;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "No Description"]
pub mod cmd;
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
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "No Description"]
pub mod cnt;
#[doc = "AUXCNT register accessor: an alias for `Reg<AUXCNT_SPEC>`"]
pub type AUXCNT = crate::Reg<auxcnt::AUXCNT_SPEC>;
#[doc = "No Description"]
pub mod auxcnt;
#[doc = "TOP register accessor: an alias for `Reg<TOP_SPEC>`"]
pub type TOP = crate::Reg<top::TOP_SPEC>;
#[doc = "No Description"]
pub mod top;
#[doc = "TOPB register accessor: an alias for `Reg<TOPB_SPEC>`"]
pub type TOPB = crate::Reg<topb::TOPB_SPEC>;
#[doc = "No Description"]
pub mod topb;
#[doc = "OVSCTRL register accessor: an alias for `Reg<OVSCTRL_SPEC>`"]
pub type OVSCTRL = crate::Reg<ovsctrl::OVSCTRL_SPEC>;
#[doc = "No Description"]
pub mod ovsctrl;
#[doc = "SYNCBUSY register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "No Description"]
pub mod syncbusy;
#[doc = "LOCK register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "No Description"]
pub mod lock;
