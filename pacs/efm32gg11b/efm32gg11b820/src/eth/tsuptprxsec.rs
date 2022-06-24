#[doc = "Register `TSUPTPRXSEC` reader"]
pub struct R(crate::R<TSUPTPRXSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSUPTPRXSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSUPTPRXSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSUPTPRXSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER` reader - PTP Event Frame Received Seconds"]
pub type TIMER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - PTP Event Frame Received Seconds"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(self.bits)
    }
}
#[doc = "PTP Event Frame Received Seconds Register 31:0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsuptprxsec](index.html) module"]
pub struct TSUPTPRXSEC_SPEC;
impl crate::RegisterSpec for TSUPTPRXSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsuptprxsec::R](R) reader structure"]
impl crate::Readable for TSUPTPRXSEC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TSUPTPRXSEC to value 0"]
impl crate::Resettable for TSUPTPRXSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
