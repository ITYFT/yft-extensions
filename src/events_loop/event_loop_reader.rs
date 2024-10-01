use std::{sync::Arc, time::Duration};

use crate::{AppLogger, ApplicationStates};

use super::{events_loop::EventsLoopMessage, EventsLoopTick};

pub async fn events_loop_reader<TModel: Send + Sync + 'static>(
    name: Arc<String>,
    event_loop_tick: Arc<dyn EventsLoopTick<TModel> + Send + Sync + 'static>,
    app_states: Arc<dyn ApplicationStates + Send + Sync + 'static>,
    logger: Arc<dyn AppLogger + Send + Sync + 'static>,
    iteration_timeout: Duration,
    mut receiver: tokio::sync::mpsc::UnboundedReceiver<EventsLoopMessage<TModel>>,
) {
    while !app_states.is_initialized() {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    let event_loop_tick_spawned = event_loop_tick.clone();
    let _ = tokio::spawn(async move {
        event_loop_tick_spawned.started().await;
    })
    .await;

    while !app_states.is_shutting_down() {
        if let Some(message) = tokio::sync::mpsc::UnboundedReceiver::recv(&mut receiver).await {
            if message.is_shutdown() {
                break;
            }

            let timer_tick = tokio::spawn(execute_timer(
                event_loop_tick.clone(),
                message.unwrap_message(),
            ));
            match tokio::time::timeout(iteration_timeout, timer_tick).await {
                Ok(result) => {
                    if result.is_err() {
                        logger.write_error(
                            format!("EventLoop {} iteration", name.as_str()),
                            "Iteration is panicked".to_string(),
                            None,
                        );
                    }
                }
                Err(_) => {
                    logger.write_error(
                        format!("EventLoop {} iteration", name.as_str()),
                        "Iteration is time outed".to_string(),
                        None,
                    );
                }
            }
        }
    }

    let event_loop_tick_spawned = event_loop_tick.clone();
    let _ = tokio::spawn(async move {
        event_loop_tick_spawned.finished().await;
    })
    .await;
}

async fn execute_timer<TModel: Send + Sync + 'static>(
    events_loop_tick: Arc<dyn EventsLoopTick<TModel> + Send + Sync + 'static>,
    model: TModel,
) {
    events_loop_tick.tick(model).await;
}
