package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.PromptTemplate;
import com.dxsrc.aiteach.mapper.PromptTemplateMapper;
import com.dxsrc.aiteach.service.PromptLibraryService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class PromptLibraryServiceImpl extends ServiceImpl<PromptTemplateMapper, PromptTemplate> implements PromptLibraryService {

    @Override
    public List<PromptTemplate> list() {
        return super.list();
    }

    @Override
    public PromptTemplate getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(PromptTemplate entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(PromptTemplate entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}
