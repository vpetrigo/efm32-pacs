#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Octal-SPI Configuration Register"]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    #[doc = "0x04 - Device Read Instruction Configuration Register"]
    pub devinstrrdconfig: crate::Reg<devinstrrdconfig::DEVINSTRRDCONFIG_SPEC>,
    #[doc = "0x08 - Device Write Instruction Configuration Register"]
    pub devinstrwrconfig: crate::Reg<devinstrwrconfig::DEVINSTRWRCONFIG_SPEC>,
    #[doc = "0x0c - Device Delay Register"]
    pub devdelay: crate::Reg<devdelay::DEVDELAY_SPEC>,
    #[doc = "0x10 - Read Data Capture Register"]
    pub rddatacapture: crate::Reg<rddatacapture::RDDATACAPTURE_SPEC>,
    #[doc = "0x14 - Device Size Configuration Register"]
    pub devsizeconfig: crate::Reg<devsizeconfig::DEVSIZECONFIG_SPEC>,
    #[doc = "0x18 - SRAM Partition Configuration Register"]
    pub srampartitioncfg: crate::Reg<srampartitioncfg::SRAMPARTITIONCFG_SPEC>,
    #[doc = "0x1c - Indirect Address Trigger Register"]
    pub indahbaddrtrigger: crate::Reg<indahbaddrtrigger::INDAHBADDRTRIGGER_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x24 - Remap Address Register"]
    pub remapaddr: crate::Reg<remapaddr::REMAPADDR_SPEC>,
    #[doc = "0x28 - Mode Bit Configuration Register"]
    pub modebitconfig: crate::Reg<modebitconfig::MODEBITCONFIG_SPEC>,
    #[doc = "0x2c - SRAM Fill Register"]
    pub sramfill: crate::Reg<sramfill::SRAMFILL_SPEC>,
    #[doc = "0x30 - TX Threshold Register"]
    pub txthresh: crate::Reg<txthresh::TXTHRESH_SPEC>,
    #[doc = "0x34 - RX Threshold Register"]
    pub rxthresh: crate::Reg<rxthresh::RXTHRESH_SPEC>,
    #[doc = "0x38 - Write Completion Control Register"]
    pub writecompletionctrl: crate::Reg<writecompletionctrl::WRITECOMPLETIONCTRL_SPEC>,
    #[doc = "0x3c - Polling Expiration Register"]
    pub noofpollsbefexp: crate::Reg<noofpollsbefexp::NOOFPOLLSBEFEXP_SPEC>,
    #[doc = "0x40 - Interrupt Status Register"]
    pub irqstatus: crate::Reg<irqstatus::IRQSTATUS_SPEC>,
    #[doc = "0x44 - Interrupt Mask"]
    pub irqmask: crate::Reg<irqmask::IRQMASK_SPEC>,
    _reserved17: [u8; 0x08],
    #[doc = "0x50 - Lower Write Protection Register"]
    pub lowerwrprot: crate::Reg<lowerwrprot::LOWERWRPROT_SPEC>,
    #[doc = "0x54 - Upper Write Protection Register"]
    pub upperwrprot: crate::Reg<upperwrprot::UPPERWRPROT_SPEC>,
    #[doc = "0x58 - Write Protection Control Register"]
    pub wrprotctrl: crate::Reg<wrprotctrl::WRPROTCTRL_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x60 - Indirect Read Transfer Control Register"]
    pub indirectreadxferctrl: crate::Reg<indirectreadxferctrl::INDIRECTREADXFERCTRL_SPEC>,
    #[doc = "0x64 - Indirect Read Transfer Watermark Register"]
    pub indirectreadxferwatermark:
        crate::Reg<indirectreadxferwatermark::INDIRECTREADXFERWATERMARK_SPEC>,
    #[doc = "0x68 - Indirect Read Transfer Start Address Register"]
    pub indirectreadxferstart: crate::Reg<indirectreadxferstart::INDIRECTREADXFERSTART_SPEC>,
    #[doc = "0x6c - Indirect Read Transfer Number Bytes Register"]
    pub indirectreadxfernumbytes:
        crate::Reg<indirectreadxfernumbytes::INDIRECTREADXFERNUMBYTES_SPEC>,
    #[doc = "0x70 - Indirect Write Transfer Control Register"]
    pub indirectwritexferctrl: crate::Reg<indirectwritexferctrl::INDIRECTWRITEXFERCTRL_SPEC>,
    #[doc = "0x74 - Indirect Write Transfer Watermark Register"]
    pub indirectwritexferwatermark:
        crate::Reg<indirectwritexferwatermark::INDIRECTWRITEXFERWATERMARK_SPEC>,
    #[doc = "0x78 - Indirect Write Transfer Start Address Register"]
    pub indirectwritexferstart: crate::Reg<indirectwritexferstart::INDIRECTWRITEXFERSTART_SPEC>,
    #[doc = "0x7c - Indirect Write Transfer Number Bytes Register"]
    pub indirectwritexfernumbytes:
        crate::Reg<indirectwritexfernumbytes::INDIRECTWRITEXFERNUMBYTES_SPEC>,
    #[doc = "0x80 - Indirect Trigger Address Range Register"]
    pub indirecttriggeraddrrange:
        crate::Reg<indirecttriggeraddrrange::INDIRECTTRIGGERADDRRANGE_SPEC>,
    _reserved29: [u8; 0x08],
    #[doc = "0x8c - Flash Command Control Memory Register (STIG)"]
    pub flashcommandctrlmem: crate::Reg<flashcommandctrlmem::FLASHCOMMANDCTRLMEM_SPEC>,
    #[doc = "0x90 - Flash Command Control Register (STIG)"]
    pub flashcmdctrl: crate::Reg<flashcmdctrl::FLASHCMDCTRL_SPEC>,
    #[doc = "0x94 - Flash Command Address Register (STIG)"]
    pub flashcmdaddr: crate::Reg<flashcmdaddr::FLASHCMDADDR_SPEC>,
    _reserved32: [u8; 0x08],
    #[doc = "0xa0 - Flash Command Read Data Register (Lower) (STIG)"]
    pub flashrddatalower: crate::Reg<flashrddatalower::FLASHRDDATALOWER_SPEC>,
    #[doc = "0xa4 - Flash Command Read Data Register (Upper) (STIG)"]
    pub flashrddataupper: crate::Reg<flashrddataupper::FLASHRDDATAUPPER_SPEC>,
    #[doc = "0xa8 - Flash Command Write Data Register (Lower) (STIG)"]
    pub flashwrdatalower: crate::Reg<flashwrdatalower::FLASHWRDATALOWER_SPEC>,
    #[doc = "0xac - Flash Command Write Data Register (Upper) (STIG)"]
    pub flashwrdataupper: crate::Reg<flashwrdataupper::FLASHWRDATAUPPER_SPEC>,
    #[doc = "0xb0 - Polling Flash Status Register"]
    pub pollingflashstatus: crate::Reg<pollingflashstatus::POLLINGFLASHSTATUS_SPEC>,
    #[doc = "0xb4 - PHY Configuration Register"]
    pub phyconfiguration: crate::Reg<phyconfiguration::PHYCONFIGURATION_SPEC>,
    _reserved38: [u8; 0x28],
    #[doc = "0xe0 - Opcode Extension Register (Lower)"]
    pub opcodeextlower: crate::Reg<opcodeextlower::OPCODEEXTLOWER_SPEC>,
    #[doc = "0xe4 - Opcode Extension Register (Upper)"]
    pub opcodeextupper: crate::Reg<opcodeextupper::OPCODEEXTUPPER_SPEC>,
    _reserved40: [u8; 0x14],
    #[doc = "0xfc - Module ID Register"]
    pub moduleid: crate::Reg<moduleid::MODULEID_SPEC>,
    _reserved41: [u8; 0x04],
    #[doc = "0x104 - I/O Routing Pin Enable Register"]
    pub routepen: crate::Reg<routepen::ROUTEPEN_SPEC>,
    #[doc = "0x108 - I/O Route Location Register 0"]
    pub routeloc0: crate::Reg<routeloc0::ROUTELOC0_SPEC>,
}
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Octal-SPI Configuration Register"]
pub mod config;
#[doc = "DEVINSTRRDCONFIG register accessor: an alias for `Reg<DEVINSTRRDCONFIG_SPEC>`"]
pub type DEVINSTRRDCONFIG = crate::Reg<devinstrrdconfig::DEVINSTRRDCONFIG_SPEC>;
#[doc = "Device Read Instruction Configuration Register"]
pub mod devinstrrdconfig;
#[doc = "DEVINSTRWRCONFIG register accessor: an alias for `Reg<DEVINSTRWRCONFIG_SPEC>`"]
pub type DEVINSTRWRCONFIG = crate::Reg<devinstrwrconfig::DEVINSTRWRCONFIG_SPEC>;
#[doc = "Device Write Instruction Configuration Register"]
pub mod devinstrwrconfig;
#[doc = "DEVDELAY register accessor: an alias for `Reg<DEVDELAY_SPEC>`"]
pub type DEVDELAY = crate::Reg<devdelay::DEVDELAY_SPEC>;
#[doc = "Device Delay Register"]
pub mod devdelay;
#[doc = "RDDATACAPTURE register accessor: an alias for `Reg<RDDATACAPTURE_SPEC>`"]
pub type RDDATACAPTURE = crate::Reg<rddatacapture::RDDATACAPTURE_SPEC>;
#[doc = "Read Data Capture Register"]
pub mod rddatacapture;
#[doc = "DEVSIZECONFIG register accessor: an alias for `Reg<DEVSIZECONFIG_SPEC>`"]
pub type DEVSIZECONFIG = crate::Reg<devsizeconfig::DEVSIZECONFIG_SPEC>;
#[doc = "Device Size Configuration Register"]
pub mod devsizeconfig;
#[doc = "SRAMPARTITIONCFG register accessor: an alias for `Reg<SRAMPARTITIONCFG_SPEC>`"]
pub type SRAMPARTITIONCFG = crate::Reg<srampartitioncfg::SRAMPARTITIONCFG_SPEC>;
#[doc = "SRAM Partition Configuration Register"]
pub mod srampartitioncfg;
#[doc = "INDAHBADDRTRIGGER register accessor: an alias for `Reg<INDAHBADDRTRIGGER_SPEC>`"]
pub type INDAHBADDRTRIGGER = crate::Reg<indahbaddrtrigger::INDAHBADDRTRIGGER_SPEC>;
#[doc = "Indirect Address Trigger Register"]
pub mod indahbaddrtrigger;
#[doc = "REMAPADDR register accessor: an alias for `Reg<REMAPADDR_SPEC>`"]
pub type REMAPADDR = crate::Reg<remapaddr::REMAPADDR_SPEC>;
#[doc = "Remap Address Register"]
pub mod remapaddr;
#[doc = "MODEBITCONFIG register accessor: an alias for `Reg<MODEBITCONFIG_SPEC>`"]
pub type MODEBITCONFIG = crate::Reg<modebitconfig::MODEBITCONFIG_SPEC>;
#[doc = "Mode Bit Configuration Register"]
pub mod modebitconfig;
#[doc = "SRAMFILL register accessor: an alias for `Reg<SRAMFILL_SPEC>`"]
pub type SRAMFILL = crate::Reg<sramfill::SRAMFILL_SPEC>;
#[doc = "SRAM Fill Register"]
pub mod sramfill;
#[doc = "TXTHRESH register accessor: an alias for `Reg<TXTHRESH_SPEC>`"]
pub type TXTHRESH = crate::Reg<txthresh::TXTHRESH_SPEC>;
#[doc = "TX Threshold Register"]
pub mod txthresh;
#[doc = "RXTHRESH register accessor: an alias for `Reg<RXTHRESH_SPEC>`"]
pub type RXTHRESH = crate::Reg<rxthresh::RXTHRESH_SPEC>;
#[doc = "RX Threshold Register"]
pub mod rxthresh;
#[doc = "WRITECOMPLETIONCTRL register accessor: an alias for `Reg<WRITECOMPLETIONCTRL_SPEC>`"]
pub type WRITECOMPLETIONCTRL = crate::Reg<writecompletionctrl::WRITECOMPLETIONCTRL_SPEC>;
#[doc = "Write Completion Control Register"]
pub mod writecompletionctrl;
#[doc = "NOOFPOLLSBEFEXP register accessor: an alias for `Reg<NOOFPOLLSBEFEXP_SPEC>`"]
pub type NOOFPOLLSBEFEXP = crate::Reg<noofpollsbefexp::NOOFPOLLSBEFEXP_SPEC>;
#[doc = "Polling Expiration Register"]
pub mod noofpollsbefexp;
#[doc = "IRQSTATUS register accessor: an alias for `Reg<IRQSTATUS_SPEC>`"]
pub type IRQSTATUS = crate::Reg<irqstatus::IRQSTATUS_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod irqstatus;
#[doc = "IRQMASK register accessor: an alias for `Reg<IRQMASK_SPEC>`"]
pub type IRQMASK = crate::Reg<irqmask::IRQMASK_SPEC>;
#[doc = "Interrupt Mask"]
pub mod irqmask;
#[doc = "LOWERWRPROT register accessor: an alias for `Reg<LOWERWRPROT_SPEC>`"]
pub type LOWERWRPROT = crate::Reg<lowerwrprot::LOWERWRPROT_SPEC>;
#[doc = "Lower Write Protection Register"]
pub mod lowerwrprot;
#[doc = "UPPERWRPROT register accessor: an alias for `Reg<UPPERWRPROT_SPEC>`"]
pub type UPPERWRPROT = crate::Reg<upperwrprot::UPPERWRPROT_SPEC>;
#[doc = "Upper Write Protection Register"]
pub mod upperwrprot;
#[doc = "WRPROTCTRL register accessor: an alias for `Reg<WRPROTCTRL_SPEC>`"]
pub type WRPROTCTRL = crate::Reg<wrprotctrl::WRPROTCTRL_SPEC>;
#[doc = "Write Protection Control Register"]
pub mod wrprotctrl;
#[doc = "INDIRECTREADXFERCTRL register accessor: an alias for `Reg<INDIRECTREADXFERCTRL_SPEC>`"]
pub type INDIRECTREADXFERCTRL = crate::Reg<indirectreadxferctrl::INDIRECTREADXFERCTRL_SPEC>;
#[doc = "Indirect Read Transfer Control Register"]
pub mod indirectreadxferctrl;
#[doc = "INDIRECTREADXFERWATERMARK register accessor: an alias for `Reg<INDIRECTREADXFERWATERMARK_SPEC>`"]
pub type INDIRECTREADXFERWATERMARK =
    crate::Reg<indirectreadxferwatermark::INDIRECTREADXFERWATERMARK_SPEC>;
