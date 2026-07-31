//! Basic `Lifecycle` usage example.
#![allow(clippy::expect_used)]

use edge_application_lifecycle::{
    Lifecycle, LifecycleError, LifecycleStateRequest, LifecycleTransitionRequest, ManagedLifecycle,
    TransitionAllowedRequest, TransitionAllowedResponse, TransitionPolicy,
};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum TaskState {
    Pending,
    Running,
    Done,
    Failed,
}

/// Only allow transitions that move a task forward; never backward.
struct ForwardOnlyPolicy;

impl TransitionPolicy for ForwardOnlyPolicy {
    type State = TaskState;

    fn is_allowed(
        &self,
        req: TransitionAllowedRequest<TaskState>,
    ) -> Result<TransitionAllowedResponse, LifecycleError> {
        Ok(TransitionAllowedResponse {
            allowed: matches!(
                (req.from, req.to),
                (TaskState::Pending, TaskState::Running)
                    | (TaskState::Running, TaskState::Done)
                    | (TaskState::Running, TaskState::Failed)
            ),
        })
    }
}

fn main() {
    tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("rt")
        .block_on(async {
            let task = ManagedLifecycle::new(TaskState::Pending, Box::new(ForwardOnlyPolicy));

            task.transition_to(LifecycleTransitionRequest { target: TaskState::Running })
                .await
                .expect("Pending→Running");
            println!("state = {:?}", task.state(LifecycleStateRequest).expect("state").state);

            task.transition_to(LifecycleTransitionRequest { target: TaskState::Done })
                .await
                .expect("Running→Done");
            println!("state = {:?}", task.state(LifecycleStateRequest).expect("state").state);

            // backward transition is rejected
            let result = task
                .transition_to(LifecycleTransitionRequest { target: TaskState::Pending })
                .await;
            println!("reverse transition: {result:?}");
        });
}
