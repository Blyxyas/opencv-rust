extern "C" {
	cv::Ptr<cv::rgbd::RgbdOdometry>* cv_PtrOfRgbdOdometry_new(cv::rgbd::RgbdOdometry* val) {
		return new cv::Ptr<cv::rgbd::RgbdOdometry>(val);
	}
	
	void cv_PtrOfRgbdOdometry_delete(cv::Ptr<cv::rgbd::RgbdOdometry>* instance) {
		delete instance;
	}

	const cv::rgbd::RgbdOdometry* cv_PtrOfRgbdOdometry_get_inner_ptr(const cv::Ptr<cv::rgbd::RgbdOdometry>* instance) {
		return instance->get();
	}

	cv::rgbd::RgbdOdometry* cv_PtrOfRgbdOdometry_get_inner_ptr_mut(cv::Ptr<cv::rgbd::RgbdOdometry>* instance) {
		return instance->get();
	}
}

