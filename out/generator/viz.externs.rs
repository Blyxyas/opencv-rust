extern "C" {
	// computeNormals(const cv::viz::Mesh &, cv::OutputArray) /usr/include/opencv2/viz/vizcore.hpp:214
	pub fn cv_viz_computeNormals_const_MeshR_const__OutputArrayR(mesh: *const c_void, normals: *const c_void, ocvrs_return: *mut Result_void);
	// getWindowByName(const cv::String &) /usr/include/opencv2/viz/vizcore.hpp:97
	pub fn cv_viz_getWindowByName_const_StringR(window_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// imshow(const cv::String &, cv::InputArray, const cv::Size &) /usr/include/opencv2/viz/vizcore.hpp:103
	pub fn cv_viz_imshow_const_StringR_const__InputArrayR_const_SizeR(window_name: *const c_char, image: *const c_void, window_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
	// makeCameraPose(const cv::Vec3d &, const cv::Vec3d &, const cv::Vec3d &) /usr/include/opencv2/viz/vizcore.hpp:84
	pub fn cv_viz_makeCameraPose_const_Vec3dR_const_Vec3dR_const_Vec3dR(position: *const core::Vec3d, focal_point: *const core::Vec3d, y_dir: *const core::Vec3d, ocvrs_return: *mut Result<core::Affine3d>);
	// makeTransformToGlobal(const cv::Vec3d &, const cv::Vec3d &, const cv::Vec3d &, const cv::Vec3d &) /usr/include/opencv2/viz/vizcore.hpp:73
	pub fn cv_viz_makeTransformToGlobal_const_Vec3dR_const_Vec3dR_const_Vec3dR_const_Vec3dR(axis_x: *const core::Vec3d, axis_y: *const core::Vec3d, axis_z: *const core::Vec3d, origin: *const core::Vec3d, ocvrs_return: *mut Result<core::Affine3d>);
	// readCloud(const cv::String &, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/viz/vizcore.hpp:160
	pub fn cv_viz_readCloud_const_StringR_const__OutputArrayR_const__OutputArrayR(file: *const c_char, colors: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// readMesh(const cv::String &) /usr/include/opencv2/viz/vizcore.hpp:165
	pub fn cv_viz_readMesh_const_StringR(file: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// readPose(const cv::String &, cv::Affine3d &, const cv::String &) /usr/include/opencv2/viz/vizcore.hpp:175
	pub fn cv_viz_readPose_const_StringR_Affine3dR_const_StringR(file: *const c_char, pose: *mut core::Affine3d, tag: *const c_char, ocvrs_return: *mut Result<bool>);
	// readTrajectory(cv::OutputArray, const cv::String &, int, int, const cv::String &) /usr/include/opencv2/viz/vizcore.hpp:206
	pub fn cv_viz_readTrajectory_const__OutputArrayR_const_StringR_int_int_const_StringR(traj: *const c_void, files_format: *const c_char, start: i32, end: i32, tag: *const c_char, ocvrs_return: *mut Result_void);
	// unregisterAllWindows() /usr/include/opencv2/viz/vizcore.hpp:100
	pub fn cv_viz_unregisterAllWindows(ocvrs_return: *mut Result_void);
	// writeCloud(const cv::String &, cv::InputArray, cv::InputArray, cv::InputArray, bool) /usr/include/opencv2/viz/vizcore.hpp:151
	pub fn cv_viz_writeCloud_const_StringR_const__InputArrayR_const__InputArrayR_const__InputArrayR_bool(file: *const c_char, cloud: *const c_void, colors: *const c_void, normals: *const c_void, binary: bool, ocvrs_return: *mut Result_void);
	// writePose(const cv::String &, const cv::Affine3d &, const cv::String &) /usr/include/opencv2/viz/vizcore.hpp:181
	pub fn cv_viz_writePose_const_StringR_const_Affine3dR_const_StringR(file: *const c_char, pose: *const core::Affine3d, tag: *const c_char, ocvrs_return: *mut Result_void);
	// writeTrajectory(cv::InputArray, const cv::String &, int, const cv::String &) /usr/include/opencv2/viz/vizcore.hpp:193
	pub fn cv_viz_writeTrajectory_const__InputArrayR_const_StringR_int_const_StringR(traj: *const c_void, files_format: *const c_char, start: i32, tag: *const c_char, ocvrs_return: *mut Result_void);
	// Camera(double, double, double, double, const cv::Size &) /usr/include/opencv2/viz/types.hpp:176
	pub fn cv_viz_Camera_Camera_double_double_double_double_const_SizeR(fx: f64, fy: f64, cx: f64, cy: f64, window_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
	// Camera(const cv::Vec2d &, const cv::Size &) /usr/include/opencv2/viz/types.hpp:183
	pub fn cv_viz_Camera_Camera_const_Vec2dR_const_SizeR(fov: *const core::Vec2d, window_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
	// Camera(const cv::Matx33d &, const cv::Size &) /usr/include/opencv2/viz/types.hpp:197
	pub fn cv_viz_Camera_Camera_const_Matx33dR_const_SizeR(k: *const core::Matx33d, window_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
	// Camera(const cv::Matx44d &, const cv::Size &) /usr/include/opencv2/viz/types.hpp:213
	pub fn cv_viz_Camera_Camera_const_Matx44dR_const_SizeR(proj: *const core::Matx44d, window_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
	// getClip() /usr/include/opencv2/viz/types.hpp:215
	pub fn cv_viz_Camera_getClip_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec2d>);
	// setClip(const cv::Vec2d &) /usr/include/opencv2/viz/types.hpp:216
	pub fn cv_viz_Camera_setClip_const_Vec2dR(instance: *mut c_void, clip: *const core::Vec2d, ocvrs_return: *mut Result_void);
	// getWindowSize() /usr/include/opencv2/viz/types.hpp:218
	pub fn cv_viz_Camera_getWindowSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
	// setWindowSize(const cv::Size &) /usr/include/opencv2/viz/types.hpp:219
	pub fn cv_viz_Camera_setWindowSize_const_SizeR(instance: *mut c_void, window_size: *const core::Size, ocvrs_return: *mut Result_void);
	// getFov() /usr/include/opencv2/viz/types.hpp:221
	pub fn cv_viz_Camera_getFov_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec2d>);
	// setFov(const cv::Vec2d &) /usr/include/opencv2/viz/types.hpp:222
	pub fn cv_viz_Camera_setFov_const_Vec2dR(instance: *mut c_void, fov: *const core::Vec2d, ocvrs_return: *mut Result_void);
	// getPrincipalPoint() /usr/include/opencv2/viz/types.hpp:224
	pub fn cv_viz_Camera_getPrincipalPoint_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec2d>);
	// getFocalLength() /usr/include/opencv2/viz/types.hpp:225
	pub fn cv_viz_Camera_getFocalLength_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec2d>);
	// computeProjectionMatrix(cv::Matx44d &) /usr/include/opencv2/viz/types.hpp:240
	pub fn cv_viz_Camera_computeProjectionMatrix_const_Matx44dR(instance: *const c_void, proj: *mut core::Matx44d, ocvrs_return: *mut Result_void);
	// KinectCamera(const cv::Size &) /usr/include/opencv2/viz/types.hpp:250
	pub fn cv_viz_Camera_KinectCamera_const_SizeR(window_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
	// Color() /usr/include/opencv2/viz/types.hpp:66
	pub fn cv_viz_Color_Color(ocvrs_return: *mut Result<*mut c_void>);
	// Color(double) /usr/include/opencv2/viz/types.hpp:68
	pub fn cv_viz_Color_Color_double(gray: f64, ocvrs_return: *mut Result<*mut c_void>);
	// Color(double, double, double) /usr/include/opencv2/viz/types.hpp:69
	pub fn cv_viz_Color_Color_double_double_double(blue: f64, green: f64, red: f64, ocvrs_return: *mut Result<*mut c_void>);
	// Color(const cv::Scalar &) /usr/include/opencv2/viz/types.hpp:71
	pub fn cv_viz_Color_Color_const_ScalarR(color: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
	// operator Vec() /usr/include/opencv2/viz/types.hpp:73
	pub fn cv_viz_Color_operator_cv_Vec3b_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec3b>);
	// black() /usr/include/opencv2/viz/types.hpp:75
	pub fn cv_viz_Color_black(ocvrs_return: *mut Result<*mut c_void>);
	// blue() /usr/include/opencv2/viz/types.hpp:76
	pub fn cv_viz_Color_blue(ocvrs_return: *mut Result<*mut c_void>);
	// green() /usr/include/opencv2/viz/types.hpp:77
	pub fn cv_viz_Color_green(ocvrs_return: *mut Result<*mut c_void>);
	// red() /usr/include/opencv2/viz/types.hpp:78
	pub fn cv_viz_Color_red(ocvrs_return: *mut Result<*mut c_void>);
	// cyan() /usr/include/opencv2/viz/types.hpp:79
	pub fn cv_viz_Color_cyan(ocvrs_return: *mut Result<*mut c_void>);
	// yellow() /usr/include/opencv2/viz/types.hpp:80
	pub fn cv_viz_Color_yellow(ocvrs_return: *mut Result<*mut c_void>);
	// magenta() /usr/include/opencv2/viz/types.hpp:81
	pub fn cv_viz_Color_magenta(ocvrs_return: *mut Result<*mut c_void>);
	// white() /usr/include/opencv2/viz/types.hpp:82
	pub fn cv_viz_Color_white(ocvrs_return: *mut Result<*mut c_void>);
	// gray() /usr/include/opencv2/viz/types.hpp:84
	pub fn cv_viz_Color_gray(ocvrs_return: *mut Result<*mut c_void>);
	// silver() /usr/include/opencv2/viz/types.hpp:85
	pub fn cv_viz_Color_silver(ocvrs_return: *mut Result<*mut c_void>);
	// mlab() /usr/include/opencv2/viz/types.hpp:87
	pub fn cv_viz_Color_mlab(ocvrs_return: *mut Result<*mut c_void>);
	// navy() /usr/include/opencv2/viz/types.hpp:89
	pub fn cv_viz_Color_navy(ocvrs_return: *mut Result<*mut c_void>);
	// maroon() /usr/include/opencv2/viz/types.hpp:90
	pub fn cv_viz_Color_maroon(ocvrs_return: *mut Result<*mut c_void>);
	// teal() /usr/include/opencv2/viz/types.hpp:91
	pub fn cv_viz_Color_teal(ocvrs_return: *mut Result<*mut c_void>);
	// olive() /usr/include/opencv2/viz/types.hpp:92
	pub fn cv_viz_Color_olive(ocvrs_return: *mut Result<*mut c_void>);
	// purple() /usr/include/opencv2/viz/types.hpp:93
	pub fn cv_viz_Color_purple(ocvrs_return: *mut Result<*mut c_void>);
	// azure() /usr/include/opencv2/viz/types.hpp:94
	pub fn cv_viz_Color_azure(ocvrs_return: *mut Result<*mut c_void>);
	// chartreuse() /usr/include/opencv2/viz/types.hpp:95
	pub fn cv_viz_Color_chartreuse(ocvrs_return: *mut Result<*mut c_void>);
	// rose() /usr/include/opencv2/viz/types.hpp:96
	pub fn cv_viz_Color_rose(ocvrs_return: *mut Result<*mut c_void>);
	// lime() /usr/include/opencv2/viz/types.hpp:98
	pub fn cv_viz_Color_lime(ocvrs_return: *mut Result<*mut c_void>);
	// gold() /usr/include/opencv2/viz/types.hpp:99
	pub fn cv_viz_Color_gold(ocvrs_return: *mut Result<*mut c_void>);
	// orange() /usr/include/opencv2/viz/types.hpp:100
	pub fn cv_viz_Color_orange(ocvrs_return: *mut Result<*mut c_void>);
	// orange_red() /usr/include/opencv2/viz/types.hpp:101
	pub fn cv_viz_Color_orange_red(ocvrs_return: *mut Result<*mut c_void>);
	// indigo() /usr/include/opencv2/viz/types.hpp:102
	pub fn cv_viz_Color_indigo(ocvrs_return: *mut Result<*mut c_void>);
	// brown() /usr/include/opencv2/viz/types.hpp:104
	pub fn cv_viz_Color_brown(ocvrs_return: *mut Result<*mut c_void>);
	// apricot() /usr/include/opencv2/viz/types.hpp:105
	pub fn cv_viz_Color_apricot(ocvrs_return: *mut Result<*mut c_void>);
	// pink() /usr/include/opencv2/viz/types.hpp:106
	pub fn cv_viz_Color_pink(ocvrs_return: *mut Result<*mut c_void>);
	// raspberry() /usr/include/opencv2/viz/types.hpp:107
	pub fn cv_viz_Color_raspberry(ocvrs_return: *mut Result<*mut c_void>);
	// cherry() /usr/include/opencv2/viz/types.hpp:108
	pub fn cv_viz_Color_cherry(ocvrs_return: *mut Result<*mut c_void>);
	// violet() /usr/include/opencv2/viz/types.hpp:109
	pub fn cv_viz_Color_violet(ocvrs_return: *mut Result<*mut c_void>);
	// amethyst() /usr/include/opencv2/viz/types.hpp:110
	pub fn cv_viz_Color_amethyst(ocvrs_return: *mut Result<*mut c_void>);
	// bluberry() /usr/include/opencv2/viz/types.hpp:111
	pub fn cv_viz_Color_bluberry(ocvrs_return: *mut Result<*mut c_void>);
	// celestial_blue() /usr/include/opencv2/viz/types.hpp:112
	pub fn cv_viz_Color_celestial_blue(ocvrs_return: *mut Result<*mut c_void>);
	// turquoise() /usr/include/opencv2/viz/types.hpp:113
	pub fn cv_viz_Color_turquoise(ocvrs_return: *mut Result<*mut c_void>);
	// not_set() /usr/include/opencv2/viz/types.hpp:115
	pub fn cv_viz_Color_not_set(ocvrs_return: *mut Result<*mut c_void>);
	// action /usr/include/opencv2/viz/types.hpp:302
	pub fn cv_viz_KeyboardEvent_getPropAction_const(instance: *const c_void, ocvrs_return: *mut crate::viz::KeyboardEvent_Action);
	// action /usr/include/opencv2/viz/types.hpp:302
	pub fn cv_viz_KeyboardEvent_setPropAction_Action(instance: *mut c_void, val: crate::viz::KeyboardEvent_Action);
	// symbol /usr/include/opencv2/viz/types.hpp:303
	pub fn cv_viz_KeyboardEvent_getPropSymbol_const(instance: *const c_void) -> *mut c_void;
	// symbol /usr/include/opencv2/viz/types.hpp:303
	pub fn cv_viz_KeyboardEvent_setPropSymbol_String(instance: *mut c_void, val: *mut c_char);
	// code /usr/include/opencv2/viz/types.hpp:304
	pub fn cv_viz_KeyboardEvent_getPropCode_const(instance: *const c_void) -> u8;
	// code /usr/include/opencv2/viz/types.hpp:304
	pub fn cv_viz_KeyboardEvent_setPropCode_unsigned_char(instance: *mut c_void, val: u8);
	// modifiers /usr/include/opencv2/viz/types.hpp:305
	pub fn cv_viz_KeyboardEvent_getPropModifiers_const(instance: *const c_void) -> i32;
	// modifiers /usr/include/opencv2/viz/types.hpp:305
	pub fn cv_viz_KeyboardEvent_setPropModifiers_int(instance: *mut c_void, val: i32);
	// KeyboardEvent(cv::viz::KeyboardEvent::Action, const cv::String &, unsigned char, int) /usr/include/opencv2/viz/types.hpp:300
	pub fn cv_viz_KeyboardEvent_KeyboardEvent_Action_const_StringR_unsigned_char_int(action: crate::viz::KeyboardEvent_Action, symbol: *const c_char, code: u8, modifiers: i32, ocvrs_return: *mut Result<*mut c_void>);
	// cloud /usr/include/opencv2/viz/types.hpp:129
	pub fn cv_viz_Mesh_getPropCloud_const(instance: *const c_void) -> *mut c_void;
	// cloud /usr/include/opencv2/viz/types.hpp:129
	pub fn cv_viz_Mesh_setPropCloud_Mat(instance: *mut c_void, val: *mut c_void);
	// colors /usr/include/opencv2/viz/types.hpp:130
	pub fn cv_viz_Mesh_getPropColors_const(instance: *const c_void) -> *mut c_void;
	// colors /usr/include/opencv2/viz/types.hpp:130
	pub fn cv_viz_Mesh_setPropColors_Mat(instance: *mut c_void, val: *mut c_void);
	// normals /usr/include/opencv2/viz/types.hpp:131
	pub fn cv_viz_Mesh_getPropNormals_const(instance: *const c_void) -> *mut c_void;
	// normals /usr/include/opencv2/viz/types.hpp:131
	pub fn cv_viz_Mesh_setPropNormals_Mat(instance: *mut c_void, val: *mut c_void);
	// polygons /usr/include/opencv2/viz/types.hpp:135
	pub fn cv_viz_Mesh_getPropPolygons_const(instance: *const c_void) -> *mut c_void;
	// polygons /usr/include/opencv2/viz/types.hpp:135
	pub fn cv_viz_Mesh_setPropPolygons_Mat(instance: *mut c_void, val: *mut c_void);
	// texture /usr/include/opencv2/viz/types.hpp:137
	pub fn cv_viz_Mesh_getPropTexture_const(instance: *const c_void) -> *mut c_void;
	// texture /usr/include/opencv2/viz/types.hpp:137
	pub fn cv_viz_Mesh_setPropTexture_Mat(instance: *mut c_void, val: *mut c_void);
	// tcoords /usr/include/opencv2/viz/types.hpp:138
	pub fn cv_viz_Mesh_getPropTcoords_const(instance: *const c_void) -> *mut c_void;
	// tcoords /usr/include/opencv2/viz/types.hpp:138
	pub fn cv_viz_Mesh_setPropTcoords_Mat(instance: *mut c_void, val: *mut c_void);
	// Mesh() /usr/include/opencv2/viz/types.hpp:140
	pub fn cv_viz_Mesh_Mesh(ocvrs_return: *mut Result<*mut c_void>);
	// load(const cv::String &, int) /usr/include/opencv2/viz/types.hpp:154
	pub fn cv_viz_Mesh_load_const_StringR_int(file: *const c_char, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// type /usr/include/opencv2/viz/types.hpp:327
	pub fn cv_viz_MouseEvent_getPropType_const(instance: *const c_void, ocvrs_return: *mut crate::viz::MouseEvent_Type);
	// type /usr/include/opencv2/viz/types.hpp:327
	pub fn cv_viz_MouseEvent_setPropType_Type(instance: *mut c_void, val: crate::viz::MouseEvent_Type);
	// button /usr/include/opencv2/viz/types.hpp:328
	pub fn cv_viz_MouseEvent_getPropButton_const(instance: *const c_void, ocvrs_return: *mut crate::viz::MouseEvent_MouseButton);
	// button /usr/include/opencv2/viz/types.hpp:328
	pub fn cv_viz_MouseEvent_setPropButton_MouseButton(instance: *mut c_void, val: crate::viz::MouseEvent_MouseButton);
	// pointer /usr/include/opencv2/viz/types.hpp:329
	pub fn cv_viz_MouseEvent_getPropPointer_const(instance: *const c_void, ocvrs_return: *mut core::Point);
	// pointer /usr/include/opencv2/viz/types.hpp:329
	pub fn cv_viz_MouseEvent_setPropPointer_Point(instance: *mut c_void, val: *const core::Point);
	// modifiers /usr/include/opencv2/viz/types.hpp:330
	pub fn cv_viz_MouseEvent_getPropModifiers_const(instance: *const c_void) -> i32;
	// modifiers /usr/include/opencv2/viz/types.hpp:330
	pub fn cv_viz_MouseEvent_setPropModifiers_int(instance: *mut c_void, val: i32);
	// MouseEvent(const cv::viz::MouseEvent::Type &, const cv::viz::MouseEvent::MouseButton &, const cv::Point &, int) /usr/include/opencv2/viz/types.hpp:325
	pub fn cv_viz_MouseEvent_MouseEvent_const_TypeR_const_MouseButtonR_const_PointR_int(typ: *const crate::viz::MouseEvent_Type, button: *const crate::viz::MouseEvent_MouseButton, pointer: *const core::Point, modifiers: i32, ocvrs_return: *mut Result<*mut c_void>);
	// Viz3d(const cv::String &) /usr/include/opencv2/viz/viz3d.hpp:78
	pub fn cv_viz_Viz3d_Viz3d_const_StringR(window_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// Viz3d(const cv::viz::Viz3d &) /usr/include/opencv2/viz/viz3d.hpp:79
	pub fn cv_viz_Viz3d_Viz3d_const_Viz3dR(unnamed: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// showWidget(const cv::String &, const cv::viz::Widget &, const cv::Affine3d &) /usr/include/opencv2/viz/viz3d.hpp:88
	pub fn cv_viz_Viz3d_showWidget_const_StringR_const_WidgetR_const_Affine3dR(instance: *mut c_void, id: *const c_char, widget: *const c_void, pose: *const core::Affine3d, ocvrs_return: *mut Result_void);
	// removeWidget(const cv::String &) /usr/include/opencv2/viz/viz3d.hpp:94
	pub fn cv_viz_Viz3d_removeWidget_const_StringR(instance: *mut c_void, id: *const c_char, ocvrs_return: *mut Result_void);
	// getWidget(const cv::String &) /usr/include/opencv2/viz/viz3d.hpp:103
	pub fn cv_viz_Viz3d_getWidget_const_const_StringR(instance: *const c_void, id: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// removeAllWidgets() /usr/include/opencv2/viz/viz3d.hpp:107
	pub fn cv_viz_Viz3d_removeAllWidgets(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// showImage(cv::InputArray, const cv::Size &) /usr/include/opencv2/viz/viz3d.hpp:114
	pub fn cv_viz_Viz3d_showImage_const__InputArrayR_const_SizeR(instance: *mut c_void, image: *const c_void, window_size: *const core::Size, ocvrs_return: *mut Result_void);
	// setWidgetPose(const cv::String &, const cv::Affine3d &) /usr/include/opencv2/viz/viz3d.hpp:120
	pub fn cv_viz_Viz3d_setWidgetPose_const_StringR_const_Affine3dR(instance: *mut c_void, id: *const c_char, pose: *const core::Affine3d, ocvrs_return: *mut Result_void);
	// updateWidgetPose(const cv::String &, const cv::Affine3d &) /usr/include/opencv2/viz/viz3d.hpp:127
	pub fn cv_viz_Viz3d_updateWidgetPose_const_StringR_const_Affine3dR(instance: *mut c_void, id: *const c_char, pose: *const core::Affine3d, ocvrs_return: *mut Result_void);
	// getWidgetPose(const cv::String &) /usr/include/opencv2/viz/viz3d.hpp:133
	pub fn cv_viz_Viz3d_getWidgetPose_const_const_StringR(instance: *const c_void, id: *const c_char, ocvrs_return: *mut Result<core::Affine3d>);
	// setCamera(const cv::viz::Camera &) /usr/include/opencv2/viz/viz3d.hpp:139
	pub fn cv_viz_Viz3d_setCamera_const_CameraR(instance: *mut c_void, camera: *const c_void, ocvrs_return: *mut Result_void);
	// getCamera() /usr/include/opencv2/viz/viz3d.hpp:143
	pub fn cv_viz_Viz3d_getCamera_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getViewerPose() /usr/include/opencv2/viz/viz3d.hpp:147
	pub fn cv_viz_Viz3d_getViewerPose_const(instance: *const c_void, ocvrs_return: *mut Result<core::Affine3d>);
	// setViewerPose(const cv::Affine3d &) /usr/include/opencv2/viz/viz3d.hpp:153
	pub fn cv_viz_Viz3d_setViewerPose_const_Affine3dR(instance: *mut c_void, pose: *const core::Affine3d, ocvrs_return: *mut Result_void);
	// resetCameraViewpoint(const cv::String &) /usr/include/opencv2/viz/viz3d.hpp:159
	pub fn cv_viz_Viz3d_resetCameraViewpoint_const_StringR(instance: *mut c_void, id: *const c_char, ocvrs_return: *mut Result_void);
	// resetCamera() /usr/include/opencv2/viz/viz3d.hpp:163
	pub fn cv_viz_Viz3d_resetCamera(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// convertToWindowCoordinates(const cv::Point3d &, cv::Point3d &) /usr/include/opencv2/viz/viz3d.hpp:170
	pub fn cv_viz_Viz3d_convertToWindowCoordinates_const_Point3dR_Point3dR(instance: *mut c_void, pt: *const core::Point3d, window_coord: *mut core::Point3d, ocvrs_return: *mut Result_void);
	// converTo3DRay(const cv::Point3d &, cv::Point3d &, cv::Vec3d &) /usr/include/opencv2/viz/viz3d.hpp:177
	pub fn cv_viz_Viz3d_converTo3DRay_const_Point3dR_Point3dR_Vec3dR(instance: *mut c_void, window_coord: *const core::Point3d, origin: *mut core::Point3d, direction: *mut core::Vec3d, ocvrs_return: *mut Result_void);
	// getWindowSize() /usr/include/opencv2/viz/viz3d.hpp:181
	pub fn cv_viz_Viz3d_getWindowSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
	// setWindowSize(const cv::Size &) /usr/include/opencv2/viz/viz3d.hpp:186
	pub fn cv_viz_Viz3d_setWindowSize_const_SizeR(instance: *mut c_void, window_size: *const core::Size, ocvrs_return: *mut Result_void);
	// getWindowName() /usr/include/opencv2/viz/viz3d.hpp:191
	pub fn cv_viz_Viz3d_getWindowName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getScreenshot() /usr/include/opencv2/viz/viz3d.hpp:195
	pub fn cv_viz_Viz3d_getScreenshot_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// saveScreenshot(const cv::String &) /usr/include/opencv2/viz/viz3d.hpp:201
	pub fn cv_viz_Viz3d_saveScreenshot_const_StringR(instance: *mut c_void, file: *const c_char, ocvrs_return: *mut Result_void);
	// setWindowPosition(const cv::Point &) /usr/include/opencv2/viz/viz3d.hpp:207
	pub fn cv_viz_Viz3d_setWindowPosition_const_PointR(instance: *mut c_void, window_position: *const core::Point, ocvrs_return: *mut Result_void);
	// setFullScreen(bool) /usr/include/opencv2/viz/viz3d.hpp:213
	pub fn cv_viz_Viz3d_setFullScreen_bool(instance: *mut c_void, mode: bool, ocvrs_return: *mut Result_void);
	// setBackgroundColor(const cv::viz::Viz3d::Color &, const cv::viz::Viz3d::Color &) /usr/include/opencv2/viz/viz3d.hpp:217
	pub fn cv_viz_Viz3d_setBackgroundColor_const_ColorR_const_ColorR(instance: *mut c_void, color: *const c_void, color2: *const c_void, ocvrs_return: *mut Result_void);
	// setBackgroundTexture(cv::InputArray) /usr/include/opencv2/viz/viz3d.hpp:218
	pub fn cv_viz_Viz3d_setBackgroundTexture_const__InputArrayR(instance: *mut c_void, image: *const c_void, ocvrs_return: *mut Result_void);
	// setBackgroundMeshLab() /usr/include/opencv2/viz/viz3d.hpp:219
	pub fn cv_viz_Viz3d_setBackgroundMeshLab(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// spin() /usr/include/opencv2/viz/viz3d.hpp:223
	pub fn cv_viz_Viz3d_spin(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// spinOnce(int, bool) /usr/include/opencv2/viz/viz3d.hpp:230
	pub fn cv_viz_Viz3d_spinOnce_int_bool(instance: *mut c_void, time: i32, force_redraw: bool, ocvrs_return: *mut Result_void);
	// setOffScreenRendering() /usr/include/opencv2/viz/viz3d.hpp:234
	pub fn cv_viz_Viz3d_setOffScreenRendering(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// removeAllLights() /usr/include/opencv2/viz/viz3d.hpp:238
	pub fn cv_viz_Viz3d_removeAllLights(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// addLight(const cv::Vec3d &, const cv::Vec3d &, const cv::viz::Viz3d::Color &, const cv::viz::Viz3d::Color &, const cv::viz::Viz3d::Color &, const cv::viz::Viz3d::Color &) /usr/include/opencv2/viz/viz3d.hpp:249
	pub fn cv_viz_Viz3d_addLight_const_Vec3dR_const_Vec3dR_const_ColorR_const_ColorR_const_ColorR_const_ColorR(instance: *mut c_void, position: *const core::Vec3d, focal_point: *const core::Vec3d, color: *const c_void, diffuse_color: *const c_void, ambient_color: *const c_void, specular_color: *const c_void, ocvrs_return: *mut Result_void);
	// wasStopped() /usr/include/opencv2/viz/viz3d.hpp:255
	pub fn cv_viz_Viz3d_wasStopped_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// close() /usr/include/opencv2/viz/viz3d.hpp:256
	pub fn cv_viz_Viz3d_close(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// registerKeyboardCallback(cv::viz::Viz3d::KeyboardCallback, void *) /usr/include/opencv2/viz/viz3d.hpp:264
	pub fn cv_viz_Viz3d_registerKeyboardCallback_KeyboardCallback_voidX(instance: *mut c_void, callback: Option<unsafe extern "C" fn(*const c_void, *mut c_void) -> ()>, cookie: *mut c_void, ocvrs_return: *mut Result_void);
	// registerMouseCallback(cv::viz::Viz3d::MouseCallback, void *) /usr/include/opencv2/viz/viz3d.hpp:271
	pub fn cv_viz_Viz3d_registerMouseCallback_MouseCallback_voidX(instance: *mut c_void, callback: Option<unsafe extern "C" fn(*const c_void, *mut c_void) -> ()>, cookie: *mut c_void, ocvrs_return: *mut Result_void);
	// setRenderingProperty(const cv::String &, int, double) /usr/include/opencv2/viz/viz3d.hpp:299
	pub fn cv_viz_Viz3d_setRenderingProperty_const_StringR_int_double(instance: *mut c_void, id: *const c_char, property: i32, value: f64, ocvrs_return: *mut Result_void);
	// getRenderingProperty(const cv::String &, int) /usr/include/opencv2/viz/viz3d.hpp:325
	pub fn cv_viz_Viz3d_getRenderingProperty_const_StringR_int(instance: *mut c_void, id: *const c_char, property: i32, ocvrs_return: *mut Result<f64>);
	// setRepresentation(int) /usr/include/opencv2/viz/viz3d.hpp:334
	pub fn cv_viz_Viz3d_setRepresentation_int(instance: *mut c_void, representation: i32, ocvrs_return: *mut Result_void);
	// setGlobalWarnings(bool) /usr/include/opencv2/viz/viz3d.hpp:336
	pub fn cv_viz_Viz3d_setGlobalWarnings_bool(instance: *mut c_void, enabled: bool, ocvrs_return: *mut Result_void);
	// WArrow(const cv::Point3d &, const cv::Point3d &, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:301
	pub fn cv_viz_WArrow_WArrow_const_Point3dR_const_Point3dR_double_const_ColorR(pt1: *const core::Point3d, pt2: *const core::Point3d, thickness: f64, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WCameraPosition(double) /usr/include/opencv2/viz/widgets.hpp:550
	pub fn cv_viz_WCameraPosition_WCameraPosition_double(scale: f64, ocvrs_return: *mut Result<*mut c_void>);
	// WCameraPosition(const cv::Matx33d &, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:560
	pub fn cv_viz_WCameraPosition_WCameraPosition_const_Matx33dR_double_const_ColorR(k: *const core::Matx33d, scale: f64, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WCameraPosition(const cv::Vec2d &, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:570
	pub fn cv_viz_WCameraPosition_WCameraPosition_const_Vec2dR_double_const_ColorR(fov: *const core::Vec2d, scale: f64, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WCameraPosition(const cv::Matx33d &, cv::InputArray, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:583
	pub fn cv_viz_WCameraPosition_WCameraPosition_const_Matx33dR_const__InputArrayR_double_const_ColorR(k: *const core::Matx33d, image: *const c_void, scale: f64, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WCameraPosition(const cv::Vec2d &, cv::InputArray, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:596
	pub fn cv_viz_WCameraPosition_WCameraPosition_const_Vec2dR_const__InputArrayR_double_const_ColorR(fov: *const core::Vec2d, image: *const c_void, scale: f64, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WCircle(double, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:315
	pub fn cv_viz_WCircle_WCircle_double_double_const_ColorR(radius: f64, thickness: f64, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WCircle(double, const cv::Point3d &, const cv::Vec3d &, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:325
	pub fn cv_viz_WCircle_WCircle_double_const_Point3dR_const_Vec3dR_double_const_ColorR(radius: f64, center: *const core::Point3d, normal: *const core::Vec3d, thickness: f64, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WCloud(cv::InputArray, cv::InputArray) /usr/include/opencv2/viz/widgets.hpp:690
	pub fn cv_viz_WCloud_WCloud_const__InputArrayR_const__InputArrayR(cloud: *const c_void, colors: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WCloud(cv::InputArray, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:698
	pub fn cv_viz_WCloud_WCloud_const__InputArrayR_const_ColorR(cloud: *const c_void, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WCloud(cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/viz/widgets.hpp:707
	pub fn cv_viz_WCloud_WCloud_const__InputArrayR_const__InputArrayR_const__InputArrayR(cloud: *const c_void, colors: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WCloud(cv::InputArray, const cv::viz::Color &, cv::InputArray) /usr/include/opencv2/viz/widgets.hpp:717
	pub fn cv_viz_WCloud_WCloud_const__InputArrayR_const_ColorR_const__InputArrayR(cloud: *const c_void, color: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WCloudCollection() /usr/include/opencv2/viz/widgets.hpp:739
	pub fn cv_viz_WCloudCollection_WCloudCollection(ocvrs_return: *mut Result<*mut c_void>);
	// addCloud(cv::InputArray, cv::InputArray, const cv::Affine3d &) /usr/include/opencv2/viz/widgets.hpp:747
	pub fn cv_viz_WCloudCollection_addCloud_const__InputArrayR_const__InputArrayR_const_Affine3dR(instance: *mut c_void, cloud: *const c_void, colors: *const c_void, pose: *const core::Affine3d, ocvrs_return: *mut Result_void);
	// addCloud(cv::InputArray, const cv::viz::Color &, const cv::Affine3d &) /usr/include/opencv2/viz/widgets.hpp:754
	pub fn cv_viz_WCloudCollection_addCloud_const__InputArrayR_const_ColorR_const_Affine3dR(instance: *mut c_void, cloud: *const c_void, color: *const c_void, pose: *const core::Affine3d, ocvrs_return: *mut Result_void);
	// finalize() /usr/include/opencv2/viz/widgets.hpp:759
	pub fn cv_viz_WCloudCollection_finalize(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// WCloudNormals(cv::InputArray, cv::InputArray, int, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:777
	pub fn cv_viz_WCloudNormals_WCloudNormals_const__InputArrayR_const__InputArrayR_int_double_const_ColorR(cloud: *const c_void, normals: *const c_void, level: i32, scale: f64, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WCone(double, double, int, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:340
	pub fn cv_viz_WCone_WCone_double_double_int_const_ColorR(length: f64, radius: f64, resolution: i32, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WCone(double, const cv::Point3d &, const cv::Point3d &, int, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:351
	pub fn cv_viz_WCone_WCone_double_const_Point3dR_const_Point3dR_int_const_ColorR(radius: f64, center: *const core::Point3d, tip: *const core::Point3d, resolution: i32, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WCoordinateSystem(double) /usr/include/opencv2/viz/widgets.hpp:520
	pub fn cv_viz_WCoordinateSystem_WCoordinateSystem_double(scale: f64, ocvrs_return: *mut Result<*mut c_void>);
	// WCube(const cv::Point3d &, const cv::Point3d &, bool, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:384
	pub fn cv_viz_WCube_WCube_const_Point3dR_const_Point3dR_bool_const_ColorR(min_point: *const core::Point3d, max_point: *const core::Point3d, wire_frame: bool, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WCylinder(const cv::Point3d &, const cv::Point3d &, double, int, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:367
	pub fn cv_viz_WCylinder_WCylinder_const_Point3dR_const_Point3dR_double_int_const_ColorR(axis_point1: *const core::Point3d, axis_point2: *const core::Point3d, radius: f64, numsides: i32, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WGrid(const cv::Vec2i &, const cv::Vec2d &, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:534
	pub fn cv_viz_WGrid_WGrid_const_Vec2iR_const_Vec2dR_const_ColorR(cells: *const core::Vec2i, cells_spacing: *const core::Vec2d, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WGrid(const cv::Point3d &, const cv::Vec3d &, const cv::Vec3d &, const cv::Vec2i &, const cv::Vec2d &, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:537
	pub fn cv_viz_WGrid_WGrid_const_Point3dR_const_Vec3dR_const_Vec3dR_const_Vec2iR_const_Vec2dR_const_ColorR(center: *const core::Point3d, normal: *const core::Vec3d, new_yaxis: *const core::Vec3d, cells: *const core::Vec2i, cells_spacing: *const core::Vec2d, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WImage3D(cv::InputArray, const cv::Size2d &) /usr/include/opencv2/viz/widgets.hpp:483
	pub fn cv_viz_WImage3D_WImage3D_const__InputArrayR_const_Size2dR(image: *const c_void, size: *const core::Size2d, ocvrs_return: *mut Result<*mut c_void>);
	// WImage3D(cv::InputArray, const cv::Size2d &, const cv::Vec3d &, const cv::Vec3d &, const cv::Vec3d &) /usr/include/opencv2/viz/widgets.hpp:493
	pub fn cv_viz_WImage3D_WImage3D_const__InputArrayR_const_Size2dR_const_Vec3dR_const_Vec3dR_const_Vec3dR(image: *const c_void, size: *const core::Size2d, center: *const core::Vec3d, normal: *const core::Vec3d, up_vector: *const core::Vec3d, ocvrs_return: *mut Result<*mut c_void>);
	// setImage(cv::InputArray) /usr/include/opencv2/viz/widgets.hpp:499
	pub fn cv_viz_WImage3D_setImage_const__InputArrayR(instance: *mut c_void, image: *const c_void, ocvrs_return: *mut Result_void);
	// setSize(const cv::Size &) /usr/include/opencv2/viz/widgets.hpp:505
	pub fn cv_viz_WImage3D_setSize_const_SizeR(instance: *mut c_void, size: *const core::Size, ocvrs_return: *mut Result_void);
	// WImageOverlay(cv::InputArray, const cv::Rect &) /usr/include/opencv2/viz/widgets.hpp:465
	pub fn cv_viz_WImageOverlay_WImageOverlay_const__InputArrayR_const_RectR(image: *const c_void, rect: *const core::Rect, ocvrs_return: *mut Result<*mut c_void>);
	// setImage(cv::InputArray) /usr/include/opencv2/viz/widgets.hpp:470
	pub fn cv_viz_WImageOverlay_setImage_const__InputArrayR(instance: *mut c_void, image: *const c_void, ocvrs_return: *mut Result_void);
	// WLine(const cv::Point3d &, const cv::Point3d &, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:244
	pub fn cv_viz_WLine_WLine_const_Point3dR_const_Point3dR_const_ColorR(pt1: *const core::Point3d, pt2: *const core::Point3d, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WMesh(const cv::viz::Mesh &) /usr/include/opencv2/viz/widgets.hpp:794
	pub fn cv_viz_WMesh_WMesh_const_MeshR(mesh: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WMesh(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/viz/widgets.hpp:795
	pub fn cv_viz_WMesh_WMesh_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(cloud: *const c_void, polygons: *const c_void, colors: *const c_void, normals: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WPaintedCloud(cv::InputArray) /usr/include/opencv2/viz/widgets.hpp:724
	pub fn cv_viz_WPaintedCloud_WPaintedCloud_const__InputArrayR(cloud: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WPaintedCloud(cv::InputArray, const cv::Point3d &, const cv::Point3d &) /usr/include/opencv2/viz/widgets.hpp:727
	pub fn cv_viz_WPaintedCloud_WPaintedCloud_const__InputArrayR_const_Point3dR_const_Point3dR(cloud: *const c_void, p1: *const core::Point3d, p2: *const core::Point3d, ocvrs_return: *mut Result<*mut c_void>);
	// WPaintedCloud(cv::InputArray, const cv::Point3d &, const cv::Point3d &, const cv::viz::Color &, const cv::viz::Color) /usr/include/opencv2/viz/widgets.hpp:730
	pub fn cv_viz_WPaintedCloud_WPaintedCloud_const__InputArrayR_const_Point3dR_const_Point3dR_const_ColorR_const_Color(cloud: *const c_void, p1: *const core::Point3d, p2: *const core::Point3d, c1: *const c_void, c2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WPlane(const cv::Size2d &, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:257
	pub fn cv_viz_WPlane_WPlane_const_Size2dR_const_ColorR(size: *const core::Size2d, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WPlane(const cv::Point3d &, const cv::Vec3d &, const cv::Vec3d &, const cv::Size2d &, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:267
	pub fn cv_viz_WPlane_WPlane_const_Point3dR_const_Vec3dR_const_Vec3dR_const_Size2dR_const_ColorR(center: *const core::Point3d, normal: *const core::Vec3d, new_yaxis: *const core::Vec3d, size: *const core::Size2d, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WPolyLine(cv::InputArray, cv::InputArray) /usr/include/opencv2/viz/widgets.hpp:393
	pub fn cv_viz_WPolyLine_WPolyLine_const__InputArrayR_const__InputArrayR(points: *const c_void, colors: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WPolyLine(cv::InputArray, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:399
	pub fn cv_viz_WPolyLine_WPolyLine_const__InputArrayR_const_ColorR(points: *const c_void, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WSphere(const cv::Point3d &, double, int, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:283
	pub fn cv_viz_WSphere_WSphere_const_Point3dR_double_int_const_ColorR(center: *const core::Point3d, radius: f64, sphere_resolution: i32, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WText(const cv::String &, const cv::Point &, int, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:417
	pub fn cv_viz_WText_WText_const_StringR_const_PointR_int_const_ColorR(text: *const c_char, pos: *const core::Point, font_size: i32, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setText(const cv::String &) /usr/include/opencv2/viz/widgets.hpp:423
	pub fn cv_viz_WText_setText_const_StringR(instance: *mut c_void, text: *const c_char, ocvrs_return: *mut Result_void);
	// getText() /usr/include/opencv2/viz/widgets.hpp:426
	pub fn cv_viz_WText_getText_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WText3D(const cv::String &, const cv::Point3d &, double, bool, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:442
	pub fn cv_viz_WText3D_WText3D_const_StringR_const_Point3dR_double_bool_const_ColorR(text: *const c_char, position: *const core::Point3d, text_scale: f64, face_camera: bool, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setText(const cv::String &) /usr/include/opencv2/viz/widgets.hpp:449
	pub fn cv_viz_WText3D_setText_const_StringR(instance: *mut c_void, text: *const c_char, ocvrs_return: *mut Result_void);
	// getText() /usr/include/opencv2/viz/widgets.hpp:452
	pub fn cv_viz_WText3D_getText_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WTrajectory(cv::InputArray, int, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:622
	pub fn cv_viz_WTrajectory_WTrajectory_const__InputArrayR_int_double_const_ColorR(path: *const c_void, display_mode: i32, scale: f64, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WTrajectoryFrustums(cv::InputArray, const cv::Matx33d &, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:639
	pub fn cv_viz_WTrajectoryFrustums_WTrajectoryFrustums_const__InputArrayR_const_Matx33dR_double_const_ColorR(path: *const c_void, k: *const core::Matx33d, scale: f64, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WTrajectoryFrustums(cv::InputArray, const cv::Vec2d &, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:650
	pub fn cv_viz_WTrajectoryFrustums_WTrajectoryFrustums_const__InputArrayR_const_Vec2dR_double_const_ColorR(path: *const c_void, fov: *const core::Vec2d, scale: f64, color: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WTrajectorySpheres(cv::InputArray, double, double, const cv::viz::Color &, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:669
	pub fn cv_viz_WTrajectorySpheres_WTrajectorySpheres_const__InputArrayR_double_double_const_ColorR_const_ColorR(path: *const c_void, line_length: f64, radius: f64, from: *const c_void, to: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// WWidgetMerger() /usr/include/opencv2/viz/widgets.hpp:808
	pub fn cv_viz_WWidgetMerger_WWidgetMerger(ocvrs_return: *mut Result<*mut c_void>);
	// addWidget(const cv::viz::Widget3D &, const cv::Affine3d &) /usr/include/opencv2/viz/widgets.hpp:811
	pub fn cv_viz_WWidgetMerger_addWidget_const_Widget3DR_const_Affine3dR(instance: *mut c_void, widget: *const c_void, pose: *const core::Affine3d, ocvrs_return: *mut Result_void);
	// finalize() /usr/include/opencv2/viz/widgets.hpp:814
	pub fn cv_viz_WWidgetMerger_finalize(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// Widget() /usr/include/opencv2/viz/widgets.hpp:95
	pub fn cv_viz_Widget_Widget(ocvrs_return: *mut Result<*mut c_void>);
	// Widget(const cv::viz::Widget &) /usr/include/opencv2/viz/widgets.hpp:96
	pub fn cv_viz_Widget_Widget_const_WidgetR(other: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// fromPlyFile(const cv::String &) /usr/include/opencv2/viz/widgets.hpp:104
	pub fn cv_viz_Widget_fromPlyFile_const_StringR(file_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// setRenderingProperty(int, double) /usr/include/opencv2/viz/widgets.hpp:131
	pub fn cv_viz_Widget_setRenderingProperty_int_double(instance: *mut c_void, property: i32, value: f64, ocvrs_return: *mut Result_void);
	// getRenderingProperty(int) /usr/include/opencv2/viz/widgets.hpp:157
	pub fn cv_viz_Widget_getRenderingProperty_const_int(instance: *const c_void, property: i32, ocvrs_return: *mut Result<f64>);
	// Widget2D() /usr/include/opencv2/viz/widgets.hpp:221
	pub fn cv_viz_Widget2D_Widget2D(ocvrs_return: *mut Result<*mut c_void>);
	// setColor(const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:227
	pub fn cv_viz_Widget2D_setColor_const_ColorR(instance: *mut c_void, color: *const c_void, ocvrs_return: *mut Result_void);
	// Widget3D() /usr/include/opencv2/viz/widgets.hpp:184
	pub fn cv_viz_Widget3D_Widget3D(ocvrs_return: *mut Result<*mut c_void>);
	// setPose(const cv::Affine3d &) /usr/include/opencv2/viz/widgets.hpp:190
	pub fn cv_viz_Widget3D_setPose_const_Affine3dR(instance: *mut c_void, pose: *const core::Affine3d, ocvrs_return: *mut Result_void);
	// updatePose(const cv::Affine3d &) /usr/include/opencv2/viz/widgets.hpp:195
	pub fn cv_viz_Widget3D_updatePose_const_Affine3dR(instance: *mut c_void, pose: *const core::Affine3d, ocvrs_return: *mut Result_void);
	// getPose() /usr/include/opencv2/viz/widgets.hpp:198
	pub fn cv_viz_Widget3D_getPose_const(instance: *const c_void, ocvrs_return: *mut Result<core::Affine3d>);
	// applyTransform(const cv::Affine3d &) /usr/include/opencv2/viz/widgets.hpp:204
	pub fn cv_viz_Widget3D_applyTransform_const_Affine3dR(instance: *mut c_void, transform: *const core::Affine3d, ocvrs_return: *mut Result_void);
	// setColor(const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:210
	pub fn cv_viz_Widget3D_setColor_const_ColorR(instance: *mut c_void, color: *const c_void, ocvrs_return: *mut Result_void);
}
