#[doc = "Register `CORECLKCONTROL` reader"]
pub struct R(crate::R<CORECLKCONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORECLKCONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORECLKCONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORECLKCONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORECLKCONTROL` writer"]
pub struct W(crate::W<CORECLKCONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORECLKCONTROL_SPEC>;
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
impl From<crate::W<CORECLKCONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORECLKCONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORECLKDIS` reader - Core Clock Disable"]
pub type CORECLKDIS_R = crate::BitReader<bool>;
#[doc = "Field `CORECLKDIS` writer - Core Clock Disable"]
pub type CORECLKDIS_W<'a> = crate::BitWriter<'a, u32, CORECLKCONTROL_SPEC, bool, 0>;
#[doc = "Field `CORECLKPRESC` reader - Clock division factor of CORECLKPRESC+1"]
pub type CORECLKPRESC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORECLKPRESC` writer - Clock division factor of CORECLKPRESC+1"]
pub type CORECLKPRESC_W<'a> = crate::FieldWriter<'a, u32, CORECLKCONTROL_SPEC, u8, u8, 3, 4>;
impl R {
    #[doc = "Bit 0 - Core Clock Disable"]
    #[inline(always)]
    pub fn coreclkdis(&self) -> CORECLKDIS_R {
        CORECLKDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Clock division factor of CORECLKPRESC+1"]
    #[inline(always)]
    pub fn coreclkpresc(&self) -> CORECLKPRESC_R {
        CORECLKPRESC_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Core Clock Disable"]
    #[inline(always)]
    pub fn coreclkdis(&mut self) -> CORECLKDIS_W {
        CORECLKDIS_W::new(self)
    }
    #[doc = "Bits 4:6 - Clock division factor of CORECLKPRESC+1"]
    #[inline(always)]
    pub fn coreclkpresc(&mut self) -> CORECLKPRESC_W {
        CORECLKPRESC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [coreclkcontrol](index.html) module"]
pub struct CORECLKCONTROL_SPEC;
impl crate::RegisterSpec for CORECLKCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [coreclkcontrol::R](R) reader structure"]
impl crate::Readable for CORECLKCONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [coreclkcontrol::W](W) writer structure"]
impl crate::Writable for CORECLKCONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORECLKCONTROL to value 0"]
impl crate::Resettable for CORECLKCONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
