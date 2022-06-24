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
#[doc = "Field `CH4PULSE` writer - Channel 4 Pulse Generation"]
pub type CH4PULSE_W<'a> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, 4>;
#[doc = "Field `CH5PULSE` writer - Channel 5 Pulse Generation"]
pub type CH5PULSE_W<'a> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, 5>;
#[doc = "Field `CH6PULSE` writer - Channel 6 Pulse Generation"]
pub type CH6PULSE_W<'a> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, 6>;
#[doc = "Field `CH7PULSE` writer - Channel 7 Pulse Generation"]
pub type CH7PULSE_W<'a> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, 7>;
#[doc = "Field `CH8PULSE` writer - Channel 8 Pulse Generation"]
pub type CH8PULSE_W<'a> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, 8>;
#[doc = "Field `CH9PULSE` writer - Channel 9 Pulse Generation"]
pub type CH9PULSE_W<'a> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, 9>;
#[doc = "Field `CH10PULSE` writer - Channel 10 Pulse Generation"]
pub type CH10PULSE_W<'a> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, 10>;
#[doc = "Field `CH11PULSE` writer - Channel 11 Pulse Generation"]
pub type CH11PULSE_W<'a> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, 11>;
#[doc = "Field `CH12PULSE` writer - Channel 12 Pulse Generation"]
pub type CH12PULSE_W<'a> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, 12>;
#[doc = "Field `CH13PULSE` writer - Channel 13 Pulse Generation"]
pub type CH13PULSE_W<'a> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, 13>;
#[doc = "Field `CH14PULSE` writer - Channel 14 Pulse Generation"]
pub type CH14PULSE_W<'a> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, 14>;
#[doc = "Field `CH15PULSE` writer - Channel 15 Pulse Generation"]
pub type CH15PULSE_W<'a> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, 15>;
#[doc = "Field `CH16PULSE` writer - Channel 16 Pulse Generation"]
pub type CH16PULSE_W<'a> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, 16>;
#[doc = "Field `CH17PULSE` writer - Channel 17 Pulse Generation"]
pub type CH17PULSE_W<'a> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, 17>;
#[doc = "Field `CH18PULSE` writer - Channel 18 Pulse Generation"]
pub type CH18PULSE_W<'a> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, 18>;
#[doc = "Field `CH19PULSE` writer - Channel 19 Pulse Generation"]
pub type CH19PULSE_W<'a> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, 19>;
#[doc = "Field `CH20PULSE` writer - Channel 20 Pulse Generation"]
pub type CH20PULSE_W<'a> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, 20>;
#[doc = "Field `CH21PULSE` writer - Channel 21 Pulse Generation"]
pub type CH21PULSE_W<'a> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, 21>;
#[doc = "Field `CH22PULSE` writer - Channel 22 Pulse Generation"]
pub type CH22PULSE_W<'a> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, 22>;
#[doc = "Field `CH23PULSE` writer - Channel 23 Pulse Generation"]
pub type CH23PULSE_W<'a> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, 23>;
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
    #[doc = "Bit 4 - Channel 4 Pulse Generation"]
    #[inline(always)]
    pub fn ch4pulse(&mut self) -> CH4PULSE_W {
        CH4PULSE_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Pulse Generation"]
    #[inline(always)]
    pub fn ch5pulse(&mut self) -> CH5PULSE_W {
        CH5PULSE_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Pulse Generation"]
    #[inline(always)]
    pub fn ch6pulse(&mut self) -> CH6PULSE_W {
        CH6PULSE_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Pulse Generation"]
    #[inline(always)]
    pub fn ch7pulse(&mut self) -> CH7PULSE_W {
        CH7PULSE_W::new(self)
    }
    #[doc = "Bit 8 - Channel 8 Pulse Generation"]
    #[inline(always)]
    pub fn ch8pulse(&mut self) -> CH8PULSE_W {
        CH8PULSE_W::new(self)
    }
    #[doc = "Bit 9 - Channel 9 Pulse Generation"]
    #[inline(always)]
    pub fn ch9pulse(&mut self) -> CH9PULSE_W {
        CH9PULSE_W::new(self)
    }
    #[doc = "Bit 10 - Channel 10 Pulse Generation"]
    #[inline(always)]
    pub fn ch10pulse(&mut self) -> CH10PULSE_W {
        CH10PULSE_W::new(self)
    }
    #[doc = "Bit 11 - Channel 11 Pulse Generation"]
    #[inline(always)]
    pub fn ch11pulse(&mut self) -> CH11PULSE_W {
        CH11PULSE_W::new(self)
    }
    #[doc = "Bit 12 - Channel 12 Pulse Generation"]
    #[inline(always)]
    pub fn ch12pulse(&mut self) -> CH12PULSE_W {
        CH12PULSE_W::new(self)
    }
    #[doc = "Bit 13 - Channel 13 Pulse Generation"]
    #[inline(always)]
    pub fn ch13pulse(&mut self) -> CH13PULSE_W {
        CH13PULSE_W::new(self)
    }
    #[doc = "Bit 14 - Channel 14 Pulse Generation"]
    #[inline(always)]
    pub fn ch14pulse(&mut self) -> CH14PULSE_W {
        CH14PULSE_W::new(self)
    }
    #[doc = "Bit 15 - Channel 15 Pulse Generation"]
    #[inline(always)]
    pub fn ch15pulse(&mut self) -> CH15PULSE_W {
        CH15PULSE_W::new(self)
    }
    #[doc = "Bit 16 - Channel 16 Pulse Generation"]
    #[inline(always)]
    pub fn ch16pulse(&mut self) -> CH16PULSE_W {
        CH16PULSE_W::new(self)
    }
    #[doc = "Bit 17 - Channel 17 Pulse Generation"]
    #[inline(always)]
    pub fn ch17pulse(&mut self) -> CH17PULSE_W {
        CH17PULSE_W::new(self)
    }
    #[doc = "Bit 18 - Channel 18 Pulse Generation"]
    #[inline(always)]
    pub fn ch18pulse(&mut self) -> CH18PULSE_W {
        CH18PULSE_W::new(self)
    }
    #[doc = "Bit 19 - Channel 19 Pulse Generation"]
    #[inline(always)]
    pub fn ch19pulse(&mut self) -> CH19PULSE_W {
        CH19PULSE_W::new(self)
    }
    #[doc = "Bit 20 - Channel 20 Pulse Generation"]
    #[inline(always)]
    pub fn ch20pulse(&mut self) -> CH20PULSE_W {
        CH20PULSE_W::new(self)
    }
    #[doc = "Bit 21 - Channel 21 Pulse Generation"]
    #[inline(always)]
    pub fn ch21pulse(&mut self) -> CH21PULSE_W {
        CH21PULSE_W::new(self)
    }
    #[doc = "Bit 22 - Channel 22 Pulse Generation"]
    #[inline(always)]
    pub fn ch22pulse(&mut self) -> CH22PULSE_W {
        CH22PULSE_W::new(self)
    }
    #[doc = "Bit 23 - Channel 23 Pulse Generation"]
    #[inline(always)]
    pub fn ch23pulse(&mut self) -> CH23PULSE_W {
        CH23PULSE_W::new(self)
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
