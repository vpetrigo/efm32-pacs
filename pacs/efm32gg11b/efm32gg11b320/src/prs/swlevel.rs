#[doc = "Register `SWLEVEL` reader"]
pub struct R(crate::R<SWLEVEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWLEVEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWLEVEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWLEVEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWLEVEL` writer"]
pub struct W(crate::W<SWLEVEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWLEVEL_SPEC>;
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
impl From<crate::W<SWLEVEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWLEVEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0LEVEL` reader - Channel 0 Software Level"]
pub type CH0LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH0LEVEL` writer - Channel 0 Software Level"]
pub type CH0LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 0>;
#[doc = "Field `CH1LEVEL` reader - Channel 1 Software Level"]
pub type CH1LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH1LEVEL` writer - Channel 1 Software Level"]
pub type CH1LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 1>;
#[doc = "Field `CH2LEVEL` reader - Channel 2 Software Level"]
pub type CH2LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH2LEVEL` writer - Channel 2 Software Level"]
pub type CH2LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 2>;
#[doc = "Field `CH3LEVEL` reader - Channel 3 Software Level"]
pub type CH3LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH3LEVEL` writer - Channel 3 Software Level"]
pub type CH3LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 3>;
#[doc = "Field `CH4LEVEL` reader - Channel 4 Software Level"]
pub type CH4LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH4LEVEL` writer - Channel 4 Software Level"]
pub type CH4LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 4>;
#[doc = "Field `CH5LEVEL` reader - Channel 5 Software Level"]
pub type CH5LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH5LEVEL` writer - Channel 5 Software Level"]
pub type CH5LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 5>;
#[doc = "Field `CH6LEVEL` reader - Channel 6 Software Level"]
pub type CH6LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH6LEVEL` writer - Channel 6 Software Level"]
pub type CH6LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 6>;
#[doc = "Field `CH7LEVEL` reader - Channel 7 Software Level"]
pub type CH7LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH7LEVEL` writer - Channel 7 Software Level"]
pub type CH7LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 7>;
#[doc = "Field `CH8LEVEL` reader - Channel 8 Software Level"]
pub type CH8LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH8LEVEL` writer - Channel 8 Software Level"]
pub type CH8LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 8>;
#[doc = "Field `CH9LEVEL` reader - Channel 9 Software Level"]
pub type CH9LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH9LEVEL` writer - Channel 9 Software Level"]
pub type CH9LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 9>;
#[doc = "Field `CH10LEVEL` reader - Channel 10 Software Level"]
pub type CH10LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH10LEVEL` writer - Channel 10 Software Level"]
pub type CH10LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 10>;
#[doc = "Field `CH11LEVEL` reader - Channel 11 Software Level"]
pub type CH11LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH11LEVEL` writer - Channel 11 Software Level"]
pub type CH11LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 11>;
#[doc = "Field `CH12LEVEL` reader - Channel 12 Software Level"]
pub type CH12LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH12LEVEL` writer - Channel 12 Software Level"]
pub type CH12LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 12>;
#[doc = "Field `CH13LEVEL` reader - Channel 13 Software Level"]
pub type CH13LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH13LEVEL` writer - Channel 13 Software Level"]
pub type CH13LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 13>;
#[doc = "Field `CH14LEVEL` reader - Channel 14 Software Level"]
pub type CH14LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH14LEVEL` writer - Channel 14 Software Level"]
pub type CH14LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 14>;
#[doc = "Field `CH15LEVEL` reader - Channel 15 Software Level"]
pub type CH15LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH15LEVEL` writer - Channel 15 Software Level"]
pub type CH15LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 15>;
#[doc = "Field `CH16LEVEL` reader - Channel 16 Software Level"]
pub type CH16LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH16LEVEL` writer - Channel 16 Software Level"]
pub type CH16LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 16>;
#[doc = "Field `CH17LEVEL` reader - Channel 17 Software Level"]
pub type CH17LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH17LEVEL` writer - Channel 17 Software Level"]
pub type CH17LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 17>;
#[doc = "Field `CH18LEVEL` reader - Channel 18 Software Level"]
pub type CH18LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH18LEVEL` writer - Channel 18 Software Level"]
pub type CH18LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 18>;
#[doc = "Field `CH19LEVEL` reader - Channel 19 Software Level"]
pub type CH19LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH19LEVEL` writer - Channel 19 Software Level"]
pub type CH19LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 19>;
#[doc = "Field `CH20LEVEL` reader - Channel 20 Software Level"]
pub type CH20LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH20LEVEL` writer - Channel 20 Software Level"]
pub type CH20LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 20>;
#[doc = "Field `CH21LEVEL` reader - Channel 21 Software Level"]
pub type CH21LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH21LEVEL` writer - Channel 21 Software Level"]
pub type CH21LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 21>;
#[doc = "Field `CH22LEVEL` reader - Channel 22 Software Level"]
pub type CH22LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH22LEVEL` writer - Channel 22 Software Level"]
pub type CH22LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 22>;
#[doc = "Field `CH23LEVEL` reader - Channel 23 Software Level"]
pub type CH23LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH23LEVEL` writer - Channel 23 Software Level"]
pub type CH23LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 23>;
impl R {
    #[doc = "Bit 0 - Channel 0 Software Level"]
    #[inline(always)]
    pub fn ch0level(&self) -> CH0LEVEL_R {
        CH0LEVEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Software Level"]
    #[inline(always)]
    pub fn ch1level(&self) -> CH1LEVEL_R {
        CH1LEVEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Software Level"]
    #[inline(always)]
    pub fn ch2level(&self) -> CH2LEVEL_R {
        CH2LEVEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Software Level"]
    #[inline(always)]
    pub fn ch3level(&self) -> CH3LEVEL_R {
        CH3LEVEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Software Level"]
    #[inline(always)]
    pub fn ch4level(&self) -> CH4LEVEL_R {
        CH4LEVEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Software Level"]
    #[inline(always)]
    pub fn ch5level(&self) -> CH5LEVEL_R {
        CH5LEVEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Software Level"]
    #[inline(always)]
    pub fn ch6level(&self) -> CH6LEVEL_R {
        CH6LEVEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Software Level"]
    #[inline(always)]
    pub fn ch7level(&self) -> CH7LEVEL_R {
        CH7LEVEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 8 Software Level"]
    #[inline(always)]
    pub fn ch8level(&self) -> CH8LEVEL_R {
        CH8LEVEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 9 Software Level"]
    #[inline(always)]
    pub fn ch9level(&self) -> CH9LEVEL_R {
        CH9LEVEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 10 Software Level"]
    #[inline(always)]
    pub fn ch10level(&self) -> CH10LEVEL_R {
        CH10LEVEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 11 Software Level"]
    #[inline(always)]
    pub fn ch11level(&self) -> CH11LEVEL_R {
        CH11LEVEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 12 Software Level"]
    #[inline(always)]
    pub fn ch12level(&self) -> CH12LEVEL_R {
        CH12LEVEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 13 Software Level"]
    #[inline(always)]
    pub fn ch13level(&self) -> CH13LEVEL_R {
        CH13LEVEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 14 Software Level"]
    #[inline(always)]
    pub fn ch14level(&self) -> CH14LEVEL_R {
        CH14LEVEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 15 Software Level"]
    #[inline(always)]
    pub fn ch15level(&self) -> CH15LEVEL_R {
        CH15LEVEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 16 Software Level"]
    #[inline(always)]
    pub fn ch16level(&self) -> CH16LEVEL_R {
        CH16LEVEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 17 Software Level"]
    #[inline(always)]
    pub fn ch17level(&self) -> CH17LEVEL_R {
        CH17LEVEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 18 Software Level"]
    #[inline(always)]
    pub fn ch18level(&self) -> CH18LEVEL_R {
        CH18LEVEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel 19 Software Level"]
    #[inline(always)]
    pub fn ch19level(&self) -> CH19LEVEL_R {
        CH19LEVEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel 20 Software Level"]
    #[inline(always)]
    pub fn ch20level(&self) -> CH20LEVEL_R {
        CH20LEVEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Channel 21 Software Level"]
    #[inline(always)]
    pub fn ch21level(&self) -> CH21LEVEL_R {
        CH21LEVEL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Channel 22 Software Level"]
    #[inline(always)]
    pub fn ch22level(&self) -> CH22LEVEL_R {
        CH22LEVEL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel 23 Software Level"]
    #[inline(always)]
    pub fn ch23level(&self) -> CH23LEVEL_R {
        CH23LEVEL_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Software Level"]
    #[inline(always)]
    pub fn ch0level(&mut self) -> CH0LEVEL_W {
        CH0LEVEL_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Software Level"]
    #[inline(always)]
    pub fn ch1level(&mut self) -> CH1LEVEL_W {
        CH1LEVEL_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Software Level"]
    #[inline(always)]
    pub fn ch2level(&mut self) -> CH2LEVEL_W {
        CH2LEVEL_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Software Level"]
    #[inline(always)]
    pub fn ch3level(&mut self) -> CH3LEVEL_W {
        CH3LEVEL_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Software Level"]
    #[inline(always)]
    pub fn ch4level(&mut self) -> CH4LEVEL_W {
        CH4LEVEL_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Software Level"]
    #[inline(always)]
    pub fn ch5level(&mut self) -> CH5LEVEL_W {
        CH5LEVEL_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Software Level"]
    #[inline(always)]
    pub fn ch6level(&mut self) -> CH6LEVEL_W {
        CH6LEVEL_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Software Level"]
    #[inline(always)]
    pub fn ch7level(&mut self) -> CH7LEVEL_W {
        CH7LEVEL_W::new(self)
    }
    #[doc = "Bit 8 - Channel 8 Software Level"]
    #[inline(always)]
    pub fn ch8level(&mut self) -> CH8LEVEL_W {
        CH8LEVEL_W::new(self)
    }
    #[doc = "Bit 9 - Channel 9 Software Level"]
    #[inline(always)]
    pub fn ch9level(&mut self) -> CH9LEVEL_W {
        CH9LEVEL_W::new(self)
    }
    #[doc = "Bit 10 - Channel 10 Software Level"]
    #[inline(always)]
    pub fn ch10level(&mut self) -> CH10LEVEL_W {
        CH10LEVEL_W::new(self)
    }
    #[doc = "Bit 11 - Channel 11 Software Level"]
    #[inline(always)]
    pub fn ch11level(&mut self) -> CH11LEVEL_W {
        CH11LEVEL_W::new(self)
    }
    #[doc = "Bit 12 - Channel 12 Software Level"]
    #[inline(always)]
    pub fn ch12level(&mut self) -> CH12LEVEL_W {
        CH12LEVEL_W::new(self)
    }
    #[doc = "Bit 13 - Channel 13 Software Level"]
    #[inline(always)]
    pub fn ch13level(&mut self) -> CH13LEVEL_W {
        CH13LEVEL_W::new(self)
    }
    #[doc = "Bit 14 - Channel 14 Software Level"]
    #[inline(always)]
    pub fn ch14level(&mut self) -> CH14LEVEL_W {
        CH14LEVEL_W::new(self)
    }
    #[doc = "Bit 15 - Channel 15 Software Level"]
    #[inline(always)]
    pub fn ch15level(&mut self) -> CH15LEVEL_W {
        CH15LEVEL_W::new(self)
    }
    #[doc = "Bit 16 - Channel 16 Software Level"]
    #[inline(always)]
    pub fn ch16level(&mut self) -> CH16LEVEL_W {
        CH16LEVEL_W::new(self)
    }
    #[doc = "Bit 17 - Channel 17 Software Level"]
    #[inline(always)]
    pub fn ch17level(&mut self) -> CH17LEVEL_W {
        CH17LEVEL_W::new(self)
    }
    #[doc = "Bit 18 - Channel 18 Software Level"]
    #[inline(always)]
    pub fn ch18level(&mut self) -> CH18LEVEL_W {
        CH18LEVEL_W::new(self)
    }
    #[doc = "Bit 19 - Channel 19 Software Level"]
    #[inline(always)]
    pub fn ch19level(&mut self) -> CH19LEVEL_W {
        CH19LEVEL_W::new(self)
    }
    #[doc = "Bit 20 - Channel 20 Software Level"]
    #[inline(always)]
    pub fn ch20level(&mut self) -> CH20LEVEL_W {
        CH20LEVEL_W::new(self)
    }
    #[doc = "Bit 21 - Channel 21 Software Level"]
    #[inline(always)]
    pub fn ch21level(&mut self) -> CH21LEVEL_W {
        CH21LEVEL_W::new(self)
    }
    #[doc = "Bit 22 - Channel 22 Software Level"]
    #[inline(always)]
    pub fn ch22level(&mut self) -> CH22LEVEL_W {
        CH22LEVEL_W::new(self)
    }
    #[doc = "Bit 23 - Channel 23 Software Level"]
    #[inline(always)]
    pub fn ch23level(&mut self) -> CH23LEVEL_W {
        CH23LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swlevel](index.html) module"]
pub struct SWLEVEL_SPEC;
impl crate::RegisterSpec for SWLEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swlevel::R](R) reader structure"]
impl crate::Readable for SWLEVEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swlevel::W](W) writer structure"]
impl crate::Writable for SWLEVEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWLEVEL to value 0"]
impl crate::Resettable for SWLEVEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
