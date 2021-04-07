#[deny(
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    bad_style,
    const_err,
    dead_code,
    improper_ctypes,
    legacy_directory_ownership,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    plugin_as_library,
    private_in_public,
    safe_extern_statics,
    unconditional_recursion,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]

///////////////////////////////////////////////////////////////////////////////////////////////////
/// Wrappers for complicated OCV functions
pub mod cv_wrapper {
    use opencv::{
        prelude::*,
        videoio,
        highgui,
        imgproc,
        Result,
    };

    /////////////////////////////////////////////////////////////////////////
    // Processing////////////////////////////////////////////////////////
    pub fn hsv_process(
        frame: Mat,
        upper_color: f64,
        lower_color: f64,
    ) -> Result<Mat, opencv::Error> {

        //Captures (these bindings are weird)
        let mut hsv_mat = Mat::default(); 
        let mut thresh = Mat::default();
        let mut open = Mat::default();
        let mut close = Mat::default();

        // Converting to HSV
        imgproc::cvt_color(
            &frame,
            &mut hsv_mat,
            imgproc::COLOR_BGR2HSV,
            0
        )?; 

        // Color Bounds
        opencv::core::in_range(
            &hsv_mat,
            &upper_color,
            &lower_color,
            &mut thresh
        )?;

        imgproc::morphology_ex(
            &thresh,
            &mut close,
            imgproc::MORPH_CLOSE,
            &imgproc::get_structuring_element(
                imgproc::MORPH_ELLIPSE,
                ,
                (10, 10)
            ).unwrap(),
            core::Point(-1, -1),
            iterations: 1,
            border_type: 0,
            border_value: 0
        )?;
    }

    pub fn hsv_process_cuda(
        frame: Mat,
        upper_color: f64,
        lower_color: f64,
    ) -> Result<Mat, opencv::Error> {

    } 
}