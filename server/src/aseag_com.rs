use serde::{Serialize, Deserialize};

const JSON_BODY_EXAMPLE: &str = r#"{"id":"t5m4vn8qkw8iy44k","ver":"1.69","lang":"deu","auth":{"type":"AID","aid":"4vV1PaulHallo511icH"},"client":{"id":"AVV_AACHEN","type":"WEB","name":"webapp","l":"vs_aseag","v":20500},"formatted":false,"svcReqL":[{"meth":"TripSearch","req":{"jnyFltrL":[{"type":"GROUP","mode":"INC","value":"OEV"},{"type":"META","mode":"INC","meta":"notBarrierfree"},{"type":"PROD","mode":"INC","value":2033}],"getPolyline":true,"getPasslist":true,"gisFltrL":[{"type":"P","mode":"FB","profile":{"type":"F","maxdist":"2000"}},{"type":"M","mode":"FBT","meta":"foot_speed_normal"},{"type":"P","mode":"FB","profile":{"type":"B","maxdist":"5000"}},{"type":"M","mode":"FBT","meta":"bike_speed_normal"},{"type":"M","mode":"FBT","meta":"car_speed_normal"}],"depLocL":[{"lid":"A=1@O=Driescher Gässchen (RWTH Aachen), AC@X=6081805@Y=50779246@U=80@L=1027@B=1@p=1739229385@","name":"Driescher Gässchen (RWTH Aachen), AC","globalIdL":[{"id":"de:05334:1027","type":"A"}],"eteId":"dep_0|S|Driescher Gässchen (RWTH Aachen), AC|1027|6081805|50779246"}],"arrLocL":[{"lid":"A=1@O=Halifaxstraße@X=6058001@Y=50779534@U=80@L=1427@B=1@p=1739229385@","name":"Halifaxstraße","globalIdL":[{"id":"de:05334:1427","type":"A"}],"eteId":"arr_0|S|Halifaxstraße|1427|6058001|50779534"}],"outFrwd":true,"outTime":"121000","outDate":"20250211","liveSearch":false,"maxChg":"1000","minChgTime":"-1"},"id":"1|25|"}]}"#;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestBody {
    id: String,
    ver: String,
    lang: String,
    auth: Auth,
    client: Client,
    formatted: bool,
    svcReqL: Vec<ServiceRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Auth {
    #[serde(rename = "type")]
    auth_type: String,
    aid: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Client {
    id: String,
    #[serde(rename = "type")]
    client_type: String,
    name: String,
    l: String,
    v: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServiceRequest {
    meth: String,
    req: Request,
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    jnyFltrL: Vec<JourneyFilter>,
    getPolyline: bool,
    getPasslist: bool,
    gisFltrL: Vec<GisFilter>,
    depLocL: Vec<Location>,
    arrLocL: Vec<Location>,
    outFrwd: bool,
    outTime: String,
    outDate: String,
    liveSearch: bool,
    maxChg: String,
    minChgTime: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ValueType {
    StringValue(String),
    U32Value(u32),
}

#[derive(Debug, Serialize, Deserialize)]
struct JourneyFilter {
    #[serde(rename = "type")]
    filter_type: String,
    mode: String,
    value: Option<ValueType>,
    meta: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GisFilter {
    #[serde(rename = "type")]
    filter_type: String,
    mode: String,
    meta: Option<String>,
    profile: Option<Profile>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Profile {
    #[serde(rename = "type")]
    profile_type: String,
    maxdist: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Location {
    lid: String,
    name: String,
    globalIdL: Vec<GlobalId>,
    eteId: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GlobalId {
    id: String,
    #[serde(rename = "type")]
    id_type: String,
}

pub fn load_template_request_body() -> Result<RequestBody, serde_json::Error> {
    print!("start loading");
    let request_body: RequestBody = serde_json::from_str(JSON_BODY_EXAMPLE)?;
    Ok(request_body)
}