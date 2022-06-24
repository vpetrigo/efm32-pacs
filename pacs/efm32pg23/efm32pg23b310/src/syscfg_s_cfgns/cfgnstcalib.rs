#[doc = "Register `CFGNSTCALIB` reader"]
pub struct R(crate::R<CFGNSTCALIB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGNSTCALIB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGNSTCALIB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGNSTCALIB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGNSTCALIB` writer"]
pub struct W(crate::W<CFGNSTCALIB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGNSTCALIB_SPEC>;
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
impl From<crate::W<CFGNSTCALIB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGNSTCALIB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TENMS` reader - Ten Milliseconds"]
pub type TENMS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TENMS` writer - Ten Milliseconds"]
pub type TENMS_W<'a> = crate::FieldWriter<'a, u32, CFGNSTCALIB_SPEC, u32, u32, 24, 0>;
#[doc = "Field `SKEW` reader - Skew"]
pub type SKEW_R = crate::BitReader<bool>;
#[doc = "Field `SKEW` writer - Skew"]
pub type SKEW_W<'a> = crate::BitWriter<'a, u32, CFGNSTCALIB_SPEC, bool, 24>;
#[doc = "No Reference\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOREF_A {
    #[doc = "0: Reference clock is implemented"]
    REF = 0,
    #[doc = "1: Reference clock is not implemented"]
    NOREF = 1,
}
impl From<NOREF_A> for bool {
    #[inline(always)]
    fn from(variant: NOREF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOREF` reader - No Reference"]
pub type NOREF_R = crate::BitReader<NOREF_A>;
impl NOREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOREF_A {
        match self.bits {
            false => NOREF_A::REF,
            true => NOREF_A::NOREF,
        }
    }
    #[doc = "Checks if the value of the field is `REF`"]
    #[inline(always)]
    pub fn is_ref(&self) -> bool {
        *self == NOREF_A::REF
    }
    #[doc = "Checks if the value of the field is `NOREF`"]
    #[inline(always)]
    pub fn is_noref(&self) -> bool {
        *self == NOREF_A::NOREF
    }
}
#[doc = "Field `NOREF` writer - No Reference"]
pub type NOREF_W<'a> = crate::BitWriter<'a, u32, CFGNSTCALIB_SPEC, NOREF_A, 25>;
impl<'a> NOREF_W<'a> {
    #[doc = "Reference clock is implemented"]
    #[inline(always)]
    pub fn ref_(self) -> &'a mut W {
        self.variant(NOREF_A::REF)
    }
    #[doc = "Reference clock is not implemented"]
    #[inline(always)]
    pub fn noref(self) -> &'a mut W {
        self.variant(NOREF_A::NOREF)
    }
}
impl R {
    #[doc = "Bits 0:23 - Ten Milliseconds"]
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 24 - Skew"]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - No Reference"]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Ten Milliseconds"]
    #[inline(always)]
    pub fn tenms(&mut self) -> TENMS_W {
        TENMS_W::new(self)
    }
    #[doc = "Bit 24 - Skew"]
    #[inline(always)]
    pub fn skew(&mut self) -> SKEW_W {
        SKEW_W::new(self)
    }
    #[doc = "Bit 25 - No Reference"]
    #[inline(always)]
    pub fn noref(&mut self) -> NOREF_W {
        NOREF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure to define the system tick for the M33.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgnstcalib](index.html) module"]
pub struct CFGNSTCALIB_SPEC;
impl crate::RegisterSpec for CFGNSTCALIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgnstcalib::R](R) reader structure"]
impl crate::Readable for CFGNSTCALIB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgnstcalib::W](W) writer structure"]
impl crate::Writable for CFGNSTCALIB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGNSTCALIB to value 0x0100_4a37"]
impl crate::Resettable for CFGNSTCALIB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_4a37
    }
}
