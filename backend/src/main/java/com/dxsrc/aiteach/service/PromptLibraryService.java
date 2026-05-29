package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.PromptTemplate;
import java.util.List;

public interface PromptLibraryService {

    List<PromptTemplate> list();

    PromptTemplate getById(Long id);

    boolean save(PromptTemplate entity);

    boolean update(PromptTemplate entity);

    boolean removeById(Long id);
}
