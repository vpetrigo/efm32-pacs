#[doc = "Register `TXLPI` reader"]
pub struct R(crate::R<TXLPI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXLPI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXLPI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXLPI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXLPI` writer"]
pub struct W(crate::W<TXLPI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXLPI_SPEC>;
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
impl From<crate::W<TXLPI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXLPI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Count of LPI transmitions"]
pub type COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COUNT` writer - Count of LPI transmitions"]
pub type COUNT_W<'a> = crate::FieldWriter<'a, u32, TXLPI_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Count of LPI transmitions"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Count of LPI transmitions"]
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
#[doc = "Transmit LPI transitions\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txlpi](index.html) module"]
pub struct TXLPI_SPEC;
impl crate::RegisterSpec for TXLPI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txlpi::R](R) reader structure"]
impl crate::Readable for TXLPI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txlpi::W](W) writer structure"]
impl crate::Writable for TXLPI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXLPI to value 0"]
impl crate::Resettable for TXLPI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
