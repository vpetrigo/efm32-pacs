#[doc = "Register `EXTIPINSELH` reader"]
pub struct R(crate::R<EXTIPINSELH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTIPINSELH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTIPINSELH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTIPINSELH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTIPINSELH` writer"]
pub struct W(crate::W<EXTIPINSELH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTIPINSELH_SPEC>;
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
impl From<crate::W<EXTIPINSELH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTIPINSELH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "External Interrupt Pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPINSEL0_A {
    #[doc = "0: OFFSET=8"]
    OFFSET8 = 0,
    #[doc = "1: OFFSET=9"]
    OFFSET9 = 1,
    #[doc = "2: OFFSET=10"]
    OFFSET10 = 2,
    #[doc = "3: OFFSET=11"]
    OFFSET11 = 3,
}
impl From<EXTIPINSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTIPINSEL0` reader - External Interrupt Pin select"]
pub type EXTIPINSEL0_R = crate::FieldReader<u8, EXTIPINSEL0_A>;
impl EXTIPINSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL0_A {
        match self.bits {
            0 => EXTIPINSEL0_A::OFFSET8,
            1 => EXTIPINSEL0_A::OFFSET9,
            2 => EXTIPINSEL0_A::OFFSET10,
            3 => EXTIPINSEL0_A::OFFSET11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFFSET8`"]
    #[inline(always)]
    pub fn is_offset8(&self) -> bool {
        *self == EXTIPINSEL0_A::OFFSET8
    }
    #[doc = "Checks if the value of the field is `OFFSET9`"]
    #[inline(always)]
    pub fn is_offset9(&self) -> bool {
        *self == EXTIPINSEL0_A::OFFSET9
    }
    #[doc = "Checks if the value of the field is `OFFSET10`"]
    #[inline(always)]
    pub fn is_offset10(&self) -> bool {
        *self == EXTIPINSEL0_A::OFFSET10
    }
    #[doc = "Checks if the value of the field is `OFFSET11`"]
    #[inline(always)]
    pub fn is_offset11(&self) -> bool {
        *self == EXTIPINSEL0_A::OFFSET11
    }
}
#[doc = "Field `EXTIPINSEL0` writer - External Interrupt Pin select"]
pub type EXTIPINSEL0_W<'a> =
    crate::FieldWriterSafe<'a, u32, EXTIPINSELH_SPEC, u8, EXTIPINSEL0_A, 2, 0>;
impl<'a> EXTIPINSEL0_W<'a> {
    #[doc = "OFFSET=8"]
    #[inline(always)]
    pub fn offset8(self) -> &'a mut W {
        self.variant(EXTIPINSEL0_A::OFFSET8)
    }
    #[doc = "OFFSET=9"]
    #[inline(always)]
    pub fn offset9(self) -> &'a mut W {
        self.variant(EXTIPINSEL0_A::OFFSET9)
    }
    #[doc = "OFFSET=10"]
    #[inline(always)]
    pub fn offset10(self) -> &'a mut W {
        self.variant(EXTIPINSEL0_A::OFFSET10)
    }
    #[doc = "OFFSET=11"]
    #[inline(always)]
    pub fn offset11(self) -> &'a mut W {
        self.variant(EXTIPINSEL0_A::OFFSET11)
    }
}
#[doc = "External Interrupt Pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPINSEL1_A {
    #[doc = "0: OFFSET=8"]
    OFFSET8 = 0,
    #[doc = "1: OFFSET=9"]
    OFFSET9 = 1,
    #[doc = "2: OFFSET=10"]
    OFFSET10 = 2,
    #[doc = "3: OFFSET=11"]
    OFFSET11 = 3,
}
impl From<EXTIPINSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTIPINSEL1` reader - External Interrupt Pin select"]
pub type EXTIPINSEL1_R = crate::FieldReader<u8, EXTIPINSEL1_A>;
impl EXTIPINSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL1_A {
        match self.bits {
            0 => EXTIPINSEL1_A::OFFSET8,
            1 => EXTIPINSEL1_A::OFFSET9,
            2 => EXTIPINSEL1_A::OFFSET10,
            3 => EXTIPINSEL1_A::OFFSET11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFFSET8`"]
    #[inline(always)]
    pub fn is_offset8(&self) -> bool {
        *self == EXTIPINSEL1_A::OFFSET8
    }
    #[doc = "Checks if the value of the field is `OFFSET9`"]
    #[inline(always)]
    pub fn is_offset9(&self) -> bool {
        *self == EXTIPINSEL1_A::OFFSET9
    }
    #[doc = "Checks if the value of the field is `OFFSET10`"]
    #[inline(always)]
    pub fn is_offset10(&self) -> bool {
        *self == EXTIPINSEL1_A::OFFSET10
    }
    #[doc = "Checks if the value of the field is `OFFSET11`"]
    #[inline(always)]
    pub fn is_offset11(&self) -> bool {
        *self == EXTIPINSEL1_A::OFFSET11
    }
}
#[doc = "Field `EXTIPINSEL1` writer - External Interrupt Pin select"]
pub type EXTIPINSEL1_W<'a> =
    crate::FieldWriterSafe<'a, u32, EXTIPINSELH_SPEC, u8, EXTIPINSEL1_A, 2, 4>;
impl<'a> EXTIPINSEL1_W<'a> {
    #[doc = "OFFSET=8"]
    #[inline(always)]
    pub fn offset8(self) -> &'a mut W {
        self.variant(EXTIPINSEL1_A::OFFSET8)
    }
    #[doc = "OFFSET=9"]
    #[inline(always)]
    pub fn offset9(self) -> &'a mut W {
        self.variant(EXTIPINSEL1_A::OFFSET9)
    }
    #[doc = "OFFSET=10"]
    #[inline(always)]
    pub fn offset10(self) -> &'a mut W {
        self.variant(EXTIPINSEL1_A::OFFSET10)
    }
    #[doc = "OFFSET=11"]
    #[inline(always)]
    pub fn offset11(self) -> &'a mut W {
        self.variant(EXTIPINSEL1_A::OFFSET11)
    }
}
#[doc = "External Interrupt Pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPINSEL2_A {
    #[doc = "0: OFFSET=8"]
    OFFSET8 = 0,
    #[doc = "1: OFFSET=9"]
    OFFSET9 = 1,
    #[doc = "2: OFFSET=10"]
    OFFSET10 = 2,
    #[doc = "3: OFFSET=11"]
    OFFSET11 = 3,
}
impl From<EXTIPINSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTIPINSEL2` reader - External Interrupt Pin select"]
pub type EXTIPINSEL2_R = crate::FieldReader<u8, EXTIPINSEL2_A>;
impl EXTIPINSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL2_A {
        match self.bits {
            0 => EXTIPINSEL2_A::OFFSET8,
            1 => EXTIPINSEL2_A::OFFSET9,
            2 => EXTIPINSEL2_A::OFFSET10,
            3 => EXTIPINSEL2_A::OFFSET11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFFSET8`"]
    #[inline(always)]
    pub fn is_offset8(&self) -> bool {
        *self == EXTIPINSEL2_A::OFFSET8
    }
    #[doc = "Checks if the value of the field is `OFFSET9`"]
    #[inline(always)]
    pub fn is_offset9(&self) -> bool {
        *self == EXTIPINSEL2_A::OFFSET9
    }
    #[doc = "Checks if the value of the field is `OFFSET10`"]
    #[inline(always)]
    pub fn is_offset10(&self) -> bool {
        *self == EXTIPINSEL2_A::OFFSET10
    }
    #[doc = "Checks if the value of the field is `OFFSET11`"]
    #[inline(always)]
    pub fn is_offset11(&self) -> bool {
        *self == EXTIPINSEL2_A::OFFSET11
    }
}
#[doc = "Field `EXTIPINSEL2` writer - External Interrupt Pin select"]
pub type EXTIPINSEL2_W<'a> =
    crate::FieldWriterSafe<'a, u32, EXTIPINSELH_SPEC, u8, EXTIPINSEL2_A, 2, 8>;
impl<'a> EXTIPINSEL2_W<'a> {
    #[doc = "OFFSET=8"]
    #[inline(always)]
    pub fn offset8(self) -> &'a mut W {
        self.variant(EXTIPINSEL2_A::OFFSET8)
    }
    #[doc = "OFFSET=9"]
    #[inline(always)]
    pub fn offset9(self) -> &'a mut W {
        self.variant(EXTIPINSEL2_A::OFFSET9)
    }
    #[doc = "OFFSET=10"]
    #[inline(always)]
    pub fn offset10(self) -> &'a mut W {
        self.variant(EXTIPINSEL2_A::OFFSET10)
    }
    #[doc = "OFFSET=11"]
    #[inline(always)]
    pub fn offset11(self) -> &'a mut W {
        self.variant(EXTIPINSEL2_A::OFFSET11)
    }
}
#[doc = "External Interrupt Pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPINSEL3_A {
    #[doc = "0: OFFSET=8"]
    OFFSET8 = 0,
    #[doc = "1: OFFSET=9"]
    OFFSET9 = 1,
    #[doc = "2: OFFSET=10"]
    OFFSET10 = 2,
    #[doc = "3: OFFSET=11"]
    OFFSET11 = 3,
}
impl From<EXTIPINSEL3_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTIPINSEL3` reader - External Interrupt Pin select"]
pub type EXTIPINSEL3_R = crate::FieldReader<u8, EXTIPINSEL3_A>;
impl EXTIPINSEL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL3_A {
        match self.bits {
            0 => EXTIPINSEL3_A::OFFSET8,
            1 => EXTIPINSEL3_A::OFFSET9,
            2 => EXTIPINSEL3_A::OFFSET10,
            3 => EXTIPINSEL3_A::OFFSET11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFFSET8`"]
    #[inline(always)]
    pub fn is_offset8(&self) -> bool {
        *self == EXTIPINSEL3_A::OFFSET8
    }
    #[doc = "Checks if the value of the field is `OFFSET9`"]
    #[inline(always)]
    pub fn is_offset9(&self) -> bool {
        *self == EXTIPINSEL3_A::OFFSET9
    }
    #[doc = "Checks if the value of the field is `OFFSET10`"]
    #[inline(always)]
    pub fn is_offset10(&self) -> bool {
        *self == EXTIPINSEL3_A::OFFSET10
    }
    #[doc = "Checks if the value of the field is `OFFSET11`"]
    #[inline(always)]
    pub fn is_offset11(&self) -> bool {
        *self == EXTIPINSEL3_A::OFFSET11
    }
}
#[doc = "Field `EXTIPINSEL3` writer - External Interrupt Pin select"]
pub type EXTIPINSEL3_W<'a> =
    crate::FieldWriterSafe<'a, u32, EXTIPINSELH_SPEC, u8, EXTIPINSEL3_A, 2, 12>;
impl<'a> EXTIPINSEL3_W<'a> {
    #[doc = "OFFSET=8"]
    #[inline(always)]
    pub fn offset8(self) -> &'a mut W {
        self.variant(EXTIPINSEL3_A::OFFSET8)
    }
    #[doc = "OFFSET=9"]
    #[inline(always)]
    pub fn offset9(self) -> &'a mut W {
        self.variant(EXTIPINSEL3_A::OFFSET9)
    }
    #[doc = "OFFSET=10"]
    #[inline(always)]
    pub fn offset10(self) -> &'a mut W {
        self.variant(EXTIPINSEL3_A::OFFSET10)
    }
    #[doc = "OFFSET=11"]
    #[inline(always)]
    pub fn offset11(self) -> &'a mut W {
        self.variant(EXTIPINSEL3_A::OFFSET11)
    }
}
impl R {
    #[doc = "Bits 0:1 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel0(&self) -> EXTIPINSEL0_R {
        EXTIPINSEL0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel1(&self) -> EXTIPINSEL1_R {
        EXTIPINSEL1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel2(&self) -> EXTIPINSEL2_R {
        EXTIPINSEL2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel3(&self) -> EXTIPINSEL3_R {
        EXTIPINSEL3_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel0(&mut self) -> EXTIPINSEL0_W {
        EXTIPINSEL0_W::new(self)
    }
    #[doc = "Bits 4:5 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel1(&mut self) -> EXTIPINSEL1_W {
        EXTIPINSEL1_W::new(self)
    }
    #[doc = "Bits 8:9 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel2(&mut self) -> EXTIPINSEL2_W {
        EXTIPINSEL2_W::new(self)
    }
    #[doc = "Bits 12:13 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel3(&mut self) -> EXTIPINSEL3_W {
        EXTIPINSEL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Interrupt Pin Select High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extipinselh](index.html) module"]
pub struct EXTIPINSELH_SPEC;
impl crate::RegisterSpec for EXTIPINSELH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extipinselh::R](R) reader structure"]
impl crate::Readable for EXTIPINSELH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extipinselh::W](W) writer structure"]
impl crate::Writable for EXTIPINSELH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTIPINSELH to value 0"]
impl crate::Resettable for EXTIPINSELH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
