pub trait Database {
    async fn create_hero(&mut self, hero: shared::CreateHeroParams) -> Result<(), eyre::Report>;
    async fn update_hero_stats(
        &mut self,
        hero: shared::UpdateHeroStatsParams,
    ) -> Result<(), eyre::Report>;
    async fn update_hero_level(&mut self, id: i64, level: i64) -> Result<(), eyre::Report>;
    async fn hero_by_id(&mut self, id: i64) -> Result<Option<shared::Hero>, eyre::Report>;
    async fn hero_by_rfid(&mut self, rfid: &str) -> Result<Option<shared::Hero>, eyre::Report>;
    async fn create_match(
        &mut self,
        create_match: shared::CreateMatchParams,
    ) -> Result<(), eyre::Report>;
}
