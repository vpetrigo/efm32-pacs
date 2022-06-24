#[doc = "Register `DEVSIZECONFIG` reader"]
pub struct R(crate::R<DEVSIZECONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVSIZECONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVSIZECONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVSIZECONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVSIZECONFIG` writer"]
pub struct W(crate::W<DEVSIZECONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVSIZECONFIG_SPEC>;
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
impl From<crate::W<DEVSIZECONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVSIZECONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NUMADDRBYTES` reader - Number of Address Bytes"]
pub type NUMADDRBYTES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUMADDRBYTES` writer - Number of Address Bytes"]
pub type NUMADDRBYTES_W<'a> = crate::FieldWriter<'a, u32, DEVSIZECONFIG_SPEC, u8, u8, 4, 0>;
#[doc = "Field `BYTESPERDEVICEPAGE` reader - Number of Bytes Per Device Page"]
pub type BYTESPERDEVICEPAGE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BYTESPERDEVICEPAGE` writer - Number of Bytes Per Device Page"]
pub type BYTESPERDEVICEPAGE_W<'a> =
    crate::FieldWriter<'a, u32, DEVSIZECONFIG_SPEC, u16, u16, 12, 4>;
#[doc = "Field `BYTESPERSUBSECTOR` reader - Number of Bytes Per Block"]
pub type BYTESPERSUBSECTOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BYTESPERSUBSECTOR` writer - Number of Bytes Per Block"]
pub type BYTESPERSUBSECTOR_W<'a> = crate::FieldWriter<'a, u32, DEVSIZECONFIG_SPEC, u8, u8, 5, 16>;
#[doc = "Field `MEMSIZEONCS0` reader - Size of Flash Device Connected to CS\\[0\\]
Pin"]
pub type MEMSIZEONCS0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEMSIZEONCS0` writer - Size of Flash Device Connected to CS\\[0\\]
Pin"]
pub type MEMSIZEONCS0_W<'a> = crate::FieldWriter<'a, u32, DEVSIZECONFIG_SPEC, u8, u8, 2, 21>;
#[doc = "Field `MEMSIZEONCS1` reader - Size of Flash Device Connected to CS\\[1\\]
Pin"]
pub type MEMSIZEONCS1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEMSIZEONCS1` writer - Size of Flash Device Connected to CS\\[1\\]
Pin"]
pub type MEMSIZEONCS1_W<'a> = crate::FieldWriter<'a, u32, DEVSIZECONFIG_SPEC, u8, u8, 2, 23>;
impl R {
    #[doc = "Bits 0:3 - Number of Address Bytes"]
    #[inline(always)]
    pub fn numaddrbytes(&self) -> NUMADDRBYTES_R {
        NUMADDRBYTES_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Number of Bytes Per Device Page"]
    #[inline(always)]
    pub fn bytesperdevicepage(&self) -> BYTESPERDEVICEPAGE_R {
        BYTESPERDEVICEPAGE_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:20 - Number of Bytes Per Block"]
    #[inline(always)]
    pub fn bytespersubsector(&self) -> BYTESPERSUBSECTOR_R {
        BYTESPERSUBSECTOR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - Size of Flash Device Connected to CS\\[0\\]
Pin"]
    #[inline(always)]
    pub fn memsizeoncs0(&self) -> MEMSIZEONCS0_R {
        MEMSIZEONCS0_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - Size of Flash Device Connected to CS\\[1\\]
Pin"]
    #[inline(always)]
    pub fn memsizeoncs1(&self) -> MEMSIZEONCS1_R {
        MEMSIZEONCS1_R::new(((self.bits >> 23) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Number of Address Bytes"]
    #[inline(always)]
    pub fn numaddrbytes(&mut self) -> NUMADDRBYTES_W {
        NUMADDRBYTES_W::new(self)
    }
    #[doc = "Bits 4:15 - Number of Bytes Per Device Page"]
    #[inline(always)]
    pub fn bytesperdevicepage(&mut self) -> BYTESPERDEVICEPAGE_W {
        BYTESPERDEVICEPAGE_W::new(self)
    }
    #[doc = "Bits 16:20 - Number of Bytes Per Block"]
    #[inline(always)]
    pub fn bytespersubsector(&mut self) -> BYTESPERSUBSECTOR_W {
        BYTESPERSUBSECTOR_W::new(self)
    }
    #[doc = "Bits 21:22 - Size of Flash Device Connected to CS\\[0\\]
Pin"]
    #[inline(always)]
    pub fn memsizeoncs0(&mut self) -> MEMSIZEONCS0_W {
        MEMSIZEONCS0_W::new(self)
    }
    #[doc = "Bits 23:24 - Size of Flash Device Connected to CS\\[1\\]
Pin"]
    #[inline(always)]
    pub fn memsizeoncs1(&mut self) -> MEMSIZEONCS1_W {
        MEMSIZEONCS1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Size Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devsizeconfig](index.html) module"]
pub struct DEVSIZECONFIG_SPEC;
impl crate::RegisterSpec for DEVSIZECONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devsizeconfig::R](R) reader structure"]
impl crate::Readable for DEVSIZECONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [devsizeconfig::W](W) writer structure"]
impl crate::Writable for DEVSIZECONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEVSIZECONFIG to value 0x0010_1002"]
impl crate::Resettable for DEVSIZECONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0010_1002
    }
}
