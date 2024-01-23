use axum::{extract::State, Json};

use crate::handlers::common::get_portfolios;
use crate::{models::portfolio::Portfolios, models::server_state::ServerState};
use std::sync::Arc;

pub async fn portfolios(
    State(server_state_arc): State<Arc<ServerState>>,
) -> Json<Portfolios> {
    let portfolios = get_portfolios(&server_state_arc).await;

    portfolios.into()
}
