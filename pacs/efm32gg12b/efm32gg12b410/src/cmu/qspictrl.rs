#[doc = "Register `QSPICTRL` reader"]
pub struct R(crate::R<QSPICTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPICTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPICTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPICTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPICTRL` writer"]
pub struct W(crate::W<QSPICTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPICTRL_SPEC>;
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
impl From<crate::W<QSPICTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPICTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "QSPI0 Reference Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum QSPI0CLKSEL_A {
    #[doc = "0: HFRCO clock is used to clock QSPI0"]
    HFRCO = 0,
    #[doc = "1: HFXO clock is used to clock QSPI0"]
    HFXO = 1,
    #[doc = "2: AUXHFRCO is used to clock QSPI0"]
    AUXHFRCO = 2,
    #[doc = "3: USHFRCO is used to clock QSPI0"]
    USHFRCO = 3,
}
impl From<QSPI0CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: QSPI0CLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `QSPI0CLKSEL` reader - QSPI0 Reference Clock Select"]
pub type QSPI0CLKSEL_R = crate::FieldReader<u8, QSPI0CLKSEL_A>;
impl QSPI0CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QSPI0CLKSEL_A {
        match self.bits {
            0 => QSPI0CLKSEL_A::HFRCO,
            1 => QSPI0CLKSEL_A::HFXO,
            2 => QSPI0CLKSEL_A::AUXHFRCO,
            3 => QSPI0CLKSEL_A::USHFRCO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == QSPI0CLKSEL_A::HFRCO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == QSPI0CLKSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == QSPI0CLKSEL_A::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `USHFRCO`"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == QSPI0CLKSEL_A::USHFRCO
    }
}
#[doc = "Field `QSPI0CLKSEL` writer - QSPI0 Reference Clock Select"]
pub type QSPI0CLKSEL_W<'a> =
    crate::FieldWriterSafe<'a, u32, QSPICTRL_SPEC, u8, QSPI0CLKSEL_A, 2, 0>;
impl<'a> QSPI0CLKSEL_W<'a> {
    #[doc = "HFRCO clock is used to clock QSPI0"]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(QSPI0CLKSEL_A::HFRCO)
    }
    #[doc = "HFXO clock is used to clock QSPI0"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(QSPI0CLKSEL_A::HFXO)
    }
    #[doc = "AUXHFRCO is used to clock QSPI0"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(QSPI0CLKSEL_A::AUXHFRCO)
    }
    #[doc = "USHFRCO is used to clock QSPI0"]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut W {
        self.variant(QSPI0CLKSEL_A::USHFRCO)
    }
}
#[doc = "Field `QSPI0CLKDIS` reader - QSPI0 Reference Clock Disable"]
pub type QSPI0CLKDIS_R = crate::BitReader<bool>;
#[doc = "Field `QSPI0CLKDIS` writer - QSPI0 Reference Clock Disable"]
pub type QSPI0CLKDIS_W<'a> = crate::BitWriter<'a, u32, QSPICTRL_SPEC, bool, 7>;
impl R {
    #[doc = "Bits 0:1 - QSPI0 Reference Clock Select"]
    #[inline(always)]
    pub fn qspi0clksel(&self) -> QSPI0CLKSEL_R {
        QSPI0CLKSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 7 - QSPI0 Reference Clock Disable"]
    #[inline(always)]
    pub fn qspi0clkdis(&self) -> QSPI0CLKDIS_R {
        QSPI0CLKDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - QSPI0 Reference Clock Select"]
    #[inline(always)]
    pub fn qspi0clksel(&mut self) -> QSPI0CLKSEL_W {
        QSPI0CLKSEL_W::new(self)
    }
    #[doc = "Bit 7 - QSPI0 Reference Clock Disable"]
    #[inline(always)]
    pub fn qspi0clkdis(&mut self) -> QSPI0CLKDIS_W {
        QSPI0CLKDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspictrl](index.html) module"]
pub struct QSPICTRL_SPEC;
impl crate::RegisterSpec for QSPICTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspictrl::R](R) reader structure"]
impl crate::Readable for QSPICTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspictrl::W](W) writer structure"]
impl crate::Writable for QSPICTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPICTRL to value 0"]
impl crate::Resettable for QSPICTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
