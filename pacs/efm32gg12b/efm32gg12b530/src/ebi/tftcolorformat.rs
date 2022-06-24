#[doc = "Register `TFTCOLORFORMAT` reader"]
pub struct R(crate::R<TFTCOLORFORMAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFTCOLORFORMAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFTCOLORFORMAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFTCOLORFORMAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TFTCOLORFORMAT` writer"]
pub struct W(crate::W<TFTCOLORFORMAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TFTCOLORFORMAT_SPEC>;
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
impl From<crate::W<TFTCOLORFORMAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TFTCOLORFORMAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Sprite Pixel Color Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PIXEL0FORMAT_A {
    #[doc = "0: ARGB data is 0555"]
    ARGB0555 = 0,
    #[doc = "1: ARGB data is 0565"]
    ARGB0565 = 1,
    #[doc = "2: ARGB data is 0666"]
    ARGB0666 = 2,
    #[doc = "3: ARGB data is 0888"]
    ARGB0888 = 3,
    #[doc = "4: ARGB data is 5555"]
    ARGB5555 = 4,
    #[doc = "5: ARGB data is 6565"]
    ARGB6565 = 5,
    #[doc = "6: ARGB data is 6666"]
    ARGB6666 = 6,
    #[doc = "7: ARGB data is 8888"]
    ARGB8888 = 7,
}
impl From<PIXEL0FORMAT_A> for u8 {
    #[inline(always)]
    fn from(variant: PIXEL0FORMAT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PIXEL0FORMAT` reader - Sprite Pixel Color Format"]
pub type PIXEL0FORMAT_R = crate::FieldReader<u8, PIXEL0FORMAT_A>;
impl PIXEL0FORMAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIXEL0FORMAT_A {
        match self.bits {
            0 => PIXEL0FORMAT_A::ARGB0555,
            1 => PIXEL0FORMAT_A::ARGB0565,
            2 => PIXEL0FORMAT_A::ARGB0666,
            3 => PIXEL0FORMAT_A::ARGB0888,
            4 => PIXEL0FORMAT_A::ARGB5555,
            5 => PIXEL0FORMAT_A::ARGB6565,
            6 => PIXEL0FORMAT_A::ARGB6666,
            7 => PIXEL0FORMAT_A::ARGB8888,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ARGB0555`"]
    #[inline(always)]
    pub fn is_argb0555(&self) -> bool {
        *self == PIXEL0FORMAT_A::ARGB0555
    }
    #[doc = "Checks if the value of the field is `ARGB0565`"]
    #[inline(always)]
    pub fn is_argb0565(&self) -> bool {
        *self == PIXEL0FORMAT_A::ARGB0565
    }
    #[doc = "Checks if the value of the field is `ARGB0666`"]
    #[inline(always)]
    pub fn is_argb0666(&self) -> bool {
        *self == PIXEL0FORMAT_A::ARGB0666
    }
    #[doc = "Checks if the value of the field is `ARGB0888`"]
    #[inline(always)]
    pub fn is_argb0888(&self) -> bool {
        *self == PIXEL0FORMAT_A::ARGB0888
    }
    #[doc = "Checks if the value of the field is `ARGB5555`"]
    #[inline(always)]
    pub fn is_argb5555(&self) -> bool {
        *self == PIXEL0FORMAT_A::ARGB5555
    }
    #[doc = "Checks if the value of the field is `ARGB6565`"]
    #[inline(always)]
    pub fn is_argb6565(&self) -> bool {
        *self == PIXEL0FORMAT_A::ARGB6565
    }
    #[doc = "Checks if the value of the field is `ARGB6666`"]
    #[inline(always)]
    pub fn is_argb6666(&self) -> bool {
        *self == PIXEL0FORMAT_A::ARGB6666
    }
    #[doc = "Checks if the value of the field is `ARGB8888`"]
    #[inline(always)]
    pub fn is_argb8888(&self) -> bool {
        *self == PIXEL0FORMAT_A::ARGB8888
    }
}
#[doc = "Field `PIXEL0FORMAT` writer - Sprite Pixel Color Format"]
pub type PIXEL0FORMAT_W<'a> =
    crate::FieldWriterSafe<'a, u32, TFTCOLORFORMAT_SPEC, u8, PIXEL0FORMAT_A, 3, 0>;
impl<'a> PIXEL0FORMAT_W<'a> {
    #[doc = "ARGB data is 0555"]
    #[inline(always)]
    pub fn argb0555(self) -> &'a mut W {
        self.variant(PIXEL0FORMAT_A::ARGB0555)
    }
    #[doc = "ARGB data is 0565"]
    #[inline(always)]
    pub fn argb0565(self) -> &'a mut W {
        self.variant(PIXEL0FORMAT_A::ARGB0565)
    }
    #[doc = "ARGB data is 0666"]
    #[inline(always)]
    pub fn argb0666(self) -> &'a mut W {
        self.variant(PIXEL0FORMAT_A::ARGB0666)
    }
    #[doc = "ARGB data is 0888"]
    #[inline(always)]
    pub fn argb0888(self) -> &'a mut W {
        self.variant(PIXEL0FORMAT_A::ARGB0888)
    }
    #[doc = "ARGB data is 5555"]
    #[inline(always)]
    pub fn argb5555(self) -> &'a mut W {
        self.variant(PIXEL0FORMAT_A::ARGB5555)
    }
    #[doc = "ARGB data is 6565"]
    #[inline(always)]
    pub fn argb6565(self) -> &'a mut W {
        self.variant(PIXEL0FORMAT_A::ARGB6565)
    }
    #[doc = "ARGB data is 6666"]
    #[inline(always)]
    pub fn argb6666(self) -> &'a mut W {
        self.variant(PIXEL0FORMAT_A::ARGB6666)
    }
    #[doc = "ARGB data is 8888"]
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut W {
        self.variant(PIXEL0FORMAT_A::ARGB8888)
    }
}
#[doc = "Source and Destination Pixel Color Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PIXEL1FORMAT_A {
    #[doc = "0: RGB data is 555"]
    RGB555 = 0,
    #[doc = "1: RGB data is 565"]
    RGB565 = 1,
    #[doc = "2: RGB data is 666"]
    RGB666 = 2,
    #[doc = "3: RGB data is 888"]
    RGB888 = 3,
}
impl From<PIXEL1FORMAT_A> for u8 {
    #[inline(always)]
    fn from(variant: PIXEL1FORMAT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PIXEL1FORMAT` reader - Source and Destination Pixel Color Format"]
pub type PIXEL1FORMAT_R = crate::FieldReader<u8, PIXEL1FORMAT_A>;
impl PIXEL1FORMAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIXEL1FORMAT_A {
        match self.bits {
            0 => PIXEL1FORMAT_A::RGB555,
            1 => PIXEL1FORMAT_A::RGB565,
            2 => PIXEL1FORMAT_A::RGB666,
            3 => PIXEL1FORMAT_A::RGB888,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RGB555`"]
    #[inline(always)]
    pub fn is_rgb555(&self) -> bool {
        *self == PIXEL1FORMAT_A::RGB555
    }
    #[doc = "Checks if the value of the field is `RGB565`"]
    #[inline(always)]
    pub fn is_rgb565(&self) -> bool {
        *self == PIXEL1FORMAT_A::RGB565
    }
    #[doc = "Checks if the value of the field is `RGB666`"]
    #[inline(always)]
    pub fn is_rgb666(&self) -> bool {
        *self == PIXEL1FORMAT_A::RGB666
    }
    #[doc = "Checks if the value of the field is `RGB888`"]
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == PIXEL1FORMAT_A::RGB888
    }
}
#[doc = "Field `PIXEL1FORMAT` writer - Source and Destination Pixel Color Format"]
pub type PIXEL1FORMAT_W<'a> =
    crate::FieldWriterSafe<'a, u32, TFTCOLORFORMAT_SPEC, u8, PIXEL1FORMAT_A, 2, 8>;
impl<'a> PIXEL1FORMAT_W<'a> {
    #[doc = "RGB data is 555"]
    #[inline(always)]
    pub fn rgb555(self) -> &'a mut W {
        self.variant(PIXEL1FORMAT_A::RGB555)
    }
    #[doc = "RGB data is 565"]
    #[inline(always)]
    pub fn rgb565(self) -> &'a mut W {
        self.variant(PIXEL1FORMAT_A::RGB565)
    }
    #[doc = "RGB data is 666"]
    #[inline(always)]
    pub fn rgb666(self) -> &'a mut W {
        self.variant(PIXEL1FORMAT_A::RGB666)
    }
    #[doc = "RGB data is 888"]
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut W {
        self.variant(PIXEL1FORMAT_A::RGB888)
    }
}
impl R {
    #[doc = "Bits 0:2 - Sprite Pixel Color Format"]
    #[inline(always)]
    pub fn pixel0format(&self) -> PIXEL0FORMAT_R {
        PIXEL0FORMAT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - Source and Destination Pixel Color Format"]
    #[inline(always)]
    pub fn pixel1format(&self) -> PIXEL1FORMAT_R {
        PIXEL1FORMAT_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sprite Pixel Color Format"]
    #[inline(always)]
    pub fn pixel0format(&mut self) -> PIXEL0FORMAT_W {
        PIXEL0FORMAT_W::new(self)
    }
    #[doc = "Bits 8:9 - Source and Destination Pixel Color Format"]
    #[inline(always)]
    pub fn pixel1format(&mut self) -> PIXEL1FORMAT_W {
        PIXEL1FORMAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Color Format Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tftcolorformat](index.html) module"]
pub struct TFTCOLORFORMAT_SPEC;
impl crate::RegisterSpec for TFTCOLORFORMAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tftcolorformat::R](R) reader structure"]
impl crate::Readable for TFTCOLORFORMAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tftcolorformat::W](W) writer structure"]
impl crate::Writable for TFTCOLORFORMAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TFTCOLORFORMAT to value 0"]
impl crate::Resettable for TFTCOLORFORMAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
