extern "C" {
	cv::Ptr<cv::rgbd::RgbdICPOdometry>* cv_PtrOfRgbdICPOdometry_new(cv::rgbd::RgbdICPOdometry* val) {
		return new cv::Ptr<cv::rgbd::RgbdICPOdometry>(val);
	}
	
	void cv_PtrOfRgbdICPOdometry_delete(cv::Ptr<cv::rgbd::RgbdICPOdometry>* instance) {
		delete instance;
	}

	const cv::rgbd::RgbdICPOdometry* cv_PtrOfRgbdICPOdometry_get_inner_ptr(const cv::Ptr<cv::rgbd::RgbdICPOdometry>* instance) {
		return instance->get();
	}

	cv::rgbd::RgbdICPOdometry* cv_PtrOfRgbdICPOdometry_get_inner_ptr_mut(cv::Ptr<cv::rgbd::RgbdICPOdometry>* instance) {
		return instance->get();
	}
}

