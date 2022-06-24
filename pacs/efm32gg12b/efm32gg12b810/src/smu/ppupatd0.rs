#[doc = "Register `PPUPATD0` reader"]
pub struct R(crate::R<PPUPATD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPUPATD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPUPATD0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPUPATD0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPUPATD0` writer"]
pub struct W(crate::W<PPUPATD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPUPATD0_SPEC>;
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
impl From<crate::W<PPUPATD0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPUPATD0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACMP0` reader - Analog Comparator 0 access control bit"]
pub type ACMP0_R = crate::BitReader<bool>;
#[doc = "Field `ACMP0` writer - Analog Comparator 0 access control bit"]
pub type ACMP0_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 0>;
#[doc = "Field `ACMP1` reader - Analog Comparator 1 access control bit"]
pub type ACMP1_R = crate::BitReader<bool>;
#[doc = "Field `ACMP1` writer - Analog Comparator 1 access control bit"]
pub type ACMP1_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 1>;
#[doc = "Field `ACMP2` reader - Analog Comparator 2 access control bit"]
pub type ACMP2_R = crate::BitReader<bool>;
#[doc = "Field `ACMP2` writer - Analog Comparator 2 access control bit"]
pub type ACMP2_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 2>;
#[doc = "Field `ADC0` reader - Analog to Digital Converter 0 access control bit"]
pub type ADC0_R = crate::BitReader<bool>;
#[doc = "Field `ADC0` writer - Analog to Digital Converter 0 access control bit"]
pub type ADC0_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 3>;
#[doc = "Field `ADC1` reader - Analog to Digital Converter 0 access control bit"]
pub type ADC1_R = crate::BitReader<bool>;
#[doc = "Field `ADC1` writer - Analog to Digital Converter 0 access control bit"]
pub type ADC1_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 4>;
#[doc = "Field `CAN0` reader - CAN 0 access control bit"]
pub type CAN0_R = crate::BitReader<bool>;
#[doc = "Field `CAN0` writer - CAN 0 access control bit"]
pub type CAN0_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 5>;
#[doc = "Field `CAN1` reader - CAN 1 access control bit"]
pub type CAN1_R = crate::BitReader<bool>;
#[doc = "Field `CAN1` writer - CAN 1 access control bit"]
pub type CAN1_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 6>;
#[doc = "Field `CMU` reader - Clock Management Unit access control bit"]
pub type CMU_R = crate::BitReader<bool>;
#[doc = "Field `CMU` writer - Clock Management Unit access control bit"]
pub type CMU_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 7>;
#[doc = "Field `CRYOTIMER` reader - CRYOTIMER access control bit"]
pub type CRYOTIMER_R = crate::BitReader<bool>;
#[doc = "Field `CRYOTIMER` writer - CRYOTIMER access control bit"]
pub type CRYOTIMER_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 8>;
#[doc = "Field `CRYPTO0` reader - Advanced Encryption Standard Accelerator access control bit"]
pub type CRYPTO0_R = crate::BitReader<bool>;
#[doc = "Field `CRYPTO0` writer - Advanced Encryption Standard Accelerator access control bit"]
pub type CRYPTO0_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 9>;
#[doc = "Field `CSEN` reader - Capacitive touch sense module access control bit"]
pub type CSEN_R = crate::BitReader<bool>;
#[doc = "Field `CSEN` writer - Capacitive touch sense module access control bit"]
pub type CSEN_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 10>;
#[doc = "Field `VDAC0` reader - Digital to Analog Converter 0 access control bit"]
pub type VDAC0_R = crate::BitReader<bool>;
#[doc = "Field `VDAC0` writer - Digital to Analog Converter 0 access control bit"]
pub type VDAC0_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 11>;
#[doc = "Field `PRS` reader - Peripheral Reflex System access control bit"]
pub type PRS_R = crate::BitReader<bool>;
#[doc = "Field `PRS` writer - Peripheral Reflex System access control bit"]
pub type PRS_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 12>;
#[doc = "Field `EBI` reader - External Bus Interface access control bit"]
pub type EBI_R = crate::BitReader<bool>;
#[doc = "Field `EBI` writer - External Bus Interface access control bit"]
pub type EBI_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 13>;
#[doc = "Field `EMU` reader - Energy Management Unit access control bit"]
pub type EMU_R = crate::BitReader<bool>;
#[doc = "Field `EMU` writer - Energy Management Unit access control bit"]
pub type EMU_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 14>;
#[doc = "Field `FPUEH` reader - FPU Exception Handler access control bit"]
pub type FPUEH_R = crate::BitReader<bool>;
#[doc = "Field `FPUEH` writer - FPU Exception Handler access control bit"]
pub type FPUEH_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 15>;
#[doc = "Field `GPCRC` reader - General Purpose CRC access control bit"]
pub type GPCRC_R = crate::BitReader<bool>;
#[doc = "Field `GPCRC` writer - General Purpose CRC access control bit"]
pub type GPCRC_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 16>;
#[doc = "Field `GPIO` reader - General purpose Input/Output access control bit"]
pub type GPIO_R = crate::BitReader<bool>;
#[doc = "Field `GPIO` writer - General purpose Input/Output access control bit"]
pub type GPIO_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 17>;
#[doc = "Field `I2C0` reader - I2C 0 access control bit"]
pub type I2C0_R = crate::BitReader<bool>;
#[doc = "Field `I2C0` writer - I2C 0 access control bit"]
pub type I2C0_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 18>;
#[doc = "Field `I2C1` reader - I2C 1 access control bit"]
pub type I2C1_R = crate::BitReader<bool>;
#[doc = "Field `I2C1` writer - I2C 1 access control bit"]
pub type I2C1_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 19>;
#[doc = "Field `IDAC0` reader - Current Digital to Analog Converter 0 access control bit"]
pub type IDAC0_R = crate::BitReader<bool>;
#[doc = "Field `IDAC0` writer - Current Digital to Analog Converter 0 access control bit"]
pub type IDAC0_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 20>;
#[doc = "Field `MSC` reader - Memory System Controller access control bit"]
pub type MSC_R = crate::BitReader<bool>;
#[doc = "Field `MSC` writer - Memory System Controller access control bit"]
pub type MSC_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 21>;
#[doc = "Field `LCD` reader - Liquid Crystal Display Controller access control bit"]
pub type LCD_R = crate::BitReader<bool>;
#[doc = "Field `LCD` writer - Liquid Crystal Display Controller access control bit"]
pub type LCD_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 22>;
#[doc = "Field `LDMA` reader - Linked Direct Memory Access Controller access control bit"]
pub type LDMA_R = crate::BitReader<bool>;
#[doc = "Field `LDMA` writer - Linked Direct Memory Access Controller access control bit"]
pub type LDMA_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 23>;
#[doc = "Field `LESENSE` reader - Low Energy Sensor Interface access control bit"]
pub type LESENSE_R = crate::BitReader<bool>;
#[doc = "Field `LESENSE` writer - Low Energy Sensor Interface access control bit"]
pub type LESENSE_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 24>;
#[doc = "Field `LETIMER0` reader - Low Energy Timer 0 access control bit"]
pub type LETIMER0_R = crate::BitReader<bool>;
#[doc = "Field `LETIMER0` writer - Low Energy Timer 0 access control bit"]
pub type LETIMER0_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 25>;
#[doc = "Field `LETIMER1` reader - Low Energy Timer 1 access control bit"]
pub type LETIMER1_R = crate::BitReader<bool>;
#[doc = "Field `LETIMER1` writer - Low Energy Timer 1 access control bit"]
pub type LETIMER1_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 26>;
#[doc = "Field `LEUART0` reader - Low Energy UART 0 access control bit"]
pub type LEUART0_R = crate::BitReader<bool>;
#[doc = "Field `LEUART0` writer - Low Energy UART 0 access control bit"]
pub type LEUART0_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 27>;
#[doc = "Field `LEUART1` reader - Low Energy UART 1 access control bit"]
pub type LEUART1_R = crate::BitReader<bool>;
#[doc = "Field `LEUART1` writer - Low Energy UART 1 access control bit"]
pub type LEUART1_W<'a> = crate::BitWriter<'a, u32, PPUPATD0_SPEC, bool, 28>;
impl R {
    #[doc = "Bit 0 - Analog Comparator 0 access control bit"]
    #[inline(always)]
    pub fn acmp0(&self) -> ACMP0_R {
        ACMP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog Comparator 1 access control bit"]
    #[inline(always)]
    pub fn acmp1(&self) -> ACMP1_R {
        ACMP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog Comparator 2 access control bit"]
    #[inline(always)]
    pub fn acmp2(&self) -> ACMP2_R {
        ACMP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog to Digital Converter 0 access control bit"]
    #[inline(always)]
    pub fn adc0(&self) -> ADC0_R {
        ADC0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog to Digital Converter 0 access control bit"]
    #[inline(always)]
    pub fn adc1(&self) -> ADC1_R {
        ADC1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CAN 0 access control bit"]
    #[inline(always)]
    pub fn can0(&self) -> CAN0_R {
        CAN0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CAN 1 access control bit"]
    #[inline(always)]
    pub fn can1(&self) -> CAN1_R {
        CAN1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock Management Unit access control bit"]
    #[inline(always)]
    pub fn cmu(&self) -> CMU_R {
        CMU_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CRYOTIMER access control bit"]
    #[inline(always)]
    pub fn cryotimer(&self) -> CRYOTIMER_R {
        CRYOTIMER_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Advanced Encryption Standard Accelerator access control bit"]
    #[inline(always)]
    pub fn crypto0(&self) -> CRYPTO0_R {
        CRYPTO0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capacitive touch sense module access control bit"]
    #[inline(always)]
    pub fn csen(&self) -> CSEN_R {
        CSEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Digital to Analog Converter 0 access control bit"]
    #[inline(always)]
    pub fn vdac0(&self) -> VDAC0_R {
        VDAC0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Peripheral Reflex System access control bit"]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - External Bus Interface access control bit"]
    #[inline(always)]
    pub fn ebi(&self) -> EBI_R {
        EBI_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Energy Management Unit access control bit"]
    #[inline(always)]
    pub fn emu(&self) -> EMU_R {
        EMU_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - FPU Exception Handler access control bit"]
    #[inline(always)]
    pub fn fpueh(&self) -> FPUEH_R {
        FPUEH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - General Purpose CRC access control bit"]
    #[inline(always)]
    pub fn gpcrc(&self) -> GPCRC_R {
        GPCRC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - General purpose Input/Output access control bit"]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I2C 0 access control bit"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - I2C 1 access control bit"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Current Digital to Analog Converter 0 access control bit"]
    #[inline(always)]
    pub fn idac0(&self) -> IDAC0_R {
        IDAC0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Memory System Controller access control bit"]
    #[inline(always)]
    pub fn msc(&self) -> MSC_R {
        MSC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Liquid Crystal Display Controller access control bit"]
    #[inline(always)]
    pub fn lcd(&self) -> LCD_R {
        LCD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Linked Direct Memory Access Controller access control bit"]
    #[inline(always)]
    pub fn ldma(&self) -> LDMA_R {
        LDMA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Low Energy Sensor Interface access control bit"]
    #[inline(always)]
    pub fn lesense(&self) -> LESENSE_R {
        LESENSE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Low Energy Timer 0 access control bit"]
    #[inline(always)]
    pub fn letimer0(&self) -> LETIMER0_R {
        LETIMER0_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Low Energy Timer 1 access control bit"]
    #[inline(always)]
    pub fn letimer1(&self) -> LETIMER1_R {
        LETIMER1_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Low Energy UART 0 access control bit"]
    #[inline(always)]
    pub fn leuart0(&self) -> LEUART0_R {
        LEUART0_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Low Energy UART 1 access control bit"]
    #[inline(always)]
    pub fn leuart1(&self) -> LEUART1_R {
        LEUART1_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Comparator 0 access control bit"]
    #[inline(always)]
    pub fn acmp0(&mut self) -> ACMP0_W {
        ACMP0_W::new(self)
    }
    #[doc = "Bit 1 - Analog Comparator 1 access control bit"]
    #[inline(always)]
    pub fn acmp1(&mut self) -> ACMP1_W {
        ACMP1_W::new(self)
    }
    #[doc = "Bit 2 - Analog Comparator 2 access control bit"]
    #[inline(always)]
    pub fn acmp2(&mut self) -> ACMP2_W {
        ACMP2_W::new(self)
    }
    #[doc = "Bit 3 - Analog to Digital Converter 0 access control bit"]
    #[inline(always)]
    pub fn adc0(&mut self) -> ADC0_W {
        ADC0_W::new(self)
    }
    #[doc = "Bit 4 - Analog to Digital Converter 0 access control bit"]
    #[inline(always)]
    pub fn adc1(&mut self) -> ADC1_W {
        ADC1_W::new(self)
    }
    #[doc = "Bit 5 - CAN 0 access control bit"]
    #[inline(always)]
    pub fn can0(&mut self) -> CAN0_W {
        CAN0_W::new(self)
    }
    #[doc = "Bit 6 - CAN 1 access control bit"]
    #[inline(always)]
    pub fn can1(&mut self) -> CAN1_W {
        CAN1_W::new(self)
    }
    #[doc = "Bit 7 - Clock Management Unit access control bit"]
    #[inline(always)]
    pub fn cmu(&mut self) -> CMU_W {
        CMU_W::new(self)
    }
    #[doc = "Bit 8 - CRYOTIMER access control bit"]
    #[inline(always)]
    pub fn cryotimer(&mut self) -> CRYOTIMER_W {
        CRYOTIMER_W::new(self)
    }
    #[doc = "Bit 9 - Advanced Encryption Standard Accelerator access control bit"]
    #[inline(always)]
    pub fn crypto0(&mut self) -> CRYPTO0_W {
        CRYPTO0_W::new(self)
    }
    #[doc = "Bit 10 - Capacitive touch sense module access control bit"]
    #[inline(always)]
    pub fn csen(&mut self) -> CSEN_W {
        CSEN_W::new(self)
    }
    #[doc = "Bit 11 - Digital to Analog Converter 0 access control bit"]
    #[inline(always)]
    pub fn vdac0(&mut self) -> VDAC0_W {
        VDAC0_W::new(self)
    }
    #[doc = "Bit 12 - Peripheral Reflex System access control bit"]
    #[inline(always)]
    pub fn prs(&mut self) -> PRS_W {
        PRS_W::new(self)
    }
    #[doc = "Bit 13 - External Bus Interface access control bit"]
    #[inline(always)]
    pub fn ebi(&mut self) -> EBI_W {
        EBI_W::new(self)
    }
    #[doc = "Bit 14 - Energy Management Unit access control bit"]
    #[inline(always)]
    pub fn emu(&mut self) -> EMU_W {
        EMU_W::new(self)
    }
    #[doc = "Bit 15 - FPU Exception Handler access control bit"]
    #[inline(always)]
    pub fn fpueh(&mut self) -> FPUEH_W {
        FPUEH_W::new(self)
    }
    #[doc = "Bit 16 - General Purpose CRC access control bit"]
    #[inline(always)]
    pub fn gpcrc(&mut self) -> GPCRC_W {
        GPCRC_W::new(self)
    }
    #[doc = "Bit 17 - General purpose Input/Output access control bit"]
    #[inline(always)]
    pub fn gpio(&mut self) -> GPIO_W {
        GPIO_W::new(self)
    }
    #[doc = "Bit 18 - I2C 0 access control bit"]
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2C0_W {
        I2C0_W::new(self)
    }
    #[doc = "Bit 19 - I2C 1 access control bit"]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2C1_W {
        I2C1_W::new(self)
    }
    #[doc = "Bit 20 - Current Digital to Analog Converter 0 access control bit"]
    #[inline(always)]
    pub fn idac0(&mut self) -> IDAC0_W {
        IDAC0_W::new(self)
    }
    #[doc = "Bit 21 - Memory System Controller access control bit"]
    #[inline(always)]
    pub fn msc(&mut self) -> MSC_W {
        MSC_W::new(self)
    }
    #[doc = "Bit 22 - Liquid Crystal Display Controller access control bit"]
    #[inline(always)]
    pub fn lcd(&mut self) -> LCD_W {
        LCD_W::new(self)
    }
    #[doc = "Bit 23 - Linked Direct Memory Access Controller access control bit"]
    #[inline(always)]
    pub fn ldma(&mut self) -> LDMA_W {
        LDMA_W::new(self)
    }
    #[doc = "Bit 24 - Low Energy Sensor Interface access control bit"]
    #[inline(always)]
    pub fn lesense(&mut self) -> LESENSE_W {
        LESENSE_W::new(self)
    }
    #[doc = "Bit 25 - Low Energy Timer 0 access control bit"]
    #[inline(always)]
    pub fn letimer0(&mut self) -> LETIMER0_W {
        LETIMER0_W::new(self)
    }
    #[doc = "Bit 26 - Low Energy Timer 1 access control bit"]
    #[inline(always)]
    pub fn letimer1(&mut self) -> LETIMER1_W {
        LETIMER1_W::new(self)
    }
    #[doc = "Bit 27 - Low Energy UART 0 access control bit"]
    #[inline(always)]
    pub fn leuart0(&mut self) -> LEUART0_W {
        LEUART0_W::new(self)
    }
    #[doc = "Bit 28 - Low Energy UART 1 access control bit"]
    #[inline(always)]
    pub fn leuart1(&mut self) -> LEUART1_W {
        LEUART1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PPU Privilege Access Type Descriptor 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppupatd0](index.html) module"]
pub struct PPUPATD0_SPEC;
impl crate::RegisterSpec for PPUPATD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppupatd0::R](R) reader structure"]
impl crate::Readable for PPUPATD0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppupatd0::W](W) writer structure"]
impl crate::Writable for PPUPATD0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PPUPATD0 to value 0"]
impl crate::Resettable for PPUPATD0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