#[doc = "Indirect Read Transfer Watermark Register"]
pub mod indirectreadxferwatermark;
#[doc = "INDIRECTREADXFERSTART register accessor: an alias for `Reg<INDIRECTREADXFERSTART_SPEC>`"]
pub type INDIRECTREADXFERSTART = crate::Reg<indirectreadxferstart::INDIRECTREADXFERSTART_SPEC>;
#[doc = "Indirect Read Transfer Start Address Register"]
pub mod indirectreadxferstart;
#[doc = "INDIRECTREADXFERNUMBYTES register accessor: an alias for `Reg<INDIRECTREADXFERNUMBYTES_SPEC>`"]
pub type INDIRECTREADXFERNUMBYTES =
    crate::Reg<indirectreadxfernumbytes::INDIRECTREADXFERNUMBYTES_SPEC>;
#[doc = "Indirect Read Transfer Number Bytes Register"]
pub mod indirectreadxfernumbytes;
#[doc = "INDIRECTWRITEXFERCTRL register accessor: an alias for `Reg<INDIRECTWRITEXFERCTRL_SPEC>`"]
pub type INDIRECTWRITEXFERCTRL = crate::Reg<indirectwritexferctrl::INDIRECTWRITEXFERCTRL_SPEC>;
#[doc = "Indirect Write Transfer Control Register"]
pub mod indirectwritexferctrl;
#[doc = "INDIRECTWRITEXFERWATERMARK register accessor: an alias for `Reg<INDIRECTWRITEXFERWATERMARK_SPEC>`"]
pub type INDIRECTWRITEXFERWATERMARK =
    crate::Reg<indirectwritexferwatermark::INDIRECTWRITEXFERWATERMARK_SPEC>;
