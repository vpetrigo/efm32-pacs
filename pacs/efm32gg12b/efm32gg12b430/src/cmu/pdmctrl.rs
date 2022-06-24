#[doc = "Register `PDMCTRL` reader"]
pub struct R(crate::R<PDMCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMCTRL` writer"]
pub struct W(crate::W<PDMCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMCTRL_SPEC>;
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
impl From<crate::W<PDMCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDM Core Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PDMCLKSEL_A {
    #[doc = "0: HFRCO clock is used to clock PDM"]
    HFRCO = 0,
    #[doc = "1: HFXO clock is used to clock PDM"]
    HFXO = 1,
    #[doc = "2: USHFRCO is used to clock PDM"]
    USHFRCO = 2,
    #[doc = "3: CLKIN0 is selected as HFCLK clock source"]
    CLKIN0 = 3,
}
impl From<PDMCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PDMCLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PDMCLKSEL` reader - PDM Core Clock Select"]
pub type PDMCLKSEL_R = crate::FieldReader<u8, PDMCLKSEL_A>;
impl PDMCLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDMCLKSEL_A {
        match self.bits {
            0 => PDMCLKSEL_A::HFRCO,
            1 => PDMCLKSEL_A::HFXO,
            2 => PDMCLKSEL_A::USHFRCO,
            3 => PDMCLKSEL_A::CLKIN0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == PDMCLKSEL_A::HFRCO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == PDMCLKSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `USHFRCO`"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == PDMCLKSEL_A::USHFRCO
    }
    #[doc = "Checks if the value of the field is `CLKIN0`"]
    #[inline(always)]
    pub fn is_clkin0(&self) -> bool {
        *self == PDMCLKSEL_A::CLKIN0
    }
}
#[doc = "Field `PDMCLKSEL` writer - PDM Core Clock Select"]
pub type PDMCLKSEL_W<'a> = crate::FieldWriterSafe<'a, u32, PDMCTRL_SPEC, u8, PDMCLKSEL_A, 2, 0>;
impl<'a> PDMCLKSEL_W<'a> {
    #[doc = "HFRCO clock is used to clock PDM"]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(PDMCLKSEL_A::HFRCO)
    }
    #[doc = "HFXO clock is used to clock PDM"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(PDMCLKSEL_A::HFXO)
    }
    #[doc = "USHFRCO is used to clock PDM"]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut W {
        self.variant(PDMCLKSEL_A::USHFRCO)
    }
    #[doc = "CLKIN0 is selected as HFCLK clock source"]
    #[inline(always)]
    pub fn clkin0(self) -> &'a mut W {
        self.variant(PDMCLKSEL_A::CLKIN0)
    }
}
#[doc = "Field `PDMCLKEN` reader - PDM Core Clock Enable"]
pub type PDMCLKEN_R = crate::BitReader<bool>;
#[doc = "Field `PDMCLKEN` writer - PDM Core Clock Enable"]
pub type PDMCLKEN_W<'a> = crate::BitWriter<'a, u32, PDMCTRL_SPEC, bool, 7>;
impl R {
    #[doc = "Bits 0:1 - PDM Core Clock Select"]
    #[inline(always)]
    pub fn pdmclksel(&self) -> PDMCLKSEL_R {
        PDMCLKSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 7 - PDM Core Clock Enable"]
    #[inline(always)]
    pub fn pdmclken(&self) -> PDMCLKEN_R {
        PDMCLKEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PDM Core Clock Select"]
    #[inline(always)]
    pub fn pdmclksel(&mut self) -> PDMCLKSEL_W {
        PDMCLKSEL_W::new(self)
    }
    #[doc = "Bit 7 - PDM Core Clock Enable"]
    #[inline(always)]
    pub fn pdmclken(&mut self) -> PDMCLKEN_W {
        PDMCLKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDM Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdmctrl](index.html) module"]
pub struct PDMCTRL_SPEC;
impl crate::RegisterSpec for PDMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdmctrl::R](R) reader structure"]
impl crate::Readable for PDMCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdmctrl::W](W) writer structure"]
impl crate::Writable for PDMCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMCTRL to value 0"]
impl crate::Resettable for PDMCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
