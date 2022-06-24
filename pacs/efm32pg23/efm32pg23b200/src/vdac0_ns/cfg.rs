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
#[doc = "Differential Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIFF_A {
    #[doc = "0: Single ended output"]
    SINGLEENDED = 0,
    #[doc = "1: Differential output"]
    DIFFERENTIAL = 1,
}
impl From<DIFF_A> for bool {
    #[inline(always)]
    fn from(variant: DIFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIFF` reader - Differential Mode"]
pub type DIFF_R = crate::BitReader<DIFF_A>;
impl DIFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIFF_A {
        match self.bits {
            false => DIFF_A::SINGLEENDED,
            true => DIFF_A::DIFFERENTIAL,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLEENDED`"]
    #[inline(always)]
    pub fn is_singleended(&self) -> bool {
        *self == DIFF_A::SINGLEENDED
    }
    #[doc = "Checks if the value of the field is `DIFFERENTIAL`"]
    #[inline(always)]
    pub fn is_differential(&self) -> bool {
        *self == DIFF_A::DIFFERENTIAL
    }
}
#[doc = "Field `DIFF` writer - Differential Mode"]
pub type DIFF_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, DIFF_A, 0>;
impl<'a> DIFF_W<'a> {
    #[doc = "Single ended output"]
    #[inline(always)]
    pub fn singleended(self) -> &'a mut W {
        self.variant(DIFF_A::SINGLEENDED)
    }
    #[doc = "Differential output"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFF_A::DIFFERENTIAL)
    }
}
#[doc = "Sine Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SINEMODE_A {
    #[doc = "0: Sine mode disabled. Sine reset to 0 degrees"]
    DISSINEMODE = 0,
    #[doc = "1: Sine mode enabled"]
    ENSINEMODE = 1,
}
impl From<SINEMODE_A> for bool {
    #[inline(always)]
    fn from(variant: SINEMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SINEMODE` reader - Sine Mode"]
pub type SINEMODE_R = crate::BitReader<SINEMODE_A>;
impl SINEMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SINEMODE_A {
        match self.bits {
            false => SINEMODE_A::DISSINEMODE,
            true => SINEMODE_A::ENSINEMODE,
        }
    }
    #[doc = "Checks if the value of the field is `DISSINEMODE`"]
    #[inline(always)]
    pub fn is_dissinemode(&self) -> bool {
        *self == SINEMODE_A::DISSINEMODE
    }
    #[doc = "Checks if the value of the field is `ENSINEMODE`"]
    #[inline(always)]
    pub fn is_ensinemode(&self) -> bool {
        *self == SINEMODE_A::ENSINEMODE
    }
}
#[doc = "Field `SINEMODE` writer - Sine Mode"]
pub type SINEMODE_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, SINEMODE_A, 1>;
impl<'a> SINEMODE_W<'a> {
    #[doc = "Sine mode disabled. Sine reset to 0 degrees"]
    #[inline(always)]
    pub fn dissinemode(self) -> &'a mut W {
        self.variant(SINEMODE_A::DISSINEMODE)
    }
    #[doc = "Sine mode enabled"]
    #[inline(always)]
    pub fn ensinemode(self) -> &'a mut W {
        self.variant(SINEMODE_A::ENSINEMODE)
    }
}
#[doc = "Field `SINERESET` reader - Sine Wave Reset When inactive"]
pub type SINERESET_R = crate::BitReader<bool>;
#[doc = "Field `SINERESET` writer - Sine Wave Reset When inactive"]
pub type SINERESET_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 2>;
#[doc = "Channel 0 Start Reset Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0PRESCRST_A {
    #[doc = "0: Prescaler not reset on channel 0 start"]
    NORESETPRESC = 0,
    #[doc = "1: Prescaler reset on channel 0 start"]
    RESETPRESC = 1,
}
impl From<CH0PRESCRST_A> for bool {
    #[inline(always)]
    fn from(variant: CH0PRESCRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0PRESCRST` reader - Channel 0 Start Reset Prescaler"]
pub type CH0PRESCRST_R = crate::BitReader<CH0PRESCRST_A>;
impl CH0PRESCRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0PRESCRST_A {
        match self.bits {
            false => CH0PRESCRST_A::NORESETPRESC,
            true => CH0PRESCRST_A::RESETPRESC,
        }
    }
    #[doc = "Checks if the value of the field is `NORESETPRESC`"]
    #[inline(always)]
    pub fn is_noresetpresc(&self) -> bool {
        *self == CH0PRESCRST_A::NORESETPRESC
    }
    #[doc = "Checks if the value of the field is `RESETPRESC`"]
    #[inline(always)]
    pub fn is_resetpresc(&self) -> bool {
        *self == CH0PRESCRST_A::RESETPRESC
    }
}
#[doc = "Field `CH0PRESCRST` writer - Channel 0 Start Reset Prescaler"]
pub type CH0PRESCRST_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, CH0PRESCRST_A, 3>;
impl<'a> CH0PRESCRST_W<'a> {
    #[doc = "Prescaler not reset on channel 0 start"]
    #[inline(always)]
    pub fn noresetpresc(self) -> &'a mut W {
        self.variant(CH0PRESCRST_A::NORESETPRESC)
    }
    #[doc = "Prescaler reset on channel 0 start"]
    #[inline(always)]
    pub fn resetpresc(self) -> &'a mut W {
        self.variant(CH0PRESCRST_A::RESETPRESC)
    }
}
#[doc = "Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFRSEL_A {
    #[doc = "0: Internal 1.25 V bandgap reference"]
    V125 = 0,
    #[doc = "1: Internal 2.5 V bandgap reference"]
    V25 = 1,
    #[doc = "2: AVDD reference"]
    VDD = 2,
    #[doc = "3: External pin reference"]
    EXT = 3,
}
impl From<REFRSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFRSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REFRSEL` reader - Reference Selection"]
pub type REFRSEL_R = crate::FieldReader<u8, REFRSEL_A>;
impl REFRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFRSEL_A {
        match self.bits {
            0 => REFRSEL_A::V125,
            1 => REFRSEL_A::V25,
            2 => REFRSEL_A::VDD,
            3 => REFRSEL_A::EXT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `V125`"]
    #[inline(always)]
    pub fn is_v125(&self) -> bool {
        *self == REFRSEL_A::V125
    }
    #[doc = "Checks if the value of the field is `V25`"]
    #[inline(always)]
    pub fn is_v25(&self) -> bool {
        *self == REFRSEL_A::V25
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == REFRSEL_A::VDD
    }
    #[doc = "Checks if the value of the field is `EXT`"]
    #[inline(always)]
    pub fn is_ext(&self) -> bool {
        *self == REFRSEL_A::EXT
    }
}
#[doc = "Field `REFRSEL` writer - Reference Selection"]
pub type REFRSEL_W<'a> = crate::FieldWriterSafe<'a, u32, CFG_SPEC, u8, REFRSEL_A, 2, 4>;
impl<'a> REFRSEL_W<'a> {
    #[doc = "Internal 1.25 V bandgap reference"]
    #[inline(always)]
    pub fn v125(self) -> &'a mut W {
        self.variant(REFRSEL_A::V125)
    }
    #[doc = "Internal 2.5 V bandgap reference"]
    #[inline(always)]
    pub fn v25(self) -> &'a mut W {
        self.variant(REFRSEL_A::V25)
    }
    #[doc = "AVDD reference"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut W {
        self.variant(REFRSEL_A::VDD)
    }
    #[doc = "External pin reference"]
    #[inline(always)]
    pub fn ext(self) -> &'a mut W {
        self.variant(REFRSEL_A::EXT)
    }
}
#[doc = "Field `PRESC` reader - Prescaler Setting for DAC clock"]
pub type PRESC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRESC` writer - Prescaler Setting for DAC clock"]
pub type PRESC_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 7, 7>;
#[doc = "Internal Timer Overflow Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMEROVRFLOWPERIOD_A {
    #[doc = "0: The Timer overflows every 2 Prescaled CLK_DAC cycles"]
    CYCLES2 = 0,
    #[doc = "1: The Timer overflows every 4 Prescaled CLK_DAC cycles"]
    CYCLES4 = 1,
    #[doc = "2: The Timer overflows every 8 Prescaled CLK_DAC cycles"]
    CYCLES8 = 2,
    #[doc = "3: The Timer overflows every 16 Prescaled CLK_DAC cycles"]
    CYCLES16 = 3,
    #[doc = "4: The Timer overflows every 32 Prescaled CLK_DAC cycles"]
    CYCLES32 = 4,
    #[doc = "5: The Timer overflows every 64 Prescaled CLK_DAC cycles"]
    CYCLES64 = 5,
}
impl From<TIMEROVRFLOWPERIOD_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEROVRFLOWPERIOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMEROVRFLOWPERIOD` reader - Internal Timer Overflow Period"]
pub type TIMEROVRFLOWPERIOD_R = crate::FieldReader<u8, TIMEROVRFLOWPERIOD_A>;
impl TIMEROVRFLOWPERIOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIMEROVRFLOWPERIOD_A> {
        match self.bits {
            0 => Some(TIMEROVRFLOWPERIOD_A::CYCLES2),
            1 => Some(TIMEROVRFLOWPERIOD_A::CYCLES4),
            2 => Some(TIMEROVRFLOWPERIOD_A::CYCLES8),
            3 => Some(TIMEROVRFLOWPERIOD_A::CYCLES16),
            4 => Some(TIMEROVRFLOWPERIOD_A::CYCLES32),
            5 => Some(TIMEROVRFLOWPERIOD_A::CYCLES64),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CYCLES2`"]
    #[inline(always)]
    pub fn is_cycles2(&self) -> bool {
        *self == TIMEROVRFLOWPERIOD_A::CYCLES2
    }
    #[doc = "Checks if the value of the field is `CYCLES4`"]
    #[inline(always)]
    pub fn is_cycles4(&self) -> bool {
        *self == TIMEROVRFLOWPERIOD_A::CYCLES4
    }
    #[doc = "Checks if the value of the field is `CYCLES8`"]
    #[inline(always)]
    pub fn is_cycles8(&self) -> bool {
        *self == TIMEROVRFLOWPERIOD_A::CYCLES8
    }
    #[doc = "Checks if the value of the field is `CYCLES16`"]
    #[inline(always)]
    pub fn is_cycles16(&self) -> bool {
        *self == TIMEROVRFLOWPERIOD_A::CYCLES16
    }
    #[doc = "Checks if the value of the field is `CYCLES32`"]
    #[inline(always)]
    pub fn is_cycles32(&self) -> bool {
        *self == TIMEROVRFLOWPERIOD_A::CYCLES32
    }
    #[doc = "Checks if the value of the field is `CYCLES64`"]
    #[inline(always)]
    pub fn is_cycles64(&self) -> bool {
        *self == TIMEROVRFLOWPERIOD_A::CYCLES64
    }
}
#[doc = "Field `TIMEROVRFLOWPERIOD` writer - Internal Timer Overflow Period"]
pub type TIMEROVRFLOWPERIOD_W<'a> =
    crate::FieldWriter<'a, u32, CFG_SPEC, u8, TIMEROVRFLOWPERIOD_A, 3, 16>;
