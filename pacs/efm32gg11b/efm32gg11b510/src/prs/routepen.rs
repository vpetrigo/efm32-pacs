#[doc = "Register `ROUTEPEN` reader"]
pub struct R(crate::R<ROUTEPEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROUTEPEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROUTEPEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROUTEPEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROUTEPEN` writer"]
pub struct W(crate::W<ROUTEPEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROUTEPEN_SPEC>;
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
impl From<crate::W<ROUTEPEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROUTEPEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0PEN` reader - CH0 Pin Enable"]
pub type CH0PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH0PEN` writer - CH0 Pin Enable"]
pub type CH0PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 0>;
#[doc = "Field `CH1PEN` reader - CH1 Pin Enable"]
pub type CH1PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH1PEN` writer - CH1 Pin Enable"]
pub type CH1PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 1>;
#[doc = "Field `CH2PEN` reader - CH2 Pin Enable"]
pub type CH2PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH2PEN` writer - CH2 Pin Enable"]
pub type CH2PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 2>;
#[doc = "Field `CH3PEN` reader - CH3 Pin Enable"]
pub type CH3PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH3PEN` writer - CH3 Pin Enable"]
pub type CH3PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 3>;
#[doc = "Field `CH4PEN` reader - CH4 Pin Enable"]
pub type CH4PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH4PEN` writer - CH4 Pin Enable"]
pub type CH4PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 4>;
#[doc = "Field `CH5PEN` reader - CH5 Pin Enable"]
pub type CH5PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH5PEN` writer - CH5 Pin Enable"]
pub type CH5PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 5>;
#[doc = "Field `CH6PEN` reader - CH6 Pin Enable"]
pub type CH6PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH6PEN` writer - CH6 Pin Enable"]
pub type CH6PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 6>;
#[doc = "Field `CH7PEN` reader - CH7 Pin Enable"]
pub type CH7PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH7PEN` writer - CH7 Pin Enable"]
pub type CH7PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 7>;
#[doc = "Field `CH8PEN` reader - CH8 Pin Enable"]
pub type CH8PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH8PEN` writer - CH8 Pin Enable"]
pub type CH8PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 8>;
#[doc = "Field `CH9PEN` reader - CH9 Pin Enable"]
pub type CH9PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH9PEN` writer - CH9 Pin Enable"]
pub type CH9PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 9>;
#[doc = "Field `CH10PEN` reader - CH10 Pin Enable"]
pub type CH10PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH10PEN` writer - CH10 Pin Enable"]
pub type CH10PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 10>;
#[doc = "Field `CH11PEN` reader - CH11 Pin Enable"]
pub type CH11PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH11PEN` writer - CH11 Pin Enable"]
pub type CH11PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 11>;
#[doc = "Field `CH12PEN` reader - CH12 Pin Enable"]
pub type CH12PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH12PEN` writer - CH12 Pin Enable"]
pub type CH12PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 12>;
#[doc = "Field `CH13PEN` reader - CH13 Pin Enable"]
pub type CH13PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH13PEN` writer - CH13 Pin Enable"]
pub type CH13PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 13>;
#[doc = "Field `CH14PEN` reader - CH14 Pin Enable"]
pub type CH14PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH14PEN` writer - CH14 Pin Enable"]
pub type CH14PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 14>;
#[doc = "Field `CH15PEN` reader - CH15 Pin Enable"]
pub type CH15PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH15PEN` writer - CH15 Pin Enable"]
pub type CH15PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 15>;
#[doc = "Field `CH16PEN` reader - CH16 Pin Enable"]
pub type CH16PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH16PEN` writer - CH16 Pin Enable"]
pub type CH16PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 16>;
#[doc = "Field `CH17PEN` reader - CH17 Pin Enable"]
pub type CH17PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH17PEN` writer - CH17 Pin Enable"]
pub type CH17PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 17>;
#[doc = "Field `CH18PEN` reader - CH18 Pin Enable"]
pub type CH18PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH18PEN` writer - CH18 Pin Enable"]
pub type CH18PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 18>;
#[doc = "Field `CH19PEN` reader - CH19 Pin Enable"]
pub type CH19PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH19PEN` writer - CH19 Pin Enable"]
pub type CH19PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 19>;
#[doc = "Field `CH20PEN` reader - CH20 Pin Enable"]
pub type CH20PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH20PEN` writer - CH20 Pin Enable"]
pub type CH20PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 20>;
#[doc = "Field `CH21PEN` reader - CH21 Pin Enable"]
pub type CH21PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH21PEN` writer - CH21 Pin Enable"]
pub type CH21PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 21>;
#[doc = "Field `CH22PEN` reader - CH22 Pin Enable"]
pub type CH22PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH22PEN` writer - CH22 Pin Enable"]
pub type CH22PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 22>;
#[doc = "Field `CH23PEN` reader - CH23 Pin Enable"]
pub type CH23PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH23PEN` writer - CH23 Pin Enable"]
pub type CH23PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 23>;
impl R {
    #[doc = "Bit 0 - CH0 Pin Enable"]
    #[inline(always)]
    pub fn ch0pen(&self) -> CH0PEN_R {
        CH0PEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1 Pin Enable"]
    #[inline(always)]
    pub fn ch1pen(&self) -> CH1PEN_R {
        CH1PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH2 Pin Enable"]
    #[inline(always)]
    pub fn ch2pen(&self) -> CH2PEN_R {
        CH2PEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH3 Pin Enable"]
    #[inline(always)]
    pub fn ch3pen(&self) -> CH3PEN_R {
        CH3PEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH4 Pin Enable"]
    #[inline(always)]
    pub fn ch4pen(&self) -> CH4PEN_R {
        CH4PEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH5 Pin Enable"]
    #[inline(always)]
    pub fn ch5pen(&self) -> CH5PEN_R {
        CH5PEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CH6 Pin Enable"]
    #[inline(always)]
    pub fn ch6pen(&self) -> CH6PEN_R {
        CH6PEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CH7 Pin Enable"]
    #[inline(always)]
    pub fn ch7pen(&self) -> CH7PEN_R {
        CH7PEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CH8 Pin Enable"]
    #[inline(always)]
    pub fn ch8pen(&self) -> CH8PEN_R {
        CH8PEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CH9 Pin Enable"]
    #[inline(always)]
    pub fn ch9pen(&self) -> CH9PEN_R {
        CH9PEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CH10 Pin Enable"]
    #[inline(always)]
    pub fn ch10pen(&self) -> CH10PEN_R {
        CH10PEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CH11 Pin Enable"]
    #[inline(always)]
    pub fn ch11pen(&self) -> CH11PEN_R {
        CH11PEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CH12 Pin Enable"]
    #[inline(always)]
    pub fn ch12pen(&self) -> CH12PEN_R {
        CH12PEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CH13 Pin Enable"]
    #[inline(always)]
    pub fn ch13pen(&self) -> CH13PEN_R {
        CH13PEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CH14 Pin Enable"]
    #[inline(always)]
    pub fn ch14pen(&self) -> CH14PEN_R {
        CH14PEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CH15 Pin Enable"]
    #[inline(always)]
    pub fn ch15pen(&self) -> CH15PEN_R {
        CH15PEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CH16 Pin Enable"]
    #[inline(always)]
    pub fn ch16pen(&self) -> CH16PEN_R {
        CH16PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CH17 Pin Enable"]
    #[inline(always)]
    pub fn ch17pen(&self) -> CH17PEN_R {
        CH17PEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CH18 Pin Enable"]
    #[inline(always)]
    pub fn ch18pen(&self) -> CH18PEN_R {
        CH18PEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CH19 Pin Enable"]
    #[inline(always)]
    pub fn ch19pen(&self) -> CH19PEN_R {
        CH19PEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CH20 Pin Enable"]
    #[inline(always)]
    pub fn ch20pen(&self) -> CH20PEN_R {
        CH20PEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CH21 Pin Enable"]
    #[inline(always)]
    pub fn ch21pen(&self) -> CH21PEN_R {
        CH21PEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CH22 Pin Enable"]
    #[inline(always)]
    pub fn ch22pen(&self) -> CH22PEN_R {
        CH22PEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CH23 Pin Enable"]
    #[inline(always)]
    pub fn ch23pen(&self) -> CH23PEN_R {
        CH23PEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0 Pin Enable"]
    #[inline(always)]
    pub fn ch0pen(&mut self) -> CH0PEN_W {
        CH0PEN_W::new(self)
    }
    #[doc = "Bit 1 - CH1 Pin Enable"]
    #[inline(always)]
    pub fn ch1pen(&mut self) -> CH1PEN_W {
        CH1PEN_W::new(self)
    }
    #[doc = "Bit 2 - CH2 Pin Enable"]
    #[inline(always)]
    pub fn ch2pen(&mut self) -> CH2PEN_W {
        CH2PEN_W::new(self)
    }
    #[doc = "Bit 3 - CH3 Pin Enable"]
    #[inline(always)]
    pub fn ch3pen(&mut self) -> CH3PEN_W {
        CH3PEN_W::new(self)
    }
    #[doc = "Bit 4 - CH4 Pin Enable"]
    #[inline(always)]
    pub fn ch4pen(&mut self) -> CH4PEN_W {
        CH4PEN_W::new(self)
    }
    #[doc = "Bit 5 - CH5 Pin Enable"]
    #[inline(always)]
    pub fn ch5pen(&mut self) -> CH5PEN_W {
        CH5PEN_W::new(self)
    }
    #[doc = "Bit 6 - CH6 Pin Enable"]
    #[inline(always)]
    pub fn ch6pen(&mut self) -> CH6PEN_W {
        CH6PEN_W::new(self)
    }
    #[doc = "Bit 7 - CH7 Pin Enable"]
    #[inline(always)]
    pub fn ch7pen(&mut self) -> CH7PEN_W {
        CH7PEN_W::new(self)
    }
    #[doc = "Bit 8 - CH8 Pin Enable"]
    #[inline(always)]
    pub fn ch8pen(&mut self) -> CH8PEN_W {
        CH8PEN_W::new(self)
    }
    #[doc = "Bit 9 - CH9 Pin Enable"]
    #[inline(always)]
    pub fn ch9pen(&mut self) -> CH9PEN_W {
        CH9PEN_W::new(self)
    }
    #[doc = "Bit 10 - CH10 Pin Enable"]
    #[inline(always)]
    pub fn ch10pen(&mut self) -> CH10PEN_W {
        CH10PEN_W::new(self)
    }
    #[doc = "Bit 11 - CH11 Pin Enable"]
    #[inline(always)]
    pub fn ch11pen(&mut self) -> CH11PEN_W {
        CH11PEN_W::new(self)
    }
    #[doc = "Bit 12 - CH12 Pin Enable"]
    #[inline(always)]
    pub fn ch12pen(&mut self) -> CH12PEN_W {
        CH12PEN_W::new(self)
    }
    #[doc = "Bit 13 - CH13 Pin Enable"]
    #[inline(always)]
    pub fn ch13pen(&mut self) -> CH13PEN_W {
        CH13PEN_W::new(self)
    }
    #[doc = "Bit 14 - CH14 Pin Enable"]
    #[inline(always)]
    pub fn ch14pen(&mut self) -> CH14PEN_W {
        CH14PEN_W::new(self)
    }
    #[doc = "Bit 15 - CH15 Pin Enable"]
    #[inline(always)]
    pub fn ch15pen(&mut self) -> CH15PEN_W {
        CH15PEN_W::new(self)
    }
    #[doc = "Bit 16 - CH16 Pin Enable"]
    #[inline(always)]
    pub fn ch16pen(&mut self) -> CH16PEN_W {
        CH16PEN_W::new(self)
    }
    #[doc = "Bit 17 - CH17 Pin Enable"]
    #[inline(always)]
    pub fn ch17pen(&mut self) -> CH17PEN_W {
        CH17PEN_W::new(self)
    }
    #[doc = "Bit 18 - CH18 Pin Enable"]
    #[inline(always)]
    pub fn ch18pen(&mut self) -> CH18PEN_W {
        CH18PEN_W::new(self)
    }
    #[doc = "Bit 19 - CH19 Pin Enable"]
    #[inline(always)]
    pub fn ch19pen(&mut self) -> CH19PEN_W {
        CH19PEN_W::new(self)
    }
    #[doc = "Bit 20 - CH20 Pin Enable"]
    #[inline(always)]
    pub fn ch20pen(&mut self) -> CH20PEN_W {
        CH20PEN_W::new(self)
    }
    #[doc = "Bit 21 - CH21 Pin Enable"]
    #[inline(always)]
    pub fn ch21pen(&mut self) -> CH21PEN_W {
        CH21PEN_W::new(self)
    }
    #[doc = "Bit 22 - CH22 Pin Enable"]
    #[inline(always)]
    pub fn ch22pen(&mut self) -> CH22PEN_W {
        CH22PEN_W::new(self)
    }
    #[doc = "Bit 23 - CH23 Pin Enable"]
    #[inline(always)]
    pub fn ch23pen(&mut self) -> CH23PEN_W {
        CH23PEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Routing Pin Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routepen](index.html) module"]
pub struct ROUTEPEN_SPEC;
impl crate::RegisterSpec for ROUTEPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [routepen::R](R) reader structure"]
impl crate::Readable for ROUTEPEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [routepen::W](W) writer structure"]
impl crate::Writable for ROUTEPEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROUTEPEN to value 0"]
impl crate::Resettable for ROUTEPEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
