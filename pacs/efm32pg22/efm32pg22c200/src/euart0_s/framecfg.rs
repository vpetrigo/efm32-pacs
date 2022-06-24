#[doc = "Register `FRAMECFG` reader"]
pub struct R(crate::R<FRAMECFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMECFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMECFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMECFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRAMECFG` writer"]
pub struct W(crate::W<FRAMECFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAMECFG_SPEC>;
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
impl From<crate::W<FRAMECFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAMECFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Data-Bit Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATABITS_A {
    #[doc = "1: Each frame contains 7 data bits"]
    SEVEN = 1,
    #[doc = "2: Each frame contains 8 data bits"]
    EIGHT = 2,
    #[doc = "3: Each frame contains 9 data bits"]
    NINE = 3,
}
impl From<DATABITS_A> for u8 {
    #[inline(always)]
    fn from(variant: DATABITS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DATABITS` reader - Data-Bit Mode"]
pub type DATABITS_R = crate::FieldReader<u8, DATABITS_A>;
impl DATABITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATABITS_A> {
        match self.bits {
            1 => Some(DATABITS_A::SEVEN),
            2 => Some(DATABITS_A::EIGHT),
            3 => Some(DATABITS_A::NINE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SEVEN`"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == DATABITS_A::SEVEN
    }
    #[doc = "Checks if the value of the field is `EIGHT`"]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == DATABITS_A::EIGHT
    }
    #[doc = "Checks if the value of the field is `NINE`"]
    #[inline(always)]
    pub fn is_nine(&self) -> bool {
        *self == DATABITS_A::NINE
    }
}
#[doc = "Field `DATABITS` writer - Data-Bit Mode"]
pub type DATABITS_W<'a> = crate::FieldWriter<'a, u32, FRAMECFG_SPEC, u8, DATABITS_A, 2, 0>;
impl<'a> DATABITS_W<'a> {
    #[doc = "Each frame contains 7 data bits"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut W {
        self.variant(DATABITS_A::SEVEN)
    }
    #[doc = "Each frame contains 8 data bits"]
    #[inline(always)]
    pub fn eight(self) -> &'a mut W {
        self.variant(DATABITS_A::EIGHT)
    }
    #[doc = "Each frame contains 9 data bits"]
    #[inline(always)]
    pub fn nine(self) -> &'a mut W {
        self.variant(DATABITS_A::NINE)
    }
}
#[doc = "Parity-Bit Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PARITY_A {
    #[doc = "0: Parity bits are not used"]
    NONE = 0,
    #[doc = "2: Even parity are used. Parity bits are automatically generated and checked by hardware."]
    EVEN = 2,
    #[doc = "3: Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    ODD = 3,
}
impl From<PARITY_A> for u8 {
    #[inline(always)]
    fn from(variant: PARITY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PARITY` reader - Parity-Bit Mode"]
pub type PARITY_R = crate::FieldReader<u8, PARITY_A>;
impl PARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PARITY_A> {
        match self.bits {
            0 => Some(PARITY_A::NONE),
            2 => Some(PARITY_A::EVEN),
            3 => Some(PARITY_A::ODD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PARITY_A::NONE
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PARITY_A::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PARITY_A::ODD
    }
}
#[doc = "Field `PARITY` writer - Parity-Bit Mode"]
pub type PARITY_W<'a> = crate::FieldWriter<'a, u32, FRAMECFG_SPEC, u8, PARITY_A, 2, 8>;
impl<'a> PARITY_W<'a> {
    #[doc = "Parity bits are not used"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(PARITY_A::NONE)
    }
    #[doc = "Even parity are used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(PARITY_A::EVEN)
    }
    #[doc = "Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(PARITY_A::ODD)
    }
}
#[doc = "Stop-Bit Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STOPBITS_A {
    #[doc = "0: The transmitter generates a half stop bit. Stop-bits are not verified by receiver"]
    HALF = 0,
    #[doc = "1: One stop bit is generated and verified"]
    ONE = 1,
    #[doc = "2: The transmitter generates one and a half stop bit. The receiver verifies the first stop bit"]
    ONEANDAHALF = 2,
    #[doc = "3: The transmitter generates two stop bits. The receiver checks the first stop-bit only"]
    TWO = 3,
}
impl From<STOPBITS_A> for u8 {
    #[inline(always)]
    fn from(variant: STOPBITS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STOPBITS` reader - Stop-Bit Mode"]
pub type STOPBITS_R = crate::FieldReader<u8, STOPBITS_A>;
impl STOPBITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPBITS_A {
        match self.bits {
            0 => STOPBITS_A::HALF,
            1 => STOPBITS_A::ONE,
            2 => STOPBITS_A::ONEANDAHALF,
            3 => STOPBITS_A::TWO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == STOPBITS_A::HALF
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == STOPBITS_A::ONE
    }
    #[doc = "Checks if the value of the field is `ONEANDAHALF`"]
    #[inline(always)]
    pub fn is_oneandahalf(&self) -> bool {
        *self == STOPBITS_A::ONEANDAHALF
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == STOPBITS_A::TWO
    }
}
#[doc = "Field `STOPBITS` writer - Stop-Bit Mode"]
pub type STOPBITS_W<'a> = crate::FieldWriterSafe<'a, u32, FRAMECFG_SPEC, u8, STOPBITS_A, 2, 12>;
impl<'a> STOPBITS_W<'a> {
    #[doc = "The transmitter generates a half stop bit. Stop-bits are not verified by receiver"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(STOPBITS_A::HALF)
    }
    #[doc = "One stop bit is generated and verified"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(STOPBITS_A::ONE)
    }
    #[doc = "The transmitter generates one and a half stop bit. The receiver verifies the first stop bit"]
    #[inline(always)]
    pub fn oneandahalf(self) -> &'a mut W {
        self.variant(STOPBITS_A::ONEANDAHALF)
    }
    #[doc = "The transmitter generates two stop bits. The receiver checks the first stop-bit only"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(STOPBITS_A::TWO)
    }
}
impl R {
    #[doc = "Bits 0:1 - Data-Bit Mode"]
    #[inline(always)]
    pub fn databits(&self) -> DATABITS_R {
        DATABITS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Parity-Bit Mode"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Stop-Bit Mode"]
    #[inline(always)]
    pub fn stopbits(&self) -> STOPBITS_R {
        STOPBITS_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Data-Bit Mode"]
    #[inline(always)]
    pub fn databits(&mut self) -> DATABITS_W {
        DATABITS_W::new(self)
    }
    #[doc = "Bits 8:9 - Parity-Bit Mode"]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W {
        PARITY_W::new(self)
    }
    #[doc = "Bits 12:13 - Stop-Bit Mode"]
    #[inline(always)]
    pub fn stopbits(&mut self) -> STOPBITS_W {
        STOPBITS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [framecfg](index.html) module"]
pub struct FRAMECFG_SPEC;
impl crate::RegisterSpec for FRAMECFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [framecfg::R](R) reader structure"]
impl crate::Readable for FRAMECFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [framecfg::W](W) writer structure"]
impl crate::Writable for FRAMECFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRAMECFG to value 0x1002"]
impl crate::Resettable for FRAMECFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1002
    }
}
