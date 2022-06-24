#[doc = "Register `ROUTELOC3` reader"]
pub struct R(crate::R<ROUTELOC3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROUTELOC3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROUTELOC3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROUTELOC3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROUTELOC3` writer"]
pub struct W(crate::W<ROUTELOC3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROUTELOC3_SPEC>;
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
impl From<crate::W<ROUTELOC3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROUTELOC3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH12LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH12LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH12LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH12LOC` reader - I/O Location"]
pub type CH12LOC_R = crate::FieldReader<u8, CH12LOC_A>;
impl CH12LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH12LOC_A> {
        match self.bits {
            0 => Some(CH12LOC_A::LOC0),
            1 => Some(CH12LOC_A::LOC1),
            2 => Some(CH12LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH12LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH12LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH12LOC_A::LOC2
    }
}
#[doc = "Field `CH12LOC` writer - I/O Location"]
pub type CH12LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC3_SPEC, u8, CH12LOC_A, 6, 0>;
impl<'a> CH12LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH12LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH12LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH12LOC_A::LOC2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH13LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH13LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH13LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH13LOC` reader - I/O Location"]
pub type CH13LOC_R = crate::FieldReader<u8, CH13LOC_A>;
impl CH13LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH13LOC_A> {
        match self.bits {
            0 => Some(CH13LOC_A::LOC0),
            1 => Some(CH13LOC_A::LOC1),
            2 => Some(CH13LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH13LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH13LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH13LOC_A::LOC2
    }
}
#[doc = "Field `CH13LOC` writer - I/O Location"]
pub type CH13LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC3_SPEC, u8, CH13LOC_A, 6, 8>;
impl<'a> CH13LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH13LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH13LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH13LOC_A::LOC2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH14LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH14LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH14LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH14LOC` reader - I/O Location"]
pub type CH14LOC_R = crate::FieldReader<u8, CH14LOC_A>;
impl CH14LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH14LOC_A> {
        match self.bits {
            0 => Some(CH14LOC_A::LOC0),
            1 => Some(CH14LOC_A::LOC1),
            2 => Some(CH14LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH14LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH14LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH14LOC_A::LOC2
    }
}
#[doc = "Field `CH14LOC` writer - I/O Location"]
pub type CH14LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC3_SPEC, u8, CH14LOC_A, 6, 16>;
impl<'a> CH14LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH14LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH14LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH14LOC_A::LOC2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH15LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH15LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH15LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH15LOC` reader - I/O Location"]
pub type CH15LOC_R = crate::FieldReader<u8, CH15LOC_A>;
impl CH15LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH15LOC_A> {
        match self.bits {
            0 => Some(CH15LOC_A::LOC0),
            1 => Some(CH15LOC_A::LOC1),
            2 => Some(CH15LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH15LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH15LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH15LOC_A::LOC2
    }
}
#[doc = "Field `CH15LOC` writer - I/O Location"]
pub type CH15LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC3_SPEC, u8, CH15LOC_A, 6, 24>;
impl<'a> CH15LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH15LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH15LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH15LOC_A::LOC2)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch12loc(&self) -> CH12LOC_R {
        CH12LOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch13loc(&self) -> CH13LOC_R {
        CH13LOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch14loc(&self) -> CH14LOC_R {
        CH14LOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch15loc(&self) -> CH15LOC_R {
        CH15LOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch12loc(&mut self) -> CH12LOC_W {
        CH12LOC_W::new(self)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch13loc(&mut self) -> CH13LOC_W {
        CH13LOC_W::new(self)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch14loc(&mut self) -> CH14LOC_W {
        CH14LOC_W::new(self)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch15loc(&mut self) -> CH15LOC_W {
        CH15LOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Routing Location Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routeloc3](index.html) module"]
pub struct ROUTELOC3_SPEC;
impl crate::RegisterSpec for ROUTELOC3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [routeloc3::R](R) reader structure"]
impl crate::Readable for ROUTELOC3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [routeloc3::W](W) writer structure"]
impl crate::Writable for ROUTELOC3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROUTELOC3 to value 0"]
impl crate::Resettable for ROUTELOC3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
