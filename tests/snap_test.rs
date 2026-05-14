use driftwm::layout::snap::*;

fn rect_h(x_low: f64, x_high: f64) -> SnapRect {
    SnapRect { x_low, x_high, y_low: -10000.0, y_high: 10000.0 }
}

fn params_h<'a>(extent: f64, others: &'a [SnapRect], gap: f64, threshold: f64) -> SnapParams<'a> {
    SnapParams {
        extent, perp_low: -10000.0, perp_high: 10000.0, horizontal: true,
        others, gap, threshold, break_force: 32.0, same_edge: false,
    }
}

#[test]
fn snap_right_edge_to_left_edge() {
    let others = vec![rect_h(310.0, 510.0)];
    let p = params_h(200.0, &others, 8.0, 16.0);
    let result = find_snap_candidate(100.0, &p);
    assert!(result.is_some());
    let (origin, _dist) = result.unwrap();
    assert!((origin - 102.0).abs() < 0.001);
}

#[test]
fn snap_left_edge_to_right_edge() {
    let others = vec![rect_h(200.0, 492.0)];
    let p = params_h(200.0, &others, 8.0, 16.0);
    let result = find_snap_candidate(500.0, &p);
    assert!(result.is_some());
    let (origin, _dist) = result.unwrap();
    assert!((origin - 500.0).abs() < 0.001);
}

#[test]
fn no_snap_when_too_far() {
    let others = vec![rect_h(500.0, 700.0)];
    let p = params_h(200.0, &others, 8.0, 16.0);
    let result = find_snap_candidate(100.0, &p);
    assert!(result.is_none());
}

#[test]
fn picks_closest_candidate() {
    let others = vec![
        rect_h(310.0, 510.0),
        rect_h(305.0, 505.0),
    ];
    let p = params_h(200.0, &others, 8.0, 16.0);
    let result = find_snap_candidate(100.0, &p);
    assert!(result.is_some());
    let (origin, _) = result.unwrap();
    assert!((origin - 97.0).abs() < 0.001);
}

#[test]
fn snap_break_and_cooldown() {
    let mut snap: Option<AxisSnap> = None;
    let mut cooldown: Option<f64> = None;
    let others = vec![rect_h(308.0, 508.0)];
    let p = SnapParams {
        extent: 200.0,
        perp_low: 0.0,
        perp_high: 100.0,
        horizontal: true,
        others: &others,
        gap: 8.0,
        threshold: 16.0,
        break_force: 32.0,
        same_edge: false,
    };

    let pos = update_axis(&mut snap, &mut cooldown, 100.0, &p);
    assert!(snap.is_some());
    assert!((pos - 100.0).abs() < 0.001);

    let pos = update_axis(&mut snap, &mut cooldown, 110.0, &p);
    assert!(snap.is_some());
    assert!((pos - 100.0).abs() < 0.001);

    let pos = update_axis(&mut snap, &mut cooldown, 140.0, &p);
    assert!(snap.is_none());
    assert!(cooldown.is_some());
    assert!((pos - 140.0).abs() < 0.001);

    let pos = update_axis(&mut snap, &mut cooldown, 105.0, &p);
    assert!(snap.is_none());
    assert!(cooldown.is_some());
    assert!((pos - 105.0).abs() < 0.001);

    let _pos = update_axis(&mut snap, &mut cooldown, 200.0, &p);
    assert!(cooldown.is_none());

    let pos = update_axis(&mut snap, &mut cooldown, 100.0, &p);
    assert!(snap.is_some());
    assert!((pos - 100.0).abs() < 0.001);
}

