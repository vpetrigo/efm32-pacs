#[doc = "Register `FLASHRDDATALOWER` reader"]
pub struct R(crate::R<FLASHRDDATALOWER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASHRDDATALOWER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASHRDDATALOWER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASHRDDATALOWER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Read Data Lower"]
pub type DATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read Data Lower"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "Flash Command Read Data Register (Lower) (STIG)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashrddatalower](index.html) module"]
pub struct FLASHRDDATALOWER_SPEC;
impl crate::RegisterSpec for FLASHRDDATALOWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flashrddatalower::R](R) reader structure"]
impl crate::Readable for FLASHRDDATALOWER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FLASHRDDATALOWER to value 0"]
impl crate::Resettable for FLASHRDDATALOWER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
