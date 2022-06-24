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
#[doc = "Field `EM2DBGEN` reader - Enable debugging in EM2"]
pub type EM2DBGEN_R = crate::BitReader<bool>;
#[doc = "Field `EM2DBGEN` writer - Enable debugging in EM2"]
pub type EM2DBGEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 0>;
#[doc = "Averaged Temperature samples num\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEMPAVGNUM_A {
    #[doc = "0: 16 measurements"]
    N16 = 0,
    #[doc = "1: 64 measurements"]
    N64 = 1,
}
impl From<TEMPAVGNUM_A> for bool {
    #[inline(always)]
    fn from(variant: TEMPAVGNUM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEMPAVGNUM` reader - Averaged Temperature samples num"]
pub type TEMPAVGNUM_R = crate::BitReader<TEMPAVGNUM_A>;
impl TEMPAVGNUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEMPAVGNUM_A {
        match self.bits {
            false => TEMPAVGNUM_A::N16,
            true => TEMPAVGNUM_A::N64,
        }
    }
    #[doc = "Checks if the value of the field is `N16`"]
    #[inline(always)]
    pub fn is_n16(&self) -> bool {
        *self == TEMPAVGNUM_A::N16
    }
    #[doc = "Checks if the value of the field is `N64`"]
    #[inline(always)]
    pub fn is_n64(&self) -> bool {
        *self == TEMPAVGNUM_A::N64
    }
}
#[doc = "Field `TEMPAVGNUM` writer - Averaged Temperature samples num"]
pub type TEMPAVGNUM_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, TEMPAVGNUM_A, 3>;
impl<'a> TEMPAVGNUM_W<'a> {
    #[doc = "16 measurements"]
    #[inline(always)]
    pub fn n16(self) -> &'a mut W {
        self.variant(TEMPAVGNUM_A::N16)
    }
    #[doc = "64 measurements"]
    #[inline(always)]
    pub fn n64(self) -> &'a mut W {
        self.variant(TEMPAVGNUM_A::N64)
    }
}
#[doc = "EM2/EM3 Vscale\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EM23VSCALE_A {
    #[doc = "0: VSCALE0. 0.9v"]
    VSCALE0 = 0,
    #[doc = "1: VSCALE1. 1.0v"]
    VSCALE1 = 1,
    #[doc = "2: VSCALE2. 1.1v"]
    VSCALE2 = 2,
}
impl From<EM23VSCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: EM23VSCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EM23VSCALE` reader - EM2/EM3 Vscale"]
pub type EM23VSCALE_R = crate::FieldReader<u8, EM23VSCALE_A>;
impl EM23VSCALE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EM23VSCALE_A> {
        match self.bits {
            0 => Some(EM23VSCALE_A::VSCALE0),
            1 => Some(EM23VSCALE_A::VSCALE1),
            2 => Some(EM23VSCALE_A::VSCALE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VSCALE0`"]
    #[inline(always)]
    pub fn is_vscale0(&self) -> bool {
        *self == EM23VSCALE_A::VSCALE0
    }
    #[doc = "Checks if the value of the field is `VSCALE1`"]
    #[inline(always)]
    pub fn is_vscale1(&self) -> bool {
        *self == EM23VSCALE_A::VSCALE1
    }
    #[doc = "Checks if the value of the field is `VSCALE2`"]
    #[inline(always)]
    pub fn is_vscale2(&self) -> bool {
        *self == EM23VSCALE_A::VSCALE2
    }
}
#[doc = "Field `EM23VSCALE` writer - EM2/EM3 Vscale"]
pub type EM23VSCALE_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, EM23VSCALE_A, 2, 8>;
impl<'a> EM23VSCALE_W<'a> {
    #[doc = "VSCALE0. 0.9v"]
    #[inline(always)]
    pub fn vscale0(self) -> &'a mut W {
        self.variant(EM23VSCALE_A::VSCALE0)
    }
    #[doc = "VSCALE1. 1.0v"]
    #[inline(always)]
    pub fn vscale1(self) -> &'a mut W {
        self.variant(EM23VSCALE_A::VSCALE1)
    }
    #[doc = "VSCALE2. 1.1v"]
    #[inline(always)]
    pub fn vscale2(self) -> &'a mut W {
        self.variant(EM23VSCALE_A::VSCALE2)
    }
}
#[doc = "Field `FLASHPWRUPONDEMAND` reader - Enable flash on demand wakeup"]
pub type FLASHPWRUPONDEMAND_R = crate::BitReader<bool>;
#[doc = "Field `FLASHPWRUPONDEMAND` writer - Enable flash on demand wakeup"]
pub type FLASHPWRUPONDEMAND_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 16>;
#[doc = "Field `EFPDIRECTMODEEN` reader - EFP Direct Mode Enable"]
pub type EFPDIRECTMODEEN_R = crate::BitReader<bool>;
#[doc = "Field `EFPDIRECTMODEEN` writer - EFP Direct Mode Enable"]
pub type EFPDIRECTMODEEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 29>;
#[doc = "Field `EFPDRVDECOUPLE` reader - EFP drives DECOUPLE"]
pub type EFPDRVDECOUPLE_R = crate::BitReader<bool>;
#[doc = "Field `EFPDRVDECOUPLE` writer - EFP drives DECOUPLE"]
pub type EFPDRVDECOUPLE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 30>;
#[doc = "Field `EFPDRVDVDD` reader - EFP drives DVDD"]
pub type EFPDRVDVDD_R = crate::BitReader<bool>;
#[doc = "Field `EFPDRVDVDD` writer - EFP drives DVDD"]
pub type EFPDRVDVDD_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Enable debugging in EM2"]
    #[inline(always)]
    pub fn em2dbgen(&self) -> EM2DBGEN_R {
        EM2DBGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Averaged Temperature samples num"]
    #[inline(always)]
    pub fn tempavgnum(&self) -> TEMPAVGNUM_R {
        TEMPAVGNUM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:9 - EM2/EM3 Vscale"]
    #[inline(always)]
    pub fn em23vscale(&self) -> EM23VSCALE_R {
        EM23VSCALE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Enable flash on demand wakeup"]
    #[inline(always)]
    pub fn flashpwrupondemand(&self) -> FLASHPWRUPONDEMAND_R {
        FLASHPWRUPONDEMAND_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 29 - EFP Direct Mode Enable"]
    #[inline(always)]
    pub fn efpdirectmodeen(&self) -> EFPDIRECTMODEEN_R {
        EFPDIRECTMODEEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - EFP drives DECOUPLE"]
    #[inline(always)]
    pub fn efpdrvdecouple(&self) -> EFPDRVDECOUPLE_R {
        EFPDRVDECOUPLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - EFP drives DVDD"]
    #[inline(always)]
    pub fn efpdrvdvdd(&self) -> EFPDRVDVDD_R {
        EFPDRVDVDD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable debugging in EM2"]
    #[inline(always)]
    pub fn em2dbgen(&mut self) -> EM2DBGEN_W {
        EM2DBGEN_W::new(self)
    }
    #[doc = "Bit 3 - Averaged Temperature samples num"]
    #[inline(always)]
    pub fn tempavgnum(&mut self) -> TEMPAVGNUM_W {
        TEMPAVGNUM_W::new(self)
    }
    #[doc = "Bits 8:9 - EM2/EM3 Vscale"]
    #[inline(always)]
    pub fn em23vscale(&mut self) -> EM23VSCALE_W {
        EM23VSCALE_W::new(self)
    }
    #[doc = "Bit 16 - Enable flash on demand wakeup"]
    #[inline(always)]
    pub fn flashpwrupondemand(&mut self) -> FLASHPWRUPONDEMAND_W {
        FLASHPWRUPONDEMAND_W::new(self)
    }
    #[doc = "Bit 29 - EFP Direct Mode Enable"]
    #[inline(always)]
    pub fn efpdirectmodeen(&mut self) -> EFPDIRECTMODEEN_W {
        EFPDIRECTMODEEN_W::new(self)
    }
    #[doc = "Bit 30 - EFP drives DECOUPLE"]
    #[inline(always)]
    pub fn efpdrvdecouple(&mut self) -> EFPDRVDECOUPLE_W {
        EFPDRVDECOUPLE_W::new(self)
    }
    #[doc = "Bit 31 - EFP drives DVDD"]
    #[inline(always)]
    pub fn efpdrvdvdd(&mut self) -> EFPDRVDVDD_W {
        EFPDRVDVDD_W::new(self)
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
#[doc = "`reset()` method sets CTRL to value 0x0200"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
