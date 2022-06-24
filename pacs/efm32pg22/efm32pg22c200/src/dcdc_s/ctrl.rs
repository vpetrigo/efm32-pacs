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
#[doc = "DCDC/Bypass Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: DCDC is OFF, bypass switch is enabled"]
    BYPASS = 0,
    #[doc = "1: Request DCDC regulation, bypass switch disabled"]
    DCDCREGULATION = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - DCDC/Bypass Mode Control"]
pub type MODE_R = crate::BitReader<MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::BYPASS,
            true => MODE_A::DCDCREGULATION,
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == MODE_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `DCDCREGULATION`"]
    #[inline(always)]
    pub fn is_dcdcregulation(&self) -> bool {
        *self == MODE_A::DCDCREGULATION
    }
}
#[doc = "Field `MODE` writer - DCDC/Bypass Mode Control"]
pub type MODE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, MODE_A, 0>;
impl<'a> MODE_W<'a> {
    #[doc = "DCDC is OFF, bypass switch is enabled"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(MODE_A::BYPASS)
    }
    #[doc = "Request DCDC regulation, bypass switch disabled"]
    #[inline(always)]
    pub fn dcdcregulation(self) -> &'a mut W {
        self.variant(MODE_A::DCDCREGULATION)
    }
}
#[doc = "DCDC DCM Only Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCMONLYEN_A {
    #[doc = "0: Support higher load current at lower battery voltage by working in CCM mode"]
    DUALMODE = 0,
    #[doc = "1: DCM only mode for normal operation, this is the default setting"]
    DCMONLYEN = 1,
}
impl From<DCMONLYEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCMONLYEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMONLYEN` reader - DCDC DCM Only Enable"]
pub type DCMONLYEN_R = crate::BitReader<DCMONLYEN_A>;
impl DCMONLYEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCMONLYEN_A {
        match self.bits {
            false => DCMONLYEN_A::DUALMODE,
            true => DCMONLYEN_A::DCMONLYEN,
        }
    }
    #[doc = "Checks if the value of the field is `DUALMODE`"]
    #[inline(always)]
    pub fn is_dualmode(&self) -> bool {
        *self == DCMONLYEN_A::DUALMODE
    }
    #[doc = "Checks if the value of the field is `DCMONLYEN`"]
    #[inline(always)]
    pub fn is_dcmonlyen(&self) -> bool {
        *self == DCMONLYEN_A::DCMONLYEN
    }
}
#[doc = "Field `DCMONLYEN` writer - DCDC DCM Only Enable"]
pub type DCMONLYEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, DCMONLYEN_A, 2>;
impl<'a> DCMONLYEN_W<'a> {
    #[doc = "Support higher load current at lower battery voltage by working in CCM mode"]
    #[inline(always)]
    pub fn dualmode(self) -> &'a mut W {
        self.variant(DCMONLYEN_A::DUALMODE)
    }
    #[doc = "DCM only mode for normal operation, this is the default setting"]
    #[inline(always)]
    pub fn dcmonlyen(self) -> &'a mut W {
        self.variant(DCMONLYEN_A::DCMONLYEN)
    }
}
#[doc = "Peak Current Timeout Control\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IPKTMAXCTRL_A {
    #[doc = "0: Ton_max disabled"]
    OFF = 0,
    #[doc = "1: 0.35us"]
    TMAX_0P35US = 1,
    #[doc = "2: 0.63us"]
    TMAX_0P63US = 2,
    #[doc = "3: 0.91us"]
    TMAX_0P91US = 3,
    #[doc = "4: 1.19us"]
    TMAX_1P19US = 4,
    #[doc = "5: 1.47us"]
    TMAX_1P47US = 5,
    #[doc = "6: 1.75us"]
    TMAX_1P75US = 6,
    #[doc = "7: 2.03us"]
    TMAX_2P03US = 7,
}
impl From<IPKTMAXCTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: IPKTMAXCTRL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IPKTMAXCTRL` reader - Peak Current Timeout Control"]
pub type IPKTMAXCTRL_R = crate::FieldReader<u8, IPKTMAXCTRL_A>;
impl IPKTMAXCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPKTMAXCTRL_A {
        match self.bits {
            0 => IPKTMAXCTRL_A::OFF,
            1 => IPKTMAXCTRL_A::TMAX_0P35US,
            2 => IPKTMAXCTRL_A::TMAX_0P63US,
            3 => IPKTMAXCTRL_A::TMAX_0P91US,
            4 => IPKTMAXCTRL_A::TMAX_1P19US,
            5 => IPKTMAXCTRL_A::TMAX_1P47US,
            6 => IPKTMAXCTRL_A::TMAX_1P75US,
            7 => IPKTMAXCTRL_A::TMAX_2P03US,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == IPKTMAXCTRL_A::OFF
    }
    #[doc = "Checks if the value of the field is `TMAX_0P35US`"]
    #[inline(always)]
    pub fn is_tmax_0p35us(&self) -> bool {
        *self == IPKTMAXCTRL_A::TMAX_0P35US
    }
    #[doc = "Checks if the value of the field is `TMAX_0P63US`"]
    #[inline(always)]
    pub fn is_tmax_0p63us(&self) -> bool {
        *self == IPKTMAXCTRL_A::TMAX_0P63US
    }
    #[doc = "Checks if the value of the field is `TMAX_0P91US`"]
    #[inline(always)]
    pub fn is_tmax_0p91us(&self) -> bool {
        *self == IPKTMAXCTRL_A::TMAX_0P91US
    }
    #[doc = "Checks if the value of the field is `TMAX_1P19US`"]
    #[inline(always)]
    pub fn is_tmax_1p19us(&self) -> bool {
        *self == IPKTMAXCTRL_A::TMAX_1P19US
    }
    #[doc = "Checks if the value of the field is `TMAX_1P47US`"]
    #[inline(always)]
    pub fn is_tmax_1p47us(&self) -> bool {
        *self == IPKTMAXCTRL_A::TMAX_1P47US
    }
    #[doc = "Checks if the value of the field is `TMAX_1P75US`"]
    #[inline(always)]
    pub fn is_tmax_1p75us(&self) -> bool {
        *self == IPKTMAXCTRL_A::TMAX_1P75US
    }
    #[doc = "Checks if the value of the field is `TMAX_2P03US`"]
    #[inline(always)]
    pub fn is_tmax_2p03us(&self) -> bool {
        *self == IPKTMAXCTRL_A::TMAX_2P03US
    }
}
#[doc = "Field `IPKTMAXCTRL` writer - Peak Current Timeout Control"]
pub type IPKTMAXCTRL_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, IPKTMAXCTRL_A, 3, 4>;
impl<'a> IPKTMAXCTRL_W<'a> {
    #[doc = "Ton_max disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(IPKTMAXCTRL_A::OFF)
    }
    #[doc = "0.35us"]
    #[inline(always)]
    pub fn tmax_0p35us(self) -> &'a mut W {
        self.variant(IPKTMAXCTRL_A::TMAX_0P35US)
    }
    #[doc = "0.63us"]
    #[inline(always)]
    pub fn tmax_0p63us(self) -> &'a mut W {
        self.variant(IPKTMAXCTRL_A::TMAX_0P63US)
    }
    #[doc = "0.91us"]
    #[inline(always)]
    pub fn tmax_0p91us(self) -> &'a mut W {
        self.variant(IPKTMAXCTRL_A::TMAX_0P91US)
    }
    #[doc = "1.19us"]
    #[inline(always)]
    pub fn tmax_1p19us(self) -> &'a mut W {
        self.variant(IPKTMAXCTRL_A::TMAX_1P19US)
    }
    #[doc = "1.47us"]
    #[inline(always)]
    pub fn tmax_1p47us(self) -> &'a mut W {
        self.variant(IPKTMAXCTRL_A::TMAX_1P47US)
    }
    #[doc = "1.75us"]
    #[inline(always)]
    pub fn tmax_1p75us(self) -> &'a mut W {
        self.variant(IPKTMAXCTRL_A::TMAX_1P75US)
    }
    #[doc = "2.03us"]
    #[inline(always)]
    pub fn tmax_2p03us(self) -> &'a mut W {
        self.variant(IPKTMAXCTRL_A::TMAX_2P03US)
    }
}
impl R {
    #[doc = "Bit 0 - DCDC/Bypass Mode Control"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - DCDC DCM Only Enable"]
    #[inline(always)]
    pub fn dcmonlyen(&self) -> DCMONLYEN_R {
        DCMONLYEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Peak Current Timeout Control"]
    #[inline(always)]
    pub fn ipktmaxctrl(&self) -> IPKTMAXCTRL_R {
        IPKTMAXCTRL_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DCDC/Bypass Mode Control"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W::new(self)
    }
    #[doc = "Bit 2 - DCDC DCM Only Enable"]
    #[inline(always)]
    pub fn dcmonlyen(&mut self) -> DCMONLYEN_W {
        DCMONLYEN_W::new(self)
    }
    #[doc = "Bits 4:6 - Peak Current Timeout Control"]
    #[inline(always)]
    pub fn ipktmaxctrl(&mut self) -> IPKTMAXCTRL_W {
        IPKTMAXCTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
#[doc = "`reset()` method sets CTRL to value 0x44"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x44
    }
}
