#[doc = "Register `PRSTVAL0` reader"]
pub struct R(crate::R<PRSTVAL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRSTVAL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRSTVAL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRSTVAL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INITSDCLKFREQVAL` reader - SD_CLK Frequency Select Value for Initialization"]
pub type INITSDCLKFREQVAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INITCLCKGENVAL` reader - Clock Generator Select Value for Initialization"]
pub type INITCLCKGENVAL_R = crate::BitReader<bool>;
#[doc = "Driver Strength Select Value for Initialization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INITDRVSTVAL_A {
    #[doc = "0: Driver Type B is selected (Default)"]
    TYPEB = 0,
    #[doc = "1: Driver Type A is selected"]
    TYPEA = 1,
    #[doc = "2: Driver Type C is selected"]
    TYPEC = 2,
    #[doc = "3: Driver Type D is selected"]
    TYPED = 3,
}
impl From<INITDRVSTVAL_A> for u8 {
    #[inline(always)]
    fn from(variant: INITDRVSTVAL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INITDRVSTVAL` reader - Driver Strength Select Value for Initialization"]
pub type INITDRVSTVAL_R = crate::FieldReader<u8, INITDRVSTVAL_A>;
impl INITDRVSTVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INITDRVSTVAL_A {
        match self.bits {
            0 => INITDRVSTVAL_A::TYPEB,
            1 => INITDRVSTVAL_A::TYPEA,
            2 => INITDRVSTVAL_A::TYPEC,
            3 => INITDRVSTVAL_A::TYPED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TYPEB`"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == INITDRVSTVAL_A::TYPEB
    }
    #[doc = "Checks if the value of the field is `TYPEA`"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == INITDRVSTVAL_A::TYPEA
    }
    #[doc = "Checks if the value of the field is `TYPEC`"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == INITDRVSTVAL_A::TYPEC
    }
    #[doc = "Checks if the value of the field is `TYPED`"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == INITDRVSTVAL_A::TYPED
    }
}
#[doc = "Field `DSPSDCLKFREQVAL` reader - SD_CLK Frequency Select Value for Default Speed"]
pub type DSPSDCLKFREQVAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DSPCLKGENVAL` reader - Clock Generator Select Value for Default Speed"]
pub type DSPCLKGENVAL_R = crate::BitReader<bool>;
#[doc = "Driver Strength Select Value for Default Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DSPDRVSTVAL_A {
    #[doc = "0: Driver Type B is selected (Default)"]
    TYPEB = 0,
    #[doc = "1: Driver Type A is selected"]
    TYPEA = 1,
    #[doc = "2: Driver Type C is selected"]
    TYPEC = 2,
    #[doc = "3: Driver Type D is selected"]
    TYPED = 3,
}
impl From<DSPDRVSTVAL_A> for u8 {
    #[inline(always)]
    fn from(variant: DSPDRVSTVAL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DSPDRVSTVAL` reader - Driver Strength Select Value for Default Speed"]
pub type DSPDRVSTVAL_R = crate::FieldReader<u8, DSPDRVSTVAL_A>;
impl DSPDRVSTVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSPDRVSTVAL_A {
        match self.bits {
            0 => DSPDRVSTVAL_A::TYPEB,
            1 => DSPDRVSTVAL_A::TYPEA,
            2 => DSPDRVSTVAL_A::TYPEC,
            3 => DSPDRVSTVAL_A::TYPED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TYPEB`"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == DSPDRVSTVAL_A::TYPEB
    }
    #[doc = "Checks if the value of the field is `TYPEA`"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == DSPDRVSTVAL_A::TYPEA
    }
    #[doc = "Checks if the value of the field is `TYPEC`"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == DSPDRVSTVAL_A::TYPEC
    }
    #[doc = "Checks if the value of the field is `TYPED`"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == DSPDRVSTVAL_A::TYPED
    }
}
impl R {
    #[doc = "Bits 0:9 - SD_CLK Frequency Select Value for Initialization"]
    #[inline(always)]
    pub fn initsdclkfreqval(&self) -> INITSDCLKFREQVAL_R {
        INITSDCLKFREQVAL_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Clock Generator Select Value for Initialization"]
    #[inline(always)]
    pub fn initclckgenval(&self) -> INITCLCKGENVAL_R {
        INITCLCKGENVAL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Driver Strength Select Value for Initialization"]
    #[inline(always)]
    pub fn initdrvstval(&self) -> INITDRVSTVAL_R {
        INITDRVSTVAL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:25 - SD_CLK Frequency Select Value for Default Speed"]
    #[inline(always)]
    pub fn dspsdclkfreqval(&self) -> DSPSDCLKFREQVAL_R {
        DSPSDCLKFREQVAL_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - Clock Generator Select Value for Default Speed"]
    #[inline(always)]
    pub fn dspclkgenval(&self) -> DSPCLKGENVAL_R {
        DSPCLKGENVAL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Driver Strength Select Value for Default Speed"]
    #[inline(always)]
    pub fn dspdrvstval(&self) -> DSPDRVSTVAL_R {
        DSPDRVSTVAL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "Preset Value for Initialization and Default Speed Mode\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prstval0](index.html) module"]
pub struct PRSTVAL0_SPEC;
impl crate::RegisterSpec for PRSTVAL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prstval0::R](R) reader structure"]
impl crate::Readable for PRSTVAL0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRSTVAL0 to value 0"]
impl crate::Resettable for PRSTVAL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
