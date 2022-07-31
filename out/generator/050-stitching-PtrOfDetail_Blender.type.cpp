extern "C" {
	cv::Ptr<cv::detail::Blender>* cv_PtrOfDetail_Blender_new(cv::detail::Blender* val) {
		return new cv::Ptr<cv::detail::Blender>(val);
	}
	
	void cv_PtrOfDetail_Blender_delete(cv::Ptr<cv::detail::Blender>* instance) {
		delete instance;
	}

	const cv::detail::Blender* cv_PtrOfDetail_Blender_get_inner_ptr(const cv::Ptr<cv::detail::Blender>* instance) {
		return instance->get();
	}

	cv::detail::Blender* cv_PtrOfDetail_Blender_get_inner_ptr_mut(cv::Ptr<cv::detail::Blender>* instance) {
		return instance->get();
	}
}

