#[doc = "Register `TXPAUSEQUANT2` reader"]
pub struct R(crate::R<TXPAUSEQUANT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXPAUSEQUANT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXPAUSEQUANT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXPAUSEQUANT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXPAUSEQUANT2` writer"]
pub struct W(crate::W<TXPAUSEQUANT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXPAUSEQUANT2_SPEC>;
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
impl From<crate::W<TXPAUSEQUANT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXPAUSEQUANT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QUANTP4` reader - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 4."]
pub type QUANTP4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `QUANTP4` writer - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 4."]
pub type QUANTP4_W<'a> = crate::FieldWriter<'a, u32, TXPAUSEQUANT2_SPEC, u16, u16, 16, 0>;
#[doc = "Field `QUANTP5` reader - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 5."]
pub type QUANTP5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `QUANTP5` writer - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 5."]
pub type QUANTP5_W<'a> = crate::FieldWriter<'a, u32, TXPAUSEQUANT2_SPEC, u16, u16, 16, 16>;
impl R {
    #[doc = "Bits 0:15 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 4."]
    #[inline(always)]
    pub fn quantp4(&self) -> QUANTP4_R {
        QUANTP4_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 5."]
    #[inline(always)]
    pub fn quantp5(&self) -> QUANTP5_R {
        QUANTP5_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 4."]
    #[inline(always)]
    pub fn quantp4(&mut self) -> QUANTP4_W {
        QUANTP4_W::new(self)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 5."]
    #[inline(always)]
    pub fn quantp5(&mut self) -> QUANTP5_W {
        QUANTP5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Pause Quantum Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txpausequant2](index.html) module"]
pub struct TXPAUSEQUANT2_SPEC;
impl crate::RegisterSpec for TXPAUSEQUANT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txpausequant2::R](R) reader structure"]
impl crate::Readable for TXPAUSEQUANT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txpausequant2::W](W) writer structure"]
impl crate::Writable for TXPAUSEQUANT2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXPAUSEQUANT2 to value 0xffff_ffff"]
impl crate::Resettable for TXPAUSEQUANT2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
