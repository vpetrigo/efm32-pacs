#[doc = "Register `TXPAUSEQUANT` reader"]
pub struct R(crate::R<TXPAUSEQUANT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXPAUSEQUANT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXPAUSEQUANT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXPAUSEQUANT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXPAUSEQUANT` writer"]
pub struct W(crate::W<TXPAUSEQUANT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXPAUSEQUANT_SPEC>;
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
impl From<crate::W<TXPAUSEQUANT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXPAUSEQUANT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QUANT` reader - Transmit pause quantum"]
pub type QUANT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `QUANT` writer - Transmit pause quantum"]
pub type QUANT_W<'a> = crate::FieldWriter<'a, u32, TXPAUSEQUANT_SPEC, u16, u16, 16, 0>;
#[doc = "Field `QUANTP1` reader - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 1."]
pub type QUANTP1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `QUANTP1` writer - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 1."]
pub type QUANTP1_W<'a> = crate::FieldWriter<'a, u32, TXPAUSEQUANT_SPEC, u16, u16, 16, 16>;
impl R {
    #[doc = "Bits 0:15 - Transmit pause quantum"]
    #[inline(always)]
    pub fn quant(&self) -> QUANT_R {
        QUANT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 1."]
    #[inline(always)]
    pub fn quantp1(&self) -> QUANTP1_R {
        QUANTP1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit pause quantum"]
    #[inline(always)]
    pub fn quant(&mut self) -> QUANT_W {
        QUANT_W::new(self)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 1."]
    #[inline(always)]
    pub fn quantp1(&mut self) -> QUANTP1_W {
        QUANTP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Pause Quantum Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txpausequant](index.html) module"]
pub struct TXPAUSEQUANT_SPEC;
impl crate::RegisterSpec for TXPAUSEQUANT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txpausequant::R](R) reader structure"]
impl crate::Readable for TXPAUSEQUANT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txpausequant::W](W) writer structure"]
impl crate::Writable for TXPAUSEQUANT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXPAUSEQUANT to value 0xffff_ffff"]
impl crate::Resettable for TXPAUSEQUANT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
