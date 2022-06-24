#[doc = "Register `BROADCASTRXED` reader"]
pub struct R(crate::R<BROADCASTRXED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BROADCASTRXED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BROADCASTRXED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BROADCASTRXED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BROADCASTRXED` writer"]
pub struct W(crate::W<BROADCASTRXED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BROADCASTRXED_SPEC>;
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
impl From<crate::W<BROADCASTRXED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BROADCASTRXED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Broadcast frames received without error"]
pub type COUNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COUNT` writer - Broadcast frames received without error"]
pub type COUNT_W<'a> = crate::FieldWriter<'a, u32, BROADCASTRXED_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Broadcast frames received without error"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Broadcast frames received without error"]
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
#[doc = "Broadcast Frames Received\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [broadcastrxed](index.html) module"]
pub struct BROADCASTRXED_SPEC;
impl crate::RegisterSpec for BROADCASTRXED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [broadcastrxed::R](R) reader structure"]
impl crate::Readable for BROADCASTRXED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [broadcastrxed::W](W) writer structure"]
impl crate::Writable for BROADCASTRXED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BROADCASTRXED to value 0"]
impl crate::Resettable for BROADCASTRXED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
