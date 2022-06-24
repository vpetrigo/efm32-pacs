#[doc = "Register `JUMBOMAXLEN` reader"]
pub struct R(crate::R<JUMBOMAXLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JUMBOMAXLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JUMBOMAXLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JUMBOMAXLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JUMBOMAXLEN` writer"]
pub struct W(crate::W<JUMBOMAXLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JUMBOMAXLEN_SPEC>;
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
impl From<crate::W<JUMBOMAXLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JUMBOMAXLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JUMBOMAXLEN` reader - Maximum Jumbo Frame Size - resets to the gem_jumbo_max_length define value."]
pub type JUMBOMAXLEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `JUMBOMAXLEN` writer - Maximum Jumbo Frame Size - resets to the gem_jumbo_max_length define value."]
pub type JUMBOMAXLEN_W<'a> = crate::FieldWriter<'a, u32, JUMBOMAXLEN_SPEC, u16, u16, 14, 0>;
impl R {
    #[doc = "Bits 0:13 - Maximum Jumbo Frame Size - resets to the gem_jumbo_max_length define value."]
    #[inline(always)]
    pub fn jumbomaxlen(&self) -> JUMBOMAXLEN_R {
        JUMBOMAXLEN_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Maximum Jumbo Frame Size - resets to the gem_jumbo_max_length define value."]
    #[inline(always)]
    pub fn jumbomaxlen(&mut self) -> JUMBOMAXLEN_W {
        JUMBOMAXLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Maximum Jumbo Frame Size.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jumbomaxlen](index.html) module"]
pub struct JUMBOMAXLEN_SPEC;
impl crate::RegisterSpec for JUMBOMAXLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jumbomaxlen::R](R) reader structure"]
impl crate::Readable for JUMBOMAXLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jumbomaxlen::W](W) writer structure"]
impl crate::Writable for JUMBOMAXLEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets JUMBOMAXLEN to value 0x2800"]
impl crate::Resettable for JUMBOMAXLEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2800
    }
}
