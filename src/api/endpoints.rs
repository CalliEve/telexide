/// This enum represents all the telegram API endpoints.
///
/// It is mostly used for letting the get and post methods in the API trait know
/// how to form the endpoint path
pub enum APIEndpoint {
    GetUpdates,
    GetMe,
    LogOut,
    Close,
    SendMessage,
    SetMyCommands,
    GetMyCommands,
    ForwardMessage,
    CopyMessage,
    SendPhoto,
    SendAudio,
    SendDocument,
    SendVideo,
    SendAnimation,
    SendVoice,
    SendVideoNote,
    SendMediaGroup,
    SendLocation,
    EditMessageLiveLocation,
    StopMessageLiveLocation,
    SendVenue,
    SendContact,
    SendPoll,
    SendDice,
    SendChatAction,
    GetUserProfilePhotos,
    GetFile,
    KickChatMember,
    UnbanChatMember,
    RestrictChatMember,
    PromoteChatMember,
    SetChatAdministratorCustomTitle,
    SetChatPermissions,
    ExportChatInviteLink,
    SetChatPhoto,
    DeleteChatPhoto,
    SetChatTitle,
    SetChatDescription,
    PinChatMessage,
    UnpinChatMessage,
    UnpinAllChatMessages,
    LeaveChat,
    GetChat,
    GetChatAdministrators,
    GetChatMembersCount,
    GetChatMember,
    SetChatStickerSet,
    DeleteChatStickerSet,
    AnswerCallbackQuery,
    EditMessageText,
    EditMessageCaption,
    EditMessageMedia,
    EditMessageReplyMarkup,
    StopPoll,
    DeleteMessage,
    SendSticker,
    GetStickerSet,
    UploadStickerFile,
    CreateNewStickerSet,
    AddStickerToSet,
    SetStickerPositionInSet,
    DeleteStickerFromSet,
    SetStickerSetThumb,
    AnswerInlineQuery,
    SendInvoice,
    AnswerShippingQuery,
    AnswerPreCheckoutQuery,
    SendGame,
    SetGameScore,
    GetGameHighScores,
    SetWebhook,
    SetPassportDataErrors,
    DeleteWebhook,
    GetWebhookInfo,
    Other(String),
}

impl APIEndpoint {
    pub fn as_str(&self) -> &str {
        match *self {
            Self::GetUpdates => "getUpdates",
            Self::GetMe => "getMe",
            Self::LogOut => "logOut",
            Self::Close => "close",
            Self::SendMessage => "sendMessage",
            Self::SetMyCommands => "setMyCommands",
            Self::GetMyCommands => "getMyCommands",
            Self::CopyMessage => "copyMessage",
            Self::ForwardMessage => "forwardMessage",
            Self::SendPhoto => "sendPhoto",
            Self::SendAudio => "sendAudio",
            Self::SendDocument => "sendDocument",
            Self::SendVideo => "sendVideo",
            Self::SendAnimation => "sendAnimation",
            Self::SendVoice => "sendVoice",
            Self::SendVideoNote => "sendVideoNote",
            Self::SendMediaGroup => "sendMediaGroup",
            Self::SendLocation => "sendLocation",
            Self::EditMessageLiveLocation => "editMessageLiveLocation",
            Self::StopMessageLiveLocation => "stopMessageLiveLocation",
            Self::SendVenue => "sendVenue",
            Self::SendContact => "sendContact",
            Self::SendPoll => "sendPoll",
            Self::SendDice => "sendDice",
            Self::SendChatAction => "sendChatAction",
            Self::GetUserProfilePhotos => "getUserProfilePhotos",
            Self::GetFile => "getFile",
            Self::KickChatMember => "kickChatMember",
            Self::UnbanChatMember => "unbanChatMember",
            Self::RestrictChatMember => "restrictChatMember",
            Self::PromoteChatMember => "promoteChatMember",
            Self::SetChatAdministratorCustomTitle => "setChatAdministratorCustomTitle",
            Self::SetChatPermissions => "setChatPermissions",
            Self::ExportChatInviteLink => "exportChatInviteLink",
            Self::SetChatPhoto => "setChatPhoto",
            Self::DeleteChatPhoto => "deleteChatPhoto",
            Self::SetChatTitle => "setChatTitle",
            Self::SetChatDescription => "setChatDescription",
            Self::PinChatMessage => "pinChatMessage",
            Self::UnpinChatMessage => "unpinChatMessage",
            Self::UnpinAllChatMessages => "unpinAllChatMessages",
            Self::LeaveChat => "leaveChat",
            Self::GetChat => "getChat",
            Self::GetChatAdministrators => "getChatAdministrators",
            Self::GetChatMembersCount => "getChatMembersCount",
            Self::GetChatMember => "getChatMember",
            Self::SetChatStickerSet => "setChatStickerSet",
            Self::DeleteChatStickerSet => "deleteChatStickerSet",
            Self::AnswerCallbackQuery => "answerCallbackQuery",
            Self::EditMessageText => "editMessageText",
            Self::EditMessageCaption => "editMessageCaption",
            Self::EditMessageMedia => "editMessageMedia",
            Self::EditMessageReplyMarkup => "editMessageReplyMarkup",
            Self::StopPoll => "stopPoll",
            Self::DeleteMessage => "deleteMessage",
            Self::SendSticker => "sendSticker",
            Self::GetStickerSet => "getStickerSet",
            Self::UploadStickerFile => "uploadStickerFile",
            Self::CreateNewStickerSet => "createNewStickerSet",
            Self::AddStickerToSet => "addStickerToSet",
            Self::SetStickerPositionInSet => "setStickerPositionInSet",
            Self::DeleteStickerFromSet => "deleteStickerFromSet",
            Self::SetStickerSetThumb => "setStickerSetThumb",
            Self::AnswerInlineQuery => "answerInlineQuery",
            Self::SendGame => "sendGame",
            Self::SetGameScore => "setGameScore",
            Self::GetGameHighScores => "getGameHighScores",
            Self::SendInvoice => "sendInvoice",
            Self::AnswerShippingQuery => "answerShippingQuery",
            Self::AnswerPreCheckoutQuery => "answerPreCheckoutQuery",
            Self::SetWebhook => "setWebHook",
            Self::SetPassportDataErrors => "setPassportDataErrors",
            Self::DeleteWebhook => "deleteWebhook",
            Self::GetWebhookInfo => "getWebhookInfo",
            Self::Other(ref e) => e,
        }
    }
}

impl std::fmt::Display for APIEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for APIEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("telegram::APIEndpoint")
            .field(&self.as_str().to_owned())
            .finish()
    }
}

impl From<String> for APIEndpoint {
    fn from(string: String) -> APIEndpoint {
        APIEndpoint::Other(string)
    }
}
