package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.KnowledgeBase;
import java.util.List;

public interface KnowledgeQaService {

    List<KnowledgeBase> list();

    KnowledgeBase getById(Long id);

    boolean save(KnowledgeBase entity);

    boolean update(KnowledgeBase entity);

    boolean removeById(Long id);
}
