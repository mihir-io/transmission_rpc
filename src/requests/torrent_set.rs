use types::Priority;
use serde_json::{Map, Number, Value};
use super::{Request, RequestArguments};

#[derive(Clone)]
pub struct TorrentSet {
    _ids: Vec<u64>,
    _fields: Map<String, Value>,
}

macro_rules! set_method {
    ($method:ident, $field:expr, Bool, $t:ty) => {
        pub fn $method(mut self, p: $t) -> Self {
            self._fields.insert($field.to_string(), Value::Bool(p.into()));
            self
        }
    };
    ($method:ident, $field:expr, String, $t:ty) => {
        pub fn $method(mut self, p: $t) -> Self {
            self._fields.insert($field.to_string(), Value::String(p.into()));
            self
        }
    };
    ($method:ident, $field:expr, f64, $t:ty) => {
        pub fn $method(mut self, p: $t) -> Self {
            self._fields.insert($field.to_string(), Value::Number(Number::from_f64(p as f64).unwrap()));
            self
        }
    };
    ($method:ident, $field:expr, $v:ident, $t:ty) => {
        pub fn $method(mut self, p: $t) -> Self {
            self._fields.insert($field.to_string(), Value::Number((p as $v).into()));
            self
        }
    }
}

impl TorrentSet {
    pub fn new() -> TorrentSet {
        TorrentSet {
            _ids: Vec::new(),
            _fields: Map::new(),
        }
    }

    pub fn id(mut self, id: u64) -> Self {
        self._ids.push(id);
        self
    }

    pub fn ids(mut self, ids: Vec<u64>) -> Self {
        self._ids = ids;
        self
    }

    // TODO: Implement the rest of the setter methods
    set_method!(set_bandwidth_priority, "bandwidthPriority", i64, Priority);
    set_method!(set_download_limit, "downloadLimit", u64, u32);
    set_method!(set_download_limited, "downloadLimited", Bool, bool);
    //set_method!(set_wanted_files, wanted_files, array);
    //set_method!(set_unwanted_files, unwanted_files, array);
    set_method!(set_honors_session_limits, "honorsSessionLimits", Bool, bool);
    set_method!(set_location, "location", String, String);
    set_method!(set_peer_limit, "peer-limit", u64, u32);
    set_method!(set_queue_position, "queuePosition", u64, u32);
    set_method!(set_seed_idle_limit, "seedIdleLimit", u64, u32);
    //set_method!(set_seed_idle_mode, seed_idle_mode, number);
    set_method!(set_seed_ratio_limit, "seedRatioLimit", f64, f64);
    //set_method!(set_seed_ratio_mode, seed_ratio_mode, number);
    //set_method!(set_tracker_add, tracker_add, array);
    //set_method!(set_tracker_remove, tracker_remove, array);
    //set_method!(set_tracker_replace, tracker_replace, array);
    set_method!(set_upload_limit, "uploadLimit", u64, u32);
    set_method!(set_upload_limited, "uploadLimited", Bool, bool);
}

#[derive(Deserialize, Debug)]
pub struct TorrentSetResponse;

impl Request for TorrentSet {
    type Response = TorrentSetResponse;

    fn method_name(&self) -> &'static str { "torrent-set" }
}

impl RequestArguments for TorrentSet {
    fn arguments(&self) -> Value {
        Value::Object({
            let mut obj = self._fields.clone();
            obj.insert("ids".to_string(), Value::Array(self._ids.clone().into_iter()
                                                       .map(|x| Value::Number((x as u64).into())).collect()));
            obj
        })
    } 
}
