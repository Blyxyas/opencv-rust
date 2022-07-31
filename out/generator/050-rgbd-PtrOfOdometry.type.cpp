extern "C" {
	void cv_PtrOfOdometry_delete(cv::Ptr<cv::rgbd::Odometry>* instance) {
		delete instance;
	}

	const cv::rgbd::Odometry* cv_PtrOfOdometry_get_inner_ptr(const cv::Ptr<cv::rgbd::Odometry>* instance) {
		return instance->get();
	}

	cv::rgbd::Odometry* cv_PtrOfOdometry_get_inner_ptr_mut(cv::Ptr<cv::rgbd::Odometry>* instance) {
		return instance->get();
	}
}

