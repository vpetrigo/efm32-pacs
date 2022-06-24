#[doc = "Register `COMMAND` reader"]
pub struct R(crate::R<COMMAND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMMAND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMMAND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMMAND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMMAND` writer"]
pub struct W(crate::W<COMMAND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMMAND_SPEC>;
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
impl From<crate::W<COMMAND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMMAND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPERATION` reader - Type of Operation"]
pub type OPERATION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPERATION` writer - Type of Operation"]
pub type OPERATION_W<'a> = crate::FieldWriter<'a, u32, COMMAND_SPEC, u8, u8, 7, 0>;
#[doc = "Field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIELD_A {
    #[doc = "0: Field is GF(p)"]
    GFP = 0,
    #[doc = "1: Field is GF(2^m)"]
    GF2M = 1,
}
impl From<FIELD_A> for bool {
    #[inline(always)]
    fn from(variant: FIELD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIELD` reader - Field"]
pub type FIELD_R = crate::BitReader<FIELD_A>;
impl FIELD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIELD_A {
        match self.bits {
            false => FIELD_A::GFP,
            true => FIELD_A::GF2M,
        }
    }
    #[doc = "Checks if the value of the field is `GFP`"]
    #[inline(always)]
    pub fn is_gfp(&self) -> bool {
        *self == FIELD_A::GFP
    }
    #[doc = "Checks if the value of the field is `GF2M`"]
    #[inline(always)]
    pub fn is_gf2m(&self) -> bool {
        *self == FIELD_A::GF2M
    }
}
#[doc = "Field `FIELD` writer - Field"]
pub type FIELD_W<'a> = crate::BitWriter<'a, u32, COMMAND_SPEC, FIELD_A, 7>;
impl<'a> FIELD_W<'a> {
    #[doc = "Field is GF(p)"]
    #[inline(always)]
    pub fn gfp(self) -> &'a mut W {
        self.variant(FIELD_A::GFP)
    }
    #[doc = "Field is GF(2^m)"]
    #[inline(always)]
    pub fn gf2m(self) -> &'a mut W {
        self.variant(FIELD_A::GF2M)
    }
}
#[doc = "Field `SIZE` reader - Size of Operands in data memory"]
pub type SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SIZE` writer - Size of Operands in data memory"]
pub type SIZE_W<'a> = crate::FieldWriter<'a, u32, COMMAND_SPEC, u16, u16, 11, 8>;
#[doc = "Select Curve\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SELCURVE_A {
    #[doc = "0: No acceleration"]
    NONE = 0,
    #[doc = "1: P256"]
    P256 = 1,
    #[doc = "4: P192"]
    P192 = 4,
}
impl From<SELCURVE_A> for u8 {
    #[inline(always)]
    fn from(variant: SELCURVE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SELCURVE` reader - Select Curve"]
