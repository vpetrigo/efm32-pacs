#[doc = "Register `HFXOSTEADYSTATECTRL` reader"]
pub struct R(crate::R<HFXOSTEADYSTATECTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFXOSTEADYSTATECTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFXOSTEADYSTATECTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFXOSTEADYSTATECTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFXOSTEADYSTATECTRL` writer"]
pub struct W(crate::W<HFXOSTEADYSTATECTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFXOSTEADYSTATECTRL_SPEC>;
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
impl From<crate::W<HFXOSTEADYSTATECTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFXOSTEADYSTATECTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IBTRIMXOCORE` reader - Sets the Steady State Oscillator Core Bias Current."]
pub type IBTRIMXOCORE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IBTRIMXOCORE` writer - Sets the Steady State Oscillator Core Bias Current."]
pub type IBTRIMXOCORE_W<'a> = crate::FieldWriter<'a, u32, HFXOSTEADYSTATECTRL_SPEC, u8, u8, 7, 0>;
#[doc = "Field `REGISH` reader - Sets the Steady State Regulator Output Current Level (shunt Regulator)"]
pub type REGISH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REGISH` writer - Sets the Steady State Regulator Output Current Level (shunt Regulator)"]
pub type REGISH_W<'a> = crate::FieldWriter<'a, u32, HFXOSTEADYSTATECTRL_SPEC, u8, u8, 4, 7>;
#[doc = "Field `CTUNE` reader - Sets Oscillator Tuning Capacitance"]
pub type CTUNE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CTUNE` writer - Sets Oscillator Tuning Capacitance"]
pub type CTUNE_W<'a> = crate::FieldWriter<'a, u32, HFXOSTEADYSTATECTRL_SPEC, u16, u16, 9, 11>;
#[doc = "Field `REGSELILOW` reader - Controls Regulator Minimum Shunt Current Detection Relative to Nominal"]
pub type REGSELILOW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REGSELILOW` writer - Controls Regulator Minimum Shunt Current Detection Relative to Nominal"]
pub type REGSELILOW_W<'a> = crate::FieldWriter<'a, u32, HFXOSTEADYSTATECTRL_SPEC, u8, u8, 2, 24>;
#[doc = "Field `PEAKDETEN` reader - Enables Oscillator Peak Detectors"]
pub type PEAKDETEN_R = crate::BitReader<bool>;
#[doc = "Field `PEAKDETEN` writer - Enables Oscillator Peak Detectors"]
pub type PEAKDETEN_W<'a> = crate::BitWriter<'a, u32, HFXOSTEADYSTATECTRL_SPEC, bool, 26>;
#[doc = "Field `REGISHUPPER` reader - Set Regulator Output Current Level (shunt Regulator). Ish = 120uA + REGISHUPPER X 120uA"]
pub type REGISHUPPER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REGISHUPPER` writer - Set Regulator Output Current Level (shunt Regulator). Ish = 120uA + REGISHUPPER X 120uA"]
pub type REGISHUPPER_W<'a> = crate::FieldWriter<'a, u32, HFXOSTEADYSTATECTRL_SPEC, u8, u8, 4, 28>;
impl R {
    #[doc = "Bits 0:6 - Sets the Steady State Oscillator Core Bias Current."]
    #[inline(always)]
    pub fn ibtrimxocore(&self) -> IBTRIMXOCORE_R {
        IBTRIMXOCORE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:10 - Sets the Steady State Regulator Output Current Level (shunt Regulator)"]
    #[inline(always)]
    pub fn regish(&self) -> REGISH_R {
        REGISH_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline(always)]
    pub fn ctune(&self) -> CTUNE_R {
        CTUNE_R::new(((self.bits >> 11) & 0x01ff) as u16)
    }
    #[doc = "Bits 24:25 - Controls Regulator Minimum Shunt Current Detection Relative to Nominal"]
    #[inline(always)]
    pub fn regselilow(&self) -> REGSELILOW_R {
        REGSELILOW_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Enables Oscillator Peak Detectors"]
    #[inline(always)]
    pub fn peakdeten(&self) -> PEAKDETEN_R {
        PEAKDETEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Set Regulator Output Current Level (shunt Regulator). Ish = 120uA + REGISHUPPER X 120uA"]
    #[inline(always)]
    pub fn regishupper(&self) -> REGISHUPPER_R {
        REGISHUPPER_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Sets the Steady State Oscillator Core Bias Current."]
    #[inline(always)]
    pub fn ibtrimxocore(&mut self) -> IBTRIMXOCORE_W {
        IBTRIMXOCORE_W::new(self)
    }
    #[doc = "Bits 7:10 - Sets the Steady State Regulator Output Current Level (shunt Regulator)"]
    #[inline(always)]
    pub fn regish(&mut self) -> REGISH_W {
        REGISH_W::new(self)
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline(always)]
    pub fn ctune(&mut self) -> CTUNE_W {
        CTUNE_W::new(self)
    }
    #[doc = "Bits 24:25 - Controls Regulator Minimum Shunt Current Detection Relative to Nominal"]
    #[inline(always)]
    pub fn regselilow(&mut self) -> REGSELILOW_W {
        REGSELILOW_W::new(self)
    }
    #[doc = "Bit 26 - Enables Oscillator Peak Detectors"]
    #[inline(always)]
    pub fn peakdeten(&mut self) -> PEAKDETEN_W {
        PEAKDETEN_W::new(self)
    }
    #[doc = "Bits 28:31 - Set Regulator Output Current Level (shunt Regulator). Ish = 120uA + REGISHUPPER X 120uA"]
    #[inline(always)]
    pub fn regishupper(&mut self) -> REGISHUPPER_W {
        REGISHUPPER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HFXO Steady State Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfxosteadystatectrl](index.html) module"]
pub struct HFXOSTEADYSTATECTRL_SPEC;
impl crate::RegisterSpec for HFXOSTEADYSTATECTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfxosteadystatectrl::R](R) reader structure"]
impl crate::Readable for HFXOSTEADYSTATECTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfxosteadystatectrl::W](W) writer structure"]
impl crate::Writable for HFXOSTEADYSTATECTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HFXOSTEADYSTATECTRL to value 0xa30b_4507"]
impl crate::Resettable for HFXOSTEADYSTATECTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa30b_4507
    }
}
