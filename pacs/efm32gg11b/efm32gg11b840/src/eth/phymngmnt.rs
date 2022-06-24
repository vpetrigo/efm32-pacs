#[doc = "Register `PHYMNGMNT` reader"]
pub struct R(crate::R<PHYMNGMNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHYMNGMNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHYMNGMNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHYMNGMNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PHYMNGMNT` writer"]
pub struct W(crate::W<PHYMNGMNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHYMNGMNT_SPEC>;
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
impl From<crate::W<PHYMNGMNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHYMNGMNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHYRWDATA` reader - PHY read write data"]
pub type PHYRWDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PHYRWDATA` writer - PHY read write data"]
pub type PHYRWDATA_W<'a> = crate::FieldWriter<'a, u32, PHYMNGMNT_SPEC, u16, u16, 16, 0>;
#[doc = "Field `WRITE10` reader - Must be written with 10."]
pub type WRITE10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRITE10` writer - Must be written with 10."]
pub type WRITE10_W<'a> = crate::FieldWriter<'a, u32, PHYMNGMNT_SPEC, u8, u8, 2, 16>;
#[doc = "Field `REGADDR` reader - Register address - specifies the register in the PHY to access."]
pub type REGADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REGADDR` writer - Register address - specifies the register in the PHY to access."]
pub type REGADDR_W<'a> = crate::FieldWriter<'a, u32, PHYMNGMNT_SPEC, u8, u8, 5, 18>;
#[doc = "Field `PHYADDR` reader - PHY address."]
pub type PHYADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PHYADDR` writer - PHY address."]
pub type PHYADDR_W<'a> = crate::FieldWriter<'a, u32, PHYMNGMNT_SPEC, u8, u8, 5, 23>;
#[doc = "Field `OPERATION` reader - Operation. For a Clause 45 frame: 00 is an addr, 01 is a write, 10 is a post read increment, 11 is a read frame. For a Clause 22 frame: 10 is a read, 01 is a write."]
pub type OPERATION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPERATION` writer - Operation. For a Clause 45 frame: 00 is an addr, 01 is a write, 10 is a post read increment, 11 is a read frame. For a Clause 22 frame: 10 is a read, 01 is a write."]
pub type OPERATION_W<'a> = crate::FieldWriter<'a, u32, PHYMNGMNT_SPEC, u8, u8, 2, 28>;
#[doc = "Field `WRITE1` reader - Must be written to 1 for a valid Clause 22 frame and to 0 for a valid Clause 45 frame."]
pub type WRITE1_R = crate::BitReader<bool>;
#[doc = "Field `WRITE1` writer - Must be written to 1 for a valid Clause 22 frame and to 0 for a valid Clause 45 frame."]
pub type WRITE1_W<'a> = crate::BitWriter<'a, u32, PHYMNGMNT_SPEC, bool, 30>;
#[doc = "Field `WRITE0` reader - Must be written with 0."]
pub type WRITE0_R = crate::BitReader<bool>;
#[doc = "Field `WRITE0` writer - Must be written with 0."]
pub type WRITE0_W<'a> = crate::BitWriter<'a, u32, PHYMNGMNT_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:15 - PHY read write data"]
    #[inline(always)]
    pub fn phyrwdata(&self) -> PHYRWDATA_R {
        PHYRWDATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - Must be written with 10."]
    #[inline(always)]
    pub fn write10(&self) -> WRITE10_R {
        WRITE10_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:22 - Register address - specifies the register in the PHY to access."]
    #[inline(always)]
    pub fn regaddr(&self) -> REGADDR_R {
        REGADDR_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:27 - PHY address."]
    #[inline(always)]
    pub fn phyaddr(&self) -> PHYADDR_R {
        PHYADDR_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29 - Operation. For a Clause 45 frame: 00 is an addr, 01 is a write, 10 is a post read increment, 11 is a read frame. For a Clause 22 frame: 10 is a read, 01 is a write."]
    #[inline(always)]
    pub fn operation(&self) -> OPERATION_R {
        OPERATION_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Must be written to 1 for a valid Clause 22 frame and to 0 for a valid Clause 45 frame."]
    #[inline(always)]
    pub fn write1(&self) -> WRITE1_R {
        WRITE1_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Must be written with 0."]
    #[inline(always)]
    pub fn write0(&self) -> WRITE0_R {
        WRITE0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - PHY read write data"]
    #[inline(always)]
    pub fn phyrwdata(&mut self) -> PHYRWDATA_W {
        PHYRWDATA_W::new(self)
    }
    #[doc = "Bits 16:17 - Must be written with 10."]
    #[inline(always)]
    pub fn write10(&mut self) -> WRITE10_W {
        WRITE10_W::new(self)
    }
    #[doc = "Bits 18:22 - Register address - specifies the register in the PHY to access."]
    #[inline(always)]
    pub fn regaddr(&mut self) -> REGADDR_W {
        REGADDR_W::new(self)
    }
    #[doc = "Bits 23:27 - PHY address."]
    #[inline(always)]
    pub fn phyaddr(&mut self) -> PHYADDR_W {
        PHYADDR_W::new(self)
    }
    #[doc = "Bits 28:29 - Operation. For a Clause 45 frame: 00 is an addr, 01 is a write, 10 is a post read increment, 11 is a read frame. For a Clause 22 frame: 10 is a read, 01 is a write."]
    #[inline(always)]
    pub fn operation(&mut self) -> OPERATION_W {
        OPERATION_W::new(self)
    }
    #[doc = "Bit 30 - Must be written to 1 for a valid Clause 22 frame and to 0 for a valid Clause 45 frame."]
    #[inline(always)]
    pub fn write1(&mut self) -> WRITE1_W {
        WRITE1_W::new(self)
    }
    #[doc = "Bit 31 - Must be written with 0."]
    #[inline(always)]
    pub fn write0(&mut self) -> WRITE0_W {
        WRITE0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY management register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phymngmnt](index.html) module"]
pub struct PHYMNGMNT_SPEC;
impl crate::RegisterSpec for PHYMNGMNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [phymngmnt::R](R) reader structure"]
impl crate::Readable for PHYMNGMNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phymngmnt::W](W) writer structure"]
impl crate::Writable for PHYMNGMNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PHYMNGMNT to value 0"]
impl crate::Resettable for PHYMNGMNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
