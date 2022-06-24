#[doc = "Register `RXQPTR` reader"]
pub struct R(crate::R<RXQPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXQPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXQPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXQPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXQPTR` writer"]
pub struct W(crate::W<RXQPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXQPTR_SPEC>;
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
impl From<crate::W<RXQPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXQPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMARXQPTR` reader - Receive buffer queue base address"]
pub type DMARXQPTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DMARXQPTR` writer - Receive buffer queue base address"]
pub type DMARXQPTR_W<'a> = crate::FieldWriter<'a, u32, RXQPTR_SPEC, u32, u32, 30, 2>;
impl R {
    #[doc = "Bits 2:31 - Receive buffer queue base address"]
    #[inline(always)]
    pub fn dmarxqptr(&self) -> DMARXQPTR_R {
        DMARXQPTR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Receive buffer queue base address"]
    #[inline(always)]
    pub fn dmarxqptr(&mut self) -> DMARXQPTR_W {
        DMARXQPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start address of the receive buffer queue\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxqptr](index.html) module"]
pub struct RXQPTR_SPEC;
impl crate::RegisterSpec for RXQPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxqptr::R](R) reader structure"]
impl crate::Readable for RXQPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxqptr::W](W) writer structure"]
impl crate::Writable for RXQPTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXQPTR to value 0"]
impl crate::Resettable for RXQPTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
