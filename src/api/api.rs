use super::{response::Response, types::*, APIEndpoint};
use crate::{
    model::*,
    utils::{
        result::{Result, TelegramError},
        FormDataFile,
    },
};
use async_trait::async_trait;
use std::vec::Vec;

/// This trait provides methods for interacting with the telegram API.
#[async_trait]
pub trait API: Sync {
    /// executes a get request to the given telegram api endpoint
    async fn get(&self, endpoint: APIEndpoint, data: Option<serde_json::Value>)
        -> Result<Response>;

    /// executes a post request to the given telegram api endpoint
    async fn post(
        &self,
        endpoint: APIEndpoint,
        data: Option<serde_json::Value>,
    ) -> Result<Response>;

    /// executes a post request to the given api endpoint and uploads the given
    /// files
    async fn post_file(
        &self,
        endpoint: APIEndpoint,
        data: Option<serde_json::Value>,
        files: Option<Vec<FormDataFile>>,
    ) -> Result<Response>;

    /// A simple method for testing your bot's auth token. Requires no
    /// parameters. Returns basic information about the bot in form of a
    /// [`User`] object.
    async fn get_me(&self) -> Result<User> {
        self.get(APIEndpoint::GetMe, None).await?.into()
    }

    /// Use this method to log out from the cloud Bot API server before
    /// launching the bot locally. You **must** log out the bot before
    /// running it locally, otherwise there is no guarantee that
    /// the bot will receive updates. After a successful call, you will not be
    /// able to log in again using the same token for 10 minutes. Returns
    /// True on success.
    async fn log_out(&self) -> Result<bool> {
        self.post(APIEndpoint::LogOut, None).await?.into()
    }

    /// Use this method to close the bot instance before moving it from one
    /// local server to another. You need to delete the webhook before
    /// calling this method to ensure that the bot isn't launched again
    /// after server restart. The method will return error 429 in the first 10
    /// minutes after the bot is launched.
    async fn close(&self) -> Result<bool> {
        self.post(APIEndpoint::Close, None).await?.into()
    }

    /// (**WARNING:** this method should not be used by the library user
    /// themselves as this gets handled by the [`Client`] object,
    /// to handle an update event, please subscribe to those using
    /// [`subscribe_handler`]) Use this method to receive incoming updates
    /// using long polling. A `Vec<`[`Update`]`>` is returned.
    ///
    /// [`Client`]: ../client/struct.Client.html
    /// [`subscribe_handler`]:
    /// ../client/struct.Client.html#method.subscribe_handler
    async fn get_updates(&self, data: GetUpdates) -> Result<Vec<Update>> {
        self.get(APIEndpoint::GetUpdates, Some(serde_json::to_value(data)?))
            .await?
            .into()
    }

    /// Use this method to specify a url and receive incoming updates via an
    /// outgoing webhook. Whenever there is an update for the bot, we will
    /// send an HTTPS POST request to the specified url, containing a
    /// JSON-serialized [Update]. In case of an unsuccessful request,
    /// we will give up after a reasonable amount of attempts. Returns True on
    /// success.
    async fn set_webhook(&self, data: SetWebhook) -> Result<bool> {
        self.post(APIEndpoint::SetWebhook, Some(serde_json::to_value(data)?))
            .await?
            .into()
    }

