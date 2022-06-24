#[doc = "Register `DCDCMISCCTRL` reader"]
pub struct R(crate::R<DCDCMISCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDCMISCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDCMISCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDCMISCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDCMISCCTRL` writer"]
pub struct W(crate::W<DCDCMISCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDCMISCCTRL_SPEC>;
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
impl From<crate::W<DCDCMISCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDCMISCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LNFORCECCM` reader - Force DCDC Into CCM Mode in Low Noise Operation"]
pub type LNFORCECCM_R = crate::BitReader<bool>;
#[doc = "Field `LNFORCECCM` writer - Force DCDC Into CCM Mode in Low Noise Operation"]
pub type LNFORCECCM_W<'a> = crate::BitWriter<'a, u32, DCDCMISCCTRL_SPEC, bool, 0>;
#[doc = "Field `PFETCNT` reader - PFET Switch Number Selection"]
pub type PFETCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PFETCNT` writer - PFET Switch Number Selection"]
pub type PFETCNT_W<'a> = crate::FieldWriter<'a, u32, DCDCMISCCTRL_SPEC, u8, u8, 4, 8>;
#[doc = "Field `NFETCNT` reader - NFET Switch Number Selection"]
pub type NFETCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NFETCNT` writer - NFET Switch Number Selection"]
pub type NFETCNT_W<'a> = crate::FieldWriter<'a, u32, DCDCMISCCTRL_SPEC, u8, u8, 4, 12>;
#[doc = "Field `BYPLIMSEL` reader - Current Limit in Bypass Mode"]
pub type BYPLIMSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BYPLIMSEL` writer - Current Limit in Bypass Mode"]
pub type BYPLIMSEL_W<'a> = crate::FieldWriter<'a, u32, DCDCMISCCTRL_SPEC, u8, u8, 4, 16>;
#[doc = "Field `LPCLIMILIMSEL` reader - Current Limit Level Selection for Current Limiter in LP Mode"]
pub type LPCLIMILIMSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPCLIMILIMSEL` writer - Current Limit Level Selection for Current Limiter in LP Mode"]
pub type LPCLIMILIMSEL_W<'a> = crate::FieldWriter<'a, u32, DCDCMISCCTRL_SPEC, u8, u8, 3, 20>;
#[doc = "Field `LNCLIMILIMSEL` reader - Current Limit Level Selection for Current Limiter in LN Mode"]
pub type LNCLIMILIMSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LNCLIMILIMSEL` writer - Current Limit Level Selection for Current Limiter in LN Mode"]
pub type LNCLIMILIMSEL_W<'a> = crate::FieldWriter<'a, u32, DCDCMISCCTRL_SPEC, u8, u8, 3, 24>;
#[doc = "LP Mode Comparator Bias Selection\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPCMPBIAS_A {
    #[doc = "0: Maximum load current less than 75uA."]
    BIAS0 = 0,
    #[doc = "1: Maximum load current less than 500uA."]
    BIAS1 = 1,
    #[doc = "2: Maximum load current less than 2.5mA."]
    BIAS2 = 2,
    #[doc = "3: Maximum load current less than 10mA."]
    BIAS3 = 3,
}
impl From<LPCMPBIAS_A> for u8 {
    #[inline(always)]
    fn from(variant: LPCMPBIAS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LPCMPBIAS` reader - LP Mode Comparator Bias Selection"]
pub type LPCMPBIAS_R = crate::FieldReader<u8, LPCMPBIAS_A>;
impl LPCMPBIAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPCMPBIAS_A {
        match self.bits {
            0 => LPCMPBIAS_A::BIAS0,
            1 => LPCMPBIAS_A::BIAS1,
            2 => LPCMPBIAS_A::BIAS2,
            3 => LPCMPBIAS_A::BIAS3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS0`"]
    #[inline(always)]
    pub fn is_bias0(&self) -> bool {
        *self == LPCMPBIAS_A::BIAS0
    }
    #[doc = "Checks if the value of the field is `BIAS1`"]
    #[inline(always)]
    pub fn is_bias1(&self) -> bool {
        *self == LPCMPBIAS_A::BIAS1
    }
    #[doc = "Checks if the value of the field is `BIAS2`"]
    #[inline(always)]
    pub fn is_bias2(&self) -> bool {
        *self == LPCMPBIAS_A::BIAS2
    }
    #[doc = "Checks if the value of the field is `BIAS3`"]
    #[inline(always)]
    pub fn is_bias3(&self) -> bool {
        *self == LPCMPBIAS_A::BIAS3
    }
}
#[doc = "Field `LPCMPBIAS` writer - LP Mode Comparator Bias Selection"]
pub type LPCMPBIAS_W<'a> =
    crate::FieldWriterSafe<'a, u32, DCDCMISCCTRL_SPEC, u8, LPCMPBIAS_A, 2, 28>;
impl<'a> LPCMPBIAS_W<'a> {
    #[doc = "Maximum load current less than 75uA."]
    #[inline(always)]
    pub fn bias0(self) -> &'a mut W {
        self.variant(LPCMPBIAS_A::BIAS0)
    }
    #[doc = "Maximum load current less than 500uA."]
    #[inline(always)]
    pub fn bias1(self) -> &'a mut W {
        self.variant(LPCMPBIAS_A::BIAS1)
    }
    #[doc = "Maximum load current less than 2.5mA."]
    #[inline(always)]
    pub fn bias2(self) -> &'a mut W {
        self.variant(LPCMPBIAS_A::BIAS2)
    }
    #[doc = "Maximum load current less than 10mA."]
    #[inline(always)]
    pub fn bias3(self) -> &'a mut W {
        self.variant(LPCMPBIAS_A::BIAS3)
    }
}
impl R {
    #[doc = "Bit 0 - Force DCDC Into CCM Mode in Low Noise Operation"]
    #[inline(always)]
    pub fn lnforceccm(&self) -> LNFORCECCM_R {
        LNFORCECCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - PFET Switch Number Selection"]
    #[inline(always)]
    pub fn pfetcnt(&self) -> PFETCNT_R {
        PFETCNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - NFET Switch Number Selection"]
    #[inline(always)]
    pub fn nfetcnt(&self) -> NFETCNT_R {
        NFETCNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Current Limit in Bypass Mode"]
    #[inline(always)]
    pub fn byplimsel(&self) -> BYPLIMSEL_R {
        BYPLIMSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Current Limit Level Selection for Current Limiter in LP Mode"]
    #[inline(always)]
    pub fn lpclimilimsel(&self) -> LPCLIMILIMSEL_R {
        LPCLIMILIMSEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Current Limit Level Selection for Current Limiter in LN Mode"]
    #[inline(always)]
    pub fn lnclimilimsel(&self) -> LNCLIMILIMSEL_R {
        LNCLIMILIMSEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - LP Mode Comparator Bias Selection"]
    #[inline(always)]
    pub fn lpcmpbias(&self) -> LPCMPBIAS_R {
        LPCMPBIAS_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Force DCDC Into CCM Mode in Low Noise Operation"]
    #[inline(always)]
    pub fn lnforceccm(&mut self) -> LNFORCECCM_W {
        LNFORCECCM_W::new(self)
    }
    #[doc = "Bits 8:11 - PFET Switch Number Selection"]
    #[inline(always)]
    pub fn pfetcnt(&mut self) -> PFETCNT_W {
        PFETCNT_W::new(self)
    }
    #[doc = "Bits 12:15 - NFET Switch Number Selection"]
    #[inline(always)]
    pub fn nfetcnt(&mut self) -> NFETCNT_W {
        NFETCNT_W::new(self)
    }
    #[doc = "Bits 16:19 - Current Limit in Bypass Mode"]
    #[inline(always)]
    pub fn byplimsel(&mut self) -> BYPLIMSEL_W {
        BYPLIMSEL_W::new(self)
    }
    #[doc = "Bits 20:22 - Current Limit Level Selection for Current Limiter in LP Mode"]
    #[inline(always)]
    pub fn lpclimilimsel(&mut self) -> LPCLIMILIMSEL_W {
        LPCLIMILIMSEL_W::new(self)
    }
    #[doc = "Bits 24:26 - Current Limit Level Selection for Current Limiter in LN Mode"]
    #[inline(always)]
    pub fn lnclimilimsel(&mut self) -> LNCLIMILIMSEL_W {
        LNCLIMILIMSEL_W::new(self)
    }
    #[doc = "Bits 28:29 - LP Mode Comparator Bias Selection"]
    #[inline(always)]
    pub fn lpcmpbias(&mut self) -> LPCMPBIAS_W {
        LPCMPBIAS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC Miscellaneous Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdcmiscctrl](index.html) module"]
pub struct DCDCMISCCTRL_SPEC;
impl crate::RegisterSpec for DCDCMISCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdcmiscctrl::R](R) reader structure"]
impl crate::Readable for DCDCMISCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdcmiscctrl::W](W) writer structure"]
impl crate::Writable for DCDCMISCCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCDCMISCCTRL to value 0x3330_7700"]
impl crate::Resettable for DCDCMISCCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3330_7700
    }
}
