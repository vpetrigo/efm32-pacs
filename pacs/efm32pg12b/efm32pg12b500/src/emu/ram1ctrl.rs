#[doc = "Register `RAM1CTRL` reader"]
pub struct R(crate::R<RAM1CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM1CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM1CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM1CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAM1CTRL` writer"]
pub struct W(crate::W<RAM1CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM1CTRL_SPEC>;
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
impl From<crate::W<RAM1CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM1CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RAM1 Blockset Power-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RAMPOWERDOWN_A {
    #[doc = "0: None of the RAM blocks powered down"]
    NONE = 0,
    #[doc = "2: Power down RAM block 1 (address range 0x20030000-0x2003FFFF)"]
    BLK1 = 2,
    #[doc = "3: Power down RAM blocks 0-1 (address range 0x20020000-0x2003FFFF)"]
    BLK0TO1 = 3,
}
impl From<RAMPOWERDOWN_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMPOWERDOWN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RAMPOWERDOWN` reader - RAM1 Blockset Power-down"]
pub type RAMPOWERDOWN_R = crate::FieldReader<u8, RAMPOWERDOWN_A>;
impl RAMPOWERDOWN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RAMPOWERDOWN_A> {
        match self.bits {
            0 => Some(RAMPOWERDOWN_A::NONE),
            2 => Some(RAMPOWERDOWN_A::BLK1),
            3 => Some(RAMPOWERDOWN_A::BLK0TO1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RAMPOWERDOWN_A::NONE
    }
    #[doc = "Checks if the value of the field is `BLK1`"]
    #[inline(always)]
    pub fn is_blk1(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK1
    }
    #[doc = "Checks if the value of the field is `BLK0TO1`"]
    #[inline(always)]
    pub fn is_blk0to1(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK0TO1
    }
}
#[doc = "Field `RAMPOWERDOWN` writer - RAM1 Blockset Power-down"]
pub type RAMPOWERDOWN_W<'a> = crate::FieldWriter<'a, u32, RAM1CTRL_SPEC, u8, RAMPOWERDOWN_A, 2, 0>;
impl<'a> RAMPOWERDOWN_W<'a> {
    #[doc = "None of the RAM blocks powered down"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::NONE)
    }
    #[doc = "Power down RAM block 1 (address range 0x20030000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk1(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK1)
    }
    #[doc = "Power down RAM blocks 0-1 (address range 0x20020000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk0to1(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK0TO1)
    }
}
impl R {
    #[doc = "Bits 0:1 - RAM1 Blockset Power-down"]
    #[inline(always)]
    pub fn rampowerdown(&self) -> RAMPOWERDOWN_R {
        RAMPOWERDOWN_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RAM1 Blockset Power-down"]
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
#[doc = "Memory Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram1ctrl](index.html) module"]
pub struct RAM1CTRL_SPEC;
impl crate::RegisterSpec for RAM1CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram1ctrl::R](R) reader structure"]
impl crate::Readable for RAM1CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ram1ctrl::W](W) writer structure"]
impl crate::Writable for RAM1CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAM1CTRL to value 0"]
impl crate::Resettable for RAM1CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
