#[doc = "Register `PERCTRL` reader"]
pub struct R(crate::R<PERCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERCTRL` writer"]
pub struct W(crate::W<PERCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERCTRL_SPEC>;
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
impl From<crate::W<PERCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DAC CH0 data selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACCH0DATA_A {
    #[doc = "0: DAC data is defined by CH0DATA in the DAC interface."]
    DACDATA = 0,
    #[doc = "1: DAC data is defined by THRES in CHx_INTERACT."]
    THRES = 1,
}
impl From<DACCH0DATA_A> for bool {
    #[inline(always)]
    fn from(variant: DACCH0DATA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACCH0DATA` reader - DAC CH0 data selection."]
pub type DACCH0DATA_R = crate::BitReader<DACCH0DATA_A>;
impl DACCH0DATA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACCH0DATA_A {
        match self.bits {
            false => DACCH0DATA_A::DACDATA,
            true => DACCH0DATA_A::THRES,
        }
    }
    #[doc = "Checks if the value of the field is `DACDATA`"]
    #[inline(always)]
    pub fn is_dacdata(&self) -> bool {
        *self == DACCH0DATA_A::DACDATA
    }
    #[doc = "Checks if the value of the field is `THRES`"]
    #[inline(always)]
    pub fn is_thres(&self) -> bool {
        *self == DACCH0DATA_A::THRES
    }
}
#[doc = "Field `DACCH0DATA` writer - DAC CH0 data selection."]
pub type DACCH0DATA_W<'a> = crate::BitWriter<'a, u32, PERCTRL_SPEC, DACCH0DATA_A, 2>;
impl<'a> DACCH0DATA_W<'a> {
    #[doc = "DAC data is defined by CH0DATA in the DAC interface."]
    #[inline(always)]
    pub fn dacdata(self) -> &'a mut W {
        self.variant(DACCH0DATA_A::DACDATA)
    }
    #[doc = "DAC data is defined by THRES in CHx_INTERACT."]
    #[inline(always)]
    pub fn thres(self) -> &'a mut W {
        self.variant(DACCH0DATA_A::THRES)
    }
}
#[doc = "DAC startup configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACSTARTUP_A {
    #[doc = "0: DAC is started a full LESENSECLK before sensor interaction starts."]
    FULLCYCLE = 0,
    #[doc = "1: DAC is started half a LESENSECLK cycle before sensor interaction starts."]
    HALFCYCLE = 1,
}
impl From<DACSTARTUP_A> for bool {
    #[inline(always)]
    fn from(variant: DACSTARTUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACSTARTUP` reader - DAC startup configuration"]
pub type DACSTARTUP_R = crate::BitReader<DACSTARTUP_A>;
impl DACSTARTUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACSTARTUP_A {
        match self.bits {
            false => DACSTARTUP_A::FULLCYCLE,
            true => DACSTARTUP_A::HALFCYCLE,
        }
    }
    #[doc = "Checks if the value of the field is `FULLCYCLE`"]
    #[inline(always)]
    pub fn is_fullcycle(&self) -> bool {
        *self == DACSTARTUP_A::FULLCYCLE
    }
    #[doc = "Checks if the value of the field is `HALFCYCLE`"]
    #[inline(always)]
    pub fn is_halfcycle(&self) -> bool {
        *self == DACSTARTUP_A::HALFCYCLE
    }
}
#[doc = "Field `DACSTARTUP` writer - DAC startup configuration"]
pub type DACSTARTUP_W<'a> = crate::BitWriter<'a, u32, PERCTRL_SPEC, DACSTARTUP_A, 6>;
impl<'a> DACSTARTUP_W<'a> {
    #[doc = "DAC is started a full LESENSECLK before sensor interaction starts."]
    #[inline(always)]
    pub fn fullcycle(self) -> &'a mut W {
        self.variant(DACSTARTUP_A::FULLCYCLE)
    }
    #[doc = "DAC is started half a LESENSECLK cycle before sensor interaction starts."]
    #[inline(always)]
    pub fn halfcycle(self) -> &'a mut W {
        self.variant(DACSTARTUP_A::HALFCYCLE)
    }
}
#[doc = "DAC conversion trigger configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACCONVTRIG_A {
    #[doc = "0: DAC is enabled before every LESENSE channle measurement."]
    CHANNELSTART = 0,
    #[doc = "1: DAC is only enabled once per scan."]
    SCANSTART = 1,
}
impl From<DACCONVTRIG_A> for bool {
    #[inline(always)]
    fn from(variant: DACCONVTRIG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACCONVTRIG` reader - DAC conversion trigger configuration"]
pub type DACCONVTRIG_R = crate::BitReader<DACCONVTRIG_A>;
impl DACCONVTRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACCONVTRIG_A {
        match self.bits {
            false => DACCONVTRIG_A::CHANNELSTART,
            true => DACCONVTRIG_A::SCANSTART,
        }
    }
    #[doc = "Checks if the value of the field is `CHANNELSTART`"]
    #[inline(always)]
    pub fn is_channelstart(&self) -> bool {
        *self == DACCONVTRIG_A::CHANNELSTART
    }
    #[doc = "Checks if the value of the field is `SCANSTART`"]
    #[inline(always)]
    pub fn is_scanstart(&self) -> bool {
        *self == DACCONVTRIG_A::SCANSTART
    }
}
#[doc = "Field `DACCONVTRIG` writer - DAC conversion trigger configuration"]
pub type DACCONVTRIG_W<'a> = crate::BitWriter<'a, u32, PERCTRL_SPEC, DACCONVTRIG_A, 8>;
impl<'a> DACCONVTRIG_W<'a> {
    #[doc = "DAC is enabled before every LESENSE channle measurement."]
    #[inline(always)]
    pub fn channelstart(self) -> &'a mut W {
        self.variant(DACCONVTRIG_A::CHANNELSTART)
    }
    #[doc = "DAC is only enabled once per scan."]
    #[inline(always)]
    pub fn scanstart(self) -> &'a mut W {
        self.variant(DACCONVTRIG_A::SCANSTART)
    }
}
#[doc = "ACMP0 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP0MODE_A {
    #[doc = "0: LESENSE controls POSSEL of ACMP0"]
    MUX = 0,
    #[doc = "1: LESENSE controls POSSEL and reference divider of ACMP0"]
    MUXTHRES = 1,
}
impl From<ACMP0MODE_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP0MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP0MODE` reader - ACMP0 mode"]
pub type ACMP0MODE_R = crate::BitReader<ACMP0MODE_A>;
impl ACMP0MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP0MODE_A {
        match self.bits {
            false => ACMP0MODE_A::MUX,
            true => ACMP0MODE_A::MUXTHRES,
        }
    }
    #[doc = "Checks if the value of the field is `MUX`"]
    #[inline(always)]
    pub fn is_mux(&self) -> bool {
        *self == ACMP0MODE_A::MUX
    }
    #[doc = "Checks if the value of the field is `MUXTHRES`"]
    #[inline(always)]
    pub fn is_muxthres(&self) -> bool {
        *self == ACMP0MODE_A::MUXTHRES
    }
}
#[doc = "Field `ACMP0MODE` writer - ACMP0 mode"]
pub type ACMP0MODE_W<'a> = crate::BitWriter<'a, u32, PERCTRL_SPEC, ACMP0MODE_A, 20>;
impl<'a> ACMP0MODE_W<'a> {
    #[doc = "LESENSE controls POSSEL of ACMP0"]
    #[inline(always)]
    pub fn mux(self) -> &'a mut W {
        self.variant(ACMP0MODE_A::MUX)
    }
    #[doc = "LESENSE controls POSSEL and reference divider of ACMP0"]
    #[inline(always)]
    pub fn muxthres(self) -> &'a mut W {
        self.variant(ACMP0MODE_A::MUXTHRES)
    }
}
#[doc = "ACMP1 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP1MODE_A {
    #[doc = "0: LESENSE controls the POSSEL of ACMP1"]
    MUX = 0,
    #[doc = "1: LESENSE POSSEL and reference divider of ACMP1"]
    MUXTHRES = 1,
}
impl From<ACMP1MODE_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP1MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP1MODE` reader - ACMP1 mode"]
pub type ACMP1MODE_R = crate::BitReader<ACMP1MODE_A>;
impl ACMP1MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP1MODE_A {
        match self.bits {
            false => ACMP1MODE_A::MUX,
            true => ACMP1MODE_A::MUXTHRES,
        }
    }
    #[doc = "Checks if the value of the field is `MUX`"]
    #[inline(always)]
    pub fn is_mux(&self) -> bool {
        *self == ACMP1MODE_A::MUX
    }
    #[doc = "Checks if the value of the field is `MUXTHRES`"]
    #[inline(always)]
    pub fn is_muxthres(&self) -> bool {
        *self == ACMP1MODE_A::MUXTHRES
    }
}
#[doc = "Field `ACMP1MODE` writer - ACMP1 mode"]
pub type ACMP1MODE_W<'a> = crate::BitWriter<'a, u32, PERCTRL_SPEC, ACMP1MODE_A, 22>;
impl<'a> ACMP1MODE_W<'a> {
    #[doc = "LESENSE controls the POSSEL of ACMP1"]
    #[inline(always)]
    pub fn mux(self) -> &'a mut W {
        self.variant(ACMP1MODE_A::MUX)
    }
    #[doc = "LESENSE POSSEL and reference divider of ACMP1"]
    #[inline(always)]
    pub fn muxthres(self) -> &'a mut W {
        self.variant(ACMP1MODE_A::MUXTHRES)
    }
}
#[doc = "Field `ACMP0INV` reader - Invert analog comparator 0 output"]
pub type ACMP0INV_R = crate::BitReader<bool>;
#[doc = "Field `ACMP0INV` writer - Invert analog comparator 0 output"]
pub type ACMP0INV_W<'a> = crate::BitWriter<'a, u32, PERCTRL_SPEC, bool, 24>;
#[doc = "Field `ACMP1INV` reader - Invert analog comparator 1 output"]
pub type ACMP1INV_R = crate::BitReader<bool>;
#[doc = "Field `ACMP1INV` writer - Invert analog comparator 1 output"]
pub type ACMP1INV_W<'a> = crate::BitWriter<'a, u32, PERCTRL_SPEC, bool, 25>;
impl R {
    #[doc = "Bit 2 - DAC CH0 data selection."]
    #[inline(always)]
    pub fn dacch0data(&self) -> DACCH0DATA_R {
        DACCH0DATA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - DAC startup configuration"]
    #[inline(always)]
    pub fn dacstartup(&self) -> DACSTARTUP_R {
        DACSTARTUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - DAC conversion trigger configuration"]
    #[inline(always)]
    pub fn dacconvtrig(&self) -> DACCONVTRIG_R {
        DACCONVTRIG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 20 - ACMP0 mode"]
    #[inline(always)]
    pub fn acmp0mode(&self) -> ACMP0MODE_R {
        ACMP0MODE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - ACMP1 mode"]
    #[inline(always)]
    pub fn acmp1mode(&self) -> ACMP1MODE_R {
        ACMP1MODE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Invert analog comparator 0 output"]
    #[inline(always)]
    pub fn acmp0inv(&self) -> ACMP0INV_R {
        ACMP0INV_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Invert analog comparator 1 output"]
    #[inline(always)]
    pub fn acmp1inv(&self) -> ACMP1INV_R {
        ACMP1INV_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - DAC CH0 data selection."]
    #[inline(always)]
    pub fn dacch0data(&mut self) -> DACCH0DATA_W {
        DACCH0DATA_W::new(self)
    }
    #[doc = "Bit 6 - DAC startup configuration"]
    #[inline(always)]
    pub fn dacstartup(&mut self) -> DACSTARTUP_W {
        DACSTARTUP_W::new(self)
    }
    #[doc = "Bit 8 - DAC conversion trigger configuration"]
    #[inline(always)]
    pub fn dacconvtrig(&mut self) -> DACCONVTRIG_W {
        DACCONVTRIG_W::new(self)
    }
    #[doc = "Bit 20 - ACMP0 mode"]
    #[inline(always)]
    pub fn acmp0mode(&mut self) -> ACMP0MODE_W {
        ACMP0MODE_W::new(self)
    }
    #[doc = "Bit 22 - ACMP1 mode"]
    #[inline(always)]
    pub fn acmp1mode(&mut self) -> ACMP1MODE_W {
        ACMP1MODE_W::new(self)
    }
    #[doc = "Bit 24 - Invert analog comparator 0 output"]
    #[inline(always)]
    pub fn acmp0inv(&mut self) -> ACMP0INV_W {
        ACMP0INV_W::new(self)
    }
    #[doc = "Bit 25 - Invert analog comparator 1 output"]
    #[inline(always)]
    pub fn acmp1inv(&mut self) -> ACMP1INV_W {
        ACMP1INV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perctrl](index.html) module"]
pub struct PERCTRL_SPEC;
impl crate::RegisterSpec for PERCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perctrl::R](R) reader structure"]
impl crate::Readable for PERCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perctrl::W](W) writer structure"]
impl crate::Writable for PERCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERCTRL to value 0"]
impl crate::Resettable for PERCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
