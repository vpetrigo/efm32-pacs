#[doc = "Register `DECCTRL` reader"]
pub struct R(crate::R<DECCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DECCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DECCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DECCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DECCTRL` writer"]
pub struct W(crate::W<DECCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DECCTRL_SPEC>;
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
impl From<crate::W<DECCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DECCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DISABLE` reader - Disable the Decoder"]
pub type DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `DISABLE` writer - Disable the Decoder"]
pub type DISABLE_W<'a> = crate::BitWriter<'a, u32, DECCTRL_SPEC, bool, 0>;
#[doc = "Field `ERRCHK` reader - Enable Check of Current State"]
pub type ERRCHK_R = crate::BitReader<bool>;
#[doc = "Field `ERRCHK` writer - Enable Check of Current State"]
pub type ERRCHK_W<'a> = crate::BitWriter<'a, u32, DECCTRL_SPEC, bool, 1>;
#[doc = "Field `INTMAP` reader - Enable Decoder to Channel Interrupt Mapping"]
pub type INTMAP_R = crate::BitReader<bool>;
#[doc = "Field `INTMAP` writer - Enable Decoder to Channel Interrupt Mapping"]
pub type INTMAP_W<'a> = crate::BitWriter<'a, u32, DECCTRL_SPEC, bool, 2>;
#[doc = "Field `HYSTPRS0` reader - Enable Decoder Hysteresis on PRS0 Output"]
pub type HYSTPRS0_R = crate::BitReader<bool>;
#[doc = "Field `HYSTPRS0` writer - Enable Decoder Hysteresis on PRS0 Output"]
pub type HYSTPRS0_W<'a> = crate::BitWriter<'a, u32, DECCTRL_SPEC, bool, 3>;
#[doc = "Field `HYSTPRS1` reader - Enable Decoder Hysteresis on PRS1 Output"]
pub type HYSTPRS1_R = crate::BitReader<bool>;
#[doc = "Field `HYSTPRS1` writer - Enable Decoder Hysteresis on PRS1 Output"]
pub type HYSTPRS1_W<'a> = crate::BitWriter<'a, u32, DECCTRL_SPEC, bool, 4>;
#[doc = "Field `HYSTPRS2` reader - Enable Decoder Hysteresis on PRS2 Output"]
pub type HYSTPRS2_R = crate::BitReader<bool>;
#[doc = "Field `HYSTPRS2` writer - Enable Decoder Hysteresis on PRS2 Output"]
pub type HYSTPRS2_W<'a> = crate::BitWriter<'a, u32, DECCTRL_SPEC, bool, 5>;
#[doc = "Field `HYSTIRQ` reader - Enable Decoder Hysteresis on Interrupt Requests"]
pub type HYSTIRQ_R = crate::BitReader<bool>;
#[doc = "Field `HYSTIRQ` writer - Enable Decoder Hysteresis on Interrupt Requests"]
pub type HYSTIRQ_W<'a> = crate::BitWriter<'a, u32, DECCTRL_SPEC, bool, 6>;
#[doc = "Field `PRSCNT` reader - Enable Count Mode on Decoder PRS Channels 0 and 1"]
pub type PRSCNT_R = crate::BitReader<bool>;
#[doc = "Field `PRSCNT` writer - Enable Count Mode on Decoder PRS Channels 0 and 1"]
pub type PRSCNT_W<'a> = crate::BitWriter<'a, u32, DECCTRL_SPEC, bool, 7>;
#[doc = "Field `INPUT` reader - LESENSE Decoder Input Configuration"]
pub type INPUT_R = crate::BitReader<bool>;
#[doc = "Field `INPUT` writer - LESENSE Decoder Input Configuration"]
pub type INPUT_W<'a> = crate::BitWriter<'a, u32, DECCTRL_SPEC, bool, 8>;
#[doc = "LESENSE Decoder PRS Input 0 Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRSSEL0_A {
    #[doc = "0: PRS Channel 0 selected as input"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    PRSCH7 = 7,
    #[doc = "8: PRS Channel 8 selected as input"]
    PRSCH8 = 8,
    #[doc = "9: PRS Channel 9 selected as input"]
    PRSCH9 = 9,
    #[doc = "10: PRS Channel 10 selected as input"]
    PRSCH10 = 10,
    #[doc = "11: PRS Channel 11 selected as input"]
    PRSCH11 = 11,
    #[doc = "12: PRS Channel 12 selected as input"]
    PRSCH12 = 12,
    #[doc = "13: PRS Channel 13 selected as input"]
    PRSCH13 = 13,
    #[doc = "14: PRS Channel 14 selected as input"]
    PRSCH14 = 14,
    #[doc = "15: PRS Channel 15 selected as input"]
    PRSCH15 = 15,
}
impl From<PRSSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRSSEL0` reader - LESENSE Decoder PRS Input 0 Configuration"]
pub type PRSSEL0_R = crate::FieldReader<u8, PRSSEL0_A>;
impl PRSSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRSSEL0_A {
        match self.bits {
            0 => PRSSEL0_A::PRSCH0,
            1 => PRSSEL0_A::PRSCH1,
            2 => PRSSEL0_A::PRSCH2,
            3 => PRSSEL0_A::PRSCH3,
            4 => PRSSEL0_A::PRSCH4,
            5 => PRSSEL0_A::PRSCH5,
            6 => PRSSEL0_A::PRSCH6,
            7 => PRSSEL0_A::PRSCH7,
            8 => PRSSEL0_A::PRSCH8,
            9 => PRSSEL0_A::PRSCH9,
            10 => PRSSEL0_A::PRSCH10,
            11 => PRSSEL0_A::PRSCH11,
            12 => PRSSEL0_A::PRSCH12,
            13 => PRSSEL0_A::PRSCH13,
            14 => PRSSEL0_A::PRSCH14,
            15 => PRSSEL0_A::PRSCH15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL0_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL0_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL0_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL0_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL0_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL0_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL0_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL0_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL0_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL0_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL0_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL0_A::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSSEL0_A::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSSEL0_A::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSSEL0_A::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSSEL0_A::PRSCH15
    }
}
#[doc = "Field `PRSSEL0` writer - LESENSE Decoder PRS Input 0 Configuration"]
pub type PRSSEL0_W<'a> = crate::FieldWriterSafe<'a, u32, DECCTRL_SPEC, u8, PRSSEL0_A, 4, 10>;
impl<'a> PRSSEL0_W<'a> {
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(PRSSEL0_A::PRSCH15)
    }
}
#[doc = "LESENSE Decoder PRS Input 1 Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRSSEL1_A {
    #[doc = "0: PRS Channel 0 selected as input"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    PRSCH7 = 7,
    #[doc = "8: PRS Channel 8 selected as input"]
    PRSCH8 = 8,
    #[doc = "9: PRS Channel 9 selected as input"]
    PRSCH9 = 9,
    #[doc = "10: PRS Channel 10 selected as input"]
    PRSCH10 = 10,
    #[doc = "11: PRS Channel 11 selected as input"]
    PRSCH11 = 11,
    #[doc = "12: PRS Channel 12 selected as input"]
    PRSCH12 = 12,
    #[doc = "13: PRS Channel 13 selected as input"]
    PRSCH13 = 13,
    #[doc = "14: PRS Channel 14 selected as input"]
    PRSCH14 = 14,
    #[doc = "15: PRS Channel 15 selected as input"]
    PRSCH15 = 15,
}
impl From<PRSSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRSSEL1` reader - LESENSE Decoder PRS Input 1 Configuration"]
pub type PRSSEL1_R = crate::FieldReader<u8, PRSSEL1_A>;
impl PRSSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRSSEL1_A {
        match self.bits {
            0 => PRSSEL1_A::PRSCH0,
            1 => PRSSEL1_A::PRSCH1,
            2 => PRSSEL1_A::PRSCH2,
            3 => PRSSEL1_A::PRSCH3,
            4 => PRSSEL1_A::PRSCH4,
            5 => PRSSEL1_A::PRSCH5,
            6 => PRSSEL1_A::PRSCH6,
            7 => PRSSEL1_A::PRSCH7,
            8 => PRSSEL1_A::PRSCH8,
            9 => PRSSEL1_A::PRSCH9,
            10 => PRSSEL1_A::PRSCH10,
            11 => PRSSEL1_A::PRSCH11,
            12 => PRSSEL1_A::PRSCH12,
            13 => PRSSEL1_A::PRSCH13,
            14 => PRSSEL1_A::PRSCH14,
            15 => PRSSEL1_A::PRSCH15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL1_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL1_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL1_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL1_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL1_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL1_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL1_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL1_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL1_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL1_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL1_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL1_A::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSSEL1_A::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSSEL1_A::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSSEL1_A::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSSEL1_A::PRSCH15
    }
}
#[doc = "Field `PRSSEL1` writer - LESENSE Decoder PRS Input 1 Configuration"]
pub type PRSSEL1_W<'a> = crate::FieldWriterSafe<'a, u32, DECCTRL_SPEC, u8, PRSSEL1_A, 4, 15>;
impl<'a> PRSSEL1_W<'a> {
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(PRSSEL1_A::PRSCH15)
    }
}
#[doc = "LESENSE Decoder PRS Input 2 Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRSSEL2_A {
    #[doc = "0: PRS Channel 0 selected as input"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    PRSCH7 = 7,
    #[doc = "8: PRS Channel 8 selected as input"]
    PRSCH8 = 8,
    #[doc = "9: PRS Channel 9 selected as input"]
    PRSCH9 = 9,
    #[doc = "10: PRS Channel 10 selected as input"]
    PRSCH10 = 10,
    #[doc = "11: PRS Channel 11 selected as input"]
    PRSCH11 = 11,
    #[doc = "12: PRS Channel 12 selected as input"]
    PRSCH12 = 12,
    #[doc = "13: PRS Channel 13 selected as input"]
    PRSCH13 = 13,
    #[doc = "14: PRS Channel 14 selected as input"]
    PRSCH14 = 14,
    #[doc = "15: PRS Channel 15 selected as input"]
    PRSCH15 = 15,
}
impl From<PRSSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRSSEL2` reader - LESENSE Decoder PRS Input 2 Configuration"]
pub type PRSSEL2_R = crate::FieldReader<u8, PRSSEL2_A>;
impl PRSSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRSSEL2_A {
        match self.bits {
            0 => PRSSEL2_A::PRSCH0,
            1 => PRSSEL2_A::PRSCH1,
            2 => PRSSEL2_A::PRSCH2,
            3 => PRSSEL2_A::PRSCH3,
            4 => PRSSEL2_A::PRSCH4,
            5 => PRSSEL2_A::PRSCH5,
            6 => PRSSEL2_A::PRSCH6,
            7 => PRSSEL2_A::PRSCH7,
            8 => PRSSEL2_A::PRSCH8,
            9 => PRSSEL2_A::PRSCH9,
            10 => PRSSEL2_A::PRSCH10,
            11 => PRSSEL2_A::PRSCH11,
            12 => PRSSEL2_A::PRSCH12,
            13 => PRSSEL2_A::PRSCH13,
            14 => PRSSEL2_A::PRSCH14,
            15 => PRSSEL2_A::PRSCH15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL2_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL2_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL2_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL2_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL2_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL2_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL2_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL2_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL2_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL2_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL2_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL2_A::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSSEL2_A::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSSEL2_A::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSSEL2_A::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSSEL2_A::PRSCH15
    }
}
#[doc = "Field `PRSSEL2` writer - LESENSE Decoder PRS Input 2 Configuration"]
pub type PRSSEL2_W<'a> = crate::FieldWriterSafe<'a, u32, DECCTRL_SPEC, u8, PRSSEL2_A, 4, 20>;
impl<'a> PRSSEL2_W<'a> {
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(PRSSEL2_A::PRSCH15)
    }
}
#[doc = "LESENSE Decoder PRS Input 3 Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRSSEL3_A {
    #[doc = "0: PRS Channel 0 selected as input"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    PRSCH7 = 7,
    #[doc = "8: PRS Channel 8 selected as input"]
    PRSCH8 = 8,
    #[doc = "9: PRS Channel 9 selected as input"]
    PRSCH9 = 9,
    #[doc = "10: PRS Channel 10 selected as input"]
    PRSCH10 = 10,
    #[doc = "11: PRS Channel 11 selected as input"]
    PRSCH11 = 11,
    #[doc = "12: PRS Channel 12 selected as input"]
    PRSCH12 = 12,
    #[doc = "13: PRS Channel 13 selected as input"]
    PRSCH13 = 13,
    #[doc = "14: PRS Channel 14 selected as input"]
    PRSCH14 = 14,
    #[doc = "15: PRS Channel 15 selected as input"]
    PRSCH15 = 15,
}
impl From<PRSSEL3_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRSSEL3` reader - LESENSE Decoder PRS Input 3 Configuration"]
pub type PRSSEL3_R = crate::FieldReader<u8, PRSSEL3_A>;
impl PRSSEL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRSSEL3_A {
        match self.bits {
            0 => PRSSEL3_A::PRSCH0,
            1 => PRSSEL3_A::PRSCH1,
            2 => PRSSEL3_A::PRSCH2,
            3 => PRSSEL3_A::PRSCH3,
            4 => PRSSEL3_A::PRSCH4,
            5 => PRSSEL3_A::PRSCH5,
            6 => PRSSEL3_A::PRSCH6,
            7 => PRSSEL3_A::PRSCH7,
            8 => PRSSEL3_A::PRSCH8,
            9 => PRSSEL3_A::PRSCH9,
            10 => PRSSEL3_A::PRSCH10,
            11 => PRSSEL3_A::PRSCH11,
            12 => PRSSEL3_A::PRSCH12,
            13 => PRSSEL3_A::PRSCH13,
            14 => PRSSEL3_A::PRSCH14,
            15 => PRSSEL3_A::PRSCH15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL3_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL3_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL3_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL3_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL3_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL3_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL3_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL3_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL3_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL3_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL3_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL3_A::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSSEL3_A::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSSEL3_A::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSSEL3_A::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSSEL3_A::PRSCH15
    }
}
#[doc = "Field `PRSSEL3` writer - LESENSE Decoder PRS Input 3 Configuration"]
pub type PRSSEL3_W<'a> = crate::FieldWriterSafe<'a, u32, DECCTRL_SPEC, u8, PRSSEL3_A, 4, 25>;
impl<'a> PRSSEL3_W<'a> {
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(PRSSEL3_A::PRSCH15)
    }
}
impl R {
    #[doc = "Bit 0 - Disable the Decoder"]
    #[inline(always)]
    pub fn disable(&self) -> DISABLE_R {
        DISABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Check of Current State"]
    #[inline(always)]
    pub fn errchk(&self) -> ERRCHK_R {
        ERRCHK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Decoder to Channel Interrupt Mapping"]
    #[inline(always)]
    pub fn intmap(&self) -> INTMAP_R {
        INTMAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Decoder Hysteresis on PRS0 Output"]
    #[inline(always)]
    pub fn hystprs0(&self) -> HYSTPRS0_R {
        HYSTPRS0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Decoder Hysteresis on PRS1 Output"]
    #[inline(always)]
    pub fn hystprs1(&self) -> HYSTPRS1_R {
        HYSTPRS1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Decoder Hysteresis on PRS2 Output"]
    #[inline(always)]
    pub fn hystprs2(&self) -> HYSTPRS2_R {
        HYSTPRS2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Decoder Hysteresis on Interrupt Requests"]
    #[inline(always)]
    pub fn hystirq(&self) -> HYSTIRQ_R {
        HYSTIRQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Count Mode on Decoder PRS Channels 0 and 1"]
    #[inline(always)]
    pub fn prscnt(&self) -> PRSCNT_R {
        PRSCNT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LESENSE Decoder Input Configuration"]
    #[inline(always)]
    pub fn input(&self) -> INPUT_R {
        INPUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:13 - LESENSE Decoder PRS Input 0 Configuration"]
    #[inline(always)]
    pub fn prssel0(&self) -> PRSSEL0_R {
        PRSSEL0_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 15:18 - LESENSE Decoder PRS Input 1 Configuration"]
    #[inline(always)]
    pub fn prssel1(&self) -> PRSSEL1_R {
        PRSSEL1_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - LESENSE Decoder PRS Input 2 Configuration"]
    #[inline(always)]
    pub fn prssel2(&self) -> PRSSEL2_R {
        PRSSEL2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 25:28 - LESENSE Decoder PRS Input 3 Configuration"]
    #[inline(always)]
    pub fn prssel3(&self) -> PRSSEL3_R {
        PRSSEL3_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Disable the Decoder"]
    #[inline(always)]
    pub fn disable(&mut self) -> DISABLE_W {
        DISABLE_W::new(self)
    }
    #[doc = "Bit 1 - Enable Check of Current State"]
    #[inline(always)]
    pub fn errchk(&mut self) -> ERRCHK_W {
        ERRCHK_W::new(self)
    }
    #[doc = "Bit 2 - Enable Decoder to Channel Interrupt Mapping"]
    #[inline(always)]
    pub fn intmap(&mut self) -> INTMAP_W {
        INTMAP_W::new(self)
    }
    #[doc = "Bit 3 - Enable Decoder Hysteresis on PRS0 Output"]
    #[inline(always)]
    pub fn hystprs0(&mut self) -> HYSTPRS0_W {
        HYSTPRS0_W::new(self)
    }
    #[doc = "Bit 4 - Enable Decoder Hysteresis on PRS1 Output"]
    #[inline(always)]
    pub fn hystprs1(&mut self) -> HYSTPRS1_W {
        HYSTPRS1_W::new(self)
    }
    #[doc = "Bit 5 - Enable Decoder Hysteresis on PRS2 Output"]
    #[inline(always)]
    pub fn hystprs2(&mut self) -> HYSTPRS2_W {
        HYSTPRS2_W::new(self)
    }
    #[doc = "Bit 6 - Enable Decoder Hysteresis on Interrupt Requests"]
    #[inline(always)]
    pub fn hystirq(&mut self) -> HYSTIRQ_W {
        HYSTIRQ_W::new(self)
    }
    #[doc = "Bit 7 - Enable Count Mode on Decoder PRS Channels 0 and 1"]
    #[inline(always)]
    pub fn prscnt(&mut self) -> PRSCNT_W {
        PRSCNT_W::new(self)
    }
    #[doc = "Bit 8 - LESENSE Decoder Input Configuration"]
    #[inline(always)]
    pub fn input(&mut self) -> INPUT_W {
        INPUT_W::new(self)
    }
    #[doc = "Bits 10:13 - LESENSE Decoder PRS Input 0 Configuration"]
    #[inline(always)]
    pub fn prssel0(&mut self) -> PRSSEL0_W {
        PRSSEL0_W::new(self)
    }
    #[doc = "Bits 15:18 - LESENSE Decoder PRS Input 1 Configuration"]
    #[inline(always)]
    pub fn prssel1(&mut self) -> PRSSEL1_W {
        PRSSEL1_W::new(self)
    }
    #[doc = "Bits 20:23 - LESENSE Decoder PRS Input 2 Configuration"]
    #[inline(always)]
    pub fn prssel2(&mut self) -> PRSSEL2_W {
        PRSSEL2_W::new(self)
    }
    #[doc = "Bits 25:28 - LESENSE Decoder PRS Input 3 Configuration"]
    #[inline(always)]
    pub fn prssel3(&mut self) -> PRSSEL3_W {
        PRSSEL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Decoder Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [decctrl](index.html) module"]
pub struct DECCTRL_SPEC;
impl crate::RegisterSpec for DECCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [decctrl::R](R) reader structure"]
impl crate::Readable for DECCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [decctrl::W](W) writer structure"]
impl crate::Writable for DECCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DECCTRL to value 0"]
impl crate::Resettable for DECCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
