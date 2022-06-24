#[doc = "Register `TSUPTPTXMSBSEC` reader"]
pub struct R(crate::R<TSUPTPTXMSBSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSUPTPTXMSBSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSUPTPTXMSBSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSUPTPTXMSBSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMERSEC` reader - PTP Event Frame TX Seconds"]
pub type TIMERSEC_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - PTP Event Frame TX Seconds"]
    #[inline(always)]
    pub fn timersec(&self) -> TIMERSEC_R {
        TIMERSEC_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "PTP Event Frame Transmitted Seconds Register 47:32\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsuptptxmsbsec](index.html) module"]
pub struct TSUPTPTXMSBSEC_SPEC;
impl crate::RegisterSpec for TSUPTPTXMSBSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsuptptxmsbsec::R](R) reader structure"]
impl crate::Readable for TSUPTPTXMSBSEC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TSUPTPTXMSBSEC to value 0"]
impl crate::Resettable for TSUPTPTXMSBSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
