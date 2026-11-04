//! Commands SoLeadWorks does not offer. SoLeadWorks is SolidCraft cut down to what building scout
//! camp furniture (froissartage: poles, mortise and tenon, half-lap joints, lashings) needs, so
//! everything about sheet metal, plastic, surfaces, meshes, point clouds, motion studies,
//! rendering, CAM, configurations and the like is taken out of the registry. The code stays in
//! the tree for now (it is what the next clean-up deletes, module by module); this list is the
//! single switch: a command whose id matches is neither listed, found, run nor placed on the
//! ribbon.

/// Whole families, by id prefix (the dot included).
const FAMILIES: &[&str] = &[
    "render.", "surface.", "sheet.", "plastic.", "motion.", "pointcloud.", "mesh.", "canvas.", "config.", "contact.", "library.",
    "settings.profile.", "diagnostics.crash_test",
];

/// Single commands.
const COMMANDS: &[&str] = &[
    // solids that need no wood: threads, coils, pipes, ribs, webs, embossing, sweeps and lofts
    "solid.sweep", "solid.loft", "solid.torus", "solid.pipe", "solid.coil", "solid.thread", "solid.rib", "solid.web", "solid.emboss",
    "solid.boundary_fill", "solid.bounding_solid", "solid.draft", "solid.replace_face",
    // 3D sketches, surface projections, curve fitting
    "sketch.line3d", "sketch.spline3d", "sketch.arc3d", "sketch.point3d", "sketch.move3d", "sketch.include_3d", "sketch.project_to_surface",
    "sketch.intersection_curve", "sketch.spun_profile", "sketch.isoparametric_curve", "sketch.fit_curves_to_section", "sketch.insert_svg",
    "sketch.equation_curve", "sketch.equation_curve.edit", "sketch.conic", "sketch.blend_curve", "sketch.spline.fit_point",
    "sketch.spline.control_point", "sketch.spline.control_point_5", "sketch.spline.insert_point", "sketch.spline.reverse",
    // surface analysis and manufacturing checks
    "inspect.curvature_comb", "inspect.minimum_radius", "inspect.isocurve", "inspect.zebra", "inspect.environment_map",
    "inspect.accessibility", "inspect.curvature_map", "inspect.continuity", "inspect.draft", "inspect.parameter_sweep",
    // slicers and meshes
    "file.insert_mesh", "file.save_mesh", "file.send_to_slicer", "file.open_in_easyprint", "file.slicers", "body.print_face",
    // mechanisms
    "joint.gear", "joint.rack_pinion", "joint.screw", "joint.cam", "joint.reach", "joint.motion_link", "joint.drive", "joint.sweep",
    // render-only view helpers
    "view.render_focus", "view.render_frame", "view.render_browser", "view.render_inspector", "view.render_select", "view.render_stage",
    "view.shots_strip", "view.render_dock", "view.ambient_occlusion",
    "parameters.users", "material.properties.validate", "material.properties.directional",
];

/// Whether SoLeadWorks leaves this command out.
pub fn is_retired(id: &str) -> bool {
    COMMANDS.contains(&id) || FAMILIES.iter().any(|f| id.starts_with(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_woodworking_core_is_kept() {
        for id in [
            "solid.extrude", "solid.box", "solid.cylinder", "solid.hole", "solid.combine", "solid.move", "solid.mirror", "sketch.create",
            "sketch.rectangle.two_point", "sketch.dimension", "component.create", "joint.create", "occurrence.move", "explode.create",
            "assembly.bom", "frame.member", "timeline.rollback", "edit.checkpoint", "edit.restore_checkpoint", "inspect.interference",
            "inspect.measure", "parameters.change",
        ] {
            assert!(!is_retired(id), "{id} must stay");
        }
    }

    #[test]
    fn sheet_metal_and_rendering_are_gone() {
        for id in ["sheet.flange", "render.start", "surface.loft", "plastic.rib", "motion.study", "solid.sweep", "joint.gear"] {
            assert!(is_retired(id), "{id} must be retired");
        }
    }
}
