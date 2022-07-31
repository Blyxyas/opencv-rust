extern "C" {
	cv::Ptr<cv::detail::MultiBandBlender>* cv_PtrOfDetail_MultiBandBlender_new(cv::detail::MultiBandBlender* val) {
		return new cv::Ptr<cv::detail::MultiBandBlender>(val);
	}
	
	void cv_PtrOfDetail_MultiBandBlender_delete(cv::Ptr<cv::detail::MultiBandBlender>* instance) {
		delete instance;
	}

	const cv::detail::MultiBandBlender* cv_PtrOfDetail_MultiBandBlender_get_inner_ptr(const cv::Ptr<cv::detail::MultiBandBlender>* instance) {
		return instance->get();
	}

	cv::detail::MultiBandBlender* cv_PtrOfDetail_MultiBandBlender_get_inner_ptr_mut(cv::Ptr<cv::detail::MultiBandBlender>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::Blender>* cv_PtrOfDetail_MultiBandBlender_to_PtrOfDetail_Blender(cv::Ptr<cv::detail::MultiBandBlender>* instance) {
		return new cv::Ptr<cv::detail::Blender>(instance->dynamicCast<cv::detail::Blender>());
	}
}

