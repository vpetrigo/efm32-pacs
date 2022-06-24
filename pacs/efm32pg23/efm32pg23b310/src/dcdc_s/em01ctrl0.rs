#[doc = "Register `EM01CTRL0` reader"]
pub struct R(crate::R<EM01CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EM01CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EM01CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EM01CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EM01CTRL0` writer"]
pub struct W(crate::W<EM01CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EM01CTRL0_SPEC>;
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
impl From<crate::W<EM01CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EM01CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "EM01 Peak Current Setting\n\nValue on reset: 9"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IPKVAL_A {
    #[doc = "3: Ipeak = 90mA, Iload = 36mA"]
    LOAD36MA = 3,
    #[doc = "4: Ipeak = 100mA, Iload = 40mA"]
    LOAD40MA = 4,
    #[doc = "5: Ipeak = 110mA, Iload = 44mA"]
    LOAD44MA = 5,
    #[doc = "6: Ipeak = 120mA, Iload = 48mA"]
    LOAD48MA = 6,
    #[doc = "7: Ipeak = 130mA, Iload = 52mA"]
    LOAD52MA = 7,
    #[doc = "8: Ipeak = 140mA, Iload = 56mA"]
    LOAD56MA = 8,
    #[doc = "9: Ipeak = 150mA, Iload = 60mA"]
    LOAD60MA = 9,
}
impl From<IPKVAL_A> for u8 {
    #[inline(always)]
    fn from(variant: IPKVAL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IPKVAL` reader - EM01 Peak Current Setting"]
pub type IPKVAL_R = crate::FieldReader<u8, IPKVAL_A>;
impl IPKVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IPKVAL_A> {
        match self.bits {
            3 => Some(IPKVAL_A::LOAD36MA),
            4 => Some(IPKVAL_A::LOAD40MA),
            5 => Some(IPKVAL_A::LOAD44MA),
            6 => Some(IPKVAL_A::LOAD48MA),
            7 => Some(IPKVAL_A::LOAD52MA),
            8 => Some(IPKVAL_A::LOAD56MA),
            9 => Some(IPKVAL_A::LOAD60MA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOAD36MA`"]
    #[inline(always)]
    pub fn is_load36m_a(&self) -> bool {
        *self == IPKVAL_A::LOAD36MA
    }
    #[doc = "Checks if the value of the field is `LOAD40MA`"]
    #[inline(always)]
    pub fn is_load40m_a(&self) -> bool {
        *self == IPKVAL_A::LOAD40MA
    }
    #[doc = "Checks if the value of the field is `LOAD44MA`"]
    #[inline(always)]
    pub fn is_load44m_a(&self) -> bool {
        *self == IPKVAL_A::LOAD44MA
    }
    #[doc = "Checks if the value of the field is `LOAD48MA`"]
    #[inline(always)]
    pub fn is_load48m_a(&self) -> bool {
        *self == IPKVAL_A::LOAD48MA
    }
    #[doc = "Checks if the value of the field is `LOAD52MA`"]
    #[inline(always)]
    pub fn is_load52m_a(&self) -> bool {
        *self == IPKVAL_A::LOAD52MA
    }
    #[doc = "Checks if the value of the field is `LOAD56MA`"]
    #[inline(always)]
    pub fn is_load56m_a(&self) -> bool {
        *self == IPKVAL_A::LOAD56MA
    }
    #[doc = "Checks if the value of the field is `LOAD60MA`"]
    #[inline(always)]
    pub fn is_load60m_a(&self) -> bool {
        *self == IPKVAL_A::LOAD60MA
    }
}
#[doc = "Field `IPKVAL` writer - EM01 Peak Current Setting"]
pub type IPKVAL_W<'a> = crate::FieldWriter<'a, u32, EM01CTRL0_SPEC, u8, IPKVAL_A, 4, 0>;
impl<'a> IPKVAL_W<'a> {
    #[doc = "Ipeak = 90mA, Iload = 36mA"]
    #[inline(always)]
    pub fn load36m_a(self) -> &'a mut W {
        self.variant(IPKVAL_A::LOAD36MA)
    }
    #[doc = "Ipeak = 100mA, Iload = 40mA"]
    #[inline(always)]
    pub fn load40m_a(self) -> &'a mut W {
        self.variant(IPKVAL_A::LOAD40MA)
    }
    #[doc = "Ipeak = 110mA, Iload = 44mA"]
    #[inline(always)]
    pub fn load44m_a(self) -> &'a mut W {
        self.variant(IPKVAL_A::LOAD44MA)
    }
    #[doc = "Ipeak = 120mA, Iload = 48mA"]
    #[inline(always)]
    pub fn load48m_a(self) -> &'a mut W {
        self.variant(IPKVAL_A::LOAD48MA)
    }
    #[doc = "Ipeak = 130mA, Iload = 52mA"]
    #[inline(always)]
    pub fn load52m_a(self) -> &'a mut W {
        self.variant(IPKVAL_A::LOAD52MA)
    }
    #[doc = "Ipeak = 140mA, Iload = 56mA"]
    #[inline(always)]
    pub fn load56m_a(self) -> &'a mut W {
        self.variant(IPKVAL_A::LOAD56MA)
    }
    #[doc = "Ipeak = 150mA, Iload = 60mA"]
    #[inline(always)]
    pub fn load60m_a(self) -> &'a mut W {
        self.variant(IPKVAL_A::LOAD60MA)
    }
}
#[doc = "EM01 Drive Speed Setting\n\nValue on reset: 1"]
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
#[doc = "Field `DRVSPEED` reader - EM01 Drive Speed Setting"]
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
#[doc = "Field `DRVSPEED` writer - EM01 Drive Speed Setting"]
pub type DRVSPEED_W<'a> = crate::FieldWriterSafe<'a, u32, EM01CTRL0_SPEC, u8, DRVSPEED_A, 2, 8>;
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
    #[doc = "Bits 0:3 - EM01 Peak Current Setting"]
    #[inline(always)]
    pub fn ipkval(&self) -> IPKVAL_R {
        IPKVAL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - EM01 Drive Speed Setting"]
    #[inline(always)]
    pub fn drvspeed(&self) -> DRVSPEED_R {
        DRVSPEED_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EM01 Peak Current Setting"]
    #[inline(always)]
    pub fn ipkval(&mut self) -> IPKVAL_W {
        IPKVAL_W::new(self)
    }
    #[doc = "Bits 8:9 - EM01 Drive Speed Setting"]
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
#[doc = "EM01 Configurations\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [em01ctrl0](index.html) module"]
pub struct EM01CTRL0_SPEC;
impl crate::RegisterSpec for EM01CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [em01ctrl0::R](R) reader structure"]
impl crate::Readable for EM01CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [em01ctrl0::W](W) writer structure"]
impl crate::Writable for EM01CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EM01CTRL0 to value 0x0109"]
impl crate::Resettable for EM01CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0109
    }
}
