#[doc = "Register `RAM2CTRL` reader"]
pub struct R(crate::R<RAM2CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM2CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM2CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM2CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAM2CTRL` writer"]
pub struct W(crate::W<RAM2CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM2CTRL_SPEC>;
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
impl From<crate::W<RAM2CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM2CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RAM2 Blockset Power-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMPOWERDOWN_A {
    #[doc = "0: None of the RAM blocks powered down"]
    NONE = 0,
    #[doc = "1: Power down RAM blocks 0-3"]
    BLK = 1,
}
impl From<RAMPOWERDOWN_A> for bool {
    #[inline(always)]
    fn from(variant: RAMPOWERDOWN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMPOWERDOWN` reader - RAM2 Blockset Power-down"]
pub type RAMPOWERDOWN_R = crate::BitReader<RAMPOWERDOWN_A>;
impl RAMPOWERDOWN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMPOWERDOWN_A {
        match self.bits {
            false => RAMPOWERDOWN_A::NONE,
            true => RAMPOWERDOWN_A::BLK,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RAMPOWERDOWN_A::NONE
    }
    #[doc = "Checks if the value of the field is `BLK`"]
    #[inline(always)]
    pub fn is_blk(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK
    }
}
#[doc = "Field `RAMPOWERDOWN` writer - RAM2 Blockset Power-down"]
pub type RAMPOWERDOWN_W<'a> = crate::BitWriter<'a, u32, RAM2CTRL_SPEC, RAMPOWERDOWN_A, 0>;
impl<'a> RAMPOWERDOWN_W<'a> {
    #[doc = "None of the RAM blocks powered down"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::NONE)
    }
    #[doc = "Power down RAM blocks 0-3"]
    #[inline(always)]
    pub fn blk(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK)
    }
}
impl R {
    #[doc = "Bit 0 - RAM2 Blockset Power-down"]
    #[inline(always)]
    pub fn rampowerdown(&self) -> RAMPOWERDOWN_R {
        RAMPOWERDOWN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM2 Blockset Power-down"]
    #[inline(always)]
    pub fn rampowerdown(&mut self) -> RAMPOWERDOWN_W {
        RAMPOWERDOWN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram2ctrl](index.html) module"]
pub struct RAM2CTRL_SPEC;
impl crate::RegisterSpec for RAM2CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram2ctrl::R](R) reader structure"]
impl crate::Readable for RAM2CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ram2ctrl::W](W) writer structure"]
impl crate::Writable for RAM2CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAM2CTRL to value 0"]
impl crate::Resettable for RAM2CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
