//! The destinations of the travel explorer.

/// The kind of a destination, used to filter them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Category {
    Mountains,
    Coast,
    Desert,
    Nature,
    City,
}

impl Category {
    pub const ALL: &'static [Category] = &[
        Category::Mountains,
        Category::Coast,
        Category::Desert,
        Category::Nature,
        Category::City,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Category::Mountains => "Mountains",
            Category::Coast => "Coast",
            Category::Desert => "Desert",
            Category::Nature => "Nature",
            Category::City => "City",
        }
    }

    /// Font Awesome icon name.
    pub fn icon(&self) -> &'static str {
        match self {
            Category::Mountains => "tree",
            Category::Coast => "anchor",
            Category::Desert => "sun-o",
            Category::Nature => "leaf",
            Category::City => "building",
        }
    }
}

/// A travel destination. The demo has a fixed set of them, see [DESTINATIONS].
pub struct Destination {
    /// Used in routes and as picture file name prefix.
    pub id: &'static str,
    pub name: &'static str,
    pub region: &'static str,
    pub country: &'static str,
    /// Where the destination is on the world map.
    pub latitude: f64,
    pub longitude: f64,
    /// The recommended stay.
    pub days: u32,
    /// The best time to travel, as shown to the user.
    pub season: &'static str,
    pub category: Category,
    /// The highlights of the destination.
    pub tags: &'static [&'static str],
    pub description: &'static str,
}

impl Destination {
    /// Every destination has this many pictures, the same scene at different times of the day.
    pub const PICTURES: usize = 3;

    /// The URL of picture `n`, counting from zero.
    pub fn picture(&self, n: usize) -> String {
        format!("images/{}-{}.svg", self.id, n + 1)
    }

    /// The region and the country, for example "Tyrol, Austria".
    pub fn place(&self) -> String {
        format!("{}, {}", self.region, self.country)
    }
}

pub static DESTINATIONS: &[Destination] = &[
    Destination {
        id: "alpine-lake",
        name: "Alpine Lake",
        region: "Tyrol",
        country: "Austria",
        latitude: 47.45,
        longitude: 11.71,
        days: 3,
        season: "June to September",
        category: Category::Mountains,
        tags: &["Hiking", "Swimming", "Cable cars"],
        description: "A turquoise lake between snow-capped peaks. Hike up through pine forests in \
            the morning, jump into the cold water at noon, and take the last cable car down while \
            the summits turn pink.",
    },
    Destination {
        id: "coastal-cliffs",
        name: "Coastal Cliffs",
        region: "Algarve",
        country: "Portugal",
        latitude: 37.09,
        longitude: -8.67,
        days: 5,
        season: "April to October",
        category: Category::Coast,
        tags: &["Beaches", "Sailing", "Seafood"],
        description: "Golden limestone cliffs drop into the Atlantic. Follow the coastal trail to \
            the old lighthouse, explore hidden coves by boat, and end the day with grilled fish \
            and a sunset over the ocean.",
    },
    Destination {
        id: "desert-dunes",
        name: "Desert Dunes",
        region: "Merzouga",
        country: "Morocco",
        latitude: 31.10,
        longitude: -4.01,
        days: 4,
        season: "October to April",
        category: Category::Desert,
        tags: &["Camel trek", "Stargazing", "Camping"],
        description: "Ride with a camel caravan into the dunes of Erg Chebbi, sleep in a desert \
            camp, and wake up early to watch the sand glow red in the first light.",
    },
    Destination {
        id: "northern-lights",
        name: "Northern Lights",
        region: "Troms",
        country: "Norway",
        latitude: 69.65,
        longitude: 18.96,
        days: 5,
        season: "September to March",
        category: Category::Nature,
        tags: &["Aurora", "Snowshoeing", "Huskies"],
        description: "Far above the Arctic Circle, green lights dance over snowy mountains. Spend \
            the long nights in a cozy cabin and the short days on snowshoes or behind a husky \
            sled.",
    },
    Destination {
        id: "island-lagoon",
        name: "Island Lagoon",
        region: "North Male Atoll",
        country: "Maldives",
        latitude: 4.30,
        longitude: 73.42,
        days: 7,
        season: "November to April",
        category: Category::Coast,
        tags: &["Snorkeling", "Diving", "Relaxing"],
        description: "A tiny island with a handful of palm trees in a warm lagoon. Snorkel with \
            turtles and reef sharks right off the beach, or just watch the color of the water \
            change through the day.",
    },
    Destination {
        id: "old-town",
        name: "Old Town",
        region: "Prague",
        country: "Czechia",
        latitude: 50.09,
        longitude: 14.42,
        days: 3,
        season: "All year",
        category: Category::City,
        tags: &["Architecture", "Museums", "Cafes"],
        description: "Colorful houses, a clock tower, and a stone bridge across the river. Get \
            lost in the narrow lanes, climb the tower for the view over the red roofs, and stay \
            for the lights at night.",
    },
    Destination {
        id: "forest-cabin",
        name: "Forest Cabin",
        region: "Black Forest",
        country: "Germany",
        latitude: 48.00,
        longitude: 8.20,
        days: 4,
        season: "May to October",
        category: Category::Nature,
        tags: &["Hiking", "Cycling", "Wellness"],
        description: "A log cabin deep in the dark fir forest. Morning mist in the valleys, long \
            walks to waterfalls, and evenings by the fireplace with a slice of cherry cake.",
    },
    Destination {
        id: "rice-terraces",
        name: "Rice Terraces",
        region: "Bali",
        country: "Indonesia",
        latitude: -8.43,
        longitude: 115.28,
        days: 6,
        season: "April to October",
        category: Category::Nature,
        tags: &["Culture", "Yoga", "Food"],
        description: "Green terraces wind down the hills like stairs for giants. Walk the paths \
            between the paddies at sunrise, visit the temples in the valley, and learn to cook \
            the local dishes.",
    },
];

pub fn find_destination(id: &str) -> Option<&'static Destination> {
    DESTINATIONS.iter().find(|d| d.id == id)
}

pub fn random_destination() -> &'static Destination {
    let index = (js_sys::Math::random() * DESTINATIONS.len() as f64) as usize;
    &DESTINATIONS[index.min(DESTINATIONS.len() - 1)]
}
