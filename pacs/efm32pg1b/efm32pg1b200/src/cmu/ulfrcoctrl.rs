#[doc = "Register `ULFRCOCTRL` reader"]
pub struct R(crate::R<ULFRCOCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ULFRCOCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ULFRCOCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ULFRCOCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ULFRCOCTRL` writer"]
pub struct W(crate::W<ULFRCOCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ULFRCOCTRL_SPEC>;
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
impl From<crate::W<ULFRCOCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ULFRCOCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TUNING` reader - ULFRCO TUNING Value"]
pub type TUNING_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TUNING` writer - ULFRCO TUNING Value"]
pub type TUNING_W<'a> = crate::FieldWriter<'a, u32, ULFRCOCTRL_SPEC, u8, u8, 6, 0>;
#[doc = "ULFRCO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: ULFRCO = 1 kHz"]
    _1KHZ = 0,
    #[doc = "1: ULFRCO = 2 kHz"]
    _2KHZ = 1,
    #[doc = "2: ULFRCO = 4 kHz"]
    _4KHZ = 2,
    #[doc = "3: ULFRCO = 32 kHz"]
    _32KHZ = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - ULFRCO Mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::_1KHZ,
            1 => MODE_A::_2KHZ,
            2 => MODE_A::_4KHZ,
            3 => MODE_A::_32KHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1KHZ`"]
    #[inline(always)]
    pub fn is_1khz(&self) -> bool {
        *self == MODE_A::_1KHZ
    }
    #[doc = "Checks if the value of the field is `_2KHZ`"]
    #[inline(always)]
    pub fn is_2khz(&self) -> bool {
        *self == MODE_A::_2KHZ
    }
    #[doc = "Checks if the value of the field is `_4KHZ`"]
    #[inline(always)]
    pub fn is_4khz(&self) -> bool {
        *self == MODE_A::_4KHZ
    }
    #[doc = "Checks if the value of the field is `_32KHZ`"]
    #[inline(always)]
    pub fn is_32khz(&self) -> bool {
        *self == MODE_A::_32KHZ
    }
}
#[doc = "Field `MODE` writer - ULFRCO Mode"]
pub type MODE_W<'a> = crate::FieldWriterSafe<'a, u32, ULFRCOCTRL_SPEC, u8, MODE_A, 2, 10>;
impl<'a> MODE_W<'a> {
    #[doc = "ULFRCO = 1 kHz"]
    #[inline(always)]
    pub fn _1khz(self) -> &'a mut W {
        self.variant(MODE_A::_1KHZ)
    }
    #[doc = "ULFRCO = 2 kHz"]
    #[inline(always)]
    pub fn _2khz(self) -> &'a mut W {
        self.variant(MODE_A::_2KHZ)
    }
    #[doc = "ULFRCO = 4 kHz"]
    #[inline(always)]
    pub fn _4khz(self) -> &'a mut W {
        self.variant(MODE_A::_4KHZ)
    }
    #[doc = "ULFRCO = 32 kHz"]
    #[inline(always)]
    pub fn _32khz(self) -> &'a mut W {
        self.variant(MODE_A::_32KHZ)
    }
}
#[doc = "Field `RESTRIM` reader - ULFRCO Resistor Trim Value (for Resistor in Bias Circuit; NOT for USE as FREQUENCY CALIBRATION)"]
pub type RESTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESTRIM` writer - ULFRCO Resistor Trim Value (for Resistor in Bias Circuit; NOT for USE as FREQUENCY CALIBRATION)"]
pub type RESTRIM_W<'a> = crate::FieldWriter<'a, u32, ULFRCOCTRL_SPEC, u8, u8, 2, 16>;
impl R {
    #[doc = "Bits 0:5 - ULFRCO TUNING Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 10:11 - ULFRCO Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 16:17 - ULFRCO Resistor Trim Value (for Resistor in Bias Circuit; NOT for USE as FREQUENCY CALIBRATION)"]
    #[inline(always)]
    pub fn restrim(&self) -> RESTRIM_R {
        RESTRIM_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - ULFRCO TUNING Value"]
    #[inline(always)]
    pub fn tuning(&mut self) -> TUNING_W {
        TUNING_W::new(self)
    }
    #[doc = "Bits 10:11 - ULFRCO Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W::new(self)
    }
    #[doc = "Bits 16:17 - ULFRCO Resistor Trim Value (for Resistor in Bias Circuit; NOT for USE as FREQUENCY CALIBRATION)"]
    #[inline(always)]
    pub fn restrim(&mut self) -> RESTRIM_W {
        RESTRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ULFRCO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ulfrcoctrl](index.html) module"]
pub struct ULFRCOCTRL_SPEC;
impl crate::RegisterSpec for ULFRCOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ulfrcoctrl::R](R) reader structure"]
impl crate::Readable for ULFRCOCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ulfrcoctrl::W](W) writer structure"]
impl crate::Writable for ULFRCOCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ULFRCOCTRL to value 0x0002_0020"]
impl crate::Resettable for ULFRCOCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0002_0020
    }
}
