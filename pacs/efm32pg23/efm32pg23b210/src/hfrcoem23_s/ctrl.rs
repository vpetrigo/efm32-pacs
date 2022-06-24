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
#[doc = "Field `FORCEEN` reader - Force Enable"]
pub type FORCEEN_R = crate::BitReader<bool>;
#[doc = "Field `FORCEEN` writer - Force Enable"]
pub type FORCEEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 0>;
#[doc = "Field `DISONDEMAND` reader - Disable On-demand"]
pub type DISONDEMAND_R = crate::BitReader<bool>;
#[doc = "Field `DISONDEMAND` writer - Disable On-demand"]
pub type DISONDEMAND_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 1>;
#[doc = "Field `EM23ONDEMAND` reader - EM23 On-demand"]
pub type EM23ONDEMAND_R = crate::BitReader<bool>;
#[doc = "Field `EM23ONDEMAND` writer - EM23 On-demand"]
pub type EM23ONDEMAND_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 2>;
impl R {
    #[doc = "Bit 0 - Force Enable"]
    #[inline(always)]
    pub fn forceen(&self) -> FORCEEN_R {
        FORCEEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable On-demand"]
    #[inline(always)]
    pub fn disondemand(&self) -> DISONDEMAND_R {
        DISONDEMAND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EM23 On-demand"]
    #[inline(always)]
    pub fn em23ondemand(&self) -> EM23ONDEMAND_R {
        EM23ONDEMAND_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force Enable"]
    #[inline(always)]
    pub fn forceen(&mut self) -> FORCEEN_W {
        FORCEEN_W::new(self)
    }
    #[doc = "Bit 1 - Disable On-demand"]
    #[inline(always)]
    pub fn disondemand(&mut self) -> DISONDEMAND_W {
        DISONDEMAND_W::new(self)
    }
    #[doc = "Bit 2 - EM23 On-demand"]
    #[inline(always)]
    pub fn em23ondemand(&mut self) -> EM23ONDEMAND_W {
        EM23ONDEMAND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
