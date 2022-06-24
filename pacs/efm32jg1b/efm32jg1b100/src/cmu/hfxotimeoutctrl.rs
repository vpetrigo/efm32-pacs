#[doc = "Register `HFXOTIMEOUTCTRL` reader"]
pub struct R(crate::R<HFXOTIMEOUTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFXOTIMEOUTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFXOTIMEOUTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFXOTIMEOUTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFXOTIMEOUTCTRL` writer"]
pub struct W(crate::W<HFXOTIMEOUTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFXOTIMEOUTCTRL_SPEC>;
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
impl From<crate::W<HFXOTIMEOUTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFXOTIMEOUTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Wait Duration in HFXO Startup Enable Wait State\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STARTUPTIMEOUT_A {
    #[doc = "0: Timeout period of 2 cycles"]
    _2CYCLES = 0,
    #[doc = "1: Timeout period of 4 cycles"]
    _4CYCLES = 1,
    #[doc = "2: Timeout period of 16 cycles"]
    _16CYCLES = 2,
    #[doc = "3: Timeout period of 32 cycles"]
    _32CYCLES = 3,
    #[doc = "4: Timeout period of 256 cycles"]
    _256CYCLES = 4,
    #[doc = "5: Timeout period of 1024 cycles"]
    _1KCYCLES = 5,
    #[doc = "6: Timeout period of 2048 cycles"]
    _2KCYCLES = 6,
    #[doc = "7: Timeout period of 4096 cycles"]
    _4KCYCLES = 7,
    #[doc = "8: Timeout period of 8192 cycles"]
    _8KCYCLES = 8,
    #[doc = "9: Timeout period of 16384 cycles"]
    _16KCYCLES = 9,
    #[doc = "10: Timeout period of 32768 cycles"]
    _32KCYCLES = 10,
}
impl From<STARTUPTIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: STARTUPTIMEOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STARTUPTIMEOUT` reader - Wait Duration in HFXO Startup Enable Wait State"]
pub type STARTUPTIMEOUT_R = crate::FieldReader<u8, STARTUPTIMEOUT_A>;
impl STARTUPTIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STARTUPTIMEOUT_A> {
        match self.bits {
            0 => Some(STARTUPTIMEOUT_A::_2CYCLES),
            1 => Some(STARTUPTIMEOUT_A::_4CYCLES),
            2 => Some(STARTUPTIMEOUT_A::_16CYCLES),
            3 => Some(STARTUPTIMEOUT_A::_32CYCLES),
            4 => Some(STARTUPTIMEOUT_A::_256CYCLES),
            5 => Some(STARTUPTIMEOUT_A::_1KCYCLES),
            6 => Some(STARTUPTIMEOUT_A::_2KCYCLES),
            7 => Some(STARTUPTIMEOUT_A::_4KCYCLES),
            8 => Some(STARTUPTIMEOUT_A::_8KCYCLES),
            9 => Some(STARTUPTIMEOUT_A::_16KCYCLES),
            10 => Some(STARTUPTIMEOUT_A::_32KCYCLES),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == STARTUPTIMEOUT_A::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == STARTUPTIMEOUT_A::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == STARTUPTIMEOUT_A::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == STARTUPTIMEOUT_A::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == STARTUPTIMEOUT_A::_256CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == STARTUPTIMEOUT_A::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_2KCYCLES`"]
    #[inline(always)]
    pub fn is_2kcycles(&self) -> bool {
        *self == STARTUPTIMEOUT_A::_2KCYCLES
    }
    #[doc = "Checks if the value of the field is `_4KCYCLES`"]
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == STARTUPTIMEOUT_A::_4KCYCLES
    }
    #[doc = "Checks if the value of the field is `_8KCYCLES`"]
    #[inline(always)]
    pub fn is_8kcycles(&self) -> bool {
        *self == STARTUPTIMEOUT_A::_8KCYCLES
    }
    #[doc = "Checks if the value of the field is `_16KCYCLES`"]
    #[inline(always)]
    pub fn is_16kcycles(&self) -> bool {
        *self == STARTUPTIMEOUT_A::_16KCYCLES
    }
    #[doc = "Checks if the value of the field is `_32KCYCLES`"]
    #[inline(always)]
    pub fn is_32kcycles(&self) -> bool {
        *self == STARTUPTIMEOUT_A::_32KCYCLES
    }
}
#[doc = "Field `STARTUPTIMEOUT` writer - Wait Duration in HFXO Startup Enable Wait State"]
pub type STARTUPTIMEOUT_W<'a> =
    crate::FieldWriter<'a, u32, HFXOTIMEOUTCTRL_SPEC, u8, STARTUPTIMEOUT_A, 4, 0>;
impl<'a> STARTUPTIMEOUT_W<'a> {
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUT_A::_2CYCLES)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUT_A::_4CYCLES)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUT_A::_16CYCLES)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUT_A::_32CYCLES)
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUT_A::_256CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUT_A::_1KCYCLES)
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline(always)]
    pub fn _2kcycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUT_A::_2KCYCLES)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUT_A::_4KCYCLES)
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline(always)]
    pub fn _8kcycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUT_A::_8KCYCLES)
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline(always)]
    pub fn _16kcycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUT_A::_16KCYCLES)
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline(always)]
    pub fn _32kcycles(self) -> &'a mut W {
        self.variant(STARTUPTIMEOUT_A::_32KCYCLES)
    }
}
#[doc = "Wait Duration in HFXO Startup Steady Wait State\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STEADYTIMEOUT_A {
    #[doc = "0: Timeout period of 2 cycles"]
    _2CYCLES = 0,
    #[doc = "1: Timeout period of 4 cycles"]
    _4CYCLES = 1,
    #[doc = "2: Timeout period of 16 cycles"]
    _16CYCLES = 2,
    #[doc = "3: Timeout period of 32 cycles"]
    _32CYCLES = 3,
    #[doc = "4: Timeout period of 256 cycles"]
    _256CYCLES = 4,
    #[doc = "5: Timeout period of 1024 cycles"]
    _1KCYCLES = 5,
    #[doc = "6: Timeout period of 2048 cycles"]
    _2KCYCLES = 6,
    #[doc = "7: Timeout period of 4096 cycles"]
    _4KCYCLES = 7,
    #[doc = "8: Timeout period of 8192 cycles"]
    _8KCYCLES = 8,
    #[doc = "9: Timeout period of 16384 cycles"]
    _16KCYCLES = 9,
    #[doc = "10: Timeout period of 32768 cycles"]
    _32KCYCLES = 10,
}
impl From<STEADYTIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: STEADYTIMEOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STEADYTIMEOUT` reader - Wait Duration in HFXO Startup Steady Wait State"]
pub type STEADYTIMEOUT_R = crate::FieldReader<u8, STEADYTIMEOUT_A>;
impl STEADYTIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STEADYTIMEOUT_A> {
        match self.bits {
            0 => Some(STEADYTIMEOUT_A::_2CYCLES),
            1 => Some(STEADYTIMEOUT_A::_4CYCLES),
            2 => Some(STEADYTIMEOUT_A::_16CYCLES),
            3 => Some(STEADYTIMEOUT_A::_32CYCLES),
            4 => Some(STEADYTIMEOUT_A::_256CYCLES),
            5 => Some(STEADYTIMEOUT_A::_1KCYCLES),
            6 => Some(STEADYTIMEOUT_A::_2KCYCLES),
            7 => Some(STEADYTIMEOUT_A::_4KCYCLES),
            8 => Some(STEADYTIMEOUT_A::_8KCYCLES),
            9 => Some(STEADYTIMEOUT_A::_16KCYCLES),
            10 => Some(STEADYTIMEOUT_A::_32KCYCLES),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == STEADYTIMEOUT_A::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == STEADYTIMEOUT_A::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == STEADYTIMEOUT_A::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == STEADYTIMEOUT_A::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == STEADYTIMEOUT_A::_256CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == STEADYTIMEOUT_A::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_2KCYCLES`"]
    #[inline(always)]
    pub fn is_2kcycles(&self) -> bool {
        *self == STEADYTIMEOUT_A::_2KCYCLES
    }
    #[doc = "Checks if the value of the field is `_4KCYCLES`"]
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == STEADYTIMEOUT_A::_4KCYCLES
    }
    #[doc = "Checks if the value of the field is `_8KCYCLES`"]
    #[inline(always)]
    pub fn is_8kcycles(&self) -> bool {
        *self == STEADYTIMEOUT_A::_8KCYCLES
    }
    #[doc = "Checks if the value of the field is `_16KCYCLES`"]
    #[inline(always)]
    pub fn is_16kcycles(&self) -> bool {
        *self == STEADYTIMEOUT_A::_16KCYCLES
    }
    #[doc = "Checks if the value of the field is `_32KCYCLES`"]
    #[inline(always)]
    pub fn is_32kcycles(&self) -> bool {
        *self == STEADYTIMEOUT_A::_32KCYCLES
    }
}
#[doc = "Field `STEADYTIMEOUT` writer - Wait Duration in HFXO Startup Steady Wait State"]
pub type STEADYTIMEOUT_W<'a> =
    crate::FieldWriter<'a, u32, HFXOTIMEOUTCTRL_SPEC, u8, STEADYTIMEOUT_A, 4, 4>;
impl<'a> STEADYTIMEOUT_W<'a> {
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUT_A::_2CYCLES)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUT_A::_4CYCLES)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUT_A::_16CYCLES)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUT_A::_32CYCLES)
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUT_A::_256CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUT_A::_1KCYCLES)
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline(always)]
    pub fn _2kcycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUT_A::_2KCYCLES)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUT_A::_4KCYCLES)
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline(always)]
    pub fn _8kcycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUT_A::_8KCYCLES)
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline(always)]
    pub fn _16kcycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUT_A::_16KCYCLES)
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline(always)]
    pub fn _32kcycles(self) -> &'a mut W {
        self.variant(STEADYTIMEOUT_A::_32KCYCLES)
    }
}
#[doc = "Field `RESERVED2` reader - Wait Duration in HFXO Warm Startup Steady Wait State"]
pub type RESERVED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED2` writer - Wait Duration in HFXO Warm Startup Steady Wait State"]
pub type RESERVED2_W<'a> = crate::FieldWriter<'a, u32, HFXOTIMEOUTCTRL_SPEC, u8, u8, 4, 8>;
#[doc = "Wait Duration in HFXO Peak Detection Wait State\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PEAKDETTIMEOUT_A {
    #[doc = "0: Timeout period of 2 cycles"]
    _2CYCLES = 0,
    #[doc = "1: Timeout period of 4 cycles"]
    _4CYCLES = 1,
    #[doc = "2: Timeout period of 16 cycles"]
    _16CYCLES = 2,
    #[doc = "3: Timeout period of 32 cycles"]
    _32CYCLES = 3,
    #[doc = "4: Timeout period of 256 cycles"]
    _256CYCLES = 4,
    #[doc = "5: Timeout period of 1024 cycles"]
    _1KCYCLES = 5,
    #[doc = "6: Timeout period of 2048 cycles"]
    _2KCYCLES = 6,
    #[doc = "7: Timeout period of 4096 cycles"]
    _4KCYCLES = 7,
    #[doc = "8: Timeout period of 8192 cycles"]
    _8KCYCLES = 8,
    #[doc = "9: Timeout period of 16384 cycles"]
    _16KCYCLES = 9,
    #[doc = "10: Timeout period of 32768 cycles"]
    _32KCYCLES = 10,
}
impl From<PEAKDETTIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: PEAKDETTIMEOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PEAKDETTIMEOUT` reader - Wait Duration in HFXO Peak Detection Wait State"]
pub type PEAKDETTIMEOUT_R = crate::FieldReader<u8, PEAKDETTIMEOUT_A>;
impl PEAKDETTIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PEAKDETTIMEOUT_A> {
        match self.bits {
            0 => Some(PEAKDETTIMEOUT_A::_2CYCLES),
            1 => Some(PEAKDETTIMEOUT_A::_4CYCLES),
            2 => Some(PEAKDETTIMEOUT_A::_16CYCLES),
            3 => Some(PEAKDETTIMEOUT_A::_32CYCLES),
            4 => Some(PEAKDETTIMEOUT_A::_256CYCLES),
            5 => Some(PEAKDETTIMEOUT_A::_1KCYCLES),
            6 => Some(PEAKDETTIMEOUT_A::_2KCYCLES),
            7 => Some(PEAKDETTIMEOUT_A::_4KCYCLES),
            8 => Some(PEAKDETTIMEOUT_A::_8KCYCLES),
            9 => Some(PEAKDETTIMEOUT_A::_16KCYCLES),
            10 => Some(PEAKDETTIMEOUT_A::_32KCYCLES),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == PEAKDETTIMEOUT_A::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == PEAKDETTIMEOUT_A::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == PEAKDETTIMEOUT_A::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == PEAKDETTIMEOUT_A::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == PEAKDETTIMEOUT_A::_256CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUT_A::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_2KCYCLES`"]
    #[inline(always)]
    pub fn is_2kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUT_A::_2KCYCLES
    }
    #[doc = "Checks if the value of the field is `_4KCYCLES`"]
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUT_A::_4KCYCLES
    }
    #[doc = "Checks if the value of the field is `_8KCYCLES`"]
    #[inline(always)]
    pub fn is_8kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUT_A::_8KCYCLES
    }
    #[doc = "Checks if the value of the field is `_16KCYCLES`"]
    #[inline(always)]
    pub fn is_16kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUT_A::_16KCYCLES
    }
    #[doc = "Checks if the value of the field is `_32KCYCLES`"]
    #[inline(always)]
    pub fn is_32kcycles(&self) -> bool {
        *self == PEAKDETTIMEOUT_A::_32KCYCLES
    }
}
#[doc = "Field `PEAKDETTIMEOUT` writer - Wait Duration in HFXO Peak Detection Wait State"]
pub type PEAKDETTIMEOUT_W<'a> =
    crate::FieldWriter<'a, u32, HFXOTIMEOUTCTRL_SPEC, u8, PEAKDETTIMEOUT_A, 4, 12>;
