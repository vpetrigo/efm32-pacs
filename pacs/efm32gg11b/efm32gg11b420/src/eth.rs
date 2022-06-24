#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Network control register"]
    pub networkctrl: crate::Reg<networkctrl::NETWORKCTRL_SPEC>,
    #[doc = "0x04 - Network configuration register"]
    pub networkcfg: crate::Reg<networkcfg::NETWORKCFG_SPEC>,
    #[doc = "0x08 - Network status register"]
    pub networkstatus: crate::Reg<networkstatus::NETWORKSTATUS_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - DMA Configuration Register"]
    pub dmacfg: crate::Reg<dmacfg::DMACFG_SPEC>,
    #[doc = "0x14 - Transmit status register"]
    pub txstatus: crate::Reg<txstatus::TXSTATUS_SPEC>,
    #[doc = "0x18 - Start address of the receive buffer queue"]
    pub rxqptr: crate::Reg<rxqptr::RXQPTR_SPEC>,
    #[doc = "0x1c - Start address of the transmit buffer queue"]
    pub txqptr: crate::Reg<txqptr::TXQPTR_SPEC>,
    #[doc = "0x20 - Receive status register"]
    pub rxstatus: crate::Reg<rxstatus::RXSTATUS_SPEC>,
    #[doc = "0x24 - Interrupt status register"]
    pub ifcr: crate::Reg<ifcr::IFCR_SPEC>,
    #[doc = "0x28 - Interrupt Enable Register"]
    pub iens: crate::Reg<iens::IENS_SPEC>,
    #[doc = "0x2c - Interrupt Disable Register"]
    pub ienc: crate::Reg<ienc::IENC_SPEC>,
    #[doc = "0x30 - Interrupt mask register"]
    pub ienro: crate::Reg<ienro::IENRO_SPEC>,
    #[doc = "0x34 - PHY management register"]
    pub phymngmnt: crate::Reg<phymngmnt::PHYMNGMNT_SPEC>,
    #[doc = "0x38 - Received Pause Quantum Register"]
    pub rxpausequant: crate::Reg<rxpausequant::RXPAUSEQUANT_SPEC>,
    #[doc = "0x3c - Transmit Pause Quantum Register"]
    pub txpausequant: crate::Reg<txpausequant::TXPAUSEQUANT_SPEC>,
    #[doc = "0x40 - TX Partial Store and Forward"]
    pub pbuftxcutthru: crate::Reg<pbuftxcutthru::PBUFTXCUTTHRU_SPEC>,
    #[doc = "0x44 - RX Partial Store and Forward"]
    pub pbufrxcutthru: crate::Reg<pbufrxcutthru::PBUFRXCUTTHRU_SPEC>,
    #[doc = "0x48 - Maximum Jumbo Frame Size."]
    pub jumbomaxlen: crate::Reg<jumbomaxlen::JUMBOMAXLEN_SPEC>,
    _reserved18: [u8; 0x10],
    #[doc = "0x5c - Interrupt moderation register"]
    pub imod: crate::Reg<imod::IMOD_SPEC>,
    #[doc = "0x60 - System wake time"]
    pub syswaketime: crate::Reg<syswaketime::SYSWAKETIME_SPEC>,
    _reserved20: [u8; 0x1c],
    #[doc = "0x80 - Hash Register Bottom \\[31:0\\]"]
    pub hashbottom: crate::Reg<hashbottom::HASHBOTTOM_SPEC>,
    #[doc = "0x84 - Hash Register Top \\[63:32\\]"]
    pub hashtop: crate::Reg<hashtop::HASHTOP_SPEC>,
    #[doc = "0x88 - Specific Address 1 Bottom"]
    pub specaddr1bottom: crate::Reg<specaddr1bottom::SPECADDR1BOTTOM_SPEC>,
    #[doc = "0x8c - Specific Address 1 Top"]
    pub specaddr1top: crate::Reg<specaddr1top::SPECADDR1TOP_SPEC>,
    #[doc = "0x90 - Specific Address 2 Bottom"]
    pub specaddr2bottom: crate::Reg<specaddr2bottom::SPECADDR2BOTTOM_SPEC>,
    #[doc = "0x94 - Specific Address 2 Top"]
    pub specaddr2top: crate::Reg<specaddr2top::SPECADDR2TOP_SPEC>,
    #[doc = "0x98 - Specific Address 3 Bottom"]
    pub specaddr3bottom: crate::Reg<specaddr3bottom::SPECADDR3BOTTOM_SPEC>,
    #[doc = "0x9c - Specific Address 3 Top"]
    pub specaddr3top: crate::Reg<specaddr3top::SPECADDR3TOP_SPEC>,
    #[doc = "0xa0 - Specific Address 4 Bottom"]
    pub specaddr4bottom: crate::Reg<specaddr4bottom::SPECADDR4BOTTOM_SPEC>,
    #[doc = "0xa4 - Specific Address 4 Top"]
    pub specaddr4top: crate::Reg<specaddr4top::SPECADDR4TOP_SPEC>,
    #[doc = "0xa8 - Type ID Match 1"]
    pub spectype1: crate::Reg<spectype1::SPECTYPE1_SPEC>,
    #[doc = "0xac - Type ID Match 2"]
    pub spectype2: crate::Reg<spectype2::SPECTYPE2_SPEC>,
    #[doc = "0xb0 - Type ID Match 3"]
    pub spectype3: crate::Reg<spectype3::SPECTYPE3_SPEC>,
    #[doc = "0xb4 - Type ID Match 4"]
    pub spectype4: crate::Reg<spectype4::SPECTYPE4_SPEC>,
    #[doc = "0xb8 - Wake on LAN Register"]
    pub wolreg: crate::Reg<wolreg::WOLREG_SPEC>,
    #[doc = "0xbc - IPG stretch register"]
    pub stretchratio: crate::Reg<stretchratio::STRETCHRATIO_SPEC>,
    #[doc = "0xc0 - Stacked VLAN Register"]
    pub stackedvlan: crate::Reg<stackedvlan::STACKEDVLAN_SPEC>,
    #[doc = "0xc4 - Transmit PFC Pause Register"]
    pub txpfcpause: crate::Reg<txpfcpause::TXPFCPAUSE_SPEC>,
    #[doc = "0xc8 - Specific Address Mask 1 Bottom 31:0"]
    pub maskadd1bottom: crate::Reg<maskadd1bottom::MASKADD1BOTTOM_SPEC>,
    #[doc = "0xcc - Specific Address Mask 1 Top 47:32"]
    pub maskadd1top: crate::Reg<maskadd1top::MASKADD1TOP_SPEC>,
    _reserved40: [u8; 0x04],
    #[doc = "0xd4 - PTP RX unicast IP destination address"]
    pub rxptpunicast: crate::Reg<rxptpunicast::RXPTPUNICAST_SPEC>,
    #[doc = "0xd8 - PTP TX unicast IP destination address"]
    pub txptpunicast: crate::Reg<txptpunicast::TXPTPUNICAST_SPEC>,
    #[doc = "0xdc - TSU timer comparison value nanoseconds"]
    pub tsunseccmp: crate::Reg<tsunseccmp::TSUNSECCMP_SPEC>,
    #[doc = "0xe0 - TSU timer comparison value seconds \\[31:0\\]"]
    pub tsuseccmp: crate::Reg<tsuseccmp::TSUSECCMP_SPEC>,
    #[doc = "0xe4 - TSU timer comparison value seconds \\[47:32\\]"]
    pub tsumsbseccmp: crate::Reg<tsumsbseccmp::TSUMSBSECCMP_SPEC>,
    #[doc = "0xe8 - PTP Event Frame Transmitted Seconds Register 47:32"]
    pub tsuptptxmsbsec: crate::Reg<tsuptptxmsbsec::TSUPTPTXMSBSEC_SPEC>,
    #[doc = "0xec - PTP Event Frame Received Seconds Register 47:32"]
    pub tsuptprxmsbsec: crate::Reg<tsuptprxmsbsec::TSUPTPRXMSBSEC_SPEC>,
    #[doc = "0xf0 - PTP Peer Event Frame Transmitted Seconds Register 47:32"]
    pub tsupeertxmsbsec: crate::Reg<tsupeertxmsbsec::TSUPEERTXMSBSEC_SPEC>,
    #[doc = "0xf4 - PTP Peer Event Frame Received Seconds Register 47:32"]
    pub tsupeerrxmsbsec: crate::Reg<tsupeerrxmsbsec::TSUPEERRXMSBSEC_SPEC>,
    _reserved49: [u8; 0x08],
    #[doc = "0x100 - Octets transmitted 31:0"]
    pub octetstxedbottom: crate::Reg<octetstxedbottom::OCTETSTXEDBOTTOM_SPEC>,
    #[doc = "0x104 - Octets Transmitted 47:32"]
    pub octetstxedtop: crate::Reg<octetstxedtop::OCTETSTXEDTOP_SPEC>,
    #[doc = "0x108 - Frames Transmitted"]
    pub framestxedok: crate::Reg<framestxedok::FRAMESTXEDOK_SPEC>,
    #[doc = "0x10c - Broadcast Frames Transmitted"]
    pub broadcasttxed: crate::Reg<broadcasttxed::BROADCASTTXED_SPEC>,
    #[doc = "0x110 - Multicast Frames Transmitted"]
    pub multicasttxed: crate::Reg<multicasttxed::MULTICASTTXED_SPEC>,
    #[doc = "0x114 - Pause Frames Transmitted"]
    pub pframestxed: crate::Reg<pframestxed::PFRAMESTXED_SPEC>,
    #[doc = "0x118 - 64 Byte Frames Transmitted"]
    pub framestxed64: crate::Reg<framestxed64::FRAMESTXED64_SPEC>,
    #[doc = "0x11c - 65 to 127 Byte Frames Transmitted"]
    pub framestxed65: crate::Reg<framestxed65::FRAMESTXED65_SPEC>,
    #[doc = "0x120 - 128 to 255 Byte Frames Transmitted"]
    pub framestxed128: crate::Reg<framestxed128::FRAMESTXED128_SPEC>,
    #[doc = "0x124 - 256 to 511 Byte Frames Transmitted"]
    pub framestxed256: crate::Reg<framestxed256::FRAMESTXED256_SPEC>,
    #[doc = "0x128 - 512 to 1023 Byte Frames Transmitted"]
    pub framestxed512: crate::Reg<framestxed512::FRAMESTXED512_SPEC>,
    #[doc = "0x12c - 1024 to 1518 Byte Frames Transmitted"]
    pub framestxed1024: crate::Reg<framestxed1024::FRAMESTXED1024_SPEC>,
    #[doc = "0x130 - Greater Than 1518 Byte Frames Transmitted"]
    pub framestxed1519: crate::Reg<framestxed1519::FRAMESTXED1519_SPEC>,
    #[doc = "0x134 - Transmit Under Runs"]
    pub txunderruns: crate::Reg<txunderruns::TXUNDERRUNS_SPEC>,
    #[doc = "0x138 - Single Collision Frames"]
    pub singlecols: crate::Reg<singlecols::SINGLECOLS_SPEC>,
    #[doc = "0x13c - Multiple Collision Frames"]
    pub multicols: crate::Reg<multicols::MULTICOLS_SPEC>,
    #[doc = "0x140 - Excessive Collisions"]
    pub excesscols: crate::Reg<excesscols::EXCESSCOLS_SPEC>,
    #[doc = "0x144 - Late Collisions"]
    pub latecols: crate::Reg<latecols::LATECOLS_SPEC>,
    #[doc = "0x148 - Deferred Transmission Frames"]
    pub deferredframes: crate::Reg<deferredframes::DEFERREDFRAMES_SPEC>,
    #[doc = "0x14c - Carrier Sense Errors"]
    pub crserrs: crate::Reg<crserrs::CRSERRS_SPEC>,
    #[doc = "0x150 - Octets Received 31:0"]
    pub octetsrxedbottom: crate::Reg<octetsrxedbottom::OCTETSRXEDBOTTOM_SPEC>,
    #[doc = "0x154 - Octets Received 47:32"]
    pub octetsrxedtop: crate::Reg<octetsrxedtop::OCTETSRXEDTOP_SPEC>,
    #[doc = "0x158 - Frames Received"]
    pub framesrxedok: crate::Reg<framesrxedok::FRAMESRXEDOK_SPEC>,
    #[doc = "0x15c - Broadcast Frames Received"]
    pub broadcastrxed: crate::Reg<broadcastrxed::BROADCASTRXED_SPEC>,
    #[doc = "0x160 - Multicast Frames Received"]
    pub multicastrxed: crate::Reg<multicastrxed::MULTICASTRXED_SPEC>,
    #[doc = "0x164 - Pause Frames Received"]
    pub pframesrxed: crate::Reg<pframesrxed::PFRAMESRXED_SPEC>,
    #[doc = "0x168 - 64 Byte Frames Received"]
    pub framesrxed64: crate::Reg<framesrxed64::FRAMESRXED64_SPEC>,
    #[doc = "0x16c - 65 to 127 Byte Frames Received"]
    pub framesrxed65: crate::Reg<framesrxed65::FRAMESRXED65_SPEC>,
    #[doc = "0x170 - 128 to 255 Byte Frames Received"]
    pub framesrxed128: crate::Reg<framesrxed128::FRAMESRXED128_SPEC>,
    #[doc = "0x174 - 256 to 511 Byte Frames Received"]
    pub framesrxed256: crate::Reg<framesrxed256::FRAMESRXED256_SPEC>,
    #[doc = "0x178 - 512 to 1023 Byte Frames Received"]
    pub framesrxed512: crate::Reg<framesrxed512::FRAMESRXED512_SPEC>,
    #[doc = "0x17c - 1024 to 1518 Byte Frames Received"]
    pub framesrxed1024: crate::Reg<framesrxed1024::FRAMESRXED1024_SPEC>,
    #[doc = "0x180 - 1519 to maximum Byte Frames Received"]
    pub framesrxed1519: crate::Reg<framesrxed1519::FRAMESRXED1519_SPEC>,
    #[doc = "0x184 - Undersized Frames Received"]
    pub undersizeframes: crate::Reg<undersizeframes::UNDERSIZEFRAMES_SPEC>,
    #[doc = "0x188 - Oversize Frames Received"]
    pub excessiverxlen: crate::Reg<excessiverxlen::EXCESSIVERXLEN_SPEC>,
    #[doc = "0x18c - Jabbers Received"]
    pub rxjabbers: crate::Reg<rxjabbers::RXJABBERS_SPEC>,
    #[doc = "0x190 - Frame Check Sequence Errors"]
    pub fcserrs: crate::Reg<fcserrs::FCSERRS_SPEC>,
    #[doc = "0x194 - Length Field Frame Errors"]
    pub rxlenerrs: crate::Reg<rxlenerrs::RXLENERRS_SPEC>,
    #[doc = "0x198 - Receive Symbol Errors"]
    pub rxsymbolerrs: crate::Reg<rxsymbolerrs::RXSYMBOLERRS_SPEC>,
    #[doc = "0x19c - Alignment Errors"]
    pub alignerrs: crate::Reg<alignerrs::ALIGNERRS_SPEC>,
    #[doc = "0x1a0 - Receive Resource Errors"]
    pub rxresourceerrs: crate::Reg<rxresourceerrs::RXRESOURCEERRS_SPEC>,
    #[doc = "0x1a4 - Receive Overruns"]
    pub rxoverruns: crate::Reg<rxoverruns::RXOVERRUNS_SPEC>,
    #[doc = "0x1a8 - IP Header Checksum Errors"]
    pub rxipckerrs: crate::Reg<rxipckerrs::RXIPCKERRS_SPEC>,
    #[doc = "0x1ac - TCP Checksum Errors"]
    pub rxtcpckerrs: crate::Reg<rxtcpckerrs::RXTCPCKERRS_SPEC>,
    #[doc = "0x1b0 - UDP Checksum Errors"]
    pub rxudpckerrs: crate::Reg<rxudpckerrs::RXUDPCKERRS_SPEC>,
    #[doc = "0x1b4 - Receive DMA Flushed Packets"]
    pub autoflushedpkts: crate::Reg<autoflushedpkts::AUTOFLUSHEDPKTS_SPEC>,
    _reserved95: [u8; 0x04],
    #[doc = "0x1bc - 1588 Timer Increment Register subscript nsec"]
    pub tsutimerincrsubnsec: crate::Reg<tsutimerincrsubnsec::TSUTIMERINCRSUBNSEC_SPEC>,
    #[doc = "0x1c0 - 1588 Timer Seconds Register 47:32"]
    pub tsutimermsbsec: crate::Reg<tsutimermsbsec::TSUTIMERMSBSEC_SPEC>,
    _reserved97: [u8; 0x0c],
    #[doc = "0x1d0 - 1588 Timer Seconds Register 31:0"]
    pub tsutimersec: crate::Reg<tsutimersec::TSUTIMERSEC_SPEC>,
    #[doc = "0x1d4 - 1588 Timer Nanoseconds Register"]
    pub tsutimernsec: crate::Reg<tsutimernsec::TSUTIMERNSEC_SPEC>,
    #[doc = "0x1d8 - This register returns all zeroes when read."]
    pub tsutimeradjust: crate::Reg<tsutimeradjust::TSUTIMERADJUST_SPEC>,
    #[doc = "0x1dc - 1588 Timer Increment Register"]
    pub tsutimerincr: crate::Reg<tsutimerincr::TSUTIMERINCR_SPEC>,
    #[doc = "0x1e0 - PTP Event Frame Transmitted Seconds Register 31:0"]
    pub tsuptptxsec: crate::Reg<tsuptptxsec::TSUPTPTXSEC_SPEC>,
    #[doc = "0x1e4 - PTP Event Frame Transmitted Nanoseconds Register"]
    pub tsuptptxnsec: crate::Reg<tsuptptxnsec::TSUPTPTXNSEC_SPEC>,
    #[doc = "0x1e8 - PTP Event Frame Received Seconds Register 31:0"]
    pub tsuptprxsec: crate::Reg<tsuptprxsec::TSUPTPRXSEC_SPEC>,
    #[doc = "0x1ec - PTP Event Frame Received Nanoseconds Register"]
    pub tsuptprxnsec: crate::Reg<tsuptprxnsec::TSUPTPRXNSEC_SPEC>,
    #[doc = "0x1f0 - PTP Peer Event Frame Transmitted Seconds Register 31:0"]
    pub tsupeertxsec: crate::Reg<tsupeertxsec::TSUPEERTXSEC_SPEC>,
    #[doc = "0x1f4 - PTP Peer Event Frame Transmitted Nanoseconds Register"]
    pub tsupeertxnsec: crate::Reg<tsupeertxnsec::TSUPEERTXNSEC_SPEC>,
    #[doc = "0x1f8 - PTP Peer Event Frame Received Seconds Register 31:0"]
    pub tsupeerrxsec: crate::Reg<tsupeerrxsec::TSUPEERRXSEC_SPEC>,
    #[doc = "0x1fc - PTP Peer Event Frame Received Nanoseconds Register"]
    pub tsupeerrxnsec: crate::Reg<tsupeerrxnsec::TSUPEERRXNSEC_SPEC>,
    _reserved109: [u8; 0x60],
    #[doc = "0x260 - Transmit Pause Quantum Register 1"]
    pub txpausequant1: crate::Reg<txpausequant1::TXPAUSEQUANT1_SPEC>,
    #[doc = "0x264 - Transmit Pause Quantum Register 2"]
    pub txpausequant2: crate::Reg<txpausequant2::TXPAUSEQUANT2_SPEC>,
    #[doc = "0x268 - Transmit Pause Quantum Register 3"]
    pub txpausequant3: crate::Reg<txpausequant3::TXPAUSEQUANT3_SPEC>,
    _reserved112: [u8; 0x04],
    #[doc = "0x270 - Received LPI transitions"]
    pub rxlpi: crate::Reg<rxlpi::RXLPI_SPEC>,
    #[doc = "0x274 - Received LPI time"]
    pub rxlpitime: crate::Reg<rxlpitime::RXLPITIME_SPEC>,
    #[doc = "0x278 - Transmit LPI transitions"]
    pub txlpi: crate::Reg<txlpi::TXLPI_SPEC>,
    #[doc = "0x27c - Transmit LPI time"]
    pub txlpitime: crate::Reg<txlpitime::TXLPITIME_SPEC>,
    _reserved116: [u8; 0x024c],
    #[doc = "0x4cc - TX BD control register"]
    pub txbdctrl: crate::Reg<txbdctrl::TXBDCTRL_SPEC>,
    #[doc = "0x4d0 - RX BD control register"]
    pub rxbdctrl: crate::Reg<rxbdctrl::RXBDCTRL_SPEC>,
    _reserved118: [u8; 0x072c],
    #[doc = "0xc00 - I/O Route Enable Register"]
    pub routepen: crate::Reg<routepen::ROUTEPEN_SPEC>,
    #[doc = "0xc04 - I/O Route Location Register 0"]
    pub routeloc0: crate::Reg<routeloc0::ROUTELOC0_SPEC>,
    _reserved120: [u8; 0x04],
    #[doc = "0xc0c - I/O Route Location Register 1"]
    pub routeloc1: crate::Reg<routeloc1::ROUTELOC1_SPEC>,
    #[doc = "0xc10 - Ethernet control register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
}
#[doc = "NETWORKCTRL register accessor: an alias for `Reg<NETWORKCTRL_SPEC>`"]
pub type NETWORKCTRL = crate::Reg<networkctrl::NETWORKCTRL_SPEC>;
#[doc = "Network control register"]
pub mod networkctrl;
#[doc = "NETWORKCFG register accessor: an alias for `Reg<NETWORKCFG_SPEC>`"]
pub type NETWORKCFG = crate::Reg<networkcfg::NETWORKCFG_SPEC>;
#[doc = "Network configuration register"]
pub mod networkcfg;
#[doc = "NETWORKSTATUS register accessor: an alias for `Reg<NETWORKSTATUS_SPEC>`"]
pub type NETWORKSTATUS = crate::Reg<networkstatus::NETWORKSTATUS_SPEC>;
#[doc = "Network status register"]
pub mod networkstatus;
#[doc = "DMACFG register accessor: an alias for `Reg<DMACFG_SPEC>`"]
pub type DMACFG = crate::Reg<dmacfg::DMACFG_SPEC>;
#[doc = "DMA Configuration Register"]
pub mod dmacfg;
#[doc = "TXSTATUS register accessor: an alias for `Reg<TXSTATUS_SPEC>`"]
pub type TXSTATUS = crate::Reg<txstatus::TXSTATUS_SPEC>;
#[doc = "Transmit status register"]
pub mod txstatus;
#[doc = "RXQPTR register accessor: an alias for `Reg<RXQPTR_SPEC>`"]
pub type RXQPTR = crate::Reg<rxqptr::RXQPTR_SPEC>;
#[doc = "Start address of the receive buffer queue"]
pub mod rxqptr;
#[doc = "TXQPTR register accessor: an alias for `Reg<TXQPTR_SPEC>`"]
pub type TXQPTR = crate::Reg<txqptr::TXQPTR_SPEC>;
#[doc = "Start address of the transmit buffer queue"]
pub mod txqptr;
#[doc = "RXSTATUS register accessor: an alias for `Reg<RXSTATUS_SPEC>`"]
pub type RXSTATUS = crate::Reg<rxstatus::RXSTATUS_SPEC>;
#[doc = "Receive status register"]
pub mod rxstatus;
#[doc = "IFCR register accessor: an alias for `Reg<IFCR_SPEC>`"]
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
#[doc = "Interrupt status register"]
pub mod ifcr;
#[doc = "IENS register accessor: an alias for `Reg<IENS_SPEC>`"]
pub type IENS = crate::Reg<iens::IENS_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod iens;
#[doc = "IENC register accessor: an alias for `Reg<IENC_SPEC>`"]
pub type IENC = crate::Reg<ienc::IENC_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod ienc;
#[doc = "IENRO register accessor: an alias for `Reg<IENRO_SPEC>`"]
pub type IENRO = crate::Reg<ienro::IENRO_SPEC>;
#[doc = "Interrupt mask register"]
pub mod ienro;
#[doc = "PHYMNGMNT register accessor: an alias for `Reg<PHYMNGMNT_SPEC>`"]
pub type PHYMNGMNT = crate::Reg<phymngmnt::PHYMNGMNT_SPEC>;
#[doc = "PHY management register"]
pub mod phymngmnt;
#[doc = "RXPAUSEQUANT register accessor: an alias for `Reg<RXPAUSEQUANT_SPEC>`"]
pub type RXPAUSEQUANT = crate::Reg<rxpausequant::RXPAUSEQUANT_SPEC>;
#[doc = "Received Pause Quantum Register"]
pub mod rxpausequant;
#[doc = "TXPAUSEQUANT register accessor: an alias for `Reg<TXPAUSEQUANT_SPEC>`"]
pub type TXPAUSEQUANT = crate::Reg<txpausequant::TXPAUSEQUANT_SPEC>;
#[doc = "Transmit Pause Quantum Register"]
pub mod txpausequant;
#[doc = "PBUFTXCUTTHRU register accessor: an alias for `Reg<PBUFTXCUTTHRU_SPEC>`"]
pub type PBUFTXCUTTHRU = crate::Reg<pbuftxcutthru::PBUFTXCUTTHRU_SPEC>;
#[doc = "TX Partial Store and Forward"]
pub mod pbuftxcutthru;
#[doc = "PBUFRXCUTTHRU register accessor: an alias for `Reg<PBUFRXCUTTHRU_SPEC>`"]
pub type PBUFRXCUTTHRU = crate::Reg<pbufrxcutthru::PBUFRXCUTTHRU_SPEC>;
#[doc = "RX Partial Store and Forward"]
pub mod pbufrxcutthru;
#[doc = "JUMBOMAXLEN register accessor: an alias for `Reg<JUMBOMAXLEN_SPEC>`"]
pub type JUMBOMAXLEN = crate::Reg<jumbomaxlen::JUMBOMAXLEN_SPEC>;
#[doc = "Maximum Jumbo Frame Size."]
pub mod jumbomaxlen;
#[doc = "IMOD register accessor: an alias for `Reg<IMOD_SPEC>`"]
pub type IMOD = crate::Reg<imod::IMOD_SPEC>;
#[doc = "Interrupt moderation register"]
pub mod imod;
#[doc = "SYSWAKETIME register accessor: an alias for `Reg<SYSWAKETIME_SPEC>`"]
pub type SYSWAKETIME = crate::Reg<syswaketime::SYSWAKETIME_SPEC>;
#[doc = "System wake time"]
pub mod syswaketime;
#[doc = "HASHBOTTOM register accessor: an alias for `Reg<HASHBOTTOM_SPEC>`"]
pub type HASHBOTTOM = crate::Reg<hashbottom::HASHBOTTOM_SPEC>;
#[doc = "Hash Register Bottom \\[31:0\\]"]
pub mod hashbottom;
#[doc = "HASHTOP register accessor: an alias for `Reg<HASHTOP_SPEC>`"]
pub type HASHTOP = crate::Reg<hashtop::HASHTOP_SPEC>;
#[doc = "Hash Register Top \\[63:32\\]"]
pub mod hashtop;
#[doc = "SPECADDR1BOTTOM register accessor: an alias for `Reg<SPECADDR1BOTTOM_SPEC>`"]
pub type SPECADDR1BOTTOM = crate::Reg<specaddr1bottom::SPECADDR1BOTTOM_SPEC>;
#[doc = "Specific Address 1 Bottom"]
pub mod specaddr1bottom;
#[doc = "SPECADDR1TOP register accessor: an alias for `Reg<SPECADDR1TOP_SPEC>`"]
pub type SPECADDR1TOP = crate::Reg<specaddr1top::SPECADDR1TOP_SPEC>;
#[doc = "Specific Address 1 Top"]
pub mod specaddr1top;
#[doc = "SPECADDR2BOTTOM register accessor: an alias for `Reg<SPECADDR2BOTTOM_SPEC>`"]
pub type SPECADDR2BOTTOM = crate::Reg<specaddr2bottom::SPECADDR2BOTTOM_SPEC>;
#[doc = "Specific Address 2 Bottom"]
pub mod specaddr2bottom;
#[doc = "SPECADDR2TOP register accessor: an alias for `Reg<SPECADDR2TOP_SPEC>`"]
pub type SPECADDR2TOP = crate::Reg<specaddr2top::SPECADDR2TOP_SPEC>;
#[doc = "Specific Address 2 Top"]
pub mod specaddr2top;
#[doc = "SPECADDR3BOTTOM register accessor: an alias for `Reg<SPECADDR3BOTTOM_SPEC>`"]
pub type SPECADDR3BOTTOM = crate::Reg<specaddr3bottom::SPECADDR3BOTTOM_SPEC>;
#[doc = "Specific Address 3 Bottom"]
pub mod specaddr3bottom;
#[doc = "SPECADDR3TOP register accessor: an alias for `Reg<SPECADDR3TOP_SPEC>`"]
pub type SPECADDR3TOP = crate::Reg<specaddr3top::SPECADDR3TOP_SPEC>;
#[doc = "Specific Address 3 Top"]
pub mod specaddr3top;
#[doc = "SPECADDR4BOTTOM register accessor: an alias for `Reg<SPECADDR4BOTTOM_SPEC>`"]
pub type SPECADDR4BOTTOM = crate::Reg<specaddr4bottom::SPECADDR4BOTTOM_SPEC>;
#[doc = "Specific Address 4 Bottom"]
pub mod specaddr4bottom;
#[doc = "SPECADDR4TOP register accessor: an alias for `Reg<SPECADDR4TOP_SPEC>`"]
pub type SPECADDR4TOP = crate::Reg<specaddr4top::SPECADDR4TOP_SPEC>;
#[doc = "Specific Address 4 Top"]
pub mod specaddr4top;
#[doc = "SPECTYPE1 register accessor: an alias for `Reg<SPECTYPE1_SPEC>`"]
pub type SPECTYPE1 = crate::Reg<spectype1::SPECTYPE1_SPEC>;
#[doc = "Type ID Match 1"]
pub mod spectype1;
#[doc = "SPECTYPE2 register accessor: an alias for `Reg<SPECTYPE2_SPEC>`"]
pub type SPECTYPE2 = crate::Reg<spectype2::SPECTYPE2_SPEC>;
#[doc = "Type ID Match 2"]
pub mod spectype2;
#[doc = "SPECTYPE3 register accessor: an alias for `Reg<SPECTYPE3_SPEC>`"]
pub type SPECTYPE3 = crate::Reg<spectype3::SPECTYPE3_SPEC>;
#[doc = "Type ID Match 3"]
pub mod spectype3;
#[doc = "SPECTYPE4 register accessor: an alias for `Reg<SPECTYPE4_SPEC>`"]
pub type SPECTYPE4 = crate::Reg<spectype4::SPECTYPE4_SPEC>;
#[doc = "Type ID Match 4"]
pub mod spectype4;
#[doc = "WOLREG register accessor: an alias for `Reg<WOLREG_SPEC>`"]
pub type WOLREG = crate::Reg<wolreg::WOLREG_SPEC>;
#[doc = "Wake on LAN Register"]
pub mod wolreg;
#[doc = "STRETCHRATIO register accessor: an alias for `Reg<STRETCHRATIO_SPEC>`"]
pub type STRETCHRATIO = crate::Reg<stretchratio::STRETCHRATIO_SPEC>;
#[doc = "IPG stretch register"]
pub mod stretchratio;
#[doc = "STACKEDVLAN register accessor: an alias for `Reg<STACKEDVLAN_SPEC>`"]
pub type STACKEDVLAN = crate::Reg<stackedvlan::STACKEDVLAN_SPEC>;
#[doc = "Stacked VLAN Register"]
pub mod stackedvlan;
#[doc = "TXPFCPAUSE register accessor: an alias for `Reg<TXPFCPAUSE_SPEC>`"]
pub type TXPFCPAUSE = crate::Reg<txpfcpause::TXPFCPAUSE_SPEC>;
#[doc = "Transmit PFC Pause Register"]
pub mod txpfcpause;
#[doc = "MASKADD1BOTTOM register accessor: an alias for `Reg<MASKADD1BOTTOM_SPEC>`"]
pub type MASKADD1BOTTOM = crate::Reg<maskadd1bottom::MASKADD1BOTTOM_SPEC>;
#[doc = "Specific Address Mask 1 Bottom 31:0"]
pub mod maskadd1bottom;
#[doc = "MASKADD1TOP register accessor: an alias for `Reg<MASKADD1TOP_SPEC>`"]
pub type MASKADD1TOP = crate::Reg<maskadd1top::MASKADD1TOP_SPEC>;
#[doc = "Specific Address Mask 1 Top 47:32"]
pub mod maskadd1top;
#[doc = "RXPTPUNICAST register accessor: an alias for `Reg<RXPTPUNICAST_SPEC>`"]
pub type RXPTPUNICAST = crate::Reg<rxptpunicast::RXPTPUNICAST_SPEC>;
#[doc = "PTP RX unicast IP destination address"]
pub mod rxptpunicast;
#[doc = "TXPTPUNICAST register accessor: an alias for `Reg<TXPTPUNICAST_SPEC>`"]
pub type TXPTPUNICAST = crate::Reg<txptpunicast::TXPTPUNICAST_SPEC>;
#[doc = "PTP TX unicast IP destination address"]
pub mod txptpunicast;
#[doc = "TSUNSECCMP register accessor: an alias for `Reg<TSUNSECCMP_SPEC>`"]
pub type TSUNSECCMP = crate::Reg<tsunseccmp::TSUNSECCMP_SPEC>;
#[doc = "TSU timer comparison value nanoseconds"]
pub mod tsunseccmp;
#[doc = "TSUSECCMP register accessor: an alias for `Reg<TSUSECCMP_SPEC>`"]
pub type TSUSECCMP = crate::Reg<tsuseccmp::TSUSECCMP_SPEC>;
#[doc = "TSU timer comparison value seconds \\[31:0\\]"]
pub mod tsuseccmp;
#[doc = "TSUMSBSECCMP register accessor: an alias for `Reg<TSUMSBSECCMP_SPEC>`"]
pub type TSUMSBSECCMP = crate::Reg<tsumsbseccmp::TSUMSBSECCMP_SPEC>;
#[doc = "TSU timer comparison value seconds \\[47:32\\]"]
pub mod tsumsbseccmp;
#[doc = "TSUPTPTXMSBSEC register accessor: an alias for `Reg<TSUPTPTXMSBSEC_SPEC>`"]
pub type TSUPTPTXMSBSEC = crate::Reg<tsuptptxmsbsec::TSUPTPTXMSBSEC_SPEC>;
#[doc = "PTP Event Frame Transmitted Seconds Register 47:32"]
pub mod tsuptptxmsbsec;
#[doc = "TSUPTPRXMSBSEC register accessor: an alias for `Reg<TSUPTPRXMSBSEC_SPEC>`"]
pub type TSUPTPRXMSBSEC = crate::Reg<tsuptprxmsbsec::TSUPTPRXMSBSEC_SPEC>;
#[doc = "PTP Event Frame Received Seconds Register 47:32"]
pub mod tsuptprxmsbsec;
#[doc = "TSUPEERTXMSBSEC register accessor: an alias for `Reg<TSUPEERTXMSBSEC_SPEC>`"]
pub type TSUPEERTXMSBSEC = crate::Reg<tsupeertxmsbsec::TSUPEERTXMSBSEC_SPEC>;
#[doc = "PTP Peer Event Frame Transmitted Seconds Register 47:32"]
pub mod tsupeertxmsbsec;
#[doc = "TSUPEERRXMSBSEC register accessor: an alias for `Reg<TSUPEERRXMSBSEC_SPEC>`"]
pub type TSUPEERRXMSBSEC = crate::Reg<tsupeerrxmsbsec::TSUPEERRXMSBSEC_SPEC>;
#[doc = "PTP Peer Event Frame Received Seconds Register 47:32"]
pub mod tsupeerrxmsbsec;
#[doc = "OCTETSTXEDBOTTOM register accessor: an alias for `Reg<OCTETSTXEDBOTTOM_SPEC>`"]
pub type OCTETSTXEDBOTTOM = crate::Reg<octetstxedbottom::OCTETSTXEDBOTTOM_SPEC>;
#[doc = "Octets transmitted 31:0"]
pub mod octetstxedbottom;
#[doc = "OCTETSTXEDTOP register accessor: an alias for `Reg<OCTETSTXEDTOP_SPEC>`"]
pub type OCTETSTXEDTOP = crate::Reg<octetstxedtop::OCTETSTXEDTOP_SPEC>;
#[doc = "Octets Transmitted 47:32"]
pub mod octetstxedtop;
#[doc = "FRAMESTXEDOK register accessor: an alias for `Reg<FRAMESTXEDOK_SPEC>`"]
pub type FRAMESTXEDOK = crate::Reg<framestxedok::FRAMESTXEDOK_SPEC>;
#[doc = "Frames Transmitted"]
pub mod framestxedok;
#[doc = "BROADCASTTXED register accessor: an alias for `Reg<BROADCASTTXED_SPEC>`"]
pub type BROADCASTTXED = crate::Reg<broadcasttxed::BROADCASTTXED_SPEC>;
#[doc = "Broadcast Frames Transmitted"]
pub mod broadcasttxed;
#[doc = "MULTICASTTXED register accessor: an alias for `Reg<MULTICASTTXED_SPEC>`"]
pub type MULTICASTTXED = crate::Reg<multicasttxed::MULTICASTTXED_SPEC>;
#[doc = "Multicast Frames Transmitted"]
pub mod multicasttxed;
#[doc = "PFRAMESTXED register accessor: an alias for `Reg<PFRAMESTXED_SPEC>`"]
pub type PFRAMESTXED = crate::Reg<pframestxed::PFRAMESTXED_SPEC>;
#[doc = "Pause Frames Transmitted"]
pub mod pframestxed;
#[doc = "FRAMESTXED64 register accessor: an alias for `Reg<FRAMESTXED64_SPEC>`"]
pub type FRAMESTXED64 = crate::Reg<framestxed64::FRAMESTXED64_SPEC>;
#[doc = "64 Byte Frames Transmitted"]
pub mod framestxed64;
#[doc = "FRAMESTXED65 register accessor: an alias for `Reg<FRAMESTXED65_SPEC>`"]
pub type FRAMESTXED65 = crate::Reg<framestxed65::FRAMESTXED65_SPEC>;
#[doc = "65 to 127 Byte Frames Transmitted"]
pub mod framestxed65;
#[doc = "FRAMESTXED128 register accessor: an alias for `Reg<FRAMESTXED128_SPEC>`"]
pub type FRAMESTXED128 = crate::Reg<framestxed128::FRAMESTXED128_SPEC>;
#[doc = "128 to 255 Byte Frames Transmitted"]
pub mod framestxed128;
#[doc = "FRAMESTXED256 register accessor: an alias for `Reg<FRAMESTXED256_SPEC>`"]
pub type FRAMESTXED256 = crate::Reg<framestxed256::FRAMESTXED256_SPEC>;
#[doc = "256 to 511 Byte Frames Transmitted"]
pub mod framestxed256;
#[doc = "FRAMESTXED512 register accessor: an alias for `Reg<FRAMESTXED512_SPEC>`"]
pub type FRAMESTXED512 = crate::Reg<framestxed512::FRAMESTXED512_SPEC>;
#[doc = "512 to 1023 Byte Frames Transmitted"]
pub mod framestxed512;
#[doc = "FRAMESTXED1024 register accessor: an alias for `Reg<FRAMESTXED1024_SPEC>`"]
pub type FRAMESTXED1024 = crate::Reg<framestxed1024::FRAMESTXED1024_SPEC>;
#[doc = "1024 to 1518 Byte Frames Transmitted"]
pub mod framestxed1024;
#[doc = "FRAMESTXED1519 register accessor: an alias for `Reg<FRAMESTXED1519_SPEC>`"]
pub type FRAMESTXED1519 = crate::Reg<framestxed1519::FRAMESTXED1519_SPEC>;
#[doc = "Greater Than 1518 Byte Frames Transmitted"]
pub mod framestxed1519;
#[doc = "TXUNDERRUNS register accessor: an alias for `Reg<TXUNDERRUNS_SPEC>`"]
pub type TXUNDERRUNS = crate::Reg<txunderruns::TXUNDERRUNS_SPEC>;
#[doc = "Transmit Under Runs"]
pub mod txunderruns;
#[doc = "SINGLECOLS register accessor: an alias for `Reg<SINGLECOLS_SPEC>`"]
pub type SINGLECOLS = crate::Reg<singlecols::SINGLECOLS_SPEC>;
#[doc = "Single Collision Frames"]
pub mod singlecols;
#[doc = "MULTICOLS register accessor: an alias for `Reg<MULTICOLS_SPEC>`"]
pub type MULTICOLS = crate::Reg<multicols::MULTICOLS_SPEC>;
#[doc = "Multiple Collision Frames"]
pub mod multicols;
#[doc = "EXCESSCOLS register accessor: an alias for `Reg<EXCESSCOLS_SPEC>`"]
pub type EXCESSCOLS = crate::Reg<excesscols::EXCESSCOLS_SPEC>;
#[doc = "Excessive Collisions"]
pub mod excesscols;
#[doc = "LATECOLS register accessor: an alias for `Reg<LATECOLS_SPEC>`"]
pub type LATECOLS = crate::Reg<latecols::LATECOLS_SPEC>;
#[doc = "Late Collisions"]
pub mod latecols;
#[doc = "DEFERREDFRAMES register accessor: an alias for `Reg<DEFERREDFRAMES_SPEC>`"]
pub type DEFERREDFRAMES = crate::Reg<deferredframes::DEFERREDFRAMES_SPEC>;
#[doc = "Deferred Transmission Frames"]
pub mod deferredframes;
#[doc = "CRSERRS register accessor: an alias for `Reg<CRSERRS_SPEC>`"]
pub type CRSERRS = crate::Reg<crserrs::CRSERRS_SPEC>;
#[doc = "Carrier Sense Errors"]
pub mod crserrs;
#[doc = "OCTETSRXEDBOTTOM register accessor: an alias for `Reg<OCTETSRXEDBOTTOM_SPEC>`"]
pub type OCTETSRXEDBOTTOM = crate::Reg<octetsrxedbottom::OCTETSRXEDBOTTOM_SPEC>;
#[doc = "Octets Received 31:0"]
pub mod octetsrxedbottom;
#[doc = "OCTETSRXEDTOP register accessor: an alias for `Reg<OCTETSRXEDTOP_SPEC>`"]
pub type OCTETSRXEDTOP = crate::Reg<octetsrxedtop::OCTETSRXEDTOP_SPEC>;
#[doc = "Octets Received 47:32"]
pub mod octetsrxedtop;
#[doc = "FRAMESRXEDOK register accessor: an alias for `Reg<FRAMESRXEDOK_SPEC>`"]
pub type FRAMESRXEDOK = crate::Reg<framesrxedok::FRAMESRXEDOK_SPEC>;
#[doc = "Frames Received"]
pub mod framesrxedok;
#[doc = "BROADCASTRXED register accessor: an alias for `Reg<BROADCASTRXED_SPEC>`"]
pub type BROADCASTRXED = crate::Reg<broadcastrxed::BROADCASTRXED_SPEC>;
#[doc = "Broadcast Frames Received"]
pub mod broadcastrxed;
#[doc = "MULTICASTRXED register accessor: an alias for `Reg<MULTICASTRXED_SPEC>`"]
pub type MULTICASTRXED = crate::Reg<multicastrxed::MULTICASTRXED_SPEC>;
#[doc = "Multicast Frames Received"]
pub mod multicastrxed;
#[doc = "PFRAMESRXED register accessor: an alias for `Reg<PFRAMESRXED_SPEC>`"]
pub type PFRAMESRXED = crate::Reg<pframesrxed::PFRAMESRXED_SPEC>;
#[doc = "Pause Frames Received"]
pub mod pframesrxed;
#[doc = "FRAMESRXED64 register accessor: an alias for `Reg<FRAMESRXED64_SPEC>`"]
pub type FRAMESRXED64 = crate::Reg<framesrxed64::FRAMESRXED64_SPEC>;
#[doc = "64 Byte Frames Received"]
pub mod framesrxed64;
#[doc = "FRAMESRXED65 register accessor: an alias for `Reg<FRAMESRXED65_SPEC>`"]
pub type FRAMESRXED65 = crate::Reg<framesrxed65::FRAMESRXED65_SPEC>;
#[doc = "65 to 127 Byte Frames Received"]
pub mod framesrxed65;
#[doc = "FRAMESRXED128 register accessor: an alias for `Reg<FRAMESRXED128_SPEC>`"]
pub type FRAMESRXED128 = crate::Reg<framesrxed128::FRAMESRXED128_SPEC>;
#[doc = "128 to 255 Byte Frames Received"]
pub mod framesrxed128;
#[doc = "FRAMESRXED256 register accessor: an alias for `Reg<FRAMESRXED256_SPEC>`"]
pub type FRAMESRXED256 = crate::Reg<framesrxed256::FRAMESRXED256_SPEC>;
#[doc = "256 to 511 Byte Frames Received"]
pub mod framesrxed256;
#[doc = "FRAMESRXED512 register accessor: an alias for `Reg<FRAMESRXED512_SPEC>`"]
pub type FRAMESRXED512 = crate::Reg<framesrxed512::FRAMESRXED512_SPEC>;
#[doc = "512 to 1023 Byte Frames Received"]
pub mod framesrxed512;
#[doc = "FRAMESRXED1024 register accessor: an alias for `Reg<FRAMESRXED1024_SPEC>`"]
pub type FRAMESRXED1024 = crate::Reg<framesrxed1024::FRAMESRXED1024_SPEC>;
#[doc = "1024 to 1518 Byte Frames Received"]
pub mod framesrxed1024;
#[doc = "FRAMESRXED1519 register accessor: an alias for `Reg<FRAMESRXED1519_SPEC>`"]
pub type FRAMESRXED1519 = crate::Reg<framesrxed1519::FRAMESRXED1519_SPEC>;
#[doc = "1519 to maximum Byte Frames Received"]
pub mod framesrxed1519;
#[doc = "UNDERSIZEFRAMES register accessor: an alias for `Reg<UNDERSIZEFRAMES_SPEC>`"]
pub type UNDERSIZEFRAMES = crate::Reg<undersizeframes::UNDERSIZEFRAMES_SPEC>;
#[doc = "Undersized Frames Received"]
pub mod undersizeframes;
#[doc = "EXCESSIVERXLEN register accessor: an alias for `Reg<EXCESSIVERXLEN_SPEC>`"]
pub type EXCESSIVERXLEN = crate::Reg<excessiverxlen::EXCESSIVERXLEN_SPEC>;
#[doc = "Oversize Frames Received"]
pub mod excessiverxlen;
#[doc = "RXJABBERS register accessor: an alias for `Reg<RXJABBERS_SPEC>`"]
pub type RXJABBERS = crate::Reg<rxjabbers::RXJABBERS_SPEC>;
#[doc = "Jabbers Received"]
pub mod rxjabbers;
#[doc = "FCSERRS register accessor: an alias for `Reg<FCSERRS_SPEC>`"]
pub type FCSERRS = crate::Reg<fcserrs::FCSERRS_SPEC>;
#[doc = "Frame Check Sequence Errors"]
pub mod fcserrs;
#[doc = "RXLENERRS register accessor: an alias for `Reg<RXLENERRS_SPEC>`"]
pub type RXLENERRS = crate::Reg<rxlenerrs::RXLENERRS_SPEC>;
#[doc = "Length Field Frame Errors"]
pub mod rxlenerrs;
#[doc = "RXSYMBOLERRS register accessor: an alias for `Reg<RXSYMBOLERRS_SPEC>`"]
pub type RXSYMBOLERRS = crate::Reg<rxsymbolerrs::RXSYMBOLERRS_SPEC>;
#[doc = "Receive Symbol Errors"]
pub mod rxsymbolerrs;
#[doc = "ALIGNERRS register accessor: an alias for `Reg<ALIGNERRS_SPEC>`"]
pub type ALIGNERRS = crate::Reg<alignerrs::ALIGNERRS_SPEC>;
#[doc = "Alignment Errors"]
pub mod alignerrs;
#[doc = "RXRESOURCEERRS register accessor: an alias for `Reg<RXRESOURCEERRS_SPEC>`"]
pub type RXRESOURCEERRS = crate::Reg<rxresourceerrs::RXRESOURCEERRS_SPEC>;
#[doc = "Receive Resource Errors"]
pub mod rxresourceerrs;
#[doc = "RXOVERRUNS register accessor: an alias for `Reg<RXOVERRUNS_SPEC>`"]
pub type RXOVERRUNS = crate::Reg<rxoverruns::RXOVERRUNS_SPEC>;
#[doc = "Receive Overruns"]
pub mod rxoverruns;
#[doc = "RXIPCKERRS register accessor: an alias for `Reg<RXIPCKERRS_SPEC>`"]
pub type RXIPCKERRS = crate::Reg<rxipckerrs::RXIPCKERRS_SPEC>;
#[doc = "IP Header Checksum Errors"]
pub mod rxipckerrs;
#[doc = "RXTCPCKERRS register accessor: an alias for `Reg<RXTCPCKERRS_SPEC>`"]
pub type RXTCPCKERRS = crate::Reg<rxtcpckerrs::RXTCPCKERRS_SPEC>;
#[doc = "TCP Checksum Errors"]
pub mod rxtcpckerrs;
#[doc = "RXUDPCKERRS register accessor: an alias for `Reg<RXUDPCKERRS_SPEC>`"]
pub type RXUDPCKERRS = crate::Reg<rxudpckerrs::RXUDPCKERRS_SPEC>;
#[doc = "UDP Checksum Errors"]
pub mod rxudpckerrs;
#[doc = "AUTOFLUSHEDPKTS register accessor: an alias for `Reg<AUTOFLUSHEDPKTS_SPEC>`"]
pub type AUTOFLUSHEDPKTS = crate::Reg<autoflushedpkts::AUTOFLUSHEDPKTS_SPEC>;
#[doc = "Receive DMA Flushed Packets"]
pub mod autoflushedpkts;
#[doc = "TSUTIMERINCRSUBNSEC register accessor: an alias for `Reg<TSUTIMERINCRSUBNSEC_SPEC>`"]
pub type TSUTIMERINCRSUBNSEC = crate::Reg<tsutimerincrsubnsec::TSUTIMERINCRSUBNSEC_SPEC>;
#[doc = "1588 Timer Increment Register subscript nsec"]
pub mod tsutimerincrsubnsec;
#[doc = "TSUTIMERMSBSEC register accessor: an alias for `Reg<TSUTIMERMSBSEC_SPEC>`"]
pub type TSUTIMERMSBSEC = crate::Reg<tsutimermsbsec::TSUTIMERMSBSEC_SPEC>;
#[doc = "1588 Timer Seconds Register 47:32"]
pub mod tsutimermsbsec;
#[doc = "TSUTIMERSEC register accessor: an alias for `Reg<TSUTIMERSEC_SPEC>`"]
pub type TSUTIMERSEC = crate::Reg<tsutimersec::TSUTIMERSEC_SPEC>;
#[doc = "1588 Timer Seconds Register 31:0"]
pub mod tsutimersec;
#[doc = "TSUTIMERNSEC register accessor: an alias for `Reg<TSUTIMERNSEC_SPEC>`"]
pub type TSUTIMERNSEC = crate::Reg<tsutimernsec::TSUTIMERNSEC_SPEC>;
#[doc = "1588 Timer Nanoseconds Register"]
pub mod tsutimernsec;
#[doc = "TSUTIMERADJUST register accessor: an alias for `Reg<TSUTIMERADJUST_SPEC>`"]
pub type TSUTIMERADJUST = crate::Reg<tsutimeradjust::TSUTIMERADJUST_SPEC>;
#[doc = "This register returns all zeroes when read."]
pub mod tsutimeradjust;
#[doc = "TSUTIMERINCR register accessor: an alias for `Reg<TSUTIMERINCR_SPEC>`"]
pub type TSUTIMERINCR = crate::Reg<tsutimerincr::TSUTIMERINCR_SPEC>;
#[doc = "1588 Timer Increment Register"]
pub mod tsutimerincr;
#[doc = "TSUPTPTXSEC register accessor: an alias for `Reg<TSUPTPTXSEC_SPEC>`"]
pub type TSUPTPTXSEC = crate::Reg<tsuptptxsec::TSUPTPTXSEC_SPEC>;
#[doc = "PTP Event Frame Transmitted Seconds Register 31:0"]
pub mod tsuptptxsec;
#[doc = "TSUPTPTXNSEC register accessor: an alias for `Reg<TSUPTPTXNSEC_SPEC>`"]
pub type TSUPTPTXNSEC = crate::Reg<tsuptptxnsec::TSUPTPTXNSEC_SPEC>;
#[doc = "PTP Event Frame Transmitted Nanoseconds Register"]
pub mod tsuptptxnsec;
#[doc = "TSUPTPRXSEC register accessor: an alias for `Reg<TSUPTPRXSEC_SPEC>`"]
pub type TSUPTPRXSEC = crate::Reg<tsuptprxsec::TSUPTPRXSEC_SPEC>;
#[doc = "PTP Event Frame Received Seconds Register 31:0"]
pub mod tsuptprxsec;
#[doc = "TSUPTPRXNSEC register accessor: an alias for `Reg<TSUPTPRXNSEC_SPEC>`"]
pub type TSUPTPRXNSEC = crate::Reg<tsuptprxnsec::TSUPTPRXNSEC_SPEC>;
#[doc = "PTP Event Frame Received Nanoseconds Register"]
pub mod tsuptprxnsec;
#[doc = "TSUPEERTXSEC register accessor: an alias for `Reg<TSUPEERTXSEC_SPEC>`"]
pub type TSUPEERTXSEC = crate::Reg<tsupeertxsec::TSUPEERTXSEC_SPEC>;
#[doc = "PTP Peer Event Frame Transmitted Seconds Register 31:0"]
pub mod tsupeertxsec;
#[doc = "TSUPEERTXNSEC register accessor: an alias for `Reg<TSUPEERTXNSEC_SPEC>`"]
pub type TSUPEERTXNSEC = crate::Reg<tsupeertxnsec::TSUPEERTXNSEC_SPEC>;
#[doc = "PTP Peer Event Frame Transmitted Nanoseconds Register"]
pub mod tsupeertxnsec;
#[doc = "TSUPEERRXSEC register accessor: an alias for `Reg<TSUPEERRXSEC_SPEC>`"]
pub type TSUPEERRXSEC = crate::Reg<tsupeerrxsec::TSUPEERRXSEC_SPEC>;
#[doc = "PTP Peer Event Frame Received Seconds Register 31:0"]
pub mod tsupeerrxsec;
#[doc = "TSUPEERRXNSEC register accessor: an alias for `Reg<TSUPEERRXNSEC_SPEC>`"]
pub type TSUPEERRXNSEC = crate::Reg<tsupeerrxnsec::TSUPEERRXNSEC_SPEC>;
#[doc = "PTP Peer Event Frame Received Nanoseconds Register"]
pub mod tsupeerrxnsec;
#[doc = "TXPAUSEQUANT1 register accessor: an alias for `Reg<TXPAUSEQUANT1_SPEC>`"]
pub type TXPAUSEQUANT1 = crate::Reg<txpausequant1::TXPAUSEQUANT1_SPEC>;
#[doc = "Transmit Pause Quantum Register 1"]
pub mod txpausequant1;
#[doc = "TXPAUSEQUANT2 register accessor: an alias for `Reg<TXPAUSEQUANT2_SPEC>`"]
pub type TXPAUSEQUANT2 = crate::Reg<txpausequant2::TXPAUSEQUANT2_SPEC>;
#[doc = "Transmit Pause Quantum Register 2"]
pub mod txpausequant2;
#[doc = "TXPAUSEQUANT3 register accessor: an alias for `Reg<TXPAUSEQUANT3_SPEC>`"]
pub type TXPAUSEQUANT3 = crate::Reg<txpausequant3::TXPAUSEQUANT3_SPEC>;
#[doc = "Transmit Pause Quantum Register 3"]
pub mod txpausequant3;
#[doc = "RXLPI register accessor: an alias for `Reg<RXLPI_SPEC>`"]
pub type RXLPI = crate::Reg<rxlpi::RXLPI_SPEC>;
#[doc = "Received LPI transitions"]
pub mod rxlpi;
#[doc = "RXLPITIME register accessor: an alias for `Reg<RXLPITIME_SPEC>`"]
pub type RXLPITIME = crate::Reg<rxlpitime::RXLPITIME_SPEC>;
#[doc = "Received LPI time"]
pub mod rxlpitime;
#[doc = "TXLPI register accessor: an alias for `Reg<TXLPI_SPEC>`"]
pub type TXLPI = crate::Reg<txlpi::TXLPI_SPEC>;
#[doc = "Transmit LPI transitions"]
pub mod txlpi;
#[doc = "TXLPITIME register accessor: an alias for `Reg<TXLPITIME_SPEC>`"]
pub type TXLPITIME = crate::Reg<txlpitime::TXLPITIME_SPEC>;
#[doc = "Transmit LPI time"]
pub mod txlpitime;
#[doc = "TXBDCTRL register accessor: an alias for `Reg<TXBDCTRL_SPEC>`"]
pub type TXBDCTRL = crate::Reg<txbdctrl::TXBDCTRL_SPEC>;
#[doc = "TX BD control register"]
pub mod txbdctrl;
#[doc = "RXBDCTRL register accessor: an alias for `Reg<RXBDCTRL_SPEC>`"]
pub type RXBDCTRL = crate::Reg<rxbdctrl::RXBDCTRL_SPEC>;
#[doc = "RX BD control register"]
pub mod rxbdctrl;
#[doc = "ROUTEPEN register accessor: an alias for `Reg<ROUTEPEN_SPEC>`"]
pub type ROUTEPEN = crate::Reg<routepen::ROUTEPEN_SPEC>;
#[doc = "I/O Route Enable Register"]
pub mod routepen;
#[doc = "ROUTELOC0 register accessor: an alias for `Reg<ROUTELOC0_SPEC>`"]
pub type ROUTELOC0 = crate::Reg<routeloc0::ROUTELOC0_SPEC>;
#[doc = "I/O Route Location Register 0"]
pub mod routeloc0;
#[doc = "ROUTELOC1 register accessor: an alias for `Reg<ROUTELOC1_SPEC>`"]
pub type ROUTELOC1 = crate::Reg<routeloc1::ROUTELOC1_SPEC>;
#[doc = "I/O Route Location Register 1"]
pub mod routeloc1;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Ethernet control register"]
pub mod ctrl;
