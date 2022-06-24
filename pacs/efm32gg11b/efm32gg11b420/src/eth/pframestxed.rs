#[doc = "Register `PFRAMESTXED` reader"]
pub struct R(crate::R<PFRAMESTXED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFRAMESTXED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFRAMESTXED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFRAMESTXED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFRAMESTXED` writer"]
pub struct W(crate::W<PFRAMESTXED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFRAMESTXED_SPEC>;
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
impl From<crate::W<PFRAMESTXED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFRAMESTXED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Transmitted pause frames"]
pub type COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COUNT` writer - Transmitted pause frames"]
pub type COUNT_W<'a> = crate::FieldWriter<'a, u32, PFRAMESTXED_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Transmitted pause frames"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmitted pause frames"]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W {
        COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pause Frames Transmitted\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pframestxed](index.html) module"]
pub struct PFRAMESTXED_SPEC;
impl crate::RegisterSpec for PFRAMESTXED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pframestxed::R](R) reader structure"]
impl crate::Readable for PFRAMESTXED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pframestxed::W](W) writer structure"]
impl crate::Writable for PFRAMESTXED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PFRAMESTXED to value 0"]
impl crate::Resettable for PFRAMESTXED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
