#[doc = "Register `TXPFCPAUSE` reader"]
pub struct R(crate::R<TXPFCPAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXPFCPAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXPFCPAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXPFCPAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXPFCPAUSE` writer"]
pub struct W(crate::W<TXPFCPAUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXPFCPAUSE_SPEC>;
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
impl From<crate::W<TXPFCPAUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXPFCPAUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VECTORENB` reader - Priority Vector Enable. If bit 17 of the network control register is written with a one then the priority enable vector of the PFC priority based pause frame will be set equal to the value stored in this register \\[7:0\\]."]
pub type VECTORENB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VECTORENB` writer - Priority Vector Enable. If bit 17 of the network control register is written with a one then the priority enable vector of the PFC priority based pause frame will be set equal to the value stored in this register \\[7:0\\]."]
pub type VECTORENB_W<'a> = crate::FieldWriter<'a, u32, TXPFCPAUSE_SPEC, u8, u8, 8, 0>;
#[doc = "Field `VECTOR` reader - Priority Vector Pause Size. If bit 17 of the network control register is written with a one then for each entry equal to zero in the Transmit PFC Pause Register\\[15:8\\], the PFC pause frame's pause quantum field associated with that entry will be taken from the transmit pause quantum register. For each entry equal to one in the Transmit PFC Pause Register \\[15:8\\], the pause quantum associated with that entry will be zero."]
pub type VECTOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VECTOR` writer - Priority Vector Pause Size. If bit 17 of the network control register is written with a one then for each entry equal to zero in the Transmit PFC Pause Register\\[15:8\\], the PFC pause frame's pause quantum field associated with that entry will be taken from the transmit pause quantum register. For each entry equal to one in the Transmit PFC Pause Register \\[15:8\\], the pause quantum associated with that entry will be zero."]
pub type VECTOR_W<'a> = crate::FieldWriter<'a, u32, TXPFCPAUSE_SPEC, u8, u8, 8, 8>;
impl R {
    #[doc = "Bits 0:7 - Priority Vector Enable. If bit 17 of the network control register is written with a one then the priority enable vector of the PFC priority based pause frame will be set equal to the value stored in this register \\[7:0\\]."]
    #[inline(always)]
    pub fn vectorenb(&self) -> VECTORENB_R {
        VECTORENB_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Priority Vector Pause Size. If bit 17 of the network control register is written with a one then for each entry equal to zero in the Transmit PFC Pause Register\\[15:8\\], the PFC pause frame's pause quantum field associated with that entry will be taken from the transmit pause quantum register. For each entry equal to one in the Transmit PFC Pause Register \\[15:8\\], the pause quantum associated with that entry will be zero."]
    #[inline(always)]
    pub fn vector(&self) -> VECTOR_R {
        VECTOR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority Vector Enable. If bit 17 of the network control register is written with a one then the priority enable vector of the PFC priority based pause frame will be set equal to the value stored in this register \\[7:0\\]."]
    #[inline(always)]
    pub fn vectorenb(&mut self) -> VECTORENB_W {
        VECTORENB_W::new(self)
    }
    #[doc = "Bits 8:15 - Priority Vector Pause Size. If bit 17 of the network control register is written with a one then for each entry equal to zero in the Transmit PFC Pause Register\\[15:8\\], the PFC pause frame's pause quantum field associated with that entry will be taken from the transmit pause quantum register. For each entry equal to one in the Transmit PFC Pause Register \\[15:8\\], the pause quantum associated with that entry will be zero."]
    #[inline(always)]
    pub fn vector(&mut self) -> VECTOR_W {
        VECTOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit PFC Pause Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txpfcpause](index.html) module"]
pub struct TXPFCPAUSE_SPEC;
impl crate::RegisterSpec for TXPFCPAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txpfcpause::R](R) reader structure"]
impl crate::Readable for TXPFCPAUSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txpfcpause::W](W) writer structure"]
impl crate::Writable for TXPFCPAUSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXPFCPAUSE to value 0"]
impl crate::Resettable for TXPFCPAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
