use crate::errors::Result;
use crate::raster::warp::GdalWarpOptions;
use crate::spatial_ref::SpatialRef;

/// Settings for [`create_and_reproject`].
#[derive(Debug, Clone, Default)]
pub struct ReprojectOptions {
    warp_options: GdalWarpOptions,
    max_error: Option<f64>,
    src_srs: Option<SpatialRef>,
    src_nodata: Option<f64>,
    dst_nodata: Option<f64>,
    output_format: Option<String>,
}

impl ReprojectOptions {
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the maximum error.
    ///
    /// Measured in input pixels, it is the allowed in approximating
    /// transformations.
    ///
    /// `0.0` indicates exact calculations.
    pub fn with_max_error(&mut self, max_error: f64) -> &mut Self {
        self.max_error = Some(max_error);
        self
    }

    /// Fetch the specified maximum error.
    ///
    /// Returns `None` if unset.
    pub fn max_error(&self) -> Option<f64> {
        self.max_error
    }

    /// Specify the source projection.
    ///
    /// If unset, the source projection is read from the source dataset.
    /// If set, the source projection os overridden.
    pub fn with_src_projection(&mut self, srs: &SpatialRef) -> &mut Self {
        self.src_srs = Some(srs.clone());
        self
    }

    /// Fetch the specified source projection, if any.
    pub fn src_projection(&self) -> Option<&SpatialRef> {
        self.src_srs.as_ref()
    }

    /// Specify the source no-data value.
    ///
    /// Overrides any no-data value specified in the source dataset.
    pub fn with_src_nodata(&mut self, nodata_value: f64) -> &mut Self {
        self.src_nodata = Some(nodata_value);
        self
    }

    /// Get the specified source no-data value, if any.
    pub fn src_nodata(&self) -> Option<f64> {
        self.src_nodata
    }

    /// Specify the destination no-data value.
    ///
    /// # Notes
    ///
    /// While this setting this value ensures the given no-data value is used when no-data needs
    /// to be written to the output, due to [this issue](https://lists.osgeo.org/pipermail/gdal-dev/2023-November/057887.html)
    /// the resulting dataset will not have `NO_DATA` set in the metadata when used with `GDALCreateAndReprojectImage`.
    pub fn with_dst_nodata(&mut self, nodata_value: f64) -> &mut Self {
        self.dst_nodata = Some(nodata_value);
        self
    }

    /// Get the specified destination no-data value, if any.
    pub fn dst_nodata(&self) -> Option<f64> {
        self.dst_nodata
    }

    /// Explicitly specify output raster format.
    ///
    /// This is equivalent to the `-of <format>` CLI flag accepted by many GDAL tools.
    ///
    /// The value of `format` must be the identifier of a driver supported by the runtime
    /// environment's GDAL library (e.g. `COG`, `JPEG`, `VRT`, etc.). A list of these identifiers
    /// is available from `gdalinfo --formats`:
    ///
    /// ```text
    /// ❯ gdalinfo --formats
    /// Supported Formats:
    ///   VRT -raster,multidimensional raster- (rw+v): Virtual Raster
    ///   DERIVED -raster- (ro): Derived datasets using VRT pixel functions
    ///   GTiff -raster- (rw+vs): GeoTIFF
    ///   COG -raster- (wv): Cloud optimized GeoTIFF generator
    ///   NITF -raster- (rw+vs): National Imagery Transmission Format
    /// ...
    /// ```
    /// If no output format is specified, then the driver from the source dataset is used.
    ///
    pub fn with_output_format(&mut self, format: &str) -> &mut Self {
        self.output_format = Some(format.to_owned());
        self
    }

    /// Fetch the specified output format driver identifier, if any.
    pub fn output_format(&self) -> Option<String> {
        self.output_format.clone()
    }

    /// Fetch an immutable reference to the general Warp options.
    pub fn warp_options(&self) -> &GdalWarpOptions {
        &self.warp_options
    }

    /// Fetch a mutable reference to the general Warp options.
    pub fn warp_options_mut(&mut self) -> &mut GdalWarpOptions {
        &mut self.warp_options
    }

    /// Clone `warp_options` and initialize any required sub-structures.
    ///
    /// We clone because `GDALCreateAndReprojectImage` and siblings require a mutable pointer to
    /// an GDALWarpOptions instance. We could either propagate mutability up the call chain
    /// or clone the given options. Given the user may want to reuse settings for consistent
    /// application across multiple files and may find mutation unexpected, we clone make a clone.
    pub(crate) fn clone_and_init_warp_options(&self, band_count: usize) -> Result<GdalWarpOptions> {
        let mut warp_options = self.warp_options().clone();

        // If nodata values are specified, we need to initialize some state in
        // `GdalWarpOptions` first.
        if self.src_nodata().is_some() || self.dst_nodata().is_some() {
            warp_options.with_band_count(band_count);
        }

        if let Some(src_nodata) = self.src_nodata() {
            warp_options.apply_src_nodata(src_nodata)?;
        }

        if let Some(dst_nodata) = self.dst_nodata() {
            warp_options.apply_dst_nodata(dst_nodata)?;
        }

        if warp_options.working_datatype().is_none() {
            warp_options.with_auto_working_datatype();
        }

        Ok(warp_options)
    }
}

