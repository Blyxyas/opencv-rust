extern "C" {
	void cv_VectorOfbool_delete(std::vector<bool>* instance) {
		delete instance;
	}

	std::vector<bool>* cv_VectorOfbool_new() {
		return new std::vector<bool>();
	}

	size_t cv_VectorOfbool_len(const std::vector<bool>* instance) {
		return instance->size();
	}

	bool cv_VectorOfbool_is_empty(const std::vector<bool>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfbool_capacity(const std::vector<bool>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfbool_resize(std::vector<bool>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfbool_shrink_to_fit(std::vector<bool>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfbool_reserve(std::vector<bool>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfbool_remove(std::vector<bool>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfbool_swap(std::vector<bool>* instance, size_t index1, size_t index2) {
		instance->swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfbool_clear(std::vector<bool>* instance) {
		instance->clear();
	}

	void cv_VectorOfbool_push(std::vector<bool>* instance, bool val) {
		instance->push_back(val);
	}

	void cv_VectorOfbool_insert(std::vector<bool>* instance, size_t index, bool val) {
		instance->insert(instance->begin() + index, val);
	}

	void cv_VectorOfbool_get(const std::vector<bool>* instance, size_t index, bool* ocvrs_return) {
		*ocvrs_return = (*instance)[index];
	}

	void cv_VectorOfbool_set(std::vector<bool>* instance, size_t index, bool val) {
		(*instance)[index] = val;
	}

}


