extern "C" {
	cv::Ptr<cv::rgbd::ICPOdometry>* cv_PtrOfICPOdometry_new(cv::rgbd::ICPOdometry* val) {
		return new cv::Ptr<cv::rgbd::ICPOdometry>(val);
	}
	
	void cv_PtrOfICPOdometry_delete(cv::Ptr<cv::rgbd::ICPOdometry>* instance) {
		delete instance;
	}

	const cv::rgbd::ICPOdometry* cv_PtrOfICPOdometry_get_inner_ptr(const cv::Ptr<cv::rgbd::ICPOdometry>* instance) {
		return instance->get();
	}

	cv::rgbd::ICPOdometry* cv_PtrOfICPOdometry_get_inner_ptr_mut(cv::Ptr<cv::rgbd::ICPOdometry>* instance) {
		return instance->get();
	}
}

