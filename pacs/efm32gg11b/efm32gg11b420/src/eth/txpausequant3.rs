#[doc = "Register `TXPAUSEQUANT3` reader"]
pub struct R(crate::R<TXPAUSEQUANT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXPAUSEQUANT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXPAUSEQUANT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXPAUSEQUANT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXPAUSEQUANT3` writer"]
pub struct W(crate::W<TXPAUSEQUANT3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXPAUSEQUANT3_SPEC>;
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
impl From<crate::W<TXPAUSEQUANT3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXPAUSEQUANT3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QUANTP6` reader - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 6."]
pub type QUANTP6_R = crate::FieldReader<u16, u16>;
#[doc = "Field `QUANTP6` writer - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 6."]
pub type QUANTP6_W<'a> = crate::FieldWriter<'a, u32, TXPAUSEQUANT3_SPEC, u16, u16, 16, 0>;
#[doc = "Field `QUANTP7` reader - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 7."]
pub type QUANTP7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `QUANTP7` writer - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 7."]
pub type QUANTP7_W<'a> = crate::FieldWriter<'a, u32, TXPAUSEQUANT3_SPEC, u16, u16, 16, 16>;
impl R {
    #[doc = "Bits 0:15 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 6."]
    #[inline(always)]
    pub fn quantp6(&self) -> QUANTP6_R {
        QUANTP6_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 7."]
    #[inline(always)]
    pub fn quantp7(&self) -> QUANTP7_R {
        QUANTP7_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 6."]
    #[inline(always)]
    pub fn quantp6(&mut self) -> QUANTP6_W {
        QUANTP6_W::new(self)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 7."]
    #[inline(always)]
    pub fn quantp7(&mut self) -> QUANTP7_W {
        QUANTP7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Pause Quantum Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txpausequant3](index.html) module"]
pub struct TXPAUSEQUANT3_SPEC;
impl crate::RegisterSpec for TXPAUSEQUANT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txpausequant3::R](R) reader structure"]
impl crate::Readable for TXPAUSEQUANT3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txpausequant3::W](W) writer structure"]
impl crate::Writable for TXPAUSEQUANT3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXPAUSEQUANT3 to value 0xffff_ffff"]
impl crate::Resettable for TXPAUSEQUANT3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
