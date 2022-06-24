#[doc = "Register `OPCODEEXTUPPER` reader"]
pub struct R(crate::R<OPCODEEXTUPPER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPCODEEXTUPPER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPCODEEXTUPPER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPCODEEXTUPPER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPCODEEXTUPPER` writer"]
pub struct W(crate::W<OPCODEEXTUPPER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPCODEEXTUPPER_SPEC>;
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
impl From<crate::W<OPCODEEXTUPPER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPCODEEXTUPPER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTWELOPCODE` reader - WEL Opcode Extension"]
pub type EXTWELOPCODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTWELOPCODE` writer - WEL Opcode Extension"]
pub type EXTWELOPCODE_W<'a> = crate::FieldWriter<'a, u32, OPCODEEXTUPPER_SPEC, u8, u8, 8, 16>;
#[doc = "Field `WELOPCODE` reader - WEL Opcode"]
pub type WELOPCODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WELOPCODE` writer - WEL Opcode"]
pub type WELOPCODE_W<'a> = crate::FieldWriter<'a, u32, OPCODEEXTUPPER_SPEC, u8, u8, 8, 24>;
impl R {
    #[doc = "Bits 16:23 - WEL Opcode Extension"]
    #[inline(always)]
    pub fn extwelopcode(&self) -> EXTWELOPCODE_R {
        EXTWELOPCODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - WEL Opcode"]
    #[inline(always)]
    pub fn welopcode(&self) -> WELOPCODE_R {
        WELOPCODE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - WEL Opcode Extension"]
    #[inline(always)]
    pub fn extwelopcode(&mut self) -> EXTWELOPCODE_W {
        EXTWELOPCODE_W::new(self)
    }
    #[doc = "Bits 24:31 - WEL Opcode"]
    #[inline(always)]
    pub fn welopcode(&mut self) -> WELOPCODE_W {
        WELOPCODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Opcode Extension Register (Upper)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opcodeextupper](index.html) module"]
pub struct OPCODEEXTUPPER_SPEC;
impl crate::RegisterSpec for OPCODEEXTUPPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opcodeextupper::R](R) reader structure"]
impl crate::Readable for OPCODEEXTUPPER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opcodeextupper::W](W) writer structure"]
impl crate::Writable for OPCODEEXTUPPER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPCODEEXTUPPER to value 0x06f9_0000"]
impl crate::Resettable for OPCODEEXTUPPER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06f9_0000
    }
}
