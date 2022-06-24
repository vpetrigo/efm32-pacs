#[doc = "Register `TIMER` reader"]
pub struct R(crate::R<TIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER` reader - Timer"]
pub type TIMER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timer"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(self.bits)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer](index.html) module"]
pub struct TIMER_SPEC;
impl crate::RegisterSpec for TIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer::R](R) reader structure"]
impl crate::Readable for TIMER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIMER to value 0"]
impl crate::Resettable for TIMER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
