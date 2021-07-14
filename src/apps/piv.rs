use crate::{Card, Result};

pub struct App {
    card: Card,
}

impl super::App for App {
    const RID: &'static [u8] = super::NIST_RID;
    const PIX: &'static [u8] = super::PIV_PIX;

    fn new(uuid: Option<[u8; 16]>) -> Result<Self> {
        Ok(Self {
            card: Self::connect(uuid)?,
        })
    }

    fn card(&mut self) -> &mut Card {
        &mut self.card
    }
}