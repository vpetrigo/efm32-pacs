#[doc = "Register `IDLECONF` reader"]
pub struct R(crate::R<IDLECONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDLECONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDLECONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDLECONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDLECONF` writer"]
pub struct W(crate::W<IDLECONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDLECONF_SPEC>;
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
impl From<crate::W<IDLECONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDLECONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHIDLE0_A {
    #[doc = "0: Channel output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: Channel output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: Channel output is low in idle phase"]
    LOW = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    DAC = 3,
}
impl From<CHIDLE0_A> for u8 {
    #[inline(always)]
    fn from(variant: CHIDLE0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHIDLE0` reader - Channel IDLE configuration"]
pub type CHIDLE0_R = crate::FieldReader<u8, CHIDLE0_A>;
impl CHIDLE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHIDLE0_A {
        match self.bits {
            0 => CHIDLE0_A::DISABLE,
            1 => CHIDLE0_A::HIGH,
            2 => CHIDLE0_A::LOW,
            3 => CHIDLE0_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHIDLE0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CHIDLE0_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CHIDLE0_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CHIDLE0_A::DAC
    }
}
#[doc = "Field `CHIDLE0` writer - Channel IDLE configuration"]
pub type CHIDLE0_W<'a> = crate::FieldWriterSafe<'a, u32, IDLECONF_SPEC, u8, CHIDLE0_A, 2, 0>;
impl<'a> CHIDLE0_W<'a> {
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHIDLE0_A::DISABLE)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CHIDLE0_A::HIGH)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CHIDLE0_A::LOW)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CHIDLE0_A::DAC)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHIDLE1_A {
    #[doc = "0: Channel output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: Channel output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: Channel output is low in idle phase"]
    LOW = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    DAC = 3,
}
impl From<CHIDLE1_A> for u8 {
    #[inline(always)]
    fn from(variant: CHIDLE1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHIDLE1` reader - Channel IDLE configuration"]
pub type CHIDLE1_R = crate::FieldReader<u8, CHIDLE1_A>;
impl CHIDLE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHIDLE1_A {
        match self.bits {
            0 => CHIDLE1_A::DISABLE,
            1 => CHIDLE1_A::HIGH,
            2 => CHIDLE1_A::LOW,
            3 => CHIDLE1_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHIDLE1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CHIDLE1_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CHIDLE1_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CHIDLE1_A::DAC
    }
}
#[doc = "Field `CHIDLE1` writer - Channel IDLE configuration"]
pub type CHIDLE1_W<'a> = crate::FieldWriterSafe<'a, u32, IDLECONF_SPEC, u8, CHIDLE1_A, 2, 2>;
impl<'a> CHIDLE1_W<'a> {
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHIDLE1_A::DISABLE)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CHIDLE1_A::HIGH)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CHIDLE1_A::LOW)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CHIDLE1_A::DAC)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHIDLE2_A {
    #[doc = "0: Channel output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: Channel output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: Channel output is low in idle phase"]
    LOW = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    DAC = 3,
}
impl From<CHIDLE2_A> for u8 {
    #[inline(always)]
    fn from(variant: CHIDLE2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHIDLE2` reader - Channel IDLE configuration"]
pub type CHIDLE2_R = crate::FieldReader<u8, CHIDLE2_A>;
impl CHIDLE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHIDLE2_A {
        match self.bits {
            0 => CHIDLE2_A::DISABLE,
            1 => CHIDLE2_A::HIGH,
            2 => CHIDLE2_A::LOW,
            3 => CHIDLE2_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHIDLE2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CHIDLE2_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CHIDLE2_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CHIDLE2_A::DAC
    }
}
#[doc = "Field `CHIDLE2` writer - Channel IDLE configuration"]
pub type CHIDLE2_W<'a> = crate::FieldWriterSafe<'a, u32, IDLECONF_SPEC, u8, CHIDLE2_A, 2, 4>;
impl<'a> CHIDLE2_W<'a> {
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHIDLE2_A::DISABLE)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CHIDLE2_A::HIGH)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CHIDLE2_A::LOW)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CHIDLE2_A::DAC)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHIDLE3_A {
    #[doc = "0: Channel output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: Channel output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: Channel output is low in idle phase"]
    LOW = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    DAC = 3,
}
impl From<CHIDLE3_A> for u8 {
    #[inline(always)]
    fn from(variant: CHIDLE3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHIDLE3` reader - Channel IDLE configuration"]
pub type CHIDLE3_R = crate::FieldReader<u8, CHIDLE3_A>;
impl CHIDLE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHIDLE3_A {
        match self.bits {
            0 => CHIDLE3_A::DISABLE,
            1 => CHIDLE3_A::HIGH,
            2 => CHIDLE3_A::LOW,
            3 => CHIDLE3_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHIDLE3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CHIDLE3_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CHIDLE3_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CHIDLE3_A::DAC
    }
}
#[doc = "Field `CHIDLE3` writer - Channel IDLE configuration"]
pub type CHIDLE3_W<'a> = crate::FieldWriterSafe<'a, u32, IDLECONF_SPEC, u8, CHIDLE3_A, 2, 6>;
impl<'a> CHIDLE3_W<'a> {
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHIDLE3_A::DISABLE)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CHIDLE3_A::HIGH)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CHIDLE3_A::LOW)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CHIDLE3_A::DAC)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHIDLE4_A {
    #[doc = "0: Channel output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: Channel output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: Channel output is low in idle phase"]
    LOW = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    DAC = 3,
}
impl From<CHIDLE4_A> for u8 {
    #[inline(always)]
    fn from(variant: CHIDLE4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHIDLE4` reader - Channel IDLE configuration"]
pub type CHIDLE4_R = crate::FieldReader<u8, CHIDLE4_A>;
impl CHIDLE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHIDLE4_A {
        match self.bits {
            0 => CHIDLE4_A::DISABLE,
            1 => CHIDLE4_A::HIGH,
            2 => CHIDLE4_A::LOW,
            3 => CHIDLE4_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHIDLE4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CHIDLE4_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CHIDLE4_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CHIDLE4_A::DAC
    }
}
#[doc = "Field `CHIDLE4` writer - Channel IDLE configuration"]
pub type CHIDLE4_W<'a> = crate::FieldWriterSafe<'a, u32, IDLECONF_SPEC, u8, CHIDLE4_A, 2, 8>;
impl<'a> CHIDLE4_W<'a> {
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHIDLE4_A::DISABLE)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CHIDLE4_A::HIGH)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CHIDLE4_A::LOW)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CHIDLE4_A::DAC)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHIDLE5_A {
    #[doc = "0: Channel output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: Channel output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: Channel output is low in idle phase"]
    LOW = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    DAC = 3,
}
impl From<CHIDLE5_A> for u8 {
    #[inline(always)]
    fn from(variant: CHIDLE5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHIDLE5` reader - Channel IDLE configuration"]
pub type CHIDLE5_R = crate::FieldReader<u8, CHIDLE5_A>;
impl CHIDLE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHIDLE5_A {
        match self.bits {
            0 => CHIDLE5_A::DISABLE,
            1 => CHIDLE5_A::HIGH,
            2 => CHIDLE5_A::LOW,
            3 => CHIDLE5_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHIDLE5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CHIDLE5_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CHIDLE5_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CHIDLE5_A::DAC
    }
}
#[doc = "Field `CHIDLE5` writer - Channel IDLE configuration"]
pub type CHIDLE5_W<'a> = crate::FieldWriterSafe<'a, u32, IDLECONF_SPEC, u8, CHIDLE5_A, 2, 10>;
impl<'a> CHIDLE5_W<'a> {
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHIDLE5_A::DISABLE)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CHIDLE5_A::HIGH)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CHIDLE5_A::LOW)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CHIDLE5_A::DAC)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHIDLE6_A {
    #[doc = "0: Channel output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: Channel output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: Channel output is low in idle phase"]
    LOW = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    DAC = 3,
}
impl From<CHIDLE6_A> for u8 {
    #[inline(always)]
    fn from(variant: CHIDLE6_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHIDLE6` reader - Channel IDLE configuration"]
pub type CHIDLE6_R = crate::FieldReader<u8, CHIDLE6_A>;
impl CHIDLE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHIDLE6_A {
        match self.bits {
            0 => CHIDLE6_A::DISABLE,
            1 => CHIDLE6_A::HIGH,
            2 => CHIDLE6_A::LOW,
            3 => CHIDLE6_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHIDLE6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CHIDLE6_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CHIDLE6_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CHIDLE6_A::DAC
    }
}
#[doc = "Field `CHIDLE6` writer - Channel IDLE configuration"]
pub type CHIDLE6_W<'a> = crate::FieldWriterSafe<'a, u32, IDLECONF_SPEC, u8, CHIDLE6_A, 2, 12>;
impl<'a> CHIDLE6_W<'a> {
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHIDLE6_A::DISABLE)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CHIDLE6_A::HIGH)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CHIDLE6_A::LOW)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CHIDLE6_A::DAC)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHIDLE7_A {
    #[doc = "0: Channel output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: Channel output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: Channel output is low in idle phase"]
    LOW = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    DAC = 3,
}
impl From<CHIDLE7_A> for u8 {
    #[inline(always)]
    fn from(variant: CHIDLE7_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHIDLE7` reader - Channel IDLE configuration"]
pub type CHIDLE7_R = crate::FieldReader<u8, CHIDLE7_A>;
impl CHIDLE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHIDLE7_A {
        match self.bits {
            0 => CHIDLE7_A::DISABLE,
            1 => CHIDLE7_A::HIGH,
            2 => CHIDLE7_A::LOW,
            3 => CHIDLE7_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHIDLE7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CHIDLE7_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CHIDLE7_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CHIDLE7_A::DAC
    }
}
#[doc = "Field `CHIDLE7` writer - Channel IDLE configuration"]
pub type CHIDLE7_W<'a> = crate::FieldWriterSafe<'a, u32, IDLECONF_SPEC, u8, CHIDLE7_A, 2, 14>;
impl<'a> CHIDLE7_W<'a> {
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHIDLE7_A::DISABLE)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CHIDLE7_A::HIGH)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CHIDLE7_A::LOW)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CHIDLE7_A::DAC)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHIDLE8_A {
    #[doc = "0: Channel output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: Channel output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: Channel output is low in idle phase"]
    LOW = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    DAC = 3,
}
impl From<CHIDLE8_A> for u8 {
    #[inline(always)]
    fn from(variant: CHIDLE8_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHIDLE8` reader - Channel IDLE configuration"]
pub type CHIDLE8_R = crate::FieldReader<u8, CHIDLE8_A>;
impl CHIDLE8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHIDLE8_A {
        match self.bits {
            0 => CHIDLE8_A::DISABLE,
            1 => CHIDLE8_A::HIGH,
            2 => CHIDLE8_A::LOW,
            3 => CHIDLE8_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHIDLE8_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CHIDLE8_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CHIDLE8_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CHIDLE8_A::DAC
    }
}
#[doc = "Field `CHIDLE8` writer - Channel IDLE configuration"]
pub type CHIDLE8_W<'a> = crate::FieldWriterSafe<'a, u32, IDLECONF_SPEC, u8, CHIDLE8_A, 2, 16>;
impl<'a> CHIDLE8_W<'a> {
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHIDLE8_A::DISABLE)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CHIDLE8_A::HIGH)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CHIDLE8_A::LOW)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CHIDLE8_A::DAC)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHIDLE9_A {
    #[doc = "0: Channel output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: Channel output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: Channel output is low in idle phase"]
    LOW = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    DAC = 3,
}
impl From<CHIDLE9_A> for u8 {
    #[inline(always)]
    fn from(variant: CHIDLE9_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHIDLE9` reader - Channel IDLE configuration"]
pub type CHIDLE9_R = crate::FieldReader<u8, CHIDLE9_A>;
impl CHIDLE9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHIDLE9_A {
        match self.bits {
            0 => CHIDLE9_A::DISABLE,
            1 => CHIDLE9_A::HIGH,
            2 => CHIDLE9_A::LOW,
            3 => CHIDLE9_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHIDLE9_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CHIDLE9_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CHIDLE9_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CHIDLE9_A::DAC
    }
}
#[doc = "Field `CHIDLE9` writer - Channel IDLE configuration"]
pub type CHIDLE9_W<'a> = crate::FieldWriterSafe<'a, u32, IDLECONF_SPEC, u8, CHIDLE9_A, 2, 18>;
impl<'a> CHIDLE9_W<'a> {
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHIDLE9_A::DISABLE)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CHIDLE9_A::HIGH)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CHIDLE9_A::LOW)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CHIDLE9_A::DAC)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHIDLE10_A {
    #[doc = "0: Channel output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: Channel output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: Channel output is low in idle phase"]
    LOW = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    DAC = 3,
}
impl From<CHIDLE10_A> for u8 {
    #[inline(always)]
    fn from(variant: CHIDLE10_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHIDLE10` reader - Channel IDLE configuration"]
pub type CHIDLE10_R = crate::FieldReader<u8, CHIDLE10_A>;
impl CHIDLE10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHIDLE10_A {
        match self.bits {
            0 => CHIDLE10_A::DISABLE,
            1 => CHIDLE10_A::HIGH,
            2 => CHIDLE10_A::LOW,
            3 => CHIDLE10_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHIDLE10_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CHIDLE10_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CHIDLE10_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CHIDLE10_A::DAC
    }
}
#[doc = "Field `CHIDLE10` writer - Channel IDLE configuration"]
pub type CHIDLE10_W<'a> = crate::FieldWriterSafe<'a, u32, IDLECONF_SPEC, u8, CHIDLE10_A, 2, 20>;
impl<'a> CHIDLE10_W<'a> {
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHIDLE10_A::DISABLE)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CHIDLE10_A::HIGH)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CHIDLE10_A::LOW)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CHIDLE10_A::DAC)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHIDLE11_A {
    #[doc = "0: Channel output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: Channel output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: Channel output is low in idle phase"]
    LOW = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    DAC = 3,
}
impl From<CHIDLE11_A> for u8 {
    #[inline(always)]
    fn from(variant: CHIDLE11_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHIDLE11` reader - Channel IDLE configuration"]
pub type CHIDLE11_R = crate::FieldReader<u8, CHIDLE11_A>;
impl CHIDLE11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHIDLE11_A {
        match self.bits {
            0 => CHIDLE11_A::DISABLE,
            1 => CHIDLE11_A::HIGH,
            2 => CHIDLE11_A::LOW,
            3 => CHIDLE11_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHIDLE11_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CHIDLE11_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CHIDLE11_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CHIDLE11_A::DAC
    }
}
#[doc = "Field `CHIDLE11` writer - Channel IDLE configuration"]
pub type CHIDLE11_W<'a> = crate::FieldWriterSafe<'a, u32, IDLECONF_SPEC, u8, CHIDLE11_A, 2, 22>;
impl<'a> CHIDLE11_W<'a> {
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHIDLE11_A::DISABLE)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CHIDLE11_A::HIGH)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CHIDLE11_A::LOW)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CHIDLE11_A::DAC)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHIDLE12_A {
    #[doc = "0: Channel output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: Channel output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: Channel output is low in idle phase"]
    LOW = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    DAC = 3,
}
impl From<CHIDLE12_A> for u8 {
    #[inline(always)]
    fn from(variant: CHIDLE12_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHIDLE12` reader - Channel IDLE configuration"]
pub type CHIDLE12_R = crate::FieldReader<u8, CHIDLE12_A>;
impl CHIDLE12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHIDLE12_A {
        match self.bits {
            0 => CHIDLE12_A::DISABLE,
            1 => CHIDLE12_A::HIGH,
            2 => CHIDLE12_A::LOW,
            3 => CHIDLE12_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHIDLE12_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CHIDLE12_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CHIDLE12_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CHIDLE12_A::DAC
    }
}
#[doc = "Field `CHIDLE12` writer - Channel IDLE configuration"]
pub type CHIDLE12_W<'a> = crate::FieldWriterSafe<'a, u32, IDLECONF_SPEC, u8, CHIDLE12_A, 2, 24>;
impl<'a> CHIDLE12_W<'a> {
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHIDLE12_A::DISABLE)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CHIDLE12_A::HIGH)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CHIDLE12_A::LOW)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CHIDLE12_A::DAC)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHIDLE13_A {
    #[doc = "0: Channel output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: Channel output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: Channel output is low in idle phase"]
    LOW = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    DAC = 3,
}
impl From<CHIDLE13_A> for u8 {
    #[inline(always)]
    fn from(variant: CHIDLE13_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHIDLE13` reader - Channel IDLE configuration"]
pub type CHIDLE13_R = crate::FieldReader<u8, CHIDLE13_A>;
impl CHIDLE13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHIDLE13_A {
        match self.bits {
            0 => CHIDLE13_A::DISABLE,
            1 => CHIDLE13_A::HIGH,
            2 => CHIDLE13_A::LOW,
            3 => CHIDLE13_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHIDLE13_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CHIDLE13_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CHIDLE13_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CHIDLE13_A::DAC
    }
}
#[doc = "Field `CHIDLE13` writer - Channel IDLE configuration"]
pub type CHIDLE13_W<'a> = crate::FieldWriterSafe<'a, u32, IDLECONF_SPEC, u8, CHIDLE13_A, 2, 26>;
impl<'a> CHIDLE13_W<'a> {
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHIDLE13_A::DISABLE)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CHIDLE13_A::HIGH)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CHIDLE13_A::LOW)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CHIDLE13_A::DAC)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHIDLE14_A {
    #[doc = "0: Channel output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: Channel output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: Channel output is low in idle phase"]
    LOW = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    DAC = 3,
}
impl From<CHIDLE14_A> for u8 {
    #[inline(always)]
    fn from(variant: CHIDLE14_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHIDLE14` reader - Channel IDLE configuration"]
pub type CHIDLE14_R = crate::FieldReader<u8, CHIDLE14_A>;
impl CHIDLE14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHIDLE14_A {
        match self.bits {
            0 => CHIDLE14_A::DISABLE,
            1 => CHIDLE14_A::HIGH,
            2 => CHIDLE14_A::LOW,
            3 => CHIDLE14_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHIDLE14_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CHIDLE14_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CHIDLE14_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CHIDLE14_A::DAC
    }
}
#[doc = "Field `CHIDLE14` writer - Channel IDLE configuration"]
pub type CHIDLE14_W<'a> = crate::FieldWriterSafe<'a, u32, IDLECONF_SPEC, u8, CHIDLE14_A, 2, 28>;
impl<'a> CHIDLE14_W<'a> {
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHIDLE14_A::DISABLE)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CHIDLE14_A::HIGH)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CHIDLE14_A::LOW)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CHIDLE14_A::DAC)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHIDLE15_A {
    #[doc = "0: Channel output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: Channel output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: Channel output is low in idle phase"]
    LOW = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    DAC = 3,
}
impl From<CHIDLE15_A> for u8 {
    #[inline(always)]
    fn from(variant: CHIDLE15_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHIDLE15` reader - Channel IDLE configuration"]
pub type CHIDLE15_R = crate::FieldReader<u8, CHIDLE15_A>;
impl CHIDLE15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHIDLE15_A {
        match self.bits {
            0 => CHIDLE15_A::DISABLE,
            1 => CHIDLE15_A::HIGH,
            2 => CHIDLE15_A::LOW,
            3 => CHIDLE15_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHIDLE15_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CHIDLE15_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CHIDLE15_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CHIDLE15_A::DAC
    }
}
#[doc = "Field `CHIDLE15` writer - Channel IDLE configuration"]
pub type CHIDLE15_W<'a> = crate::FieldWriterSafe<'a, u32, IDLECONF_SPEC, u8, CHIDLE15_A, 2, 30>;
impl<'a> CHIDLE15_W<'a> {
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHIDLE15_A::DISABLE)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CHIDLE15_A::HIGH)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CHIDLE15_A::LOW)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CHIDLE15_A::DAC)
    }
}
impl R {
    #[doc = "Bits 0:1 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle0(&self) -> CHIDLE0_R {
        CHIDLE0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle1(&self) -> CHIDLE1_R {
        CHIDLE1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle2(&self) -> CHIDLE2_R {
        CHIDLE2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle3(&self) -> CHIDLE3_R {
        CHIDLE3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle4(&self) -> CHIDLE4_R {
        CHIDLE4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle5(&self) -> CHIDLE5_R {
        CHIDLE5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle6(&self) -> CHIDLE6_R {
        CHIDLE6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle7(&self) -> CHIDLE7_R {
        CHIDLE7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle8(&self) -> CHIDLE8_R {
        CHIDLE8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle9(&self) -> CHIDLE9_R {
        CHIDLE9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle10(&self) -> CHIDLE10_R {
        CHIDLE10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle11(&self) -> CHIDLE11_R {
        CHIDLE11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle12(&self) -> CHIDLE12_R {
        CHIDLE12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle13(&self) -> CHIDLE13_R {
        CHIDLE13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle14(&self) -> CHIDLE14_R {
        CHIDLE14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle15(&self) -> CHIDLE15_R {
        CHIDLE15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle0(&mut self) -> CHIDLE0_W {
        CHIDLE0_W::new(self)
    }
    #[doc = "Bits 2:3 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle1(&mut self) -> CHIDLE1_W {
        CHIDLE1_W::new(self)
    }
    #[doc = "Bits 4:5 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle2(&mut self) -> CHIDLE2_W {
        CHIDLE2_W::new(self)
    }
    #[doc = "Bits 6:7 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle3(&mut self) -> CHIDLE3_W {
        CHIDLE3_W::new(self)
    }
    #[doc = "Bits 8:9 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle4(&mut self) -> CHIDLE4_W {
        CHIDLE4_W::new(self)
    }
    #[doc = "Bits 10:11 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle5(&mut self) -> CHIDLE5_W {
        CHIDLE5_W::new(self)
    }
    #[doc = "Bits 12:13 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle6(&mut self) -> CHIDLE6_W {
        CHIDLE6_W::new(self)
    }
    #[doc = "Bits 14:15 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle7(&mut self) -> CHIDLE7_W {
        CHIDLE7_W::new(self)
    }
    #[doc = "Bits 16:17 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle8(&mut self) -> CHIDLE8_W {
        CHIDLE8_W::new(self)
    }
    #[doc = "Bits 18:19 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle9(&mut self) -> CHIDLE9_W {
        CHIDLE9_W::new(self)
    }
    #[doc = "Bits 20:21 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle10(&mut self) -> CHIDLE10_W {
        CHIDLE10_W::new(self)
    }
    #[doc = "Bits 22:23 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle11(&mut self) -> CHIDLE11_W {
        CHIDLE11_W::new(self)
    }
    #[doc = "Bits 24:25 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle12(&mut self) -> CHIDLE12_W {
        CHIDLE12_W::new(self)
    }
    #[doc = "Bits 26:27 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle13(&mut self) -> CHIDLE13_W {
        CHIDLE13_W::new(self)
    }
    #[doc = "Bits 28:29 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle14(&mut self) -> CHIDLE14_W {
        CHIDLE14_W::new(self)
    }
    #[doc = "Bits 30:31 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle15(&mut self) -> CHIDLE15_W {
        CHIDLE15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Idle phase configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idleconf](index.html) module"]
pub struct IDLECONF_SPEC;
impl crate::RegisterSpec for IDLECONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idleconf::R](R) reader structure"]
impl crate::Readable for IDLECONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idleconf::W](W) writer structure"]
impl crate::Writable for IDLECONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IDLECONF to value 0"]
impl crate::Resettable for IDLECONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
