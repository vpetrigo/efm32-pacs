#[doc = "Register `TSUPTPTXNSEC` reader"]
pub struct R(crate::R<TSUPTPTXNSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSUPTPTXNSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSUPTPTXNSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSUPTPTXNSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER` reader - PTP Event Frame Transmitted Nanoseconds"]
pub type TIMER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:29 - PTP Event Frame Transmitted Nanoseconds"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
#[doc = "PTP Event Frame Transmitted Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsuptptxnsec](index.html) module"]
pub struct TSUPTPTXNSEC_SPEC;
impl crate::RegisterSpec for TSUPTPTXNSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsuptptxnsec::R](R) reader structure"]
impl crate::Readable for TSUPTPTXNSEC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TSUPTPTXNSEC to value 0"]
impl crate::Resettable for TSUPTPTXNSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
