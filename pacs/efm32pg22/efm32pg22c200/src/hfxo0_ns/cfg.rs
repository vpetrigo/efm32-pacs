#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Crystal Oscillator Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: crystal oscillator"]
    XTAL = 0,
    #[doc = "1: external sinusoidal clock can be supplied on XI pin."]
    EXTCLK = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Crystal Oscillator Mode"]
pub type MODE_R = crate::BitReader<MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::XTAL,
            true => MODE_A::EXTCLK,
        }
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == MODE_A::XTAL
    }
    #[doc = "Checks if the value of the field is `EXTCLK`"]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        *self == MODE_A::EXTCLK
    }
}
#[doc = "Field `MODE` writer - Crystal Oscillator Mode"]
pub type MODE_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, MODE_A, 0>;
impl<'a> MODE_W<'a> {
    #[doc = "crystal oscillator"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut W {
        self.variant(MODE_A::XTAL)
    }
    #[doc = "external sinusoidal clock can be supplied on XI pin."]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut W {
        self.variant(MODE_A::EXTCLK)
    }
}
#[doc = "Field `ENXIDCBIASANA` reader - Enable XI Internal DC Bias"]
pub type ENXIDCBIASANA_R = crate::BitReader<bool>;
#[doc = "Field `ENXIDCBIASANA` writer - Enable XI Internal DC Bias"]
pub type ENXIDCBIASANA_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 2>;
#[doc = "Squaring Buffer Schmitt Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SQBUFSCHTRGANA_A {
    #[doc = "0: Squaring buffer schmitt trigger is disabled"]
    DISABLE = 0,
    #[doc = "1: Squaring buffer schmitt trigger is enabled"]
    ENABLE = 1,
}
impl From<SQBUFSCHTRGANA_A> for bool {
    #[inline(always)]
    fn from(variant: SQBUFSCHTRGANA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SQBUFSCHTRGANA` reader - Squaring Buffer Schmitt Trigger"]
pub type SQBUFSCHTRGANA_R = crate::BitReader<SQBUFSCHTRGANA_A>;
impl SQBUFSCHTRGANA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SQBUFSCHTRGANA_A {
        match self.bits {
            false => SQBUFSCHTRGANA_A::DISABLE,
            true => SQBUFSCHTRGANA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SQBUFSCHTRGANA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SQBUFSCHTRGANA_A::ENABLE
    }
}
#[doc = "Field `SQBUFSCHTRGANA` writer - Squaring Buffer Schmitt Trigger"]
pub type SQBUFSCHTRGANA_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, SQBUFSCHTRGANA_A, 3>;
impl<'a> SQBUFSCHTRGANA_W<'a> {
    #[doc = "Squaring buffer schmitt trigger is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SQBUFSCHTRGANA_A::DISABLE)
    }
    #[doc = "Squaring buffer schmitt trigger is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SQBUFSCHTRGANA_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Crystal Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Enable XI Internal DC Bias"]
    #[inline(always)]
    pub fn enxidcbiasana(&self) -> ENXIDCBIASANA_R {
        ENXIDCBIASANA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Squaring Buffer Schmitt Trigger"]
    #[inline(always)]
    pub fn sqbufschtrgana(&self) -> SQBUFSCHTRGANA_R {
        SQBUFSCHTRGANA_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Crystal Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W::new(self)
    }
    #[doc = "Bit 2 - Enable XI Internal DC Bias"]
    #[inline(always)]
    pub fn enxidcbiasana(&mut self) -> ENXIDCBIASANA_W {
        ENXIDCBIASANA_W::new(self)
    }
    #[doc = "Bit 3 - Squaring Buffer Schmitt Trigger"]
    #[inline(always)]
    pub fn sqbufschtrgana(&mut self) -> SQBUFSCHTRGANA_W {
        SQBUFSCHTRGANA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0x1000_0000"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000_0000
    }
}
