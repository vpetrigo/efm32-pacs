#[doc = "Register `CC0_CTRL` reader"]
pub struct R(crate::R<CC0_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC0_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC0_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC0_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CC0_CTRL` writer"]
pub struct W(crate::W<CC0_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CC0_CTRL_SPEC>;
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
impl From<crate::W<CC0_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CC0_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CC Channel Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Compare/Capture channel turned off"]
    OFF = 0,
    #[doc = "1: Input capture"]
    INPUTCAPTURE = 1,
    #[doc = "2: Output compare"]
    OUTPUTCOMPARE = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - CC Channel Mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::OFF),
            1 => Some(MODE_A::INPUTCAPTURE),
            2 => Some(MODE_A::OUTPUTCOMPARE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == MODE_A::OFF
    }
    #[doc = "Checks if the value of the field is `INPUTCAPTURE`"]
    #[inline(always)]
    pub fn is_inputcapture(&self) -> bool {
        *self == MODE_A::INPUTCAPTURE
    }
    #[doc = "Checks if the value of the field is `OUTPUTCOMPARE`"]
    #[inline(always)]
    pub fn is_outputcompare(&self) -> bool {
        *self == MODE_A::OUTPUTCOMPARE
    }
}
#[doc = "Field `MODE` writer - CC Channel Mode"]
pub type MODE_W<'a> = crate::FieldWriter<'a, u32, CC0_CTRL_SPEC, u8, MODE_A, 2, 0>;
impl<'a> MODE_W<'a> {
    #[doc = "Compare/Capture channel turned off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(MODE_A::OFF)
    }
    #[doc = "Input capture"]
    #[inline(always)]
    pub fn inputcapture(self) -> &'a mut W {
        self.variant(MODE_A::INPUTCAPTURE)
    }
    #[doc = "Output compare"]
    #[inline(always)]
    pub fn outputcompare(self) -> &'a mut W {
        self.variant(MODE_A::OUTPUTCOMPARE)
    }
}
#[doc = "Compare Match Output Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMOA_A {
    #[doc = "0: A single clock cycle pulse is generated on output"]
    PULSE = 0,
    #[doc = "1: Toggle output on compare match"]
    TOGGLE = 1,
    #[doc = "2: Clear output on compare match"]
    CLEAR = 2,
    #[doc = "3: Set output on compare match"]
    SET = 3,
}
impl From<CMOA_A> for u8 {
    #[inline(always)]
    fn from(variant: CMOA_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMOA` reader - Compare Match Output Action"]
pub type CMOA_R = crate::FieldReader<u8, CMOA_A>;
impl CMOA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMOA_A {
        match self.bits {
            0 => CMOA_A::PULSE,
            1 => CMOA_A::TOGGLE,
            2 => CMOA_A::CLEAR,
            3 => CMOA_A::SET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULSE`"]
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == CMOA_A::PULSE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == CMOA_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CMOA_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CMOA_A::SET
    }
}
#[doc = "Field `CMOA` writer - Compare Match Output Action"]
pub type CMOA_W<'a> = crate::FieldWriterSafe<'a, u32, CC0_CTRL_SPEC, u8, CMOA_A, 2, 2>;
impl<'a> CMOA_W<'a> {
    #[doc = "A single clock cycle pulse is generated on output"]
    #[inline(always)]
    pub fn pulse(self) -> &'a mut W {
        self.variant(CMOA_A::PULSE)
    }
    #[doc = "Toggle output on compare match"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(CMOA_A::TOGGLE)
    }
    #[doc = "Clear output on compare match"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMOA_A::CLEAR)
    }
    #[doc = "Set output on compare match"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CMOA_A::SET)
    }
}
#[doc = "Capture compare channel comparison base.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPBASE_A {
    #[doc = "0: RTCC_CCx_ICVALUE/OCVALUE is compared with CNT register."]
    CNT = 0,
    #[doc = "1: Least significant bits of RTCC_CCx_ICVALUE/OCVALUE are compared with COMBCNT."]
    PRECNT = 1,
}
impl From<COMPBASE_A> for bool {
    #[inline(always)]
    fn from(variant: COMPBASE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPBASE` reader - Capture compare channel comparison base."]
pub type COMPBASE_R = crate::BitReader<COMPBASE_A>;
impl COMPBASE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPBASE_A {
        match self.bits {
            false => COMPBASE_A::CNT,
            true => COMPBASE_A::PRECNT,
        }
    }
    #[doc = "Checks if the value of the field is `CNT`"]
    #[inline(always)]
    pub fn is_cnt(&self) -> bool {
        *self == COMPBASE_A::CNT
    }
    #[doc = "Checks if the value of the field is `PRECNT`"]
    #[inline(always)]
    pub fn is_precnt(&self) -> bool {
        *self == COMPBASE_A::PRECNT
    }
}
#[doc = "Field `COMPBASE` writer - Capture compare channel comparison base."]
pub type COMPBASE_W<'a> = crate::BitWriter<'a, u32, CC0_CTRL_SPEC, COMPBASE_A, 4>;
impl<'a> COMPBASE_W<'a> {
    #[doc = "RTCC_CCx_ICVALUE/OCVALUE is compared with CNT register."]
    #[inline(always)]
    pub fn cnt(self) -> &'a mut W {
        self.variant(COMPBASE_A::CNT)
    }
    #[doc = "Least significant bits of RTCC_CCx_ICVALUE/OCVALUE are compared with COMBCNT."]
    #[inline(always)]
    pub fn precnt(self) -> &'a mut W {
        self.variant(COMPBASE_A::PRECNT)
    }
}
#[doc = "Input Capture Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ICEDGE_A {
    #[doc = "0: Rising edges detected"]
    RISING = 0,
    #[doc = "1: Falling edges detected"]
    FALLING = 1,
    #[doc = "2: Both edges detected"]
    BOTH = 2,
    #[doc = "3: No edge detection, signal is left as it is"]
    NONE = 3,
}
impl From<ICEDGE_A> for u8 {
    #[inline(always)]
    fn from(variant: ICEDGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ICEDGE` reader - Input Capture Edge Select"]
pub type ICEDGE_R = crate::FieldReader<u8, ICEDGE_A>;
impl ICEDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICEDGE_A {
        match self.bits {
            0 => ICEDGE_A::RISING,
            1 => ICEDGE_A::FALLING,
            2 => ICEDGE_A::BOTH,
            3 => ICEDGE_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == ICEDGE_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == ICEDGE_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == ICEDGE_A::BOTH
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ICEDGE_A::NONE
    }
}
#[doc = "Field `ICEDGE` writer - Input Capture Edge Select"]
pub type ICEDGE_W<'a> = crate::FieldWriterSafe<'a, u32, CC0_CTRL_SPEC, u8, ICEDGE_A, 2, 5>;
impl<'a> ICEDGE_W<'a> {
    #[doc = "Rising edges detected"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(ICEDGE_A::RISING)
    }
    #[doc = "Falling edges detected"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(ICEDGE_A::FALLING)
    }
    #[doc = "Both edges detected"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(ICEDGE_A::BOTH)
    }
    #[doc = "No edge detection, signal is left as it is"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ICEDGE_A::NONE)
    }
}
impl R {
    #[doc = "Bits 0:1 - CC Channel Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Compare Match Output Action"]
    #[inline(always)]
    pub fn cmoa(&self) -> CMOA_R {
        CMOA_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Capture compare channel comparison base."]
    #[inline(always)]
    pub fn compbase(&self) -> COMPBASE_R {
        COMPBASE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Input Capture Edge Select"]
    #[inline(always)]
    pub fn icedge(&self) -> ICEDGE_R {
        ICEDGE_R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC Channel Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W::new(self)
    }
    #[doc = "Bits 2:3 - Compare Match Output Action"]
    #[inline(always)]
    pub fn cmoa(&mut self) -> CMOA_W {
        CMOA_W::new(self)
    }
    #[doc = "Bit 4 - Capture compare channel comparison base."]
    #[inline(always)]
    pub fn compbase(&mut self) -> COMPBASE_W {
        COMPBASE_W::new(self)
    }
    #[doc = "Bits 5:6 - Input Capture Edge Select"]
    #[inline(always)]
    pub fn icedge(&mut self) -> ICEDGE_W {
        ICEDGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc0_ctrl](index.html) module"]
pub struct CC0_CTRL_SPEC;
impl crate::RegisterSpec for CC0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc0_ctrl::R](R) reader structure"]
impl crate::Readable for CC0_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cc0_ctrl::W](W) writer structure"]
impl crate::Writable for CC0_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CC0_CTRL to value 0"]
impl crate::Resettable for CC0_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