impl<'a> TIMEROVRFLOWPERIOD_W<'a> {
    #[doc = "The Timer overflows every 2 Prescaled CLK_DAC cycles"]
    #[inline(always)]
    pub fn cycles2(self) -> &'a mut W {
        self.variant(TIMEROVRFLOWPERIOD_A::CYCLES2)
    }
    #[doc = "The Timer overflows every 4 Prescaled CLK_DAC cycles"]
    #[inline(always)]
    pub fn cycles4(self) -> &'a mut W {
        self.variant(TIMEROVRFLOWPERIOD_A::CYCLES4)
    }
    #[doc = "The Timer overflows every 8 Prescaled CLK_DAC cycles"]
    #[inline(always)]
    pub fn cycles8(self) -> &'a mut W {
        self.variant(TIMEROVRFLOWPERIOD_A::CYCLES8)
    }
    #[doc = "The Timer overflows every 16 Prescaled CLK_DAC cycles"]
    #[inline(always)]
    pub fn cycles16(self) -> &'a mut W {
        self.variant(TIMEROVRFLOWPERIOD_A::CYCLES16)
    }
    #[doc = "The Timer overflows every 32 Prescaled CLK_DAC cycles"]
    #[inline(always)]
    pub fn cycles32(self) -> &'a mut W {
        self.variant(TIMEROVRFLOWPERIOD_A::CYCLES32)
    }
    #[doc = "The Timer overflows every 64 Prescaled CLK_DAC cycles"]
    #[inline(always)]
    pub fn cycles64(self) -> &'a mut W {
        self.variant(TIMEROVRFLOWPERIOD_A::CYCLES64)
    }
}
#[doc = "Refresh Timer Overflow Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFRESHPERIOD_A {
    #[doc = "0: All channels with enabled refresh are refreshed every 2 CLK_REFRESH cycles"]
    CYCLES2 = 0,
    #[doc = "1: All channels with enabled refresh are refreshed every 4 CLK_REFRESH cycles"]
    CYCLES4 = 1,
    #[doc = "2: All channels with enabled refresh are refreshed every 8 CLK_REFRESH cycles"]
    CYCLES8 = 2,
    #[doc = "3: All channels with enabled refresh are refreshed every 16 CLK_REFRESH cycles"]
    CYCLES16 = 3,
    #[doc = "4: All channels with enabled refresh are refreshed every 32 CLK_REFRESH cycles"]
    CYCLES32 = 4,
    #[doc = "5: All channels with enabled refresh are refreshed every 64 CLK_REFRESH cycles"]
    CYCLES64 = 5,
    #[doc = "6: All channels with enabled refresh are refreshed every 128 CLK_REFRESH cycles"]
    CYCLES128 = 6,
    #[doc = "7: All channels with enabled refresh are refreshed every 256 CLK_REFRESH cycles"]
    CYCLES256 = 7,
}
impl From<REFRESHPERIOD_A> for u8 {
    #[inline(always)]
    fn from(variant: REFRESHPERIOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REFRESHPERIOD` reader - Refresh Timer Overflow Period"]
pub type REFRESHPERIOD_R = crate::FieldReader<u8, REFRESHPERIOD_A>;
impl REFRESHPERIOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFRESHPERIOD_A {
        match self.bits {
            0 => REFRESHPERIOD_A::CYCLES2,
            1 => REFRESHPERIOD_A::CYCLES4,
            2 => REFRESHPERIOD_A::CYCLES8,
            3 => REFRESHPERIOD_A::CYCLES16,
            4 => REFRESHPERIOD_A::CYCLES32,
            5 => REFRESHPERIOD_A::CYCLES64,
            6 => REFRESHPERIOD_A::CYCLES128,
            7 => REFRESHPERIOD_A::CYCLES256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLES2`"]
    #[inline(always)]
    pub fn is_cycles2(&self) -> bool {
        *self == REFRESHPERIOD_A::CYCLES2
    }
    #[doc = "Checks if the value of the field is `CYCLES4`"]
    #[inline(always)]
    pub fn is_cycles4(&self) -> bool {
        *self == REFRESHPERIOD_A::CYCLES4
    }
    #[doc = "Checks if the value of the field is `CYCLES8`"]
    #[inline(always)]
    pub fn is_cycles8(&self) -> bool {
        *self == REFRESHPERIOD_A::CYCLES8
    }
    #[doc = "Checks if the value of the field is `CYCLES16`"]
    #[inline(always)]
    pub fn is_cycles16(&self) -> bool {
        *self == REFRESHPERIOD_A::CYCLES16
    }
    #[doc = "Checks if the value of the field is `CYCLES32`"]
    #[inline(always)]
    pub fn is_cycles32(&self) -> bool {
        *self == REFRESHPERIOD_A::CYCLES32
    }
    #[doc = "Checks if the value of the field is `CYCLES64`"]
    #[inline(always)]
    pub fn is_cycles64(&self) -> bool {
        *self == REFRESHPERIOD_A::CYCLES64
    }
    #[doc = "Checks if the value of the field is `CYCLES128`"]
    #[inline(always)]
    pub fn is_cycles128(&self) -> bool {
        *self == REFRESHPERIOD_A::CYCLES128
    }
    #[doc = "Checks if the value of the field is `CYCLES256`"]
    #[inline(always)]
    pub fn is_cycles256(&self) -> bool {
        *self == REFRESHPERIOD_A::CYCLES256
    }
}
#[doc = "Field `REFRESHPERIOD` writer - Refresh Timer Overflow Period"]
pub type REFRESHPERIOD_W<'a> =
    crate::FieldWriterSafe<'a, u32, CFG_SPEC, u8, REFRESHPERIOD_A, 3, 20>;
impl<'a> REFRESHPERIOD_W<'a> {
    #[doc = "All channels with enabled refresh are refreshed every 2 CLK_REFRESH cycles"]
    #[inline(always)]
    pub fn cycles2(self) -> &'a mut W {
        self.variant(REFRESHPERIOD_A::CYCLES2)
    }
    #[doc = "All channels with enabled refresh are refreshed every 4 CLK_REFRESH cycles"]
    #[inline(always)]
    pub fn cycles4(self) -> &'a mut W {
        self.variant(REFRESHPERIOD_A::CYCLES4)
    }
    #[doc = "All channels with enabled refresh are refreshed every 8 CLK_REFRESH cycles"]
    #[inline(always)]
    pub fn cycles8(self) -> &'a mut W {
        self.variant(REFRESHPERIOD_A::CYCLES8)
    }
    #[doc = "All channels with enabled refresh are refreshed every 16 CLK_REFRESH cycles"]
    #[inline(always)]
    pub fn cycles16(self) -> &'a mut W {
        self.variant(REFRESHPERIOD_A::CYCLES16)
    }
    #[doc = "All channels with enabled refresh are refreshed every 32 CLK_REFRESH cycles"]
    #[inline(always)]
    pub fn cycles32(self) -> &'a mut W {
        self.variant(REFRESHPERIOD_A::CYCLES32)
    }
    #[doc = "All channels with enabled refresh are refreshed every 64 CLK_REFRESH cycles"]
    #[inline(always)]
    pub fn cycles64(self) -> &'a mut W {
        self.variant(REFRESHPERIOD_A::CYCLES64)
    }
    #[doc = "All channels with enabled refresh are refreshed every 128 CLK_REFRESH cycles"]
    #[inline(always)]
    pub fn cycles128(self) -> &'a mut W {
        self.variant(REFRESHPERIOD_A::CYCLES128)
    }
    #[doc = "All channels with enabled refresh are refreshed every 256 CLK_REFRESH cycles"]
    #[inline(always)]
    pub fn cycles256(self) -> &'a mut W {
        self.variant(REFRESHPERIOD_A::CYCLES256)
    }
}
#[doc = "Field `BIASKEEPWARM` reader - Bias Keepwarm Mode Enable"]
pub type BIASKEEPWARM_R = crate::BitReader<bool>;
#[doc = "Field `BIASKEEPWARM` writer - Bias Keepwarm Mode Enable"]
pub type BIASKEEPWARM_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 24>;
#[doc = "Field `DMAWU` reader - VDAC DMA Wakeup"]
pub type DMAWU_R = crate::BitReader<bool>;
#[doc = "Field `DMAWU` writer - VDAC DMA Wakeup"]
pub type DMAWU_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 25>;
#[doc = "Field `ONDEMANDCLK` reader - Always allow clk_dac"]
pub type ONDEMANDCLK_R = crate::BitReader<bool>;
#[doc = "Field `ONDEMANDCLK` writer - Always allow clk_dac"]
pub type ONDEMANDCLK_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 26>;
#[doc = "Debug Halt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGHALT_A {
    #[doc = "0: Continue operation as normal during debug mode"]
    NORMAL = 0,
    #[doc = "1: Complete the current conversion and then halt during debug mode"]
    HALT = 1,
}
impl From<DBGHALT_A> for bool {
    #[inline(always)]
    fn from(variant: DBGHALT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGHALT` reader - Debug Halt"]
pub type DBGHALT_R = crate::BitReader<DBGHALT_A>;
impl DBGHALT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGHALT_A {
        match self.bits {
            false => DBGHALT_A::NORMAL,
            true => DBGHALT_A::HALT,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == DBGHALT_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `HALT`"]
    #[inline(always)]
    pub fn is_halt(&self) -> bool {
        *self == DBGHALT_A::HALT
    }
}
#[doc = "Field `DBGHALT` writer - Debug Halt"]
pub type DBGHALT_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, DBGHALT_A, 27>;
impl<'a> DBGHALT_W<'a> {
    #[doc = "Continue operation as normal during debug mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(DBGHALT_A::NORMAL)
    }
    #[doc = "Complete the current conversion and then halt during debug mode"]
    #[inline(always)]
    pub fn halt(self) -> &'a mut W {
        self.variant(DBGHALT_A::HALT)
    }
}
#[doc = "Field `WARMUPTIME` reader - DAC Warmup Time"]
pub type WARMUPTIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WARMUPTIME` writer - DAC Warmup Time"]
pub type WARMUPTIME_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 3, 28>;
impl R {
    #[doc = "Bit 0 - Differential Mode"]
    #[inline(always)]
    pub fn diff(&self) -> DIFF_R {
        DIFF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sine Mode"]
    #[inline(always)]
    pub fn sinemode(&self) -> SINEMODE_R {
        SINEMODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sine Wave Reset When inactive"]
    #[inline(always)]
    pub fn sinereset(&self) -> SINERESET_R {
        SINERESET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 0 Start Reset Prescaler"]
    #[inline(always)]
    pub fn ch0prescrst(&self) -> CH0PRESCRST_R {
        CH0PRESCRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Reference Selection"]
    #[inline(always)]
    pub fn refrsel(&self) -> REFRSEL_R {
        REFRSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 7:13 - Prescaler Setting for DAC clock"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 16:18 - Internal Timer Overflow Period"]
    #[inline(always)]
    pub fn timerovrflowperiod(&self) -> TIMEROVRFLOWPERIOD_R {
        TIMEROVRFLOWPERIOD_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Refresh Timer Overflow Period"]
    #[inline(always)]
    pub fn refreshperiod(&self) -> REFRESHPERIOD_R {
        REFRESHPERIOD_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 24 - Bias Keepwarm Mode Enable"]
    #[inline(always)]
    pub fn biaskeepwarm(&self) -> BIASKEEPWARM_R {
        BIASKEEPWARM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - VDAC DMA Wakeup"]
    #[inline(always)]
    pub fn dmawu(&self) -> DMAWU_R {
        DMAWU_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Always allow clk_dac"]
    #[inline(always)]
    pub fn ondemandclk(&self) -> ONDEMANDCLK_R {
        ONDEMANDCLK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Debug Halt"]
    #[inline(always)]
    pub fn dbghalt(&self) -> DBGHALT_R {
        DBGHALT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - DAC Warmup Time"]
    #[inline(always)]
    pub fn warmuptime(&self) -> WARMUPTIME_R {
        WARMUPTIME_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Differential Mode"]
    #[inline(always)]
    pub fn diff(&mut self) -> DIFF_W {
        DIFF_W::new(self)
    }
    #[doc = "Bit 1 - Sine Mode"]
    #[inline(always)]
    pub fn sinemode(&mut self) -> SINEMODE_W {
        SINEMODE_W::new(self)
    }
    #[doc = "Bit 2 - Sine Wave Reset When inactive"]
    #[inline(always)]
    pub fn sinereset(&mut self) -> SINERESET_W {
        SINERESET_W::new(self)
    }
    #[doc = "Bit 3 - Channel 0 Start Reset Prescaler"]
    #[inline(always)]
    pub fn ch0prescrst(&mut self) -> CH0PRESCRST_W {
        CH0PRESCRST_W::new(self)
    }
    #[doc = "Bits 4:5 - Reference Selection"]
    #[inline(always)]
    pub fn refrsel(&mut self) -> REFRSEL_W {
        REFRSEL_W::new(self)
    }
    #[doc = "Bits 7:13 - Prescaler Setting for DAC clock"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W::new(self)
    }
    #[doc = "Bits 16:18 - Internal Timer Overflow Period"]
    #[inline(always)]
    pub fn timerovrflowperiod(&mut self) -> TIMEROVRFLOWPERIOD_W {
        TIMEROVRFLOWPERIOD_W::new(self)
    }
    #[doc = "Bits 20:22 - Refresh Timer Overflow Period"]
    #[inline(always)]
    pub fn refreshperiod(&mut self) -> REFRESHPERIOD_W {
        REFRESHPERIOD_W::new(self)
    }
    #[doc = "Bit 24 - Bias Keepwarm Mode Enable"]
    #[inline(always)]
    pub fn biaskeepwarm(&mut self) -> BIASKEEPWARM_W {
        BIASKEEPWARM_W::new(self)
    }
    #[doc = "Bit 25 - VDAC DMA Wakeup"]
    #[inline(always)]
    pub fn dmawu(&mut self) -> DMAWU_W {
        DMAWU_W::new(self)
    }
    #[doc = "Bit 26 - Always allow clk_dac"]
    #[inline(always)]
    pub fn ondemandclk(&mut self) -> ONDEMANDCLK_W {
        ONDEMANDCLK_W::new(self)
    }
    #[doc = "Bit 27 - Debug Halt"]
    #[inline(always)]
    pub fn dbghalt(&mut self) -> DBGHALT_W {
        DBGHALT_W::new(self)
    }
    #[doc = "Bits 28:30 - DAC Warmup Time"]
    #[inline(always)]
    pub fn warmuptime(&mut self) -> WARMUPTIME_W {
        WARMUPTIME_W::new(self)
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
#[doc = "`reset()` method sets CFG to value 0x2000_0000"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000_0000
    }
}
