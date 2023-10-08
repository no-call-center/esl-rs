use serde_json::Value;
use std::{collections::HashMap, ops::Deref};

pub(crate) fn get_header_end(s: &[u8]) -> Option<usize> {
    let mut i = 0;
    let mut last = 0;
    for c in s {
        if *c == b'\n' {
            if last == b'\n' {
                return Some(i + 1);
            }
            last = *c;
        } else {
            last = *c;
        }
        i += 1;
    }
    None
}

pub(crate) fn parse_header(header: &[u8]) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let mut key = String::new();
    let mut value = String::new();
    let mut is_key = true;
    for c in header {
        if *c == b':' {
            is_key = false;
        } else if *c == b'\n' {
            map.insert(key, value);
            key = String::new();
            value = String::new();
            is_key = true;
        } else if is_key {
            key.push(*c as char);
        } else {
            value.push(*c as char);
        }
    }
    map
}

/*


    public final static String API = "API";

    public final static String HEARTBEAT = "HEARTBEAT";

    public final static String RECV_RTCP_MESSAGE = "RECV_RTCP_MESSAGE";

    public final static String CHANNEL_CREATE = "CHANNEL_CREATE";

    public final static String CHANNEL_ORIGINATE = "CHANNEL_ORIGINATE";

    public final static String CHANNEL_STATE = "CHANNEL_STATE";

    public final static String CHANNEL_PROGRESS = "CHANNEL_PROGRESS";

    public final static String CHANNEL_CALLSTATE = "CHANNEL_CALLSTATE";

    public final static String CALL_UPDATE = "CALL_UPDATE";

    public final static String CHANNEL_EXECUTE = "CHANNEL_EXECUTE";

    public final static String CHANNEL_PARK = "CHANNEL_PARK";

    public final static String CHANNEL_UNPARK = "CHANNEL_UNPARK";

    public final static String PRIVATE_COMMAND = "PRIVATE_COMMAND";

    public final static String PRESENCE_IN = "PRESENCE_IN";

    public final static String CHANNEL_EXECUTE_COMPLETE = "CHANNEL_EXECUTE_COMPLETE";

    public final static String CHANNEL_HANGUP = "CHANNEL_HANGUP";

    public final static String CHANNEL_HANGUP_COMPLETE = "CHANNEL_HANGUP_COMPLETE";

    public final static String CHANNEL_OUTGOING = "CHANNEL_OUTGOING";

    public final static String CHANNEL_ANSWER = "CHANNEL_ANSWER";

    public final static String CHANNEL_DESTROY = "CHANNEL_DESTROY";

    public final static String CHANNEL_BRIDGE = "CHANNEL_BRIDGE";

    public final static String RECORD_START = "RECORD_START";

    public final static String MEDIA_BUG_START = "MEDIA_BUG_START";

    public final static String MEDIA_BUG_STOP = "MEDIA_BUG_STOP";

    public final static String PLAYBACK_START = "PLAYBACK_START";

    public final static String PLAYBACK_STOP = "PLAYBACK_STOP";

    public final static String CHANNEL_UNBRIDGE = "CHANNEL_UNBRIDGE";

    public final static String DETECTED_SPEECH = "DETECTED_SPEECH";

    public final static String CODEC = "CODEC";

    public final static String RECV_INFO = "RECV_INFO";

    public final static String DTMF = "DTMF";

    public final static String DTMF_SEND = "DTMF_SEND";

    public final static String SEND_DTMF = "SEND_DTMF";

    public final static String DEL_SCHEDULE = "DEL_SCHEDULE";

    public final static String RE_SCHEDULE = "RE_SCHEDULE";

    public final static String ADD_SCHEDULE = "ADD_SCHEDULE";

    public final static String CHANNEL_PROGRESS_MEDIA = "CHANNEL_PROGRESS_MEDIA";

    public final static String RECORD_STOP = "RECORD_STOP";

    public final static String CUSTOM = "CUSTOM";

    public final static String RING_ASR = "RING_ASR";

    public final static String RELOADXML = "RELOADXML";

    public final static String CHANNEL_HOLD = "CHANNEL_HOLD";

    public final static String CHANNEL_UNHOLD = "CHANNEL_UNHOLD";

    public final static String CONFIRMED = "confirmed";

    public final static String CONFERENCE_SEND_PRESENCE = "conference_send_presence";

*/
#[derive(Debug, Clone, strum::Display)]
pub enum Event {
    Api(EventData),
    Heartbeat(EventData),
    RecvRtcpMessage(EventData),
    ChannelCreate(EventData),
    ChannelOriginate(EventData),
    ChannelState(EventData),
    ChannelProgress(EventData),
    ChannelCallState(EventData),
    CallUpdate(EventData),
    ChannelExecute(EventData),
    ChannelPark(EventData),
    ChannelUnpark(EventData),
    PrivateCommand(EventData),
    PresenceIn(EventData),
    ChannelExecuteComplete(EventData),
    ChannelHangup(EventData),
    ChannelHangupComplete(EventData),
    ChannelOutgoing(EventData),
    ChannelAnswer(EventData),
    ChannelDestroy(EventData),
    ChannelBridge(EventData),
    RecordStart(EventData),
    MediaBugStart(EventData),
    MediaBugStop(EventData),
    PlaybackStart(EventData),
    PlaybackStop(EventData),
    ChannelUnbridge(EventData),
    DetectedSpeech(EventData),
    Codec(EventData),
    RecvInfo(EventData),
    Dtmf(EventData),
    DtmfSend(EventData),
    SendDtmf(EventData),
    DelSchedule(EventData),
    ReSchedule(EventData),
    AddSchedule(EventData),
    ChannelProgressMedia(EventData),
    RecordStop(EventData),
    Custom(EventData),
    RingAsr(EventData),
    ReloadXml(EventData),
    ChannelHold(EventData),
    ChannelUnhold(EventData),
    Confirmed(EventData),
    ConferenceSendPresence(EventData),
    Unknown(EventData),
}

