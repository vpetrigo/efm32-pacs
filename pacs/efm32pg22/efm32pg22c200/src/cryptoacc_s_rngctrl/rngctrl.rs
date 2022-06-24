#[doc = "Register `RNGCTRL` reader"]
pub struct R(crate::R<RNGCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RNGCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RNGCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RNGCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RNGCTRL` writer"]
pub struct W(crate::W<RNGCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RNGCTRL_SPEC>;
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
impl From<crate::W<RNGCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RNGCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TRNG Module Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Module disabled"]
    DISABLED = 0,
    #[doc = "1: Module enabled"]
    ENABLED = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - TRNG Module Enable"]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLED,
            true => ENABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE_A::ENABLED
    }
}
#[doc = "Field `ENABLE` writer - TRNG Module Enable"]
pub type ENABLE_W<'a> = crate::BitWriter<'a, u32, RNGCTRL_SPEC, ENABLE_A, 0>;
impl<'a> ENABLE_W<'a> {
    #[doc = "Module disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLED)
    }
    #[doc = "Module enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLED)
    }
}
#[doc = "Test Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TESTEN_A {
    #[doc = "0: Non-determinsitc random number generation"]
    NOISE = 0,
    #[doc = "1: Pseudo-random number generation"]
    TESTDATA = 1,
}
impl From<TESTEN_A> for bool {
    #[inline(always)]
    fn from(variant: TESTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TESTEN` reader - Test Enable"]
pub type TESTEN_R = crate::BitReader<TESTEN_A>;
impl TESTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TESTEN_A {
        match self.bits {
            false => TESTEN_A::NOISE,
            true => TESTEN_A::TESTDATA,
        }
    }
    #[doc = "Checks if the value of the field is `NOISE`"]
    #[inline(always)]
    pub fn is_noise(&self) -> bool {
        *self == TESTEN_A::NOISE
    }
    #[doc = "Checks if the value of the field is `TESTDATA`"]
    #[inline(always)]
    pub fn is_testdata(&self) -> bool {
        *self == TESTEN_A::TESTDATA
    }
}
#[doc = "Field `TESTEN` writer - Test Enable"]
pub type TESTEN_W<'a> = crate::BitWriter<'a, u32, RNGCTRL_SPEC, TESTEN_A, 2>;
impl<'a> TESTEN_W<'a> {
    #[doc = "Non-determinsitc random number generation"]
    #[inline(always)]
    pub fn noise(self) -> &'a mut W {
        self.variant(TESTEN_A::NOISE)
    }
    #[doc = "Pseudo-random number generation"]
    #[inline(always)]
    pub fn testdata(self) -> &'a mut W {
        self.variant(TESTEN_A::TESTDATA)
    }
}
#[doc = "Conditioning Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONDBYPASS_A {
    #[doc = "0: The conditionig function is used"]
    NORMAL = 0,
    #[doc = "1: The conditioning function is bypassed"]
    BYPASS = 1,
}
impl From<CONDBYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: CONDBYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONDBYPASS` reader - Conditioning Bypass"]
pub type CONDBYPASS_R = crate::BitReader<CONDBYPASS_A>;
impl CONDBYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONDBYPASS_A {
        match self.bits {
            false => CONDBYPASS_A::NORMAL,
            true => CONDBYPASS_A::BYPASS,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CONDBYPASS_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == CONDBYPASS_A::BYPASS
    }
}
#[doc = "Field `CONDBYPASS` writer - Conditioning Bypass"]
pub type CONDBYPASS_W<'a> = crate::BitWriter<'a, u32, RNGCTRL_SPEC, CONDBYPASS_A, 3>;
impl<'a> CONDBYPASS_W<'a> {
    #[doc = "The conditionig function is used"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(CONDBYPASS_A::NORMAL)
    }
    #[doc = "The conditioning function is bypassed"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(CONDBYPASS_A::BYPASS)
    }
}
#[doc = "Field `REPCOUNTIEN` reader - IRQ enable for Repetition Count Test"]
pub type REPCOUNTIEN_R = crate::BitReader<bool>;
#[doc = "Field `REPCOUNTIEN` writer - IRQ enable for Repetition Count Test"]
pub type REPCOUNTIEN_W<'a> = crate::BitWriter<'a, u32, RNGCTRL_SPEC, bool, 4>;
#[doc = "Field `APT64IEN` reader - IRQ enable for APT64IF"]
pub type APT64IEN_R = crate::BitReader<bool>;
#[doc = "Field `APT64IEN` writer - IRQ enable for APT64IF"]
pub type APT64IEN_W<'a> = crate::BitWriter<'a, u32, RNGCTRL_SPEC, bool, 5>;
#[doc = "Field `APT4096IEN` reader - IRQ enable for APT4096IF"]
pub type APT4096IEN_R = crate::BitReader<bool>;
#[doc = "Field `APT4096IEN` writer - IRQ enable for APT4096IF"]
pub type APT4096IEN_W<'a> = crate::BitWriter<'a, u32, RNGCTRL_SPEC, bool, 6>;
#[doc = "Field `FULLIEN` reader - IRQ enable for FIFO full"]
pub type FULLIEN_R = crate::BitReader<bool>;
#[doc = "Field `FULLIEN` writer - IRQ enable for FIFO full"]
pub type FULLIEN_W<'a> = crate::BitWriter<'a, u32, RNGCTRL_SPEC, bool, 7>;
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFTRESET_A {
    #[doc = "0: Module not in reset"]
    NORMAL = 0,
    #[doc = "1: The continuous test, the conditioning function and the FIFO are reset"]
    RESET = 1,
}
impl From<SOFTRESET_A> for bool {
    #[inline(always)]
    fn from(variant: SOFTRESET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFTRESET` reader - Software Reset"]
