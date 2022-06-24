#[doc = "Register `CRYPTOACCCLKCTRL` reader"]
pub struct R(crate::R<CRYPTOACCCLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYPTOACCCLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYPTOACCCLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYPTOACCCLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRYPTOACCCLKCTRL` writer"]
pub struct W(crate::W<CRYPTOACCCLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYPTOACCCLKCTRL_SPEC>;
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
impl From<crate::W<CRYPTOACCCLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYPTOACCCLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PKEN` reader - PK Enable"]
pub type PKEN_R = crate::BitReader<bool>;
#[doc = "Field `PKEN` writer - PK Enable"]
pub type PKEN_W<'a> = crate::BitWriter<'a, u32, CRYPTOACCCLKCTRL_SPEC, bool, 0>;
#[doc = "Field `AESEN` reader - AES Enable"]
pub type AESEN_R = crate::BitReader<bool>;
#[doc = "Field `AESEN` writer - AES Enable"]
pub type AESEN_W<'a> = crate::BitWriter<'a, u32, CRYPTOACCCLKCTRL_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - PK Enable"]
    #[inline(always)]
    pub fn pken(&self) -> PKEN_R {
        PKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AES Enable"]
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PK Enable"]
    #[inline(always)]
    pub fn pken(&mut self) -> PKEN_W {
        PKEN_W::new(self)
    }
    #[doc = "Bit 1 - AES Enable"]
    #[inline(always)]
    pub fn aesen(&mut self) -> AESEN_W {
        AESEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryptoaccclkctrl](index.html) module"]
pub struct CRYPTOACCCLKCTRL_SPEC;
impl crate::RegisterSpec for CRYPTOACCCLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cryptoaccclkctrl::R](R) reader structure"]
impl crate::Readable for CRYPTOACCCLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cryptoaccclkctrl::W](W) writer structure"]
impl crate::Writable for CRYPTOACCCLKCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRYPTOACCCLKCTRL to value 0"]
impl crate::Resettable for CRYPTOACCCLKCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
