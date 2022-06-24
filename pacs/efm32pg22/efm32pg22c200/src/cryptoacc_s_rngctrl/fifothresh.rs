#[doc = "Register `FIFOTHRESH` reader"]
pub struct R(crate::R<FIFOTHRESH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOTHRESH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOTHRESH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOTHRESH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFOTHRESH` reader - FIFO threshold level"]
pub type FIFOTHRESH_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - FIFO threshold level"]
    #[inline(always)]
    pub fn fifothresh(&self) -> FIFOTHRESH_R {
        FIFOTHRESH_R::new(self.bits)
    }
}
#[doc = "FIFO level at which the rings are restarted when in the FIFOFull_Off state, expressed in number of 128bit blocks\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifothresh](index.html) module"]
pub struct FIFOTHRESH_SPEC;
impl crate::RegisterSpec for FIFOTHRESH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifothresh::R](R) reader structure"]
impl crate::Readable for FIFOTHRESH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FIFOTHRESH to value 0x3f"]
impl crate::Resettable for FIFOTHRESH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f
    }
}
