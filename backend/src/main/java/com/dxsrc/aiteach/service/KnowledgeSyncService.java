package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.KnowledgeSource;
import java.util.List;

public interface KnowledgeSyncService {

    List<KnowledgeSource> list();

    KnowledgeSource getById(Long id);

    boolean save(KnowledgeSource entity);

    boolean update(KnowledgeSource entity);

    boolean removeById(Long id);
}
