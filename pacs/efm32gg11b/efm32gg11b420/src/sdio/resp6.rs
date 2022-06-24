#[doc = "Register `RESP6` reader"]
pub struct R(crate::R<RESP6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESP6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESP6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESP6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMDRESP3` reader - Command Response 3"]
pub type CMDRESP3_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response 3"]
    #[inline(always)]
    pub fn cmdresp3(&self) -> CMDRESP3_R {
        CMDRESP3_R::new(self.bits)
    }
}
#[doc = "Response6 and Response7 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp6](index.html) module"]
pub struct RESP6_SPEC;
impl crate::RegisterSpec for RESP6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resp6::R](R) reader structure"]
impl crate::Readable for RESP6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESP6 to value 0"]
impl crate::Resettable for RESP6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
