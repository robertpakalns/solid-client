use discord_rich_presence::{
    DiscordIpc, DiscordIpcClient,
    activity::{Activity, Button, Timestamps},
};
use std::{
    thread::{sleep, spawn},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub fn drpc_init() {
    spawn(move || {
        let client_id = "1376977830818218004";

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let activity = Activity::new()
            .details("Playing Solid Client")
            .buttons(vec![
                Button::new("Community Server", "https://discord.gg/yPjrUrvSzv"),
                Button::new("GitHub", "https://github.com/robertpakalns/solid-client"),
            ])
            .timestamps(Timestamps::new().start(timestamp));

        let mut client: DiscordIpcClient = DiscordIpcClient::new(client_id).unwrap();
        client.connect().unwrap();
        client.set_activity(activity.clone()).unwrap();

        loop {
            client.set_activity(activity.clone()).unwrap();
            sleep(Duration::from_secs(15));
        }
    });
}
