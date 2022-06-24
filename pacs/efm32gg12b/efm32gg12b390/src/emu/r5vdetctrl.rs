#[doc = "Register `R5VDETCTRL` reader"]
pub struct R(crate::R<R5VDETCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R5VDETCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R5VDETCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R5VDETCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R5VDETCTRL` writer"]
pub struct W(crate::W<R5VDETCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R5VDETCTRL_SPEC>;
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
impl From<crate::W<R5VDETCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R5VDETCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VREGIDETDIS` reader - VREGI Detector Disable"]
pub type VREGIDETDIS_R = crate::BitReader<bool>;
#[doc = "Field `VREGIDETDIS` writer - VREGI Detector Disable"]
pub type VREGIDETDIS_W<'a> = crate::BitWriter<'a, u32, R5VDETCTRL_SPEC, bool, 0>;
#[doc = "Field `VBUSDETDIS` reader - VBUS Detector Disable"]
pub type VBUSDETDIS_R = crate::BitReader<bool>;
#[doc = "Field `VBUSDETDIS` writer - VBUS Detector Disable"]
pub type VBUSDETDIS_W<'a> = crate::BitWriter<'a, u32, R5VDETCTRL_SPEC, bool, 1>;
#[doc = "Field `VREGODETDIS` reader - VREGO Detector Disable"]
pub type VREGODETDIS_R = crate::BitReader<bool>;
#[doc = "Field `VREGODETDIS` writer - VREGO Detector Disable"]
pub type VREGODETDIS_W<'a> = crate::BitWriter<'a, u32, R5VDETCTRL_SPEC, bool, 2>;
impl R {
    #[doc = "Bit 0 - VREGI Detector Disable"]
    #[inline(always)]
    pub fn vregidetdis(&self) -> VREGIDETDIS_R {
        VREGIDETDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBUS Detector Disable"]
    #[inline(always)]
    pub fn vbusdetdis(&self) -> VBUSDETDIS_R {
        VBUSDETDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VREGO Detector Disable"]
    #[inline(always)]
    pub fn vregodetdis(&self) -> VREGODETDIS_R {
        VREGODETDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VREGI Detector Disable"]
    #[inline(always)]
    pub fn vregidetdis(&mut self) -> VREGIDETDIS_W {
        VREGIDETDIS_W::new(self)
    }
    #[doc = "Bit 1 - VBUS Detector Disable"]
    #[inline(always)]
    pub fn vbusdetdis(&mut self) -> VBUSDETDIS_W {
        VBUSDETDIS_W::new(self)
    }
    #[doc = "Bit 2 - VREGO Detector Disable"]
    #[inline(always)]
    pub fn vregodetdis(&mut self) -> VREGODETDIS_W {
        VREGODETDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "5V Detector Enables\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r5vdetctrl](index.html) module"]
pub struct R5VDETCTRL_SPEC;
impl crate::RegisterSpec for R5VDETCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r5vdetctrl::R](R) reader structure"]
impl crate::Readable for R5VDETCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r5vdetctrl::W](W) writer structure"]
impl crate::Writable for R5VDETCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R5VDETCTRL to value 0"]
impl crate::Resettable for R5VDETCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
