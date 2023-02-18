# Player vs Boss

## Feature
- Register
- Buy times play boss and earn reward when roll win

Function:
1. register
near call <id-battle-contract> register --args '{"player_info":{
  "name": "Truong",
  "wallet_address": "truongphan.testnet",
  "times_play": 0,
  "score": 0
}}' --accountId <your-id>

2. get_player_info:
near call <id-battle-contract> get_player_info --args '{"account_id": "<your-id>"}'  --accountId <your-id>

3. buy_times_play
near call <id-battle-contract> buy_times_play --amount 1 --args '{"amount":1}' --accountId <your-id>

4. set_cost_per_times:
near call <id-battle-contract> set_cost_per_times --args '{"cost":1}' --accountId <your-id>

5. get_cost_per_times
near call <id-battle-contract> get_cost_per_times --accountId <your-id>

6. fight_boss
near call <id-battle-contract> fight_boss --args '{"boss_name":"A3"}' --accountId <your-id>


## Example
1. We need call func <1> for create new account
2. Use func <2> for more infomation about user
3. Call func <3> for adding more ticket battle with boss
4. Call func <6> when you want to beat boss