#[doc = "Register `RXBDCTRL` reader"]
pub struct R(crate::R<RXBDCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXBDCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXBDCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXBDCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXBDCTRL` writer"]
pub struct W(crate::W<RXBDCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXBDCTRL_SPEC>;
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
impl From<crate::W<RXBDCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXBDCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXBDTSMODE` reader - RX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
pub type RXBDTSMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXBDTSMODE` writer - RX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
pub type RXBDTSMODE_W<'a> = crate::FieldWriter<'a, u32, RXBDCTRL_SPEC, u8, u8, 2, 4>;
impl R {
    #[doc = "Bits 4:5 - RX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
    #[inline(always)]
    pub fn rxbdtsmode(&self) -> RXBDTSMODE_R {
        RXBDTSMODE_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - RX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
    #[inline(always)]
    pub fn rxbdtsmode(&mut self) -> RXBDTSMODE_W {
        RXBDTSMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX BD control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbdctrl](index.html) module"]
pub struct RXBDCTRL_SPEC;
impl crate::RegisterSpec for RXBDCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxbdctrl::R](R) reader structure"]
impl crate::Readable for RXBDCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxbdctrl::W](W) writer structure"]
impl crate::Writable for RXBDCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXBDCTRL to value 0"]
impl crate::Resettable for RXBDCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
