#[doc = "Register `EM01GRPBCLKCTRL` reader"]
pub struct R(crate::R<EM01GRPBCLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EM01GRPBCLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EM01GRPBCLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EM01GRPBCLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EM01GRPBCLKCTRL` writer"]
pub struct W(crate::W<EM01GRPBCLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EM01GRPBCLKCTRL_SPEC>;
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
impl From<crate::W<EM01GRPBCLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EM01GRPBCLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "1: HFRCODPLL is clocking EM01GRPBCLK"]
    HFRCODPLL = 1,
    #[doc = "2: HFXO is clocking EM01GRPBCLK"]
    HFXO = 2,
    #[doc = "3: FSRCO is clocking EM01GRPBCLK"]
    FSRCO = 3,
    #[doc = "4: CLKIN0 is clocking EM01GRPBCLK"]
    CLKIN0 = 4,
    #[doc = "5: HFRCODPLL (re-timed) is clocking EM01GRPBCLK"]
    HFRCODPLLRT = 5,
    #[doc = "6: HFXO (re-timed) is clocking EM01GRPBCLK"]
    HFXORT = 6,
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
            4 => Some(CLKSEL_A::CLKIN0),
            5 => Some(CLKSEL_A::HFRCODPLLRT),
            6 => Some(CLKSEL_A::HFXORT),
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
    #[doc = "Checks if the value of the field is `CLKIN0`"]
    #[inline(always)]
    pub fn is_clkin0(&self) -> bool {
        *self == CLKSEL_A::CLKIN0
    }
    #[doc = "Checks if the value of the field is `HFRCODPLLRT`"]
    #[inline(always)]
    pub fn is_hfrcodpllrt(&self) -> bool {
        *self == CLKSEL_A::HFRCODPLLRT
    }
    #[doc = "Checks if the value of the field is `HFXORT`"]
    #[inline(always)]
    pub fn is_hfxort(&self) -> bool {
        *self == CLKSEL_A::HFXORT
    }
}
#[doc = "Field `CLKSEL` writer - Clock Select"]
pub type CLKSEL_W<'a> = crate::FieldWriter<'a, u32, EM01GRPBCLKCTRL_SPEC, u8, CLKSEL_A, 3, 0>;
impl<'a> CLKSEL_W<'a> {
    #[doc = "HFRCODPLL is clocking EM01GRPBCLK"]
    #[inline(always)]
    pub fn hfrcodpll(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFRCODPLL)
    }
    #[doc = "HFXO is clocking EM01GRPBCLK"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFXO)
    }
    #[doc = "FSRCO is clocking EM01GRPBCLK"]
    #[inline(always)]
    pub fn fsrco(self) -> &'a mut W {
        self.variant(CLKSEL_A::FSRCO)
    }
    #[doc = "CLKIN0 is clocking EM01GRPBCLK"]
    #[inline(always)]
    pub fn clkin0(self) -> &'a mut W {
        self.variant(CLKSEL_A::CLKIN0)
    }
    #[doc = "HFRCODPLL (re-timed) is clocking EM01GRPBCLK"]
    #[inline(always)]
    pub fn hfrcodpllrt(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFRCODPLLRT)
    }
    #[doc = "HFXO (re-timed) is clocking EM01GRPBCLK"]
    #[inline(always)]
    pub fn hfxort(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFXORT)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select"]
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
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [em01grpbclkctrl](index.html) module"]
pub struct EM01GRPBCLKCTRL_SPEC;
impl crate::RegisterSpec for EM01GRPBCLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [em01grpbclkctrl::R](R) reader structure"]
impl crate::Readable for EM01GRPBCLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [em01grpbclkctrl::W](W) writer structure"]
impl crate::Writable for EM01GRPBCLKCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EM01GRPBCLKCTRL to value 0x01"]
impl crate::Resettable for EM01GRPBCLKCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