impl Deref for Event {
    type Target = EventData;

    fn deref(&self) -> &Self::Target {
        match self {
            Event::Api(data)
            | Event::Heartbeat(data)
            | Event::RecvRtcpMessage(data)
            | Event::ChannelCreate(data)
            | Event::ChannelOriginate(data)
            | Event::ChannelState(data)
            | Event::ChannelProgress(data)
            | Event::ChannelCallState(data)
            | Event::CallUpdate(data)
            | Event::ChannelExecute(data)
            | Event::ChannelPark(data)
            | Event::ChannelUnpark(data)
            | Event::PrivateCommand(data)
            | Event::PresenceIn(data)
            | Event::ChannelExecuteComplete(data)
            | Event::ChannelHangup(data)
            | Event::ChannelHangupComplete(data)
            | Event::ChannelOutgoing(data)
            | Event::ChannelAnswer(data)
            | Event::ChannelDestroy(data)
            | Event::ChannelBridge(data)
            | Event::RecordStart(data)
            | Event::MediaBugStart(data)
            | Event::MediaBugStop(data)
            | Event::PlaybackStart(data)
            | Event::PlaybackStop(data)
            | Event::ChannelUnbridge(data)
            | Event::DetectedSpeech(data)
            | Event::Codec(data)
            | Event::RecvInfo(data)
            | Event::Dtmf(data)
            | Event::DtmfSend(data)
            | Event::SendDtmf(data)
            | Event::DelSchedule(data)
            | Event::ReSchedule(data)
            | Event::AddSchedule(data)
            | Event::ChannelProgressMedia(data)
            | Event::RecordStop(data)
            | Event::Custom(data)
            | Event::RingAsr(data)
            | Event::ReloadXml(data)
            | Event::ChannelHold(data)
            | Event::ChannelUnhold(data)
            | Event::Confirmed(data)
            | Event::ConferenceSendPresence(data)
            | Event::Unknown(data) => data,
        }
    }
}

