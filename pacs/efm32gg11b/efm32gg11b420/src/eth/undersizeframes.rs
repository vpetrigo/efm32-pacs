#[doc = "Register `UNDERSIZEFRAMES` reader"]
pub struct R(crate::R<UNDERSIZEFRAMES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UNDERSIZEFRAMES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UNDERSIZEFRAMES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UNDERSIZEFRAMES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UNDERSIZEFRAMES` writer"]
pub struct W(crate::W<UNDERSIZEFRAMES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UNDERSIZEFRAMES_SPEC>;
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
impl From<crate::W<UNDERSIZEFRAMES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UNDERSIZEFRAMES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Undersize frames received"]
pub type COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COUNT` writer - Undersize frames received"]
pub type COUNT_W<'a> = crate::FieldWriter<'a, u32, UNDERSIZEFRAMES_SPEC, u16, u16, 10, 0>;
impl R {
    #[doc = "Bits 0:9 - Undersize frames received"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Undersize frames received"]
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
#[doc = "Undersized Frames Received\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [undersizeframes](index.html) module"]
pub struct UNDERSIZEFRAMES_SPEC;
impl crate::RegisterSpec for UNDERSIZEFRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [undersizeframes::R](R) reader structure"]
impl crate::Readable for UNDERSIZEFRAMES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [undersizeframes::W](W) writer structure"]
impl crate::Writable for UNDERSIZEFRAMES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UNDERSIZEFRAMES to value 0"]
impl crate::Resettable for UNDERSIZEFRAMES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
