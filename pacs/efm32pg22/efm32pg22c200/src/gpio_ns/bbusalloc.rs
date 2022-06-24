#[doc = "Register `BBUSALLOC` reader"]
pub struct R(crate::R<BBUSALLOC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BBUSALLOC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BBUSALLOC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BBUSALLOC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BBUSALLOC` writer"]
pub struct W(crate::W<BBUSALLOC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BBUSALLOC_SPEC>;
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
impl From<crate::W<BBUSALLOC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BBUSALLOC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "B Bus Even 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BEVEN0_A {
    #[doc = "0: The bus is not allocated"]
    TRISTATE = 0,
    #[doc = "1: The bus is allocated to ADC0"]
    ADC0 = 1,
}
impl From<BEVEN0_A> for u8 {
    #[inline(always)]
    fn from(variant: BEVEN0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BEVEN0` reader - B Bus Even 0"]
pub type BEVEN0_R = crate::FieldReader<u8, BEVEN0_A>;
impl BEVEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BEVEN0_A> {
        match self.bits {
            0 => Some(BEVEN0_A::TRISTATE),
            1 => Some(BEVEN0_A::ADC0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRISTATE`"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == BEVEN0_A::TRISTATE
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == BEVEN0_A::ADC0
    }
}
#[doc = "Field `BEVEN0` writer - B Bus Even 0"]
pub type BEVEN0_W<'a> = crate::FieldWriter<'a, u32, BBUSALLOC_SPEC, u8, BEVEN0_A, 4, 0>;
impl<'a> BEVEN0_W<'a> {
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut W {
        self.variant(BEVEN0_A::TRISTATE)
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut W {
        self.variant(BEVEN0_A::ADC0)
    }
}
#[doc = "B Bus Even 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BEVEN1_A {
    #[doc = "0: The bus is not allocated"]
    TRISTATE = 0,
    #[doc = "1: The bus is allocated to ADC0"]
    ADC0 = 1,
}
impl From<BEVEN1_A> for u8 {
    #[inline(always)]
    fn from(variant: BEVEN1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BEVEN1` reader - B Bus Even 1"]
pub type BEVEN1_R = crate::FieldReader<u8, BEVEN1_A>;
impl BEVEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BEVEN1_A> {
        match self.bits {
            0 => Some(BEVEN1_A::TRISTATE),
            1 => Some(BEVEN1_A::ADC0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRISTATE`"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == BEVEN1_A::TRISTATE
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == BEVEN1_A::ADC0
    }
}
#[doc = "Field `BEVEN1` writer - B Bus Even 1"]
pub type BEVEN1_W<'a> = crate::FieldWriter<'a, u32, BBUSALLOC_SPEC, u8, BEVEN1_A, 4, 8>;
impl<'a> BEVEN1_W<'a> {
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut W {
        self.variant(BEVEN1_A::TRISTATE)
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut W {
        self.variant(BEVEN1_A::ADC0)
    }
}
#[doc = "B Bus Odd 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BODD0_A {
    #[doc = "0: The bus is not allocated"]
    TRISTATE = 0,
    #[doc = "1: The bus is allocated to ADC0"]
    ADC0 = 1,
}
impl From<BODD0_A> for u8 {
    #[inline(always)]
    fn from(variant: BODD0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BODD0` reader - B Bus Odd 0"]
pub type BODD0_R = crate::FieldReader<u8, BODD0_A>;
impl BODD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BODD0_A> {
        match self.bits {
            0 => Some(BODD0_A::TRISTATE),
            1 => Some(BODD0_A::ADC0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRISTATE`"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == BODD0_A::TRISTATE
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == BODD0_A::ADC0
    }
}
#[doc = "Field `BODD0` writer - B Bus Odd 0"]
pub type BODD0_W<'a> = crate::FieldWriter<'a, u32, BBUSALLOC_SPEC, u8, BODD0_A, 4, 16>;
impl<'a> BODD0_W<'a> {
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut W {
        self.variant(BODD0_A::TRISTATE)
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut W {
        self.variant(BODD0_A::ADC0)
    }
}
#[doc = "B Bus Odd 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BODD1_A {
    #[doc = "0: The bus is not allocated"]
    TRISTATE = 0,
    #[doc = "1: The bus is allocated to ADC0"]
    ADC0 = 1,
}
impl From<BODD1_A> for u8 {
    #[inline(always)]
    fn from(variant: BODD1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BODD1` reader - B Bus Odd 1"]
pub type BODD1_R = crate::FieldReader<u8, BODD1_A>;
impl BODD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BODD1_A> {
        match self.bits {
            0 => Some(BODD1_A::TRISTATE),
            1 => Some(BODD1_A::ADC0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRISTATE`"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == BODD1_A::TRISTATE
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == BODD1_A::ADC0
    }
}
#[doc = "Field `BODD1` writer - B Bus Odd 1"]
pub type BODD1_W<'a> = crate::FieldWriter<'a, u32, BBUSALLOC_SPEC, u8, BODD1_A, 4, 24>;
impl<'a> BODD1_W<'a> {
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut W {
        self.variant(BODD1_A::TRISTATE)
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut W {
        self.variant(BODD1_A::ADC0)
    }
}
impl R {
    #[doc = "Bits 0:3 - B Bus Even 0"]
    #[inline(always)]
    pub fn beven0(&self) -> BEVEN0_R {
        BEVEN0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - B Bus Even 1"]
    #[inline(always)]
    pub fn beven1(&self) -> BEVEN1_R {
        BEVEN1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - B Bus Odd 0"]
    #[inline(always)]
    pub fn bodd0(&self) -> BODD0_R {
        BODD0_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - B Bus Odd 1"]
    #[inline(always)]
    pub fn bodd1(&self) -> BODD1_R {
        BODD1_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - B Bus Even 0"]
    #[inline(always)]
    pub fn beven0(&mut self) -> BEVEN0_W {
        BEVEN0_W::new(self)
    }
    #[doc = "Bits 8:11 - B Bus Even 1"]
    #[inline(always)]
    pub fn beven1(&mut self) -> BEVEN1_W {
        BEVEN1_W::new(self)
    }
    #[doc = "Bits 16:19 - B Bus Odd 0"]
    #[inline(always)]
    pub fn bodd0(&mut self) -> BODD0_W {
        BODD0_W::new(self)
    }
    #[doc = "Bits 24:27 - B Bus Odd 1"]
    #[inline(always)]
    pub fn bodd1(&mut self) -> BODD1_W {
        BODD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "B Bus allocation\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bbusalloc](index.html) module"]
pub struct BBUSALLOC_SPEC;
impl crate::RegisterSpec for BBUSALLOC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bbusalloc::R](R) reader structure"]
impl crate::Readable for BBUSALLOC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bbusalloc::W](W) writer structure"]
impl crate::Writable for BBUSALLOC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BBUSALLOC to value 0"]
impl crate::Resettable for BBUSALLOC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
