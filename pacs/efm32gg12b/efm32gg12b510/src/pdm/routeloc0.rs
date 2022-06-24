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
#[doc = "I/O Location for DAT0 pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DAT0LOC_A {
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
}
impl From<DAT0LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: DAT0LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DAT0LOC` reader - I/O Location for DAT0 pins"]
pub type DAT0LOC_R = crate::FieldReader<u8, DAT0LOC_A>;
impl DAT0LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DAT0LOC_A> {
        match self.bits {
            0 => Some(DAT0LOC_A::LOC0),
            1 => Some(DAT0LOC_A::LOC1),
            2 => Some(DAT0LOC_A::LOC2),
            3 => Some(DAT0LOC_A::LOC3),
            4 => Some(DAT0LOC_A::LOC4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == DAT0LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == DAT0LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == DAT0LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == DAT0LOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == DAT0LOC_A::LOC4
    }
}
#[doc = "Field `DAT0LOC` writer - I/O Location for DAT0 pins"]
pub type DAT0LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC0_SPEC, u8, DAT0LOC_A, 6, 0>;
impl<'a> DAT0LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(DAT0LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(DAT0LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(DAT0LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(DAT0LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(DAT0LOC_A::LOC4)
    }
}
#[doc = "I/O Location for DAT1 pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DAT1LOC_A {
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
}
impl From<DAT1LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: DAT1LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DAT1LOC` reader - I/O Location for DAT1 pins"]
pub type DAT1LOC_R = crate::FieldReader<u8, DAT1LOC_A>;
impl DAT1LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DAT1LOC_A> {
        match self.bits {
            0 => Some(DAT1LOC_A::LOC0),
            1 => Some(DAT1LOC_A::LOC1),
            2 => Some(DAT1LOC_A::LOC2),
            3 => Some(DAT1LOC_A::LOC3),
            4 => Some(DAT1LOC_A::LOC4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == DAT1LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == DAT1LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == DAT1LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == DAT1LOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == DAT1LOC_A::LOC4
    }
}
#[doc = "Field `DAT1LOC` writer - I/O Location for DAT1 pins"]
pub type DAT1LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC0_SPEC, u8, DAT1LOC_A, 6, 8>;
impl<'a> DAT1LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(DAT1LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(DAT1LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(DAT1LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(DAT1LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(DAT1LOC_A::LOC4)
    }
}
#[doc = "I/O Location for DAT2 pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DAT2LOC_A {
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
}
impl From<DAT2LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: DAT2LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DAT2LOC` reader - I/O Location for DAT2 pins"]
pub type DAT2LOC_R = crate::FieldReader<u8, DAT2LOC_A>;
impl DAT2LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DAT2LOC_A> {
        match self.bits {
            0 => Some(DAT2LOC_A::LOC0),
            1 => Some(DAT2LOC_A::LOC1),
            2 => Some(DAT2LOC_A::LOC2),
            3 => Some(DAT2LOC_A::LOC3),
            4 => Some(DAT2LOC_A::LOC4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == DAT2LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == DAT2LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == DAT2LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == DAT2LOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == DAT2LOC_A::LOC4
    }
}
#[doc = "Field `DAT2LOC` writer - I/O Location for DAT2 pins"]
pub type DAT2LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC0_SPEC, u8, DAT2LOC_A, 6, 16>;
impl<'a> DAT2LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(DAT2LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(DAT2LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(DAT2LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(DAT2LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(DAT2LOC_A::LOC4)
    }
}
#[doc = "I/O Location for DAT3 pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DAT3LOC_A {
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
}
impl From<DAT3LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: DAT3LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DAT3LOC` reader - I/O Location for DAT3 pins"]
pub type DAT3LOC_R = crate::FieldReader<u8, DAT3LOC_A>;
impl DAT3LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DAT3LOC_A> {
        match self.bits {
            0 => Some(DAT3LOC_A::LOC0),
            1 => Some(DAT3LOC_A::LOC1),
            2 => Some(DAT3LOC_A::LOC2),
            3 => Some(DAT3LOC_A::LOC3),
            4 => Some(DAT3LOC_A::LOC4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == DAT3LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == DAT3LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == DAT3LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == DAT3LOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == DAT3LOC_A::LOC4
    }
}
#[doc = "Field `DAT3LOC` writer - I/O Location for DAT3 pins"]
pub type DAT3LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC0_SPEC, u8, DAT3LOC_A, 6, 24>;
impl<'a> DAT3LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(DAT3LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(DAT3LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(DAT3LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(DAT3LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(DAT3LOC_A::LOC4)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location for DAT0 pins"]
    #[inline(always)]
    pub fn dat0loc(&self) -> DAT0LOC_R {
        DAT0LOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location for DAT1 pins"]
    #[inline(always)]
    pub fn dat1loc(&self) -> DAT1LOC_R {
        DAT1LOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location for DAT2 pins"]
    #[inline(always)]
    pub fn dat2loc(&self) -> DAT2LOC_R {
        DAT2LOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location for DAT3 pins"]
    #[inline(always)]
    pub fn dat3loc(&self) -> DAT3LOC_R {
        DAT3LOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location for DAT0 pins"]
    #[inline(always)]
    pub fn dat0loc(&mut self) -> DAT0LOC_W {
        DAT0LOC_W::new(self)
    }
    #[doc = "Bits 8:13 - I/O Location for DAT1 pins"]
    #[inline(always)]
    pub fn dat1loc(&mut self) -> DAT1LOC_W {
        DAT1LOC_W::new(self)
    }
    #[doc = "Bits 16:21 - I/O Location for DAT2 pins"]
    #[inline(always)]
    pub fn dat2loc(&mut self) -> DAT2LOC_W {
        DAT2LOC_W::new(self)
    }
    #[doc = "Bits 24:29 - I/O Location for DAT3 pins"]
    #[inline(always)]
    pub fn dat3loc(&mut self) -> DAT3LOC_W {
        DAT3LOC_W::new(self)
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
