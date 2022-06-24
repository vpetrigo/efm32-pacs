#[doc = "Register `USBCTRL` reader"]
pub struct R(crate::R<USBCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBCTRL` writer"]
pub struct W(crate::W<USBCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBCTRL_SPEC>;
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
impl From<crate::W<USBCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB Rate Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USBCLKSEL_A {
    #[doc = "0: USHFRCO (clock recovery) is clocking USB"]
    USHFRCO = 0,
    #[doc = "1: HFXO clock is used to clock USB"]
    HFXO = 1,
    #[doc = "2: HFXO clock doubler is used to clock USB"]
    HFXOX2 = 2,
    #[doc = "3: HFRCO clock is used to clock USB"]
    HFRCO = 3,
    #[doc = "4: LFXO clock is used to clock USB"]
    LFXO = 4,
    #[doc = "5: LFRCO clock is used to clock USB"]
    LFRCO = 5,
}
impl From<USBCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USBCLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USBCLKSEL` reader - USB Rate Clock Select"]
pub type USBCLKSEL_R = crate::FieldReader<u8, USBCLKSEL_A>;
impl USBCLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USBCLKSEL_A> {
        match self.bits {
            0 => Some(USBCLKSEL_A::USHFRCO),
            1 => Some(USBCLKSEL_A::HFXO),
            2 => Some(USBCLKSEL_A::HFXOX2),
            3 => Some(USBCLKSEL_A::HFRCO),
            4 => Some(USBCLKSEL_A::LFXO),
            5 => Some(USBCLKSEL_A::LFRCO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `USHFRCO`"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == USBCLKSEL_A::USHFRCO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == USBCLKSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `HFXOX2`"]
    #[inline(always)]
    pub fn is_hfxox2(&self) -> bool {
        *self == USBCLKSEL_A::HFXOX2
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == USBCLKSEL_A::HFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == USBCLKSEL_A::LFXO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == USBCLKSEL_A::LFRCO
    }
}
#[doc = "Field `USBCLKSEL` writer - USB Rate Clock Select"]
pub type USBCLKSEL_W<'a> = crate::FieldWriter<'a, u32, USBCTRL_SPEC, u8, USBCLKSEL_A, 3, 0>;
impl<'a> USBCLKSEL_W<'a> {
    #[doc = "USHFRCO (clock recovery) is clocking USB"]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut W {
        self.variant(USBCLKSEL_A::USHFRCO)
    }
    #[doc = "HFXO clock is used to clock USB"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(USBCLKSEL_A::HFXO)
    }
    #[doc = "HFXO clock doubler is used to clock USB"]
    #[inline(always)]
    pub fn hfxox2(self) -> &'a mut W {
        self.variant(USBCLKSEL_A::HFXOX2)
    }
    #[doc = "HFRCO clock is used to clock USB"]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(USBCLKSEL_A::HFRCO)
    }
    #[doc = "LFXO clock is used to clock USB"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(USBCLKSEL_A::LFXO)
    }
    #[doc = "LFRCO clock is used to clock USB"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(USBCLKSEL_A::LFRCO)
    }
}
#[doc = "Field `USBCLKEN` reader - USB Rate Clock Enable"]
pub type USBCLKEN_R = crate::BitReader<bool>;
#[doc = "Field `USBCLKEN` writer - USB Rate Clock Enable"]
pub type USBCLKEN_W<'a> = crate::BitWriter<'a, u32, USBCTRL_SPEC, bool, 7>;
impl R {
    #[doc = "Bits 0:2 - USB Rate Clock Select"]
    #[inline(always)]
    pub fn usbclksel(&self) -> USBCLKSEL_R {
        USBCLKSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 7 - USB Rate Clock Enable"]
    #[inline(always)]
    pub fn usbclken(&self) -> USBCLKEN_R {
        USBCLKEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - USB Rate Clock Select"]
    #[inline(always)]
    pub fn usbclksel(&mut self) -> USBCLKSEL_W {
        USBCLKSEL_W::new(self)
    }
    #[doc = "Bit 7 - USB Rate Clock Enable"]
    #[inline(always)]
    pub fn usbclken(&mut self) -> USBCLKEN_W {
        USBCLKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbctrl](index.html) module"]
pub struct USBCTRL_SPEC;
impl crate::RegisterSpec for USBCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbctrl::R](R) reader structure"]
impl crate::Readable for USBCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbctrl::W](W) writer structure"]
impl crate::Writable for USBCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBCTRL to value 0"]
impl crate::Resettable for USBCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
