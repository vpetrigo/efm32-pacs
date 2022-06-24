#[doc = "Register `CH5_EVALCFG` reader"]
pub struct R(crate::R<CH5_EVALCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH5_EVALCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH5_EVALCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH5_EVALCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH5_EVALCFG` writer"]
pub struct W(crate::W<CH5_EVALCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH5_EVALCFG_SPEC>;
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
impl From<crate::W<CH5_EVALCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH5_EVALCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DECODE` reader - Send result to decoder"]
pub type DECODE_R = crate::BitReader<bool>;
#[doc = "Field `DECODE` writer - Send result to decoder"]
pub type DECODE_W<'a> = crate::BitWriter<'a, u32, CH5_EVALCFG_SPEC, bool, 2>;
#[doc = "Select mode for threshold comparison\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP_A {
    #[doc = "0: Comparison evaluates to 1 if sensor data is less than CTRTHRESHOLD, or if the ACMP output is 0"]
    LESS = 0,
    #[doc = "1: Comparison evaluates to 1 if sensor data is greater than, or equal to CTRTHRESHOLD, or if the ACMP output is 1"]
    GE = 1,
}
impl From<COMP_A> for bool {
    #[inline(always)]
    fn from(variant: COMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP` reader - Select mode for threshold comparison"]
pub type COMP_R = crate::BitReader<COMP_A>;
impl COMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP_A {
        match self.bits {
            false => COMP_A::LESS,
            true => COMP_A::GE,
        }
    }
    #[doc = "Checks if the value of the field is `LESS`"]
    #[inline(always)]
    pub fn is_less(&self) -> bool {
        *self == COMP_A::LESS
    }
    #[doc = "Checks if the value of the field is `GE`"]
    #[inline(always)]
    pub fn is_ge(&self) -> bool {
        *self == COMP_A::GE
    }
}
#[doc = "Field `COMP` writer - Select mode for threshold comparison"]
pub type COMP_W<'a> = crate::BitWriter<'a, u32, CH5_EVALCFG_SPEC, COMP_A, 3>;
impl<'a> COMP_W<'a> {
    #[doc = "Comparison evaluates to 1 if sensor data is less than CTRTHRESHOLD, or if the ACMP output is 0"]
    #[inline(always)]
    pub fn less(self) -> &'a mut W {
        self.variant(COMP_A::LESS)
    }
    #[doc = "Comparison evaluates to 1 if sensor data is greater than, or equal to CTRTHRESHOLD, or if the ACMP output is 1"]
    #[inline(always)]
    pub fn ge(self) -> &'a mut W {
        self.variant(COMP_A::GE)
    }
}
#[doc = "Enable storing of sensor sample in resul\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STRSAMPLE_A {
    #[doc = "0: Nothing will be stored in the result buffer."]
    DISABLE = 0,
    #[doc = "1: The sensor sample data will be stored in the result buffer."]
    DATA = 1,
    #[doc = "2: The data source, i.e. the channel, will be stored alongside the sensor sample data."]
    DATASRC = 2,
}
impl From<STRSAMPLE_A> for u8 {
    #[inline(always)]
    fn from(variant: STRSAMPLE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STRSAMPLE` reader - Enable storing of sensor sample in resul"]
pub type STRSAMPLE_R = crate::FieldReader<u8, STRSAMPLE_A>;
impl STRSAMPLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STRSAMPLE_A> {
        match self.bits {
            0 => Some(STRSAMPLE_A::DISABLE),
            1 => Some(STRSAMPLE_A::DATA),
            2 => Some(STRSAMPLE_A::DATASRC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STRSAMPLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == STRSAMPLE_A::DATA
    }
    #[doc = "Checks if the value of the field is `DATASRC`"]
    #[inline(always)]
    pub fn is_datasrc(&self) -> bool {
        *self == STRSAMPLE_A::DATASRC
    }
}
#[doc = "Field `STRSAMPLE` writer - Enable storing of sensor sample in resul"]
pub type STRSAMPLE_W<'a> = crate::FieldWriter<'a, u32, CH5_EVALCFG_SPEC, u8, STRSAMPLE_A, 2, 4>;
impl<'a> STRSAMPLE_W<'a> {
    #[doc = "Nothing will be stored in the result buffer."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(STRSAMPLE_A::DISABLE)
    }
    #[doc = "The sensor sample data will be stored in the result buffer."]
    #[inline(always)]
    pub fn data(self) -> &'a mut W {
        self.variant(STRSAMPLE_A::DATA)
    }
    #[doc = "The data source, i.e. the channel, will be stored alongside the sensor sample data."]
    #[inline(always)]
    pub fn datasrc(self) -> &'a mut W {
        self.variant(STRSAMPLE_A::DATASRC)
    }
}
#[doc = "Field `SCANRESINV` reader - Enable inversion of result"]
pub type SCANRESINV_R = crate::BitReader<bool>;
#[doc = "Field `SCANRESINV` writer - Enable inversion of result"]
pub type SCANRESINV_W<'a> = crate::BitWriter<'a, u32, CH5_EVALCFG_SPEC, bool, 6>;
#[doc = "Configure evaluation mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Threshold comparison is used to evaluate sensor result"]
    THRES = 0,
    #[doc = "1: Sliding window is used to evaluate sensor result"]
    SLIDINGWIN = 1,
    #[doc = "2: Step detection is used to evaluate sensor result"]
    STEPDET = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - Configure evaluation mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::THRES),
            1 => Some(MODE_A::SLIDINGWIN),
            2 => Some(MODE_A::STEPDET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `THRES`"]
    #[inline(always)]
    pub fn is_thres(&self) -> bool {
        *self == MODE_A::THRES
    }
    #[doc = "Checks if the value of the field is `SLIDINGWIN`"]
    #[inline(always)]
    pub fn is_slidingwin(&self) -> bool {
        *self == MODE_A::SLIDINGWIN
    }
    #[doc = "Checks if the value of the field is `STEPDET`"]
    #[inline(always)]
    pub fn is_stepdet(&self) -> bool {
        *self == MODE_A::STEPDET
    }
}
#[doc = "Field `MODE` writer - Configure evaluation mode"]
pub type MODE_W<'a> = crate::FieldWriter<'a, u32, CH5_EVALCFG_SPEC, u8, MODE_A, 2, 8>;
impl<'a> MODE_W<'a> {
    #[doc = "Threshold comparison is used to evaluate sensor result"]
    #[inline(always)]
    pub fn thres(self) -> &'a mut W {
        self.variant(MODE_A::THRES)
    }
    #[doc = "Sliding window is used to evaluate sensor result"]
    #[inline(always)]
    pub fn slidingwin(self) -> &'a mut W {
        self.variant(MODE_A::SLIDINGWIN)
    }
    #[doc = "Step detection is used to evaluate sensor result"]
    #[inline(always)]
    pub fn stepdet(self) -> &'a mut W {
        self.variant(MODE_A::STEPDET)
    }
}
impl R {
    #[doc = "Bit 2 - Send result to decoder"]
    #[inline(always)]
    pub fn decode(&self) -> DECODE_R {
        DECODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Select mode for threshold comparison"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Enable storing of sensor sample in resul"]
    #[inline(always)]
    pub fn strsample(&self) -> STRSAMPLE_R {
        STRSAMPLE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Enable inversion of result"]
    #[inline(always)]
    pub fn scanresinv(&self) -> SCANRESINV_R {
        SCANRESINV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Configure evaluation mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Send result to decoder"]
    #[inline(always)]
    pub fn decode(&mut self) -> DECODE_W {
        DECODE_W::new(self)
    }
    #[doc = "Bit 3 - Select mode for threshold comparison"]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W {
        COMP_W::new(self)
    }
    #[doc = "Bits 4:5 - Enable storing of sensor sample in resul"]
    #[inline(always)]
    pub fn strsample(&mut self) -> STRSAMPLE_W {
        STRSAMPLE_W::new(self)
    }
    #[doc = "Bit 6 - Enable inversion of result"]
    #[inline(always)]
    pub fn scanresinv(&mut self) -> SCANRESINV_W {
        SCANRESINV_W::new(self)
    }
    #[doc = "Bits 8:9 - Configure evaluation mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_evalcfg](index.html) module"]
pub struct CH5_EVALCFG_SPEC;
impl crate::RegisterSpec for CH5_EVALCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch5_evalcfg::R](R) reader structure"]
impl crate::Readable for CH5_EVALCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch5_evalcfg::W](W) writer structure"]
impl crate::Writable for CH5_EVALCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH5_EVALCFG to value 0"]
impl crate::Resettable for CH5_EVALCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
