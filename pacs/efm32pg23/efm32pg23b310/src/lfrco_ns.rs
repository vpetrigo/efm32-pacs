#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Contains the LFRCO ip version."]
    pub ipversion: crate::Reg<ipversion::IPVERSION_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Status register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x0c - Calibration register"]
    pub cal: crate::Reg<cal::CAL_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x14 - Interrupt flag register"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x18 - Interrupt enable register."]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    #[doc = "0x1c - Synchronization busy register"]
    pub syncbusy: crate::Reg<syncbusy::SYNCBUSY_SPEC>,
    #[doc = "0x20 - Configuration lock register. Locks/unlocks access to cofiguration registers."]
    pub lock: crate::Reg<lock::LOCK_SPEC>,
}
#[doc = "IPVERSION register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "Contains the LFRCO ip version."]
pub mod ipversion;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status register"]
pub mod status;
#[doc = "CAL register accessor: an alias for `Reg<CAL_SPEC>`"]
pub type CAL = crate::Reg<cal::CAL_SPEC>;
#[doc = "Calibration register"]
pub mod cal;
#[doc = "IF register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt flag register"]
pub mod if_;
#[doc = "IEN register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt enable register."]
pub mod ien;
#[doc = "SYNCBUSY register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization busy register"]
pub mod syncbusy;
#[doc = "LOCK register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Configuration lock register. Locks/unlocks access to cofiguration registers."]
pub mod lock;
