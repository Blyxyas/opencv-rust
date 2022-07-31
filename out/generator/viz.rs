#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # 3D Visualizer
//! 
//! This section describes 3D visualization window as well as classes and methods that are used to
//! interact with it.
//! 
//! 3D visualization window (see Viz3d) is used to display widgets (see Widget), and it provides several
//! methods to interact with scene and widgets.
//!    # Widget
//! 
//! In this section, the widget framework is explained. Widgets represent 2D or 3D objects, varying from
//! simple ones such as lines to complex ones such as point clouds and meshes.
//! 
//! Widgets are **implicitly shared**. Therefore, one can add a widget to the scene, and modify the
//! widget without re-adding the widget.
//! 
//! ```ignore
//! // Create a cloud widget
//! viz::WCloud cw(cloud, viz::Color::red());
//! // Display it in a window
//! myWindow.showWidget("CloudWidget1", cw);
//! // Modify it, and it will be modified in the window.
//! cw.setColor(viz::Color::yellow());
//! ```
//! 
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::ColorTraitConst, super::ColorTrait, super::MeshTraitConst, super::MeshTrait, super::CameraTraitConst, super::CameraTrait, super::KeyboardEventTraitConst, super::KeyboardEventTrait, super::MouseEventTraitConst, super::MouseEventTrait, super::WidgetTraitConst, super::WidgetTrait, super::Widget3DTraitConst, super::Widget3DTrait, super::Widget2DTraitConst, super::Widget2DTrait, super::WLineTraitConst, super::WLineTrait, super::WPlaneTraitConst, super::WPlaneTrait, super::WSphereTraitConst, super::WSphereTrait, super::WArrowTraitConst, super::WArrowTrait, super::WCircleTraitConst, super::WCircleTrait, super::WConeTraitConst, super::WConeTrait, super::WCylinderTraitConst, super::WCylinderTrait, super::WCubeTraitConst, super::WCubeTrait, super::WPolyLineTraitConst, super::WPolyLineTrait, super::WTextTraitConst, super::WTextTrait, super::WText3DTraitConst, super::WText3DTrait, super::WImageOverlayTraitConst, super::WImageOverlayTrait, super::WImage3DTraitConst, super::WImage3DTrait, super::WCoordinateSystemTraitConst, super::WCoordinateSystemTrait, super::WGridTraitConst, super::WGridTrait, super::WCameraPositionTraitConst, super::WCameraPositionTrait, super::WTrajectoryTraitConst, super::WTrajectoryTrait, super::WTrajectoryFrustumsTraitConst, super::WTrajectoryFrustumsTrait, super::WTrajectorySpheresTraitConst, super::WTrajectorySpheresTrait, super::WCloudTraitConst, super::WCloudTrait, super::WPaintedCloudTraitConst, super::WPaintedCloudTrait, super::WCloudCollectionTraitConst, super::WCloudCollectionTrait, super::WCloudNormalsTraitConst, super::WCloudNormalsTrait, super::WMeshTraitConst, super::WMeshTrait, super::WWidgetMergerTraitConst, super::WWidgetMergerTrait, super::Viz3dTraitConst, super::Viz3dTrait };
}

// AMBIENT /usr/include/opencv2/viz/widgets.hpp:70
pub const AMBIENT: i32 = 7;
// FONT_SIZE /usr/include/opencv2/viz/widgets.hpp:66
pub const FONT_SIZE: i32 = 3;
// IMMEDIATE_RENDERING /usr/include/opencv2/viz/widgets.hpp:68
pub const IMMEDIATE_RENDERING: i32 = 5;
// ALT /usr/include/opencv2/viz/types.hpp:290
pub const KeyboardEvent_ALT: i32 = 1;
// CTRL /usr/include/opencv2/viz/types.hpp:290
pub const KeyboardEvent_CTRL: i32 = 2;
// NONE /usr/include/opencv2/viz/types.hpp:290
pub const KeyboardEvent_NONE: i32 = 0;
// SHIFT /usr/include/opencv2/viz/types.hpp:290
pub const KeyboardEvent_SHIFT: i32 = 4;
// LIGHTING /usr/include/opencv2/viz/widgets.hpp:71
pub const LIGHTING: i32 = 8;
// LINE_WIDTH /usr/include/opencv2/viz/widgets.hpp:65
pub const LINE_WIDTH: i32 = 2;
// LOAD_AUTO /usr/include/opencv2/viz/types.hpp:124
pub const Mesh_LOAD_AUTO: i32 = 0;
// LOAD_OBJ /usr/include/opencv2/viz/types.hpp:126
pub const Mesh_LOAD_OBJ: i32 = 2;
// LOAD_PLY /usr/include/opencv2/viz/types.hpp:125
pub const Mesh_LOAD_PLY: i32 = 1;
// OPACITY /usr/include/opencv2/viz/widgets.hpp:64
pub const OPACITY: i32 = 1;
// POINT_SIZE /usr/include/opencv2/viz/widgets.hpp:63
pub const POINT_SIZE: i32 = 0;
// REPRESENTATION /usr/include/opencv2/viz/widgets.hpp:67
pub const REPRESENTATION: i32 = 4;
// REPRESENTATION_POINTS /usr/include/opencv2/viz/widgets.hpp:76
pub const REPRESENTATION_POINTS: i32 = 0;
// REPRESENTATION_SURFACE /usr/include/opencv2/viz/widgets.hpp:78
pub const REPRESENTATION_SURFACE: i32 = 2;
// REPRESENTATION_WIREFRAME /usr/include/opencv2/viz/widgets.hpp:77
pub const REPRESENTATION_WIREFRAME: i32 = 1;
// SHADING /usr/include/opencv2/viz/widgets.hpp:69
pub const SHADING: i32 = 6;
// SHADING_FLAT /usr/include/opencv2/viz/widgets.hpp:83
pub const SHADING_FLAT: i32 = 0;
// SHADING_GOURAUD /usr/include/opencv2/viz/widgets.hpp:84
pub const SHADING_GOURAUD: i32 = 1;
// SHADING_PHONG /usr/include/opencv2/viz/widgets.hpp:85
pub const SHADING_PHONG: i32 = 2;
// BOTH /usr/include/opencv2/viz/widgets.hpp:607
pub const WTrajectory_BOTH: i32 = 3;
// FRAMES /usr/include/opencv2/viz/widgets.hpp:607
pub const WTrajectory_FRAMES: i32 = 1;
// PATH /usr/include/opencv2/viz/widgets.hpp:607
pub const WTrajectory_PATH: i32 = 2;
// Action /usr/include/opencv2/viz/types.hpp:291
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum KeyboardEvent_Action {
	KEY_UP = 0,
	KEY_DOWN = 1,
}

opencv_type_enum! { crate::viz::KeyboardEvent_Action }

// MouseButton /usr/include/opencv2/viz/types.hpp:314
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MouseEvent_MouseButton {
	NoButton = 0,
	LeftButton = 1,
	MiddleButton = 2,
	RightButton = 3,
	VScroll = 4,
}

opencv_type_enum! { crate::viz::MouseEvent_MouseButton }

// Type /usr/include/opencv2/viz/types.hpp:313
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MouseEvent_Type {
	MouseMove = 1,
	MouseButtonPress = 2,
	MouseButtonRelease = 3,
	MouseScrollDown = 4,
	MouseScrollUp = 5,
	MouseDblClick = 6,
}

opencv_type_enum! { crate::viz::MouseEvent_Type }

// RenderingProperties /usr/include/opencv2/viz/widgets.hpp:61
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RenderingProperties {
	POINT_SIZE = 0,
	OPACITY = 1,
	LINE_WIDTH = 2,
	FONT_SIZE = 3,
	REPRESENTATION = 4,
	IMMEDIATE_RENDERING = 5,
	SHADING = 6,
	AMBIENT = 7,
	LIGHTING = 8,
}

opencv_type_enum! { crate::viz::RenderingProperties }

// RepresentationValues /usr/include/opencv2/viz/widgets.hpp:74
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RepresentationValues {
	REPRESENTATION_POINTS = 0,
	REPRESENTATION_WIREFRAME = 1,
	REPRESENTATION_SURFACE = 2,
}

opencv_type_enum! { crate::viz::RepresentationValues }

// ShadingValues /usr/include/opencv2/viz/widgets.hpp:81
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShadingValues {
	SHADING_FLAT = 0,
	SHADING_GOURAUD = 1,
	SHADING_PHONG = 2,
}

opencv_type_enum! { crate::viz::ShadingValues }

