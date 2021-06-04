pub mod card_div;
pub mod line_chart;
pub mod map;
pub mod paged_list;

#[macro_export]
macro_rules! create_fetch_task {
    ( $self:ident, $url:expr, $req:expr, $res_ty:ty, $res_msg:ident $(,)? ) => {
        let body = serde_json::to_value(&$req).unwrap();
        let request = Request::post($url)
            .header("Content-Type", "application/json")
            .body(Json(&body))
            .expect("Failed to construct fetch task");
        let callback = $self
            .link
            .callback(|response: Response<Json<anyhow::Result<$res_ty>>>| {
                let Json(data) = response.into_body();
                if let Ok(result) = data {
                    Msg::$res_msg(result)
                } else {
                    Msg::$res_msg(<$res_ty>::err("Unknown error"))
                }
            });
        let task = FetchService::fetch(request, callback).expect("Failed to start request");
        $self.fetch_task = Some(task);
    };
    ( $self:ident, $url:expr, $req:expr, $res_msg:ident $(,)? ) => {
        crate::create_fetch_task!($self, $url, $req, SimpleResponse, $res_msg)
    };
}

#[macro_export]
macro_rules! fluent {
    ( $lang:expr, $text_id:expr, { $($key:expr => $value:expr),* $(,)? } ) => {
        {
            let args = maplit::hashmap! {
                $($key => $value.into(),)*
            };
            LOCALES.lookup_with_args(&$lang, $text_id, &args)
        }
    };
    ( $lang:expr, $text_id:expr ) => {
        {
            LOCALES.lookup(&$lang, $text_id)
        }
    };
}
