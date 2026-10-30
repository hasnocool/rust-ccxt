//! Plugin system: trait-based extensible plugins.

use async_trait::async_trait;

#[async_trait]
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    async fn run(&self, ctx: &PluginContext);
    fn enabled(&self) -> bool { true }
}

pub struct PluginContext {
    pub symbol: String,
    pub price: f64,
    pub sentiment: f64,
}

pub struct PluginRegistry {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self { Self { plugins: vec![] } }
    pub fn register(&mut self, p: Box<dyn Plugin>) { self.plugins.push(p); }
    pub async fn run_all(&self, ctx: &PluginContext) {
        for p in &self.plugins {
            if p.enabled() {
                p.run(ctx).await;
            }
        }
    }
    pub fn list(&self) -> Vec<&str> { self.plugins.iter().map(|p| p.name()).collect() }
}

/// Example plugin: console logger.
pub struct ConsoleLogger;
#[async_trait]
impl Plugin for ConsoleLogger {
    fn name(&self) -> &str { "console_logger" }
    async fn run(&self, ctx: &PluginContext) {
        println!("[plugin:console] {} price={:.2} sentiment={:.3}", ctx.symbol, ctx.price, ctx.sentiment);
    }
}
