#[doc = "Register `TIMINGCFG` reader"]
pub struct R(crate::R<TIMINGCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMINGCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMINGCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMINGCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMINGCFG` writer"]
pub struct W(crate::W<TIMINGCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMINGCFG_SPEC>;
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
impl From<crate::W<TIMINGCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMINGCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TX Delay Transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXDELAY_A {
    #[doc = "0: Frames are transmitted immediately."]
    NONE = 0,
    #[doc = "1: Transmission of new frames is delayed by a single bit period."]
    SINGLE = 1,
    #[doc = "2: Transmission of new frames is delayed by a two bit periods."]
    DOUBLE = 2,
    #[doc = "3: Transmission of new frames is delayed by a three bit periods."]
    TRIPPLE = 3,
}
impl From<TXDELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: TXDELAY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TXDELAY` reader - TX Delay Transmission"]
pub type TXDELAY_R = crate::FieldReader<u8, TXDELAY_A>;
impl TXDELAY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDELAY_A {
        match self.bits {
            0 => TXDELAY_A::NONE,
            1 => TXDELAY_A::SINGLE,
            2 => TXDELAY_A::DOUBLE,
            3 => TXDELAY_A::TRIPPLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TXDELAY_A::NONE
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == TXDELAY_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DOUBLE`"]
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        *self == TXDELAY_A::DOUBLE
    }
    #[doc = "Checks if the value of the field is `TRIPPLE`"]
    #[inline(always)]
    pub fn is_tripple(&self) -> bool {
        *self == TXDELAY_A::TRIPPLE
    }
}
#[doc = "Field `TXDELAY` writer - TX Delay Transmission"]
pub type TXDELAY_W<'a> = crate::FieldWriterSafe<'a, u32, TIMINGCFG_SPEC, u8, TXDELAY_A, 2, 0>;
impl<'a> TXDELAY_W<'a> {
    #[doc = "Frames are transmitted immediately."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(TXDELAY_A::NONE)
    }
    #[doc = "Transmission of new frames is delayed by a single bit period."]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(TXDELAY_A::SINGLE)
    }
    #[doc = "Transmission of new frames is delayed by a two bit periods."]
    #[inline(always)]
    pub fn double(self) -> &'a mut W {
        self.variant(TXDELAY_A::DOUBLE)
    }
    #[doc = "Transmission of new frames is delayed by a three bit periods."]
    #[inline(always)]
    pub fn tripple(self) -> &'a mut W {
        self.variant(TXDELAY_A::TRIPPLE)
    }
}
impl R {
    #[doc = "Bits 0:1 - TX Delay Transmission"]
    #[inline(always)]
    pub fn txdelay(&self) -> TXDELAY_R {
        TXDELAY_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - TX Delay Transmission"]
    #[inline(always)]
    pub fn txdelay(&mut self) -> TXDELAY_W {
        TXDELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timingcfg](index.html) module"]
pub struct TIMINGCFG_SPEC;
impl crate::RegisterSpec for TIMINGCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timingcfg::R](R) reader structure"]
impl crate::Readable for TIMINGCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timingcfg::W](W) writer structure"]
impl crate::Writable for TIMINGCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMINGCFG to value 0"]
impl crate::Resettable for TIMINGCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
