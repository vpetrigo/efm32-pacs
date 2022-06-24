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
pub enum TSUEXTCLKLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
}
impl From<TSUEXTCLKLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: TSUEXTCLKLOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TSUEXTCLKLOC` reader - I/O Location"]
pub type TSUEXTCLKLOC_R = crate::FieldReader<u8, TSUEXTCLKLOC_A>;
impl TSUEXTCLKLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSUEXTCLKLOC_A> {
        match self.bits {
            0 => Some(TSUEXTCLKLOC_A::LOC0),
            1 => Some(TSUEXTCLKLOC_A::LOC1),
            2 => Some(TSUEXTCLKLOC_A::LOC2),
            3 => Some(TSUEXTCLKLOC_A::LOC3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == TSUEXTCLKLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == TSUEXTCLKLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == TSUEXTCLKLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == TSUEXTCLKLOC_A::LOC3
    }
}
#[doc = "Field `TSUEXTCLKLOC` writer - I/O Location"]
pub type TSUEXTCLKLOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC1_SPEC, u8, TSUEXTCLKLOC_A, 6, 0>;
impl<'a> TSUEXTCLKLOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(TSUEXTCLKLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(TSUEXTCLKLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(TSUEXTCLKLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(TSUEXTCLKLOC_A::LOC3)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSUTMRTOGLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
}
impl From<TSUTMRTOGLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: TSUTMRTOGLOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TSUTMRTOGLOC` reader - I/O Location"]
pub type TSUTMRTOGLOC_R = crate::FieldReader<u8, TSUTMRTOGLOC_A>;
impl TSUTMRTOGLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSUTMRTOGLOC_A> {
        match self.bits {
            0 => Some(TSUTMRTOGLOC_A::LOC0),
            1 => Some(TSUTMRTOGLOC_A::LOC1),
            2 => Some(TSUTMRTOGLOC_A::LOC2),
            3 => Some(TSUTMRTOGLOC_A::LOC3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == TSUTMRTOGLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == TSUTMRTOGLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == TSUTMRTOGLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == TSUTMRTOGLOC_A::LOC3
    }
}
#[doc = "Field `TSUTMRTOGLOC` writer - I/O Location"]
pub type TSUTMRTOGLOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC1_SPEC, u8, TSUTMRTOGLOC_A, 6, 8>;
impl<'a> TSUTMRTOGLOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(TSUTMRTOGLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(TSUTMRTOGLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(TSUTMRTOGLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(TSUTMRTOGLOC_A::LOC3)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MDIOLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
}
impl From<MDIOLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: MDIOLOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MDIOLOC` reader - I/O Location"]
pub type MDIOLOC_R = crate::FieldReader<u8, MDIOLOC_A>;
impl MDIOLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MDIOLOC_A> {
        match self.bits {
            0 => Some(MDIOLOC_A::LOC0),
            1 => Some(MDIOLOC_A::LOC1),
            2 => Some(MDIOLOC_A::LOC2),
            3 => Some(MDIOLOC_A::LOC3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == MDIOLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == MDIOLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == MDIOLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == MDIOLOC_A::LOC3
    }
}
#[doc = "Field `MDIOLOC` writer - I/O Location"]
pub type MDIOLOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC1_SPEC, u8, MDIOLOC_A, 6, 16>;
impl<'a> MDIOLOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(MDIOLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(MDIOLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(MDIOLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(MDIOLOC_A::LOC3)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RMIILOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
}
impl From<RMIILOC_A> for u8 {
    #[inline(always)]
    fn from(variant: RMIILOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RMIILOC` reader - I/O Location"]
pub type RMIILOC_R = crate::FieldReader<u8, RMIILOC_A>;
impl RMIILOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RMIILOC_A> {
        match self.bits {
            0 => Some(RMIILOC_A::LOC0),
            1 => Some(RMIILOC_A::LOC1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == RMIILOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == RMIILOC_A::LOC1
    }
}
#[doc = "Field `RMIILOC` writer - I/O Location"]
pub type RMIILOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC1_SPEC, u8, RMIILOC_A, 6, 24>;
impl<'a> RMIILOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(RMIILOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(RMIILOC_A::LOC1)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn tsuextclkloc(&self) -> TSUEXTCLKLOC_R {
        TSUEXTCLKLOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn tsutmrtogloc(&self) -> TSUTMRTOGLOC_R {
        TSUTMRTOGLOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn mdioloc(&self) -> MDIOLOC_R {
        MDIOLOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn rmiiloc(&self) -> RMIILOC_R {
        RMIILOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn tsuextclkloc(&mut self) -> TSUEXTCLKLOC_W {
        TSUEXTCLKLOC_W::new(self)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn tsutmrtogloc(&mut self) -> TSUTMRTOGLOC_W {
        TSUTMRTOGLOC_W::new(self)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn mdioloc(&mut self) -> MDIOLOC_W {
        MDIOLOC_W::new(self)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn rmiiloc(&mut self) -> RMIILOC_W {
        RMIILOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Route Location Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routeloc1](index.html) module"]
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
