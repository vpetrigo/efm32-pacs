#[doc = "Register `TXBDCTRL` reader"]
pub struct R(crate::R<TXBDCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBDCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBDCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBDCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXBDCTRL` writer"]
pub struct W(crate::W<TXBDCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBDCTRL_SPEC>;
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
impl From<crate::W<TXBDCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBDCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXBDTSMODE` reader - TX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
pub type TXBDTSMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXBDTSMODE` writer - TX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
pub type TXBDTSMODE_W<'a> = crate::FieldWriter<'a, u32, TXBDCTRL_SPEC, u8, u8, 2, 4>;
impl R {
    #[doc = "Bits 4:5 - TX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
    #[inline(always)]
    pub fn txbdtsmode(&self) -> TXBDTSMODE_R {
        TXBDTSMODE_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - TX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
    #[inline(always)]
    pub fn txbdtsmode(&mut self) -> TXBDTSMODE_W {
        TXBDTSMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX BD control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbdctrl](index.html) module"]
pub struct TXBDCTRL_SPEC;
impl crate::RegisterSpec for TXBDCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbdctrl::R](R) reader structure"]
impl crate::Readable for TXBDCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txbdctrl::W](W) writer structure"]
impl crate::Writable for TXBDCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXBDCTRL to value 0"]
impl crate::Resettable for TXBDCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
