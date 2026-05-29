package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.KnowledgeBase;
import com.dxsrc.aiteach.mapper.KnowledgeBaseMapper;
import com.dxsrc.aiteach.service.KnowledgeQaService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class KnowledgeQaServiceImpl extends ServiceImpl<KnowledgeBaseMapper, KnowledgeBase> implements KnowledgeQaService {

    @Override
    public List<KnowledgeBase> list() {
        return super.list();
    }

    @Override
    public KnowledgeBase getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(KnowledgeBase entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(KnowledgeBase entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}
