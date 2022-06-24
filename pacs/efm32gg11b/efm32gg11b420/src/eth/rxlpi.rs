#[doc = "Register `RXLPI` reader"]
pub struct R(crate::R<RXLPI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXLPI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXLPI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXLPI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXLPI` writer"]
pub struct W(crate::W<RXLPI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXLPI_SPEC>;
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
impl From<crate::W<RXLPI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXLPI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Count of RX LPI transitions"]
pub type COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COUNT` writer - Count of RX LPI transitions"]
pub type COUNT_W<'a> = crate::FieldWriter<'a, u32, RXLPI_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Count of RX LPI transitions"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Count of RX LPI transitions"]
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
#[doc = "Received LPI transitions\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxlpi](index.html) module"]
pub struct RXLPI_SPEC;
impl crate::RegisterSpec for RXLPI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxlpi::R](R) reader structure"]
impl crate::Readable for RXLPI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxlpi::W](W) writer structure"]
impl crate::Writable for RXLPI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXLPI to value 0"]
impl crate::Resettable for RXLPI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
