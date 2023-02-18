use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde_json::to_vec;
use near_sdk::{env, log, near_bindgen, AccountId, Promise};
use serde::{Deserialize, Serialize};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct BattleContract {
    cost_per_times: u32,
    list_boss: LookupMap<String, Boss>,
    list_player: LookupMap<AccountId, PlayerInfo>,
}

impl Default for BattleContract {
    fn default() -> Self {
        let mut boss = LookupMap::new(b"boss".to_vec());
        let players = LookupMap::new(b"players".to_vec());
        boss.insert(
            &String::from("A1"),
            &Boss {
                name: String::from("A1"),
                win_rate: 90,
                reward: 10,
            },
        );

        boss.insert(
            &String::from("A2"),
            &Boss {
                name: String::from("A2"),
                win_rate: 80,
                reward: 20,
            },
        );

        boss.insert(
            &String::from("A3"),
            &Boss {
                name: String::from("A3"),
                win_rate: 10,
                reward: 30,
            },
        );
        Self {
            list_boss: boss,
            list_player: players,
            cost_per_times: 10,
        }
    }
}

#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Boss {
    name: String,
    win_rate: u8,
    reward: u128,
}

impl Boss {
    fn get_win_rate(&self) -> u8 {
        self.win_rate
    }

    fn get_reward(&self) -> u128 {
        self.reward
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
pub struct PlayerInfo {
    name: String,
    wallet_address: String,
    times_play: u64,
    score: u128,
}

impl PlayerInfo {
    fn add_times_play(&mut self) -> u64 {
        self.times_play += 1;
        self.times_play
    }

    fn decrease_times_play(&mut self) -> u64 {
        self.times_play -= 1;
        self.times_play
    }

    fn increase_reward(&mut self, reward: u128) {
        self.score += reward
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
pub struct ResultBattle {
    status_code: bool,
    status_battle: bool,
    reward: u128,
}

#[near_bindgen]
impl BattleContract {
    pub fn set_cost_per_times(&mut self, cost: u32) -> bool {
        if self.get_owner().eq(&env::signer_account_id()) {
            self.cost_per_times = cost;
            true
        } else {
            false
        }
    }

    pub fn get_cost_per_times(&self) -> u128 {
        self.cost_per_times as u128
    }

    pub fn get_owner(&self) -> AccountId {
        env::predecessor_account_id()
    }

    pub fn get_player_info(&self, account_id: AccountId) -> PlayerInfo {
        self.list_player.get(&account_id).unwrap()
    }

    pub fn register(&mut self, player_info: PlayerInfo) -> PlayerInfo {
        let contains_user = self.list_player.contains_key(&env::signer_account_id());
        if contains_user {
            log!(
                "User exist! | Id = [{:?}]",
                self.list_player.get(&env::signer_account_id())
            );
            player_info
        } else {
            self.list_player
                .insert(&env::signer_account_id(), &player_info);
            player_info
        }
    }

    #[payable]
    pub fn buy_times_play(&mut self, amount: f64) {
        if amount != self.get_cost_per_times() as f64 {
            env::panic(
                format!(
                    "Error: not enough attached deposit. Required: {}, attached: {}",
                    self.get_cost_per_times(),
                    amount
                )
                .as_bytes(),
            );
        } else {
            log!("User = [{}] buy 1 times", env::signer_account_id());
            let mut player = self.get_player_info(env::signer_account_id());
            player.add_times_play();
            self.list_player.insert(&env::signer_account_id(), &player);
            Promise::new(AccountId::new_unchecked(String::from("truongphan.testnet")))
                .transfer(amount as u128);
        }
    }

    fn generate_random_number(&self) -> u8 {
        let seed = env::random_seed();
        let hash = env::sha256(&seed);
        let slice = &hash[0..1];
        let num = slice[0] as u8;
        num % 100
    }

    fn get_boss_info(&self, boss_name: String) -> Boss {
        self.list_boss.get(&boss_name).unwrap()
    }

    pub fn fight_boss(&mut self, boss_name: String) -> ResultBattle {
        let mut player_info = self.get_player_info(env::signer_account_id());
        let times_play = player_info.times_play;
        if times_play <= 0 {
            log!("You do not have enough ticket to fight");
            ResultBattle {
                status_code: false,
                status_battle: false,
                reward: 0,
            }
        } else {
            let mut reward = 0;
            let mut status_battle = false;
            let roll = self.generate_random_number();
            let boss_info = self.get_boss_info(boss_name);
            log!("User roll = [{}]", roll);
            if roll > boss_info.get_win_rate() {
                reward = boss_info.get_reward();
                player_info.increase_reward(reward);
                status_battle = true;
            }
            player_info.decrease_times_play();
            self.list_player
                .insert(&env::signer_account_id(), &player_info);
            ResultBattle {
                status_code: true,
                status_battle: status_battle,
                reward: reward,
            }
        }
    }
}
