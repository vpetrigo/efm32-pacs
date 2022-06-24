#[doc = "Register `IMOD` reader"]
pub struct R(crate::R<IMOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMOD` writer"]
pub struct W(crate::W<IMOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMOD_SPEC>;
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
impl From<crate::W<IMOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXINTMOD` reader - Count of 800ns periods before bit 1 is set in the interrupt status register after a frame is received"]
pub type RXINTMOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXINTMOD` writer - Count of 800ns periods before bit 1 is set in the interrupt status register after a frame is received"]
pub type RXINTMOD_W<'a> = crate::FieldWriter<'a, u32, IMOD_SPEC, u8, u8, 8, 0>;
#[doc = "Field `TXINTMOD` reader - Count of 800ns periods before bit 7 is set in the interrupt status register after a frame is transmitted"]
pub type TXINTMOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXINTMOD` writer - Count of 800ns periods before bit 7 is set in the interrupt status register after a frame is transmitted"]
pub type TXINTMOD_W<'a> = crate::FieldWriter<'a, u32, IMOD_SPEC, u8, u8, 8, 16>;
impl R {
    #[doc = "Bits 0:7 - Count of 800ns periods before bit 1 is set in the interrupt status register after a frame is received"]
    #[inline(always)]
    pub fn rxintmod(&self) -> RXINTMOD_R {
        RXINTMOD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Count of 800ns periods before bit 7 is set in the interrupt status register after a frame is transmitted"]
    #[inline(always)]
    pub fn txintmod(&self) -> TXINTMOD_R {
        TXINTMOD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Count of 800ns periods before bit 1 is set in the interrupt status register after a frame is received"]
    #[inline(always)]
    pub fn rxintmod(&mut self) -> RXINTMOD_W {
        RXINTMOD_W::new(self)
    }
    #[doc = "Bits 16:23 - Count of 800ns periods before bit 7 is set in the interrupt status register after a frame is transmitted"]
    #[inline(always)]
    pub fn txintmod(&mut self) -> TXINTMOD_W {
        TXINTMOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt moderation register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imod](index.html) module"]
pub struct IMOD_SPEC;
impl crate::RegisterSpec for IMOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imod::R](R) reader structure"]
impl crate::Readable for IMOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imod::W](W) writer structure"]
impl crate::Writable for IMOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMOD to value 0"]
impl crate::Resettable for IMOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
