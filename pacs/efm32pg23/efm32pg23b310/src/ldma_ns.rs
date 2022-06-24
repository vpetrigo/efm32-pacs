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
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x10 - No Description"]
    pub syncswset: crate::Reg<syncswset::SYNCSWSET_SPEC>,
    #[doc = "0x14 - No Description"]
    pub syncswclr: crate::Reg<syncswclr::SYNCSWCLR_SPEC>,
    #[doc = "0x18 - No Description"]
    pub synchwen: crate::Reg<synchwen::SYNCHWEN_SPEC>,
    #[doc = "0x1c - No Description"]
    pub synchwsel: crate::Reg<synchwsel::SYNCHWSEL_SPEC>,
    #[doc = "0x20 - No Description"]
    pub syncstatus: crate::Reg<syncstatus::SYNCSTATUS_SPEC>,
    #[doc = "0x24 - No Description"]
    pub chen: crate::Reg<chen::CHEN_SPEC>,
    #[doc = "0x28 - No Description"]
    pub chdis: crate::Reg<chdis::CHDIS_SPEC>,
    #[doc = "0x2c - No Description"]
    pub chstatus: crate::Reg<chstatus::CHSTATUS_SPEC>,
    #[doc = "0x30 - No Description"]
    pub chbusy: crate::Reg<chbusy::CHBUSY_SPEC>,
    #[doc = "0x34 - No Description"]
    pub chdone: crate::Reg<chdone::CHDONE_SPEC>,
    #[doc = "0x38 - No Description"]
    pub dbghalt: crate::Reg<dbghalt::DBGHALT_SPEC>,
    #[doc = "0x3c - No Description"]
    pub swreq: crate::Reg<swreq::SWREQ_SPEC>,
    #[doc = "0x40 - No Description"]
    pub reqdis: crate::Reg<reqdis::REQDIS_SPEC>,
    #[doc = "0x44 - No Description"]
    pub reqpend: crate::Reg<reqpend::REQPEND_SPEC>,
    #[doc = "0x48 - No Description"]
    pub linkload: crate::Reg<linkload::LINKLOAD_SPEC>,
    #[doc = "0x4c - No Description"]
    pub reqclear: crate::Reg<reqclear::REQCLEAR_SPEC>,
    #[doc = "0x50 - No Description"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x54 - No Description"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    _reserved22: [u8; 0x04],
    #[doc = "0x5c - No Description"]
    pub ch0_cfg: crate::Reg<ch0_cfg::CH0_CFG_SPEC>,
    #[doc = "0x60 - No Description"]
    pub ch0_loop: crate::Reg<ch0_loop::CH0_LOOP_SPEC>,
    #[doc = "0x64 - No Description"]
    pub ch0_ctrl: crate::Reg<ch0_ctrl::CH0_CTRL_SPEC>,
    #[doc = "0x68 - No Description"]
    pub ch0_src: crate::Reg<ch0_src::CH0_SRC_SPEC>,
    #[doc = "0x6c - No Description"]
    pub ch0_dst: crate::Reg<ch0_dst::CH0_DST_SPEC>,
    #[doc = "0x70 - No Description"]
    pub ch0_link: crate::Reg<ch0_link::CH0_LINK_SPEC>,
    _reserved28: [u8; 0x18],
    #[doc = "0x8c - No Description"]
    pub ch1_cfg: crate::Reg<ch1_cfg::CH1_CFG_SPEC>,
    #[doc = "0x90 - No Description"]
    pub ch1_loop: crate::Reg<ch1_loop::CH1_LOOP_SPEC>,
    #[doc = "0x94 - No Description"]
    pub ch1_ctrl: crate::Reg<ch1_ctrl::CH1_CTRL_SPEC>,
    #[doc = "0x98 - No Description"]
    pub ch1_src: crate::Reg<ch1_src::CH1_SRC_SPEC>,
    #[doc = "0x9c - No Description"]
    pub ch1_dst: crate::Reg<ch1_dst::CH1_DST_SPEC>,
    #[doc = "0xa0 - No Description"]
    pub ch1_link: crate::Reg<ch1_link::CH1_LINK_SPEC>,
    _reserved34: [u8; 0x18],
    #[doc = "0xbc - No Description"]
    pub ch2_cfg: crate::Reg<ch2_cfg::CH2_CFG_SPEC>,
    #[doc = "0xc0 - No Description"]
    pub ch2_loop: crate::Reg<ch2_loop::CH2_LOOP_SPEC>,
    #[doc = "0xc4 - No Description"]
    pub ch2_ctrl: crate::Reg<ch2_ctrl::CH2_CTRL_SPEC>,
    #[doc = "0xc8 - No Description"]
    pub ch2_src: crate::Reg<ch2_src::CH2_SRC_SPEC>,
    #[doc = "0xcc - No Description"]
    pub ch2_dst: crate::Reg<ch2_dst::CH2_DST_SPEC>,
    #[doc = "0xd0 - No Description"]
    pub ch2_link: crate::Reg<ch2_link::CH2_LINK_SPEC>,
    _reserved40: [u8; 0x18],
    #[doc = "0xec - No Description"]
    pub ch3_cfg: crate::Reg<ch3_cfg::CH3_CFG_SPEC>,
    #[doc = "0xf0 - No Description"]
    pub ch3_loop: crate::Reg<ch3_loop::CH3_LOOP_SPEC>,
    #[doc = "0xf4 - No Description"]
    pub ch3_ctrl: crate::Reg<ch3_ctrl::CH3_CTRL_SPEC>,
    #[doc = "0xf8 - No Description"]
    pub ch3_src: crate::Reg<ch3_src::CH3_SRC_SPEC>,
    #[doc = "0xfc - No Description"]
    pub ch3_dst: crate::Reg<ch3_dst::CH3_DST_SPEC>,
    #[doc = "0x100 - No Description"]
    pub ch3_link: crate::Reg<ch3_link::CH3_LINK_SPEC>,
    _reserved46: [u8; 0x18],
    #[doc = "0x11c - No Description"]
    pub ch4_cfg: crate::Reg<ch4_cfg::CH4_CFG_SPEC>,
    #[doc = "0x120 - No Description"]
    pub ch4_loop: crate::Reg<ch4_loop::CH4_LOOP_SPEC>,
    #[doc = "0x124 - No Description"]
    pub ch4_ctrl: crate::Reg<ch4_ctrl::CH4_CTRL_SPEC>,
    #[doc = "0x128 - No Description"]
    pub ch4_src: crate::Reg<ch4_src::CH4_SRC_SPEC>,
    #[doc = "0x12c - No Description"]
    pub ch4_dst: crate::Reg<ch4_dst::CH4_DST_SPEC>,
    #[doc = "0x130 - No Description"]
    pub ch4_link: crate::Reg<ch4_link::CH4_LINK_SPEC>,
    _reserved52: [u8; 0x18],
    #[doc = "0x14c - No Description"]
    pub ch5_cfg: crate::Reg<ch5_cfg::CH5_CFG_SPEC>,
    #[doc = "0x150 - No Description"]
    pub ch5_loop: crate::Reg<ch5_loop::CH5_LOOP_SPEC>,
    #[doc = "0x154 - No Description"]
    pub ch5_ctrl: crate::Reg<ch5_ctrl::CH5_CTRL_SPEC>,
    #[doc = "0x158 - No Description"]
    pub ch5_src: crate::Reg<ch5_src::CH5_SRC_SPEC>,
    #[doc = "0x15c - No Description"]
    pub ch5_dst: crate::Reg<ch5_dst::CH5_DST_SPEC>,
    #[doc = "0x160 - No Description"]
    pub ch5_link: crate::Reg<ch5_link::CH5_LINK_SPEC>,
    _reserved58: [u8; 0x18],
    #[doc = "0x17c - No Description"]
    pub ch6_cfg: crate::Reg<ch6_cfg::CH6_CFG_SPEC>,
    #[doc = "0x180 - No Description"]
    pub ch6_loop: crate::Reg<ch6_loop::CH6_LOOP_SPEC>,
    #[doc = "0x184 - No Description"]
    pub ch6_ctrl: crate::Reg<ch6_ctrl::CH6_CTRL_SPEC>,
    #[doc = "0x188 - No Description"]
    pub ch6_src: crate::Reg<ch6_src::CH6_SRC_SPEC>,
    #[doc = "0x18c - No Description"]
    pub ch6_dst: crate::Reg<ch6_dst::CH6_DST_SPEC>,
    #[doc = "0x190 - No Description"]
    pub ch6_link: crate::Reg<ch6_link::CH6_LINK_SPEC>,
    _reserved64: [u8; 0x18],
    #[doc = "0x1ac - No Description"]
    pub ch7_cfg: crate::Reg<ch7_cfg::CH7_CFG_SPEC>,
    #[doc = "0x1b0 - No Description"]
    pub ch7_loop: crate::Reg<ch7_loop::CH7_LOOP_SPEC>,
    #[doc = "0x1b4 - No Description"]
    pub ch7_ctrl: crate::Reg<ch7_ctrl::CH7_CTRL_SPEC>,
    #[doc = "0x1b8 - No Description"]
    pub ch7_src: crate::Reg<ch7_src::CH7_SRC_SPEC>,
    #[doc = "0x1bc - No Description"]
    pub ch7_dst: crate::Reg<ch7_dst::CH7_DST_SPEC>,
    #[doc = "0x1c0 - No Description"]
    pub ch7_link: crate::Reg<ch7_link::CH7_LINK_SPEC>,
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
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "No Description"]
pub mod status;
#[doc = "SYNCSWSET register accessor: an alias for `Reg<SYNCSWSET_SPEC>`"]
pub type SYNCSWSET = crate::Reg<syncswset::SYNCSWSET_SPEC>;
#[doc = "No Description"]
pub mod syncswset;
#[doc = "SYNCSWCLR register accessor: an alias for `Reg<SYNCSWCLR_SPEC>`"]
pub type SYNCSWCLR = crate::Reg<syncswclr::SYNCSWCLR_SPEC>;
#[doc = "No Description"]
pub mod syncswclr;
#[doc = "SYNCHWEN register accessor: an alias for `Reg<SYNCHWEN_SPEC>`"]
pub type SYNCHWEN = crate::Reg<synchwen::SYNCHWEN_SPEC>;
#[doc = "No Description"]
pub mod synchwen;
#[doc = "SYNCHWSEL register accessor: an alias for `Reg<SYNCHWSEL_SPEC>`"]
pub type SYNCHWSEL = crate::Reg<synchwsel::SYNCHWSEL_SPEC>;
#[doc = "No Description"]
pub mod synchwsel;
#[doc = "SYNCSTATUS register accessor: an alias for `Reg<SYNCSTATUS_SPEC>`"]
pub type SYNCSTATUS = crate::Reg<syncstatus::SYNCSTATUS_SPEC>;
#[doc = "No Description"]
pub mod syncstatus;
#[doc = "CHEN register accessor: an alias for `Reg<CHEN_SPEC>`"]
pub type CHEN = crate::Reg<chen::CHEN_SPEC>;
#[doc = "No Description"]
pub mod chen;
#[doc = "CHDIS register accessor: an alias for `Reg<CHDIS_SPEC>`"]
pub type CHDIS = crate::Reg<chdis::CHDIS_SPEC>;
#[doc = "No Description"]
pub mod chdis;
#[doc = "CHSTATUS register accessor: an alias for `Reg<CHSTATUS_SPEC>`"]
pub type CHSTATUS = crate::Reg<chstatus::CHSTATUS_SPEC>;
#[doc = "No Description"]
pub mod chstatus;
#[doc = "CHBUSY register accessor: an alias for `Reg<CHBUSY_SPEC>`"]
pub type CHBUSY = crate::Reg<chbusy::CHBUSY_SPEC>;
#[doc = "No Description"]
pub mod chbusy;
#[doc = "CHDONE register accessor: an alias for `Reg<CHDONE_SPEC>`"]
pub type CHDONE = crate::Reg<chdone::CHDONE_SPEC>;
#[doc = "No Description"]
pub mod chdone;
#[doc = "DBGHALT register accessor: an alias for `Reg<DBGHALT_SPEC>`"]
pub type DBGHALT = crate::Reg<dbghalt::DBGHALT_SPEC>;
#[doc = "No Description"]
pub mod dbghalt;
#[doc = "SWREQ register accessor: an alias for `Reg<SWREQ_SPEC>`"]
pub type SWREQ = crate::Reg<swreq::SWREQ_SPEC>;
#[doc = "No Description"]
pub mod swreq;
#[doc = "REQDIS register accessor: an alias for `Reg<REQDIS_SPEC>`"]
pub type REQDIS = crate::Reg<reqdis::REQDIS_SPEC>;
#[doc = "No Description"]
pub mod reqdis;
#[doc = "REQPEND register accessor: an alias for `Reg<REQPEND_SPEC>`"]
pub type REQPEND = crate::Reg<reqpend::REQPEND_SPEC>;
#[doc = "No Description"]
pub mod reqpend;
#[doc = "LINKLOAD register accessor: an alias for `Reg<LINKLOAD_SPEC>`"]
pub type LINKLOAD = crate::Reg<linkload::LINKLOAD_SPEC>;
#[doc = "No Description"]
pub mod linkload;
#[doc = "REQCLEAR register accessor: an alias for `Reg<REQCLEAR_SPEC>`"]
pub type REQCLEAR = crate::Reg<reqclear::REQCLEAR_SPEC>;
#[doc = "No Description"]
pub mod reqclear;
#[doc = "IF register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "No Description"]
pub mod if_;
#[doc = "IEN register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "No Description"]
pub mod ien;
#[doc = "CH0_CFG register accessor: an alias for `Reg<CH0_CFG_SPEC>`"]
pub type CH0_CFG = crate::Reg<ch0_cfg::CH0_CFG_SPEC>;
#[doc = "No Description"]
pub mod ch0_cfg;
#[doc = "CH0_LOOP register accessor: an alias for `Reg<CH0_LOOP_SPEC>`"]
pub type CH0_LOOP = crate::Reg<ch0_loop::CH0_LOOP_SPEC>;
#[doc = "No Description"]
pub mod ch0_loop;
#[doc = "CH0_CTRL register accessor: an alias for `Reg<CH0_CTRL_SPEC>`"]
pub type CH0_CTRL = crate::Reg<ch0_ctrl::CH0_CTRL_SPEC>;
#[doc = "No Description"]
pub mod ch0_ctrl;
#[doc = "CH0_SRC register accessor: an alias for `Reg<CH0_SRC_SPEC>`"]
pub type CH0_SRC = crate::Reg<ch0_src::CH0_SRC_SPEC>;
#[doc = "No Description"]
pub mod ch0_src;
#[doc = "CH0_DST register accessor: an alias for `Reg<CH0_DST_SPEC>`"]
pub type CH0_DST = crate::Reg<ch0_dst::CH0_DST_SPEC>;
#[doc = "No Description"]
pub mod ch0_dst;
#[doc = "CH0_LINK register accessor: an alias for `Reg<CH0_LINK_SPEC>`"]
pub type CH0_LINK = crate::Reg<ch0_link::CH0_LINK_SPEC>;
#[doc = "No Description"]
pub mod ch0_link;
#[doc = "CH1_CFG register accessor: an alias for `Reg<CH1_CFG_SPEC>`"]
pub type CH1_CFG = crate::Reg<ch1_cfg::CH1_CFG_SPEC>;
#[doc = "No Description"]
pub mod ch1_cfg;
#[doc = "CH1_LOOP register accessor: an alias for `Reg<CH1_LOOP_SPEC>`"]
pub type CH1_LOOP = crate::Reg<ch1_loop::CH1_LOOP_SPEC>;
#[doc = "No Description"]
pub mod ch1_loop;
#[doc = "CH1_CTRL register accessor: an alias for `Reg<CH1_CTRL_SPEC>`"]
pub type CH1_CTRL = crate::Reg<ch1_ctrl::CH1_CTRL_SPEC>;
#[doc = "No Description"]
pub mod ch1_ctrl;
#[doc = "CH1_SRC register accessor: an alias for `Reg<CH1_SRC_SPEC>`"]
pub type CH1_SRC = crate::Reg<ch1_src::CH1_SRC_SPEC>;
#[doc = "No Description"]
pub mod ch1_src;
#[doc = "CH1_DST register accessor: an alias for `Reg<CH1_DST_SPEC>`"]
pub type CH1_DST = crate::Reg<ch1_dst::CH1_DST_SPEC>;
#[doc = "No Description"]
pub mod ch1_dst;
#[doc = "CH1_LINK register accessor: an alias for `Reg<CH1_LINK_SPEC>`"]
pub type CH1_LINK = crate::Reg<ch1_link::CH1_LINK_SPEC>;
#[doc = "No Description"]
pub mod ch1_link;
#[doc = "CH2_CFG register accessor: an alias for `Reg<CH2_CFG_SPEC>`"]
pub type CH2_CFG = crate::Reg<ch2_cfg::CH2_CFG_SPEC>;
#[doc = "No Description"]
pub mod ch2_cfg;
#[doc = "CH2_LOOP register accessor: an alias for `Reg<CH2_LOOP_SPEC>`"]
pub type CH2_LOOP = crate::Reg<ch2_loop::CH2_LOOP_SPEC>;
#[doc = "No Description"]
pub mod ch2_loop;
#[doc = "CH2_CTRL register accessor: an alias for `Reg<CH2_CTRL_SPEC>`"]
pub type CH2_CTRL = crate::Reg<ch2_ctrl::CH2_CTRL_SPEC>;
#[doc = "No Description"]
pub mod ch2_ctrl;
#[doc = "CH2_SRC register accessor: an alias for `Reg<CH2_SRC_SPEC>`"]
pub type CH2_SRC = crate::Reg<ch2_src::CH2_SRC_SPEC>;
#[doc = "No Description"]
pub mod ch2_src;
#[doc = "CH2_DST register accessor: an alias for `Reg<CH2_DST_SPEC>`"]
pub type CH2_DST = crate::Reg<ch2_dst::CH2_DST_SPEC>;
#[doc = "No Description"]
pub mod ch2_dst;
#[doc = "CH2_LINK register accessor: an alias for `Reg<CH2_LINK_SPEC>`"]
pub type CH2_LINK = crate::Reg<ch2_link::CH2_LINK_SPEC>;
#[doc = "No Description"]
pub mod ch2_link;
#[doc = "CH3_CFG register accessor: an alias for `Reg<CH3_CFG_SPEC>`"]
pub type CH3_CFG = crate::Reg<ch3_cfg::CH3_CFG_SPEC>;
#[doc = "No Description"]
pub mod ch3_cfg;
#[doc = "CH3_LOOP register accessor: an alias for `Reg<CH3_LOOP_SPEC>`"]
pub type CH3_LOOP = crate::Reg<ch3_loop::CH3_LOOP_SPEC>;
#[doc = "No Description"]
pub mod ch3_loop;
#[doc = "CH3_CTRL register accessor: an alias for `Reg<CH3_CTRL_SPEC>`"]
pub type CH3_CTRL = crate::Reg<ch3_ctrl::CH3_CTRL_SPEC>;
#[doc = "No Description"]
pub mod ch3_ctrl;
#[doc = "CH3_SRC register accessor: an alias for `Reg<CH3_SRC_SPEC>`"]
pub type CH3_SRC = crate::Reg<ch3_src::CH3_SRC_SPEC>;
#[doc = "No Description"]
pub mod ch3_src;
#[doc = "CH3_DST register accessor: an alias for `Reg<CH3_DST_SPEC>`"]
pub type CH3_DST = crate::Reg<ch3_dst::CH3_DST_SPEC>;
#[doc = "No Description"]
pub mod ch3_dst;
#[doc = "CH3_LINK register accessor: an alias for `Reg<CH3_LINK_SPEC>`"]
pub type CH3_LINK = crate::Reg<ch3_link::CH3_LINK_SPEC>;
#[doc = "No Description"]
pub mod ch3_link;
#[doc = "CH4_CFG register accessor: an alias for `Reg<CH4_CFG_SPEC>`"]
pub type CH4_CFG = crate::Reg<ch4_cfg::CH4_CFG_SPEC>;
#[doc = "No Description"]
pub mod ch4_cfg;
#[doc = "CH4_LOOP register accessor: an alias for `Reg<CH4_LOOP_SPEC>`"]
pub type CH4_LOOP = crate::Reg<ch4_loop::CH4_LOOP_SPEC>;
#[doc = "No Description"]
pub mod ch4_loop;
#[doc = "CH4_CTRL register accessor: an alias for `Reg<CH4_CTRL_SPEC>`"]
pub type CH4_CTRL = crate::Reg<ch4_ctrl::CH4_CTRL_SPEC>;
#[doc = "No Description"]
pub mod ch4_ctrl;
#[doc = "CH4_SRC register accessor: an alias for `Reg<CH4_SRC_SPEC>`"]
pub type CH4_SRC = crate::Reg<ch4_src::CH4_SRC_SPEC>;
#[doc = "No Description"]
pub mod ch4_src;
#[doc = "CH4_DST register accessor: an alias for `Reg<CH4_DST_SPEC>`"]
pub type CH4_DST = crate::Reg<ch4_dst::CH4_DST_SPEC>;
#[doc = "No Description"]
pub mod ch4_dst;
#[doc = "CH4_LINK register accessor: an alias for `Reg<CH4_LINK_SPEC>`"]
pub type CH4_LINK = crate::Reg<ch4_link::CH4_LINK_SPEC>;
#[doc = "No Description"]
pub mod ch4_link;
#[doc = "CH5_CFG register accessor: an alias for `Reg<CH5_CFG_SPEC>`"]
pub type CH5_CFG = crate::Reg<ch5_cfg::CH5_CFG_SPEC>;
#[doc = "No Description"]
pub mod ch5_cfg;
#[doc = "CH5_LOOP register accessor: an alias for `Reg<CH5_LOOP_SPEC>`"]
pub type CH5_LOOP = crate::Reg<ch5_loop::CH5_LOOP_SPEC>;
#[doc = "No Description"]
pub mod ch5_loop;
#[doc = "CH5_CTRL register accessor: an alias for `Reg<CH5_CTRL_SPEC>`"]
pub type CH5_CTRL = crate::Reg<ch5_ctrl::CH5_CTRL_SPEC>;
#[doc = "No Description"]
pub mod ch5_ctrl;
#[doc = "CH5_SRC register accessor: an alias for `Reg<CH5_SRC_SPEC>`"]
pub type CH5_SRC = crate::Reg<ch5_src::CH5_SRC_SPEC>;
#[doc = "No Description"]
pub mod ch5_src;
#[doc = "CH5_DST register accessor: an alias for `Reg<CH5_DST_SPEC>`"]
pub type CH5_DST = crate::Reg<ch5_dst::CH5_DST_SPEC>;
#[doc = "No Description"]
pub mod ch5_dst;
#[doc = "CH5_LINK register accessor: an alias for `Reg<CH5_LINK_SPEC>`"]
pub type CH5_LINK = crate::Reg<ch5_link::CH5_LINK_SPEC>;
#[doc = "No Description"]
pub mod ch5_link;
#[doc = "CH6_CFG register accessor: an alias for `Reg<CH6_CFG_SPEC>`"]
pub type CH6_CFG = crate::Reg<ch6_cfg::CH6_CFG_SPEC>;
#[doc = "No Description"]
pub mod ch6_cfg;
#[doc = "CH6_LOOP register accessor: an alias for `Reg<CH6_LOOP_SPEC>`"]
pub type CH6_LOOP = crate::Reg<ch6_loop::CH6_LOOP_SPEC>;
#[doc = "No Description"]
pub mod ch6_loop;
#[doc = "CH6_CTRL register accessor: an alias for `Reg<CH6_CTRL_SPEC>`"]
pub type CH6_CTRL = crate::Reg<ch6_ctrl::CH6_CTRL_SPEC>;
#[doc = "No Description"]
pub mod ch6_ctrl;
#[doc = "CH6_SRC register accessor: an alias for `Reg<CH6_SRC_SPEC>`"]
pub type CH6_SRC = crate::Reg<ch6_src::CH6_SRC_SPEC>;
#[doc = "No Description"]
pub mod ch6_src;
#[doc = "CH6_DST register accessor: an alias for `Reg<CH6_DST_SPEC>`"]
pub type CH6_DST = crate::Reg<ch6_dst::CH6_DST_SPEC>;
#[doc = "No Description"]
pub mod ch6_dst;
#[doc = "CH6_LINK register accessor: an alias for `Reg<CH6_LINK_SPEC>`"]
pub type CH6_LINK = crate::Reg<ch6_link::CH6_LINK_SPEC>;
#[doc = "No Description"]
pub mod ch6_link;
#[doc = "CH7_CFG register accessor: an alias for `Reg<CH7_CFG_SPEC>`"]
pub type CH7_CFG = crate::Reg<ch7_cfg::CH7_CFG_SPEC>;
#[doc = "No Description"]
pub mod ch7_cfg;
#[doc = "CH7_LOOP register accessor: an alias for `Reg<CH7_LOOP_SPEC>`"]
pub type CH7_LOOP = crate::Reg<ch7_loop::CH7_LOOP_SPEC>;
#[doc = "No Description"]
pub mod ch7_loop;
#[doc = "CH7_CTRL register accessor: an alias for `Reg<CH7_CTRL_SPEC>`"]
pub type CH7_CTRL = crate::Reg<ch7_ctrl::CH7_CTRL_SPEC>;
#[doc = "No Description"]
pub mod ch7_ctrl;
#[doc = "CH7_SRC register accessor: an alias for `Reg<CH7_SRC_SPEC>`"]
pub type CH7_SRC = crate::Reg<ch7_src::CH7_SRC_SPEC>;
#[doc = "No Description"]
pub mod ch7_src;
#[doc = "CH7_DST register accessor: an alias for `Reg<CH7_DST_SPEC>`"]
pub type CH7_DST = crate::Reg<ch7_dst::CH7_DST_SPEC>;
#[doc = "No Description"]
pub mod ch7_dst;
#[doc = "CH7_LINK register accessor: an alias for `Reg<CH7_LINK_SPEC>`"]
pub type CH7_LINK = crate::Reg<ch7_link::CH7_LINK_SPEC>;
#[doc = "No Description"]
pub mod ch7_link;
