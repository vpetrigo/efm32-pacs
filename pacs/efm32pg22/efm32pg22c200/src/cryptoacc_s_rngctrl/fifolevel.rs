#[doc = "Register `FIFOLEVEL` reader"]
pub struct R(crate::R<FIFOLEVEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOLEVEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOLEVEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOLEVEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFOLEVEL` reader - FIFO Level"]
pub type FIFOLEVEL_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - FIFO Level"]
    #[inline(always)]
    pub fn fifolevel(&self) -> FIFOLEVEL_R {
        FIFOLEVEL_R::new(self.bits)
    }
}
#[doc = "Number of 32 bits words of random available in the FIFO. Writing to this register clears the FIFO full interrupt\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifolevel](index.html) module"]
pub struct FIFOLEVEL_SPEC;
impl crate::RegisterSpec for FIFOLEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifolevel::R](R) reader structure"]
impl crate::Readable for FIFOLEVEL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FIFOLEVEL to value 0"]
impl crate::Resettable for FIFOLEVEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