impl From<EventData> for Event {
    fn from(value: EventData) -> Self {
        match value.get_event_name().as_ref().map(|s| s.as_str()) {
            Some("API") => Self::Api(value),
            Some("HEARTBEAT") => Self::Heartbeat(value),
            Some("RECV_RTCP_MESSAGE") => Self::RecvRtcpMessage(value),
            Some("CHANNEL_CREATE") => Self::ChannelCreate(value),
            Some("CHANNEL_ORIGINATE") => Self::ChannelOriginate(value),
            Some("CHANNEL_STATE") => Self::ChannelState(value),
            Some("CHANNEL_PROGRESS") => Self::ChannelProgress(value),
            Some("CHANNEL_CALLSTATE") => Self::ChannelCallState(value),
            Some("CALL_UPDATE") => Self::CallUpdate(value),
            Some("CHANNEL_EXECUTE") => Self::ChannelExecute(value),
            Some("CHANNEL_PARK") => Self::ChannelPark(value),
            Some("CHANNEL_UNPARK") => Self::ChannelUnpark(value),
            Some("PRIVATE_COMMAND") => Self::PrivateCommand(value),
            Some("PRESENCE_IN") => Self::PresenceIn(value),
            Some("CHANNEL_EXECUTE_COMPLETE") => Self::ChannelExecuteComplete(value),
            Some("CHANNEL_HANGUP") => Self::ChannelHangup(value),
            Some("CHANNEL_HANGUP_COMPLETE") => Self::ChannelHangupComplete(value),
            Some("CHANNEL_OUTGOING") => Self::ChannelOutgoing(value),
            Some("CHANNEL_ANSWER") => Self::ChannelAnswer(value),
            Some("CHANNEL_DESTROY") => Self::ChannelDestroy(value),
            Some("CHANNEL_BRIDGE") => Self::ChannelBridge(value),
            Some("RECORD_START") => Self::RecordStart(value),
            Some("MEDIA_BUG_START") => Self::MediaBugStart(value),
            Some("MEDIA_BUG_STOP") => Self::MediaBugStop(value),
            Some("PLAYBACK_START") => Self::PlaybackStart(value),
            Some("PLAYBACK_STOP") => Self::PlaybackStop(value),
            Some("CHANNEL_UNBRIDGE") => Self::ChannelUnbridge(value),
            Some("DETECTED_SPEECH") => Self::DetectedSpeech(value),
            Some("CODEC") => Self::Codec(value),
            Some("RECV_INFO") => Self::RecvInfo(value),
            Some("DTMF") => Self::Dtmf(value),
            Some("DTMF_SEND") => Self::DtmfSend(value),
            Some("SEND_DTMF") => Self::SendDtmf(value),
            Some("DEL_SCHEDULE") => Self::DelSchedule(value),
            Some("RE_SCHEDULE") => Self::ReSchedule(value),
            Some("ADD_SCHEDULE") => Self::AddSchedule(value),
            Some("CHANNEL_PROGRESS_MEDIA") => Self::ChannelProgressMedia(value),
            Some("RECORD_STOP") => Self::RecordStop(value),
            Some("CUSTOM") => Self::Custom(value),
            Some("RING_ASR") => Self::RingAsr(value),
            Some("RELOADXML") => Self::ReloadXml(value),
            Some("CHANNEL_HOLD") => Self::ChannelHold(value),
            Some("CHANNEL_UNHOLD") => Self::ChannelUnhold(value),
            Some("CONFIRMED") => Self::Confirmed(value),
            Some("CONFERENCE_SEND_PRESENCE") => Self::ConferenceSendPresence(value),
            _ => Self::Unknown(value), // 处理未知事件
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct EventData {
    pub headers: HashMap<String, String>,
    pub raw_body: Option<String>,
    pub body: Option<HashMap<String, String>>,
}

impl EventData {
    pub fn new(headers: HashMap<String, String>, raw_body: Option<String>) -> Self {
        // 如果有body，则json反序列化解析body
        let body = if let Some(body) = raw_body.clone() {
            if let Ok(body) = serde_json::from_str::<Value>(&body) {
                let mut map = HashMap::new();
                if let Value::Object(body) = body {
                    for (k, v) in body {
                        if let Value::String(v) = v {
                            map.insert(k, v);
                        }
                    }
                }
                Some(map)
            } else {
                None
            }
        } else {
            None
        };
        Self {
            headers,
            body,
            raw_body,
        }
    }

    pub fn get_header(&self, key: &str) -> Option<String> {
        self.headers.get(key).map(|s| s.to_string())
    }

    pub fn get_job_uuid(&self) -> Option<String> {
        self.get_header(&"Job-UUID").map(|s| s.to_string())
    }

    pub fn get_body(&self) -> Option<&HashMap<String, String>> {
        self.body.as_ref()
    }

    pub fn get_body_by_key(&self, key: &str) -> Option<String> {
        self.get_body()
            .and_then(|v| v.get(key))
            .map(|s| s.to_string())
    }

    pub fn get_event_name(&self) -> Option<String> {
        self.get_body_by_key("Event-Name")
    }

    pub fn get_channel_call_uuid(&self) -> Option<String> {
        self.get_body_by_key("Channel-Call-UUID")
    }

    pub fn get_var(&self, key: &str) -> Option<String> {
        self.get_body_by_key(&format!("variable_{}", key))
    }
}
