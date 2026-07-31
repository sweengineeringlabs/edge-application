//! Integration test verifying that edge-pipeline is wired into edge-domain.
//!
//! @covers edge-domain pipeline feature integration

#![cfg(feature = "pipeline")]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use async_trait::async_trait;
use edge_application::{
    ContextMutationRequest, PipelineBuilder, PipelineConfig, PipelineEmptinessRequest,
    PipelineError, PipelineRunRequest, PipelineSvc, Step, StepCountRequest, StepNameRequest,
    StepNameResponse,
};
use edge_pipeline::PipelineConfig as RawPipelineConfig;
use std::sync::Arc;
use std::time::Duration;

struct NoopStep;

#[async_trait]
impl Step for NoopStep {
    type Ctx = i32;
    type ExecutionError = String;

    async fn execute(&self, _req: ContextMutationRequest<'_, i32>) -> Result<(), String> {
        Ok(())
    }

    fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<String>> {
        Ok(StepNameResponse {
            name: "noop".to_string(),
        })
    }
}

struct IncrementStep;

#[async_trait]
impl Step for IncrementStep {
    type Ctx = i32;
    type ExecutionError = String;

    async fn execute(&self, req: ContextMutationRequest<'_, i32>) -> Result<(), String> {
        *req.ctx += 1;
        Ok(())
    }
}

fn steps() -> Vec<Arc<dyn Step<Ctx = i32, ExecutionError = String>>> {
    vec![Arc::new(NoopStep), Arc::new(IncrementStep)]
}

#[test]
fn test_pipeline_types_accessible_through_edge_domain() {
    let pipeline = PipelineSvc::build(PipelineBuilder {
        steps: steps(),
        config: PipelineConfig {
            timeout_per_step: Some(Duration::from_secs(5)),
            emit_lifecycle_events: false,
            abort_on_error: true,
        },
        event_bus: None,
    });

    assert_eq!(pipeline.step_count(StepCountRequest).unwrap().count, 2);
    assert!(!pipeline.is_empty(PipelineEmptinessRequest).unwrap().empty);
}

#[tokio::test]
async fn test_pipeline_execution_through_edge_domain() {
    let pipeline = PipelineSvc::build(PipelineBuilder {
        steps: steps(),
        config: PipelineConfig::default(),
        event_bus: None,
    });

    let mut ctx = 0;
    let result = pipeline.run(PipelineRunRequest { ctx: &mut ctx }).await;
    assert!(result.is_ok());
    assert_eq!(ctx, 1);
}

#[test]
fn test_pipeline_error_through_edge_domain() {
    let err: PipelineError<String> = PipelineError::ConfigError("test".to_string());
    assert!(!format!("{err:?}").is_empty());
}

#[tokio::test]
async fn test_pipeline_dyn_dispatch_through_edge_domain() {
    let step: Arc<dyn Step<Ctx = i32, ExecutionError = String>> = Arc::new(NoopStep);
    assert_eq!(step.name(StepNameRequest).unwrap().name, "noop".to_string());
    let mut ctx = 0;
    assert!(step
        .execute(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
}

#[test]
fn test_pipeline_config_through_edge_domain() {
    let config = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(10)),
        emit_lifecycle_events: true,
        abort_on_error: false,
    };

    assert_eq!(config.timeout_per_step, Some(Duration::from_secs(10)));
    assert!(config.emit_lifecycle_events);
    assert!(!config.abort_on_error);
}

/// @covers: edge_application's re-exported pipeline types are the underlying edge_pipeline types
#[test]
fn test_edge_domain_pipeline_config_is_edge_pipeline_config_edge() {
    // Proves the facade re-export is a type alias, not a look-alike wrapper —
    // a RawPipelineConfig (imported directly from edge_pipeline) must be
    // directly assignable to the edge_application::PipelineConfig binding used
    // throughout this file.
    let direct: RawPipelineConfig = PipelineConfig::default();
    let via_facade: PipelineConfig = direct;
    assert_eq!(
        via_facade.abort_on_error,
        PipelineConfig::default().abort_on_error
    );
}
