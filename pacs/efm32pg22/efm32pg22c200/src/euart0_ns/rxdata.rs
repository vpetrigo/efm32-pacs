#[doc = "Register `RXDATA` reader"]
pub struct R(crate::R<RXDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXDATA` reader - RX Data"]
pub type RXDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PERR` reader - Parity Error"]
pub type PERR_R = crate::BitReader<bool>;
#[doc = "Field `FERR` reader - Framing Error"]
pub type FERR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:8 - RX Data"]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - Parity Error"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Framing Error"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdata](index.html) module"]
pub struct RXDATA_SPEC;
impl crate::RegisterSpec for RXDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdata::R](R) reader structure"]
impl crate::Readable for RXDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXDATA to value 0"]
impl crate::Resettable for RXDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
