#[doc = "Register `SWPULSE` writer"]
pub struct W(crate::W<SWPULSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWPULSE_SPEC>;
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
impl From<crate::W<SWPULSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWPULSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0PULSE` writer - Channel 0 Pulse Generation"]
pub type CH0PULSE_W<'a> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, 0>;
#[doc = "Field `CH1PULSE` writer - Channel 1 Pulse Generation"]
pub type CH1PULSE_W<'a> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, 1>;
#[doc = "Field `CH2PULSE` writer - Channel 2 Pulse Generation"]
pub type CH2PULSE_W<'a> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, 2>;
#[doc = "Field `CH3PULSE` writer - Channel 3 Pulse Generation"]
pub type CH3PULSE_W<'a> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, 3>;
impl W {
    #[doc = "Bit 0 - Channel 0 Pulse Generation"]
    #[inline(always)]
    pub fn ch0pulse(&mut self) -> CH0PULSE_W {
        CH0PULSE_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Pulse Generation"]
    #[inline(always)]
    pub fn ch1pulse(&mut self) -> CH1PULSE_W {
        CH1PULSE_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Pulse Generation"]
    #[inline(always)]
    pub fn ch2pulse(&mut self) -> CH2PULSE_W {
        CH2PULSE_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Pulse Generation"]
    #[inline(always)]
    pub fn ch3pulse(&mut self) -> CH3PULSE_W {
        CH3PULSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Pulse Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swpulse](index.html) module"]
pub struct SWPULSE_SPEC;
impl crate::RegisterSpec for SWPULSE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [swpulse::W](W) writer structure"]
impl crate::Writable for SWPULSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWPULSE to value 0"]
impl crate::Resettable for SWPULSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
