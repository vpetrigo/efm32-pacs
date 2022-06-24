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
pub enum ADLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
}
impl From<ADLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: ADLOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADLOC` reader - I/O Location"]
pub type ADLOC_R = crate::FieldReader<u8, ADLOC_A>;
impl ADLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADLOC_A> {
        match self.bits {
            0 => Some(ADLOC_A::LOC0),
            1 => Some(ADLOC_A::LOC1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == ADLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == ADLOC_A::LOC1
    }
}
#[doc = "Field `ADLOC` writer - I/O Location"]
pub type ADLOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC1_SPEC, u8, ADLOC_A, 6, 0>;
impl<'a> ADLOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(ADLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(ADLOC_A::LOC1)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ALOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
}
impl From<ALOC_A> for u8 {
    #[inline(always)]
    fn from(variant: ALOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ALOC` reader - I/O Location"]
pub type ALOC_R = crate::FieldReader<u8, ALOC_A>;
impl ALOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ALOC_A> {
        match self.bits {
            0 => Some(ALOC_A::LOC0),
            1 => Some(ALOC_A::LOC1),
            2 => Some(ALOC_A::LOC2),
            3 => Some(ALOC_A::LOC3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == ALOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == ALOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == ALOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == ALOC_A::LOC3
    }
}
#[doc = "Field `ALOC` writer - I/O Location"]
pub type ALOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC1_SPEC, u8, ALOC_A, 6, 8>;
impl<'a> ALOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(ALOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(ALOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(ALOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(ALOC_A::LOC3)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RDYLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
    #[doc = "4: Location 4"]
    LOC4 = 4,
    #[doc = "5: Location 5"]
    LOC5 = 5,
}
impl From<RDYLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: RDYLOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RDYLOC` reader - I/O Location"]
pub type RDYLOC_R = crate::FieldReader<u8, RDYLOC_A>;
impl RDYLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RDYLOC_A> {
        match self.bits {
            0 => Some(RDYLOC_A::LOC0),
            1 => Some(RDYLOC_A::LOC1),
            2 => Some(RDYLOC_A::LOC2),
            3 => Some(RDYLOC_A::LOC3),
            4 => Some(RDYLOC_A::LOC4),
            5 => Some(RDYLOC_A::LOC5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == RDYLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == RDYLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == RDYLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == RDYLOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == RDYLOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == RDYLOC_A::LOC5
    }
}
#[doc = "Field `RDYLOC` writer - I/O Location"]
pub type RDYLOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC1_SPEC, u8, RDYLOC_A, 6, 16>;
impl<'a> RDYLOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(RDYLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(RDYLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(RDYLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(RDYLOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(RDYLOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(RDYLOC_A::LOC5)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn adloc(&self) -> ADLOC_R {
        ADLOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn aloc(&self) -> ALOC_R {
        ALOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn rdyloc(&self) -> RDYLOC_R {
        RDYLOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn adloc(&mut self) -> ADLOC_W {
        ADLOC_W::new(self)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn aloc(&mut self) -> ALOC_W {
        ALOC_W::new(self)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn rdyloc(&mut self) -> RDYLOC_W {
        RDYLOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Routing Location Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routeloc1](index.html) module"]
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
