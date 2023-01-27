use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use chrono::NaiveDate;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::{game::GameState, network::*, player::PlayerState, utils::string_to_board};

const SERVER_ADDR: &str = "https://dice15puzzle-server.haje.org"; // actual server

// const SERVER_ADDR: &str = "http://localhost:1515"; // local server

#[derive(Component)]
pub struct NetworkResponse(ResponseType);

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_network_channel).add_system_set(
            SystemSet::on_update(PlayerState::ResponseWaiting).with_system(response_waiting_system),
        );
    }
}

#[derive(Resource)]
pub struct NetworkChannel {
    input: UnboundedSender<RequestType>,
    output: UnboundedReceiver<ResponseType>,
}

pub struct Network;

impl Network {
    pub fn get_daily_puzzle(
        date: NaiveDate,
        player_state: &mut ResMut<State<PlayerState>>,
        network_channel: &mut Res<NetworkChannel>,
    ) {
        network_channel
            .input
            .send(RequestType::GetDailyPuzzle(date))
            .unwrap();
        player_state.push(PlayerState::ResponseWaiting).unwrap();
        info!("get_daily_puzzle");
    }
}

fn init_network_channel(mut commands: Commands) {
    info!("init response");
    let thread_pool = AsyncComputeTaskPool::get();
    let (req_tx, mut req_rx) = unbounded_channel::<RequestType>();
    let (res_tx, res_rx) = unbounded_channel::<ResponseType>();
    thread_pool.spawn(async move {
        loop {
            if let Some(req) = req_rx.recv().await {
                let client = reqwest::Client::new();
                let res = client
                    .post(SERVER_ADDR)
                    .body(serde_json::to_string(&req).unwrap())
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                let response_type: ResponseType = serde_json::from_str(&res).unwrap();
                res_tx.send(response_type).unwrap();
            }
        }
    });
    commands.insert_resource(NetworkChannel {
        input: req_tx,
        output: res_rx,
    });
}

fn response_waiting_system(
    mut player_state: ResMut<State<PlayerState>>,
    mut transforms: Query<&mut Transform>,
    mut game_query: Query<&mut GameState>,
    mut network_channel: ResMut<NetworkChannel>,
) {
    if let Ok(response_type) = network_channel.output.try_recv() {
        info!("get response {:?}", response_type);
        match response_type {
            ResponseType::GetDailyPuzzle(board_string) => {
                let mut game = game_query.single_mut();
                string_to_board(board_string, &mut transforms, &mut game);
                player_state.replace(PlayerState::Shuffled).unwrap();
            }
        }
    }
}
