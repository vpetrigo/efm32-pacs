#[doc = "Register `EM01GRPACLKCTRL` reader"]
pub struct R(crate::R<EM01GRPACLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EM01GRPACLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EM01GRPACLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EM01GRPACLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EM01GRPACLKCTRL` writer"]
pub struct W(crate::W<EM01GRPACLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EM01GRPACLKCTRL_SPEC>;
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
impl From<crate::W<EM01GRPACLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EM01GRPACLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "1: HFRCODPLL is clocking EM01GRPACLK"]
    HFRCODPLL = 1,
    #[doc = "2: HFXO is clocking EM01GRPACLK"]
    HFXO = 2,
    #[doc = "3: FSRCO is clocking EM01GRPACLK"]
    FSRCO = 3,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLKSEL` reader - Clock Select"]
pub type CLKSEL_R = crate::FieldReader<u8, CLKSEL_A>;
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKSEL_A> {
        match self.bits {
            1 => Some(CLKSEL_A::HFRCODPLL),
            2 => Some(CLKSEL_A::HFXO),
            3 => Some(CLKSEL_A::FSRCO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HFRCODPLL`"]
    #[inline(always)]
    pub fn is_hfrcodpll(&self) -> bool {
        *self == CLKSEL_A::HFRCODPLL
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == CLKSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `FSRCO`"]
    #[inline(always)]
    pub fn is_fsrco(&self) -> bool {
        *self == CLKSEL_A::FSRCO
    }
}
#[doc = "Field `CLKSEL` writer - Clock Select"]
pub type CLKSEL_W<'a> = crate::FieldWriter<'a, u32, EM01GRPACLKCTRL_SPEC, u8, CLKSEL_A, 2, 0>;
impl<'a> CLKSEL_W<'a> {
    #[doc = "HFRCODPLL is clocking EM01GRPACLK"]
    #[inline(always)]
    pub fn hfrcodpll(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFRCODPLL)
    }
    #[doc = "HFXO is clocking EM01GRPACLK"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFXO)
    }
    #[doc = "FSRCO is clocking EM01GRPACLK"]
    #[inline(always)]
    pub fn fsrco(self) -> &'a mut W {
        self.variant(CLKSEL_A::FSRCO)
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock Select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Select"]
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W {
        CLKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [em01grpaclkctrl](index.html) module"]
pub struct EM01GRPACLKCTRL_SPEC;
impl crate::RegisterSpec for EM01GRPACLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [em01grpaclkctrl::R](R) reader structure"]
impl crate::Readable for EM01GRPACLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [em01grpaclkctrl::W](W) writer structure"]
impl crate::Writable for EM01GRPACLKCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EM01GRPACLKCTRL to value 0x01"]
impl crate::Resettable for EM01GRPACLKCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
