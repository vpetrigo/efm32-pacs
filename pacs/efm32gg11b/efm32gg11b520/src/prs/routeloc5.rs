#[doc = "Register `ROUTELOC5` reader"]
pub struct R(crate::R<ROUTELOC5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROUTELOC5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROUTELOC5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROUTELOC5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROUTELOC5` writer"]
pub struct W(crate::W<ROUTELOC5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROUTELOC5_SPEC>;
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
impl From<crate::W<ROUTELOC5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROUTELOC5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH20LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH20LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH20LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH20LOC` reader - I/O Location"]
pub type CH20LOC_R = crate::FieldReader<u8, CH20LOC_A>;
impl CH20LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH20LOC_A> {
        match self.bits {
            0 => Some(CH20LOC_A::LOC0),
            1 => Some(CH20LOC_A::LOC1),
            2 => Some(CH20LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH20LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH20LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH20LOC_A::LOC2
    }
}
#[doc = "Field `CH20LOC` writer - I/O Location"]
pub type CH20LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC5_SPEC, u8, CH20LOC_A, 6, 0>;
impl<'a> CH20LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH20LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH20LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH20LOC_A::LOC2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH21LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH21LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH21LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH21LOC` reader - I/O Location"]
pub type CH21LOC_R = crate::FieldReader<u8, CH21LOC_A>;
impl CH21LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH21LOC_A> {
        match self.bits {
            0 => Some(CH21LOC_A::LOC0),
            1 => Some(CH21LOC_A::LOC1),
            2 => Some(CH21LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH21LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH21LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH21LOC_A::LOC2
    }
}
#[doc = "Field `CH21LOC` writer - I/O Location"]
pub type CH21LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC5_SPEC, u8, CH21LOC_A, 6, 8>;
impl<'a> CH21LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH21LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH21LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH21LOC_A::LOC2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH22LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH22LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH22LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH22LOC` reader - I/O Location"]
pub type CH22LOC_R = crate::FieldReader<u8, CH22LOC_A>;
impl CH22LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH22LOC_A> {
        match self.bits {
            0 => Some(CH22LOC_A::LOC0),
            1 => Some(CH22LOC_A::LOC1),
            2 => Some(CH22LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH22LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH22LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH22LOC_A::LOC2
    }
}
#[doc = "Field `CH22LOC` writer - I/O Location"]
pub type CH22LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC5_SPEC, u8, CH22LOC_A, 6, 16>;
impl<'a> CH22LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH22LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH22LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH22LOC_A::LOC2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH23LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH23LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH23LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH23LOC` reader - I/O Location"]
pub type CH23LOC_R = crate::FieldReader<u8, CH23LOC_A>;
impl CH23LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH23LOC_A> {
        match self.bits {
            0 => Some(CH23LOC_A::LOC0),
            1 => Some(CH23LOC_A::LOC1),
            2 => Some(CH23LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH23LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH23LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH23LOC_A::LOC2
    }
}
#[doc = "Field `CH23LOC` writer - I/O Location"]
pub type CH23LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC5_SPEC, u8, CH23LOC_A, 6, 24>;
impl<'a> CH23LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH23LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH23LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH23LOC_A::LOC2)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch20loc(&self) -> CH20LOC_R {
        CH20LOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch21loc(&self) -> CH21LOC_R {
        CH21LOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch22loc(&self) -> CH22LOC_R {
        CH22LOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch23loc(&self) -> CH23LOC_R {
        CH23LOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch20loc(&mut self) -> CH20LOC_W {
        CH20LOC_W::new(self)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch21loc(&mut self) -> CH21LOC_W {
        CH21LOC_W::new(self)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch22loc(&mut self) -> CH22LOC_W {
        CH22LOC_W::new(self)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch23loc(&mut self) -> CH23LOC_W {
        CH23LOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Routing Location Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routeloc5](index.html) module"]
pub struct ROUTELOC5_SPEC;
impl crate::RegisterSpec for ROUTELOC5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [routeloc5::R](R) reader structure"]
impl crate::Readable for ROUTELOC5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [routeloc5::W](W) writer structure"]
impl crate::Writable for ROUTELOC5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROUTELOC5 to value 0"]
impl crate::Resettable for ROUTELOC5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
