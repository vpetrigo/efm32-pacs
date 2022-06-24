#[doc = "Register `RAM1ECCADDR` reader"]
pub struct R(crate::R<RAM1ECCADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM1ECCADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM1ECCADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM1ECCADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RAM1ECCADDR` reader - RAM1 ECC Error Address"]
pub type RAM1ECCADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - RAM1 ECC Error Address"]
    #[inline(always)]
    pub fn ram1eccaddr(&self) -> RAM1ECCADDR_R {
        RAM1ECCADDR_R::new(self.bits)
    }
}
#[doc = "RAM1 ECC Error Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram1eccaddr](index.html) module"]
pub struct RAM1ECCADDR_SPEC;
impl crate::RegisterSpec for RAM1ECCADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram1eccaddr::R](R) reader structure"]
impl crate::Readable for RAM1ECCADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RAM1ECCADDR to value 0"]
impl crate::Resettable for RAM1ECCADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
