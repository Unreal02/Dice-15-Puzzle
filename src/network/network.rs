use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use chrono::NaiveDate;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::{daily_puzzle_info::DailyPuzzleInfo, game::GameState, network::*, player::PlayerState};

#[cfg(not(feature = "local_server"))]
const SERVER_ADDR: &str = "https://dice15puzzle-server.haje.org"; // actual server

#[cfg(feature = "local_server")]
const SERVER_ADDR: &str = "http://localhost:1515"; // local server

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
        network_channel: &Res<NetworkChannel>,
    ) {
        network_channel
            .input
            .send(RequestType::GetDailyPuzzle(date))
            .unwrap();
        // ModeSelectionPopup 또는 DateSelectionPopup을 pop하고 ResponseWaiting을 push하기 위해 set 사용
        assert_eq!(player_state.inactives().len(), 1);
        let _ = player_state.set(PlayerState::ResponseWaiting);
        info!("get_daily_puzzle");
    }

    pub fn get_daily_puzzle_date(
        player_state: &mut ResMut<State<PlayerState>>,
        network_channel: &mut Res<NetworkChannel>,
    ) {
        network_channel
            .input
            .send(RequestType::GetDailyPuzzleDate)
            .unwrap();
        // ModeSelectionPopup을 pop하고 ResponseWaiting을 push하기 위해 set 사용
        assert_eq!(player_state.inactives().len(), 1);
        player_state.set(PlayerState::ResponseWaiting).unwrap();
        info!("get_daily_puzzle_date");
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
    mut daily_puzzle_info_query: Query<&mut DailyPuzzleInfo>,
) {
    let mut game = game_query.single_mut();
    let mut daily_puzzle_info = daily_puzzle_info_query.single_mut();

    if let Ok(response_type) = network_channel.output.try_recv() {
        info!("get response {:?}", response_type);
        match response_type {
            ResponseType::GetDailyPuzzle(date, board_string) => {
                daily_puzzle_info.insert_daily_puzzle(date, board_string);
                let load_result = daily_puzzle_info.load_daily_puzzle(
                    date,
                    &mut transforms,
                    &mut game,
                    &mut player_state,
                    &mut Res::from(network_channel),
                );
                assert_eq!(load_result, true);
            }
            ResponseType::GetDailyPuzzleDate { first, last } => {
                daily_puzzle_info.first_date = first;
                daily_puzzle_info.last_date = last;
                daily_puzzle_info.current_date = last;
                let _ = daily_puzzle_info.load_daily_puzzle(
                    last,
                    &mut transforms,
                    &mut game,
                    &mut player_state,
                    &mut Res::from(network_channel),
                );
            }
            ResponseType::GenerateDailyPuzzle(_) => unreachable!(),
            ResponseType::EnrollPuzzleState(_) => todo!(),
            ResponseType::GetPuzzleState(_) => todo!(),
        }
    }
}
