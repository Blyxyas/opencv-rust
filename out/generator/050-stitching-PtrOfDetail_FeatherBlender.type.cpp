extern "C" {
	cv::Ptr<cv::detail::FeatherBlender>* cv_PtrOfDetail_FeatherBlender_new(cv::detail::FeatherBlender* val) {
		return new cv::Ptr<cv::detail::FeatherBlender>(val);
	}
	
	void cv_PtrOfDetail_FeatherBlender_delete(cv::Ptr<cv::detail::FeatherBlender>* instance) {
		delete instance;
	}

	const cv::detail::FeatherBlender* cv_PtrOfDetail_FeatherBlender_get_inner_ptr(const cv::Ptr<cv::detail::FeatherBlender>* instance) {
		return instance->get();
	}

	cv::detail::FeatherBlender* cv_PtrOfDetail_FeatherBlender_get_inner_ptr_mut(cv::Ptr<cv::detail::FeatherBlender>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::Blender>* cv_PtrOfDetail_FeatherBlender_to_PtrOfDetail_Blender(cv::Ptr<cv::detail::FeatherBlender>* instance) {
		return new cv::Ptr<cv::detail::Blender>(instance->dynamicCast<cv::detail::Blender>());
	}
}

