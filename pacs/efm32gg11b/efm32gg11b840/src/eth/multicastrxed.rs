#[doc = "Register `MULTICASTRXED` reader"]
pub struct R(crate::R<MULTICASTRXED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MULTICASTRXED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MULTICASTRXED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MULTICASTRXED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MULTICASTRXED` writer"]
pub struct W(crate::W<MULTICASTRXED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MULTICASTRXED_SPEC>;
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
impl From<crate::W<MULTICASTRXED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MULTICASTRXED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Multicast frames received without error"]
pub type COUNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COUNT` writer - Multicast frames received without error"]
pub type COUNT_W<'a> = crate::FieldWriter<'a, u32, MULTICASTRXED_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Multicast frames received without error"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Multicast frames received without error"]
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
#[doc = "Multicast Frames Received\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [multicastrxed](index.html) module"]
pub struct MULTICASTRXED_SPEC;
impl crate::RegisterSpec for MULTICASTRXED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [multicastrxed::R](R) reader structure"]
impl crate::Readable for MULTICASTRXED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [multicastrxed::W](W) writer structure"]
impl crate::Writable for MULTICASTRXED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MULTICASTRXED to value 0"]
impl crate::Resettable for MULTICASTRXED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
