#[doc = "Register `ROUTEPEN` reader"]
pub struct R(crate::R<ROUTEPEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROUTEPEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROUTEPEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROUTEPEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROUTEPEN` writer"]
pub struct W(crate::W<ROUTEPEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROUTEPEN_SPEC>;
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
impl From<crate::W<ROUTEPEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROUTEPEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDIOPEN` reader - MDIO I/O Enable"]
pub type MDIOPEN_R = crate::BitReader<bool>;
#[doc = "Field `MDIOPEN` writer - MDIO I/O Enable"]
pub type MDIOPEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 0>;
#[doc = "Field `MIITXERPEN` reader - MII TX ER I/O Enable"]
pub type MIITXERPEN_R = crate::BitReader<bool>;
#[doc = "Field `MIITXERPEN` writer - MII TX ER I/O Enable"]
pub type MIITXERPEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 1>;
#[doc = "Field `MIIRXERPEN` reader - MII TX ER I/O Enable"]
pub type MIIRXERPEN_R = crate::BitReader<bool>;
#[doc = "Field `MIIRXERPEN` writer - MII TX ER I/O Enable"]
pub type MIIRXERPEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 2>;
#[doc = "Field `MIIPEN` reader - MII I/O Enable"]
pub type MIIPEN_R = crate::BitReader<bool>;
#[doc = "Field `MIIPEN` writer - MII I/O Enable"]
pub type MIIPEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 3>;
#[doc = "Field `RMIIPEN` reader - RMII I/O Enable"]
pub type RMIIPEN_R = crate::BitReader<bool>;
#[doc = "Field `RMIIPEN` writer - RMII I/O Enable"]
pub type RMIIPEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 4>;
#[doc = "Field `TSUTMRTOGPEN` reader - TSU_TMR_CNT_SEC Output Enable"]
pub type TSUTMRTOGPEN_R = crate::BitReader<bool>;
#[doc = "Field `TSUTMRTOGPEN` writer - TSU_TMR_CNT_SEC Output Enable"]
pub type TSUTMRTOGPEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - MDIO I/O Enable"]
    #[inline(always)]
    pub fn mdiopen(&self) -> MDIOPEN_R {
        MDIOPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MII TX ER I/O Enable"]
    #[inline(always)]
    pub fn miitxerpen(&self) -> MIITXERPEN_R {
        MIITXERPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MII TX ER I/O Enable"]
    #[inline(always)]
    pub fn miirxerpen(&self) -> MIIRXERPEN_R {
        MIIRXERPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MII I/O Enable"]
    #[inline(always)]
    pub fn miipen(&self) -> MIIPEN_R {
        MIIPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RMII I/O Enable"]
    #[inline(always)]
    pub fn rmiipen(&self) -> RMIIPEN_R {
        RMIIPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TSU_TMR_CNT_SEC Output Enable"]
    #[inline(always)]
    pub fn tsutmrtogpen(&self) -> TSUTMRTOGPEN_R {
        TSUTMRTOGPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDIO I/O Enable"]
    #[inline(always)]
    pub fn mdiopen(&mut self) -> MDIOPEN_W {
        MDIOPEN_W::new(self)
    }
    #[doc = "Bit 1 - MII TX ER I/O Enable"]
    #[inline(always)]
    pub fn miitxerpen(&mut self) -> MIITXERPEN_W {
        MIITXERPEN_W::new(self)
    }
    #[doc = "Bit 2 - MII TX ER I/O Enable"]
    #[inline(always)]
    pub fn miirxerpen(&mut self) -> MIIRXERPEN_W {
        MIIRXERPEN_W::new(self)
    }
    #[doc = "Bit 3 - MII I/O Enable"]
    #[inline(always)]
    pub fn miipen(&mut self) -> MIIPEN_W {
        MIIPEN_W::new(self)
    }
    #[doc = "Bit 4 - RMII I/O Enable"]
    #[inline(always)]
    pub fn rmiipen(&mut self) -> RMIIPEN_W {
        RMIIPEN_W::new(self)
    }
    #[doc = "Bit 5 - TSU_TMR_CNT_SEC Output Enable"]
    #[inline(always)]
    pub fn tsutmrtogpen(&mut self) -> TSUTMRTOGPEN_W {
        TSUTMRTOGPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Route Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routepen](index.html) module"]
pub struct ROUTEPEN_SPEC;
impl crate::RegisterSpec for ROUTEPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [routepen::R](R) reader structure"]
impl crate::Readable for ROUTEPEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [routepen::W](W) writer structure"]
impl crate::Writable for ROUTEPEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROUTEPEN to value 0"]
impl crate::Resettable for ROUTEPEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
