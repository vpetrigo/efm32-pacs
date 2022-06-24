#[doc = "Register `FLASHRDDATAUPPER` reader"]
pub struct R(crate::R<FLASHRDDATAUPPER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASHRDDATAUPPER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASHRDDATAUPPER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASHRDDATAUPPER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Read Data Upper"]
pub type DATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read Data Upper"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "Flash Command Read Data Register (Upper) (STIG)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashrddataupper](index.html) module"]
pub struct FLASHRDDATAUPPER_SPEC;
impl crate::RegisterSpec for FLASHRDDATAUPPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flashrddataupper::R](R) reader structure"]
impl crate::Readable for FLASHRDDATAUPPER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FLASHRDDATAUPPER to value 0"]
impl crate::Resettable for FLASHRDDATAUPPER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
