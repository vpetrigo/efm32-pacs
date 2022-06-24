#[doc = "Register `CURPROG` reader"]
pub struct R(crate::R<CURPROG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CURPROG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CURPROG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CURPROG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CURPROG` writer"]
pub struct W(crate::W<CURPROG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CURPROG_SPEC>;
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
impl From<crate::W<CURPROG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CURPROG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Current Range Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RANGESEL_A {
    #[doc = "0: Current range set to 0 - 1.6 uA."]
    RANGE0 = 0,
    #[doc = "1: Current range set to 1.6 - 4.7 uA."]
    RANGE1 = 1,
    #[doc = "2: Current range set to 0.5 - 16 uA."]
    RANGE2 = 2,
    #[doc = "3: Current range set to 2 - 64 uA."]
    RANGE3 = 3,
}
impl From<RANGESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RANGESEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RANGESEL` reader - Current Range Select"]
pub type RANGESEL_R = crate::FieldReader<u8, RANGESEL_A>;
impl RANGESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RANGESEL_A {
        match self.bits {
            0 => RANGESEL_A::RANGE0,
            1 => RANGESEL_A::RANGE1,
            2 => RANGESEL_A::RANGE2,
            3 => RANGESEL_A::RANGE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RANGE0`"]
    #[inline(always)]
    pub fn is_range0(&self) -> bool {
        *self == RANGESEL_A::RANGE0
    }
    #[doc = "Checks if the value of the field is `RANGE1`"]
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        *self == RANGESEL_A::RANGE1
    }
    #[doc = "Checks if the value of the field is `RANGE2`"]
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        *self == RANGESEL_A::RANGE2
    }
    #[doc = "Checks if the value of the field is `RANGE3`"]
    #[inline(always)]
    pub fn is_range3(&self) -> bool {
        *self == RANGESEL_A::RANGE3
    }
}
#[doc = "Field `RANGESEL` writer - Current Range Select"]
pub type RANGESEL_W<'a> = crate::FieldWriterSafe<'a, u32, CURPROG_SPEC, u8, RANGESEL_A, 2, 0>;
impl<'a> RANGESEL_W<'a> {
    #[doc = "Current range set to 0 - 1.6 uA."]
    #[inline(always)]
    pub fn range0(self) -> &'a mut W {
        self.variant(RANGESEL_A::RANGE0)
    }
    #[doc = "Current range set to 1.6 - 4.7 uA."]
    #[inline(always)]
    pub fn range1(self) -> &'a mut W {
        self.variant(RANGESEL_A::RANGE1)
    }
    #[doc = "Current range set to 0.5 - 16 uA."]
    #[inline(always)]
    pub fn range2(self) -> &'a mut W {
        self.variant(RANGESEL_A::RANGE2)
    }
    #[doc = "Current range set to 2 - 64 uA."]
    #[inline(always)]
    pub fn range3(self) -> &'a mut W {
        self.variant(RANGESEL_A::RANGE3)
    }
}
#[doc = "Field `STEPSEL` reader - Current Step Size Select"]
pub type STEPSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STEPSEL` writer - Current Step Size Select"]
pub type STEPSEL_W<'a> = crate::FieldWriter<'a, u32, CURPROG_SPEC, u8, u8, 5, 8>;
#[doc = "Field `TUNING` reader - Tune the Current to Given Accuracy"]
pub type TUNING_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TUNING` writer - Tune the Current to Given Accuracy"]
pub type TUNING_W<'a> = crate::FieldWriter<'a, u32, CURPROG_SPEC, u8, u8, 8, 16>;
impl R {
    #[doc = "Bits 0:1 - Current Range Select"]
    #[inline(always)]
    pub fn rangesel(&self) -> RANGESEL_R {
        RANGESEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:12 - Current Step Size Select"]
    #[inline(always)]
    pub fn stepsel(&self) -> STEPSEL_R {
        STEPSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - Tune the Current to Given Accuracy"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Current Range Select"]
    #[inline(always)]
    pub fn rangesel(&mut self) -> RANGESEL_W {
        RANGESEL_W::new(self)
    }
    #[doc = "Bits 8:12 - Current Step Size Select"]
    #[inline(always)]
    pub fn stepsel(&mut self) -> STEPSEL_W {
        STEPSEL_W::new(self)
    }
    #[doc = "Bits 16:23 - Tune the Current to Given Accuracy"]
    #[inline(always)]
    pub fn tuning(&mut self) -> TUNING_W {
        TUNING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Current Programming Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [curprog](index.html) module"]
pub struct CURPROG_SPEC;
impl crate::RegisterSpec for CURPROG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [curprog::R](R) reader structure"]
impl crate::Readable for CURPROG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [curprog::W](W) writer structure"]
impl crate::Writable for CURPROG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CURPROG to value 0x009b_0000"]
impl crate::Resettable for CURPROG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x009b_0000
    }
}
