extern "C" {
	cv::Ptr<cv::rgbd::FastICPOdometry>* cv_PtrOfFastICPOdometry_new(cv::rgbd::FastICPOdometry* val) {
		return new cv::Ptr<cv::rgbd::FastICPOdometry>(val);
	}
	
	void cv_PtrOfFastICPOdometry_delete(cv::Ptr<cv::rgbd::FastICPOdometry>* instance) {
		delete instance;
	}

	const cv::rgbd::FastICPOdometry* cv_PtrOfFastICPOdometry_get_inner_ptr(const cv::Ptr<cv::rgbd::FastICPOdometry>* instance) {
		return instance->get();
	}

	cv::rgbd::FastICPOdometry* cv_PtrOfFastICPOdometry_get_inner_ptr_mut(cv::Ptr<cv::rgbd::FastICPOdometry>* instance) {
		return instance->get();
	}
}

