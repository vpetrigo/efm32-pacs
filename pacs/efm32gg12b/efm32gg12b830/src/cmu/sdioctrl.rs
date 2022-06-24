#[doc = "Register `SDIOCTRL` reader"]
pub struct R(crate::R<SDIOCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIOCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIOCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIOCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIOCTRL` writer"]
pub struct W(crate::W<SDIOCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIOCTRL_SPEC>;
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
impl From<crate::W<SDIOCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIOCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SDIO Reference Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SDIOCLKSEL_A {
    #[doc = "0: HFRCO clock is used to clock SDIO"]
    HFRCO = 0,
    #[doc = "1: HFXO clock is used to clock SDIO"]
    HFXO = 1,
    #[doc = "2: AUXHFRCO is used to clock SDIO"]
    AUXHFRCO = 2,
    #[doc = "3: USHFRCO is used to clock SDIO"]
    USHFRCO = 3,
}
impl From<SDIOCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SDIOCLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SDIOCLKSEL` reader - SDIO Reference Clock Select"]
pub type SDIOCLKSEL_R = crate::FieldReader<u8, SDIOCLKSEL_A>;
impl SDIOCLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIOCLKSEL_A {
        match self.bits {
            0 => SDIOCLKSEL_A::HFRCO,
            1 => SDIOCLKSEL_A::HFXO,
            2 => SDIOCLKSEL_A::AUXHFRCO,
            3 => SDIOCLKSEL_A::USHFRCO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == SDIOCLKSEL_A::HFRCO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == SDIOCLKSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == SDIOCLKSEL_A::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `USHFRCO`"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == SDIOCLKSEL_A::USHFRCO
    }
}
#[doc = "Field `SDIOCLKSEL` writer - SDIO Reference Clock Select"]
pub type SDIOCLKSEL_W<'a> = crate::FieldWriterSafe<'a, u32, SDIOCTRL_SPEC, u8, SDIOCLKSEL_A, 2, 0>;
impl<'a> SDIOCLKSEL_W<'a> {
    #[doc = "HFRCO clock is used to clock SDIO"]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(SDIOCLKSEL_A::HFRCO)
    }
    #[doc = "HFXO clock is used to clock SDIO"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(SDIOCLKSEL_A::HFXO)
    }
    #[doc = "AUXHFRCO is used to clock SDIO"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(SDIOCLKSEL_A::AUXHFRCO)
    }
    #[doc = "USHFRCO is used to clock SDIO"]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut W {
        self.variant(SDIOCLKSEL_A::USHFRCO)
    }
}
#[doc = "Field `SDIOCLKDIS` reader - SDIO Reference Clock Disable"]
pub type SDIOCLKDIS_R = crate::BitReader<bool>;
#[doc = "Field `SDIOCLKDIS` writer - SDIO Reference Clock Disable"]
pub type SDIOCLKDIS_W<'a> = crate::BitWriter<'a, u32, SDIOCTRL_SPEC, bool, 7>;
impl R {
    #[doc = "Bits 0:1 - SDIO Reference Clock Select"]
    #[inline(always)]
    pub fn sdioclksel(&self) -> SDIOCLKSEL_R {
        SDIOCLKSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 7 - SDIO Reference Clock Disable"]
    #[inline(always)]
    pub fn sdioclkdis(&self) -> SDIOCLKDIS_R {
        SDIOCLKDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SDIO Reference Clock Select"]
    #[inline(always)]
    pub fn sdioclksel(&mut self) -> SDIOCLKSEL_W {
        SDIOCLKSEL_W::new(self)
    }
    #[doc = "Bit 7 - SDIO Reference Clock Disable"]
    #[inline(always)]
    pub fn sdioclkdis(&mut self) -> SDIOCLKDIS_W {
        SDIOCLKDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDIO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdioctrl](index.html) module"]
pub struct SDIOCTRL_SPEC;
impl crate::RegisterSpec for SDIOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdioctrl::R](R) reader structure"]
impl crate::Readable for SDIOCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdioctrl::W](W) writer structure"]
impl crate::Writable for SDIOCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDIOCTRL to value 0"]
impl crate::Resettable for SDIOCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
