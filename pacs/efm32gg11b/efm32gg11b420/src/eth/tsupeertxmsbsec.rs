#[doc = "Register `TSUPEERTXMSBSEC` reader"]
pub struct R(crate::R<TSUPEERTXMSBSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSUPEERTXMSBSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSUPEERTXMSBSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSUPEERTXMSBSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMERSEC` reader - PTP Peer Event Frame TX Seconds"]
pub type TIMERSEC_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - PTP Peer Event Frame TX Seconds"]
    #[inline(always)]
    pub fn timersec(&self) -> TIMERSEC_R {
        TIMERSEC_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "PTP Peer Event Frame Transmitted Seconds Register 47:32\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsupeertxmsbsec](index.html) module"]
pub struct TSUPEERTXMSBSEC_SPEC;
impl crate::RegisterSpec for TSUPEERTXMSBSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsupeertxmsbsec::R](R) reader structure"]
impl crate::Readable for TSUPEERTXMSBSEC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TSUPEERTXMSBSEC to value 0"]
impl crate::Resettable for TSUPEERTXMSBSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
