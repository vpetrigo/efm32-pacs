#[doc = "Register `PFRAMESRXED` reader"]
pub struct R(crate::R<PFRAMESRXED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFRAMESRXED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFRAMESRXED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFRAMESRXED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFRAMESRXED` writer"]
pub struct W(crate::W<PFRAMESRXED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFRAMESRXED_SPEC>;
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
impl From<crate::W<PFRAMESRXED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFRAMESRXED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Received pause frames"]
pub type COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COUNT` writer - Received pause frames"]
pub type COUNT_W<'a> = crate::FieldWriter<'a, u32, PFRAMESRXED_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Received pause frames"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Received pause frames"]
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
#[doc = "Pause Frames Received\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pframesrxed](index.html) module"]
pub struct PFRAMESRXED_SPEC;
impl crate::RegisterSpec for PFRAMESRXED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pframesrxed::R](R) reader structure"]
impl crate::Readable for PFRAMESRXED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pframesrxed::W](W) writer structure"]
impl crate::Writable for PFRAMESRXED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PFRAMESRXED to value 0"]
impl crate::Resettable for PFRAMESRXED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
