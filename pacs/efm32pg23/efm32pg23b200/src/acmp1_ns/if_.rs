#[doc = "Register `IF` reader"]
pub struct R(crate::R<IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IF` writer"]
pub struct W(crate::W<IF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF_SPEC>;
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
impl From<crate::W<IF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RISE` reader - Rising Edge Triggered Interrupt Flag"]
pub type RISE_R = crate::BitReader<bool>;
#[doc = "Field `RISE` writer - Rising Edge Triggered Interrupt Flag"]
pub type RISE_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 0>;
#[doc = "Field `FALL` reader - Falling Edge Triggered Interrupt Flag"]
pub type FALL_R = crate::BitReader<bool>;
#[doc = "Field `FALL` writer - Falling Edge Triggered Interrupt Flag"]
pub type FALL_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 1>;
#[doc = "Field `ACMPRDY` reader - ACMP ready Interrupt flag"]
pub type ACMPRDY_R = crate::BitReader<bool>;
#[doc = "Field `ACMPRDY` writer - ACMP ready Interrupt flag"]
pub type ACMPRDY_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 2>;
#[doc = "Field `INPUTCONFLICT` reader - Input conflict"]
pub type INPUTCONFLICT_R = crate::BitReader<bool>;
#[doc = "Field `INPUTCONFLICT` writer - Input conflict"]
pub type INPUTCONFLICT_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 3>;
#[doc = "Field `PORTALLOCERR` reader - Port allocation error"]
pub type PORTALLOCERR_R = crate::BitReader<bool>;
#[doc = "Field `PORTALLOCERR` writer - Port allocation error"]
pub type PORTALLOCERR_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 0 - Rising Edge Triggered Interrupt Flag"]
    #[inline(always)]
    pub fn rise(&self) -> RISE_R {
        RISE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling Edge Triggered Interrupt Flag"]
    #[inline(always)]
    pub fn fall(&self) -> FALL_R {
        FALL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ACMP ready Interrupt flag"]
    #[inline(always)]
    pub fn acmprdy(&self) -> ACMPRDY_R {
        ACMPRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Input conflict"]
    #[inline(always)]
    pub fn inputconflict(&self) -> INPUTCONFLICT_R {
        INPUTCONFLICT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port allocation error"]
    #[inline(always)]
    pub fn portallocerr(&self) -> PORTALLOCERR_R {
        PORTALLOCERR_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising Edge Triggered Interrupt Flag"]
    #[inline(always)]
    pub fn rise(&mut self) -> RISE_W {
        RISE_W::new(self)
    }
    #[doc = "Bit 1 - Falling Edge Triggered Interrupt Flag"]
    #[inline(always)]
    pub fn fall(&mut self) -> FALL_W {
        FALL_W::new(self)
    }
    #[doc = "Bit 2 - ACMP ready Interrupt flag"]
    #[inline(always)]
    pub fn acmprdy(&mut self) -> ACMPRDY_W {
        ACMPRDY_W::new(self)
    }
    #[doc = "Bit 3 - Input conflict"]
    #[inline(always)]
    pub fn inputconflict(&mut self) -> INPUTCONFLICT_W {
        INPUTCONFLICT_W::new(self)
    }
    #[doc = "Bit 4 - Port allocation error"]
    #[inline(always)]
    pub fn portallocerr(&mut self) -> PORTALLOCERR_W {
        PORTALLOCERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_::R](R) reader structure"]
impl crate::Readable for IF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if_::W](W) writer structure"]
impl crate::Writable for IF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
