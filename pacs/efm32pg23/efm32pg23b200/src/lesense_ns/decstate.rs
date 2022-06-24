#[doc = "Register `DECSTATE` reader"]
pub struct R(crate::R<DECSTATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DECSTATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DECSTATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DECSTATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DECSTATE` reader - Shows the current decoder state"]
pub type DECSTATE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - Shows the current decoder state"]
    #[inline(always)]
    pub fn decstate(&self) -> DECSTATE_R {
        DECSTATE_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Current decoder state\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [decstate](index.html) module"]
pub struct DECSTATE_SPEC;
impl crate::RegisterSpec for DECSTATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [decstate::R](R) reader structure"]
impl crate::Readable for DECSTATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DECSTATE to value 0"]
impl crate::Resettable for DECSTATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
