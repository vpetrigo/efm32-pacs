#[doc = "Register `RXPAUSEQUANT` reader"]
pub struct R(crate::R<RXPAUSEQUANT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXPAUSEQUANT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXPAUSEQUANT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXPAUSEQUANT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `QUANT` reader - Received pause quantum"]
pub type QUANT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Received pause quantum"]
    #[inline(always)]
    pub fn quant(&self) -> QUANT_R {
        QUANT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Received Pause Quantum Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxpausequant](index.html) module"]
pub struct RXPAUSEQUANT_SPEC;
impl crate::RegisterSpec for RXPAUSEQUANT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxpausequant::R](R) reader structure"]
impl crate::Readable for RXPAUSEQUANT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXPAUSEQUANT to value 0"]
impl crate::Resettable for RXPAUSEQUANT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