pub type SOFTRESET_R = crate::BitReader<SOFTRESET_A>;
impl SOFTRESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFTRESET_A {
        match self.bits {
            false => SOFTRESET_A::NORMAL,
            true => SOFTRESET_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SOFTRESET_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SOFTRESET_A::RESET
    }
}
#[doc = "Field `SOFTRESET` writer - Software Reset"]
pub type SOFTRESET_W<'a> = crate::BitWriter<'a, u32, RNGCTRL_SPEC, SOFTRESET_A, 8>;
impl<'a> SOFTRESET_W<'a> {
    #[doc = "Module not in reset"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SOFTRESET_A::NORMAL)
    }
    #[doc = "The continuous test, the conditioning function and the FIFO are reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SOFTRESET_A::RESET)
    }
}
#[doc = "Field `PREIEN` reader - IRQ enable for AIS31 prelim. noise alarm"]
pub type PREIEN_R = crate::BitReader<bool>;
#[doc = "Field `PREIEN` writer - IRQ enable for AIS31 prelim. noise alarm"]
pub type PREIEN_W<'a> = crate::BitWriter<'a, u32, RNGCTRL_SPEC, bool, 9>;
#[doc = "Field `ALMIEN` reader - IRQ enable for AIS31 noise alarm"]
pub type ALMIEN_R = crate::BitReader<bool>;
#[doc = "Field `ALMIEN` writer - IRQ enable for AIS31 noise alarm"]
pub type ALMIEN_W<'a> = crate::BitWriter<'a, u32, RNGCTRL_SPEC, bool, 10>;
#[doc = "Oscillator Force Run\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCERUN_A {
    #[doc = "0: Oscillators will shut down when FIFO is full"]
    NORMAL = 0,
    #[doc = "1: Oscillators will continue to run even after FIFO is full"]
    RUN = 1,
}
impl From<FORCERUN_A> for bool {
    #[inline(always)]
    fn from(variant: FORCERUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCERUN` reader - Oscillator Force Run"]
pub type FORCERUN_R = crate::BitReader<FORCERUN_A>;
impl FORCERUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCERUN_A {
        match self.bits {
            false => FORCERUN_A::NORMAL,
            true => FORCERUN_A::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == FORCERUN_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == FORCERUN_A::RUN
    }
}
#[doc = "Field `FORCERUN` writer - Oscillator Force Run"]
pub type FORCERUN_W<'a> = crate::BitWriter<'a, u32, RNGCTRL_SPEC, FORCERUN_A, 11>;
impl<'a> FORCERUN_W<'a> {
    #[doc = "Oscillators will shut down when FIFO is full"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(FORCERUN_A::NORMAL)
    }
    #[doc = "Oscillators will continue to run even after FIFO is full"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(FORCERUN_A::RUN)
    }
}
#[doc = "NIST Start-up Test Bypass.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPNIST_A {
    #[doc = "0: NIST-800-90B startup test is applied. No data will be written to the FIFO until the test passes."]
    NORMAL = 0,
    #[doc = "1: NIST-800-90B startup test is bypassed."]
    BYPASS = 1,
}
impl From<BYPNIST_A> for bool {
    #[inline(always)]
    fn from(variant: BYPNIST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPNIST` reader - NIST Start-up Test Bypass."]
pub type BYPNIST_R = crate::BitReader<BYPNIST_A>;
impl BYPNIST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPNIST_A {
        match self.bits {
            false => BYPNIST_A::NORMAL,
            true => BYPNIST_A::BYPASS,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == BYPNIST_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == BYPNIST_A::BYPASS
    }
}
#[doc = "Field `BYPNIST` writer - NIST Start-up Test Bypass."]
pub type BYPNIST_W<'a> = crate::BitWriter<'a, u32, RNGCTRL_SPEC, BYPNIST_A, 12>;
impl<'a> BYPNIST_W<'a> {
    #[doc = "NIST-800-90B startup test is applied. No data will be written to the FIFO until the test passes."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(BYPNIST_A::NORMAL)
    }
    #[doc = "NIST-800-90B startup test is bypassed."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(BYPNIST_A::BYPASS)
    }
}
#[doc = "AIS31 Start-up Test Bypass.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPAIS31_A {
    #[doc = "0: AIS31 startup test is applied. No data will be written to the FIFO until the test passes."]
    NORMAL = 0,
    #[doc = "1: AIS31 startup test is bypassed."]
    BYPASS = 1,
}
impl From<BYPAIS31_A> for bool {
    #[inline(always)]
    fn from(variant: BYPAIS31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPAIS31` reader - AIS31 Start-up Test Bypass."]
pub type BYPAIS31_R = crate::BitReader<BYPAIS31_A>;
impl BYPAIS31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPAIS31_A {
        match self.bits {
            false => BYPAIS31_A::NORMAL,
            true => BYPAIS31_A::BYPASS,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == BYPAIS31_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == BYPAIS31_A::BYPASS
    }
}
#[doc = "Field `BYPAIS31` writer - AIS31 Start-up Test Bypass."]
pub type BYPAIS31_W<'a> = crate::BitWriter<'a, u32, RNGCTRL_SPEC, BYPAIS31_A, 13>;
impl<'a> BYPAIS31_W<'a> {
    #[doc = "AIS31 startup test is applied. No data will be written to the FIFO until the test passes."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(BYPAIS31_A::NORMAL)
    }
    #[doc = "AIS31 startup test is bypassed."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(BYPAIS31_A::BYPASS)
    }
}
#[doc = "Health test input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HEALTHTESTSEL_A {
    #[doc = "0: Before conditioning"]
    BEFORE = 0,
    #[doc = "1: After conditioning"]
    AFTER = 1,
}
impl From<HEALTHTESTSEL_A> for bool {
    #[inline(always)]
    fn from(variant: HEALTHTESTSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HEALTHTESTSEL` reader - Health test input select"]
pub type HEALTHTESTSEL_R = crate::BitReader<HEALTHTESTSEL_A>;
impl HEALTHTESTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HEALTHTESTSEL_A {
        match self.bits {
            false => HEALTHTESTSEL_A::BEFORE,
            true => HEALTHTESTSEL_A::AFTER,
        }
    }
    #[doc = "Checks if the value of the field is `BEFORE`"]
    #[inline(always)]
    pub fn is_before(&self) -> bool {
        *self == HEALTHTESTSEL_A::BEFORE
    }
    #[doc = "Checks if the value of the field is `AFTER`"]
    #[inline(always)]
    pub fn is_after(&self) -> bool {
        *self == HEALTHTESTSEL_A::AFTER
    }
}
#[doc = "Field `HEALTHTESTSEL` writer - Health test input select"]
pub type HEALTHTESTSEL_W<'a> = crate::BitWriter<'a, u32, RNGCTRL_SPEC, HEALTHTESTSEL_A, 14>;
impl<'a> HEALTHTESTSEL_W<'a> {
    #[doc = "Before conditioning"]
    #[inline(always)]
    pub fn before(self) -> &'a mut W {
        self.variant(HEALTHTESTSEL_A::BEFORE)
    }
    #[doc = "After conditioning"]
    #[inline(always)]
    pub fn after(self) -> &'a mut W {
        self.variant(HEALTHTESTSEL_A::AFTER)
    }
}
#[doc = "AIS31 test input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIS31TESTSEL_A {
    #[doc = "0: Before conditioning"]
    BEFORE = 0,
    #[doc = "1: After conditioning"]
    AFTER = 1,
}
impl From<AIS31TESTSEL_A> for bool {
    #[inline(always)]
    fn from(variant: AIS31TESTSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AIS31TESTSEL` reader - AIS31 test input select"]
pub type AIS31TESTSEL_R = crate::BitReader<AIS31TESTSEL_A>;
impl AIS31TESTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AIS31TESTSEL_A {
        match self.bits {
            false => AIS31TESTSEL_A::BEFORE,
            true => AIS31TESTSEL_A::AFTER,
        }
    }
    #[doc = "Checks if the value of the field is `BEFORE`"]
    #[inline(always)]
    pub fn is_before(&self) -> bool {
        *self == AIS31TESTSEL_A::BEFORE
    }
    #[doc = "Checks if the value of the field is `AFTER`"]
    #[inline(always)]
    pub fn is_after(&self) -> bool {
        *self == AIS31TESTSEL_A::AFTER
    }
}
#[doc = "Field `AIS31TESTSEL` writer - AIS31 test input select"]
pub type AIS31TESTSEL_W<'a> = crate::BitWriter<'a, u32, RNGCTRL_SPEC, AIS31TESTSEL_A, 15>;
impl<'a> AIS31TESTSEL_W<'a> {
    #[doc = "Before conditioning"]
    #[inline(always)]
    pub fn before(self) -> &'a mut W {
        self.variant(AIS31TESTSEL_A::BEFORE)
    }
    #[doc = "After conditioning"]
    #[inline(always)]
    pub fn after(self) -> &'a mut W {
        self.variant(AIS31TESTSEL_A::AFTER)
    }
}
#[doc = "Field `NB128BITBLOCKS` reader - Number of 128b blocks in AES-CBCMAC"]
pub type NB128BITBLOCKS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NB128BITBLOCKS` writer - Number of 128b blocks in AES-CBCMAC"]
pub type NB128BITBLOCKS_W<'a> = crate::FieldWriter<'a, u32, RNGCTRL_SPEC, u8, u8, 4, 16>;
#[doc = "Field `FIFOWRSTARTUP` reader - Fifo Write Start Up"]
pub type FIFOWRSTARTUP_R = crate::BitReader<bool>;
#[doc = "Field `FIFOWRSTARTUP` writer - Fifo Write Start Up"]
pub type FIFOWRSTARTUP_W<'a> = crate::BitWriter<'a, u32, RNGCTRL_SPEC, bool, 20>;
impl R {
    #[doc = "Bit 0 - TRNG Module Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Test Enable"]
    #[inline(always)]
    pub fn testen(&self) -> TESTEN_R {
        TESTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Conditioning Bypass"]
    #[inline(always)]
    pub fn condbypass(&self) -> CONDBYPASS_R {
        CONDBYPASS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IRQ enable for Repetition Count Test"]
    #[inline(always)]
    pub fn repcountien(&self) -> REPCOUNTIEN_R {
        REPCOUNTIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IRQ enable for APT64IF"]
    #[inline(always)]
    pub fn apt64ien(&self) -> APT64IEN_R {
        APT64IEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IRQ enable for APT4096IF"]
    #[inline(always)]
    pub fn apt4096ien(&self) -> APT4096IEN_R {
        APT4096IEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IRQ enable for FIFO full"]
    #[inline(always)]
    pub fn fullien(&self) -> FULLIEN_R {
        FULLIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software Reset"]
    #[inline(always)]
    pub fn softreset(&self) -> SOFTRESET_R {
        SOFTRESET_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IRQ enable for AIS31 prelim. noise alarm"]
    #[inline(always)]
    pub fn preien(&self) -> PREIEN_R {
        PREIEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IRQ enable for AIS31 noise alarm"]
    #[inline(always)]
    pub fn almien(&self) -> ALMIEN_R {
        ALMIEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Oscillator Force Run"]
    #[inline(always)]
    pub fn forcerun(&self) -> FORCERUN_R {
        FORCERUN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NIST Start-up Test Bypass."]
    #[inline(always)]
    pub fn bypnist(&self) -> BYPNIST_R {
        BYPNIST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AIS31 Start-up Test Bypass."]
    #[inline(always)]
    pub fn bypais31(&self) -> BYPAIS31_R {
        BYPAIS31_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Health test input select"]
    #[inline(always)]
    pub fn healthtestsel(&self) -> HEALTHTESTSEL_R {
        HEALTHTESTSEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AIS31 test input select"]
    #[inline(always)]
    pub fn ais31testsel(&self) -> AIS31TESTSEL_R {
        AIS31TESTSEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Number of 128b blocks in AES-CBCMAC"]
    #[inline(always)]
    pub fn nb128bitblocks(&self) -> NB128BITBLOCKS_R {
        NB128BITBLOCKS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Fifo Write Start Up"]
    #[inline(always)]
    pub fn fifowrstartup(&self) -> FIFOWRSTARTUP_R {
        FIFOWRSTARTUP_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TRNG Module Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Test Enable"]
    #[inline(always)]
    pub fn testen(&mut self) -> TESTEN_W {
        TESTEN_W::new(self)
    }
    #[doc = "Bit 3 - Conditioning Bypass"]
    #[inline(always)]
    pub fn condbypass(&mut self) -> CONDBYPASS_W {
        CONDBYPASS_W::new(self)
    }
    #[doc = "Bit 4 - IRQ enable for Repetition Count Test"]
    #[inline(always)]
    pub fn repcountien(&mut self) -> REPCOUNTIEN_W {
        REPCOUNTIEN_W::new(self)
    }
    #[doc = "Bit 5 - IRQ enable for APT64IF"]
    #[inline(always)]
    pub fn apt64ien(&mut self) -> APT64IEN_W {
        APT64IEN_W::new(self)
    }
    #[doc = "Bit 6 - IRQ enable for APT4096IF"]
    #[inline(always)]
    pub fn apt4096ien(&mut self) -> APT4096IEN_W {
        APT4096IEN_W::new(self)
    }
    #[doc = "Bit 7 - IRQ enable for FIFO full"]
    #[inline(always)]
    pub fn fullien(&mut self) -> FULLIEN_W {
        FULLIEN_W::new(self)
    }
    #[doc = "Bit 8 - Software Reset"]
    #[inline(always)]
    pub fn softreset(&mut self) -> SOFTRESET_W {
        SOFTRESET_W::new(self)
    }
    #[doc = "Bit 9 - IRQ enable for AIS31 prelim. noise alarm"]
    #[inline(always)]
    pub fn preien(&mut self) -> PREIEN_W {
        PREIEN_W::new(self)
    }
    #[doc = "Bit 10 - IRQ enable for AIS31 noise alarm"]
    #[inline(always)]
    pub fn almien(&mut self) -> ALMIEN_W {
        ALMIEN_W::new(self)
    }
    #[doc = "Bit 11 - Oscillator Force Run"]
    #[inline(always)]
    pub fn forcerun(&mut self) -> FORCERUN_W {
        FORCERUN_W::new(self)
    }
    #[doc = "Bit 12 - NIST Start-up Test Bypass."]
    #[inline(always)]
    pub fn bypnist(&mut self) -> BYPNIST_W {
        BYPNIST_W::new(self)
    }
    #[doc = "Bit 13 - AIS31 Start-up Test Bypass."]
    #[inline(always)]
    pub fn bypais31(&mut self) -> BYPAIS31_W {
        BYPAIS31_W::new(self)
    }
    #[doc = "Bit 14 - Health test input select"]
    #[inline(always)]
    pub fn healthtestsel(&mut self) -> HEALTHTESTSEL_W {
        HEALTHTESTSEL_W::new(self)
    }
    #[doc = "Bit 15 - AIS31 test input select"]
    #[inline(always)]
    pub fn ais31testsel(&mut self) -> AIS31TESTSEL_W {
        AIS31TESTSEL_W::new(self)
    }
    #[doc = "Bits 16:19 - Number of 128b blocks in AES-CBCMAC"]
    #[inline(always)]
    pub fn nb128bitblocks(&mut self) -> NB128BITBLOCKS_W {
        NB128BITBLOCKS_W::new(self)
    }
    #[doc = "Bit 20 - Fifo Write Start Up"]
    #[inline(always)]
    pub fn fifowrstartup(&mut self) -> FIFOWRSTARTUP_W {
        FIFOWRSTARTUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rngctrl](index.html) module"]
pub struct RNGCTRL_SPEC;
impl crate::RegisterSpec for RNGCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rngctrl::R](R) reader structure"]
impl crate::Readable for RNGCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rngctrl::W](W) writer structure"]
impl crate::Writable for RNGCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RNGCTRL to value 0x0004_0000"]
impl crate::Resettable for RNGCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0004_0000
    }
}
