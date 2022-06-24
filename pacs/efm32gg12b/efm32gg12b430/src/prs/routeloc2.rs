#[doc = "Register `ROUTELOC2` reader"]
pub struct R(crate::R<ROUTELOC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROUTELOC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROUTELOC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROUTELOC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROUTELOC2` writer"]
pub struct W(crate::W<ROUTELOC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROUTELOC2_SPEC>;
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
impl From<crate::W<ROUTELOC2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROUTELOC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH8LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH8LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH8LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH8LOC` reader - I/O Location"]
pub type CH8LOC_R = crate::FieldReader<u8, CH8LOC_A>;
impl CH8LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH8LOC_A> {
        match self.bits {
            0 => Some(CH8LOC_A::LOC0),
            1 => Some(CH8LOC_A::LOC1),
            2 => Some(CH8LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH8LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH8LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH8LOC_A::LOC2
    }
}
#[doc = "Field `CH8LOC` writer - I/O Location"]
pub type CH8LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC2_SPEC, u8, CH8LOC_A, 6, 0>;
impl<'a> CH8LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH8LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH8LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH8LOC_A::LOC2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH9LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH9LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH9LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH9LOC` reader - I/O Location"]
pub type CH9LOC_R = crate::FieldReader<u8, CH9LOC_A>;
impl CH9LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH9LOC_A> {
        match self.bits {
            0 => Some(CH9LOC_A::LOC0),
            1 => Some(CH9LOC_A::LOC1),
            2 => Some(CH9LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH9LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH9LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH9LOC_A::LOC2
    }
}
#[doc = "Field `CH9LOC` writer - I/O Location"]
pub type CH9LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC2_SPEC, u8, CH9LOC_A, 6, 8>;
impl<'a> CH9LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH9LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH9LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH9LOC_A::LOC2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH10LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH10LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH10LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH10LOC` reader - I/O Location"]
pub type CH10LOC_R = crate::FieldReader<u8, CH10LOC_A>;
impl CH10LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH10LOC_A> {
        match self.bits {
            0 => Some(CH10LOC_A::LOC0),
            1 => Some(CH10LOC_A::LOC1),
            2 => Some(CH10LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH10LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH10LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH10LOC_A::LOC2
    }
}
#[doc = "Field `CH10LOC` writer - I/O Location"]
pub type CH10LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC2_SPEC, u8, CH10LOC_A, 6, 16>;
impl<'a> CH10LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH10LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH10LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH10LOC_A::LOC2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH11LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH11LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH11LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH11LOC` reader - I/O Location"]
pub type CH11LOC_R = crate::FieldReader<u8, CH11LOC_A>;
impl CH11LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH11LOC_A> {
        match self.bits {
            0 => Some(CH11LOC_A::LOC0),
            1 => Some(CH11LOC_A::LOC1),
            2 => Some(CH11LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH11LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH11LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH11LOC_A::LOC2
    }
}
#[doc = "Field `CH11LOC` writer - I/O Location"]
pub type CH11LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC2_SPEC, u8, CH11LOC_A, 6, 24>;
impl<'a> CH11LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH11LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH11LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH11LOC_A::LOC2)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch8loc(&self) -> CH8LOC_R {
        CH8LOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch9loc(&self) -> CH9LOC_R {
        CH9LOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch10loc(&self) -> CH10LOC_R {
        CH10LOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch11loc(&self) -> CH11LOC_R {
        CH11LOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch8loc(&mut self) -> CH8LOC_W {
        CH8LOC_W::new(self)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch9loc(&mut self) -> CH9LOC_W {
        CH9LOC_W::new(self)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch10loc(&mut self) -> CH10LOC_W {
        CH10LOC_W::new(self)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch11loc(&mut self) -> CH11LOC_W {
        CH11LOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Routing Location Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routeloc2](index.html) module"]
pub struct ROUTELOC2_SPEC;
impl crate::RegisterSpec for ROUTELOC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [routeloc2::R](R) reader structure"]
impl crate::Readable for ROUTELOC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [routeloc2::W](W) writer structure"]
impl crate::Writable for ROUTELOC2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROUTELOC2 to value 0"]
impl crate::Resettable for ROUTELOC2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
