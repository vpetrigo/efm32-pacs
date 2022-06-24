#[doc = "Register `RAM2ECCADDR` reader"]
pub struct R(crate::R<RAM2ECCADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM2ECCADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM2ECCADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM2ECCADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RAM2ECCADDR` reader - RAM2 ECC Error Address"]
pub type RAM2ECCADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - RAM2 ECC Error Address"]
    #[inline(always)]
    pub fn ram2eccaddr(&self) -> RAM2ECCADDR_R {
        RAM2ECCADDR_R::new(self.bits)
    }
}
#[doc = "RAM2 ECC Error Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram2eccaddr](index.html) module"]
pub struct RAM2ECCADDR_SPEC;
impl crate::RegisterSpec for RAM2ECCADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram2eccaddr::R](R) reader structure"]
impl crate::Readable for RAM2ECCADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RAM2ECCADDR to value 0"]
impl crate::Resettable for RAM2ECCADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
