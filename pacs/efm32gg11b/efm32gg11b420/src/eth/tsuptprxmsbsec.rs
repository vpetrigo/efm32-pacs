#[doc = "Register `TSUPTPRXMSBSEC` reader"]
pub struct R(crate::R<TSUPTPRXMSBSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSUPTPRXMSBSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSUPTPRXMSBSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSUPTPRXMSBSEC_SPEC>) -> Self {
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
#[doc = "PTP Event Frame Received Seconds Register 47:32\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsuptprxmsbsec](index.html) module"]
pub struct TSUPTPRXMSBSEC_SPEC;
impl crate::RegisterSpec for TSUPTPRXMSBSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsuptprxmsbsec::R](R) reader structure"]
impl crate::Readable for TSUPTPRXMSBSEC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TSUPTPRXMSBSEC to value 0"]
impl crate::Resettable for TSUPTPRXMSBSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
