#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - DMA Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x08 - DMA Synchronization Trigger Register (Single-Cycle RMW)"]
    pub sync: crate::Reg<sync::SYNC_SPEC>,
    _reserved3: [u8; 0x14],
    #[doc = "0x20 - DMA Channel Enable Register (Single-Cycle RMW)"]
    pub chen: crate::Reg<chen::CHEN_SPEC>,
    #[doc = "0x24 - DMA Channel Busy Register"]
    pub chbusy: crate::Reg<chbusy::CHBUSY_SPEC>,
    #[doc = "0x28 - DMA Channel Linking Done Register (Single-Cycle RMW)"]
    pub chdone: crate::Reg<chdone::CHDONE_SPEC>,
    #[doc = "0x2c - DMA Channel Debug Halt Register"]
    pub dbghalt: crate::Reg<dbghalt::DBGHALT_SPEC>,
    #[doc = "0x30 - DMA Channel Software Transfer Request Register"]
    pub swreq: crate::Reg<swreq::SWREQ_SPEC>,
    #[doc = "0x34 - DMA Channel Request Disable Register"]
    pub reqdis: crate::Reg<reqdis::REQDIS_SPEC>,
    #[doc = "0x38 - DMA Channel Requests Pending Register"]
    pub reqpend: crate::Reg<reqpend::REQPEND_SPEC>,
    #[doc = "0x3c - DMA Channel Link Load Register"]
    pub linkload: crate::Reg<linkload::LINKLOAD_SPEC>,
    #[doc = "0x40 - DMA Channel Request Clear Register"]
    pub reqclear: crate::Reg<reqclear::REQCLEAR_SPEC>,
    _reserved12: [u8; 0x1c],
    #[doc = "0x60 - Interrupt Flag Register"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x64 - Interrupt Flag Set Register"]
    pub ifs: crate::Reg<ifs::IFS_SPEC>,
    #[doc = "0x68 - Interrupt Flag Clear Register"]
    pub ifc: crate::Reg<ifc::IFC_SPEC>,
    #[doc = "0x6c - Interrupt Enable Register"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    _reserved16: [u8; 0x10],
    #[doc = "0x80 - Channel Peripheral Request Select Register"]
    pub ch0_reqsel: crate::Reg<ch0_reqsel::CH0_REQSEL_SPEC>,
    #[doc = "0x84 - Channel Configuration Register"]
    pub ch0_cfg: crate::Reg<ch0_cfg::CH0_CFG_SPEC>,
    #[doc = "0x88 - Channel Loop Counter Register"]
    pub ch0_loop: crate::Reg<ch0_loop::CH0_LOOP_SPEC>,
    #[doc = "0x8c - Channel Descriptor Control Word Register"]
    pub ch0_ctrl: crate::Reg<ch0_ctrl::CH0_CTRL_SPEC>,
    #[doc = "0x90 - Channel Descriptor Source Data Address Register"]
    pub ch0_src: crate::Reg<ch0_src::CH0_SRC_SPEC>,
    #[doc = "0x94 - Channel Descriptor Destination Data Address Register"]
    pub ch0_dst: crate::Reg<ch0_dst::CH0_DST_SPEC>,
    #[doc = "0x98 - Channel Descriptor Link Structure Address Register"]
    pub ch0_link: crate::Reg<ch0_link::CH0_LINK_SPEC>,
    _reserved23: [u8; 0x14],
    #[doc = "0xb0 - Channel Peripheral Request Select Register"]
    pub ch1_reqsel: crate::Reg<ch1_reqsel::CH1_REQSEL_SPEC>,
    #[doc = "0xb4 - Channel Configuration Register"]
    pub ch1_cfg: crate::Reg<ch1_cfg::CH1_CFG_SPEC>,
    #[doc = "0xb8 - Channel Loop Counter Register"]
    pub ch1_loop: crate::Reg<ch1_loop::CH1_LOOP_SPEC>,
    #[doc = "0xbc - Channel Descriptor Control Word Register"]
    pub ch1_ctrl: crate::Reg<ch1_ctrl::CH1_CTRL_SPEC>,
    #[doc = "0xc0 - Channel Descriptor Source Data Address Register"]
    pub ch1_src: crate::Reg<ch1_src::CH1_SRC_SPEC>,
    #[doc = "0xc4 - Channel Descriptor Destination Data Address Register"]
    pub ch1_dst: crate::Reg<ch1_dst::CH1_DST_SPEC>,
    #[doc = "0xc8 - Channel Descriptor Link Structure Address Register"]
    pub ch1_link: crate::Reg<ch1_link::CH1_LINK_SPEC>,
    _reserved30: [u8; 0x14],
    #[doc = "0xe0 - Channel Peripheral Request Select Register"]
    pub ch2_reqsel: crate::Reg<ch2_reqsel::CH2_REQSEL_SPEC>,
    #[doc = "0xe4 - Channel Configuration Register"]
    pub ch2_cfg: crate::Reg<ch2_cfg::CH2_CFG_SPEC>,
    #[doc = "0xe8 - Channel Loop Counter Register"]
    pub ch2_loop: crate::Reg<ch2_loop::CH2_LOOP_SPEC>,
    #[doc = "0xec - Channel Descriptor Control Word Register"]
    pub ch2_ctrl: crate::Reg<ch2_ctrl::CH2_CTRL_SPEC>,
    #[doc = "0xf0 - Channel Descriptor Source Data Address Register"]
    pub ch2_src: crate::Reg<ch2_src::CH2_SRC_SPEC>,
    #[doc = "0xf4 - Channel Descriptor Destination Data Address Register"]
    pub ch2_dst: crate::Reg<ch2_dst::CH2_DST_SPEC>,
    #[doc = "0xf8 - Channel Descriptor Link Structure Address Register"]
    pub ch2_link: crate::Reg<ch2_link::CH2_LINK_SPEC>,
    _reserved37: [u8; 0x14],
    #[doc = "0x110 - Channel Peripheral Request Select Register"]
    pub ch3_reqsel: crate::Reg<ch3_reqsel::CH3_REQSEL_SPEC>,
    #[doc = "0x114 - Channel Configuration Register"]
    pub ch3_cfg: crate::Reg<ch3_cfg::CH3_CFG_SPEC>,
    #[doc = "0x118 - Channel Loop Counter Register"]
    pub ch3_loop: crate::Reg<ch3_loop::CH3_LOOP_SPEC>,
    #[doc = "0x11c - Channel Descriptor Control Word Register"]
    pub ch3_ctrl: crate::Reg<ch3_ctrl::CH3_CTRL_SPEC>,
    #[doc = "0x120 - Channel Descriptor Source Data Address Register"]
    pub ch3_src: crate::Reg<ch3_src::CH3_SRC_SPEC>,
    #[doc = "0x124 - Channel Descriptor Destination Data Address Register"]
    pub ch3_dst: crate::Reg<ch3_dst::CH3_DST_SPEC>,
    #[doc = "0x128 - Channel Descriptor Link Structure Address Register"]
    pub ch3_link: crate::Reg<ch3_link::CH3_LINK_SPEC>,
    _reserved44: [u8; 0x14],
    #[doc = "0x140 - Channel Peripheral Request Select Register"]
    pub ch4_reqsel: crate::Reg<ch4_reqsel::CH4_REQSEL_SPEC>,
    #[doc = "0x144 - Channel Configuration Register"]
    pub ch4_cfg: crate::Reg<ch4_cfg::CH4_CFG_SPEC>,
    #[doc = "0x148 - Channel Loop Counter Register"]
    pub ch4_loop: crate::Reg<ch4_loop::CH4_LOOP_SPEC>,
    #[doc = "0x14c - Channel Descriptor Control Word Register"]
    pub ch4_ctrl: crate::Reg<ch4_ctrl::CH4_CTRL_SPEC>,
    #[doc = "0x150 - Channel Descriptor Source Data Address Register"]
    pub ch4_src: crate::Reg<ch4_src::CH4_SRC_SPEC>,
    #[doc = "0x154 - Channel Descriptor Destination Data Address Register"]
    pub ch4_dst: crate::Reg<ch4_dst::CH4_DST_SPEC>,
    #[doc = "0x158 - Channel Descriptor Link Structure Address Register"]
    pub ch4_link: crate::Reg<ch4_link::CH4_LINK_SPEC>,
    _reserved51: [u8; 0x14],
    #[doc = "0x170 - Channel Peripheral Request Select Register"]
    pub ch5_reqsel: crate::Reg<ch5_reqsel::CH5_REQSEL_SPEC>,
    #[doc = "0x174 - Channel Configuration Register"]
    pub ch5_cfg: crate::Reg<ch5_cfg::CH5_CFG_SPEC>,
    #[doc = "0x178 - Channel Loop Counter Register"]
    pub ch5_loop: crate::Reg<ch5_loop::CH5_LOOP_SPEC>,
    #[doc = "0x17c - Channel Descriptor Control Word Register"]
    pub ch5_ctrl: crate::Reg<ch5_ctrl::CH5_CTRL_SPEC>,
    #[doc = "0x180 - Channel Descriptor Source Data Address Register"]
    pub ch5_src: crate::Reg<ch5_src::CH5_SRC_SPEC>,
    #[doc = "0x184 - Channel Descriptor Destination Data Address Register"]
    pub ch5_dst: crate::Reg<ch5_dst::CH5_DST_SPEC>,
    #[doc = "0x188 - Channel Descriptor Link Structure Address Register"]
    pub ch5_link: crate::Reg<ch5_link::CH5_LINK_SPEC>,
    _reserved58: [u8; 0x14],
    #[doc = "0x1a0 - Channel Peripheral Request Select Register"]
    pub ch6_reqsel: crate::Reg<ch6_reqsel::CH6_REQSEL_SPEC>,
    #[doc = "0x1a4 - Channel Configuration Register"]
    pub ch6_cfg: crate::Reg<ch6_cfg::CH6_CFG_SPEC>,
    #[doc = "0x1a8 - Channel Loop Counter Register"]
    pub ch6_loop: crate::Reg<ch6_loop::CH6_LOOP_SPEC>,
    #[doc = "0x1ac - Channel Descriptor Control Word Register"]
    pub ch6_ctrl: crate::Reg<ch6_ctrl::CH6_CTRL_SPEC>,
    #[doc = "0x1b0 - Channel Descriptor Source Data Address Register"]
    pub ch6_src: crate::Reg<ch6_src::CH6_SRC_SPEC>,
    #[doc = "0x1b4 - Channel Descriptor Destination Data Address Register"]
    pub ch6_dst: crate::Reg<ch6_dst::CH6_DST_SPEC>,
    #[doc = "0x1b8 - Channel Descriptor Link Structure Address Register"]
    pub ch6_link: crate::Reg<ch6_link::CH6_LINK_SPEC>,
    _reserved65: [u8; 0x14],
    #[doc = "0x1d0 - Channel Peripheral Request Select Register"]
    pub ch7_reqsel: crate::Reg<ch7_reqsel::CH7_REQSEL_SPEC>,
    #[doc = "0x1d4 - Channel Configuration Register"]
    pub ch7_cfg: crate::Reg<ch7_cfg::CH7_CFG_SPEC>,
    #[doc = "0x1d8 - Channel Loop Counter Register"]
    pub ch7_loop: crate::Reg<ch7_loop::CH7_LOOP_SPEC>,
    #[doc = "0x1dc - Channel Descriptor Control Word Register"]
    pub ch7_ctrl: crate::Reg<ch7_ctrl::CH7_CTRL_SPEC>,
    #[doc = "0x1e0 - Channel Descriptor Source Data Address Register"]
    pub ch7_src: crate::Reg<ch7_src::CH7_SRC_SPEC>,
    #[doc = "0x1e4 - Channel Descriptor Destination Data Address Register"]
    pub ch7_dst: crate::Reg<ch7_dst::CH7_DST_SPEC>,
    #[doc = "0x1e8 - Channel Descriptor Link Structure Address Register"]
    pub ch7_link: crate::Reg<ch7_link::CH7_LINK_SPEC>,
    _reserved72: [u8; 0x14],
    #[doc = "0x200 - Channel Peripheral Request Select Register"]
    pub ch8_reqsel: crate::Reg<ch8_reqsel::CH8_REQSEL_SPEC>,
    #[doc = "0x204 - Channel Configuration Register"]
    pub ch8_cfg: crate::Reg<ch8_cfg::CH8_CFG_SPEC>,
    #[doc = "0x208 - Channel Loop Counter Register"]
    pub ch8_loop: crate::Reg<ch8_loop::CH8_LOOP_SPEC>,
    #[doc = "0x20c - Channel Descriptor Control Word Register"]
    pub ch8_ctrl: crate::Reg<ch8_ctrl::CH8_CTRL_SPEC>,
    #[doc = "0x210 - Channel Descriptor Source Data Address Register"]
    pub ch8_src: crate::Reg<ch8_src::CH8_SRC_SPEC>,
    #[doc = "0x214 - Channel Descriptor Destination Data Address Register"]
    pub ch8_dst: crate::Reg<ch8_dst::CH8_DST_SPEC>,
    #[doc = "0x218 - Channel Descriptor Link Structure Address Register"]
    pub ch8_link: crate::Reg<ch8_link::CH8_LINK_SPEC>,
    _reserved79: [u8; 0x14],
    #[doc = "0x230 - Channel Peripheral Request Select Register"]
    pub ch9_reqsel: crate::Reg<ch9_reqsel::CH9_REQSEL_SPEC>,
    #[doc = "0x234 - Channel Configuration Register"]
    pub ch9_cfg: crate::Reg<ch9_cfg::CH9_CFG_SPEC>,
    #[doc = "0x238 - Channel Loop Counter Register"]
    pub ch9_loop: crate::Reg<ch9_loop::CH9_LOOP_SPEC>,
    #[doc = "0x23c - Channel Descriptor Control Word Register"]
    pub ch9_ctrl: crate::Reg<ch9_ctrl::CH9_CTRL_SPEC>,
    #[doc = "0x240 - Channel Descriptor Source Data Address Register"]
    pub ch9_src: crate::Reg<ch9_src::CH9_SRC_SPEC>,
    #[doc = "0x244 - Channel Descriptor Destination Data Address Register"]
    pub ch9_dst: crate::Reg<ch9_dst::CH9_DST_SPEC>,
    #[doc = "0x248 - Channel Descriptor Link Structure Address Register"]
    pub ch9_link: crate::Reg<ch9_link::CH9_LINK_SPEC>,
    _reserved86: [u8; 0x14],
    #[doc = "0x260 - Channel Peripheral Request Select Register"]
    pub ch10_reqsel: crate::Reg<ch10_reqsel::CH10_REQSEL_SPEC>,
    #[doc = "0x264 - Channel Configuration Register"]
    pub ch10_cfg: crate::Reg<ch10_cfg::CH10_CFG_SPEC>,
    #[doc = "0x268 - Channel Loop Counter Register"]
    pub ch10_loop: crate::Reg<ch10_loop::CH10_LOOP_SPEC>,
    #[doc = "0x26c - Channel Descriptor Control Word Register"]
    pub ch10_ctrl: crate::Reg<ch10_ctrl::CH10_CTRL_SPEC>,
    #[doc = "0x270 - Channel Descriptor Source Data Address Register"]
    pub ch10_src: crate::Reg<ch10_src::CH10_SRC_SPEC>,
    #[doc = "0x274 - Channel Descriptor Destination Data Address Register"]
    pub ch10_dst: crate::Reg<ch10_dst::CH10_DST_SPEC>,
    #[doc = "0x278 - Channel Descriptor Link Structure Address Register"]
    pub ch10_link: crate::Reg<ch10_link::CH10_LINK_SPEC>,
    _reserved93: [u8; 0x14],
    #[doc = "0x290 - Channel Peripheral Request Select Register"]
    pub ch11_reqsel: crate::Reg<ch11_reqsel::CH11_REQSEL_SPEC>,
    #[doc = "0x294 - Channel Configuration Register"]
    pub ch11_cfg: crate::Reg<ch11_cfg::CH11_CFG_SPEC>,
    #[doc = "0x298 - Channel Loop Counter Register"]
    pub ch11_loop: crate::Reg<ch11_loop::CH11_LOOP_SPEC>,
    #[doc = "0x29c - Channel Descriptor Control Word Register"]
    pub ch11_ctrl: crate::Reg<ch11_ctrl::CH11_CTRL_SPEC>,
    #[doc = "0x2a0 - Channel Descriptor Source Data Address Register"]
    pub ch11_src: crate::Reg<ch11_src::CH11_SRC_SPEC>,
    #[doc = "0x2a4 - Channel Descriptor Destination Data Address Register"]
    pub ch11_dst: crate::Reg<ch11_dst::CH11_DST_SPEC>,
    #[doc = "0x2a8 - Channel Descriptor Link Structure Address Register"]
    pub ch11_link: crate::Reg<ch11_link::CH11_LINK_SPEC>,
    _reserved100: [u8; 0x14],
    #[doc = "0x2c0 - Channel Peripheral Request Select Register"]
    pub ch12_reqsel: crate::Reg<ch12_reqsel::CH12_REQSEL_SPEC>,
    #[doc = "0x2c4 - Channel Configuration Register"]
    pub ch12_cfg: crate::Reg<ch12_cfg::CH12_CFG_SPEC>,
    #[doc = "0x2c8 - Channel Loop Counter Register"]
    pub ch12_loop: crate::Reg<ch12_loop::CH12_LOOP_SPEC>,
    #[doc = "0x2cc - Channel Descriptor Control Word Register"]
    pub ch12_ctrl: crate::Reg<ch12_ctrl::CH12_CTRL_SPEC>,
    #[doc = "0x2d0 - Channel Descriptor Source Data Address Register"]
    pub ch12_src: crate::Reg<ch12_src::CH12_SRC_SPEC>,
    #[doc = "0x2d4 - Channel Descriptor Destination Data Address Register"]
    pub ch12_dst: crate::Reg<ch12_dst::CH12_DST_SPEC>,
    #[doc = "0x2d8 - Channel Descriptor Link Structure Address Register"]
    pub ch12_link: crate::Reg<ch12_link::CH12_LINK_SPEC>,
    _reserved107: [u8; 0x14],
    #[doc = "0x2f0 - Channel Peripheral Request Select Register"]
    pub ch13_reqsel: crate::Reg<ch13_reqsel::CH13_REQSEL_SPEC>,
    #[doc = "0x2f4 - Channel Configuration Register"]
    pub ch13_cfg: crate::Reg<ch13_cfg::CH13_CFG_SPEC>,
    #[doc = "0x2f8 - Channel Loop Counter Register"]
    pub ch13_loop: crate::Reg<ch13_loop::CH13_LOOP_SPEC>,
    #[doc = "0x2fc - Channel Descriptor Control Word Register"]
    pub ch13_ctrl: crate::Reg<ch13_ctrl::CH13_CTRL_SPEC>,
    #[doc = "0x300 - Channel Descriptor Source Data Address Register"]
    pub ch13_src: crate::Reg<ch13_src::CH13_SRC_SPEC>,
    #[doc = "0x304 - Channel Descriptor Destination Data Address Register"]
    pub ch13_dst: crate::Reg<ch13_dst::CH13_DST_SPEC>,
    #[doc = "0x308 - Channel Descriptor Link Structure Address Register"]
    pub ch13_link: crate::Reg<ch13_link::CH13_LINK_SPEC>,
    _reserved114: [u8; 0x14],
    #[doc = "0x320 - Channel Peripheral Request Select Register"]
    pub ch14_reqsel: crate::Reg<ch14_reqsel::CH14_REQSEL_SPEC>,
    #[doc = "0x324 - Channel Configuration Register"]
    pub ch14_cfg: crate::Reg<ch14_cfg::CH14_CFG_SPEC>,
    #[doc = "0x328 - Channel Loop Counter Register"]
    pub ch14_loop: crate::Reg<ch14_loop::CH14_LOOP_SPEC>,
    #[doc = "0x32c - Channel Descriptor Control Word Register"]
    pub ch14_ctrl: crate::Reg<ch14_ctrl::CH14_CTRL_SPEC>,
    #[doc = "0x330 - Channel Descriptor Source Data Address Register"]
    pub ch14_src: crate::Reg<ch14_src::CH14_SRC_SPEC>,
    #[doc = "0x334 - Channel Descriptor Destination Data Address Register"]
    pub ch14_dst: crate::Reg<ch14_dst::CH14_DST_SPEC>,
    #[doc = "0x338 - Channel Descriptor Link Structure Address Register"]
    pub ch14_link: crate::Reg<ch14_link::CH14_LINK_SPEC>,
    _reserved121: [u8; 0x14],
    #[doc = "0x350 - Channel Peripheral Request Select Register"]
    pub ch15_reqsel: crate::Reg<ch15_reqsel::CH15_REQSEL_SPEC>,
    #[doc = "0x354 - Channel Configuration Register"]
    pub ch15_cfg: crate::Reg<ch15_cfg::CH15_CFG_SPEC>,
    #[doc = "0x358 - Channel Loop Counter Register"]
    pub ch15_loop: crate::Reg<ch15_loop::CH15_LOOP_SPEC>,
    #[doc = "0x35c - Channel Descriptor Control Word Register"]
    pub ch15_ctrl: crate::Reg<ch15_ctrl::CH15_CTRL_SPEC>,
    #[doc = "0x360 - Channel Descriptor Source Data Address Register"]
    pub ch15_src: crate::Reg<ch15_src::CH15_SRC_SPEC>,
    #[doc = "0x364 - Channel Descriptor Destination Data Address Register"]
    pub ch15_dst: crate::Reg<ch15_dst::CH15_DST_SPEC>,
    #[doc = "0x368 - Channel Descriptor Link Structure Address Register"]
    pub ch15_link: crate::Reg<ch15_link::CH15_LINK_SPEC>,
    _reserved128: [u8; 0x14],
    #[doc = "0x380 - Channel Peripheral Request Select Register"]
    pub ch16_reqsel: crate::Reg<ch16_reqsel::CH16_REQSEL_SPEC>,
    #[doc = "0x384 - Channel Configuration Register"]
    pub ch16_cfg: crate::Reg<ch16_cfg::CH16_CFG_SPEC>,
    #[doc = "0x388 - Channel Loop Counter Register"]
    pub ch16_loop: crate::Reg<ch16_loop::CH16_LOOP_SPEC>,
    #[doc = "0x38c - Channel Descriptor Control Word Register"]
    pub ch16_ctrl: crate::Reg<ch16_ctrl::CH16_CTRL_SPEC>,
    #[doc = "0x390 - Channel Descriptor Source Data Address Register"]
    pub ch16_src: crate::Reg<ch16_src::CH16_SRC_SPEC>,
    #[doc = "0x394 - Channel Descriptor Destination Data Address Register"]
    pub ch16_dst: crate::Reg<ch16_dst::CH16_DST_SPEC>,
    #[doc = "0x398 - Channel Descriptor Link Structure Address Register"]
    pub ch16_link: crate::Reg<ch16_link::CH16_LINK_SPEC>,
    _reserved135: [u8; 0x14],
    #[doc = "0x3b0 - Channel Peripheral Request Select Register"]
    pub ch17_reqsel: crate::Reg<ch17_reqsel::CH17_REQSEL_SPEC>,
    #[doc = "0x3b4 - Channel Configuration Register"]
    pub ch17_cfg: crate::Reg<ch17_cfg::CH17_CFG_SPEC>,
    #[doc = "0x3b8 - Channel Loop Counter Register"]
    pub ch17_loop: crate::Reg<ch17_loop::CH17_LOOP_SPEC>,
    #[doc = "0x3bc - Channel Descriptor Control Word Register"]
    pub ch17_ctrl: crate::Reg<ch17_ctrl::CH17_CTRL_SPEC>,
    #[doc = "0x3c0 - Channel Descriptor Source Data Address Register"]
    pub ch17_src: crate::Reg<ch17_src::CH17_SRC_SPEC>,
    #[doc = "0x3c4 - Channel Descriptor Destination Data Address Register"]
    pub ch17_dst: crate::Reg<ch17_dst::CH17_DST_SPEC>,
    #[doc = "0x3c8 - Channel Descriptor Link Structure Address Register"]
    pub ch17_link: crate::Reg<ch17_link::CH17_LINK_SPEC>,
    _reserved142: [u8; 0x14],
    #[doc = "0x3e0 - Channel Peripheral Request Select Register"]
    pub ch18_reqsel: crate::Reg<ch18_reqsel::CH18_REQSEL_SPEC>,
    #[doc = "0x3e4 - Channel Configuration Register"]
    pub ch18_cfg: crate::Reg<ch18_cfg::CH18_CFG_SPEC>,
    #[doc = "0x3e8 - Channel Loop Counter Register"]
    pub ch18_loop: crate::Reg<ch18_loop::CH18_LOOP_SPEC>,
    #[doc = "0x3ec - Channel Descriptor Control Word Register"]
    pub ch18_ctrl: crate::Reg<ch18_ctrl::CH18_CTRL_SPEC>,
    #[doc = "0x3f0 - Channel Descriptor Source Data Address Register"]
    pub ch18_src: crate::Reg<ch18_src::CH18_SRC_SPEC>,
    #[doc = "0x3f4 - Channel Descriptor Destination Data Address Register"]
    pub ch18_dst: crate::Reg<ch18_dst::CH18_DST_SPEC>,
    #[doc = "0x3f8 - Channel Descriptor Link Structure Address Register"]
    pub ch18_link: crate::Reg<ch18_link::CH18_LINK_SPEC>,
    _reserved149: [u8; 0x14],
    #[doc = "0x410 - Channel Peripheral Request Select Register"]
    pub ch19_reqsel: crate::Reg<ch19_reqsel::CH19_REQSEL_SPEC>,
    #[doc = "0x414 - Channel Configuration Register"]
    pub ch19_cfg: crate::Reg<ch19_cfg::CH19_CFG_SPEC>,
    #[doc = "0x418 - Channel Loop Counter Register"]
    pub ch19_loop: crate::Reg<ch19_loop::CH19_LOOP_SPEC>,
    #[doc = "0x41c - Channel Descriptor Control Word Register"]
    pub ch19_ctrl: crate::Reg<ch19_ctrl::CH19_CTRL_SPEC>,
    #[doc = "0x420 - Channel Descriptor Source Data Address Register"]
    pub ch19_src: crate::Reg<ch19_src::CH19_SRC_SPEC>,
    #[doc = "0x424 - Channel Descriptor Destination Data Address Register"]
    pub ch19_dst: crate::Reg<ch19_dst::CH19_DST_SPEC>,
    #[doc = "0x428 - Channel Descriptor Link Structure Address Register"]
    pub ch19_link: crate::Reg<ch19_link::CH19_LINK_SPEC>,
    _reserved156: [u8; 0x14],
    #[doc = "0x440 - Channel Peripheral Request Select Register"]
    pub ch20_reqsel: crate::Reg<ch20_reqsel::CH20_REQSEL_SPEC>,
    #[doc = "0x444 - Channel Configuration Register"]
    pub ch20_cfg: crate::Reg<ch20_cfg::CH20_CFG_SPEC>,
    #[doc = "0x448 - Channel Loop Counter Register"]
    pub ch20_loop: crate::Reg<ch20_loop::CH20_LOOP_SPEC>,
    #[doc = "0x44c - Channel Descriptor Control Word Register"]
    pub ch20_ctrl: crate::Reg<ch20_ctrl::CH20_CTRL_SPEC>,
    #[doc = "0x450 - Channel Descriptor Source Data Address Register"]
    pub ch20_src: crate::Reg<ch20_src::CH20_SRC_SPEC>,
    #[doc = "0x454 - Channel Descriptor Destination Data Address Register"]
    pub ch20_dst: crate::Reg<ch20_dst::CH20_DST_SPEC>,
    #[doc = "0x458 - Channel Descriptor Link Structure Address Register"]
    pub ch20_link: crate::Reg<ch20_link::CH20_LINK_SPEC>,
    _reserved163: [u8; 0x14],
    #[doc = "0x470 - Channel Peripheral Request Select Register"]
    pub ch21_reqsel: crate::Reg<ch21_reqsel::CH21_REQSEL_SPEC>,
    #[doc = "0x474 - Channel Configuration Register"]
    pub ch21_cfg: crate::Reg<ch21_cfg::CH21_CFG_SPEC>,
    #[doc = "0x478 - Channel Loop Counter Register"]
    pub ch21_loop: crate::Reg<ch21_loop::CH21_LOOP_SPEC>,
    #[doc = "0x47c - Channel Descriptor Control Word Register"]
    pub ch21_ctrl: crate::Reg<ch21_ctrl::CH21_CTRL_SPEC>,
    #[doc = "0x480 - Channel Descriptor Source Data Address Register"]
    pub ch21_src: crate::Reg<ch21_src::CH21_SRC_SPEC>,
    #[doc = "0x484 - Channel Descriptor Destination Data Address Register"]
    pub ch21_dst: crate::Reg<ch21_dst::CH21_DST_SPEC>,
    #[doc = "0x488 - Channel Descriptor Link Structure Address Register"]
    pub ch21_link: crate::Reg<ch21_link::CH21_LINK_SPEC>,
    _reserved170: [u8; 0x14],
    #[doc = "0x4a0 - Channel Peripheral Request Select Register"]
    pub ch22_reqsel: crate::Reg<ch22_reqsel::CH22_REQSEL_SPEC>,
    #[doc = "0x4a4 - Channel Configuration Register"]
    pub ch22_cfg: crate::Reg<ch22_cfg::CH22_CFG_SPEC>,
    #[doc = "0x4a8 - Channel Loop Counter Register"]
    pub ch22_loop: crate::Reg<ch22_loop::CH22_LOOP_SPEC>,
    #[doc = "0x4ac - Channel Descriptor Control Word Register"]
    pub ch22_ctrl: crate::Reg<ch22_ctrl::CH22_CTRL_SPEC>,
    #[doc = "0x4b0 - Channel Descriptor Source Data Address Register"]
    pub ch22_src: crate::Reg<ch22_src::CH22_SRC_SPEC>,
    #[doc = "0x4b4 - Channel Descriptor Destination Data Address Register"]
    pub ch22_dst: crate::Reg<ch22_dst::CH22_DST_SPEC>,
    #[doc = "0x4b8 - Channel Descriptor Link Structure Address Register"]
    pub ch22_link: crate::Reg<ch22_link::CH22_LINK_SPEC>,
    _reserved177: [u8; 0x14],
    #[doc = "0x4d0 - Channel Peripheral Request Select Register"]
    pub ch23_reqsel: crate::Reg<ch23_reqsel::CH23_REQSEL_SPEC>,
    #[doc = "0x4d4 - Channel Configuration Register"]
    pub ch23_cfg: crate::Reg<ch23_cfg::CH23_CFG_SPEC>,
    #[doc = "0x4d8 - Channel Loop Counter Register"]
    pub ch23_loop: crate::Reg<ch23_loop::CH23_LOOP_SPEC>,
    #[doc = "0x4dc - Channel Descriptor Control Word Register"]
    pub ch23_ctrl: crate::Reg<ch23_ctrl::CH23_CTRL_SPEC>,
    #[doc = "0x4e0 - Channel Descriptor Source Data Address Register"]
    pub ch23_src: crate::Reg<ch23_src::CH23_SRC_SPEC>,
    #[doc = "0x4e4 - Channel Descriptor Destination Data Address Register"]
    pub ch23_dst: crate::Reg<ch23_dst::CH23_DST_SPEC>,
    #[doc = "0x4e8 - Channel Descriptor Link Structure Address Register"]
    pub ch23_link: crate::Reg<ch23_link::CH23_LINK_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "DMA Control Register"]
