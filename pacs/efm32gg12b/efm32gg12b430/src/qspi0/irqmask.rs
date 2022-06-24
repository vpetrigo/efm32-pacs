#[doc = "Register `IRQMASK` reader"]
pub struct R(crate::R<IRQMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQMASK` writer"]
pub struct W(crate::W<IRQMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQMASK_SPEC>;
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
impl From<crate::W<IRQMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODEMFAILMASK` reader - Mode M Failure Mask"]
pub type MODEMFAILMASK_R = crate::BitReader<bool>;
#[doc = "Field `MODEMFAILMASK` writer - Mode M Failure Mask"]
pub type MODEMFAILMASK_W<'a> = crate::BitWriter<'a, u32, IRQMASK_SPEC, bool, 0>;
#[doc = "Field `UNDERFLOWDETMASK` reader - Underflow Detected Mask"]
pub type UNDERFLOWDETMASK_R = crate::BitReader<bool>;
#[doc = "Field `UNDERFLOWDETMASK` writer - Underflow Detected Mask"]
pub type UNDERFLOWDETMASK_W<'a> = crate::BitWriter<'a, u32, IRQMASK_SPEC, bool, 1>;
#[doc = "Field `INDIRECTOPDONEMASK` reader - Indirect Complete Mask"]
pub type INDIRECTOPDONEMASK_R = crate::BitReader<bool>;
#[doc = "Field `INDIRECTOPDONEMASK` writer - Indirect Complete Mask"]
pub type INDIRECTOPDONEMASK_W<'a> = crate::BitWriter<'a, u32, IRQMASK_SPEC, bool, 2>;
#[doc = "Field `INDIRECTREADREJECTMASK` reader - Indirect Read Reject Mask"]
pub type INDIRECTREADREJECTMASK_R = crate::BitReader<bool>;
#[doc = "Field `INDIRECTREADREJECTMASK` writer - Indirect Read Reject Mask"]
pub type INDIRECTREADREJECTMASK_W<'a> = crate::BitWriter<'a, u32, IRQMASK_SPEC, bool, 3>;
#[doc = "Field `PROTWRATTEMPTMASK` reader - Protected Area Write Attempt Mask"]
pub type PROTWRATTEMPTMASK_R = crate::BitReader<bool>;
#[doc = "Field `PROTWRATTEMPTMASK` writer - Protected Area Write Attempt Mask"]
pub type PROTWRATTEMPTMASK_W<'a> = crate::BitWriter<'a, u32, IRQMASK_SPEC, bool, 4>;
#[doc = "Field `ILLEGALACCESSDETMASK` reader - Illegal Access Detected Mask"]
pub type ILLEGALACCESSDETMASK_R = crate::BitReader<bool>;
#[doc = "Field `ILLEGALACCESSDETMASK` writer - Illegal Access Detected Mask"]
pub type ILLEGALACCESSDETMASK_W<'a> = crate::BitWriter<'a, u32, IRQMASK_SPEC, bool, 5>;
#[doc = "Field `INDIRECTXFERLEVELBREACHMASK` reader - Transfer Watermark Breach Mask"]
pub type INDIRECTXFERLEVELBREACHMASK_R = crate::BitReader<bool>;
#[doc = "Field `INDIRECTXFERLEVELBREACHMASK` writer - Transfer Watermark Breach Mask"]
pub type INDIRECTXFERLEVELBREACHMASK_W<'a> = crate::BitWriter<'a, u32, IRQMASK_SPEC, bool, 6>;
#[doc = "Field `RECVOVERFLOWMASK` reader - Receive Overflow Mask"]
pub type RECVOVERFLOWMASK_R = crate::BitReader<bool>;
#[doc = "Field `RECVOVERFLOWMASK` writer - Receive Overflow Mask"]
pub type RECVOVERFLOWMASK_W<'a> = crate::BitWriter<'a, u32, IRQMASK_SPEC, bool, 7>;
#[doc = "Field `TXFIFONOTFULLMASK` reader - Small TX FIFO Not Full Mask"]
pub type TXFIFONOTFULLMASK_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFONOTFULLMASK` writer - Small TX FIFO Not Full Mask"]
pub type TXFIFONOTFULLMASK_W<'a> = crate::BitWriter<'a, u32, IRQMASK_SPEC, bool, 8>;
#[doc = "Field `TXFIFOFULLMASK` reader - Small TX FIFO Full Mask"]
pub type TXFIFOFULLMASK_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFOFULLMASK` writer - Small TX FIFO Full Mask"]
pub type TXFIFOFULLMASK_W<'a> = crate::BitWriter<'a, u32, IRQMASK_SPEC, bool, 9>;
#[doc = "Field `RXFIFONOTEMPTYMASK` reader - Small RX FIFO Not Empty Mask"]
pub type RXFIFONOTEMPTYMASK_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFONOTEMPTYMASK` writer - Small RX FIFO Not Empty Mask"]
pub type RXFIFONOTEMPTYMASK_W<'a> = crate::BitWriter<'a, u32, IRQMASK_SPEC, bool, 10>;
#[doc = "Field `RXFIFOFULLMASK` reader - Small RX FIFO Full Mask"]
pub type RXFIFOFULLMASK_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFOFULLMASK` writer - Small RX FIFO Full Mask"]
pub type RXFIFOFULLMASK_W<'a> = crate::BitWriter<'a, u32, IRQMASK_SPEC, bool, 11>;
#[doc = "Field `INDRDSRAMFULLMASK` reader - Indirect Read Partition Overflow Mask"]
pub type INDRDSRAMFULLMASK_R = crate::BitReader<bool>;
#[doc = "Field `INDRDSRAMFULLMASK` writer - Indirect Read Partition Overflow Mask"]
pub type INDRDSRAMFULLMASK_W<'a> = crate::BitWriter<'a, u32, IRQMASK_SPEC, bool, 12>;
#[doc = "Field `POLLEXPINTMASK` reader - Polling Expiration Detected Mask"]
pub type POLLEXPINTMASK_R = crate::BitReader<bool>;
#[doc = "Field `POLLEXPINTMASK` writer - Polling Expiration Detected Mask"]
pub type POLLEXPINTMASK_W<'a> = crate::BitWriter<'a, u32, IRQMASK_SPEC, bool, 13>;
#[doc = "Field `STIGREQMASK` reader - STIG Request Completion Mask"]
pub type STIGREQMASK_R = crate::BitReader<bool>;
#[doc = "Field `STIGREQMASK` writer - STIG Request Completion Mask"]
pub type STIGREQMASK_W<'a> = crate::BitWriter<'a, u32, IRQMASK_SPEC, bool, 14>;
#[doc = "Field `RXCRCDATAERRMASK` reader - RX CRC Data Error Mask"]
pub type RXCRCDATAERRMASK_R = crate::BitReader<bool>;
#[doc = "Field `RXCRCDATAERRMASK` writer - RX CRC Data Error Mask"]
pub type RXCRCDATAERRMASK_W<'a> = crate::BitWriter<'a, u32, IRQMASK_SPEC, bool, 16>;
#[doc = "Field `RXCRCDATAVALMASK` reader - RX CRC Data Valid Mask"]
pub type RXCRCDATAVALMASK_R = crate::BitReader<bool>;
#[doc = "Field `RXCRCDATAVALMASK` writer - RX CRC Data Valid Mask"]
pub type RXCRCDATAVALMASK_W<'a> = crate::BitWriter<'a, u32, IRQMASK_SPEC, bool, 17>;
#[doc = "Field `TXCRCCHUNKBRKMASK` reader - TX CRC Chunk Was Broken Mask"]
pub type TXCRCCHUNKBRKMASK_R = crate::BitReader<bool>;
#[doc = "Field `TXCRCCHUNKBRKMASK` writer - TX CRC Chunk Was Broken Mask"]
pub type TXCRCCHUNKBRKMASK_W<'a> = crate::BitWriter<'a, u32, IRQMASK_SPEC, bool, 18>;
impl R {
    #[doc = "Bit 0 - Mode M Failure Mask"]
    #[inline(always)]
    pub fn modemfailmask(&self) -> MODEMFAILMASK_R {
        MODEMFAILMASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Underflow Detected Mask"]
    #[inline(always)]
    pub fn underflowdetmask(&self) -> UNDERFLOWDETMASK_R {
        UNDERFLOWDETMASK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indirect Complete Mask"]
    #[inline(always)]
    pub fn indirectopdonemask(&self) -> INDIRECTOPDONEMASK_R {
        INDIRECTOPDONEMASK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indirect Read Reject Mask"]
    #[inline(always)]
    pub fn indirectreadrejectmask(&self) -> INDIRECTREADREJECTMASK_R {
        INDIRECTREADREJECTMASK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Protected Area Write Attempt Mask"]
    #[inline(always)]
    pub fn protwrattemptmask(&self) -> PROTWRATTEMPTMASK_R {
        PROTWRATTEMPTMASK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Illegal Access Detected Mask"]
    #[inline(always)]
    pub fn illegalaccessdetmask(&self) -> ILLEGALACCESSDETMASK_R {
        ILLEGALACCESSDETMASK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer Watermark Breach Mask"]
    #[inline(always)]
    pub fn indirectxferlevelbreachmask(&self) -> INDIRECTXFERLEVELBREACHMASK_R {
        INDIRECTXFERLEVELBREACHMASK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Overflow Mask"]
    #[inline(always)]
    pub fn recvoverflowmask(&self) -> RECVOVERFLOWMASK_R {
        RECVOVERFLOWMASK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Small TX FIFO Not Full Mask"]
    #[inline(always)]
    pub fn txfifonotfullmask(&self) -> TXFIFONOTFULLMASK_R {
        TXFIFONOTFULLMASK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Small TX FIFO Full Mask"]
    #[inline(always)]
    pub fn txfifofullmask(&self) -> TXFIFOFULLMASK_R {
        TXFIFOFULLMASK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Small RX FIFO Not Empty Mask"]
    #[inline(always)]
    pub fn rxfifonotemptymask(&self) -> RXFIFONOTEMPTYMASK_R {
        RXFIFONOTEMPTYMASK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Small RX FIFO Full Mask"]
    #[inline(always)]
    pub fn rxfifofullmask(&self) -> RXFIFOFULLMASK_R {
        RXFIFOFULLMASK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Indirect Read Partition Overflow Mask"]
    #[inline(always)]
    pub fn indrdsramfullmask(&self) -> INDRDSRAMFULLMASK_R {
        INDRDSRAMFULLMASK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Polling Expiration Detected Mask"]
    #[inline(always)]
    pub fn pollexpintmask(&self) -> POLLEXPINTMASK_R {
        POLLEXPINTMASK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - STIG Request Completion Mask"]
    #[inline(always)]
    pub fn stigreqmask(&self) -> STIGREQMASK_R {
        STIGREQMASK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - RX CRC Data Error Mask"]
    #[inline(always)]
    pub fn rxcrcdataerrmask(&self) -> RXCRCDATAERRMASK_R {
        RXCRCDATAERRMASK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RX CRC Data Valid Mask"]
    #[inline(always)]
    pub fn rxcrcdatavalmask(&self) -> RXCRCDATAVALMASK_R {
        RXCRCDATAVALMASK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TX CRC Chunk Was Broken Mask"]
    #[inline(always)]
    pub fn txcrcchunkbrkmask(&self) -> TXCRCCHUNKBRKMASK_R {
        TXCRCCHUNKBRKMASK_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mode M Failure Mask"]
    #[inline(always)]
    pub fn modemfailmask(&mut self) -> MODEMFAILMASK_W {
        MODEMFAILMASK_W::new(self)
    }
    #[doc = "Bit 1 - Underflow Detected Mask"]
    #[inline(always)]
    pub fn underflowdetmask(&mut self) -> UNDERFLOWDETMASK_W {
        UNDERFLOWDETMASK_W::new(self)
    }
    #[doc = "Bit 2 - Indirect Complete Mask"]
    #[inline(always)]
    pub fn indirectopdonemask(&mut self) -> INDIRECTOPDONEMASK_W {
        INDIRECTOPDONEMASK_W::new(self)
    }
    #[doc = "Bit 3 - Indirect Read Reject Mask"]
    #[inline(always)]
    pub fn indirectreadrejectmask(&mut self) -> INDIRECTREADREJECTMASK_W {
        INDIRECTREADREJECTMASK_W::new(self)
    }
    #[doc = "Bit 4 - Protected Area Write Attempt Mask"]
    #[inline(always)]
    pub fn protwrattemptmask(&mut self) -> PROTWRATTEMPTMASK_W {
        PROTWRATTEMPTMASK_W::new(self)
    }
    #[doc = "Bit 5 - Illegal Access Detected Mask"]
    #[inline(always)]
    pub fn illegalaccessdetmask(&mut self) -> ILLEGALACCESSDETMASK_W {
        ILLEGALACCESSDETMASK_W::new(self)
    }
    #[doc = "Bit 6 - Transfer Watermark Breach Mask"]
    #[inline(always)]
    pub fn indirectxferlevelbreachmask(&mut self) -> INDIRECTXFERLEVELBREACHMASK_W {
        INDIRECTXFERLEVELBREACHMASK_W::new(self)
    }
    #[doc = "Bit 7 - Receive Overflow Mask"]
    #[inline(always)]
    pub fn recvoverflowmask(&mut self) -> RECVOVERFLOWMASK_W {
        RECVOVERFLOWMASK_W::new(self)
    }
    #[doc = "Bit 8 - Small TX FIFO Not Full Mask"]
    #[inline(always)]
    pub fn txfifonotfullmask(&mut self) -> TXFIFONOTFULLMASK_W {
        TXFIFONOTFULLMASK_W::new(self)
    }
    #[doc = "Bit 9 - Small TX FIFO Full Mask"]
    #[inline(always)]
    pub fn txfifofullmask(&mut self) -> TXFIFOFULLMASK_W {
        TXFIFOFULLMASK_W::new(self)
    }
    #[doc = "Bit 10 - Small RX FIFO Not Empty Mask"]
    #[inline(always)]
    pub fn rxfifonotemptymask(&mut self) -> RXFIFONOTEMPTYMASK_W {
        RXFIFONOTEMPTYMASK_W::new(self)
    }
    #[doc = "Bit 11 - Small RX FIFO Full Mask"]
    #[inline(always)]
    pub fn rxfifofullmask(&mut self) -> RXFIFOFULLMASK_W {
        RXFIFOFULLMASK_W::new(self)
    }
    #[doc = "Bit 12 - Indirect Read Partition Overflow Mask"]
    #[inline(always)]
    pub fn indrdsramfullmask(&mut self) -> INDRDSRAMFULLMASK_W {
        INDRDSRAMFULLMASK_W::new(self)
    }
    #[doc = "Bit 13 - Polling Expiration Detected Mask"]
    #[inline(always)]
    pub fn pollexpintmask(&mut self) -> POLLEXPINTMASK_W {
        POLLEXPINTMASK_W::new(self)
    }
    #[doc = "Bit 14 - STIG Request Completion Mask"]
    #[inline(always)]
    pub fn stigreqmask(&mut self) -> STIGREQMASK_W {
        STIGREQMASK_W::new(self)
    }
    #[doc = "Bit 16 - RX CRC Data Error Mask"]
    #[inline(always)]
    pub fn rxcrcdataerrmask(&mut self) -> RXCRCDATAERRMASK_W {
        RXCRCDATAERRMASK_W::new(self)
    }
    #[doc = "Bit 17 - RX CRC Data Valid Mask"]
    #[inline(always)]
    pub fn rxcrcdatavalmask(&mut self) -> RXCRCDATAVALMASK_W {
        RXCRCDATAVALMASK_W::new(self)
    }
    #[doc = "Bit 18 - TX CRC Chunk Was Broken Mask"]
    #[inline(always)]
    pub fn txcrcchunkbrkmask(&mut self) -> TXCRCCHUNKBRKMASK_W {
        TXCRCCHUNKBRKMASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqmask](index.html) module"]
pub struct IRQMASK_SPEC;
impl crate::RegisterSpec for IRQMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irqmask::R](R) reader structure"]
impl crate::Readable for IRQMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irqmask::W](W) writer structure"]
impl crate::Writable for IRQMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRQMASK to value 0"]
impl crate::Resettable for IRQMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
