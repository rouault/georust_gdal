mod create_and_reproject;
mod resample;
mod warp_options;

pub use create_and_reproject::*;
pub use resample::*;
use std::path::Path;
pub use warp_options::*;

use crate::dataset::Dataset;
use crate::utils::_last_cpl_err;
use gdal_sys::{self, CPLErr, GDALResampleAlg};
use std::ptr::{null, null_mut};

use crate::errors::*;
use crate::spatial_ref::SpatialRef;

/// Reproject raster dataset into the given [`SpatialRef`] and save result to `dst_file`.
pub fn create_and_reproject<P: AsRef<Path>>(
    ds: &Dataset,
    dst_file: P,
    dst_srs: &SpatialRef,
    options: &CreateReprojectOptions,
) -> Result<()> {
    let dest_file = dst_file.as_ref();
    create_and_reproject::create_and_reproject_image(ds, dest_file, dst_srs, options)
}

/// Reproject one dataset into another dataset.
///
/// Assumes destination dataset is properly sized and setup with a [`SpatialRef`][crate::SpatialRef],
/// [`GeoTransform`][crate::GeoTransform], [`Rasterband`][crate::Rasterband], etc.
///
/// See [`create_and_reproject`] for a more flexible alternative.
pub fn reproject_image(src: &Dataset, dst: &Dataset) -> Result<()> {
    let rv = unsafe {
        gdal_sys::GDALReprojectImage(
            src.c_dataset(),
            null(),
            dst.c_dataset(),
            null(),
            GDALResampleAlg::GRA_Bilinear,
            0.0,
            0.0,
            None,
            null_mut(),
            null_mut(),
        )
    };
    if rv != CPLErr::CE_None {
        return Err(_last_cpl_err(rv));
    }
    Ok(())
}
