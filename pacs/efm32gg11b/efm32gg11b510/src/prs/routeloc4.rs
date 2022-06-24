#[doc = "Register `ROUTELOC4` reader"]
pub struct R(crate::R<ROUTELOC4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROUTELOC4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROUTELOC4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROUTELOC4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROUTELOC4` writer"]
pub struct W(crate::W<ROUTELOC4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROUTELOC4_SPEC>;
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
impl From<crate::W<ROUTELOC4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROUTELOC4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH16LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH16LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH16LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH16LOC` reader - I/O Location"]
pub type CH16LOC_R = crate::FieldReader<u8, CH16LOC_A>;
impl CH16LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH16LOC_A> {
        match self.bits {
            0 => Some(CH16LOC_A::LOC0),
            1 => Some(CH16LOC_A::LOC1),
            2 => Some(CH16LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH16LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH16LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH16LOC_A::LOC2
    }
}
#[doc = "Field `CH16LOC` writer - I/O Location"]
pub type CH16LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC4_SPEC, u8, CH16LOC_A, 6, 0>;
impl<'a> CH16LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH16LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH16LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH16LOC_A::LOC2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH17LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH17LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH17LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH17LOC` reader - I/O Location"]
pub type CH17LOC_R = crate::FieldReader<u8, CH17LOC_A>;
impl CH17LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH17LOC_A> {
        match self.bits {
            0 => Some(CH17LOC_A::LOC0),
            1 => Some(CH17LOC_A::LOC1),
            2 => Some(CH17LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH17LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH17LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH17LOC_A::LOC2
    }
}
#[doc = "Field `CH17LOC` writer - I/O Location"]
pub type CH17LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC4_SPEC, u8, CH17LOC_A, 6, 8>;
impl<'a> CH17LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH17LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH17LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH17LOC_A::LOC2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH18LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH18LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH18LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH18LOC` reader - I/O Location"]
pub type CH18LOC_R = crate::FieldReader<u8, CH18LOC_A>;
impl CH18LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH18LOC_A> {
        match self.bits {
            0 => Some(CH18LOC_A::LOC0),
            1 => Some(CH18LOC_A::LOC1),
            2 => Some(CH18LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH18LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH18LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH18LOC_A::LOC2
    }
}
#[doc = "Field `CH18LOC` writer - I/O Location"]
pub type CH18LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC4_SPEC, u8, CH18LOC_A, 6, 16>;
impl<'a> CH18LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH18LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH18LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH18LOC_A::LOC2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH19LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH19LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH19LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH19LOC` reader - I/O Location"]
pub type CH19LOC_R = crate::FieldReader<u8, CH19LOC_A>;
impl CH19LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH19LOC_A> {
        match self.bits {
            0 => Some(CH19LOC_A::LOC0),
            1 => Some(CH19LOC_A::LOC1),
            2 => Some(CH19LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH19LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH19LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH19LOC_A::LOC2
    }
}
#[doc = "Field `CH19LOC` writer - I/O Location"]
pub type CH19LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC4_SPEC, u8, CH19LOC_A, 6, 24>;
impl<'a> CH19LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH19LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH19LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH19LOC_A::LOC2)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch16loc(&self) -> CH16LOC_R {
        CH16LOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch17loc(&self) -> CH17LOC_R {
        CH17LOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch18loc(&self) -> CH18LOC_R {
        CH18LOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch19loc(&self) -> CH19LOC_R {
        CH19LOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch16loc(&mut self) -> CH16LOC_W {
        CH16LOC_W::new(self)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch17loc(&mut self) -> CH17LOC_W {
        CH17LOC_W::new(self)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch18loc(&mut self) -> CH18LOC_W {
        CH18LOC_W::new(self)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch19loc(&mut self) -> CH19LOC_W {
        CH19LOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Routing Location Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routeloc4](index.html) module"]
pub struct ROUTELOC4_SPEC;
impl crate::RegisterSpec for ROUTELOC4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [routeloc4::R](R) reader structure"]
impl crate::Readable for ROUTELOC4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [routeloc4::W](W) writer structure"]
impl crate::Writable for ROUTELOC4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROUTELOC4 to value 0"]
impl crate::Resettable for ROUTELOC4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
