#[doc = "Register `CDBUSALLOC` reader"]
pub struct R(crate::R<CDBUSALLOC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDBUSALLOC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDBUSALLOC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDBUSALLOC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CDBUSALLOC` writer"]
pub struct W(crate::W<CDBUSALLOC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDBUSALLOC_SPEC>;
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
impl From<crate::W<CDBUSALLOC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDBUSALLOC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CD Bus Even 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CDEVEN0_A {
    #[doc = "0: The bus is not allocated"]
    TRISTATE = 0,
    #[doc = "1: The bus is allocated to ADC0"]
    ADC0 = 1,
}
impl From<CDEVEN0_A> for u8 {
    #[inline(always)]
    fn from(variant: CDEVEN0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CDEVEN0` reader - CD Bus Even 0"]
pub type CDEVEN0_R = crate::FieldReader<u8, CDEVEN0_A>;
impl CDEVEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CDEVEN0_A> {
        match self.bits {
            0 => Some(CDEVEN0_A::TRISTATE),
            1 => Some(CDEVEN0_A::ADC0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRISTATE`"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == CDEVEN0_A::TRISTATE
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == CDEVEN0_A::ADC0
    }
}
#[doc = "Field `CDEVEN0` writer - CD Bus Even 0"]
pub type CDEVEN0_W<'a> = crate::FieldWriter<'a, u32, CDBUSALLOC_SPEC, u8, CDEVEN0_A, 4, 0>;
impl<'a> CDEVEN0_W<'a> {
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut W {
        self.variant(CDEVEN0_A::TRISTATE)
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut W {
        self.variant(CDEVEN0_A::ADC0)
    }
}
#[doc = "CD Bus Even 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CDEVEN1_A {
    #[doc = "0: The bus is not allocated"]
    TRISTATE = 0,
    #[doc = "1: The bus is allocated to ADC0"]
    ADC0 = 1,
}
impl From<CDEVEN1_A> for u8 {
    #[inline(always)]
    fn from(variant: CDEVEN1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CDEVEN1` reader - CD Bus Even 1"]
pub type CDEVEN1_R = crate::FieldReader<u8, CDEVEN1_A>;
impl CDEVEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CDEVEN1_A> {
        match self.bits {
            0 => Some(CDEVEN1_A::TRISTATE),
            1 => Some(CDEVEN1_A::ADC0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRISTATE`"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == CDEVEN1_A::TRISTATE
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == CDEVEN1_A::ADC0
    }
}
#[doc = "Field `CDEVEN1` writer - CD Bus Even 1"]
pub type CDEVEN1_W<'a> = crate::FieldWriter<'a, u32, CDBUSALLOC_SPEC, u8, CDEVEN1_A, 4, 8>;
impl<'a> CDEVEN1_W<'a> {
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut W {
        self.variant(CDEVEN1_A::TRISTATE)
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut W {
        self.variant(CDEVEN1_A::ADC0)
    }
}
#[doc = "CD Bus Odd 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CDODD0_A {
    #[doc = "0: The bus is not allocated"]
    TRISTATE = 0,
    #[doc = "1: The bus is allocated to ADC0"]
    ADC0 = 1,
}
impl From<CDODD0_A> for u8 {
    #[inline(always)]
    fn from(variant: CDODD0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CDODD0` reader - CD Bus Odd 0"]
pub type CDODD0_R = crate::FieldReader<u8, CDODD0_A>;
impl CDODD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CDODD0_A> {
        match self.bits {
            0 => Some(CDODD0_A::TRISTATE),
            1 => Some(CDODD0_A::ADC0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRISTATE`"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == CDODD0_A::TRISTATE
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == CDODD0_A::ADC0
    }
}
#[doc = "Field `CDODD0` writer - CD Bus Odd 0"]
pub type CDODD0_W<'a> = crate::FieldWriter<'a, u32, CDBUSALLOC_SPEC, u8, CDODD0_A, 4, 16>;
impl<'a> CDODD0_W<'a> {
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut W {
        self.variant(CDODD0_A::TRISTATE)
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut W {
        self.variant(CDODD0_A::ADC0)
    }
}
#[doc = "CD Bus Odd 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CDODD1_A {
    #[doc = "0: The bus is not allocated"]
    TRISTATE = 0,
    #[doc = "1: The bus is allocated to ADC0"]
    ADC0 = 1,
}
impl From<CDODD1_A> for u8 {
    #[inline(always)]
    fn from(variant: CDODD1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CDODD1` reader - CD Bus Odd 1"]
pub type CDODD1_R = crate::FieldReader<u8, CDODD1_A>;
impl CDODD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CDODD1_A> {
        match self.bits {
            0 => Some(CDODD1_A::TRISTATE),
            1 => Some(CDODD1_A::ADC0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRISTATE`"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == CDODD1_A::TRISTATE
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == CDODD1_A::ADC0
    }
}
#[doc = "Field `CDODD1` writer - CD Bus Odd 1"]
pub type CDODD1_W<'a> = crate::FieldWriter<'a, u32, CDBUSALLOC_SPEC, u8, CDODD1_A, 4, 24>;
impl<'a> CDODD1_W<'a> {
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut W {
        self.variant(CDODD1_A::TRISTATE)
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut W {
        self.variant(CDODD1_A::ADC0)
    }
}
impl R {
    #[doc = "Bits 0:3 - CD Bus Even 0"]
    #[inline(always)]
    pub fn cdeven0(&self) -> CDEVEN0_R {
        CDEVEN0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - CD Bus Even 1"]
    #[inline(always)]
    pub fn cdeven1(&self) -> CDEVEN1_R {
        CDEVEN1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - CD Bus Odd 0"]
    #[inline(always)]
    pub fn cdodd0(&self) -> CDODD0_R {
        CDODD0_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - CD Bus Odd 1"]
    #[inline(always)]
    pub fn cdodd1(&self) -> CDODD1_R {
        CDODD1_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CD Bus Even 0"]
    #[inline(always)]
    pub fn cdeven0(&mut self) -> CDEVEN0_W {
        CDEVEN0_W::new(self)
    }
    #[doc = "Bits 8:11 - CD Bus Even 1"]
    #[inline(always)]
    pub fn cdeven1(&mut self) -> CDEVEN1_W {
        CDEVEN1_W::new(self)
    }
    #[doc = "Bits 16:19 - CD Bus Odd 0"]
    #[inline(always)]
    pub fn cdodd0(&mut self) -> CDODD0_W {
        CDODD0_W::new(self)
    }
    #[doc = "Bits 24:27 - CD Bus Odd 1"]
    #[inline(always)]
    pub fn cdodd1(&mut self) -> CDODD1_W {
        CDODD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CD Bus allocation\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdbusalloc](index.html) module"]
pub struct CDBUSALLOC_SPEC;
impl crate::RegisterSpec for CDBUSALLOC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdbusalloc::R](R) reader structure"]
impl crate::Readable for CDBUSALLOC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cdbusalloc::W](W) writer structure"]
impl crate::Writable for CDBUSALLOC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CDBUSALLOC to value 0"]
impl crate::Resettable for CDBUSALLOC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
