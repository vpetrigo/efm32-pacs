#[doc = "Register `DIEP4_DTXFSTS` reader"]
pub struct R(crate::R<DIEP4_DTXFSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEP4_DTXFSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEP4_DTXFSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEP4_DTXFSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPCAVAIL` reader - IN Endpoint TxFIFO Space Avail"]
pub type SPCAVAIL_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN Endpoint TxFIFO Space Avail"]
    #[inline(always)]
    pub fn spcavail(&self) -> SPCAVAIL_R {
        SPCAVAIL_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep4_dtxfsts](index.html) module"]
pub struct DIEP4_DTXFSTS_SPEC;
impl crate::RegisterSpec for DIEP4_DTXFSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diep4_dtxfsts::R](R) reader structure"]
impl crate::Readable for DIEP4_DTXFSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIEP4_DTXFSTS to value 0x0200"]
impl crate::Resettable for DIEP4_DTXFSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
