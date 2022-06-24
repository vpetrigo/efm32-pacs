#[doc = "Register `ROUTELOC1` reader"]
pub struct R(crate::R<ROUTELOC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROUTELOC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROUTELOC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROUTELOC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROUTELOC1` writer"]
pub struct W(crate::W<ROUTELOC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROUTELOC1_SPEC>;
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
impl From<crate::W<ROUTELOC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROUTELOC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ETMTCLKLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
}
impl From<ETMTCLKLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: ETMTCLKLOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ETMTCLKLOC` reader - I/O Location"]
pub type ETMTCLKLOC_R = crate::FieldReader<u8, ETMTCLKLOC_A>;
impl ETMTCLKLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ETMTCLKLOC_A> {
        match self.bits {
            0 => Some(ETMTCLKLOC_A::LOC0),
            1 => Some(ETMTCLKLOC_A::LOC1),
            2 => Some(ETMTCLKLOC_A::LOC2),
            3 => Some(ETMTCLKLOC_A::LOC3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == ETMTCLKLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == ETMTCLKLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == ETMTCLKLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == ETMTCLKLOC_A::LOC3
    }
}
#[doc = "Field `ETMTCLKLOC` writer - I/O Location"]
pub type ETMTCLKLOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC1_SPEC, u8, ETMTCLKLOC_A, 6, 0>;
impl<'a> ETMTCLKLOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(ETMTCLKLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(ETMTCLKLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(ETMTCLKLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(ETMTCLKLOC_A::LOC3)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ETMTD0LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
}
impl From<ETMTD0LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: ETMTD0LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ETMTD0LOC` reader - I/O Location"]
pub type ETMTD0LOC_R = crate::FieldReader<u8, ETMTD0LOC_A>;
impl ETMTD0LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ETMTD0LOC_A> {
        match self.bits {
            0 => Some(ETMTD0LOC_A::LOC0),
            1 => Some(ETMTD0LOC_A::LOC1),
            2 => Some(ETMTD0LOC_A::LOC2),
            3 => Some(ETMTD0LOC_A::LOC3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == ETMTD0LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == ETMTD0LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == ETMTD0LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == ETMTD0LOC_A::LOC3
    }
}
#[doc = "Field `ETMTD0LOC` writer - I/O Location"]
pub type ETMTD0LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC1_SPEC, u8, ETMTD0LOC_A, 6, 8>;
impl<'a> ETMTD0LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(ETMTD0LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(ETMTD0LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(ETMTD0LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(ETMTD0LOC_A::LOC3)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ETMTD1LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
}
impl From<ETMTD1LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: ETMTD1LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ETMTD1LOC` reader - I/O Location"]
pub type ETMTD1LOC_R = crate::FieldReader<u8, ETMTD1LOC_A>;
impl ETMTD1LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ETMTD1LOC_A> {
        match self.bits {
            0 => Some(ETMTD1LOC_A::LOC0),
            1 => Some(ETMTD1LOC_A::LOC1),
            2 => Some(ETMTD1LOC_A::LOC2),
            3 => Some(ETMTD1LOC_A::LOC3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == ETMTD1LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == ETMTD1LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == ETMTD1LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == ETMTD1LOC_A::LOC3
    }
}
#[doc = "Field `ETMTD1LOC` writer - I/O Location"]
pub type ETMTD1LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC1_SPEC, u8, ETMTD1LOC_A, 6, 14>;
impl<'a> ETMTD1LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(ETMTD1LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(ETMTD1LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(ETMTD1LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(ETMTD1LOC_A::LOC3)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ETMTD2LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
}
impl From<ETMTD2LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: ETMTD2LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ETMTD2LOC` reader - I/O Location"]
pub type ETMTD2LOC_R = crate::FieldReader<u8, ETMTD2LOC_A>;
impl ETMTD2LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ETMTD2LOC_A> {
        match self.bits {
            0 => Some(ETMTD2LOC_A::LOC0),
            1 => Some(ETMTD2LOC_A::LOC1),
            2 => Some(ETMTD2LOC_A::LOC2),
            3 => Some(ETMTD2LOC_A::LOC3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == ETMTD2LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == ETMTD2LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == ETMTD2LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == ETMTD2LOC_A::LOC3
    }
}
#[doc = "Field `ETMTD2LOC` writer - I/O Location"]
pub type ETMTD2LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC1_SPEC, u8, ETMTD2LOC_A, 6, 20>;
impl<'a> ETMTD2LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(ETMTD2LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(ETMTD2LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(ETMTD2LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(ETMTD2LOC_A::LOC3)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ETMTD3LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
}
impl From<ETMTD3LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: ETMTD3LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ETMTD3LOC` reader - I/O Location"]
pub type ETMTD3LOC_R = crate::FieldReader<u8, ETMTD3LOC_A>;
impl ETMTD3LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ETMTD3LOC_A> {
        match self.bits {
            0 => Some(ETMTD3LOC_A::LOC0),
            1 => Some(ETMTD3LOC_A::LOC1),
            2 => Some(ETMTD3LOC_A::LOC2),
            3 => Some(ETMTD3LOC_A::LOC3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == ETMTD3LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == ETMTD3LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == ETMTD3LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == ETMTD3LOC_A::LOC3
    }
}
#[doc = "Field `ETMTD3LOC` writer - I/O Location"]
pub type ETMTD3LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC1_SPEC, u8, ETMTD3LOC_A, 6, 26>;
impl<'a> ETMTD3LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(ETMTD3LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(ETMTD3LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(ETMTD3LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(ETMTD3LOC_A::LOC3)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn etmtclkloc(&self) -> ETMTCLKLOC_R {
        ETMTCLKLOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn etmtd0loc(&self) -> ETMTD0LOC_R {
        ETMTD0LOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:19 - I/O Location"]
    #[inline(always)]
    pub fn etmtd1loc(&self) -> ETMTD1LOC_R {
        ETMTD1LOC_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bits 20:25 - I/O Location"]
    #[inline(always)]
    pub fn etmtd2loc(&self) -> ETMTD2LOC_R {
        ETMTD2LOC_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bits 26:31 - I/O Location"]
    #[inline(always)]
    pub fn etmtd3loc(&self) -> ETMTD3LOC_R {
        ETMTD3LOC_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn etmtclkloc(&mut self) -> ETMTCLKLOC_W {
        ETMTCLKLOC_W::new(self)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn etmtd0loc(&mut self) -> ETMTD0LOC_W {
        ETMTD0LOC_W::new(self)
    }
    #[doc = "Bits 14:19 - I/O Location"]
    #[inline(always)]
    pub fn etmtd1loc(&mut self) -> ETMTD1LOC_W {
        ETMTD1LOC_W::new(self)
    }
    #[doc = "Bits 20:25 - I/O Location"]
    #[inline(always)]
    pub fn etmtd2loc(&mut self) -> ETMTD2LOC_W {
        ETMTD2LOC_W::new(self)
    }
    #[doc = "Bits 26:31 - I/O Location"]
    #[inline(always)]
    pub fn etmtd3loc(&mut self) -> ETMTD3LOC_W {
        ETMTD3LOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Routing Location Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routeloc1](index.html) module"]
pub struct ROUTELOC1_SPEC;
impl crate::RegisterSpec for ROUTELOC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [routeloc1::R](R) reader structure"]
impl crate::Readable for ROUTELOC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [routeloc1::W](W) writer structure"]
impl crate::Writable for ROUTELOC1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROUTELOC1 to value 0"]
impl crate::Resettable for ROUTELOC1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
