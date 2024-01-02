pub const POSSIBLE_SIZES: [&str; 5] = ["micro", "small", "med", "large", "xl"];
pub const POSSIBLE_REGIONS: [&str; 17] = [
    "ams", "bom", "cdg", "dfw", "fra", "hkg", "iad", "lax", "lhr", "nrt", "ord", "scl", "sea",
    "sin", "sjc", "syd", "yyz",
];

pub const PATH: &str = "instances.csv";
pub const HEADERS: [&str; 4] = ["machine_id", "name", "size", "region"];
