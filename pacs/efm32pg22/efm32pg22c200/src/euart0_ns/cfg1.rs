#[doc = "Register `CFG1` reader"]
pub struct R(crate::R<CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG1` writer"]
pub struct W(crate::W<CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Debug halt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGHALT_A {
    #[doc = "0: Continue normal UART operation even if core is halted"]
    DISABLE = 0,
    #[doc = "1: If core is halted, receive one frame and then halt reception by deactivating RTS. Next frame reception happens when the core is unhalted during single stepping."]
    ENABLE = 1,
}
impl From<DBGHALT_A> for bool {
    #[inline(always)]
    fn from(variant: DBGHALT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGHALT` reader - Debug halt"]
pub type DBGHALT_R = crate::BitReader<DBGHALT_A>;
impl DBGHALT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGHALT_A {
        match self.bits {
            false => DBGHALT_A::DISABLE,
            true => DBGHALT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DBGHALT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DBGHALT_A::ENABLE
    }
}
#[doc = "Field `DBGHALT` writer - Debug halt"]
pub type DBGHALT_W<'a> = crate::BitWriter<'a, u32, CFG1_SPEC, DBGHALT_A, 0>;
impl<'a> DBGHALT_W<'a> {
    #[doc = "Continue normal UART operation even if core is halted"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DBGHALT_A::DISABLE)
    }
    #[doc = "If core is halted, receive one frame and then halt reception by deactivating RTS. Next frame reception happens when the core is unhalted during single stepping."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DBGHALT_A::ENABLE)
    }
}
#[doc = "Clear-to-send Invert Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSINV_A {
    #[doc = "0: The CTS pin is active low"]
    DISABLE = 0,
    #[doc = "1: The CTS pin is active high"]
    ENABLE = 1,
}
impl From<CTSINV_A> for bool {
    #[inline(always)]
    fn from(variant: CTSINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSINV` reader - Clear-to-send Invert Enable"]
pub type CTSINV_R = crate::BitReader<CTSINV_A>;
impl CTSINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSINV_A {
        match self.bits {
            false => CTSINV_A::DISABLE,
            true => CTSINV_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTSINV_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTSINV_A::ENABLE
    }
}
#[doc = "Field `CTSINV` writer - Clear-to-send Invert Enable"]
pub type CTSINV_W<'a> = crate::BitWriter<'a, u32, CFG1_SPEC, CTSINV_A, 1>;
impl<'a> CTSINV_W<'a> {
    #[doc = "The CTS pin is active low"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTSINV_A::DISABLE)
    }
    #[doc = "The CTS pin is active high"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTSINV_A::ENABLE)
    }
}
#[doc = "Clear-to-send Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSEN_A {
    #[doc = "0: Ignore CTS"]
    DISABLE = 0,
    #[doc = "1: Stop transmitting when CTS is inactive"]
    ENABLE = 1,
}
impl From<CTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSEN` reader - Clear-to-send Enable"]
pub type CTSEN_R = crate::BitReader<CTSEN_A>;
impl CTSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSEN_A {
        match self.bits {
            false => CTSEN_A::DISABLE,
            true => CTSEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTSEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTSEN_A::ENABLE
    }
}
#[doc = "Field `CTSEN` writer - Clear-to-send Enable"]
pub type CTSEN_W<'a> = crate::BitWriter<'a, u32, CFG1_SPEC, CTSEN_A, 2>;
impl<'a> CTSEN_W<'a> {
    #[doc = "Ignore CTS"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTSEN_A::DISABLE)
    }
    #[doc = "Stop transmitting when CTS is inactive"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTSEN_A::ENABLE)
    }
}
#[doc = "Request-to-send Invert Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSINV_A {
    #[doc = "0: The RTS pin is active low"]
    DISABLE = 0,
    #[doc = "1: The RTS pin is active high"]
    ENABLE = 1,
}
impl From<RTSINV_A> for bool {
    #[inline(always)]
    fn from(variant: RTSINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSINV` reader - Request-to-send Invert Enable"]
pub type RTSINV_R = crate::BitReader<RTSINV_A>;
impl RTSINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTSINV_A {
        match self.bits {
            false => RTSINV_A::DISABLE,
            true => RTSINV_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RTSINV_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTSINV_A::ENABLE
    }
}
#[doc = "Field `RTSINV` writer - Request-to-send Invert Enable"]
pub type RTSINV_W<'a> = crate::BitWriter<'a, u32, CFG1_SPEC, RTSINV_A, 3>;
impl<'a> RTSINV_W<'a> {
    #[doc = "The RTS pin is active low"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RTSINV_A::DISABLE)
    }
    #[doc = "The RTS pin is active high"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTSINV_A::ENABLE)
    }
}
#[doc = "Field `TXDMAWU` reader - Transmitter DMA Wakeup"]
pub type TXDMAWU_R = crate::BitReader<bool>;
#[doc = "Field `TXDMAWU` writer - Transmitter DMA Wakeup"]
pub type TXDMAWU_W<'a> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, 9>;
#[doc = "Field `RXDMAWU` reader - Receiver DMA Wakeup"]
pub type RXDMAWU_R = crate::BitReader<bool>;
#[doc = "Field `RXDMAWU` writer - Receiver DMA Wakeup"]
pub type RXDMAWU_W<'a> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, 10>;
#[doc = "Field `SFUBRX` reader - Start Frame Unblock Receiver"]
pub type SFUBRX_R = crate::BitReader<bool>;
#[doc = "Field `SFUBRX` writer - Start Frame Unblock Receiver"]
pub type SFUBRX_W<'a> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, 11>;
#[doc = "Field `RXPRSEN` reader - PRS RX Enable"]
pub type RXPRSEN_R = crate::BitReader<bool>;
#[doc = "Field `RXPRSEN` writer - PRS RX Enable"]
pub type RXPRSEN_W<'a> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, 15>;
#[doc = "TX FIFO Interrupt Watermark\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXFIW_A {
    #[doc = "0: TXFL status flag and IF are set when the TX FIFO has space for at least one more frame."]
    ONEFRAME = 0,
    #[doc = "1: TXFL status flag and IF are set when the TX FIFO has space for at least two more frames."]
    TWOFRAMES = 1,
    #[doc = "2: TXFL status flag and IF are set when the TX FIFO has space for at least three more frames."]
    THREEFRAMES = 2,
    #[doc = "3: TXFL status flag and IF are set when the TX FIFO has space for at least four more frames."]
    FOURFRAMES = 3,
}
impl From<TXFIW_A> for u8 {
    #[inline(always)]
    fn from(variant: TXFIW_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TXFIW` reader - TX FIFO Interrupt Watermark"]
pub type TXFIW_R = crate::FieldReader<u8, TXFIW_A>;
impl TXFIW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFIW_A {
        match self.bits {
            0 => TXFIW_A::ONEFRAME,
            1 => TXFIW_A::TWOFRAMES,
            2 => TXFIW_A::THREEFRAMES,
            3 => TXFIW_A::FOURFRAMES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONEFRAME`"]
    #[inline(always)]
    pub fn is_oneframe(&self) -> bool {
        *self == TXFIW_A::ONEFRAME
    }
    #[doc = "Checks if the value of the field is `TWOFRAMES`"]
    #[inline(always)]
    pub fn is_twoframes(&self) -> bool {
        *self == TXFIW_A::TWOFRAMES
    }
    #[doc = "Checks if the value of the field is `THREEFRAMES`"]
    #[inline(always)]
    pub fn is_threeframes(&self) -> bool {
        *self == TXFIW_A::THREEFRAMES
    }
    #[doc = "Checks if the value of the field is `FOURFRAMES`"]
    #[inline(always)]
    pub fn is_fourframes(&self) -> bool {
        *self == TXFIW_A::FOURFRAMES
    }
}
#[doc = "Field `TXFIW` writer - TX FIFO Interrupt Watermark"]
pub type TXFIW_W<'a> = crate::FieldWriterSafe<'a, u32, CFG1_SPEC, u8, TXFIW_A, 2, 16>;
impl<'a> TXFIW_W<'a> {
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least one more frame."]
    #[inline(always)]
    pub fn oneframe(self) -> &'a mut W {
        self.variant(TXFIW_A::ONEFRAME)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least two more frames."]
    #[inline(always)]
    pub fn twoframes(self) -> &'a mut W {
        self.variant(TXFIW_A::TWOFRAMES)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least three more frames."]
    #[inline(always)]
    pub fn threeframes(self) -> &'a mut W {
        self.variant(TXFIW_A::THREEFRAMES)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least four more frames."]
    #[inline(always)]
    pub fn fourframes(self) -> &'a mut W {
        self.variant(TXFIW_A::FOURFRAMES)
    }
}
#[doc = "RX FIFO Interrupt Watermark\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXFIW_A {
    #[doc = "0: RXFL status flag and IF are set when the RX FIFO has at least one frame in it."]
    ONEFRAME = 0,
    #[doc = "1: RXFL status flag and IF are set when the RX FIFO has at least two frames in it."]
    TWOFRAMES = 1,
    #[doc = "2: RXFL status flag and IF are set when the RX FIFO has at least three frames in it."]
    THREEFRAMES = 2,
    #[doc = "3: RXFL status flag and IF are set when the RX FIFO has four frames in it."]
    FOURFRAMES = 3,
}
impl From<RXFIW_A> for u8 {
    #[inline(always)]
    fn from(variant: RXFIW_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RXFIW` reader - RX FIFO Interrupt Watermark"]
pub type RXFIW_R = crate::FieldReader<u8, RXFIW_A>;
impl RXFIW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFIW_A {
        match self.bits {
            0 => RXFIW_A::ONEFRAME,
            1 => RXFIW_A::TWOFRAMES,
            2 => RXFIW_A::THREEFRAMES,
            3 => RXFIW_A::FOURFRAMES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONEFRAME`"]
    #[inline(always)]
    pub fn is_oneframe(&self) -> bool {
        *self == RXFIW_A::ONEFRAME
    }
    #[doc = "Checks if the value of the field is `TWOFRAMES`"]
    #[inline(always)]
    pub fn is_twoframes(&self) -> bool {
        *self == RXFIW_A::TWOFRAMES
    }
    #[doc = "Checks if the value of the field is `THREEFRAMES`"]
    #[inline(always)]
    pub fn is_threeframes(&self) -> bool {
        *self == RXFIW_A::THREEFRAMES
    }
    #[doc = "Checks if the value of the field is `FOURFRAMES`"]
    #[inline(always)]
    pub fn is_fourframes(&self) -> bool {
        *self == RXFIW_A::FOURFRAMES
    }
}
#[doc = "Field `RXFIW` writer - RX FIFO Interrupt Watermark"]
pub type RXFIW_W<'a> = crate::FieldWriterSafe<'a, u32, CFG1_SPEC, u8, RXFIW_A, 2, 19>;
impl<'a> RXFIW_W<'a> {
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least one frame in it."]
    #[inline(always)]
    pub fn oneframe(self) -> &'a mut W {
        self.variant(RXFIW_A::ONEFRAME)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least two frames in it."]
    #[inline(always)]
    pub fn twoframes(self) -> &'a mut W {
        self.variant(RXFIW_A::TWOFRAMES)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least three frames in it."]
    #[inline(always)]
    pub fn threeframes(self) -> &'a mut W {
        self.variant(RXFIW_A::THREEFRAMES)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has four frames in it."]
    #[inline(always)]
    pub fn fourframes(self) -> &'a mut W {
        self.variant(RXFIW_A::FOURFRAMES)
    }
}
#[doc = "Request-to-send RX FIFO Watermark\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTSRXFW_A {
    #[doc = "0: RTS is set if there is space for at least one more frame in the RX FIFO."]
    ONEFRAME = 0,
    #[doc = "1: RTS is set if there is space for at least two more frames in the RX FIFO."]
    TWOFRAMES = 1,
    #[doc = "2: RTS is set if there is space for at least three more frames in the RX FIFO."]
    THREEFRAMES = 2,
    #[doc = "3: RTS is set if there is space for four more frames in the RX FIFO."]
    FOURFRAMES = 3,
}
impl From<RTSRXFW_A> for u8 {
    #[inline(always)]
    fn from(variant: RTSRXFW_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RTSRXFW` reader - Request-to-send RX FIFO Watermark"]
pub type RTSRXFW_R = crate::FieldReader<u8, RTSRXFW_A>;
impl RTSRXFW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTSRXFW_A {
        match self.bits {
            0 => RTSRXFW_A::ONEFRAME,
            1 => RTSRXFW_A::TWOFRAMES,
            2 => RTSRXFW_A::THREEFRAMES,
            3 => RTSRXFW_A::FOURFRAMES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONEFRAME`"]
    #[inline(always)]
    pub fn is_oneframe(&self) -> bool {
        *self == RTSRXFW_A::ONEFRAME
    }
    #[doc = "Checks if the value of the field is `TWOFRAMES`"]
    #[inline(always)]
    pub fn is_twoframes(&self) -> bool {
        *self == RTSRXFW_A::TWOFRAMES
    }
    #[doc = "Checks if the value of the field is `THREEFRAMES`"]
    #[inline(always)]
    pub fn is_threeframes(&self) -> bool {
        *self == RTSRXFW_A::THREEFRAMES
    }
    #[doc = "Checks if the value of the field is `FOURFRAMES`"]
    #[inline(always)]
    pub fn is_fourframes(&self) -> bool {
        *self == RTSRXFW_A::FOURFRAMES
    }
}
#[doc = "Field `RTSRXFW` writer - Request-to-send RX FIFO Watermark"]
pub type RTSRXFW_W<'a> = crate::FieldWriterSafe<'a, u32, CFG1_SPEC, u8, RTSRXFW_A, 2, 22>;
impl<'a> RTSRXFW_W<'a> {
    #[doc = "RTS is set if there is space for at least one more frame in the RX FIFO."]
    #[inline(always)]
    pub fn oneframe(self) -> &'a mut W {
        self.variant(RTSRXFW_A::ONEFRAME)
    }
    #[doc = "RTS is set if there is space for at least two more frames in the RX FIFO."]
    #[inline(always)]
    pub fn twoframes(self) -> &'a mut W {
        self.variant(RTSRXFW_A::TWOFRAMES)
    }
    #[doc = "RTS is set if there is space for at least three more frames in the RX FIFO."]
    #[inline(always)]
    pub fn threeframes(self) -> &'a mut W {
        self.variant(RTSRXFW_A::THREEFRAMES)
    }
    #[doc = "RTS is set if there is space for four more frames in the RX FIFO."]
    #[inline(always)]
    pub fn fourframes(self) -> &'a mut W {
        self.variant(RTSRXFW_A::FOURFRAMES)
    }
}
impl R {
    #[doc = "Bit 0 - Debug halt"]
    #[inline(always)]
    pub fn dbghalt(&self) -> DBGHALT_R {
        DBGHALT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear-to-send Invert Enable"]
    #[inline(always)]
    pub fn ctsinv(&self) -> CTSINV_R {
        CTSINV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear-to-send Enable"]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Request-to-send Invert Enable"]
    #[inline(always)]
    pub fn rtsinv(&self) -> RTSINV_R {
        RTSINV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmitter DMA Wakeup"]
    #[inline(always)]
    pub fn txdmawu(&self) -> TXDMAWU_R {
        TXDMAWU_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receiver DMA Wakeup"]
    #[inline(always)]
    pub fn rxdmawu(&self) -> RXDMAWU_R {
        RXDMAWU_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Start Frame Unblock Receiver"]
    #[inline(always)]
    pub fn sfubrx(&self) -> SFUBRX_R {
        SFUBRX_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - PRS RX Enable"]
    #[inline(always)]
    pub fn rxprsen(&self) -> RXPRSEN_R {
        RXPRSEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - TX FIFO Interrupt Watermark"]
    #[inline(always)]
    pub fn txfiw(&self) -> TXFIW_R {
        TXFIW_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 19:20 - RX FIFO Interrupt Watermark"]
    #[inline(always)]
    pub fn rxfiw(&self) -> RXFIW_R {
        RXFIW_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Request-to-send RX FIFO Watermark"]
    #[inline(always)]
    pub fn rtsrxfw(&self) -> RTSRXFW_R {
        RTSRXFW_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Debug halt"]
    #[inline(always)]
    pub fn dbghalt(&mut self) -> DBGHALT_W {
        DBGHALT_W::new(self)
    }
    #[doc = "Bit 1 - Clear-to-send Invert Enable"]
    #[inline(always)]
    pub fn ctsinv(&mut self) -> CTSINV_W {
        CTSINV_W::new(self)
    }
    #[doc = "Bit 2 - Clear-to-send Enable"]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CTSEN_W {
        CTSEN_W::new(self)
    }
    #[doc = "Bit 3 - Request-to-send Invert Enable"]
    #[inline(always)]
    pub fn rtsinv(&mut self) -> RTSINV_W {
        RTSINV_W::new(self)
    }
    #[doc = "Bit 9 - Transmitter DMA Wakeup"]
    #[inline(always)]
    pub fn txdmawu(&mut self) -> TXDMAWU_W {
        TXDMAWU_W::new(self)
    }
    #[doc = "Bit 10 - Receiver DMA Wakeup"]
    #[inline(always)]
    pub fn rxdmawu(&mut self) -> RXDMAWU_W {
        RXDMAWU_W::new(self)
    }
    #[doc = "Bit 11 - Start Frame Unblock Receiver"]
    #[inline(always)]
    pub fn sfubrx(&mut self) -> SFUBRX_W {
        SFUBRX_W::new(self)
    }
    #[doc = "Bit 15 - PRS RX Enable"]
    #[inline(always)]
    pub fn rxprsen(&mut self) -> RXPRSEN_W {
        RXPRSEN_W::new(self)
    }
    #[doc = "Bits 16:17 - TX FIFO Interrupt Watermark"]
    #[inline(always)]
    pub fn txfiw(&mut self) -> TXFIW_W {
        TXFIW_W::new(self)
    }
    #[doc = "Bits 19:20 - RX FIFO Interrupt Watermark"]
    #[inline(always)]
    pub fn rxfiw(&mut self) -> RXFIW_W {
        RXFIW_W::new(self)
    }
    #[doc = "Bits 22:23 - Request-to-send RX FIFO Watermark"]
    #[inline(always)]
    pub fn rtsrxfw(&mut self) -> RTSRXFW_W {
        RTSRXFW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg1](index.html) module"]
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg1::R](R) reader structure"]
impl crate::Readable for CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg1::W](W) writer structure"]
impl crate::Writable for CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
