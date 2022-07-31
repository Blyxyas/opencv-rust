extern "C" {
	cv::Ptr<cv::videostab::NullLog>* cv_PtrOfNullLog_new(cv::videostab::NullLog* val) {
		return new cv::Ptr<cv::videostab::NullLog>(val);
	}
	
	void cv_PtrOfNullLog_delete(cv::Ptr<cv::videostab::NullLog>* instance) {
		delete instance;
	}

	const cv::videostab::NullLog* cv_PtrOfNullLog_get_inner_ptr(const cv::Ptr<cv::videostab::NullLog>* instance) {
		return instance->get();
	}

	cv::videostab::NullLog* cv_PtrOfNullLog_get_inner_ptr_mut(cv::Ptr<cv::videostab::NullLog>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::ILog>* cv_PtrOfNullLog_to_PtrOfILog(cv::Ptr<cv::videostab::NullLog>* instance) {
		return new cv::Ptr<cv::videostab::ILog>(instance->dynamicCast<cv::videostab::ILog>());
	}
}

