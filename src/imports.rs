use wasm_bindgen::prelude::wasm_bindgen;
use crate::input_json::InputJson;
use crate::json_objects::chat_client::ChatClient;
use crate::json_objects::chat_message::ChatMessage;
use crate::json_objects::user::User;
use crate::json_objects::user_register_request::UserRegisterRequest;
use crate::json_objects::user_register_result::UserRegisterResult;
use crate::output_json::OutputJson;

// TODO import other owncast functions

#[wasm_bindgen]
extern "C" {
    // Chat
    #[wasm_bindgen(js_namespace = ["owncast", "chat"], js_name = send)]
    pub fn owncast_send_chat(_: &str);

    #[wasm_bindgen(js_namespace = ["owncast", "chat"], js_name = sendAction)]
    pub fn owncast_send_chat_action(_: &str);

    #[wasm_bindgen(js_namespace = ["owncast", "chat"], js_name = system)]
    pub fn owncast_send_chat_system(_: &str);

    #[wasm_bindgen(js_namespace = ["owncast", "chat"], js_name = sendTo)]
    pub fn owncast_send_chat_to(_: u64, _: &str);

    #[wasm_bindgen(js_namespace = ["owncast", "chat"], js_name = replyTo)]
    pub fn owncast_send_chat_reply_u64(_: u64, _: &str) -> bool;
    #[wasm_bindgen(js_namespace = ["owncast", "chat"], js_name = replyTo)]
    pub fn owncast_send_chat_reply(_: OutputJson<ChatMessage>, _: &str) -> bool;

    #[wasm_bindgen(js_namespace = ["owncast", "chat"], js_name = history)]
    pub fn owncast_chat_history(_: Option<u64>) -> InputJson<Vec<ChatMessage>>;

    #[wasm_bindgen(js_namespace = ["owncast", "chat"], js_name = deleteMessage)]
    pub fn owncast_delete_message(_: &str);

    #[wasm_bindgen(js_namespace = ["owncast", "chat"], js_name = kick)]
    pub fn owncast_kick_client(_: u64);

    #[wasm_bindgen(js_namespace = ["owncast", "chat"], js_name = clients)]
    pub fn owncast_chat_clients() -> InputJson<Vec<ChatClient>>;

    // Users
    #[wasm_bindgen(js_namespace = ["owncast", "users"], js_name = list)]
    pub fn owncast_users_list() -> InputJson<Vec<User>>;

    #[wasm_bindgen(js_namespace = ["owncast", "users"], js_name = get)]
    pub fn owncast_user_get(_: &str) -> InputJson<Option<User>>;

    #[wasm_bindgen(js_namespace = ["owncast", "users"], js_name = setEnabled)]
    pub fn owncast_user_set_enabled(_: &str, _: bool, _: Option<String>);

    #[wasm_bindgen(js_namespace = ["owncast", "users"], js_name = banIP)]
    pub fn owncast_ban_ip(_: &str);

    #[wasm_bindgen(js_namespace = ["owncast", "users"], js_name = register)]
    pub fn owncast_users_register(_: OutputJson<UserRegisterRequest>) -> InputJson<UserRegisterResult>;
    #[wasm_bindgen(js_namespace = ["owncast", "users"], js_name = register)]
    pub fn owncast_users_register_string(_: &str) -> InputJson<UserRegisterResult>;


    // Auth

    // Storage

    // FS

    // Fediverse
    #[wasm_bindgen(js_namespace = ["owncast", "fediverse"], js_name = post)]
    pub fn owncast_fediverse_post(_: &str) -> Option<String>;

    // Notifications

    // KV

    // Config

    // Assets

    // Events

    // Actions

    // SSE

    // Timer

    // HTTP

    // Stream

    // Server

    // Video Config
}

/*
users: {
    list(): User[];
    get(id: string): User | null;
    setEnabled(id: string, enabled: boolean, reason?: string): void;
    banIP(ip: string): void;
    register(opts: UserRegisterRequest | string): UserRegisterResult;
};
auth: {
    grantSession(opts: GrantSessionRequest | string): void;
    endSession(): void;
};
storage: {
    upload(name: string, data: Uint8Array | string): UploadResult | null;
};
fs: {
    read(path: string): Uint8Array | null;
    readText(path: string): string | null;
    write(path: string, data: Uint8Array | string): FsResult;
    list(dir: string): string[];
    delete(path: string): FsResult;
    exists(path: string): boolean;
};
notifications: {
    discord(text: string): void;
    browserPush(payload: string | BrowserPushPayload): void;
    fediverse(payload: FediversePayload): void;
};
kv: {
    get(key: string): string | null;
    set(key: string, value: string | number): void;
    getJSON<T = unknown>(key: string, fallback?: T): T;
    setJSON(key: string, value: unknown): void;
};
config: {
    get<T = unknown>(key: string, fallback?: T): T;
};
assets: {
    read(path: string): Uint8Array | null;
    readText(path: string): string | null;
};
events: {
    emit(eventType: string, payload: unknown): void;
};
actions: {
    add(actions: ActionButton | ActionButton[]): void;
    clear(): void;
};
sse: {
    send(channel: string, event: string, data: unknown): void;
};
timer: {
    setTimeout(fn: () => void, ms: number): number;
    setInterval(fn: () => void, ms: number): number;
    clear(id: number): void;
};
http: {
    fetch(url: string, opts?: HttpRequestOpts): HttpResponse;
};
stream: {
    current(): StreamInfo;
    broadcaster(): StreamBroadcaster;
};
server: {
    info(): ServerInfo;
    socials(): SocialHandle[];
    emotes(): Emote[];
    federation(): FederationInfo;
    tags(): string[];
};
videoConfig: {
    read(): VideoConfig;
    write(config: VideoConfigUpdate): void;
};
 */