#[doc = "Register `DTHRCTL` reader"]
pub struct R(crate::R<DTHRCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTHRCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTHRCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTHRCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTHRCTL` writer"]
pub struct W(crate::W<DTHRCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTHRCTL_SPEC>;
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
impl From<crate::W<DTHRCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTHRCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NONISOTHREN` reader - Non-ISO IN Endpoints Threshold Enable"]
pub type NONISOTHREN_R = crate::BitReader<bool>;
#[doc = "Field `NONISOTHREN` writer - Non-ISO IN Endpoints Threshold Enable"]
pub type NONISOTHREN_W<'a> = crate::BitWriter<'a, u32, DTHRCTL_SPEC, bool, 0>;
#[doc = "Field `ISOTHREN` reader - ISO IN Endpoints Threshold Enable"]
pub type ISOTHREN_R = crate::BitReader<bool>;
#[doc = "Field `ISOTHREN` writer - ISO IN Endpoints Threshold Enable"]
pub type ISOTHREN_W<'a> = crate::BitWriter<'a, u32, DTHRCTL_SPEC, bool, 1>;
#[doc = "Field `TXTHRLEN` reader - Transmit Threshold Length"]
pub type TXTHRLEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TXTHRLEN` writer - Transmit Threshold Length"]
pub type TXTHRLEN_W<'a> = crate::FieldWriter<'a, u32, DTHRCTL_SPEC, u16, u16, 9, 2>;
#[doc = "AHB Threshold Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AHBTHRRATIO_A {
    #[doc = "0: AHB threshold = MAC threshold."]
    DIV1 = 0,
    #[doc = "1: AHB threshold = MAC threshold / 2."]
    DIV2 = 1,
    #[doc = "2: AHB threshold = MAC threshold / 4."]
    DIV4 = 2,
    #[doc = "3: AHB threshold = MAC threshold / 8."]
    DIV8 = 3,
}
impl From<AHBTHRRATIO_A> for u8 {
    #[inline(always)]
    fn from(variant: AHBTHRRATIO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AHBTHRRATIO` reader - AHB Threshold Ratio"]
pub type AHBTHRRATIO_R = crate::FieldReader<u8, AHBTHRRATIO_A>;
impl AHBTHRRATIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHBTHRRATIO_A {
        match self.bits {
            0 => AHBTHRRATIO_A::DIV1,
            1 => AHBTHRRATIO_A::DIV2,
            2 => AHBTHRRATIO_A::DIV4,
            3 => AHBTHRRATIO_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == AHBTHRRATIO_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == AHBTHRRATIO_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == AHBTHRRATIO_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == AHBTHRRATIO_A::DIV8
    }
}
#[doc = "Field `AHBTHRRATIO` writer - AHB Threshold Ratio"]
pub type AHBTHRRATIO_W<'a> =
    crate::FieldWriterSafe<'a, u32, DTHRCTL_SPEC, u8, AHBTHRRATIO_A, 2, 11>;
impl<'a> AHBTHRRATIO_W<'a> {
    #[doc = "AHB threshold = MAC threshold."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(AHBTHRRATIO_A::DIV1)
    }
    #[doc = "AHB threshold = MAC threshold / 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(AHBTHRRATIO_A::DIV2)
    }
    #[doc = "AHB threshold = MAC threshold / 4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(AHBTHRRATIO_A::DIV4)
    }
    #[doc = "AHB threshold = MAC threshold / 8."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(AHBTHRRATIO_A::DIV8)
    }
}
#[doc = "Field `RXTHREN` reader - Receive Threshold Enable"]
pub type RXTHREN_R = crate::BitReader<bool>;
#[doc = "Field `RXTHREN` writer - Receive Threshold Enable"]
pub type RXTHREN_W<'a> = crate::BitWriter<'a, u32, DTHRCTL_SPEC, bool, 16>;
#[doc = "Field `RXTHRLEN` reader - Receive Threshold Length"]
pub type RXTHRLEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RXTHRLEN` writer - Receive Threshold Length"]
pub type RXTHRLEN_W<'a> = crate::FieldWriter<'a, u32, DTHRCTL_SPEC, u16, u16, 9, 17>;
#[doc = "Field `ARBPRKEN` reader - Arbiter Parking Enable"]
pub type ARBPRKEN_R = crate::BitReader<bool>;
#[doc = "Field `ARBPRKEN` writer - Arbiter Parking Enable"]
pub type ARBPRKEN_W<'a> = crate::BitWriter<'a, u32, DTHRCTL_SPEC, bool, 27>;
impl R {
    #[doc = "Bit 0 - Non-ISO IN Endpoints Threshold Enable"]
    #[inline(always)]
    pub fn nonisothren(&self) -> NONISOTHREN_R {
        NONISOTHREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ISO IN Endpoints Threshold Enable"]
    #[inline(always)]
    pub fn isothren(&self) -> ISOTHREN_R {
        ISOTHREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:10 - Transmit Threshold Length"]
    #[inline(always)]
    pub fn txthrlen(&self) -> TXTHRLEN_R {
        TXTHRLEN_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bits 11:12 - AHB Threshold Ratio"]
    #[inline(always)]
    pub fn ahbthrratio(&self) -> AHBTHRRATIO_R {
        AHBTHRRATIO_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 16 - Receive Threshold Enable"]
    #[inline(always)]
    pub fn rxthren(&self) -> RXTHREN_R {
        RXTHREN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:25 - Receive Threshold Length"]
    #[inline(always)]
    pub fn rxthrlen(&self) -> RXTHRLEN_R {
        RXTHRLEN_R::new(((self.bits >> 17) & 0x01ff) as u16)
    }
    #[doc = "Bit 27 - Arbiter Parking Enable"]
    #[inline(always)]
    pub fn arbprken(&self) -> ARBPRKEN_R {
        ARBPRKEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non-ISO IN Endpoints Threshold Enable"]
    #[inline(always)]
    pub fn nonisothren(&mut self) -> NONISOTHREN_W {
        NONISOTHREN_W::new(self)
    }
    #[doc = "Bit 1 - ISO IN Endpoints Threshold Enable"]
    #[inline(always)]
    pub fn isothren(&mut self) -> ISOTHREN_W {
        ISOTHREN_W::new(self)
    }
    #[doc = "Bits 2:10 - Transmit Threshold Length"]
    #[inline(always)]
    pub fn txthrlen(&mut self) -> TXTHRLEN_W {
        TXTHRLEN_W::new(self)
    }
    #[doc = "Bits 11:12 - AHB Threshold Ratio"]
    #[inline(always)]
    pub fn ahbthrratio(&mut self) -> AHBTHRRATIO_W {
        AHBTHRRATIO_W::new(self)
    }
    #[doc = "Bit 16 - Receive Threshold Enable"]
    #[inline(always)]
    pub fn rxthren(&mut self) -> RXTHREN_W {
        RXTHREN_W::new(self)
    }
    #[doc = "Bits 17:25 - Receive Threshold Length"]
    #[inline(always)]
    pub fn rxthrlen(&mut self) -> RXTHRLEN_W {
        RXTHRLEN_W::new(self)
    }
    #[doc = "Bit 27 - Arbiter Parking Enable"]
    #[inline(always)]
    pub fn arbprken(&mut self) -> ARBPRKEN_W {
        ARBPRKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Threshold Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dthrctl](index.html) module"]
pub struct DTHRCTL_SPEC;
impl crate::RegisterSpec for DTHRCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dthrctl::R](R) reader structure"]
impl crate::Readable for DTHRCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dthrctl::W](W) writer structure"]
impl crate::Writable for DTHRCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTHRCTL to value 0x0810_0020"]
impl crate::Resettable for DTHRCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0810_0020
    }
}
