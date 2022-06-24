#[doc = "Register `ITTRIGOUT` reader"]
pub struct R(crate::R<ITTRIGOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITTRIGOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITTRIGOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITTRIGOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ITTRIGOUT` writer"]
pub struct W(crate::W<ITTRIGOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ITTRIGOUT_SPEC>;
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
impl From<crate::W<ITTRIGOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ITTRIGOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIGGEROUT` reader - Trigger output value"]
pub type TRIGGEROUT_R = crate::BitReader<bool>;
#[doc = "Field `TRIGGEROUT` writer - Trigger output value"]
pub type TRIGGEROUT_W<'a> = crate::BitWriter<'a, u32, ITTRIGOUT_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Trigger output value"]
    #[inline(always)]
    pub fn triggerout(&self) -> TRIGGEROUT_R {
        TRIGGEROUT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger output value"]
    #[inline(always)]
    pub fn triggerout(&mut self) -> TRIGGEROUT_W {
        TRIGGEROUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Integration Test Trigger Out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ittrigout](index.html) module"]
pub struct ITTRIGOUT_SPEC;
impl crate::RegisterSpec for ITTRIGOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ittrigout::R](R) reader structure"]
impl crate::Readable for ITTRIGOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ittrigout::W](W) writer structure"]
impl crate::Writable for ITTRIGOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ITTRIGOUT to value 0"]
impl crate::Resettable for ITTRIGOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
