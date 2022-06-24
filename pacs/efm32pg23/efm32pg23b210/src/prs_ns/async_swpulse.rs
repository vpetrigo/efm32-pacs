#[doc = "Register `ASYNC_SWPULSE` writer"]
pub struct W(crate::W<ASYNC_SWPULSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASYNC_SWPULSE_SPEC>;
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
impl From<crate::W<ASYNC_SWPULSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASYNC_SWPULSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0PULSE` writer - Channel pulse"]
pub type CH0PULSE_W<'a> = crate::BitWriter<'a, u32, ASYNC_SWPULSE_SPEC, bool, 0>;
#[doc = "Field `CH1PULSE` writer - Channel pulse"]
pub type CH1PULSE_W<'a> = crate::BitWriter<'a, u32, ASYNC_SWPULSE_SPEC, bool, 1>;
#[doc = "Field `CH2PULSE` writer - Channel pulse"]
pub type CH2PULSE_W<'a> = crate::BitWriter<'a, u32, ASYNC_SWPULSE_SPEC, bool, 2>;
#[doc = "Field `CH3PULSE` writer - Channel pulse"]
pub type CH3PULSE_W<'a> = crate::BitWriter<'a, u32, ASYNC_SWPULSE_SPEC, bool, 3>;
#[doc = "Field `CH4PULSE` writer - Channel pulse"]
pub type CH4PULSE_W<'a> = crate::BitWriter<'a, u32, ASYNC_SWPULSE_SPEC, bool, 4>;
#[doc = "Field `CH5PULSE` writer - Channel pulse"]
pub type CH5PULSE_W<'a> = crate::BitWriter<'a, u32, ASYNC_SWPULSE_SPEC, bool, 5>;
#[doc = "Field `CH6PULSE` writer - Channel pulse"]
pub type CH6PULSE_W<'a> = crate::BitWriter<'a, u32, ASYNC_SWPULSE_SPEC, bool, 6>;
#[doc = "Field `CH7PULSE` writer - Channel pulse"]
pub type CH7PULSE_W<'a> = crate::BitWriter<'a, u32, ASYNC_SWPULSE_SPEC, bool, 7>;
#[doc = "Field `CH8PULSE` writer - Channel pulse"]
pub type CH8PULSE_W<'a> = crate::BitWriter<'a, u32, ASYNC_SWPULSE_SPEC, bool, 8>;
#[doc = "Field `CH9PULSE` writer - Channel pulse"]
pub type CH9PULSE_W<'a> = crate::BitWriter<'a, u32, ASYNC_SWPULSE_SPEC, bool, 9>;
#[doc = "Field `CH10PULSE` writer - Channel pulse"]
pub type CH10PULSE_W<'a> = crate::BitWriter<'a, u32, ASYNC_SWPULSE_SPEC, bool, 10>;
#[doc = "Field `CH11PULSE` writer - Channel pulse"]
pub type CH11PULSE_W<'a> = crate::BitWriter<'a, u32, ASYNC_SWPULSE_SPEC, bool, 11>;
impl W {
    #[doc = "Bit 0 - Channel pulse"]
    #[inline(always)]
    pub fn ch0pulse(&mut self) -> CH0PULSE_W {
        CH0PULSE_W::new(self)
    }
    #[doc = "Bit 1 - Channel pulse"]
    #[inline(always)]
    pub fn ch1pulse(&mut self) -> CH1PULSE_W {
        CH1PULSE_W::new(self)
    }
    #[doc = "Bit 2 - Channel pulse"]
    #[inline(always)]
    pub fn ch2pulse(&mut self) -> CH2PULSE_W {
        CH2PULSE_W::new(self)
    }
    #[doc = "Bit 3 - Channel pulse"]
    #[inline(always)]
    pub fn ch3pulse(&mut self) -> CH3PULSE_W {
        CH3PULSE_W::new(self)
    }
    #[doc = "Bit 4 - Channel pulse"]
    #[inline(always)]
    pub fn ch4pulse(&mut self) -> CH4PULSE_W {
        CH4PULSE_W::new(self)
    }
    #[doc = "Bit 5 - Channel pulse"]
    #[inline(always)]
    pub fn ch5pulse(&mut self) -> CH5PULSE_W {
        CH5PULSE_W::new(self)
    }
    #[doc = "Bit 6 - Channel pulse"]
    #[inline(always)]
    pub fn ch6pulse(&mut self) -> CH6PULSE_W {
        CH6PULSE_W::new(self)
    }
    #[doc = "Bit 7 - Channel pulse"]
    #[inline(always)]
    pub fn ch7pulse(&mut self) -> CH7PULSE_W {
        CH7PULSE_W::new(self)
    }
    #[doc = "Bit 8 - Channel pulse"]
    #[inline(always)]
    pub fn ch8pulse(&mut self) -> CH8PULSE_W {
        CH8PULSE_W::new(self)
    }
    #[doc = "Bit 9 - Channel pulse"]
    #[inline(always)]
    pub fn ch9pulse(&mut self) -> CH9PULSE_W {
        CH9PULSE_W::new(self)
    }
    #[doc = "Bit 10 - Channel pulse"]
    #[inline(always)]
    pub fn ch10pulse(&mut self) -> CH10PULSE_W {
        CH10PULSE_W::new(self)
    }
    #[doc = "Bit 11 - Channel pulse"]
    #[inline(always)]
    pub fn ch11pulse(&mut self) -> CH11PULSE_W {
        CH11PULSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [async_swpulse](index.html) module"]
pub struct ASYNC_SWPULSE_SPEC;
impl crate::RegisterSpec for ASYNC_SWPULSE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [async_swpulse::W](W) writer structure"]
impl crate::Writable for ASYNC_SWPULSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ASYNC_SWPULSE to value 0"]
impl crate::Resettable for ASYNC_SWPULSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
