extern "C" {
	void cv_PtrOfTonemapMantiuk_delete(cv::Ptr<cv::TonemapMantiuk>* instance) {
		delete instance;
	}

	const cv::TonemapMantiuk* cv_PtrOfTonemapMantiuk_get_inner_ptr(const cv::Ptr<cv::TonemapMantiuk>* instance) {
		return instance->get();
	}

	cv::TonemapMantiuk* cv_PtrOfTonemapMantiuk_get_inner_ptr_mut(cv::Ptr<cv::TonemapMantiuk>* instance) {
		return instance->get();
	}
}

