use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::{PointMarker, PointStyle, LineStyle};

fn main() {
    // Scatter plots expect a list of pairs

    let inc = vec![
        (2.0, 0.0466),
        (4.0, 1.0172),
        (6.0, 67.5087),
        (8.0, 723.5087)
        ];

    // let g1 = vec![
    //     (2.0, 0.0926),
    //     (4.0, 0.2387),
    //     (6.0, 11.616)
    //     ];

    let g2 = vec![
        (2.0, 0.1089),
        (4.0, 0.2392),
        (6.0, 0.5003),
        (8.0, 0.9351),
        (10.0, 1.6503),
        (12.0, 2.7410),
        (14.0, 4.1613),
        (16.0, 6.3566),
        (18.0, 9.4947),
        (20.0, 13.4284),
        (22.0, 18.2384),
        (24.0, 24.6879),
        (26.0, 32.0478),
        (28.0, 42.5238),
        (30.0, 54.5594),
        (32.0, 68.1481),
        (34.0, 84.8150),
        (36.0, 105.3843),
        (38.0, 128.2080),
        (40.0, 152.8153),
    ];

    let g3 = vec![
        (2.0, 0.0887),
        (4.0, 0.1727),
        (6.0, 0.3413),
        (8.0, 0.5974),
        (10.0, 0.9998),
        (12.0, 1.7164),
        (14.0, 2.4527),
        (16.0, 3.8189),
        (18.0, 5.4847),
        (20.0, 7.8671),
        (22.0, 10.6586),
        (24.0, 14.0648),
        (26.0, 18.4096),
        (28.0, 23.8836),
        (30.0, 30.2119),
        (32.0, 38.2234),
        (34.0, 47.3344),
        (36.0, 59.0869),
        (38.0, 72.1037),
        (40.0, 86.5870),
    ];

    let s1: Plot = Plot::new(inc).point_style(
        PointStyle::new()
        .size(3.0)
            .marker(PointMarker::Square) // setting the marker to be a square
            .colour("#404040"),
    );

    // let s2: Plot = Plot::new(g1).point_style(
    //     PointStyle::new()
    //     .size(3.0)
    //         .marker(PointMarker::Cross) // setting the marker to be a square
    //         .colour("#D1B623"),
    // );

    let s3: Plot = Plot::new(g2).point_style(
        PointStyle::new()
        .size(3.0)
            .marker(PointMarker::Circle) // setting the marker to be a square
            .colour("#404040"),
    );

    let s4: Plot = Plot::new(g3).point_style(
        PointStyle::new()
        .size(3.0)
            .marker(PointMarker::Cross) // setting the marker to be a square
            .colour("#404040"),
    );


    let v = ContinuousView::new()
        // .add(s1)
        // .add(s2)
        .add(s3)
        .add(s4)
        .add(s1)
        // .add(s2)
        .x_range(0.0, 40.0)
        .y_range(-2.0, 180.0)
        .x_label("Instance (balls)")
        .y_label("Seconds");

    // A page with a single view is then saved to an SVG file
    Page::single(&v).save("gripper.jpg").unwrap();
}