#[doc = "Register `TSUPTPTXSEC` reader"]
pub struct R(crate::R<TSUPTPTXSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSUPTPTXSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSUPTPTXSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSUPTPTXSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER` reader - PTP Event Frame Transmitted Seconds"]
pub type TIMER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - PTP Event Frame Transmitted Seconds"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(self.bits)
    }
}
#[doc = "PTP Event Frame Transmitted Seconds Register 31:0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsuptptxsec](index.html) module"]
pub struct TSUPTPTXSEC_SPEC;
impl crate::RegisterSpec for TSUPTPTXSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsuptptxsec::R](R) reader structure"]
impl crate::Readable for TSUPTPTXSEC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TSUPTPTXSEC to value 0"]
impl crate::Resettable for TSUPTPTXSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
