package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.KnowledgeSource;
import com.dxsrc.aiteach.mapper.KnowledgeSourceMapper;
import com.dxsrc.aiteach.service.KnowledgeSyncService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class KnowledgeSyncServiceImpl extends ServiceImpl<KnowledgeSourceMapper, KnowledgeSource> implements KnowledgeSyncService {

    @Override
    public List<KnowledgeSource> list() {
        return super.list();
    }

    @Override
    public KnowledgeSource getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(KnowledgeSource entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(KnowledgeSource entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}
