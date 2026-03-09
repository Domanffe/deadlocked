use crate::{
    cs2::{CS2, entity::player::Player},
    os::leaderboard,
};

#[derive(Debug, Default)]
pub struct LeaderboardData {
    pub kills: i32,
}

impl CS2 {
    pub fn leaderboard(&mut self) {
        let Some(local_player) = Player::local_player(self) else {
            return;
        };

        let Some(kills) = local_player.round_kills(self) else {
            return;
        };

        // only report kills when every enemy has a proper steamid
        // as to not report kills from bot matches (hopefully)
        if !self.is_player_match() {
            return;
        }

        // new round
        if kills < self.leaderboard_data.kills {
            self.leaderboard_data.kills = 0;
        }

        while self.leaderboard_data.kills < kills {
            self.leaderboard_data.kills += 1;
            leaderboard::add_kill(local_player.steam_id(self), local_player.name(self));
        }
    }
}
