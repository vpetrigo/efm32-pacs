#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Current DAC Enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Current DAC Enable"]
pub type EN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 0>;
#[doc = "Field `CURSINK` reader - Current Sink Enable"]
pub type CURSINK_R = crate::BitReader<bool>;
#[doc = "Field `CURSINK` writer - Current Sink Enable"]
pub type CURSINK_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 1>;
#[doc = "Field `MINOUTTRANS` reader - Minimum Output Transition Enable"]
pub type MINOUTTRANS_R = crate::BitReader<bool>;
#[doc = "Field `MINOUTTRANS` writer - Minimum Output Transition Enable"]
pub type MINOUTTRANS_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 2>;
#[doc = "Field `APORTOUTEN` reader - APORT Output Enable"]
pub type APORTOUTEN_R = crate::BitReader<bool>;
#[doc = "Field `APORTOUTEN` writer - APORT Output Enable"]
pub type APORTOUTEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 3>;
#[doc = "APORT Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum APORTOUTSEL_A {
    #[doc = "32: APORT1X Channel 0"]
    APORT1XCH0 = 32,
    #[doc = "33: APORT1Y Channel 1"]
    APORT1YCH1 = 33,
    #[doc = "34: APORT1X Channel 2"]
    APORT1XCH2 = 34,
    #[doc = "35: APORT1Y Channel 3"]
    APORT1YCH3 = 35,
    #[doc = "36: APORT1X Channel 4"]
    APORT1XCH4 = 36,
    #[doc = "37: APORT1Y Channel 5"]
    APORT1YCH5 = 37,
    #[doc = "38: APORT1X Channel 6"]
    APORT1XCH6 = 38,
    #[doc = "39: APORT1Y Channel 7"]
    APORT1YCH7 = 39,
    #[doc = "40: APORT1X Channel 8"]
    APORT1XCH8 = 40,
    #[doc = "41: APORT1Y Channel 9"]
    APORT1YCH9 = 41,
    #[doc = "42: APORT1X Channel 10"]
    APORT1XCH10 = 42,
    #[doc = "43: APORT1Y Channel 11"]
    APORT1YCH11 = 43,
    #[doc = "44: APORT1X Channel 12"]
    APORT1XCH12 = 44,
    #[doc = "45: APORT1Y Channel 13"]
    APORT1YCH13 = 45,
    #[doc = "46: APORT1X Channel 14"]
    APORT1XCH14 = 46,
    #[doc = "47: APORT1Y Channel 15"]
    APORT1YCH15 = 47,
    #[doc = "48: APORT1X Channel 16"]
    APORT1XCH16 = 48,
    #[doc = "49: APORT1Y Channel 17"]
    APORT1YCH17 = 49,
    #[doc = "50: APORT1X Channel 18"]
    APORT1XCH18 = 50,
    #[doc = "51: APORT1Y Channel 19"]
    APORT1YCH19 = 51,
    #[doc = "52: APORT1X Channel 20"]
    APORT1XCH20 = 52,
    #[doc = "53: APORT1Y Channel 21"]
    APORT1YCH21 = 53,
    #[doc = "54: APORT1X Channel 22"]
    APORT1XCH22 = 54,
    #[doc = "55: APORT1Y Channel 23"]
    APORT1YCH23 = 55,
    #[doc = "56: APORT1X Channel 24"]
    APORT1XCH24 = 56,
    #[doc = "57: APORT1Y Channel 25"]
    APORT1YCH25 = 57,
    #[doc = "58: APORT1X Channel 26"]
    APORT1XCH26 = 58,
    #[doc = "59: APORT1Y Channel 27"]
    APORT1YCH27 = 59,
    #[doc = "60: APORT1X Channel 28"]
    APORT1XCH28 = 60,
    #[doc = "61: APORT1Y Channel 29"]
    APORT1YCH29 = 61,
    #[doc = "62: APORT1X Channel 30"]
    APORT1XCH30 = 62,
    #[doc = "63: APORT1Y Channel 31"]
    APORT1YCH31 = 63,
}
impl From<APORTOUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: APORTOUTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `APORTOUTSEL` reader - APORT Output Select"]
pub type APORTOUTSEL_R = crate::FieldReader<u8, APORTOUTSEL_A>;
impl APORTOUTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<APORTOUTSEL_A> {
        match self.bits {
            32 => Some(APORTOUTSEL_A::APORT1XCH0),
            33 => Some(APORTOUTSEL_A::APORT1YCH1),
            34 => Some(APORTOUTSEL_A::APORT1XCH2),
            35 => Some(APORTOUTSEL_A::APORT1YCH3),
            36 => Some(APORTOUTSEL_A::APORT1XCH4),
            37 => Some(APORTOUTSEL_A::APORT1YCH5),
            38 => Some(APORTOUTSEL_A::APORT1XCH6),
            39 => Some(APORTOUTSEL_A::APORT1YCH7),
            40 => Some(APORTOUTSEL_A::APORT1XCH8),
            41 => Some(APORTOUTSEL_A::APORT1YCH9),
            42 => Some(APORTOUTSEL_A::APORT1XCH10),
            43 => Some(APORTOUTSEL_A::APORT1YCH11),
            44 => Some(APORTOUTSEL_A::APORT1XCH12),
            45 => Some(APORTOUTSEL_A::APORT1YCH13),
            46 => Some(APORTOUTSEL_A::APORT1XCH14),
            47 => Some(APORTOUTSEL_A::APORT1YCH15),
            48 => Some(APORTOUTSEL_A::APORT1XCH16),
            49 => Some(APORTOUTSEL_A::APORT1YCH17),
            50 => Some(APORTOUTSEL_A::APORT1XCH18),
            51 => Some(APORTOUTSEL_A::APORT1YCH19),
            52 => Some(APORTOUTSEL_A::APORT1XCH20),
            53 => Some(APORTOUTSEL_A::APORT1YCH21),
            54 => Some(APORTOUTSEL_A::APORT1XCH22),
            55 => Some(APORTOUTSEL_A::APORT1YCH23),
            56 => Some(APORTOUTSEL_A::APORT1XCH24),
            57 => Some(APORTOUTSEL_A::APORT1YCH25),
            58 => Some(APORTOUTSEL_A::APORT1XCH26),
            59 => Some(APORTOUTSEL_A::APORT1YCH27),
            60 => Some(APORTOUTSEL_A::APORT1XCH28),
            61 => Some(APORTOUTSEL_A::APORT1YCH29),
            62 => Some(APORTOUTSEL_A::APORT1XCH30),
            63 => Some(APORTOUTSEL_A::APORT1YCH31),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `APORT1XCH0`"]
    #[inline(always)]
    pub fn is_aport1xch0(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH0
    }
    #[doc = "Checks if the value of the field is `APORT1YCH1`"]
    #[inline(always)]
    pub fn is_aport1ych1(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH1
    }
    #[doc = "Checks if the value of the field is `APORT1XCH2`"]
    #[inline(always)]
    pub fn is_aport1xch2(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH2
    }
    #[doc = "Checks if the value of the field is `APORT1YCH3`"]
    #[inline(always)]
    pub fn is_aport1ych3(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH3
    }
    #[doc = "Checks if the value of the field is `APORT1XCH4`"]
    #[inline(always)]
    pub fn is_aport1xch4(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH4
    }
    #[doc = "Checks if the value of the field is `APORT1YCH5`"]
    #[inline(always)]
    pub fn is_aport1ych5(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH5
    }
    #[doc = "Checks if the value of the field is `APORT1XCH6`"]
    #[inline(always)]
    pub fn is_aport1xch6(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH6
    }
    #[doc = "Checks if the value of the field is `APORT1YCH7`"]
    #[inline(always)]
    pub fn is_aport1ych7(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH7
    }
    #[doc = "Checks if the value of the field is `APORT1XCH8`"]
    #[inline(always)]
    pub fn is_aport1xch8(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH8
    }
    #[doc = "Checks if the value of the field is `APORT1YCH9`"]
    #[inline(always)]
    pub fn is_aport1ych9(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH9
    }
    #[doc = "Checks if the value of the field is `APORT1XCH10`"]
    #[inline(always)]
    pub fn is_aport1xch10(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH10
    }
    #[doc = "Checks if the value of the field is `APORT1YCH11`"]
    #[inline(always)]
    pub fn is_aport1ych11(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH11
    }
    #[doc = "Checks if the value of the field is `APORT1XCH12`"]
    #[inline(always)]
    pub fn is_aport1xch12(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH12
    }
    #[doc = "Checks if the value of the field is `APORT1YCH13`"]
    #[inline(always)]
    pub fn is_aport1ych13(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH13
    }
    #[doc = "Checks if the value of the field is `APORT1XCH14`"]
    #[inline(always)]
    pub fn is_aport1xch14(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH14
    }
    #[doc = "Checks if the value of the field is `APORT1YCH15`"]
    #[inline(always)]
    pub fn is_aport1ych15(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH15
    }
    #[doc = "Checks if the value of the field is `APORT1XCH16`"]
    #[inline(always)]
    pub fn is_aport1xch16(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH16
    }
    #[doc = "Checks if the value of the field is `APORT1YCH17`"]
    #[inline(always)]
    pub fn is_aport1ych17(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH17
    }
    #[doc = "Checks if the value of the field is `APORT1XCH18`"]
    #[inline(always)]
    pub fn is_aport1xch18(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH18
    }
    #[doc = "Checks if the value of the field is `APORT1YCH19`"]
    #[inline(always)]
    pub fn is_aport1ych19(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH19
    }
    #[doc = "Checks if the value of the field is `APORT1XCH20`"]
    #[inline(always)]
    pub fn is_aport1xch20(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH20
    }
    #[doc = "Checks if the value of the field is `APORT1YCH21`"]
    #[inline(always)]
    pub fn is_aport1ych21(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH21
    }
    #[doc = "Checks if the value of the field is `APORT1XCH22`"]
    #[inline(always)]
    pub fn is_aport1xch22(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH22
    }
    #[doc = "Checks if the value of the field is `APORT1YCH23`"]
    #[inline(always)]
    pub fn is_aport1ych23(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH23
    }
    #[doc = "Checks if the value of the field is `APORT1XCH24`"]
    #[inline(always)]
    pub fn is_aport1xch24(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH24
    }
    #[doc = "Checks if the value of the field is `APORT1YCH25`"]
    #[inline(always)]
    pub fn is_aport1ych25(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH25
    }
    #[doc = "Checks if the value of the field is `APORT1XCH26`"]
    #[inline(always)]
    pub fn is_aport1xch26(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH26
    }
    #[doc = "Checks if the value of the field is `APORT1YCH27`"]
    #[inline(always)]
    pub fn is_aport1ych27(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH27
    }
    #[doc = "Checks if the value of the field is `APORT1XCH28`"]
    #[inline(always)]
    pub fn is_aport1xch28(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH28
    }
    #[doc = "Checks if the value of the field is `APORT1YCH29`"]
    #[inline(always)]
    pub fn is_aport1ych29(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH29
    }
    #[doc = "Checks if the value of the field is `APORT1XCH30`"]
    #[inline(always)]
    pub fn is_aport1xch30(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1XCH30
    }
    #[doc = "Checks if the value of the field is `APORT1YCH31`"]
    #[inline(always)]
    pub fn is_aport1ych31(&self) -> bool {
        *self == APORTOUTSEL_A::APORT1YCH31
    }
}
#[doc = "Field `APORTOUTSEL` writer - APORT Output Select"]
pub type APORTOUTSEL_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, APORTOUTSEL_A, 8, 4>;
impl<'a> APORTOUTSEL_W<'a> {
    #[doc = "APORT1X Channel 0"]
    #[inline(always)]
    pub fn aport1xch0(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH0)
    }
    #[doc = "APORT1Y Channel 1"]
    #[inline(always)]
    pub fn aport1ych1(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH1)
    }
    #[doc = "APORT1X Channel 2"]
    #[inline(always)]
    pub fn aport1xch2(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH2)
    }
    #[doc = "APORT1Y Channel 3"]
    #[inline(always)]
    pub fn aport1ych3(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH3)
    }
    #[doc = "APORT1X Channel 4"]
    #[inline(always)]
    pub fn aport1xch4(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH4)
    }
    #[doc = "APORT1Y Channel 5"]
    #[inline(always)]
    pub fn aport1ych5(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH5)
    }
    #[doc = "APORT1X Channel 6"]
    #[inline(always)]
    pub fn aport1xch6(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH6)
    }
    #[doc = "APORT1Y Channel 7"]
    #[inline(always)]
    pub fn aport1ych7(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH7)
    }
    #[doc = "APORT1X Channel 8"]
    #[inline(always)]
    pub fn aport1xch8(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH8)
    }
    #[doc = "APORT1Y Channel 9"]
    #[inline(always)]
    pub fn aport1ych9(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH9)
    }
    #[doc = "APORT1X Channel 10"]
    #[inline(always)]
    pub fn aport1xch10(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH10)
    }
    #[doc = "APORT1Y Channel 11"]
    #[inline(always)]
    pub fn aport1ych11(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH11)
    }
    #[doc = "APORT1X Channel 12"]
    #[inline(always)]
    pub fn aport1xch12(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH12)
    }
    #[doc = "APORT1Y Channel 13"]
    #[inline(always)]
    pub fn aport1ych13(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH13)
    }
    #[doc = "APORT1X Channel 14"]
    #[inline(always)]
    pub fn aport1xch14(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH14)
    }
    #[doc = "APORT1Y Channel 15"]
    #[inline(always)]
    pub fn aport1ych15(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH15)
    }
    #[doc = "APORT1X Channel 16"]
    #[inline(always)]
    pub fn aport1xch16(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH16)
    }
    #[doc = "APORT1Y Channel 17"]
    #[inline(always)]
    pub fn aport1ych17(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH17)
    }
    #[doc = "APORT1X Channel 18"]
    #[inline(always)]
    pub fn aport1xch18(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH18)
    }
    #[doc = "APORT1Y Channel 19"]
    #[inline(always)]
    pub fn aport1ych19(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH19)
    }
    #[doc = "APORT1X Channel 20"]
    #[inline(always)]
    pub fn aport1xch20(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH20)
    }
    #[doc = "APORT1Y Channel 21"]
    #[inline(always)]
    pub fn aport1ych21(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH21)
    }
    #[doc = "APORT1X Channel 22"]
    #[inline(always)]
    pub fn aport1xch22(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH22)
    }
    #[doc = "APORT1Y Channel 23"]
    #[inline(always)]
    pub fn aport1ych23(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH23)
    }
    #[doc = "APORT1X Channel 24"]
    #[inline(always)]
    pub fn aport1xch24(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH24)
    }
    #[doc = "APORT1Y Channel 25"]
    #[inline(always)]
    pub fn aport1ych25(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH25)
    }
    #[doc = "APORT1X Channel 26"]
    #[inline(always)]
    pub fn aport1xch26(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH26)
    }
    #[doc = "APORT1Y Channel 27"]
    #[inline(always)]
    pub fn aport1ych27(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH27)
    }
    #[doc = "APORT1X Channel 28"]
    #[inline(always)]
    pub fn aport1xch28(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH28)
    }
    #[doc = "APORT1Y Channel 29"]
    #[inline(always)]
    pub fn aport1ych29(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH29)
    }
    #[doc = "APORT1X Channel 30"]
    #[inline(always)]
    pub fn aport1xch30(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1XCH30)
    }
    #[doc = "APORT1Y Channel 31"]
    #[inline(always)]
    pub fn aport1ych31(self) -> &'a mut W {
        self.variant(APORTOUTSEL_A::APORT1YCH31)
    }
}
#[doc = "Field `PWRSEL` reader - Power Select"]
pub type PWRSEL_R = crate::BitReader<bool>;
#[doc = "Field `PWRSEL` writer - Power Select"]
pub type PWRSEL_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 12>;
#[doc = "Field `EM2DELAY` reader - EM2 Delay"]
pub type EM2DELAY_R = crate::BitReader<bool>;
#[doc = "Field `EM2DELAY` writer - EM2 Delay"]
pub type EM2DELAY_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 13>;
#[doc = "Field `APORTMASTERDIS` reader - APORT Bus Master Disable"]
pub type APORTMASTERDIS_R = crate::BitReader<bool>;
#[doc = "Field `APORTMASTERDIS` writer - APORT Bus Master Disable"]
pub type APORTMASTERDIS_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 14>;
#[doc = "Field `APORTOUTENPRS` reader - PRS Controlled APORT Output Enable"]
pub type APORTOUTENPRS_R = crate::BitReader<bool>;
#[doc = "Field `APORTOUTENPRS` writer - PRS Controlled APORT Output Enable"]
pub type APORTOUTENPRS_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 16>;
#[doc = "IDAC Output Enable PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRSSEL_A {
    #[doc = "0: PRS Channel 0 selected."]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected."]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected."]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected."]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected."]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected."]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected."]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected."]
    PRSCH7 = 7,
    #[doc = "8: PRS Channel 8 selected."]
    PRSCH8 = 8,
    #[doc = "9: PRS Channel 9 selected."]
    PRSCH9 = 9,
    #[doc = "10: PRS Channel 10 selected."]
    PRSCH10 = 10,
    #[doc = "11: PRS Channel 11 selected."]
    PRSCH11 = 11,
}
impl From<PRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRSSEL` reader - IDAC Output Enable PRS Channel Select"]
pub type PRSSEL_R = crate::FieldReader<u8, PRSSEL_A>;
impl PRSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRSSEL_A> {
        match self.bits {
            0 => Some(PRSSEL_A::PRSCH0),
            1 => Some(PRSSEL_A::PRSCH1),
            2 => Some(PRSSEL_A::PRSCH2),
            3 => Some(PRSSEL_A::PRSCH3),
            4 => Some(PRSSEL_A::PRSCH4),
            5 => Some(PRSSEL_A::PRSCH5),
            6 => Some(PRSSEL_A::PRSCH6),
            7 => Some(PRSSEL_A::PRSCH7),
            8 => Some(PRSSEL_A::PRSCH8),
            9 => Some(PRSSEL_A::PRSCH9),
            10 => Some(PRSSEL_A::PRSCH10),
            11 => Some(PRSSEL_A::PRSCH11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL_A::PRSCH11
    }
}
#[doc = "Field `PRSSEL` writer - IDAC Output Enable PRS Channel Select"]
pub type PRSSEL_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, PRSSEL_A, 4, 20>;
impl<'a> PRSSEL_W<'a> {
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected."]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected."]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected."]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected."]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected."]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected."]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH11)
    }
}
impl R {
    #[doc = "Bit 0 - Current DAC Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Current Sink Enable"]
    #[inline(always)]
    pub fn cursink(&self) -> CURSINK_R {
        CURSINK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Minimum Output Transition Enable"]
    #[inline(always)]
    pub fn minouttrans(&self) -> MINOUTTRANS_R {
        MINOUTTRANS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - APORT Output Enable"]
    #[inline(always)]
    pub fn aportouten(&self) -> APORTOUTEN_R {
        APORTOUTEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11 - APORT Output Select"]
    #[inline(always)]
    pub fn aportoutsel(&self) -> APORTOUTSEL_R {
        APORTOUTSEL_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 12 - Power Select"]
    #[inline(always)]
    pub fn pwrsel(&self) -> PWRSEL_R {
        PWRSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EM2 Delay"]
    #[inline(always)]
    pub fn em2delay(&self) -> EM2DELAY_R {
        EM2DELAY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - APORT Bus Master Disable"]
    #[inline(always)]
    pub fn aportmasterdis(&self) -> APORTMASTERDIS_R {
        APORTMASTERDIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - PRS Controlled APORT Output Enable"]
    #[inline(always)]
    pub fn aportoutenprs(&self) -> APORTOUTENPRS_R {
        APORTOUTENPRS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:23 - IDAC Output Enable PRS Channel Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Current DAC Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Current Sink Enable"]
    #[inline(always)]
    pub fn cursink(&mut self) -> CURSINK_W {
        CURSINK_W::new(self)
    }
    #[doc = "Bit 2 - Minimum Output Transition Enable"]
    #[inline(always)]
    pub fn minouttrans(&mut self) -> MINOUTTRANS_W {
        MINOUTTRANS_W::new(self)
    }
    #[doc = "Bit 3 - APORT Output Enable"]
    #[inline(always)]
    pub fn aportouten(&mut self) -> APORTOUTEN_W {
        APORTOUTEN_W::new(self)
    }
    #[doc = "Bits 4:11 - APORT Output Select"]
    #[inline(always)]
    pub fn aportoutsel(&mut self) -> APORTOUTSEL_W {
        APORTOUTSEL_W::new(self)
    }
    #[doc = "Bit 12 - Power Select"]
    #[inline(always)]
    pub fn pwrsel(&mut self) -> PWRSEL_W {
        PWRSEL_W::new(self)
    }
    #[doc = "Bit 13 - EM2 Delay"]
    #[inline(always)]
    pub fn em2delay(&mut self) -> EM2DELAY_W {
        EM2DELAY_W::new(self)
    }
    #[doc = "Bit 14 - APORT Bus Master Disable"]
    #[inline(always)]
    pub fn aportmasterdis(&mut self) -> APORTMASTERDIS_W {
        APORTMASTERDIS_W::new(self)
    }
    #[doc = "Bit 16 - PRS Controlled APORT Output Enable"]
    #[inline(always)]
    pub fn aportoutenprs(&mut self) -> APORTOUTENPRS_W {
        APORTOUTENPRS_W::new(self)
    }
    #[doc = "Bits 20:23 - IDAC Output Enable PRS Channel Select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PRSSEL_W {
        PRSSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
