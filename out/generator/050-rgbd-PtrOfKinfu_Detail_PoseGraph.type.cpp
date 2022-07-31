extern "C" {
	void cv_PtrOfKinfu_Detail_PoseGraph_delete(cv::Ptr<cv::kinfu::detail::PoseGraph>* instance) {
		delete instance;
	}

	const cv::kinfu::detail::PoseGraph* cv_PtrOfKinfu_Detail_PoseGraph_get_inner_ptr(const cv::Ptr<cv::kinfu::detail::PoseGraph>* instance) {
		return instance->get();
	}

	cv::kinfu::detail::PoseGraph* cv_PtrOfKinfu_Detail_PoseGraph_get_inner_ptr_mut(cv::Ptr<cv::kinfu::detail::PoseGraph>* instance) {
		return instance->get();
	}
}

