#[doc = "Register `MODEBITCONFIG` reader"]
pub struct R(crate::R<MODEBITCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODEBITCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODEBITCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODEBITCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODEBITCONFIG` writer"]
pub struct W(crate::W<MODEBITCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODEBITCONFIG_SPEC>;
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
impl From<crate::W<MODEBITCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODEBITCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Mode Bits"]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - Mode Bits"]
pub type MODE_W<'a> = crate::FieldWriter<'a, u32, MODEBITCONFIG_SPEC, u8, u8, 8, 0>;
#[doc = "Field `CHUNKSIZE` reader - Chunk Size"]
pub type CHUNKSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHUNKSIZE` writer - Chunk Size"]
pub type CHUNKSIZE_W<'a> = crate::FieldWriter<'a, u32, MODEBITCONFIG_SPEC, u8, u8, 3, 8>;
#[doc = "Field `CRCOUTENABLE` reader - CRC# Output Enable Bit"]
pub type CRCOUTENABLE_R = crate::BitReader<bool>;
#[doc = "Field `CRCOUTENABLE` writer - CRC# Output Enable Bit"]
pub type CRCOUTENABLE_W<'a> = crate::BitWriter<'a, u32, MODEBITCONFIG_SPEC, bool, 15>;
#[doc = "Field `RXCRCDATAUP` reader - RX CRC Data (upper)"]
pub type RXCRCDATAUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXCRCDATALOW` reader - RX CRC Data (lower)"]
pub type RXCRCDATALOW_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Mode Bits"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Chunk Size"]
    #[inline(always)]
    pub fn chunksize(&self) -> CHUNKSIZE_R {
        CHUNKSIZE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - CRC# Output Enable Bit"]
    #[inline(always)]
    pub fn crcoutenable(&self) -> CRCOUTENABLE_R {
        CRCOUTENABLE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - RX CRC Data (upper)"]
    #[inline(always)]
    pub fn rxcrcdataup(&self) -> RXCRCDATAUP_R {
        RXCRCDATAUP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - RX CRC Data (lower)"]
    #[inline(always)]
    pub fn rxcrcdatalow(&self) -> RXCRCDATALOW_R {
        RXCRCDATALOW_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Mode Bits"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W::new(self)
    }
    #[doc = "Bits 8:10 - Chunk Size"]
    #[inline(always)]
    pub fn chunksize(&mut self) -> CHUNKSIZE_W {
        CHUNKSIZE_W::new(self)
    }
    #[doc = "Bit 15 - CRC# Output Enable Bit"]
    #[inline(always)]
    pub fn crcoutenable(&mut self) -> CRCOUTENABLE_W {
        CRCOUTENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Bit Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modebitconfig](index.html) module"]
pub struct MODEBITCONFIG_SPEC;
impl crate::RegisterSpec for MODEBITCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modebitconfig::R](R) reader structure"]
impl crate::Readable for MODEBITCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [modebitconfig::W](W) writer structure"]
impl crate::Writable for MODEBITCONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODEBITCONFIG to value 0x0200"]
impl crate::Resettable for MODEBITCONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
