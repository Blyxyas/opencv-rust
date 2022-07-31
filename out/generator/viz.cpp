#include "ocvrs_common.hpp"
#include <opencv2/viz.hpp>
#include "viz_types.hpp"

extern "C" {
	// computeNormals(const cv::viz::Mesh &, cv::OutputArray) /usr/include/opencv2/viz/vizcore.hpp:214
	void cv_viz_computeNormals_const_MeshR_const__OutputArrayR(const cv::viz::Mesh* mesh, const cv::_OutputArray* normals, Result_void* ocvrs_return) {
		try {
			cv::viz::computeNormals(*mesh, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getWindowByName(const cv::String &) /usr/include/opencv2/viz/vizcore.hpp:97
	void cv_viz_getWindowByName_const_StringR(const char* window_name, Result<cv::viz::Viz3d*>* ocvrs_return) {
		try {
			cv::viz::Viz3d ret = cv::viz::getWindowByName(std::string(window_name));
			Ok(new cv::viz::Viz3d(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Viz3d*>))
	}
	
	// imshow(const cv::String &, cv::InputArray, const cv::Size &) /usr/include/opencv2/viz/vizcore.hpp:103
	void cv_viz_imshow_const_StringR_const__InputArrayR_const_SizeR(const char* window_name, const cv::_InputArray* image, const cv::Size* window_size, Result<cv::viz::Viz3d*>* ocvrs_return) {
		try {
			cv::viz::Viz3d ret = cv::viz::imshow(std::string(window_name), *image, *window_size);
			Ok(new cv::viz::Viz3d(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Viz3d*>))
	}
	
	// makeCameraPose(const cv::Vec3d &, const cv::Vec3d &, const cv::Vec3d &) /usr/include/opencv2/viz/vizcore.hpp:84
	void cv_viz_makeCameraPose_const_Vec3dR_const_Vec3dR_const_Vec3dR(const cv::Vec3d* position, const cv::Vec3d* focal_point, const cv::Vec3d* y_dir, Result<cv::Affine3d>* ocvrs_return) {
		try {
			cv::Affine3d ret = cv::viz::makeCameraPose(*position, *focal_point, *y_dir);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Affine3d>))
	}
	
	// makeTransformToGlobal(const cv::Vec3d &, const cv::Vec3d &, const cv::Vec3d &, const cv::Vec3d &) /usr/include/opencv2/viz/vizcore.hpp:73
	void cv_viz_makeTransformToGlobal_const_Vec3dR_const_Vec3dR_const_Vec3dR_const_Vec3dR(const cv::Vec3d* axis_x, const cv::Vec3d* axis_y, const cv::Vec3d* axis_z, const cv::Vec3d* origin, Result<cv::Affine3d>* ocvrs_return) {
		try {
			cv::Affine3d ret = cv::viz::makeTransformToGlobal(*axis_x, *axis_y, *axis_z, *origin);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Affine3d>))
	}
	
	// readCloud(const cv::String &, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/viz/vizcore.hpp:160
	void cv_viz_readCloud_const_StringR_const__OutputArrayR_const__OutputArrayR(const char* file, const cv::_OutputArray* colors, const cv::_OutputArray* normals, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::viz::readCloud(std::string(file), *colors, *normals);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// readMesh(const cv::String &) /usr/include/opencv2/viz/vizcore.hpp:165
	void cv_viz_readMesh_const_StringR(const char* file, Result<cv::viz::Mesh*>* ocvrs_return) {
		try {
			cv::viz::Mesh ret = cv::viz::readMesh(std::string(file));
			Ok(new cv::viz::Mesh(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Mesh*>))
	}
	
	// readPose(const cv::String &, cv::Affine3d &, const cv::String &) /usr/include/opencv2/viz/vizcore.hpp:175
	void cv_viz_readPose_const_StringR_Affine3dR_const_StringR(const char* file, cv::Affine3d* pose, const char* tag, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::viz::readPose(std::string(file), *pose, std::string(tag));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// readTrajectory(cv::OutputArray, const cv::String &, int, int, const cv::String &) /usr/include/opencv2/viz/vizcore.hpp:206
	void cv_viz_readTrajectory_const__OutputArrayR_const_StringR_int_int_const_StringR(const cv::_OutputArray* traj, const char* files_format, int start, int end, const char* tag, Result_void* ocvrs_return) {
		try {
			cv::viz::readTrajectory(*traj, std::string(files_format), start, end, std::string(tag));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// unregisterAllWindows() /usr/include/opencv2/viz/vizcore.hpp:100
	void cv_viz_unregisterAllWindows(Result_void* ocvrs_return) {
		try {
			cv::viz::unregisterAllWindows();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// writeCloud(const cv::String &, cv::InputArray, cv::InputArray, cv::InputArray, bool) /usr/include/opencv2/viz/vizcore.hpp:151
	void cv_viz_writeCloud_const_StringR_const__InputArrayR_const__InputArrayR_const__InputArrayR_bool(const char* file, const cv::_InputArray* cloud, const cv::_InputArray* colors, const cv::_InputArray* normals, bool binary, Result_void* ocvrs_return) {
		try {
			cv::viz::writeCloud(std::string(file), *cloud, *colors, *normals, binary);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// writePose(const cv::String &, const cv::Affine3d &, const cv::String &) /usr/include/opencv2/viz/vizcore.hpp:181
	void cv_viz_writePose_const_StringR_const_Affine3dR_const_StringR(const char* file, const cv::Affine3d* pose, const char* tag, Result_void* ocvrs_return) {
		try {
			cv::viz::writePose(std::string(file), *pose, std::string(tag));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// writeTrajectory(cv::InputArray, const cv::String &, int, const cv::String &) /usr/include/opencv2/viz/vizcore.hpp:193
	void cv_viz_writeTrajectory_const__InputArrayR_const_StringR_int_const_StringR(const cv::_InputArray* traj, const char* files_format, int start, const char* tag, Result_void* ocvrs_return) {
		try {
			cv::viz::writeTrajectory(*traj, std::string(files_format), start, std::string(tag));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Camera_delete(cv::viz::Camera* instance) {
		delete instance;
	}
	// Camera(double, double, double, double, const cv::Size &) /usr/include/opencv2/viz/types.hpp:176
	void cv_viz_Camera_Camera_double_double_double_double_const_SizeR(double fx, double fy, double cx, double cy, const cv::Size* window_size, Result<cv::viz::Camera*>* ocvrs_return) {
		try {
			cv::viz::Camera* ret = new cv::viz::Camera(fx, fy, cx, cy, *window_size);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Camera*>))
	}
	
	// Camera(const cv::Vec2d &, const cv::Size &) /usr/include/opencv2/viz/types.hpp:183
	void cv_viz_Camera_Camera_const_Vec2dR_const_SizeR(const cv::Vec2d* fov, const cv::Size* window_size, Result<cv::viz::Camera*>* ocvrs_return) {
		try {
			cv::viz::Camera* ret = new cv::viz::Camera(*fov, *window_size);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Camera*>))
	}
	
	// Camera(const cv::Matx33d &, const cv::Size &) /usr/include/opencv2/viz/types.hpp:197
	void cv_viz_Camera_Camera_const_Matx33dR_const_SizeR(const cv::Matx33d* K, const cv::Size* window_size, Result<cv::viz::Camera*>* ocvrs_return) {
		try {
			cv::viz::Camera* ret = new cv::viz::Camera(*K, *window_size);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Camera*>))
	}
	
	// Camera(const cv::Matx44d &, const cv::Size &) /usr/include/opencv2/viz/types.hpp:213
	void cv_viz_Camera_Camera_const_Matx44dR_const_SizeR(const cv::Matx44d* proj, const cv::Size* window_size, Result<cv::viz::Camera*>* ocvrs_return) {
		try {
			cv::viz::Camera* ret = new cv::viz::Camera(*proj, *window_size);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Camera*>))
	}
	
	// getClip() /usr/include/opencv2/viz/types.hpp:215
	void cv_viz_Camera_getClip_const(const cv::viz::Camera* instance, Result<cv::Vec2d>* ocvrs_return) {
		try {
			const cv::Vec2d ret = instance->getClip();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec2d>))
	}
	
	// setClip(const cv::Vec2d &) /usr/include/opencv2/viz/types.hpp:216
	void cv_viz_Camera_setClip_const_Vec2dR(cv::viz::Camera* instance, const cv::Vec2d* clip, Result_void* ocvrs_return) {
		try {
			instance->setClip(*clip);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getWindowSize() /usr/include/opencv2/viz/types.hpp:218
	void cv_viz_Camera_getWindowSize_const(const cv::viz::Camera* instance, Result<cv::Size>* ocvrs_return) {
		try {
			const cv::Size ret = instance->getWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// setWindowSize(const cv::Size &) /usr/include/opencv2/viz/types.hpp:219
	void cv_viz_Camera_setWindowSize_const_SizeR(cv::viz::Camera* instance, const cv::Size* window_size, Result_void* ocvrs_return) {
		try {
			instance->setWindowSize(*window_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getFov() /usr/include/opencv2/viz/types.hpp:221
	void cv_viz_Camera_getFov_const(const cv::viz::Camera* instance, Result<cv::Vec2d>* ocvrs_return) {
		try {
			const cv::Vec2d ret = instance->getFov();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec2d>))
	}
	
	// setFov(const cv::Vec2d &) /usr/include/opencv2/viz/types.hpp:222
	void cv_viz_Camera_setFov_const_Vec2dR(cv::viz::Camera* instance, const cv::Vec2d* fov, Result_void* ocvrs_return) {
		try {
			instance->setFov(*fov);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getPrincipalPoint() /usr/include/opencv2/viz/types.hpp:224
	void cv_viz_Camera_getPrincipalPoint_const(const cv::viz::Camera* instance, Result<cv::Vec2d>* ocvrs_return) {
		try {
			const cv::Vec2d ret = instance->getPrincipalPoint();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec2d>))
	}
	
	// getFocalLength() /usr/include/opencv2/viz/types.hpp:225
	void cv_viz_Camera_getFocalLength_const(const cv::viz::Camera* instance, Result<cv::Vec2d>* ocvrs_return) {
		try {
			const cv::Vec2d ret = instance->getFocalLength();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec2d>))
	}
	
	// computeProjectionMatrix(cv::Matx44d &) /usr/include/opencv2/viz/types.hpp:240
	void cv_viz_Camera_computeProjectionMatrix_const_Matx44dR(const cv::viz::Camera* instance, cv::Matx44d* proj, Result_void* ocvrs_return) {
		try {
			instance->computeProjectionMatrix(*proj);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// KinectCamera(const cv::Size &) /usr/include/opencv2/viz/types.hpp:250
	void cv_viz_Camera_KinectCamera_const_SizeR(const cv::Size* window_size, Result<cv::viz::Camera*>* ocvrs_return) {
		try {
			cv::viz::Camera ret = cv::viz::Camera::KinectCamera(*window_size);
			Ok(new cv::viz::Camera(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Camera*>))
	}
	
	void cv_Color_delete(cv::viz::Color* instance) {
		delete instance;
	}
	// Color() /usr/include/opencv2/viz/types.hpp:66
	void cv_viz_Color_Color(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color* ret = new cv::viz::Color();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// Color(double) /usr/include/opencv2/viz/types.hpp:68
	void cv_viz_Color_Color_double(double gray, Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color* ret = new cv::viz::Color(gray);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// Color(double, double, double) /usr/include/opencv2/viz/types.hpp:69
	void cv_viz_Color_Color_double_double_double(double blue, double green, double red, Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color* ret = new cv::viz::Color(blue, green, red);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// Color(const cv::Scalar &) /usr/include/opencv2/viz/types.hpp:71
	void cv_viz_Color_Color_const_ScalarR(const cv::Scalar* color, Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color* ret = new cv::viz::Color(*color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// operator Vec() /usr/include/opencv2/viz/types.hpp:73
	void cv_viz_Color_operator_cv_Vec3b_const(const cv::viz::Color* instance, Result<cv::Vec3b>* ocvrs_return) {
		try {
			cv::Vec3b ret = instance->operator cv::Vec3b();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec3b>))
	}
	
	// black() /usr/include/opencv2/viz/types.hpp:75
	void cv_viz_Color_black(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::black();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// blue() /usr/include/opencv2/viz/types.hpp:76
	void cv_viz_Color_blue(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::blue();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// green() /usr/include/opencv2/viz/types.hpp:77
	void cv_viz_Color_green(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::green();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// red() /usr/include/opencv2/viz/types.hpp:78
	void cv_viz_Color_red(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::red();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// cyan() /usr/include/opencv2/viz/types.hpp:79
	void cv_viz_Color_cyan(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::cyan();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// yellow() /usr/include/opencv2/viz/types.hpp:80
	void cv_viz_Color_yellow(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::yellow();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// magenta() /usr/include/opencv2/viz/types.hpp:81
	void cv_viz_Color_magenta(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::magenta();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// white() /usr/include/opencv2/viz/types.hpp:82
	void cv_viz_Color_white(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::white();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// gray() /usr/include/opencv2/viz/types.hpp:84
	void cv_viz_Color_gray(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::gray();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// silver() /usr/include/opencv2/viz/types.hpp:85
	void cv_viz_Color_silver(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::silver();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// mlab() /usr/include/opencv2/viz/types.hpp:87
	void cv_viz_Color_mlab(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::mlab();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// navy() /usr/include/opencv2/viz/types.hpp:89
	void cv_viz_Color_navy(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::navy();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// maroon() /usr/include/opencv2/viz/types.hpp:90
	void cv_viz_Color_maroon(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::maroon();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// teal() /usr/include/opencv2/viz/types.hpp:91
	void cv_viz_Color_teal(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::teal();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// olive() /usr/include/opencv2/viz/types.hpp:92
	void cv_viz_Color_olive(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::olive();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// purple() /usr/include/opencv2/viz/types.hpp:93
	void cv_viz_Color_purple(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::purple();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// azure() /usr/include/opencv2/viz/types.hpp:94
	void cv_viz_Color_azure(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::azure();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// chartreuse() /usr/include/opencv2/viz/types.hpp:95
	void cv_viz_Color_chartreuse(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::chartreuse();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// rose() /usr/include/opencv2/viz/types.hpp:96
	void cv_viz_Color_rose(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::rose();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// lime() /usr/include/opencv2/viz/types.hpp:98
	void cv_viz_Color_lime(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::lime();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// gold() /usr/include/opencv2/viz/types.hpp:99
	void cv_viz_Color_gold(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::gold();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// orange() /usr/include/opencv2/viz/types.hpp:100
	void cv_viz_Color_orange(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::orange();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// orange_red() /usr/include/opencv2/viz/types.hpp:101
	void cv_viz_Color_orange_red(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::orange_red();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// indigo() /usr/include/opencv2/viz/types.hpp:102
	void cv_viz_Color_indigo(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::indigo();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// brown() /usr/include/opencv2/viz/types.hpp:104
	void cv_viz_Color_brown(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::brown();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// apricot() /usr/include/opencv2/viz/types.hpp:105
	void cv_viz_Color_apricot(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::apricot();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// pink() /usr/include/opencv2/viz/types.hpp:106
	void cv_viz_Color_pink(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::pink();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// raspberry() /usr/include/opencv2/viz/types.hpp:107
	void cv_viz_Color_raspberry(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::raspberry();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// cherry() /usr/include/opencv2/viz/types.hpp:108
	void cv_viz_Color_cherry(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::cherry();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// violet() /usr/include/opencv2/viz/types.hpp:109
	void cv_viz_Color_violet(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::violet();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// amethyst() /usr/include/opencv2/viz/types.hpp:110
	void cv_viz_Color_amethyst(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::amethyst();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// bluberry() /usr/include/opencv2/viz/types.hpp:111
	void cv_viz_Color_bluberry(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::bluberry();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// celestial_blue() /usr/include/opencv2/viz/types.hpp:112
	void cv_viz_Color_celestial_blue(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::celestial_blue();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// turquoise() /usr/include/opencv2/viz/types.hpp:113
	void cv_viz_Color_turquoise(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::turquoise();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// not_set() /usr/include/opencv2/viz/types.hpp:115
	void cv_viz_Color_not_set(Result<cv::viz::Color*>* ocvrs_return) {
		try {
			cv::viz::Color ret = cv::viz::Color::not_set();
			Ok(new cv::viz::Color(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Color*>))
	}
	
	// action /usr/include/opencv2/viz/types.hpp:302
	void cv_viz_KeyboardEvent_getPropAction_const(const cv::viz::KeyboardEvent* instance, cv::viz::KeyboardEvent::Action* ocvrs_return) {
			cv::viz::KeyboardEvent::Action ret = instance->action;
			*ocvrs_return = ret;
	}
	
	// action /usr/include/opencv2/viz/types.hpp:302
	void cv_viz_KeyboardEvent_setPropAction_Action(cv::viz::KeyboardEvent* instance, cv::viz::KeyboardEvent::Action val) {
			instance->action = val;
	}
	
	// symbol /usr/include/opencv2/viz/types.hpp:303
	void* cv_viz_KeyboardEvent_getPropSymbol_const(const cv::viz::KeyboardEvent* instance) {
			cv::String ret = instance->symbol;
			return ocvrs_create_string(ret.c_str());
	}
	
	// symbol /usr/include/opencv2/viz/types.hpp:303
	void cv_viz_KeyboardEvent_setPropSymbol_String(cv::viz::KeyboardEvent* instance, char* val) {
			instance->symbol = std::string(val);
	}
	
	// code /usr/include/opencv2/viz/types.hpp:304
	unsigned char cv_viz_KeyboardEvent_getPropCode_const(const cv::viz::KeyboardEvent* instance) {
			unsigned char ret = instance->code;
			return ret;
	}
	
	// code /usr/include/opencv2/viz/types.hpp:304
	void cv_viz_KeyboardEvent_setPropCode_unsigned_char(cv::viz::KeyboardEvent* instance, unsigned char val) {
			instance->code = val;
	}
	
	// modifiers /usr/include/opencv2/viz/types.hpp:305
	int cv_viz_KeyboardEvent_getPropModifiers_const(const cv::viz::KeyboardEvent* instance) {
			int ret = instance->modifiers;
			return ret;
	}
	
	// modifiers /usr/include/opencv2/viz/types.hpp:305
	void cv_viz_KeyboardEvent_setPropModifiers_int(cv::viz::KeyboardEvent* instance, int val) {
			instance->modifiers = val;
	}
	
	void cv_KeyboardEvent_delete(cv::viz::KeyboardEvent* instance) {
		delete instance;
	}
	// KeyboardEvent(cv::viz::KeyboardEvent::Action, const cv::String &, unsigned char, int) /usr/include/opencv2/viz/types.hpp:300
	void cv_viz_KeyboardEvent_KeyboardEvent_Action_const_StringR_unsigned_char_int(cv::viz::KeyboardEvent::Action action, const char* symbol, unsigned char code, int modifiers, Result<cv::viz::KeyboardEvent*>* ocvrs_return) {
		try {
			cv::viz::KeyboardEvent* ret = new cv::viz::KeyboardEvent(action, std::string(symbol), code, modifiers);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::KeyboardEvent*>))
	}
	
	// cloud /usr/include/opencv2/viz/types.hpp:129
	cv::Mat* cv_viz_Mesh_getPropCloud_const(const cv::viz::Mesh* instance) {
			cv::Mat ret = instance->cloud;
			return new cv::Mat(ret);
	}
	
	// cloud /usr/include/opencv2/viz/types.hpp:129
	void cv_viz_Mesh_setPropCloud_Mat(cv::viz::Mesh* instance, cv::Mat* val) {
			instance->cloud = *val;
	}
	
	// colors /usr/include/opencv2/viz/types.hpp:130
	cv::Mat* cv_viz_Mesh_getPropColors_const(const cv::viz::Mesh* instance) {
			cv::Mat ret = instance->colors;
			return new cv::Mat(ret);
	}
	
	// colors /usr/include/opencv2/viz/types.hpp:130
	void cv_viz_Mesh_setPropColors_Mat(cv::viz::Mesh* instance, cv::Mat* val) {
			instance->colors = *val;
	}
	
	// normals /usr/include/opencv2/viz/types.hpp:131
	cv::Mat* cv_viz_Mesh_getPropNormals_const(const cv::viz::Mesh* instance) {
			cv::Mat ret = instance->normals;
			return new cv::Mat(ret);
	}
	
	// normals /usr/include/opencv2/viz/types.hpp:131
	void cv_viz_Mesh_setPropNormals_Mat(cv::viz::Mesh* instance, cv::Mat* val) {
			instance->normals = *val;
	}
	
	// polygons /usr/include/opencv2/viz/types.hpp:135
	cv::Mat* cv_viz_Mesh_getPropPolygons_const(const cv::viz::Mesh* instance) {
			cv::Mat ret = instance->polygons;
			return new cv::Mat(ret);
	}
	
	// polygons /usr/include/opencv2/viz/types.hpp:135
	void cv_viz_Mesh_setPropPolygons_Mat(cv::viz::Mesh* instance, cv::Mat* val) {
			instance->polygons = *val;
	}
	
	// texture /usr/include/opencv2/viz/types.hpp:137
	cv::Mat* cv_viz_Mesh_getPropTexture_const(const cv::viz::Mesh* instance) {
			cv::Mat ret = instance->texture;
			return new cv::Mat(ret);
	}
	
	// texture /usr/include/opencv2/viz/types.hpp:137
	void cv_viz_Mesh_setPropTexture_Mat(cv::viz::Mesh* instance, cv::Mat* val) {
			instance->texture = *val;
	}
	
	// tcoords /usr/include/opencv2/viz/types.hpp:138
	cv::Mat* cv_viz_Mesh_getPropTcoords_const(const cv::viz::Mesh* instance) {
			cv::Mat ret = instance->tcoords;
			return new cv::Mat(ret);
	}
	
	// tcoords /usr/include/opencv2/viz/types.hpp:138
	void cv_viz_Mesh_setPropTcoords_Mat(cv::viz::Mesh* instance, cv::Mat* val) {
			instance->tcoords = *val;
	}
	
	void cv_Mesh_delete(cv::viz::Mesh* instance) {
		delete instance;
	}
	// Mesh() /usr/include/opencv2/viz/types.hpp:140
	void cv_viz_Mesh_Mesh(Result<cv::viz::Mesh*>* ocvrs_return) {
		try {
			cv::viz::Mesh* ret = new cv::viz::Mesh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Mesh*>))
	}
	
	// load(const cv::String &, int) /usr/include/opencv2/viz/types.hpp:154
	void cv_viz_Mesh_load_const_StringR_int(const char* file, int type, Result<cv::viz::Mesh*>* ocvrs_return) {
		try {
			cv::viz::Mesh ret = cv::viz::Mesh::load(std::string(file), type);
			Ok(new cv::viz::Mesh(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Mesh*>))
	}
	
	// type /usr/include/opencv2/viz/types.hpp:327
	void cv_viz_MouseEvent_getPropType_const(const cv::viz::MouseEvent* instance, cv::viz::MouseEvent::Type* ocvrs_return) {
			cv::viz::MouseEvent::Type ret = instance->type;
			*ocvrs_return = ret;
	}
	
	// type /usr/include/opencv2/viz/types.hpp:327
	void cv_viz_MouseEvent_setPropType_Type(cv::viz::MouseEvent* instance, cv::viz::MouseEvent::Type val) {
			instance->type = val;
	}
	
	// button /usr/include/opencv2/viz/types.hpp:328
	void cv_viz_MouseEvent_getPropButton_const(const cv::viz::MouseEvent* instance, cv::viz::MouseEvent::MouseButton* ocvrs_return) {
			cv::viz::MouseEvent::MouseButton ret = instance->button;
			*ocvrs_return = ret;
	}
	
	// button /usr/include/opencv2/viz/types.hpp:328
	void cv_viz_MouseEvent_setPropButton_MouseButton(cv::viz::MouseEvent* instance, cv::viz::MouseEvent::MouseButton val) {
			instance->button = val;
	}
	
	// pointer /usr/include/opencv2/viz/types.hpp:329
	void cv_viz_MouseEvent_getPropPointer_const(const cv::viz::MouseEvent* instance, cv::Point* ocvrs_return) {
			cv::Point ret = instance->pointer;
			*ocvrs_return = ret;
	}
	
	// pointer /usr/include/opencv2/viz/types.hpp:329
	void cv_viz_MouseEvent_setPropPointer_Point(cv::viz::MouseEvent* instance, cv::Point* val) {
			instance->pointer = *val;
	}
	
	// modifiers /usr/include/opencv2/viz/types.hpp:330
	int cv_viz_MouseEvent_getPropModifiers_const(const cv::viz::MouseEvent* instance) {
			int ret = instance->modifiers;
			return ret;
	}
	
	// modifiers /usr/include/opencv2/viz/types.hpp:330
	void cv_viz_MouseEvent_setPropModifiers_int(cv::viz::MouseEvent* instance, int val) {
			instance->modifiers = val;
	}
	
	void cv_MouseEvent_delete(cv::viz::MouseEvent* instance) {
		delete instance;
	}
	// MouseEvent(const cv::viz::MouseEvent::Type &, const cv::viz::MouseEvent::MouseButton &, const cv::Point &, int) /usr/include/opencv2/viz/types.hpp:325
	void cv_viz_MouseEvent_MouseEvent_const_TypeR_const_MouseButtonR_const_PointR_int(const cv::viz::MouseEvent::Type* type, const cv::viz::MouseEvent::MouseButton* button, const cv::Point* pointer, int modifiers, Result<cv::viz::MouseEvent*>* ocvrs_return) {
		try {
			cv::viz::MouseEvent* ret = new cv::viz::MouseEvent(*type, *button, *pointer, modifiers);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::MouseEvent*>))
	}
	
	void cv_Viz3d_delete(cv::viz::Viz3d* instance) {
		delete instance;
	}
	// Viz3d(const cv::String &) /usr/include/opencv2/viz/viz3d.hpp:78
	void cv_viz_Viz3d_Viz3d_const_StringR(const char* window_name, Result<cv::viz::Viz3d*>* ocvrs_return) {
		try {
			cv::viz::Viz3d* ret = new cv::viz::Viz3d(std::string(window_name));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Viz3d*>))
	}
	
	// Viz3d(const cv::viz::Viz3d &) /usr/include/opencv2/viz/viz3d.hpp:79
	void cv_viz_Viz3d_Viz3d_const_Viz3dR(const cv::viz::Viz3d* unnamed, Result<cv::viz::Viz3d*>* ocvrs_return) {
		try {
			cv::viz::Viz3d* ret = new cv::viz::Viz3d(*unnamed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Viz3d*>))
	}
	
	// showWidget(const cv::String &, const cv::viz::Widget &, const cv::Affine3d &) /usr/include/opencv2/viz/viz3d.hpp:88
	void cv_viz_Viz3d_showWidget_const_StringR_const_WidgetR_const_Affine3dR(cv::viz::Viz3d* instance, const char* id, const cv::viz::Widget* widget, const cv::Affine3d* pose, Result_void* ocvrs_return) {
		try {
			instance->showWidget(std::string(id), *widget, *pose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// removeWidget(const cv::String &) /usr/include/opencv2/viz/viz3d.hpp:94
	void cv_viz_Viz3d_removeWidget_const_StringR(cv::viz::Viz3d* instance, const char* id, Result_void* ocvrs_return) {
		try {
			instance->removeWidget(std::string(id));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getWidget(const cv::String &) /usr/include/opencv2/viz/viz3d.hpp:103
	void cv_viz_Viz3d_getWidget_const_const_StringR(const cv::viz::Viz3d* instance, const char* id, Result<cv::viz::Widget*>* ocvrs_return) {
		try {
			cv::viz::Widget ret = instance->getWidget(std::string(id));
			Ok(new cv::viz::Widget(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Widget*>))
	}
	
	// removeAllWidgets() /usr/include/opencv2/viz/viz3d.hpp:107
	void cv_viz_Viz3d_removeAllWidgets(cv::viz::Viz3d* instance, Result_void* ocvrs_return) {
		try {
			instance->removeAllWidgets();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// showImage(cv::InputArray, const cv::Size &) /usr/include/opencv2/viz/viz3d.hpp:114
	void cv_viz_Viz3d_showImage_const__InputArrayR_const_SizeR(cv::viz::Viz3d* instance, const cv::_InputArray* image, const cv::Size* window_size, Result_void* ocvrs_return) {
		try {
			instance->showImage(*image, *window_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setWidgetPose(const cv::String &, const cv::Affine3d &) /usr/include/opencv2/viz/viz3d.hpp:120
	void cv_viz_Viz3d_setWidgetPose_const_StringR_const_Affine3dR(cv::viz::Viz3d* instance, const char* id, const cv::Affine3d* pose, Result_void* ocvrs_return) {
		try {
			instance->setWidgetPose(std::string(id), *pose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// updateWidgetPose(const cv::String &, const cv::Affine3d &) /usr/include/opencv2/viz/viz3d.hpp:127
	void cv_viz_Viz3d_updateWidgetPose_const_StringR_const_Affine3dR(cv::viz::Viz3d* instance, const char* id, const cv::Affine3d* pose, Result_void* ocvrs_return) {
		try {
			instance->updateWidgetPose(std::string(id), *pose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getWidgetPose(const cv::String &) /usr/include/opencv2/viz/viz3d.hpp:133
	void cv_viz_Viz3d_getWidgetPose_const_const_StringR(const cv::viz::Viz3d* instance, const char* id, Result<cv::Affine3d>* ocvrs_return) {
		try {
			cv::Affine3d ret = instance->getWidgetPose(std::string(id));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Affine3d>))
	}
	
	// setCamera(const cv::viz::Camera &) /usr/include/opencv2/viz/viz3d.hpp:139
	void cv_viz_Viz3d_setCamera_const_CameraR(cv::viz::Viz3d* instance, const cv::viz::Camera* camera, Result_void* ocvrs_return) {
		try {
			instance->setCamera(*camera);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getCamera() /usr/include/opencv2/viz/viz3d.hpp:143
	void cv_viz_Viz3d_getCamera_const(const cv::viz::Viz3d* instance, Result<cv::viz::Camera*>* ocvrs_return) {
		try {
			cv::viz::Camera ret = instance->getCamera();
			Ok(new cv::viz::Camera(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Camera*>))
	}
	
	// getViewerPose() /usr/include/opencv2/viz/viz3d.hpp:147
	void cv_viz_Viz3d_getViewerPose_const(const cv::viz::Viz3d* instance, Result<cv::Affine3d>* ocvrs_return) {
		try {
			cv::Affine3d ret = instance->getViewerPose();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Affine3d>))
	}
	
	// setViewerPose(const cv::Affine3d &) /usr/include/opencv2/viz/viz3d.hpp:153
	void cv_viz_Viz3d_setViewerPose_const_Affine3dR(cv::viz::Viz3d* instance, const cv::Affine3d* pose, Result_void* ocvrs_return) {
		try {
			instance->setViewerPose(*pose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// resetCameraViewpoint(const cv::String &) /usr/include/opencv2/viz/viz3d.hpp:159
	void cv_viz_Viz3d_resetCameraViewpoint_const_StringR(cv::viz::Viz3d* instance, const char* id, Result_void* ocvrs_return) {
		try {
			instance->resetCameraViewpoint(std::string(id));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// resetCamera() /usr/include/opencv2/viz/viz3d.hpp:163
	void cv_viz_Viz3d_resetCamera(cv::viz::Viz3d* instance, Result_void* ocvrs_return) {
		try {
			instance->resetCamera();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// convertToWindowCoordinates(const cv::Point3d &, cv::Point3d &) /usr/include/opencv2/viz/viz3d.hpp:170
	void cv_viz_Viz3d_convertToWindowCoordinates_const_Point3dR_Point3dR(cv::viz::Viz3d* instance, const cv::Point3d* pt, cv::Point3d* window_coord, Result_void* ocvrs_return) {
		try {
			instance->convertToWindowCoordinates(*pt, *window_coord);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// converTo3DRay(const cv::Point3d &, cv::Point3d &, cv::Vec3d &) /usr/include/opencv2/viz/viz3d.hpp:177
	void cv_viz_Viz3d_converTo3DRay_const_Point3dR_Point3dR_Vec3dR(cv::viz::Viz3d* instance, const cv::Point3d* window_coord, cv::Point3d* origin, cv::Vec3d* direction, Result_void* ocvrs_return) {
		try {
			instance->converTo3DRay(*window_coord, *origin, *direction);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getWindowSize() /usr/include/opencv2/viz/viz3d.hpp:181
	void cv_viz_Viz3d_getWindowSize_const(const cv::viz::Viz3d* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// setWindowSize(const cv::Size &) /usr/include/opencv2/viz/viz3d.hpp:186
	void cv_viz_Viz3d_setWindowSize_const_SizeR(cv::viz::Viz3d* instance, const cv::Size* window_size, Result_void* ocvrs_return) {
		try {
			instance->setWindowSize(*window_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getWindowName() /usr/include/opencv2/viz/viz3d.hpp:191
	void cv_viz_Viz3d_getWindowName_const(const cv::viz::Viz3d* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getWindowName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// getScreenshot() /usr/include/opencv2/viz/viz3d.hpp:195
	void cv_viz_Viz3d_getScreenshot_const(const cv::viz::Viz3d* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getScreenshot();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// saveScreenshot(const cv::String &) /usr/include/opencv2/viz/viz3d.hpp:201
	void cv_viz_Viz3d_saveScreenshot_const_StringR(cv::viz::Viz3d* instance, const char* file, Result_void* ocvrs_return) {
		try {
			instance->saveScreenshot(std::string(file));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setWindowPosition(const cv::Point &) /usr/include/opencv2/viz/viz3d.hpp:207
	void cv_viz_Viz3d_setWindowPosition_const_PointR(cv::viz::Viz3d* instance, const cv::Point* window_position, Result_void* ocvrs_return) {
		try {
			instance->setWindowPosition(*window_position);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setFullScreen(bool) /usr/include/opencv2/viz/viz3d.hpp:213
	void cv_viz_Viz3d_setFullScreen_bool(cv::viz::Viz3d* instance, bool mode, Result_void* ocvrs_return) {
		try {
			instance->setFullScreen(mode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setBackgroundColor(const cv::viz::Viz3d::Color &, const cv::viz::Viz3d::Color &) /usr/include/opencv2/viz/viz3d.hpp:217
	void cv_viz_Viz3d_setBackgroundColor_const_ColorR_const_ColorR(cv::viz::Viz3d* instance, const cv::viz::Viz3d::Color* color, const cv::viz::Viz3d::Color* color2, Result_void* ocvrs_return) {
		try {
			instance->setBackgroundColor(*color, *color2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setBackgroundTexture(cv::InputArray) /usr/include/opencv2/viz/viz3d.hpp:218
	void cv_viz_Viz3d_setBackgroundTexture_const__InputArrayR(cv::viz::Viz3d* instance, const cv::_InputArray* image, Result_void* ocvrs_return) {
		try {
			instance->setBackgroundTexture(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setBackgroundMeshLab() /usr/include/opencv2/viz/viz3d.hpp:219
	void cv_viz_Viz3d_setBackgroundMeshLab(cv::viz::Viz3d* instance, Result_void* ocvrs_return) {
		try {
			instance->setBackgroundMeshLab();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// spin() /usr/include/opencv2/viz/viz3d.hpp:223
	void cv_viz_Viz3d_spin(cv::viz::Viz3d* instance, Result_void* ocvrs_return) {
		try {
			instance->spin();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// spinOnce(int, bool) /usr/include/opencv2/viz/viz3d.hpp:230
	void cv_viz_Viz3d_spinOnce_int_bool(cv::viz::Viz3d* instance, int time, bool force_redraw, Result_void* ocvrs_return) {
		try {
			instance->spinOnce(time, force_redraw);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setOffScreenRendering() /usr/include/opencv2/viz/viz3d.hpp:234
	void cv_viz_Viz3d_setOffScreenRendering(cv::viz::Viz3d* instance, Result_void* ocvrs_return) {
		try {
			instance->setOffScreenRendering();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// removeAllLights() /usr/include/opencv2/viz/viz3d.hpp:238
	void cv_viz_Viz3d_removeAllLights(cv::viz::Viz3d* instance, Result_void* ocvrs_return) {
		try {
			instance->removeAllLights();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// addLight(const cv::Vec3d &, const cv::Vec3d &, const cv::viz::Viz3d::Color &, const cv::viz::Viz3d::Color &, const cv::viz::Viz3d::Color &, const cv::viz::Viz3d::Color &) /usr/include/opencv2/viz/viz3d.hpp:249
	void cv_viz_Viz3d_addLight_const_Vec3dR_const_Vec3dR_const_ColorR_const_ColorR_const_ColorR_const_ColorR(cv::viz::Viz3d* instance, const cv::Vec3d* position, const cv::Vec3d* focalPoint, const cv::viz::Viz3d::Color* color, const cv::viz::Viz3d::Color* diffuseColor, const cv::viz::Viz3d::Color* ambientColor, const cv::viz::Viz3d::Color* specularColor, Result_void* ocvrs_return) {
		try {
			instance->addLight(*position, *focalPoint, *color, *diffuseColor, *ambientColor, *specularColor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// wasStopped() /usr/include/opencv2/viz/viz3d.hpp:255
	void cv_viz_Viz3d_wasStopped_const(const cv::viz::Viz3d* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->wasStopped();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// close() /usr/include/opencv2/viz/viz3d.hpp:256
	void cv_viz_Viz3d_close(cv::viz::Viz3d* instance, Result_void* ocvrs_return) {
		try {
			instance->close();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// registerKeyboardCallback(cv::viz::Viz3d::KeyboardCallback, void *) /usr/include/opencv2/viz/viz3d.hpp:264
	void cv_viz_Viz3d_registerKeyboardCallback_KeyboardCallback_voidX(cv::viz::Viz3d* instance, cv::viz::Viz3d::KeyboardCallback callback, void* cookie, Result_void* ocvrs_return) {
		try {
			instance->registerKeyboardCallback(callback, cookie);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// registerMouseCallback(cv::viz::Viz3d::MouseCallback, void *) /usr/include/opencv2/viz/viz3d.hpp:271
	void cv_viz_Viz3d_registerMouseCallback_MouseCallback_voidX(cv::viz::Viz3d* instance, cv::viz::Viz3d::MouseCallback callback, void* cookie, Result_void* ocvrs_return) {
		try {
			instance->registerMouseCallback(callback, cookie);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setRenderingProperty(const cv::String &, int, double) /usr/include/opencv2/viz/viz3d.hpp:299
	void cv_viz_Viz3d_setRenderingProperty_const_StringR_int_double(cv::viz::Viz3d* instance, const char* id, int property, double value, Result_void* ocvrs_return) {
		try {
			instance->setRenderingProperty(std::string(id), property, value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getRenderingProperty(const cv::String &, int) /usr/include/opencv2/viz/viz3d.hpp:325
	void cv_viz_Viz3d_getRenderingProperty_const_StringR_int(cv::viz::Viz3d* instance, const char* id, int property, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getRenderingProperty(std::string(id), property);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setRepresentation(int) /usr/include/opencv2/viz/viz3d.hpp:334
	void cv_viz_Viz3d_setRepresentation_int(cv::viz::Viz3d* instance, int representation, Result_void* ocvrs_return) {
		try {
			instance->setRepresentation(representation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setGlobalWarnings(bool) /usr/include/opencv2/viz/viz3d.hpp:336
	void cv_viz_Viz3d_setGlobalWarnings_bool(cv::viz::Viz3d* instance, bool enabled, Result_void* ocvrs_return) {
		try {
			instance->setGlobalWarnings(enabled);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	cv::viz::Widget* cv_WArrow_to_Widget(cv::viz::WArrow* instance) {
		return dynamic_cast<cv::viz::Widget*>(instance);
	}
	
	cv::viz::Widget3D* cv_WArrow_to_Widget3D(cv::viz::WArrow* instance) {
		return dynamic_cast<cv::viz::Widget3D*>(instance);
	}
	
	void cv_WArrow_delete(cv::viz::WArrow* instance) {
		delete instance;
	}
	// WArrow(const cv::Point3d &, const cv::Point3d &, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:301
	void cv_viz_WArrow_WArrow_const_Point3dR_const_Point3dR_double_const_ColorR(const cv::Point3d* pt1, const cv::Point3d* pt2, double thickness, const cv::viz::Color* color, Result<cv::viz::WArrow*>* ocvrs_return) {
		try {
			cv::viz::WArrow* ret = new cv::viz::WArrow(*pt1, *pt2, thickness, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WArrow*>))
	}
	
	cv::viz::Widget* cv_WCameraPosition_to_Widget(cv::viz::WCameraPosition* instance) {
		return dynamic_cast<cv::viz::Widget*>(instance);
	}
	
	cv::viz::Widget3D* cv_WCameraPosition_to_Widget3D(cv::viz::WCameraPosition* instance) {
		return dynamic_cast<cv::viz::Widget3D*>(instance);
	}
	
	void cv_WCameraPosition_delete(cv::viz::WCameraPosition* instance) {
		delete instance;
	}
	// WCameraPosition(double) /usr/include/opencv2/viz/widgets.hpp:550
	void cv_viz_WCameraPosition_WCameraPosition_double(double scale, Result<cv::viz::WCameraPosition*>* ocvrs_return) {
		try {
			cv::viz::WCameraPosition* ret = new cv::viz::WCameraPosition(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCameraPosition*>))
	}
	
	// WCameraPosition(const cv::Matx33d &, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:560
	void cv_viz_WCameraPosition_WCameraPosition_const_Matx33dR_double_const_ColorR(const cv::Matx33d* K, double scale, const cv::viz::Color* color, Result<cv::viz::WCameraPosition*>* ocvrs_return) {
		try {
			cv::viz::WCameraPosition* ret = new cv::viz::WCameraPosition(*K, scale, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCameraPosition*>))
	}
	
	// WCameraPosition(const cv::Vec2d &, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:570
	void cv_viz_WCameraPosition_WCameraPosition_const_Vec2dR_double_const_ColorR(const cv::Vec2d* fov, double scale, const cv::viz::Color* color, Result<cv::viz::WCameraPosition*>* ocvrs_return) {
		try {
			cv::viz::WCameraPosition* ret = new cv::viz::WCameraPosition(*fov, scale, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCameraPosition*>))
	}
	
	// WCameraPosition(const cv::Matx33d &, cv::InputArray, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:583
	void cv_viz_WCameraPosition_WCameraPosition_const_Matx33dR_const__InputArrayR_double_const_ColorR(const cv::Matx33d* K, const cv::_InputArray* image, double scale, const cv::viz::Color* color, Result<cv::viz::WCameraPosition*>* ocvrs_return) {
		try {
			cv::viz::WCameraPosition* ret = new cv::viz::WCameraPosition(*K, *image, scale, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCameraPosition*>))
	}
	
	// WCameraPosition(const cv::Vec2d &, cv::InputArray, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:596
	void cv_viz_WCameraPosition_WCameraPosition_const_Vec2dR_const__InputArrayR_double_const_ColorR(const cv::Vec2d* fov, const cv::_InputArray* image, double scale, const cv::viz::Color* color, Result<cv::viz::WCameraPosition*>* ocvrs_return) {
		try {
			cv::viz::WCameraPosition* ret = new cv::viz::WCameraPosition(*fov, *image, scale, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCameraPosition*>))
	}
	
	cv::viz::Widget* cv_WCircle_to_Widget(cv::viz::WCircle* instance) {
		return dynamic_cast<cv::viz::Widget*>(instance);
	}
	
	cv::viz::Widget3D* cv_WCircle_to_Widget3D(cv::viz::WCircle* instance) {
		return dynamic_cast<cv::viz::Widget3D*>(instance);
	}
	
	void cv_WCircle_delete(cv::viz::WCircle* instance) {
		delete instance;
	}
	// WCircle(double, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:315
	void cv_viz_WCircle_WCircle_double_double_const_ColorR(double radius, double thickness, const cv::viz::Color* color, Result<cv::viz::WCircle*>* ocvrs_return) {
		try {
			cv::viz::WCircle* ret = new cv::viz::WCircle(radius, thickness, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCircle*>))
	}
	
	// WCircle(double, const cv::Point3d &, const cv::Vec3d &, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:325
	void cv_viz_WCircle_WCircle_double_const_Point3dR_const_Vec3dR_double_const_ColorR(double radius, const cv::Point3d* center, const cv::Vec3d* normal, double thickness, const cv::viz::Color* color, Result<cv::viz::WCircle*>* ocvrs_return) {
		try {
			cv::viz::WCircle* ret = new cv::viz::WCircle(radius, *center, *normal, thickness, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCircle*>))
	}
	
	cv::viz::Widget* cv_WCloud_to_Widget(cv::viz::WCloud* instance) {
		return dynamic_cast<cv::viz::Widget*>(instance);
	}
	
	cv::viz::Widget3D* cv_WCloud_to_Widget3D(cv::viz::WCloud* instance) {
		return dynamic_cast<cv::viz::Widget3D*>(instance);
	}
	
	void cv_WCloud_delete(cv::viz::WCloud* instance) {
		delete instance;
	}
	// WCloud(cv::InputArray, cv::InputArray) /usr/include/opencv2/viz/widgets.hpp:690
	void cv_viz_WCloud_WCloud_const__InputArrayR_const__InputArrayR(const cv::_InputArray* cloud, const cv::_InputArray* colors, Result<cv::viz::WCloud*>* ocvrs_return) {
		try {
			cv::viz::WCloud* ret = new cv::viz::WCloud(*cloud, *colors);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCloud*>))
	}
	
	// WCloud(cv::InputArray, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:698
	void cv_viz_WCloud_WCloud_const__InputArrayR_const_ColorR(const cv::_InputArray* cloud, const cv::viz::Color* color, Result<cv::viz::WCloud*>* ocvrs_return) {
		try {
			cv::viz::WCloud* ret = new cv::viz::WCloud(*cloud, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCloud*>))
	}
	
	// WCloud(cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/viz/widgets.hpp:707
	void cv_viz_WCloud_WCloud_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* cloud, const cv::_InputArray* colors, const cv::_InputArray* normals, Result<cv::viz::WCloud*>* ocvrs_return) {
		try {
			cv::viz::WCloud* ret = new cv::viz::WCloud(*cloud, *colors, *normals);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCloud*>))
	}
	
	// WCloud(cv::InputArray, const cv::viz::Color &, cv::InputArray) /usr/include/opencv2/viz/widgets.hpp:717
	void cv_viz_WCloud_WCloud_const__InputArrayR_const_ColorR_const__InputArrayR(const cv::_InputArray* cloud, const cv::viz::Color* color, const cv::_InputArray* normals, Result<cv::viz::WCloud*>* ocvrs_return) {
		try {
			cv::viz::WCloud* ret = new cv::viz::WCloud(*cloud, *color, *normals);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCloud*>))
	}
	
	cv::viz::Widget* cv_WCloudCollection_to_Widget(cv::viz::WCloudCollection* instance) {
		return dynamic_cast<cv::viz::Widget*>(instance);
	}
	
	cv::viz::Widget3D* cv_WCloudCollection_to_Widget3D(cv::viz::WCloudCollection* instance) {
		return dynamic_cast<cv::viz::Widget3D*>(instance);
	}
	
	void cv_WCloudCollection_delete(cv::viz::WCloudCollection* instance) {
		delete instance;
	}
	// WCloudCollection() /usr/include/opencv2/viz/widgets.hpp:739
	void cv_viz_WCloudCollection_WCloudCollection(Result<cv::viz::WCloudCollection*>* ocvrs_return) {
		try {
			cv::viz::WCloudCollection* ret = new cv::viz::WCloudCollection();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCloudCollection*>))
	}
	
	// addCloud(cv::InputArray, cv::InputArray, const cv::Affine3d &) /usr/include/opencv2/viz/widgets.hpp:747
	void cv_viz_WCloudCollection_addCloud_const__InputArrayR_const__InputArrayR_const_Affine3dR(cv::viz::WCloudCollection* instance, const cv::_InputArray* cloud, const cv::_InputArray* colors, const cv::Affine3d* pose, Result_void* ocvrs_return) {
		try {
			instance->addCloud(*cloud, *colors, *pose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// addCloud(cv::InputArray, const cv::viz::Color &, const cv::Affine3d &) /usr/include/opencv2/viz/widgets.hpp:754
	void cv_viz_WCloudCollection_addCloud_const__InputArrayR_const_ColorR_const_Affine3dR(cv::viz::WCloudCollection* instance, const cv::_InputArray* cloud, const cv::viz::Color* color, const cv::Affine3d* pose, Result_void* ocvrs_return) {
		try {
			instance->addCloud(*cloud, *color, *pose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// finalize() /usr/include/opencv2/viz/widgets.hpp:759
	void cv_viz_WCloudCollection_finalize(cv::viz::WCloudCollection* instance, Result_void* ocvrs_return) {
		try {
			instance->finalize();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	cv::viz::Widget* cv_WCloudNormals_to_Widget(cv::viz::WCloudNormals* instance) {
		return dynamic_cast<cv::viz::Widget*>(instance);
	}
	
	cv::viz::Widget3D* cv_WCloudNormals_to_Widget3D(cv::viz::WCloudNormals* instance) {
		return dynamic_cast<cv::viz::Widget3D*>(instance);
	}
	
	void cv_WCloudNormals_delete(cv::viz::WCloudNormals* instance) {
		delete instance;
	}
	// WCloudNormals(cv::InputArray, cv::InputArray, int, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:777
	void cv_viz_WCloudNormals_WCloudNormals_const__InputArrayR_const__InputArrayR_int_double_const_ColorR(const cv::_InputArray* cloud, const cv::_InputArray* normals, int level, double scale, const cv::viz::Color* color, Result<cv::viz::WCloudNormals*>* ocvrs_return) {
		try {
			cv::viz::WCloudNormals* ret = new cv::viz::WCloudNormals(*cloud, *normals, level, scale, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCloudNormals*>))
	}
	
	cv::viz::Widget* cv_WCone_to_Widget(cv::viz::WCone* instance) {
		return dynamic_cast<cv::viz::Widget*>(instance);
	}
	
	cv::viz::Widget3D* cv_WCone_to_Widget3D(cv::viz::WCone* instance) {
		return dynamic_cast<cv::viz::Widget3D*>(instance);
	}
	
	void cv_WCone_delete(cv::viz::WCone* instance) {
		delete instance;
	}
	// WCone(double, double, int, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:340
	void cv_viz_WCone_WCone_double_double_int_const_ColorR(double length, double radius, int resolution, const cv::viz::Color* color, Result<cv::viz::WCone*>* ocvrs_return) {
		try {
			cv::viz::WCone* ret = new cv::viz::WCone(length, radius, resolution, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCone*>))
	}
	
	// WCone(double, const cv::Point3d &, const cv::Point3d &, int, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:351
	void cv_viz_WCone_WCone_double_const_Point3dR_const_Point3dR_int_const_ColorR(double radius, const cv::Point3d* center, const cv::Point3d* tip, int resolution, const cv::viz::Color* color, Result<cv::viz::WCone*>* ocvrs_return) {
		try {
			cv::viz::WCone* ret = new cv::viz::WCone(radius, *center, *tip, resolution, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCone*>))
	}
	
	cv::viz::Widget* cv_WCoordinateSystem_to_Widget(cv::viz::WCoordinateSystem* instance) {
		return dynamic_cast<cv::viz::Widget*>(instance);
	}
	
	cv::viz::Widget3D* cv_WCoordinateSystem_to_Widget3D(cv::viz::WCoordinateSystem* instance) {
		return dynamic_cast<cv::viz::Widget3D*>(instance);
	}
	
	void cv_WCoordinateSystem_delete(cv::viz::WCoordinateSystem* instance) {
		delete instance;
	}
	// WCoordinateSystem(double) /usr/include/opencv2/viz/widgets.hpp:520
	void cv_viz_WCoordinateSystem_WCoordinateSystem_double(double scale, Result<cv::viz::WCoordinateSystem*>* ocvrs_return) {
		try {
			cv::viz::WCoordinateSystem* ret = new cv::viz::WCoordinateSystem(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCoordinateSystem*>))
	}
	
	cv::viz::Widget* cv_WCube_to_Widget(cv::viz::WCube* instance) {
		return dynamic_cast<cv::viz::Widget*>(instance);
	}
	
	cv::viz::Widget3D* cv_WCube_to_Widget3D(cv::viz::WCube* instance) {
		return dynamic_cast<cv::viz::Widget3D*>(instance);
	}
	
	void cv_WCube_delete(cv::viz::WCube* instance) {
		delete instance;
	}
	// WCube(const cv::Point3d &, const cv::Point3d &, bool, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:384
	void cv_viz_WCube_WCube_const_Point3dR_const_Point3dR_bool_const_ColorR(const cv::Point3d* min_point, const cv::Point3d* max_point, bool wire_frame, const cv::viz::Color* color, Result<cv::viz::WCube*>* ocvrs_return) {
		try {
			cv::viz::WCube* ret = new cv::viz::WCube(*min_point, *max_point, wire_frame, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCube*>))
	}
	
	cv::viz::Widget* cv_WCylinder_to_Widget(cv::viz::WCylinder* instance) {
		return dynamic_cast<cv::viz::Widget*>(instance);
	}
	
	cv::viz::Widget3D* cv_WCylinder_to_Widget3D(cv::viz::WCylinder* instance) {
		return dynamic_cast<cv::viz::Widget3D*>(instance);
	}
	
	void cv_WCylinder_delete(cv::viz::WCylinder* instance) {
		delete instance;
	}
	// WCylinder(const cv::Point3d &, const cv::Point3d &, double, int, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:367
	void cv_viz_WCylinder_WCylinder_const_Point3dR_const_Point3dR_double_int_const_ColorR(const cv::Point3d* axis_point1, const cv::Point3d* axis_point2, double radius, int numsides, const cv::viz::Color* color, Result<cv::viz::WCylinder*>* ocvrs_return) {
		try {
			cv::viz::WCylinder* ret = new cv::viz::WCylinder(*axis_point1, *axis_point2, radius, numsides, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WCylinder*>))
	}
	
	cv::viz::Widget* cv_WGrid_to_Widget(cv::viz::WGrid* instance) {
		return dynamic_cast<cv::viz::Widget*>(instance);
	}
	
	cv::viz::Widget3D* cv_WGrid_to_Widget3D(cv::viz::WGrid* instance) {
		return dynamic_cast<cv::viz::Widget3D*>(instance);
	}
	
	void cv_WGrid_delete(cv::viz::WGrid* instance) {
		delete instance;
	}
	// WGrid(const cv::Vec2i &, const cv::Vec2d &, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:534
	void cv_viz_WGrid_WGrid_const_Vec2iR_const_Vec2dR_const_ColorR(const cv::Vec2i* cells, const cv::Vec2d* cells_spacing, const cv::viz::Color* color, Result<cv::viz::WGrid*>* ocvrs_return) {
		try {
			cv::viz::WGrid* ret = new cv::viz::WGrid(*cells, *cells_spacing, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WGrid*>))
	}
	
	// WGrid(const cv::Point3d &, const cv::Vec3d &, const cv::Vec3d &, const cv::Vec2i &, const cv::Vec2d &, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:537
	void cv_viz_WGrid_WGrid_const_Point3dR_const_Vec3dR_const_Vec3dR_const_Vec2iR_const_Vec2dR_const_ColorR(const cv::Point3d* center, const cv::Vec3d* normal, const cv::Vec3d* new_yaxis, const cv::Vec2i* cells, const cv::Vec2d* cells_spacing, const cv::viz::Color* color, Result<cv::viz::WGrid*>* ocvrs_return) {
		try {
			cv::viz::WGrid* ret = new cv::viz::WGrid(*center, *normal, *new_yaxis, *cells, *cells_spacing, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WGrid*>))
	}
	
	cv::viz::Widget* cv_WImage3D_to_Widget(cv::viz::WImage3D* instance) {
		return dynamic_cast<cv::viz::Widget*>(instance);
	}
	
	cv::viz::Widget3D* cv_WImage3D_to_Widget3D(cv::viz::WImage3D* instance) {
		return dynamic_cast<cv::viz::Widget3D*>(instance);
	}
	
	void cv_WImage3D_delete(cv::viz::WImage3D* instance) {
		delete instance;
	}
	// WImage3D(cv::InputArray, const cv::Size2d &) /usr/include/opencv2/viz/widgets.hpp:483
	void cv_viz_WImage3D_WImage3D_const__InputArrayR_const_Size2dR(const cv::_InputArray* image, const cv::Size2d* size, Result<cv::viz::WImage3D*>* ocvrs_return) {
		try {
			cv::viz::WImage3D* ret = new cv::viz::WImage3D(*image, *size);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WImage3D*>))
	}
	
	// WImage3D(cv::InputArray, const cv::Size2d &, const cv::Vec3d &, const cv::Vec3d &, const cv::Vec3d &) /usr/include/opencv2/viz/widgets.hpp:493
	void cv_viz_WImage3D_WImage3D_const__InputArrayR_const_Size2dR_const_Vec3dR_const_Vec3dR_const_Vec3dR(const cv::_InputArray* image, const cv::Size2d* size, const cv::Vec3d* center, const cv::Vec3d* normal, const cv::Vec3d* up_vector, Result<cv::viz::WImage3D*>* ocvrs_return) {
		try {
			cv::viz::WImage3D* ret = new cv::viz::WImage3D(*image, *size, *center, *normal, *up_vector);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WImage3D*>))
	}
	
	// setImage(cv::InputArray) /usr/include/opencv2/viz/widgets.hpp:499
	void cv_viz_WImage3D_setImage_const__InputArrayR(cv::viz::WImage3D* instance, const cv::_InputArray* image, Result_void* ocvrs_return) {
		try {
			instance->setImage(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setSize(const cv::Size &) /usr/include/opencv2/viz/widgets.hpp:505
	void cv_viz_WImage3D_setSize_const_SizeR(cv::viz::WImage3D* instance, const cv::Size* size, Result_void* ocvrs_return) {
		try {
			instance->setSize(*size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	cv::viz::Widget* cv_WImageOverlay_to_Widget(cv::viz::WImageOverlay* instance) {
		return dynamic_cast<cv::viz::Widget*>(instance);
	}
	
	cv::viz::Widget2D* cv_WImageOverlay_to_Widget2D(cv::viz::WImageOverlay* instance) {
		return dynamic_cast<cv::viz::Widget2D*>(instance);
	}
	
	void cv_WImageOverlay_delete(cv::viz::WImageOverlay* instance) {
		delete instance;
	}
	// WImageOverlay(cv::InputArray, const cv::Rect &) /usr/include/opencv2/viz/widgets.hpp:465
	void cv_viz_WImageOverlay_WImageOverlay_const__InputArrayR_const_RectR(const cv::_InputArray* image, const cv::Rect* rect, Result<cv::viz::WImageOverlay*>* ocvrs_return) {
		try {
			cv::viz::WImageOverlay* ret = new cv::viz::WImageOverlay(*image, *rect);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WImageOverlay*>))
	}
	
	// setImage(cv::InputArray) /usr/include/opencv2/viz/widgets.hpp:470
	void cv_viz_WImageOverlay_setImage_const__InputArrayR(cv::viz::WImageOverlay* instance, const cv::_InputArray* image, Result_void* ocvrs_return) {
		try {
			instance->setImage(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	cv::viz::Widget* cv_WLine_to_Widget(cv::viz::WLine* instance) {
		return dynamic_cast<cv::viz::Widget*>(instance);
	}
	
	cv::viz::Widget3D* cv_WLine_to_Widget3D(cv::viz::WLine* instance) {
		return dynamic_cast<cv::viz::Widget3D*>(instance);
	}
	
	void cv_WLine_delete(cv::viz::WLine* instance) {
		delete instance;
	}
	// WLine(const cv::Point3d &, const cv::Point3d &, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:244
	void cv_viz_WLine_WLine_const_Point3dR_const_Point3dR_const_ColorR(const cv::Point3d* pt1, const cv::Point3d* pt2, const cv::viz::Color* color, Result<cv::viz::WLine*>* ocvrs_return) {
		try {
			cv::viz::WLine* ret = new cv::viz::WLine(*pt1, *pt2, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WLine*>))
	}
	
	cv::viz::Widget* cv_WMesh_to_Widget(cv::viz::WMesh* instance) {
		return dynamic_cast<cv::viz::Widget*>(instance);
	}
	
	cv::viz::Widget3D* cv_WMesh_to_Widget3D(cv::viz::WMesh* instance) {
		return dynamic_cast<cv::viz::Widget3D*>(instance);
	}
	
	void cv_WMesh_delete(cv::viz::WMesh* instance) {
		delete instance;
	}
	// WMesh(const cv::viz::Mesh &) /usr/include/opencv2/viz/widgets.hpp:794
	void cv_viz_WMesh_WMesh_const_MeshR(const cv::viz::Mesh* mesh, Result<cv::viz::WMesh*>* ocvrs_return) {
		try {
			cv::viz::WMesh* ret = new cv::viz::WMesh(*mesh);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WMesh*>))
	}
	
	// WMesh(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/viz/widgets.hpp:795
	void cv_viz_WMesh_WMesh_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* cloud, const cv::_InputArray* polygons, const cv::_InputArray* colors, const cv::_InputArray* normals, Result<cv::viz::WMesh*>* ocvrs_return) {
		try {
			cv::viz::WMesh* ret = new cv::viz::WMesh(*cloud, *polygons, *colors, *normals);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WMesh*>))
	}
	
	cv::viz::Widget* cv_WPaintedCloud_to_Widget(cv::viz::WPaintedCloud* instance) {
		return dynamic_cast<cv::viz::Widget*>(instance);
	}
	
	cv::viz::Widget3D* cv_WPaintedCloud_to_Widget3D(cv::viz::WPaintedCloud* instance) {
		return dynamic_cast<cv::viz::Widget3D*>(instance);
	}
	
	void cv_WPaintedCloud_delete(cv::viz::WPaintedCloud* instance) {
		delete instance;
	}
	// WPaintedCloud(cv::InputArray) /usr/include/opencv2/viz/widgets.hpp:724
	void cv_viz_WPaintedCloud_WPaintedCloud_const__InputArrayR(const cv::_InputArray* cloud, Result<cv::viz::WPaintedCloud*>* ocvrs_return) {
		try {
			cv::viz::WPaintedCloud* ret = new cv::viz::WPaintedCloud(*cloud);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WPaintedCloud*>))
	}
	
	// WPaintedCloud(cv::InputArray, const cv::Point3d &, const cv::Point3d &) /usr/include/opencv2/viz/widgets.hpp:727
	void cv_viz_WPaintedCloud_WPaintedCloud_const__InputArrayR_const_Point3dR_const_Point3dR(const cv::_InputArray* cloud, const cv::Point3d* p1, const cv::Point3d* p2, Result<cv::viz::WPaintedCloud*>* ocvrs_return) {
		try {
			cv::viz::WPaintedCloud* ret = new cv::viz::WPaintedCloud(*cloud, *p1, *p2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WPaintedCloud*>))
	}
	
	// WPaintedCloud(cv::InputArray, const cv::Point3d &, const cv::Point3d &, const cv::viz::Color &, const cv::viz::Color) /usr/include/opencv2/viz/widgets.hpp:730
	void cv_viz_WPaintedCloud_WPaintedCloud_const__InputArrayR_const_Point3dR_const_Point3dR_const_ColorR_const_Color(const cv::_InputArray* cloud, const cv::Point3d* p1, const cv::Point3d* p2, const cv::viz::Color* c1, const cv::viz::Color* c2, Result<cv::viz::WPaintedCloud*>* ocvrs_return) {
		try {
			cv::viz::WPaintedCloud* ret = new cv::viz::WPaintedCloud(*cloud, *p1, *p2, *c1, *c2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WPaintedCloud*>))
	}
	
	cv::viz::Widget* cv_WPlane_to_Widget(cv::viz::WPlane* instance) {
		return dynamic_cast<cv::viz::Widget*>(instance);
	}
	
	cv::viz::Widget3D* cv_WPlane_to_Widget3D(cv::viz::WPlane* instance) {
		return dynamic_cast<cv::viz::Widget3D*>(instance);
	}
	
	void cv_WPlane_delete(cv::viz::WPlane* instance) {
		delete instance;
	}
	// WPlane(const cv::Size2d &, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:257
	void cv_viz_WPlane_WPlane_const_Size2dR_const_ColorR(const cv::Size2d* size, const cv::viz::Color* color, Result<cv::viz::WPlane*>* ocvrs_return) {
		try {
			cv::viz::WPlane* ret = new cv::viz::WPlane(*size, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WPlane*>))
	}
	
	// WPlane(const cv::Point3d &, const cv::Vec3d &, const cv::Vec3d &, const cv::Size2d &, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:267
	void cv_viz_WPlane_WPlane_const_Point3dR_const_Vec3dR_const_Vec3dR_const_Size2dR_const_ColorR(const cv::Point3d* center, const cv::Vec3d* normal, const cv::Vec3d* new_yaxis, const cv::Size2d* size, const cv::viz::Color* color, Result<cv::viz::WPlane*>* ocvrs_return) {
		try {
			cv::viz::WPlane* ret = new cv::viz::WPlane(*center, *normal, *new_yaxis, *size, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WPlane*>))
	}
	
	cv::viz::Widget* cv_WPolyLine_to_Widget(cv::viz::WPolyLine* instance) {
		return dynamic_cast<cv::viz::Widget*>(instance);
	}
	
	cv::viz::Widget3D* cv_WPolyLine_to_Widget3D(cv::viz::WPolyLine* instance) {
		return dynamic_cast<cv::viz::Widget3D*>(instance);
	}
	
	void cv_WPolyLine_delete(cv::viz::WPolyLine* instance) {
		delete instance;
	}
	// WPolyLine(cv::InputArray, cv::InputArray) /usr/include/opencv2/viz/widgets.hpp:393
	void cv_viz_WPolyLine_WPolyLine_const__InputArrayR_const__InputArrayR(const cv::_InputArray* points, const cv::_InputArray* colors, Result<cv::viz::WPolyLine*>* ocvrs_return) {
		try {
			cv::viz::WPolyLine* ret = new cv::viz::WPolyLine(*points, *colors);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WPolyLine*>))
	}
	
	// WPolyLine(cv::InputArray, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:399
	void cv_viz_WPolyLine_WPolyLine_const__InputArrayR_const_ColorR(const cv::_InputArray* points, const cv::viz::Color* color, Result<cv::viz::WPolyLine*>* ocvrs_return) {
		try {
			cv::viz::WPolyLine* ret = new cv::viz::WPolyLine(*points, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WPolyLine*>))
	}
	
	cv::viz::Widget* cv_WSphere_to_Widget(cv::viz::WSphere* instance) {
		return dynamic_cast<cv::viz::Widget*>(instance);
	}
	
	cv::viz::Widget3D* cv_WSphere_to_Widget3D(cv::viz::WSphere* instance) {
		return dynamic_cast<cv::viz::Widget3D*>(instance);
	}
	
	void cv_WSphere_delete(cv::viz::WSphere* instance) {
		delete instance;
	}
	// WSphere(const cv::Point3d &, double, int, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:283
	void cv_viz_WSphere_WSphere_const_Point3dR_double_int_const_ColorR(const cv::Point3d* center, double radius, int sphere_resolution, const cv::viz::Color* color, Result<cv::viz::WSphere*>* ocvrs_return) {
		try {
			cv::viz::WSphere* ret = new cv::viz::WSphere(*center, radius, sphere_resolution, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WSphere*>))
	}
	
	cv::viz::Widget* cv_WText_to_Widget(cv::viz::WText* instance) {
		return dynamic_cast<cv::viz::Widget*>(instance);
	}
	
	cv::viz::Widget2D* cv_WText_to_Widget2D(cv::viz::WText* instance) {
		return dynamic_cast<cv::viz::Widget2D*>(instance);
	}
	
	void cv_WText_delete(cv::viz::WText* instance) {
		delete instance;
	}
	// WText(const cv::String &, const cv::Point &, int, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:417
	void cv_viz_WText_WText_const_StringR_const_PointR_int_const_ColorR(const char* text, const cv::Point* pos, int font_size, const cv::viz::Color* color, Result<cv::viz::WText*>* ocvrs_return) {
		try {
			cv::viz::WText* ret = new cv::viz::WText(std::string(text), *pos, font_size, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WText*>))
	}
	
	// setText(const cv::String &) /usr/include/opencv2/viz/widgets.hpp:423
	void cv_viz_WText_setText_const_StringR(cv::viz::WText* instance, const char* text, Result_void* ocvrs_return) {
		try {
			instance->setText(std::string(text));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getText() /usr/include/opencv2/viz/widgets.hpp:426
	void cv_viz_WText_getText_const(const cv::viz::WText* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getText();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	cv::viz::Widget* cv_WText3D_to_Widget(cv::viz::WText3D* instance) {
		return dynamic_cast<cv::viz::Widget*>(instance);
	}
	
	cv::viz::Widget3D* cv_WText3D_to_Widget3D(cv::viz::WText3D* instance) {
		return dynamic_cast<cv::viz::Widget3D*>(instance);
	}
	
	void cv_WText3D_delete(cv::viz::WText3D* instance) {
		delete instance;
	}
	// WText3D(const cv::String &, const cv::Point3d &, double, bool, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:442
	void cv_viz_WText3D_WText3D_const_StringR_const_Point3dR_double_bool_const_ColorR(const char* text, const cv::Point3d* position, double text_scale, bool face_camera, const cv::viz::Color* color, Result<cv::viz::WText3D*>* ocvrs_return) {
		try {
			cv::viz::WText3D* ret = new cv::viz::WText3D(std::string(text), *position, text_scale, face_camera, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WText3D*>))
	}
	
	// setText(const cv::String &) /usr/include/opencv2/viz/widgets.hpp:449
	void cv_viz_WText3D_setText_const_StringR(cv::viz::WText3D* instance, const char* text, Result_void* ocvrs_return) {
		try {
			instance->setText(std::string(text));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getText() /usr/include/opencv2/viz/widgets.hpp:452
	void cv_viz_WText3D_getText_const(const cv::viz::WText3D* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getText();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	cv::viz::Widget* cv_WTrajectory_to_Widget(cv::viz::WTrajectory* instance) {
		return dynamic_cast<cv::viz::Widget*>(instance);
	}
	
	cv::viz::Widget3D* cv_WTrajectory_to_Widget3D(cv::viz::WTrajectory* instance) {
		return dynamic_cast<cv::viz::Widget3D*>(instance);
	}
	
	void cv_WTrajectory_delete(cv::viz::WTrajectory* instance) {
		delete instance;
	}
	// WTrajectory(cv::InputArray, int, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:622
	void cv_viz_WTrajectory_WTrajectory_const__InputArrayR_int_double_const_ColorR(const cv::_InputArray* path, int display_mode, double scale, const cv::viz::Color* color, Result<cv::viz::WTrajectory*>* ocvrs_return) {
		try {
			cv::viz::WTrajectory* ret = new cv::viz::WTrajectory(*path, display_mode, scale, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WTrajectory*>))
	}
	
	cv::viz::Widget* cv_WTrajectoryFrustums_to_Widget(cv::viz::WTrajectoryFrustums* instance) {
		return dynamic_cast<cv::viz::Widget*>(instance);
	}
	
	cv::viz::Widget3D* cv_WTrajectoryFrustums_to_Widget3D(cv::viz::WTrajectoryFrustums* instance) {
		return dynamic_cast<cv::viz::Widget3D*>(instance);
	}
	
	void cv_WTrajectoryFrustums_delete(cv::viz::WTrajectoryFrustums* instance) {
		delete instance;
	}
	// WTrajectoryFrustums(cv::InputArray, const cv::Matx33d &, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:639
	void cv_viz_WTrajectoryFrustums_WTrajectoryFrustums_const__InputArrayR_const_Matx33dR_double_const_ColorR(const cv::_InputArray* path, const cv::Matx33d* K, double scale, const cv::viz::Color* color, Result<cv::viz::WTrajectoryFrustums*>* ocvrs_return) {
		try {
			cv::viz::WTrajectoryFrustums* ret = new cv::viz::WTrajectoryFrustums(*path, *K, scale, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WTrajectoryFrustums*>))
	}
	
	// WTrajectoryFrustums(cv::InputArray, const cv::Vec2d &, double, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:650
	void cv_viz_WTrajectoryFrustums_WTrajectoryFrustums_const__InputArrayR_const_Vec2dR_double_const_ColorR(const cv::_InputArray* path, const cv::Vec2d* fov, double scale, const cv::viz::Color* color, Result<cv::viz::WTrajectoryFrustums*>* ocvrs_return) {
		try {
			cv::viz::WTrajectoryFrustums* ret = new cv::viz::WTrajectoryFrustums(*path, *fov, scale, *color);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WTrajectoryFrustums*>))
	}
	
	cv::viz::Widget* cv_WTrajectorySpheres_to_Widget(cv::viz::WTrajectorySpheres* instance) {
		return dynamic_cast<cv::viz::Widget*>(instance);
	}
	
	cv::viz::Widget3D* cv_WTrajectorySpheres_to_Widget3D(cv::viz::WTrajectorySpheres* instance) {
		return dynamic_cast<cv::viz::Widget3D*>(instance);
	}
	
	void cv_WTrajectorySpheres_delete(cv::viz::WTrajectorySpheres* instance) {
		delete instance;
	}
	// WTrajectorySpheres(cv::InputArray, double, double, const cv::viz::Color &, const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:669
	void cv_viz_WTrajectorySpheres_WTrajectorySpheres_const__InputArrayR_double_double_const_ColorR_const_ColorR(const cv::_InputArray* path, double line_length, double radius, const cv::viz::Color* from, const cv::viz::Color* to, Result<cv::viz::WTrajectorySpheres*>* ocvrs_return) {
		try {
			cv::viz::WTrajectorySpheres* ret = new cv::viz::WTrajectorySpheres(*path, line_length, radius, *from, *to);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WTrajectorySpheres*>))
	}
	
	cv::viz::Widget* cv_WWidgetMerger_to_Widget(cv::viz::WWidgetMerger* instance) {
		return dynamic_cast<cv::viz::Widget*>(instance);
	}
	
	cv::viz::Widget3D* cv_WWidgetMerger_to_Widget3D(cv::viz::WWidgetMerger* instance) {
		return dynamic_cast<cv::viz::Widget3D*>(instance);
	}
	
	void cv_WWidgetMerger_delete(cv::viz::WWidgetMerger* instance) {
		delete instance;
	}
	// WWidgetMerger() /usr/include/opencv2/viz/widgets.hpp:808
	void cv_viz_WWidgetMerger_WWidgetMerger(Result<cv::viz::WWidgetMerger*>* ocvrs_return) {
		try {
			cv::viz::WWidgetMerger* ret = new cv::viz::WWidgetMerger();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::WWidgetMerger*>))
	}
	
	// addWidget(const cv::viz::Widget3D &, const cv::Affine3d &) /usr/include/opencv2/viz/widgets.hpp:811
	void cv_viz_WWidgetMerger_addWidget_const_Widget3DR_const_Affine3dR(cv::viz::WWidgetMerger* instance, const cv::viz::Widget3D* widget, const cv::Affine3d* pose, Result_void* ocvrs_return) {
		try {
			instance->addWidget(*widget, *pose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// finalize() /usr/include/opencv2/viz/widgets.hpp:814
	void cv_viz_WWidgetMerger_finalize(cv::viz::WWidgetMerger* instance, Result_void* ocvrs_return) {
		try {
			instance->finalize();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Widget_delete(cv::viz::Widget* instance) {
		delete instance;
	}
	// Widget() /usr/include/opencv2/viz/widgets.hpp:95
	void cv_viz_Widget_Widget(Result<cv::viz::Widget*>* ocvrs_return) {
		try {
			cv::viz::Widget* ret = new cv::viz::Widget();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Widget*>))
	}
	
	// Widget(const cv::viz::Widget &) /usr/include/opencv2/viz/widgets.hpp:96
	void cv_viz_Widget_Widget_const_WidgetR(const cv::viz::Widget* other, Result<cv::viz::Widget*>* ocvrs_return) {
		try {
			cv::viz::Widget* ret = new cv::viz::Widget(*other);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Widget*>))
	}
	
	// fromPlyFile(const cv::String &) /usr/include/opencv2/viz/widgets.hpp:104
	void cv_viz_Widget_fromPlyFile_const_StringR(const char* file_name, Result<cv::viz::Widget*>* ocvrs_return) {
		try {
			cv::viz::Widget ret = cv::viz::Widget::fromPlyFile(std::string(file_name));
			Ok(new cv::viz::Widget(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Widget*>))
	}
	
	// setRenderingProperty(int, double) /usr/include/opencv2/viz/widgets.hpp:131
	void cv_viz_Widget_setRenderingProperty_int_double(cv::viz::Widget* instance, int property, double value, Result_void* ocvrs_return) {
		try {
			instance->setRenderingProperty(property, value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getRenderingProperty(int) /usr/include/opencv2/viz/widgets.hpp:157
	void cv_viz_Widget_getRenderingProperty_const_int(const cv::viz::Widget* instance, int property, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getRenderingProperty(property);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	cv::viz::Widget* cv_Widget2D_to_Widget(cv::viz::Widget2D* instance) {
		return dynamic_cast<cv::viz::Widget*>(instance);
	}
	
	void cv_Widget2D_delete(cv::viz::Widget2D* instance) {
		delete instance;
	}
	// Widget2D() /usr/include/opencv2/viz/widgets.hpp:221
	void cv_viz_Widget2D_Widget2D(Result<cv::viz::Widget2D*>* ocvrs_return) {
		try {
			cv::viz::Widget2D* ret = new cv::viz::Widget2D();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Widget2D*>))
	}
	
	// setColor(const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:227
	void cv_viz_Widget2D_setColor_const_ColorR(cv::viz::Widget2D* instance, const cv::viz::Color* color, Result_void* ocvrs_return) {
		try {
			instance->setColor(*color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	cv::viz::Widget* cv_Widget3D_to_Widget(cv::viz::Widget3D* instance) {
		return dynamic_cast<cv::viz::Widget*>(instance);
	}
	
	void cv_Widget3D_delete(cv::viz::Widget3D* instance) {
		delete instance;
	}
	// Widget3D() /usr/include/opencv2/viz/widgets.hpp:184
	void cv_viz_Widget3D_Widget3D(Result<cv::viz::Widget3D*>* ocvrs_return) {
		try {
			cv::viz::Widget3D* ret = new cv::viz::Widget3D();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::viz::Widget3D*>))
	}
	
	// setPose(const cv::Affine3d &) /usr/include/opencv2/viz/widgets.hpp:190
	void cv_viz_Widget3D_setPose_const_Affine3dR(cv::viz::Widget3D* instance, const cv::Affine3d* pose, Result_void* ocvrs_return) {
		try {
			instance->setPose(*pose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// updatePose(const cv::Affine3d &) /usr/include/opencv2/viz/widgets.hpp:195
	void cv_viz_Widget3D_updatePose_const_Affine3dR(cv::viz::Widget3D* instance, const cv::Affine3d* pose, Result_void* ocvrs_return) {
		try {
			instance->updatePose(*pose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getPose() /usr/include/opencv2/viz/widgets.hpp:198
	void cv_viz_Widget3D_getPose_const(const cv::viz::Widget3D* instance, Result<cv::Affine3d>* ocvrs_return) {
		try {
			cv::Affine3d ret = instance->getPose();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Affine3d>))
	}
	
	// applyTransform(const cv::Affine3d &) /usr/include/opencv2/viz/widgets.hpp:204
	void cv_viz_Widget3D_applyTransform_const_Affine3dR(cv::viz::Widget3D* instance, const cv::Affine3d* transform, Result_void* ocvrs_return) {
		try {
			instance->applyTransform(*transform);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setColor(const cv::viz::Color &) /usr/include/opencv2/viz/widgets.hpp:210
	void cv_viz_Widget3D_setColor_const_ColorR(cv::viz::Widget3D* instance, const cv::viz::Color* color, Result_void* ocvrs_return) {
		try {
			instance->setColor(*color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
