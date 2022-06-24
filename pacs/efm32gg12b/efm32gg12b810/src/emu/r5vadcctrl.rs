#[doc = "Register `R5VADCCTRL` reader"]
pub struct R(crate::R<R5VADCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R5VADCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R5VADCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R5VADCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R5VADCCTRL` writer"]
pub struct W(crate::W<R5VADCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R5VADCCTRL_SPEC>;
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
impl From<crate::W<R5VADCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R5VADCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENAMUX` reader - Enable the 5V Subsystem ADC MUX"]
pub type ENAMUX_R = crate::BitReader<bool>;
#[doc = "Field `ENAMUX` writer - Enable the 5V Subsystem ADC MUX"]
pub type ENAMUX_W<'a> = crate::BitWriter<'a, u32, R5VADCCTRL_SPEC, bool, 0>;
#[doc = "ADC Mux Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AMUXSEL_A {
    #[doc = "0: VBUS divided by 10"]
    VBUSDIV10 = 0,
    #[doc = "1: VREGI divided by 10"]
    VREGIDIV10 = 1,
    #[doc = "2: VREGO divided by 6"]
    VREGODIV6 = 2,
    #[doc = "3: VREGI current monitor"]
    VREGIIMON = 3,
    #[doc = "4: VBUS current monitor"]
    VBUSIMON = 4,
}
impl From<AMUXSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: AMUXSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AMUXSEL` reader - ADC Mux Selection"]
pub type AMUXSEL_R = crate::FieldReader<u8, AMUXSEL_A>;
impl AMUXSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AMUXSEL_A> {
        match self.bits {
            0 => Some(AMUXSEL_A::VBUSDIV10),
            1 => Some(AMUXSEL_A::VREGIDIV10),
            2 => Some(AMUXSEL_A::VREGODIV6),
            3 => Some(AMUXSEL_A::VREGIIMON),
            4 => Some(AMUXSEL_A::VBUSIMON),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VBUSDIV10`"]
    #[inline(always)]
    pub fn is_vbusdiv10(&self) -> bool {
        *self == AMUXSEL_A::VBUSDIV10
    }
    #[doc = "Checks if the value of the field is `VREGIDIV10`"]
    #[inline(always)]
    pub fn is_vregidiv10(&self) -> bool {
        *self == AMUXSEL_A::VREGIDIV10
    }
    #[doc = "Checks if the value of the field is `VREGODIV6`"]
    #[inline(always)]
    pub fn is_vregodiv6(&self) -> bool {
        *self == AMUXSEL_A::VREGODIV6
    }
    #[doc = "Checks if the value of the field is `VREGIIMON`"]
    #[inline(always)]
    pub fn is_vregiimon(&self) -> bool {
        *self == AMUXSEL_A::VREGIIMON
    }
    #[doc = "Checks if the value of the field is `VBUSIMON`"]
    #[inline(always)]
    pub fn is_vbusimon(&self) -> bool {
        *self == AMUXSEL_A::VBUSIMON
    }
}
#[doc = "Field `AMUXSEL` writer - ADC Mux Selection"]
pub type AMUXSEL_W<'a> = crate::FieldWriter<'a, u32, R5VADCCTRL_SPEC, u8, AMUXSEL_A, 4, 12>;
impl<'a> AMUXSEL_W<'a> {
    #[doc = "VBUS divided by 10"]
    #[inline(always)]
    pub fn vbusdiv10(self) -> &'a mut W {
        self.variant(AMUXSEL_A::VBUSDIV10)
    }
    #[doc = "VREGI divided by 10"]
    #[inline(always)]
    pub fn vregidiv10(self) -> &'a mut W {
        self.variant(AMUXSEL_A::VREGIDIV10)
    }
    #[doc = "VREGO divided by 6"]
    #[inline(always)]
    pub fn vregodiv6(self) -> &'a mut W {
        self.variant(AMUXSEL_A::VREGODIV6)
    }
    #[doc = "VREGI current monitor"]
    #[inline(always)]
    pub fn vregiimon(self) -> &'a mut W {
        self.variant(AMUXSEL_A::VREGIIMON)
    }
    #[doc = "VBUS current monitor"]
    #[inline(always)]
    pub fn vbusimon(self) -> &'a mut W {
        self.variant(AMUXSEL_A::VBUSIMON)
    }
}
impl R {
    #[doc = "Bit 0 - Enable the 5V Subsystem ADC MUX"]
    #[inline(always)]
    pub fn enamux(&self) -> ENAMUX_R {
        ENAMUX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 12:15 - ADC Mux Selection"]
    #[inline(always)]
    pub fn amuxsel(&self) -> AMUXSEL_R {
        AMUXSEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the 5V Subsystem ADC MUX"]
    #[inline(always)]
    pub fn enamux(&mut self) -> ENAMUX_W {
        ENAMUX_W::new(self)
    }
    #[doc = "Bits 12:15 - ADC Mux Selection"]
    #[inline(always)]
    pub fn amuxsel(&mut self) -> AMUXSEL_W {
        AMUXSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "5V Regulator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r5vadcctrl](index.html) module"]
pub struct R5VADCCTRL_SPEC;
impl crate::RegisterSpec for R5VADCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r5vadcctrl::R](R) reader structure"]
impl crate::Readable for R5VADCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r5vadcctrl::W](W) writer structure"]
impl crate::Writable for R5VADCCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R5VADCCTRL to value 0"]
impl crate::Resettable for R5VADCCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
