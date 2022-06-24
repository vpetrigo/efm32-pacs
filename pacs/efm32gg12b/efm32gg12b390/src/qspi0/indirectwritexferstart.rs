#[doc = "Register `INDIRECTWRITEXFERSTART` reader"]
pub struct R(crate::R<INDIRECTWRITEXFERSTART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INDIRECTWRITEXFERSTART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INDIRECTWRITEXFERSTART_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INDIRECTWRITEXFERSTART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INDIRECTWRITEXFERSTART` writer"]
pub struct W(crate::W<INDIRECTWRITEXFERSTART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INDIRECTWRITEXFERSTART_SPEC>;
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
impl From<crate::W<INDIRECTWRITEXFERSTART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INDIRECTWRITEXFERSTART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Start of Indirect Access"]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - Start of Indirect Access"]
pub type ADDR_W<'a> = crate::FieldWriter<'a, u32, INDIRECTWRITEXFERSTART_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Start of Indirect Access"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of Indirect Access"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indirect Write Transfer Start Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [indirectwritexferstart](index.html) module"]
pub struct INDIRECTWRITEXFERSTART_SPEC;
impl crate::RegisterSpec for INDIRECTWRITEXFERSTART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [indirectwritexferstart::R](R) reader structure"]
impl crate::Readable for INDIRECTWRITEXFERSTART_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [indirectwritexferstart::W](W) writer structure"]
impl crate::Writable for INDIRECTWRITEXFERSTART_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INDIRECTWRITEXFERSTART to value 0"]
impl crate::Resettable for INDIRECTWRITEXFERSTART_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