#[doc = "Indirect Write Transfer Watermark Register"]
pub mod indirectwritexferwatermark;
#[doc = "INDIRECTWRITEXFERSTART register accessor: an alias for `Reg<INDIRECTWRITEXFERSTART_SPEC>`"]
pub type INDIRECTWRITEXFERSTART = crate::Reg<indirectwritexferstart::INDIRECTWRITEXFERSTART_SPEC>;
#[doc = "Indirect Write Transfer Start Address Register"]
pub mod indirectwritexferstart;
#[doc = "INDIRECTWRITEXFERNUMBYTES register accessor: an alias for `Reg<INDIRECTWRITEXFERNUMBYTES_SPEC>`"]
pub type INDIRECTWRITEXFERNUMBYTES =
    crate::Reg<indirectwritexfernumbytes::INDIRECTWRITEXFERNUMBYTES_SPEC>;
#[doc = "Indirect Write Transfer Number Bytes Register"]
pub mod indirectwritexfernumbytes;
#[doc = "INDIRECTTRIGGERADDRRANGE register accessor: an alias for `Reg<INDIRECTTRIGGERADDRRANGE_SPEC>`"]
pub type INDIRECTTRIGGERADDRRANGE =
    crate::Reg<indirecttriggeraddrrange::INDIRECTTRIGGERADDRRANGE_SPEC>;
