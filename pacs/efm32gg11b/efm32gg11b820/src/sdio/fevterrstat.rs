#[doc = "Register `FEVTERRSTAT` reader"]
pub struct R(crate::R<FEVTERRSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEVTERRSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEVTERRSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEVTERRSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FEVTERRSTAT` writer"]
pub struct W(crate::W<FEVTERRSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FEVTERRSTAT_SPEC>;
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
impl From<crate::W<FEVTERRSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FEVTERRSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AC12NEX` reader - Force Event for Command Not Issued By Auto CM12 Not Executed"]
pub type AC12NEX_R = crate::BitReader<bool>;
#[doc = "Field `AC12NEX` writer - Force Event for Command Not Issued By Auto CM12 Not Executed"]
pub type AC12NEX_W<'a> = crate::BitWriter<'a, u32, FEVTERRSTAT_SPEC, bool, 0>;
#[doc = "Field `AC12TOE` reader - Force Event for Auto CMD Timeout Error"]
pub type AC12TOE_R = crate::BitReader<bool>;
#[doc = "Field `AC12TOE` writer - Force Event for Auto CMD Timeout Error"]
pub type AC12TOE_W<'a> = crate::BitWriter<'a, u32, FEVTERRSTAT_SPEC, bool, 1>;
#[doc = "Field `AC12CRCE` reader - Force Event for Auto CMD CRC Error"]
pub type AC12CRCE_R = crate::BitReader<bool>;
#[doc = "Field `AC12CRCE` writer - Force Event for Auto CMD CRC Error"]
pub type AC12CRCE_W<'a> = crate::BitWriter<'a, u32, FEVTERRSTAT_SPEC, bool, 2>;
#[doc = "Field `AC12EBE` reader - Force Event for Auto CMD End Bit Error"]
pub type AC12EBE_R = crate::BitReader<bool>;
#[doc = "Field `AC12EBE` writer - Force Event for Auto CMD End Bit Error"]
pub type AC12EBE_W<'a> = crate::BitWriter<'a, u32, FEVTERRSTAT_SPEC, bool, 3>;
#[doc = "Field `AC12INDXE` reader - Force Event for Auto CMD Index Error"]
pub type AC12INDXE_R = crate::BitReader<bool>;
#[doc = "Field `AC12INDXE` writer - Force Event for Auto CMD Index Error"]
pub type AC12INDXE_W<'a> = crate::BitWriter<'a, u32, FEVTERRSTAT_SPEC, bool, 4>;
#[doc = "Field `CNIBAC12E` reader - Force Event for Command Not Issued By Auto CMD12 Error"]
pub type CNIBAC12E_R = crate::BitReader<bool>;
#[doc = "Field `CNIBAC12E` writer - Force Event for Command Not Issued By Auto CMD12 Error"]
pub type CNIBAC12E_W<'a> = crate::BitWriter<'a, u32, FEVTERRSTAT_SPEC, bool, 7>;
#[doc = "Field `CMDTOE` reader - Force Event for Command Timeout Error"]
pub type CMDTOE_R = crate::BitReader<bool>;
#[doc = "Field `CMDTOE` writer - Force Event for Command Timeout Error"]
pub type CMDTOE_W<'a> = crate::BitWriter<'a, u32, FEVTERRSTAT_SPEC, bool, 16>;
#[doc = "Field `CMDCRCE` reader - Force Event for Command CRC Error"]
pub type CMDCRCE_R = crate::BitReader<bool>;
#[doc = "Field `CMDCRCE` writer - Force Event for Command CRC Error"]
pub type CMDCRCE_W<'a> = crate::BitWriter<'a, u32, FEVTERRSTAT_SPEC, bool, 17>;
#[doc = "Field `CMDEBE` reader - Force Event for Command End Bit Error"]
pub type CMDEBE_R = crate::BitReader<bool>;
#[doc = "Field `CMDEBE` writer - Force Event for Command End Bit Error"]
pub type CMDEBE_W<'a> = crate::BitWriter<'a, u32, FEVTERRSTAT_SPEC, bool, 18>;
#[doc = "Field `CMDINDXE` reader - Force Event for Command Index Error"]
pub type CMDINDXE_R = crate::BitReader<bool>;
#[doc = "Field `CMDINDXE` writer - Force Event for Command Index Error"]
pub type CMDINDXE_W<'a> = crate::BitWriter<'a, u32, FEVTERRSTAT_SPEC, bool, 19>;
#[doc = "Field `DATTOE` reader - Force Event for Data Timeout Error"]
pub type DATTOE_R = crate::BitReader<bool>;
#[doc = "Field `DATTOE` writer - Force Event for Data Timeout Error"]
pub type DATTOE_W<'a> = crate::BitWriter<'a, u32, FEVTERRSTAT_SPEC, bool, 20>;
#[doc = "Field `DATCRCE` reader - Force Event for Data CRC Error"]
pub type DATCRCE_R = crate::BitReader<bool>;
#[doc = "Field `DATCRCE` writer - Force Event for Data CRC Error"]
pub type DATCRCE_W<'a> = crate::BitWriter<'a, u32, FEVTERRSTAT_SPEC, bool, 21>;
#[doc = "Field `DATEBE` reader - Force Event for Data End Bit Error"]
pub type DATEBE_R = crate::BitReader<bool>;
#[doc = "Field `DATEBE` writer - Force Event for Data End Bit Error"]
pub type DATEBE_W<'a> = crate::BitWriter<'a, u32, FEVTERRSTAT_SPEC, bool, 22>;
#[doc = "Field `CURLIMITE` reader - Force Event for Current Limit Error"]
pub type CURLIMITE_R = crate::BitReader<bool>;
#[doc = "Field `CURLIMITE` writer - Force Event for Current Limit Error"]
pub type CURLIMITE_W<'a> = crate::BitWriter<'a, u32, FEVTERRSTAT_SPEC, bool, 23>;
#[doc = "Field `AC12E` reader - Force Event for Auto CMD Error"]
pub type AC12E_R = crate::BitReader<bool>;
#[doc = "Field `AC12E` writer - Force Event for Auto CMD Error"]
pub type AC12E_W<'a> = crate::BitWriter<'a, u32, FEVTERRSTAT_SPEC, bool, 24>;
#[doc = "Field `ADMAE` reader - Force Event for ADMA Error"]
pub type ADMAE_R = crate::BitReader<bool>;
#[doc = "Field `ADMAE` writer - Force Event for ADMA Error"]
pub type ADMAE_W<'a> = crate::BitWriter<'a, u32, FEVTERRSTAT_SPEC, bool, 25>;
#[doc = "Field `TUNINGE` reader - Force Event for Tuning Errro"]
pub type TUNINGE_R = crate::BitReader<bool>;
#[doc = "Field `VENSPECE` reader - Force Event for Vendox Specific Error Status"]
pub type VENSPECE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Force Event for Command Not Issued By Auto CM12 Not Executed"]
    #[inline(always)]
    pub fn ac12nex(&self) -> AC12NEX_R {
        AC12NEX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force Event for Auto CMD Timeout Error"]
    #[inline(always)]
    pub fn ac12toe(&self) -> AC12TOE_R {
        AC12TOE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force Event for Auto CMD CRC Error"]
    #[inline(always)]
    pub fn ac12crce(&self) -> AC12CRCE_R {
        AC12CRCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force Event for Auto CMD End Bit Error"]
    #[inline(always)]
    pub fn ac12ebe(&self) -> AC12EBE_R {
        AC12EBE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Force Event for Auto CMD Index Error"]
    #[inline(always)]
    pub fn ac12indxe(&self) -> AC12INDXE_R {
        AC12INDXE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Force Event for Command Not Issued By Auto CMD12 Error"]
    #[inline(always)]
    pub fn cnibac12e(&self) -> CNIBAC12E_R {
        CNIBAC12E_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Force Event for Command Timeout Error"]
    #[inline(always)]
    pub fn cmdtoe(&self) -> CMDTOE_R {
        CMDTOE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Force Event for Command CRC Error"]
    #[inline(always)]
    pub fn cmdcrce(&self) -> CMDCRCE_R {
        CMDCRCE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Force Event for Command End Bit Error"]
    #[inline(always)]
    pub fn cmdebe(&self) -> CMDEBE_R {
        CMDEBE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Force Event for Command Index Error"]
    #[inline(always)]
    pub fn cmdindxe(&self) -> CMDINDXE_R {
        CMDINDXE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Force Event for Data Timeout Error"]
    #[inline(always)]
    pub fn dattoe(&self) -> DATTOE_R {
        DATTOE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Force Event for Data CRC Error"]
    #[inline(always)]
    pub fn datcrce(&self) -> DATCRCE_R {
        DATCRCE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Force Event for Data End Bit Error"]
    #[inline(always)]
    pub fn datebe(&self) -> DATEBE_R {
        DATEBE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Force Event for Current Limit Error"]
    #[inline(always)]
    pub fn curlimite(&self) -> CURLIMITE_R {
        CURLIMITE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Force Event for Auto CMD Error"]
    #[inline(always)]
    pub fn ac12e(&self) -> AC12E_R {
        AC12E_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Force Event for ADMA Error"]
    #[inline(always)]
    pub fn admae(&self) -> ADMAE_R {
        ADMAE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Force Event for Tuning Errro"]
    #[inline(always)]
    pub fn tuninge(&self) -> TUNINGE_R {
        TUNINGE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Force Event for Vendox Specific Error Status"]
    #[inline(always)]
    pub fn venspece(&self) -> VENSPECE_R {
        VENSPECE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Force Event for Command Not Issued By Auto CM12 Not Executed"]
    #[inline(always)]
    pub fn ac12nex(&mut self) -> AC12NEX_W {
        AC12NEX_W::new(self)
    }
    #[doc = "Bit 1 - Force Event for Auto CMD Timeout Error"]
    #[inline(always)]
    pub fn ac12toe(&mut self) -> AC12TOE_W {
        AC12TOE_W::new(self)
    }
    #[doc = "Bit 2 - Force Event for Auto CMD CRC Error"]
    #[inline(always)]
    pub fn ac12crce(&mut self) -> AC12CRCE_W {
        AC12CRCE_W::new(self)
    }
    #[doc = "Bit 3 - Force Event for Auto CMD End Bit Error"]
    #[inline(always)]
    pub fn ac12ebe(&mut self) -> AC12EBE_W {
        AC12EBE_W::new(self)
    }
    #[doc = "Bit 4 - Force Event for Auto CMD Index Error"]
    #[inline(always)]
    pub fn ac12indxe(&mut self) -> AC12INDXE_W {
        AC12INDXE_W::new(self)
    }
    #[doc = "Bit 7 - Force Event for Command Not Issued By Auto CMD12 Error"]
    #[inline(always)]
    pub fn cnibac12e(&mut self) -> CNIBAC12E_W {
        CNIBAC12E_W::new(self)
    }
    #[doc = "Bit 16 - Force Event for Command Timeout Error"]
    #[inline(always)]
    pub fn cmdtoe(&mut self) -> CMDTOE_W {
        CMDTOE_W::new(self)
    }
    #[doc = "Bit 17 - Force Event for Command CRC Error"]
    #[inline(always)]
    pub fn cmdcrce(&mut self) -> CMDCRCE_W {
        CMDCRCE_W::new(self)
    }
    #[doc = "Bit 18 - Force Event for Command End Bit Error"]
    #[inline(always)]
    pub fn cmdebe(&mut self) -> CMDEBE_W {
        CMDEBE_W::new(self)
    }
    #[doc = "Bit 19 - Force Event for Command Index Error"]
    #[inline(always)]
    pub fn cmdindxe(&mut self) -> CMDINDXE_W {
        CMDINDXE_W::new(self)
    }
    #[doc = "Bit 20 - Force Event for Data Timeout Error"]
    #[inline(always)]
    pub fn dattoe(&mut self) -> DATTOE_W {
        DATTOE_W::new(self)
    }
    #[doc = "Bit 21 - Force Event for Data CRC Error"]
    #[inline(always)]
    pub fn datcrce(&mut self) -> DATCRCE_W {
        DATCRCE_W::new(self)
    }
    #[doc = "Bit 22 - Force Event for Data End Bit Error"]
    #[inline(always)]
    pub fn datebe(&mut self) -> DATEBE_W {
        DATEBE_W::new(self)
    }
    #[doc = "Bit 23 - Force Event for Current Limit Error"]
    #[inline(always)]
    pub fn curlimite(&mut self) -> CURLIMITE_W {
        CURLIMITE_W::new(self)
    }
    #[doc = "Bit 24 - Force Event for Auto CMD Error"]
    #[inline(always)]
    pub fn ac12e(&mut self) -> AC12E_W {
        AC12E_W::new(self)
    }
    #[doc = "Bit 25 - Force Event for ADMA Error"]
    #[inline(always)]
    pub fn admae(&mut self) -> ADMAE_W {
        ADMAE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Force Event Register for Auto CMD Error Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fevterrstat](index.html) module"]
pub struct FEVTERRSTAT_SPEC;
impl crate::RegisterSpec for FEVTERRSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fevterrstat::R](R) reader structure"]
impl crate::Readable for FEVTERRSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fevterrstat::W](W) writer structure"]
impl crate::Writable for FEVTERRSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FEVTERRSTAT to value 0"]
impl crate::Resettable for FEVTERRSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