pub mod ctrl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "DMA Status Register"]
pub mod status;
#[doc = "SYNC register accessor: an alias for `Reg<SYNC_SPEC>`"]
pub type SYNC = crate::Reg<sync::SYNC_SPEC>;
#[doc = "DMA Synchronization Trigger Register (Single-Cycle RMW)"]
pub mod sync;
#[doc = "CHEN register accessor: an alias for `Reg<CHEN_SPEC>`"]
pub type CHEN = crate::Reg<chen::CHEN_SPEC>;
#[doc = "DMA Channel Enable Register (Single-Cycle RMW)"]
pub mod chen;
#[doc = "CHBUSY register accessor: an alias for `Reg<CHBUSY_SPEC>`"]
pub type CHBUSY = crate::Reg<chbusy::CHBUSY_SPEC>;
#[doc = "DMA Channel Busy Register"]
pub mod chbusy;
#[doc = "CHDONE register accessor: an alias for `Reg<CHDONE_SPEC>`"]
pub type CHDONE = crate::Reg<chdone::CHDONE_SPEC>;
#[doc = "DMA Channel Linking Done Register (Single-Cycle RMW)"]
pub mod chdone;
#[doc = "DBGHALT register accessor: an alias for `Reg<DBGHALT_SPEC>`"]
pub type DBGHALT = crate::Reg<dbghalt::DBGHALT_SPEC>;
#[doc = "DMA Channel Debug Halt Register"]
pub mod dbghalt;
#[doc = "SWREQ register accessor: an alias for `Reg<SWREQ_SPEC>`"]
pub type SWREQ = crate::Reg<swreq::SWREQ_SPEC>;
#[doc = "DMA Channel Software Transfer Request Register"]
pub mod swreq;
#[doc = "REQDIS register accessor: an alias for `Reg<REQDIS_SPEC>`"]
pub type REQDIS = crate::Reg<reqdis::REQDIS_SPEC>;
#[doc = "DMA Channel Request Disable Register"]
pub mod reqdis;
#[doc = "REQPEND register accessor: an alias for `Reg<REQPEND_SPEC>`"]
pub type REQPEND = crate::Reg<reqpend::REQPEND_SPEC>;
#[doc = "DMA Channel Requests Pending Register"]
pub mod reqpend;
#[doc = "LINKLOAD register accessor: an alias for `Reg<LINKLOAD_SPEC>`"]
pub type LINKLOAD = crate::Reg<linkload::LINKLOAD_SPEC>;
#[doc = "DMA Channel Link Load Register"]
pub mod linkload;
#[doc = "REQCLEAR register accessor: an alias for `Reg<REQCLEAR_SPEC>`"]
pub type REQCLEAR = crate::Reg<reqclear::REQCLEAR_SPEC>;
#[doc = "DMA Channel Request Clear Register"]
pub mod reqclear;
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
#[doc = "CH0_REQSEL register accessor: an alias for `Reg<CH0_REQSEL_SPEC>`"]
pub type CH0_REQSEL = crate::Reg<ch0_reqsel::CH0_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch0_reqsel;
#[doc = "CH0_CFG register accessor: an alias for `Reg<CH0_CFG_SPEC>`"]
pub type CH0_CFG = crate::Reg<ch0_cfg::CH0_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch0_cfg;
#[doc = "CH0_LOOP register accessor: an alias for `Reg<CH0_LOOP_SPEC>`"]
pub type CH0_LOOP = crate::Reg<ch0_loop::CH0_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch0_loop;
#[doc = "CH0_CTRL register accessor: an alias for `Reg<CH0_CTRL_SPEC>`"]
pub type CH0_CTRL = crate::Reg<ch0_ctrl::CH0_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch0_ctrl;
#[doc = "CH0_SRC register accessor: an alias for `Reg<CH0_SRC_SPEC>`"]
pub type CH0_SRC = crate::Reg<ch0_src::CH0_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch0_src;
#[doc = "CH0_DST register accessor: an alias for `Reg<CH0_DST_SPEC>`"]
pub type CH0_DST = crate::Reg<ch0_dst::CH0_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch0_dst;
#[doc = "CH0_LINK register accessor: an alias for `Reg<CH0_LINK_SPEC>`"]
pub type CH0_LINK = crate::Reg<ch0_link::CH0_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch0_link;
#[doc = "CH1_REQSEL register accessor: an alias for `Reg<CH1_REQSEL_SPEC>`"]
pub type CH1_REQSEL = crate::Reg<ch1_reqsel::CH1_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch1_reqsel;
#[doc = "CH1_CFG register accessor: an alias for `Reg<CH1_CFG_SPEC>`"]
pub type CH1_CFG = crate::Reg<ch1_cfg::CH1_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch1_cfg;
#[doc = "CH1_LOOP register accessor: an alias for `Reg<CH1_LOOP_SPEC>`"]
pub type CH1_LOOP = crate::Reg<ch1_loop::CH1_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch1_loop;
#[doc = "CH1_CTRL register accessor: an alias for `Reg<CH1_CTRL_SPEC>`"]
pub type CH1_CTRL = crate::Reg<ch1_ctrl::CH1_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch1_ctrl;
#[doc = "CH1_SRC register accessor: an alias for `Reg<CH1_SRC_SPEC>`"]
pub type CH1_SRC = crate::Reg<ch1_src::CH1_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch1_src;
#[doc = "CH1_DST register accessor: an alias for `Reg<CH1_DST_SPEC>`"]
pub type CH1_DST = crate::Reg<ch1_dst::CH1_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch1_dst;
#[doc = "CH1_LINK register accessor: an alias for `Reg<CH1_LINK_SPEC>`"]
pub type CH1_LINK = crate::Reg<ch1_link::CH1_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch1_link;
#[doc = "CH2_REQSEL register accessor: an alias for `Reg<CH2_REQSEL_SPEC>`"]
pub type CH2_REQSEL = crate::Reg<ch2_reqsel::CH2_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch2_reqsel;
#[doc = "CH2_CFG register accessor: an alias for `Reg<CH2_CFG_SPEC>`"]
pub type CH2_CFG = crate::Reg<ch2_cfg::CH2_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch2_cfg;
#[doc = "CH2_LOOP register accessor: an alias for `Reg<CH2_LOOP_SPEC>`"]
pub type CH2_LOOP = crate::Reg<ch2_loop::CH2_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch2_loop;
#[doc = "CH2_CTRL register accessor: an alias for `Reg<CH2_CTRL_SPEC>`"]
pub type CH2_CTRL = crate::Reg<ch2_ctrl::CH2_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch2_ctrl;
#[doc = "CH2_SRC register accessor: an alias for `Reg<CH2_SRC_SPEC>`"]
pub type CH2_SRC = crate::Reg<ch2_src::CH2_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch2_src;
#[doc = "CH2_DST register accessor: an alias for `Reg<CH2_DST_SPEC>`"]
pub type CH2_DST = crate::Reg<ch2_dst::CH2_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch2_dst;
#[doc = "CH2_LINK register accessor: an alias for `Reg<CH2_LINK_SPEC>`"]
pub type CH2_LINK = crate::Reg<ch2_link::CH2_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch2_link;
#[doc = "CH3_REQSEL register accessor: an alias for `Reg<CH3_REQSEL_SPEC>`"]
pub type CH3_REQSEL = crate::Reg<ch3_reqsel::CH3_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch3_reqsel;
#[doc = "CH3_CFG register accessor: an alias for `Reg<CH3_CFG_SPEC>`"]
pub type CH3_CFG = crate::Reg<ch3_cfg::CH3_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch3_cfg;
#[doc = "CH3_LOOP register accessor: an alias for `Reg<CH3_LOOP_SPEC>`"]
pub type CH3_LOOP = crate::Reg<ch3_loop::CH3_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch3_loop;
#[doc = "CH3_CTRL register accessor: an alias for `Reg<CH3_CTRL_SPEC>`"]
pub type CH3_CTRL = crate::Reg<ch3_ctrl::CH3_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch3_ctrl;
#[doc = "CH3_SRC register accessor: an alias for `Reg<CH3_SRC_SPEC>`"]
pub type CH3_SRC = crate::Reg<ch3_src::CH3_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch3_src;
#[doc = "CH3_DST register accessor: an alias for `Reg<CH3_DST_SPEC>`"]
pub type CH3_DST = crate::Reg<ch3_dst::CH3_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch3_dst;
#[doc = "CH3_LINK register accessor: an alias for `Reg<CH3_LINK_SPEC>`"]
pub type CH3_LINK = crate::Reg<ch3_link::CH3_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch3_link;
#[doc = "CH4_REQSEL register accessor: an alias for `Reg<CH4_REQSEL_SPEC>`"]
pub type CH4_REQSEL = crate::Reg<ch4_reqsel::CH4_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch4_reqsel;
#[doc = "CH4_CFG register accessor: an alias for `Reg<CH4_CFG_SPEC>`"]
pub type CH4_CFG = crate::Reg<ch4_cfg::CH4_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch4_cfg;
#[doc = "CH4_LOOP register accessor: an alias for `Reg<CH4_LOOP_SPEC>`"]
pub type CH4_LOOP = crate::Reg<ch4_loop::CH4_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch4_loop;
#[doc = "CH4_CTRL register accessor: an alias for `Reg<CH4_CTRL_SPEC>`"]
pub type CH4_CTRL = crate::Reg<ch4_ctrl::CH4_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch4_ctrl;
#[doc = "CH4_SRC register accessor: an alias for `Reg<CH4_SRC_SPEC>`"]
pub type CH4_SRC = crate::Reg<ch4_src::CH4_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch4_src;
#[doc = "CH4_DST register accessor: an alias for `Reg<CH4_DST_SPEC>`"]
pub type CH4_DST = crate::Reg<ch4_dst::CH4_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch4_dst;
#[doc = "CH4_LINK register accessor: an alias for `Reg<CH4_LINK_SPEC>`"]
pub type CH4_LINK = crate::Reg<ch4_link::CH4_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch4_link;
#[doc = "CH5_REQSEL register accessor: an alias for `Reg<CH5_REQSEL_SPEC>`"]
pub type CH5_REQSEL = crate::Reg<ch5_reqsel::CH5_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch5_reqsel;
#[doc = "CH5_CFG register accessor: an alias for `Reg<CH5_CFG_SPEC>`"]
pub type CH5_CFG = crate::Reg<ch5_cfg::CH5_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch5_cfg;
#[doc = "CH5_LOOP register accessor: an alias for `Reg<CH5_LOOP_SPEC>`"]
pub type CH5_LOOP = crate::Reg<ch5_loop::CH5_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch5_loop;
#[doc = "CH5_CTRL register accessor: an alias for `Reg<CH5_CTRL_SPEC>`"]
pub type CH5_CTRL = crate::Reg<ch5_ctrl::CH5_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch5_ctrl;
#[doc = "CH5_SRC register accessor: an alias for `Reg<CH5_SRC_SPEC>`"]
pub type CH5_SRC = crate::Reg<ch5_src::CH5_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch5_src;
#[doc = "CH5_DST register accessor: an alias for `Reg<CH5_DST_SPEC>`"]
pub type CH5_DST = crate::Reg<ch5_dst::CH5_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch5_dst;
#[doc = "CH5_LINK register accessor: an alias for `Reg<CH5_LINK_SPEC>`"]
pub type CH5_LINK = crate::Reg<ch5_link::CH5_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch5_link;
#[doc = "CH6_REQSEL register accessor: an alias for `Reg<CH6_REQSEL_SPEC>`"]
pub type CH6_REQSEL = crate::Reg<ch6_reqsel::CH6_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch6_reqsel;
#[doc = "CH6_CFG register accessor: an alias for `Reg<CH6_CFG_SPEC>`"]
pub type CH6_CFG = crate::Reg<ch6_cfg::CH6_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch6_cfg;
#[doc = "CH6_LOOP register accessor: an alias for `Reg<CH6_LOOP_SPEC>`"]
pub type CH6_LOOP = crate::Reg<ch6_loop::CH6_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch6_loop;
#[doc = "CH6_CTRL register accessor: an alias for `Reg<CH6_CTRL_SPEC>`"]
pub type CH6_CTRL = crate::Reg<ch6_ctrl::CH6_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch6_ctrl;
#[doc = "CH6_SRC register accessor: an alias for `Reg<CH6_SRC_SPEC>`"]
pub type CH6_SRC = crate::Reg<ch6_src::CH6_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch6_src;
#[doc = "CH6_DST register accessor: an alias for `Reg<CH6_DST_SPEC>`"]
pub type CH6_DST = crate::Reg<ch6_dst::CH6_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch6_dst;
#[doc = "CH6_LINK register accessor: an alias for `Reg<CH6_LINK_SPEC>`"]
pub type CH6_LINK = crate::Reg<ch6_link::CH6_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch6_link;
#[doc = "CH7_REQSEL register accessor: an alias for `Reg<CH7_REQSEL_SPEC>`"]
pub type CH7_REQSEL = crate::Reg<ch7_reqsel::CH7_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch7_reqsel;
#[doc = "CH7_CFG register accessor: an alias for `Reg<CH7_CFG_SPEC>`"]
pub type CH7_CFG = crate::Reg<ch7_cfg::CH7_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch7_cfg;
#[doc = "CH7_LOOP register accessor: an alias for `Reg<CH7_LOOP_SPEC>`"]
pub type CH7_LOOP = crate::Reg<ch7_loop::CH7_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch7_loop;
#[doc = "CH7_CTRL register accessor: an alias for `Reg<CH7_CTRL_SPEC>`"]
pub type CH7_CTRL = crate::Reg<ch7_ctrl::CH7_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch7_ctrl;
#[doc = "CH7_SRC register accessor: an alias for `Reg<CH7_SRC_SPEC>`"]
pub type CH7_SRC = crate::Reg<ch7_src::CH7_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch7_src;
#[doc = "CH7_DST register accessor: an alias for `Reg<CH7_DST_SPEC>`"]
pub type CH7_DST = crate::Reg<ch7_dst::CH7_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch7_dst;
#[doc = "CH7_LINK register accessor: an alias for `Reg<CH7_LINK_SPEC>`"]
pub type CH7_LINK = crate::Reg<ch7_link::CH7_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch7_link;
#[doc = "CH8_REQSEL register accessor: an alias for `Reg<CH8_REQSEL_SPEC>`"]
pub type CH8_REQSEL = crate::Reg<ch8_reqsel::CH8_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch8_reqsel;
#[doc = "CH8_CFG register accessor: an alias for `Reg<CH8_CFG_SPEC>`"]
pub type CH8_CFG = crate::Reg<ch8_cfg::CH8_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch8_cfg;
#[doc = "CH8_LOOP register accessor: an alias for `Reg<CH8_LOOP_SPEC>`"]
pub type CH8_LOOP = crate::Reg<ch8_loop::CH8_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch8_loop;
#[doc = "CH8_CTRL register accessor: an alias for `Reg<CH8_CTRL_SPEC>`"]
pub type CH8_CTRL = crate::Reg<ch8_ctrl::CH8_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch8_ctrl;
#[doc = "CH8_SRC register accessor: an alias for `Reg<CH8_SRC_SPEC>`"]
pub type CH8_SRC = crate::Reg<ch8_src::CH8_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch8_src;
#[doc = "CH8_DST register accessor: an alias for `Reg<CH8_DST_SPEC>`"]
pub type CH8_DST = crate::Reg<ch8_dst::CH8_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch8_dst;
#[doc = "CH8_LINK register accessor: an alias for `Reg<CH8_LINK_SPEC>`"]
pub type CH8_LINK = crate::Reg<ch8_link::CH8_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch8_link;
#[doc = "CH9_REQSEL register accessor: an alias for `Reg<CH9_REQSEL_SPEC>`"]
pub type CH9_REQSEL = crate::Reg<ch9_reqsel::CH9_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch9_reqsel;
#[doc = "CH9_CFG register accessor: an alias for `Reg<CH9_CFG_SPEC>`"]
pub type CH9_CFG = crate::Reg<ch9_cfg::CH9_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch9_cfg;
#[doc = "CH9_LOOP register accessor: an alias for `Reg<CH9_LOOP_SPEC>`"]
pub type CH9_LOOP = crate::Reg<ch9_loop::CH9_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch9_loop;
#[doc = "CH9_CTRL register accessor: an alias for `Reg<CH9_CTRL_SPEC>`"]
pub type CH9_CTRL = crate::Reg<ch9_ctrl::CH9_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch9_ctrl;
#[doc = "CH9_SRC register accessor: an alias for `Reg<CH9_SRC_SPEC>`"]
pub type CH9_SRC = crate::Reg<ch9_src::CH9_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch9_src;
#[doc = "CH9_DST register accessor: an alias for `Reg<CH9_DST_SPEC>`"]
pub type CH9_DST = crate::Reg<ch9_dst::CH9_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch9_dst;
#[doc = "CH9_LINK register accessor: an alias for `Reg<CH9_LINK_SPEC>`"]
pub type CH9_LINK = crate::Reg<ch9_link::CH9_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch9_link;
#[doc = "CH10_REQSEL register accessor: an alias for `Reg<CH10_REQSEL_SPEC>`"]
pub type CH10_REQSEL = crate::Reg<ch10_reqsel::CH10_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch10_reqsel;
#[doc = "CH10_CFG register accessor: an alias for `Reg<CH10_CFG_SPEC>`"]
pub type CH10_CFG = crate::Reg<ch10_cfg::CH10_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch10_cfg;
#[doc = "CH10_LOOP register accessor: an alias for `Reg<CH10_LOOP_SPEC>`"]
pub type CH10_LOOP = crate::Reg<ch10_loop::CH10_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch10_loop;
#[doc = "CH10_CTRL register accessor: an alias for `Reg<CH10_CTRL_SPEC>`"]
pub type CH10_CTRL = crate::Reg<ch10_ctrl::CH10_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch10_ctrl;
#[doc = "CH10_SRC register accessor: an alias for `Reg<CH10_SRC_SPEC>`"]
pub type CH10_SRC = crate::Reg<ch10_src::CH10_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch10_src;
#[doc = "CH10_DST register accessor: an alias for `Reg<CH10_DST_SPEC>`"]
pub type CH10_DST = crate::Reg<ch10_dst::CH10_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch10_dst;
#[doc = "CH10_LINK register accessor: an alias for `Reg<CH10_LINK_SPEC>`"]
pub type CH10_LINK = crate::Reg<ch10_link::CH10_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch10_link;
#[doc = "CH11_REQSEL register accessor: an alias for `Reg<CH11_REQSEL_SPEC>`"]
pub type CH11_REQSEL = crate::Reg<ch11_reqsel::CH11_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch11_reqsel;
#[doc = "CH11_CFG register accessor: an alias for `Reg<CH11_CFG_SPEC>`"]
pub type CH11_CFG = crate::Reg<ch11_cfg::CH11_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch11_cfg;
#[doc = "CH11_LOOP register accessor: an alias for `Reg<CH11_LOOP_SPEC>`"]
pub type CH11_LOOP = crate::Reg<ch11_loop::CH11_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch11_loop;
#[doc = "CH11_CTRL register accessor: an alias for `Reg<CH11_CTRL_SPEC>`"]
pub type CH11_CTRL = crate::Reg<ch11_ctrl::CH11_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch11_ctrl;
#[doc = "CH11_SRC register accessor: an alias for `Reg<CH11_SRC_SPEC>`"]
pub type CH11_SRC = crate::Reg<ch11_src::CH11_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch11_src;
#[doc = "CH11_DST register accessor: an alias for `Reg<CH11_DST_SPEC>`"]
pub type CH11_DST = crate::Reg<ch11_dst::CH11_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch11_dst;
#[doc = "CH11_LINK register accessor: an alias for `Reg<CH11_LINK_SPEC>`"]
pub type CH11_LINK = crate::Reg<ch11_link::CH11_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch11_link;
#[doc = "CH12_REQSEL register accessor: an alias for `Reg<CH12_REQSEL_SPEC>`"]
pub type CH12_REQSEL = crate::Reg<ch12_reqsel::CH12_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch12_reqsel;
#[doc = "CH12_CFG register accessor: an alias for `Reg<CH12_CFG_SPEC>`"]
pub type CH12_CFG = crate::Reg<ch12_cfg::CH12_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch12_cfg;
#[doc = "CH12_LOOP register accessor: an alias for `Reg<CH12_LOOP_SPEC>`"]
pub type CH12_LOOP = crate::Reg<ch12_loop::CH12_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch12_loop;
#[doc = "CH12_CTRL register accessor: an alias for `Reg<CH12_CTRL_SPEC>`"]
pub type CH12_CTRL = crate::Reg<ch12_ctrl::CH12_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch12_ctrl;
#[doc = "CH12_SRC register accessor: an alias for `Reg<CH12_SRC_SPEC>`"]
pub type CH12_SRC = crate::Reg<ch12_src::CH12_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch12_src;
#[doc = "CH12_DST register accessor: an alias for `Reg<CH12_DST_SPEC>`"]
pub type CH12_DST = crate::Reg<ch12_dst::CH12_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch12_dst;
#[doc = "CH12_LINK register accessor: an alias for `Reg<CH12_LINK_SPEC>`"]
pub type CH12_LINK = crate::Reg<ch12_link::CH12_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch12_link;
#[doc = "CH13_REQSEL register accessor: an alias for `Reg<CH13_REQSEL_SPEC>`"]
pub type CH13_REQSEL = crate::Reg<ch13_reqsel::CH13_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch13_reqsel;
#[doc = "CH13_CFG register accessor: an alias for `Reg<CH13_CFG_SPEC>`"]
pub type CH13_CFG = crate::Reg<ch13_cfg::CH13_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch13_cfg;
#[doc = "CH13_LOOP register accessor: an alias for `Reg<CH13_LOOP_SPEC>`"]
pub type CH13_LOOP = crate::Reg<ch13_loop::CH13_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch13_loop;
#[doc = "CH13_CTRL register accessor: an alias for `Reg<CH13_CTRL_SPEC>`"]
pub type CH13_CTRL = crate::Reg<ch13_ctrl::CH13_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch13_ctrl;
#[doc = "CH13_SRC register accessor: an alias for `Reg<CH13_SRC_SPEC>`"]
pub type CH13_SRC = crate::Reg<ch13_src::CH13_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch13_src;
#[doc = "CH13_DST register accessor: an alias for `Reg<CH13_DST_SPEC>`"]
pub type CH13_DST = crate::Reg<ch13_dst::CH13_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch13_dst;
#[doc = "CH13_LINK register accessor: an alias for `Reg<CH13_LINK_SPEC>`"]
pub type CH13_LINK = crate::Reg<ch13_link::CH13_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch13_link;
#[doc = "CH14_REQSEL register accessor: an alias for `Reg<CH14_REQSEL_SPEC>`"]
pub type CH14_REQSEL = crate::Reg<ch14_reqsel::CH14_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch14_reqsel;
#[doc = "CH14_CFG register accessor: an alias for `Reg<CH14_CFG_SPEC>`"]
pub type CH14_CFG = crate::Reg<ch14_cfg::CH14_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch14_cfg;
#[doc = "CH14_LOOP register accessor: an alias for `Reg<CH14_LOOP_SPEC>`"]
pub type CH14_LOOP = crate::Reg<ch14_loop::CH14_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch14_loop;
#[doc = "CH14_CTRL register accessor: an alias for `Reg<CH14_CTRL_SPEC>`"]
pub type CH14_CTRL = crate::Reg<ch14_ctrl::CH14_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch14_ctrl;
#[doc = "CH14_SRC register accessor: an alias for `Reg<CH14_SRC_SPEC>`"]
pub type CH14_SRC = crate::Reg<ch14_src::CH14_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch14_src;
#[doc = "CH14_DST register accessor: an alias for `Reg<CH14_DST_SPEC>`"]
pub type CH14_DST = crate::Reg<ch14_dst::CH14_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch14_dst;
#[doc = "CH14_LINK register accessor: an alias for `Reg<CH14_LINK_SPEC>`"]
pub type CH14_LINK = crate::Reg<ch14_link::CH14_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch14_link;
#[doc = "CH15_REQSEL register accessor: an alias for `Reg<CH15_REQSEL_SPEC>`"]
pub type CH15_REQSEL = crate::Reg<ch15_reqsel::CH15_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch15_reqsel;
#[doc = "CH15_CFG register accessor: an alias for `Reg<CH15_CFG_SPEC>`"]
pub type CH15_CFG = crate::Reg<ch15_cfg::CH15_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch15_cfg;
#[doc = "CH15_LOOP register accessor: an alias for `Reg<CH15_LOOP_SPEC>`"]
pub type CH15_LOOP = crate::Reg<ch15_loop::CH15_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch15_loop;
#[doc = "CH15_CTRL register accessor: an alias for `Reg<CH15_CTRL_SPEC>`"]
pub type CH15_CTRL = crate::Reg<ch15_ctrl::CH15_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch15_ctrl;
#[doc = "CH15_SRC register accessor: an alias for `Reg<CH15_SRC_SPEC>`"]
pub type CH15_SRC = crate::Reg<ch15_src::CH15_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch15_src;
#[doc = "CH15_DST register accessor: an alias for `Reg<CH15_DST_SPEC>`"]
pub type CH15_DST = crate::Reg<ch15_dst::CH15_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch15_dst;
#[doc = "CH15_LINK register accessor: an alias for `Reg<CH15_LINK_SPEC>`"]
pub type CH15_LINK = crate::Reg<ch15_link::CH15_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch15_link;
#[doc = "CH16_REQSEL register accessor: an alias for `Reg<CH16_REQSEL_SPEC>`"]
pub type CH16_REQSEL = crate::Reg<ch16_reqsel::CH16_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch16_reqsel;
#[doc = "CH16_CFG register accessor: an alias for `Reg<CH16_CFG_SPEC>`"]
pub type CH16_CFG = crate::Reg<ch16_cfg::CH16_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch16_cfg;
#[doc = "CH16_LOOP register accessor: an alias for `Reg<CH16_LOOP_SPEC>`"]
pub type CH16_LOOP = crate::Reg<ch16_loop::CH16_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch16_loop;
#[doc = "CH16_CTRL register accessor: an alias for `Reg<CH16_CTRL_SPEC>`"]
pub type CH16_CTRL = crate::Reg<ch16_ctrl::CH16_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch16_ctrl;
#[doc = "CH16_SRC register accessor: an alias for `Reg<CH16_SRC_SPEC>`"]
pub type CH16_SRC = crate::Reg<ch16_src::CH16_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch16_src;
#[doc = "CH16_DST register accessor: an alias for `Reg<CH16_DST_SPEC>`"]
pub type CH16_DST = crate::Reg<ch16_dst::CH16_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch16_dst;
#[doc = "CH16_LINK register accessor: an alias for `Reg<CH16_LINK_SPEC>`"]
pub type CH16_LINK = crate::Reg<ch16_link::CH16_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch16_link;
#[doc = "CH17_REQSEL register accessor: an alias for `Reg<CH17_REQSEL_SPEC>`"]
pub type CH17_REQSEL = crate::Reg<ch17_reqsel::CH17_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch17_reqsel;
#[doc = "CH17_CFG register accessor: an alias for `Reg<CH17_CFG_SPEC>`"]
pub type CH17_CFG = crate::Reg<ch17_cfg::CH17_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch17_cfg;
#[doc = "CH17_LOOP register accessor: an alias for `Reg<CH17_LOOP_SPEC>`"]
pub type CH17_LOOP = crate::Reg<ch17_loop::CH17_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch17_loop;
#[doc = "CH17_CTRL register accessor: an alias for `Reg<CH17_CTRL_SPEC>`"]
pub type CH17_CTRL = crate::Reg<ch17_ctrl::CH17_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch17_ctrl;
#[doc = "CH17_SRC register accessor: an alias for `Reg<CH17_SRC_SPEC>`"]
pub type CH17_SRC = crate::Reg<ch17_src::CH17_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch17_src;
#[doc = "CH17_DST register accessor: an alias for `Reg<CH17_DST_SPEC>`"]
pub type CH17_DST = crate::Reg<ch17_dst::CH17_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch17_dst;
#[doc = "CH17_LINK register accessor: an alias for `Reg<CH17_LINK_SPEC>`"]
pub type CH17_LINK = crate::Reg<ch17_link::CH17_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch17_link;
#[doc = "CH18_REQSEL register accessor: an alias for `Reg<CH18_REQSEL_SPEC>`"]
pub type CH18_REQSEL = crate::Reg<ch18_reqsel::CH18_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch18_reqsel;
#[doc = "CH18_CFG register accessor: an alias for `Reg<CH18_CFG_SPEC>`"]
pub type CH18_CFG = crate::Reg<ch18_cfg::CH18_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch18_cfg;
#[doc = "CH18_LOOP register accessor: an alias for `Reg<CH18_LOOP_SPEC>`"]
pub type CH18_LOOP = crate::Reg<ch18_loop::CH18_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch18_loop;
#[doc = "CH18_CTRL register accessor: an alias for `Reg<CH18_CTRL_SPEC>`"]
pub type CH18_CTRL = crate::Reg<ch18_ctrl::CH18_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch18_ctrl;
#[doc = "CH18_SRC register accessor: an alias for `Reg<CH18_SRC_SPEC>`"]
pub type CH18_SRC = crate::Reg<ch18_src::CH18_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch18_src;
#[doc = "CH18_DST register accessor: an alias for `Reg<CH18_DST_SPEC>`"]
pub type CH18_DST = crate::Reg<ch18_dst::CH18_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch18_dst;
#[doc = "CH18_LINK register accessor: an alias for `Reg<CH18_LINK_SPEC>`"]
pub type CH18_LINK = crate::Reg<ch18_link::CH18_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch18_link;
#[doc = "CH19_REQSEL register accessor: an alias for `Reg<CH19_REQSEL_SPEC>`"]
pub type CH19_REQSEL = crate::Reg<ch19_reqsel::CH19_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch19_reqsel;
#[doc = "CH19_CFG register accessor: an alias for `Reg<CH19_CFG_SPEC>`"]
pub type CH19_CFG = crate::Reg<ch19_cfg::CH19_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch19_cfg;
#[doc = "CH19_LOOP register accessor: an alias for `Reg<CH19_LOOP_SPEC>`"]
pub type CH19_LOOP = crate::Reg<ch19_loop::CH19_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch19_loop;
#[doc = "CH19_CTRL register accessor: an alias for `Reg<CH19_CTRL_SPEC>`"]
pub type CH19_CTRL = crate::Reg<ch19_ctrl::CH19_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch19_ctrl;
#[doc = "CH19_SRC register accessor: an alias for `Reg<CH19_SRC_SPEC>`"]
pub type CH19_SRC = crate::Reg<ch19_src::CH19_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch19_src;
#[doc = "CH19_DST register accessor: an alias for `Reg<CH19_DST_SPEC>`"]
pub type CH19_DST = crate::Reg<ch19_dst::CH19_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch19_dst;
#[doc = "CH19_LINK register accessor: an alias for `Reg<CH19_LINK_SPEC>`"]
pub type CH19_LINK = crate::Reg<ch19_link::CH19_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch19_link;
#[doc = "CH20_REQSEL register accessor: an alias for `Reg<CH20_REQSEL_SPEC>`"]
pub type CH20_REQSEL = crate::Reg<ch20_reqsel::CH20_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch20_reqsel;
#[doc = "CH20_CFG register accessor: an alias for `Reg<CH20_CFG_SPEC>`"]
pub type CH20_CFG = crate::Reg<ch20_cfg::CH20_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch20_cfg;
#[doc = "CH20_LOOP register accessor: an alias for `Reg<CH20_LOOP_SPEC>`"]
pub type CH20_LOOP = crate::Reg<ch20_loop::CH20_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch20_loop;
#[doc = "CH20_CTRL register accessor: an alias for `Reg<CH20_CTRL_SPEC>`"]
pub type CH20_CTRL = crate::Reg<ch20_ctrl::CH20_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch20_ctrl;
#[doc = "CH20_SRC register accessor: an alias for `Reg<CH20_SRC_SPEC>`"]
pub type CH20_SRC = crate::Reg<ch20_src::CH20_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch20_src;
#[doc = "CH20_DST register accessor: an alias for `Reg<CH20_DST_SPEC>`"]
pub type CH20_DST = crate::Reg<ch20_dst::CH20_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch20_dst;
#[doc = "CH20_LINK register accessor: an alias for `Reg<CH20_LINK_SPEC>`"]
pub type CH20_LINK = crate::Reg<ch20_link::CH20_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch20_link;
#[doc = "CH21_REQSEL register accessor: an alias for `Reg<CH21_REQSEL_SPEC>`"]
pub type CH21_REQSEL = crate::Reg<ch21_reqsel::CH21_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch21_reqsel;
#[doc = "CH21_CFG register accessor: an alias for `Reg<CH21_CFG_SPEC>`"]
pub type CH21_CFG = crate::Reg<ch21_cfg::CH21_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch21_cfg;
#[doc = "CH21_LOOP register accessor: an alias for `Reg<CH21_LOOP_SPEC>`"]
pub type CH21_LOOP = crate::Reg<ch21_loop::CH21_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch21_loop;
#[doc = "CH21_CTRL register accessor: an alias for `Reg<CH21_CTRL_SPEC>`"]
pub type CH21_CTRL = crate::Reg<ch21_ctrl::CH21_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch21_ctrl;
#[doc = "CH21_SRC register accessor: an alias for `Reg<CH21_SRC_SPEC>`"]
pub type CH21_SRC = crate::Reg<ch21_src::CH21_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch21_src;
#[doc = "CH21_DST register accessor: an alias for `Reg<CH21_DST_SPEC>`"]
pub type CH21_DST = crate::Reg<ch21_dst::CH21_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch21_dst;
#[doc = "CH21_LINK register accessor: an alias for `Reg<CH21_LINK_SPEC>`"]
pub type CH21_LINK = crate::Reg<ch21_link::CH21_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch21_link;
#[doc = "CH22_REQSEL register accessor: an alias for `Reg<CH22_REQSEL_SPEC>`"]
pub type CH22_REQSEL = crate::Reg<ch22_reqsel::CH22_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch22_reqsel;
#[doc = "CH22_CFG register accessor: an alias for `Reg<CH22_CFG_SPEC>`"]
pub type CH22_CFG = crate::Reg<ch22_cfg::CH22_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch22_cfg;
#[doc = "CH22_LOOP register accessor: an alias for `Reg<CH22_LOOP_SPEC>`"]
pub type CH22_LOOP = crate::Reg<ch22_loop::CH22_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch22_loop;
#[doc = "CH22_CTRL register accessor: an alias for `Reg<CH22_CTRL_SPEC>`"]
pub type CH22_CTRL = crate::Reg<ch22_ctrl::CH22_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch22_ctrl;
#[doc = "CH22_SRC register accessor: an alias for `Reg<CH22_SRC_SPEC>`"]
pub type CH22_SRC = crate::Reg<ch22_src::CH22_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch22_src;
#[doc = "CH22_DST register accessor: an alias for `Reg<CH22_DST_SPEC>`"]
pub type CH22_DST = crate::Reg<ch22_dst::CH22_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch22_dst;
#[doc = "CH22_LINK register accessor: an alias for `Reg<CH22_LINK_SPEC>`"]
pub type CH22_LINK = crate::Reg<ch22_link::CH22_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch22_link;
#[doc = "CH23_REQSEL register accessor: an alias for `Reg<CH23_REQSEL_SPEC>`"]
pub type CH23_REQSEL = crate::Reg<ch23_reqsel::CH23_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch23_reqsel;
#[doc = "CH23_CFG register accessor: an alias for `Reg<CH23_CFG_SPEC>`"]
pub type CH23_CFG = crate::Reg<ch23_cfg::CH23_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch23_cfg;
#[doc = "CH23_LOOP register accessor: an alias for `Reg<CH23_LOOP_SPEC>`"]
pub type CH23_LOOP = crate::Reg<ch23_loop::CH23_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch23_loop;
#[doc = "CH23_CTRL register accessor: an alias for `Reg<CH23_CTRL_SPEC>`"]
pub type CH23_CTRL = crate::Reg<ch23_ctrl::CH23_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch23_ctrl;
#[doc = "CH23_SRC register accessor: an alias for `Reg<CH23_SRC_SPEC>`"]
pub type CH23_SRC = crate::Reg<ch23_src::CH23_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch23_src;
#[doc = "CH23_DST register accessor: an alias for `Reg<CH23_DST_SPEC>`"]
pub type CH23_DST = crate::Reg<ch23_dst::CH23_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch23_dst;
#[doc = "CH23_LINK register accessor: an alias for `Reg<CH23_LINK_SPEC>`"]
pub type CH23_LINK = crate::Reg<ch23_link::CH23_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch23_link;
