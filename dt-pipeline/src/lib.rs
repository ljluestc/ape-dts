pub mod base_pipeline;
pub mod filter_processor;
pub mod http_server_pipeline;
pub mod lua_processor;
pub mod processor_chain;
pub mod router_processor;

use async_trait::async_trait;

#[async_trait]
pub trait Pipeline {
    async fn start(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn stop(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}
