#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The read only IPVERSION field gives the version for this module. There may be minor software changes required for modules with different values of IPVERSION."]
    pub ipversion: crate::Reg<ipversion::IPVERSION_SPEC>,
    #[doc = "0x04 - Read to get SMU status."]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x08 - Access to Lock/unlock the SMU Configuration."]
    pub lock: crate::Reg<lock::LOCK_SPEC>,
    #[doc = "0x0c - Read to get status of SMU interrupts."]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x10 - Write to Enable/Disable SMU interrupts."]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    _reserved5: [u8; 0x0c],
    #[doc = "0x20 - Holds the M33 control settings."]
    pub m33ctrl: crate::Reg<m33ctrl::M33CTRL_SPEC>,
    _reserved6: [u8; 0x1c],
    #[doc = "0x40 - Set peripheral bits to 1 to mark as privileged access only."]
    pub ppupatd0: crate::Reg<ppupatd0::PPUPATD0_SPEC>,
    #[doc = "0x44 - Set peripheral bits to 1 to mark as privileged access only."]
    pub ppupatd1: crate::Reg<ppupatd1::PPUPATD1_SPEC>,
    _reserved8: [u8; 0x18],
    #[doc = "0x60 - Set peripheral bits to 1 to mark as secure access only."]
    pub ppusatd0: crate::Reg<ppusatd0::PPUSATD0_SPEC>,
    #[doc = "0x64 - Set peripheral bits to 1 to mark as secure access only."]
    pub ppusatd1: crate::Reg<ppusatd1::PPUSATD1_SPEC>,
    _reserved10: [u8; 0xd8],
    #[doc = "0x140 - Read to get fault status of SMU."]
    pub ppufs: crate::Reg<ppufs::PPUFS_SPEC>,
    _reserved11: [u8; 0x0c],
    #[doc = "0x150 - Set master bits to 1 to mark as a privileged master."]
    pub bmpupatd0: crate::Reg<bmpupatd0::BMPUPATD0_SPEC>,
    _reserved12: [u8; 0x1c],
    #[doc = "0x170 - Set master bits to 1 to mark as a secure master."]
    pub bmpusatd0: crate::Reg<bmpusatd0::BMPUSATD0_SPEC>,
    _reserved13: [u8; 0xdc],
    #[doc = "0x250 - Read to get status about the master that triggered a fault."]
    pub bmpufs: crate::Reg<bmpufs::BMPUFS_SPEC>,
    #[doc = "0x254 - Read to get the access address that triggered a fault."]
    pub bmpufsaddr: crate::Reg<bmpufsaddr::BMPUFSADDR_SPEC>,
    _reserved15: [u8; 0x08],
    #[doc = "0x260 - Write to specify if a region is secure or non-secure."]
    pub esaurtypes0: crate::Reg<esaurtypes0::ESAURTYPES0_SPEC>,
    #[doc = "0x264 - Write to specify if a region is secure or non-secure."]
    pub esaurtypes1: crate::Reg<esaurtypes1::ESAURTYPES1_SPEC>,
    _reserved17: [u8; 0x08],
    #[doc = "0x270 - Specify the boundary between regions 0 and 1."]
    pub esaumrb01: crate::Reg<esaumrb01::ESAUMRB01_SPEC>,
    #[doc = "0x274 - Specify the boundary between regions 1 and 2."]
    pub esaumrb12: crate::Reg<esaumrb12::ESAUMRB12_SPEC>,
    _reserved19: [u8; 0x08],
    #[doc = "0x280 - Specify the boundary between regions 4 and 5."]
    pub esaumrb45: crate::Reg<esaumrb45::ESAUMRB45_SPEC>,
    #[doc = "0x284 - Specify the boundary between regions 5 and 6."]
    pub esaumrb56: crate::Reg<esaumrb56::ESAUMRB56_SPEC>,
}
#[doc = "IPVERSION register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "The read only IPVERSION field gives the version for this module. There may be minor software changes required for modules with different values of IPVERSION."]
pub mod ipversion;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Read to get SMU status."]
pub mod status;
#[doc = "LOCK register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Access to Lock/unlock the SMU Configuration."]
pub mod lock;
#[doc = "IF register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Read to get status of SMU interrupts."]
pub mod if_;
#[doc = "IEN register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Write to Enable/Disable SMU interrupts."]
pub mod ien;
#[doc = "M33CTRL register accessor: an alias for `Reg<M33CTRL_SPEC>`"]
pub type M33CTRL = crate::Reg<m33ctrl::M33CTRL_SPEC>;
#[doc = "Holds the M33 control settings."]
pub mod m33ctrl;
#[doc = "PPUPATD0 register accessor: an alias for `Reg<PPUPATD0_SPEC>`"]
pub type PPUPATD0 = crate::Reg<ppupatd0::PPUPATD0_SPEC>;
#[doc = "Set peripheral bits to 1 to mark as privileged access only."]
pub mod ppupatd0;
#[doc = "PPUPATD1 register accessor: an alias for `Reg<PPUPATD1_SPEC>`"]
pub type PPUPATD1 = crate::Reg<ppupatd1::PPUPATD1_SPEC>;
#[doc = "Set peripheral bits to 1 to mark as privileged access only."]
pub mod ppupatd1;
#[doc = "PPUSATD0 register accessor: an alias for `Reg<PPUSATD0_SPEC>`"]
pub type PPUSATD0 = crate::Reg<ppusatd0::PPUSATD0_SPEC>;
#[doc = "Set peripheral bits to 1 to mark as secure access only."]
pub mod ppusatd0;
#[doc = "PPUSATD1 register accessor: an alias for `Reg<PPUSATD1_SPEC>`"]
pub type PPUSATD1 = crate::Reg<ppusatd1::PPUSATD1_SPEC>;
#[doc = "Set peripheral bits to 1 to mark as secure access only."]
pub mod ppusatd1;
#[doc = "PPUFS register accessor: an alias for `Reg<PPUFS_SPEC>`"]
pub type PPUFS = crate::Reg<ppufs::PPUFS_SPEC>;
#[doc = "Read to get fault status of SMU."]
pub mod ppufs;
#[doc = "BMPUPATD0 register accessor: an alias for `Reg<BMPUPATD0_SPEC>`"]
pub type BMPUPATD0 = crate::Reg<bmpupatd0::BMPUPATD0_SPEC>;
#[doc = "Set master bits to 1 to mark as a privileged master."]
pub mod bmpupatd0;
#[doc = "BMPUSATD0 register accessor: an alias for `Reg<BMPUSATD0_SPEC>`"]
pub type BMPUSATD0 = crate::Reg<bmpusatd0::BMPUSATD0_SPEC>;
#[doc = "Set master bits to 1 to mark as a secure master."]
pub mod bmpusatd0;
#[doc = "BMPUFS register accessor: an alias for `Reg<BMPUFS_SPEC>`"]
pub type BMPUFS = crate::Reg<bmpufs::BMPUFS_SPEC>;
#[doc = "Read to get status about the master that triggered a fault."]
pub mod bmpufs;
#[doc = "BMPUFSADDR register accessor: an alias for `Reg<BMPUFSADDR_SPEC>`"]
pub type BMPUFSADDR = crate::Reg<bmpufsaddr::BMPUFSADDR_SPEC>;
#[doc = "Read to get the access address that triggered a fault."]
pub mod bmpufsaddr;
#[doc = "ESAURTYPES0 register accessor: an alias for `Reg<ESAURTYPES0_SPEC>`"]
pub type ESAURTYPES0 = crate::Reg<esaurtypes0::ESAURTYPES0_SPEC>;
#[doc = "Write to specify if a region is secure or non-secure."]
pub mod esaurtypes0;
#[doc = "ESAURTYPES1 register accessor: an alias for `Reg<ESAURTYPES1_SPEC>`"]
pub type ESAURTYPES1 = crate::Reg<esaurtypes1::ESAURTYPES1_SPEC>;
#[doc = "Write to specify if a region is secure or non-secure."]
pub mod esaurtypes1;
#[doc = "ESAUMRB01 register accessor: an alias for `Reg<ESAUMRB01_SPEC>`"]
pub type ESAUMRB01 = crate::Reg<esaumrb01::ESAUMRB01_SPEC>;
#[doc = "Specify the boundary between regions 0 and 1."]
pub mod esaumrb01;
#[doc = "ESAUMRB12 register accessor: an alias for `Reg<ESAUMRB12_SPEC>`"]
pub type ESAUMRB12 = crate::Reg<esaumrb12::ESAUMRB12_SPEC>;
#[doc = "Specify the boundary between regions 1 and 2."]
pub mod esaumrb12;
#[doc = "ESAUMRB45 register accessor: an alias for `Reg<ESAUMRB45_SPEC>`"]
pub type ESAUMRB45 = crate::Reg<esaumrb45::ESAUMRB45_SPEC>;
#[doc = "Specify the boundary between regions 4 and 5."]
pub mod esaumrb45;
#[doc = "ESAUMRB56 register accessor: an alias for `Reg<ESAUMRB56_SPEC>`"]
pub type ESAUMRB56 = crate::Reg<esaumrb56::ESAUMRB56_SPEC>;
#[doc = "Specify the boundary between regions 5 and 6."]
pub mod esaumrb56;
