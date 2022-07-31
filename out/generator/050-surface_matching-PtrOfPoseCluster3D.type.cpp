extern "C" {
	cv::Ptr<cv::ppf_match_3d::PoseCluster3D>* cv_PtrOfPoseCluster3D_new(cv::ppf_match_3d::PoseCluster3D* val) {
		return new cv::Ptr<cv::ppf_match_3d::PoseCluster3D>(val);
	}
	
	void cv_PtrOfPoseCluster3D_delete(cv::Ptr<cv::ppf_match_3d::PoseCluster3D>* instance) {
		delete instance;
	}

	const cv::ppf_match_3d::PoseCluster3D* cv_PtrOfPoseCluster3D_get_inner_ptr(const cv::Ptr<cv::ppf_match_3d::PoseCluster3D>* instance) {
		return instance->get();
	}

	cv::ppf_match_3d::PoseCluster3D* cv_PtrOfPoseCluster3D_get_inner_ptr_mut(cv::Ptr<cv::ppf_match_3d::PoseCluster3D>* instance) {
		return instance->get();
	}
}

