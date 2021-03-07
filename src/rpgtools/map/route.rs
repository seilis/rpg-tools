// Routing
#[derive(Debug)]
pub enum RouteMethod {
    HorizontalFirst, // Manhattan routing; Horizontal direction first
    VerticalFirst,   // Manhattan routing; Vertical direction first
    //    Direct,          // Straight line/diagonal routing
    Manhattan, // Split route into horizontal and vertical components
               // do them one at a time.
               //    Subway,          // Subway-map style (or circuit-board style, if you prefer).
               // Allows routes at 0 degrees, 90 degrees and 45 degrees.
}