    /// Use this method to remove webhook integration if you decide to switch
    /// back to using [API::get_updates]. Returns True on success.
    async fn delete_webhook(&self, data: DeleteWebhook) -> Result<bool> {
        self.get(
            APIEndpoint::DeleteWebhook,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to get current webhook status. On success, returns a
    /// [WebhookInfo] object. If the bot is using [API::get_updates], will
    /// return a [WebhookInfo] object with the url field empty.
    async fn get_webhook_info(&self) -> Result<WebhookInfo> {
        self.get(APIEndpoint::GetWebhookInfo, None).await?.into()
    }

    /// Use this method to send text messages. On success, the sent [`Message`]
    /// is returned.
    async fn send_message(&self, data: SendMessage) -> Result<Message> {
        self.post(APIEndpoint::SendMessage, Some(serde_json::to_value(data)?))
            .await?
            .into()
    }

    /// Use this method to change the list of the bot's commands. Returns True
    /// on success.
    async fn set_my_commands(&self, data: SetMyCommands) -> Result<bool> {
        self.post(
            APIEndpoint::SetMyCommands,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to get the current list of the bot's commands. Requires
    /// no parameters. Returns a `Vec<`[`BotCommand`]`>` on success.
    async fn get_my_commands(&self) -> Result<Vec<BotCommand>> {
        self.get(APIEndpoint::GetMyCommands, None).await?.into()
    }

    /// Use this method to forward messages of any kind. On success, the sent
    /// [`Message`] is returned.
    async fn forward_message(&self, data: ForwardMessage) -> Result<Message> {
        self.post(
            APIEndpoint::ForwardMessage,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to copy messages of any kind. The method is analogous to
    /// the method [`forward_message`], but the copied message doesn't have
    /// a link to the original message. Returns the [`MessageId`] of the
    /// sent message on success.
    ///
    /// [`forward_message`]: API::forward_message
    async fn copy_message(&self, data: CopyMessage) -> Result<MessageId> {
        self.post(APIEndpoint::CopyMessage, Some(serde_json::to_value(data)?))
            .await?
            .into()
    }

    /// Use this method to send photos. On success, the sent [`Message`] is
    /// returned.
    async fn send_photo(&self, data: SendPhoto) -> Result<Message> {
        match &data.photo {
            InputFile::String(_) => self
                .post(APIEndpoint::SendPhoto, Some(serde_json::to_value(&data)?))
                .await?
                .into(),
            InputFile::File(f) => self
                .post_file(
                    APIEndpoint::SendPhoto,
                    Some(serde_json::to_value(&data)?),
                    Some(vec![f.clone()]),
                )
                .await?
                .into(),
        }
    }

    /// Use this method to send audio files, if you want Telegram clients to
    /// display them in the music player. Your audio must be in the .MP3 or
    /// .M4A format. On success, the sent [`Message`] is returned.
    /// Bots can currently send audio files of up to 50 MB in size, this limit
    /// may be changed in the future.
    async fn send_audio(&self, data: SendAudio) -> Result<Message> {
        let mut files = Vec::new();
        if let InputFile::File(f) = &data.audio {
            files.push(f.clone());
        }
        if data.thumb.is_some() {
            if let InputFile::File(f) = data.thumb.as_ref().unwrap() {
                files.push(f.clone());
            }
        }

        self.post_file(
            APIEndpoint::SendDocument,
            Some(serde_json::to_value(&data)?),
            Some(files),
        )
        .await?
        .into()
    }

    /// Use this method to send general files. On success, the sent [`Message`]
    /// is returned. Bots can currently send files of any type of up to 50
    /// MB in size, this limit may be changed in the future.
    async fn send_document(&self, data: SendDocument) -> Result<Message> {
        let mut files = Vec::new();
        if let InputFile::File(f) = &data.document {
            files.push(f.clone());
        }

        if data.thumb.is_some() {
            if let InputFile::File(f) = data.thumb.as_ref().unwrap() {
                files.push(f.clone());
            }
        }

        self.post_file(
            APIEndpoint::SendDocument,
            Some(serde_json::to_value(&data)?),
            Some(files),
        )
        .await?
        .into()
    }

    /// Use this method to send video files, Telegram clients support mp4 videos
    /// (other formats may be sent as [`Document`]). On success, the sent
    /// [`Message`] is returned. Bots can currently send video files of up to 50
    /// MB in size, this limit may be changed in the future.
    async fn send_video(&self, data: SendVideo) -> Result<Message> {
        let mut files = Vec::new();
        if let InputFile::File(f) = &data.video {
            files.push(f.clone());
        }

        if data.thumb.is_some() {
            if let InputFile::File(f) = data.thumb.as_ref().unwrap() {
                files.push(f.clone());
            }
        }

        self.post_file(
            APIEndpoint::SendDocument,
            Some(serde_json::to_value(&data)?),
            Some(files),
        )
        .await?
        .into()
    }

    /// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video
    /// without sound). On success, the sent [`Message`] is returned. Bots
    /// can currently send animation files of up to 50 MB in size, this limit
    /// may be changed in the future.
    async fn send_animation(&self, data: SendAnimation) -> Result<Message> {
        let mut files = Vec::new();
        if let InputFile::File(f) = &data.animation {
            files.push(f.clone());
        }

        if data.thumb.is_some() {
            if let InputFile::File(f) = data.thumb.as_ref().unwrap() {
                files.push(f.clone());
            }
        }

        self.post_file(
            APIEndpoint::SendDocument,
            Some(serde_json::to_value(&data)?),
            Some(files),
        )
        .await?
        .into()
    }

    /// Use this method to send audio files, if you want Telegram clients to
    /// display the file as a playable voice message. For this to work, your
    /// audio must be in an .OGG file encoded with OPUS (other formats may be
    /// sent as [`Audio`] or [`Document`]). On success, the sent [`Message`]
    /// is returned. Bots can currently send voice messages of up to 50 MB in
    /// size, this limit may be changed in the future.
    async fn send_voice(&self, data: SendVoice) -> Result<Message> {
        let mut files = Vec::new();
        if let InputFile::File(f) = &data.voice {
            files.push(f.clone());
        }

        self.post_file(
            APIEndpoint::SendDocument,
            Some(serde_json::to_value(&data)?),
            Some(files),
        )
        .await?
        .into()
    }

    /// As of v.4.0, Telegram clients support rounded square mp4 videos of up to
    /// 1 minute long. Use this method to send video messages. On success,
    /// the sent [`Message`] is returned.
    async fn send_video_note(&self, data: SendVideoNote) -> Result<Message> {
        let mut files = Vec::new();
        if let InputFile::File(f) = &data.video_note {
            files.push(f.clone());
        }

        if data.thumb.is_some() {
            if let InputFile::File(f) = data.thumb.as_ref().unwrap() {
                files.push(f.clone());
            }
        }

        self.post_file(
            APIEndpoint::SendDocument,
            Some(serde_json::to_value(&data)?),
            Some(files),
        )
        .await?
        .into()
    }

    /// Use this method to send a group of photos or videos as an album.
    /// On success, a [`Vec<Message>`] is returned.
    async fn send_media_group(&self, data: SendMediaGroup) -> Result<Vec<Message>> {
        let mut files = Vec::new();
        for media in &data.media {
            if let InputFile::File(f) = media.get_media() {
                files.push(f.clone());
            }
        }

        files.dedup_by(|f1, f2| f1 == f2);

        self.post_file(
            APIEndpoint::SendMediaGroup,
            Some(serde_json::to_value(&data)?),
            Some(files),
        )
        .await?
        .into()
    }

    /// Use this method to send a point on the map. On success, the sent
    /// [`Message`] is returned.
    async fn send_location(&self, data: SendLocation) -> Result<Message> {
        self.post(APIEndpoint::SendLocation, Some(serde_json::to_value(data)?))
            .await?
            .into()
    }

    /// Use this method to send information about a venue. On success, the sent
    /// [`Message`] is returned.
    async fn send_venue(&self, data: SendVenue) -> Result<Message> {
        self.post(APIEndpoint::SendVenue, Some(serde_json::to_value(data)?))
            .await?
            .into()
    }

    /// Use this method to send phone contacts. On success, the sent [`Message`]
    /// is returned.
    async fn send_contact(&self, data: SendContact) -> Result<Message> {
        self.post(APIEndpoint::SendContact, Some(serde_json::to_value(data)?))
            .await?
            .into()
    }

    /// Use this method to send a native poll. On success, the sent [`Message`]
    /// is returned.
    async fn send_poll(&self, data: SendPoll) -> Result<Message> {
        self.post(APIEndpoint::SendPoll, Some(serde_json::to_value(data)?))
            .await?
            .into()
    }

    /// Use this method to send a dice, which will have a random value from 1 to
    /// 6. On success, the sent [Message] is returned.
    async fn send_dice(&self, data: SendDice) -> Result<Message> {
        self.post(APIEndpoint::SendDice, Some(serde_json::to_value(data)?))
            .await?
            .into()
    }

    /// Use this method when you need to tell the user that something is
    /// happening on the bot's side. The status is set for 5 seconds or less
    /// (when a message arrives from your bot, Telegram clients clear its typing
    /// status). Returns True on success.
    async fn send_chat_action(&self, data: SendChatAction) -> Result<bool> {
        self.post(
            APIEndpoint::SendChatAction,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to edit text and game messages. On success, if edited
    /// message is sent by the bot, the edited [`Message`] is returned,
    /// otherwise True is returned.
    async fn edit_message_text(&self, data: EditMessageText) -> Result<TrueOrObject<Message>> {
        self.post(
            APIEndpoint::EditMessageText,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to edit captions of messages. On success, if edited
    /// message is sent by the bot, the edited [`Message`] is returned,
    /// otherwise True is returned.
    async fn edit_message_caption(
        &self,
        data: EditMessageCaption,
    ) -> Result<TrueOrObject<Message>> {
        self.post(
            APIEndpoint::EditMessageCaption,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to edit animation, audio, document, photo, or video
    /// messages. If a message is a part of a message album, then it can be
    /// edited only to a photo or a video. Otherwise, message type can be
    /// changed arbitrarily. When inline message is edited, new file can't be
    /// uploaded. Use previously uploaded file via its file_id or specify a
    /// URL. On success, if the edited message was sent by the bot, the
    /// edited [`Message`] is returned, otherwise True is returned.
    async fn edit_message_media(&self, data: EditMessageMedia) -> Result<TrueOrObject<Message>> {
        self.post(
            APIEndpoint::EditMessageMedia,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to edit only the reply markup of messages. On success,
    /// if edited message is sent by the bot, the edited [`Message`] is
    /// returned, otherwise True is returned.
    async fn edit_message_reply_markup(
        &self,
        data: EditMessageReplyMarkup,
    ) -> Result<TrueOrObject<Message>> {
        self.post(
            APIEndpoint::EditMessageReplyMarkup,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to stop a poll which was sent by the bot. On success,
    /// the stopped [`Poll`] with the final results is returned.
    async fn stop_poll(&self, data: StopPoll) -> Result<Poll> {
        self.post(APIEndpoint::StopPoll, Some(serde_json::to_value(data)?))
            .await?
            .into()
    }

    /// Use this method to delete a message, including service messages, with
    /// the following limitations:
    /// - A message can only be deleted if it was sent less than 48 hours ago.
    /// - A dice message in a private chat can only be deleted if it was sent
    ///   more than 24 hours ago.
    /// - Bots can delete outgoing messages in private chats, groups, and
    ///   supergroups.
    /// - Bots can delete incoming messages in private chats.
    /// - Bots granted can_post_messages permissions can delete outgoing
    ///   messages in channels.
    /// - If the bot is an administrator of a group, it can delete any message
    ///   there.
    /// - If the bot has can_delete_messages permission in a supergroup or a
    ///   channel, it can delete any message there.
    /// Returns True on success.
    async fn delete_message(&self, data: DeleteMessage) -> Result<bool> {
        self.post(
            APIEndpoint::DeleteMessage,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to edit live location messages.
    /// A location can be edited until its live_period expires or editing is
    /// explicitly disabled by a call to stopMessageLiveLocation.
    /// On success, if the edited message was sent by the bot, the edited
    /// [`Message`] is returned, otherwise True is returned.
    async fn edit_message_live_location(
        &self,
        data: EditMessageLiveLocation,
    ) -> Result<TrueOrObject<Message>> {
        self.post(
            APIEndpoint::EditMessageLiveLocation,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to stop updating a live location message before
    /// live_period expires. On success, if the message was sent by the bot,
    /// the sent [`Message`] is returned, otherwise True is returned.
    async fn stop_message_live_location(
        &self,
        data: StopMessageLiveLocation,
    ) -> Result<TrueOrObject<Message>> {
        self.post(
            APIEndpoint::StopMessageLiveLocation,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to get a list of profile pictures for a user. Returns a
    /// [`UserProfilePhotos`] object.
    async fn get_user_profile_photos(
        &self,
        data: GetUserProfilePhotos,
    ) -> Result<UserProfilePhotos> {
        self.post(
            APIEndpoint::GetUserProfilePhotos,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to get basic info about a file and prepare it for
    /// downloading. For the moment, bots can download files of up to 20MB
    /// in size. On success, a [`File`] object is returned. The file can then be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`, where <file_path> is taken from the response.
    /// It is guaranteed that the link will be valid for at least 1 hour. When
    /// the link expires, a new one can be requested by calling
    /// [`API::get_file`] again.
    async fn get_file(&self, data: GetFile) -> Result<File> {
        self.post(APIEndpoint::GetFile, Some(serde_json::to_value(data)?))
            .await?
            .into()
    }

    /// Use this method to unban a previously kicked user in a supergroup or
    /// channel. The user will not return to the group or channel
    /// automatically, but will be able to join via link, etc. The bot must
    /// be an administrator for this to work. Returns True on success.
    async fn unban_chat_member(&self, data: UnbanChatMember) -> Result<bool> {
        self.post(
            APIEndpoint::UnbanChatMember,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to kick a user from a group, a supergroup or a channel.
    /// In the case of supergroups and channels, the user will not be able to
    /// return to the group on their own using invite links, etc., unless
    /// unbanned first. The bot must be an administrator in the chat for
    /// this to work and must have the appropriate admin rights. Returns True on
    /// success.
    async fn kick_chat_member(&self, data: KickChatMember) -> Result<bool> {
        self.post(
            APIEndpoint::KickChatMember,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to restrict a user in a supergroup.
    /// The bot must be an administrator in the supergroup for this to work and
    /// must have the appropriate admin rights. Pass True for all
    /// permissions to lift restrictions from a user. Returns True on success.
    async fn restrict_chat_member(&self, data: RestrictChatMember) -> Result<bool> {
        self.post(
            APIEndpoint::RestrictChatMember,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to promote or demote a user in a supergroup or a
    /// channel. The bot must be an administrator in the chat for this to
    /// work and must have the appropriate admin rights. Pass False for all
    /// boolean parameters to demote a user. Returns True on success.
    async fn promote_chat_member(&self, data: PromoteChatMember) -> Result<bool> {
        self.post(
            APIEndpoint::PromoteChatMember,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to set a custom title for an administrator in a
    /// supergroup promoted by the bot. Returns True on success.
    async fn set_chat_administrator_custom_title(
        &self,
        data: SetChatAdministratorCustomTitle,
    ) -> Result<bool> {
        self.post(
            APIEndpoint::SetChatAdministratorCustomTitle,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to set default chat permissions for all members.
    /// The bot must be an administrator in the group or a supergroup for this
    /// to work and must have the can_restrict_members admin rights. Returns
    /// True on success.
    async fn set_chat_permissions(&self, data: SetChatPermissions) -> Result<bool> {
        self.post(
            APIEndpoint::SetChatPermissions,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to generate a new invite link for a chat; any previously
    /// generated link is revoked. The bot must be an administrator in the
    /// chat for this to work and must have the appropriate admin rights.
    /// Returns the new invite link as String on success.
    ///
    /// Note: Each administrator in a chat generates their own invite links.
    /// Bots can't use invite links generated by other administrators.
    /// If you want your bot to work with invite links, it will need to generate
    /// its own link using [`API::export_chat_invite_link`] – after this the
    /// link will become available to the bot via the [`API::get_chat`] method.
    /// If your bot needs to generate a new invite link replacing its previous
    /// one, use [`API::export_chat_invite_link`] again.
    async fn export_chat_invite_link(&self, data: ExportChatInviteLink) -> Result<String> {
        self.post(
            APIEndpoint::ExportChatInviteLink,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to set a new profile photo for the chat. Photos can't be
    /// changed for private chats. The bot must be an administrator in the
    /// chat for this to work and must have the appropriate admin rights.
    /// Returns True on success.
    async fn set_chat_photo(&self, data: SetChatPhoto) -> Result<bool> {
        let mut files = Vec::new();
        match &data.photo {
            InputFile::File(f) => files.push(f.clone()),
            _ => {
                return Err(TelegramError::InvalidArgument(
                    "this endpoint only accepts files to be uploaded".to_owned(),
                )
                .into())
            },
        }

        self.post_file(
            APIEndpoint::SetChatPhoto,
            Some(serde_json::to_value(&data)?),
            Some(files),
        )
        .await?
        .into()
    }

    /// Use this method to delete a chat photo. Photos can't be changed for
    /// private chats. The bot must be an administrator in the chat for this
    /// to work and must have the appropriate admin rights. Returns True on
    /// success.
    async fn delete_chat_photo(&self, data: DeleteChatPhoto) -> Result<bool> {
        self.post(
            APIEndpoint::DeleteChatPhoto,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to change the title of a chat. Titles can't be changed
    /// for private chats. The bot must be an administrator in the chat for
    /// this to work and must have the appropriate admin rights.
    /// Returns True on success.
    async fn set_chat_title(&self, data: SetChatTitle) -> Result<bool> {
        self.post(APIEndpoint::SetChatTitle, Some(serde_json::to_value(data)?))
            .await?
            .into()
    }

    /// Use this method to change the description of a group, a supergroup or a
    /// channel. The bot must be an administrator in the chat for this to
    /// work and must have the appropriate admin rights. Returns True on
    /// success.
    async fn set_chat_description(&self, data: SetChatDescription) -> Result<bool> {
        self.post(
            APIEndpoint::SetChatDescription,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to pin a message in a group, a supergroup, or a channel.
    /// The bot must be an administrator in the chat for this to work and must
    /// have the ‘can_pin_messages’ admin right in the supergroup
    /// or ‘can_edit_messages’ admin right in the channel. Returns True on
    /// success.
    async fn pin_chat_message(&self, data: PinChatMessage) -> Result<bool> {
        self.post(
            APIEndpoint::PinChatMessage,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to remove a message from the list of pinned messages in
    /// a chat. If the chat is not a private chat, the bot must be an
    /// administrator in the chat for this to work and must have the
    /// 'can_pin_messages' admin right in a supergroup or 'can_edit_messages'
    /// admin right in a channel. Returns True on success.
    async fn unpin_chat_message(&self, data: UnpinChatMessage) -> Result<bool> {
        self.post(
            APIEndpoint::UnpinChatMessage,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to clear the list of pinned messages in a chat. If the
    /// chat is not a private chat, the bot must be an administrator in the
    /// chat for this to work and must have the 'can_pin_messages' admin
    /// right in a supergroup or 'can_edit_messages' admin right in a
    /// channel. Returns True on success.
    async fn unpin_all_chat_messages(&self, data: UnpinAllChatMessages) -> Result<bool> {
        self.post(
            APIEndpoint::UnpinAllChatMessages,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method for your bot to leave a group, supergroup or channel.
    /// Returns True on success.
    async fn leave_chat(&self, data: LeaveChat) -> Result<bool> {
        self.post(APIEndpoint::LeaveChat, Some(serde_json::to_value(data)?))
            .await?
            .into()
    }

    /// Use this method to get up to date information about the chat
    /// (current name of the user for one-on-one conversations, current username
    /// of a user, group or channel, etc.). Returns a [`Chat`] object on
    /// success.
    async fn get_chat(&self, data: GetChat) -> Result<Chat> {
        self.get(APIEndpoint::GetChat, Some(serde_json::to_value(data)?))
            .await?
            .into()
    }

    /// Use this method to get a list of administrators in a chat.
    /// On success, returns a `Vec<`[`ChatMember`]`>` that contains information
    /// about all chat administrators except other bots. If the chat is a
    /// group or a supergroup and no administrators were appointed, only the
    /// creator will be returned.
    async fn get_chat_administrators(
        &self,
        data: GetChatAdministrators,
    ) -> Result<Vec<ChatMember>> {
        self.get(
            APIEndpoint::GetChatAdministrators,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to get the number of members in a chat. Returns i64 on
    /// success.
    async fn get_members_count(&self, data: GetChatMembersCount) -> Result<i64> {
        self.get(
            APIEndpoint::GetChatMembersCount,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to get information about a member of a chat. Returns a
    /// [`ChatMember`] object on success.
    async fn get_chat_member(&self, data: GetChatMember) -> Result<ChatMember> {
        self.get(
            APIEndpoint::GetChatMember,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to set a new group sticker set for a supergroup.
    /// The bot must be an administrator in the chat for this to work and must
    /// have the appropriate admin rights. Use the field can_set_sticker_set
    /// optionally returned in [`API::get_chat`] requests to check if the bot
    /// can use this method. Returns True on success.
    async fn set_chat_sticker_set(&self, data: SetChatStickerSet) -> Result<bool> {
        self.post(
            APIEndpoint::SetChatStickerSet,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to delete a group sticker set from a supergroup.
    /// The bot must be an administrator in the chat for this to work and must
    /// have the appropriate admin rights. Use the field can_set_sticker_set
    /// optionally returned in [`API::get_chat`] requests to check if the bot
    /// can use this method. Returns True on success.
    async fn delete_chat_sticker_set(&self, data: DeleteChatStickerSet) -> Result<bool> {
        self.post(
            APIEndpoint::DeleteChatStickerSet,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to send answers to callback queries sent from [inline keyboards](https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating).
    /// The answer will be displayed to the user as a notification at the top of
    /// the chat screen or as an alert. On success, True is returned.
    async fn answer_callback_query(&self, data: AnswerCallbackQuery) -> Result<bool> {
        self.post(
            APIEndpoint::AnswerCallbackQuery,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to send static .WEBP or animated .TGS stickers. On
    /// success, the sent [Message] is returned.
    async fn send_sticker(&self, data: SendSticker) -> Result<Message> {
        match &data.sticker {
            InputFile::String(_) => self
                .post(APIEndpoint::SendSticker, Some(serde_json::to_value(&data)?))
                .await?
                .into(),
            InputFile::File(f) => self
                .post_file(
                    APIEndpoint::SendSticker,
                    Some(serde_json::to_value(&data)?),
                    Some(vec![f.clone()]),
                )
                .await?
                .into(),
        }
    }

    /// Use this method to get a sticker set. On success, a [StickerSet] object
    /// is returned.
    async fn get_sticker_set(&self, data: GetStickerSet) -> Result<StickerSet> {
        self.post(
            APIEndpoint::GetStickerSet,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to upload a .PNG file with a sticker for later use in
    /// createNewStickerSet and addStickerToSet methods (can be used
    /// multiple times). Returns the uploaded [File] on success.
    async fn upload_sticker_file(&self, data: UploadStickerFile) -> Result<File> {
        match &data.png_sticker {
            InputFile::File(f) => self
                .post_file(
                    APIEndpoint::UploadStickerFile,
                    Some(serde_json::to_value(&data)?),
                    Some(vec![f.clone()]),
                )
                .await?
                .into(),
            _ => Err(TelegramError::InvalidArgument(
                "upload_sticker_file only accepts files, not urls/ids".to_owned(),
            )
            .into()),
        }
    }

    /// Use this method to create a new sticker set owned by a user.
    /// The bot will be able to edit the sticker set thus created.
    /// You must use exactly one of the fields png_sticker or tgs_sticker.
    /// Returns True on success.
    async fn create_new_sticker_set(&self, data: CreateNewStickerSet) -> Result<bool> {
        if (data.png_sticker.is_some() && data.tgs_sticker.is_some())
            || (data.png_sticker.is_none() && data.tgs_sticker.is_none())
        {
            return Err(TelegramError::InvalidArgument(
                "You must use exactly one of the fields png_sticker or tgs_sticker".to_owned(),
            )
            .into());
        }

        let mut files = Vec::new();

        if data.png_sticker.is_some() {
            if let InputFile::File(f) = data.png_sticker.as_ref().unwrap() {
                files.push(f.clone());
            }
        }

        if data.tgs_sticker.is_some() {
            match data.tgs_sticker.as_ref().unwrap() {
                InputFile::File(f) => files.push(f.clone()),
                _ => {
                    return Err(TelegramError::InvalidArgument(
                        "tgs_sticker only accepts files, not urls/ids".to_owned(),
                    )
                    .into())
                },
            }
        }

        self.post_file(
            APIEndpoint::CreateNewStickerSet,
            Some(serde_json::to_value(&data)?),
            Some(files),
        )
        .await?
        .into()
    }

    /// Use this method to add a new sticker to a set created by the bot.
    /// You must use exactly one of the fields png_sticker or tgs_sticker.
    /// Animated stickers can be added to animated sticker sets and only to
    /// them. Animated sticker sets can have up to 50 stickers. Static
    /// sticker sets can have up to 120 stickers. Returns True on success.
    async fn add_sticker_to_set(&self, data: AddStickerToSet) -> Result<bool> {
        if (data.png_sticker.is_some() && data.tgs_sticker.is_some())
            || (data.png_sticker.is_none() && data.tgs_sticker.is_none())
        {
            return Err(TelegramError::InvalidArgument(
                "You must use exactly one of the fields png_sticker or tgs_sticker.".to_owned(),
            )
            .into());
        }

        let mut files = Vec::new();

        if data.png_sticker.is_some() {
            if let InputFile::File(f) = data.png_sticker.as_ref().unwrap() {
                files.push(f.clone());
            }
        }

        if data.tgs_sticker.is_some() {
            match data.tgs_sticker.as_ref().unwrap() {
                InputFile::File(f) => files.push(f.clone()),
                _ => {
                    return Err(TelegramError::InvalidArgument(
                        "tgs_sticker only accepts files, not urls/ids.".to_owned(),
                    )
                    .into())
                },
            }
        }

        self.post_file(
            APIEndpoint::AddStickerToSet,
            Some(serde_json::to_value(&data)?),
            Some(files),
        )
        .await?
        .into()
    }

    /// Use this method to move a sticker in a set created by the bot to a
    /// specific position. Returns True on success.
    async fn set_sticker_position_in_set(&self, data: SetStickerPositionInSet) -> Result<bool> {
        self.post(
            APIEndpoint::SetStickerPositionInSet,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to delete a sticker from a set created by the bot.
    /// Returns True on success.
    async fn delete_sticker_from_set(&self, data: DeleteStickerFromSet) -> Result<bool> {
        self.post(
            APIEndpoint::DeleteStickerFromSet,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to set the thumbnail of a sticker set.
    /// Animated thumbnails can be set for animated sticker sets only. Returns
    /// True on success.
    async fn set_sticker_set_thumb(&self, data: SetStickerSetThumb) -> Result<bool> {
        match &data.thumb {
            Some(InputFile::String(_)) | None => self
                .post(
                    APIEndpoint::SetStickerSetThumb,
                    Some(serde_json::to_value(&data)?),
                )
                .await?
                .into(),
            Some(InputFile::File(f)) => self
                .post_file(
                    APIEndpoint::SetStickerSetThumb,
                    Some(serde_json::to_value(&data)?),
                    Some(vec![f.clone()]),
                )
                .await?
                .into(),
        }
    }

    /// Use this method to send answers to an inline query. On success, True is
    /// returned. No more than 50 results per query are allowed.
    async fn answer_inline_query(&self, data: AnswerInlineQuery) -> Result<bool> {
        if data.results.len() > 50 {
            return Err(TelegramError::InvalidArgument(
                "No more than 50 results per query are allowed.".to_owned(),
            )
            .into());
        }

        self.post(
            APIEndpoint::AnswerInlineQuery,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to send invoices. On success, the sent [Message] is
    /// returned.
    async fn send_invoice(&self, data: SendInvoice) -> Result<Message> {
        self.post(APIEndpoint::SendInvoice, Some(serde_json::to_value(data)?))
            .await?
            .into()
    }

    /// If you sent an invoice requesting a shipping address and the parameter
    /// is_flexible was specified, the Bot API will send an [Update] with a
    /// shipping_query field to the bot. Use this method to reply to
    /// shipping queries. On success, True is returned.
    async fn answer_shipping_query(&self, data: AnswerShippingQuery) -> Result<bool> {
        self.post(
            APIEndpoint::AnswerShippingQuery,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Once the user has confirmed their payment and shipping details, the Bot
    /// API sends the final confirmation in the form of an [Update] with the
    /// field pre_checkout_query. Use this method to respond to such
    /// pre-checkout queries. On success, True is returned.
    /// **Note:** The Bot API must receive an answer within 10 seconds after the
    /// pre-checkout query was sent.
    async fn answer_pre_checkout_query(&self, data: AnswerPreCheckoutQuery) -> Result<bool> {
        self.post(
            APIEndpoint::AnswerPreCheckoutQuery,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Use this method to send a game. On success, the sent [Message] is
    /// returned.
    async fn send_game(&self, data: SendGame) -> Result<Message> {
        self.post(APIEndpoint::SendGame, Some(serde_json::to_value(data)?))
            .await?
            .into()
    }

    /// Use this method to set the score of the specified user in a game.
    /// On success, if the message was sent by the bot, returns the edited
    /// Message, otherwise returns True. Returns an error, if the new score
    /// is not greater than the user's current score in the chat and force is
    /// False.
    async fn set_game_score(&self, data: SetGameScore) -> Result<TrueOrObject<Message>> {
        self.post(APIEndpoint::SetGameScore, Some(serde_json::to_value(data)?))
            .await?
            .into()
    }

    /// Use this method to get data for high score tables. Will return the score
    /// of the specified user and several of his neighbors in a game.
    /// On success, returns a Vec of [GameHighScore] objects.
    async fn get_game_high_scores(&self, data: GetGameHighScores) -> Result<Vec<GameHighScore>> {
        self.post(
            APIEndpoint::GetGameHighScores,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }

    /// Informs a user that some of the Telegram Passport elements they provided
    /// contains errors. The user will not be able to re-submit their
    /// Passport to you until the errors are fixed (the contents of the
    /// field for which you returned the error must change). Returns True on
    /// success.
    ///
    /// Use this if the data submitted by the user doesn't satisfy the standards
    /// your service requires for any reason. For example, if a birthday
    /// date seems invalid, a submitted document is blurry, a scan shows
    /// evidence of tampering, etc. Supply some details in the error message
    /// to make sure the user knows how to correct the issues.
    async fn set_passport_data_errors(&self, data: SetPassportDataErrors) -> Result<bool> {
        self.post(
            APIEndpoint::SetPassportDataErrors,
            Some(serde_json::to_value(data)?),
        )
        .await?
        .into()
    }
}
