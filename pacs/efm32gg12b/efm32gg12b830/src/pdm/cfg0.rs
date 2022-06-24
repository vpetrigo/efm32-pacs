#[doc = "Register `CFG0` reader"]
pub struct R(crate::R<CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG0` writer"]
pub struct W(crate::W<CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG0_SPEC>;
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
impl From<crate::W<CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Filter order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FORDER_A {
    #[doc = "0: Second order filter."]
    SECOND = 0,
    #[doc = "1: Third order filter."]
    THIRD = 1,
    #[doc = "2: Fourth order filter."]
    FOURTH = 2,
    #[doc = "3: Fifth order filter."]
    FIFTH = 3,
}
impl From<FORDER_A> for u8 {
    #[inline(always)]
    fn from(variant: FORDER_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FORDER` reader - Filter order"]
pub type FORDER_R = crate::FieldReader<u8, FORDER_A>;
impl FORDER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORDER_A {
        match self.bits {
            0 => FORDER_A::SECOND,
            1 => FORDER_A::THIRD,
            2 => FORDER_A::FOURTH,
            3 => FORDER_A::FIFTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SECOND`"]
    #[inline(always)]
    pub fn is_second(&self) -> bool {
        *self == FORDER_A::SECOND
    }
    #[doc = "Checks if the value of the field is `THIRD`"]
    #[inline(always)]
    pub fn is_third(&self) -> bool {
        *self == FORDER_A::THIRD
    }
    #[doc = "Checks if the value of the field is `FOURTH`"]
    #[inline(always)]
    pub fn is_fourth(&self) -> bool {
        *self == FORDER_A::FOURTH
    }
    #[doc = "Checks if the value of the field is `FIFTH`"]
    #[inline(always)]
    pub fn is_fifth(&self) -> bool {
        *self == FORDER_A::FIFTH
    }
}
#[doc = "Field `FORDER` writer - Filter order"]
pub type FORDER_W<'a> = crate::FieldWriterSafe<'a, u32, CFG0_SPEC, u8, FORDER_A, 2, 0>;
impl<'a> FORDER_W<'a> {
    #[doc = "Second order filter."]
    #[inline(always)]
    pub fn second(self) -> &'a mut W {
        self.variant(FORDER_A::SECOND)
    }
    #[doc = "Third order filter."]
    #[inline(always)]
    pub fn third(self) -> &'a mut W {
        self.variant(FORDER_A::THIRD)
    }
    #[doc = "Fourth order filter."]
    #[inline(always)]
    pub fn fourth(self) -> &'a mut W {
        self.variant(FORDER_A::FOURTH)
    }
    #[doc = "Fifth order filter."]
    #[inline(always)]
    pub fn fifth(self) -> &'a mut W {
        self.variant(FORDER_A::FIFTH)
    }
}
#[doc = "Number of Channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NUMCH_A {
    #[doc = "0: Only one Channel."]
    ONE = 0,
    #[doc = "1: Two Channels."]
    TWO = 1,
    #[doc = "2: Three Channels."]
    THREE = 2,
    #[doc = "3: Four Channels."]
    FOUR = 3,
}
impl From<NUMCH_A> for u8 {
    #[inline(always)]
    fn from(variant: NUMCH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NUMCH` reader - Number of Channels"]
pub type NUMCH_R = crate::FieldReader<u8, NUMCH_A>;
impl NUMCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NUMCH_A {
        match self.bits {
            0 => NUMCH_A::ONE,
            1 => NUMCH_A::TWO,
            2 => NUMCH_A::THREE,
            3 => NUMCH_A::FOUR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == NUMCH_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == NUMCH_A::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == NUMCH_A::THREE
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == NUMCH_A::FOUR
    }
}
#[doc = "Field `NUMCH` writer - Number of Channels"]
pub type NUMCH_W<'a> = crate::FieldWriterSafe<'a, u32, CFG0_SPEC, u8, NUMCH_A, 2, 4>;
impl<'a> NUMCH_W<'a> {
    #[doc = "Only one Channel."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(NUMCH_A::ONE)
    }
    #[doc = "Two Channels."]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(NUMCH_A::TWO)
    }
    #[doc = "Three Channels."]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(NUMCH_A::THREE)
    }
    #[doc = "Four Channels."]
    #[inline(always)]
    pub fn four(self) -> &'a mut W {
        self.variant(NUMCH_A::FOUR)
    }
}
#[doc = "Filter output format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATAFORMAT_A {
    #[doc = "0: Right aligned 16-bit, left bits are sign extended."]
    RIGHT16 = 0,
    #[doc = "1: Pack two 16-bit samples into one 32-bit word."]
    DOUBLE16 = 1,
    #[doc = "2: Right aligned 24bit, left bits are sign extended."]
    RIGHT24 = 2,
    #[doc = "3: 32 bit data."]
    FULL32BIT = 3,
    #[doc = "4: Left aligned 16-bit, right bits are zeros."]
    LEFT16 = 4,
    #[doc = "5: Left aligned 24-bit, right bits are zeros."]
    LEFT24 = 5,
    #[doc = "6: RAW 32 bit data from Integrator."]
    RAW32BIT = 6,
}
impl From<DATAFORMAT_A> for u8 {
    #[inline(always)]
    fn from(variant: DATAFORMAT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DATAFORMAT` reader - Filter output format"]
pub type DATAFORMAT_R = crate::FieldReader<u8, DATAFORMAT_A>;
impl DATAFORMAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATAFORMAT_A> {
        match self.bits {
            0 => Some(DATAFORMAT_A::RIGHT16),
            1 => Some(DATAFORMAT_A::DOUBLE16),
            2 => Some(DATAFORMAT_A::RIGHT24),
            3 => Some(DATAFORMAT_A::FULL32BIT),
            4 => Some(DATAFORMAT_A::LEFT16),
            5 => Some(DATAFORMAT_A::LEFT24),
            6 => Some(DATAFORMAT_A::RAW32BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RIGHT16`"]
    #[inline(always)]
    pub fn is_right16(&self) -> bool {
        *self == DATAFORMAT_A::RIGHT16
    }
    #[doc = "Checks if the value of the field is `DOUBLE16`"]
    #[inline(always)]
    pub fn is_double16(&self) -> bool {
        *self == DATAFORMAT_A::DOUBLE16
    }
    #[doc = "Checks if the value of the field is `RIGHT24`"]
    #[inline(always)]
    pub fn is_right24(&self) -> bool {
        *self == DATAFORMAT_A::RIGHT24
    }
    #[doc = "Checks if the value of the field is `FULL32BIT`"]
    #[inline(always)]
    pub fn is_full32bit(&self) -> bool {
        *self == DATAFORMAT_A::FULL32BIT
    }
    #[doc = "Checks if the value of the field is `LEFT16`"]
    #[inline(always)]
    pub fn is_left16(&self) -> bool {
        *self == DATAFORMAT_A::LEFT16
    }
    #[doc = "Checks if the value of the field is `LEFT24`"]
    #[inline(always)]
    pub fn is_left24(&self) -> bool {
        *self == DATAFORMAT_A::LEFT24
    }
    #[doc = "Checks if the value of the field is `RAW32BIT`"]
    #[inline(always)]
    pub fn is_raw32bit(&self) -> bool {
        *self == DATAFORMAT_A::RAW32BIT
    }
}
#[doc = "Field `DATAFORMAT` writer - Filter output format"]
pub type DATAFORMAT_W<'a> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, DATAFORMAT_A, 3, 8>;
impl<'a> DATAFORMAT_W<'a> {
    #[doc = "Right aligned 16-bit, left bits are sign extended."]
    #[inline(always)]
    pub fn right16(self) -> &'a mut W {
        self.variant(DATAFORMAT_A::RIGHT16)
    }
    #[doc = "Pack two 16-bit samples into one 32-bit word."]
    #[inline(always)]
    pub fn double16(self) -> &'a mut W {
        self.variant(DATAFORMAT_A::DOUBLE16)
    }
    #[doc = "Right aligned 24bit, left bits are sign extended."]
    #[inline(always)]
    pub fn right24(self) -> &'a mut W {
        self.variant(DATAFORMAT_A::RIGHT24)
    }
    #[doc = "32 bit data."]
    #[inline(always)]
    pub fn full32bit(self) -> &'a mut W {
        self.variant(DATAFORMAT_A::FULL32BIT)
    }
    #[doc = "Left aligned 16-bit, right bits are zeros."]
    #[inline(always)]
    pub fn left16(self) -> &'a mut W {
        self.variant(DATAFORMAT_A::LEFT16)
    }
    #[doc = "Left aligned 24-bit, right bits are zeros."]
    #[inline(always)]
    pub fn left24(self) -> &'a mut W {
        self.variant(DATAFORMAT_A::LEFT24)
    }
    #[doc = "RAW 32 bit data from Integrator."]
    #[inline(always)]
    pub fn raw32bit(self) -> &'a mut W {
        self.variant(DATAFORMAT_A::RAW32BIT)
    }
}
#[doc = "Data Valid level in FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FIFODVL_A {
    #[doc = "0: Atleast one word."]
    ONE = 0,
    #[doc = "1: Two words."]
    TWO = 1,
    #[doc = "2: Three words."]
    THREE = 2,
    #[doc = "3: Four words."]
    FOUR = 3,
}
impl From<FIFODVL_A> for u8 {
    #[inline(always)]
    fn from(variant: FIFODVL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FIFODVL` reader - Data Valid level in FIFO"]
pub type FIFODVL_R = crate::FieldReader<u8, FIFODVL_A>;
impl FIFODVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFODVL_A {
        match self.bits {
            0 => FIFODVL_A::ONE,
            1 => FIFODVL_A::TWO,
            2 => FIFODVL_A::THREE,
            3 => FIFODVL_A::FOUR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == FIFODVL_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == FIFODVL_A::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == FIFODVL_A::THREE
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == FIFODVL_A::FOUR
    }
}
#[doc = "Field `FIFODVL` writer - Data Valid level in FIFO"]
pub type FIFODVL_W<'a> = crate::FieldWriterSafe<'a, u32, CFG0_SPEC, u8, FIFODVL_A, 2, 12>;
impl<'a> FIFODVL_W<'a> {
    #[doc = "Atleast one word."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(FIFODVL_A::ONE)
    }
    #[doc = "Two words."]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(FIFODVL_A::TWO)
    }
    #[doc = "Three words."]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(FIFODVL_A::THREE)
    }
    #[doc = "Four words."]
    #[inline(always)]
    pub fn four(self) -> &'a mut W {
        self.variant(FIFODVL_A::FOUR)
    }
}
#[doc = "Field `STEREOMODECH01` reader - Stereo mode CH01"]
pub type STEREOMODECH01_R = crate::BitReader<bool>;
#[doc = "Field `STEREOMODECH01` writer - Stereo mode CH01"]
pub type STEREOMODECH01_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, 16>;
#[doc = "Field `STEREOMODECH23` reader - Stereo mode CH23"]
pub type STEREOMODECH23_R = crate::BitReader<bool>;
#[doc = "Field `STEREOMODECH23` writer - Stereo mode CH23"]
pub type STEREOMODECH23_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, 17>;
#[doc = "Field `CH0CLKPOL` reader - CH0 CLK Polarity"]
pub type CH0CLKPOL_R = crate::BitReader<bool>;
#[doc = "Field `CH0CLKPOL` writer - CH0 CLK Polarity"]
pub type CH0CLKPOL_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, 24>;
#[doc = "Field `CH1CLKPOL` reader - CH1 CLK Polarity"]
pub type CH1CLKPOL_R = crate::BitReader<bool>;
#[doc = "Field `CH1CLKPOL` writer - CH1 CLK Polarity"]
pub type CH1CLKPOL_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, 25>;
#[doc = "Field `CH2CLKPOL` reader - CH2 CLK Polarity"]
pub type CH2CLKPOL_R = crate::BitReader<bool>;
#[doc = "Field `CH2CLKPOL` writer - CH2 CLK Polarity"]
pub type CH2CLKPOL_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, 26>;
#[doc = "Field `CH3CLKPOL` reader - CH3 CLK Polarity"]
pub type CH3CLKPOL_R = crate::BitReader<bool>;
#[doc = "Field `CH3CLKPOL` writer - CH3 CLK Polarity"]
pub type CH3CLKPOL_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, 27>;
impl R {
    #[doc = "Bits 0:1 - Filter order"]
    #[inline(always)]
    pub fn forder(&self) -> FORDER_R {
        FORDER_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Number of Channels"]
    #[inline(always)]
    pub fn numch(&self) -> NUMCH_R {
        NUMCH_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Filter output format"]
    #[inline(always)]
    pub fn dataformat(&self) -> DATAFORMAT_R {
        DATAFORMAT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Data Valid level in FIFO"]
    #[inline(always)]
    pub fn fifodvl(&self) -> FIFODVL_R {
        FIFODVL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16 - Stereo mode CH01"]
    #[inline(always)]
    pub fn stereomodech01(&self) -> STEREOMODECH01_R {
        STEREOMODECH01_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Stereo mode CH23"]
    #[inline(always)]
    pub fn stereomodech23(&self) -> STEREOMODECH23_R {
        STEREOMODECH23_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - CH0 CLK Polarity"]
    #[inline(always)]
    pub fn ch0clkpol(&self) -> CH0CLKPOL_R {
        CH0CLKPOL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CH1 CLK Polarity"]
    #[inline(always)]
    pub fn ch1clkpol(&self) -> CH1CLKPOL_R {
        CH1CLKPOL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CH2 CLK Polarity"]
    #[inline(always)]
    pub fn ch2clkpol(&self) -> CH2CLKPOL_R {
        CH2CLKPOL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CH3 CLK Polarity"]
    #[inline(always)]
    pub fn ch3clkpol(&self) -> CH3CLKPOL_R {
        CH3CLKPOL_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Filter order"]
    #[inline(always)]
    pub fn forder(&mut self) -> FORDER_W {
        FORDER_W::new(self)
    }
    #[doc = "Bits 4:5 - Number of Channels"]
    #[inline(always)]
    pub fn numch(&mut self) -> NUMCH_W {
        NUMCH_W::new(self)
    }
    #[doc = "Bits 8:10 - Filter output format"]
    #[inline(always)]
    pub fn dataformat(&mut self) -> DATAFORMAT_W {
        DATAFORMAT_W::new(self)
    }
    #[doc = "Bits 12:13 - Data Valid level in FIFO"]
    #[inline(always)]
    pub fn fifodvl(&mut self) -> FIFODVL_W {
        FIFODVL_W::new(self)
    }
    #[doc = "Bit 16 - Stereo mode CH01"]
    #[inline(always)]
    pub fn stereomodech01(&mut self) -> STEREOMODECH01_W {
        STEREOMODECH01_W::new(self)
    }
    #[doc = "Bit 17 - Stereo mode CH23"]
    #[inline(always)]
    pub fn stereomodech23(&mut self) -> STEREOMODECH23_W {
        STEREOMODECH23_W::new(self)
    }
    #[doc = "Bit 24 - CH0 CLK Polarity"]
    #[inline(always)]
    pub fn ch0clkpol(&mut self) -> CH0CLKPOL_W {
        CH0CLKPOL_W::new(self)
    }
    #[doc = "Bit 25 - CH1 CLK Polarity"]
    #[inline(always)]
    pub fn ch1clkpol(&mut self) -> CH1CLKPOL_W {
        CH1CLKPOL_W::new(self)
    }
    #[doc = "Bit 26 - CH2 CLK Polarity"]
    #[inline(always)]
    pub fn ch2clkpol(&mut self) -> CH2CLKPOL_W {
        CH2CLKPOL_W::new(self)
    }
    #[doc = "Bit 27 - CH3 CLK Polarity"]
    #[inline(always)]
    pub fn ch3clkpol(&mut self) -> CH3CLKPOL_W {
        CH3CLKPOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDM Core Configuration Register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg0](index.html) module"]
pub struct CFG0_SPEC;
impl crate::RegisterSpec for CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg0::R](R) reader structure"]
impl crate::Readable for CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg0::W](W) writer structure"]
impl crate::Writable for CFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG0 to value 0"]
impl crate::Resettable for CFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
