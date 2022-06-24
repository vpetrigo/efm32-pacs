#[doc = "Register `PRSTVAL2` reader"]
pub struct R(crate::R<PRSTVAL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRSTVAL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRSTVAL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRSTVAL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HSPSDCLKFREQVAL` reader - SD_CLK Frequency Select Value for High Speed"]
pub type HSPSDCLKFREQVAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HSPCLKGENVAL` reader - Clock Generator Select Value for High Speed"]
pub type HSPCLKGENVAL_R = crate::BitReader<bool>;
#[doc = "Driver Strength Select Value for High Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HSPDRVSTVAL_A {
    #[doc = "0: Driver Type B is selected (Default)"]
    TYPEB = 0,
    #[doc = "1: Driver Type A is selected"]
    TYPEA = 1,
    #[doc = "2: Driver Type C is selected"]
    TYPEC = 2,
    #[doc = "3: Driver Type D is selected"]
    TYPED = 3,
}
impl From<HSPDRVSTVAL_A> for u8 {
    #[inline(always)]
    fn from(variant: HSPDRVSTVAL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HSPDRVSTVAL` reader - Driver Strength Select Value for High Speed"]
pub type HSPDRVSTVAL_R = crate::FieldReader<u8, HSPDRVSTVAL_A>;
impl HSPDRVSTVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSPDRVSTVAL_A {
        match self.bits {
            0 => HSPDRVSTVAL_A::TYPEB,
            1 => HSPDRVSTVAL_A::TYPEA,
            2 => HSPDRVSTVAL_A::TYPEC,
            3 => HSPDRVSTVAL_A::TYPED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TYPEB`"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == HSPDRVSTVAL_A::TYPEB
    }
    #[doc = "Checks if the value of the field is `TYPEA`"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == HSPDRVSTVAL_A::TYPEA
    }
    #[doc = "Checks if the value of the field is `TYPEC`"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == HSPDRVSTVAL_A::TYPEC
    }
    #[doc = "Checks if the value of the field is `TYPED`"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == HSPDRVSTVAL_A::TYPED
    }
}
#[doc = "Field `SDR12SDCLKFREQVAL` reader - SD_CLK Frequency Select Value for SDR12"]
pub type SDR12SDCLKFREQVAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SDR12CLKGENVAL` reader - Clock Generator Select Value for SDR12"]
pub type SDR12CLKGENVAL_R = crate::BitReader<bool>;
#[doc = "Driver Strength Select Value for SDR12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SDR12DRVSTVAL_A {
    #[doc = "0: Driver Type B is selected (Default)"]
    TYPEB = 0,
    #[doc = "1: Driver Type A is selected"]
    TYPEA = 1,
    #[doc = "2: Driver Type C is selected"]
    TYPEC = 2,
    #[doc = "3: Driver Type D is selected"]
    TYPED = 3,
}
impl From<SDR12DRVSTVAL_A> for u8 {
    #[inline(always)]
    fn from(variant: SDR12DRVSTVAL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SDR12DRVSTVAL` reader - Driver Strength Select Value for SDR12"]
pub type SDR12DRVSTVAL_R = crate::FieldReader<u8, SDR12DRVSTVAL_A>;
impl SDR12DRVSTVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDR12DRVSTVAL_A {
        match self.bits {
            0 => SDR12DRVSTVAL_A::TYPEB,
            1 => SDR12DRVSTVAL_A::TYPEA,
            2 => SDR12DRVSTVAL_A::TYPEC,
            3 => SDR12DRVSTVAL_A::TYPED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TYPEB`"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == SDR12DRVSTVAL_A::TYPEB
    }
    #[doc = "Checks if the value of the field is `TYPEA`"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == SDR12DRVSTVAL_A::TYPEA
    }
    #[doc = "Checks if the value of the field is `TYPEC`"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == SDR12DRVSTVAL_A::TYPEC
    }
    #[doc = "Checks if the value of the field is `TYPED`"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == SDR12DRVSTVAL_A::TYPED
    }
}
impl R {
    #[doc = "Bits 0:9 - SD_CLK Frequency Select Value for High Speed"]
    #[inline(always)]
    pub fn hspsdclkfreqval(&self) -> HSPSDCLKFREQVAL_R {
        HSPSDCLKFREQVAL_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Clock Generator Select Value for High Speed"]
    #[inline(always)]
    pub fn hspclkgenval(&self) -> HSPCLKGENVAL_R {
        HSPCLKGENVAL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Driver Strength Select Value for High Speed"]
    #[inline(always)]
    pub fn hspdrvstval(&self) -> HSPDRVSTVAL_R {
        HSPDRVSTVAL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:25 - SD_CLK Frequency Select Value for SDR12"]
    #[inline(always)]
    pub fn sdr12sdclkfreqval(&self) -> SDR12SDCLKFREQVAL_R {
        SDR12SDCLKFREQVAL_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - Clock Generator Select Value for SDR12"]
    #[inline(always)]
    pub fn sdr12clkgenval(&self) -> SDR12CLKGENVAL_R {
        SDR12CLKGENVAL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Driver Strength Select Value for SDR12"]
    #[inline(always)]
    pub fn sdr12drvstval(&self) -> SDR12DRVSTVAL_R {
        SDR12DRVSTVAL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "Preset Value for High Speed and SDR12 Modes\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prstval2](index.html) module"]
pub struct PRSTVAL2_SPEC;
impl crate::RegisterSpec for PRSTVAL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prstval2::R](R) reader structure"]
impl crate::Readable for PRSTVAL2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRSTVAL2 to value 0"]
impl crate::Resettable for PRSTVAL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
