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
#[doc = "Field `VBUSDETH` reader - VBUSDETH Interrupt Enable"]
pub type VBUSDETH_R = crate::BitReader<bool>;
#[doc = "Field `VBUSDETH` writer - VBUSDETH Interrupt Enable"]
pub type VBUSDETH_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 0>;
#[doc = "Field `VBUSDETL` reader - VBUSDETL Interrupt Enable"]
pub type VBUSDETL_R = crate::BitReader<bool>;
#[doc = "Field `VBUSDETL` writer - VBUSDETL Interrupt Enable"]
pub type VBUSDETL_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 1>;
#[doc = "Field `ERR` reader - ERR Interrupt Enable"]
pub type ERR_R = crate::BitReader<bool>;
#[doc = "Field `ERR` writer - ERR Interrupt Enable"]
pub type ERR_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 8>;
#[doc = "Field `DCD` reader - DCD Interrupt Enable"]
pub type DCD_R = crate::BitReader<bool>;
#[doc = "Field `DCD` writer - DCD Interrupt Enable"]
pub type DCD_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 9>;
#[doc = "Field `PD` reader - PD Interrupt Enable"]
pub type PD_R = crate::BitReader<bool>;
#[doc = "Field `PD` writer - PD Interrupt Enable"]
pub type PD_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 10>;
#[doc = "Field `SD` reader - SD Interrupt Enable"]
pub type SD_R = crate::BitReader<bool>;
#[doc = "Field `SD` writer - SD Interrupt Enable"]
pub type SD_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 11>;
impl R {
    #[doc = "Bit 0 - VBUSDETH Interrupt Enable"]
    #[inline(always)]
    pub fn vbusdeth(&self) -> VBUSDETH_R {
        VBUSDETH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBUSDETL Interrupt Enable"]
    #[inline(always)]
    pub fn vbusdetl(&self) -> VBUSDETL_R {
        VBUSDETL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - ERR Interrupt Enable"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DCD Interrupt Enable"]
    #[inline(always)]
    pub fn dcd(&self) -> DCD_R {
        DCD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PD Interrupt Enable"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SD Interrupt Enable"]
    #[inline(always)]
    pub fn sd(&self) -> SD_R {
        SD_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBUSDETH Interrupt Enable"]
    #[inline(always)]
    pub fn vbusdeth(&mut self) -> VBUSDETH_W {
        VBUSDETH_W::new(self)
    }
    #[doc = "Bit 1 - VBUSDETL Interrupt Enable"]
    #[inline(always)]
    pub fn vbusdetl(&mut self) -> VBUSDETL_W {
        VBUSDETL_W::new(self)
    }
    #[doc = "Bit 8 - ERR Interrupt Enable"]
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W {
        ERR_W::new(self)
    }
    #[doc = "Bit 9 - DCD Interrupt Enable"]
    #[inline(always)]
    pub fn dcd(&mut self) -> DCD_W {
        DCD_W::new(self)
    }
    #[doc = "Bit 10 - PD Interrupt Enable"]
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W {
        PD_W::new(self)
    }
    #[doc = "Bit 11 - SD Interrupt Enable"]
    #[inline(always)]
    pub fn sd(&mut self) -> SD_W {
        SD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
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
