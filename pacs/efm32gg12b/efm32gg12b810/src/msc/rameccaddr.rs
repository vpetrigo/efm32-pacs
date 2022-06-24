#[doc = "Register `RAMECCADDR` reader"]
pub struct R(crate::R<RAMECCADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAMECCADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAMECCADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAMECCADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RAMECCADDR` reader - RAM ECC Error Address"]
pub type RAMECCADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - RAM ECC Error Address"]
    #[inline(always)]
    pub fn rameccaddr(&self) -> RAMECCADDR_R {
        RAMECCADDR_R::new(self.bits)
    }
}
#[doc = "RAM ECC Error Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rameccaddr](index.html) module"]
pub struct RAMECCADDR_SPEC;
impl crate::RegisterSpec for RAMECCADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rameccaddr::R](R) reader structure"]
impl crate::Readable for RAMECCADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RAMECCADDR to value 0"]
impl crate::Resettable for RAMECCADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