#[test]
fn snap_from_inside_does_not_immediately_break() {
    let mut snap: Option<AxisSnap> = None;
    let mut cooldown: Option<f64> = None;
    let others = vec![rect_h(0.0, 500.0)];
    let p = SnapParams {
        extent: 200.0,
        perp_low: 0.0,
        perp_high: 100.0,
        horizontal: true,
        others: &others,
        gap: 12.0,
        threshold: 24.0,
        break_force: 32.0,
        same_edge: false,
    };

    let pos = update_axis(&mut snap, &mut cooldown, 480.0, &p);
    assert!(snap.is_some(), "should engage");
    assert!((pos - 512.0).abs() < 0.001);

    let pos = update_axis(&mut snap, &mut cooldown, 500.0, &p);
    assert!(snap.is_some(), "should stay snapped moving toward snap");
    assert!((pos - 512.0).abs() < 0.001);

    let pos = update_axis(&mut snap, &mut cooldown, 440.0, &p);
    assert!(snap.is_none(), "should break on retreat past engage point");
    assert!((pos - 440.0).abs() < 0.001);
}

#[test]
fn no_snap_without_perpendicular_overlap() {
    let others = vec![SnapRect { x_low: 310.0, x_high: 510.0, y_low: 1000.0, y_high: 1200.0 }];
    let p = SnapParams {
        extent: 200.0, perp_low: 0.0, perp_high: 100.0, horizontal: true,
        others: &others, gap: 8.0, threshold: 16.0, break_force: 32.0, same_edge: false,
    };
    let result = find_snap_candidate(100.0, &p);
    assert!(result.is_none(), "should not snap to window with no Y overlap");
}

#[test]
fn no_snap_when_perp_edges_only_touch() {
    // perp_high (100) exactly meets other.y_low (100) — zero shared length.
    // Strict overlap (post-tightening) rejects this: edges meeting at a
    // point is not overlap, so the corresponding axis won't snap.
    let others = vec![SnapRect { x_low: 310.0, x_high: 510.0, y_low: 100.0, y_high: 300.0 }];
    let p = SnapParams {
        extent: 200.0, perp_low: 0.0, perp_high: 100.0, horizontal: true,
        others: &others, gap: 8.0, threshold: 16.0, break_force: 32.0, same_edge: false,
    };
    let result = find_snap_candidate(100.0, &p);
    assert!(
        result.is_none(),
        "exact perpendicular edge-touch should not count as overlap",
    );
}

#[test]
fn no_snap_perpendicular_gap_exceeds_tolerance() {
    let others = vec![SnapRect { x_low: 310.0, x_high: 510.0, y_low: 200.0, y_high: 400.0 }];
    let p = SnapParams {
        extent: 200.0, perp_low: 0.0, perp_high: 100.0, horizontal: true,
        others: &others, gap: 8.0, threshold: 16.0, break_force: 32.0, same_edge: false,
    };
    let result = find_snap_candidate(100.0, &p);
    assert!(result.is_none(), "should not snap when perp gap exceeds threshold");
}

#[test]
fn same_edge_aligns_left_edges_when_perpendicular_adjacent() {
    // Vertical stack scenario: top window at y=[0,100], dragged bottom window
    // already snapped at y_low=112 (other.y_high + gap). User slides the bottom
    // window horizontally to align left edges. perp y has no overlap with other
    // (windows are stacked, not side-by-side) but they're within gap+threshold
    // of each other perpendicular — same-edge should engage.
    let others = vec![SnapRect { x_low: 100.0, x_high: 300.0, y_low: 0.0, y_high: 100.0 }];
    let p = SnapParams {
        extent: 200.0, perp_low: 112.0, perp_high: 192.0, horizontal: true,
        others: &others, gap: 12.0, threshold: 24.0, break_force: 32.0, same_edge: true,
    };
    // Natural left at 95 (5px from target.left of 100) — same-edge L→L → origin=100.
    let (origin, _) = find_snap_candidate(95.0, &p)
        .expect("same-edge L→L should engage for vertically stacked windows");
    assert!((origin - 100.0).abs() < 0.001, "expected origin=100, got {origin}");
}

