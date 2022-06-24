#[doc = "Register `RESP4` reader"]
pub struct R(crate::R<RESP4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESP4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESP4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESP4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMDRESP2` reader - Command Response 2"]
pub type CMDRESP2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response 2"]
    #[inline(always)]
    pub fn cmdresp2(&self) -> CMDRESP2_R {
        CMDRESP2_R::new(self.bits)
    }
}
#[doc = "Response4 and Response5 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp4](index.html) module"]
pub struct RESP4_SPEC;
impl crate::RegisterSpec for RESP4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resp4::R](R) reader structure"]
impl crate::Readable for RESP4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESP4 to value 0"]
impl crate::Resettable for RESP4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
