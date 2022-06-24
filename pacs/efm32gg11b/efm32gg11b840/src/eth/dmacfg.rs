#[doc = "Register `DMACFG` reader"]
pub struct R(crate::R<DMACFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACFG` writer"]
pub struct W(crate::W<DMACFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACFG_SPEC>;
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
impl From<crate::W<DMACFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AMBABRSTLEN` reader - Selects the burst length to use on the AMBA (AHB) when transferring frame data."]
pub type AMBABRSTLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AMBABRSTLEN` writer - Selects the burst length to use on the AMBA (AHB) when transferring frame data."]
pub type AMBABRSTLEN_W<'a> = crate::FieldWriter<'a, u32, DMACFG_SPEC, u8, u8, 5, 0>;
#[doc = "Field `HDRDATASPLITEN` reader - Enable header data Splitting."]
pub type HDRDATASPLITEN_R = crate::BitReader<bool>;
#[doc = "Field `HDRDATASPLITEN` writer - Enable header data Splitting."]
pub type HDRDATASPLITEN_W<'a> = crate::BitWriter<'a, u32, DMACFG_SPEC, bool, 5>;
#[doc = "Receiver packet buffer memory size select.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXPBUFSIZE_A {
    #[doc = "0: Do not use top three address bits (0.5 Kb)"]
    SIZE0 = 0,
    #[doc = "1: Do not use top two address bits (1 Kb)"]
    SIZE1 = 1,
    #[doc = "2: Do not use top address bit (2 Kb)"]
    SIZE2 = 2,
    #[doc = "3: Use full configured addressable space (4 Kb)"]
    SIZE3 = 3,
}
impl From<RXPBUFSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: RXPBUFSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RXPBUFSIZE` reader - Receiver packet buffer memory size select."]
pub type RXPBUFSIZE_R = crate::FieldReader<u8, RXPBUFSIZE_A>;
impl RXPBUFSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPBUFSIZE_A {
        match self.bits {
            0 => RXPBUFSIZE_A::SIZE0,
            1 => RXPBUFSIZE_A::SIZE1,
            2 => RXPBUFSIZE_A::SIZE2,
            3 => RXPBUFSIZE_A::SIZE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SIZE0`"]
    #[inline(always)]
    pub fn is_size0(&self) -> bool {
        *self == RXPBUFSIZE_A::SIZE0
    }
    #[doc = "Checks if the value of the field is `SIZE1`"]
    #[inline(always)]
    pub fn is_size1(&self) -> bool {
        *self == RXPBUFSIZE_A::SIZE1
    }
    #[doc = "Checks if the value of the field is `SIZE2`"]
    #[inline(always)]
    pub fn is_size2(&self) -> bool {
        *self == RXPBUFSIZE_A::SIZE2
    }
    #[doc = "Checks if the value of the field is `SIZE3`"]
    #[inline(always)]
    pub fn is_size3(&self) -> bool {
        *self == RXPBUFSIZE_A::SIZE3
    }
}
#[doc = "Field `RXPBUFSIZE` writer - Receiver packet buffer memory size select."]
pub type RXPBUFSIZE_W<'a> = crate::FieldWriterSafe<'a, u32, DMACFG_SPEC, u8, RXPBUFSIZE_A, 2, 8>;
impl<'a> RXPBUFSIZE_W<'a> {
    #[doc = "Do not use top three address bits (0.5 Kb)"]
    #[inline(always)]
    pub fn size0(self) -> &'a mut W {
        self.variant(RXPBUFSIZE_A::SIZE0)
    }
    #[doc = "Do not use top two address bits (1 Kb)"]
    #[inline(always)]
    pub fn size1(self) -> &'a mut W {
        self.variant(RXPBUFSIZE_A::SIZE1)
    }
    #[doc = "Do not use top address bit (2 Kb)"]
    #[inline(always)]
    pub fn size2(self) -> &'a mut W {
        self.variant(RXPBUFSIZE_A::SIZE2)
    }
    #[doc = "Use full configured addressable space (4 Kb)"]
    #[inline(always)]
    pub fn size3(self) -> &'a mut W {
        self.variant(RXPBUFSIZE_A::SIZE3)
    }
}
#[doc = "Field `TXPBUFSIZE` reader - Transmitter packet buffer memory size select."]
pub type TXPBUFSIZE_R = crate::BitReader<bool>;
#[doc = "Field `TXPBUFSIZE` writer - Transmitter packet buffer memory size select."]
pub type TXPBUFSIZE_W<'a> = crate::BitWriter<'a, u32, DMACFG_SPEC, bool, 10>;
#[doc = "Field `TXPBUFTCPEN` reader - Transmitter IP, TCP and UDP checksum generation offload enable"]
pub type TXPBUFTCPEN_R = crate::BitReader<bool>;
#[doc = "Field `TXPBUFTCPEN` writer - Transmitter IP, TCP and UDP checksum generation offload enable"]
pub type TXPBUFTCPEN_W<'a> = crate::BitWriter<'a, u32, DMACFG_SPEC, bool, 11>;
#[doc = "Field `INFLASTDBUFSIZEEN` reader - Forces the DMA"]
pub type INFLASTDBUFSIZEEN_R = crate::BitReader<bool>;
#[doc = "Field `INFLASTDBUFSIZEEN` writer - Forces the DMA"]
pub type INFLASTDBUFSIZEEN_W<'a> = crate::BitWriter<'a, u32, DMACFG_SPEC, bool, 12>;
#[doc = "Field `RXBUFSIZE` reader - DMA receive buffer size in external AMBA (AHB) system memory."]
pub type RXBUFSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXBUFSIZE` writer - DMA receive buffer size in external AMBA (AHB) system memory."]
pub type RXBUFSIZE_W<'a> = crate::FieldWriter<'a, u32, DMACFG_SPEC, u8, u8, 8, 16>;
#[doc = "Field `FRCDISCARDONERR` reader - Auto Discard RX pkts during lack of resource."]
pub type FRCDISCARDONERR_R = crate::BitReader<bool>;
#[doc = "Field `FRCDISCARDONERR` writer - Auto Discard RX pkts during lack of resource."]
pub type FRCDISCARDONERR_W<'a> = crate::BitWriter<'a, u32, DMACFG_SPEC, bool, 24>;
#[doc = "Field `FRCMAXAMBABRSTRX` reader - Force max length bursts on RX."]
pub type FRCMAXAMBABRSTRX_R = crate::BitReader<bool>;
#[doc = "Field `FRCMAXAMBABRSTRX` writer - Force max length bursts on RX."]
pub type FRCMAXAMBABRSTRX_W<'a> = crate::BitWriter<'a, u32, DMACFG_SPEC, bool, 25>;
#[doc = "Field `FRCMAXAMBABRSTTX` reader - Force max length bursts on TX."]
pub type FRCMAXAMBABRSTTX_R = crate::BitReader<bool>;
#[doc = "Field `FRCMAXAMBABRSTTX` writer - Force max length bursts on TX."]
pub type FRCMAXAMBABRSTTX_W<'a> = crate::BitWriter<'a, u32, DMACFG_SPEC, bool, 26>;
#[doc = "Field `RXBDEXTNDMODEEN` reader - Enable RX extended BD mode."]
pub type RXBDEXTNDMODEEN_R = crate::BitReader<bool>;
#[doc = "Field `RXBDEXTNDMODEEN` writer - Enable RX extended BD mode."]
pub type RXBDEXTNDMODEEN_W<'a> = crate::BitWriter<'a, u32, DMACFG_SPEC, bool, 28>;
#[doc = "Field `TXBDEXTENDMODEEN` reader - Enable TX extended BD mode."]
pub type TXBDEXTENDMODEEN_R = crate::BitReader<bool>;
#[doc = "Field `TXBDEXTENDMODEEN` writer - Enable TX extended BD mode."]
pub type TXBDEXTENDMODEEN_W<'a> = crate::BitWriter<'a, u32, DMACFG_SPEC, bool, 29>;
impl R {
    #[doc = "Bits 0:4 - Selects the burst length to use on the AMBA (AHB) when transferring frame data."]
    #[inline(always)]
    pub fn ambabrstlen(&self) -> AMBABRSTLEN_R {
        AMBABRSTLEN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Enable header data Splitting."]
    #[inline(always)]
    pub fn hdrdataspliten(&self) -> HDRDATASPLITEN_R {
        HDRDATASPLITEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Receiver packet buffer memory size select."]
    #[inline(always)]
    pub fn rxpbufsize(&self) -> RXPBUFSIZE_R {
        RXPBUFSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Transmitter packet buffer memory size select."]
    #[inline(always)]
    pub fn txpbufsize(&self) -> TXPBUFSIZE_R {
        TXPBUFSIZE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmitter IP, TCP and UDP checksum generation offload enable"]
    #[inline(always)]
    pub fn txpbuftcpen(&self) -> TXPBUFTCPEN_R {
        TXPBUFTCPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Forces the DMA"]
    #[inline(always)]
    pub fn inflastdbufsizeen(&self) -> INFLASTDBUFSIZEEN_R {
        INFLASTDBUFSIZEEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:23 - DMA receive buffer size in external AMBA (AHB) system memory."]
    #[inline(always)]
    pub fn rxbufsize(&self) -> RXBUFSIZE_R {
        RXBUFSIZE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Auto Discard RX pkts during lack of resource."]
    #[inline(always)]
    pub fn frcdiscardonerr(&self) -> FRCDISCARDONERR_R {
        FRCDISCARDONERR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Force max length bursts on RX."]
    #[inline(always)]
    pub fn frcmaxambabrstrx(&self) -> FRCMAXAMBABRSTRX_R {
        FRCMAXAMBABRSTRX_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Force max length bursts on TX."]
    #[inline(always)]
    pub fn frcmaxambabrsttx(&self) -> FRCMAXAMBABRSTTX_R {
        FRCMAXAMBABRSTTX_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable RX extended BD mode."]
    #[inline(always)]
    pub fn rxbdextndmodeen(&self) -> RXBDEXTNDMODEEN_R {
        RXBDEXTNDMODEEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable TX extended BD mode."]
    #[inline(always)]
    pub fn txbdextendmodeen(&self) -> TXBDEXTENDMODEEN_R {
        TXBDEXTENDMODEEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects the burst length to use on the AMBA (AHB) when transferring frame data."]
    #[inline(always)]
    pub fn ambabrstlen(&mut self) -> AMBABRSTLEN_W {
        AMBABRSTLEN_W::new(self)
    }
    #[doc = "Bit 5 - Enable header data Splitting."]
    #[inline(always)]
    pub fn hdrdataspliten(&mut self) -> HDRDATASPLITEN_W {
        HDRDATASPLITEN_W::new(self)
    }
    #[doc = "Bits 8:9 - Receiver packet buffer memory size select."]
    #[inline(always)]
    pub fn rxpbufsize(&mut self) -> RXPBUFSIZE_W {
        RXPBUFSIZE_W::new(self)
    }
    #[doc = "Bit 10 - Transmitter packet buffer memory size select."]
    #[inline(always)]
    pub fn txpbufsize(&mut self) -> TXPBUFSIZE_W {
        TXPBUFSIZE_W::new(self)
    }
    #[doc = "Bit 11 - Transmitter IP, TCP and UDP checksum generation offload enable"]
    #[inline(always)]
    pub fn txpbuftcpen(&mut self) -> TXPBUFTCPEN_W {
        TXPBUFTCPEN_W::new(self)
    }
    #[doc = "Bit 12 - Forces the DMA"]
    #[inline(always)]
    pub fn inflastdbufsizeen(&mut self) -> INFLASTDBUFSIZEEN_W {
        INFLASTDBUFSIZEEN_W::new(self)
    }
    #[doc = "Bits 16:23 - DMA receive buffer size in external AMBA (AHB) system memory."]
    #[inline(always)]
    pub fn rxbufsize(&mut self) -> RXBUFSIZE_W {
        RXBUFSIZE_W::new(self)
    }
    #[doc = "Bit 24 - Auto Discard RX pkts during lack of resource."]
    #[inline(always)]
    pub fn frcdiscardonerr(&mut self) -> FRCDISCARDONERR_W {
        FRCDISCARDONERR_W::new(self)
    }
    #[doc = "Bit 25 - Force max length bursts on RX."]
    #[inline(always)]
    pub fn frcmaxambabrstrx(&mut self) -> FRCMAXAMBABRSTRX_W {
        FRCMAXAMBABRSTRX_W::new(self)
    }
    #[doc = "Bit 26 - Force max length bursts on TX."]
    #[inline(always)]
    pub fn frcmaxambabrsttx(&mut self) -> FRCMAXAMBABRSTTX_W {
        FRCMAXAMBABRSTTX_W::new(self)
    }
    #[doc = "Bit 28 - Enable RX extended BD mode."]
    #[inline(always)]
    pub fn rxbdextndmodeen(&mut self) -> RXBDEXTNDMODEEN_W {
        RXBDEXTNDMODEEN_W::new(self)
    }
    #[doc = "Bit 29 - Enable TX extended BD mode."]
    #[inline(always)]
    pub fn txbdextendmodeen(&mut self) -> TXBDEXTENDMODEEN_W {
        TXBDEXTENDMODEEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacfg](index.html) module"]
pub struct DMACFG_SPEC;
impl crate::RegisterSpec for DMACFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmacfg::R](R) reader structure"]
impl crate::Readable for DMACFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmacfg::W](W) writer structure"]
impl crate::Writable for DMACFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACFG to value 0x0002_0704"]
impl crate::Resettable for DMACFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0002_0704
    }
}
