#[doc = "Register `BUFDATPORT` reader"]
pub struct R(crate::R<BUFDATPORT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUFDATPORT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUFDATPORT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUFDATPORT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUFDATPORT` writer"]
pub struct W(crate::W<BUFDATPORT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUFDATPORT_SPEC>;
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
impl From<crate::W<BUFDATPORT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUFDATPORT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUFDAT` reader - Buffer Data"]
pub type BUFDAT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BUFDAT` writer - Buffer Data"]
pub type BUFDAT_W<'a> = crate::FieldWriter<'a, u32, BUFDATPORT_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Buffer Data"]
    #[inline(always)]
    pub fn bufdat(&self) -> BUFDAT_R {
        BUFDAT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Data"]
    #[inline(always)]
    pub fn bufdat(&mut self) -> BUFDAT_W {
        BUFDAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buffer Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bufdatport](index.html) module"]
pub struct BUFDATPORT_SPEC;
impl crate::RegisterSpec for BUFDATPORT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bufdatport::R](R) reader structure"]
impl crate::Readable for BUFDATPORT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bufdatport::W](W) writer structure"]
impl crate::Writable for BUFDATPORT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUFDATPORT to value 0"]
impl crate::Resettable for BUFDATPORT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
