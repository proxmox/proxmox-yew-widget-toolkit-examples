use std::cell::RefCell;
use std::rc::Rc;

use geojson::GeoJson;

use pwt::css::FlexFit;
use pwt::prelude::*;
use pwt::state::SharedStateObserver;
use pwt::widget::charts::{Location, MapPointData, PointsRenderArgs, WorldMap, WorldPoint};
use pwt::widget::Fa;

use super::pin_info::PinInfo;
use crate::data::{find_destination, DESTINATIONS};
use crate::notice::notice;
use crate::{app_state, observe_app_state, AppState};

/// All destinations that match the filter on a world map.
#[derive(Properties, PartialEq, Default)]
pub struct DestinationMap {}

impl DestinationMap {
    pub fn new() -> Self {
        Self {}
    }
}

into_vnode!(DestinationMap => DestinationMapComp);

/// Copied into the build by a hook in Trunk.toml.
const WORLD_MAP_URL: &str = "geojson/world-map.json";

thread_local! {
    // loaded once, the map is shown again on every switch to the map view
    static WORLD_MAP: RefCell<Option<Rc<GeoJson>>> = const { RefCell::new(None) };
}

async fn load_world_map() -> Result<Rc<GeoJson>, String> {
    let response = gloo_net::http::Request::get(WORLD_MAP_URL)
        .send()
        .await
        .map_err(|err| err.to_string())?;
    if !response.ok() {
        return Err(format!("{} {}", response.status(), response.status_text()));
    }
    let world_map: GeoJson = response.json().await.map_err(|err| err.to_string())?;
    Ok(Rc::new(world_map))
}

#[derive(Clone, PartialEq)]
struct Pin(&'static str);

impl MapPointData for Pin {
    fn render_title(&self) -> AttrValue {
        find_destination(self.0)
            .map(|destination| destination.name)
            .unwrap_or_default()
            .into()
    }

    fn render_info(args: &PointsRenderArgs<Self>) -> Html {
        PinInfo::new(args.points.iter().map(|point| point.data.0).collect()).into()
    }
}

enum Msg {
    StateChanged,
    Loaded(Result<Rc<GeoJson>, String>),
}

struct DestinationMapComp {
    _observer: SharedStateObserver<AppState>,
    world_map: Option<Result<Rc<GeoJson>, String>>,
}

impl Component for DestinationMapComp {
    type Message = Msg;
    type Properties = DestinationMap;

    fn create(ctx: &Context<Self>) -> Self {
        let _observer = observe_app_state(ctx.link(), || Msg::StateChanged);
        let world_map = WORLD_MAP.with(|map| map.borrow().clone()).map(Ok);
        if world_map.is_none() {
            ctx.link()
                .send_future(async { Msg::Loaded(load_world_map().await) });
        }
        Self {
            _observer,
            world_map,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StateChanged => {}
            Msg::Loaded(result) => {
                if let Ok(map) = &result {
                    WORLD_MAP.with(|cache| *cache.borrow_mut() = Some(map.clone()));
                }
                self.world_map = Some(result);
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.world_map {
            None => notice(Fa::new("spinner").pulse(), "Loading the map...").into(),
            Some(Err(err)) => notice(
                Fa::new("exclamation-triangle"),
                format!("Unable to load the world map ({err}). Is proxmox-geojson-data installed?"),
            )
            .into(),
            Some(Ok(map)) => {
                let filter = app_state().read().filter.clone();
                let points: Vec<WorldPoint<Pin>> = DESTINATIONS
                    .iter()
                    .filter(|destination| filter.matches(destination))
                    .map(|destination| WorldPoint {
                        location: Location::new(destination.longitude, destination.latitude),
                        data: Pin(destination.id),
                    })
                    .collect();
                WorldMap::new(map.clone())
                    .class(FlexFit)
                    .points(points)
                    .into()
            }
        }
    }
}
