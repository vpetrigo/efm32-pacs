#[doc = "Register `RESCOUNT` reader"]
pub struct R(crate::R<RESCOUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESCOUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESCOUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESCOUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT` reader - Result Fifo Count"]
pub type COUNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - Result Fifo Count"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Result FIFO Count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rescount](index.html) module"]
pub struct RESCOUNT_SPEC;
impl crate::RegisterSpec for RESCOUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rescount::R](R) reader structure"]
impl crate::Readable for RESCOUNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESCOUNT to value 0"]
impl crate::Resettable for RESCOUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