impl<'a> PEAKDETTIMEOUT_W<'a> {
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUT_A::_2CYCLES)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUT_A::_4CYCLES)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUT_A::_16CYCLES)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUT_A::_32CYCLES)
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUT_A::_256CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUT_A::_1KCYCLES)
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline(always)]
    pub fn _2kcycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUT_A::_2KCYCLES)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUT_A::_4KCYCLES)
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline(always)]
    pub fn _8kcycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUT_A::_8KCYCLES)
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline(always)]
    pub fn _16kcycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUT_A::_16KCYCLES)
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline(always)]
    pub fn _32kcycles(self) -> &'a mut W {
        self.variant(PEAKDETTIMEOUT_A::_32KCYCLES)
    }
}
#[doc = "Wait Duration in HFXO Shunt Current Optimization Wait State\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SHUNTOPTTIMEOUT_A {
    #[doc = "0: Timeout period of 2 cycles"]
    _2CYCLES = 0,
    #[doc = "1: Timeout period of 4 cycles"]
    _4CYCLES = 1,
    #[doc = "2: Timeout period of 16 cycles"]
    _16CYCLES = 2,
    #[doc = "3: Timeout period of 32 cycles"]
    _32CYCLES = 3,
    #[doc = "4: Timeout period of 256 cycles"]
    _256CYCLES = 4,
    #[doc = "5: Timeout period of 1024 cycles"]
    _1KCYCLES = 5,
    #[doc = "6: Timeout period of 2048 cycles"]
    _2KCYCLES = 6,
    #[doc = "7: Timeout period of 4096 cycles"]
    _4KCYCLES = 7,
    #[doc = "8: Timeout period of 8192 cycles"]
    _8KCYCLES = 8,
    #[doc = "9: Timeout period of 16384 cycles"]
    _16KCYCLES = 9,
    #[doc = "10: Timeout period of 32768 cycles"]
    _32KCYCLES = 10,
}
impl From<SHUNTOPTTIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: SHUNTOPTTIMEOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SHUNTOPTTIMEOUT` reader - Wait Duration in HFXO Shunt Current Optimization Wait State"]
pub type SHUNTOPTTIMEOUT_R = crate::FieldReader<u8, SHUNTOPTTIMEOUT_A>;
impl SHUNTOPTTIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SHUNTOPTTIMEOUT_A> {
        match self.bits {
            0 => Some(SHUNTOPTTIMEOUT_A::_2CYCLES),
            1 => Some(SHUNTOPTTIMEOUT_A::_4CYCLES),
            2 => Some(SHUNTOPTTIMEOUT_A::_16CYCLES),
            3 => Some(SHUNTOPTTIMEOUT_A::_32CYCLES),
            4 => Some(SHUNTOPTTIMEOUT_A::_256CYCLES),
            5 => Some(SHUNTOPTTIMEOUT_A::_1KCYCLES),
            6 => Some(SHUNTOPTTIMEOUT_A::_2KCYCLES),
            7 => Some(SHUNTOPTTIMEOUT_A::_4KCYCLES),
            8 => Some(SHUNTOPTTIMEOUT_A::_8KCYCLES),
            9 => Some(SHUNTOPTTIMEOUT_A::_16KCYCLES),
            10 => Some(SHUNTOPTTIMEOUT_A::_32KCYCLES),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT_A::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT_A::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT_A::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT_A::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT_A::_256CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT_A::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_2KCYCLES`"]
    #[inline(always)]
    pub fn is_2kcycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT_A::_2KCYCLES
    }
    #[doc = "Checks if the value of the field is `_4KCYCLES`"]
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT_A::_4KCYCLES
    }
    #[doc = "Checks if the value of the field is `_8KCYCLES`"]
    #[inline(always)]
    pub fn is_8kcycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT_A::_8KCYCLES
    }
    #[doc = "Checks if the value of the field is `_16KCYCLES`"]
    #[inline(always)]
    pub fn is_16kcycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT_A::_16KCYCLES
    }
    #[doc = "Checks if the value of the field is `_32KCYCLES`"]
    #[inline(always)]
    pub fn is_32kcycles(&self) -> bool {
        *self == SHUNTOPTTIMEOUT_A::_32KCYCLES
    }
}
#[doc = "Field `SHUNTOPTTIMEOUT` writer - Wait Duration in HFXO Shunt Current Optimization Wait State"]
pub type SHUNTOPTTIMEOUT_W<'a> =
    crate::FieldWriter<'a, u32, HFXOTIMEOUTCTRL_SPEC, u8, SHUNTOPTTIMEOUT_A, 4, 16>;
