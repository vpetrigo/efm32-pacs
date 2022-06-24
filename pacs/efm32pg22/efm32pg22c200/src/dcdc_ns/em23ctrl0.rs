#[doc = "Register `EM23CTRL0` reader"]
pub struct R(crate::R<EM23CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EM23CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EM23CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EM23CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EM23CTRL0` writer"]
pub struct W(crate::W<EM23CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EM23CTRL0_SPEC>;
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
impl From<crate::W<EM23CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EM23CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "EM23 Peak Current Setting\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IPKVAL_A {
    #[doc = "3: Ipeak = 90mA, Iload = 5 mA"]
    LOAD5MA = 3,
    #[doc = "9: Ipeak = 150mA, Iload = 10 mA"]
    LOAD10MA = 9,
}
impl From<IPKVAL_A> for u8 {
    #[inline(always)]
    fn from(variant: IPKVAL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IPKVAL` reader - EM23 Peak Current Setting"]
pub type IPKVAL_R = crate::FieldReader<u8, IPKVAL_A>;
impl IPKVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IPKVAL_A> {
        match self.bits {
            3 => Some(IPKVAL_A::LOAD5MA),
            9 => Some(IPKVAL_A::LOAD10MA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOAD5MA`"]
    #[inline(always)]
    pub fn is_load5ma(&self) -> bool {
        *self == IPKVAL_A::LOAD5MA
    }
    #[doc = "Checks if the value of the field is `LOAD10MA`"]
    #[inline(always)]
    pub fn is_load10ma(&self) -> bool {
        *self == IPKVAL_A::LOAD10MA
    }
}
#[doc = "Field `IPKVAL` writer - EM23 Peak Current Setting"]
pub type IPKVAL_W<'a> = crate::FieldWriter<'a, u32, EM23CTRL0_SPEC, u8, IPKVAL_A, 4, 0>;
impl<'a> IPKVAL_W<'a> {
    #[doc = "Ipeak = 90mA, Iload = 5 mA"]
    #[inline(always)]
    pub fn load5ma(self) -> &'a mut W {
        self.variant(IPKVAL_A::LOAD5MA)
    }
    #[doc = "Ipeak = 150mA, Iload = 10 mA"]
    #[inline(always)]
    pub fn load10ma(self) -> &'a mut W {
        self.variant(IPKVAL_A::LOAD10MA)
    }
}
#[doc = "EM23 Drive Speed Setting\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DRVSPEED_A {
    #[doc = "0: Lowest Efficiency, Lowest EMI.. Small decrease in efficiency from default setting"]
    BEST_EMI = 0,
    #[doc = "1: Default Efficiency, Acceptable EMI level"]
    DEFAULT_SETTING = 1,
    #[doc = "2: Small increase in efficiency from the default setting"]
    INTERMEDIATE = 2,
    #[doc = "3: Highest Efficiency, Highest EMI.. Small increase in efficiency from INTERMEDIATE setting"]
    BEST_EFFICIENCY = 3,
}
impl From<DRVSPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: DRVSPEED_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DRVSPEED` reader - EM23 Drive Speed Setting"]
pub type DRVSPEED_R = crate::FieldReader<u8, DRVSPEED_A>;
impl DRVSPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRVSPEED_A {
        match self.bits {
            0 => DRVSPEED_A::BEST_EMI,
            1 => DRVSPEED_A::DEFAULT_SETTING,
            2 => DRVSPEED_A::INTERMEDIATE,
            3 => DRVSPEED_A::BEST_EFFICIENCY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BEST_EMI`"]
    #[inline(always)]
    pub fn is_best_emi(&self) -> bool {
        *self == DRVSPEED_A::BEST_EMI
    }
    #[doc = "Checks if the value of the field is `DEFAULT_SETTING`"]
    #[inline(always)]
    pub fn is_default_setting(&self) -> bool {
        *self == DRVSPEED_A::DEFAULT_SETTING
    }
    #[doc = "Checks if the value of the field is `INTERMEDIATE`"]
    #[inline(always)]
    pub fn is_intermediate(&self) -> bool {
        *self == DRVSPEED_A::INTERMEDIATE
    }
    #[doc = "Checks if the value of the field is `BEST_EFFICIENCY`"]
    #[inline(always)]
    pub fn is_best_efficiency(&self) -> bool {
        *self == DRVSPEED_A::BEST_EFFICIENCY
    }
}
#[doc = "Field `DRVSPEED` writer - EM23 Drive Speed Setting"]
pub type DRVSPEED_W<'a> = crate::FieldWriterSafe<'a, u32, EM23CTRL0_SPEC, u8, DRVSPEED_A, 2, 8>;
impl<'a> DRVSPEED_W<'a> {
    #[doc = "Lowest Efficiency, Lowest EMI.. Small decrease in efficiency from default setting"]
    #[inline(always)]
    pub fn best_emi(self) -> &'a mut W {
        self.variant(DRVSPEED_A::BEST_EMI)
    }
    #[doc = "Default Efficiency, Acceptable EMI level"]
    #[inline(always)]
    pub fn default_setting(self) -> &'a mut W {
        self.variant(DRVSPEED_A::DEFAULT_SETTING)
    }
    #[doc = "Small increase in efficiency from the default setting"]
    #[inline(always)]
    pub fn intermediate(self) -> &'a mut W {
        self.variant(DRVSPEED_A::INTERMEDIATE)
    }
    #[doc = "Highest Efficiency, Highest EMI.. Small increase in efficiency from INTERMEDIATE setting"]
    #[inline(always)]
    pub fn best_efficiency(self) -> &'a mut W {
        self.variant(DRVSPEED_A::BEST_EFFICIENCY)
    }
}
impl R {
    #[doc = "Bits 0:3 - EM23 Peak Current Setting"]
    #[inline(always)]
    pub fn ipkval(&self) -> IPKVAL_R {
        IPKVAL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - EM23 Drive Speed Setting"]
    #[inline(always)]
    pub fn drvspeed(&self) -> DRVSPEED_R {
        DRVSPEED_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EM23 Peak Current Setting"]
    #[inline(always)]
    pub fn ipkval(&mut self) -> IPKVAL_W {
        IPKVAL_W::new(self)
    }
    #[doc = "Bits 8:9 - EM23 Drive Speed Setting"]
    #[inline(always)]
    pub fn drvspeed(&mut self) -> DRVSPEED_W {
        DRVSPEED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EM23 Configurations\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [em23ctrl0](index.html) module"]
pub struct EM23CTRL0_SPEC;
impl crate::RegisterSpec for EM23CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [em23ctrl0::R](R) reader structure"]
impl crate::Readable for EM23CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [em23ctrl0::W](W) writer structure"]
impl crate::Writable for EM23CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EM23CTRL0 to value 0x0103"]
impl crate::Resettable for EM23CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0103
    }
}
