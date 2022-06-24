#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCEHP` reader - Force High Power"]
pub type FORCEHP_R = crate::BitReader<bool>;
#[doc = "Field `FORCEHP` writer - Force High Power"]
pub type FORCEHP_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 0>;
#[doc = "Field `FORCELP` reader - Force Low Power"]
pub type FORCELP_R = crate::BitReader<bool>;
#[doc = "Field `FORCELP` writer - Force Low Power"]
pub type FORCELP_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 1>;
#[doc = "Field `FORCERUN` reader - Force run"]
pub type FORCERUN_R = crate::BitReader<bool>;
#[doc = "Field `FORCERUN` writer - Force run"]
pub type FORCERUN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 4>;
#[doc = "Field `FORCESTOP` reader - Force stop"]
pub type FORCESTOP_R = crate::BitReader<bool>;
#[doc = "Field `FORCESTOP` writer - Force stop"]
pub type FORCESTOP_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - Force High Power"]
    #[inline(always)]
    pub fn forcehp(&self) -> FORCEHP_R {
        FORCEHP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force Low Power"]
    #[inline(always)]
    pub fn forcelp(&self) -> FORCELP_R {
        FORCELP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Force run"]
    #[inline(always)]
    pub fn forcerun(&self) -> FORCERUN_R {
        FORCERUN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Force stop"]
    #[inline(always)]
    pub fn forcestop(&self) -> FORCESTOP_R {
        FORCESTOP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force High Power"]
    #[inline(always)]
    pub fn forcehp(&mut self) -> FORCEHP_W {
        FORCEHP_W::new(self)
    }
    #[doc = "Bit 1 - Force Low Power"]
    #[inline(always)]
    pub fn forcelp(&mut self) -> FORCELP_W {
        FORCELP_W::new(self)
    }
    #[doc = "Bit 4 - Force run"]
    #[inline(always)]
    pub fn forcerun(&mut self) -> FORCERUN_W {
        FORCERUN_W::new(self)
    }
    #[doc = "Bit 5 - Force stop"]
    #[inline(always)]
    pub fn forcestop(&mut self) -> FORCESTOP_W {
        FORCESTOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