#[test]
fn same_edge_aligns_right_edges_when_perpendicular_adjacent() {
    // Same vertical stack, but dragging so right edges align.
    let others = vec![SnapRect { x_low: 100.0, x_high: 300.0, y_low: 0.0, y_high: 100.0 }];
    let p = SnapParams {
        extent: 200.0, perp_low: 112.0, perp_high: 192.0, horizontal: true,
        others: &others, gap: 12.0, threshold: 24.0, break_force: 32.0, same_edge: true,
    };
    // Natural left at 105 → natural right = 305, 5px from target.right(300).
    // Same-edge R→R → origin = 300 - 200 = 100.
    let (origin, _) = find_snap_candidate(105.0, &p)
        .expect("same-edge R→R should engage for vertically stacked windows");
    assert!((origin - 100.0).abs() < 0.001, "expected origin=100, got {origin}");
}

#[test]
fn same_edge_does_not_engage_when_perpendicular_far() {
    // Same vertical stack but far apart perpendicular (200px below target's
    // bottom — well beyond gap+threshold = 36). Same-edge must NOT pull the
    // dragged window across a large vertical gap.
    let others = vec![SnapRect { x_low: 100.0, x_high: 300.0, y_low: 0.0, y_high: 100.0 }];
    let p = SnapParams {
        extent: 200.0, perp_low: 300.0, perp_high: 380.0, horizontal: true,
        others: &others, gap: 12.0, threshold: 24.0, break_force: 32.0, same_edge: true,
    };
    assert!(
        find_snap_candidate(95.0, &p).is_none(),
        "same-edge must require perpendicular proximity within gap+threshold",
    );
}

#[test]
fn opposite_edge_unaffected_by_perpendicular_adjacency() {
    // Regression: opposite-edge snap must NOT fire for perp-adjacent (non-
    // overlapping) windows. Otherwise dragging a window below another window
    // toward its right edge would magnetically dock it side-by-side even
    // though the windows aren't actually beside each other.
    let others = vec![SnapRect { x_low: 100.0, x_high: 300.0, y_low: 0.0, y_high: 100.0 }];
    let p = SnapParams {
        extent: 200.0, perp_low: 112.0, perp_high: 192.0, horizontal: true,
        others: &others, gap: 12.0, threshold: 24.0, break_force: 32.0, same_edge: false,
    };
    // Natural left at 310 — close to other.x_high(300)+gap(12)=312 (opposite L→R),
    // but the windows don't perp-overlap. With same_edge=false, no snap.
    assert!(
        find_snap_candidate(310.0, &p).is_none(),
        "opposite-edge must require perpendicular overlap (same_edge=false)",
    );
}

#[test]
fn y_axis_snap_filters_by_x_overlap() {
    let others = vec![
        SnapRect { x_low: 0.0, x_high: 300.0, y_low: 310.0, y_high: 510.0 },
        SnapRect { x_low: 5000.0, x_high: 5300.0, y_low: 310.0, y_high: 510.0 },
    ];
    let p = SnapParams {
        extent: 200.0, perp_low: 0.0, perp_high: 300.0, horizontal: false,
        others: &others, gap: 8.0, threshold: 16.0, break_force: 32.0, same_edge: false,
    };
    let result = find_snap_candidate(100.0, &p);
    assert!(result.is_some(), "should snap to Y-nearby window with X overlap");
    let (origin, _) = result.unwrap();
    assert!((origin - 102.0).abs() < 0.001);

    let far_only = vec![
        SnapRect { x_low: 5000.0, x_high: 5300.0, y_low: 310.0, y_high: 510.0 },
    ];
    let p2 = SnapParams {
        extent: 200.0, perp_low: 0.0, perp_high: 300.0, horizontal: false,
        others: &far_only, gap: 8.0, threshold: 16.0, break_force: 32.0, same_edge: false,
    };
    let result = find_snap_candidate(100.0, &p2);
    assert!(result.is_none(), "should not snap when only far window exists");
}
