#[doc = "Register `SPECADDR1BOTTOM` reader"]
pub struct R(crate::R<SPECADDR1BOTTOM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPECADDR1BOTTOM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPECADDR1BOTTOM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPECADDR1BOTTOM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPECADDR1BOTTOM` writer"]
pub struct W(crate::W<SPECADDR1BOTTOM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPECADDR1BOTTOM_SPEC>;
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
impl From<crate::W<SPECADDR1BOTTOM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPECADDR1BOTTOM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Least significant 32 bits of the destination address"]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - Least significant 32 bits of the destination address"]
pub type ADDR_W<'a> = crate::FieldWriter<'a, u32, SPECADDR1BOTTOM_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Least significant 32 bits of the destination address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Least significant 32 bits of the destination address"]
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
#[doc = "Specific Address 1 Bottom\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [specaddr1bottom](index.html) module"]
pub struct SPECADDR1BOTTOM_SPEC;
impl crate::RegisterSpec for SPECADDR1BOTTOM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [specaddr1bottom::R](R) reader structure"]
impl crate::Readable for SPECADDR1BOTTOM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [specaddr1bottom::W](W) writer structure"]
impl crate::Writable for SPECADDR1BOTTOM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPECADDR1BOTTOM to value 0"]
impl crate::Resettable for SPECADDR1BOTTOM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
