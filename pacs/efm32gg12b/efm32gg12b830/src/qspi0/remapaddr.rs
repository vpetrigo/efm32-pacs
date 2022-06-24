#[doc = "Register `REMAPADDR` reader"]
pub struct R(crate::R<REMAPADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REMAPADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REMAPADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REMAPADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REMAPADDR` writer"]
pub struct W(crate::W<REMAPADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REMAPADDR_SPEC>;
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
impl From<crate::W<REMAPADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REMAPADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - Remap Address Value"]
pub type VALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `VALUE` writer - Remap Address Value"]
pub type VALUE_W<'a> = crate::FieldWriter<'a, u32, REMAPADDR_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Remap Address Value"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Remap Address Value"]
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
#[doc = "Remap Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [remapaddr](index.html) module"]
pub struct REMAPADDR_SPEC;
impl crate::RegisterSpec for REMAPADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [remapaddr::R](R) reader structure"]
impl crate::Readable for REMAPADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [remapaddr::W](W) writer structure"]
impl crate::Writable for REMAPADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REMAPADDR to value 0"]
impl crate::Resettable for REMAPADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
