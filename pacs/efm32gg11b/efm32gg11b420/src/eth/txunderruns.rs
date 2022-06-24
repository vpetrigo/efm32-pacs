#[doc = "Register `TXUNDERRUNS` reader"]
pub struct R(crate::R<TXUNDERRUNS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXUNDERRUNS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXUNDERRUNS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXUNDERRUNS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXUNDERRUNS` writer"]
pub struct W(crate::W<TXUNDERRUNS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXUNDERRUNS_SPEC>;
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
impl From<crate::W<TXUNDERRUNS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXUNDERRUNS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Transmit under runs"]
pub type COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COUNT` writer - Transmit under runs"]
pub type COUNT_W<'a> = crate::FieldWriter<'a, u32, TXUNDERRUNS_SPEC, u16, u16, 10, 0>;
impl R {
    #[doc = "Bits 0:9 - Transmit under runs"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Transmit under runs"]
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
#[doc = "Transmit Under Runs\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txunderruns](index.html) module"]
pub struct TXUNDERRUNS_SPEC;
impl crate::RegisterSpec for TXUNDERRUNS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txunderruns::R](R) reader structure"]
impl crate::Readable for TXUNDERRUNS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txunderruns::W](W) writer structure"]
impl crate::Writable for TXUNDERRUNS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXUNDERRUNS to value 0"]
impl crate::Resettable for TXUNDERRUNS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
