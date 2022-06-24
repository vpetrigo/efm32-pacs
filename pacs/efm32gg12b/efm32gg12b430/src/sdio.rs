#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SDMA System Address Register"]
    pub sdmasysaddr: crate::Reg<sdmasysaddr::SDMASYSADDR_SPEC>,
    #[doc = "0x04 - Block Size and Block Count Register"]
    pub blksize: crate::Reg<blksize::BLKSIZE_SPEC>,
    #[doc = "0x08 - SD Command Argument Register"]
    pub cmdarg1: crate::Reg<cmdarg1::CMDARG1_SPEC>,
    #[doc = "0x0c - Transfer Mode and Command Register"]
    pub tfrmode: crate::Reg<tfrmode::TFRMODE_SPEC>,
    #[doc = "0x10 - Response0 and Response1 Register"]
    pub resp0: crate::Reg<resp0::RESP0_SPEC>,
    #[doc = "0x14 - Response2 and Response3 Register"]
    pub resp2: crate::Reg<resp2::RESP2_SPEC>,
    #[doc = "0x18 - Response4 and Response5 Register"]
    pub resp4: crate::Reg<resp4::RESP4_SPEC>,
    #[doc = "0x1c - Response6 and Response7 Register"]
    pub resp6: crate::Reg<resp6::RESP6_SPEC>,
    #[doc = "0x20 - Buffer Data Register"]
    pub bufdatport: crate::Reg<bufdatport::BUFDATPORT_SPEC>,
    #[doc = "0x24 - Present State Register"]
    pub prsstat: crate::Reg<prsstat::PRSSTAT_SPEC>,
    #[doc = "0x28 - Host Control1, Power, Block Gap and Wakeup-up Control Register"]
    pub hostctrl1: crate::Reg<hostctrl1::HOSTCTRL1_SPEC>,
    #[doc = "0x2c - Clock Control, Timeout Control and Software Register"]
    pub clockctrl: crate::Reg<clockctrl::CLOCKCTRL_SPEC>,
    #[doc = "0x30 - Normal and Error Interrupt Status Register"]
    pub ifcr: crate::Reg<ifcr::IFCR_SPEC>,
    #[doc = "0x34 - Normal and Error Interrupt Status Enable Register"]
    pub ifenc: crate::Reg<ifenc::IFENC_SPEC>,
    #[doc = "0x38 - Normal and Error Interrupt Signal Enable Register"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    #[doc = "0x3c - AUTO CMD12 Error Status and Host Control2 Register"]
    pub ac12errstat: crate::Reg<ac12errstat::AC12ERRSTAT_SPEC>,
    #[doc = "0x40 - Capabilities Register to Hold Bits 31~0"]
    pub capab0: crate::Reg<capab0::CAPAB0_SPEC>,
    #[doc = "0x44 - Capabilities Register to Hold Bits 63~32"]
    pub capab2: crate::Reg<capab2::CAPAB2_SPEC>,
    #[doc = "0x48 - Maximum Current Capabilities Register"]
    pub maxcurcapab: crate::Reg<maxcurcapab::MAXCURCAPAB_SPEC>,
    _reserved19: [u8; 0x04],
    #[doc = "0x50 - Force Event Register for Auto CMD Error Status"]
    pub fevterrstat: crate::Reg<fevterrstat::FEVTERRSTAT_SPEC>,
    #[doc = "0x54 - ADMA Error Status Register"]
    pub admaes: crate::Reg<admaes::ADMAES_SPEC>,
    #[doc = "0x58 - ADMA System Address Register"]
    pub adsaddr: crate::Reg<adsaddr::ADSADDR_SPEC>,
    _reserved22: [u8; 0x04],
    #[doc = "0x60 - Preset Value for Initialization and Default Speed Mode"]
    pub prstval0: crate::Reg<prstval0::PRSTVAL0_SPEC>,
    #[doc = "0x64 - Preset Value for High Speed and SDR12 Modes"]
    pub prstval2: crate::Reg<prstval2::PRSTVAL2_SPEC>,
    #[doc = "0x68 - Preset Value for SDR25 and SDR50 Modes"]
    pub prstval4: crate::Reg<prstval4::PRSTVAL4_SPEC>,
    #[doc = "0x6c - Preset Value for SDR104 and DDR50 Modes"]
    pub prstval6: crate::Reg<prstval6::PRSTVAL6_SPEC>,
    #[doc = "0x70 - Boot Timeout Control Register"]
    pub boottoctrl: crate::Reg<boottoctrl::BOOTTOCTRL_SPEC>,
    _reserved27: [u8; 0x88],
    #[doc = "0xfc - Slot Interrupt Status Register"]
    pub slotintstat: crate::Reg<slotintstat::SLOTINTSTAT_SPEC>,
    _reserved28: [u8; 0x0700],
    #[doc = "0x800 - Core Control Signals"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x804 - Core Configuration 0"]
    pub cfg0: crate::Reg<cfg0::CFG0_SPEC>,
    #[doc = "0x808 - Core Configuration 1"]
    pub cfg1: crate::Reg<cfg1::CFG1_SPEC>,
    #[doc = "0x80c - Core Configuration Preset Value 0"]
    pub cfgpresetval0: crate::Reg<cfgpresetval0::CFGPRESETVAL0_SPEC>,
    #[doc = "0x810 - Core Configuration Preset Value 1"]
    pub cfgpresetval1: crate::Reg<cfgpresetval1::CFGPRESETVAL1_SPEC>,
    #[doc = "0x814 - Core Configuration Preset Value 2"]
    pub cfgpresetval2: crate::Reg<cfgpresetval2::CFGPRESETVAL2_SPEC>,
    #[doc = "0x818 - Core Configuration Preset Value 3"]
    pub cfgpresetval3: crate::Reg<cfgpresetval3::CFGPRESETVAL3_SPEC>,
    #[doc = "0x81c - I/O LOCATION Register"]
    pub routeloc0: crate::Reg<routeloc0::ROUTELOC0_SPEC>,
    #[doc = "0x820 - I/O LOCATION Register"]
    pub routeloc1: crate::Reg<routeloc1::ROUTELOC1_SPEC>,
    #[doc = "0x824 - I/O LOCATION Enable Register"]
    pub routepen: crate::Reg<routepen::ROUTEPEN_SPEC>,
}
#[doc = "SDMASYSADDR register accessor: an alias for `Reg<SDMASYSADDR_SPEC>`"]
pub type SDMASYSADDR = crate::Reg<sdmasysaddr::SDMASYSADDR_SPEC>;
#[doc = "SDMA System Address Register"]
pub mod sdmasysaddr;
#[doc = "BLKSIZE register accessor: an alias for `Reg<BLKSIZE_SPEC>`"]
pub type BLKSIZE = crate::Reg<blksize::BLKSIZE_SPEC>;
#[doc = "Block Size and Block Count Register"]
pub mod blksize;
#[doc = "CMDARG1 register accessor: an alias for `Reg<CMDARG1_SPEC>`"]
pub type CMDARG1 = crate::Reg<cmdarg1::CMDARG1_SPEC>;
#[doc = "SD Command Argument Register"]
pub mod cmdarg1;
#[doc = "TFRMODE register accessor: an alias for `Reg<TFRMODE_SPEC>`"]
pub type TFRMODE = crate::Reg<tfrmode::TFRMODE_SPEC>;
#[doc = "Transfer Mode and Command Register"]
pub mod tfrmode;
#[doc = "RESP0 register accessor: an alias for `Reg<RESP0_SPEC>`"]
pub type RESP0 = crate::Reg<resp0::RESP0_SPEC>;
#[doc = "Response0 and Response1 Register"]
pub mod resp0;
#[doc = "RESP2 register accessor: an alias for `Reg<RESP2_SPEC>`"]
pub type RESP2 = crate::Reg<resp2::RESP2_SPEC>;
#[doc = "Response2 and Response3 Register"]
pub mod resp2;
#[doc = "RESP4 register accessor: an alias for `Reg<RESP4_SPEC>`"]
pub type RESP4 = crate::Reg<resp4::RESP4_SPEC>;
#[doc = "Response4 and Response5 Register"]
pub mod resp4;
#[doc = "RESP6 register accessor: an alias for `Reg<RESP6_SPEC>`"]
pub type RESP6 = crate::Reg<resp6::RESP6_SPEC>;
#[doc = "Response6 and Response7 Register"]
pub mod resp6;
#[doc = "BUFDATPORT register accessor: an alias for `Reg<BUFDATPORT_SPEC>`"]
pub type BUFDATPORT = crate::Reg<bufdatport::BUFDATPORT_SPEC>;
#[doc = "Buffer Data Register"]
pub mod bufdatport;
#[doc = "PRSSTAT register accessor: an alias for `Reg<PRSSTAT_SPEC>`"]
pub type PRSSTAT = crate::Reg<prsstat::PRSSTAT_SPEC>;
#[doc = "Present State Register"]
pub mod prsstat;
#[doc = "HOSTCTRL1 register accessor: an alias for `Reg<HOSTCTRL1_SPEC>`"]
pub type HOSTCTRL1 = crate::Reg<hostctrl1::HOSTCTRL1_SPEC>;
#[doc = "Host Control1, Power, Block Gap and Wakeup-up Control Register"]
pub mod hostctrl1;
#[doc = "CLOCKCTRL register accessor: an alias for `Reg<CLOCKCTRL_SPEC>`"]
pub type CLOCKCTRL = crate::Reg<clockctrl::CLOCKCTRL_SPEC>;
#[doc = "Clock Control, Timeout Control and Software Register"]
pub mod clockctrl;
#[doc = "IFCR register accessor: an alias for `Reg<IFCR_SPEC>`"]
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
#[doc = "Normal and Error Interrupt Status Register"]
pub mod ifcr;
#[doc = "IFENC register accessor: an alias for `Reg<IFENC_SPEC>`"]
pub type IFENC = crate::Reg<ifenc::IFENC_SPEC>;
#[doc = "Normal and Error Interrupt Status Enable Register"]
pub mod ifenc;
#[doc = "IEN register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Normal and Error Interrupt Signal Enable Register"]
pub mod ien;
#[doc = "AC12ERRSTAT register accessor: an alias for `Reg<AC12ERRSTAT_SPEC>`"]
pub type AC12ERRSTAT = crate::Reg<ac12errstat::AC12ERRSTAT_SPEC>;
#[doc = "AUTO CMD12 Error Status and Host Control2 Register"]
pub mod ac12errstat;
#[doc = "CAPAB0 register accessor: an alias for `Reg<CAPAB0_SPEC>`"]
pub type CAPAB0 = crate::Reg<capab0::CAPAB0_SPEC>;
#[doc = "Capabilities Register to Hold Bits 31~0"]
pub mod capab0;
#[doc = "CAPAB2 register accessor: an alias for `Reg<CAPAB2_SPEC>`"]
pub type CAPAB2 = crate::Reg<capab2::CAPAB2_SPEC>;
#[doc = "Capabilities Register to Hold Bits 63~32"]
pub mod capab2;
#[doc = "MAXCURCAPAB register accessor: an alias for `Reg<MAXCURCAPAB_SPEC>`"]
pub type MAXCURCAPAB = crate::Reg<maxcurcapab::MAXCURCAPAB_SPEC>;
#[doc = "Maximum Current Capabilities Register"]
pub mod maxcurcapab;
#[doc = "FEVTERRSTAT register accessor: an alias for `Reg<FEVTERRSTAT_SPEC>`"]
pub type FEVTERRSTAT = crate::Reg<fevterrstat::FEVTERRSTAT_SPEC>;
#[doc = "Force Event Register for Auto CMD Error Status"]
pub mod fevterrstat;
#[doc = "ADMAES register accessor: an alias for `Reg<ADMAES_SPEC>`"]
pub type ADMAES = crate::Reg<admaes::ADMAES_SPEC>;
#[doc = "ADMA Error Status Register"]
pub mod admaes;
#[doc = "ADSADDR register accessor: an alias for `Reg<ADSADDR_SPEC>`"]
pub type ADSADDR = crate::Reg<adsaddr::ADSADDR_SPEC>;
#[doc = "ADMA System Address Register"]
pub mod adsaddr;
#[doc = "PRSTVAL0 register accessor: an alias for `Reg<PRSTVAL0_SPEC>`"]
pub type PRSTVAL0 = crate::Reg<prstval0::PRSTVAL0_SPEC>;
#[doc = "Preset Value for Initialization and Default Speed Mode"]
pub mod prstval0;
#[doc = "PRSTVAL2 register accessor: an alias for `Reg<PRSTVAL2_SPEC>`"]
pub type PRSTVAL2 = crate::Reg<prstval2::PRSTVAL2_SPEC>;
#[doc = "Preset Value for High Speed and SDR12 Modes"]
pub mod prstval2;
#[doc = "PRSTVAL4 register accessor: an alias for `Reg<PRSTVAL4_SPEC>`"]
pub type PRSTVAL4 = crate::Reg<prstval4::PRSTVAL4_SPEC>;
#[doc = "Preset Value for SDR25 and SDR50 Modes"]
pub mod prstval4;
#[doc = "PRSTVAL6 register accessor: an alias for `Reg<PRSTVAL6_SPEC>`"]
pub type PRSTVAL6 = crate::Reg<prstval6::PRSTVAL6_SPEC>;
#[doc = "Preset Value for SDR104 and DDR50 Modes"]
pub mod prstval6;
#[doc = "BOOTTOCTRL register accessor: an alias for `Reg<BOOTTOCTRL_SPEC>`"]
pub type BOOTTOCTRL = crate::Reg<boottoctrl::BOOTTOCTRL_SPEC>;
#[doc = "Boot Timeout Control Register"]
pub mod boottoctrl;
#[doc = "SLOTINTSTAT register accessor: an alias for `Reg<SLOTINTSTAT_SPEC>`"]
pub type SLOTINTSTAT = crate::Reg<slotintstat::SLOTINTSTAT_SPEC>;
#[doc = "Slot Interrupt Status Register"]
pub mod slotintstat;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Core Control Signals"]
pub mod ctrl;
#[doc = "CFG0 register accessor: an alias for `Reg<CFG0_SPEC>`"]
pub type CFG0 = crate::Reg<cfg0::CFG0_SPEC>;
#[doc = "Core Configuration 0"]
pub mod cfg0;
#[doc = "CFG1 register accessor: an alias for `Reg<CFG1_SPEC>`"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "Core Configuration 1"]
pub mod cfg1;
#[doc = "CFGPRESETVAL0 register accessor: an alias for `Reg<CFGPRESETVAL0_SPEC>`"]
pub type CFGPRESETVAL0 = crate::Reg<cfgpresetval0::CFGPRESETVAL0_SPEC>;
#[doc = "Core Configuration Preset Value 0"]
pub mod cfgpresetval0;
#[doc = "CFGPRESETVAL1 register accessor: an alias for `Reg<CFGPRESETVAL1_SPEC>`"]
pub type CFGPRESETVAL1 = crate::Reg<cfgpresetval1::CFGPRESETVAL1_SPEC>;
#[doc = "Core Configuration Preset Value 1"]
pub mod cfgpresetval1;
#[doc = "CFGPRESETVAL2 register accessor: an alias for `Reg<CFGPRESETVAL2_SPEC>`"]
pub type CFGPRESETVAL2 = crate::Reg<cfgpresetval2::CFGPRESETVAL2_SPEC>;
#[doc = "Core Configuration Preset Value 2"]
pub mod cfgpresetval2;
#[doc = "CFGPRESETVAL3 register accessor: an alias for `Reg<CFGPRESETVAL3_SPEC>`"]
pub type CFGPRESETVAL3 = crate::Reg<cfgpresetval3::CFGPRESETVAL3_SPEC>;
#[doc = "Core Configuration Preset Value 3"]
pub mod cfgpresetval3;
#[doc = "ROUTELOC0 register accessor: an alias for `Reg<ROUTELOC0_SPEC>`"]
pub type ROUTELOC0 = crate::Reg<routeloc0::ROUTELOC0_SPEC>;
#[doc = "I/O LOCATION Register"]
pub mod routeloc0;
#[doc = "ROUTELOC1 register accessor: an alias for `Reg<ROUTELOC1_SPEC>`"]
pub type ROUTELOC1 = crate::Reg<routeloc1::ROUTELOC1_SPEC>;
#[doc = "I/O LOCATION Register"]
pub mod routeloc1;
#[doc = "ROUTEPEN register accessor: an alias for `Reg<ROUTEPEN_SPEC>`"]
pub type ROUTEPEN = crate::Reg<routepen::ROUTEPEN_SPEC>;
#[doc = "I/O LOCATION Enable Register"]
pub mod routepen;
