#[doc = "Register `TRIGCTRL` reader"]
pub struct R(crate::R<TRIGCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIGCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIGCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIGCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIGCTRL` writer"]
pub struct W(crate::W<TRIGCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIGCTRL_SPEC>;
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
impl From<crate::W<TRIGCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIGCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXTEN` reader - Receive Trigger Enable"]
pub type RXTEN_R = crate::BitReader<bool>;
#[doc = "Field `RXTEN` writer - Receive Trigger Enable"]
pub type RXTEN_W<'a> = crate::BitWriter<'a, u32, TRIGCTRL_SPEC, bool, 4>;
#[doc = "Field `TXTEN` reader - Transmit Trigger Enable"]
pub type TXTEN_R = crate::BitReader<bool>;
#[doc = "Field `TXTEN` writer - Transmit Trigger Enable"]
pub type TXTEN_W<'a> = crate::BitWriter<'a, u32, TRIGCTRL_SPEC, bool, 5>;
#[doc = "Field `AUTOTXTEN` reader - AUTOTX Trigger Enable"]
pub type AUTOTXTEN_R = crate::BitReader<bool>;
#[doc = "Field `AUTOTXTEN` writer - AUTOTX Trigger Enable"]
pub type AUTOTXTEN_W<'a> = crate::BitWriter<'a, u32, TRIGCTRL_SPEC, bool, 6>;
#[doc = "Field `TXARX0EN` reader - Enable Transmit Trigger After RX End of Frame Plus TCMP0VAL"]
pub type TXARX0EN_R = crate::BitReader<bool>;
#[doc = "Field `TXARX0EN` writer - Enable Transmit Trigger After RX End of Frame Plus TCMP0VAL"]
pub type TXARX0EN_W<'a> = crate::BitWriter<'a, u32, TRIGCTRL_SPEC, bool, 7>;
#[doc = "Field `TXARX1EN` reader - Enable Transmit Trigger After RX End of Frame Plus TCMP1VAL"]
pub type TXARX1EN_R = crate::BitReader<bool>;
#[doc = "Field `TXARX1EN` writer - Enable Transmit Trigger After RX End of Frame Plus TCMP1VAL"]
pub type TXARX1EN_W<'a> = crate::BitWriter<'a, u32, TRIGCTRL_SPEC, bool, 8>;
#[doc = "Field `TXARX2EN` reader - Enable Transmit Trigger After RX End of Frame Plus TCMP2VAL"]
pub type TXARX2EN_R = crate::BitReader<bool>;
#[doc = "Field `TXARX2EN` writer - Enable Transmit Trigger After RX End of Frame Plus TCMP2VAL"]
pub type TXARX2EN_W<'a> = crate::BitWriter<'a, u32, TRIGCTRL_SPEC, bool, 9>;
#[doc = "Field `RXATX0EN` reader - Enable Receive Trigger After TX End of Frame Plus TCMPVAL0 Baud-times"]
pub type RXATX0EN_R = crate::BitReader<bool>;
#[doc = "Field `RXATX0EN` writer - Enable Receive Trigger After TX End of Frame Plus TCMPVAL0 Baud-times"]
pub type RXATX0EN_W<'a> = crate::BitWriter<'a, u32, TRIGCTRL_SPEC, bool, 10>;
#[doc = "Field `RXATX1EN` reader - Enable Receive Trigger After TX End of Frame Plus TCMPVAL1 Baud-times"]
pub type RXATX1EN_R = crate::BitReader<bool>;
#[doc = "Field `RXATX1EN` writer - Enable Receive Trigger After TX End of Frame Plus TCMPVAL1 Baud-times"]
pub type RXATX1EN_W<'a> = crate::BitWriter<'a, u32, TRIGCTRL_SPEC, bool, 11>;
#[doc = "Field `RXATX2EN` reader - Enable Receive Trigger After TX End of Frame Plus TCMPVAL2 Baud-times"]
pub type RXATX2EN_R = crate::BitReader<bool>;
#[doc = "Field `RXATX2EN` writer - Enable Receive Trigger After TX End of Frame Plus TCMPVAL2 Baud-times"]
pub type RXATX2EN_W<'a> = crate::BitWriter<'a, u32, TRIGCTRL_SPEC, bool, 12>;
#[doc = "Trigger PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSEL_A {
    #[doc = "0: PRS Channel 0 selected"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected"]
    PRSCH7 = 7,
    #[doc = "8: PRS Channel 8 selected"]
    PRSCH8 = 8,
    #[doc = "9: PRS Channel 9 selected"]
    PRSCH9 = 9,
    #[doc = "10: PRS Channel 10 selected"]
    PRSCH10 = 10,
    #[doc = "11: PRS Channel 11 selected"]
    PRSCH11 = 11,
    #[doc = "12: PRS Channel 12 selected"]
    PRSCH12 = 12,
    #[doc = "13: PRS Channel 13 selected"]
    PRSCH13 = 13,
    #[doc = "14: PRS Channel 14 selected"]
    PRSCH14 = 14,
    #[doc = "15: PRS Channel 15 selected"]
    PRSCH15 = 15,
    #[doc = "16: PRS Channel 16 selected"]
    PRSCH16 = 16,
    #[doc = "17: PRS Channel 17 selected"]
    PRSCH17 = 17,
    #[doc = "18: PRS Channel 18 selected"]
    PRSCH18 = 18,
    #[doc = "19: PRS Channel 19 selected"]
    PRSCH19 = 19,
    #[doc = "20: PRS Channel 20 selected"]
    PRSCH20 = 20,
    #[doc = "21: PRS Channel 21 selected"]
    PRSCH21 = 21,
    #[doc = "22: PRS Channel 22 selected"]
    PRSCH22 = 22,
    #[doc = "23: PRS Channel 23 selected"]
    PRSCH23 = 23,
}
impl From<TSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TSEL` reader - Trigger PRS Channel Select"]
pub type TSEL_R = crate::FieldReader<u8, TSEL_A>;
impl TSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSEL_A> {
        match self.bits {
            0 => Some(TSEL_A::PRSCH0),
            1 => Some(TSEL_A::PRSCH1),
            2 => Some(TSEL_A::PRSCH2),
            3 => Some(TSEL_A::PRSCH3),
            4 => Some(TSEL_A::PRSCH4),
            5 => Some(TSEL_A::PRSCH5),
            6 => Some(TSEL_A::PRSCH6),
            7 => Some(TSEL_A::PRSCH7),
            8 => Some(TSEL_A::PRSCH8),
            9 => Some(TSEL_A::PRSCH9),
            10 => Some(TSEL_A::PRSCH10),
            11 => Some(TSEL_A::PRSCH11),
            12 => Some(TSEL_A::PRSCH12),
            13 => Some(TSEL_A::PRSCH13),
            14 => Some(TSEL_A::PRSCH14),
            15 => Some(TSEL_A::PRSCH15),
            16 => Some(TSEL_A::PRSCH16),
            17 => Some(TSEL_A::PRSCH17),
            18 => Some(TSEL_A::PRSCH18),
            19 => Some(TSEL_A::PRSCH19),
            20 => Some(TSEL_A::PRSCH20),
            21 => Some(TSEL_A::PRSCH21),
            22 => Some(TSEL_A::PRSCH22),
            23 => Some(TSEL_A::PRSCH23),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == TSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == TSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == TSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == TSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == TSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == TSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == TSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == TSEL_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == TSEL_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == TSEL_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == TSEL_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == TSEL_A::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == TSEL_A::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == TSEL_A::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == TSEL_A::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == TSEL_A::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == TSEL_A::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == TSEL_A::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == TSEL_A::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == TSEL_A::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == TSEL_A::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == TSEL_A::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == TSEL_A::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == TSEL_A::PRSCH23
    }
}
#[doc = "Field `TSEL` writer - Trigger PRS Channel Select"]
pub type TSEL_W<'a> = crate::FieldWriter<'a, u32, TRIGCTRL_SPEC, u8, TSEL_A, 5, 16>;
impl<'a> TSEL_W<'a> {
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(TSEL_A::PRSCH23)
    }
}
impl R {
    #[doc = "Bit 4 - Receive Trigger Enable"]
    #[inline(always)]
    pub fn rxten(&self) -> RXTEN_R {
        RXTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Trigger Enable"]
    #[inline(always)]
    pub fn txten(&self) -> TXTEN_R {
        TXTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AUTOTX Trigger Enable"]
    #[inline(always)]
    pub fn autotxten(&self) -> AUTOTXTEN_R {
        AUTOTXTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Transmit Trigger After RX End of Frame Plus TCMP0VAL"]
    #[inline(always)]
    pub fn txarx0en(&self) -> TXARX0EN_R {
        TXARX0EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Transmit Trigger After RX End of Frame Plus TCMP1VAL"]
    #[inline(always)]
    pub fn txarx1en(&self) -> TXARX1EN_R {
        TXARX1EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Transmit Trigger After RX End of Frame Plus TCMP2VAL"]
    #[inline(always)]
    pub fn txarx2en(&self) -> TXARX2EN_R {
        TXARX2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Receive Trigger After TX End of Frame Plus TCMPVAL0 Baud-times"]
    #[inline(always)]
    pub fn rxatx0en(&self) -> RXATX0EN_R {
        RXATX0EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Receive Trigger After TX End of Frame Plus TCMPVAL1 Baud-times"]
    #[inline(always)]
    pub fn rxatx1en(&self) -> RXATX1EN_R {
        RXATX1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Receive Trigger After TX End of Frame Plus TCMPVAL2 Baud-times"]
    #[inline(always)]
    pub fn rxatx2en(&self) -> RXATX2EN_R {
        RXATX2EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Trigger PRS Channel Select"]
    #[inline(always)]
    pub fn tsel(&self) -> TSEL_R {
        TSEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - Receive Trigger Enable"]
    #[inline(always)]
    pub fn rxten(&mut self) -> RXTEN_W {
        RXTEN_W::new(self)
    }
    #[doc = "Bit 5 - Transmit Trigger Enable"]
    #[inline(always)]
    pub fn txten(&mut self) -> TXTEN_W {
        TXTEN_W::new(self)
    }
    #[doc = "Bit 6 - AUTOTX Trigger Enable"]
    #[inline(always)]
    pub fn autotxten(&mut self) -> AUTOTXTEN_W {
        AUTOTXTEN_W::new(self)
    }
    #[doc = "Bit 7 - Enable Transmit Trigger After RX End of Frame Plus TCMP0VAL"]
    #[inline(always)]
    pub fn txarx0en(&mut self) -> TXARX0EN_W {
        TXARX0EN_W::new(self)
    }
    #[doc = "Bit 8 - Enable Transmit Trigger After RX End of Frame Plus TCMP1VAL"]
    #[inline(always)]
    pub fn txarx1en(&mut self) -> TXARX1EN_W {
        TXARX1EN_W::new(self)
    }
    #[doc = "Bit 9 - Enable Transmit Trigger After RX End of Frame Plus TCMP2VAL"]
    #[inline(always)]
    pub fn txarx2en(&mut self) -> TXARX2EN_W {
        TXARX2EN_W::new(self)
    }
    #[doc = "Bit 10 - Enable Receive Trigger After TX End of Frame Plus TCMPVAL0 Baud-times"]
    #[inline(always)]
    pub fn rxatx0en(&mut self) -> RXATX0EN_W {
        RXATX0EN_W::new(self)
    }
    #[doc = "Bit 11 - Enable Receive Trigger After TX End of Frame Plus TCMPVAL1 Baud-times"]
    #[inline(always)]
    pub fn rxatx1en(&mut self) -> RXATX1EN_W {
        RXATX1EN_W::new(self)
    }
    #[doc = "Bit 12 - Enable Receive Trigger After TX End of Frame Plus TCMPVAL2 Baud-times"]
    #[inline(always)]
    pub fn rxatx2en(&mut self) -> RXATX2EN_W {
        RXATX2EN_W::new(self)
    }
    #[doc = "Bits 16:20 - Trigger PRS Channel Select"]
    #[inline(always)]
    pub fn tsel(&mut self) -> TSEL_W {
        TSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART Trigger Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trigctrl](index.html) module"]
pub struct TRIGCTRL_SPEC;
impl crate::RegisterSpec for TRIGCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trigctrl::R](R) reader structure"]
impl crate::Readable for TRIGCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trigctrl::W](W) writer structure"]
impl crate::Writable for TRIGCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIGCTRL to value 0"]
impl crate::Resettable for TRIGCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
