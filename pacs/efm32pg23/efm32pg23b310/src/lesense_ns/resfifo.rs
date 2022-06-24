#[doc = "Register `RESFIFO` reader"]
pub struct R(crate::R<RESFIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESFIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESFIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUFDATASRC` reader - Result data and source"]
pub type BUFDATASRC_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:19 - Result data and source"]
    #[inline(always)]
    pub fn bufdatasrc(&self) -> BUFDATASRC_R {
        BUFDATASRC_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
#[doc = "Result Fifo\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resfifo](index.html) module"]
pub struct RESFIFO_SPEC;
impl crate::RegisterSpec for RESFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resfifo::R](R) reader structure"]
impl crate::Readable for RESFIFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESFIFO to value 0"]
impl crate::Resettable for RESFIFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
