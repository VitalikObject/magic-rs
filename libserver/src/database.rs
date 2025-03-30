use rand::distr::{Alphanumeric, SampleString};
use rusqlite::{params, Connection, Result};
use tracing::error;

use crate::{
    byte_stream::ByteStream, logic::avatar::LogicClientAvatar, math::LogicLong,
    resources::ResourceManager, sc_string::StringBuilder,
};

pub struct DatabaseConnection(Connection);

pub struct PlayerSaveData {
    pub id: LogicLong,
    pub pass_token: String,
    pub home_json: String,
    pub client_avatar_blob: String,
}

impl DatabaseConnection {
    pub fn connect(path: &str) -> Result<Self> {
        const INIT_QUERY: &str = r#"
            CREATE TABLE IF NOT EXISTS t_player_data (
                id INTEGER PRIMARY KEY,
                pass_token TEXT NOT NULL,
                home_json TEXT NOT NULL,
                client_avatar_blob TEXT NOT NULL,
                score INTEGER NOT NULL
            )
        "#;

        let connection = rusqlite::Connection::open(path)?;
        connection.execute(INIT_QUERY, [])?;

        Ok(Self(connection))
    }

    pub fn fetch_or_create_player(&self, id: &LogicLong) -> Result<Option<PlayerSaveData>> {
        if id.is_zero() {
            Ok(Some(self.create_new_player_data()?))
        } else {
            self.load_existing_player_data(id)
        }
    }

    pub fn fetch_player(&self, id: &LogicLong) -> Result<Option<PlayerSaveData>> {
        if !id.is_zero() {
            self.load_existing_player_data(id)
        } else {
            Ok(None)
        } 
    }

    pub fn save_player_data(
        &self,
        id: &LogicLong,
        home_json: &str,
        avatar: &LogicClientAvatar,
    ) -> Result<()> {
        const UPDATE_QUERY: &str =
            r#"UPDATE t_player_data SET home_json = ?1, client_avatar_blob = ?2 WHERE id = ?3"#;

        let mut byte_stream = ByteStream::new(10);
        avatar.encode(&mut byte_stream);
        let client_avatar_blob = rbase64::encode(byte_stream.get_byte_array());

        self.0.execute(
            UPDATE_QUERY,
            params![home_json, &client_avatar_blob, id.lower_int],
        )?;

        Ok(())
    }

    fn create_new_player_data(&self) -> Result<PlayerSaveData> {
        const INSERT_QUERY: &str = r#"
            INSERT INTO t_player_data (pass_token, home_json, client_avatar_blob, score)
            values (?1, ?2, ?3, ?4) RETURNING *
        "#;

        let pass_token = Alphanumeric.sample_string(&mut rand::rng(), 40);

        let mut sb = StringBuilder::new();
        ResourceManager::get_json("level/starting_home.json").write_to_string(&mut sb);
        let home_json = sb.to_string();

        let logic_client_avatar = LogicClientAvatar::get_default_avatar();
        let mut byte_stream = ByteStream::new(10);
        logic_client_avatar.encode(&mut byte_stream);
        let client_avatar_blob = rbase64::encode(byte_stream.get_byte_array());

        let id: i32 = self
            .0
            .prepare(INSERT_QUERY)
            .inspect_err(|err| {
                error!("db::prepare `insert into t_player_data` failed: {err}");
            })?
            .query_map(
                params![&pass_token, &home_json, &client_avatar_blob, 0],
                |row| row.get(0),
            )?
            .next()
            .expect("query didn't return inserted data")?;

        Ok(PlayerSaveData {
            id: LogicLong::new(0, id),
            pass_token,
            home_json,
            client_avatar_blob,
        })
    }

    fn load_existing_player_data(&self, id: &LogicLong) -> Result<Option<PlayerSaveData>> {
        const SELECT_QUERY: &str = r#"SELECT * FROM t_player_data WHERE id = (?1)"#;

        self.0
            .prepare(SELECT_QUERY)
            .inspect_err(|err| {
                error!("db::prepare `select from t_player_data` failed: {err}");
            })?
            .query_map(params![id.lower_int], |row| {
                Ok(PlayerSaveData {
                    id: LogicLong::new(0, row.get(0)?),
                    pass_token: row.get(1)?,
                    home_json: row.get(2)?,
                    client_avatar_blob: row.get(3)?,
                })
            })?
            .into_iter()
            .next()
            .transpose()
    }
}