#[doc = "Indirect Trigger Address Range Register"]
pub mod indirecttriggeraddrrange;
#[doc = "FLASHCOMMANDCTRLMEM register accessor: an alias for `Reg<FLASHCOMMANDCTRLMEM_SPEC>`"]
pub type FLASHCOMMANDCTRLMEM = crate::Reg<flashcommandctrlmem::FLASHCOMMANDCTRLMEM_SPEC>;
#[doc = "Flash Command Control Memory Register (STIG)"]
pub mod flashcommandctrlmem;
#[doc = "FLASHCMDCTRL register accessor: an alias for `Reg<FLASHCMDCTRL_SPEC>`"]
pub type FLASHCMDCTRL = crate::Reg<flashcmdctrl::FLASHCMDCTRL_SPEC>;
#[doc = "Flash Command Control Register (STIG)"]
pub mod flashcmdctrl;
#[doc = "FLASHCMDADDR register accessor: an alias for `Reg<FLASHCMDADDR_SPEC>`"]
pub type FLASHCMDADDR = crate::Reg<flashcmdaddr::FLASHCMDADDR_SPEC>;
#[doc = "Flash Command Address Register (STIG)"]
pub mod flashcmdaddr;
#[doc = "FLASHRDDATALOWER register accessor: an alias for `Reg<FLASHRDDATALOWER_SPEC>`"]
pub type FLASHRDDATALOWER = crate::Reg<flashrddatalower::FLASHRDDATALOWER_SPEC>;
#[doc = "Flash Command Read Data Register (Lower) (STIG)"]
pub mod flashrddatalower;
#[doc = "FLASHRDDATAUPPER register accessor: an alias for `Reg<FLASHRDDATAUPPER_SPEC>`"]
pub type FLASHRDDATAUPPER = crate::Reg<flashrddataupper::FLASHRDDATAUPPER_SPEC>;
#[doc = "Flash Command Read Data Register (Upper) (STIG)"]
pub mod flashrddataupper;
#[doc = "FLASHWRDATALOWER register accessor: an alias for `Reg<FLASHWRDATALOWER_SPEC>`"]
pub type FLASHWRDATALOWER = crate::Reg<flashwrdatalower::FLASHWRDATALOWER_SPEC>;
#[doc = "Flash Command Write Data Register (Lower) (STIG)"]
pub mod flashwrdatalower;
#[doc = "FLASHWRDATAUPPER register accessor: an alias for `Reg<FLASHWRDATAUPPER_SPEC>`"]
pub type FLASHWRDATAUPPER = crate::Reg<flashwrdataupper::FLASHWRDATAUPPER_SPEC>;
#[doc = "Flash Command Write Data Register (Upper) (STIG)"]
pub mod flashwrdataupper;
#[doc = "POLLINGFLASHSTATUS register accessor: an alias for `Reg<POLLINGFLASHSTATUS_SPEC>`"]
pub type POLLINGFLASHSTATUS = crate::Reg<pollingflashstatus::POLLINGFLASHSTATUS_SPEC>;
#[doc = "Polling Flash Status Register"]
pub mod pollingflashstatus;
#[doc = "PHYCONFIGURATION register accessor: an alias for `Reg<PHYCONFIGURATION_SPEC>`"]
pub type PHYCONFIGURATION = crate::Reg<phyconfiguration::PHYCONFIGURATION_SPEC>;
#[doc = "PHY Configuration Register"]
pub mod phyconfiguration;
#[doc = "OPCODEEXTLOWER register accessor: an alias for `Reg<OPCODEEXTLOWER_SPEC>`"]
pub type OPCODEEXTLOWER = crate::Reg<opcodeextlower::OPCODEEXTLOWER_SPEC>;
#[doc = "Opcode Extension Register (Lower)"]
pub mod opcodeextlower;
#[doc = "OPCODEEXTUPPER register accessor: an alias for `Reg<OPCODEEXTUPPER_SPEC>`"]
pub type OPCODEEXTUPPER = crate::Reg<opcodeextupper::OPCODEEXTUPPER_SPEC>;
#[doc = "Opcode Extension Register (Upper)"]
pub mod opcodeextupper;
#[doc = "MODULEID register accessor: an alias for `Reg<MODULEID_SPEC>`"]
pub type MODULEID = crate::Reg<moduleid::MODULEID_SPEC>;
#[doc = "Module ID Register"]
pub mod moduleid;
#[doc = "ROUTEPEN register accessor: an alias for `Reg<ROUTEPEN_SPEC>`"]
pub type ROUTEPEN = crate::Reg<routepen::ROUTEPEN_SPEC>;
#[doc = "I/O Routing Pin Enable Register"]
pub mod routepen;
#[doc = "ROUTELOC0 register accessor: an alias for `Reg<ROUTELOC0_SPEC>`"]
pub type ROUTELOC0 = crate::Reg<routeloc0::ROUTELOC0_SPEC>;
#[doc = "I/O Route Location Register 0"]
pub mod routeloc0;