impl<'a> SHUNTOPTTIMEOUT_W<'a> {
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUT_A::_2CYCLES)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUT_A::_4CYCLES)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUT_A::_16CYCLES)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUT_A::_32CYCLES)
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUT_A::_256CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUT_A::_1KCYCLES)
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline(always)]
    pub fn _2kcycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUT_A::_2KCYCLES)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUT_A::_4KCYCLES)
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline(always)]
    pub fn _8kcycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUT_A::_8KCYCLES)
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline(always)]
    pub fn _16kcycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUT_A::_16KCYCLES)
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline(always)]
    pub fn _32kcycles(self) -> &'a mut W {
        self.variant(SHUNTOPTTIMEOUT_A::_32KCYCLES)
    }
}
impl R {
    #[doc = "Bits 0:3 - Wait Duration in HFXO Startup Enable Wait State"]
    #[inline(always)]
    pub fn startuptimeout(&self) -> STARTUPTIMEOUT_R {
        STARTUPTIMEOUT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Wait Duration in HFXO Startup Steady Wait State"]
    #[inline(always)]
    pub fn steadytimeout(&self) -> STEADYTIMEOUT_R {
        STEADYTIMEOUT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Wait Duration in HFXO Warm Startup Steady Wait State"]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Wait Duration in HFXO Peak Detection Wait State"]
    #[inline(always)]
    pub fn peakdettimeout(&self) -> PEAKDETTIMEOUT_R {
        PEAKDETTIMEOUT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Wait Duration in HFXO Shunt Current Optimization Wait State"]
    #[inline(always)]
    pub fn shuntopttimeout(&self) -> SHUNTOPTTIMEOUT_R {
        SHUNTOPTTIMEOUT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Wait Duration in HFXO Startup Enable Wait State"]
    #[inline(always)]
    pub fn startuptimeout(&mut self) -> STARTUPTIMEOUT_W {
        STARTUPTIMEOUT_W::new(self)
    }
    #[doc = "Bits 4:7 - Wait Duration in HFXO Startup Steady Wait State"]
    #[inline(always)]
    pub fn steadytimeout(&mut self) -> STEADYTIMEOUT_W {
        STEADYTIMEOUT_W::new(self)
    }
    #[doc = "Bits 8:11 - Wait Duration in HFXO Warm Startup Steady Wait State"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W::new(self)
    }
    #[doc = "Bits 12:15 - Wait Duration in HFXO Peak Detection Wait State"]
    #[inline(always)]
    pub fn peakdettimeout(&mut self) -> PEAKDETTIMEOUT_W {
        PEAKDETTIMEOUT_W::new(self)
    }
    #[doc = "Bits 16:19 - Wait Duration in HFXO Shunt Current Optimization Wait State"]
    #[inline(always)]
    pub fn shuntopttimeout(&mut self) -> SHUNTOPTTIMEOUT_W {
        SHUNTOPTTIMEOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HFXO Timeout Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfxotimeoutctrl](index.html) module"]
pub struct HFXOTIMEOUTCTRL_SPEC;
impl crate::RegisterSpec for HFXOTIMEOUTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfxotimeoutctrl::R](R) reader structure"]
impl crate::Readable for HFXOTIMEOUTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfxotimeoutctrl::W](W) writer structure"]
impl crate::Writable for HFXOTIMEOUTCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HFXOTIMEOUTCTRL to value 0x0002_6667"]
impl crate::Resettable for HFXOTIMEOUTCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0002_6667
    }
}
