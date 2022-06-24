#[doc = "Register `CMDARG1` reader"]
pub struct R(crate::R<CMDARG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDARG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDARG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDARG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDARG1` writer"]
pub struct W(crate::W<CMDARG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDARG1_SPEC>;
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
impl From<crate::W<CMDARG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDARG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDARG1` reader - Command Argument 1"]
pub type CMDARG1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CMDARG1` writer - Command Argument 1"]
pub type CMDARG1_W<'a> = crate::FieldWriter<'a, u32, CMDARG1_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Command Argument 1"]
    #[inline(always)]
    pub fn cmdarg1(&self) -> CMDARG1_R {
        CMDARG1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Argument 1"]
    #[inline(always)]
    pub fn cmdarg1(&mut self) -> CMDARG1_W {
        CMDARG1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD Command Argument Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdarg1](index.html) module"]
pub struct CMDARG1_SPEC;
impl crate::RegisterSpec for CMDARG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdarg1::R](R) reader structure"]
impl crate::Readable for CMDARG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdarg1::W](W) writer structure"]
impl crate::Writable for CMDARG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMDARG1 to value 0"]
impl crate::Resettable for CMDARG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