pub type SELCURVE_R = crate::FieldReader<u8, SELCURVE_A>;
impl SELCURVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SELCURVE_A> {
        match self.bits {
            0 => Some(SELCURVE_A::NONE),
            1 => Some(SELCURVE_A::P256),
            4 => Some(SELCURVE_A::P192),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SELCURVE_A::NONE
    }
    #[doc = "Checks if the value of the field is `P256`"]
    #[inline(always)]
    pub fn is_p256(&self) -> bool {
        *self == SELCURVE_A::P256
    }
    #[doc = "Checks if the value of the field is `P192`"]
    #[inline(always)]
    pub fn is_p192(&self) -> bool {
        *self == SELCURVE_A::P192
    }
}
#[doc = "Field `SELCURVE` writer - Select Curve"]
pub type SELCURVE_W<'a> = crate::FieldWriter<'a, u32, COMMAND_SPEC, u8, SELCURVE_A, 3, 20>;
impl<'a> SELCURVE_W<'a> {
    #[doc = "No acceleration"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SELCURVE_A::NONE)
    }
    #[doc = "P256"]
    #[inline(always)]
    pub fn p256(self) -> &'a mut W {
        self.variant(SELCURVE_A::P256)
    }
    #[doc = "P192"]
    #[inline(always)]
    pub fn p192(self) -> &'a mut W {
        self.variant(SELCURVE_A::P192)
    }
}
#[doc = "Field `EDWARDS` reader - Edwards Curve Enable"]
pub type EDWARDS_R = crate::BitReader<bool>;
#[doc = "Field `EDWARDS` writer - Edwards Curve Enable"]
pub type EDWARDS_W<'a> = crate::BitWriter<'a, u32, COMMAND_SPEC, bool, 26>;
#[doc = "Buffer Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFSEL_A {
    #[doc = "0: use data in data memory 0"]
    MEM0 = 0,
}
impl From<BUFSEL_A> for bool {
    #[inline(always)]
    fn from(variant: BUFSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFSEL` reader - Buffer Select"]
pub type BUFSEL_R = crate::BitReader<BUFSEL_A>;
impl BUFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BUFSEL_A> {
        match self.bits {
            false => Some(BUFSEL_A::MEM0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MEM0`"]
    #[inline(always)]
    pub fn is_mem0(&self) -> bool {
        *self == BUFSEL_A::MEM0
    }
}
#[doc = "Field `BUFSEL` writer - Buffer Select"]
pub type BUFSEL_W<'a> = crate::BitWriter<'a, u32, COMMAND_SPEC, BUFSEL_A, 27>;
impl<'a> BUFSEL_W<'a> {
    #[doc = "use data in data memory 0"]
    #[inline(always)]
    pub fn mem0(self) -> &'a mut W {
        self.variant(BUFSEL_A::MEM0)
    }
}
#[doc = "Swap bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWAPBYTES_A {
    #[doc = "0: Native format (little endian)"]
    NATIVE = 0,
    #[doc = "1: Byte swapped (big endian)"]
    SWAPPED = 1,
}
impl From<SWAPBYTES_A> for bool {
    #[inline(always)]
    fn from(variant: SWAPBYTES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWAPBYTES` reader - Swap bytes"]
pub type SWAPBYTES_R = crate::BitReader<SWAPBYTES_A>;
impl SWAPBYTES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWAPBYTES_A {
        match self.bits {
            false => SWAPBYTES_A::NATIVE,
            true => SWAPBYTES_A::SWAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `NATIVE`"]
    #[inline(always)]
    pub fn is_native(&self) -> bool {
        *self == SWAPBYTES_A::NATIVE
    }
    #[doc = "Checks if the value of the field is `SWAPPED`"]
    #[inline(always)]
    pub fn is_swapped(&self) -> bool {
        *self == SWAPBYTES_A::SWAPPED
    }
}
#[doc = "Field `SWAPBYTES` writer - Swap bytes"]
pub type SWAPBYTES_W<'a> = crate::BitWriter<'a, u32, COMMAND_SPEC, SWAPBYTES_A, 28>;
impl<'a> SWAPBYTES_W<'a> {
    #[doc = "Native format (little endian)"]
    #[inline(always)]
    pub fn native(self) -> &'a mut W {
        self.variant(SWAPBYTES_A::NATIVE)
    }
    #[doc = "Byte swapped (big endian)"]
    #[inline(always)]
    pub fn swapped(self) -> &'a mut W {
        self.variant(SWAPBYTES_A::SWAPPED)
    }
}
#[doc = "Field `FLAGA` reader - Flag A"]
pub type FLAGA_R = crate::BitReader<bool>;
#[doc = "Field `FLAGA` writer - Flag A"]
pub type FLAGA_W<'a> = crate::BitWriter<'a, u32, COMMAND_SPEC, bool, 29>;
#[doc = "Field `FLAGB` reader - Flag B"]
pub type FLAGB_R = crate::BitReader<bool>;
#[doc = "Field `FLAGB` writer - Flag B"]
pub type FLAGB_W<'a> = crate::BitWriter<'a, u32, COMMAND_SPEC, bool, 30>;
#[doc = "Calculate R2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALCR2_A {
    #[doc = "0: don't recalculate R² mod N"]
    FALSE = 0,
    #[doc = "1: re-calculate R² mod N"]
    TRUE = 1,
}
impl From<CALCR2_A> for bool {
    #[inline(always)]
    fn from(variant: CALCR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALCR2` reader - Calculate R2"]
pub type CALCR2_R = crate::BitReader<CALCR2_A>;
impl CALCR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALCR2_A {
        match self.bits {
            false => CALCR2_A::FALSE,
            true => CALCR2_A::TRUE,
        }
    }
    #[doc = "Checks if the value of the field is `FALSE`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == CALCR2_A::FALSE
    }
    #[doc = "Checks if the value of the field is `TRUE`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == CALCR2_A::TRUE
    }
}
#[doc = "Field `CALCR2` writer - Calculate R2"]
pub type CALCR2_W<'a> = crate::BitWriter<'a, u32, COMMAND_SPEC, CALCR2_A, 31>;
impl<'a> CALCR2_W<'a> {
    #[doc = "don't recalculate R² mod N"]
    #[inline(always)]
    pub fn false_(self) -> &'a mut W {
        self.variant(CALCR2_A::FALSE)
    }
    #[doc = "re-calculate R² mod N"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut W {
        self.variant(CALCR2_A::TRUE)
    }
}
impl R {
    #[doc = "Bits 0:6 - Type of Operation"]
    #[inline(always)]
    pub fn operation(&self) -> OPERATION_R {
        OPERATION_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Field"]
    #[inline(always)]
    pub fn field(&self) -> FIELD_R {
        FIELD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:18 - Size of Operands in data memory"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 8) & 0x07ff) as u16)
    }
    #[doc = "Bits 20:22 - Select Curve"]
    #[inline(always)]
    pub fn selcurve(&self) -> SELCURVE_R {
        SELCURVE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 26 - Edwards Curve Enable"]
    #[inline(always)]
    pub fn edwards(&self) -> EDWARDS_R {
        EDWARDS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Buffer Select"]
    #[inline(always)]
    pub fn bufsel(&self) -> BUFSEL_R {
        BUFSEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Swap bytes"]
    #[inline(always)]
    pub fn swapbytes(&self) -> SWAPBYTES_R {
        SWAPBYTES_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Flag A"]
    #[inline(always)]
    pub fn flaga(&self) -> FLAGA_R {
        FLAGA_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Flag B"]
    #[inline(always)]
    pub fn flagb(&self) -> FLAGB_R {
        FLAGB_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Calculate R2"]
    #[inline(always)]
    pub fn calcr2(&self) -> CALCR2_R {
        CALCR2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Type of Operation"]
    #[inline(always)]
    pub fn operation(&mut self) -> OPERATION_W {
        OPERATION_W::new(self)
    }
    #[doc = "Bit 7 - Field"]
    #[inline(always)]
    pub fn field(&mut self) -> FIELD_W {
        FIELD_W::new(self)
    }
    #[doc = "Bits 8:18 - Size of Operands in data memory"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W::new(self)
    }
    #[doc = "Bits 20:22 - Select Curve"]
    #[inline(always)]
    pub fn selcurve(&mut self) -> SELCURVE_W {
        SELCURVE_W::new(self)
    }
    #[doc = "Bit 26 - Edwards Curve Enable"]
    #[inline(always)]
    pub fn edwards(&mut self) -> EDWARDS_W {
        EDWARDS_W::new(self)
    }
    #[doc = "Bit 27 - Buffer Select"]
    #[inline(always)]
    pub fn bufsel(&mut self) -> BUFSEL_W {
        BUFSEL_W::new(self)
    }
    #[doc = "Bit 28 - Swap bytes"]
    #[inline(always)]
    pub fn swapbytes(&mut self) -> SWAPBYTES_W {
        SWAPBYTES_W::new(self)
    }
    #[doc = "Bit 29 - Flag A"]
    #[inline(always)]
    pub fn flaga(&mut self) -> FLAGA_W {
        FLAGA_W::new(self)
    }
    #[doc = "Bit 30 - Flag B"]
    #[inline(always)]
    pub fn flagb(&mut self) -> FLAGB_W {
        FLAGB_W::new(self)
    }
    #[doc = "Bit 31 - Calculate R2"]
    #[inline(always)]
    pub fn calcr2(&mut self) -> CALCR2_W {
        CALCR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [command](index.html) module"]
pub struct COMMAND_SPEC;
impl crate::RegisterSpec for COMMAND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [command::R](R) reader structure"]
impl crate::Readable for COMMAND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [command::W](W) writer structure"]
impl crate::Writable for COMMAND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMMAND to value 0"]
impl crate::Resettable for COMMAND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
