#[doc = "Register `EXTILEVEL` reader"]
pub struct R(crate::R<EXTILEVEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTILEVEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTILEVEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTILEVEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTILEVEL` writer"]
pub struct W(crate::W<EXTILEVEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTILEVEL_SPEC>;
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
impl From<crate::W<EXTILEVEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTILEVEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EM4WU0` reader - EM4 Wake Up Level for EM4WU0 Pin"]
pub type EM4WU0_R = crate::BitReader<bool>;
#[doc = "Field `EM4WU0` writer - EM4 Wake Up Level for EM4WU0 Pin"]
pub type EM4WU0_W<'a> = crate::BitWriter<'a, u32, EXTILEVEL_SPEC, bool, 16>;
#[doc = "Field `EM4WU1` reader - EM4 Wake Up Level for EM4WU1 Pin"]
pub type EM4WU1_R = crate::BitReader<bool>;
#[doc = "Field `EM4WU1` writer - EM4 Wake Up Level for EM4WU1 Pin"]
pub type EM4WU1_W<'a> = crate::BitWriter<'a, u32, EXTILEVEL_SPEC, bool, 17>;
#[doc = "Field `EM4WU4` reader - EM4 Wake Up Level for EM4WU4 Pin"]
pub type EM4WU4_R = crate::BitReader<bool>;
#[doc = "Field `EM4WU4` writer - EM4 Wake Up Level for EM4WU4 Pin"]
pub type EM4WU4_W<'a> = crate::BitWriter<'a, u32, EXTILEVEL_SPEC, bool, 20>;
#[doc = "Field `EM4WU8` reader - EM4 Wake Up Level for EM4WU8 Pin"]
pub type EM4WU8_R = crate::BitReader<bool>;
#[doc = "Field `EM4WU8` writer - EM4 Wake Up Level for EM4WU8 Pin"]
pub type EM4WU8_W<'a> = crate::BitWriter<'a, u32, EXTILEVEL_SPEC, bool, 24>;
#[doc = "Field `EM4WU9` reader - EM4 Wake Up Level for EM4WU9 Pin"]
pub type EM4WU9_R = crate::BitReader<bool>;
#[doc = "Field `EM4WU9` writer - EM4 Wake Up Level for EM4WU9 Pin"]
pub type EM4WU9_W<'a> = crate::BitWriter<'a, u32, EXTILEVEL_SPEC, bool, 25>;
#[doc = "Field `EM4WU12` reader - EM4 Wake Up Level for EM4WU12 Pin"]
pub type EM4WU12_R = crate::BitReader<bool>;
#[doc = "Field `EM4WU12` writer - EM4 Wake Up Level for EM4WU12 Pin"]
pub type EM4WU12_W<'a> = crate::BitWriter<'a, u32, EXTILEVEL_SPEC, bool, 28>;
impl R {
    #[doc = "Bit 16 - EM4 Wake Up Level for EM4WU0 Pin"]
    #[inline(always)]
    pub fn em4wu0(&self) -> EM4WU0_R {
        EM4WU0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - EM4 Wake Up Level for EM4WU1 Pin"]
    #[inline(always)]
    pub fn em4wu1(&self) -> EM4WU1_R {
        EM4WU1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - EM4 Wake Up Level for EM4WU4 Pin"]
    #[inline(always)]
    pub fn em4wu4(&self) -> EM4WU4_R {
        EM4WU4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - EM4 Wake Up Level for EM4WU8 Pin"]
    #[inline(always)]
    pub fn em4wu8(&self) -> EM4WU8_R {
        EM4WU8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - EM4 Wake Up Level for EM4WU9 Pin"]
    #[inline(always)]
    pub fn em4wu9(&self) -> EM4WU9_R {
        EM4WU9_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - EM4 Wake Up Level for EM4WU12 Pin"]
    #[inline(always)]
    pub fn em4wu12(&self) -> EM4WU12_R {
        EM4WU12_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - EM4 Wake Up Level for EM4WU0 Pin"]
    #[inline(always)]
    pub fn em4wu0(&mut self) -> EM4WU0_W {
        EM4WU0_W::new(self)
    }
    #[doc = "Bit 17 - EM4 Wake Up Level for EM4WU1 Pin"]
    #[inline(always)]
    pub fn em4wu1(&mut self) -> EM4WU1_W {
        EM4WU1_W::new(self)
    }
    #[doc = "Bit 20 - EM4 Wake Up Level for EM4WU4 Pin"]
    #[inline(always)]
    pub fn em4wu4(&mut self) -> EM4WU4_W {
        EM4WU4_W::new(self)
    }
    #[doc = "Bit 24 - EM4 Wake Up Level for EM4WU8 Pin"]
    #[inline(always)]
    pub fn em4wu8(&mut self) -> EM4WU8_W {
        EM4WU8_W::new(self)
    }
    #[doc = "Bit 25 - EM4 Wake Up Level for EM4WU9 Pin"]
    #[inline(always)]
    pub fn em4wu9(&mut self) -> EM4WU9_W {
        EM4WU9_W::new(self)
    }
    #[doc = "Bit 28 - EM4 Wake Up Level for EM4WU12 Pin"]
    #[inline(always)]
    pub fn em4wu12(&mut self) -> EM4WU12_W {
        EM4WU12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Interrupt Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extilevel](index.html) module"]
pub struct EXTILEVEL_SPEC;
impl crate::RegisterSpec for EXTILEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extilevel::R](R) reader structure"]
impl crate::Readable for EXTILEVEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extilevel::W](W) writer structure"]
impl crate::Writable for EXTILEVEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTILEVEL to value 0"]
impl crate::Resettable for EXTILEVEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
