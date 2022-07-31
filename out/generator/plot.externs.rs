extern "C" {
	// setMinX(double) /usr/include/opencv2/plot.hpp:69
	pub fn cv_plot_Plot2d_setMinX_double(instance: *mut c_void, _plot_min_x: f64, ocvrs_return: *mut Result_void);
	// setMinY(double) /usr/include/opencv2/plot.hpp:70
	pub fn cv_plot_Plot2d_setMinY_double(instance: *mut c_void, _plot_min_y: f64, ocvrs_return: *mut Result_void);
	// setMaxX(double) /usr/include/opencv2/plot.hpp:71
	pub fn cv_plot_Plot2d_setMaxX_double(instance: *mut c_void, _plot_max_x: f64, ocvrs_return: *mut Result_void);
	// setMaxY(double) /usr/include/opencv2/plot.hpp:72
	pub fn cv_plot_Plot2d_setMaxY_double(instance: *mut c_void, _plot_max_y: f64, ocvrs_return: *mut Result_void);
	// setPlotLineWidth(int) /usr/include/opencv2/plot.hpp:73
	pub fn cv_plot_Plot2d_setPlotLineWidth_int(instance: *mut c_void, _plot_line_width: i32, ocvrs_return: *mut Result_void);
	// setNeedPlotLine(bool) /usr/include/opencv2/plot.hpp:80
	pub fn cv_plot_Plot2d_setNeedPlotLine_bool(instance: *mut c_void, _need_plot_line: bool, ocvrs_return: *mut Result_void);
	// setPlotLineColor(cv::Scalar) /usr/include/opencv2/plot.hpp:81
	pub fn cv_plot_Plot2d_setPlotLineColor_Scalar(instance: *mut c_void, _plot_line_color: *const core::Scalar, ocvrs_return: *mut Result_void);
	// setPlotBackgroundColor(cv::Scalar) /usr/include/opencv2/plot.hpp:82
	pub fn cv_plot_Plot2d_setPlotBackgroundColor_Scalar(instance: *mut c_void, _plot_background_color: *const core::Scalar, ocvrs_return: *mut Result_void);
	// setPlotAxisColor(cv::Scalar) /usr/include/opencv2/plot.hpp:83
	pub fn cv_plot_Plot2d_setPlotAxisColor_Scalar(instance: *mut c_void, _plot_axis_color: *const core::Scalar, ocvrs_return: *mut Result_void);
	// setPlotGridColor(cv::Scalar) /usr/include/opencv2/plot.hpp:84
	pub fn cv_plot_Plot2d_setPlotGridColor_Scalar(instance: *mut c_void, _plot_grid_color: *const core::Scalar, ocvrs_return: *mut Result_void);
	// setPlotTextColor(cv::Scalar) /usr/include/opencv2/plot.hpp:85
	pub fn cv_plot_Plot2d_setPlotTextColor_Scalar(instance: *mut c_void, _plot_text_color: *const core::Scalar, ocvrs_return: *mut Result_void);
	// setPlotSize(int, int) /usr/include/opencv2/plot.hpp:86
	pub fn cv_plot_Plot2d_setPlotSize_int_int(instance: *mut c_void, _plot_size_width: i32, _plot_size_height: i32, ocvrs_return: *mut Result_void);
	// setShowGrid(bool) /usr/include/opencv2/plot.hpp:87
	pub fn cv_plot_Plot2d_setShowGrid_bool(instance: *mut c_void, need_show_grid: bool, ocvrs_return: *mut Result_void);
	// setShowText(bool) /usr/include/opencv2/plot.hpp:88
	pub fn cv_plot_Plot2d_setShowText_bool(instance: *mut c_void, need_show_text: bool, ocvrs_return: *mut Result_void);
	// setGridLinesNumber(int) /usr/include/opencv2/plot.hpp:89
	pub fn cv_plot_Plot2d_setGridLinesNumber_int(instance: *mut c_void, grid_lines_number: i32, ocvrs_return: *mut Result_void);
	// setInvertOrientation(bool) /usr/include/opencv2/plot.hpp:90
	pub fn cv_plot_Plot2d_setInvertOrientation_bool(instance: *mut c_void, _invert_orientation: bool, ocvrs_return: *mut Result_void);
	// setPointIdxToPrint(int) /usr/include/opencv2/plot.hpp:96
	pub fn cv_plot_Plot2d_setPointIdxToPrint_int(instance: *mut c_void, point_idx: i32, ocvrs_return: *mut Result_void);
	// render(cv::OutputArray) /usr/include/opencv2/plot.hpp:97
	pub fn cv_plot_Plot2d_render_const__OutputArrayR(instance: *mut c_void, _plot_result: *const c_void, ocvrs_return: *mut Result_void);
	// create(cv::InputArray) /usr/include/opencv2/plot.hpp:105
	pub fn cv_plot_Plot2d_create_const__InputArrayR(data: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(cv::InputArray, cv::InputArray) /usr/include/opencv2/plot.hpp:113
	pub fn cv_plot_Plot2d_create_const__InputArrayR_const__InputArrayR(data_x: *const c_void, data_y: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
}
