#[doc = "Register `INDIRECTREADXFERNUMBYTES` reader"]
pub struct R(crate::R<INDIRECTREADXFERNUMBYTES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INDIRECTREADXFERNUMBYTES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INDIRECTREADXFERNUMBYTES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INDIRECTREADXFERNUMBYTES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INDIRECTREADXFERNUMBYTES` writer"]
pub struct W(crate::W<INDIRECTREADXFERNUMBYTES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INDIRECTREADXFERNUMBYTES_SPEC>;
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
impl From<crate::W<INDIRECTREADXFERNUMBYTES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INDIRECTREADXFERNUMBYTES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - Indirect Read Transfer Number Bytes"]
pub type VALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `VALUE` writer - Indirect Read Transfer Number Bytes"]
pub type VALUE_W<'a> = crate::FieldWriter<'a, u32, INDIRECTREADXFERNUMBYTES_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Indirect Read Transfer Number Bytes"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Indirect Read Transfer Number Bytes"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indirect Read Transfer Number Bytes Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [indirectreadxfernumbytes](index.html) module"]
pub struct INDIRECTREADXFERNUMBYTES_SPEC;
impl crate::RegisterSpec for INDIRECTREADXFERNUMBYTES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [indirectreadxfernumbytes::R](R) reader structure"]
impl crate::Readable for INDIRECTREADXFERNUMBYTES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [indirectreadxfernumbytes::W](W) writer structure"]
impl crate::Writable for INDIRECTREADXFERNUMBYTES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INDIRECTREADXFERNUMBYTES to value 0"]
impl crate::Resettable for INDIRECTREADXFERNUMBYTES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
