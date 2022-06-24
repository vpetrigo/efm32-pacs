#[doc = "Register `HFXOCTRL` reader"]
pub struct R(crate::R<HFXOCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFXOCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFXOCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFXOCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFXOCTRL` writer"]
pub struct W(crate::W<HFXOCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFXOCTRL_SPEC>;
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
impl From<crate::W<HFXOCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFXOCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - HFXO Mode"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - HFXO Mode"]
pub type MODE_W<'a> = crate::BitWriter<'a, u32, HFXOCTRL_SPEC, bool, 0>;
#[doc = "HFXO Automatic Peak Detection and Shunt Current Optimization Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PEAKDETSHUNTOPTMODE_A {
    #[doc = "0: Automatic control of HFXO peak detection and shunt optimization sequences. CMU_CMD HFXOPEAKDETSTART and HFXOSHUNTOPTSTART can also be used."]
    AUTOCMD = 0,
    #[doc = "1: CMU_CMD HFXOPEAKDETSTART and HFXOSHUNTOPTSTART can be used to trigger peak detection and shunt optimization sequences."]
    CMD = 1,
    #[doc = "2: CMU_HFXOSTEADYSTATECTRL IBTRIMXOCORE, REGISH, REGSELILOW, and PEAKDETEN are under full software control and are allowed to be changed once HFXO is ready."]
    MANUAL = 2,
}
impl From<PEAKDETSHUNTOPTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PEAKDETSHUNTOPTMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PEAKDETSHUNTOPTMODE` reader - HFXO Automatic Peak Detection and Shunt Current Optimization Mode"]
pub type PEAKDETSHUNTOPTMODE_R = crate::FieldReader<u8, PEAKDETSHUNTOPTMODE_A>;
impl PEAKDETSHUNTOPTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PEAKDETSHUNTOPTMODE_A> {
        match self.bits {
            0 => Some(PEAKDETSHUNTOPTMODE_A::AUTOCMD),
            1 => Some(PEAKDETSHUNTOPTMODE_A::CMD),
            2 => Some(PEAKDETSHUNTOPTMODE_A::MANUAL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AUTOCMD`"]
    #[inline(always)]
    pub fn is_autocmd(&self) -> bool {
        *self == PEAKDETSHUNTOPTMODE_A::AUTOCMD
    }
    #[doc = "Checks if the value of the field is `CMD`"]
    #[inline(always)]
    pub fn is_cmd(&self) -> bool {
        *self == PEAKDETSHUNTOPTMODE_A::CMD
    }
    #[doc = "Checks if the value of the field is `MANUAL`"]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == PEAKDETSHUNTOPTMODE_A::MANUAL
    }
}
#[doc = "Field `PEAKDETSHUNTOPTMODE` writer - HFXO Automatic Peak Detection and Shunt Current Optimization Mode"]
pub type PEAKDETSHUNTOPTMODE_W<'a> =
    crate::FieldWriter<'a, u32, HFXOCTRL_SPEC, u8, PEAKDETSHUNTOPTMODE_A, 2, 4>;
impl<'a> PEAKDETSHUNTOPTMODE_W<'a> {
    #[doc = "Automatic control of HFXO peak detection and shunt optimization sequences. CMU_CMD HFXOPEAKDETSTART and HFXOSHUNTOPTSTART can also be used."]
    #[inline(always)]
    pub fn autocmd(self) -> &'a mut W {
        self.variant(PEAKDETSHUNTOPTMODE_A::AUTOCMD)
    }
    #[doc = "CMU_CMD HFXOPEAKDETSTART and HFXOSHUNTOPTSTART can be used to trigger peak detection and shunt optimization sequences."]
    #[inline(always)]
    pub fn cmd(self) -> &'a mut W {
        self.variant(PEAKDETSHUNTOPTMODE_A::CMD)
    }
    #[doc = "CMU_HFXOSTEADYSTATECTRL IBTRIMXOCORE, REGISH, REGSELILOW, and PEAKDETEN are under full software control and are allowed to be changed once HFXO is ready."]
    #[inline(always)]
    pub fn manual(self) -> &'a mut W {
        self.variant(PEAKDETSHUNTOPTMODE_A::MANUAL)
    }
}
#[doc = "Field `LOWPOWER` reader - Low Power Mode Control"]
pub type LOWPOWER_R = crate::BitReader<bool>;
#[doc = "Field `LOWPOWER` writer - Low Power Mode Control"]
pub type LOWPOWER_W<'a> = crate::BitWriter<'a, u32, HFXOCTRL_SPEC, bool, 8>;
#[doc = "Field `XTI2GND` reader - Clamp HFXTAL_N Pin to Ground When HFXO Oscillator is Off"]
pub type XTI2GND_R = crate::BitReader<bool>;
#[doc = "Field `XTI2GND` writer - Clamp HFXTAL_N Pin to Ground When HFXO Oscillator is Off"]
pub type XTI2GND_W<'a> = crate::BitWriter<'a, u32, HFXOCTRL_SPEC, bool, 9>;
#[doc = "Field `XTO2GND` reader - Clamp HFXTAL_P Pin to Ground When HFXO Oscillator is Off"]
pub type XTO2GND_R = crate::BitReader<bool>;
#[doc = "Field `XTO2GND` writer - Clamp HFXTAL_P Pin to Ground When HFXO Oscillator is Off"]
pub type XTO2GND_W<'a> = crate::BitWriter<'a, u32, HFXOCTRL_SPEC, bool, 10>;
#[doc = "HFXO Low Frequency Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LFTIMEOUT_A {
    #[doc = "0: Timeout period of 0 cycles (disabled)"]
    _0CYCLES = 0,
    #[doc = "1: Timeout period of 2 cycles"]
    _2CYCLES = 1,
    #[doc = "2: Timeout period of 4 cycles"]
    _4CYCLES = 2,
    #[doc = "3: Timeout period of 16 cycles"]
    _16CYCLES = 3,
    #[doc = "4: Timeout period of 32 cycles"]
    _32CYCLES = 4,
    #[doc = "5: Timeout period of 64 cycles"]
    _64CYCLES = 5,
    #[doc = "6: Timeout period of 1024 cycles"]
    _1KCYCLES = 6,
    #[doc = "7: Timeout period of 4096 cycles"]
    _4KCYCLES = 7,
}
impl From<LFTIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: LFTIMEOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LFTIMEOUT` reader - HFXO Low Frequency Timeout"]
pub type LFTIMEOUT_R = crate::FieldReader<u8, LFTIMEOUT_A>;
impl LFTIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFTIMEOUT_A {
        match self.bits {
            0 => LFTIMEOUT_A::_0CYCLES,
            1 => LFTIMEOUT_A::_2CYCLES,
            2 => LFTIMEOUT_A::_4CYCLES,
            3 => LFTIMEOUT_A::_16CYCLES,
            4 => LFTIMEOUT_A::_32CYCLES,
            5 => LFTIMEOUT_A::_64CYCLES,
            6 => LFTIMEOUT_A::_1KCYCLES,
            7 => LFTIMEOUT_A::_4KCYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0CYCLES`"]
    #[inline(always)]
    pub fn is_0cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_0CYCLES
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_64CYCLES`"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_64CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == LFTIMEOUT_A::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_4KCYCLES`"]
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == LFTIMEOUT_A::_4KCYCLES
    }
}
#[doc = "Field `LFTIMEOUT` writer - HFXO Low Frequency Timeout"]
pub type LFTIMEOUT_W<'a> = crate::FieldWriterSafe<'a, u32, HFXOCTRL_SPEC, u8, LFTIMEOUT_A, 3, 24>;
impl<'a> LFTIMEOUT_W<'a> {
    #[doc = "Timeout period of 0 cycles (disabled)"]
    #[inline(always)]
    pub fn _0cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_0CYCLES)
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_2CYCLES)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_4CYCLES)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_16CYCLES)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_32CYCLES)
    }
    #[doc = "Timeout period of 64 cycles"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_64CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_1KCYCLES)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_4KCYCLES)
    }
}
#[doc = "Field `AUTOSTARTEM0EM1` reader - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
pub type AUTOSTARTEM0EM1_R = crate::BitReader<bool>;
#[doc = "Field `AUTOSTARTEM0EM1` writer - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
pub type AUTOSTARTEM0EM1_W<'a> = crate::BitWriter<'a, u32, HFXOCTRL_SPEC, bool, 28>;
#[doc = "Field `AUTOSTARTSELEM0EM1` reader - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
pub type AUTOSTARTSELEM0EM1_R = crate::BitReader<bool>;
#[doc = "Field `AUTOSTARTSELEM0EM1` writer - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
pub type AUTOSTARTSELEM0EM1_W<'a> = crate::BitWriter<'a, u32, HFXOCTRL_SPEC, bool, 29>;
impl R {
    #[doc = "Bit 0 - HFXO Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - HFXO Automatic Peak Detection and Shunt Current Optimization Mode"]
    #[inline(always)]
    pub fn peakdetshuntoptmode(&self) -> PEAKDETSHUNTOPTMODE_R {
        PEAKDETSHUNTOPTMODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Low Power Mode Control"]
    #[inline(always)]
    pub fn lowpower(&self) -> LOWPOWER_R {
        LOWPOWER_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clamp HFXTAL_N Pin to Ground When HFXO Oscillator is Off"]
    #[inline(always)]
    pub fn xti2gnd(&self) -> XTI2GND_R {
        XTI2GND_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clamp HFXTAL_P Pin to Ground When HFXO Oscillator is Off"]
    #[inline(always)]
    pub fn xto2gnd(&self) -> XTO2GND_R {
        XTO2GND_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 24:26 - HFXO Low Frequency Timeout"]
    #[inline(always)]
    pub fn lftimeout(&self) -> LFTIMEOUT_R {
        LFTIMEOUT_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28 - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    pub fn autostartem0em1(&self) -> AUTOSTARTEM0EM1_R {
        AUTOSTARTEM0EM1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    pub fn autostartselem0em1(&self) -> AUTOSTARTSELEM0EM1_R {
        AUTOSTARTSELEM0EM1_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HFXO Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - HFXO Automatic Peak Detection and Shunt Current Optimization Mode"]
    #[inline(always)]
    pub fn peakdetshuntoptmode(&mut self) -> PEAKDETSHUNTOPTMODE_W {
        PEAKDETSHUNTOPTMODE_W::new(self)
    }
    #[doc = "Bit 8 - Low Power Mode Control"]
    #[inline(always)]
    pub fn lowpower(&mut self) -> LOWPOWER_W {
        LOWPOWER_W::new(self)
    }
    #[doc = "Bit 9 - Clamp HFXTAL_N Pin to Ground When HFXO Oscillator is Off"]
    #[inline(always)]
    pub fn xti2gnd(&mut self) -> XTI2GND_W {
        XTI2GND_W::new(self)
    }
    #[doc = "Bit 10 - Clamp HFXTAL_P Pin to Ground When HFXO Oscillator is Off"]
    #[inline(always)]
    pub fn xto2gnd(&mut self) -> XTO2GND_W {
        XTO2GND_W::new(self)
    }
    #[doc = "Bits 24:26 - HFXO Low Frequency Timeout"]
    #[inline(always)]
    pub fn lftimeout(&mut self) -> LFTIMEOUT_W {
        LFTIMEOUT_W::new(self)
    }
    #[doc = "Bit 28 - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    pub fn autostartem0em1(&mut self) -> AUTOSTARTEM0EM1_W {
        AUTOSTARTEM0EM1_W::new(self)
    }
    #[doc = "Bit 29 - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    pub fn autostartselem0em1(&mut self) -> AUTOSTARTSELEM0EM1_W {
        AUTOSTARTSELEM0EM1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HFXO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfxoctrl](index.html) module"]
pub struct HFXOCTRL_SPEC;
impl crate::RegisterSpec for HFXOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfxoctrl::R](R) reader structure"]
impl crate::Readable for HFXOCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfxoctrl::W](W) writer structure"]
impl crate::Writable for HFXOCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HFXOCTRL to value 0"]
impl crate::Resettable for HFXOCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
