#[doc = "Register `TSUPTPRXNSEC` reader"]
pub struct R(crate::R<TSUPTPRXNSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSUPTPRXNSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSUPTPRXNSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSUPTPRXNSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER` reader - PTP Event Frame Received Nanoseconds"]
pub type TIMER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:29 - PTP Event Frame Received Nanoseconds"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
#[doc = "PTP Event Frame Received Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsuptprxnsec](index.html) module"]
pub struct TSUPTPRXNSEC_SPEC;
impl crate::RegisterSpec for TSUPTPRXNSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsuptprxnsec::R](R) reader structure"]
impl crate::Readable for TSUPTPRXNSEC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TSUPTPRXNSEC to value 0"]
impl crate::Resettable for TSUPTPRXNSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
