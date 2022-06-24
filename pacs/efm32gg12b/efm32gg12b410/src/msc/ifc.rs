#[doc = "Register `IFC` writer"]
pub struct W(crate::W<IFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFC_SPEC>;
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
impl From<crate::W<IFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERASE` writer - Clear ERASE Interrupt Flag"]
pub type ERASE_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 0>;
#[doc = "Field `WRITE` writer - Clear WRITE Interrupt Flag"]
pub type WRITE_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 1>;
#[doc = "Field `CHOF` writer - Clear CHOF Interrupt Flag"]
pub type CHOF_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 2>;
#[doc = "Field `CMOF` writer - Clear CMOF Interrupt Flag"]
pub type CMOF_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 3>;
#[doc = "Field `PWRUPF` writer - Clear PWRUPF Interrupt Flag"]
pub type PWRUPF_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 4>;
#[doc = "Field `ICACHERR` writer - Clear ICACHERR Interrupt Flag"]
pub type ICACHERR_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 5>;
#[doc = "Field `WDATAOV` writer - Clear WDATAOV Interrupt Flag"]
pub type WDATAOV_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 6>;
#[doc = "Field `LVEWRITE` writer - Clear LVEWRITE Interrupt Flag"]
pub type LVEWRITE_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 8>;
#[doc = "Field `RAMERR1B` writer - Clear RAMERR1B Interrupt Flag"]
pub type RAMERR1B_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 16>;
#[doc = "Field `RAMERR2B` writer - Clear RAMERR2B Interrupt Flag"]
pub type RAMERR2B_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 17>;
#[doc = "Field `RAM1ERR1B` writer - Clear RAM1ERR1B Interrupt Flag"]
pub type RAM1ERR1B_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 18>;
#[doc = "Field `RAM1ERR2B` writer - Clear RAM1ERR2B Interrupt Flag"]
pub type RAM1ERR2B_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 19>;
#[doc = "Field `RAM2ERR1B` writer - Clear RAM2ERR1B Interrupt Flag"]
pub type RAM2ERR1B_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 20>;
#[doc = "Field `RAM2ERR2B` writer - Clear RAM2ERR2B Interrupt Flag"]
pub type RAM2ERR2B_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 21>;
impl W {
    #[doc = "Bit 0 - Clear ERASE Interrupt Flag"]
    #[inline(always)]
    pub fn erase(&mut self) -> ERASE_W {
        ERASE_W::new(self)
    }
    #[doc = "Bit 1 - Clear WRITE Interrupt Flag"]
    #[inline(always)]
    pub fn write(&mut self) -> WRITE_W {
        WRITE_W::new(self)
    }
    #[doc = "Bit 2 - Clear CHOF Interrupt Flag"]
    #[inline(always)]
    pub fn chof(&mut self) -> CHOF_W {
        CHOF_W::new(self)
    }
    #[doc = "Bit 3 - Clear CMOF Interrupt Flag"]
    #[inline(always)]
    pub fn cmof(&mut self) -> CMOF_W {
        CMOF_W::new(self)
    }
    #[doc = "Bit 4 - Clear PWRUPF Interrupt Flag"]
    #[inline(always)]
    pub fn pwrupf(&mut self) -> PWRUPF_W {
        PWRUPF_W::new(self)
    }
    #[doc = "Bit 5 - Clear ICACHERR Interrupt Flag"]
    #[inline(always)]
    pub fn icacherr(&mut self) -> ICACHERR_W {
        ICACHERR_W::new(self)
    }
    #[doc = "Bit 6 - Clear WDATAOV Interrupt Flag"]
    #[inline(always)]
    pub fn wdataov(&mut self) -> WDATAOV_W {
        WDATAOV_W::new(self)
    }
    #[doc = "Bit 8 - Clear LVEWRITE Interrupt Flag"]
    #[inline(always)]
    pub fn lvewrite(&mut self) -> LVEWRITE_W {
        LVEWRITE_W::new(self)
    }
    #[doc = "Bit 16 - Clear RAMERR1B Interrupt Flag"]
    #[inline(always)]
    pub fn ramerr1b(&mut self) -> RAMERR1B_W {
        RAMERR1B_W::new(self)
    }
    #[doc = "Bit 17 - Clear RAMERR2B Interrupt Flag"]
    #[inline(always)]
    pub fn ramerr2b(&mut self) -> RAMERR2B_W {
        RAMERR2B_W::new(self)
    }
    #[doc = "Bit 18 - Clear RAM1ERR1B Interrupt Flag"]
    #[inline(always)]
    pub fn ram1err1b(&mut self) -> RAM1ERR1B_W {
        RAM1ERR1B_W::new(self)
    }
    #[doc = "Bit 19 - Clear RAM1ERR2B Interrupt Flag"]
    #[inline(always)]
    pub fn ram1err2b(&mut self) -> RAM1ERR2B_W {
        RAM1ERR2B_W::new(self)
    }
    #[doc = "Bit 20 - Clear RAM2ERR1B Interrupt Flag"]
    #[inline(always)]
    pub fn ram2err1b(&mut self) -> RAM2ERR1B_W {
        RAM2ERR1B_W::new(self)
    }
    #[doc = "Bit 21 - Clear RAM2ERR2B Interrupt Flag"]
    #[inline(always)]
    pub fn ram2err2b(&mut self) -> RAM2ERR2B_W {
        RAM2ERR2B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifc](index.html) module"]
pub struct IFC_SPEC;
impl crate::RegisterSpec for IFC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifc::W](W) writer structure"]
impl crate::Writable for IFC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
