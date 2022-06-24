#[doc = "Register `ROUTELOC0` reader"]
pub struct R(crate::R<ROUTELOC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROUTELOC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROUTELOC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROUTELOC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROUTELOC0` writer"]
pub struct W(crate::W<ROUTELOC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROUTELOC0_SPEC>;
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
impl From<crate::W<ROUTELOC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROUTELOC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "I/O Location for D0-7 Pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
}
impl From<DATLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: DATLOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DATLOC` reader - I/O Location for D0-7 Pins"]
pub type DATLOC_R = crate::FieldReader<u8, DATLOC_A>;
impl DATLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATLOC_A> {
        match self.bits {
            0 => Some(DATLOC_A::LOC0),
            1 => Some(DATLOC_A::LOC1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == DATLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == DATLOC_A::LOC1
    }
}
#[doc = "Field `DATLOC` writer - I/O Location for D0-7 Pins"]
pub type DATLOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC0_SPEC, u8, DATLOC_A, 6, 0>;
impl<'a> DATLOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(DATLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(DATLOC_A::LOC1)
    }
}
#[doc = "I/O Location for CD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CDLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
}
impl From<CDLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CDLOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CDLOC` reader - I/O Location for CD"]
pub type CDLOC_R = crate::FieldReader<u8, CDLOC_A>;
impl CDLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CDLOC_A> {
        match self.bits {
            0 => Some(CDLOC_A::LOC0),
            1 => Some(CDLOC_A::LOC1),
            2 => Some(CDLOC_A::LOC2),
            3 => Some(CDLOC_A::LOC3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CDLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CDLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CDLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CDLOC_A::LOC3
    }
}
#[doc = "Field `CDLOC` writer - I/O Location for CD"]
pub type CDLOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC0_SPEC, u8, CDLOC_A, 6, 8>;
impl<'a> CDLOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CDLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CDLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CDLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CDLOC_A::LOC3)
    }
}
#[doc = "I/O Location for WP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WPLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
}
impl From<WPLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: WPLOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WPLOC` reader - I/O Location for WP"]
pub type WPLOC_R = crate::FieldReader<u8, WPLOC_A>;
impl WPLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WPLOC_A> {
        match self.bits {
            0 => Some(WPLOC_A::LOC0),
            1 => Some(WPLOC_A::LOC1),
            2 => Some(WPLOC_A::LOC2),
            3 => Some(WPLOC_A::LOC3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == WPLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == WPLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == WPLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == WPLOC_A::LOC3
    }
}
#[doc = "Field `WPLOC` writer - I/O Location for WP"]
pub type WPLOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC0_SPEC, u8, WPLOC_A, 6, 16>;
impl<'a> WPLOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(WPLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(WPLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(WPLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(WPLOC_A::LOC3)
    }
}
#[doc = "I/O Location for CLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
}
impl From<CLKLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKLOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLKLOC` reader - I/O Location for CLK"]
pub type CLKLOC_R = crate::FieldReader<u8, CLKLOC_A>;
impl CLKLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKLOC_A> {
        match self.bits {
            0 => Some(CLKLOC_A::LOC0),
            1 => Some(CLKLOC_A::LOC1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CLKLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CLKLOC_A::LOC1
    }
}
#[doc = "Field `CLKLOC` writer - I/O Location for CLK"]
pub type CLKLOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC0_SPEC, u8, CLKLOC_A, 6, 24>;
impl<'a> CLKLOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CLKLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CLKLOC_A::LOC1)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location for D0-7 Pins"]
    #[inline(always)]
    pub fn datloc(&self) -> DATLOC_R {
        DATLOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location for CD"]
    #[inline(always)]
    pub fn cdloc(&self) -> CDLOC_R {
        CDLOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location for WP"]
    #[inline(always)]
    pub fn wploc(&self) -> WPLOC_R {
        WPLOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location for CLK"]
    #[inline(always)]
    pub fn clkloc(&self) -> CLKLOC_R {
        CLKLOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location for D0-7 Pins"]
    #[inline(always)]
    pub fn datloc(&mut self) -> DATLOC_W {
        DATLOC_W::new(self)
    }
    #[doc = "Bits 8:13 - I/O Location for CD"]
    #[inline(always)]
    pub fn cdloc(&mut self) -> CDLOC_W {
        CDLOC_W::new(self)
    }
    #[doc = "Bits 16:21 - I/O Location for WP"]
    #[inline(always)]
    pub fn wploc(&mut self) -> WPLOC_W {
        WPLOC_W::new(self)
    }
    #[doc = "Bits 24:29 - I/O Location for CLK"]
    #[inline(always)]
    pub fn clkloc(&mut self) -> CLKLOC_W {
        CLKLOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O LOCATION Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routeloc0](index.html) module"]
pub struct ROUTELOC0_SPEC;
impl crate::RegisterSpec for ROUTELOC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [routeloc0::R](R) reader structure"]
impl crate::Readable for ROUTELOC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [routeloc0::W](W) writer structure"]
impl crate::Writable for ROUTELOC0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROUTELOC0 to value 0"]
impl crate::Resettable for ROUTELOC0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
