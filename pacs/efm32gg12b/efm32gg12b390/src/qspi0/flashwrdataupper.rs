#[doc = "Register `FLASHWRDATAUPPER` reader"]
pub struct R(crate::R<FLASHWRDATAUPPER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASHWRDATAUPPER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASHWRDATAUPPER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASHWRDATAUPPER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASHWRDATAUPPER` writer"]
pub struct W(crate::W<FLASHWRDATAUPPER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASHWRDATAUPPER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FLASHWRDATAUPPER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASHWRDATAUPPER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Command Write Data Upper Byte"]
pub type DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA` writer - Command Write Data Upper Byte"]
pub type DATA_W<'a> = crate::FieldWriter<'a, u32, FLASHWRDATAUPPER_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Command Write Data Upper Byte"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Write Data Upper Byte"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Command Write Data Register (Upper) (STIG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashwrdataupper](index.html) module"]
pub struct FLASHWRDATAUPPER_SPEC;
impl crate::RegisterSpec for FLASHWRDATAUPPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flashwrdataupper::R](R) reader structure"]
impl crate::Readable for FLASHWRDATAUPPER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flashwrdataupper::W](W) writer structure"]
impl crate::Writable for FLASHWRDATAUPPER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASHWRDATAUPPER to value 0"]
impl crate::Resettable for FLASHWRDATAUPPER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
