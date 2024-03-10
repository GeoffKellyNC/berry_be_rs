use berry_lib::twitch::twitch_user_data::{TwitchUserData, UserTwitchData};
use sqlx::PgPool;
use uuid::Uuid;

pub enum SetUserReturn {
    UserExists(UserTwitchData),
    NotFound(UserTwitchData),
}

pub async fn set_user_to_db(
    data: &TwitchUserData,
    pool: &PgPool,
) -> Result<SetUserReturn, sqlx::Error> {
    println!("Setting User to DB"); // !REMOVE
    let user = sqlx::query!(
        "SELECT * FROM user_data WHERE twitch_id = $1",
        data.twitch_id
    )
    .fetch_optional(pool)
    .await?;

    let view_count_str = data.view_count.map(|count| count.to_string());
    let current_date_time = chrono::Utc::now().to_rfc3339();

    match user {
        Some(_) => {
            // If user exists, update the user data and return the user
            let updated_user = sqlx::query!(
                "UPDATE user_data SET 
                twitch_login = $2,
                twitch_description = $3,
                twitch_image = $4,
                twitch_email = $5,
                broadcast_type = $6,
                view_count = $7,
                twitch_created = $8
                WHERE twitch_id = $1
                RETURNING *",
                data.twitch_id,
                data.twitch_login,
                data.twitch_description,
                data.twitch_image,
                data.twitch_email,
                data.broadcast_type,
                view_count_str.as_ref().map(|s| s.as_str()),
                data.twitch_created,
            )
            .fetch_one(pool)
            .await?;

            // Map the updated SQL row to UserTwitchData
            Ok(SetUserReturn::UserExists(UserTwitchData {
                unxid: updated_user.unxid,
                twitch_id: updated_user.twitch_id,
                twitch_login: updated_user.twitch_login,
                twitch_description: updated_user.twitch_description,
                twitch_image: updated_user.twitch_image,
                twitch_email: updated_user.twitch_email,
                broadcast_type: updated_user.broadcast_type,
                view_count: updated_user.view_count,
                twitch_created: updated_user.twitch_created,
                app_created: updated_user.app_created,
            }))
        }
        None => {
            let new_unxid = Uuid::new_v4().to_string();
            let new_user = sqlx::query!(
                "INSERT INTO user_data (unxid, twitch_id, twitch_login, twitch_description, twitch_image, twitch_email, broadcast_type, view_count, twitch_created, app_created) 
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                RETURNING *",
                new_unxid,
                data.twitch_id,
                data.twitch_login,
                data.twitch_description,
                data.twitch_image,
                data.twitch_email,
                data.broadcast_type,
                view_count_str.as_ref().map(|s| s.as_str()),
                data.twitch_created,
                current_date_time,
            )
            .fetch_one(pool)
            .await?;

            // Map the new SQL row to UserTwitchData
            Ok(SetUserReturn::NotFound(UserTwitchData {
                unxid: new_user.unxid,
                twitch_id: new_user.twitch_id,
                twitch_login: new_user.twitch_login,
                twitch_description: new_user.twitch_description,
                twitch_image: new_user.twitch_image,
                twitch_email: new_user.twitch_email,
                broadcast_type: new_user.broadcast_type,
                view_count: new_user.view_count,
                twitch_created: new_user.twitch_created,
                app_created: new_user.app_created,
            }))
        }
    }
}
