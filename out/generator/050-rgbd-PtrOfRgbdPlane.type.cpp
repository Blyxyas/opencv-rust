extern "C" {
	cv::Ptr<cv::rgbd::RgbdPlane>* cv_PtrOfRgbdPlane_new(cv::rgbd::RgbdPlane* val) {
		return new cv::Ptr<cv::rgbd::RgbdPlane>(val);
	}
	
	void cv_PtrOfRgbdPlane_delete(cv::Ptr<cv::rgbd::RgbdPlane>* instance) {
		delete instance;
	}

	const cv::rgbd::RgbdPlane* cv_PtrOfRgbdPlane_get_inner_ptr(const cv::Ptr<cv::rgbd::RgbdPlane>* instance) {
		return instance->get();
	}

	cv::rgbd::RgbdPlane* cv_PtrOfRgbdPlane_get_inner_ptr_mut(cv::Ptr<cv::rgbd::RgbdPlane>* instance) {
		return instance->get();
	}
}

