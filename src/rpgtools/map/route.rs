/// How paths are placed
#[derive(Debug)]
pub enum RouteMethod {
    /// Manhattan routing; horizontal direction first.
    HorizontalFirst,
    /// Manhattan routing; vertical direction first.
    VerticalFirst,
    // Straight-line/diagonal routing
    //Direct,
    /// Split route into horizontal and vertical components and do them one at a time.
    Manhattan,
    // Subway-map style: allows routes at 0 degrees, 90 degrees and 45 degrees.
    //Subway,
}
