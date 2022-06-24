#[doc = "Register `DMEM0ECCADDR` reader"]
pub struct R(crate::R<DMEM0ECCADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMEM0ECCADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMEM0ECCADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMEM0ECCADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMEM0ECCADDR` reader - DMEM0 RAM ECC Error Address"]
pub type DMEM0ECCADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMEM0 RAM ECC Error Address"]
    #[inline(always)]
    pub fn dmem0eccaddr(&self) -> DMEM0ECCADDR_R {
        DMEM0ECCADDR_R::new(self.bits)
    }
}
#[doc = "Read to get status of the DMEM0 ECC error address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmem0eccaddr](index.html) module"]
pub struct DMEM0ECCADDR_SPEC;
impl crate::RegisterSpec for DMEM0ECCADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmem0eccaddr::R](R) reader structure"]
impl crate::Readable for DMEM0ECCADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMEM0ECCADDR to value 0"]
impl crate::Resettable for DMEM0ECCADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
