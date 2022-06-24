#[doc = "Register `HFXOCTRL1` reader"]
pub struct R(crate::R<HFXOCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFXOCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFXOCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFXOCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFXOCTRL1` writer"]
pub struct W(crate::W<HFXOCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFXOCTRL1_SPEC>;
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
impl From<crate::W<HFXOCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFXOCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEAKDETTHR` reader - Sets the Peak Detector amplitude detection threshold levels"]
pub type PEAKDETTHR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PEAKDETTHR` writer - Sets the Peak Detector amplitude detection threshold levels"]
pub type PEAKDETTHR_W<'a> = crate::FieldWriter<'a, u32, HFXOCTRL1_SPEC, u8, u8, 3, 0>;
#[doc = "Field `REGLVL` reader - Reserved for internal use. Do not change."]
pub type REGLVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REGLVL` writer - Reserved for internal use. Do not change."]
pub type REGLVL_W<'a> = crate::FieldWriter<'a, u32, HFXOCTRL1_SPEC, u8, u8, 3, 4>;
#[doc = "Field `XTIBIASEN` reader - Reserved for internal use. Do not change."]
pub type XTIBIASEN_R = crate::BitReader<bool>;
#[doc = "Field `XTIBIASEN` writer - Reserved for internal use. Do not change."]
pub type XTIBIASEN_W<'a> = crate::BitWriter<'a, u32, HFXOCTRL1_SPEC, bool, 9>;
impl R {
    #[doc = "Bits 0:2 - Sets the Peak Detector amplitude detection threshold levels"]
    #[inline(always)]
    pub fn peakdetthr(&self) -> PEAKDETTHR_R {
        PEAKDETTHR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn reglvl(&self) -> REGLVL_R {
        REGLVL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 9 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn xtibiasen(&self) -> XTIBIASEN_R {
        XTIBIASEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sets the Peak Detector amplitude detection threshold levels"]
    #[inline(always)]
    pub fn peakdetthr(&mut self) -> PEAKDETTHR_W {
        PEAKDETTHR_W::new(self)
    }
    #[doc = "Bits 4:6 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn reglvl(&mut self) -> REGLVL_W {
        REGLVL_W::new(self)
    }
    #[doc = "Bit 9 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn xtibiasen(&mut self) -> XTIBIASEN_W {
        XTIBIASEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HFXO Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfxoctrl1](index.html) module"]
pub struct HFXOCTRL1_SPEC;
impl crate::RegisterSpec for HFXOCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfxoctrl1::R](R) reader structure"]
impl crate::Readable for HFXOCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfxoctrl1::W](W) writer structure"]
impl crate::Writable for HFXOCTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HFXOCTRL1 to value 0x0240"]
impl crate::Resettable for HFXOCTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0240
    }
}
