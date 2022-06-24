#[doc = "Register `IEN` reader"]
pub struct R(crate::R<IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEN` writer"]
pub struct W(crate::W<IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEN_SPEC>;
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
impl From<crate::W<IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDY` reader - Digital Clock Ready Interrupt"]
pub type RDY_R = crate::BitReader<bool>;
#[doc = "Field `RDY` writer - Digital Clock Ready Interrupt"]
pub type RDY_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 0>;
#[doc = "Field `COREBIASOPTRDY` reader - Core Bias Optimization Ready Interrupt"]
pub type COREBIASOPTRDY_R = crate::BitReader<bool>;
#[doc = "Field `COREBIASOPTRDY` writer - Core Bias Optimization Ready Interrupt"]
pub type COREBIASOPTRDY_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 1>;
#[doc = "Field `PRSRDY` reader - PRS Ready Interrupt"]
pub type PRSRDY_R = crate::BitReader<bool>;
#[doc = "Field `PRSRDY` writer - PRS Ready Interrupt"]
pub type PRSRDY_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 2>;
#[doc = "Field `BUFOUTRDY` reader - BUFOUT Ready Interrupt"]
pub type BUFOUTRDY_R = crate::BitReader<bool>;
#[doc = "Field `BUFOUTRDY` writer - BUFOUT Ready Interrupt"]
pub type BUFOUTRDY_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 3>;
#[doc = "Field `BUFOUTFROZEN` reader - BUFOUT FROZEN Interrupt"]
pub type BUFOUTFROZEN_R = crate::BitReader<bool>;
#[doc = "Field `BUFOUTFROZEN` writer - BUFOUT FROZEN Interrupt"]
pub type BUFOUTFROZEN_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 15>;
#[doc = "Field `PRSERR` reader - PRS Requset Error Interrupt"]
pub type PRSERR_R = crate::BitReader<bool>;
#[doc = "Field `PRSERR` writer - PRS Requset Error Interrupt"]
pub type PRSERR_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 20>;
#[doc = "Field `BUFOUTERR` reader - BUFOUT Request Error Interrupt"]
pub type BUFOUTERR_R = crate::BitReader<bool>;
#[doc = "Field `BUFOUTERR` writer - BUFOUT Request Error Interrupt"]
pub type BUFOUTERR_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 21>;
#[doc = "Field `BUFOUTFREEZEERR` reader - BUFOUT Freeze Error Interrupt"]
pub type BUFOUTFREEZEERR_R = crate::BitReader<bool>;
#[doc = "Field `BUFOUTFREEZEERR` writer - BUFOUT Freeze Error Interrupt"]
pub type BUFOUTFREEZEERR_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 27>;
#[doc = "Field `BUFOUTDNSERR` reader - BUFOUT Did Not Start Error Interrupt"]
pub type BUFOUTDNSERR_R = crate::BitReader<bool>;
#[doc = "Field `BUFOUTDNSERR` writer - BUFOUT Did Not Start Error Interrupt"]
pub type BUFOUTDNSERR_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 28>;
#[doc = "Field `DNSERR` reader - Did Not Start Error Interrupt"]
pub type DNSERR_R = crate::BitReader<bool>;
#[doc = "Field `DNSERR` writer - Did Not Start Error Interrupt"]
pub type DNSERR_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 29>;
#[doc = "Field `LFTIMEOUTERR` reader - Low Frequency Timeout Error Interrupt"]
pub type LFTIMEOUTERR_R = crate::BitReader<bool>;
#[doc = "Field `LFTIMEOUTERR` writer - Low Frequency Timeout Error Interrupt"]
pub type LFTIMEOUTERR_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 30>;
#[doc = "Field `COREBIASOPTERR` reader - Core Bias Optimization Error Interrupt"]
pub type COREBIASOPTERR_R = crate::BitReader<bool>;
#[doc = "Field `COREBIASOPTERR` writer - Core Bias Optimization Error Interrupt"]
pub type COREBIASOPTERR_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Digital Clock Ready Interrupt"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Core Bias Optimization Ready Interrupt"]
    #[inline(always)]
    pub fn corebiasoptrdy(&self) -> COREBIASOPTRDY_R {
        COREBIASOPTRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PRS Ready Interrupt"]
    #[inline(always)]
    pub fn prsrdy(&self) -> PRSRDY_R {
        PRSRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BUFOUT Ready Interrupt"]
    #[inline(always)]
    pub fn bufoutrdy(&self) -> BUFOUTRDY_R {
        BUFOUTRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 15 - BUFOUT FROZEN Interrupt"]
    #[inline(always)]
    pub fn bufoutfrozen(&self) -> BUFOUTFROZEN_R {
        BUFOUTFROZEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - PRS Requset Error Interrupt"]
    #[inline(always)]
    pub fn prserr(&self) -> PRSERR_R {
        PRSERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - BUFOUT Request Error Interrupt"]
    #[inline(always)]
    pub fn bufouterr(&self) -> BUFOUTERR_R {
        BUFOUTERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 27 - BUFOUT Freeze Error Interrupt"]
    #[inline(always)]
    pub fn bufoutfreezeerr(&self) -> BUFOUTFREEZEERR_R {
        BUFOUTFREEZEERR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - BUFOUT Did Not Start Error Interrupt"]
    #[inline(always)]
    pub fn bufoutdnserr(&self) -> BUFOUTDNSERR_R {
        BUFOUTDNSERR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Did Not Start Error Interrupt"]
    #[inline(always)]
    pub fn dnserr(&self) -> DNSERR_R {
        DNSERR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Low Frequency Timeout Error Interrupt"]
    #[inline(always)]
    pub fn lftimeouterr(&self) -> LFTIMEOUTERR_R {
        LFTIMEOUTERR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Core Bias Optimization Error Interrupt"]
    #[inline(always)]
    pub fn corebiasopterr(&self) -> COREBIASOPTERR_R {
        COREBIASOPTERR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Digital Clock Ready Interrupt"]
    #[inline(always)]
    pub fn rdy(&mut self) -> RDY_W {
        RDY_W::new(self)
    }
    #[doc = "Bit 1 - Core Bias Optimization Ready Interrupt"]
    #[inline(always)]
    pub fn corebiasoptrdy(&mut self) -> COREBIASOPTRDY_W {
        COREBIASOPTRDY_W::new(self)
    }
    #[doc = "Bit 2 - PRS Ready Interrupt"]
    #[inline(always)]
    pub fn prsrdy(&mut self) -> PRSRDY_W {
        PRSRDY_W::new(self)
    }
    #[doc = "Bit 3 - BUFOUT Ready Interrupt"]
    #[inline(always)]
    pub fn bufoutrdy(&mut self) -> BUFOUTRDY_W {
        BUFOUTRDY_W::new(self)
    }
    #[doc = "Bit 15 - BUFOUT FROZEN Interrupt"]
    #[inline(always)]
    pub fn bufoutfrozen(&mut self) -> BUFOUTFROZEN_W {
        BUFOUTFROZEN_W::new(self)
    }
    #[doc = "Bit 20 - PRS Requset Error Interrupt"]
    #[inline(always)]
    pub fn prserr(&mut self) -> PRSERR_W {
        PRSERR_W::new(self)
    }
    #[doc = "Bit 21 - BUFOUT Request Error Interrupt"]
    #[inline(always)]
    pub fn bufouterr(&mut self) -> BUFOUTERR_W {
        BUFOUTERR_W::new(self)
    }
    #[doc = "Bit 27 - BUFOUT Freeze Error Interrupt"]
    #[inline(always)]
    pub fn bufoutfreezeerr(&mut self) -> BUFOUTFREEZEERR_W {
        BUFOUTFREEZEERR_W::new(self)
    }
    #[doc = "Bit 28 - BUFOUT Did Not Start Error Interrupt"]
    #[inline(always)]
    pub fn bufoutdnserr(&mut self) -> BUFOUTDNSERR_W {
        BUFOUTDNSERR_W::new(self)
    }
    #[doc = "Bit 29 - Did Not Start Error Interrupt"]
    #[inline(always)]
    pub fn dnserr(&mut self) -> DNSERR_W {
        DNSERR_W::new(self)
    }
    #[doc = "Bit 30 - Low Frequency Timeout Error Interrupt"]
    #[inline(always)]
    pub fn lftimeouterr(&mut self) -> LFTIMEOUTERR_W {
        LFTIMEOUTERR_W::new(self)
    }
    #[doc = "Bit 31 - Core Bias Optimization Error Interrupt"]
    #[inline(always)]
    pub fn corebiasopterr(&mut self) -> COREBIASOPTERR_W {
        COREBIASOPTERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ien::R](R) reader structure"]
impl crate::Readable for IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ien::W](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
