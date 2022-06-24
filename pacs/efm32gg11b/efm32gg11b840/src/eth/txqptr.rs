#[doc = "Register `TXQPTR` reader"]
pub struct R(crate::R<TXQPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXQPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXQPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXQPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXQPTR` writer"]
pub struct W(crate::W<TXQPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXQPTR_SPEC>;
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
impl From<crate::W<TXQPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXQPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMATXQPTR` reader - Transmit buffer queue base address"]
pub type DMATXQPTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DMATXQPTR` writer - Transmit buffer queue base address"]
pub type DMATXQPTR_W<'a> = crate::FieldWriter<'a, u32, TXQPTR_SPEC, u32, u32, 30, 2>;
impl R {
    #[doc = "Bits 2:31 - Transmit buffer queue base address"]
    #[inline(always)]
    pub fn dmatxqptr(&self) -> DMATXQPTR_R {
        DMATXQPTR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Transmit buffer queue base address"]
    #[inline(always)]
    pub fn dmatxqptr(&mut self) -> DMATXQPTR_W {
        DMATXQPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start address of the transmit buffer queue\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txqptr](index.html) module"]
pub struct TXQPTR_SPEC;
impl crate::RegisterSpec for TXQPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txqptr::R](R) reader structure"]
impl crate::Readable for TXQPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txqptr::W](W) writer structure"]
impl crate::Writable for TXQPTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXQPTR to value 0"]
impl crate::Resettable for TXQPTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