/// Settings for [`reproject_intop`].
#[derive(Debug, Clone, Default)]
pub struct ReprojectIntoOptions {
    warp_options: GdalWarpOptions,
    max_error: Option<f64>,
    src_srs: Option<SpatialRef>,
    dst_srs: Option<SpatialRef>,
    src_nodata: Option<f64>,
    dst_nodata: Option<f64>,
}

impl ReprojectIntoOptions {
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the maximum error.
    ///
    /// Measured in input pixels, it is the allowed in approximating
    /// transformations.
    ///
    /// `0.0` indicates exact calculations.
    pub fn with_max_error(&mut self, max_error: f64) -> &mut Self {
        self.max_error = Some(max_error);
        self
    }

    /// Fetch the specified maximum error.
    ///
    /// Returns `None` if unset.
    pub fn max_error(&self) -> Option<f64> {
        self.max_error
    }

    /// Specify the source projection.
    ///
    /// If unset, the source projection is read from the source dataset.
    /// If set, the source projection os overridden.
    pub fn with_src_projection(&mut self, srs: &SpatialRef) -> &mut Self {
        self.src_srs = Some(srs.clone());
        self
    }

    /// Fetch the specified source projection, if any.
    pub fn src_projection(&self) -> Option<&SpatialRef> {
        self.src_srs.as_ref()
    }

    /// Specify the destination projection.
    ///
    /// If unset, the destination projection is read from the destination dataset.
    /// If set, the destination projection os overridden.
    pub fn with_dst_projection(&mut self, srs: &SpatialRef) -> &mut Self {
        self.dst_srs = Some(srs.clone());
        self
    }

    /// Fetch the specified destination projection, if any.
    pub fn dst_projection(&self) -> Option<&SpatialRef> {
        self.dst_srs.as_ref()
    }

    /// Specify the source no-data value.
    ///
    /// Overrides any no-data value specified in the source dataset.
    pub fn with_src_nodata(&mut self, nodata_value: f64) -> &mut Self {
        self.src_nodata = Some(nodata_value);
        self
    }

    /// Get the specified source no-data value, if any.
    pub fn src_nodata(&self) -> Option<f64> {
        self.src_nodata
    }

    /// Specify the destination no-data value.
    ///
    /// # Notes
    ///
    /// While this setting this value ensures the given no-data value is used when no-data needs
    /// to be written to the output, due to [this issue](https://lists.osgeo.org/pipermail/gdal-dev/2023-November/057887.html)
    /// the resulting dataset will not have `NO_DATA` set in the metadata when used with `GDALCreateAndReprojectImage`.
    pub fn with_dst_nodata(&mut self, nodata_value: f64) -> &mut Self {
        self.dst_nodata = Some(nodata_value);
        self
    }

    /// Get the specified destination no-data value, if any.
    pub fn dst_nodata(&self) -> Option<f64> {
        self.dst_nodata
    }

    /// Fetch an immutable reference to the general Warp options.
    pub fn warp_options(&self) -> &GdalWarpOptions {
        &self.warp_options
    }

    /// Fetch a mutable reference to the general Warp options.
    pub fn warp_options_mut(&mut self) -> &mut GdalWarpOptions {
        &mut self.warp_options
    }

    /// Clone `warp_options` and initialize any required sub-structures.
    ///
    /// We clone because `GDALCreateAndReprojectImage` and siblings require a mutable pointer to
    /// an GDALWarpOptions instance. We could either propagate mutability up the call chain
    /// or clone the given options. Given the user may want to reuse settings for consistent
    /// application across multiple files and may find mutation unexpected, we clone make a clone.
    pub(crate) fn clone_and_init_warp_options(&self, band_count: usize) -> Result<GdalWarpOptions> {
        let mut warp_options = self.warp_options().clone();

        // If nodata values are specified, we need to initialize some state in
        // `GdalWarpOptions` first.
        if self.src_nodata().is_some() || self.dst_nodata().is_some() {
            warp_options.with_band_count(band_count);
        }

        if let Some(src_nodata) = self.src_nodata() {
            warp_options.apply_src_nodata(src_nodata)?;
        }

        if let Some(dst_nodata) = self.dst_nodata() {
            warp_options.apply_dst_nodata(dst_nodata)?;
        }

        if warp_options.working_datatype().is_none() {
            warp_options.with_auto_working_datatype();
        }

        Ok(warp_options)
    }
}
