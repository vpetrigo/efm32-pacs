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
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x10 - No Description"]
    pub init: crate::Reg<init::INIT_SPEC>,
    #[doc = "0x14 - No Description"]
    pub poly: crate::Reg<poly::POLY_SPEC>,
    #[doc = "0x18 - No Description"]
    pub inputdata: crate::Reg<inputdata::INPUTDATA_SPEC>,
    #[doc = "0x1c - No Description"]
    pub inputdatahword: crate::Reg<inputdatahword::INPUTDATAHWORD_SPEC>,
    #[doc = "0x20 - No Description"]
    pub inputdatabyte: crate::Reg<inputdatabyte::INPUTDATABYTE_SPEC>,
    #[doc = "0x24 - No Description"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x28 - No Description"]
    pub datarev: crate::Reg<datarev::DATAREV_SPEC>,
    #[doc = "0x2c - No Description"]
    pub databyterev: crate::Reg<databyterev::DATABYTEREV_SPEC>,
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
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "INIT register accessor: an alias for `Reg<INIT_SPEC>`"]
pub type INIT = crate::Reg<init::INIT_SPEC>;
#[doc = "No Description"]
pub mod init;
#[doc = "POLY register accessor: an alias for `Reg<POLY_SPEC>`"]
pub type POLY = crate::Reg<poly::POLY_SPEC>;
#[doc = "No Description"]
pub mod poly;
#[doc = "INPUTDATA register accessor: an alias for `Reg<INPUTDATA_SPEC>`"]
pub type INPUTDATA = crate::Reg<inputdata::INPUTDATA_SPEC>;
#[doc = "No Description"]
pub mod inputdata;
#[doc = "INPUTDATAHWORD register accessor: an alias for `Reg<INPUTDATAHWORD_SPEC>`"]
pub type INPUTDATAHWORD = crate::Reg<inputdatahword::INPUTDATAHWORD_SPEC>;
#[doc = "No Description"]
pub mod inputdatahword;
#[doc = "INPUTDATABYTE register accessor: an alias for `Reg<INPUTDATABYTE_SPEC>`"]
pub type INPUTDATABYTE = crate::Reg<inputdatabyte::INPUTDATABYTE_SPEC>;
#[doc = "No Description"]
pub mod inputdatabyte;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "No Description"]
pub mod data;
#[doc = "DATAREV register accessor: an alias for `Reg<DATAREV_SPEC>`"]
pub type DATAREV = crate::Reg<datarev::DATAREV_SPEC>;
#[doc = "No Description"]
pub mod datarev;
#[doc = "DATABYTEREV register accessor: an alias for `Reg<DATABYTEREV_SPEC>`"]
pub type DATABYTEREV = crate::Reg<databyterev::DATABYTEREV_SPEC>;
#[doc = "No Description"]
pub mod databyterev;
