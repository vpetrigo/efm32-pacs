#[doc = "Register `ADMAES` reader"]
pub struct R(crate::R<ADMAES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADMAES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADMAES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADMAES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADMAES` reader - ADMA Error State"]
pub type ADMAES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADMALME` reader - ADMA Length Mismatch Error"]
pub type ADMALME_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1 - ADMA Error State"]
    #[inline(always)]
    pub fn admaes(&self) -> ADMAES_R {
        ADMAES_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - ADMA Length Mismatch Error"]
    #[inline(always)]
    pub fn admalme(&self) -> ADMALME_R {
        ADMALME_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "ADMA Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [admaes](index.html) module"]
pub struct ADMAES_SPEC;
impl crate::RegisterSpec for ADMAES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [admaes::R](R) reader structure"]
impl crate::Readable for ADMAES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADMAES to value 0"]
impl crate::Resettable for ADMAES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
