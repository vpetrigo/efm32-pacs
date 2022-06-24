#[doc = "Register `EUART0_ROUTEEN` reader"]
pub struct R(crate::R<EUART0_ROUTEEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EUART0_ROUTEEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EUART0_ROUTEEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EUART0_ROUTEEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EUART0_ROUTEEN` writer"]
pub struct W(crate::W<EUART0_ROUTEEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EUART0_ROUTEEN_SPEC>;
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
impl From<crate::W<EUART0_ROUTEEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EUART0_ROUTEEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTSPEN` reader - RTS pin enable control bit"]
pub type RTSPEN_R = crate::BitReader<bool>;
#[doc = "Field `RTSPEN` writer - RTS pin enable control bit"]
pub type RTSPEN_W<'a> = crate::BitWriter<'a, u32, EUART0_ROUTEEN_SPEC, bool, 0>;
#[doc = "Field `TXPEN` reader - TX pin enable control bit"]
pub type TXPEN_R = crate::BitReader<bool>;
#[doc = "Field `TXPEN` writer - TX pin enable control bit"]
pub type TXPEN_W<'a> = crate::BitWriter<'a, u32, EUART0_ROUTEEN_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - RTS pin enable control bit"]
    #[inline(always)]
    pub fn rtspen(&self) -> RTSPEN_R {
        RTSPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX pin enable control bit"]
    #[inline(always)]
    pub fn txpen(&self) -> TXPEN_R {
        TXPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTS pin enable control bit"]
    #[inline(always)]
    pub fn rtspen(&mut self) -> RTSPEN_W {
        RTSPEN_W::new(self)
    }
    #[doc = "Bit 1 - TX pin enable control bit"]
    #[inline(always)]
    pub fn txpen(&mut self) -> TXPEN_W {
        TXPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EUART pin enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [euart0_routeen](index.html) module"]
pub struct EUART0_ROUTEEN_SPEC;
impl crate::RegisterSpec for EUART0_ROUTEEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [euart0_routeen::R](R) reader structure"]
impl crate::Readable for EUART0_ROUTEEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [euart0_routeen::W](W) writer structure"]
impl crate::Writable for EUART0_ROUTEEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EUART0_ROUTEEN to value 0"]
impl crate::Resettable for EUART0_ROUTEEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
