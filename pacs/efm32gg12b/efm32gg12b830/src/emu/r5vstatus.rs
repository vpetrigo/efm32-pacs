#[doc = "Register `R5VSTATUS` reader"]
pub struct R(crate::R<R5VSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R5VSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R5VSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R5VSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VREGIDET` reader - VREGI Detected"]
pub type VREGIDET_R = crate::BitReader<bool>;
#[doc = "Field `VBUSDET` reader - USB VBUS Detected"]
pub type VBUSDET_R = crate::BitReader<bool>;
#[doc = "Field `VREGODET` reader - VREGO Detected"]
pub type VREGODET_R = crate::BitReader<bool>;
#[doc = "Field `VBUSGTVREGI` reader - Output of the Supply Comparator Between VBUS and VREGI"]
pub type VBUSGTVREGI_R = crate::BitReader<bool>;
#[doc = "Field `LDODROPOUTDET` reader - Regulator Dropout Detection"]
pub type LDODROPOUTDET_R = crate::BitReader<bool>;
#[doc = "Field `COLDSTART` reader - Indicates If the Regulator is Going Through a Cold Start"]
pub type COLDSTART_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - VREGI Detected"]
    #[inline(always)]
    pub fn vregidet(&self) -> VREGIDET_R {
        VREGIDET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB VBUS Detected"]
    #[inline(always)]
    pub fn vbusdet(&self) -> VBUSDET_R {
        VBUSDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VREGO Detected"]
    #[inline(always)]
    pub fn vregodet(&self) -> VREGODET_R {
        VREGODET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output of the Supply Comparator Between VBUS and VREGI"]
    #[inline(always)]
    pub fn vbusgtvregi(&self) -> VBUSGTVREGI_R {
        VBUSGTVREGI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Regulator Dropout Detection"]
    #[inline(always)]
    pub fn ldodropoutdet(&self) -> LDODROPOUTDET_R {
        LDODROPOUTDET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates If the Regulator is Going Through a Cold Start"]
    #[inline(always)]
    pub fn coldstart(&self) -> COLDSTART_R {
        COLDSTART_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "5V Detector Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r5vstatus](index.html) module"]
pub struct R5VSTATUS_SPEC;
impl crate::RegisterSpec for R5VSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r5vstatus::R](R) reader structure"]
impl crate::Readable for R5VSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets R5VSTATUS to value 0x20"]
impl crate::Resettable for R5VSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
