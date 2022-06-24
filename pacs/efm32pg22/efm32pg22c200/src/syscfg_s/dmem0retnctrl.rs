#[doc = "Register `DMEM0RETNCTRL` reader"]
pub struct R(crate::R<DMEM0RETNCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMEM0RETNCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMEM0RETNCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMEM0RETNCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMEM0RETNCTRL` writer"]
pub struct W(crate::W<DMEM0RETNCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMEM0RETNCTRL_SPEC>;
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
impl From<crate::W<DMEM0RETNCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMEM0RETNCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DMEM0 blockset retention control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RAMRETNCTRL_A {
    #[doc = "0: None of the RAM blocks powered down"]
    ALLON = 0,
    #[doc = "1: Power down RAM block 0"]
    BLK0 = 1,
    #[doc = "2: Power down RAM block 1"]
    BLK1 = 2,
}
impl From<RAMRETNCTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMRETNCTRL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RAMRETNCTRL` reader - DMEM0 blockset retention control"]
pub type RAMRETNCTRL_R = crate::FieldReader<u8, RAMRETNCTRL_A>;
impl RAMRETNCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RAMRETNCTRL_A> {
        match self.bits {
            0 => Some(RAMRETNCTRL_A::ALLON),
            1 => Some(RAMRETNCTRL_A::BLK0),
            2 => Some(RAMRETNCTRL_A::BLK1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ALLON`"]
    #[inline(always)]
    pub fn is_allon(&self) -> bool {
        *self == RAMRETNCTRL_A::ALLON
    }
    #[doc = "Checks if the value of the field is `BLK0`"]
    #[inline(always)]
    pub fn is_blk0(&self) -> bool {
        *self == RAMRETNCTRL_A::BLK0
    }
    #[doc = "Checks if the value of the field is `BLK1`"]
    #[inline(always)]
    pub fn is_blk1(&self) -> bool {
        *self == RAMRETNCTRL_A::BLK1
    }
}
#[doc = "Field `RAMRETNCTRL` writer - DMEM0 blockset retention control"]
pub type RAMRETNCTRL_W<'a> =
    crate::FieldWriter<'a, u32, DMEM0RETNCTRL_SPEC, u8, RAMRETNCTRL_A, 2, 0>;
impl<'a> RAMRETNCTRL_W<'a> {
    #[doc = "None of the RAM blocks powered down"]
    #[inline(always)]
    pub fn allon(self) -> &'a mut W {
        self.variant(RAMRETNCTRL_A::ALLON)
    }
    #[doc = "Power down RAM block 0"]
    #[inline(always)]
    pub fn blk0(self) -> &'a mut W {
        self.variant(RAMRETNCTRL_A::BLK0)
    }
    #[doc = "Power down RAM block 1"]
    #[inline(always)]
    pub fn blk1(self) -> &'a mut W {
        self.variant(RAMRETNCTRL_A::BLK1)
    }
}
impl R {
    #[doc = "Bits 0:1 - DMEM0 blockset retention control"]
    #[inline(always)]
    pub fn ramretnctrl(&self) -> RAMRETNCTRL_R {
        RAMRETNCTRL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DMEM0 blockset retention control"]
    #[inline(always)]
    pub fn ramretnctrl(&mut self) -> RAMRETNCTRL_W {
        RAMRETNCTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure to provide general RAM retention configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmem0retnctrl](index.html) module"]
pub struct DMEM0RETNCTRL_SPEC;
impl crate::RegisterSpec for DMEM0RETNCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmem0retnctrl::R](R) reader structure"]
impl crate::Readable for DMEM0RETNCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmem0retnctrl::W](W) writer structure"]
impl crate::Writable for DMEM0RETNCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMEM0RETNCTRL to value 0"]
impl crate::Resettable for DMEM0RETNCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
