#[doc = "Register `TXPAUSEQUANT1` reader"]
pub struct R(crate::R<TXPAUSEQUANT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXPAUSEQUANT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXPAUSEQUANT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXPAUSEQUANT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXPAUSEQUANT1` writer"]
pub struct W(crate::W<TXPAUSEQUANT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXPAUSEQUANT1_SPEC>;
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
impl From<crate::W<TXPAUSEQUANT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXPAUSEQUANT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QUANTP2` reader - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 2."]
pub type QUANTP2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `QUANTP2` writer - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 2."]
pub type QUANTP2_W<'a> = crate::FieldWriter<'a, u32, TXPAUSEQUANT1_SPEC, u16, u16, 16, 0>;
#[doc = "Field `QUANTP3` reader - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 3."]
pub type QUANTP3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `QUANTP3` writer - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 3."]
pub type QUANTP3_W<'a> = crate::FieldWriter<'a, u32, TXPAUSEQUANT1_SPEC, u16, u16, 16, 16>;
impl R {
    #[doc = "Bits 0:15 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 2."]
    #[inline(always)]
    pub fn quantp2(&self) -> QUANTP2_R {
        QUANTP2_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 3."]
    #[inline(always)]
    pub fn quantp3(&self) -> QUANTP3_R {
        QUANTP3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 2."]
    #[inline(always)]
    pub fn quantp2(&mut self) -> QUANTP2_W {
        QUANTP2_W::new(self)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 3."]
    #[inline(always)]
    pub fn quantp3(&mut self) -> QUANTP3_W {
        QUANTP3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Pause Quantum Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txpausequant1](index.html) module"]
pub struct TXPAUSEQUANT1_SPEC;
impl crate::RegisterSpec for TXPAUSEQUANT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txpausequant1::R](R) reader structure"]
impl crate::Readable for TXPAUSEQUANT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txpausequant1::W](W) writer structure"]
impl crate::Writable for TXPAUSEQUANT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXPAUSEQUANT1 to value 0xffff_ffff"]
impl crate::Resettable for TXPAUSEQUANT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
