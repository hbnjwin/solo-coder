package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.TranslationProject;
import java.util.List;

public interface TranslationService {

    List<TranslationProject> list();

    TranslationProject getById(Long id);

    boolean save(TranslationProject entity);

    boolean update(TranslationProject entity);

    boolean removeById(Long id);
}