// Color /usr/include/opencv2/viz/viz3d.hpp:70
pub type Viz3d_Color = crate::viz::Color;
// KeyboardCallback /usr/include/opencv2/viz/viz3d.hpp:71
pub type Viz3d_KeyboardCallback = Option<Box<dyn FnMut(*const c_void) -> () + Send + Sync + 'static>>;
// MouseCallback /usr/include/opencv2/viz/viz3d.hpp:72
pub type Viz3d_MouseCallback = Option<Box<dyn FnMut(*const c_void) -> () + Send + Sync + 'static>>;
// computeNormals(const cv::viz::Mesh &, cv::OutputArray) /usr/include/opencv2/viz/vizcore.hpp:214
#[inline]
pub fn compute_normals(mesh: &crate::viz::Mesh, normals: &mut dyn core::ToOutputArray) -> Result<()> {
	output_array_arg!(normals);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_viz_computeNormals_const_MeshR_const__OutputArrayR(mesh.as_raw_Mesh(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getWindowByName(const cv::String &) /usr/include/opencv2/viz/vizcore.hpp:97
#[inline]
pub fn get_window_by_name(window_name: &str) -> Result<crate::viz::Viz3d> {
	extern_container_arg!(window_name);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_viz_getWindowByName_const_StringR(window_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::viz::Viz3d::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * window_size: Size(-1,-1)
// imshow(const cv::String &, cv::InputArray, const cv::Size &) /usr/include/opencv2/viz/vizcore.hpp:103
#[inline]
pub fn imshow(window_name: &str, image: &dyn core::ToInputArray, window_size: core::Size) -> Result<crate::viz::Viz3d> {
	extern_container_arg!(window_name);
	input_array_arg!(image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_viz_imshow_const_StringR_const__InputArrayR_const_SizeR(window_name.opencv_as_extern(), image.as_raw__InputArray(), &window_size, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::viz::Viz3d::opencv_from_extern(ret) };
	Ok(ret)
}

// makeCameraPose(const cv::Vec3d &, const cv::Vec3d &, const cv::Vec3d &) /usr/include/opencv2/viz/vizcore.hpp:84
#[inline]
pub fn make_camera_pose(position: core::Vec3d, focal_point: core::Vec3d, y_dir: core::Vec3d) -> Result<core::Affine3d> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_viz_makeCameraPose_const_Vec3dR_const_Vec3dR_const_Vec3dR(&position, &focal_point, &y_dir, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * origin: Vec3d::all(0)
// makeTransformToGlobal(const cv::Vec3d &, const cv::Vec3d &, const cv::Vec3d &, const cv::Vec3d &) /usr/include/opencv2/viz/vizcore.hpp:73
#[inline]
pub fn make_transform_to_global(axis_x: core::Vec3d, axis_y: core::Vec3d, axis_z: core::Vec3d, origin: core::Vec3d) -> Result<core::Affine3d> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_viz_makeTransformToGlobal_const_Vec3dR_const_Vec3dR_const_Vec3dR_const_Vec3dR(&axis_x, &axis_y, &axis_z, &origin, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * colors: noArray()
/// * normals: noArray()
// readCloud(const cv::String &, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/viz/vizcore.hpp:160
#[inline]
pub fn read_cloud(file: &str, colors: &mut dyn core::ToOutputArray, normals: &mut dyn core::ToOutputArray) -> Result<core::Mat> {
	extern_container_arg!(file);
	output_array_arg!(colors);
	output_array_arg!(normals);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_viz_readCloud_const_StringR_const__OutputArrayR_const__OutputArrayR(file.opencv_as_extern(), colors.as_raw__OutputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

// readMesh(const cv::String &) /usr/include/opencv2/viz/vizcore.hpp:165
#[inline]
pub fn read_mesh(file: &str) -> Result<crate::viz::Mesh> {
	extern_container_arg!(file);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_viz_readMesh_const_StringR(file.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::viz::Mesh::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * tag: "pose"
// readPose(const cv::String &, cv::Affine3d &, const cv::String &) /usr/include/opencv2/viz/vizcore.hpp:175
#[inline]
pub fn read_pose(file: &str, pose: &mut core::Affine3d, tag: &str) -> Result<bool> {
	extern_container_arg!(file);
	extern_container_arg!(tag);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_viz_readPose_const_StringR_Affine3dR_const_StringR(file.opencv_as_extern(), pose, tag.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * files_format: "pose%05d.xml"
/// * start: 0
/// * end: INT_MAX
/// * tag: "pose"
// readTrajectory(cv::OutputArray, const cv::String &, int, int, const cv::String &) /usr/include/opencv2/viz/vizcore.hpp:206
#[inline]
pub fn read_trajectory(traj: &mut dyn core::ToOutputArray, files_format: &str, start: i32, end: i32, tag: &str) -> Result<()> {
	output_array_arg!(traj);
	extern_container_arg!(files_format);
	extern_container_arg!(tag);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_viz_readTrajectory_const__OutputArrayR_const_StringR_int_int_const_StringR(traj.as_raw__OutputArray(), files_format.opencv_as_extern(), start, end, tag.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// unregisterAllWindows() /usr/include/opencv2/viz/vizcore.hpp:100
#[inline]
pub fn unregister_all_windows() -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_viz_unregisterAllWindows(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * colors: noArray()
/// * normals: noArray()
/// * binary: false
// writeCloud(const cv::String &, cv::InputArray, cv::InputArray, cv::InputArray, bool) /usr/include/opencv2/viz/vizcore.hpp:151
#[inline]
pub fn write_cloud(file: &str, cloud: &dyn core::ToInputArray, colors: &dyn core::ToInputArray, normals: &dyn core::ToInputArray, binary: bool) -> Result<()> {
	extern_container_arg!(file);
	input_array_arg!(cloud);
	input_array_arg!(colors);
	input_array_arg!(normals);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_viz_writeCloud_const_StringR_const__InputArrayR_const__InputArrayR_const__InputArrayR_bool(file.opencv_as_extern(), cloud.as_raw__InputArray(), colors.as_raw__InputArray(), normals.as_raw__InputArray(), binary, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * tag: "pose"
// writePose(const cv::String &, const cv::Affine3d &, const cv::String &) /usr/include/opencv2/viz/vizcore.hpp:181
#[inline]
pub fn write_pose(file: &str, pose: core::Affine3d, tag: &str) -> Result<()> {
	extern_container_arg!(file);
	extern_container_arg!(tag);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_viz_writePose_const_StringR_const_Affine3dR_const_StringR(file.opencv_as_extern(), &pose, tag.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * files_format: "pose%05d.xml"
/// * start: 0
/// * tag: "pose"
// writeTrajectory(cv::InputArray, const cv::String &, int, const cv::String &) /usr/include/opencv2/viz/vizcore.hpp:193
#[inline]
pub fn write_trajectory(traj: &dyn core::ToInputArray, files_format: &str, start: i32, tag: &str) -> Result<()> {
	input_array_arg!(traj);
	extern_container_arg!(files_format);
	extern_container_arg!(tag);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_viz_writeTrajectory_const__InputArrayR_const_StringR_int_const_StringR(traj.as_raw__InputArray(), files_format.opencv_as_extern(), start, tag.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// Camera /usr/include/opencv2/viz/types.hpp:163
pub trait CameraTraitConst {
	fn as_raw_Camera(&self) -> *const c_void;

	// getClip() /usr/include/opencv2/viz/types.hpp:215
	#[inline]
	fn get_clip(&self) -> Result<core::Vec2d> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Camera_getClip_const(self.as_raw_Camera(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getWindowSize() /usr/include/opencv2/viz/types.hpp:218
	#[inline]
	fn get_window_size(&self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Camera_getWindowSize_const(self.as_raw_Camera(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getFov() /usr/include/opencv2/viz/types.hpp:221
	#[inline]
	fn get_fov(&self) -> Result<core::Vec2d> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Camera_getFov_const(self.as_raw_Camera(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getPrincipalPoint() /usr/include/opencv2/viz/types.hpp:224
	#[inline]
	fn get_principal_point(&self) -> Result<core::Vec2d> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Camera_getPrincipalPoint_const(self.as_raw_Camera(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getFocalLength() /usr/include/opencv2/viz/types.hpp:225
	#[inline]
	fn get_focal_length(&self) -> Result<core::Vec2d> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Camera_getFocalLength_const(self.as_raw_Camera(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// computeProjectionMatrix(cv::Matx44d &) /usr/include/opencv2/viz/types.hpp:240
	#[inline]
	fn compute_projection_matrix(&self, proj: &mut core::Matx44d) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Camera_computeProjectionMatrix_const_Matx44dR(self.as_raw_Camera(), proj, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait CameraTrait: crate::viz::CameraTraitConst {
	fn as_raw_mut_Camera(&mut self) -> *mut c_void;

	// setClip(const cv::Vec2d &) /usr/include/opencv2/viz/types.hpp:216
	#[inline]
	fn set_clip(&mut self, clip: core::Vec2d) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Camera_setClip_const_Vec2dR(self.as_raw_mut_Camera(), &clip, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setWindowSize(const cv::Size &) /usr/include/opencv2/viz/types.hpp:219
	#[inline]
	fn set_window_size(&mut self, window_size: core::Size) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Camera_setWindowSize_const_SizeR(self.as_raw_mut_Camera(), &window_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setFov(const cv::Vec2d &) /usr/include/opencv2/viz/types.hpp:222
	#[inline]
	fn set_fov(&mut self, fov: core::Vec2d) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Camera_setFov_const_Vec2dR(self.as_raw_mut_Camera(), &fov, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Camera /usr/include/opencv2/viz/types.hpp:163
pub struct Camera {
	ptr: *mut c_void
}

opencv_type_boxed! { Camera }

impl Drop for Camera {
	fn drop(&mut self) {
		extern "C" { fn cv_Camera_delete(instance: *mut c_void); }
		unsafe { cv_Camera_delete(self.as_raw_mut_Camera()) };
	}
}

unsafe impl Send for Camera {}

impl crate::viz::CameraTraitConst for Camera {
	#[inline] fn as_raw_Camera(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::CameraTrait for Camera {
	#[inline] fn as_raw_mut_Camera(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Camera {
	// Camera(double, double, double, double, const cv::Size &) /usr/include/opencv2/viz/types.hpp:176
	#[inline]
	pub fn new(fx: f64, fy: f64, cx: f64, cy: f64, window_size: core::Size) -> Result<crate::viz::Camera> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Camera_Camera_double_double_double_double_const_SizeR(fx, fy, cx, cy, &window_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Camera::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Camera(const cv::Vec2d &, const cv::Size &) /usr/include/opencv2/viz/types.hpp:183
	#[inline]
	pub fn new_1(fov: core::Vec2d, window_size: core::Size) -> Result<crate::viz::Camera> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Camera_Camera_const_Vec2dR_const_SizeR(&fov, &window_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Camera::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Camera(const cv::Matx33d &, const cv::Size &) /usr/include/opencv2/viz/types.hpp:197
	#[inline]
	pub fn new_2(k: core::Matx33d, window_size: core::Size) -> Result<crate::viz::Camera> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Camera_Camera_const_Matx33dR_const_SizeR(&k, &window_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Camera::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Camera(const cv::Matx44d &, const cv::Size &) /usr/include/opencv2/viz/types.hpp:213
	#[inline]
	pub fn new_3(proj: core::Matx44d, window_size: core::Size) -> Result<crate::viz::Camera> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Camera_Camera_const_Matx44dR_const_SizeR(&proj, &window_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Camera::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// KinectCamera(const cv::Size &) /usr/include/opencv2/viz/types.hpp:250
	#[inline]
	pub fn kinect_camera(window_size: core::Size) -> Result<crate::viz::Camera> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Camera_KinectCamera_const_SizeR(&window_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Camera::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// Color /usr/include/opencv2/viz/types.hpp:63
pub trait ColorTraitConst {
	fn as_raw_Color(&self) -> *const c_void;

	// operator Vec() /usr/include/opencv2/viz/types.hpp:73
	#[inline]
	fn to_vec3b(&self) -> Result<core::Vec3b> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_operator_cv_Vec3b_const(self.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait ColorTrait: crate::viz::ColorTraitConst {
	fn as_raw_mut_Color(&mut self) -> *mut c_void;

}

// Color /usr/include/opencv2/viz/types.hpp:63
pub struct Color {
	ptr: *mut c_void
}

opencv_type_boxed! { Color }

impl Drop for Color {
	fn drop(&mut self) {
		extern "C" { fn cv_Color_delete(instance: *mut c_void); }
		unsafe { cv_Color_delete(self.as_raw_mut_Color()) };
	}
}

unsafe impl Send for Color {}

impl crate::viz::ColorTraitConst for Color {
	#[inline] fn as_raw_Color(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::ColorTrait for Color {
	#[inline] fn as_raw_mut_Color(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Color {
	// Color() /usr/include/opencv2/viz/types.hpp:66
	#[inline]
	pub fn default() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_Color(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Color(double) /usr/include/opencv2/viz/types.hpp:68
	#[inline]
	pub fn new(gray: f64) -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_Color_double(gray, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Color(double, double, double) /usr/include/opencv2/viz/types.hpp:69
	#[inline]
	pub fn new_1(blue: f64, green: f64, red: f64) -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_Color_double_double_double(blue, green, red, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Color(const cv::Scalar &) /usr/include/opencv2/viz/types.hpp:71
	#[inline]
	pub fn new_2(color: core::Scalar) -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_Color_const_ScalarR(&color, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// black() /usr/include/opencv2/viz/types.hpp:75
	#[inline]
	pub fn black() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_black(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// blue() /usr/include/opencv2/viz/types.hpp:76
	#[inline]
	pub fn blue() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_blue(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// green() /usr/include/opencv2/viz/types.hpp:77
	#[inline]
	pub fn green() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_green(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// red() /usr/include/opencv2/viz/types.hpp:78
	#[inline]
	pub fn red() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_red(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// cyan() /usr/include/opencv2/viz/types.hpp:79
	#[inline]
	pub fn cyan() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_cyan(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// yellow() /usr/include/opencv2/viz/types.hpp:80
	#[inline]
	pub fn yellow() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_yellow(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// magenta() /usr/include/opencv2/viz/types.hpp:81
	#[inline]
	pub fn magenta() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_magenta(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// white() /usr/include/opencv2/viz/types.hpp:82
	#[inline]
	pub fn white() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_white(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// gray() /usr/include/opencv2/viz/types.hpp:84
	#[inline]
	pub fn gray() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_gray(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// silver() /usr/include/opencv2/viz/types.hpp:85
	#[inline]
	pub fn silver() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_silver(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// mlab() /usr/include/opencv2/viz/types.hpp:87
	#[inline]
	pub fn mlab() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_mlab(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// navy() /usr/include/opencv2/viz/types.hpp:89
	#[inline]
	pub fn navy() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_navy(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// maroon() /usr/include/opencv2/viz/types.hpp:90
	#[inline]
	pub fn maroon() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_maroon(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// teal() /usr/include/opencv2/viz/types.hpp:91
	#[inline]
	pub fn teal() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_teal(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// olive() /usr/include/opencv2/viz/types.hpp:92
	#[inline]
	pub fn olive() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_olive(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// purple() /usr/include/opencv2/viz/types.hpp:93
	#[inline]
	pub fn purple() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_purple(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// azure() /usr/include/opencv2/viz/types.hpp:94
	#[inline]
	pub fn azure() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_azure(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// chartreuse() /usr/include/opencv2/viz/types.hpp:95
	#[inline]
	pub fn chartreuse() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_chartreuse(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// rose() /usr/include/opencv2/viz/types.hpp:96
	#[inline]
	pub fn rose() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_rose(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// lime() /usr/include/opencv2/viz/types.hpp:98
	#[inline]
	pub fn lime() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_lime(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// gold() /usr/include/opencv2/viz/types.hpp:99
	#[inline]
	pub fn gold() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_gold(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// orange() /usr/include/opencv2/viz/types.hpp:100
	#[inline]
	pub fn orange() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_orange(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// orange_red() /usr/include/opencv2/viz/types.hpp:101
	#[inline]
	pub fn orange_red() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_orange_red(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// indigo() /usr/include/opencv2/viz/types.hpp:102
	#[inline]
	pub fn indigo() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_indigo(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// brown() /usr/include/opencv2/viz/types.hpp:104
	#[inline]
	pub fn brown() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_brown(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// apricot() /usr/include/opencv2/viz/types.hpp:105
	#[inline]
	pub fn apricot() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_apricot(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// pink() /usr/include/opencv2/viz/types.hpp:106
	#[inline]
	pub fn pink() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_pink(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// raspberry() /usr/include/opencv2/viz/types.hpp:107
	#[inline]
	pub fn raspberry() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_raspberry(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// cherry() /usr/include/opencv2/viz/types.hpp:108
	#[inline]
	pub fn cherry() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_cherry(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// violet() /usr/include/opencv2/viz/types.hpp:109
	#[inline]
	pub fn violet() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_violet(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// amethyst() /usr/include/opencv2/viz/types.hpp:110
	#[inline]
	pub fn amethyst() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_amethyst(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// bluberry() /usr/include/opencv2/viz/types.hpp:111
	#[inline]
	pub fn bluberry() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_bluberry(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// celestial_blue() /usr/include/opencv2/viz/types.hpp:112
	#[inline]
	pub fn celestial_blue() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_celestial_blue(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// turquoise() /usr/include/opencv2/viz/types.hpp:113
	#[inline]
	pub fn turquoise() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_turquoise(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// not_set() /usr/include/opencv2/viz/types.hpp:115
	#[inline]
	pub fn not_set() -> Result<crate::viz::Color> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Color_not_set(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Color::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// KeyboardEvent /usr/include/opencv2/viz/types.hpp:287
pub trait KeyboardEventTraitConst {
	fn as_raw_KeyboardEvent(&self) -> *const c_void;

	// action /usr/include/opencv2/viz/types.hpp:302
	#[inline]
	fn action(&self) -> crate::viz::KeyboardEvent_Action {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_KeyboardEvent_getPropAction_const(self.as_raw_KeyboardEvent(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	// symbol /usr/include/opencv2/viz/types.hpp:303
	#[inline]
	fn symbol(&self) -> String {
		let ret = unsafe { sys::cv_viz_KeyboardEvent_getPropSymbol_const(self.as_raw_KeyboardEvent()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	// code /usr/include/opencv2/viz/types.hpp:304
	#[inline]
	fn code(&self) -> u8 {
		let ret = unsafe { sys::cv_viz_KeyboardEvent_getPropCode_const(self.as_raw_KeyboardEvent()) };
		ret
	}
	
	// modifiers /usr/include/opencv2/viz/types.hpp:305
	#[inline]
	fn modifiers(&self) -> i32 {
		let ret = unsafe { sys::cv_viz_KeyboardEvent_getPropModifiers_const(self.as_raw_KeyboardEvent()) };
		ret
	}
	
}

pub trait KeyboardEventTrait: crate::viz::KeyboardEventTraitConst {
	fn as_raw_mut_KeyboardEvent(&mut self) -> *mut c_void;

	// action /usr/include/opencv2/viz/types.hpp:302
	#[inline]
	fn set_action(&mut self, val: crate::viz::KeyboardEvent_Action) {
		let ret = unsafe { sys::cv_viz_KeyboardEvent_setPropAction_Action(self.as_raw_mut_KeyboardEvent(), val) };
		ret
	}
	
	// symbol /usr/include/opencv2/viz/types.hpp:303
	#[inline]
	fn set_symbol(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_viz_KeyboardEvent_setPropSymbol_String(self.as_raw_mut_KeyboardEvent(), val.opencv_as_extern_mut()) };
		ret
	}
	
	// code /usr/include/opencv2/viz/types.hpp:304
	#[inline]
	fn set_code(&mut self, val: u8) {
		let ret = unsafe { sys::cv_viz_KeyboardEvent_setPropCode_unsigned_char(self.as_raw_mut_KeyboardEvent(), val) };
		ret
	}
	
	// modifiers /usr/include/opencv2/viz/types.hpp:305
	#[inline]
	fn set_modifiers(&mut self, val: i32) {
		let ret = unsafe { sys::cv_viz_KeyboardEvent_setPropModifiers_int(self.as_raw_mut_KeyboardEvent(), val) };
		ret
	}
	
}

// KeyboardEvent /usr/include/opencv2/viz/types.hpp:287
pub struct KeyboardEvent {
	ptr: *mut c_void
}

opencv_type_boxed! { KeyboardEvent }

impl Drop for KeyboardEvent {
	fn drop(&mut self) {
		extern "C" { fn cv_KeyboardEvent_delete(instance: *mut c_void); }
		unsafe { cv_KeyboardEvent_delete(self.as_raw_mut_KeyboardEvent()) };
	}
}

unsafe impl Send for KeyboardEvent {}

impl crate::viz::KeyboardEventTraitConst for KeyboardEvent {
	#[inline] fn as_raw_KeyboardEvent(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::KeyboardEventTrait for KeyboardEvent {
	#[inline] fn as_raw_mut_KeyboardEvent(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl KeyboardEvent {
	// KeyboardEvent(cv::viz::KeyboardEvent::Action, const cv::String &, unsigned char, int) /usr/include/opencv2/viz/types.hpp:300
	#[inline]
	pub fn new(action: crate::viz::KeyboardEvent_Action, symbol: &str, code: u8, modifiers: i32) -> Result<crate::viz::KeyboardEvent> {
		extern_container_arg!(symbol);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_KeyboardEvent_KeyboardEvent_Action_const_StringR_unsigned_char_int(action, symbol.opencv_as_extern(), code, modifiers, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::KeyboardEvent::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// Mesh /usr/include/opencv2/viz/types.hpp:120
pub trait MeshTraitConst {
	fn as_raw_Mesh(&self) -> *const c_void;

	// cloud /usr/include/opencv2/viz/types.hpp:129
	#[inline]
	fn cloud(&self) -> core::Mat {
		let ret = unsafe { sys::cv_viz_Mesh_getPropCloud_const(self.as_raw_Mesh()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	// colors /usr/include/opencv2/viz/types.hpp:130
	#[inline]
	fn colors(&self) -> core::Mat {
		let ret = unsafe { sys::cv_viz_Mesh_getPropColors_const(self.as_raw_Mesh()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	// normals /usr/include/opencv2/viz/types.hpp:131
	#[inline]
	fn normals(&self) -> core::Mat {
		let ret = unsafe { sys::cv_viz_Mesh_getPropNormals_const(self.as_raw_Mesh()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	// polygons /usr/include/opencv2/viz/types.hpp:135
	#[inline]
	fn polygons(&self) -> core::Mat {
		let ret = unsafe { sys::cv_viz_Mesh_getPropPolygons_const(self.as_raw_Mesh()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	// texture /usr/include/opencv2/viz/types.hpp:137
	#[inline]
	fn texture(&self) -> core::Mat {
		let ret = unsafe { sys::cv_viz_Mesh_getPropTexture_const(self.as_raw_Mesh()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	// tcoords /usr/include/opencv2/viz/types.hpp:138
	#[inline]
	fn tcoords(&self) -> core::Mat {
		let ret = unsafe { sys::cv_viz_Mesh_getPropTcoords_const(self.as_raw_Mesh()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
}

pub trait MeshTrait: crate::viz::MeshTraitConst {
	fn as_raw_mut_Mesh(&mut self) -> *mut c_void;

	// cloud /usr/include/opencv2/viz/types.hpp:129
	#[inline]
	fn set_cloud(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_viz_Mesh_setPropCloud_Mat(self.as_raw_mut_Mesh(), val.as_raw_mut_Mat()) };
		ret
	}
	
	// colors /usr/include/opencv2/viz/types.hpp:130
	#[inline]
	fn set_colors(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_viz_Mesh_setPropColors_Mat(self.as_raw_mut_Mesh(), val.as_raw_mut_Mat()) };
		ret
	}
	
	// normals /usr/include/opencv2/viz/types.hpp:131
	#[inline]
	fn set_normals(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_viz_Mesh_setPropNormals_Mat(self.as_raw_mut_Mesh(), val.as_raw_mut_Mat()) };
		ret
	}
	
	// polygons /usr/include/opencv2/viz/types.hpp:135
	#[inline]
	fn set_polygons(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_viz_Mesh_setPropPolygons_Mat(self.as_raw_mut_Mesh(), val.as_raw_mut_Mat()) };
		ret
	}
	
	// texture /usr/include/opencv2/viz/types.hpp:137
	#[inline]
	fn set_texture(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_viz_Mesh_setPropTexture_Mat(self.as_raw_mut_Mesh(), val.as_raw_mut_Mat()) };
		ret
	}
	
	// tcoords /usr/include/opencv2/viz/types.hpp:138
	#[inline]
	fn set_tcoords(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_viz_Mesh_setPropTcoords_Mat(self.as_raw_mut_Mesh(), val.as_raw_mut_Mat()) };
		ret
	}
	
}

// Mesh /usr/include/opencv2/viz/types.hpp:120
pub struct Mesh {
	ptr: *mut c_void
}

opencv_type_boxed! { Mesh }

impl Drop for Mesh {
	fn drop(&mut self) {
		extern "C" { fn cv_Mesh_delete(instance: *mut c_void); }
		unsafe { cv_Mesh_delete(self.as_raw_mut_Mesh()) };
	}
}

unsafe impl Send for Mesh {}

impl crate::viz::MeshTraitConst for Mesh {
	#[inline] fn as_raw_Mesh(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::MeshTrait for Mesh {
	#[inline] fn as_raw_mut_Mesh(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Mesh {
	// Mesh() /usr/include/opencv2/viz/types.hpp:140
	#[inline]
	pub fn default() -> Result<crate::viz::Mesh> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Mesh_Mesh(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Mesh::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * typ: LOAD_PLY
	// load(const cv::String &, int) /usr/include/opencv2/viz/types.hpp:154
	#[inline]
	pub fn load(file: &str, typ: i32) -> Result<crate::viz::Mesh> {
		extern_container_arg!(file);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Mesh_load_const_StringR_int(file.opencv_as_extern(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Mesh::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// MouseEvent /usr/include/opencv2/viz/types.hpp:310
pub trait MouseEventTraitConst {
	fn as_raw_MouseEvent(&self) -> *const c_void;

	// type /usr/include/opencv2/viz/types.hpp:327
	#[inline]
	fn typ(&self) -> crate::viz::MouseEvent_Type {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_MouseEvent_getPropType_const(self.as_raw_MouseEvent(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	// button /usr/include/opencv2/viz/types.hpp:328
	#[inline]
	fn button(&self) -> crate::viz::MouseEvent_MouseButton {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_MouseEvent_getPropButton_const(self.as_raw_MouseEvent(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	// pointer /usr/include/opencv2/viz/types.hpp:329
	#[inline]
	fn pointer(&self) -> core::Point {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_MouseEvent_getPropPointer_const(self.as_raw_MouseEvent(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	// modifiers /usr/include/opencv2/viz/types.hpp:330
	#[inline]
	fn modifiers(&self) -> i32 {
		let ret = unsafe { sys::cv_viz_MouseEvent_getPropModifiers_const(self.as_raw_MouseEvent()) };
		ret
	}
	
}

pub trait MouseEventTrait: crate::viz::MouseEventTraitConst {
	fn as_raw_mut_MouseEvent(&mut self) -> *mut c_void;

	// type /usr/include/opencv2/viz/types.hpp:327
	#[inline]
	fn set_type(&mut self, val: crate::viz::MouseEvent_Type) {
		let ret = unsafe { sys::cv_viz_MouseEvent_setPropType_Type(self.as_raw_mut_MouseEvent(), val) };
		ret
	}
	
	// button /usr/include/opencv2/viz/types.hpp:328
	#[inline]
	fn set_button(&mut self, val: crate::viz::MouseEvent_MouseButton) {
		let ret = unsafe { sys::cv_viz_MouseEvent_setPropButton_MouseButton(self.as_raw_mut_MouseEvent(), val) };
		ret
	}
	
	// pointer /usr/include/opencv2/viz/types.hpp:329
	#[inline]
	fn set_pointer(&mut self, val: core::Point) {
		let ret = unsafe { sys::cv_viz_MouseEvent_setPropPointer_Point(self.as_raw_mut_MouseEvent(), val.opencv_as_extern()) };
		ret
	}
	
	// modifiers /usr/include/opencv2/viz/types.hpp:330
	#[inline]
	fn set_modifiers(&mut self, val: i32) {
		let ret = unsafe { sys::cv_viz_MouseEvent_setPropModifiers_int(self.as_raw_mut_MouseEvent(), val) };
		ret
	}
	
}

// MouseEvent /usr/include/opencv2/viz/types.hpp:310
pub struct MouseEvent {
	ptr: *mut c_void
}

opencv_type_boxed! { MouseEvent }

impl Drop for MouseEvent {
	fn drop(&mut self) {
		extern "C" { fn cv_MouseEvent_delete(instance: *mut c_void); }
		unsafe { cv_MouseEvent_delete(self.as_raw_mut_MouseEvent()) };
	}
}

unsafe impl Send for MouseEvent {}

impl crate::viz::MouseEventTraitConst for MouseEvent {
	#[inline] fn as_raw_MouseEvent(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::MouseEventTrait for MouseEvent {
	#[inline] fn as_raw_mut_MouseEvent(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MouseEvent {
	// MouseEvent(const cv::viz::MouseEvent::Type &, const cv::viz::MouseEvent::MouseButton &, const cv::Point &, int) /usr/include/opencv2/viz/types.hpp:325
	#[inline]
	pub fn new(typ: crate::viz::MouseEvent_Type, button: crate::viz::MouseEvent_MouseButton, pointer: core::Point, modifiers: i32) -> Result<crate::viz::MouseEvent> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_MouseEvent_MouseEvent_const_TypeR_const_MouseButtonR_const_PointR_int(&typ, &button, &pointer, modifiers, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::MouseEvent::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// Viz3d /usr/include/opencv2/viz/viz3d.hpp:67
pub trait Viz3dTraitConst {
	fn as_raw_Viz3d(&self) -> *const c_void;

	// getWidget(const cv::String &) /usr/include/opencv2/viz/viz3d.hpp:103
	#[inline]
	fn get_widget(&self, id: &str) -> Result<crate::viz::Widget> {
		extern_container_arg!(id);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_getWidget_const_const_StringR(self.as_raw_Viz3d(), id.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Widget::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getWidgetPose(const cv::String &) /usr/include/opencv2/viz/viz3d.hpp:133
	#[inline]
	fn get_widget_pose(&self, id: &str) -> Result<core::Affine3d> {
		extern_container_arg!(id);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_getWidgetPose_const_const_StringR(self.as_raw_Viz3d(), id.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getCamera() /usr/include/opencv2/viz/viz3d.hpp:143
	#[inline]
	fn get_camera(&self) -> Result<crate::viz::Camera> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_getCamera_const(self.as_raw_Viz3d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Camera::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getViewerPose() /usr/include/opencv2/viz/viz3d.hpp:147
	#[inline]
	fn get_viewer_pose(&self) -> Result<core::Affine3d> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_getViewerPose_const(self.as_raw_Viz3d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getWindowSize() /usr/include/opencv2/viz/viz3d.hpp:181
	#[inline]
	fn get_window_size(&self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_getWindowSize_const(self.as_raw_Viz3d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getWindowName() /usr/include/opencv2/viz/viz3d.hpp:191
	#[inline]
	fn get_window_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_getWindowName_const(self.as_raw_Viz3d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getScreenshot() /usr/include/opencv2/viz/viz3d.hpp:195
	#[inline]
	fn get_screenshot(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_getScreenshot_const(self.as_raw_Viz3d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// wasStopped() /usr/include/opencv2/viz/viz3d.hpp:255
	#[inline]
	fn was_stopped(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_wasStopped_const(self.as_raw_Viz3d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait Viz3dTrait: crate::viz::Viz3dTraitConst {
	fn as_raw_mut_Viz3d(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * pose: Affine3d::Identity()
	// showWidget(const cv::String &, const cv::viz::Widget &, const cv::Affine3d &) /usr/include/opencv2/viz/viz3d.hpp:88
	#[inline]
	fn show_widget(&mut self, id: &str, widget: &crate::viz::Widget, pose: core::Affine3d) -> Result<()> {
		extern_container_arg!(id);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_showWidget_const_StringR_const_WidgetR_const_Affine3dR(self.as_raw_mut_Viz3d(), id.opencv_as_extern(), widget.as_raw_Widget(), &pose, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// removeWidget(const cv::String &) /usr/include/opencv2/viz/viz3d.hpp:94
	#[inline]
	fn remove_widget(&mut self, id: &str) -> Result<()> {
		extern_container_arg!(id);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_removeWidget_const_StringR(self.as_raw_mut_Viz3d(), id.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// removeAllWidgets() /usr/include/opencv2/viz/viz3d.hpp:107
	#[inline]
	fn remove_all_widgets(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_removeAllWidgets(self.as_raw_mut_Viz3d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * window_size: Size(-1,-1)
	// showImage(cv::InputArray, const cv::Size &) /usr/include/opencv2/viz/viz3d.hpp:114
	#[inline]
	fn show_image(&mut self, image: &dyn core::ToInputArray, window_size: core::Size) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_showImage_const__InputArrayR_const_SizeR(self.as_raw_mut_Viz3d(), image.as_raw__InputArray(), &window_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setWidgetPose(const cv::String &, const cv::Affine3d &) /usr/include/opencv2/viz/viz3d.hpp:120
	#[inline]
	fn set_widget_pose(&mut self, id: &str, pose: core::Affine3d) -> Result<()> {
		extern_container_arg!(id);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_setWidgetPose_const_StringR_const_Affine3dR(self.as_raw_mut_Viz3d(), id.opencv_as_extern(), &pose, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// updateWidgetPose(const cv::String &, const cv::Affine3d &) /usr/include/opencv2/viz/viz3d.hpp:127
	#[inline]
	fn update_widget_pose(&mut self, id: &str, pose: core::Affine3d) -> Result<()> {
		extern_container_arg!(id);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_updateWidgetPose_const_StringR_const_Affine3dR(self.as_raw_mut_Viz3d(), id.opencv_as_extern(), &pose, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setCamera(const cv::viz::Camera &) /usr/include/opencv2/viz/viz3d.hpp:139
	#[inline]
	fn set_camera(&mut self, camera: &crate::viz::Camera) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_setCamera_const_CameraR(self.as_raw_mut_Viz3d(), camera.as_raw_Camera(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setViewerPose(const cv::Affine3d &) /usr/include/opencv2/viz/viz3d.hpp:153
	#[inline]
	fn set_viewer_pose(&mut self, pose: core::Affine3d) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_setViewerPose_const_Affine3dR(self.as_raw_mut_Viz3d(), &pose, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// resetCameraViewpoint(const cv::String &) /usr/include/opencv2/viz/viz3d.hpp:159
	#[inline]
	fn reset_camera_viewpoint(&mut self, id: &str) -> Result<()> {
		extern_container_arg!(id);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_resetCameraViewpoint_const_StringR(self.as_raw_mut_Viz3d(), id.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// resetCamera() /usr/include/opencv2/viz/viz3d.hpp:163
	#[inline]
	fn reset_camera(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_resetCamera(self.as_raw_mut_Viz3d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// convertToWindowCoordinates(const cv::Point3d &, cv::Point3d &) /usr/include/opencv2/viz/viz3d.hpp:170
	#[inline]
	fn convert_to_window_coordinates(&mut self, pt: core::Point3d, window_coord: &mut core::Point3d) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_convertToWindowCoordinates_const_Point3dR_Point3dR(self.as_raw_mut_Viz3d(), &pt, window_coord, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// converTo3DRay(const cv::Point3d &, cv::Point3d &, cv::Vec3d &) /usr/include/opencv2/viz/viz3d.hpp:177
	#[inline]
	fn conver_to3_d_ray(&mut self, window_coord: core::Point3d, origin: &mut core::Point3d, direction: &mut core::Vec3d) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_converTo3DRay_const_Point3dR_Point3dR_Vec3dR(self.as_raw_mut_Viz3d(), &window_coord, origin, direction, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setWindowSize(const cv::Size &) /usr/include/opencv2/viz/viz3d.hpp:186
	#[inline]
	fn set_window_size(&mut self, window_size: core::Size) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_setWindowSize_const_SizeR(self.as_raw_mut_Viz3d(), &window_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// saveScreenshot(const cv::String &) /usr/include/opencv2/viz/viz3d.hpp:201
	#[inline]
	fn save_screenshot(&mut self, file: &str) -> Result<()> {
		extern_container_arg!(file);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_saveScreenshot_const_StringR(self.as_raw_mut_Viz3d(), file.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setWindowPosition(const cv::Point &) /usr/include/opencv2/viz/viz3d.hpp:207
	#[inline]
	fn set_window_position(&mut self, window_position: core::Point) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_setWindowPosition_const_PointR(self.as_raw_mut_Viz3d(), &window_position, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * mode: true
	// setFullScreen(bool) /usr/include/opencv2/viz/viz3d.hpp:213
	#[inline]
	fn set_full_screen(&mut self, mode: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_setFullScreen_bool(self.as_raw_mut_Viz3d(), mode, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * color: Color::black()
	/// * color2: Color::not_set()
	// setBackgroundColor(const cv::viz::Viz3d::Color &, const cv::viz::Viz3d::Color &) /usr/include/opencv2/viz/viz3d.hpp:217
	#[inline]
	fn set_background_color(&mut self, color: &crate::viz::Viz3d_Color, color2: &crate::viz::Viz3d_Color) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_setBackgroundColor_const_ColorR_const_ColorR(self.as_raw_mut_Viz3d(), color.as_raw_Color(), color2.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * image: noArray()
	// setBackgroundTexture(cv::InputArray) /usr/include/opencv2/viz/viz3d.hpp:218
	#[inline]
	fn set_background_texture(&mut self, image: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_setBackgroundTexture_const__InputArrayR(self.as_raw_mut_Viz3d(), image.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setBackgroundMeshLab() /usr/include/opencv2/viz/viz3d.hpp:219
	#[inline]
	fn set_background_mesh_lab(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_setBackgroundMeshLab(self.as_raw_mut_Viz3d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// spin() /usr/include/opencv2/viz/viz3d.hpp:223
	#[inline]
	fn spin(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_spin(self.as_raw_mut_Viz3d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * time: 1
	/// * force_redraw: false
	// spinOnce(int, bool) /usr/include/opencv2/viz/viz3d.hpp:230
	#[inline]
	fn spin_once(&mut self, time: i32, force_redraw: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_spinOnce_int_bool(self.as_raw_mut_Viz3d(), time, force_redraw, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setOffScreenRendering() /usr/include/opencv2/viz/viz3d.hpp:234
	#[inline]
	fn set_off_screen_rendering(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_setOffScreenRendering(self.as_raw_mut_Viz3d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// removeAllLights() /usr/include/opencv2/viz/viz3d.hpp:238
	#[inline]
	fn remove_all_lights(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_removeAllLights(self.as_raw_mut_Viz3d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * focal_point: Vec3d(0,0,0)
	/// * color: Color::white()
	/// * diffuse_color: Color::white()
	/// * ambient_color: Color::black()
	/// * specular_color: Color::white()
	// addLight(const cv::Vec3d &, const cv::Vec3d &, const cv::viz::Viz3d::Color &, const cv::viz::Viz3d::Color &, const cv::viz::Viz3d::Color &, const cv::viz::Viz3d::Color &) /usr/include/opencv2/viz/viz3d.hpp:249
	#[inline]
	fn add_light(&mut self, position: core::Vec3d, focal_point: core::Vec3d, color: &crate::viz::Viz3d_Color, diffuse_color: &crate::viz::Viz3d_Color, ambient_color: &crate::viz::Viz3d_Color, specular_color: &crate::viz::Viz3d_Color) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_addLight_const_Vec3dR_const_Vec3dR_const_ColorR_const_ColorR_const_ColorR_const_ColorR(self.as_raw_mut_Viz3d(), &position, &focal_point, color.as_raw_Color(), diffuse_color.as_raw_Color(), ambient_color.as_raw_Color(), specular_color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// close() /usr/include/opencv2/viz/viz3d.hpp:256
	#[inline]
	fn close(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_close(self.as_raw_mut_Viz3d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * cookie: 0
	// registerKeyboardCallback(cv::viz::Viz3d::KeyboardCallback, void *) /usr/include/opencv2/viz/viz3d.hpp:264
	#[inline]
	fn register_keyboard_callback(&mut self, callback: crate::viz::Viz3d_KeyboardCallback) -> Result<()> {
		callback_arg!(callback_trampoline(unnamed: *const c_void, unnamed_1: *mut c_void) -> () => unnamed_1 in callbacks => callback(unnamed: *const c_void) -> ());
		userdata_arg!(cookie in callbacks => callback);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_registerKeyboardCallback_KeyboardCallback_voidX(self.as_raw_mut_Viz3d(), callback_trampoline, cookie, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * cookie: 0
	// registerMouseCallback(cv::viz::Viz3d::MouseCallback, void *) /usr/include/opencv2/viz/viz3d.hpp:271
	#[inline]
	fn register_mouse_callback(&mut self, callback: crate::viz::Viz3d_MouseCallback) -> Result<()> {
		callback_arg!(callback_trampoline(unnamed: *const c_void, unnamed_1: *mut c_void) -> () => unnamed_1 in callbacks => callback(unnamed: *const c_void) -> ());
		userdata_arg!(cookie in callbacks => callback);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_registerMouseCallback_MouseCallback_voidX(self.as_raw_mut_Viz3d(), callback_trampoline, cookie, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setRenderingProperty(const cv::String &, int, double) /usr/include/opencv2/viz/viz3d.hpp:299
	#[inline]
	fn set_rendering_property(&mut self, id: &str, property: i32, value: f64) -> Result<()> {
		extern_container_arg!(id);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_setRenderingProperty_const_StringR_int_double(self.as_raw_mut_Viz3d(), id.opencv_as_extern(), property, value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getRenderingProperty(const cv::String &, int) /usr/include/opencv2/viz/viz3d.hpp:325
	#[inline]
	fn get_rendering_property(&mut self, id: &str, property: i32) -> Result<f64> {
		extern_container_arg!(id);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_getRenderingProperty_const_StringR_int(self.as_raw_mut_Viz3d(), id.opencv_as_extern(), property, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setRepresentation(int) /usr/include/opencv2/viz/viz3d.hpp:334
	#[inline]
	fn set_representation(&mut self, representation: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_setRepresentation_int(self.as_raw_mut_Viz3d(), representation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * enabled: false
	// setGlobalWarnings(bool) /usr/include/opencv2/viz/viz3d.hpp:336
	#[inline]
	fn set_global_warnings(&mut self, enabled: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_setGlobalWarnings_bool(self.as_raw_mut_Viz3d(), enabled, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Viz3d /usr/include/opencv2/viz/viz3d.hpp:67
pub struct Viz3d {
	ptr: *mut c_void
}

opencv_type_boxed! { Viz3d }

impl Drop for Viz3d {
	fn drop(&mut self) {
		extern "C" { fn cv_Viz3d_delete(instance: *mut c_void); }
		unsafe { cv_Viz3d_delete(self.as_raw_mut_Viz3d()) };
	}
}

unsafe impl Send for Viz3d {}

impl crate::viz::Viz3dTraitConst for Viz3d {
	#[inline] fn as_raw_Viz3d(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Viz3dTrait for Viz3d {
	#[inline] fn as_raw_mut_Viz3d(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Viz3d {
	/// ## C++ default parameters
	/// * window_name: String()
	// Viz3d(const cv::String &) /usr/include/opencv2/viz/viz3d.hpp:78
	#[inline]
	pub fn new(window_name: &str) -> Result<crate::viz::Viz3d> {
		extern_container_arg!(window_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_Viz3d_const_StringR(window_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Viz3d::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Viz3d(const cv::viz::Viz3d &) /usr/include/opencv2/viz/viz3d.hpp:79
	#[inline]
	pub fn copy(unnamed: &crate::viz::Viz3d) -> Result<crate::viz::Viz3d> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Viz3d_Viz3d_const_Viz3dR(unnamed.as_raw_Viz3d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Viz3d::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// WArrow /usr/include/opencv2/viz/widgets.hpp:288
pub trait WArrowTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WArrow(&self) -> *const c_void;

}

pub trait WArrowTrait: crate::viz::WArrowTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WArrow(&mut self) -> *mut c_void;

}

// WArrow /usr/include/opencv2/viz/widgets.hpp:288
pub struct WArrow {
	ptr: *mut c_void
}

opencv_type_boxed! { WArrow }

impl Drop for WArrow {
	fn drop(&mut self) {
		extern "C" { fn cv_WArrow_delete(instance: *mut c_void); }
		unsafe { cv_WArrow_delete(self.as_raw_mut_WArrow()) };
	}
}

unsafe impl Send for WArrow {}

impl crate::viz::WidgetTraitConst for WArrow {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WArrow {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WArrow {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WArrow {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WArrowTraitConst for WArrow {
	#[inline] fn as_raw_WArrow(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WArrowTrait for WArrow {
	#[inline] fn as_raw_mut_WArrow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WArrow {
	/// ## C++ default parameters
	/// * thickness: 0.03
	/// * color: Color::white()
	// WArrow(const cv::Point3d &, const cv::Point3d &, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:301
	#[inline]
	pub fn new(pt1: core::Point3d, pt2: core::Point3d, thickness: f64, color: &crate::viz::Color) -> Result<crate::viz::WArrow> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WArrow_WArrow_const_Point3dR_const_Point3dR_double_const_ColorR(&pt1, &pt2, thickness, color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WArrow::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { WArrow, crate::viz::Widget, cv_WArrow_to_Widget }

boxed_cast_base! { WArrow, crate::viz::Widget3D, cv_WArrow_to_Widget3D }

// WCameraPosition /usr/include/opencv2/viz/widgets.hpp:543
pub trait WCameraPositionTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WCameraPosition(&self) -> *const c_void;

}

pub trait WCameraPositionTrait: crate::viz::WCameraPositionTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WCameraPosition(&mut self) -> *mut c_void;

}

// WCameraPosition /usr/include/opencv2/viz/widgets.hpp:543
pub struct WCameraPosition {
	ptr: *mut c_void
}

opencv_type_boxed! { WCameraPosition }

impl Drop for WCameraPosition {
	fn drop(&mut self) {
		extern "C" { fn cv_WCameraPosition_delete(instance: *mut c_void); }
		unsafe { cv_WCameraPosition_delete(self.as_raw_mut_WCameraPosition()) };
	}
}

unsafe impl Send for WCameraPosition {}

impl crate::viz::WidgetTraitConst for WCameraPosition {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WCameraPosition {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WCameraPosition {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WCameraPosition {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WCameraPositionTraitConst for WCameraPosition {
	#[inline] fn as_raw_WCameraPosition(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WCameraPositionTrait for WCameraPosition {
	#[inline] fn as_raw_mut_WCameraPosition(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WCameraPosition {
	/// ## C++ default parameters
	/// * scale: 1.0
	// WCameraPosition(double) /usr/include/opencv2/viz/widgets.hpp:550
	#[inline]
	pub fn new(scale: f64) -> Result<crate::viz::WCameraPosition> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WCameraPosition_WCameraPosition_double(scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WCameraPosition::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * scale: 1.0
	/// * color: Color::white()
	// WCameraPosition(const cv::Matx33d &, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:560
	#[inline]
	pub fn new_1(k: core::Matx33d, scale: f64, color: &crate::viz::Color) -> Result<crate::viz::WCameraPosition> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WCameraPosition_WCameraPosition_const_Matx33dR_double_const_ColorR(&k, scale, color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WCameraPosition::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * scale: 1.0
	/// * color: Color::white()
	// WCameraPosition(const cv::Vec2d &, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:570
	#[inline]
	pub fn new_2(fov: core::Vec2d, scale: f64, color: &crate::viz::Color) -> Result<crate::viz::WCameraPosition> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WCameraPosition_WCameraPosition_const_Vec2dR_double_const_ColorR(&fov, scale, color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WCameraPosition::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * scale: 1.0
	/// * color: Color::white()
	// WCameraPosition(const cv::Matx33d &, cv::InputArray, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:583
	#[inline]
	pub fn new_3(k: core::Matx33d, image: &dyn core::ToInputArray, scale: f64, color: &crate::viz::Color) -> Result<crate::viz::WCameraPosition> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WCameraPosition_WCameraPosition_const_Matx33dR_const__InputArrayR_double_const_ColorR(&k, image.as_raw__InputArray(), scale, color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WCameraPosition::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * scale: 1.0
	/// * color: Color::white()
	// WCameraPosition(const cv::Vec2d &, cv::InputArray, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:596
	#[inline]
	pub fn new_4(fov: core::Vec2d, image: &dyn core::ToInputArray, scale: f64, color: &crate::viz::Color) -> Result<crate::viz::WCameraPosition> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WCameraPosition_WCameraPosition_const_Vec2dR_const__InputArrayR_double_const_ColorR(&fov, image.as_raw__InputArray(), scale, color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WCameraPosition::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { WCameraPosition, crate::viz::Widget, cv_WCameraPosition_to_Widget }

boxed_cast_base! { WCameraPosition, crate::viz::Widget3D, cv_WCameraPosition_to_Widget3D }

// WCircle /usr/include/opencv2/viz/widgets.hpp:306
pub trait WCircleTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WCircle(&self) -> *const c_void;

}

pub trait WCircleTrait: crate::viz::WCircleTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WCircle(&mut self) -> *mut c_void;

}

// WCircle /usr/include/opencv2/viz/widgets.hpp:306
pub struct WCircle {
	ptr: *mut c_void
}

opencv_type_boxed! { WCircle }

impl Drop for WCircle {
	fn drop(&mut self) {
		extern "C" { fn cv_WCircle_delete(instance: *mut c_void); }
		unsafe { cv_WCircle_delete(self.as_raw_mut_WCircle()) };
	}
}

unsafe impl Send for WCircle {}

impl crate::viz::WidgetTraitConst for WCircle {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WCircle {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WCircle {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WCircle {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WCircleTraitConst for WCircle {
	#[inline] fn as_raw_WCircle(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WCircleTrait for WCircle {
	#[inline] fn as_raw_mut_WCircle(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WCircle {
	/// ## C++ default parameters
	/// * thickness: 0.01
	/// * color: Color::white()
	// WCircle(double, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:315
	#[inline]
	pub fn new(radius: f64, thickness: f64, color: &crate::viz::Color) -> Result<crate::viz::WCircle> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WCircle_WCircle_double_double_const_ColorR(radius, thickness, color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WCircle::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * thickness: 0.01
	/// * color: Color::white()
	// WCircle(double, const cv::Point3d &, const cv::Vec3d &, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:325
	#[inline]
	pub fn new_1(radius: f64, center: core::Point3d, normal: core::Vec3d, thickness: f64, color: &crate::viz::Color) -> Result<crate::viz::WCircle> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WCircle_WCircle_double_const_Point3dR_const_Vec3dR_double_const_ColorR(radius, &center, &normal, thickness, color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WCircle::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { WCircle, crate::viz::Widget, cv_WCircle_to_Widget }

boxed_cast_base! { WCircle, crate::viz::Widget3D, cv_WCircle_to_Widget3D }

// WCloud /usr/include/opencv2/viz/widgets.hpp:680
pub trait WCloudTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WCloud(&self) -> *const c_void;

}

pub trait WCloudTrait: crate::viz::WCloudTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WCloud(&mut self) -> *mut c_void;

}

// WCloud /usr/include/opencv2/viz/widgets.hpp:680
pub struct WCloud {
	ptr: *mut c_void
}

opencv_type_boxed! { WCloud }

impl Drop for WCloud {
	fn drop(&mut self) {
		extern "C" { fn cv_WCloud_delete(instance: *mut c_void); }
		unsafe { cv_WCloud_delete(self.as_raw_mut_WCloud()) };
	}
}

unsafe impl Send for WCloud {}

impl crate::viz::WidgetTraitConst for WCloud {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WCloud {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WCloud {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WCloud {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WCloudTraitConst for WCloud {
	#[inline] fn as_raw_WCloud(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WCloudTrait for WCloud {
	#[inline] fn as_raw_mut_WCloud(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WCloud {
	// WCloud(cv::InputArray, cv::InputArray) /usr/include/opencv2/viz/widgets.hpp:690
	#[inline]
	pub fn new(cloud: &dyn core::ToInputArray, colors: &dyn core::ToInputArray) -> Result<crate::viz::WCloud> {
		input_array_arg!(cloud);
		input_array_arg!(colors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WCloud_WCloud_const__InputArrayR_const__InputArrayR(cloud.as_raw__InputArray(), colors.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WCloud::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * color: Color::white()
	// WCloud(cv::InputArray, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:698
	#[inline]
	pub fn new_1(cloud: &dyn core::ToInputArray, color: &crate::viz::Color) -> Result<crate::viz::WCloud> {
		input_array_arg!(cloud);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WCloud_WCloud_const__InputArrayR_const_ColorR(cloud.as_raw__InputArray(), color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WCloud::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// WCloud(cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/viz/widgets.hpp:707
	#[inline]
	pub fn new_2(cloud: &dyn core::ToInputArray, colors: &dyn core::ToInputArray, normals: &dyn core::ToInputArray) -> Result<crate::viz::WCloud> {
		input_array_arg!(cloud);
		input_array_arg!(colors);
		input_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WCloud_WCloud_const__InputArrayR_const__InputArrayR_const__InputArrayR(cloud.as_raw__InputArray(), colors.as_raw__InputArray(), normals.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WCloud::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// WCloud(cv::InputArray, const cv::viz::Color &, cv::InputArray) /usr/include/opencv2/viz/widgets.hpp:717
	#[inline]
	pub fn new_3(cloud: &dyn core::ToInputArray, color: &crate::viz::Color, normals: &dyn core::ToInputArray) -> Result<crate::viz::WCloud> {
		input_array_arg!(cloud);
		input_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WCloud_WCloud_const__InputArrayR_const_ColorR_const__InputArrayR(cloud.as_raw__InputArray(), color.as_raw_Color(), normals.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WCloud::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { WCloud, crate::viz::Widget, cv_WCloud_to_Widget }

boxed_cast_base! { WCloud, crate::viz::Widget3D, cv_WCloud_to_Widget3D }

// WCloudCollection /usr/include/opencv2/viz/widgets.hpp:736
pub trait WCloudCollectionTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WCloudCollection(&self) -> *const c_void;

}

pub trait WCloudCollectionTrait: crate::viz::WCloudCollectionTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WCloudCollection(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * pose: Affine3d::Identity()
	// addCloud(cv::InputArray, cv::InputArray, const cv::Affine3d &) /usr/include/opencv2/viz/widgets.hpp:747
	#[inline]
	fn add_cloud(&mut self, cloud: &dyn core::ToInputArray, colors: &dyn core::ToInputArray, pose: core::Affine3d) -> Result<()> {
		input_array_arg!(cloud);
		input_array_arg!(colors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WCloudCollection_addCloud_const__InputArrayR_const__InputArrayR_const_Affine3dR(self.as_raw_mut_WCloudCollection(), cloud.as_raw__InputArray(), colors.as_raw__InputArray(), &pose, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * color: Color::white()
	/// * pose: Affine3d::Identity()
	// addCloud(cv::InputArray, const cv::viz::Color &, const cv::Affine3d &) /usr/include/opencv2/viz/widgets.hpp:754
	#[inline]
	fn add_cloud_1(&mut self, cloud: &dyn core::ToInputArray, color: &crate::viz::Color, pose: core::Affine3d) -> Result<()> {
		input_array_arg!(cloud);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WCloudCollection_addCloud_const__InputArrayR_const_ColorR_const_Affine3dR(self.as_raw_mut_WCloudCollection(), cloud.as_raw__InputArray(), color.as_raw_Color(), &pose, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// finalize() /usr/include/opencv2/viz/widgets.hpp:759
	#[inline]
	fn finalize(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WCloudCollection_finalize(self.as_raw_mut_WCloudCollection(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// WCloudCollection /usr/include/opencv2/viz/widgets.hpp:736
pub struct WCloudCollection {
	ptr: *mut c_void
}

opencv_type_boxed! { WCloudCollection }

impl Drop for WCloudCollection {
	fn drop(&mut self) {
		extern "C" { fn cv_WCloudCollection_delete(instance: *mut c_void); }
		unsafe { cv_WCloudCollection_delete(self.as_raw_mut_WCloudCollection()) };
	}
}

unsafe impl Send for WCloudCollection {}

impl crate::viz::WidgetTraitConst for WCloudCollection {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WCloudCollection {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WCloudCollection {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WCloudCollection {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WCloudCollectionTraitConst for WCloudCollection {
	#[inline] fn as_raw_WCloudCollection(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WCloudCollectionTrait for WCloudCollection {
	#[inline] fn as_raw_mut_WCloudCollection(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WCloudCollection {
	// WCloudCollection() /usr/include/opencv2/viz/widgets.hpp:739
	#[inline]
	pub fn default() -> Result<crate::viz::WCloudCollection> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WCloudCollection_WCloudCollection(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WCloudCollection::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { WCloudCollection, crate::viz::Widget, cv_WCloudCollection_to_Widget }

boxed_cast_base! { WCloudCollection, crate::viz::Widget3D, cv_WCloudCollection_to_Widget3D }

// WCloudNormals /usr/include/opencv2/viz/widgets.hpp:764
pub trait WCloudNormalsTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WCloudNormals(&self) -> *const c_void;

}

pub trait WCloudNormalsTrait: crate::viz::WCloudNormalsTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WCloudNormals(&mut self) -> *mut c_void;

}

// WCloudNormals /usr/include/opencv2/viz/widgets.hpp:764
pub struct WCloudNormals {
	ptr: *mut c_void
}

opencv_type_boxed! { WCloudNormals }

impl Drop for WCloudNormals {
	fn drop(&mut self) {
		extern "C" { fn cv_WCloudNormals_delete(instance: *mut c_void); }
		unsafe { cv_WCloudNormals_delete(self.as_raw_mut_WCloudNormals()) };
	}
}

unsafe impl Send for WCloudNormals {}

impl crate::viz::WidgetTraitConst for WCloudNormals {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WCloudNormals {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WCloudNormals {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WCloudNormals {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WCloudNormalsTraitConst for WCloudNormals {
	#[inline] fn as_raw_WCloudNormals(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WCloudNormalsTrait for WCloudNormals {
	#[inline] fn as_raw_mut_WCloudNormals(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WCloudNormals {
	/// ## C++ default parameters
	/// * level: 64
	/// * scale: 0.1
	/// * color: Color::white()
	// WCloudNormals(cv::InputArray, cv::InputArray, int, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:777
	#[inline]
	pub fn new(cloud: &dyn core::ToInputArray, normals: &dyn core::ToInputArray, level: i32, scale: f64, color: &crate::viz::Color) -> Result<crate::viz::WCloudNormals> {
		input_array_arg!(cloud);
		input_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WCloudNormals_WCloudNormals_const__InputArrayR_const__InputArrayR_int_double_const_ColorR(cloud.as_raw__InputArray(), normals.as_raw__InputArray(), level, scale, color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WCloudNormals::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { WCloudNormals, crate::viz::Widget, cv_WCloudNormals_to_Widget }

boxed_cast_base! { WCloudNormals, crate::viz::Widget3D, cv_WCloudNormals_to_Widget3D }

// WCone /usr/include/opencv2/viz/widgets.hpp:330
pub trait WConeTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WCone(&self) -> *const c_void;

}

pub trait WConeTrait: crate::viz::WConeTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WCone(&mut self) -> *mut c_void;

}

// WCone /usr/include/opencv2/viz/widgets.hpp:330
pub struct WCone {
	ptr: *mut c_void
}

opencv_type_boxed! { WCone }

impl Drop for WCone {
	fn drop(&mut self) {
		extern "C" { fn cv_WCone_delete(instance: *mut c_void); }
		unsafe { cv_WCone_delete(self.as_raw_mut_WCone()) };
	}
}

unsafe impl Send for WCone {}

impl crate::viz::WidgetTraitConst for WCone {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WCone {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WCone {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WCone {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WConeTraitConst for WCone {
	#[inline] fn as_raw_WCone(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WConeTrait for WCone {
	#[inline] fn as_raw_mut_WCone(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WCone {
	/// ## C++ default parameters
	/// * resolution: 6
	/// * color: Color::white()
	// WCone(double, double, int, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:340
	#[inline]
	pub fn new(length: f64, radius: f64, resolution: i32, color: &crate::viz::Color) -> Result<crate::viz::WCone> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WCone_WCone_double_double_int_const_ColorR(length, radius, resolution, color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WCone::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * resolution: 6
	/// * color: Color::white()
	// WCone(double, const cv::Point3d &, const cv::Point3d &, int, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:351
	#[inline]
	pub fn new_1(radius: f64, center: core::Point3d, tip: core::Point3d, resolution: i32, color: &crate::viz::Color) -> Result<crate::viz::WCone> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WCone_WCone_double_const_Point3dR_const_Point3dR_int_const_ColorR(radius, &center, &tip, resolution, color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WCone::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { WCone, crate::viz::Widget, cv_WCone_to_Widget }

boxed_cast_base! { WCone, crate::viz::Widget3D, cv_WCone_to_Widget3D }

// WCoordinateSystem /usr/include/opencv2/viz/widgets.hpp:513
pub trait WCoordinateSystemTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WCoordinateSystem(&self) -> *const c_void;

}

pub trait WCoordinateSystemTrait: crate::viz::WCoordinateSystemTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WCoordinateSystem(&mut self) -> *mut c_void;

}

// WCoordinateSystem /usr/include/opencv2/viz/widgets.hpp:513
pub struct WCoordinateSystem {
	ptr: *mut c_void
}

opencv_type_boxed! { WCoordinateSystem }

impl Drop for WCoordinateSystem {
	fn drop(&mut self) {
		extern "C" { fn cv_WCoordinateSystem_delete(instance: *mut c_void); }
		unsafe { cv_WCoordinateSystem_delete(self.as_raw_mut_WCoordinateSystem()) };
	}
}

unsafe impl Send for WCoordinateSystem {}

impl crate::viz::WidgetTraitConst for WCoordinateSystem {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WCoordinateSystem {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WCoordinateSystem {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WCoordinateSystem {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WCoordinateSystemTraitConst for WCoordinateSystem {
	#[inline] fn as_raw_WCoordinateSystem(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WCoordinateSystemTrait for WCoordinateSystem {
	#[inline] fn as_raw_mut_WCoordinateSystem(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WCoordinateSystem {
	/// ## C++ default parameters
	/// * scale: 1.0
	// WCoordinateSystem(double) /usr/include/opencv2/viz/widgets.hpp:520
	#[inline]
	pub fn new(scale: f64) -> Result<crate::viz::WCoordinateSystem> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WCoordinateSystem_WCoordinateSystem_double(scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WCoordinateSystem::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { WCoordinateSystem, crate::viz::Widget, cv_WCoordinateSystem_to_Widget }

boxed_cast_base! { WCoordinateSystem, crate::viz::Widget3D, cv_WCoordinateSystem_to_Widget3D }

// WCube /usr/include/opencv2/viz/widgets.hpp:372
pub trait WCubeTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WCube(&self) -> *const c_void;

}

pub trait WCubeTrait: crate::viz::WCubeTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WCube(&mut self) -> *mut c_void;

}

// WCube /usr/include/opencv2/viz/widgets.hpp:372
pub struct WCube {
	ptr: *mut c_void
}

opencv_type_boxed! { WCube }

impl Drop for WCube {
	fn drop(&mut self) {
		extern "C" { fn cv_WCube_delete(instance: *mut c_void); }
		unsafe { cv_WCube_delete(self.as_raw_mut_WCube()) };
	}
}

unsafe impl Send for WCube {}

impl crate::viz::WidgetTraitConst for WCube {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WCube {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WCube {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WCube {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WCubeTraitConst for WCube {
	#[inline] fn as_raw_WCube(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WCubeTrait for WCube {
	#[inline] fn as_raw_mut_WCube(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WCube {
	/// ## C++ default parameters
	/// * min_point: Vec3d::all(-0.5)
	/// * max_point: Vec3d::all(0.5)
	/// * wire_frame: true
	/// * color: Color::white()
	// WCube(const cv::Point3d &, const cv::Point3d &, bool, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:384
	#[inline]
	pub fn new(min_point: core::Point3d, max_point: core::Point3d, wire_frame: bool, color: &crate::viz::Color) -> Result<crate::viz::WCube> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WCube_WCube_const_Point3dR_const_Point3dR_bool_const_ColorR(&min_point, &max_point, wire_frame, color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WCube::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { WCube, crate::viz::Widget, cv_WCube_to_Widget }

boxed_cast_base! { WCube, crate::viz::Widget3D, cv_WCube_to_Widget3D }

// WCylinder /usr/include/opencv2/viz/widgets.hpp:356
pub trait WCylinderTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WCylinder(&self) -> *const c_void;

}

pub trait WCylinderTrait: crate::viz::WCylinderTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WCylinder(&mut self) -> *mut c_void;

}

// WCylinder /usr/include/opencv2/viz/widgets.hpp:356
pub struct WCylinder {
	ptr: *mut c_void
}

opencv_type_boxed! { WCylinder }

impl Drop for WCylinder {
	fn drop(&mut self) {
		extern "C" { fn cv_WCylinder_delete(instance: *mut c_void); }
		unsafe { cv_WCylinder_delete(self.as_raw_mut_WCylinder()) };
	}
}

unsafe impl Send for WCylinder {}

impl crate::viz::WidgetTraitConst for WCylinder {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WCylinder {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WCylinder {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WCylinder {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WCylinderTraitConst for WCylinder {
	#[inline] fn as_raw_WCylinder(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WCylinderTrait for WCylinder {
	#[inline] fn as_raw_mut_WCylinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WCylinder {
	/// ## C++ default parameters
	/// * numsides: 30
	/// * color: Color::white()
	// WCylinder(const cv::Point3d &, const cv::Point3d &, double, int, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:367
	#[inline]
	pub fn new(axis_point1: core::Point3d, axis_point2: core::Point3d, radius: f64, numsides: i32, color: &crate::viz::Color) -> Result<crate::viz::WCylinder> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WCylinder_WCylinder_const_Point3dR_const_Point3dR_double_int_const_ColorR(&axis_point1, &axis_point2, radius, numsides, color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WCylinder::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { WCylinder, crate::viz::Widget, cv_WCylinder_to_Widget }

boxed_cast_base! { WCylinder, crate::viz::Widget3D, cv_WCylinder_to_Widget3D }

// WGrid /usr/include/opencv2/viz/widgets.hpp:525
pub trait WGridTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WGrid(&self) -> *const c_void;

}

pub trait WGridTrait: crate::viz::WGridTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WGrid(&mut self) -> *mut c_void;

}

// WGrid /usr/include/opencv2/viz/widgets.hpp:525
pub struct WGrid {
	ptr: *mut c_void
}

opencv_type_boxed! { WGrid }

impl Drop for WGrid {
	fn drop(&mut self) {
		extern "C" { fn cv_WGrid_delete(instance: *mut c_void); }
		unsafe { cv_WGrid_delete(self.as_raw_mut_WGrid()) };
	}
}

unsafe impl Send for WGrid {}

impl crate::viz::WidgetTraitConst for WGrid {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WGrid {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WGrid {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WGrid {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WGridTraitConst for WGrid {
	#[inline] fn as_raw_WGrid(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WGridTrait for WGrid {
	#[inline] fn as_raw_mut_WGrid(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WGrid {
	/// ## C++ default parameters
	/// * cells: Vec2i::all(10)
	/// * cells_spacing: Vec2d::all(1.0)
	/// * color: Color::white()
	// WGrid(const cv::Vec2i &, const cv::Vec2d &, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:534
	#[inline]
	pub fn new(cells: core::Vec2i, cells_spacing: core::Vec2d, color: &crate::viz::Color) -> Result<crate::viz::WGrid> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WGrid_WGrid_const_Vec2iR_const_Vec2dR_const_ColorR(&cells, &cells_spacing, color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WGrid::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * cells: Vec2i::all(10)
	/// * cells_spacing: Vec2d::all(1.0)
	/// * color: Color::white()
	// WGrid(const cv::Point3d &, const cv::Vec3d &, const cv::Vec3d &, const cv::Vec2i &, const cv::Vec2d &, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:537
	#[inline]
	pub fn new_1(center: core::Point3d, normal: core::Vec3d, new_yaxis: core::Vec3d, cells: core::Vec2i, cells_spacing: core::Vec2d, color: &crate::viz::Color) -> Result<crate::viz::WGrid> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WGrid_WGrid_const_Point3dR_const_Vec3dR_const_Vec3dR_const_Vec2iR_const_Vec2dR_const_ColorR(&center, &normal, &new_yaxis, &cells, &cells_spacing, color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WGrid::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { WGrid, crate::viz::Widget, cv_WGrid_to_Widget }

boxed_cast_base! { WGrid, crate::viz::Widget3D, cv_WGrid_to_Widget3D }

// WImage3D /usr/include/opencv2/viz/widgets.hpp:475
pub trait WImage3DTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WImage3D(&self) -> *const c_void;

}

pub trait WImage3DTrait: crate::viz::WImage3DTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WImage3D(&mut self) -> *mut c_void;

	// setImage(cv::InputArray) /usr/include/opencv2/viz/widgets.hpp:499
	#[inline]
	fn set_image(&mut self, image: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WImage3D_setImage_const__InputArrayR(self.as_raw_mut_WImage3D(), image.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setSize(const cv::Size &) /usr/include/opencv2/viz/widgets.hpp:505
	#[inline]
	fn set_size(&mut self, size: core::Size) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WImage3D_setSize_const_SizeR(self.as_raw_mut_WImage3D(), &size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// WImage3D /usr/include/opencv2/viz/widgets.hpp:475
pub struct WImage3D {
	ptr: *mut c_void
}

opencv_type_boxed! { WImage3D }

impl Drop for WImage3D {
	fn drop(&mut self) {
		extern "C" { fn cv_WImage3D_delete(instance: *mut c_void); }
		unsafe { cv_WImage3D_delete(self.as_raw_mut_WImage3D()) };
	}
}

unsafe impl Send for WImage3D {}

impl crate::viz::WidgetTraitConst for WImage3D {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WImage3D {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WImage3D {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WImage3D {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WImage3DTraitConst for WImage3D {
	#[inline] fn as_raw_WImage3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WImage3DTrait for WImage3D {
	#[inline] fn as_raw_mut_WImage3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WImage3D {
	// WImage3D(cv::InputArray, const cv::Size2d &) /usr/include/opencv2/viz/widgets.hpp:483
	#[inline]
	pub fn new(image: &dyn core::ToInputArray, size: core::Size2d) -> Result<crate::viz::WImage3D> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WImage3D_WImage3D_const__InputArrayR_const_Size2dR(image.as_raw__InputArray(), &size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WImage3D::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// WImage3D(cv::InputArray, const cv::Size2d &, const cv::Vec3d &, const cv::Vec3d &, const cv::Vec3d &) /usr/include/opencv2/viz/widgets.hpp:493
	#[inline]
	pub fn new_1(image: &dyn core::ToInputArray, size: core::Size2d, center: core::Vec3d, normal: core::Vec3d, up_vector: core::Vec3d) -> Result<crate::viz::WImage3D> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WImage3D_WImage3D_const__InputArrayR_const_Size2dR_const_Vec3dR_const_Vec3dR_const_Vec3dR(image.as_raw__InputArray(), &size, &center, &normal, &up_vector, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WImage3D::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { WImage3D, crate::viz::Widget, cv_WImage3D_to_Widget }

boxed_cast_base! { WImage3D, crate::viz::Widget3D, cv_WImage3D_to_Widget3D }

// WImageOverlay /usr/include/opencv2/viz/widgets.hpp:457
pub trait WImageOverlayTraitConst: crate::viz::Widget2DTraitConst {
	fn as_raw_WImageOverlay(&self) -> *const c_void;

}

pub trait WImageOverlayTrait: crate::viz::WImageOverlayTraitConst + crate::viz::Widget2DTrait {
	fn as_raw_mut_WImageOverlay(&mut self) -> *mut c_void;

	// setImage(cv::InputArray) /usr/include/opencv2/viz/widgets.hpp:470
	#[inline]
	fn set_image(&mut self, image: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WImageOverlay_setImage_const__InputArrayR(self.as_raw_mut_WImageOverlay(), image.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// WImageOverlay /usr/include/opencv2/viz/widgets.hpp:457
pub struct WImageOverlay {
	ptr: *mut c_void
}

opencv_type_boxed! { WImageOverlay }

impl Drop for WImageOverlay {
	fn drop(&mut self) {
		extern "C" { fn cv_WImageOverlay_delete(instance: *mut c_void); }
		unsafe { cv_WImageOverlay_delete(self.as_raw_mut_WImageOverlay()) };
	}
}

unsafe impl Send for WImageOverlay {}

impl crate::viz::WidgetTraitConst for WImageOverlay {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WImageOverlay {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget2DTraitConst for WImageOverlay {
	#[inline] fn as_raw_Widget2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget2DTrait for WImageOverlay {
	#[inline] fn as_raw_mut_Widget2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WImageOverlayTraitConst for WImageOverlay {
	#[inline] fn as_raw_WImageOverlay(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WImageOverlayTrait for WImageOverlay {
	#[inline] fn as_raw_mut_WImageOverlay(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WImageOverlay {
	// WImageOverlay(cv::InputArray, const cv::Rect &) /usr/include/opencv2/viz/widgets.hpp:465
	#[inline]
	pub fn new(image: &dyn core::ToInputArray, rect: core::Rect) -> Result<crate::viz::WImageOverlay> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WImageOverlay_WImageOverlay_const__InputArrayR_const_RectR(image.as_raw__InputArray(), &rect, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WImageOverlay::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { WImageOverlay, crate::viz::Widget, cv_WImageOverlay_to_Widget }

boxed_cast_base! { WImageOverlay, crate::viz::Widget2D, cv_WImageOverlay_to_Widget2D }

// WLine /usr/include/opencv2/viz/widgets.hpp:235
pub trait WLineTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WLine(&self) -> *const c_void;

}

pub trait WLineTrait: crate::viz::WLineTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WLine(&mut self) -> *mut c_void;

}

// WLine /usr/include/opencv2/viz/widgets.hpp:235
pub struct WLine {
	ptr: *mut c_void
}

opencv_type_boxed! { WLine }

impl Drop for WLine {
	fn drop(&mut self) {
		extern "C" { fn cv_WLine_delete(instance: *mut c_void); }
		unsafe { cv_WLine_delete(self.as_raw_mut_WLine()) };
	}
}

unsafe impl Send for WLine {}

impl crate::viz::WidgetTraitConst for WLine {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WLine {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WLine {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WLine {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WLineTraitConst for WLine {
	#[inline] fn as_raw_WLine(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WLineTrait for WLine {
	#[inline] fn as_raw_mut_WLine(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WLine {
	/// ## C++ default parameters
	/// * color: Color::white()
	// WLine(const cv::Point3d &, const cv::Point3d &, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:244
	#[inline]
	pub fn new(pt1: core::Point3d, pt2: core::Point3d, color: &crate::viz::Color) -> Result<crate::viz::WLine> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WLine_WLine_const_Point3dR_const_Point3dR_const_ColorR(&pt1, &pt2, color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WLine::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { WLine, crate::viz::Widget, cv_WLine_to_Widget }

boxed_cast_base! { WLine, crate::viz::Widget3D, cv_WLine_to_Widget3D }

// WMesh /usr/include/opencv2/viz/widgets.hpp:788
pub trait WMeshTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WMesh(&self) -> *const c_void;

}

pub trait WMeshTrait: crate::viz::WMeshTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WMesh(&mut self) -> *mut c_void;

}

// WMesh /usr/include/opencv2/viz/widgets.hpp:788
pub struct WMesh {
	ptr: *mut c_void
}

opencv_type_boxed! { WMesh }

impl Drop for WMesh {
	fn drop(&mut self) {
		extern "C" { fn cv_WMesh_delete(instance: *mut c_void); }
		unsafe { cv_WMesh_delete(self.as_raw_mut_WMesh()) };
	}
}

unsafe impl Send for WMesh {}

impl crate::viz::WidgetTraitConst for WMesh {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WMesh {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WMesh {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WMesh {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WMeshTraitConst for WMesh {
	#[inline] fn as_raw_WMesh(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WMeshTrait for WMesh {
	#[inline] fn as_raw_mut_WMesh(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WMesh {
	// WMesh(const cv::viz::Mesh &) /usr/include/opencv2/viz/widgets.hpp:794
	#[inline]
	pub fn new(mesh: &crate::viz::Mesh) -> Result<crate::viz::WMesh> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WMesh_WMesh_const_MeshR(mesh.as_raw_Mesh(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WMesh::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * colors: noArray()
	/// * normals: noArray()
	// WMesh(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/viz/widgets.hpp:795
	#[inline]
	pub fn new_1(cloud: &dyn core::ToInputArray, polygons: &dyn core::ToInputArray, colors: &dyn core::ToInputArray, normals: &dyn core::ToInputArray) -> Result<crate::viz::WMesh> {
		input_array_arg!(cloud);
		input_array_arg!(polygons);
		input_array_arg!(colors);
		input_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WMesh_WMesh_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(cloud.as_raw__InputArray(), polygons.as_raw__InputArray(), colors.as_raw__InputArray(), normals.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WMesh::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { WMesh, crate::viz::Widget, cv_WMesh_to_Widget }

boxed_cast_base! { WMesh, crate::viz::Widget3D, cv_WMesh_to_Widget3D }

// WPaintedCloud /usr/include/opencv2/viz/widgets.hpp:720
pub trait WPaintedCloudTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WPaintedCloud(&self) -> *const c_void;

}

pub trait WPaintedCloudTrait: crate::viz::WPaintedCloudTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WPaintedCloud(&mut self) -> *mut c_void;

}

// WPaintedCloud /usr/include/opencv2/viz/widgets.hpp:720
pub struct WPaintedCloud {
	ptr: *mut c_void
}

opencv_type_boxed! { WPaintedCloud }

impl Drop for WPaintedCloud {
	fn drop(&mut self) {
		extern "C" { fn cv_WPaintedCloud_delete(instance: *mut c_void); }
		unsafe { cv_WPaintedCloud_delete(self.as_raw_mut_WPaintedCloud()) };
	}
}

unsafe impl Send for WPaintedCloud {}

impl crate::viz::WidgetTraitConst for WPaintedCloud {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WPaintedCloud {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WPaintedCloud {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WPaintedCloud {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WPaintedCloudTraitConst for WPaintedCloud {
	#[inline] fn as_raw_WPaintedCloud(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WPaintedCloudTrait for WPaintedCloud {
	#[inline] fn as_raw_mut_WPaintedCloud(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WPaintedCloud {
	// WPaintedCloud(cv::InputArray) /usr/include/opencv2/viz/widgets.hpp:724
	#[inline]
	pub fn new(cloud: &dyn core::ToInputArray) -> Result<crate::viz::WPaintedCloud> {
		input_array_arg!(cloud);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WPaintedCloud_WPaintedCloud_const__InputArrayR(cloud.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WPaintedCloud::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// WPaintedCloud(cv::InputArray, const cv::Point3d &, const cv::Point3d &) /usr/include/opencv2/viz/widgets.hpp:727
	#[inline]
	pub fn new_1(cloud: &dyn core::ToInputArray, p1: core::Point3d, p2: core::Point3d) -> Result<crate::viz::WPaintedCloud> {
		input_array_arg!(cloud);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WPaintedCloud_WPaintedCloud_const__InputArrayR_const_Point3dR_const_Point3dR(cloud.as_raw__InputArray(), &p1, &p2, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WPaintedCloud::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// WPaintedCloud(cv::InputArray, const cv::Point3d &, const cv::Point3d &, const cv::viz::Color &, const cv::viz::Color) /usr/include/opencv2/viz/widgets.hpp:730
	#[inline]
	pub fn new_2(cloud: &dyn core::ToInputArray, p1: core::Point3d, p2: core::Point3d, c1: &crate::viz::Color, c2: crate::viz::Color) -> Result<crate::viz::WPaintedCloud> {
		input_array_arg!(cloud);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WPaintedCloud_WPaintedCloud_const__InputArrayR_const_Point3dR_const_Point3dR_const_ColorR_const_Color(cloud.as_raw__InputArray(), &p1, &p2, c1.as_raw_Color(), c2.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WPaintedCloud::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { WPaintedCloud, crate::viz::Widget, cv_WPaintedCloud_to_Widget }

boxed_cast_base! { WPaintedCloud, crate::viz::Widget3D, cv_WPaintedCloud_to_Widget3D }

// WPlane /usr/include/opencv2/viz/widgets.hpp:249
pub trait WPlaneTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WPlane(&self) -> *const c_void;

}

pub trait WPlaneTrait: crate::viz::WPlaneTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WPlane(&mut self) -> *mut c_void;

}

// WPlane /usr/include/opencv2/viz/widgets.hpp:249
pub struct WPlane {
	ptr: *mut c_void
}

opencv_type_boxed! { WPlane }

impl Drop for WPlane {
	fn drop(&mut self) {
		extern "C" { fn cv_WPlane_delete(instance: *mut c_void); }
		unsafe { cv_WPlane_delete(self.as_raw_mut_WPlane()) };
	}
}

unsafe impl Send for WPlane {}

impl crate::viz::WidgetTraitConst for WPlane {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WPlane {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WPlane {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WPlane {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WPlaneTraitConst for WPlane {
	#[inline] fn as_raw_WPlane(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WPlaneTrait for WPlane {
	#[inline] fn as_raw_mut_WPlane(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WPlane {
	/// ## C++ default parameters
	/// * size: Size2d(1.0,1.0)
	/// * color: Color::white()
	// WPlane(const cv::Size2d &, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:257
	#[inline]
	pub fn new(size: core::Size2d, color: &crate::viz::Color) -> Result<crate::viz::WPlane> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WPlane_WPlane_const_Size2dR_const_ColorR(&size, color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WPlane::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * size: Size2d(1.0,1.0)
	/// * color: Color::white()
	// WPlane(const cv::Point3d &, const cv::Vec3d &, const cv::Vec3d &, const cv::Size2d &, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:267
	#[inline]
	pub fn new_1(center: core::Point3d, normal: core::Vec3d, new_yaxis: core::Vec3d, size: core::Size2d, color: &crate::viz::Color) -> Result<crate::viz::WPlane> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WPlane_WPlane_const_Point3dR_const_Vec3dR_const_Vec3dR_const_Size2dR_const_ColorR(&center, &normal, &new_yaxis, &size, color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WPlane::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { WPlane, crate::viz::Widget, cv_WPlane_to_Widget }

boxed_cast_base! { WPlane, crate::viz::Widget3D, cv_WPlane_to_Widget3D }

// WPolyLine /usr/include/opencv2/viz/widgets.hpp:390
pub trait WPolyLineTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WPolyLine(&self) -> *const c_void;

}

pub trait WPolyLineTrait: crate::viz::WPolyLineTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WPolyLine(&mut self) -> *mut c_void;

}

// WPolyLine /usr/include/opencv2/viz/widgets.hpp:390
pub struct WPolyLine {
	ptr: *mut c_void
}

opencv_type_boxed! { WPolyLine }

impl Drop for WPolyLine {
	fn drop(&mut self) {
		extern "C" { fn cv_WPolyLine_delete(instance: *mut c_void); }
		unsafe { cv_WPolyLine_delete(self.as_raw_mut_WPolyLine()) };
	}
}

unsafe impl Send for WPolyLine {}

impl crate::viz::WidgetTraitConst for WPolyLine {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WPolyLine {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WPolyLine {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WPolyLine {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WPolyLineTraitConst for WPolyLine {
	#[inline] fn as_raw_WPolyLine(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WPolyLineTrait for WPolyLine {
	#[inline] fn as_raw_mut_WPolyLine(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WPolyLine {
	// WPolyLine(cv::InputArray, cv::InputArray) /usr/include/opencv2/viz/widgets.hpp:393
	#[inline]
	pub fn new(points: &dyn core::ToInputArray, colors: &dyn core::ToInputArray) -> Result<crate::viz::WPolyLine> {
		input_array_arg!(points);
		input_array_arg!(colors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WPolyLine_WPolyLine_const__InputArrayR_const__InputArrayR(points.as_raw__InputArray(), colors.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WPolyLine::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * color: Color::white()
	// WPolyLine(cv::InputArray, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:399
	#[inline]
	pub fn new_1(points: &dyn core::ToInputArray, color: &crate::viz::Color) -> Result<crate::viz::WPolyLine> {
		input_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WPolyLine_WPolyLine_const__InputArrayR_const_ColorR(points.as_raw__InputArray(), color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WPolyLine::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { WPolyLine, crate::viz::Widget, cv_WPolyLine_to_Widget }

boxed_cast_base! { WPolyLine, crate::viz::Widget3D, cv_WPolyLine_to_Widget3D }

// WSphere /usr/include/opencv2/viz/widgets.hpp:273
pub trait WSphereTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WSphere(&self) -> *const c_void;

}

pub trait WSphereTrait: crate::viz::WSphereTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WSphere(&mut self) -> *mut c_void;

}

// WSphere /usr/include/opencv2/viz/widgets.hpp:273
pub struct WSphere {
	ptr: *mut c_void
}

opencv_type_boxed! { WSphere }

impl Drop for WSphere {
	fn drop(&mut self) {
		extern "C" { fn cv_WSphere_delete(instance: *mut c_void); }
		unsafe { cv_WSphere_delete(self.as_raw_mut_WSphere()) };
	}
}

unsafe impl Send for WSphere {}

impl crate::viz::WidgetTraitConst for WSphere {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WSphere {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WSphere {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WSphere {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WSphereTraitConst for WSphere {
	#[inline] fn as_raw_WSphere(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WSphereTrait for WSphere {
	#[inline] fn as_raw_mut_WSphere(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WSphere {
	/// ## C++ default parameters
	/// * sphere_resolution: 10
	/// * color: Color::white()
	// WSphere(const cv::Point3d &, double, int, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:283
	#[inline]
	pub fn new(center: core::Point3d, radius: f64, sphere_resolution: i32, color: &crate::viz::Color) -> Result<crate::viz::WSphere> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WSphere_WSphere_const_Point3dR_double_int_const_ColorR(&center, radius, sphere_resolution, color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WSphere::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { WSphere, crate::viz::Widget, cv_WSphere_to_Widget }

boxed_cast_base! { WSphere, crate::viz::Widget3D, cv_WSphere_to_Widget3D }

// WText /usr/include/opencv2/viz/widgets.hpp:407
pub trait WTextTraitConst: crate::viz::Widget2DTraitConst {
	fn as_raw_WText(&self) -> *const c_void;

	// getText() /usr/include/opencv2/viz/widgets.hpp:426
	#[inline]
	fn get_text(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WText_getText_const(self.as_raw_WText(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait WTextTrait: crate::viz::WTextTraitConst + crate::viz::Widget2DTrait {
	fn as_raw_mut_WText(&mut self) -> *mut c_void;

	// setText(const cv::String &) /usr/include/opencv2/viz/widgets.hpp:423
	#[inline]
	fn set_text(&mut self, text: &str) -> Result<()> {
		extern_container_arg!(text);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WText_setText_const_StringR(self.as_raw_mut_WText(), text.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// WText /usr/include/opencv2/viz/widgets.hpp:407
pub struct WText {
	ptr: *mut c_void
}

opencv_type_boxed! { WText }

impl Drop for WText {
	fn drop(&mut self) {
		extern "C" { fn cv_WText_delete(instance: *mut c_void); }
		unsafe { cv_WText_delete(self.as_raw_mut_WText()) };
	}
}

unsafe impl Send for WText {}

impl crate::viz::WidgetTraitConst for WText {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WText {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget2DTraitConst for WText {
	#[inline] fn as_raw_Widget2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget2DTrait for WText {
	#[inline] fn as_raw_mut_Widget2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WTextTraitConst for WText {
	#[inline] fn as_raw_WText(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WTextTrait for WText {
	#[inline] fn as_raw_mut_WText(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WText {
	/// ## C++ default parameters
	/// * font_size: 20
	/// * color: Color::white()
	// WText(const cv::String &, const cv::Point &, int, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:417
	#[inline]
	pub fn new(text: &str, pos: core::Point, font_size: i32, color: &crate::viz::Color) -> Result<crate::viz::WText> {
		extern_container_arg!(text);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WText_WText_const_StringR_const_PointR_int_const_ColorR(text.opencv_as_extern(), &pos, font_size, color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WText::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { WText, crate::viz::Widget, cv_WText_to_Widget }

boxed_cast_base! { WText, crate::viz::Widget2D, cv_WText_to_Widget2D }

// WText3D /usr/include/opencv2/viz/widgets.hpp:431
pub trait WText3DTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WText3D(&self) -> *const c_void;

	// getText() /usr/include/opencv2/viz/widgets.hpp:452
	#[inline]
	fn get_text(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WText3D_getText_const(self.as_raw_WText3D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait WText3DTrait: crate::viz::WText3DTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WText3D(&mut self) -> *mut c_void;

	// setText(const cv::String &) /usr/include/opencv2/viz/widgets.hpp:449
	#[inline]
	fn set_text(&mut self, text: &str) -> Result<()> {
		extern_container_arg!(text);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WText3D_setText_const_StringR(self.as_raw_mut_WText3D(), text.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// WText3D /usr/include/opencv2/viz/widgets.hpp:431
pub struct WText3D {
	ptr: *mut c_void
}

opencv_type_boxed! { WText3D }

impl Drop for WText3D {
	fn drop(&mut self) {
		extern "C" { fn cv_WText3D_delete(instance: *mut c_void); }
		unsafe { cv_WText3D_delete(self.as_raw_mut_WText3D()) };
	}
}

unsafe impl Send for WText3D {}

impl crate::viz::WidgetTraitConst for WText3D {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WText3D {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WText3D {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WText3D {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WText3DTraitConst for WText3D {
	#[inline] fn as_raw_WText3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WText3DTrait for WText3D {
	#[inline] fn as_raw_mut_WText3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WText3D {
	/// ## C++ default parameters
	/// * text_scale: 1.
	/// * face_camera: true
	/// * color: Color::white()
	// WText3D(const cv::String &, const cv::Point3d &, double, bool, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:442
	#[inline]
	pub fn new(text: &str, position: core::Point3d, text_scale: f64, face_camera: bool, color: &crate::viz::Color) -> Result<crate::viz::WText3D> {
		extern_container_arg!(text);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WText3D_WText3D_const_StringR_const_Point3dR_double_bool_const_ColorR(text.opencv_as_extern(), &position, text_scale, face_camera, color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WText3D::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { WText3D, crate::viz::Widget, cv_WText3D_to_Widget }

boxed_cast_base! { WText3D, crate::viz::Widget3D, cv_WText3D_to_Widget3D }

// WTrajectory /usr/include/opencv2/viz/widgets.hpp:604
pub trait WTrajectoryTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WTrajectory(&self) -> *const c_void;

}

pub trait WTrajectoryTrait: crate::viz::WTrajectoryTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WTrajectory(&mut self) -> *mut c_void;

}

// WTrajectory /usr/include/opencv2/viz/widgets.hpp:604
pub struct WTrajectory {
	ptr: *mut c_void
}

opencv_type_boxed! { WTrajectory }

impl Drop for WTrajectory {
	fn drop(&mut self) {
		extern "C" { fn cv_WTrajectory_delete(instance: *mut c_void); }
		unsafe { cv_WTrajectory_delete(self.as_raw_mut_WTrajectory()) };
	}
}

unsafe impl Send for WTrajectory {}

impl crate::viz::WidgetTraitConst for WTrajectory {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WTrajectory {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WTrajectory {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WTrajectory {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WTrajectoryTraitConst for WTrajectory {
	#[inline] fn as_raw_WTrajectory(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WTrajectoryTrait for WTrajectory {
	#[inline] fn as_raw_mut_WTrajectory(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WTrajectory {
	/// ## C++ default parameters
	/// * display_mode: WTrajectory::PATH
	/// * scale: 1.0
	/// * color: Color::white()
	// WTrajectory(cv::InputArray, int, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:622
	#[inline]
	pub fn new(path: &dyn core::ToInputArray, display_mode: i32, scale: f64, color: &crate::viz::Color) -> Result<crate::viz::WTrajectory> {
		input_array_arg!(path);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WTrajectory_WTrajectory_const__InputArrayR_int_double_const_ColorR(path.as_raw__InputArray(), display_mode, scale, color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WTrajectory::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { WTrajectory, crate::viz::Widget, cv_WTrajectory_to_Widget }

boxed_cast_base! { WTrajectory, crate::viz::Widget3D, cv_WTrajectory_to_Widget3D }

// WTrajectoryFrustums /usr/include/opencv2/viz/widgets.hpp:627
pub trait WTrajectoryFrustumsTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WTrajectoryFrustums(&self) -> *const c_void;

}

pub trait WTrajectoryFrustumsTrait: crate::viz::WTrajectoryFrustumsTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WTrajectoryFrustums(&mut self) -> *mut c_void;

}

// WTrajectoryFrustums /usr/include/opencv2/viz/widgets.hpp:627
pub struct WTrajectoryFrustums {
	ptr: *mut c_void
}

opencv_type_boxed! { WTrajectoryFrustums }

impl Drop for WTrajectoryFrustums {
	fn drop(&mut self) {
		extern "C" { fn cv_WTrajectoryFrustums_delete(instance: *mut c_void); }
		unsafe { cv_WTrajectoryFrustums_delete(self.as_raw_mut_WTrajectoryFrustums()) };
	}
}

unsafe impl Send for WTrajectoryFrustums {}

impl crate::viz::WidgetTraitConst for WTrajectoryFrustums {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WTrajectoryFrustums {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WTrajectoryFrustums {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WTrajectoryFrustums {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WTrajectoryFrustumsTraitConst for WTrajectoryFrustums {
	#[inline] fn as_raw_WTrajectoryFrustums(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WTrajectoryFrustumsTrait for WTrajectoryFrustums {
	#[inline] fn as_raw_mut_WTrajectoryFrustums(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WTrajectoryFrustums {
	/// ## C++ default parameters
	/// * scale: 1.
	/// * color: Color::white()
	// WTrajectoryFrustums(cv::InputArray, const cv::Matx33d &, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:639
	#[inline]
	pub fn new(path: &dyn core::ToInputArray, k: core::Matx33d, scale: f64, color: &crate::viz::Color) -> Result<crate::viz::WTrajectoryFrustums> {
		input_array_arg!(path);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WTrajectoryFrustums_WTrajectoryFrustums_const__InputArrayR_const_Matx33dR_double_const_ColorR(path.as_raw__InputArray(), &k, scale, color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WTrajectoryFrustums::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * scale: 1.
	/// * color: Color::white()
	// WTrajectoryFrustums(cv::InputArray, const cv::Vec2d &, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:650
	#[inline]
	pub fn new_1(path: &dyn core::ToInputArray, fov: core::Vec2d, scale: f64, color: &crate::viz::Color) -> Result<crate::viz::WTrajectoryFrustums> {
		input_array_arg!(path);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WTrajectoryFrustums_WTrajectoryFrustums_const__InputArrayR_const_Vec2dR_double_const_ColorR(path.as_raw__InputArray(), &fov, scale, color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WTrajectoryFrustums::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { WTrajectoryFrustums, crate::viz::Widget, cv_WTrajectoryFrustums_to_Widget }

boxed_cast_base! { WTrajectoryFrustums, crate::viz::Widget3D, cv_WTrajectoryFrustums_to_Widget3D }

// WTrajectorySpheres /usr/include/opencv2/viz/widgets.hpp:658
pub trait WTrajectorySpheresTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WTrajectorySpheres(&self) -> *const c_void;

}

pub trait WTrajectorySpheresTrait: crate::viz::WTrajectorySpheresTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WTrajectorySpheres(&mut self) -> *mut c_void;

}

// WTrajectorySpheres /usr/include/opencv2/viz/widgets.hpp:658
pub struct WTrajectorySpheres {
	ptr: *mut c_void
}

opencv_type_boxed! { WTrajectorySpheres }

impl Drop for WTrajectorySpheres {
	fn drop(&mut self) {
		extern "C" { fn cv_WTrajectorySpheres_delete(instance: *mut c_void); }
		unsafe { cv_WTrajectorySpheres_delete(self.as_raw_mut_WTrajectorySpheres()) };
	}
}

unsafe impl Send for WTrajectorySpheres {}

impl crate::viz::WidgetTraitConst for WTrajectorySpheres {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WTrajectorySpheres {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WTrajectorySpheres {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WTrajectorySpheres {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WTrajectorySpheresTraitConst for WTrajectorySpheres {
	#[inline] fn as_raw_WTrajectorySpheres(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WTrajectorySpheresTrait for WTrajectorySpheres {
	#[inline] fn as_raw_mut_WTrajectorySpheres(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WTrajectorySpheres {
	/// ## C++ default parameters
	/// * line_length: 0.05
	/// * radius: 0.007
	/// * from: Color::red()
	/// * to: Color::white()
	// WTrajectorySpheres(cv::InputArray, double, double, const cv::viz::Color &, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:669
	#[inline]
	pub fn new(path: &dyn core::ToInputArray, line_length: f64, radius: f64, from: &crate::viz::Color, to: &crate::viz::Color) -> Result<crate::viz::WTrajectorySpheres> {
		input_array_arg!(path);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WTrajectorySpheres_WTrajectorySpheres_const__InputArrayR_double_double_const_ColorR_const_ColorR(path.as_raw__InputArray(), line_length, radius, from.as_raw_Color(), to.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WTrajectorySpheres::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { WTrajectorySpheres, crate::viz::Widget, cv_WTrajectorySpheres_to_Widget }

boxed_cast_base! { WTrajectorySpheres, crate::viz::Widget3D, cv_WTrajectorySpheres_to_Widget3D }

// WWidgetMerger /usr/include/opencv2/viz/widgets.hpp:805
pub trait WWidgetMergerTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WWidgetMerger(&self) -> *const c_void;

}

pub trait WWidgetMergerTrait: crate::viz::WWidgetMergerTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WWidgetMerger(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * pose: Affine3d::Identity()
	// addWidget(const cv::viz::Widget3D &, const cv::Affine3d &) /usr/include/opencv2/viz/widgets.hpp:811
	#[inline]
	fn add_widget(&mut self, widget: &crate::viz::Widget3D, pose: core::Affine3d) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WWidgetMerger_addWidget_const_Widget3DR_const_Affine3dR(self.as_raw_mut_WWidgetMerger(), widget.as_raw_Widget3D(), &pose, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// finalize() /usr/include/opencv2/viz/widgets.hpp:814
	#[inline]
	fn finalize(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WWidgetMerger_finalize(self.as_raw_mut_WWidgetMerger(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// WWidgetMerger /usr/include/opencv2/viz/widgets.hpp:805
pub struct WWidgetMerger {
	ptr: *mut c_void
}

opencv_type_boxed! { WWidgetMerger }

impl Drop for WWidgetMerger {
	fn drop(&mut self) {
		extern "C" { fn cv_WWidgetMerger_delete(instance: *mut c_void); }
		unsafe { cv_WWidgetMerger_delete(self.as_raw_mut_WWidgetMerger()) };
	}
}

unsafe impl Send for WWidgetMerger {}

impl crate::viz::WidgetTraitConst for WWidgetMerger {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WWidgetMerger {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WWidgetMerger {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WWidgetMerger {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WWidgetMergerTraitConst for WWidgetMerger {
	#[inline] fn as_raw_WWidgetMerger(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WWidgetMergerTrait for WWidgetMerger {
	#[inline] fn as_raw_mut_WWidgetMerger(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WWidgetMerger {
	// WWidgetMerger() /usr/include/opencv2/viz/widgets.hpp:808
	#[inline]
	pub fn default() -> Result<crate::viz::WWidgetMerger> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_WWidgetMerger_WWidgetMerger(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::WWidgetMerger::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { WWidgetMerger, crate::viz::Widget, cv_WWidgetMerger_to_Widget }

boxed_cast_base! { WWidgetMerger, crate::viz::Widget3D, cv_WWidgetMerger_to_Widget3D }

// Widget /usr/include/opencv2/viz/widgets.hpp:92
pub trait WidgetTraitConst {
	fn as_raw_Widget(&self) -> *const c_void;

	// getRenderingProperty(int) /usr/include/opencv2/viz/widgets.hpp:157
	#[inline]
	fn get_rendering_property(&self, property: i32) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Widget_getRenderingProperty_const_int(self.as_raw_Widget(), property, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait WidgetTrait: crate::viz::WidgetTraitConst {
	fn as_raw_mut_Widget(&mut self) -> *mut c_void;

	// setRenderingProperty(int, double) /usr/include/opencv2/viz/widgets.hpp:131
	#[inline]
	fn set_rendering_property(&mut self, property: i32, value: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Widget_setRenderingProperty_int_double(self.as_raw_mut_Widget(), property, value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Widget /usr/include/opencv2/viz/widgets.hpp:92
pub struct Widget {
	ptr: *mut c_void
}

opencv_type_boxed! { Widget }

impl Drop for Widget {
	fn drop(&mut self) {
		extern "C" { fn cv_Widget_delete(instance: *mut c_void); }
		unsafe { cv_Widget_delete(self.as_raw_mut_Widget()) };
	}
}

unsafe impl Send for Widget {}

impl crate::viz::WidgetTraitConst for Widget {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for Widget {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Widget {
	// Widget() /usr/include/opencv2/viz/widgets.hpp:95
	#[inline]
	pub fn default() -> Result<crate::viz::Widget> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Widget_Widget(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Widget::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Widget(const cv::viz::Widget &) /usr/include/opencv2/viz/widgets.hpp:96
	#[inline]
	pub fn copy(other: &crate::viz::Widget) -> Result<crate::viz::Widget> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Widget_Widget_const_WidgetR(other.as_raw_Widget(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Widget::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// fromPlyFile(const cv::String &) /usr/include/opencv2/viz/widgets.hpp:104
	#[inline]
	pub fn from_ply_file(file_name: &str) -> Result<crate::viz::Widget> {
		extern_container_arg!(file_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Widget_fromPlyFile_const_StringR(file_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Widget::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// Widget2D /usr/include/opencv2/viz/widgets.hpp:218
pub trait Widget2DTraitConst: crate::viz::WidgetTraitConst {
	fn as_raw_Widget2D(&self) -> *const c_void;

}

pub trait Widget2DTrait: crate::viz::Widget2DTraitConst + crate::viz::WidgetTrait {
	fn as_raw_mut_Widget2D(&mut self) -> *mut c_void;

	// setColor(const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:227
	#[inline]
	fn set_color(&mut self, color: &crate::viz::Color) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Widget2D_setColor_const_ColorR(self.as_raw_mut_Widget2D(), color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Widget2D /usr/include/opencv2/viz/widgets.hpp:218
pub struct Widget2D {
	ptr: *mut c_void
}

opencv_type_boxed! { Widget2D }

impl Drop for Widget2D {
	fn drop(&mut self) {
		extern "C" { fn cv_Widget2D_delete(instance: *mut c_void); }
		unsafe { cv_Widget2D_delete(self.as_raw_mut_Widget2D()) };
	}
}

unsafe impl Send for Widget2D {}

impl crate::viz::WidgetTraitConst for Widget2D {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for Widget2D {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget2DTraitConst for Widget2D {
	#[inline] fn as_raw_Widget2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget2DTrait for Widget2D {
	#[inline] fn as_raw_mut_Widget2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Widget2D {
	// Widget2D() /usr/include/opencv2/viz/widgets.hpp:221
	#[inline]
	pub fn default() -> Result<crate::viz::Widget2D> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Widget2D_Widget2D(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Widget2D::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { Widget2D, crate::viz::Widget, cv_Widget2D_to_Widget }

// Widget3D /usr/include/opencv2/viz/widgets.hpp:181
pub trait Widget3DTraitConst: crate::viz::WidgetTraitConst {
	fn as_raw_Widget3D(&self) -> *const c_void;

	// getPose() /usr/include/opencv2/viz/widgets.hpp:198
	#[inline]
	fn get_pose(&self) -> Result<core::Affine3d> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Widget3D_getPose_const(self.as_raw_Widget3D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait Widget3DTrait: crate::viz::Widget3DTraitConst + crate::viz::WidgetTrait {
	fn as_raw_mut_Widget3D(&mut self) -> *mut c_void;

	// setPose(const cv::Affine3d &) /usr/include/opencv2/viz/widgets.hpp:190
	#[inline]
	fn set_pose(&mut self, pose: core::Affine3d) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Widget3D_setPose_const_Affine3dR(self.as_raw_mut_Widget3D(), &pose, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// updatePose(const cv::Affine3d &) /usr/include/opencv2/viz/widgets.hpp:195
	#[inline]
	fn update_pose(&mut self, pose: core::Affine3d) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Widget3D_updatePose_const_Affine3dR(self.as_raw_mut_Widget3D(), &pose, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// applyTransform(const cv::Affine3d &) /usr/include/opencv2/viz/widgets.hpp:204
	#[inline]
	fn apply_transform(&mut self, transform: core::Affine3d) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Widget3D_applyTransform_const_Affine3dR(self.as_raw_mut_Widget3D(), &transform, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setColor(const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:210
	#[inline]
	fn set_color(&mut self, color: &crate::viz::Color) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Widget3D_setColor_const_ColorR(self.as_raw_mut_Widget3D(), color.as_raw_Color(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Widget3D /usr/include/opencv2/viz/widgets.hpp:181
pub struct Widget3D {
	ptr: *mut c_void
}

opencv_type_boxed! { Widget3D }

impl Drop for Widget3D {
	fn drop(&mut self) {
		extern "C" { fn cv_Widget3D_delete(instance: *mut c_void); }
		unsafe { cv_Widget3D_delete(self.as_raw_mut_Widget3D()) };
	}
}

unsafe impl Send for Widget3D {}

impl crate::viz::WidgetTraitConst for Widget3D {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for Widget3D {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for Widget3D {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for Widget3D {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Widget3D {
	// Widget3D() /usr/include/opencv2/viz/widgets.hpp:184
	#[inline]
	pub fn default() -> Result<crate::viz::Widget3D> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_viz_Widget3D_Widget3D(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::viz::Widget3D::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { Widget3D, crate::viz::Widget, cv_Widget3D_to_Widget }
