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
    #[doc = "128: Power down RAM block 7 (address range 0x2003C000-0x2003FFFF)"]
    BLK7 = 128,
    #[doc = "192: Power down RAM blocks 6-7 (address range 0x20038000-0x2003FFFF)"]
    BLK6TO7 = 192,
    #[doc = "224: Power down RAM blocks 5-7 (address range 0x20034000-0x2003FFFF)"]
    BLK5TO7 = 224,
    #[doc = "240: Power down RAM blocks 4-7 (address range 0x20030000-0x2003FFFF)"]
    BLK4TO7 = 240,
    #[doc = "248: Power down RAM blocks 3-7 (address range 0x2002C000-0x2003FFFF)"]
    BLK3TO7 = 248,
    #[doc = "252: Power down RAM blocks 2-7 (address range 0x20028000-0x2003FFFF)"]
    BLK2TO7 = 252,
    #[doc = "254: Power down RAM blocks 1-7 (address range 0x20024000-0x2003FFFF)"]
    BLK1TO7 = 254,
    #[doc = "255: Power down RAM blocks 0-7 (address range 0x20020000-0x2003FFFF)"]
    BLK0TO7 = 255,
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
            128 => Some(RAMPOWERDOWN_A::BLK7),
            192 => Some(RAMPOWERDOWN_A::BLK6TO7),
            224 => Some(RAMPOWERDOWN_A::BLK5TO7),
            240 => Some(RAMPOWERDOWN_A::BLK4TO7),
            248 => Some(RAMPOWERDOWN_A::BLK3TO7),
            252 => Some(RAMPOWERDOWN_A::BLK2TO7),
            254 => Some(RAMPOWERDOWN_A::BLK1TO7),
            255 => Some(RAMPOWERDOWN_A::BLK0TO7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RAMPOWERDOWN_A::NONE
    }
    #[doc = "Checks if the value of the field is `BLK7`"]
    #[inline(always)]
    pub fn is_blk7(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK7
    }
    #[doc = "Checks if the value of the field is `BLK6TO7`"]
    #[inline(always)]
    pub fn is_blk6to7(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK6TO7
    }
    #[doc = "Checks if the value of the field is `BLK5TO7`"]
    #[inline(always)]
    pub fn is_blk5to7(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK5TO7
    }
    #[doc = "Checks if the value of the field is `BLK4TO7`"]
    #[inline(always)]
    pub fn is_blk4to7(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK4TO7
    }
    #[doc = "Checks if the value of the field is `BLK3TO7`"]
    #[inline(always)]
    pub fn is_blk3to7(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK3TO7
    }
    #[doc = "Checks if the value of the field is `BLK2TO7`"]
    #[inline(always)]
    pub fn is_blk2to7(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK2TO7
    }
    #[doc = "Checks if the value of the field is `BLK1TO7`"]
    #[inline(always)]
    pub fn is_blk1to7(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK1TO7
    }
    #[doc = "Checks if the value of the field is `BLK0TO7`"]
    #[inline(always)]
    pub fn is_blk0to7(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK0TO7
    }
}
#[doc = "Field `RAMPOWERDOWN` writer - RAM1 Blockset Power-down"]
pub type RAMPOWERDOWN_W<'a> = crate::FieldWriter<'a, u32, RAM1CTRL_SPEC, u8, RAMPOWERDOWN_A, 8, 0>;
impl<'a> RAMPOWERDOWN_W<'a> {
    #[doc = "None of the RAM blocks powered down"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::NONE)
    }
    #[doc = "Power down RAM block 7 (address range 0x2003C000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK7)
    }
    #[doc = "Power down RAM blocks 6-7 (address range 0x20038000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk6to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK6TO7)
    }
    #[doc = "Power down RAM blocks 5-7 (address range 0x20034000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk5to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK5TO7)
    }
    #[doc = "Power down RAM blocks 4-7 (address range 0x20030000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk4to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK4TO7)
    }
    #[doc = "Power down RAM blocks 3-7 (address range 0x2002C000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk3to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK3TO7)
    }
    #[doc = "Power down RAM blocks 2-7 (address range 0x20028000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk2to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK2TO7)
    }
    #[doc = "Power down RAM blocks 1-7 (address range 0x20024000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk1to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK1TO7)
    }
    #[doc = "Power down RAM blocks 0-7 (address range 0x20020000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk0to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK0TO7)
    }
}
impl R {
    #[doc = "Bits 0:7 - RAM1 Blockset Power-down"]
    #[inline(always)]
    pub fn rampowerdown(&self) -> RAMPOWERDOWN_R {
        RAMPOWERDOWN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RAM1 Blockset Power-down"]
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
