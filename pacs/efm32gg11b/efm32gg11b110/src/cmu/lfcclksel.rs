#[doc = "Register `LFCCLKSEL` reader"]
pub struct R(crate::R<LFCCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFCCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFCCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFCCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LFCCLKSEL` writer"]
pub struct W(crate::W<LFCCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LFCCLKSEL_SPEC>;
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
impl From<crate::W<LFCCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LFCCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock Select for LFC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LFC_A {
    #[doc = "0: LFCCLK is disabled"]
    DISABLED = 0,
    #[doc = "1: LFRCO selected as LFCCLK"]
    LFRCO = 1,
    #[doc = "2: LFXO selected as LFCCLK"]
    LFXO = 2,
    #[doc = "4: ULFRCO selected as LFCCLK"]
    ULFRCO = 4,
}
impl From<LFC_A> for u8 {
    #[inline(always)]
    fn from(variant: LFC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LFC` reader - Clock Select for LFC"]
pub type LFC_R = crate::FieldReader<u8, LFC_A>;
impl LFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LFC_A> {
        match self.bits {
            0 => Some(LFC_A::DISABLED),
            1 => Some(LFC_A::LFRCO),
            2 => Some(LFC_A::LFXO),
            4 => Some(LFC_A::ULFRCO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == LFC_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == LFC_A::LFXO
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == LFC_A::ULFRCO
    }
}
#[doc = "Field `LFC` writer - Clock Select for LFC"]
pub type LFC_W<'a> = crate::FieldWriter<'a, u32, LFCCLKSEL_SPEC, u8, LFC_A, 3, 0>;
impl<'a> LFC_W<'a> {
    #[doc = "LFCCLK is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LFC_A::DISABLED)
    }
    #[doc = "LFRCO selected as LFCCLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(LFC_A::LFRCO)
    }
    #[doc = "LFXO selected as LFCCLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(LFC_A::LFXO)
    }
    #[doc = "ULFRCO selected as LFCCLK"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(LFC_A::ULFRCO)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Select for LFC"]
    #[inline(always)]
    pub fn lfc(&self) -> LFC_R {
        LFC_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select for LFC"]
    #[inline(always)]
    pub fn lfc(&mut self) -> LFC_W {
        LFC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Frequency C Clock Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfcclksel](index.html) module"]
pub struct LFCCLKSEL_SPEC;
impl crate::RegisterSpec for LFCCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfcclksel::R](R) reader structure"]
impl crate::Readable for LFCCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lfcclksel::W](W) writer structure"]
impl crate::Writable for LFCCLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LFCCLKSEL to value 0"]
impl crate::Resettable for LFCCLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
